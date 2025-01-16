///Module containing a contract's types and functions.
/**

```solidity
library BN254 {
    struct G1Point { uint256 X; uint256 Y; }
    struct G2Point { uint256[2] X; uint256[2] Y; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BN254 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct G1Point { uint256 X; uint256 Y; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G1Point {
        pub X: alloy::sol_types::private::primitives::aliases::U256,
        pub Y: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { X: tuple.0, Y: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.Y),
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
        impl alloy_sol_types::SolType for G1Point {
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
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 X,uint256 Y)")
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
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
struct G2Point { uint256[2] X; uint256[2] Y; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G2Point {
        pub X: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        pub Y: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<256>,
                2usize,
            >,
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<256>,
                2usize,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
        impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
            fn from(value: G2Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { X: tuple.0, Y: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G2Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.Y),
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
        impl alloy_sol_types::SolType for G2Point {
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
        impl alloy_sol_types::SolStruct for G2Point {
            const NAME: &'static str = "G2Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "G2Point(uint256[2] X,uint256[2] Y)",
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
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G2Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
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
    /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

See the [wrapper's documentation](`BN254Instance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254Instance<T, P, N> {
        BN254Instance::<T, P, N>::new(address, provider)
    }
    /**A [`BN254`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BN254`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BN254Instance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BN254Instance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BN254Instance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

See the [wrapper's documentation](`BN254Instance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> BN254Instance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254Instance<T, P, N> {
            BN254Instance {
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
    > BN254Instance<T, P, N> {
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
    > BN254Instance<T, P, N> {
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
library IAllocationManagerTypes {
    struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
    struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256[] wadsToSlash; string description; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IAllocationManagerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CreateSetParams {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
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
        impl ::core::convert::From<CreateSetParams> for UnderlyingRustTuple<'_> {
            fn from(value: CreateSetParams) -> Self {
                (value.operatorSetId, value.strategies)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CreateSetParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorSetId: tuple.0,
                    strategies: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CreateSetParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CreateSetParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
        impl alloy_sol_types::SolType for CreateSetParams {
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
        impl alloy_sol_types::SolStruct for CreateSetParams {
            const NAME: &'static str = "CreateSetParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CreateSetParams(uint32 operatorSetId,address[] strategies)",
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSetId)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CreateSetParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
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
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
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
library IBLSSignatureChecker {
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IBLSSignatureChecker {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerStakesAndSignature {
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        pub nonSignerPubkeys: alloy::sol_types::private::Vec<
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        >,
        pub quorumApks: alloy::sol_types::private::Vec<
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        >,
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        pub nonSignerStakeIndices: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<u32>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<BN254::G1Point>,
            alloy::sol_types::sol_data::Array<BN254::G1Point>,
            BN254::G2Point,
            BN254::G1Point,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            >,
            <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
        impl ::core::convert::From<NonSignerStakesAndSignature>
        for UnderlyingRustTuple<'_> {
            fn from(value: NonSignerStakesAndSignature) -> Self {
                (
                    value.nonSignerQuorumBitmapIndices,
                    value.nonSignerPubkeys,
                    value.quorumApks,
                    value.apkG2,
                    value.sigma,
                    value.quorumApkIndices,
                    value.totalStakeIndices,
                    value.nonSignerStakeIndices,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NonSignerStakesAndSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonSignerQuorumBitmapIndices: tuple.0,
                    nonSignerPubkeys: tuple.1,
                    quorumApks: tuple.2,
                    apkG2: tuple.3,
                    sigma: tuple.4,
                    quorumApkIndices: tuple.5,
                    totalStakeIndices: tuple.6,
                    nonSignerStakeIndices: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for NonSignerStakesAndSignature {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for NonSignerStakesAndSignature {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerQuorumBitmapIndices,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerPubkeys),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApks),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApkIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerStakeIndices),
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
        impl alloy_sol_types::SolType for NonSignerStakesAndSignature {
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
        impl alloy_sol_types::SolStruct for NonSignerStakesAndSignature {
            const NAME: &'static str = "NonSignerStakesAndSignature";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "NonSignerStakesAndSignature(uint32[] nonSignerQuorumBitmapIndices,BN254.G1Point[] nonSignerPubkeys,BN254.G1Point[] quorumApks,BN254.G2Point apkG2,BN254.G1Point sigma,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(4);
                components
                    .push(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BN254::G2Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BN254::G2Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerQuorumBitmapIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerPubkeys,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumApks)
                        .0,
                    <BN254::G2Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.apkG2,
                        )
                        .0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sigma,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumApkIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerStakeIndices,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for NonSignerStakesAndSignature {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerQuorumBitmapIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerPubkeys,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApks,
                    )
                    + <BN254::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.apkG2,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigma,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApkIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerStakeIndices,
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
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerQuorumBitmapIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254::G1Point,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerPubkeys,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254::G1Point,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApks,
                    out,
                );
                <BN254::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.apkG2,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigma,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApkIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    >,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerStakeIndices,
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
struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumStakeTotals {
        pub signedStakeForQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
        >,
        pub totalStakeForQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U96,
            >,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U96,
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
        impl ::core::convert::From<QuorumStakeTotals> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumStakeTotals) -> Self {
                (value.signedStakeForQuorum, value.totalStakeForQuorum)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumStakeTotals {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signedStakeForQuorum: tuple.0,
                    totalStakeForQuorum: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumStakeTotals {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumStakeTotals {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.signedStakeForQuorum),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeForQuorum),
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
        impl alloy_sol_types::SolType for QuorumStakeTotals {
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
        impl alloy_sol_types::SolStruct for QuorumStakeTotals {
            const NAME: &'static str = "QuorumStakeTotals";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumStakeTotals(uint96[] signedStakeForQuorum,uint96[] totalStakeForQuorum)",
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signedStakeForQuorum,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeForQuorum,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumStakeTotals {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signedStakeForQuorum,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeForQuorum,
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
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<96>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signedStakeForQuorum,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<96>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeForQuorum,
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
    /**Creates a new wrapper around an on-chain [`IBLSSignatureChecker`](self) contract instance.

See the [wrapper's documentation](`IBLSSignatureCheckerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBLSSignatureCheckerInstance<T, P, N> {
        IBLSSignatureCheckerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IBLSSignatureChecker`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IBLSSignatureChecker`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBLSSignatureCheckerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBLSSignatureCheckerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBLSSignatureCheckerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IBLSSignatureCheckerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IBLSSignatureChecker`](self) contract instance.

See the [wrapper's documentation](`IBLSSignatureCheckerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IBLSSignatureCheckerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBLSSignatureCheckerInstance<T, P, N> {
            IBLSSignatureCheckerInstance {
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
    > IBLSSignatureCheckerInstance<T, P, N> {
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
    > IBLSSignatureCheckerInstance<T, P, N> {
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
library IRewardsCoordinatorTypes {
    struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
    struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IRewardsCoordinatorTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RewardsSubmission {
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        pub token: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub startTimestamp: u32,
        pub duration: u32,
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
            alloy::sol_types::sol_data::Array<StrategyAndMultiplier>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            u32,
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
        impl ::core::convert::From<RewardsSubmission> for UnderlyingRustTuple<'_> {
            fn from(value: RewardsSubmission) -> Self {
                (
                    value.strategiesAndMultipliers,
                    value.token,
                    value.amount,
                    value.startTimestamp,
                    value.duration,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RewardsSubmission {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategiesAndMultipliers: tuple.0,
                    token: tuple.1,
                    amount: tuple.2,
                    startTimestamp: tuple.3,
                    duration: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RewardsSubmission {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RewardsSubmission {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.strategiesAndMultipliers,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
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
        impl alloy_sol_types::SolType for RewardsSubmission {
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
        impl alloy_sol_types::SolStruct for RewardsSubmission {
            const NAME: &'static str = "RewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,uint256 amount,uint32 startTimestamp,uint32 duration)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategiesAndMultipliers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.duration)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RewardsSubmission {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategiesAndMultipliers,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.duration,
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
                <alloy::sol_types::sol_data::Array<
                    StrategyAndMultiplier,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategiesAndMultipliers,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.duration,
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
struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyAndMultiplier {
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
        impl ::core::convert::From<StrategyAndMultiplier> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyAndMultiplier) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyAndMultiplier {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyAndMultiplier {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyAndMultiplier {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
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
        impl alloy_sol_types::SolType for StrategyAndMultiplier {
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
        impl alloy_sol_types::SolStruct for StrategyAndMultiplier {
            const NAME: &'static str = "StrategyAndMultiplier";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyAndMultiplier(address strategy,uint96 multiplier)",
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
        impl alloy_sol_types::EventTopic for StrategyAndMultiplier {
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
    /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRewardsCoordinatorTypesInstance<T, P, N> {
        IRewardsCoordinatorTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRewardsCoordinatorTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IRewardsCoordinatorTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRewardsCoordinatorTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRewardsCoordinatorTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRewardsCoordinatorTypesInstance")
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
    > IRewardsCoordinatorTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IRewardsCoordinatorTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRewardsCoordinatorTypesInstance<T, P, N> {
            IRewardsCoordinatorTypesInstance {
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
    > IRewardsCoordinatorTypesInstance<T, P, N> {
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
    > IRewardsCoordinatorTypesInstance<T, P, N> {
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
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISignatureUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithSaltAndExpiry {
        pub signature: alloy::sol_types::private::Bytes,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<SignatureWithSaltAndExpiry>
        for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SignatureWithSaltAndExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    salt: tuple.1,
                    expiry: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithSaltAndExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for SignatureWithSaltAndExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
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
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithSaltAndExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
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
    /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsInstance<T, P, N> {
        ISignatureUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISignatureUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsInstance<T, P, N> {
            ISignatureUtilsInstance {
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
    > ISignatureUtilsInstance<T, P, N> {
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
    > ISignatureUtilsInstance<T, P, N> {
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
library BN254 {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }
}

library IAllocationManagerTypes {
    struct CreateSetParams {
        uint32 operatorSetId;
        address[] strategies;
    }
    struct SlashingParams {
        address operator;
        uint32 operatorSetId;
        address[] strategies;
        uint256[] wadsToSlash;
        string description;
    }
}

library IBLSSignatureChecker {
    struct NonSignerStakesAndSignature {
        uint32[] nonSignerQuorumBitmapIndices;
        BN254.G1Point[] nonSignerPubkeys;
        BN254.G1Point[] quorumApks;
        BN254.G2Point apkG2;
        BN254.G1Point sigma;
        uint32[] quorumApkIndices;
        uint32[] totalStakeIndices;
        uint32[][] nonSignerStakeIndices;
    }
    struct QuorumStakeTotals {
        uint96[] signedStakeForQuorum;
        uint96[] totalStakeForQuorum;
    }
}

library IRewardsCoordinatorTypes {
    struct RewardsSubmission {
        StrategyAndMultiplier[] strategiesAndMultipliers;
        address token;
        uint256 amount;
        uint32 startTimestamp;
        uint32 duration;
    }
    struct StrategyAndMultiplier {
        address strategy;
        uint96 multiplier;
    }
}

library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface MockAvsServiceManager {
    event Initialized(uint8 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);
    event SlasherProposed(address newSlasher, uint256 slasherProposalTimestamp);
    event SlasherUpdated(address prevSlasher, address newSlasher);
    event StaleStakesForbiddenUpdate(bool value);

    constructor(address _registryCoordinator, address _avsDirectory, address _rewardsCoordinator, address _allocationManager);

    function SLASHER_PROPOSAL_DELAY() external view returns (uint256);
    function acceptProposedSlasher() external;
    function addStrategyToOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    function allocationManager() external view returns (address);
    function avsDirectory() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureChecker.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureChecker.QuorumStakeTotals memory, bytes32);
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function createOperatorSets(IAllocationManagerTypes.CreateSetParams[] memory params) external;
    function delegation() external view returns (address);
    function deregisterOperatorFromAVS(address operator) external;
    function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
    function getOperatorRestakedStrategies(address operator) external view returns (address[] memory);
    function getRestakeableStrategies() external view returns (address[] memory);
    function initialize(address _initialOwner) external;
    function migrationFinalized() external view returns (bool);
    function owner() external view returns (address);
    function proposeNewSlasher(address newSlasher) external;
    function proposedSlasher() external view returns (address);
    function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function registryCoordinator() external view returns (address);
    function removeStrategiesFromOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    function renounceOwnership() external;
    function rewardsInitiator() external view returns (address);
    function setAVSRegistrar(address registrar) external;
    function setRewardsInitiator(address newRewardsInitiator) external;
    function setStaleStakesForbidden(bool value) external;
    function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
    function slasher() external view returns (address);
    function slasherProposalTimestamp() external view returns (uint256);
    function stakeRegistry() external view returns (address);
    function staleStakesForbidden() external view returns (bool);
    function transferOwnership(address newOwner) external;
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
    function updateAVSMetadataURI(string memory _metadataURI) external;
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
      },
      {
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      },
      {
        "name": "_rewardsCoordinator",
        "type": "address",
        "internalType": "contract IRewardsCoordinator"
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
    "name": "SLASHER_PROPOSAL_DELAY",
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
    "name": "acceptProposedSlasher",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addStrategyToOperatorSet",
    "inputs": [
      {
        "name": "operatorSetId",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
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
        "internalType": "address"
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
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "blsApkRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBLSApkRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkSignatures",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IBLSSignatureChecker.NonSignerStakesAndSignature",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerPubkeys",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApks",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apkG2",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "sigma",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IBLSSignatureChecker.QuorumStakeTotals",
        "components": [
          {
            "name": "signedStakeForQuorum",
            "type": "uint96[]",
            "internalType": "uint96[]"
          },
          {
            "name": "totalStakeForQuorum",
            "type": "uint96[]",
            "internalType": "uint96[]"
          }
        ]
      },
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
    "name": "createAVSRewardsSubmission",
    "inputs": [
      {
        "name": "rewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinatorTypes.RewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinatorTypes.StrategyAndMultiplier[]",
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
          },
          {
            "name": "token",
            "type": "address",
            "internalType": "contract IERC20"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "duration",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createOperatorSets",
    "inputs": [
      {
        "name": "params",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.CreateSetParams[]",
        "components": [
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "deregisterOperatorFromAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperatorFromOperatorSets",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getOperatorRestakedStrategies",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRestakeableStrategies",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_initialOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "migrationFinalized",
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
    "name": "owner",
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
    "name": "proposeNewSlasher",
    "inputs": [
      {
        "name": "newSlasher",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "proposedSlasher",
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
    "name": "registerOperatorToAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
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
        "internalType": "contract IRegistryCoordinator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "removeStrategiesFromOperatorSet",
    "inputs": [
      {
        "name": "operatorSetId",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rewardsInitiator",
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
    "name": "setAVSRegistrar",
    "inputs": [
      {
        "name": "registrar",
        "type": "address",
        "internalType": "contract IAVSRegistrar"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setRewardsInitiator",
    "inputs": [
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setStaleStakesForbidden",
    "inputs": [
      {
        "name": "value",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slashOperator",
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
    "name": "slasherProposalTimestamp",
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
    "name": "stakeRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "staleStakesForbidden",
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
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "trySignatureAndApkVerification",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "apk",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "apkG2",
        "type": "tuple",
        "internalType": "struct BN254.G2Point",
        "components": [
          {
            "name": "X",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          },
          {
            "name": "Y",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          }
        ]
      },
      {
        "name": "sigma",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "pairingSuccessful",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "siganatureIsValid",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "updateAVSMetadataURI",
    "inputs": [
      {
        "name": "_metadataURI",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RewardsInitiatorUpdated",
    "inputs": [
      {
        "name": "prevRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlasherProposed",
    "inputs": [
      {
        "name": "newSlasher",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "slasherProposalTimestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlasherUpdated",
    "inputs": [
      {
        "name": "prevSlasher",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newSlasher",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StaleStakesForbiddenUpdate",
    "inputs": [
      {
        "name": "value",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
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
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod MockAvsServiceManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101a080604052346102b0576080816148cd803803809161002082856103e6565b8339810103126102b05780516001600160a01b038116918282036102b05760208101516001600160a01b03811681036102b0576040820151916001600160a01b03831683036102b05760600151916001600160a01b03831683036102b057604051636830483560e01b815291602083600481895afa9283156102bc575f936103c5575b5060805260a0528260c05260e052610100525f5460ff8160081c166103705760ff80821610610336575b5061012052604051636830483560e01b8152602081600481855afa9182156102bc576004926020925f91610309575b5061014052604051632efa2ca360e11b815292839182905afa9081156102bc575f916102c7575b50610160526101405160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa9081156102bc575f91610276575b5061018052604051614490908161043d8239608051818181610a5f01528181611216015281816112ba015261330b015260a051816118c5015260c0518181816111e6015281816113e90152818161170b01528181611e34015281816131d3015281816132db015261347d015260e051818181611ed0015281816134c90152613ed7015261010051818181610b2201528181611419015281816115ca015281816116110152818161173b015281816121a6015261320301526101205181818161088301528181610aa30152818161293d01528181612a300152612ffb015261014051818181610a1b01528181612d4e0152612eac0152610160518181816109ba0152612c8a0152610180518181816116760152612b430152f35b90506020813d6020116102b4575b81610291602093836103e6565b810103126102b057516001600160a01b03811681036102b0575f61015d565b5f80fd5b3d9150610284565b6040513d5f823e3d90fd5b90506020813d602011610301575b816102e2602093836103e6565b810103126102b057516001600160a01b03811681036102b0575f610123565b3d91506102d5565b6103299150833d851161032f575b61032181836103e6565b81019061041d565b5f6100fc565b503d610317565b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100cd565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b6103df91935060203d60201161032f5761032181836103e6565b915f6100a3565b601f909101601f19168101906001600160401b0382119082101761040957604052565b634e487b7160e01b5f52604160045260245ffd5b908160209103126102b057516001600160a01b03811681036102b0579056fe60806040526004361015610011575f80fd5b5f3560e01c8063171f1d5b1461024457806326f017e21461023f57806333cfb7b71461023a5780633bc28c8c146102355780633d07142214610230578063416c7e5e1461022b5780635df459461461022657806367940c8914610221578063683048351461021c5780636b3aa72e146102175780636d14a987146102125780636ecbccfe1461020d5780636efb463614610208578063715018a614610203578063847d634f146101fe5780638999817f146101f95780638d68349a146101f45780638da5cb5b146101ef5780639926ee7d146101ea578063a364f4da146101e5578063a98fb355146101e0578063b1344271146101db578063b98d0908146101d6578063c1a8e2c5146101d1578063c4d66de8146101cc578063ca8aa7c7146101c7578063ce7b5e4b146101c2578063df5cf723146101bd578063e46f1816146101b8578063e481af9d146101b3578063f25f1610146101ae578063f2fde38b146101a9578063fc299dee146101a4578063fcd1c3751461019f5763fce36c7d1461019a575f80fd5b61187e565b611861565b611839565b6117a8565b6116e8565b6116cd565b6116a5565b611661565b6115f9565b6115b5565b6114bb565b6113a2565b611350565b611328565b61127f565b6111c2565b611126565b6110fe565b6110dc565b611051565b610f65565b610f0a565b610e75565b610b0a565b610a8e565b610a4a565b610a06565b6109e9565b6109a5565b610851565b610770565b6105d3565b61059b565b610481565b61040f565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761027857604052565b610249565b606081019081106001600160401b0382111761027857604052565b90601f801991011681019081106001600160401b0382111761027857604052565b604051906102c860a083610298565b565b604051906102c861010083610298565b604051906102c8606083610298565b604051906102c8604083610298565b906102c86040519283610298565b60409060e319011261032f576040519061031f8261025d565b60e4358252610104356020830152565b5f80fd5b919082604091031261032f5760405161034b8161025d565b6020808294803584520135910152565b9080601f8301121561032f5760405191610376604084610298565b82906040810192831161032f57905b8282106103925750505090565b8135815260209182019101610385565b90608060631983011261032f576040516103bb8161025d565b60206103d682946103cd81606461035b565b845260a461035b565b910152565b919060808382031261032f5760206103d6604051926103f98461025d565b60408496610407838261035b565b86520161035b565b3461032f5761012036600319011261032f57600435604036602319011261032f5761046760409182516104418161025d565b60243581526044356020820152610457366103a2565b9061046136610306565b92611b9e565b8251911515825215156020820152f35b5f91031261032f57565b3461032f575f36600319011261032f57610499613ce5565b60685462093a8081018091116105385742106104dc576067546104c9906001600160a01b0316613d3d565b613d3d565b606780546001600160a01b03191690555b005b60405162461bcd60e51b815260206004820152602e60248201527f536572766963654d616e616765723a20536c61736865722070726f706f73616c60448201526d0819195b185e481b9bdd081b595d60921b6064820152608490fd5b611c84565b6001600160a01b0381160361032f57565b35906102c88261053d565b60206040818301928281528451809452019201905f5b81811061057c5750505090565b82516001600160a01b031684526020938401939092019160010161056f565b3461032f57602036600319011261032f576105cf6105c36004356105be8161053d565b611e15565b60405191829182610559565b0390f35b3461032f57602036600319011261032f576104da6004356105f38161053d565b6105fb613ce5565b613e39565b63ffffffff81160361032f57565b604435906102c882610600565b35906102c882610600565b6001600160401b0381116102785760051b60200190565b9080601f8301121561032f57813561065481610626565b926106626040519485610298565b81845260208085019260051b82010192831161032f57602001905b82821061068a5750505090565b6020809183356106998161053d565b81520191019061067d565b9080601f8301121561032f5781356106bb81610626565b926106c96040519485610298565b81845260208085019260051b82010192831161032f57602001905b8282106106f15750505090565b81358152602091820191016106e4565b6001600160401b03811161027857601f01601f191660200190565b92919261072882610701565b916107366040519384610298565b82948184528183011161032f578281602093845f960137010152565b9080601f8301121561032f5781602061076d9335910161071c565b90565b3461032f57602036600319011261032f576004356001600160401b03811161032f5760a0600319823603011261032f576107a86102b9565b6107b48260040161054e565b81526107c26024830161061b565b602082015260448201356001600160401b03811161032f576107ea906004369185010161063d565b604082015260648201356001600160401b03811161032f5761081290600436918501016106a4565b606082015260848201356001600160401b03811161032f576104da92600461083d9236920101610752565b6080820152612192565b8015150361032f57565b3461032f57602036600319011261032f5760043561086e81610847565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156109a0575f91610965575b506001600160a01b031633036108d4576104da90613e97565b60405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a490fd5b90506020813d602011610998575b8161098060209383610298565b8101031261032f57516109928161053d565b5f6108bb565b3d9150610973565b611cfa565b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461032f575f36600319011261032f57602060405162093a808152f35b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b90604060031983011261032f57600435610aeb81610600565b91602435906001600160401b03821161032f5761076d9160040161063d565b3461032f57610b1836610ad2565b610b20613ed5565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561032f57604051630287f75160e51b8152915f918391610b7291306004850161237b565b93818381819703925af180156109a057610b8a575080f35b6104da91505f90610298565b929190610ba281610626565b93610bb06040519586610298565b602085838152019160051b810192831161032f57905b828210610bd257505050565b602080918335610be181610600565b815201910190610bc6565b9080601f8301121561032f5781602061076d93359101610b96565b81601f8201121561032f578035610c1d81610626565b92610c2b6040519485610298565b81845260208085019260061b8401019281841161032f57602001915b838310610c55575050505090565b6020604091610c648486610333565b815201920191610c47565b9080601f8301121561032f578135610c8681610626565b92610c946040519485610298565b81845260208085019260051b8201019183831161032f5760208201905b838210610cc057505050505090565b81356001600160401b03811161032f57602091610ce287848094880101610bec565b815201910190610cb1565b9190916101808184031261032f57610d036102ca565b9281356001600160401b03811161032f5781610d20918401610bec565b845260208201356001600160401b03811161032f5781610d41918401610c07565b602085015260408201356001600160401b03811161032f5781610d65918401610c07565b6040850152610d7781606084016103db565b6060850152610d898160e08401610333565b60808501526101208201356001600160401b03811161032f5781610dae918401610bec565b60a08501526101408201356001600160401b03811161032f5781610dd3918401610bec565b60c08501526101608201356001600160401b03811161032f57610df69201610c6f565b60e0830152565b90602080835192838152019201905f5b818110610e1a5750505090565b82516001600160601b0316845260209384019390920191600101610e0d565b929190610e706020916040865282610e5c82516040808a01526080890190610dfd565b910151868203603f19016060880152610dfd565b930152565b3461032f57608036600319011261032f576004356024356001600160401b03811161032f573660238201121561032f5780600401356001600160401b03811161032f57366024828401011161032f57610ecc61060e565b90606435936001600160401b03851161032f576024610ef2610efa963690600401610ced565b940190612860565b906105cf60405192839283610e39565b3461032f575f36600319011261032f57610f22613ce5565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461032f57602036600319011261032f576004356001600160401b03811161032f573660238201121561032f57806004013590610fa182610626565b90610faf6040519283610298565b8282526024602083019360051b8201019036821161032f5760248101935b828510610fdd576104da846131cd565b84356001600160401b03811161032f5782016040602319823603011261032f576040519161100a8361025d565b602482013561101881610600565b83526044820135926001600160401b03841161032f5761104260209493602486953692010161063d565b83820152815201940193610fcd565b3461032f57602036600319011261032f577f2f8afc8a78fd958f3301c0233aa326b9c4b9a2884a7483227d6b0555aaa03adb6110d76004356110928161053d565b61109a613ce5565b606780546001600160a01b0319166001600160a01b0392909216918217905542606881905560408051928352602083019190915290918291820190565b0390a1005b3461032f575f36600319011261032f57602060ff606954166040519015158152f35b3461032f575f36600319011261032f576033546040516001600160a01b039091168152602090f35b3461032f57604036600319011261032f576004356111438161053d565b602435906001600160401b03821161032f576060600319833603011261032f57604051906111708261027d565b82600401356001600160401b03811161032f5783013660238201121561032f576104da936111aa604492369060246004820135910161071c565b845260248101356020850152013560408301526132d5565b3461032f575f602036600319011261032f576004356111e08161053d565b611214337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614613140565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b1561032f576040516351b27a6d60e11b81526001600160a01b039091166004820152905f908290602490829084905af180156109a057610b8a575080f35b3461032f575f602036600319011261032f576004356001600160401b03811161032f576112b0903690600401610752565b6112b8613ce5565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561032f5760405163a98fb35560e01b815260206004820152915f918391829084908290611316906024830190612357565b03925af180156109a057610b8a575080f35b3461032f575f36600319011261032f576066546040516001600160a01b039091168152602090f35b3461032f575f36600319011261032f57602060ff609754166040519015158152f35b9181601f8401121561032f578235916001600160401b03831161032f576020808501948460051b01011161032f57565b3461032f57604036600319011261032f576004356113bf8161053d565b6024356001600160401b03811161032f576113e1611469913690600401611372565b9190611417337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614613140565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169261145c61144d6102da565b6001600160a01b039096168652565b3060208601523691610b96565b6040830152803b1561032f57604051636e3492b560e01b8152905f90829081838161149788600483016133a1565b03925af180156109a0576114a757005b806114b55f6104da93610298565b80610477565b3461032f57602036600319011261032f576004356114d88161053d565b6115295f549161150c6114f66114f28560ff9060081c1690565b1590565b809481956115a7575b8115611587575b5061340b565b8261151d600160ff195f5416175f55565b611570575b80806141ec565b61152f57005b61153d61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016110d7565b61158261010061ff00195f5416175f55565b611522565b303b15915081611599575b505f611506565b60ff1660011490505f611592565b600160ff82161091506114ff565b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461032f5761160736610ad2565b61160f613ed5565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561032f5760405163b66bd98960e01b8152915f918391610b7291306004850161237b565b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461032f575f36600319011261032f576067546040516001600160a01b039091168152602090f35b3461032f575f36600319011261032f576105cf6105c361346e565b3461032f57602036600319011261032f576004356117058161053d565b611739337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614613140565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b1561032f576040516334f65bfd60e21b81523060048201526001600160a01b039091166024820152905f908290604490829084905af180156109a0576114a757005b3461032f57602036600319011261032f576004356117c58161053d565b6117cd613ce5565b6001600160a01b038116156117e5576104da906141a4565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461032f575f36600319011261032f576065546040516001600160a01b039091168152602090f35b3461032f575f36600319011261032f576020606854604051908152f35b3461032f57602036600319011261032f576004356001600160401b03811161032f576118ae903690600401611372565b6065549091906001600160a01b03163303611ae0577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b8181106119225750823b1561032f57611497925f928360405180968195829463fce36c7d60e01b845260048401613739565b5f602061198961194c6119408361193a87898b61369b565b016136bd565b6001600160a01b031690565b604061195986888a61369b565b6040516323b872dd60e01b8152336004820152306024820152910135604482015293849283919082906064820190565b03925af180156109a057611ac4575b506119ac611940602061193a84868861369b565b604051636eb1769f60e11b81523060048201526001600160a01b03861660248201529190602090839060449082905afa80156109a057611a4f6020915f948591611a97575b50611a1c611a076119408561193a888b8d61369b565b916040611a15878a8c61369b565b0135611cde565b60405163095ea7b360e01b81526001600160a01b038a166004820152602481019190915294859283919082906044820190565b03925af19182156109a057600192611a69575b50016118f0565b611a899060203d8111611a90575b611a818183610298565b8101906136c7565b505f611a62565b503d611a77565b611ab79150833d8111611abd575b611aaf8183610298565b810190611ceb565b5f6119f1565b503d611aa5565b611adb9060203d8111611a9057611a818183610298565b611998565b60405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a490fd5b634e487b7160e01b5f52603260045260245ffd5b906002811015611b855760051b0190565b611b60565b634e487b7160e01b5f52601260045260245ffd5b611c7a611c57611c8095611c51611c4a85875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e0840152610100830152611c2181610120840103601f198101835282610298565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b809661388c565b906138d2565b92611c51611c6c611c6661395a565b94613a51565b91611c75613b6d565b61388c565b91613bb7565b9091565b634e487b7160e01b5f52601160045260245ffd5b906001820180921161053857565b906002820180921161053857565b906003820180921161053857565b906004820180921161053857565b906005820180921161053857565b9190820180921161053857565b9081602091031261032f575190565b6040513d5f823e3d90fd5b9081602091031261032f57516001600160c01b038116810361032f5790565b9081602091031261032f575160ff8116810361032f5790565b60405190611d4c602083610298565b5f808352366020840137565b90611d6282610626565b611d6f6040519182610298565b8281528092611d80601f1991610626565b0190602036910137565b805115611b855760200190565b908151811015611b85570160200190565b6001600160601b0381160361032f57565b9081604091031261032f57602060405191611dd38361025d565b8051611dde8161053d565b83520151611deb81611da8565b602082015290565b8051821015611b855760209160051b010190565b5f1981146105385760010190565b6040516309aa152760e11b81526001600160a01b0391821660048201527f000000000000000000000000000000000000000000000000000000000000000090911690602081602481855afa9081156109a057611e95916020915f91612175575b506040518093819263871ef04960e01b8352600483019190602083019252565b0381855afa9081156109a0575f91612146575b506001600160c01b03169081159081156120e3575b506120da57611ecb90613d9b565b5f91907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b8151851015611f9757611f506020611f2d611f27611f198987611d97565b516001600160f81b03191690565b60f81c90565b604051633ca5a5f560e01b815260ff909116600482015291829081906024820190565b0381875afa80156109a057600192611f6f925f92611f77575b50611cde565b940193611efb565b611f9091925060203d8111611abd57611aaf8183610298565b905f611f69565b611fa2919450611d58565b925f905f5b81518110156120d457611fc0611f27611f198385611d97565b604051633ca5a5f560e01b815260ff8216600482015290602082602481895afa9182156109a0575f926120b4575b50905f915b81831061200557505050600101611fa7565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156109a0576120788b6120698361206361194060019861207d985f91612086575b50516001600160a01b031690565b92611df3565b6001600160a01b039091169052565b611e07565b95019190611ff3565b6120a7915060403d81116120ad575b61209f8183610298565b810190611db9565b5f612055565b503d612095565b6120cd91925060203d8111611abd57611aaf8183610298565b905f611fee565b50505050565b5061076d611d3d565b604051639aa1653d60e01b81529150602090829060049082905afa80156109a05760ff915f91612117575b5016155f611ebd565b612139915060203d60201161213f575b6121318183610298565b810190611d24565b5f61210e565b503d612127565b612168915060203d60201161216e575b6121608183610298565b810190611d05565b5f611ea8565b503d612156565b61218c9150823d8411611abd57611aaf8183610298565b5f611e75565b6066546001600160a01b031633036122b0577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561032f5760408051633635205760e01b81523060048201526024810182905283516001600160a01b03166044820152602084015163ffffffff1660648201529083015160a060848301529092839161222d9060e484019061231b565b906060810151916043198482030160a4850152602080845192838152019301905f5b818110612297575050505f836122788296946080839501516043198483030160c4850152612357565b03925af180156109a0576122895750565b806114b55f6102c893610298565b825185528795506020948501949092019160010161224f565b60405162461bcd60e51b815260206004820152603960248201527f536572766963654d616e61676572426173652e6f6e6c79536c61736865723a2060448201527f63616c6c6572206973206e6f742074686520736c6173686572000000000000006064820152608490fd5b90602080835192838152019201905f5b8181106123385750505090565b82516001600160a01b031684526020938401939092019160010161232b565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b63ffffffff61076d949360609360018060a01b03168352166020820152816040820152019061231b565b604051906123b28261025d565b60606020838281520152565b156123c557565b60405162461bcd60e51b815260206004820152603760248201525f51602061443b5f395f51905f5260448201527f7265733a20656d7074792071756f72756d20696e7075740000000000000000006064820152608490fd5b1561242457565b60405162461bcd60e51b815260206004820152604160248201525f51602061443b5f395f51905f5260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a490fd5b1561248d57565b60a460405162461bcd60e51b815260206004820152604460248201525f51602061443b5f395f51905f5260448201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b6084820152fd5b156124f857565b60405162461bcd60e51b815260206004820152603c60248201525f51602061443b5f395f51905f5260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608490fd5b5f1981019190821161053857565b1561256557565b608460405162461bcd60e51b815260206004820152604060248201525f51602061443b5f395f51905f5260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152fd5b9081602091031261032f575161076d81610600565b90821015611b85570190565b156125e457565b60405162461bcd60e51b815260206004820152606660248201525f51602061443b5f395f51905f5260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c490fd5b9081602091031261032f575167ffffffffffffffff198116810361032f5790565b1561269957565b60405162461bcd60e51b815260206004820152606160248201525f51602061443b5f395f51905f5260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c490fd5b9081602091031261032f575161076d81611da8565b906001600160601b03809116911603906001600160601b03821161053857565b1561275d57565b60405162461bcd60e51b815260206004820152604360248201525f51602061443b5f395f51905f5260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a490fd5b156127c857565b60405162461bcd60e51b815260206004820152603960248201525f51602061443b5f395f51905f5260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608490fd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b81811061284a5750505090565b825184526020938401939092019160010161283d565b94939290919361286e6123a5565b5061287a8515156123be565b604084015151851480613132575b80613124575b80613116575b61289d9061241d565b6128af60208501515185515114612486565b6128c663ffffffff431663ffffffff8416106124f1565b6128ce6102e9565b5f81525f6020820152926128e06123a5565b6128e987611d58565b60208201526128f787611d58565b81526129016123a5565b92612910602088015151611d58565b8452612920602088015151611d58565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156109a057612989915f916130f7575b50612984368b8761071c565b613f81565b985f965b60208901518051891015612b05576020886129fa6129f08c6129e88f96868e6129cd6129ba868095611df3565b5180515f526020015160205260405f2090565b6129da8484840151611df3565b5282612ad2575b0151611df3565b519551611df3565b5163ffffffff1690565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9182156109a057611c518a612aa78f612aa08f8460208f92612a9793612a8f8460019e612aad9e5f91612ab5575b508f8060c01b03169251611df3565b520151611df3565b51938d51611df3565b5116614008565b90614039565b97019661298d565b612acc9150863d811161216e576121608183610298565b5f612a80565b612b00612ae28484840151611df3565b51612af984840151612af387612550565b90611df3565b511061255e565b6129e1565b50909597949650612b1a91989392995061411f565b91612b2760975460ff1690565b9081156130ef576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156109a0575f916130c0575b5091905b5f925b818410612be657505050505092612bbf612bba612bb3612be09585612bd29860806060602099015192015192611b9e565b9190612756565b6127c1565b0151604051928391602083019586612820565b03601f198101835282610298565b51902090565b92989596909399919794878b888c888d612fba575b6129f08260a0612c49611f27612c3b84612c5197612c35612c276129ba8f9c604060209f9e0151611df3565b67ffffffffffffffff191690565b9b6125d1565b356001600160f81b03191690565b970151611df3565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156109a057612d156129f08f958f90612d0d8f978f96848f612d0760c096612d00848f60209f906129e1612c3b99604093611f279c5f91612f8c575b5067ffffffffffffffff19918216911614612692565b51906138d2565b9c6125d1565b960151611df3565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156109a057612da2918c8f925f92612f68575b506020612d9492930151611df3565b906001600160601b03169052565b612dcf8c612d948c612dc8612dbb826020860151611df3565b516001600160601b031690565b9251611df3565b5f985f5b60208a015151811015612f4f578b8d612e1189612e04611f27612c3b868f89612dfc9151611df3565b5194876125d1565b60ff161c60019081161490565b612e20575b5050600101612dd3565b8a8a612ea8859f948f9686612e628f9360e0612e596129f0956020612e51611f27612c3b839f612e689c89916125d1565b9a0151611df3565b519b0151611df3565b51611df3565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156109a0578f612f0e908f936001959486955f92612f19575b50612063612d9492935193612f09612dbb8487611df3565b612736565b019a90508b8d612e16565b612d949250612f416120639160203d8111612f48575b612f398183610298565b810190612721565b9250612ef1565b503d612f2f565b5093919796996001919699509a94929a01929190612b82565b612d949250612f85602091823d8111612f4857612f398183610298565b9250612d85565b6020612fad92503d8111612fb3575b612fa58183610298565b810190612671565b5f612cea565b503d612f9b565b612ff79450612fd49250611f2791612c3b916020956125d1565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156109a057602089612c518f938f60a08f97611f27612c3b8f8f90612c35612c276129ba8f60408b96918f88936129f09f61307b9061308193612c499f5f92613097575b5063ffffffff809116931690611cde565b116125dd565b5050505050509750505050505092935050612bfb565b602063ffffffff92935082916130b8913d8111611abd57611aaf8183610298565b92915061306a565b6130e2915060203d6020116130e8575b6130da8183610298565b8101906125bc565b5f612b7b565b503d6130d0565b5f9190612b7f565b613110915060203d60201161213f576121318183610298565b5f612978565b5060e0840151518514612894565b5060c084015151851461288e565b5060a0840151518514612888565b1561314757565b60405162461bcd60e51b815260206004820152605260248201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360448201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560648201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608482015260a490fd5b613201337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614613140565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561032f576040518092630130fc2760e51b8252604482013060048401526040602484015281518091526064830190602060648260051b8601019301915f905b82821061328f575050505091815f81819503925af180156109a0576122895750565b919360019193955060206132c581926063198b82030186526040838a5163ffffffff81511684520151918185820152019061231b565b960192019201869493919261326d565b613309337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614613140565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561032f575f928392604051948580948193639926ee7d60e01b835260018060a01b0316600483015260406024830152604061337e82516060604486015260a4850190612357565b91602081015160648501520151608483015203925af180156109a0576122895750565b602080825282516001600160a01b039081168284015281840151166040808401919091529092015160608083015280516080830181905260a09092019201905f5b8181106133ef5750505090565b825163ffffffff168452602093840193909201916001016133e2565b1561341257565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156109a05760ff915f9161367c575b50168015613672577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b80831061362e57506135099150611d58565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156109a05760ff915f91613610575b501681101561360957604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa9182156109a0575f926135e9575b50905f915b8183106135835750505060010161350e565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156109a0576120788b612069836120636119406001986135e0985f916120865750516001600160a01b031690565b95019190613571565b61360291925060203d8111611abd57611aaf8183610298565b905f61356c565b5092505050565b613628915060203d811161213f576121318183610298565b5f613534565b604051633ca5a5f560e01b815260ff84166004820152909190602081602481885afa80156109a057600192613669925f92611f775750611cde565b920191906134f7565b505061076d611d3d565b613695915060203d60201161213f576121318183610298565b5f6134bf565b9190811015611b855760051b81013590609e198136030182121561032f570190565b3561076d8161053d565b9081602091031261032f575161076d81610847565b916020908281520191905f5b8181106136f55750505090565b90919260408060019286356137098161053d565b848060a01b031681526001600160601b03602088013561372881611da8565b1660208201520194019291016136e8565b9180602084016020855252604083019060408160051b85010193835f91609e1982360301905b848410613770575050505050505090565b90919293949596603f1982820301875287358381121561032f578401908135601e198336030181121561032f57820191602083359301906001600160401b03841161032f578360061b3603821361032f5761383983608061382e816137e46020989760019a60a08b9a5260a08701916136dc565b956138026137f389830161054e565b6001600160a01b0316868a0152565b6040810135604086015261382861381b6060830161061b565b63ffffffff166060870152565b0161061b565b63ffffffff16910152565b9901970195940192919061375f565b604051906138558261025d565b5f6020838281520152565b604051906101806138718184610298565b368337565b60405190613885602083610298565b6020368337565b9190604090606061389b613848565b94859260208551926138ad8585610298565b8436853780518452015160208301528482015260076107cf195a01fa156138d057565bfe5b6020929160806040926138e3613848565b958693818651936138f48686610298565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa80156138d0571561392557565b60405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606490fd5b6040516139668161025d565b60409081516139758382610298565b823682378152602082519161398a8484610298565b833684370152805161399c8282610298565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed60208201528151906139f28383610298565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152613a4783519384610298565b8252602082015290565b5f51602061441b5f395f51905f5290613a68613848565b505f919006602060c0835b613b68575f935f51602061441b5f395f51905f5260038186818180090908604051613a9e8582610298565b84368237848185604051613ab28282610298565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f51602061441b5f395f51905f5260a082015260056107cf195a01fa80156138d057613b1c906143ce565b5191613b68575f51602061441b5f395f51905f5282800914613b5357505f51602061441b5f395f51905f5260015f94089293613a73565b92935050613b5f6102e9565b92835282015290565b611b8a565b613b75613848565b50604051613b828161025d565b600181526002602082015290565b9060068202918083046006149015171561053857565b90600c811015611b855760051b0190565b93929091613bc560406102f8565b9485526020850152613bd760406102f8565b9182526020820152613be7613860565b925f5b60028110613c1457505050602061018092613c03613876565b93849160086201d4c0fa9151151590565b80613c20600192613b90565b613c2a8285611b74565b5151613c368289613ba6565b526020613c438386611b74565b510151613c58613c5283611c98565b89613ba6565b52613c638286611b74565b515151613c72613c5283611ca6565b52613c88613c808387611b74565b515160200190565b51613c95613c5283611cb4565b526020613ca28387611b74565b51015151613cb2613c5283611cc2565b52613cde613cd8613cd16020613cc8868a611b74565b51015160200190565b5192611cd0565b88613ba6565b5201613bea565b6033546001600160a01b03163303613cf957565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606654604080516001600160a01b038084168252841660208201529192917fe0d49a54274423183dadecbdf239eaac6e06ba88320b26fe8cc5ec9d050a63959190a16001600160a01b03166001600160a01b03199190911617606655565b61ffff613da782614008565b16613db181610701565b90613dbf6040519283610298565b808252613dce601f1991610701565b013660208301375f5f5b8251821080613e2e575b15613e27576001811b8416613e00575b613dfb90611e07565b613dd8565b906001613dfb9160ff60f81b8460f81b165f1a613e1d8287611d97565b5301919050613df2565b5050905090565b506101008110613de2565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303613f0757565b60405162461bcd60e51b815260206004820152604660248201527f536572766963654d616e61676572426173652e6f6e6c795374616b655265676960448201527f737472793a2063616c6c6572206973206e6f7420746865207374616b6520726560648201526567697374727960d01b608482015260a490fd5b906001613f8f60ff936142e7565b928392161b1115613f9d5790565b60405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608490fd5b805f915b614014575090565b5f1981018181116105385761ffff9116911661ffff811461053857600101908061400c565b90614042613848565b5061ffff8116906102008210156140e757600182146140e2576140636102e9565b5f81525f602082015292906001905f925b61ffff831685101561408857505050505090565b600161ffff831660ff86161c8116146140c2575b60016140b86140ad8360ff946138d2565b9460011b61fffe1690565b9401169291614074565b9460016140b86140ad6140d78960ff956138d2565b98935050505061409c565b505090565b60405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606490fd5b614127613848565b50805190811580614198575b15614154575050604051614148604082610298565b5f81525f602082015290565b60205f51602061441b5f395f51905f52910151065f51602061441b5f395f51905f52035f51602061441b5f395f51905f5281116105385760405191613a478361025d565b50602081015115614133565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b91909160ff5f5460081c161561420c576102c8926105fb6104c4926141a4565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b1561426c57565b60405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a490fd5b90610100825111614357578151156143525761431561430b611f27611f1985611d8a565b60ff600191161b90565b6001905b835182101561434d5760019061433861430b611f27611f198689611d97565b90614344818311614265565b17910190614319565b925050565b5f9150565b60a460405162461bcd60e51b815260206004820152604460248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b6084820152fd5b156143d557565b60405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606490fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a264697066735822122015b49915606efbd5fd86c23517e49ea1166cc7c4b393730e7d86081f8015756964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\xA0\x80`@R4a\x02\xB0W`\x80\x81aH\xCD\x808\x03\x80\x91a\0 \x82\x85a\x03\xE6V[\x839\x81\x01\x03\x12a\x02\xB0W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x82\x82\x03a\x02\xB0W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xB0W`@\x82\x01Q\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x83\x03a\x02\xB0W``\x01Q\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x83\x03a\x02\xB0W`@Qch0H5`\xE0\x1B\x81R\x91` \x83`\x04\x81\x89Z\xFA\x92\x83\x15a\x02\xBCW_\x93a\x03\xC5W[P`\x80R`\xA0R\x82`\xC0R`\xE0Ra\x01\0R_T`\xFF\x81`\x08\x1C\x16a\x03pW`\xFF\x80\x82\x16\x10a\x036W[Pa\x01 R`@Qch0H5`\xE0\x1B\x81R` \x81`\x04\x81\x85Z\xFA\x91\x82\x15a\x02\xBCW`\x04\x92` \x92_\x91a\x03\tW[Pa\x01@R`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x92\x83\x91\x82\x90Z\xFA\x90\x81\x15a\x02\xBCW_\x91a\x02\xC7W[Pa\x01`Ra\x01@Q`@Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x02\xBCW_\x91a\x02vW[Pa\x01\x80R`@QaD\x90\x90\x81a\x04=\x829`\x80Q\x81\x81\x81a\n_\x01R\x81\x81a\x12\x16\x01R\x81\x81a\x12\xBA\x01Ra3\x0B\x01R`\xA0Q\x81a\x18\xC5\x01R`\xC0Q\x81\x81\x81a\x11\xE6\x01R\x81\x81a\x13\xE9\x01R\x81\x81a\x17\x0B\x01R\x81\x81a\x1E4\x01R\x81\x81a1\xD3\x01R\x81\x81a2\xDB\x01Ra4}\x01R`\xE0Q\x81\x81\x81a\x1E\xD0\x01R\x81\x81a4\xC9\x01Ra>\xD7\x01Ra\x01\0Q\x81\x81\x81a\x0B\"\x01R\x81\x81a\x14\x19\x01R\x81\x81a\x15\xCA\x01R\x81\x81a\x16\x11\x01R\x81\x81a\x17;\x01R\x81\x81a!\xA6\x01Ra2\x03\x01Ra\x01 Q\x81\x81\x81a\x08\x83\x01R\x81\x81a\n\xA3\x01R\x81\x81a)=\x01R\x81\x81a*0\x01Ra/\xFB\x01Ra\x01@Q\x81\x81\x81a\n\x1B\x01R\x81\x81a-N\x01Ra.\xAC\x01Ra\x01`Q\x81\x81\x81a\t\xBA\x01Ra,\x8A\x01Ra\x01\x80Q\x81\x81\x81a\x16v\x01Ra+C\x01R\xF3[\x90P` \x81=` \x11a\x02\xB4W[\x81a\x02\x91` \x93\x83a\x03\xE6V[\x81\x01\x03\x12a\x02\xB0WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xB0W_a\x01]V[_\x80\xFD[=\x91Pa\x02\x84V[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x03\x01W[\x81a\x02\xE2` \x93\x83a\x03\xE6V[\x81\x01\x03\x12a\x02\xB0WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xB0W_a\x01#V[=\x91Pa\x02\xD5V[a\x03)\x91P\x83=\x85\x11a\x03/W[a\x03!\x81\x83a\x03\xE6V[\x81\x01\x90a\x04\x1DV[_a\0\xFCV[P=a\x03\x17V[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xCDV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xDF\x91\x93P` =` \x11a\x03/Wa\x03!\x81\x83a\x03\xE6V[\x91_a\0\xA3V[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x04\tW`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90\x81` \x91\x03\x12a\x02\xB0WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xB0W\x90V\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x17\x1F\x1D[\x14a\x02DW\x80c&\xF0\x17\xE2\x14a\x02?W\x80c3\xCF\xB7\xB7\x14a\x02:W\x80c;\xC2\x8C\x8C\x14a\x025W\x80c=\x07\x14\"\x14a\x020W\x80cAl~^\x14a\x02+W\x80c]\xF4YF\x14a\x02&W\x80cg\x94\x0C\x89\x14a\x02!W\x80ch0H5\x14a\x02\x1CW\x80ck:\xA7.\x14a\x02\x17W\x80cm\x14\xA9\x87\x14a\x02\x12W\x80cn\xCB\xCC\xFE\x14a\x02\rW\x80cn\xFBF6\x14a\x02\x08W\x80cqP\x18\xA6\x14a\x02\x03W\x80c\x84}cO\x14a\x01\xFEW\x80c\x89\x99\x81\x7F\x14a\x01\xF9W\x80c\x8Dh4\x9A\x14a\x01\xF4W\x80c\x8D\xA5\xCB[\x14a\x01\xEFW\x80c\x99&\xEE}\x14a\x01\xEAW\x80c\xA3d\xF4\xDA\x14a\x01\xE5W\x80c\xA9\x8F\xB3U\x14a\x01\xE0W\x80c\xB14Bq\x14a\x01\xDBW\x80c\xB9\x8D\t\x08\x14a\x01\xD6W\x80c\xC1\xA8\xE2\xC5\x14a\x01\xD1W\x80c\xC4\xD6m\xE8\x14a\x01\xCCW\x80c\xCA\x8A\xA7\xC7\x14a\x01\xC7W\x80c\xCE{^K\x14a\x01\xC2W\x80c\xDF\\\xF7#\x14a\x01\xBDW\x80c\xE4o\x18\x16\x14a\x01\xB8W\x80c\xE4\x81\xAF\x9D\x14a\x01\xB3W\x80c\xF2_\x16\x10\x14a\x01\xAEW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xA9W\x80c\xFC)\x9D\xEE\x14a\x01\xA4W\x80c\xFC\xD1\xC3u\x14a\x01\x9FWc\xFC\xE3l}\x14a\x01\x9AW_\x80\xFD[a\x18~V[a\x18aV[a\x189V[a\x17\xA8V[a\x16\xE8V[a\x16\xCDV[a\x16\xA5V[a\x16aV[a\x15\xF9V[a\x15\xB5V[a\x14\xBBV[a\x13\xA2V[a\x13PV[a\x13(V[a\x12\x7FV[a\x11\xC2V[a\x11&V[a\x10\xFEV[a\x10\xDCV[a\x10QV[a\x0FeV[a\x0F\nV[a\x0EuV[a\x0B\nV[a\n\x8EV[a\nJV[a\n\x06V[a\t\xE9V[a\t\xA5V[a\x08QV[a\x07pV[a\x05\xD3V[a\x05\x9BV[a\x04\x81V[a\x04\x0FV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02xW`@RV[a\x02IV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02xW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02xW`@RV[`@Q\x90a\x02\xC8`\xA0\x83a\x02\x98V[V[`@Q\x90a\x02\xC8a\x01\0\x83a\x02\x98V[`@Q\x90a\x02\xC8``\x83a\x02\x98V[`@Q\x90a\x02\xC8`@\x83a\x02\x98V[\x90a\x02\xC8`@Q\x92\x83a\x02\x98V[`@\x90`\xE3\x19\x01\x12a\x03/W`@Q\x90a\x03\x1F\x82a\x02]V[`\xE45\x82Ra\x01\x045` \x83\x01RV[_\x80\xFD[\x91\x90\x82`@\x91\x03\x12a\x03/W`@Qa\x03K\x81a\x02]V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W`@Q\x91a\x03v`@\x84a\x02\x98V[\x82\x90`@\x81\x01\x92\x83\x11a\x03/W\x90[\x82\x82\x10a\x03\x92WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x03\x85V[\x90`\x80`c\x19\x83\x01\x12a\x03/W`@Qa\x03\xBB\x81a\x02]V[` a\x03\xD6\x82\x94a\x03\xCD\x81`da\x03[V[\x84R`\xA4a\x03[V[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03/W` a\x03\xD6`@Q\x92a\x03\xF9\x84a\x02]V[`@\x84\x96a\x04\x07\x83\x82a\x03[V[\x86R\x01a\x03[V[4a\x03/Wa\x01 6`\x03\x19\x01\x12a\x03/W`\x045`@6`#\x19\x01\x12a\x03/Wa\x04g`@\x91\x82Qa\x04A\x81a\x02]V[`$5\x81R`D5` \x82\x01Ra\x04W6a\x03\xA2V[\x90a\x04a6a\x03\x06V[\x92a\x1B\x9EV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x03/WV[4a\x03/W_6`\x03\x19\x01\x12a\x03/Wa\x04\x99a<\xE5V[`hTb\t:\x80\x81\x01\x80\x91\x11a\x058WB\x10a\x04\xDCW`gTa\x04\xC9\x90`\x01`\x01`\xA0\x1B\x03\x16a==V[a==V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FServiceManager: Slasher proposal`D\x82\x01Rm\x08\x19\x19[\x18^H\x1B\x9B\xDD\x08\x1BY]`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a\x1C\x84V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03/WV[5\x90a\x02\xC8\x82a\x05=V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x05|WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x05oV[4a\x03/W` 6`\x03\x19\x01\x12a\x03/Wa\x05\xCFa\x05\xC3`\x045a\x05\xBE\x81a\x05=V[a\x1E\x15V[`@Q\x91\x82\x91\x82a\x05YV[\x03\x90\xF3[4a\x03/W` 6`\x03\x19\x01\x12a\x03/Wa\x04\xDA`\x045a\x05\xF3\x81a\x05=V[a\x05\xFBa<\xE5V[a>9V[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03/WV[`D5\x90a\x02\xC8\x82a\x06\0V[5\x90a\x02\xC8\x82a\x06\0V[`\x01`\x01`@\x1B\x03\x81\x11a\x02xW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W\x815a\x06T\x81a\x06&V[\x92a\x06b`@Q\x94\x85a\x02\x98V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03/W` \x01\x90[\x82\x82\x10a\x06\x8AWPPP\x90V[` \x80\x91\x835a\x06\x99\x81a\x05=V[\x81R\x01\x91\x01\x90a\x06}V[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W\x815a\x06\xBB\x81a\x06&V[\x92a\x06\xC9`@Q\x94\x85a\x02\x98V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03/W` \x01\x90[\x82\x82\x10a\x06\xF1WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x06\xE4V[`\x01`\x01`@\x1B\x03\x81\x11a\x02xW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07(\x82a\x07\x01V[\x91a\x076`@Q\x93\x84a\x02\x98V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03/W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W\x81` a\x07m\x935\x91\x01a\x07\x1CV[\x90V[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03/W`\xA0`\x03\x19\x826\x03\x01\x12a\x03/Wa\x07\xA8a\x02\xB9V[a\x07\xB4\x82`\x04\x01a\x05NV[\x81Ra\x07\xC2`$\x83\x01a\x06\x1BV[` \x82\x01R`D\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x07\xEA\x90`\x046\x91\x85\x01\x01a\x06=V[`@\x82\x01R`d\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x08\x12\x90`\x046\x91\x85\x01\x01a\x06\xA4V[``\x82\x01R`\x84\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x04\xDA\x92`\x04a\x08=\x926\x92\x01\x01a\x07RV[`\x80\x82\x01Ra!\x92V[\x80\x15\x15\x03a\x03/WV[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045a\x08n\x81a\x08GV[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\xA0W_\x91a\teW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x08\xD4Wa\x04\xDA\x90a>\x97V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[\x90P` \x81=` \x11a\t\x98W[\x81a\t\x80` \x93\x83a\x02\x98V[\x81\x01\x03\x12a\x03/WQa\t\x92\x81a\x05=V[_a\x08\xBBV[=\x91Pa\tsV[a\x1C\xFAV[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W` `@Qb\t:\x80\x81R\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90`@`\x03\x19\x83\x01\x12a\x03/W`\x045a\n\xEB\x81a\x06\0V[\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03/Wa\x07m\x91`\x04\x01a\x06=V[4a\x03/Wa\x0B\x186a\n\xD2V[a\x0B a>\xD5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x03/W`@Qc\x02\x87\xF7Q`\xE5\x1B\x81R\x91_\x91\x83\x91a\x0Br\x910`\x04\x85\x01a#{V[\x93\x81\x83\x81\x81\x97\x03\x92Z\xF1\x80\x15a\t\xA0Wa\x0B\x8AWP\x80\xF3[a\x04\xDA\x91P_\x90a\x02\x98V[\x92\x91\x90a\x0B\xA2\x81a\x06&V[\x93a\x0B\xB0`@Q\x95\x86a\x02\x98V[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x03/W\x90[\x82\x82\x10a\x0B\xD2WPPPV[` \x80\x91\x835a\x0B\xE1\x81a\x06\0V[\x81R\x01\x91\x01\x90a\x0B\xC6V[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W\x81` a\x07m\x935\x91\x01a\x0B\x96V[\x81`\x1F\x82\x01\x12\x15a\x03/W\x805a\x0C\x1D\x81a\x06&V[\x92a\x0C+`@Q\x94\x85a\x02\x98V[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03/W` \x01\x91[\x83\x83\x10a\x0CUWPPPP\x90V[` `@\x91a\x0Cd\x84\x86a\x033V[\x81R\x01\x92\x01\x91a\x0CGV[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W\x815a\x0C\x86\x81a\x06&V[\x92a\x0C\x94`@Q\x94\x85a\x02\x98V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03/W` \x82\x01\x90[\x83\x82\x10a\x0C\xC0WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03/W` \x91a\x0C\xE2\x87\x84\x80\x94\x88\x01\x01a\x0B\xECV[\x81R\x01\x91\x01\x90a\x0C\xB1V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03/Wa\r\x03a\x02\xCAV[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x81a\r \x91\x84\x01a\x0B\xECV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x81a\rA\x91\x84\x01a\x0C\x07V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x81a\re\x91\x84\x01a\x0C\x07V[`@\x85\x01Ra\rw\x81``\x84\x01a\x03\xDBV[``\x85\x01Ra\r\x89\x81`\xE0\x84\x01a\x033V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x81a\r\xAE\x91\x84\x01a\x0B\xECV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x81a\r\xD3\x91\x84\x01a\x0B\xECV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\r\xF6\x92\x01a\x0CoV[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0E\x1AWPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0E\rV[\x92\x91\x90a\x0Ep` \x91`@\x86R\x82a\x0E\\\x82Q`@\x80\x8A\x01R`\x80\x89\x01\x90a\r\xFDV[\x91\x01Q\x86\x82\x03`?\x19\x01``\x88\x01Ra\r\xFDV[\x93\x01RV[4a\x03/W`\x806`\x03\x19\x01\x12a\x03/W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03/W6`#\x82\x01\x12\x15a\x03/W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W6`$\x82\x84\x01\x01\x11a\x03/Wa\x0E\xCCa\x06\x0EV[\x90`d5\x93`\x01`\x01`@\x1B\x03\x85\x11a\x03/W`$a\x0E\xF2a\x0E\xFA\x966\x90`\x04\x01a\x0C\xEDV[\x94\x01\x90a(`V[\x90a\x05\xCF`@Q\x92\x83\x92\x83a\x0E9V[4a\x03/W_6`\x03\x19\x01\x12a\x03/Wa\x0F\"a<\xE5V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03/W6`#\x82\x01\x12\x15a\x03/W\x80`\x04\x015\x90a\x0F\xA1\x82a\x06&V[\x90a\x0F\xAF`@Q\x92\x83a\x02\x98V[\x82\x82R`$` \x83\x01\x93`\x05\x1B\x82\x01\x01\x906\x82\x11a\x03/W`$\x81\x01\x93[\x82\x85\x10a\x0F\xDDWa\x04\xDA\x84a1\xCDV[\x845`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x82\x01`@`#\x19\x826\x03\x01\x12a\x03/W`@Q\x91a\x10\n\x83a\x02]V[`$\x82\x015a\x10\x18\x81a\x06\0V[\x83R`D\x82\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03/Wa\x10B` \x94\x93`$\x86\x956\x92\x01\x01a\x06=V[\x83\x82\x01R\x81R\x01\x94\x01\x93a\x0F\xCDV[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W\x7F/\x8A\xFC\x8Ax\xFD\x95\x8F3\x01\xC0#:\xA3&\xB9\xC4\xB9\xA2\x88Jt\x83\"}k\x05U\xAA\xA0:\xDBa\x10\xD7`\x045a\x10\x92\x81a\x05=V[a\x10\x9Aa<\xE5V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90UB`h\x81\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1\0[4a\x03/W_6`\x03\x19\x01\x12a\x03/W` `\xFF`iT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03/W`@6`\x03\x19\x01\x12a\x03/W`\x045a\x11C\x81a\x05=V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03/W```\x03\x19\x836\x03\x01\x12a\x03/W`@Q\x90a\x11p\x82a\x02}V[\x82`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x83\x016`#\x82\x01\x12\x15a\x03/Wa\x04\xDA\x93a\x11\xAA`D\x926\x90`$`\x04\x82\x015\x91\x01a\x07\x1CV[\x84R`$\x81\x015` \x85\x01R\x015`@\x83\x01Ra2\xD5V[4a\x03/W_` 6`\x03\x19\x01\x12a\x03/W`\x045a\x11\xE0\x81a\x05=V[a\x12\x143\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a1@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x03/W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xA0Wa\x0B\x8AWP\x80\xF3[4a\x03/W_` 6`\x03\x19\x01\x12a\x03/W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x12\xB0\x906\x90`\x04\x01a\x07RV[a\x12\xB8a<\xE5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03/W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x13\x16\x90`$\x83\x01\x90a#WV[\x03\x92Z\xF1\x80\x15a\t\xA0Wa\x0B\x8AWP\x80\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`fT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x03/W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03/W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x03/WV[4a\x03/W`@6`\x03\x19\x01\x12a\x03/W`\x045a\x13\xBF\x81a\x05=V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x13\xE1a\x14i\x916\x90`\x04\x01a\x13rV[\x91\x90a\x14\x173\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a1@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92a\x14\\a\x14Ma\x02\xDAV[`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x86RV[0` \x86\x01R6\x91a\x0B\x96V[`@\x83\x01R\x80;\x15a\x03/W`@Qcn4\x92\xB5`\xE0\x1B\x81R\x90_\x90\x82\x90\x81\x83\x81a\x14\x97\x88`\x04\x83\x01a3\xA1V[\x03\x92Z\xF1\x80\x15a\t\xA0Wa\x14\xA7W\0[\x80a\x14\xB5_a\x04\xDA\x93a\x02\x98V[\x80a\x04wV[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045a\x14\xD8\x81a\x05=V[a\x15)_T\x91a\x15\x0Ca\x14\xF6a\x14\xF2\x85`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x94\x81\x95a\x15\xA7W[\x81\x15a\x15\x87W[Pa4\x0BV[\x82a\x15\x1D`\x01`\xFF\x19_T\x16\x17_UV[a\x15pW[\x80\x80aA\xECV[a\x15/W\0[a\x15=a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x10\xD7V[a\x15\x82a\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x15\"V[0;\x15\x91P\x81a\x15\x99W[P_a\x15\x06V[`\xFF\x16`\x01\x14\x90P_a\x15\x92V[`\x01`\xFF\x82\x16\x10\x91Pa\x14\xFFV[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03/Wa\x16\x076a\n\xD2V[a\x16\x0Fa>\xD5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x03/W`@Qc\xB6k\xD9\x89`\xE0\x1B\x81R\x91_\x91\x83\x91a\x0Br\x910`\x04\x85\x01a#{V[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/Wa\x05\xCFa\x05\xC3a4nV[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045a\x17\x05\x81a\x05=V[a\x1793\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a1@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x03/W`@Qc4\xF6[\xFD`\xE2\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R\x90_\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xA0Wa\x14\xA7W\0[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045a\x17\xC5\x81a\x05=V[a\x17\xCDa<\xE5V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x17\xE5Wa\x04\xDA\x90aA\xA4V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W` `hT`@Q\x90\x81R\xF3[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x18\xAE\x906\x90`\x04\x01a\x13rV[`eT\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1A\xE0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a\x19\"WP\x82;\x15a\x03/Wa\x14\x97\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xFC\xE3l}`\xE0\x1B\x84R`\x04\x84\x01a79V[_` a\x19\x89a\x19La\x19@\x83a\x19:\x87\x89\x8Ba6\x9BV[\x01a6\xBDV[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@a\x19Y\x86\x88\x8Aa6\x9BV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x91\x015`D\x82\x01R\x93\x84\x92\x83\x91\x90\x82\x90`d\x82\x01\x90V[\x03\x92Z\xF1\x80\x15a\t\xA0Wa\x1A\xC4W[Pa\x19\xACa\x19@` a\x19:\x84\x86\x88a6\x9BV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x16`$\x82\x01R\x91\x90` \x90\x83\x90`D\x90\x82\x90Z\xFA\x80\x15a\t\xA0Wa\x1AO` \x91_\x94\x85\x91a\x1A\x97W[Pa\x1A\x1Ca\x1A\x07a\x19@\x85a\x19:\x88\x8B\x8Da6\x9BV[\x91`@a\x1A\x15\x87\x8A\x8Ca6\x9BV[\x015a\x1C\xDEV[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x94\x85\x92\x83\x91\x90\x82\x90`D\x82\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\t\xA0W`\x01\x92a\x1AiW[P\x01a\x18\xF0V[a\x1A\x89\x90` =\x81\x11a\x1A\x90W[a\x1A\x81\x81\x83a\x02\x98V[\x81\x01\x90a6\xC7V[P_a\x1AbV[P=a\x1AwV[a\x1A\xB7\x91P\x83=\x81\x11a\x1A\xBDW[a\x1A\xAF\x81\x83a\x02\x98V[\x81\x01\x90a\x1C\xEBV[_a\x19\xF1V[P=a\x1A\xA5V[a\x1A\xDB\x90` =\x81\x11a\x1A\x90Wa\x1A\x81\x81\x83a\x02\x98V[a\x19\x98V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FServiceManagerBase.onlyRewardsIn`D\x82\x01R\x7Fitiator: caller is not the rewar`d\x82\x01Rk29\x904\xB74\xBA4\xB0\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x1B\x85W`\x05\x1B\x01\x90V[a\x1B`V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x1Cza\x1CWa\x1C\x80\x95a\x1CQa\x1CJ\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x1C!\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x98V[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96a8\x8CV[\x90a8\xD2V[\x92a\x1CQa\x1Cla\x1Cfa9ZV[\x94a:QV[\x91a\x1Cua;mV[a8\x8CV[\x91a;\xB7V[\x90\x91V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x058WV[\x90`\x02\x82\x01\x80\x92\x11a\x058WV[\x90`\x03\x82\x01\x80\x92\x11a\x058WV[\x90`\x04\x82\x01\x80\x92\x11a\x058WV[\x90`\x05\x82\x01\x80\x92\x11a\x058WV[\x91\x90\x82\x01\x80\x92\x11a\x058WV[\x90\x81` \x91\x03\x12a\x03/WQ\x90V[`@Q=_\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x03/WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03/W\x90V[\x90\x81` \x91\x03\x12a\x03/WQ`\xFF\x81\x16\x81\x03a\x03/W\x90V[`@Q\x90a\x1DL` \x83a\x02\x98V[_\x80\x83R6` \x84\x017V[\x90a\x1Db\x82a\x06&V[a\x1Do`@Q\x91\x82a\x02\x98V[\x82\x81R\x80\x92a\x1D\x80`\x1F\x19\x91a\x06&V[\x01\x90` 6\x91\x017V[\x80Q\x15a\x1B\x85W` \x01\x90V[\x90\x81Q\x81\x10\x15a\x1B\x85W\x01` \x01\x90V[`\x01`\x01``\x1B\x03\x81\x16\x03a\x03/WV[\x90\x81`@\x91\x03\x12a\x03/W` `@Q\x91a\x1D\xD3\x83a\x02]V[\x80Qa\x1D\xDE\x81a\x05=V[\x83R\x01Qa\x1D\xEB\x81a\x1D\xA8V[` \x82\x01R\x90V[\x80Q\x82\x10\x15a\x1B\x85W` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x058W`\x01\x01\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\t\xA0Wa\x1E\x95\x91` \x91_\x91a!uW[P`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x90\x81\x15a\t\xA0W_\x91a!FW[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81\x15\x90\x81\x15a \xE3W[Pa \xDAWa\x1E\xCB\x90a=\x9BV[_\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x1F\x97Wa\x1FP` a\x1F-a\x1F'a\x1F\x19\x89\x87a\x1D\x97V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x87Z\xFA\x80\x15a\t\xA0W`\x01\x92a\x1Fo\x92_\x92a\x1FwW[Pa\x1C\xDEV[\x94\x01\x93a\x1E\xFBV[a\x1F\x90\x91\x92P` =\x81\x11a\x1A\xBDWa\x1A\xAF\x81\x83a\x02\x98V[\x90_a\x1FiV[a\x1F\xA2\x91\x94Pa\x1DXV[\x92_\x90_[\x81Q\x81\x10\x15a \xD4Wa\x1F\xC0a\x1F'a\x1F\x19\x83\x85a\x1D\x97V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x90` \x82`$\x81\x89Z\xFA\x91\x82\x15a\t\xA0W_\x92a \xB4W[P\x90_\x91[\x81\x83\x10a \x05WPPP`\x01\x01a\x1F\xA7V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\t\xA0Wa x\x8Ba i\x83a ca\x19@`\x01\x98a }\x98_\x91a \x86W[PQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x1D\xF3V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x1E\x07V[\x95\x01\x91\x90a\x1F\xF3V[a \xA7\x91P`@=\x81\x11a \xADW[a \x9F\x81\x83a\x02\x98V[\x81\x01\x90a\x1D\xB9V[_a UV[P=a \x95V[a \xCD\x91\x92P` =\x81\x11a\x1A\xBDWa\x1A\xAF\x81\x83a\x02\x98V[\x90_a\x1F\xEEV[PPPPV[Pa\x07ma\x1D=V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\t\xA0W`\xFF\x91_\x91a!\x17W[P\x16\x15_a\x1E\xBDV[a!9\x91P` =` \x11a!?W[a!1\x81\x83a\x02\x98V[\x81\x01\x90a\x1D$V[_a!\x0EV[P=a!'V[a!h\x91P` =` \x11a!nW[a!`\x81\x83a\x02\x98V[\x81\x01\x90a\x1D\x05V[_a\x1E\xA8V[P=a!VV[a!\x8C\x91P\x82=\x84\x11a\x1A\xBDWa\x1A\xAF\x81\x83a\x02\x98V[_a\x1EuV[`fT`\x01`\x01`\xA0\x1B\x03\x163\x03a\"\xB0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03/W`@\x80Qc65 W`\xE0\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x82\x90R\x83Q`\x01`\x01`\xA0\x1B\x03\x16`D\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`d\x82\x01R\x90\x83\x01Q`\xA0`\x84\x83\x01R\x90\x92\x83\x91a\"-\x90`\xE4\x84\x01\x90a#\x1BV[\x90``\x81\x01Q\x91`C\x19\x84\x82\x03\x01`\xA4\x85\x01R` \x80\x84Q\x92\x83\x81R\x01\x93\x01\x90_[\x81\x81\x10a\"\x97WPPP_\x83a\"x\x82\x96\x94`\x80\x83\x95\x01Q`C\x19\x84\x83\x03\x01`\xC4\x85\x01Ra#WV[\x03\x92Z\xF1\x80\x15a\t\xA0Wa\"\x89WPV[\x80a\x14\xB5_a\x02\xC8\x93a\x02\x98V[\x82Q\x85R\x87\x95P` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a\"OV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FServiceManagerBase.onlySlasher: `D\x82\x01R\x7Fcaller is not the slasher\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a#8WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a#+V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[c\xFF\xFF\xFF\xFFa\x07m\x94\x93``\x93`\x01\x80`\xA0\x1B\x03\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a#\x1BV[`@Q\x90a#\xB2\x82a\x02]V[``` \x83\x82\x81R\x01RV[\x15a#\xC5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a$$WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a$\x8DWV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R\xFD[\x15a$\xF8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x90\xFD[_\x19\x81\x01\x91\x90\x82\x11a\x058WV[\x15a%eWV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R\xFD[\x90\x81` \x91\x03\x12a\x03/WQa\x07m\x81a\x06\0V[\x90\x82\x10\x15a\x1B\x85W\x01\x90V[\x15a%\xE4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90\x81` \x91\x03\x12a\x03/WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03/W\x90V[\x15a&\x99WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90\x81` \x91\x03\x12a\x03/WQa\x07m\x81a\x1D\xA8V[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a\x058WV[\x15a']WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a'\xC8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a(JWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a(=V[\x94\x93\x92\x90\x91\x93a(na#\xA5V[Pa(z\x85\x15\x15a#\xBEV[`@\x84\x01QQ\x85\x14\x80a12W[\x80a1$W[\x80a1\x16W[a(\x9D\x90a$\x1DV[a(\xAF` \x85\x01QQ\x85QQ\x14a$\x86V[a(\xC6c\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a$\xF1V[a(\xCEa\x02\xE9V[_\x81R_` \x82\x01R\x92a(\xE0a#\xA5V[a(\xE9\x87a\x1DXV[` \x82\x01Ra(\xF7\x87a\x1DXV[\x81Ra)\x01a#\xA5V[\x92a)\x10` \x88\x01QQa\x1DXV[\x84Ra) ` \x88\x01QQa\x1DXV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\xA0Wa)\x89\x91_\x91a0\xF7W[Pa)\x846\x8B\x87a\x07\x1CV[a?\x81V[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a+\x05W` \x88a)\xFAa)\xF0\x8Ca)\xE8\x8F\x96\x86\x8Ea)\xCDa)\xBA\x86\x80\x95a\x1D\xF3V[Q\x80Q_R` \x01Q` R`@_ \x90V[a)\xDA\x84\x84\x84\x01Qa\x1D\xF3V[R\x82a*\xD2W[\x01Qa\x1D\xF3V[Q\x95Qa\x1D\xF3V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\t\xA0Wa\x1CQ\x8Aa*\xA7\x8Fa*\xA0\x8F\x84` \x8F\x92a*\x97\x93a*\x8F\x84`\x01\x9Ea*\xAD\x9E_\x91a*\xB5W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\x1D\xF3V[R\x01Qa\x1D\xF3V[Q\x93\x8DQa\x1D\xF3V[Q\x16a@\x08V[\x90a@9V[\x97\x01\x96a)\x8DV[a*\xCC\x91P\x86=\x81\x11a!nWa!`\x81\x83a\x02\x98V[_a*\x80V[a+\0a*\xE2\x84\x84\x84\x01Qa\x1D\xF3V[Qa*\xF9\x84\x84\x01Qa*\xF3\x87a%PV[\x90a\x1D\xF3V[Q\x10a%^V[a)\xE1V[P\x90\x95\x97\x94\x96Pa+\x1A\x91\x98\x93\x92\x99PaA\x1FV[\x91a+'`\x97T`\xFF\x16\x90V[\x90\x81\x15a0\xEFW`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\xA0W_\x91a0\xC0W[P\x91\x90[_\x92[\x81\x84\x10a+\xE6WPPPPP\x92a+\xBFa+\xBAa+\xB3a+\xE0\x95\x85a+\xD2\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x1B\x9EV[\x91\x90a'VV[a'\xC1V[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a( V[\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x98V[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da/\xBAW[a)\xF0\x82`\xA0a,Ia\x1F'a,;\x84a,Q\x97a,5a,'a)\xBA\x8F\x9C`@` \x9F\x9E\x01Qa\x1D\xF3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba%\xD1V[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[\x97\x01Qa\x1D\xF3V[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\t\xA0Wa-\x15a)\xF0\x8F\x95\x8F\x90a-\r\x8F\x97\x8F\x96\x84\x8Fa-\x07`\xC0\x96a-\0\x84\x8F` \x9F\x90a)\xE1a,;\x99`@\x93a\x1F'\x9C_\x91a/\x8CW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a&\x92V[Q\x90a8\xD2V[\x9Ca%\xD1V[\x96\x01Qa\x1D\xF3V[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\t\xA0Wa-\xA2\x91\x8C\x8F\x92_\x92a/hW[P` a-\x94\x92\x93\x01Qa\x1D\xF3V[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a-\xCF\x8Ca-\x94\x8Ca-\xC8a-\xBB\x82` \x86\x01Qa\x1D\xF3V[Q`\x01`\x01``\x1B\x03\x16\x90V[\x92Qa\x1D\xF3V[_\x98_[` \x8A\x01QQ\x81\x10\x15a/OW\x8B\x8Da.\x11\x89a.\x04a\x1F'a,;\x86\x8F\x89a-\xFC\x91Qa\x1D\xF3V[Q\x94\x87a%\xD1V[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a. W[PP`\x01\x01a-\xD3V[\x8A\x8Aa.\xA8\x85\x9F\x94\x8F\x96\x86a.b\x8F\x93`\xE0a.Ya)\xF0\x95` a.Qa\x1F'a,;\x83\x9Fa.h\x9C\x89\x91a%\xD1V[\x9A\x01Qa\x1D\xF3V[Q\x9B\x01Qa\x1D\xF3V[Qa\x1D\xF3V[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\xA0W\x8Fa/\x0E\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a/\x19W[Pa ca-\x94\x92\x93Q\x93a/\ta-\xBB\x84\x87a\x1D\xF3V[a'6V[\x01\x9A\x90P\x8B\x8Da.\x16V[a-\x94\x92Pa/Aa c\x91` =\x81\x11a/HW[a/9\x81\x83a\x02\x98V[\x81\x01\x90a'!V[\x92Pa.\xF1V[P=a//V[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a+\x82V[a-\x94\x92Pa/\x85` \x91\x82=\x81\x11a/HWa/9\x81\x83a\x02\x98V[\x92Pa-\x85V[` a/\xAD\x92P=\x81\x11a/\xB3W[a/\xA5\x81\x83a\x02\x98V[\x81\x01\x90a&qV[_a,\xEAV[P=a/\x9BV[a/\xF7\x94Pa/\xD4\x92Pa\x1F'\x91a,;\x91` \x95a%\xD1V[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\xA0W` \x89a,Q\x8F\x93\x8F`\xA0\x8F\x97a\x1F'a,;\x8F\x8F\x90a,5a,'a)\xBA\x8F`@\x8B\x96\x91\x8F\x88\x93a)\xF0\x9Fa0{\x90a0\x81\x93a,I\x9F_\x92a0\x97W[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a\x1C\xDEV[\x11a%\xDDV[PPPPPP\x97PPPPPP\x92\x93PPa+\xFBV[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a0\xB8\x91=\x81\x11a\x1A\xBDWa\x1A\xAF\x81\x83a\x02\x98V[\x92\x91Pa0jV[a0\xE2\x91P` =` \x11a0\xE8W[a0\xDA\x81\x83a\x02\x98V[\x81\x01\x90a%\xBCV[_a+{V[P=a0\xD0V[_\x91\x90a+\x7FV[a1\x10\x91P` =` \x11a!?Wa!1\x81\x83a\x02\x98V[_a)xV[P`\xE0\x84\x01QQ\x85\x14a(\x94V[P`\xC0\x84\x01QQ\x85\x14a(\x8EV[P`\xA0\x84\x01QQ\x85\x14a(\x88V[\x15a1GWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FServiceManagerBase.onlyRegistryC`D\x82\x01R\x7Foordinator: caller is not the re`d\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x84\x82\x01R`\xA4\x90\xFD[a2\x013\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a1@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03/W`@Q\x80\x92c\x010\xFC'`\xE5\x1B\x82R`D\x82\x010`\x04\x84\x01R`@`$\x84\x01R\x81Q\x80\x91R`d\x83\x01\x90` `d\x82`\x05\x1B\x86\x01\x01\x93\x01\x91_\x90[\x82\x82\x10a2\x8FWPPPP\x91\x81_\x81\x81\x95\x03\x92Z\xF1\x80\x15a\t\xA0Wa\"\x89WPV[\x91\x93`\x01\x91\x93\x95P` a2\xC5\x81\x92`c\x19\x8B\x82\x03\x01\x86R`@\x83\x8AQc\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a#\x1BV[\x96\x01\x92\x01\x92\x01\x86\x94\x93\x91\x92a2mV[a3\t3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a1@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x03/W_\x92\x83\x92`@Q\x94\x85\x80\x94\x81\x93c\x99&\xEE}`\xE0\x1B\x83R`\x01\x80`\xA0\x1B\x03\x16`\x04\x83\x01R`@`$\x83\x01R`@a3~\x82Q```D\x86\x01R`\xA4\x85\x01\x90a#WV[\x91` \x81\x01Q`d\x85\x01R\x01Q`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\t\xA0Wa\"\x89WPV[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x84\x01R\x81\x84\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x90\x92\x01Q``\x80\x83\x01R\x80Q`\x80\x83\x01\x81\x90R`\xA0\x90\x92\x01\x92\x01\x90_[\x81\x81\x10a3\xEFWPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a3\xE2V[\x15a4\x12WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\t\xA0W`\xFF\x91_\x91a6|W[P\x16\x80\x15a6rW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a6.WPa5\t\x91Pa\x1DXV[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\t\xA0W`\xFF\x91_\x91a6\x10W[P\x16\x81\x10\x15a6\tW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\t\xA0W_\x92a5\xE9W[P\x90_\x91[\x81\x83\x10a5\x83WPPP`\x01\x01a5\x0EV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\t\xA0Wa x\x8Ba i\x83a ca\x19@`\x01\x98a5\xE0\x98_\x91a \x86WPQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01\x91\x90a5qV[a6\x02\x91\x92P` =\x81\x11a\x1A\xBDWa\x1A\xAF\x81\x83a\x02\x98V[\x90_a5lV[P\x92PPPV[a6(\x91P` =\x81\x11a!?Wa!1\x81\x83a\x02\x98V[_a54V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x88Z\xFA\x80\x15a\t\xA0W`\x01\x92a6i\x92_\x92a\x1FwWPa\x1C\xDEV[\x92\x01\x91\x90a4\xF7V[PPa\x07ma\x1D=V[a6\x95\x91P` =` \x11a!?Wa!1\x81\x83a\x02\x98V[_a4\xBFV[\x91\x90\x81\x10\x15a\x1B\x85W`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x03/W\x01\x90V[5a\x07m\x81a\x05=V[\x90\x81` \x91\x03\x12a\x03/WQa\x07m\x81a\x08GV[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a6\xF5WPPP\x90V[\x90\x91\x92`@\x80`\x01\x92\x865a7\t\x81a\x05=V[\x84\x80`\xA0\x1B\x03\x16\x81R`\x01`\x01``\x1B\x03` \x88\x015a7(\x81a\x1D\xA8V[\x16` \x82\x01R\x01\x94\x01\x92\x91\x01a6\xE8V[\x91\x80` \x84\x01` \x85RR`@\x83\x01\x90`@\x81`\x05\x1B\x85\x01\x01\x93\x83_\x91`\x9E\x19\x826\x03\x01\x90[\x84\x84\x10a7pWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`?\x19\x82\x82\x03\x01\x87R\x875\x83\x81\x12\x15a\x03/W\x84\x01\x90\x815`\x1E\x19\x836\x03\x01\x81\x12\x15a\x03/W\x82\x01\x91` \x835\x93\x01\x90`\x01`\x01`@\x1B\x03\x84\x11a\x03/W\x83`\x06\x1B6\x03\x82\x13a\x03/Wa89\x83`\x80a8.\x81a7\xE4` \x98\x97`\x01\x9A`\xA0\x8B\x9AR`\xA0\x87\x01\x91a6\xDCV[\x95a8\x02a7\xF3\x89\x83\x01a\x05NV[`\x01`\x01`\xA0\x1B\x03\x16\x86\x8A\x01RV[`@\x81\x015`@\x86\x01Ra8(a8\x1B``\x83\x01a\x06\x1BV[c\xFF\xFF\xFF\xFF\x16``\x87\x01RV[\x01a\x06\x1BV[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x99\x01\x97\x01\x95\x94\x01\x92\x91\x90a7_V[`@Q\x90a8U\x82a\x02]V[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80a8q\x81\x84a\x02\x98V[6\x837V[`@Q\x90a8\x85` \x83a\x02\x98V[` 6\x837V[\x91\x90`@\x90``a8\x9Ba8HV[\x94\x85\x92` \x85Q\x92a8\xAD\x85\x85a\x02\x98V[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15a8\xD0WV[\xFE[` \x92\x91`\x80`@\x92a8\xE3a8HV[\x95\x86\x93\x81\x86Q\x93a8\xF4\x86\x86a\x02\x98V[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15a8\xD0W\x15a9%WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x90\xFD[`@Qa9f\x81a\x02]V[`@\x90\x81Qa9u\x83\x82a\x02\x98V[\x826\x827\x81R` \x82Q\x91a9\x8A\x84\x84a\x02\x98V[\x836\x847\x01R\x80Qa9\x9C\x82\x82a\x02\x98V[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90a9\xF2\x83\x83a\x02\x98V[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01Ra:G\x83Q\x93\x84a\x02\x98V[\x82R` \x82\x01R\x90V[_Q` aD\x1B_9_Q\x90_R\x90a:ha8HV[P_\x91\x90\x06` `\xC0\x83[a;hW_\x93_Q` aD\x1B_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa:\x9E\x85\x82a\x02\x98V[\x846\x827\x84\x81\x85`@Qa:\xB2\x82\x82a\x02\x98V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aD\x1B_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a8\xD0Wa;\x1C\x90aC\xCEV[Q\x91a;hW_Q` aD\x1B_9_Q\x90_R\x82\x80\t\x14a;SWP_Q` aD\x1B_9_Q\x90_R`\x01_\x94\x08\x92\x93a:sV[\x92\x93PPa;_a\x02\xE9V[\x92\x83R\x82\x01R\x90V[a\x1B\x8AV[a;ua8HV[P`@Qa;\x82\x81a\x02]V[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a\x058WV[\x90`\x0C\x81\x10\x15a\x1B\x85W`\x05\x1B\x01\x90V[\x93\x92\x90\x91a;\xC5`@a\x02\xF8V[\x94\x85R` \x85\x01Ra;\xD7`@a\x02\xF8V[\x91\x82R` \x82\x01Ra;\xE7a8`V[\x92_[`\x02\x81\x10a<\x14WPPP` a\x01\x80\x92a<\x03a8vV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80a< `\x01\x92a;\x90V[a<*\x82\x85a\x1BtV[QQa<6\x82\x89a;\xA6V[R` a<C\x83\x86a\x1BtV[Q\x01Qa<Xa<R\x83a\x1C\x98V[\x89a;\xA6V[Ra<c\x82\x86a\x1BtV[QQQa<ra<R\x83a\x1C\xA6V[Ra<\x88a<\x80\x83\x87a\x1BtV[QQ` \x01\x90V[Qa<\x95a<R\x83a\x1C\xB4V[R` a<\xA2\x83\x87a\x1BtV[Q\x01QQa<\xB2a<R\x83a\x1C\xC2V[Ra<\xDEa<\xD8a<\xD1` a<\xC8\x86\x8Aa\x1BtV[Q\x01Q` \x01\x90V[Q\x92a\x1C\xD0V[\x88a;\xA6V[R\x01a;\xEAV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a<\xF9WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`fT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE0\xD4\x9AT'D#\x18=\xAD\xEC\xBD\xF29\xEA\xACn\x06\xBA\x882\x0B&\xFE\x8C\xC5\xEC\x9D\x05\nc\x95\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`fUV[a\xFF\xFFa=\xA7\x82a@\x08V[\x16a=\xB1\x81a\x07\x01V[\x90a=\xBF`@Q\x92\x83a\x02\x98V[\x80\x82Ra=\xCE`\x1F\x19\x91a\x07\x01V[\x016` \x83\x017__[\x82Q\x82\x10\x80a>.W[\x15a>'W`\x01\x81\x1B\x84\x16a>\0W[a=\xFB\x90a\x1E\x07V[a=\xD8V[\x90`\x01a=\xFB\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa>\x1D\x82\x87a\x1D\x97V[S\x01\x91\x90Pa=\xF2V[PP\x90P\x90V[Pa\x01\0\x81\x10a=\xE2V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eUV[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a?\x07WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FServiceManagerBase.onlyStakeRegi`D\x82\x01R\x7Fstry: caller is not the stake re`d\x82\x01Registry`\xD0\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90`\x01a?\x8F`\xFF\x93aB\xE7V[\x92\x83\x92\x16\x1B\x11\x15a?\x9DW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x90\xFD[\x80_\x91[a@\x14WP\x90V[_\x19\x81\x01\x81\x81\x11a\x058Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x058W`\x01\x01\x90\x80a@\x0CV[\x90a@Ba8HV[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15a@\xE7W`\x01\x82\x14a@\xE2Wa@ca\x02\xE9V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15a@\x88WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14a@\xC2W[`\x01a@\xB8a@\xAD\x83`\xFF\x94a8\xD2V[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91a@tV[\x94`\x01a@\xB8a@\xADa@\xD7\x89`\xFF\x95a8\xD2V[\x98\x93PPPPa@\x9CV[PP\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x90\xFD[aA'a8HV[P\x80Q\x90\x81\x15\x80aA\x98W[\x15aATWPP`@QaAH`@\x82a\x02\x98V[_\x81R_` \x82\x01R\x90V[` _Q` aD\x1B_9_Q\x90_R\x91\x01Q\x06_Q` aD\x1B_9_Q\x90_R\x03_Q` aD\x1B_9_Q\x90_R\x81\x11a\x058W`@Q\x91a:G\x83a\x02]V[P` \x81\x01Q\x15aA3V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x91\x90\x91`\xFF_T`\x08\x1C\x16\x15aB\x0CWa\x02\xC8\x92a\x05\xFBa\x04\xC4\x92aA\xA4V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[\x15aBlWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90a\x01\0\x82Q\x11aCWW\x81Q\x15aCRWaC\x15aC\x0Ba\x1F'a\x1F\x19\x85a\x1D\x8AV[`\xFF`\x01\x91\x16\x1B\x90V[`\x01\x90[\x83Q\x82\x10\x15aCMW`\x01\x90aC8aC\x0Ba\x1F'a\x1F\x19\x86\x89a\x1D\x97V[\x90aCD\x81\x83\x11aBeV[\x17\x91\x01\x90aC\x19V[\x92PPV[_\x91PV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R\xFD[\x15aC\xD5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \x15\xB4\x99\x15`n\xFB\xD5\xFD\x86\xC25\x17\xE4\x9E\xA1\x16l\xC7\xC4\xB3\x93s\x0E}\x86\x08\x1F\x80\x15uidsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8063171f1d5b1461024457806326f017e21461023f57806333cfb7b71461023a5780633bc28c8c146102355780633d07142214610230578063416c7e5e1461022b5780635df459461461022657806367940c8914610221578063683048351461021c5780636b3aa72e146102175780636d14a987146102125780636ecbccfe1461020d5780636efb463614610208578063715018a614610203578063847d634f146101fe5780638999817f146101f95780638d68349a146101f45780638da5cb5b146101ef5780639926ee7d146101ea578063a364f4da146101e5578063a98fb355146101e0578063b1344271146101db578063b98d0908146101d6578063c1a8e2c5146101d1578063c4d66de8146101cc578063ca8aa7c7146101c7578063ce7b5e4b146101c2578063df5cf723146101bd578063e46f1816146101b8578063e481af9d146101b3578063f25f1610146101ae578063f2fde38b146101a9578063fc299dee146101a4578063fcd1c3751461019f5763fce36c7d1461019a575f80fd5b61187e565b611861565b611839565b6117a8565b6116e8565b6116cd565b6116a5565b611661565b6115f9565b6115b5565b6114bb565b6113a2565b611350565b611328565b61127f565b6111c2565b611126565b6110fe565b6110dc565b611051565b610f65565b610f0a565b610e75565b610b0a565b610a8e565b610a4a565b610a06565b6109e9565b6109a5565b610851565b610770565b6105d3565b61059b565b610481565b61040f565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761027857604052565b610249565b606081019081106001600160401b0382111761027857604052565b90601f801991011681019081106001600160401b0382111761027857604052565b604051906102c860a083610298565b565b604051906102c861010083610298565b604051906102c8606083610298565b604051906102c8604083610298565b906102c86040519283610298565b60409060e319011261032f576040519061031f8261025d565b60e4358252610104356020830152565b5f80fd5b919082604091031261032f5760405161034b8161025d565b6020808294803584520135910152565b9080601f8301121561032f5760405191610376604084610298565b82906040810192831161032f57905b8282106103925750505090565b8135815260209182019101610385565b90608060631983011261032f576040516103bb8161025d565b60206103d682946103cd81606461035b565b845260a461035b565b910152565b919060808382031261032f5760206103d6604051926103f98461025d565b60408496610407838261035b565b86520161035b565b3461032f5761012036600319011261032f57600435604036602319011261032f5761046760409182516104418161025d565b60243581526044356020820152610457366103a2565b9061046136610306565b92611b9e565b8251911515825215156020820152f35b5f91031261032f57565b3461032f575f36600319011261032f57610499613ce5565b60685462093a8081018091116105385742106104dc576067546104c9906001600160a01b0316613d3d565b613d3d565b606780546001600160a01b03191690555b005b60405162461bcd60e51b815260206004820152602e60248201527f536572766963654d616e616765723a20536c61736865722070726f706f73616c60448201526d0819195b185e481b9bdd081b595d60921b6064820152608490fd5b611c84565b6001600160a01b0381160361032f57565b35906102c88261053d565b60206040818301928281528451809452019201905f5b81811061057c5750505090565b82516001600160a01b031684526020938401939092019160010161056f565b3461032f57602036600319011261032f576105cf6105c36004356105be8161053d565b611e15565b60405191829182610559565b0390f35b3461032f57602036600319011261032f576104da6004356105f38161053d565b6105fb613ce5565b613e39565b63ffffffff81160361032f57565b604435906102c882610600565b35906102c882610600565b6001600160401b0381116102785760051b60200190565b9080601f8301121561032f57813561065481610626565b926106626040519485610298565b81845260208085019260051b82010192831161032f57602001905b82821061068a5750505090565b6020809183356106998161053d565b81520191019061067d565b9080601f8301121561032f5781356106bb81610626565b926106c96040519485610298565b81845260208085019260051b82010192831161032f57602001905b8282106106f15750505090565b81358152602091820191016106e4565b6001600160401b03811161027857601f01601f191660200190565b92919261072882610701565b916107366040519384610298565b82948184528183011161032f578281602093845f960137010152565b9080601f8301121561032f5781602061076d9335910161071c565b90565b3461032f57602036600319011261032f576004356001600160401b03811161032f5760a0600319823603011261032f576107a86102b9565b6107b48260040161054e565b81526107c26024830161061b565b602082015260448201356001600160401b03811161032f576107ea906004369185010161063d565b604082015260648201356001600160401b03811161032f5761081290600436918501016106a4565b606082015260848201356001600160401b03811161032f576104da92600461083d9236920101610752565b6080820152612192565b8015150361032f57565b3461032f57602036600319011261032f5760043561086e81610847565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156109a0575f91610965575b506001600160a01b031633036108d4576104da90613e97565b60405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a490fd5b90506020813d602011610998575b8161098060209383610298565b8101031261032f57516109928161053d565b5f6108bb565b3d9150610973565b611cfa565b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461032f575f36600319011261032f57602060405162093a808152f35b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b90604060031983011261032f57600435610aeb81610600565b91602435906001600160401b03821161032f5761076d9160040161063d565b3461032f57610b1836610ad2565b610b20613ed5565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561032f57604051630287f75160e51b8152915f918391610b7291306004850161237b565b93818381819703925af180156109a057610b8a575080f35b6104da91505f90610298565b929190610ba281610626565b93610bb06040519586610298565b602085838152019160051b810192831161032f57905b828210610bd257505050565b602080918335610be181610600565b815201910190610bc6565b9080601f8301121561032f5781602061076d93359101610b96565b81601f8201121561032f578035610c1d81610626565b92610c2b6040519485610298565b81845260208085019260061b8401019281841161032f57602001915b838310610c55575050505090565b6020604091610c648486610333565b815201920191610c47565b9080601f8301121561032f578135610c8681610626565b92610c946040519485610298565b81845260208085019260051b8201019183831161032f5760208201905b838210610cc057505050505090565b81356001600160401b03811161032f57602091610ce287848094880101610bec565b815201910190610cb1565b9190916101808184031261032f57610d036102ca565b9281356001600160401b03811161032f5781610d20918401610bec565b845260208201356001600160401b03811161032f5781610d41918401610c07565b602085015260408201356001600160401b03811161032f5781610d65918401610c07565b6040850152610d7781606084016103db565b6060850152610d898160e08401610333565b60808501526101208201356001600160401b03811161032f5781610dae918401610bec565b60a08501526101408201356001600160401b03811161032f5781610dd3918401610bec565b60c08501526101608201356001600160401b03811161032f57610df69201610c6f565b60e0830152565b90602080835192838152019201905f5b818110610e1a5750505090565b82516001600160601b0316845260209384019390920191600101610e0d565b929190610e706020916040865282610e5c82516040808a01526080890190610dfd565b910151868203603f19016060880152610dfd565b930152565b3461032f57608036600319011261032f576004356024356001600160401b03811161032f573660238201121561032f5780600401356001600160401b03811161032f57366024828401011161032f57610ecc61060e565b90606435936001600160401b03851161032f576024610ef2610efa963690600401610ced565b940190612860565b906105cf60405192839283610e39565b3461032f575f36600319011261032f57610f22613ce5565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461032f57602036600319011261032f576004356001600160401b03811161032f573660238201121561032f57806004013590610fa182610626565b90610faf6040519283610298565b8282526024602083019360051b8201019036821161032f5760248101935b828510610fdd576104da846131cd565b84356001600160401b03811161032f5782016040602319823603011261032f576040519161100a8361025d565b602482013561101881610600565b83526044820135926001600160401b03841161032f5761104260209493602486953692010161063d565b83820152815201940193610fcd565b3461032f57602036600319011261032f577f2f8afc8a78fd958f3301c0233aa326b9c4b9a2884a7483227d6b0555aaa03adb6110d76004356110928161053d565b61109a613ce5565b606780546001600160a01b0319166001600160a01b0392909216918217905542606881905560408051928352602083019190915290918291820190565b0390a1005b3461032f575f36600319011261032f57602060ff606954166040519015158152f35b3461032f575f36600319011261032f576033546040516001600160a01b039091168152602090f35b3461032f57604036600319011261032f576004356111438161053d565b602435906001600160401b03821161032f576060600319833603011261032f57604051906111708261027d565b82600401356001600160401b03811161032f5783013660238201121561032f576104da936111aa604492369060246004820135910161071c565b845260248101356020850152013560408301526132d5565b3461032f575f602036600319011261032f576004356111e08161053d565b611214337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614613140565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b1561032f576040516351b27a6d60e11b81526001600160a01b039091166004820152905f908290602490829084905af180156109a057610b8a575080f35b3461032f575f602036600319011261032f576004356001600160401b03811161032f576112b0903690600401610752565b6112b8613ce5565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561032f5760405163a98fb35560e01b815260206004820152915f918391829084908290611316906024830190612357565b03925af180156109a057610b8a575080f35b3461032f575f36600319011261032f576066546040516001600160a01b039091168152602090f35b3461032f575f36600319011261032f57602060ff609754166040519015158152f35b9181601f8401121561032f578235916001600160401b03831161032f576020808501948460051b01011161032f57565b3461032f57604036600319011261032f576004356113bf8161053d565b6024356001600160401b03811161032f576113e1611469913690600401611372565b9190611417337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614613140565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169261145c61144d6102da565b6001600160a01b039096168652565b3060208601523691610b96565b6040830152803b1561032f57604051636e3492b560e01b8152905f90829081838161149788600483016133a1565b03925af180156109a0576114a757005b806114b55f6104da93610298565b80610477565b3461032f57602036600319011261032f576004356114d88161053d565b6115295f549161150c6114f66114f28560ff9060081c1690565b1590565b809481956115a7575b8115611587575b5061340b565b8261151d600160ff195f5416175f55565b611570575b80806141ec565b61152f57005b61153d61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016110d7565b61158261010061ff00195f5416175f55565b611522565b303b15915081611599575b505f611506565b60ff1660011490505f611592565b600160ff82161091506114ff565b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461032f5761160736610ad2565b61160f613ed5565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561032f5760405163b66bd98960e01b8152915f918391610b7291306004850161237b565b3461032f575f36600319011261032f576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461032f575f36600319011261032f576067546040516001600160a01b039091168152602090f35b3461032f575f36600319011261032f576105cf6105c361346e565b3461032f57602036600319011261032f576004356117058161053d565b611739337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614613140565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b1561032f576040516334f65bfd60e21b81523060048201526001600160a01b039091166024820152905f908290604490829084905af180156109a0576114a757005b3461032f57602036600319011261032f576004356117c58161053d565b6117cd613ce5565b6001600160a01b038116156117e5576104da906141a4565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461032f575f36600319011261032f576065546040516001600160a01b039091168152602090f35b3461032f575f36600319011261032f576020606854604051908152f35b3461032f57602036600319011261032f576004356001600160401b03811161032f576118ae903690600401611372565b6065549091906001600160a01b03163303611ae0577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b8181106119225750823b1561032f57611497925f928360405180968195829463fce36c7d60e01b845260048401613739565b5f602061198961194c6119408361193a87898b61369b565b016136bd565b6001600160a01b031690565b604061195986888a61369b565b6040516323b872dd60e01b8152336004820152306024820152910135604482015293849283919082906064820190565b03925af180156109a057611ac4575b506119ac611940602061193a84868861369b565b604051636eb1769f60e11b81523060048201526001600160a01b03861660248201529190602090839060449082905afa80156109a057611a4f6020915f948591611a97575b50611a1c611a076119408561193a888b8d61369b565b916040611a15878a8c61369b565b0135611cde565b60405163095ea7b360e01b81526001600160a01b038a166004820152602481019190915294859283919082906044820190565b03925af19182156109a057600192611a69575b50016118f0565b611a899060203d8111611a90575b611a818183610298565b8101906136c7565b505f611a62565b503d611a77565b611ab79150833d8111611abd575b611aaf8183610298565b810190611ceb565b5f6119f1565b503d611aa5565b611adb9060203d8111611a9057611a818183610298565b611998565b60405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a490fd5b634e487b7160e01b5f52603260045260245ffd5b906002811015611b855760051b0190565b611b60565b634e487b7160e01b5f52601260045260245ffd5b611c7a611c57611c8095611c51611c4a85875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e0840152610100830152611c2181610120840103601f198101835282610298565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b809661388c565b906138d2565b92611c51611c6c611c6661395a565b94613a51565b91611c75613b6d565b61388c565b91613bb7565b9091565b634e487b7160e01b5f52601160045260245ffd5b906001820180921161053857565b906002820180921161053857565b906003820180921161053857565b906004820180921161053857565b906005820180921161053857565b9190820180921161053857565b9081602091031261032f575190565b6040513d5f823e3d90fd5b9081602091031261032f57516001600160c01b038116810361032f5790565b9081602091031261032f575160ff8116810361032f5790565b60405190611d4c602083610298565b5f808352366020840137565b90611d6282610626565b611d6f6040519182610298565b8281528092611d80601f1991610626565b0190602036910137565b805115611b855760200190565b908151811015611b85570160200190565b6001600160601b0381160361032f57565b9081604091031261032f57602060405191611dd38361025d565b8051611dde8161053d565b83520151611deb81611da8565b602082015290565b8051821015611b855760209160051b010190565b5f1981146105385760010190565b6040516309aa152760e11b81526001600160a01b0391821660048201527f000000000000000000000000000000000000000000000000000000000000000090911690602081602481855afa9081156109a057611e95916020915f91612175575b506040518093819263871ef04960e01b8352600483019190602083019252565b0381855afa9081156109a0575f91612146575b506001600160c01b03169081159081156120e3575b506120da57611ecb90613d9b565b5f91907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b8151851015611f9757611f506020611f2d611f27611f198987611d97565b516001600160f81b03191690565b60f81c90565b604051633ca5a5f560e01b815260ff909116600482015291829081906024820190565b0381875afa80156109a057600192611f6f925f92611f77575b50611cde565b940193611efb565b611f9091925060203d8111611abd57611aaf8183610298565b905f611f69565b611fa2919450611d58565b925f905f5b81518110156120d457611fc0611f27611f198385611d97565b604051633ca5a5f560e01b815260ff8216600482015290602082602481895afa9182156109a0575f926120b4575b50905f915b81831061200557505050600101611fa7565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156109a0576120788b6120698361206361194060019861207d985f91612086575b50516001600160a01b031690565b92611df3565b6001600160a01b039091169052565b611e07565b95019190611ff3565b6120a7915060403d81116120ad575b61209f8183610298565b810190611db9565b5f612055565b503d612095565b6120cd91925060203d8111611abd57611aaf8183610298565b905f611fee565b50505050565b5061076d611d3d565b604051639aa1653d60e01b81529150602090829060049082905afa80156109a05760ff915f91612117575b5016155f611ebd565b612139915060203d60201161213f575b6121318183610298565b810190611d24565b5f61210e565b503d612127565b612168915060203d60201161216e575b6121608183610298565b810190611d05565b5f611ea8565b503d612156565b61218c9150823d8411611abd57611aaf8183610298565b5f611e75565b6066546001600160a01b031633036122b0577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561032f5760408051633635205760e01b81523060048201526024810182905283516001600160a01b03166044820152602084015163ffffffff1660648201529083015160a060848301529092839161222d9060e484019061231b565b906060810151916043198482030160a4850152602080845192838152019301905f5b818110612297575050505f836122788296946080839501516043198483030160c4850152612357565b03925af180156109a0576122895750565b806114b55f6102c893610298565b825185528795506020948501949092019160010161224f565b60405162461bcd60e51b815260206004820152603960248201527f536572766963654d616e61676572426173652e6f6e6c79536c61736865723a2060448201527f63616c6c6572206973206e6f742074686520736c6173686572000000000000006064820152608490fd5b90602080835192838152019201905f5b8181106123385750505090565b82516001600160a01b031684526020938401939092019160010161232b565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b63ffffffff61076d949360609360018060a01b03168352166020820152816040820152019061231b565b604051906123b28261025d565b60606020838281520152565b156123c557565b60405162461bcd60e51b815260206004820152603760248201525f51602061443b5f395f51905f5260448201527f7265733a20656d7074792071756f72756d20696e7075740000000000000000006064820152608490fd5b1561242457565b60405162461bcd60e51b815260206004820152604160248201525f51602061443b5f395f51905f5260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a490fd5b1561248d57565b60a460405162461bcd60e51b815260206004820152604460248201525f51602061443b5f395f51905f5260448201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b6084820152fd5b156124f857565b60405162461bcd60e51b815260206004820152603c60248201525f51602061443b5f395f51905f5260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608490fd5b5f1981019190821161053857565b1561256557565b608460405162461bcd60e51b815260206004820152604060248201525f51602061443b5f395f51905f5260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152fd5b9081602091031261032f575161076d81610600565b90821015611b85570190565b156125e457565b60405162461bcd60e51b815260206004820152606660248201525f51602061443b5f395f51905f5260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c490fd5b9081602091031261032f575167ffffffffffffffff198116810361032f5790565b1561269957565b60405162461bcd60e51b815260206004820152606160248201525f51602061443b5f395f51905f5260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c490fd5b9081602091031261032f575161076d81611da8565b906001600160601b03809116911603906001600160601b03821161053857565b1561275d57565b60405162461bcd60e51b815260206004820152604360248201525f51602061443b5f395f51905f5260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a490fd5b156127c857565b60405162461bcd60e51b815260206004820152603960248201525f51602061443b5f395f51905f5260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608490fd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b81811061284a5750505090565b825184526020938401939092019160010161283d565b94939290919361286e6123a5565b5061287a8515156123be565b604084015151851480613132575b80613124575b80613116575b61289d9061241d565b6128af60208501515185515114612486565b6128c663ffffffff431663ffffffff8416106124f1565b6128ce6102e9565b5f81525f6020820152926128e06123a5565b6128e987611d58565b60208201526128f787611d58565b81526129016123a5565b92612910602088015151611d58565b8452612920602088015151611d58565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156109a057612989915f916130f7575b50612984368b8761071c565b613f81565b985f965b60208901518051891015612b05576020886129fa6129f08c6129e88f96868e6129cd6129ba868095611df3565b5180515f526020015160205260405f2090565b6129da8484840151611df3565b5282612ad2575b0151611df3565b519551611df3565b5163ffffffff1690565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9182156109a057611c518a612aa78f612aa08f8460208f92612a9793612a8f8460019e612aad9e5f91612ab5575b508f8060c01b03169251611df3565b520151611df3565b51938d51611df3565b5116614008565b90614039565b97019661298d565b612acc9150863d811161216e576121608183610298565b5f612a80565b612b00612ae28484840151611df3565b51612af984840151612af387612550565b90611df3565b511061255e565b6129e1565b50909597949650612b1a91989392995061411f565b91612b2760975460ff1690565b9081156130ef576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156109a0575f916130c0575b5091905b5f925b818410612be657505050505092612bbf612bba612bb3612be09585612bd29860806060602099015192015192611b9e565b9190612756565b6127c1565b0151604051928391602083019586612820565b03601f198101835282610298565b51902090565b92989596909399919794878b888c888d612fba575b6129f08260a0612c49611f27612c3b84612c5197612c35612c276129ba8f9c604060209f9e0151611df3565b67ffffffffffffffff191690565b9b6125d1565b356001600160f81b03191690565b970151611df3565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156109a057612d156129f08f958f90612d0d8f978f96848f612d0760c096612d00848f60209f906129e1612c3b99604093611f279c5f91612f8c575b5067ffffffffffffffff19918216911614612692565b51906138d2565b9c6125d1565b960151611df3565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156109a057612da2918c8f925f92612f68575b506020612d9492930151611df3565b906001600160601b03169052565b612dcf8c612d948c612dc8612dbb826020860151611df3565b516001600160601b031690565b9251611df3565b5f985f5b60208a015151811015612f4f578b8d612e1189612e04611f27612c3b868f89612dfc9151611df3565b5194876125d1565b60ff161c60019081161490565b612e20575b5050600101612dd3565b8a8a612ea8859f948f9686612e628f9360e0612e596129f0956020612e51611f27612c3b839f612e689c89916125d1565b9a0151611df3565b519b0151611df3565b51611df3565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156109a0578f612f0e908f936001959486955f92612f19575b50612063612d9492935193612f09612dbb8487611df3565b612736565b019a90508b8d612e16565b612d949250612f416120639160203d8111612f48575b612f398183610298565b810190612721565b9250612ef1565b503d612f2f565b5093919796996001919699509a94929a01929190612b82565b612d949250612f85602091823d8111612f4857612f398183610298565b9250612d85565b6020612fad92503d8111612fb3575b612fa58183610298565b810190612671565b5f612cea565b503d612f9b565b612ff79450612fd49250611f2791612c3b916020956125d1565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156109a057602089612c518f938f60a08f97611f27612c3b8f8f90612c35612c276129ba8f60408b96918f88936129f09f61307b9061308193612c499f5f92613097575b5063ffffffff809116931690611cde565b116125dd565b5050505050509750505050505092935050612bfb565b602063ffffffff92935082916130b8913d8111611abd57611aaf8183610298565b92915061306a565b6130e2915060203d6020116130e8575b6130da8183610298565b8101906125bc565b5f612b7b565b503d6130d0565b5f9190612b7f565b613110915060203d60201161213f576121318183610298565b5f612978565b5060e0840151518514612894565b5060c084015151851461288e565b5060a0840151518514612888565b1561314757565b60405162461bcd60e51b815260206004820152605260248201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360448201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560648201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608482015260a490fd5b613201337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614613140565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561032f576040518092630130fc2760e51b8252604482013060048401526040602484015281518091526064830190602060648260051b8601019301915f905b82821061328f575050505091815f81819503925af180156109a0576122895750565b919360019193955060206132c581926063198b82030186526040838a5163ffffffff81511684520151918185820152019061231b565b960192019201869493919261326d565b613309337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614613140565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561032f575f928392604051948580948193639926ee7d60e01b835260018060a01b0316600483015260406024830152604061337e82516060604486015260a4850190612357565b91602081015160648501520151608483015203925af180156109a0576122895750565b602080825282516001600160a01b039081168284015281840151166040808401919091529092015160608083015280516080830181905260a09092019201905f5b8181106133ef5750505090565b825163ffffffff168452602093840193909201916001016133e2565b1561341257565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156109a05760ff915f9161367c575b50168015613672577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b80831061362e57506135099150611d58565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156109a05760ff915f91613610575b501681101561360957604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa9182156109a0575f926135e9575b50905f915b8183106135835750505060010161350e565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156109a0576120788b612069836120636119406001986135e0985f916120865750516001600160a01b031690565b95019190613571565b61360291925060203d8111611abd57611aaf8183610298565b905f61356c565b5092505050565b613628915060203d811161213f576121318183610298565b5f613534565b604051633ca5a5f560e01b815260ff84166004820152909190602081602481885afa80156109a057600192613669925f92611f775750611cde565b920191906134f7565b505061076d611d3d565b613695915060203d60201161213f576121318183610298565b5f6134bf565b9190811015611b855760051b81013590609e198136030182121561032f570190565b3561076d8161053d565b9081602091031261032f575161076d81610847565b916020908281520191905f5b8181106136f55750505090565b90919260408060019286356137098161053d565b848060a01b031681526001600160601b03602088013561372881611da8565b1660208201520194019291016136e8565b9180602084016020855252604083019060408160051b85010193835f91609e1982360301905b848410613770575050505050505090565b90919293949596603f1982820301875287358381121561032f578401908135601e198336030181121561032f57820191602083359301906001600160401b03841161032f578360061b3603821361032f5761383983608061382e816137e46020989760019a60a08b9a5260a08701916136dc565b956138026137f389830161054e565b6001600160a01b0316868a0152565b6040810135604086015261382861381b6060830161061b565b63ffffffff166060870152565b0161061b565b63ffffffff16910152565b9901970195940192919061375f565b604051906138558261025d565b5f6020838281520152565b604051906101806138718184610298565b368337565b60405190613885602083610298565b6020368337565b9190604090606061389b613848565b94859260208551926138ad8585610298565b8436853780518452015160208301528482015260076107cf195a01fa156138d057565bfe5b6020929160806040926138e3613848565b958693818651936138f48686610298565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa80156138d0571561392557565b60405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606490fd5b6040516139668161025d565b60409081516139758382610298565b823682378152602082519161398a8484610298565b833684370152805161399c8282610298565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed60208201528151906139f28383610298565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152613a4783519384610298565b8252602082015290565b5f51602061441b5f395f51905f5290613a68613848565b505f919006602060c0835b613b68575f935f51602061441b5f395f51905f5260038186818180090908604051613a9e8582610298565b84368237848185604051613ab28282610298565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f51602061441b5f395f51905f5260a082015260056107cf195a01fa80156138d057613b1c906143ce565b5191613b68575f51602061441b5f395f51905f5282800914613b5357505f51602061441b5f395f51905f5260015f94089293613a73565b92935050613b5f6102e9565b92835282015290565b611b8a565b613b75613848565b50604051613b828161025d565b600181526002602082015290565b9060068202918083046006149015171561053857565b90600c811015611b855760051b0190565b93929091613bc560406102f8565b9485526020850152613bd760406102f8565b9182526020820152613be7613860565b925f5b60028110613c1457505050602061018092613c03613876565b93849160086201d4c0fa9151151590565b80613c20600192613b90565b613c2a8285611b74565b5151613c368289613ba6565b526020613c438386611b74565b510151613c58613c5283611c98565b89613ba6565b52613c638286611b74565b515151613c72613c5283611ca6565b52613c88613c808387611b74565b515160200190565b51613c95613c5283611cb4565b526020613ca28387611b74565b51015151613cb2613c5283611cc2565b52613cde613cd8613cd16020613cc8868a611b74565b51015160200190565b5192611cd0565b88613ba6565b5201613bea565b6033546001600160a01b03163303613cf957565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606654604080516001600160a01b038084168252841660208201529192917fe0d49a54274423183dadecbdf239eaac6e06ba88320b26fe8cc5ec9d050a63959190a16001600160a01b03166001600160a01b03199190911617606655565b61ffff613da782614008565b16613db181610701565b90613dbf6040519283610298565b808252613dce601f1991610701565b013660208301375f5f5b8251821080613e2e575b15613e27576001811b8416613e00575b613dfb90611e07565b613dd8565b906001613dfb9160ff60f81b8460f81b165f1a613e1d8287611d97565b5301919050613df2565b5050905090565b506101008110613de2565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303613f0757565b60405162461bcd60e51b815260206004820152604660248201527f536572766963654d616e61676572426173652e6f6e6c795374616b655265676960448201527f737472793a2063616c6c6572206973206e6f7420746865207374616b6520726560648201526567697374727960d01b608482015260a490fd5b906001613f8f60ff936142e7565b928392161b1115613f9d5790565b60405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608490fd5b805f915b614014575090565b5f1981018181116105385761ffff9116911661ffff811461053857600101908061400c565b90614042613848565b5061ffff8116906102008210156140e757600182146140e2576140636102e9565b5f81525f602082015292906001905f925b61ffff831685101561408857505050505090565b600161ffff831660ff86161c8116146140c2575b60016140b86140ad8360ff946138d2565b9460011b61fffe1690565b9401169291614074565b9460016140b86140ad6140d78960ff956138d2565b98935050505061409c565b505090565b60405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606490fd5b614127613848565b50805190811580614198575b15614154575050604051614148604082610298565b5f81525f602082015290565b60205f51602061441b5f395f51905f52910151065f51602061441b5f395f51905f52035f51602061441b5f395f51905f5281116105385760405191613a478361025d565b50602081015115614133565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b91909160ff5f5460081c161561420c576102c8926105fb6104c4926141a4565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b1561426c57565b60405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a490fd5b90610100825111614357578151156143525761431561430b611f27611f1985611d8a565b60ff600191161b90565b6001905b835182101561434d5760019061433861430b611f27611f198689611d97565b90614344818311614265565b17910190614319565b925050565b5f9150565b60a460405162461bcd60e51b815260206004820152604460248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b6084820152fd5b156143d557565b60405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606490fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a264697066735822122015b49915606efbd5fd86c23517e49ea1166cc7c4b393730e7d86081f8015756964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x17\x1F\x1D[\x14a\x02DW\x80c&\xF0\x17\xE2\x14a\x02?W\x80c3\xCF\xB7\xB7\x14a\x02:W\x80c;\xC2\x8C\x8C\x14a\x025W\x80c=\x07\x14\"\x14a\x020W\x80cAl~^\x14a\x02+W\x80c]\xF4YF\x14a\x02&W\x80cg\x94\x0C\x89\x14a\x02!W\x80ch0H5\x14a\x02\x1CW\x80ck:\xA7.\x14a\x02\x17W\x80cm\x14\xA9\x87\x14a\x02\x12W\x80cn\xCB\xCC\xFE\x14a\x02\rW\x80cn\xFBF6\x14a\x02\x08W\x80cqP\x18\xA6\x14a\x02\x03W\x80c\x84}cO\x14a\x01\xFEW\x80c\x89\x99\x81\x7F\x14a\x01\xF9W\x80c\x8Dh4\x9A\x14a\x01\xF4W\x80c\x8D\xA5\xCB[\x14a\x01\xEFW\x80c\x99&\xEE}\x14a\x01\xEAW\x80c\xA3d\xF4\xDA\x14a\x01\xE5W\x80c\xA9\x8F\xB3U\x14a\x01\xE0W\x80c\xB14Bq\x14a\x01\xDBW\x80c\xB9\x8D\t\x08\x14a\x01\xD6W\x80c\xC1\xA8\xE2\xC5\x14a\x01\xD1W\x80c\xC4\xD6m\xE8\x14a\x01\xCCW\x80c\xCA\x8A\xA7\xC7\x14a\x01\xC7W\x80c\xCE{^K\x14a\x01\xC2W\x80c\xDF\\\xF7#\x14a\x01\xBDW\x80c\xE4o\x18\x16\x14a\x01\xB8W\x80c\xE4\x81\xAF\x9D\x14a\x01\xB3W\x80c\xF2_\x16\x10\x14a\x01\xAEW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xA9W\x80c\xFC)\x9D\xEE\x14a\x01\xA4W\x80c\xFC\xD1\xC3u\x14a\x01\x9FWc\xFC\xE3l}\x14a\x01\x9AW_\x80\xFD[a\x18~V[a\x18aV[a\x189V[a\x17\xA8V[a\x16\xE8V[a\x16\xCDV[a\x16\xA5V[a\x16aV[a\x15\xF9V[a\x15\xB5V[a\x14\xBBV[a\x13\xA2V[a\x13PV[a\x13(V[a\x12\x7FV[a\x11\xC2V[a\x11&V[a\x10\xFEV[a\x10\xDCV[a\x10QV[a\x0FeV[a\x0F\nV[a\x0EuV[a\x0B\nV[a\n\x8EV[a\nJV[a\n\x06V[a\t\xE9V[a\t\xA5V[a\x08QV[a\x07pV[a\x05\xD3V[a\x05\x9BV[a\x04\x81V[a\x04\x0FV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02xW`@RV[a\x02IV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02xW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02xW`@RV[`@Q\x90a\x02\xC8`\xA0\x83a\x02\x98V[V[`@Q\x90a\x02\xC8a\x01\0\x83a\x02\x98V[`@Q\x90a\x02\xC8``\x83a\x02\x98V[`@Q\x90a\x02\xC8`@\x83a\x02\x98V[\x90a\x02\xC8`@Q\x92\x83a\x02\x98V[`@\x90`\xE3\x19\x01\x12a\x03/W`@Q\x90a\x03\x1F\x82a\x02]V[`\xE45\x82Ra\x01\x045` \x83\x01RV[_\x80\xFD[\x91\x90\x82`@\x91\x03\x12a\x03/W`@Qa\x03K\x81a\x02]V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W`@Q\x91a\x03v`@\x84a\x02\x98V[\x82\x90`@\x81\x01\x92\x83\x11a\x03/W\x90[\x82\x82\x10a\x03\x92WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x03\x85V[\x90`\x80`c\x19\x83\x01\x12a\x03/W`@Qa\x03\xBB\x81a\x02]V[` a\x03\xD6\x82\x94a\x03\xCD\x81`da\x03[V[\x84R`\xA4a\x03[V[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03/W` a\x03\xD6`@Q\x92a\x03\xF9\x84a\x02]V[`@\x84\x96a\x04\x07\x83\x82a\x03[V[\x86R\x01a\x03[V[4a\x03/Wa\x01 6`\x03\x19\x01\x12a\x03/W`\x045`@6`#\x19\x01\x12a\x03/Wa\x04g`@\x91\x82Qa\x04A\x81a\x02]V[`$5\x81R`D5` \x82\x01Ra\x04W6a\x03\xA2V[\x90a\x04a6a\x03\x06V[\x92a\x1B\x9EV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x03/WV[4a\x03/W_6`\x03\x19\x01\x12a\x03/Wa\x04\x99a<\xE5V[`hTb\t:\x80\x81\x01\x80\x91\x11a\x058WB\x10a\x04\xDCW`gTa\x04\xC9\x90`\x01`\x01`\xA0\x1B\x03\x16a==V[a==V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FServiceManager: Slasher proposal`D\x82\x01Rm\x08\x19\x19[\x18^H\x1B\x9B\xDD\x08\x1BY]`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a\x1C\x84V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03/WV[5\x90a\x02\xC8\x82a\x05=V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x05|WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x05oV[4a\x03/W` 6`\x03\x19\x01\x12a\x03/Wa\x05\xCFa\x05\xC3`\x045a\x05\xBE\x81a\x05=V[a\x1E\x15V[`@Q\x91\x82\x91\x82a\x05YV[\x03\x90\xF3[4a\x03/W` 6`\x03\x19\x01\x12a\x03/Wa\x04\xDA`\x045a\x05\xF3\x81a\x05=V[a\x05\xFBa<\xE5V[a>9V[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03/WV[`D5\x90a\x02\xC8\x82a\x06\0V[5\x90a\x02\xC8\x82a\x06\0V[`\x01`\x01`@\x1B\x03\x81\x11a\x02xW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W\x815a\x06T\x81a\x06&V[\x92a\x06b`@Q\x94\x85a\x02\x98V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03/W` \x01\x90[\x82\x82\x10a\x06\x8AWPPP\x90V[` \x80\x91\x835a\x06\x99\x81a\x05=V[\x81R\x01\x91\x01\x90a\x06}V[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W\x815a\x06\xBB\x81a\x06&V[\x92a\x06\xC9`@Q\x94\x85a\x02\x98V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03/W` \x01\x90[\x82\x82\x10a\x06\xF1WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x06\xE4V[`\x01`\x01`@\x1B\x03\x81\x11a\x02xW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07(\x82a\x07\x01V[\x91a\x076`@Q\x93\x84a\x02\x98V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03/W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W\x81` a\x07m\x935\x91\x01a\x07\x1CV[\x90V[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03/W`\xA0`\x03\x19\x826\x03\x01\x12a\x03/Wa\x07\xA8a\x02\xB9V[a\x07\xB4\x82`\x04\x01a\x05NV[\x81Ra\x07\xC2`$\x83\x01a\x06\x1BV[` \x82\x01R`D\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x07\xEA\x90`\x046\x91\x85\x01\x01a\x06=V[`@\x82\x01R`d\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x08\x12\x90`\x046\x91\x85\x01\x01a\x06\xA4V[``\x82\x01R`\x84\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x04\xDA\x92`\x04a\x08=\x926\x92\x01\x01a\x07RV[`\x80\x82\x01Ra!\x92V[\x80\x15\x15\x03a\x03/WV[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045a\x08n\x81a\x08GV[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\xA0W_\x91a\teW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x08\xD4Wa\x04\xDA\x90a>\x97V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[\x90P` \x81=` \x11a\t\x98W[\x81a\t\x80` \x93\x83a\x02\x98V[\x81\x01\x03\x12a\x03/WQa\t\x92\x81a\x05=V[_a\x08\xBBV[=\x91Pa\tsV[a\x1C\xFAV[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W` `@Qb\t:\x80\x81R\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90`@`\x03\x19\x83\x01\x12a\x03/W`\x045a\n\xEB\x81a\x06\0V[\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03/Wa\x07m\x91`\x04\x01a\x06=V[4a\x03/Wa\x0B\x186a\n\xD2V[a\x0B a>\xD5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x03/W`@Qc\x02\x87\xF7Q`\xE5\x1B\x81R\x91_\x91\x83\x91a\x0Br\x910`\x04\x85\x01a#{V[\x93\x81\x83\x81\x81\x97\x03\x92Z\xF1\x80\x15a\t\xA0Wa\x0B\x8AWP\x80\xF3[a\x04\xDA\x91P_\x90a\x02\x98V[\x92\x91\x90a\x0B\xA2\x81a\x06&V[\x93a\x0B\xB0`@Q\x95\x86a\x02\x98V[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x03/W\x90[\x82\x82\x10a\x0B\xD2WPPPV[` \x80\x91\x835a\x0B\xE1\x81a\x06\0V[\x81R\x01\x91\x01\x90a\x0B\xC6V[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W\x81` a\x07m\x935\x91\x01a\x0B\x96V[\x81`\x1F\x82\x01\x12\x15a\x03/W\x805a\x0C\x1D\x81a\x06&V[\x92a\x0C+`@Q\x94\x85a\x02\x98V[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03/W` \x01\x91[\x83\x83\x10a\x0CUWPPPP\x90V[` `@\x91a\x0Cd\x84\x86a\x033V[\x81R\x01\x92\x01\x91a\x0CGV[\x90\x80`\x1F\x83\x01\x12\x15a\x03/W\x815a\x0C\x86\x81a\x06&V[\x92a\x0C\x94`@Q\x94\x85a\x02\x98V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03/W` \x82\x01\x90[\x83\x82\x10a\x0C\xC0WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03/W` \x91a\x0C\xE2\x87\x84\x80\x94\x88\x01\x01a\x0B\xECV[\x81R\x01\x91\x01\x90a\x0C\xB1V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03/Wa\r\x03a\x02\xCAV[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x81a\r \x91\x84\x01a\x0B\xECV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x81a\rA\x91\x84\x01a\x0C\x07V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x81a\re\x91\x84\x01a\x0C\x07V[`@\x85\x01Ra\rw\x81``\x84\x01a\x03\xDBV[``\x85\x01Ra\r\x89\x81`\xE0\x84\x01a\x033V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x81a\r\xAE\x91\x84\x01a\x0B\xECV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x81a\r\xD3\x91\x84\x01a\x0B\xECV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\r\xF6\x92\x01a\x0CoV[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0E\x1AWPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0E\rV[\x92\x91\x90a\x0Ep` \x91`@\x86R\x82a\x0E\\\x82Q`@\x80\x8A\x01R`\x80\x89\x01\x90a\r\xFDV[\x91\x01Q\x86\x82\x03`?\x19\x01``\x88\x01Ra\r\xFDV[\x93\x01RV[4a\x03/W`\x806`\x03\x19\x01\x12a\x03/W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03/W6`#\x82\x01\x12\x15a\x03/W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W6`$\x82\x84\x01\x01\x11a\x03/Wa\x0E\xCCa\x06\x0EV[\x90`d5\x93`\x01`\x01`@\x1B\x03\x85\x11a\x03/W`$a\x0E\xF2a\x0E\xFA\x966\x90`\x04\x01a\x0C\xEDV[\x94\x01\x90a(`V[\x90a\x05\xCF`@Q\x92\x83\x92\x83a\x0E9V[4a\x03/W_6`\x03\x19\x01\x12a\x03/Wa\x0F\"a<\xE5V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03/W6`#\x82\x01\x12\x15a\x03/W\x80`\x04\x015\x90a\x0F\xA1\x82a\x06&V[\x90a\x0F\xAF`@Q\x92\x83a\x02\x98V[\x82\x82R`$` \x83\x01\x93`\x05\x1B\x82\x01\x01\x906\x82\x11a\x03/W`$\x81\x01\x93[\x82\x85\x10a\x0F\xDDWa\x04\xDA\x84a1\xCDV[\x845`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x82\x01`@`#\x19\x826\x03\x01\x12a\x03/W`@Q\x91a\x10\n\x83a\x02]V[`$\x82\x015a\x10\x18\x81a\x06\0V[\x83R`D\x82\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03/Wa\x10B` \x94\x93`$\x86\x956\x92\x01\x01a\x06=V[\x83\x82\x01R\x81R\x01\x94\x01\x93a\x0F\xCDV[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W\x7F/\x8A\xFC\x8Ax\xFD\x95\x8F3\x01\xC0#:\xA3&\xB9\xC4\xB9\xA2\x88Jt\x83\"}k\x05U\xAA\xA0:\xDBa\x10\xD7`\x045a\x10\x92\x81a\x05=V[a\x10\x9Aa<\xE5V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90UB`h\x81\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1\0[4a\x03/W_6`\x03\x19\x01\x12a\x03/W` `\xFF`iT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03/W`@6`\x03\x19\x01\x12a\x03/W`\x045a\x11C\x81a\x05=V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03/W```\x03\x19\x836\x03\x01\x12a\x03/W`@Q\x90a\x11p\x82a\x02}V[\x82`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03/W\x83\x016`#\x82\x01\x12\x15a\x03/Wa\x04\xDA\x93a\x11\xAA`D\x926\x90`$`\x04\x82\x015\x91\x01a\x07\x1CV[\x84R`$\x81\x015` \x85\x01R\x015`@\x83\x01Ra2\xD5V[4a\x03/W_` 6`\x03\x19\x01\x12a\x03/W`\x045a\x11\xE0\x81a\x05=V[a\x12\x143\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a1@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x03/W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xA0Wa\x0B\x8AWP\x80\xF3[4a\x03/W_` 6`\x03\x19\x01\x12a\x03/W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x12\xB0\x906\x90`\x04\x01a\x07RV[a\x12\xB8a<\xE5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03/W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x13\x16\x90`$\x83\x01\x90a#WV[\x03\x92Z\xF1\x80\x15a\t\xA0Wa\x0B\x8AWP\x80\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`fT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x03/W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03/W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x03/WV[4a\x03/W`@6`\x03\x19\x01\x12a\x03/W`\x045a\x13\xBF\x81a\x05=V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x13\xE1a\x14i\x916\x90`\x04\x01a\x13rV[\x91\x90a\x14\x173\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a1@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92a\x14\\a\x14Ma\x02\xDAV[`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x86RV[0` \x86\x01R6\x91a\x0B\x96V[`@\x83\x01R\x80;\x15a\x03/W`@Qcn4\x92\xB5`\xE0\x1B\x81R\x90_\x90\x82\x90\x81\x83\x81a\x14\x97\x88`\x04\x83\x01a3\xA1V[\x03\x92Z\xF1\x80\x15a\t\xA0Wa\x14\xA7W\0[\x80a\x14\xB5_a\x04\xDA\x93a\x02\x98V[\x80a\x04wV[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045a\x14\xD8\x81a\x05=V[a\x15)_T\x91a\x15\x0Ca\x14\xF6a\x14\xF2\x85`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x94\x81\x95a\x15\xA7W[\x81\x15a\x15\x87W[Pa4\x0BV[\x82a\x15\x1D`\x01`\xFF\x19_T\x16\x17_UV[a\x15pW[\x80\x80aA\xECV[a\x15/W\0[a\x15=a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x10\xD7V[a\x15\x82a\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x15\"V[0;\x15\x91P\x81a\x15\x99W[P_a\x15\x06V[`\xFF\x16`\x01\x14\x90P_a\x15\x92V[`\x01`\xFF\x82\x16\x10\x91Pa\x14\xFFV[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03/Wa\x16\x076a\n\xD2V[a\x16\x0Fa>\xD5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x03/W`@Qc\xB6k\xD9\x89`\xE0\x1B\x81R\x91_\x91\x83\x91a\x0Br\x910`\x04\x85\x01a#{V[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/Wa\x05\xCFa\x05\xC3a4nV[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045a\x17\x05\x81a\x05=V[a\x1793\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a1@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x03/W`@Qc4\xF6[\xFD`\xE2\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R\x90_\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xA0Wa\x14\xA7W\0[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045a\x17\xC5\x81a\x05=V[a\x17\xCDa<\xE5V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x17\xE5Wa\x04\xDA\x90aA\xA4V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03/W_6`\x03\x19\x01\x12a\x03/W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03/W_6`\x03\x19\x01\x12a\x03/W` `hT`@Q\x90\x81R\xF3[4a\x03/W` 6`\x03\x19\x01\x12a\x03/W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03/Wa\x18\xAE\x906\x90`\x04\x01a\x13rV[`eT\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1A\xE0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a\x19\"WP\x82;\x15a\x03/Wa\x14\x97\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xFC\xE3l}`\xE0\x1B\x84R`\x04\x84\x01a79V[_` a\x19\x89a\x19La\x19@\x83a\x19:\x87\x89\x8Ba6\x9BV[\x01a6\xBDV[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@a\x19Y\x86\x88\x8Aa6\x9BV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x91\x015`D\x82\x01R\x93\x84\x92\x83\x91\x90\x82\x90`d\x82\x01\x90V[\x03\x92Z\xF1\x80\x15a\t\xA0Wa\x1A\xC4W[Pa\x19\xACa\x19@` a\x19:\x84\x86\x88a6\x9BV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x16`$\x82\x01R\x91\x90` \x90\x83\x90`D\x90\x82\x90Z\xFA\x80\x15a\t\xA0Wa\x1AO` \x91_\x94\x85\x91a\x1A\x97W[Pa\x1A\x1Ca\x1A\x07a\x19@\x85a\x19:\x88\x8B\x8Da6\x9BV[\x91`@a\x1A\x15\x87\x8A\x8Ca6\x9BV[\x015a\x1C\xDEV[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x94\x85\x92\x83\x91\x90\x82\x90`D\x82\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\t\xA0W`\x01\x92a\x1AiW[P\x01a\x18\xF0V[a\x1A\x89\x90` =\x81\x11a\x1A\x90W[a\x1A\x81\x81\x83a\x02\x98V[\x81\x01\x90a6\xC7V[P_a\x1AbV[P=a\x1AwV[a\x1A\xB7\x91P\x83=\x81\x11a\x1A\xBDW[a\x1A\xAF\x81\x83a\x02\x98V[\x81\x01\x90a\x1C\xEBV[_a\x19\xF1V[P=a\x1A\xA5V[a\x1A\xDB\x90` =\x81\x11a\x1A\x90Wa\x1A\x81\x81\x83a\x02\x98V[a\x19\x98V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FServiceManagerBase.onlyRewardsIn`D\x82\x01R\x7Fitiator: caller is not the rewar`d\x82\x01Rk29\x904\xB74\xBA4\xB0\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x1B\x85W`\x05\x1B\x01\x90V[a\x1B`V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x1Cza\x1CWa\x1C\x80\x95a\x1CQa\x1CJ\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x1C!\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x98V[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96a8\x8CV[\x90a8\xD2V[\x92a\x1CQa\x1Cla\x1Cfa9ZV[\x94a:QV[\x91a\x1Cua;mV[a8\x8CV[\x91a;\xB7V[\x90\x91V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x058WV[\x90`\x02\x82\x01\x80\x92\x11a\x058WV[\x90`\x03\x82\x01\x80\x92\x11a\x058WV[\x90`\x04\x82\x01\x80\x92\x11a\x058WV[\x90`\x05\x82\x01\x80\x92\x11a\x058WV[\x91\x90\x82\x01\x80\x92\x11a\x058WV[\x90\x81` \x91\x03\x12a\x03/WQ\x90V[`@Q=_\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x03/WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03/W\x90V[\x90\x81` \x91\x03\x12a\x03/WQ`\xFF\x81\x16\x81\x03a\x03/W\x90V[`@Q\x90a\x1DL` \x83a\x02\x98V[_\x80\x83R6` \x84\x017V[\x90a\x1Db\x82a\x06&V[a\x1Do`@Q\x91\x82a\x02\x98V[\x82\x81R\x80\x92a\x1D\x80`\x1F\x19\x91a\x06&V[\x01\x90` 6\x91\x017V[\x80Q\x15a\x1B\x85W` \x01\x90V[\x90\x81Q\x81\x10\x15a\x1B\x85W\x01` \x01\x90V[`\x01`\x01``\x1B\x03\x81\x16\x03a\x03/WV[\x90\x81`@\x91\x03\x12a\x03/W` `@Q\x91a\x1D\xD3\x83a\x02]V[\x80Qa\x1D\xDE\x81a\x05=V[\x83R\x01Qa\x1D\xEB\x81a\x1D\xA8V[` \x82\x01R\x90V[\x80Q\x82\x10\x15a\x1B\x85W` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x058W`\x01\x01\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\t\xA0Wa\x1E\x95\x91` \x91_\x91a!uW[P`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x90\x81\x15a\t\xA0W_\x91a!FW[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81\x15\x90\x81\x15a \xE3W[Pa \xDAWa\x1E\xCB\x90a=\x9BV[_\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x1F\x97Wa\x1FP` a\x1F-a\x1F'a\x1F\x19\x89\x87a\x1D\x97V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x87Z\xFA\x80\x15a\t\xA0W`\x01\x92a\x1Fo\x92_\x92a\x1FwW[Pa\x1C\xDEV[\x94\x01\x93a\x1E\xFBV[a\x1F\x90\x91\x92P` =\x81\x11a\x1A\xBDWa\x1A\xAF\x81\x83a\x02\x98V[\x90_a\x1FiV[a\x1F\xA2\x91\x94Pa\x1DXV[\x92_\x90_[\x81Q\x81\x10\x15a \xD4Wa\x1F\xC0a\x1F'a\x1F\x19\x83\x85a\x1D\x97V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x90` \x82`$\x81\x89Z\xFA\x91\x82\x15a\t\xA0W_\x92a \xB4W[P\x90_\x91[\x81\x83\x10a \x05WPPP`\x01\x01a\x1F\xA7V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\t\xA0Wa x\x8Ba i\x83a ca\x19@`\x01\x98a }\x98_\x91a \x86W[PQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x1D\xF3V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x1E\x07V[\x95\x01\x91\x90a\x1F\xF3V[a \xA7\x91P`@=\x81\x11a \xADW[a \x9F\x81\x83a\x02\x98V[\x81\x01\x90a\x1D\xB9V[_a UV[P=a \x95V[a \xCD\x91\x92P` =\x81\x11a\x1A\xBDWa\x1A\xAF\x81\x83a\x02\x98V[\x90_a\x1F\xEEV[PPPPV[Pa\x07ma\x1D=V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\t\xA0W`\xFF\x91_\x91a!\x17W[P\x16\x15_a\x1E\xBDV[a!9\x91P` =` \x11a!?W[a!1\x81\x83a\x02\x98V[\x81\x01\x90a\x1D$V[_a!\x0EV[P=a!'V[a!h\x91P` =` \x11a!nW[a!`\x81\x83a\x02\x98V[\x81\x01\x90a\x1D\x05V[_a\x1E\xA8V[P=a!VV[a!\x8C\x91P\x82=\x84\x11a\x1A\xBDWa\x1A\xAF\x81\x83a\x02\x98V[_a\x1EuV[`fT`\x01`\x01`\xA0\x1B\x03\x163\x03a\"\xB0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03/W`@\x80Qc65 W`\xE0\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x82\x90R\x83Q`\x01`\x01`\xA0\x1B\x03\x16`D\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`d\x82\x01R\x90\x83\x01Q`\xA0`\x84\x83\x01R\x90\x92\x83\x91a\"-\x90`\xE4\x84\x01\x90a#\x1BV[\x90``\x81\x01Q\x91`C\x19\x84\x82\x03\x01`\xA4\x85\x01R` \x80\x84Q\x92\x83\x81R\x01\x93\x01\x90_[\x81\x81\x10a\"\x97WPPP_\x83a\"x\x82\x96\x94`\x80\x83\x95\x01Q`C\x19\x84\x83\x03\x01`\xC4\x85\x01Ra#WV[\x03\x92Z\xF1\x80\x15a\t\xA0Wa\"\x89WPV[\x80a\x14\xB5_a\x02\xC8\x93a\x02\x98V[\x82Q\x85R\x87\x95P` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a\"OV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FServiceManagerBase.onlySlasher: `D\x82\x01R\x7Fcaller is not the slasher\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a#8WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a#+V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[c\xFF\xFF\xFF\xFFa\x07m\x94\x93``\x93`\x01\x80`\xA0\x1B\x03\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a#\x1BV[`@Q\x90a#\xB2\x82a\x02]V[``` \x83\x82\x81R\x01RV[\x15a#\xC5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a$$WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a$\x8DWV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R\xFD[\x15a$\xF8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x90\xFD[_\x19\x81\x01\x91\x90\x82\x11a\x058WV[\x15a%eWV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R\xFD[\x90\x81` \x91\x03\x12a\x03/WQa\x07m\x81a\x06\0V[\x90\x82\x10\x15a\x1B\x85W\x01\x90V[\x15a%\xE4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90\x81` \x91\x03\x12a\x03/WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03/W\x90V[\x15a&\x99WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90\x81` \x91\x03\x12a\x03/WQa\x07m\x81a\x1D\xA8V[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a\x058WV[\x15a']WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a'\xC8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R_Q` aD;_9_Q\x90_R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a(JWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a(=V[\x94\x93\x92\x90\x91\x93a(na#\xA5V[Pa(z\x85\x15\x15a#\xBEV[`@\x84\x01QQ\x85\x14\x80a12W[\x80a1$W[\x80a1\x16W[a(\x9D\x90a$\x1DV[a(\xAF` \x85\x01QQ\x85QQ\x14a$\x86V[a(\xC6c\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a$\xF1V[a(\xCEa\x02\xE9V[_\x81R_` \x82\x01R\x92a(\xE0a#\xA5V[a(\xE9\x87a\x1DXV[` \x82\x01Ra(\xF7\x87a\x1DXV[\x81Ra)\x01a#\xA5V[\x92a)\x10` \x88\x01QQa\x1DXV[\x84Ra) ` \x88\x01QQa\x1DXV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\xA0Wa)\x89\x91_\x91a0\xF7W[Pa)\x846\x8B\x87a\x07\x1CV[a?\x81V[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a+\x05W` \x88a)\xFAa)\xF0\x8Ca)\xE8\x8F\x96\x86\x8Ea)\xCDa)\xBA\x86\x80\x95a\x1D\xF3V[Q\x80Q_R` \x01Q` R`@_ \x90V[a)\xDA\x84\x84\x84\x01Qa\x1D\xF3V[R\x82a*\xD2W[\x01Qa\x1D\xF3V[Q\x95Qa\x1D\xF3V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\t\xA0Wa\x1CQ\x8Aa*\xA7\x8Fa*\xA0\x8F\x84` \x8F\x92a*\x97\x93a*\x8F\x84`\x01\x9Ea*\xAD\x9E_\x91a*\xB5W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\x1D\xF3V[R\x01Qa\x1D\xF3V[Q\x93\x8DQa\x1D\xF3V[Q\x16a@\x08V[\x90a@9V[\x97\x01\x96a)\x8DV[a*\xCC\x91P\x86=\x81\x11a!nWa!`\x81\x83a\x02\x98V[_a*\x80V[a+\0a*\xE2\x84\x84\x84\x01Qa\x1D\xF3V[Qa*\xF9\x84\x84\x01Qa*\xF3\x87a%PV[\x90a\x1D\xF3V[Q\x10a%^V[a)\xE1V[P\x90\x95\x97\x94\x96Pa+\x1A\x91\x98\x93\x92\x99PaA\x1FV[\x91a+'`\x97T`\xFF\x16\x90V[\x90\x81\x15a0\xEFW`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\xA0W_\x91a0\xC0W[P\x91\x90[_\x92[\x81\x84\x10a+\xE6WPPPPP\x92a+\xBFa+\xBAa+\xB3a+\xE0\x95\x85a+\xD2\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x1B\x9EV[\x91\x90a'VV[a'\xC1V[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a( V[\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x98V[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da/\xBAW[a)\xF0\x82`\xA0a,Ia\x1F'a,;\x84a,Q\x97a,5a,'a)\xBA\x8F\x9C`@` \x9F\x9E\x01Qa\x1D\xF3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba%\xD1V[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[\x97\x01Qa\x1D\xF3V[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\t\xA0Wa-\x15a)\xF0\x8F\x95\x8F\x90a-\r\x8F\x97\x8F\x96\x84\x8Fa-\x07`\xC0\x96a-\0\x84\x8F` \x9F\x90a)\xE1a,;\x99`@\x93a\x1F'\x9C_\x91a/\x8CW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a&\x92V[Q\x90a8\xD2V[\x9Ca%\xD1V[\x96\x01Qa\x1D\xF3V[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\t\xA0Wa-\xA2\x91\x8C\x8F\x92_\x92a/hW[P` a-\x94\x92\x93\x01Qa\x1D\xF3V[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a-\xCF\x8Ca-\x94\x8Ca-\xC8a-\xBB\x82` \x86\x01Qa\x1D\xF3V[Q`\x01`\x01``\x1B\x03\x16\x90V[\x92Qa\x1D\xF3V[_\x98_[` \x8A\x01QQ\x81\x10\x15a/OW\x8B\x8Da.\x11\x89a.\x04a\x1F'a,;\x86\x8F\x89a-\xFC\x91Qa\x1D\xF3V[Q\x94\x87a%\xD1V[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a. W[PP`\x01\x01a-\xD3V[\x8A\x8Aa.\xA8\x85\x9F\x94\x8F\x96\x86a.b\x8F\x93`\xE0a.Ya)\xF0\x95` a.Qa\x1F'a,;\x83\x9Fa.h\x9C\x89\x91a%\xD1V[\x9A\x01Qa\x1D\xF3V[Q\x9B\x01Qa\x1D\xF3V[Qa\x1D\xF3V[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\xA0W\x8Fa/\x0E\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a/\x19W[Pa ca-\x94\x92\x93Q\x93a/\ta-\xBB\x84\x87a\x1D\xF3V[a'6V[\x01\x9A\x90P\x8B\x8Da.\x16V[a-\x94\x92Pa/Aa c\x91` =\x81\x11a/HW[a/9\x81\x83a\x02\x98V[\x81\x01\x90a'!V[\x92Pa.\xF1V[P=a//V[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a+\x82V[a-\x94\x92Pa/\x85` \x91\x82=\x81\x11a/HWa/9\x81\x83a\x02\x98V[\x92Pa-\x85V[` a/\xAD\x92P=\x81\x11a/\xB3W[a/\xA5\x81\x83a\x02\x98V[\x81\x01\x90a&qV[_a,\xEAV[P=a/\x9BV[a/\xF7\x94Pa/\xD4\x92Pa\x1F'\x91a,;\x91` \x95a%\xD1V[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\xA0W` \x89a,Q\x8F\x93\x8F`\xA0\x8F\x97a\x1F'a,;\x8F\x8F\x90a,5a,'a)\xBA\x8F`@\x8B\x96\x91\x8F\x88\x93a)\xF0\x9Fa0{\x90a0\x81\x93a,I\x9F_\x92a0\x97W[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a\x1C\xDEV[\x11a%\xDDV[PPPPPP\x97PPPPPP\x92\x93PPa+\xFBV[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a0\xB8\x91=\x81\x11a\x1A\xBDWa\x1A\xAF\x81\x83a\x02\x98V[\x92\x91Pa0jV[a0\xE2\x91P` =` \x11a0\xE8W[a0\xDA\x81\x83a\x02\x98V[\x81\x01\x90a%\xBCV[_a+{V[P=a0\xD0V[_\x91\x90a+\x7FV[a1\x10\x91P` =` \x11a!?Wa!1\x81\x83a\x02\x98V[_a)xV[P`\xE0\x84\x01QQ\x85\x14a(\x94V[P`\xC0\x84\x01QQ\x85\x14a(\x8EV[P`\xA0\x84\x01QQ\x85\x14a(\x88V[\x15a1GWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FServiceManagerBase.onlyRegistryC`D\x82\x01R\x7Foordinator: caller is not the re`d\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x84\x82\x01R`\xA4\x90\xFD[a2\x013\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a1@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03/W`@Q\x80\x92c\x010\xFC'`\xE5\x1B\x82R`D\x82\x010`\x04\x84\x01R`@`$\x84\x01R\x81Q\x80\x91R`d\x83\x01\x90` `d\x82`\x05\x1B\x86\x01\x01\x93\x01\x91_\x90[\x82\x82\x10a2\x8FWPPPP\x91\x81_\x81\x81\x95\x03\x92Z\xF1\x80\x15a\t\xA0Wa\"\x89WPV[\x91\x93`\x01\x91\x93\x95P` a2\xC5\x81\x92`c\x19\x8B\x82\x03\x01\x86R`@\x83\x8AQc\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a#\x1BV[\x96\x01\x92\x01\x92\x01\x86\x94\x93\x91\x92a2mV[a3\t3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a1@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x03/W_\x92\x83\x92`@Q\x94\x85\x80\x94\x81\x93c\x99&\xEE}`\xE0\x1B\x83R`\x01\x80`\xA0\x1B\x03\x16`\x04\x83\x01R`@`$\x83\x01R`@a3~\x82Q```D\x86\x01R`\xA4\x85\x01\x90a#WV[\x91` \x81\x01Q`d\x85\x01R\x01Q`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\t\xA0Wa\"\x89WPV[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x84\x01R\x81\x84\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x90\x92\x01Q``\x80\x83\x01R\x80Q`\x80\x83\x01\x81\x90R`\xA0\x90\x92\x01\x92\x01\x90_[\x81\x81\x10a3\xEFWPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a3\xE2V[\x15a4\x12WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\t\xA0W`\xFF\x91_\x91a6|W[P\x16\x80\x15a6rW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a6.WPa5\t\x91Pa\x1DXV[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\t\xA0W`\xFF\x91_\x91a6\x10W[P\x16\x81\x10\x15a6\tW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\t\xA0W_\x92a5\xE9W[P\x90_\x91[\x81\x83\x10a5\x83WPPP`\x01\x01a5\x0EV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\t\xA0Wa x\x8Ba i\x83a ca\x19@`\x01\x98a5\xE0\x98_\x91a \x86WPQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01\x91\x90a5qV[a6\x02\x91\x92P` =\x81\x11a\x1A\xBDWa\x1A\xAF\x81\x83a\x02\x98V[\x90_a5lV[P\x92PPPV[a6(\x91P` =\x81\x11a!?Wa!1\x81\x83a\x02\x98V[_a54V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x88Z\xFA\x80\x15a\t\xA0W`\x01\x92a6i\x92_\x92a\x1FwWPa\x1C\xDEV[\x92\x01\x91\x90a4\xF7V[PPa\x07ma\x1D=V[a6\x95\x91P` =` \x11a!?Wa!1\x81\x83a\x02\x98V[_a4\xBFV[\x91\x90\x81\x10\x15a\x1B\x85W`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x03/W\x01\x90V[5a\x07m\x81a\x05=V[\x90\x81` \x91\x03\x12a\x03/WQa\x07m\x81a\x08GV[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a6\xF5WPPP\x90V[\x90\x91\x92`@\x80`\x01\x92\x865a7\t\x81a\x05=V[\x84\x80`\xA0\x1B\x03\x16\x81R`\x01`\x01``\x1B\x03` \x88\x015a7(\x81a\x1D\xA8V[\x16` \x82\x01R\x01\x94\x01\x92\x91\x01a6\xE8V[\x91\x80` \x84\x01` \x85RR`@\x83\x01\x90`@\x81`\x05\x1B\x85\x01\x01\x93\x83_\x91`\x9E\x19\x826\x03\x01\x90[\x84\x84\x10a7pWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`?\x19\x82\x82\x03\x01\x87R\x875\x83\x81\x12\x15a\x03/W\x84\x01\x90\x815`\x1E\x19\x836\x03\x01\x81\x12\x15a\x03/W\x82\x01\x91` \x835\x93\x01\x90`\x01`\x01`@\x1B\x03\x84\x11a\x03/W\x83`\x06\x1B6\x03\x82\x13a\x03/Wa89\x83`\x80a8.\x81a7\xE4` \x98\x97`\x01\x9A`\xA0\x8B\x9AR`\xA0\x87\x01\x91a6\xDCV[\x95a8\x02a7\xF3\x89\x83\x01a\x05NV[`\x01`\x01`\xA0\x1B\x03\x16\x86\x8A\x01RV[`@\x81\x015`@\x86\x01Ra8(a8\x1B``\x83\x01a\x06\x1BV[c\xFF\xFF\xFF\xFF\x16``\x87\x01RV[\x01a\x06\x1BV[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x99\x01\x97\x01\x95\x94\x01\x92\x91\x90a7_V[`@Q\x90a8U\x82a\x02]V[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80a8q\x81\x84a\x02\x98V[6\x837V[`@Q\x90a8\x85` \x83a\x02\x98V[` 6\x837V[\x91\x90`@\x90``a8\x9Ba8HV[\x94\x85\x92` \x85Q\x92a8\xAD\x85\x85a\x02\x98V[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15a8\xD0WV[\xFE[` \x92\x91`\x80`@\x92a8\xE3a8HV[\x95\x86\x93\x81\x86Q\x93a8\xF4\x86\x86a\x02\x98V[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15a8\xD0W\x15a9%WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x90\xFD[`@Qa9f\x81a\x02]V[`@\x90\x81Qa9u\x83\x82a\x02\x98V[\x826\x827\x81R` \x82Q\x91a9\x8A\x84\x84a\x02\x98V[\x836\x847\x01R\x80Qa9\x9C\x82\x82a\x02\x98V[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90a9\xF2\x83\x83a\x02\x98V[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01Ra:G\x83Q\x93\x84a\x02\x98V[\x82R` \x82\x01R\x90V[_Q` aD\x1B_9_Q\x90_R\x90a:ha8HV[P_\x91\x90\x06` `\xC0\x83[a;hW_\x93_Q` aD\x1B_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa:\x9E\x85\x82a\x02\x98V[\x846\x827\x84\x81\x85`@Qa:\xB2\x82\x82a\x02\x98V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aD\x1B_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a8\xD0Wa;\x1C\x90aC\xCEV[Q\x91a;hW_Q` aD\x1B_9_Q\x90_R\x82\x80\t\x14a;SWP_Q` aD\x1B_9_Q\x90_R`\x01_\x94\x08\x92\x93a:sV[\x92\x93PPa;_a\x02\xE9V[\x92\x83R\x82\x01R\x90V[a\x1B\x8AV[a;ua8HV[P`@Qa;\x82\x81a\x02]V[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a\x058WV[\x90`\x0C\x81\x10\x15a\x1B\x85W`\x05\x1B\x01\x90V[\x93\x92\x90\x91a;\xC5`@a\x02\xF8V[\x94\x85R` \x85\x01Ra;\xD7`@a\x02\xF8V[\x91\x82R` \x82\x01Ra;\xE7a8`V[\x92_[`\x02\x81\x10a<\x14WPPP` a\x01\x80\x92a<\x03a8vV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80a< `\x01\x92a;\x90V[a<*\x82\x85a\x1BtV[QQa<6\x82\x89a;\xA6V[R` a<C\x83\x86a\x1BtV[Q\x01Qa<Xa<R\x83a\x1C\x98V[\x89a;\xA6V[Ra<c\x82\x86a\x1BtV[QQQa<ra<R\x83a\x1C\xA6V[Ra<\x88a<\x80\x83\x87a\x1BtV[QQ` \x01\x90V[Qa<\x95a<R\x83a\x1C\xB4V[R` a<\xA2\x83\x87a\x1BtV[Q\x01QQa<\xB2a<R\x83a\x1C\xC2V[Ra<\xDEa<\xD8a<\xD1` a<\xC8\x86\x8Aa\x1BtV[Q\x01Q` \x01\x90V[Q\x92a\x1C\xD0V[\x88a;\xA6V[R\x01a;\xEAV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a<\xF9WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`fT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE0\xD4\x9AT'D#\x18=\xAD\xEC\xBD\xF29\xEA\xACn\x06\xBA\x882\x0B&\xFE\x8C\xC5\xEC\x9D\x05\nc\x95\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`fUV[a\xFF\xFFa=\xA7\x82a@\x08V[\x16a=\xB1\x81a\x07\x01V[\x90a=\xBF`@Q\x92\x83a\x02\x98V[\x80\x82Ra=\xCE`\x1F\x19\x91a\x07\x01V[\x016` \x83\x017__[\x82Q\x82\x10\x80a>.W[\x15a>'W`\x01\x81\x1B\x84\x16a>\0W[a=\xFB\x90a\x1E\x07V[a=\xD8V[\x90`\x01a=\xFB\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa>\x1D\x82\x87a\x1D\x97V[S\x01\x91\x90Pa=\xF2V[PP\x90P\x90V[Pa\x01\0\x81\x10a=\xE2V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eUV[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a?\x07WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FServiceManagerBase.onlyStakeRegi`D\x82\x01R\x7Fstry: caller is not the stake re`d\x82\x01Registry`\xD0\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90`\x01a?\x8F`\xFF\x93aB\xE7V[\x92\x83\x92\x16\x1B\x11\x15a?\x9DW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x90\xFD[\x80_\x91[a@\x14WP\x90V[_\x19\x81\x01\x81\x81\x11a\x058Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x058W`\x01\x01\x90\x80a@\x0CV[\x90a@Ba8HV[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15a@\xE7W`\x01\x82\x14a@\xE2Wa@ca\x02\xE9V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15a@\x88WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14a@\xC2W[`\x01a@\xB8a@\xAD\x83`\xFF\x94a8\xD2V[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91a@tV[\x94`\x01a@\xB8a@\xADa@\xD7\x89`\xFF\x95a8\xD2V[\x98\x93PPPPa@\x9CV[PP\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x90\xFD[aA'a8HV[P\x80Q\x90\x81\x15\x80aA\x98W[\x15aATWPP`@QaAH`@\x82a\x02\x98V[_\x81R_` \x82\x01R\x90V[` _Q` aD\x1B_9_Q\x90_R\x91\x01Q\x06_Q` aD\x1B_9_Q\x90_R\x03_Q` aD\x1B_9_Q\x90_R\x81\x11a\x058W`@Q\x91a:G\x83a\x02]V[P` \x81\x01Q\x15aA3V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x91\x90\x91`\xFF_T`\x08\x1C\x16\x15aB\x0CWa\x02\xC8\x92a\x05\xFBa\x04\xC4\x92aA\xA4V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[\x15aBlWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90a\x01\0\x82Q\x11aCWW\x81Q\x15aCRWaC\x15aC\x0Ba\x1F'a\x1F\x19\x85a\x1D\x8AV[`\xFF`\x01\x91\x16\x1B\x90V[`\x01\x90[\x83Q\x82\x10\x15aCMW`\x01\x90aC8aC\x0Ba\x1F'a\x1F\x19\x86\x89a\x1D\x97V[\x90aCD\x81\x83\x11aBeV[\x17\x91\x01\x90aC\x19V[\x92PPV[_\x91PV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R\xFD[\x15aC\xD5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \x15\xB4\x99\x15`n\xFB\xD5\xFD\x86\xC25\x17\xE4\x9E\xA1\x16l\xC7\xC4\xB3\x93s\x0E}\x86\x08\x1F\x80\x15uidsolcC\0\x08\x1B\x003",
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
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsInitiatorUpdated(address,address)` and selector `0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3`.
```solidity
event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsInitiatorUpdated {
        #[allow(missing_docs)]
        pub prevRewardsInitiator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newRewardsInitiator: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RewardsInitiatorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RewardsInitiatorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                225u8,
                28u8,
                221u8,
                241u8,
                129u8,
                106u8,
                67u8,
                49u8,
                140u8,
                161u8,
                117u8,
                187u8,
                197u8,
                44u8,
                208u8,
                24u8,
                84u8,
                54u8,
                233u8,
                203u8,
                234u8,
                215u8,
                200u8,
                58u8,
                204u8,
                84u8,
                167u8,
                62u8,
                70u8,
                23u8,
                23u8,
                227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevRewardsInitiator: data.0,
                    newRewardsInitiator: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.prevRewardsInitiator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newRewardsInitiator,
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
        impl alloy_sol_types::private::IntoLogData for RewardsInitiatorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsInitiatorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RewardsInitiatorUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlasherProposed(address,uint256)` and selector `0x2f8afc8a78fd958f3301c0233aa326b9c4b9a2884a7483227d6b0555aaa03adb`.
```solidity
event SlasherProposed(address newSlasher, uint256 slasherProposalTimestamp);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlasherProposed {
        #[allow(missing_docs)]
        pub newSlasher: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub slasherProposalTimestamp: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for SlasherProposed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlasherProposed(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                138u8,
                252u8,
                138u8,
                120u8,
                253u8,
                149u8,
                143u8,
                51u8,
                1u8,
                192u8,
                35u8,
                58u8,
                163u8,
                38u8,
                185u8,
                196u8,
                185u8,
                162u8,
                136u8,
                74u8,
                116u8,
                131u8,
                34u8,
                125u8,
                107u8,
                5u8,
                85u8,
                170u8,
                160u8,
                58u8,
                219u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newSlasher: data.0,
                    slasherProposalTimestamp: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newSlasher,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.slasherProposalTimestamp,
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
        impl alloy_sol_types::private::IntoLogData for SlasherProposed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlasherProposed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SlasherProposed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlasherUpdated(address,address)` and selector `0xe0d49a54274423183dadecbdf239eaac6e06ba88320b26fe8cc5ec9d050a6395`.
```solidity
event SlasherUpdated(address prevSlasher, address newSlasher);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlasherUpdated {
        #[allow(missing_docs)]
        pub prevSlasher: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newSlasher: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for SlasherUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlasherUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                224u8,
                212u8,
                154u8,
                84u8,
                39u8,
                68u8,
                35u8,
                24u8,
                61u8,
                173u8,
                236u8,
                189u8,
                242u8,
                57u8,
                234u8,
                172u8,
                110u8,
                6u8,
                186u8,
                136u8,
                50u8,
                11u8,
                38u8,
                254u8,
                140u8,
                197u8,
                236u8,
                157u8,
                5u8,
                10u8,
                99u8,
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
                    prevSlasher: data.0,
                    newSlasher: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.prevSlasher,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newSlasher,
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
        impl alloy_sol_types::private::IntoLogData for SlasherUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlasherUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SlasherUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StaleStakesForbiddenUpdate(bool)` and selector `0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc`.
```solidity
event StaleStakesForbiddenUpdate(bool value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StaleStakesForbiddenUpdate {
        #[allow(missing_docs)]
        pub value: bool,
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
        impl alloy_sol_types::SolEvent for StaleStakesForbiddenUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StaleStakesForbiddenUpdate(bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8,
                228u8,
                237u8,
                136u8,
                10u8,
                41u8,
                224u8,
                246u8,
                221u8,
                206u8,
                48u8,
                116u8,
                87u8,
                251u8,
                117u8,
                205u8,
                223u8,
                79u8,
                238u8,
                247u8,
                211u8,
                236u8,
                176u8,
                48u8,
                27u8,
                253u8,
                244u8,
                151u8,
                106u8,
                14u8,
                45u8,
                252u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { value: data.0 }
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.value,
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
        impl alloy_sol_types::private::IntoLogData for StaleStakesForbiddenUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StaleStakesForbiddenUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &StaleStakesForbiddenUpdate,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _registryCoordinator, address _avsDirectory, address _rewardsCoordinator, address _allocationManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _registryCoordinator: alloy::sol_types::private::Address,
        pub _avsDirectory: alloy::sol_types::private::Address,
        pub _rewardsCoordinator: alloy::sol_types::private::Address,
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
                    (
                        value._registryCoordinator,
                        value._avsDirectory,
                        value._rewardsCoordinator,
                        value._allocationManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                        _avsDirectory: tuple.1,
                        _rewardsCoordinator: tuple.2,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._rewardsCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `SLASHER_PROPOSAL_DELAY()` and selector `0x67940c89`.
```solidity
function SLASHER_PROPOSAL_DELAY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLASHER_PROPOSAL_DELAYCall {}
    ///Container type for the return parameters of the [`SLASHER_PROPOSAL_DELAY()`](SLASHER_PROPOSAL_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLASHER_PROPOSAL_DELAYReturn {
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
            impl ::core::convert::From<SLASHER_PROPOSAL_DELAYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SLASHER_PROPOSAL_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SLASHER_PROPOSAL_DELAYCall {
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
            impl ::core::convert::From<SLASHER_PROPOSAL_DELAYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SLASHER_PROPOSAL_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SLASHER_PROPOSAL_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SLASHER_PROPOSAL_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = SLASHER_PROPOSAL_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SLASHER_PROPOSAL_DELAY()";
            const SELECTOR: [u8; 4] = [103u8, 148u8, 12u8, 137u8];
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
    /**Function with signature `acceptProposedSlasher()` and selector `0x26f017e2`.
```solidity
function acceptProposedSlasher() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptProposedSlasherCall {}
    ///Container type for the return parameters of the [`acceptProposedSlasher()`](acceptProposedSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptProposedSlasherReturn {}
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
            impl ::core::convert::From<acceptProposedSlasherCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptProposedSlasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptProposedSlasherCall {
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
            impl ::core::convert::From<acceptProposedSlasherReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptProposedSlasherReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptProposedSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptProposedSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptProposedSlasherReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptProposedSlasher()";
            const SELECTOR: [u8; 4] = [38u8, 240u8, 23u8, 226u8];
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
    /**Function with signature `addStrategyToOperatorSet(uint32,address[])` and selector `0x6ecbccfe`.
```solidity
function addStrategyToOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategyToOperatorSetCall {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`addStrategyToOperatorSet(uint32,address[])`](addStrategyToOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategyToOperatorSetReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
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
            impl ::core::convert::From<addStrategyToOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addStrategyToOperatorSetCall) -> Self {
                    (value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addStrategyToOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetId: tuple.0,
                        strategies: tuple.1,
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
            impl ::core::convert::From<addStrategyToOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addStrategyToOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addStrategyToOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addStrategyToOperatorSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStrategyToOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addStrategyToOperatorSet(uint32,address[])";
            const SELECTOR: [u8; 4] = [110u8, 203u8, 204u8, 254u8];
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
            impl ::core::convert::From<allocationManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerCall {
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
            impl ::core::convert::From<allocationManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `blsApkRegistry()` and selector `0x5df45946`.
```solidity
function blsApkRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryCall {}
    ///Container type for the return parameters of the [`blsApkRegistry()`](blsApkRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryReturn {
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
            impl ::core::convert::From<blsApkRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryCall {
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
            impl ::core::convert::From<blsApkRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blsApkRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blsApkRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = blsApkRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blsApkRegistry()";
            const SELECTOR: [u8; 4] = [93u8, 244u8, 89u8, 70u8];
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
    /**Function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x6efb4636`.
```solidity
function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureChecker.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureChecker.QuorumStakeTotals memory, bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesCall {
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        pub referenceBlockNumber: u32,
        pub params: <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](checkSignaturesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesReturn {
        pub _0: <IBLSSignatureChecker::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
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
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureChecker::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                u32,
                <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<checkSignaturesCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesCall) -> Self {
                    (
                        value.msgHash,
                        value.quorumNumbers,
                        value.referenceBlockNumber,
                        value.params,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        quorumNumbers: tuple.1,
                        referenceBlockNumber: tuple.2,
                        params: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IBLSSignatureChecker::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IBLSSignatureChecker::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<checkSignaturesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkSignaturesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkSignaturesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureChecker::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkSignaturesReturn;
            type ReturnTuple<'a> = (
                IBLSSignatureChecker::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [110u8, 251u8, 70u8, 54u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                    <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xfce36c7d`.
```solidity
function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionCall {
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])`](createAVSRewardsSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionReturn {}
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
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createAVSRewardsSubmissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createAVSRewardsSubmissionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        rewardsSubmissions: tuple.0,
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
            impl ::core::convert::From<createAVSRewardsSubmissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createAVSRewardsSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [252u8, 227u8, 108u8, 125u8];
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
                        IRewardsCoordinatorTypes::RewardsSubmission,
                    > as alloy_sol_types::SolType>::tokenize(&self.rewardsSubmissions),
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
    /**Function with signature `createOperatorSets((uint32,address[])[])` and selector `0x847d634f`.
```solidity
function createOperatorSets(IAllocationManagerTypes.CreateSetParams[] memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsCall {
        pub params: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorSets((uint32,address[])[])`](createOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsReturn {}
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
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::CreateSetParams,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetsCall {
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
            impl ::core::convert::From<createOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::CreateSetParams,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorSets((uint32,address[])[])";
            const SELECTOR: [u8; 4] = [132u8, 125u8, 99u8, 79u8];
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
                        IAllocationManagerTypes::CreateSetParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.params),
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `deregisterOperatorFromAVS(address)` and selector `0xa364f4da`.
```solidity
function deregisterOperatorFromAVS(address operator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromAVS(address)`](deregisterOperatorFromAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSReturn {}
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
            impl ::core::convert::From<deregisterOperatorFromAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromAVSCall {
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
            impl ::core::convert::From<deregisterOperatorFromAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromAVS(address)";
            const SELECTOR: [u8; 4] = [163u8, 100u8, 244u8, 218u8];
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
    /**Function with signature `deregisterOperatorFromOperatorSets(address,uint32[])` and selector `0xc1a8e2c5`.
```solidity
function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromOperatorSets(address,uint32[])`](deregisterOperatorFromOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
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
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsCall) -> Self {
                    (value.operator, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSetIds: tuple.1,
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
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromOperatorSets(address,uint32[])";
            const SELECTOR: [u8; 4] = [193u8, 168u8, 226u8, 197u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
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
    /**Function with signature `getOperatorRestakedStrategies(address)` and selector `0x33cfb7b7`.
```solidity
function getOperatorRestakedStrategies(address operator) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorRestakedStrategies(address)`](getOperatorRestakedStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesReturn {
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
            impl ::core::convert::From<getOperatorRestakedStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorRestakedStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<getOperatorRestakedStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorRestakedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorRestakedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorRestakedStrategiesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorRestakedStrategies(address)";
            const SELECTOR: [u8; 4] = [51u8, 207u8, 183u8, 183u8];
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
    /**Function with signature `getRestakeableStrategies()` and selector `0xe481af9d`.
```solidity
function getRestakeableStrategies() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesCall {}
    ///Container type for the return parameters of the [`getRestakeableStrategies()`](getRestakeableStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesReturn {
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
            impl ::core::convert::From<getRestakeableStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRestakeableStrategiesCall {
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
            impl ::core::convert::From<getRestakeableStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRestakeableStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRestakeableStrategiesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRestakeableStrategiesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRestakeableStrategies()";
            const SELECTOR: [u8; 4] = [228u8, 129u8, 175u8, 157u8];
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
    /**Function with signature `initialize(address)` and selector `0xc4d66de8`.
```solidity
function initialize(address _initialOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _initialOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address)`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._initialOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _initialOwner: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address)";
            const SELECTOR: [u8; 4] = [196u8, 214u8, 109u8, 232u8];
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
                        &self._initialOwner,
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
    /**Function with signature `migrationFinalized()` and selector `0x8d68349a`.
```solidity
function migrationFinalized() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrationFinalizedCall {}
    ///Container type for the return parameters of the [`migrationFinalized()`](migrationFinalizedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrationFinalizedReturn {
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
            impl ::core::convert::From<migrationFinalizedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrationFinalizedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrationFinalizedCall {
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
            impl ::core::convert::From<migrationFinalizedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrationFinalizedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrationFinalizedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrationFinalizedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrationFinalizedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrationFinalized()";
            const SELECTOR: [u8; 4] = [141u8, 104u8, 52u8, 154u8];
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
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
    /**Function with signature `proposeNewSlasher(address)` and selector `0x8999817f`.
```solidity
function proposeNewSlasher(address newSlasher) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeNewSlasherCall {
        pub newSlasher: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`proposeNewSlasher(address)`](proposeNewSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeNewSlasherReturn {}
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
            impl ::core::convert::From<proposeNewSlasherCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposeNewSlasherCall) -> Self {
                    (value.newSlasher,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposeNewSlasherCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newSlasher: tuple.0 }
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
            impl ::core::convert::From<proposeNewSlasherReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposeNewSlasherReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposeNewSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposeNewSlasherCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposeNewSlasherReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposeNewSlasher(address)";
            const SELECTOR: [u8; 4] = [137u8, 153u8, 129u8, 127u8];
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
                        &self.newSlasher,
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
    /**Function with signature `proposedSlasher()` and selector `0xe46f1816`.
```solidity
function proposedSlasher() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposedSlasherCall {}
    ///Container type for the return parameters of the [`proposedSlasher()`](proposedSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposedSlasherReturn {
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
            impl ::core::convert::From<proposedSlasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposedSlasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposedSlasherCall {
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
            impl ::core::convert::From<proposedSlasherReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposedSlasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposedSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposedSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposedSlasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposedSlasher()";
            const SELECTOR: [u8; 4] = [228u8, 111u8, 24u8, 22u8];
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
    /**Function with signature `registerOperatorToAVS(address,(bytes,bytes32,uint256))` and selector `0x9926ee7d`.
```solidity
function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperatorToAVS(address,(bytes,bytes32,uint256))`](registerOperatorToAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSReturn {}
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
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerOperatorToAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSCall) -> Self {
                    (value.operator, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSignature: tuple.1,
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
            impl ::core::convert::From<registerOperatorToAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorToAVSCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorToAVS(address,(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [153u8, 38u8, 238u8, 125u8];
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
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
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
            impl ::core::convert::From<registryCoordinatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registryCoordinatorCall {
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
            impl ::core::convert::From<registryCoordinatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registryCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registryCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registryCoordinatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `removeStrategiesFromOperatorSet(uint32,address[])` and selector `0xce7b5e4b`.
```solidity
function removeStrategiesFromOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesFromOperatorSetCall {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`removeStrategiesFromOperatorSet(uint32,address[])`](removeStrategiesFromOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesFromOperatorSetReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
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
            impl ::core::convert::From<removeStrategiesFromOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetCall) -> Self {
                    (value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesFromOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetId: tuple.0,
                        strategies: tuple.1,
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
            impl ::core::convert::From<removeStrategiesFromOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesFromOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeStrategiesFromOperatorSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeStrategiesFromOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeStrategiesFromOperatorSet(uint32,address[])";
            const SELECTOR: [u8; 4] = [206u8, 123u8, 94u8, 75u8];
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
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
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
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
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
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
    /**Function with signature `rewardsInitiator()` and selector `0xfc299dee`.
```solidity
function rewardsInitiator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorCall {}
    ///Container type for the return parameters of the [`rewardsInitiator()`](rewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorReturn {
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
            impl ::core::convert::From<rewardsInitiatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsInitiatorCall {
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
            impl ::core::convert::From<rewardsInitiatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsInitiatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsInitiatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rewardsInitiator()";
            const SELECTOR: [u8; 4] = [252u8, 41u8, 157u8, 238u8];
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
    /**Function with signature `setAVSRegistrar(address)` and selector `0xf25f1610`.
```solidity
function setAVSRegistrar(address registrar) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarCall {
        pub registrar: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setAVSRegistrar(address)`](setAVSRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarReturn {}
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
            impl ::core::convert::From<setAVSRegistrarCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarCall) -> Self {
                    (value.registrar,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSRegistrarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { registrar: tuple.0 }
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
            impl ::core::convert::From<setAVSRegistrarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAVSRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAVSRegistrarCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAVSRegistrarReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAVSRegistrar(address)";
            const SELECTOR: [u8; 4] = [242u8, 95u8, 22u8, 16u8];
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
                        &self.registrar,
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
    /**Function with signature `setRewardsInitiator(address)` and selector `0x3bc28c8c`.
```solidity
function setRewardsInitiator(address newRewardsInitiator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorCall {
        pub newRewardsInitiator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setRewardsInitiator(address)`](setRewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorReturn {}
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
            impl ::core::convert::From<setRewardsInitiatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorCall) -> Self {
                    (value.newRewardsInitiator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsInitiatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newRewardsInitiator: tuple.0,
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
            impl ::core::convert::From<setRewardsInitiatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsInitiatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsInitiatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRewardsInitiator(address)";
            const SELECTOR: [u8; 4] = [59u8, 194u8, 140u8, 140u8];
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
                        &self.newRewardsInitiator,
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
    /**Function with signature `setStaleStakesForbidden(bool)` and selector `0x416c7e5e`.
```solidity
function setStaleStakesForbidden(bool value) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStaleStakesForbiddenCall {
        pub value: bool,
    }
    ///Container type for the return parameters of the [`setStaleStakesForbidden(bool)`](setStaleStakesForbiddenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStaleStakesForbiddenReturn {}
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
            impl ::core::convert::From<setStaleStakesForbiddenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setStaleStakesForbiddenCall) -> Self {
                    (value.value,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setStaleStakesForbiddenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { value: tuple.0 }
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
            impl ::core::convert::From<setStaleStakesForbiddenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setStaleStakesForbiddenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setStaleStakesForbiddenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setStaleStakesForbiddenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bool,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setStaleStakesForbiddenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setStaleStakesForbidden(bool)";
            const SELECTOR: [u8; 4] = [65u8, 108u8, 126u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.value,
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
    /**Function with signature `slashOperator((address,uint32,address[],uint256[],string))` and selector `0x3d071422`.
```solidity
function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        pub params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`slashOperator((address,uint32,address[],uint256[],string))`](slashOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorReturn {}
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
            impl ::core::convert::From<slashOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorCall {
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
            impl ::core::convert::From<slashOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashOperatorCall {
            type Parameters<'a> = (IAllocationManagerTypes::SlashingParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashOperator((address,uint32,address[],uint256[],string))";
            const SELECTOR: [u8; 4] = [61u8, 7u8, 20u8, 34u8];
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
    /**Function with signature `slasherProposalTimestamp()` and selector `0xfcd1c375`.
```solidity
function slasherProposalTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherProposalTimestampCall {}
    ///Container type for the return parameters of the [`slasherProposalTimestamp()`](slasherProposalTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherProposalTimestampReturn {
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
            impl ::core::convert::From<slasherProposalTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slasherProposalTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slasherProposalTimestampCall {
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
            impl ::core::convert::From<slasherProposalTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slasherProposalTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slasherProposalTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slasherProposalTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slasherProposalTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slasherProposalTimestamp()";
            const SELECTOR: [u8; 4] = [252u8, 209u8, 195u8, 117u8];
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
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
```solidity
function stakeRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    ///Container type for the return parameters of the [`stakeRegistry()`](stakeRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryReturn {
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
            impl ::core::convert::From<stakeRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryCall {
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
            impl ::core::convert::From<stakeRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeRegistry()";
            const SELECTOR: [u8; 4] = [104u8, 48u8, 72u8, 53u8];
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
    /**Function with signature `staleStakesForbidden()` and selector `0xb98d0908`.
```solidity
function staleStakesForbidden() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct staleStakesForbiddenCall {}
    ///Container type for the return parameters of the [`staleStakesForbidden()`](staleStakesForbiddenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct staleStakesForbiddenReturn {
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
            impl ::core::convert::From<staleStakesForbiddenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: staleStakesForbiddenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for staleStakesForbiddenCall {
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
            impl ::core::convert::From<staleStakesForbiddenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: staleStakesForbiddenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for staleStakesForbiddenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for staleStakesForbiddenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = staleStakesForbiddenReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "staleStakesForbidden()";
            const SELECTOR: [u8; 4] = [185u8, 141u8, 9u8, 8u8];
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
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
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
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
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
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
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
                        &self.newOwner,
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
    /**Function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`.
```solidity
function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationCall {
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        pub apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`](trySignatureAndApkVerificationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationReturn {
        pub pairingSuccessful: bool,
        pub siganatureIsValid: bool,
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
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                <BN254::G2Point as alloy::sol_types::SolType>::RustType,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<trySignatureAndApkVerificationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationCall) -> Self {
                    (value.msgHash, value.apk, value.apkG2, value.sigma)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for trySignatureAndApkVerificationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        apk: tuple.1,
                        apkG2: tuple.2,
                        sigma: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, bool);
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
            impl ::core::convert::From<trySignatureAndApkVerificationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationReturn) -> Self {
                    (value.pairingSuccessful, value.siganatureIsValid)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for trySignatureAndApkVerificationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pairingSuccessful: tuple.0,
                        siganatureIsValid: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for trySignatureAndApkVerificationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = trySignatureAndApkVerificationReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))";
            const SELECTOR: [u8; 4] = [23u8, 31u8, 29u8, 91u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.apk),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
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
    /**Function with signature `updateAVSMetadataURI(string)` and selector `0xa98fb355`.
```solidity
function updateAVSMetadataURI(string memory _metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        pub _metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateAVSMetadataURI(string)`](updateAVSMetadataURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURIReturn {}
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
            impl ::core::convert::From<updateAVSMetadataURICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value._metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _metadataURI: tuple.0 }
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
            impl ::core::convert::From<updateAVSMetadataURIReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateAVSMetadataURICall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAVSMetadataURI(string)";
            const SELECTOR: [u8; 4] = [169u8, 143u8, 179u8, 85u8];
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
                        &self._metadataURI,
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
    ///Container for all the [`MockAvsServiceManager`](self) function calls.
    pub enum MockAvsServiceManagerCalls {
        SLASHER_PROPOSAL_DELAY(SLASHER_PROPOSAL_DELAYCall),
        acceptProposedSlasher(acceptProposedSlasherCall),
        addStrategyToOperatorSet(addStrategyToOperatorSetCall),
        allocationManager(allocationManagerCall),
        avsDirectory(avsDirectoryCall),
        blsApkRegistry(blsApkRegistryCall),
        checkSignatures(checkSignaturesCall),
        createAVSRewardsSubmission(createAVSRewardsSubmissionCall),
        createOperatorSets(createOperatorSetsCall),
        delegation(delegationCall),
        deregisterOperatorFromAVS(deregisterOperatorFromAVSCall),
        deregisterOperatorFromOperatorSets(deregisterOperatorFromOperatorSetsCall),
        getOperatorRestakedStrategies(getOperatorRestakedStrategiesCall),
        getRestakeableStrategies(getRestakeableStrategiesCall),
        initialize(initializeCall),
        migrationFinalized(migrationFinalizedCall),
        owner(ownerCall),
        proposeNewSlasher(proposeNewSlasherCall),
        proposedSlasher(proposedSlasherCall),
        registerOperatorToAVS(registerOperatorToAVSCall),
        registryCoordinator(registryCoordinatorCall),
        removeStrategiesFromOperatorSet(removeStrategiesFromOperatorSetCall),
        renounceOwnership(renounceOwnershipCall),
        rewardsInitiator(rewardsInitiatorCall),
        setAVSRegistrar(setAVSRegistrarCall),
        setRewardsInitiator(setRewardsInitiatorCall),
        setStaleStakesForbidden(setStaleStakesForbiddenCall),
        slashOperator(slashOperatorCall),
        slasher(slasherCall),
        slasherProposalTimestamp(slasherProposalTimestampCall),
        stakeRegistry(stakeRegistryCall),
        staleStakesForbidden(staleStakesForbiddenCall),
        transferOwnership(transferOwnershipCall),
        trySignatureAndApkVerification(trySignatureAndApkVerificationCall),
        updateAVSMetadataURI(updateAVSMetadataURICall),
    }
    #[automatically_derived]
    impl MockAvsServiceManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [23u8, 31u8, 29u8, 91u8],
            [38u8, 240u8, 23u8, 226u8],
            [51u8, 207u8, 183u8, 183u8],
            [59u8, 194u8, 140u8, 140u8],
            [61u8, 7u8, 20u8, 34u8],
            [65u8, 108u8, 126u8, 94u8],
            [93u8, 244u8, 89u8, 70u8],
            [103u8, 148u8, 12u8, 137u8],
            [104u8, 48u8, 72u8, 53u8],
            [107u8, 58u8, 167u8, 46u8],
            [109u8, 20u8, 169u8, 135u8],
            [110u8, 203u8, 204u8, 254u8],
            [110u8, 251u8, 70u8, 54u8],
            [113u8, 80u8, 24u8, 166u8],
            [132u8, 125u8, 99u8, 79u8],
            [137u8, 153u8, 129u8, 127u8],
            [141u8, 104u8, 52u8, 154u8],
            [141u8, 165u8, 203u8, 91u8],
            [153u8, 38u8, 238u8, 125u8],
            [163u8, 100u8, 244u8, 218u8],
            [169u8, 143u8, 179u8, 85u8],
            [177u8, 52u8, 66u8, 113u8],
            [185u8, 141u8, 9u8, 8u8],
            [193u8, 168u8, 226u8, 197u8],
            [196u8, 214u8, 109u8, 232u8],
            [202u8, 138u8, 167u8, 199u8],
            [206u8, 123u8, 94u8, 75u8],
            [223u8, 92u8, 247u8, 35u8],
            [228u8, 111u8, 24u8, 22u8],
            [228u8, 129u8, 175u8, 157u8],
            [242u8, 95u8, 22u8, 16u8],
            [242u8, 253u8, 227u8, 139u8],
            [252u8, 41u8, 157u8, 238u8],
            [252u8, 209u8, 195u8, 117u8],
            [252u8, 227u8, 108u8, 125u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockAvsServiceManagerCalls {
        const NAME: &'static str = "MockAvsServiceManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 35usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::SLASHER_PROPOSAL_DELAY(_) => {
                    <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptProposedSlasher(_) => {
                    <acceptProposedSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStrategyToOperatorSet(_) => {
                    <addStrategyToOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkSignatures(_) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createAVSRewardsSubmission(_) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createOperatorSets(_) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromAVS(_) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromOperatorSets(_) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorRestakedStrategies(_) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRestakeableStrategies(_) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::migrationFinalized(_) => {
                    <migrationFinalizedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proposeNewSlasher(_) => {
                    <proposeNewSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposedSlasher(_) => {
                    <proposedSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperatorToAVS(_) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeStrategiesFromOperatorSet(_) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardsInitiator(_) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAVSRegistrar(_) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRewardsInitiator(_) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setStaleStakesForbidden(_) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashOperator(_) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slasher(_) => <slasherCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::slasherProposalTimestamp(_) => {
                    <slasherProposalTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::staleStakesForbidden(_) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::trySignatureAndApkVerification(_) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateAVSMetadataURI(_) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls>] = &[
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerCalls::trySignatureAndApkVerification,
                            )
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn acceptProposedSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <acceptProposedSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::acceptProposedSlasher)
                    }
                    acceptProposedSlasher
                },
                {
                    fn getOperatorRestakedStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerCalls::getOperatorRestakedStrategies,
                            )
                    }
                    getOperatorRestakedStrategies
                },
                {
                    fn setRewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::setRewardsInitiator)
                    }
                    setRewardsInitiator
                },
                {
                    fn slashOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <slashOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::slashOperator)
                    }
                    slashOperator
                },
                {
                    fn setStaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::setStaleStakesForbidden)
                    }
                    setStaleStakesForbidden
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn SLASHER_PROPOSAL_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::SLASHER_PROPOSAL_DELAY)
                    }
                    SLASHER_PROPOSAL_DELAY
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn addStrategyToOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <addStrategyToOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::addStrategyToOperatorSet)
                    }
                    addStrategyToOperatorSet
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn createOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::createOperatorSets)
                    }
                    createOperatorSets
                },
                {
                    fn proposeNewSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <proposeNewSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::proposeNewSlasher)
                    }
                    proposeNewSlasher
                },
                {
                    fn migrationFinalized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <migrationFinalizedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::migrationFinalized)
                    }
                    migrationFinalized
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn registerOperatorToAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::registerOperatorToAVS)
                    }
                    registerOperatorToAVS
                },
                {
                    fn deregisterOperatorFromAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::deregisterOperatorFromAVS)
                    }
                    deregisterOperatorFromAVS
                },
                {
                    fn updateAVSMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::updateAVSMetadataURI)
                    }
                    updateAVSMetadataURI
                },
                {
                    fn slasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <slasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::slasher)
                    }
                    slasher
                },
                {
                    fn staleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::staleStakesForbidden)
                    }
                    staleStakesForbidden
                },
                {
                    fn deregisterOperatorFromOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerCalls::deregisterOperatorFromOperatorSets,
                            )
                    }
                    deregisterOperatorFromOperatorSets
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn removeStrategiesFromOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerCalls::removeStrategiesFromOperatorSet,
                            )
                    }
                    removeStrategiesFromOperatorSet
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn proposedSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <proposedSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::proposedSlasher)
                    }
                    proposedSlasher
                },
                {
                    fn getRestakeableStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::getRestakeableStrategies)
                    }
                    getRestakeableStrategies
                },
                {
                    fn setAVSRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::setAVSRegistrar)
                    }
                    setAVSRegistrar
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn rewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::rewardsInitiator)
                    }
                    rewardsInitiator
                },
                {
                    fn slasherProposalTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <slasherProposalTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::slasherProposalTimestamp)
                    }
                    slasherProposalTimestamp
                },
                {
                    fn createAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::createAVSRewardsSubmission)
                    }
                    createAVSRewardsSubmission
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
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::SLASHER_PROPOSAL_DELAY(inner) => {
                    <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::acceptProposedSlasher(inner) => {
                    <acceptProposedSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addStrategyToOperatorSet(inner) => {
                    <addStrategyToOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createOperatorSets(inner) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::migrationFinalized(inner) => {
                    <migrationFinalizedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proposeNewSlasher(inner) => {
                    <proposeNewSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposedSlasher(inner) => {
                    <proposedSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeStrategiesFromOperatorSet(inner) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAVSRegistrar(inner) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setStaleStakesForbidden(inner) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashOperator(inner) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::slasherProposalTimestamp(inner) => {
                    <slasherProposalTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::staleStakesForbidden(inner) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::SLASHER_PROPOSAL_DELAY(inner) => {
                    <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::acceptProposedSlasher(inner) => {
                    <acceptProposedSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addStrategyToOperatorSet(inner) => {
                    <addStrategyToOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createOperatorSets(inner) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::migrationFinalized(inner) => {
                    <migrationFinalizedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proposeNewSlasher(inner) => {
                    <proposeNewSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposedSlasher(inner) => {
                    <proposedSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::removeStrategiesFromOperatorSet(inner) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAVSRegistrar(inner) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setStaleStakesForbidden(inner) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashOperator(inner) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::slasherProposalTimestamp(inner) => {
                    <slasherProposalTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::staleStakesForbidden(inner) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`MockAvsServiceManager`](self) events.
    pub enum MockAvsServiceManagerEvents {
        Initialized(Initialized),
        OwnershipTransferred(OwnershipTransferred),
        RewardsInitiatorUpdated(RewardsInitiatorUpdated),
        SlasherProposed(SlasherProposed),
        SlasherUpdated(SlasherUpdated),
        StaleStakesForbiddenUpdate(StaleStakesForbiddenUpdate),
    }
    #[automatically_derived]
    impl MockAvsServiceManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                47u8,
                138u8,
                252u8,
                138u8,
                120u8,
                253u8,
                149u8,
                143u8,
                51u8,
                1u8,
                192u8,
                35u8,
                58u8,
                163u8,
                38u8,
                185u8,
                196u8,
                185u8,
                162u8,
                136u8,
                74u8,
                116u8,
                131u8,
                34u8,
                125u8,
                107u8,
                5u8,
                85u8,
                170u8,
                160u8,
                58u8,
                219u8,
            ],
            [
                64u8,
                228u8,
                237u8,
                136u8,
                10u8,
                41u8,
                224u8,
                246u8,
                221u8,
                206u8,
                48u8,
                116u8,
                87u8,
                251u8,
                117u8,
                205u8,
                223u8,
                79u8,
                238u8,
                247u8,
                211u8,
                236u8,
                176u8,
                48u8,
                27u8,
                253u8,
                244u8,
                151u8,
                106u8,
                14u8,
                45u8,
                252u8,
            ],
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
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                224u8,
                212u8,
                154u8,
                84u8,
                39u8,
                68u8,
                35u8,
                24u8,
                61u8,
                173u8,
                236u8,
                189u8,
                242u8,
                57u8,
                234u8,
                172u8,
                110u8,
                6u8,
                186u8,
                136u8,
                50u8,
                11u8,
                38u8,
                254u8,
                140u8,
                197u8,
                236u8,
                157u8,
                5u8,
                10u8,
                99u8,
                149u8,
            ],
            [
                225u8,
                28u8,
                221u8,
                241u8,
                129u8,
                106u8,
                67u8,
                49u8,
                140u8,
                161u8,
                117u8,
                187u8,
                197u8,
                44u8,
                208u8,
                24u8,
                84u8,
                54u8,
                233u8,
                203u8,
                234u8,
                215u8,
                200u8,
                58u8,
                204u8,
                84u8,
                167u8,
                62u8,
                70u8,
                23u8,
                23u8,
                227u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for MockAvsServiceManagerEvents {
        const NAME: &'static str = "MockAvsServiceManagerEvents";
        const COUNT: usize = 6usize;
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
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsInitiatorUpdated)
                }
                Some(<SlasherProposed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlasherProposed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlasherProposed)
                }
                Some(<SlasherUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlasherUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlasherUpdated)
                }
                Some(
                    <StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StaleStakesForbiddenUpdate)
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
    impl alloy_sol_types::private::IntoLogData for MockAvsServiceManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlasherProposed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlasherUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlasherProposed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlasherUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`MockAvsServiceManager`](self) contract instance.

See the [wrapper's documentation](`MockAvsServiceManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> MockAvsServiceManagerInstance<T, P, N> {
        MockAvsServiceManagerInstance::<T, P, N>::new(address, provider)
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
        _avsDirectory: alloy::sol_types::private::Address,
        _rewardsCoordinator: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<MockAvsServiceManagerInstance<T, P, N>>,
    > {
        MockAvsServiceManagerInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _registryCoordinator,
            _avsDirectory,
            _rewardsCoordinator,
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
        _registryCoordinator: alloy::sol_types::private::Address,
        _avsDirectory: alloy::sol_types::private::Address,
        _rewardsCoordinator: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        MockAvsServiceManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _registryCoordinator,
            _avsDirectory,
            _rewardsCoordinator,
            _allocationManager,
        )
    }
    /**A [`MockAvsServiceManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`MockAvsServiceManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct MockAvsServiceManagerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for MockAvsServiceManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MockAvsServiceManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > MockAvsServiceManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`MockAvsServiceManager`](self) contract instance.

See the [wrapper's documentation](`MockAvsServiceManagerInstance`) for more details.*/
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
            _registryCoordinator: alloy::sol_types::private::Address,
            _avsDirectory: alloy::sol_types::private::Address,
            _rewardsCoordinator: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<MockAvsServiceManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _registryCoordinator,
                _avsDirectory,
                _rewardsCoordinator,
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
            _registryCoordinator: alloy::sol_types::private::Address,
            _avsDirectory: alloy::sol_types::private::Address,
            _rewardsCoordinator: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _registryCoordinator,
                            _avsDirectory,
                            _rewardsCoordinator,
                            _allocationManager,
                        },
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
    impl<T, P: ::core::clone::Clone, N> MockAvsServiceManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> MockAvsServiceManagerInstance<T, P, N> {
            MockAvsServiceManagerInstance {
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
    > MockAvsServiceManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`SLASHER_PROPOSAL_DELAY`] function.
        pub fn SLASHER_PROPOSAL_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, SLASHER_PROPOSAL_DELAYCall, N> {
            self.call_builder(&SLASHER_PROPOSAL_DELAYCall {})
        }
        ///Creates a new call builder for the [`acceptProposedSlasher`] function.
        pub fn acceptProposedSlasher(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, acceptProposedSlasherCall, N> {
            self.call_builder(&acceptProposedSlasherCall {})
        }
        ///Creates a new call builder for the [`addStrategyToOperatorSet`] function.
        pub fn addStrategyToOperatorSet(
            &self,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStrategyToOperatorSetCall, N> {
            self.call_builder(
                &addStrategyToOperatorSetCall {
                    operatorSetId,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`blsApkRegistry`] function.
        pub fn blsApkRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, blsApkRegistryCall, N> {
            self.call_builder(&blsApkRegistryCall {})
        }
        ///Creates a new call builder for the [`checkSignatures`] function.
        pub fn checkSignatures(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
            referenceBlockNumber: u32,
            params: <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkSignaturesCall, N> {
            self.call_builder(
                &checkSignaturesCall {
                    msgHash,
                    quorumNumbers,
                    referenceBlockNumber,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`createAVSRewardsSubmission`] function.
        pub fn createAVSRewardsSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createAVSRewardsSubmissionCall, N> {
            self.call_builder(
                &createAVSRewardsSubmissionCall {
                    rewardsSubmissions,
                },
            )
        }
        ///Creates a new call builder for the [`createOperatorSets`] function.
        pub fn createOperatorSets(
            &self,
            params: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorSetsCall, N> {
            self.call_builder(&createOperatorSetsCall { params })
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`deregisterOperatorFromAVS`] function.
        pub fn deregisterOperatorFromAVS(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromAVSCall, N> {
            self.call_builder(
                &deregisterOperatorFromAVSCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`deregisterOperatorFromOperatorSets`] function.
        pub fn deregisterOperatorFromOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            deregisterOperatorFromOperatorSetsCall,
            N,
        > {
            self.call_builder(
                &deregisterOperatorFromOperatorSetsCall {
                    operator,
                    operatorSetIds,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorRestakedStrategies`] function.
        pub fn getOperatorRestakedStrategies(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getOperatorRestakedStrategiesCall,
            N,
        > {
            self.call_builder(
                &getOperatorRestakedStrategiesCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getRestakeableStrategies`] function.
        pub fn getRestakeableStrategies(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRestakeableStrategiesCall, N> {
            self.call_builder(&getRestakeableStrategiesCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _initialOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall { _initialOwner })
        }
        ///Creates a new call builder for the [`migrationFinalized`] function.
        pub fn migrationFinalized(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, migrationFinalizedCall, N> {
            self.call_builder(&migrationFinalizedCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`proposeNewSlasher`] function.
        pub fn proposeNewSlasher(
            &self,
            newSlasher: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposeNewSlasherCall, N> {
            self.call_builder(
                &proposeNewSlasherCall {
                    newSlasher,
                },
            )
        }
        ///Creates a new call builder for the [`proposedSlasher`] function.
        pub fn proposedSlasher(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposedSlasherCall, N> {
            self.call_builder(&proposedSlasherCall {})
        }
        ///Creates a new call builder for the [`registerOperatorToAVS`] function.
        pub fn registerOperatorToAVS(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorToAVSCall, N> {
            self.call_builder(
                &registerOperatorToAVSCall {
                    operator,
                    operatorSignature,
                },
            )
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`removeStrategiesFromOperatorSet`] function.
        pub fn removeStrategiesFromOperatorSet(
            &self,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            removeStrategiesFromOperatorSetCall,
            N,
        > {
            self.call_builder(
                &removeStrategiesFromOperatorSetCall {
                    operatorSetId,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`rewardsInitiator`] function.
        pub fn rewardsInitiator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, rewardsInitiatorCall, N> {
            self.call_builder(&rewardsInitiatorCall {})
        }
        ///Creates a new call builder for the [`setAVSRegistrar`] function.
        pub fn setAVSRegistrar(
            &self,
            registrar: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAVSRegistrarCall, N> {
            self.call_builder(&setAVSRegistrarCall { registrar })
        }
        ///Creates a new call builder for the [`setRewardsInitiator`] function.
        pub fn setRewardsInitiator(
            &self,
            newRewardsInitiator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsInitiatorCall, N> {
            self.call_builder(
                &setRewardsInitiatorCall {
                    newRewardsInitiator,
                },
            )
        }
        ///Creates a new call builder for the [`setStaleStakesForbidden`] function.
        pub fn setStaleStakesForbidden(
            &self,
            value: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setStaleStakesForbiddenCall, N> {
            self.call_builder(
                &setStaleStakesForbiddenCall {
                    value,
                },
            )
        }
        ///Creates a new call builder for the [`slashOperator`] function.
        pub fn slashOperator(
            &self,
            params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashOperatorCall, N> {
            self.call_builder(&slashOperatorCall { params })
        }
        ///Creates a new call builder for the [`slasher`] function.
        pub fn slasher(&self) -> alloy_contract::SolCallBuilder<T, &P, slasherCall, N> {
            self.call_builder(&slasherCall {})
        }
        ///Creates a new call builder for the [`slasherProposalTimestamp`] function.
        pub fn slasherProposalTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, slasherProposalTimestampCall, N> {
            self.call_builder(&slasherProposalTimestampCall {})
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`staleStakesForbidden`] function.
        pub fn staleStakesForbidden(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, staleStakesForbiddenCall, N> {
            self.call_builder(&staleStakesForbiddenCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`trySignatureAndApkVerification`] function.
        pub fn trySignatureAndApkVerification(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            trySignatureAndApkVerificationCall,
            N,
        > {
            self.call_builder(
                &trySignatureAndApkVerificationCall {
                    msgHash,
                    apk,
                    apkG2,
                    sigma,
                },
            )
        }
        ///Creates a new call builder for the [`updateAVSMetadataURI`] function.
        pub fn updateAVSMetadataURI(
            &self,
            _metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(
                &updateAVSMetadataURICall {
                    _metadataURI,
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
    > MockAvsServiceManagerInstance<T, P, N> {
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
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`RewardsInitiatorUpdated`] event.
        pub fn RewardsInitiatorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsInitiatorUpdated, N> {
            self.event_filter::<RewardsInitiatorUpdated>()
        }
        ///Creates a new event filter for the [`SlasherProposed`] event.
        pub fn SlasherProposed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlasherProposed, N> {
            self.event_filter::<SlasherProposed>()
        }
        ///Creates a new event filter for the [`SlasherUpdated`] event.
        pub fn SlasherUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlasherUpdated, N> {
            self.event_filter::<SlasherUpdated>()
        }
        ///Creates a new event filter for the [`StaleStakesForbiddenUpdate`] event.
        pub fn StaleStakesForbiddenUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StaleStakesForbiddenUpdate, N> {
            self.event_filter::<StaleStakesForbiddenUpdate>()
        }
    }
}
