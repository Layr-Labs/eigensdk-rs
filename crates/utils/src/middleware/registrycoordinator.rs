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
        #[allow(missing_docs)]
        pub X: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.X,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.Y,
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
        impl alloy_sol_types::SolType for G1Point {
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
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 X,uint256 Y)")
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
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
    struct G2Point { uint256[2] X; uint256[2] Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G2Point {
        #[allow(missing_docs)]
        pub X: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        #[allow(missing_docs)]
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
            alloy::sol_types::sol_data::FixedArray<alloy::sol_types::sol_data::Uint<256>, 2usize>,
            alloy::sol_types::sol_data::FixedArray<alloy::sol_types::sol_data::Uint<256>, 2usize>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
        impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
            fn from(value: G2Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
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
        impl alloy_sol_types::SolType for G2Point {
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
        impl alloy_sol_types::SolStruct for G2Point {
            const NAME: &'static str = "G2Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G2Point(uint256[2] X,uint256[2] Y)")
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
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.X
                    )
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.Y
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.X, out
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.Y, out
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
        > BN254Instance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

        See the [wrapper's documentation](`BN254Instance`) for more details.*/
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
        > BN254Instance<T, P, N>
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
        > BN254Instance<T, P, N>
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
library IBLSApkRegistry {
    struct PubkeyRegistrationParams { BN254.G1Point pubkeyRegistrationSignature; BN254.G1Point pubkeyG1; BN254.G2Point pubkeyG2; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IBLSApkRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct PubkeyRegistrationParams { BN254.G1Point pubkeyRegistrationSignature; BN254.G1Point pubkeyG1; BN254.G2Point pubkeyG2; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PubkeyRegistrationParams {
        #[allow(missing_docs)]
        pub pubkeyRegistrationSignature: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pubkeyG1: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pubkeyG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (BN254::G1Point, BN254::G1Point, BN254::G2Point);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G2Point as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<PubkeyRegistrationParams> for UnderlyingRustTuple<'_> {
            fn from(value: PubkeyRegistrationParams) -> Self {
                (
                    value.pubkeyRegistrationSignature,
                    value.pubkeyG1,
                    value.pubkeyG2,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PubkeyRegistrationParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    pubkeyRegistrationSignature: tuple.0,
                    pubkeyG1: tuple.1,
                    pubkeyG2: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PubkeyRegistrationParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PubkeyRegistrationParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(
                        &self.pubkeyRegistrationSignature,
                    ),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.pubkeyG1),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.pubkeyG2),
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
        impl alloy_sol_types::SolType for PubkeyRegistrationParams {
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
        impl alloy_sol_types::SolStruct for PubkeyRegistrationParams {
            const NAME: &'static str = "PubkeyRegistrationParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PubkeyRegistrationParams(BN254.G1Point pubkeyRegistrationSignature,BN254.G1Point pubkeyG1,BN254.G2Point pubkeyG2)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(3);
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                        &self.pubkeyRegistrationSignature,
                    )
                    .0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.pubkeyG1)
                        .0,
                    <BN254::G2Point as alloy_sol_types::SolType>::eip712_data_word(&self.pubkeyG2)
                        .0,
                ]
                .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PubkeyRegistrationParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pubkeyRegistrationSignature,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pubkeyG1,
                    )
                    + <BN254::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pubkeyG2,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pubkeyRegistrationSignature,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pubkeyG1,
                    out,
                );
                <BN254::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pubkeyG2,
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
    /**Creates a new wrapper around an on-chain [`IBLSApkRegistry`](self) contract instance.

    See the [wrapper's documentation](`IBLSApkRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBLSApkRegistryInstance<T, P, N> {
        IBLSApkRegistryInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IBLSApkRegistry`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IBLSApkRegistry`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBLSApkRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBLSApkRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBLSApkRegistryInstance")
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
        > IBLSApkRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IBLSApkRegistry`](self) contract instance.

        See the [wrapper's documentation](`IBLSApkRegistryInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IBLSApkRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBLSApkRegistryInstance<T, P, N> {
            IBLSApkRegistryInstance {
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
        > IBLSApkRegistryInstance<T, P, N>
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
        > IBLSApkRegistryInstance<T, P, N>
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
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::private::SolTypeValue<Self> for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            f.debug_tuple("ISignatureUtilsInstance")
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
        > ISignatureUtilsInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

        See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
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
        > ISignatureUtilsInstance<T, P, N>
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
        > ISignatureUtilsInstance<T, P, N>
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
library ISlashingRegistryCoordinator {
    type OperatorStatus is uint8;
    struct OperatorInfo { bytes32 operatorId; OperatorStatus status; }
    struct OperatorKickParam { uint8 quorumNumber; address operator; }
    struct OperatorSetParam { uint32 maxOperatorCount; uint16 kickBIPsOfOperatorStake; uint16 kickBIPsOfTotalStake; }
    struct QuorumBitmapUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint192 quorumBitmap; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISlashingRegistryCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<OperatorStatus> for u8 {
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
        impl OperatorStatus {
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
        impl alloy_sol_types::SolType for OperatorStatus {
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
        impl alloy_sol_types::EventTopic for OperatorStatus {
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
    struct OperatorInfo { bytes32 operatorId; OperatorStatus status; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorInfo {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub status: <OperatorStatus as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>, OperatorStatus);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            <OperatorStatus as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<OperatorInfo> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorInfo) -> Self {
                (value.operatorId, value.status)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorId: tuple.0,
                    status: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <OperatorStatus as alloy_sol_types::SolType>::tokenize(&self.status),
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
        impl alloy_sol_types::SolType for OperatorInfo {
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
        impl alloy_sol_types::SolStruct for OperatorInfo {
            const NAME: &'static str = "OperatorInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorInfo(bytes32 operatorId,uint8 status)",
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorId)
                        .0,
                    <OperatorStatus as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorId,
                    )
                    + <OperatorStatus as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorId,
                    out,
                );
                <OperatorStatus as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
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
    struct OperatorKickParam { uint8 quorumNumber; address operator; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorKickParam {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<OperatorKickParam> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorKickParam) -> Self {
                (value.quorumNumber, value.operator)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorKickParam {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    quorumNumber: tuple.0,
                    operator: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorKickParam {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorKickParam {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
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
        impl alloy_sol_types::SolType for OperatorKickParam {
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
        impl alloy_sol_types::SolStruct for OperatorKickParam {
            const NAME: &'static str = "OperatorKickParam";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorKickParam(uint8 quorumNumber,address operator)",
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
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumNumber)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operator,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorKickParam {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumNumber,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
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
    struct OperatorSetParam { uint32 maxOperatorCount; uint16 kickBIPsOfOperatorStake; uint16 kickBIPsOfTotalStake; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSetParam {
        #[allow(missing_docs)]
        pub maxOperatorCount: u32,
        #[allow(missing_docs)]
        pub kickBIPsOfOperatorStake: u16,
        #[allow(missing_docs)]
        pub kickBIPsOfTotalStake: u16,
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
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Uint<16>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, u16, u16);
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
        impl ::core::convert::From<OperatorSetParam> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSetParam) -> Self {
                (
                    value.maxOperatorCount,
                    value.kickBIPsOfOperatorStake,
                    value.kickBIPsOfTotalStake,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSetParam {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxOperatorCount: tuple.0,
                    kickBIPsOfOperatorStake: tuple.1,
                    kickBIPsOfTotalStake: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorSetParam {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorSetParam {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.maxOperatorCount,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.kickBIPsOfOperatorStake,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.kickBIPsOfTotalStake,
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
        impl alloy_sol_types::SolType for OperatorSetParam {
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
        impl alloy_sol_types::SolStruct for OperatorSetParam {
            const NAME: &'static str = "OperatorSetParam";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorSetParam(uint32 maxOperatorCount,uint16 kickBIPsOfOperatorStake,uint16 kickBIPsOfTotalStake)",
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
                            &self.maxOperatorCount,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.kickBIPsOfOperatorStake,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.kickBIPsOfTotalStake,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorSetParam {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxOperatorCount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kickBIPsOfOperatorStake,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kickBIPsOfTotalStake,
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
                    &rust.maxOperatorCount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kickBIPsOfOperatorStake,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kickBIPsOfTotalStake,
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
    struct QuorumBitmapUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint192 quorumBitmap; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumBitmapUpdate {
        #[allow(missing_docs)]
        pub updateBlockNumber: u32,
        #[allow(missing_docs)]
        pub nextUpdateBlockNumber: u32,
        #[allow(missing_docs)]
        pub quorumBitmap: alloy::sol_types::private::primitives::aliases::U192,
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
            alloy::sol_types::sol_data::Uint<192>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            u32,
            alloy::sol_types::private::primitives::aliases::U192,
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
        impl ::core::convert::From<QuorumBitmapUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumBitmapUpdate) -> Self {
                (
                    value.updateBlockNumber,
                    value.nextUpdateBlockNumber,
                    value.quorumBitmap,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumBitmapUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    updateBlockNumber: tuple.0,
                    nextUpdateBlockNumber: tuple.1,
                    quorumBitmap: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumBitmapUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumBitmapUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.updateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.nextUpdateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<192> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumBitmap,
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
        impl alloy_sol_types::SolType for QuorumBitmapUpdate {
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
        impl alloy_sol_types::SolStruct for QuorumBitmapUpdate {
            const NAME: &'static str = "QuorumBitmapUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumBitmapUpdate(uint32 updateBlockNumber,uint32 nextUpdateBlockNumber,uint192 quorumBitmap)",
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
                        192,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumBitmap)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumBitmapUpdate {
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
                        192,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumBitmap,
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
                    192,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumBitmap,
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
    /**Creates a new wrapper around an on-chain [`ISlashingRegistryCoordinator`](self) contract instance.

    See the [wrapper's documentation](`ISlashingRegistryCoordinatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISlashingRegistryCoordinatorInstance<T, P, N> {
        ISlashingRegistryCoordinatorInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISlashingRegistryCoordinator`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISlashingRegistryCoordinator`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISlashingRegistryCoordinatorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISlashingRegistryCoordinatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISlashingRegistryCoordinatorInstance")
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
        > ISlashingRegistryCoordinatorInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISlashingRegistryCoordinator`](self) contract instance.

        See the [wrapper's documentation](`ISlashingRegistryCoordinatorInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ISlashingRegistryCoordinatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISlashingRegistryCoordinatorInstance<T, P, N> {
            ISlashingRegistryCoordinatorInstance {
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
        > ISlashingRegistryCoordinatorInstance<T, P, N>
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
        > ISlashingRegistryCoordinatorInstance<T, P, N>
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
library IStakeRegistry {
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
pub mod IStakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct StrategyParams { address strategy; uint96 multiplier; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyParams {
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
    /**Creates a new wrapper around an on-chain [`IStakeRegistry`](self) contract instance.

    See the [wrapper's documentation](`IStakeRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IStakeRegistryInstance<T, P, N> {
        IStakeRegistryInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IStakeRegistry`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IStakeRegistry`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IStakeRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IStakeRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IStakeRegistryInstance")
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
        > IStakeRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IStakeRegistry`](self) contract instance.

        See the [wrapper's documentation](`IStakeRegistryInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IStakeRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IStakeRegistryInstance<T, P, N> {
            IStakeRegistryInstance {
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
        > IStakeRegistryInstance<T, P, N>
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
        > IStakeRegistryInstance<T, P, N>
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

library IBLSApkRegistry {
    struct PubkeyRegistrationParams {
        BN254.G1Point pubkeyRegistrationSignature;
        BN254.G1Point pubkeyG1;
        BN254.G2Point pubkeyG2;
    }
}

library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

library ISlashingRegistryCoordinator {
    type OperatorStatus is uint8;
    struct OperatorInfo {
        bytes32 operatorId;
        OperatorStatus status;
    }
    struct OperatorKickParam {
        uint8 quorumNumber;
        address operator;
    }
    struct OperatorSetParam {
        uint32 maxOperatorCount;
        uint16 kickBIPsOfOperatorStake;
        uint16 kickBIPsOfTotalStake;
    }
    struct QuorumBitmapUpdate {
        uint32 updateBlockNumber;
        uint32 nextUpdateBlockNumber;
        uint192 quorumBitmap;
    }
}

library IStakeRegistry {
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }
}

interface RegistryCoordinator {
    error AlreadyRegisteredForQuorums();
    error BitmapCannotBeZero();
    error BitmapEmpty();
    error BitmapUpdateIsAfterBlockNumber();
    error BitmapValueTooLarge();
    error BytesArrayLengthTooLong();
    error BytesArrayNotOrdered();
    error CannotChurnSelf();
    error CannotKickOperatorAboveThreshold();
    error CannotReregisterYet();
    error ChurnApproverSaltUsed();
    error CurrentlyPaused();
    error ExpModFailed();
    error InputAddressZero();
    error InputLengthMismatch();
    error InsufficientStakeForChurn();
    error InvalidNewPausedStatus();
    error InvalidRegistrationType();
    error InvalidSignature();
    error M2QuorumsAlreadyDisabled();
    error MaxQuorumsReached();
    error NextBitmapUpdateIsBeforeBlockNumber();
    error NotRegistered();
    error NotRegisteredForQuorum();
    error NotSorted();
    error OnlyAllocationManager();
    error OnlyEjector();
    error OnlyPauser();
    error OnlyUnpauser();
    error OperatorSetsAlreadyEnabled();
    error OperatorSetsNotEnabled();
    error QuorumDoesNotExist();
    error QuorumOperatorCountMismatch();
    error RegistryCoordinatorSignatureExpired();
    error SaltAlreadyUsed();
    error SignatureExpired();

    event ChurnApproverUpdated(address prevChurnApprover, address newChurnApprover);
    event EjectorUpdated(address prevEjector, address newEjector);
    event Initialized(uint8 version);
    event M2QuorumsDisabled();
    event OperatorDeregistered(address indexed operator, bytes32 indexed operatorId);
    event OperatorRegistered(address indexed operator, bytes32 indexed operatorId);
    event OperatorSetParamsUpdated(uint8 indexed quorumNumber, ISlashingRegistryCoordinator.OperatorSetParam operatorSetParams);
    event OperatorSetsEnabled();
    event OperatorSocketUpdate(bytes32 indexed operatorId, string socket);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event QuorumBlockNumberUpdated(uint8 indexed quorumNumber, uint256 blocknumber);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _serviceManager, address _stakeRegistry, address _blsApkRegistry, address _indexRegistry, address _allocationManager, address _pauserRegistry);

    function OPERATOR_CHURN_APPROVAL_TYPEHASH() external view returns (bytes32);
    function PUBKEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
    function accountIdentifier() external view returns (address);
    function allocationManager() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function calculateOperatorChurnApprovalDigestHash(address registeringOperator, bytes32 registeringOperatorId, ISlashingRegistryCoordinator.OperatorKickParam[] memory operatorKickParams, bytes32 salt, uint256 expiry) external view returns (bytes32);
    function churnApprover() external view returns (address);
    function createSlashableStakeQuorum(ISlashingRegistryCoordinator.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory strategyParams, uint32 lookAheadPeriod) external;
    function createTotalDelegatedStakeQuorum(ISlashingRegistryCoordinator.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory strategyParams) external;
    function deregisterOperator(address operator, uint32[] memory operatorSetIds) external;
    function deregisterOperator(bytes memory quorumNumbers) external;
    function disableM2QuorumRegistration() external;
    function ejectOperator(address operator, bytes memory quorumNumbers) external;
    function ejectionCooldown() external view returns (uint256);
    function ejector() external view returns (address);
    function enableOperatorSets() external;
    function getCurrentQuorumBitmap(bytes32 operatorId) external view returns (uint192);
    function getOperator(address operator) external view returns (ISlashingRegistryCoordinator.OperatorInfo memory);
    function getOperatorFromId(bytes32 operatorId) external view returns (address);
    function getOperatorId(address operator) external view returns (bytes32);
    function getOperatorSetParams(uint8 quorumNumber) external view returns (ISlashingRegistryCoordinator.OperatorSetParam memory);
    function getOperatorStatus(address operator) external view returns (ISlashingRegistryCoordinator.OperatorStatus);
    function getQuorumBitmapAtBlockNumberByIndex(bytes32 operatorId, uint32 blockNumber, uint256 index) external view returns (uint192);
    function getQuorumBitmapHistoryLength(bytes32 operatorId) external view returns (uint256);
    function getQuorumBitmapIndicesAtBlockNumber(uint32 blockNumber, bytes32[] memory operatorIds) external view returns (uint32[] memory);
    function getQuorumBitmapUpdateByIndex(bytes32 operatorId, uint256 index) external view returns (ISlashingRegistryCoordinator.QuorumBitmapUpdate memory);
    function indexRegistry() external view returns (address);
    function initialize(address _initialOwner, address _churnApprover, address _ejector, uint256 _initialPausedStatus, address _accountIdentifier) external;
    function isChurnApproverSaltUsed(bytes32) external view returns (bool);
    function isM2Quorum(uint8 quorumNumber) external view returns (bool);
    function lastEjectionTimestamp(address) external view returns (uint256);
    function m2QuorumsDisabled() external view returns (bool);
    function numRegistries() external view returns (uint256);
    function operatorSetsEnabled() external view returns (bool);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function pubkeyRegistrationMessageHash(address operator) external view returns (BN254.G1Point memory);
    function quorumCount() external view returns (uint8);
    function quorumUpdateBlockNumber(uint8) external view returns (uint256);
    function registerOperator(bytes memory quorumNumbers, string memory socket, IBLSApkRegistry.PubkeyRegistrationParams memory params, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function registerOperator(address operator, uint32[] memory operatorSetIds, bytes memory data) external;
    function registerOperatorWithChurn(bytes memory quorumNumbers, string memory socket, IBLSApkRegistry.PubkeyRegistrationParams memory params, ISlashingRegistryCoordinator.OperatorKickParam[] memory operatorKickParams, ISignatureUtils.SignatureWithSaltAndExpiry memory churnApproverSignature, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function registries(uint256) external view returns (address);
    function renounceOwnership() external;
    function serviceManager() external view returns (address);
    function setAccountIdentifier(address _accountIdentifier) external;
    function setChurnApprover(address _churnApprover) external;
    function setEjectionCooldown(uint256 _ejectionCooldown) external;
    function setEjector(address _ejector) external;
    function setOperatorSetParams(uint8 quorumNumber, ISlashingRegistryCoordinator.OperatorSetParam memory operatorSetParams) external;
    function stakeRegistry() external view returns (address);
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
    function updateOperators(address[] memory operators) external;
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory quorumNumbers) external;
    function updateSocket(string memory socket) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "contract IServiceManager"
      },
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      },
      {
        "name": "_blsApkRegistry",
        "type": "address",
        "internalType": "contract IBLSApkRegistry"
      },
      {
        "name": "_indexRegistry",
        "type": "address",
        "internalType": "contract IIndexRegistry"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      },
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "OPERATOR_CHURN_APPROVAL_TYPEHASH",
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
    "name": "PUBKEY_REGISTRATION_TYPEHASH",
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
    "name": "accountIdentifier",
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
    "name": "calculateOperatorChurnApprovalDigestHash",
    "inputs": [
      {
        "name": "registeringOperator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "registeringOperatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "operatorKickParams",
        "type": "tuple[]",
        "internalType": "struct ISlashingRegistryCoordinator.OperatorKickParam[]",
        "components": [
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
        ]
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
    ],
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
    "name": "churnApprover",
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
    "name": "createSlashableStakeQuorum",
    "inputs": [
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinator.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistry.StrategyParams[]",
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
        "name": "lookAheadPeriod",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createTotalDelegatedStakeQuorum",
    "inputs": [
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinator.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistry.StrategyParams[]",
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
    "name": "deregisterOperator",
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
    "name": "deregisterOperator",
    "inputs": [
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
    "name": "disableM2QuorumRegistration",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "ejectOperator",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
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
    "name": "ejectionCooldown",
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
    "name": "ejector",
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
    "name": "enableOperatorSets",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getCurrentQuorumBitmap",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperator",
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
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinator.OperatorInfo",
        "components": [
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum ISlashingRegistryCoordinator.OperatorStatus"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorFromId",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
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
    "name": "getOperatorId",
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
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorSetParams",
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
        "internalType": "struct ISlashingRegistryCoordinator.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorStatus",
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
        "type": "uint8",
        "internalType": "enum ISlashingRegistryCoordinator.OperatorStatus"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapAtBlockNumberByIndex",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
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
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapHistoryLength",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "getQuorumBitmapIndicesAtBlockNumber",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
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
    "name": "getQuorumBitmapUpdateByIndex",
    "inputs": [
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
        "internalType": "struct ISlashingRegistryCoordinator.QuorumBitmapUpdate",
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
            "name": "quorumBitmap",
            "type": "uint192",
            "internalType": "uint192"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "indexRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IIndexRegistry"
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
      },
      {
        "name": "_churnApprover",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_ejector",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_initialPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_accountIdentifier",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isChurnApproverSaltUsed",
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
    "name": "isM2Quorum",
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
    "name": "lastEjectionTimestamp",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
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
    "name": "m2QuorumsDisabled",
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
    "name": "numRegistries",
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
    "name": "operatorSetsEnabled",
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
    "name": "pause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pauseAll",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [
      {
        "name": "index",
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
    "name": "paused",
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
    "name": "pauserRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pubkeyRegistrationMessageHash",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorumCount",
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
    "name": "quorumUpdateBlockNumber",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerOperator",
    "inputs": [
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "socket",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IBLSApkRegistry.PubkeyRegistrationParams",
        "components": [
          {
            "name": "pubkeyRegistrationSignature",
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
            "name": "pubkeyG1",
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
            "name": "pubkeyG2",
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
          }
        ]
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
    "name": "registerOperator",
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
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerOperatorWithChurn",
    "inputs": [
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "socket",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IBLSApkRegistry.PubkeyRegistrationParams",
        "components": [
          {
            "name": "pubkeyRegistrationSignature",
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
            "name": "pubkeyG1",
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
            "name": "pubkeyG2",
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
          }
        ]
      },
      {
        "name": "operatorKickParams",
        "type": "tuple[]",
        "internalType": "struct ISlashingRegistryCoordinator.OperatorKickParam[]",
        "components": [
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
        ]
      },
      {
        "name": "churnApproverSignature",
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
    "name": "registries",
    "inputs": [
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
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
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
    "name": "serviceManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IServiceManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setAccountIdentifier",
    "inputs": [
      {
        "name": "_accountIdentifier",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setChurnApprover",
    "inputs": [
      {
        "name": "_churnApprover",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setEjectionCooldown",
    "inputs": [
      {
        "name": "_ejectionCooldown",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setEjector",
    "inputs": [
      {
        "name": "_ejector",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setOperatorSetParams",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinator.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "unpause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperators",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorsForQuorum",
    "inputs": [
      {
        "name": "operatorsPerQuorum",
        "type": "address[][]",
        "internalType": "address[][]"
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
    "name": "updateSocket",
    "inputs": [
      {
        "name": "socket",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "ChurnApproverUpdated",
    "inputs": [
      {
        "name": "prevChurnApprover",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newChurnApprover",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EjectorUpdated",
    "inputs": [
      {
        "name": "prevEjector",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newEjector",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
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
    "name": "M2QuorumsDisabled",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorDeregistered",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSetParamsUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct ISlashingRegistryCoordinator.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSetsEnabled",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSocketUpdate",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "socket",
        "type": "string",
        "indexed": false,
        "internalType": "string"
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
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumBlockNumberUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "blocknumber",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AlreadyRegisteredForQuorums",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapCannotBeZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapEmpty",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapUpdateIsAfterBlockNumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapValueTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayLengthTooLong",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayNotOrdered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotChurnSelf",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotKickOperatorAboveThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotReregisterYet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ChurnApproverSaltUsed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpModFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputAddressZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientStakeForChurn",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidRegistrationType",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "M2QuorumsAlreadyDisabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MaxQuorumsReached",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NextBitmapUpdateIsBeforeBlockNumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotRegisteredForQuorum",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyAllocationManager",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEjector",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyPauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyUnpauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorSetsAlreadyEnabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorSetsNotEnabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumDoesNotExist",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumOperatorCountMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "RegistryCoordinatorSignatureExpired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SaltAlreadyUsed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SignatureExpired",
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
pub mod RegistryCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610200604052348015610010575f5ffd5b50604051615e37380380615e3783398101604081905261002f91610293565b604080518082018252601681527f4156535265676973747279436f6f7264696e61746f720000000000000000000060208083019182528351808501909452600684526576302e302e3160d01b908401528151902060e08190527f6bda7e3f385e48841048390444cced5cc795af87758af67622e5f4f0882c4a996101008190524660a05288938893889388938893889388938893889388939290917f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6101398184846040805160208101859052908101839052606081018290524660808201523060a08201525f9060c0016040516020818303038152906040528051906020012090509392505050565b6080523060c052610120525050506001600160a01b0382169050610170576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b0390811661014052938416610180529183166101605282166101a052166101c0526101a06101bf565b5050506001600160a01b039097166101e0525061031695505050505050565b5f54610100900460ff161561022a5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff908116101561027a575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b0381168114610290575f5ffd5b50565b5f5f5f5f5f5f60c087890312156102a8575f5ffd5b86516102b38161027c565b60208801519096506102c48161027c565b60408801519095506102d58161027c565b60608801519094506102e68161027c565b60808801519093506102f78161027c565b60a08801519092506103088161027c565b809150509295509295509295565b60805160a05160c05160e05161010051610120516101405161016051610180516101a0516101c0516101e0516159da61045d5f395f81816105ca0152818161186901528181611b070152613ad801525f81816108e60152818161274c01528181612f2c01526132f301525f81816107ea01528181610f9f0152818161138e0152818161290801528181612db3015261364f01525f81816106e60152818161131b015281816121b8015281816127ec0152818161288701528181612d38015281816135ad0152613c6301525f81816106ac01528181610d55015281816113590152818161297d01528181612cbf01528181612fb50152818161302c015261353401525f818161077601528181610bdb0152818161149c0152611f9101525f6138ae01525f6138fd01525f6138d801525f61383101525f61385b01525f61388501526159da5ff3fe608060405234801561000f575f5ffd5b5060043610610392575f3560e01c806368304835116101df578063a4d7871f11610109578063ca4f2d97116100a9578063ee31882111610079578063ee318821146109b2578063f2fde38b146109ba578063fabc1cbc146109cd578063fd39105a146109e0575f5ffd5b8063ca4f2d97146108ce578063ca8aa7c7146108e1578063d72d8dd614610908578063e65797ad14610910575f5ffd5b8063adcf73f7116100e4578063adcf73f714610862578063b2d8678d14610875578063c391425e14610887578063ca0de882146108a7575f5ffd5b8063a4d7871f14610833578063a50857bf14610846578063a96f783e14610859575f5ffd5b8063871ef0491161017f5780639b5d177b1161014f5780639b5d177b146107bf5780639d8e0c23146107d25780639e9923c2146107e55780639feab8591461080c575f5ffd5b8063871ef0491461075e578063886f1195146107715780638da5cb5b146107985780639aa1653d146107a0575f5ffd5b8063733b7507116101ba578063733b75071461072357806381f936d21461072b5780638281ab751461073857806384ca52131461074b575f5ffd5b806368304835146106e15780636e3b17db14610708578063715018a61461071b575f5ffd5b8063296bb064116102c0578063530b97a4116102605780635b0b829f116102305780635b0b829f1461068c5780635c975abb1461069f5780635df45946146106a75780636347c900146106ce575f5ffd5b8063530b97a4146106325780635865c60c14610645578063595c6a67146106655780635ac86ab71461066d575f5ffd5b80633998fdd31161029b5780633998fdd3146105c55780633c2a7f4c146105ec5780633eef3a511461060c5780635140a5481461061f575f5ffd5b8063296bb0641461058c57806329d1e0c31461059f5780632cdd1e86146105b2575f5ffd5b8063125e0584116103365780631478851f116103065780631478851f146104df5780631eb812da14610511578063249a0c421461055a57806328f61b3114610579575f5ffd5b8063125e05841461047257806313542a4e14610491578063136439dd146104b9578063143e5915146104cc575f5ffd5b8063054310e611610371578063054310e6146104085780630764cb93146104335780630cf4b7671461044c5780630d3f21341461045f575f5ffd5b8062cf2ab51461039657806303fd3492146103ab57806304ec6351146103dd575b5f5ffd5b6103a96103a436600461453a565b610a1b565b005b6103ca6103b936600461456b565b5f9081526098602052604090205490565b6040519081526020015b60405180910390f35b6103f06103eb366004614593565b610b10565b6040516001600160c01b0390911681526020016103d4565b609d5461041b906001600160a01b031681565b6040516001600160a01b0390911681526020016103d4565b60a15461041b906201000090046001600160a01b031681565b6103a961045a366004614639565b610b28565b6103a961046d36600461456b565b610bb9565b6103ca61048036600461466a565b609f6020525f908152604090205481565b6103ca61049f36600461466a565b6001600160a01b03165f9081526099602052604090205490565b6103a96104c736600461456b565b610bc6565b6103a96104da36600461466a565b610c9b565b6105016104ed36600461456b565b609a6020525f908152604090205460ff1681565b60405190151581526020016103d4565b61052461051f366004614685565b610caf565b60408051825163ffffffff908116825260208085015190911690820152918101516001600160c01b0316908201526060016103d4565b6103ca6105683660046146ba565b609b6020525f908152604090205481565b609e5461041b906001600160a01b031681565b61041b61059a36600461456b565b610d3d565b6103a96105ad36600461466a565b610dc6565b6103a96105c036600461466a565b610dd7565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b6105ff6105fa36600461466a565b610de8565b6040516103d491906146d3565b6103a961061a3660046147f4565b610e66565b6103a961062d3660046148a0565b610ea5565b6103a961064036600461497d565b6111ef565b61065861065336600461466a565b611415565b6040516103d49190614a15565b6103a9611487565b61050161067b3660046146ba565b6001805460ff9092161b9081161490565b6103a961069a366004614a30565b611536565b6001546103ca565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b61041b6106dc36600461456b565b611552565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b6103a9610716366004614a62565b61157a565b6103a961163c565b6103a961164d565b60a1546105019060ff1681565b6103a9610746366004614aae565b6116b1565b6103ca610759366004614b9e565b6116c6565b6103f061076c36600461456b565b61170f565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b61041b611719565b6096546107ad9060ff1681565b60405160ff90911681526020016103d4565b6103a96107cd366004614d59565b611731565b6103a96107e0366004614eb5565b611906565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b6103ca7f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de681565b6105016108413660046146ba565b61196e565b6103a9610854366004614ef7565b611978565b6103ca60a05481565b6103a9610870366004614f96565b611ba2565b60a15461050190610100900460ff1681565b61089a61089536600461500d565b611e05565b6040516103d491906150b2565b6103ca7f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a81565b6103a96108dc366004614639565b611e13565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b609c546103ca565b61097e61091e3660046146ba565b60408051606080820183525f808352602080840182905292840181905260ff9490941684526097825292829020825193840183525463ffffffff8116845261ffff600160201b8204811692850192909252600160301b9004169082015290565b60408051825163ffffffff16815260208084015161ffff9081169183019190915292820151909216908201526060016103d4565b6103a9611ea4565b6103a96109c836600461466a565b611f19565b6103a96109db36600461456b565b611f8f565b610a0e6109ee36600461466a565b6001600160a01b03165f9081526099602052604090206001015460ff1690565b6040516103d491906150ef565b600154600290600490811603610a445760405163840a48d560e01b815260040160405180910390fd5b5f5b8251811015610b0b575f838281518110610a6257610a626150fd565b6020908102919091018101516001600160a01b0381165f9081526099835260408082208151808301909252805482526001810154939550919390929083019060ff166002811115610ab557610ab56149e1565b6002811115610ac657610ac66149e1565b90525080519091505f610ad8826120a6565b90505f610aed826001600160c01b03166120b2565b9050610afa85858361217b565b505060019093019250610a46915050565b505050565b5f610b1e609885858561225d565b90505b9392505050565b6001335f9081526099602052604090206001015460ff166002811115610b5057610b506149e1565b14610b6e5760405163aba4733960e01b815260040160405180910390fd5b335f90815260996020526040908190205490517fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa90610bae90849061513f565b60405180910390a250565b610bc1612341565b60a055565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015610c28573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c4c9190615151565b610c6957604051631d77d47760e21b815260040160405180910390fd5b6001548181168114610c8e5760405163c61dca5d60e01b815260040160405180910390fd5b610c97826123a0565b5050565b610ca3612341565b610cac816123d7565b50565b604080516060810182525f80825260208201819052918101919091525f838152609860205260409020805483908110610cea57610cea6150fd565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160c01b03600160401b909304929092169082015290505b92915050565b6040516308f6629d60e31b8152600481018290525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906347b314e890602401602060405180830381865afa158015610da2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d379190615170565b610dce612341565b610cac81612401565b610ddf612341565b610cac8161246a565b604080518082019091525f8082526020820152610d37610e617f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de684604051602001610e469291909182526001600160a01b0316602082015260400190565b604051602081830303815290604052805190602001206124d3565b61251f565b610e6e612341565b60a15460ff16610e9157604051635b77901960e01b815260040160405180910390fd5b610e9f8484846001856125a9565b50505050565b600154600290600490811603610ece5760405163840a48d560e01b815260040160405180910390fd5b610f1383838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505060965460ff1691506129e79050565b5083518214610f355760405163aaad13f760e01b815260040160405180910390fd5b5f5b828110156111e8575f848483818110610f5257610f526150fd565b885192013560f81c92505f9188915084908110610f7157610f716150fd565b60209081029190910101516040516379a0849160e11b815260ff841660048201529091506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f341092290602401602060405180830381865afa158015610fe4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611008919061518b565b63ffffffff1681511461102e57604051638e5aeee760e01b815260040160405180910390fd5b5f805b825181101561118f575f83828151811061104d5761104d6150fd565b6020908102919091018101516001600160a01b0381165f9081526099835260408082208151808301909252805482526001810154939550919390929083019060ff1660028111156110a0576110a06149e1565b60028111156110b1576110b16149e1565b90525080519091505f6110c3826120a6565b905060016001600160c01b03821660ff8a161c8116146110f65760405163d053aa2160e01b815260040160405180910390fd5b856001600160a01b0316846001600160a01b0316116111285760405163ba50f91160e01b815260040160405180910390fd5b5061118283838d8b8e61113c8260016151ba565b92611149939291906151cd565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061217b92505050565b5090925050600101611031565b5060ff83165f818152609b6020908152604091829020439081905591519182527f46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db4910160405180910390a2505050806001019050610f37565b5050505050565b5f54610100900460ff161580801561120d57505f54600160ff909116105b806112265750303b15801561122657505f5460ff166001145b61128e5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156112af575f805461ff0019166101001790555b6112b886612a1b565b6112c185612401565b6112ca836123a0565b6112d38461246a565b6112dc826123d7565b609c8054600181810183555f8390527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c91820180546001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166001600160a01b03199283161790925584548084018655840180547f000000000000000000000000000000000000000000000000000000000000000084169083161790558454928301909455910180547f00000000000000000000000000000000000000000000000000000000000000009092169190921617905560a1805461010161ffff19909116179055801561140d575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050565b604080518082019091525f80825260208201526001600160a01b0382165f908152609960209081526040918290208251808401909352805483526001810154909183019060ff16600281111561146d5761146d6149e1565b600281111561147e5761147e6149e1565b90525092915050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa1580156114e9573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061150d9190615151565b61152a57604051631d77d47760e21b815260040160405180910390fd5b6115345f196123a0565b565b61153e612341565b8161154881612a6c565b610b0b8383612a95565b609c8181548110611561575f80fd5b5f918252602090912001546001600160a01b0316905081565b611582612b3a565b6001600160a01b0382165f908152609f60209081526040808320429055609990915281208054609654919290916115bd90859060ff166129e7565b90505f6115c9836120a6565b905060018085015460ff1660028111156115e5576115e56149e1565b1480156115fa57506001600160c01b03821615155b801561161857506116186001600160c01b0383811690831681161490565b1561140d576116278686612b65565b60a15460ff161561140d5761140d8686612e25565b611644612341565b6115345f612a1b565b611655612341565b60a15460ff1661167857604051635b77901960e01b815260040160405180910390fd5b60a1805461ff0019166101001790556040517fa4cd42920ed0d1372ba4051d4577279f236fbbe677a67f3f7d645e82425dd98d905f90a1565b6116b9612341565b610b0b8383835f5f6125a9565b5f6117057f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a8787878787604051602001610e46969594939291906151f4565b9695505050505050565b5f610d37826120a6565b5f61172c6064546001600160a01b031690565b905090565b600180545f91908116036117585760405163840a48d560e01b815260040160405180910390fd5b60a154610100900460ff161561178157604051631d76201f60e31b815260040160405180910390fd5b5f61178c3387612f94565b90506117d333828b8b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508d92508b91508a90506130c2565b6001335f9081526099602052604090206001015460ff1660028111156117fb576117fb6149e1565b146118fb5760408051808201825282815260016020808301828152335f908152609990925293902082518155925183820180549394939192909160ff19169083600281111561184c5761184c6149e1565b021790555050604051639926ee7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169150639926ee7d906118a19033908790600401615279565b5f604051808303815f87803b1580156118b8575f5ffd5b505af11580156118ca573d5f5f3e3d5ffd5b50506040518392503391507fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe905f90a35b505050505050505050565b61190e6132e8565b600180546002908116036119355760405163840a48d560e01b815260040160405180910390fd5b60a15460ff1661195857604051635b77901960e01b815260040160405180910390fd5b5f61196283613331565b9050610e9f8482612b65565b5f610d37826133d9565b600180545f919081160361199f5760405163840a48d560e01b815260040160405180910390fd5b60a154610100900460ff16156119c857604051631d76201f60e31b815260040160405180910390fd5b5f6119d33385612f94565b90505f6119e2338389896133f0565b5190505f5b8751811015611a70575f888281518110611a0357611a036150fd565b0160209081015160f81c5f8181526097909252604090912054845191925063ffffffff1690849084908110611a3a57611a3a6150fd565b602002602001015163ffffffff161115611a6757604051633cb89c9760e01b815260040160405180910390fd5b506001016119e7565b506001335f9081526099602052604090206001015460ff166002811115611a9957611a996149e1565b14611b995760408051808201825283815260016020808301828152335f908152609990925293902082518155925183820180549394939192909160ff191690836002811115611aea57611aea6149e1565b021790555050604051639926ee7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169150639926ee7d90611b3f9033908890600401615279565b5f604051808303815f87803b158015611b56575f5ffd5b505af1158015611b68573d5f5f3e3d5ffd5b50506040518492503391507fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe905f90a35b50505050505050565b611baa6132e8565b600180545f9190811603611bd15760405163840a48d560e01b815260040160405180910390fd5b60a15460ff16611bf457604051635b77901960e01b815260040160405180910390fd5b5f611bfe85613331565b90505f8080611c0f868801886152d1565b9250925092505f611c208a83612f94565b90505f846001811115611c3557611c356149e1565b03611cdc575f611c478b8388876133f0565b5190505f5b8651811015611cd5575f878281518110611c6857611c686150fd565b0160209081015160f81c5f8181526097909252604090912054845191925063ffffffff1690849084908110611c9f57611c9f6150fd565b602002602001015163ffffffff161115611ccc57604051633cb89c9760e01b815260040160405180910390fd5b50600101611c4c565b5050611d31565b6001846001811115611cf057611cf06149e1565b03611d18575f80611d03898b018b61532c565b94509450505050611cd58c84898886866130c2565b60405163354bb8ab60e01b815260040160405180910390fd5b60016001600160a01b038b165f9081526099602052604090206001015460ff166002811115611d6257611d626149e1565b14611df957604080518082018252828152600160208083018281526001600160a01b038f165f908152609990925293902082518155925183820180549394939192909160ff191690836002811115611dbc57611dbc6149e1565b0217905550506040518291506001600160a01b038c16907fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe905f90a35b50505050505050505050565b6060610b21609884846136d4565b60018054600290811603611e3a5760405163840a48d560e01b815260040160405180910390fd5b5f5b8251811015611e995760a15460ff161580611e745750611e74838281518110611e6757611e676150fd565b016020015160f81c6133d9565b611e915760405163b2e18e0560e01b815260040160405180910390fd5b600101611e3c565b50610c973383612b65565b611eac612341565b60a15460ff1615611ed05760405163b2e18e0560e01b815260040160405180910390fd5b609654611edf9060ff16613783565b60a25560a1805460ff191660011790556040517f0b88306ff4627121f5b3e5b1c5f88f6b1e42fd2c0478ef1c91662d49d1f07755905f90a1565b611f21612341565b6001600160a01b038116611f865760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401611285565b610cac81612a1b565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611feb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061200f9190615170565b6001600160a01b0316336001600160a01b0316146120405760405163794821ff60e01b815260040160405180910390fd5b600154801982198116146120675760405163c61dca5d60e01b815260040160405180910390fd5b600182905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020015b60405180910390a25050565b5f610d37609883613791565b60605f5f6120bf846137fb565b61ffff166001600160401b038111156120da576120da614403565b6040519080825280601f01601f191660200182016040528015612104576020820181803683370190505b5090505f805b82518210801561211b575061010081105b15612171576001811b935085841615612161578060f81b838381518110612144576121446150fd565b60200101906001600160f81b03191690815f1a9053508160010191505b61216a816153dc565b905061210a565b5090949350505050565b600182602001516002811115612193576121936149e1565b1461219d57505050565b81516040516333567f7f60e11b81525f906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906366acfefe906121f1908890869088906004016153f4565b6020604051808303815f875af115801561220d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122319190615423565b90506001600160c01b038116156111e8576111e885612258836001600160c01b03166120b2565b612b65565b5f83815260208590526040812080548291908490811061227f5761227f6150fd565b5f91825260209182902060408051606081018252929091015463ffffffff808216808552600160201b8304821695850195909552600160401b9091046001600160c01b031691830191909152909250851610156122ef57604051636cb19aff60e01b815260040160405180910390fd5b602081015163ffffffff1615806123155750806020015163ffffffff168463ffffffff16105b6123325760405163bbba60cb60e01b815260040160405180910390fd5b6040015190505b949350505050565b3361234a611719565b6001600160a01b0316146115345760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401611285565b600181905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d90602001610bae565b60a180546001600160a01b03909216620100000262010000600160b01b0319909216919091179055565b609d54604080516001600160a01b03928316815291831660208301527f315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c910160405180910390a1609d80546001600160a01b0319166001600160a01b0392909216919091179055565b609e54604080516001600160a01b03928316815291831660208301527f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9910160405180910390a1609e80546001600160a01b0319166001600160a01b0392909216919091179055565b5f610d376124df613825565b8360405161190160f01b602082015260228101839052604281018290525f9060620160405160208183030381529060405280519060200120905092915050565b604080518082019091525f80825260208201525f808061254c5f5160206159855f395f51905f528661545d565b90505b6125588161394b565b90935091505f5160206159855f395f51905f528283098303612590576040805180820190915290815260208101919091529392505050565b5f5160206159855f395f51905f5260018208905061254f565b60965460ff1660c081106125d057604051633cb89c9760e01b815260040160405180910390fd5b6125db816001615470565b6096805460ff191660ff92909216919091179055806125fa8188612a95565b60a15460ff1680156126125750612610816133d9565b155b156127bd576040805160018082528183019092525f91816020015b604080518082019091525f81526060602082015281526020019060019003908161262d5790505090505f86516001600160401b0381111561267057612670614403565b604051908082528060200260200182016040528015612699578160200160208202803683370190505b5090505f5b87518110156126f6578781815181106126b9576126b96150fd565b60200260200101515f01518282815181106126d6576126d66150fd565b6001600160a01b039092166020928302919091019091015260010161269e565b5060405180604001604052808460ff1663ffffffff16815260200182815250825f81518110612727576127276150fd565b602090810291909101015260a154604051630130fc2760e51b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169263261f84e09261278d9262010000909204909116908690600401615489565b5f604051808303815f87803b1580156127a4575f5ffd5b505af11580156127b6573d5f5f3e3d5ffd5b5050505050505b5f8460018111156127d0576127d06149e1565b0361285757604051633aea0b9d60e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906375d4173a906128259084908a908a906004016155a3565b5f604051808303815f87803b15801561283c575f5ffd5b505af115801561284e573d5f5f3e3d5ffd5b505050506128f0565b600184600181111561286b5761286b6149e1565b036128f057604051630662d3e160e51b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063cc5a7c20906128c29084908a9088908b906004016155cd565b5f604051808303815f87803b1580156128d9575f5ffd5b505af11580156128eb573d5f5f3e3d5ffd5b505050505b60405163136ca0f960e11b815260ff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906326d941f2906024015f604051808303815f87803b158015612951575f5ffd5b505af1158015612963573d5f5f3e3d5ffd5b505060405163136ca0f960e11b815260ff841660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692506326d941f291506024015f604051808303815f87803b1580156129c8575f5ffd5b505af11580156129da573d5f5f3e3d5ffd5b5050505050505050505050565b5f5f6129f2846139c7565b9050808360ff166001901b11610b215760405163ca95733360e01b815260040160405180910390fd5b606480546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b60965460ff90811690821610610cac57604051637310cff560e11b815260040160405180910390fd5b60ff82165f81815260976020908152604091829020845181548684018051888701805163ffffffff90951665ffffffffffff199094168417600160201b61ffff938416021767ffff0000000000001916600160301b95831695909502949094179094558551918252518316938101939093525116918101919091527f3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac9060600161209a565b609e546001600160a01b03163314611534576040516376d8ab1760e11b815260040160405180910390fd5b6001600160a01b0382165f90815260996020526040812080549091612b89826120a6565b905060018084015460ff166002811115612ba557612ba56149e1565b14612bc35760405163aba4733960e01b815260040160405180910390fd5b6096545f90612bd690869060ff166129e7565b90506001600160c01b038116612bff576040516368b6a87560e11b815260040160405180910390fd5b612c166001600160c01b0382811690841681161490565b612c335760405163d053aa2160e01b815260040160405180910390fd5b6001600160c01b0381811619831616612c4c8482613a87565b6001600160c01b038116612ca8576001600160a01b0387165f81815260996020526040808220600101805460ff19166002179055518692917f396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e491a35b60405163f4e24fe560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f4e24fe590612cf6908a908a90600401615603565b5f604051808303815f87803b158015612d0d575f5ffd5b505af1158015612d1f573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd9150612d719087908a90600401615626565b5f604051808303815f87803b158015612d88575f5ffd5b505af1158015612d9a573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd9150612dec9087908a90600401615626565b5f604051808303815f87803b158015612e03575f5ffd5b505af1158015612e15573d5f5f3e3d5ffd5b50505050611b9987858884613a93565b5f81516001600160401b03811115612e3f57612e3f614403565b604051908082528060200260200182016040528015612e68578160200160208202803683370190505b5090505f805b8351811015612ee7575f848281518110612e8a57612e8a6150fd565b016020015160f81c9050612e9d816133d9565b15612ede5760ff81168484612eb1816153dc565b955081518110612ec357612ec36150fd565b602002602001019063ffffffff16908163ffffffff16815250505b50600101612e6e565b508015610e9f57808252604080516060810182526001600160a01b03868116825260a154620100009004811660208301528183018590529151636e3492b560e01b81527f000000000000000000000000000000000000000000000000000000000000000090921691636e3492b591612f619160040161563e565b5f604051808303815f87803b158015612f78575f5ffd5b505af1158015612f8a573d5f5f3e3d5ffd5b5050505050505050565b6040516309aa152760e11b81526001600160a01b0383811660048301525f917f0000000000000000000000000000000000000000000000000000000000000000909116906313542a4e90602401602060405180830381865afa158015612ffc573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061302091906156ac565b90505f819003610d37577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bf79ce58848461306487610de8565b6040518463ffffffff1660e01b8152600401613082939291906156e5565b6020604051808303815f875af115801561309e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b2191906156ac565b83518251146130e45760405163aaad13f760e01b815260040160405180910390fd5b6130f086868484613b2b565b5f6130fd878787876133f0565b90505f5b8551811015612f8a575f60975f888481518110613120576131206150fd565b0160209081015160f81c82528181019290925260409081015f208151606081018352905463ffffffff811680835261ffff600160201b8304811695840195909552600160301b9091049093169181019190915284518051919350908490811061318b5761318b6150fd565b602002602001015163ffffffff1611156132df5761321f8783815181106131b4576131b46150fd565b602001015160f81c60f81b60f81c846040015184815181106131d8576131d86150fd565b60200260200101518b866020015186815181106131f7576131f76150fd565b6020026020010151898781518110613211576132116150fd565b602002602001015186613bd6565b6040805160018082528183019092525f91602082018180368337019050509050878381518110613251576132516150fd565b602001015160f81c60f81b815f8151811061326e5761326e6150fd565b60200101906001600160f81b03191690815f1a9053506132ab868481518110613299576132996150fd565b60200260200101516020015182612b65565b60a15460ff16156132dd576132dd8684815181106132cb576132cb6150fd565b60200260200101516020015182612e25565b505b50600101613101565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611534576040516323d871a560e01b815260040160405180910390fd5b60605f82516001600160401b0381111561334d5761334d614403565b6040519080825280601f01601f191660200182016040528015613377576020820181803683370190505b5090505f5b83518110156133d257838181518110613397576133976150fd565b602002602001015160f81b8282815181106133b4576133b46150fd565b60200101906001600160f81b03191690815f1a90535060010161337c565b5092915050565b60a2545f90610d37908360ff161c60019081161490565b61341460405180606001604052806060815260200160608152602001606081525090565b6096545f9061342790859060ff166129e7565b90505f613433866120a6565b90506001600160c01b03821661345c576040516313ca465760e01b815260040160405180910390fd5b8082166001600160c01b03161561348657604051630c6816cd60e01b815260040160405180910390fd5b60a0546001600160a01b0388165f908152609f60205260409020546001600160c01b03838116908516179142916134bd91906151ba565b106134db57604051631968677d60e11b815260040160405180910390fd5b6134e58782613a87565b867fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa86604051613515919061513f565b60405180910390a2604051631fd93ca960e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633fb279529061356b908b908a90600401615603565b5f604051808303815f87803b158015613582575f5ffd5b505af1158015613594573d5f5f3e3d5ffd5b5050604051632550477760e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169250632550477791506135e8908b908b908b906004016153f4565b5f604051808303815f875af1158015613603573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261362a91908101906157c2565b60408087019190915260208601919091525162bff04d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062bff04d90613685908a908a90600401615626565b5f604051808303815f875af11580156136a0573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526136c7919081019061581b565b8452505050949350505050565b60605f82516001600160401b038111156136f0576136f0614403565b604051908082528060200260200182016040528015613719578160200160208202803683370190505b5090505f5b835181101561377a5761374b868686848151811061373e5761373e6150fd565b6020026020010151613d4c565b82828151811061375d5761375d6150fd565b63ffffffff9092166020928302919091019091015260010161371e565b50949350505050565b5f610d37600180841b6158aa565b5f818152602083905260408120548082036137af575f915050610d37565b5f8381526020859052604090206137c76001836158aa565b815481106137d7576137d76150fd565b5f91825260209091200154600160401b90046001600160c01b03169150610d379050565b5f805b8215610d375761380f6001846158aa565b909216918061381d816158bd565b9150506137fe565b5f306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614801561387d57507f000000000000000000000000000000000000000000000000000000000000000046145b156138a757507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b5f80805f5160206159855f395f51905f5260035f5160206159855f395f51905f52865f5160206159855f395f51905f52888909090890505f6139bb827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f5160206159855f395f51905f52613e64565b91959194509092505050565b5f610100825111156139ec57604051637da54e4760e11b815260040160405180910390fd5b81515f036139fb57505f919050565b5f5f835f81518110613a0f57613a0f6150fd565b0160200151600160f89190911c81901b92505b8451811015613a7957848181518110613a3d57613a3d6150fd565b0160200151600160f89190911c1b9150828211613a6d57604051631019106960e31b815260040160405180910390fd5b91811791600101613a22565b50909392505050565b191690565b610c9760988383613edd565b5f613ab260a254836001600160c01b0316613a8290919063ffffffff16565b9050806111e8576040516351b27a6d60e11b81526001600160a01b0386811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da906024015f604051808303815f87803b158015613b19575f5ffd5b505af11580156118fb573d5f5f3e3d5ffd5b6020808201515f908152609a909152604090205460ff1615613b6057604051636fbefec360e11b815260040160405180910390fd5b4281604001511015613b8557604051630819bdcd60e01b815260040160405180910390fd5b602080820180515f908152609a909252604091829020805460ff19166001179055609d54905191830151610e9f926001600160a01b0390921691613bcf91889188918891906116c6565b8351614096565b6020808301516001600160a01b038082165f8181526099909452604090932054919290871603613c19576040516356168b4160e11b815260040160405180910390fd5b8760ff16845f015160ff1614613c4257604051638e5aeee760e01b815260040160405180910390fd5b604051635401ed2760e01b81526004810182905260ff891660248201525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635401ed2790604401602060405180830381865afa158015613cb0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cd491906158dd565b9050613ce081856140be565b6001600160601b0316866001600160601b031611613d1157604051634c44995d60e01b815260040160405180910390fd5b613d1b88856140e1565b6001600160601b0316816001600160601b0316106118fb5760405163b187e86960e01b815260040160405180910390fd5b5f81815260208490526040812054815b81811015613dcf576001613d7082846158aa565b613d7a91906158aa565b92508463ffffffff16865f8681526020019081526020015f208463ffffffff1681548110613daa57613daa6150fd565b5f9182526020909120015463ffffffff1611613dc7575050610b21565b600101613d5c565b5060405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a401611285565b5f5f613e6e6143c7565b613e766143e5565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280613eb357fe5b5082613ed25760405163d51edae360e01b815260040160405180910390fd5b505195945050505050565b5f8281526020849052604081205490819003613f81575f83815260208581526040808320815160608101835263ffffffff43811682528185018681526001600160c01b03808a16958401958652845460018101865594885295909620915191909201805495519351909416600160401b026001600160401b03938316600160201b0267ffffffffffffffff1990961691909216179390931716919091179055610e9f565b5f838152602085905260408120613f996001846158aa565b81548110613fa957613fa96150fd565b5f918252602090912001805490915063ffffffff438116911603613fea5780546001600160401b0316600160401b6001600160c01b038516021781556111e8565b805463ffffffff438116600160201b81810267ffffffff00000000199094169390931784555f8781526020898152604080832081516060810183529485528483018481526001600160c01b03808c1693870193845282546001810184559286529390942094519401805493519151909216600160401b026001600160401b0391861690960267ffffffffffffffff19909316939094169290921717919091169190911790555050505050565b6140a18383836140fa565b610b0b57604051638baa579f60e01b815260040160405180910390fd5b60208101515f90612710906140d79061ffff16856158f8565b610b21919061591a565b60408101515f90612710906140d79061ffff16856158f8565b5f5f5f614107858561423f565b90925090505f81600481111561411f5761411f6149e1565b14801561413d5750856001600160a01b0316826001600160a01b0316145b1561414d57600192505050610b21565b5f5f876001600160a01b0316631626ba7e60e01b8888604051602401614174929190615626565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516141b29190615947565b5f60405180830381855afa9150503d805f81146141ea576040519150601f19603f3d011682016040523d82523d5f602084013e6141ef565b606091505b5091509150818015614202575080516020145b801561423357508051630b135d3f60e11b90614227908301602090810190840161595d565b6001600160e01b031916145b98975050505050505050565b5f5f8251604103614273576020830151604084015160608501515f1a614267878285856142aa565b945094505050506142a3565b825160400361429c576020830151604084015161429186838361438f565b9350935050506142a3565b505f905060025b9250929050565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156142df57505f90506003614386565b8460ff16601b141580156142f757508460ff16601c14155b1561430757505f90506004614386565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015614358573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116614380575f60019250925050614386565b91505f90505b94509492505050565b5f806001600160ff1b038316816143ab60ff86901c601b6151ba565b90506143b9878288856142aa565b935093505050935093915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b038111828210171561443957614439614403565b60405290565b604080519081016001600160401b038111828210171561443957614439614403565b604051601f8201601f191681016001600160401b038111828210171561448957614489614403565b604052919050565b5f6001600160401b038211156144a9576144a9614403565b5060051b60200190565b6001600160a01b0381168114610cac575f5ffd5b5f82601f8301126144d6575f5ffd5b81356144e96144e482614491565b614461565b8082825260208201915060208360051b86010192508583111561450a575f5ffd5b602085015b83811015614530578035614522816144b3565b83526020928301920161450f565b5095945050505050565b5f6020828403121561454a575f5ffd5b81356001600160401b0381111561455f575f5ffd5b612339848285016144c7565b5f6020828403121561457b575f5ffd5b5035919050565b63ffffffff81168114610cac575f5ffd5b5f5f5f606084860312156145a5575f5ffd5b8335925060208401356145b781614582565b929592945050506040919091013590565b5f82601f8301126145d7575f5ffd5b8135602083015f5f6001600160401b038411156145f6576145f6614403565b50601f8301601f191660200161460b81614461565b91505082815285838301111561461f575f5ffd5b828260208301375f92810160200192909252509392505050565b5f60208284031215614649575f5ffd5b81356001600160401b0381111561465e575f5ffd5b612339848285016145c8565b5f6020828403121561467a575f5ffd5b8135610b21816144b3565b5f5f60408385031215614696575f5ffd5b50508035926020909101359150565b803560ff811681146146b5575f5ffd5b919050565b5f602082840312156146ca575f5ffd5b610b21826146a5565b815181526020808301519082015260408101610d37565b803561ffff811681146146b5575f5ffd5b5f6060828403121561470b575f5ffd5b614713614417565b9050813561472081614582565b815261472e602083016146ea565b602082015261473f604083016146ea565b604082015292915050565b6001600160601b0381168114610cac575f5ffd5b5f82601f83011261476d575f5ffd5b813561477b6144e482614491565b8082825260208201915060208360061b86010192508583111561479c575f5ffd5b602085015b8381101561453057604081880312156147b8575f5ffd5b6147c061443f565b81356147cb816144b3565b815260208201356147db8161474a565b60208281019190915290845292909201916040016147a1565b5f5f5f5f60c08587031215614807575f5ffd5b61481186866146fb565b935060608501356148218161474a565b925060808501356001600160401b0381111561483b575f5ffd5b6148478782880161475e565b92505060a085013561485881614582565b939692955090935050565b5f5f83601f840112614873575f5ffd5b5081356001600160401b03811115614889575f5ffd5b6020830191508360208285010111156142a3575f5ffd5b5f5f5f604084860312156148b2575f5ffd5b83356001600160401b038111156148c7575f5ffd5b8401601f810186136148d7575f5ffd5b80356148e56144e482614491565b8082825260208201915060208360051b850101925088831115614906575f5ffd5b602084015b838110156149465780356001600160401b03811115614928575f5ffd5b6149378b6020838901016144c7565b8452506020928301920161490b565b50955050505060208401356001600160401b03811115614964575f5ffd5b61497086828701614863565b9497909650939450505050565b5f5f5f5f5f60a08688031215614991575f5ffd5b853561499c816144b3565b945060208601356149ac816144b3565b935060408601356149bc816144b3565b92506060860135915060808601356149d3816144b3565b809150509295509295909350565b634e487b7160e01b5f52602160045260245ffd5b60038110614a1157634e487b7160e01b5f52602160045260245ffd5b9052565b8151815260208083015160408301916133d2908401826149f5565b5f5f60808385031215614a41575f5ffd5b614a4a836146a5565b9150614a5984602085016146fb565b90509250929050565b5f5f60408385031215614a73575f5ffd5b8235614a7e816144b3565b915060208301356001600160401b03811115614a98575f5ffd5b614aa4858286016145c8565b9150509250929050565b5f5f5f60a08486031215614ac0575f5ffd5b614aca85856146fb565b92506060840135614ada8161474a565b915060808401356001600160401b03811115614af4575f5ffd5b614b008682870161475e565b9150509250925092565b5f82601f830112614b19575f5ffd5b8135614b276144e482614491565b8082825260208201915060208360061b860101925085831115614b48575f5ffd5b602085015b838110156145305760408188031215614b64575f5ffd5b614b6c61443f565b614b75826146a5565b81526020820135614b85816144b3565b6020828101919091529084529290920191604001614b4d565b5f5f5f5f5f60a08688031215614bb2575f5ffd5b8535614bbd816144b3565b94506020860135935060408601356001600160401b03811115614bde575f5ffd5b614bea88828901614b0a565b9598949750949560608101359550608001359392505050565b5f60408284031215614c13575f5ffd5b614c1b61443f565b823581526020928301359281019290925250919050565b5f82601f830112614c41575f5ffd5b614c4961443f565b806040840185811115614c5a575f5ffd5b845b81811015614c74578035845260209384019301614c5c565b509095945050505050565b5f818303610100811215614c91575f5ffd5b614c99614417565b9150614ca58484614c03565b8252614cb48460408501614c03565b60208301526080607f1982011215614cca575f5ffd5b50614cd361443f565b614ce08460808501614c32565b8152614cef8460c08501614c32565b6020820152604082015292915050565b5f60608284031215614d0f575f5ffd5b614d17614417565b905081356001600160401b03811115614d2e575f5ffd5b614d3a848285016145c8565b8252506020828101359082015260409182013591810191909152919050565b5f5f5f5f5f5f5f6101a0888a031215614d70575f5ffd5b87356001600160401b03811115614d85575f5ffd5b614d918a828b01614863565b90985096505060208801356001600160401b03811115614daf575f5ffd5b614dbb8a828b016145c8565b955050614dcb8960408a01614c7f565b93506101408801356001600160401b03811115614de6575f5ffd5b614df28a828b01614b0a565b9350506101608801356001600160401b03811115614e0e575f5ffd5b614e1a8a828b01614cff565b9250506101808801356001600160401b03811115614e36575f5ffd5b614e428a828b01614cff565b91505092959891949750929550565b5f82601f830112614e60575f5ffd5b8135614e6e6144e482614491565b8082825260208201915060208360051b860101925085831115614e8f575f5ffd5b602085015b83811015614530578035614ea781614582565b835260209283019201614e94565b5f5f60408385031215614ec6575f5ffd5b8235614ed1816144b3565b915060208301356001600160401b03811115614eeb575f5ffd5b614aa485828601614e51565b5f5f5f5f6101608587031215614f0b575f5ffd5b84356001600160401b03811115614f20575f5ffd5b614f2c878288016145c8565b94505060208501356001600160401b03811115614f47575f5ffd5b614f53878288016145c8565b935050614f638660408701614c7f565b91506101408501356001600160401b03811115614f7e575f5ffd5b614f8a87828801614cff565b91505092959194509250565b5f5f5f5f60608587031215614fa9575f5ffd5b8435614fb4816144b3565b935060208501356001600160401b03811115614fce575f5ffd5b614fda87828801614e51565b93505060408501356001600160401b03811115614ff5575f5ffd5b61500187828801614863565b95989497509550505050565b5f5f6040838503121561501e575f5ffd5b823561502981614582565b915060208301356001600160401b03811115615043575f5ffd5b8301601f81018513615053575f5ffd5b80356150616144e482614491565b8082825260208201915060208360051b850101925087831115615082575f5ffd5b6020840193505b828410156150a4578335825260209384019390910190615089565b809450505050509250929050565b602080825282518282018190525f918401906040840190835b81811015614c7457835163ffffffff168352602093840193909201916001016150cb565b60208101610d3782846149f5565b634e487b7160e01b5f52603260045260245ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f610b216020830184615111565b5f60208284031215615161575f5ffd5b81518015158114610b21575f5ffd5b5f60208284031215615180575f5ffd5b8151610b21816144b3565b5f6020828403121561519b575f5ffd5b8151610b2181614582565b634e487b7160e01b5f52601160045260245ffd5b80820180821115610d3757610d376151a6565b5f5f858511156151db575f5ffd5b838611156151e7575f5ffd5b5050820193919092039150565b5f60c0820188835260018060a01b038816602084015286604084015260c0606084015280865180835260e0850191506020880192505f5b81811015615261578351805160ff1684526020908101516001600160a01b0316818501529093019260409092019160010161522b565b50506080840195909552505060a00152949350505050565b60018060a01b0383168152604060208201525f8251606060408401526152a260a0840182615111565b90506020840151606084015260408401516080840152809150509392505050565b8035600281106146b5575f5ffd5b5f5f5f61014084860312156152e4575f5ffd5b6152ed846152c3565b925060208401356001600160401b03811115615307575f5ffd5b615313868287016145c8565b9250506153238560408601614c7f565b90509250925092565b5f5f5f5f5f6101808688031215615341575f5ffd5b61534a866152c3565b945060208601356001600160401b03811115615364575f5ffd5b615370888289016145c8565b9450506153808760408801614c7f565b92506101408601356001600160401b0381111561539b575f5ffd5b6153a788828901614b0a565b9250506101608601356001600160401b038111156153c3575f5ffd5b6153cf88828901614cff565b9150509295509295909350565b5f600182016153ed576153ed6151a6565b5060010190565b60018060a01b0384168152826020820152606060408201525f61541a6060830184615111565b95945050505050565b5f60208284031215615433575f5ffd5b81516001600160c01b0381168114610b21575f5ffd5b634e487b7160e01b5f52601260045260245ffd5b5f8261546b5761546b615449565b500690565b60ff8181168382160190811115610d3757610d376151a6565b5f6040820160018060a01b03851683526040602084015280845180835260608501915060608160051b8601019250602086015f5b8281101561553e57868503605f190184528151805163ffffffff168652602090810151604082880181905281519088018190529101905f9060608801905b808310156155265783516001600160a01b0316825260209384019360019390930192909101906154fb565b509650505060209384019391909101906001016154bd565b5092979650505050505050565b5f8151808452602084019350602083015f5b8281101561559957815180516001600160a01b031687526020908101516001600160601b0316818801526040909601959091019060010161555d565b5093949350505050565b60ff841681526001600160601b0383166020820152606060408201525f61541a606083018461554b565b60ff851681526001600160601b038416602082015263ffffffff83166040820152608060608201525f611705608083018461554b565b6001600160a01b03831681526040602082018190525f90610b1e90830184615111565b828152604060208201525f610b1e6040830184615111565b602080825282516001600160a01b039081168383015283820151166040808401919091528301516060808401528051608084018190525f929190910190829060a08501905b808310156145305763ffffffff8451168252602082019150602084019350600183019250615683565b5f602082840312156156bc575f5ffd5b5051919050565b805f5b6002811015610e9f5781518452602093840193909101906001016156c6565b6001600160a01b0384168152825180516020808401919091520151604082015261016081016020848101518051606085015290810151608084015250604084015161573460a0840182516156c3565b6020015161574560e08401826156c3565b5082516101208301526020830151610140830152612339565b5f82601f83011261576d575f5ffd5b815161577b6144e482614491565b8082825260208201915060208360051b86010192508583111561579c575f5ffd5b602085015b838110156145305780516157b48161474a565b8352602092830192016157a1565b5f5f604083850312156157d3575f5ffd5b82516001600160401b038111156157e8575f5ffd5b6157f48582860161575e565b92505060208301516001600160401b0381111561580f575f5ffd5b614aa48582860161575e565b5f6020828403121561582b575f5ffd5b81516001600160401b03811115615840575f5ffd5b8201601f81018413615850575f5ffd5b805161585e6144e482614491565b8082825260208201915060208360051b85010192508683111561587f575f5ffd5b6020840193505b8284101561170557835161589981614582565b825260209384019390910190615886565b81810381811115610d3757610d376151a6565b5f61ffff821661ffff81036158d4576158d46151a6565b60010192915050565b5f602082840312156158ed575f5ffd5b8151610b218161474a565b6001600160601b0381811683821602908116908181146133d2576133d26151a6565b5f6001600160601b0383168061593257615932615449565b806001600160601b0384160491505092915050565b5f82518060208501845e5f920191825250919050565b5f6020828403121561596d575f5ffd5b81516001600160e01b031981168114610b21575f5ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220a2ada9834d29610bf3e5aca508a2b5a61a2b0c9f846e8383d380ca01181b0fdd64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x02\0`@R4\x80\x15a\0\x10W__\xFD[P`@Qa^78\x03\x80a^7\x839\x81\x01`@\x81\x90Ra\0/\x91a\x02\x93V[`@\x80Q\x80\x82\x01\x82R`\x16\x81R\x7FAVSRegistryCoordinator\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x82R\x83Q\x80\x85\x01\x90\x94R`\x06\x84Rev0.0.1`\xD0\x1B\x90\x84\x01R\x81Q\x90 `\xE0\x81\x90R\x7Fk\xDA~?8^H\x84\x10H9\x04D\xCC\xED\\\xC7\x95\xAF\x87u\x8A\xF6v\"\xE5\xF4\xF0\x88,J\x99a\x01\0\x81\x90RF`\xA0R\x88\x93\x88\x93\x88\x93\x88\x93\x88\x93\x88\x93\x88\x93\x88\x93\x88\x93\x88\x93\x92\x90\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x019\x81\x84\x84`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\x80R0`\xC0Ra\x01 RPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x90Pa\x01pW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01@R\x93\x84\x16a\x01\x80R\x91\x83\x16a\x01`R\x82\x16a\x01\xA0R\x16a\x01\xC0Ra\x01\xA0a\x01\xBFV[PPP`\x01`\x01`\xA0\x1B\x03\x90\x97\x16a\x01\xE0RPa\x03\x16\x95PPPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x02*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x10\x15a\x02zW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x90W__\xFD[PV[______`\xC0\x87\x89\x03\x12\x15a\x02\xA8W__\xFD[\x86Qa\x02\xB3\x81a\x02|V[` \x88\x01Q\x90\x96Pa\x02\xC4\x81a\x02|V[`@\x88\x01Q\x90\x95Pa\x02\xD5\x81a\x02|V[``\x88\x01Q\x90\x94Pa\x02\xE6\x81a\x02|V[`\x80\x88\x01Q\x90\x93Pa\x02\xF7\x81a\x02|V[`\xA0\x88\x01Q\x90\x92Pa\x03\x08\x81a\x02|V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0Qa\x01\xE0QaY\xDAa\x04]_9_\x81\x81a\x05\xCA\x01R\x81\x81a\x18i\x01R\x81\x81a\x1B\x07\x01Ra:\xD8\x01R_\x81\x81a\x08\xE6\x01R\x81\x81a'L\x01R\x81\x81a/,\x01Ra2\xF3\x01R_\x81\x81a\x07\xEA\x01R\x81\x81a\x0F\x9F\x01R\x81\x81a\x13\x8E\x01R\x81\x81a)\x08\x01R\x81\x81a-\xB3\x01Ra6O\x01R_\x81\x81a\x06\xE6\x01R\x81\x81a\x13\x1B\x01R\x81\x81a!\xB8\x01R\x81\x81a'\xEC\x01R\x81\x81a(\x87\x01R\x81\x81a-8\x01R\x81\x81a5\xAD\x01Ra<c\x01R_\x81\x81a\x06\xAC\x01R\x81\x81a\rU\x01R\x81\x81a\x13Y\x01R\x81\x81a)}\x01R\x81\x81a,\xBF\x01R\x81\x81a/\xB5\x01R\x81\x81a0,\x01Ra54\x01R_\x81\x81a\x07v\x01R\x81\x81a\x0B\xDB\x01R\x81\x81a\x14\x9C\x01Ra\x1F\x91\x01R_a8\xAE\x01R_a8\xFD\x01R_a8\xD8\x01R_a81\x01R_a8[\x01R_a8\x85\x01RaY\xDA_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x03\x92W_5`\xE0\x1C\x80ch0H5\x11a\x01\xDFW\x80c\xA4\xD7\x87\x1F\x11a\x01\tW\x80c\xCAO-\x97\x11a\0\xA9W\x80c\xEE1\x88!\x11a\0yW\x80c\xEE1\x88!\x14a\t\xB2W\x80c\xF2\xFD\xE3\x8B\x14a\t\xBAW\x80c\xFA\xBC\x1C\xBC\x14a\t\xCDW\x80c\xFD9\x10Z\x14a\t\xE0W__\xFD[\x80c\xCAO-\x97\x14a\x08\xCEW\x80c\xCA\x8A\xA7\xC7\x14a\x08\xE1W\x80c\xD7-\x8D\xD6\x14a\t\x08W\x80c\xE6W\x97\xAD\x14a\t\x10W__\xFD[\x80c\xAD\xCFs\xF7\x11a\0\xE4W\x80c\xAD\xCFs\xF7\x14a\x08bW\x80c\xB2\xD8g\x8D\x14a\x08uW\x80c\xC3\x91B^\x14a\x08\x87W\x80c\xCA\r\xE8\x82\x14a\x08\xA7W__\xFD[\x80c\xA4\xD7\x87\x1F\x14a\x083W\x80c\xA5\x08W\xBF\x14a\x08FW\x80c\xA9ox>\x14a\x08YW__\xFD[\x80c\x87\x1E\xF0I\x11a\x01\x7FW\x80c\x9B]\x17{\x11a\x01OW\x80c\x9B]\x17{\x14a\x07\xBFW\x80c\x9D\x8E\x0C#\x14a\x07\xD2W\x80c\x9E\x99#\xC2\x14a\x07\xE5W\x80c\x9F\xEA\xB8Y\x14a\x08\x0CW__\xFD[\x80c\x87\x1E\xF0I\x14a\x07^W\x80c\x88o\x11\x95\x14a\x07qW\x80c\x8D\xA5\xCB[\x14a\x07\x98W\x80c\x9A\xA1e=\x14a\x07\xA0W__\xFD[\x80cs;u\x07\x11a\x01\xBAW\x80cs;u\x07\x14a\x07#W\x80c\x81\xF96\xD2\x14a\x07+W\x80c\x82\x81\xABu\x14a\x078W\x80c\x84\xCAR\x13\x14a\x07KW__\xFD[\x80ch0H5\x14a\x06\xE1W\x80cn;\x17\xDB\x14a\x07\x08W\x80cqP\x18\xA6\x14a\x07\x1BW__\xFD[\x80c)k\xB0d\x11a\x02\xC0W\x80cS\x0B\x97\xA4\x11a\x02`W\x80c[\x0B\x82\x9F\x11a\x020W\x80c[\x0B\x82\x9F\x14a\x06\x8CW\x80c\\\x97Z\xBB\x14a\x06\x9FW\x80c]\xF4YF\x14a\x06\xA7W\x80ccG\xC9\0\x14a\x06\xCEW__\xFD[\x80cS\x0B\x97\xA4\x14a\x062W\x80cXe\xC6\x0C\x14a\x06EW\x80cY\\jg\x14a\x06eW\x80cZ\xC8j\xB7\x14a\x06mW__\xFD[\x80c9\x98\xFD\xD3\x11a\x02\x9BW\x80c9\x98\xFD\xD3\x14a\x05\xC5W\x80c<*\x7FL\x14a\x05\xECW\x80c>\xEF:Q\x14a\x06\x0CW\x80cQ@\xA5H\x14a\x06\x1FW__\xFD[\x80c)k\xB0d\x14a\x05\x8CW\x80c)\xD1\xE0\xC3\x14a\x05\x9FW\x80c,\xDD\x1E\x86\x14a\x05\xB2W__\xFD[\x80c\x12^\x05\x84\x11a\x036W\x80c\x14x\x85\x1F\x11a\x03\x06W\x80c\x14x\x85\x1F\x14a\x04\xDFW\x80c\x1E\xB8\x12\xDA\x14a\x05\x11W\x80c$\x9A\x0CB\x14a\x05ZW\x80c(\xF6\x1B1\x14a\x05yW__\xFD[\x80c\x12^\x05\x84\x14a\x04rW\x80c\x13T*N\x14a\x04\x91W\x80c\x13d9\xDD\x14a\x04\xB9W\x80c\x14>Y\x15\x14a\x04\xCCW__\xFD[\x80c\x05C\x10\xE6\x11a\x03qW\x80c\x05C\x10\xE6\x14a\x04\x08W\x80c\x07d\xCB\x93\x14a\x043W\x80c\x0C\xF4\xB7g\x14a\x04LW\x80c\r?!4\x14a\x04_W__\xFD[\x80b\xCF*\xB5\x14a\x03\x96W\x80c\x03\xFD4\x92\x14a\x03\xABW\x80c\x04\xECcQ\x14a\x03\xDDW[__\xFD[a\x03\xA9a\x03\xA46`\x04aE:V[a\n\x1BV[\0[a\x03\xCAa\x03\xB96`\x04aEkV[_\x90\x81R`\x98` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\xF0a\x03\xEB6`\x04aE\x93V[a\x0B\x10V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xD4V[`\x9DTa\x04\x1B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xD4V[`\xA1Ta\x04\x1B\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xA9a\x04Z6`\x04aF9V[a\x0B(V[a\x03\xA9a\x04m6`\x04aEkV[a\x0B\xB9V[a\x03\xCAa\x04\x806`\x04aFjV[`\x9F` R_\x90\x81R`@\x90 T\x81V[a\x03\xCAa\x04\x9F6`\x04aFjV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x99` R`@\x90 T\x90V[a\x03\xA9a\x04\xC76`\x04aEkV[a\x0B\xC6V[a\x03\xA9a\x04\xDA6`\x04aFjV[a\x0C\x9BV[a\x05\x01a\x04\xED6`\x04aEkV[`\x9A` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xD4V[a\x05$a\x05\x1F6`\x04aF\x85V[a\x0C\xAFV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x03\xD4V[a\x03\xCAa\x05h6`\x04aF\xBAV[`\x9B` R_\x90\x81R`@\x90 T\x81V[`\x9ETa\x04\x1B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x1Ba\x05\x9A6`\x04aEkV[a\r=V[a\x03\xA9a\x05\xAD6`\x04aFjV[a\r\xC6V[a\x03\xA9a\x05\xC06`\x04aFjV[a\r\xD7V[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xFFa\x05\xFA6`\x04aFjV[a\r\xE8V[`@Qa\x03\xD4\x91\x90aF\xD3V[a\x03\xA9a\x06\x1A6`\x04aG\xF4V[a\x0EfV[a\x03\xA9a\x06-6`\x04aH\xA0V[a\x0E\xA5V[a\x03\xA9a\x06@6`\x04aI}V[a\x11\xEFV[a\x06Xa\x06S6`\x04aFjV[a\x14\x15V[`@Qa\x03\xD4\x91\x90aJ\x15V[a\x03\xA9a\x14\x87V[a\x05\x01a\x06{6`\x04aF\xBAV[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x03\xA9a\x06\x9A6`\x04aJ0V[a\x156V[`\x01Ta\x03\xCAV[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\x1Ba\x06\xDC6`\x04aEkV[a\x15RV[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xA9a\x07\x166`\x04aJbV[a\x15zV[a\x03\xA9a\x16<V[a\x03\xA9a\x16MV[`\xA1Ta\x05\x01\x90`\xFF\x16\x81V[a\x03\xA9a\x07F6`\x04aJ\xAEV[a\x16\xB1V[a\x03\xCAa\x07Y6`\x04aK\x9EV[a\x16\xC6V[a\x03\xF0a\x07l6`\x04aEkV[a\x17\x0FV[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\x1Ba\x17\x19V[`\x96Ta\x07\xAD\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03\xD4V[a\x03\xA9a\x07\xCD6`\x04aMYV[a\x171V[a\x03\xA9a\x07\xE06`\x04aN\xB5V[a\x19\x06V[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xCA\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x05\x01a\x08A6`\x04aF\xBAV[a\x19nV[a\x03\xA9a\x08T6`\x04aN\xF7V[a\x19xV[a\x03\xCA`\xA0T\x81V[a\x03\xA9a\x08p6`\x04aO\x96V[a\x1B\xA2V[`\xA1Ta\x05\x01\x90a\x01\0\x90\x04`\xFF\x16\x81V[a\x08\x9Aa\x08\x956`\x04aP\rV[a\x1E\x05V[`@Qa\x03\xD4\x91\x90aP\xB2V[a\x03\xCA\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81V[a\x03\xA9a\x08\xDC6`\x04aF9V[a\x1E\x13V[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x9CTa\x03\xCAV[a\t~a\t\x1E6`\x04aF\xBAV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x97\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x03\xD4V[a\x03\xA9a\x1E\xA4V[a\x03\xA9a\t\xC86`\x04aFjV[a\x1F\x19V[a\x03\xA9a\t\xDB6`\x04aEkV[a\x1F\x8FV[a\n\x0Ea\t\xEE6`\x04aFjV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x03\xD4\x91\x90aP\xEFV[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\nDW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82Q\x81\x10\x15a\x0B\x0BW_\x83\x82\x81Q\x81\x10a\nbWa\nbaP\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x99\x83R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x93\x95P\x91\x93\x90\x92\x90\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\n\xB5Wa\n\xB5aI\xE1V[`\x02\x81\x11\x15a\n\xC6Wa\n\xC6aI\xE1V[\x90RP\x80Q\x90\x91P_a\n\xD8\x82a \xA6V[\x90P_a\n\xED\x82`\x01`\x01`\xC0\x1B\x03\x16a \xB2V[\x90Pa\n\xFA\x85\x85\x83a!{V[PP`\x01\x90\x93\x01\x92Pa\nF\x91PPV[PPPV[_a\x0B\x1E`\x98\x85\x85\x85a\"]V[\x90P[\x93\x92PPPV[`\x013_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x0BPWa\x0BPaI\xE1V[\x14a\x0BnW`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x99` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\x0B\xAE\x90\x84\x90aQ?V[`@Q\x80\x91\x03\x90\xA2PV[a\x0B\xC1a#AV[`\xA0UV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CL\x91\x90aQQV[a\x0CiW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x81\x81\x16\x81\x14a\x0C\x8EW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\x97\x82a#\xA0V[PPV[a\x0C\xA3a#AV[a\x0C\xAC\x81a#\xD7V[PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R_\x83\x81R`\x98` R`@\x90 \x80T\x83\x90\x81\x10a\x0C\xEAWa\x0C\xEAaP\xFDV[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r7\x91\x90aQpV[a\r\xCEa#AV[a\x0C\xAC\x81a$\x01V[a\r\xDFa#AV[a\x0C\xAC\x81a$jV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\r7a\x0Ea\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x84`@Q` \x01a\x0EF\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a$\xD3V[a%\x1FV[a\x0Ena#AV[`\xA1T`\xFF\x16a\x0E\x91W`@Qc[w\x90\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\x9F\x84\x84\x84`\x01\x85a%\xA9V[PPPPV[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\x0E\xCEW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x13\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa)\xE7\x90PV[P\x83Q\x82\x14a\x0F5W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\x11\xE8W_\x84\x84\x83\x81\x81\x10a\x0FRWa\x0FRaP\xFDV[\x88Q\x92\x015`\xF8\x1C\x92P_\x91\x88\x91P\x84\x90\x81\x10a\x0FqWa\x0FqaP\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xE4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x08\x91\x90aQ\x8BV[c\xFF\xFF\xFF\xFF\x16\x81Q\x14a\x10.W`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80[\x82Q\x81\x10\x15a\x11\x8FW_\x83\x82\x81Q\x81\x10a\x10MWa\x10MaP\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x99\x83R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x93\x95P\x91\x93\x90\x92\x90\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x10\xA0Wa\x10\xA0aI\xE1V[`\x02\x81\x11\x15a\x10\xB1Wa\x10\xB1aI\xE1V[\x90RP\x80Q\x90\x91P_a\x10\xC3\x82a \xA6V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x8A\x16\x1C\x81\x16\x14a\x10\xF6W`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11a\x11(W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa\x11\x82\x83\x83\x8D\x8B\x8Ea\x11<\x82`\x01aQ\xBAV[\x92a\x11I\x93\x92\x91\x90aQ\xCDV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa!{\x92PPPV[P\x90\x92PP`\x01\x01a\x101V[P`\xFF\x83\x16_\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPP\x80`\x01\x01\x90Pa\x0F7V[PPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\rWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12&WP0;\x15\x80\x15a\x12&WP_T`\xFF\x16`\x01\x14[a\x12\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12\xAFW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12\xB8\x86a*\x1BV[a\x12\xC1\x85a$\x01V[a\x12\xCA\x83a#\xA0V[a\x12\xD3\x84a$jV[a\x12\xDC\x82a#\xD7V[`\x9C\x80T`\x01\x81\x81\x01\x83U_\x83\x90R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x91\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U\x84T\x80\x84\x01\x86U\x84\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x84T\x92\x83\x01\x90\x94U\x91\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x92\x16\x17\x90U`\xA1\x80Ta\x01\x01a\xFF\xFF\x19\x90\x91\x16\x17\x90U\x80\x15a\x14\rW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x99` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x14mWa\x14maI\xE1V[`\x02\x81\x11\x15a\x14~Wa\x14~aI\xE1V[\x90RP\x92\x91PPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xE9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\r\x91\x90aQQV[a\x15*W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x154_\x19a#\xA0V[V[a\x15>a#AV[\x81a\x15H\x81a*lV[a\x0B\x0B\x83\x83a*\x95V[`\x9C\x81\x81T\x81\x10a\x15aW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[a\x15\x82a+:V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 B\x90U`\x99\x90\x91R\x81 \x80T`\x96T\x91\x92\x90\x91a\x15\xBD\x90\x85\x90`\xFF\x16a)\xE7V[\x90P_a\x15\xC9\x83a \xA6V[\x90P`\x01\x80\x85\x01T`\xFF\x16`\x02\x81\x11\x15a\x15\xE5Wa\x15\xE5aI\xE1V[\x14\x80\x15a\x15\xFAWP`\x01`\x01`\xC0\x1B\x03\x82\x16\x15\x15[\x80\x15a\x16\x18WPa\x16\x18`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[\x15a\x14\rWa\x16'\x86\x86a+eV[`\xA1T`\xFF\x16\x15a\x14\rWa\x14\r\x86\x86a.%V[a\x16Da#AV[a\x154_a*\x1BV[a\x16Ua#AV[`\xA1T`\xFF\x16a\x16xW`@Qc[w\x90\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U`@Q\x7F\xA4\xCDB\x92\x0E\xD0\xD17+\xA4\x05\x1DEw'\x9F#o\xBB\xE6w\xA6\x7F?}d^\x82B]\xD9\x8D\x90_\x90\xA1V[a\x16\xB9a#AV[a\x0B\x0B\x83\x83\x83__a%\xA9V[_a\x17\x05\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87\x87\x87\x87\x87`@Q` \x01a\x0EF\x96\x95\x94\x93\x92\x91\x90aQ\xF4V[\x96\x95PPPPPPV[_a\r7\x82a \xA6V[_a\x17,`dT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\x01\x80T_\x91\x90\x81\x16\x03a\x17XW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1Ta\x01\0\x90\x04`\xFF\x16\x15a\x17\x81W`@Qc\x1Dv \x1F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x17\x8C3\x87a/\x94V[\x90Pa\x17\xD33\x82\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8D\x92P\x8B\x91P\x8A\x90Pa0\xC2V[`\x013_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x17\xFBWa\x17\xFBaI\xE1V[\x14a\x18\xFBW`@\x80Q\x80\x82\x01\x82R\x82\x81R`\x01` \x80\x83\x01\x82\x81R3_\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a\x18LWa\x18LaI\xE1V[\x02\x17\x90UPP`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pc\x99&\xEE}\x90a\x18\xA1\x903\x90\x87\x90`\x04\x01aRyV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18\xB8W__\xFD[PZ\xF1\x15\x80\x15a\x18\xCAW=__>=_\xFD[PP`@Q\x83\x92P3\x91P\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90_\x90\xA3[PPPPPPPPPV[a\x19\x0Ea2\xE8V[`\x01\x80T`\x02\x90\x81\x16\x03a\x195W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1T`\xFF\x16a\x19XW`@Qc[w\x90\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x19b\x83a31V[\x90Pa\x0E\x9F\x84\x82a+eV[_a\r7\x82a3\xD9V[`\x01\x80T_\x91\x90\x81\x16\x03a\x19\x9FW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1Ta\x01\0\x90\x04`\xFF\x16\x15a\x19\xC8W`@Qc\x1Dv \x1F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x19\xD33\x85a/\x94V[\x90P_a\x19\xE23\x83\x89\x89a3\xF0V[Q\x90P_[\x87Q\x81\x10\x15a\x1ApW_\x88\x82\x81Q\x81\x10a\x1A\x03Wa\x1A\x03aP\xFDV[\x01` \x90\x81\x01Q`\xF8\x1C_\x81\x81R`\x97\x90\x92R`@\x90\x91 T\x84Q\x91\x92Pc\xFF\xFF\xFF\xFF\x16\x90\x84\x90\x84\x90\x81\x10a\x1A:Wa\x1A:aP\xFDV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1AgW`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x19\xE7V[P`\x013_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x1A\x99Wa\x1A\x99aI\xE1V[\x14a\x1B\x99W`@\x80Q\x80\x82\x01\x82R\x83\x81R`\x01` \x80\x83\x01\x82\x81R3_\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a\x1A\xEAWa\x1A\xEAaI\xE1V[\x02\x17\x90UPP`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pc\x99&\xEE}\x90a\x1B?\x903\x90\x88\x90`\x04\x01aRyV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1BVW__\xFD[PZ\xF1\x15\x80\x15a\x1BhW=__>=_\xFD[PP`@Q\x84\x92P3\x91P\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90_\x90\xA3[PPPPPPPV[a\x1B\xAAa2\xE8V[`\x01\x80T_\x91\x90\x81\x16\x03a\x1B\xD1W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1T`\xFF\x16a\x1B\xF4W`@Qc[w\x90\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1B\xFE\x85a31V[\x90P_\x80\x80a\x1C\x0F\x86\x88\x01\x88aR\xD1V[\x92P\x92P\x92P_a\x1C \x8A\x83a/\x94V[\x90P_\x84`\x01\x81\x11\x15a\x1C5Wa\x1C5aI\xE1V[\x03a\x1C\xDCW_a\x1CG\x8B\x83\x88\x87a3\xF0V[Q\x90P_[\x86Q\x81\x10\x15a\x1C\xD5W_\x87\x82\x81Q\x81\x10a\x1ChWa\x1ChaP\xFDV[\x01` \x90\x81\x01Q`\xF8\x1C_\x81\x81R`\x97\x90\x92R`@\x90\x91 T\x84Q\x91\x92Pc\xFF\xFF\xFF\xFF\x16\x90\x84\x90\x84\x90\x81\x10a\x1C\x9FWa\x1C\x9FaP\xFDV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1C\xCCW`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x1CLV[PPa\x1D1V[`\x01\x84`\x01\x81\x11\x15a\x1C\xF0Wa\x1C\xF0aI\xE1V[\x03a\x1D\x18W_\x80a\x1D\x03\x89\x8B\x01\x8BaS,V[\x94P\x94PPPPa\x1C\xD5\x8C\x84\x89\x88\x86\x86a0\xC2V[`@Qc5K\xB8\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\x01`\xA0\x1B\x03\x8B\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x1DbWa\x1DbaI\xE1V[\x14a\x1D\xF9W`@\x80Q\x80\x82\x01\x82R\x82\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16_\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a\x1D\xBCWa\x1D\xBCaI\xE1V[\x02\x17\x90UPP`@Q\x82\x91P`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90_\x90\xA3[PPPPPPPPPPV[``a\x0B!`\x98\x84\x84a6\xD4V[`\x01\x80T`\x02\x90\x81\x16\x03a\x1E:W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82Q\x81\x10\x15a\x1E\x99W`\xA1T`\xFF\x16\x15\x80a\x1EtWPa\x1Et\x83\x82\x81Q\x81\x10a\x1EgWa\x1EgaP\xFDV[\x01` \x01Q`\xF8\x1Ca3\xD9V[a\x1E\x91W`@Qc\xB2\xE1\x8E\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\x1E<V[Pa\x0C\x973\x83a+eV[a\x1E\xACa#AV[`\xA1T`\xFF\x16\x15a\x1E\xD0W`@Qc\xB2\xE1\x8E\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x96Ta\x1E\xDF\x90`\xFF\x16a7\x83V[`\xA2U`\xA1\x80T`\xFF\x19\x16`\x01\x17\x90U`@Q\x7F\x0B\x880o\xF4bq!\xF5\xB3\xE5\xB1\xC5\xF8\x8Fk\x1EB\xFD,\x04x\xEF\x1C\x91f-I\xD1\xF0wU\x90_\x90\xA1V[a\x1F!a#AV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1F\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x12\x85V[a\x0C\xAC\x81a*\x1BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x0F\x91\x90aQpV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a @W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x80\x19\x82\x19\x81\x16\x14a gW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPV[_a\r7`\x98\x83a7\x91V[``__a \xBF\x84a7\xFBV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a \xDAWa \xDAaD\x03V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a!\x04W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_\x80[\x82Q\x82\x10\x80\x15a!\x1BWPa\x01\0\x81\x10[\x15a!qW`\x01\x81\x1B\x93P\x85\x84\x16\x15a!aW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a!DWa!DaP\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x81`\x01\x01\x91P[a!j\x81aS\xDCV[\x90Pa!\nV[P\x90\x94\x93PPPPV[`\x01\x82` \x01Q`\x02\x81\x11\x15a!\x93Wa!\x93aI\xE1V[\x14a!\x9DWPPPV[\x81Q`@Qc3V\x7F\x7F`\xE1\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cf\xAC\xFE\xFE\x90a!\xF1\x90\x88\x90\x86\x90\x88\x90`\x04\x01aS\xF4V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"1\x91\x90aT#V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15a\x11\xE8Wa\x11\xE8\x85a\"X\x83`\x01`\x01`\xC0\x1B\x03\x16a \xB2V[a+eV[_\x83\x81R` \x85\x90R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\"\x7FWa\"\x7FaP\xFDV[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\"\xEFW`@Qcl\xB1\x9A\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a#\x15WP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a#2W`@Qc\xBB\xBA`\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x01Q\x90P[\x94\x93PPPPV[3a#Ja\x17\x19V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x154W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x12\x85V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\x0B\xAEV[`\xA1\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_a\r7a$\xDFa8%V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R_\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a%L_Q` aY\x85_9_Q\x90_R\x86aT]V[\x90P[a%X\x81a9KV[\x90\x93P\x91P_Q` aY\x85_9_Q\x90_R\x82\x83\t\x83\x03a%\x90W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` aY\x85_9_Q\x90_R`\x01\x82\x08\x90Pa%OV[`\x96T`\xFF\x16`\xC0\x81\x10a%\xD0W`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a%\xDB\x81`\x01aTpV[`\x96\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a%\xFA\x81\x88a*\x95V[`\xA1T`\xFF\x16\x80\x15a&\x12WPa&\x10\x81a3\xD9V[\x15[\x15a'\xBDW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a&-W\x90PP\x90P_\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&pWa&paD\x03V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&\x99W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x87Q\x81\x10\x15a&\xF6W\x87\x81\x81Q\x81\x10a&\xB9Wa&\xB9aP\xFDV[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a&\xD6Wa&\xD6aP\xFDV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a&\x9EV[P`@Q\x80`@\x01`@R\x80\x84`\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x82_\x81Q\x81\x10a''Wa''aP\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xA1T`@Qc\x010\xFC'`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92c&\x1F\x84\xE0\x92a'\x8D\x92b\x01\0\0\x90\x92\x04\x90\x91\x16\x90\x86\x90`\x04\x01aT\x89V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'\xA4W__\xFD[PZ\xF1\x15\x80\x15a'\xB6W=__>=_\xFD[PPPPPP[_\x84`\x01\x81\x11\x15a'\xD0Wa'\xD0aI\xE1V[\x03a(WW`@Qc:\xEA\x0B\x9D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cu\xD4\x17:\x90a(%\x90\x84\x90\x8A\x90\x8A\x90`\x04\x01aU\xA3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(<W__\xFD[PZ\xF1\x15\x80\x15a(NW=__>=_\xFD[PPPPa(\xF0V[`\x01\x84`\x01\x81\x11\x15a(kWa(kaI\xE1V[\x03a(\xF0W`@Qc\x06b\xD3\xE1`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xCCZ| \x90a(\xC2\x90\x84\x90\x8A\x90\x88\x90\x8B\x90`\x04\x01aU\xCDV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(\xD9W__\xFD[PZ\xF1\x15\x80\x15a(\xEBW=__>=_\xFD[PPPP[`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c&\xD9A\xF2\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a)QW__\xFD[PZ\xF1\x15\x80\x15a)cW=__>=_\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a)\xC8W__\xFD[PZ\xF1\x15\x80\x15a)\xDAW=__>=_\xFD[PPPPPPPPPPPV[__a)\xF2\x84a9\xC7V[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x0B!W`@Qc\xCA\x95s3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\x96T`\xFF\x90\x81\x16\x90\x82\x16\x10a\x0C\xACW`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x16_\x81\x81R`\x97` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01a \x9AV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\x154W`@Qcv\xD8\xAB\x17`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x99` R`@\x81 \x80T\x90\x91a+\x89\x82a \xA6V[\x90P`\x01\x80\x84\x01T`\xFF\x16`\x02\x81\x11\x15a+\xA5Wa+\xA5aI\xE1V[\x14a+\xC3W`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x96T_\x90a+\xD6\x90\x86\x90`\xFF\x16a)\xE7V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a+\xFFW`@Qch\xB6\xA8u`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,\x16`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x90\x84\x16\x81\x16\x14\x90V[a,3W`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xC0\x1B\x03\x81\x81\x16\x19\x83\x16\x16a,L\x84\x82a:\x87V[`\x01`\x01`\xC0\x1B\x03\x81\x16a,\xA8W`\x01`\x01`\xA0\x1B\x03\x87\x16_\x81\x81R`\x99` R`@\x80\x82 `\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UQ\x86\x92\x91\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x91\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a,\xF6\x90\x8A\x90\x8A\x90`\x04\x01aV\x03V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\rW__\xFD[PZ\xF1\x15\x80\x15a-\x1FW=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa-q\x90\x87\x90\x8A\x90`\x04\x01aV&V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\x88W__\xFD[PZ\xF1\x15\x80\x15a-\x9AW=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa-\xEC\x90\x87\x90\x8A\x90`\x04\x01aV&V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a.\x03W__\xFD[PZ\xF1\x15\x80\x15a.\x15W=__>=_\xFD[PPPPa\x1B\x99\x87\x85\x88\x84a:\x93V[_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a.?Wa.?aD\x03V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a.hW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x83Q\x81\x10\x15a.\xE7W_\x84\x82\x81Q\x81\x10a.\x8AWa.\x8AaP\xFDV[\x01` \x01Q`\xF8\x1C\x90Pa.\x9D\x81a3\xD9V[\x15a.\xDEW`\xFF\x81\x16\x84\x84a.\xB1\x81aS\xDCV[\x95P\x81Q\x81\x10a.\xC3Wa.\xC3aP\xFDV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPP[P`\x01\x01a.nV[P\x80\x15a\x0E\x9FW\x80\x82R`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R`\xA1Tb\x01\0\0\x90\x04\x81\x16` \x83\x01R\x81\x83\x01\x85\x90R\x91Qcn4\x92\xB5`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cn4\x92\xB5\x91a/a\x91`\x04\x01aV>V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/xW__\xFD[PZ\xF1\x15\x80\x15a/\x8AW=__>=_\xFD[PPPPPPPPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xFCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0 \x91\x90aV\xACV[\x90P_\x81\x90\x03a\r7W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBFy\xCEX\x84\x84a0d\x87a\r\xE8V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\x82\x93\x92\x91\x90aV\xE5V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B!\x91\x90aV\xACV[\x83Q\x82Q\x14a0\xE4W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a0\xF0\x86\x86\x84\x84a;+V[_a0\xFD\x87\x87\x87\x87a3\xF0V[\x90P_[\x85Q\x81\x10\x15a/\x8AW_`\x97_\x88\x84\x81Q\x81\x10a1 Wa1 aP\xFDV[\x01` \x90\x81\x01Q`\xF8\x1C\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a1\x8BWa1\x8BaP\xFDV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a2\xDFWa2\x1F\x87\x83\x81Q\x81\x10a1\xB4Wa1\xB4aP\xFDV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a1\xD8Wa1\xD8aP\xFDV[` \x02` \x01\x01Q\x8B\x86` \x01Q\x86\x81Q\x81\x10a1\xF7Wa1\xF7aP\xFDV[` \x02` \x01\x01Q\x89\x87\x81Q\x81\x10a2\x11Wa2\x11aP\xFDV[` \x02` \x01\x01Q\x86a;\xD6V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a2QWa2QaP\xFDV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x81_\x81Q\x81\x10a2nWa2naP\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa2\xAB\x86\x84\x81Q\x81\x10a2\x99Wa2\x99aP\xFDV[` \x02` \x01\x01Q` \x01Q\x82a+eV[`\xA1T`\xFF\x16\x15a2\xDDWa2\xDD\x86\x84\x81Q\x81\x10a2\xCBWa2\xCBaP\xFDV[` \x02` \x01\x01Q` \x01Q\x82a.%V[P[P`\x01\x01a1\x01V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x154W`@Qc#\xD8q\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a3MWa3MaD\x03V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a3wW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a3\xD2W\x83\x81\x81Q\x81\x10a3\x97Wa3\x97aP\xFDV[` \x02` \x01\x01Q`\xF8\x1B\x82\x82\x81Q\x81\x10a3\xB4Wa3\xB4aP\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a3|V[P\x92\x91PPV[`\xA2T_\x90a\r7\x90\x83`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a4\x14`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\x96T_\x90a4'\x90\x85\x90`\xFF\x16a)\xE7V[\x90P_a43\x86a \xA6V[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a4\\W`@Qc\x13\xCAFW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a4\x86W`@Qc\x0Ch\x16\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA0T`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9F` R`@\x90 T`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17\x91B\x91a4\xBD\x91\x90aQ\xBAV[\x10a4\xDBW`@Qc\x19hg}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a4\xE5\x87\x82a:\x87V[\x86\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x86`@Qa5\x15\x91\x90aQ?V[`@Q\x80\x91\x03\x90\xA2`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a5k\x90\x8B\x90\x8A\x90`\x04\x01aV\x03V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5\x82W__\xFD[PZ\xF1\x15\x80\x15a5\x94W=__>=_\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa5\xE8\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aS\xF4V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a6\x03W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6*\x91\x90\x81\x01\x90aW\xC2V[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a6\x85\x90\x8A\x90\x8A\x90`\x04\x01aV&V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a6\xA0W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6\xC7\x91\x90\x81\x01\x90aX\x1BV[\x84RPPP\x94\x93PPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a6\xF0Wa6\xF0aD\x03V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7\x19W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a7zWa7K\x86\x86\x86\x84\x81Q\x81\x10a7>Wa7>aP\xFDV[` \x02` \x01\x01Qa=LV[\x82\x82\x81Q\x81\x10a7]Wa7]aP\xFDV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a7\x1EV[P\x94\x93PPPPV[_a\r7`\x01\x80\x84\x1BaX\xAAV[_\x81\x81R` \x83\x90R`@\x81 T\x80\x82\x03a7\xAFW_\x91PPa\r7V[_\x83\x81R` \x85\x90R`@\x90 a7\xC7`\x01\x83aX\xAAV[\x81T\x81\x10a7\xD7Wa7\xD7aP\xFDV[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x91Pa\r7\x90PV[_\x80[\x82\x15a\r7Wa8\x0F`\x01\x84aX\xAAV[\x90\x92\x16\x91\x80a8\x1D\x81aX\xBDV[\x91PPa7\xFEV[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a8}WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a8\xA7WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[_\x80\x80_Q` aY\x85_9_Q\x90_R`\x03_Q` aY\x85_9_Q\x90_R\x86_Q` aY\x85_9_Q\x90_R\x88\x89\t\t\x08\x90P_a9\xBB\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` aY\x85_9_Q\x90_Ra>dV[\x91\x95\x91\x94P\x90\x92PPPV[_a\x01\0\x82Q\x11\x15a9\xECW`@Qc}\xA5NG`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q_\x03a9\xFBWP_\x91\x90PV[__\x83_\x81Q\x81\x10a:\x0FWa:\x0FaP\xFDV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a:yW\x84\x81\x81Q\x81\x10a:=Wa:=aP\xFDV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a:mW`@Qc\x10\x19\x10i`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x81\x17\x91`\x01\x01a:\"V[P\x90\x93\x92PPPV[\x19\x16\x90V[a\x0C\x97`\x98\x83\x83a>\xDDV[_a:\xB2`\xA2T\x83`\x01`\x01`\xC0\x1B\x03\x16a:\x82\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80a\x11\xE8W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a;\x19W__\xFD[PZ\xF1\x15\x80\x15a\x18\xFBW=__>=_\xFD[` \x80\x82\x01Q_\x90\x81R`\x9A\x90\x91R`@\x90 T`\xFF\x16\x15a;`W`@Qco\xBE\xFE\xC3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x81`@\x01Q\x10\x15a;\x85W`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x82\x01\x80Q_\x90\x81R`\x9A\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9DT\x90Q\x91\x83\x01Qa\x0E\x9F\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a;\xCF\x91\x88\x91\x88\x91\x88\x91\x90a\x16\xC6V[\x83Qa@\x96V[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x81\x81R`\x99\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x03a<\x19W`@QcV\x16\x8BA`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87`\xFF\x16\x84_\x01Q`\xFF\x16\x14a<BW`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\xB0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xD4\x91\x90aX\xDDV[\x90Pa<\xE0\x81\x85a@\xBEV[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11a=\x11W`@QcLD\x99]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a=\x1B\x88\x85a@\xE1V[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a\x18\xFBW`@Qc\xB1\x87\xE8i`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R` \x84\x90R`@\x81 T\x81[\x81\x81\x10\x15a=\xCFW`\x01a=p\x82\x84aX\xAAV[a=z\x91\x90aX\xAAV[\x92P\x84c\xFF\xFF\xFF\xFF\x16\x86_\x86\x81R` \x01\x90\x81R` \x01_ \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a=\xAAWa=\xAAaP\xFDV[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a=\xC7WPPa\x0B!V[`\x01\x01a=\\V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x12\x85V[__a>naC\xC7V[a>vaC\xE5V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a>\xB3W\xFE[P\x82a>\xD2W`@Qc\xD5\x1E\xDA\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x95\x94PPPPPV[_\x82\x81R` \x84\x90R`@\x81 T\x90\x81\x90\x03a?\x81W_\x83\x81R` \x85\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\x0E\x9FV[_\x83\x81R` \x85\x90R`@\x81 a?\x99`\x01\x84aX\xAAV[\x81T\x81\x10a?\xA9Wa?\xA9aP\xFDV[_\x91\x82R` \x90\x91 \x01\x80T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a?\xEAW\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\x11\xE8V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U_\x87\x81R` \x89\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPPV[a@\xA1\x83\x83\x83a@\xFAV[a\x0B\x0BW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_\x90a'\x10\x90a@\xD7\x90a\xFF\xFF\x16\x85aX\xF8V[a\x0B!\x91\x90aY\x1AV[`@\x81\x01Q_\x90a'\x10\x90a@\xD7\x90a\xFF\xFF\x16\x85aX\xF8V[___aA\x07\x85\x85aB?V[\x90\x92P\x90P_\x81`\x04\x81\x11\x15aA\x1FWaA\x1FaI\xE1V[\x14\x80\x15aA=WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15aAMW`\x01\x92PPPa\x0B!V[__\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01aAt\x92\x91\x90aV&V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90QaA\xB2\x91\x90aYGV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14aA\xEAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aA\xEFV[``\x91P[P\x91P\x91P\x81\x80\x15aB\x02WP\x80Q` \x14[\x80\x15aB3WP\x80Qc\x0B\x13]?`\xE1\x1B\x90aB'\x90\x83\x01` \x90\x81\x01\x90\x84\x01aY]V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[__\x82Q`A\x03aBsW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1AaBg\x87\x82\x85\x85aB\xAAV[\x94P\x94PPPPaB\xA3V[\x82Q`@\x03aB\x9CW` \x83\x01Q`@\x84\x01QaB\x91\x86\x83\x83aC\x8FV[\x93P\x93PPPaB\xA3V[P_\x90P`\x02[\x92P\x92\x90PV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aB\xDFWP_\x90P`\x03aC\x86V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aB\xF7WP\x84`\xFF\x16`\x1C\x14\x15[\x15aC\x07WP_\x90P`\x04aC\x86V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aCXW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aC\x80W_`\x01\x92P\x92PPaC\x86V[\x91P_\x90P[\x94P\x94\x92PPPV[_\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aC\xAB`\xFF\x86\x90\x1C`\x1BaQ\xBAV[\x90PaC\xB9\x87\x82\x88\x85aB\xAAV[\x93P\x93PPP\x93P\x93\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD9WaD9aD\x03V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD9WaD9aD\x03V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD\x89WaD\x89aD\x03V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aD\xA9WaD\xA9aD\x03V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\xACW__\xFD[_\x82`\x1F\x83\x01\x12aD\xD6W__\xFD[\x815aD\xE9aD\xE4\x82aD\x91V[aDaV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aE\nW__\xFD[` \x85\x01[\x83\x81\x10\x15aE0W\x805aE\"\x81aD\xB3V[\x83R` \x92\x83\x01\x92\x01aE\x0FV[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15aEJW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aE_W__\xFD[a#9\x84\x82\x85\x01aD\xC7V[_` \x82\x84\x03\x12\x15aE{W__\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\xACW__\xFD[___``\x84\x86\x03\x12\x15aE\xA5W__\xFD[\x835\x92P` \x84\x015aE\xB7\x81aE\x82V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_\x82`\x1F\x83\x01\x12aE\xD7W__\xFD[\x815` \x83\x01__`\x01`\x01`@\x1B\x03\x84\x11\x15aE\xF6WaE\xF6aD\x03V[P`\x1F\x83\x01`\x1F\x19\x16` \x01aF\x0B\x81aDaV[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15aF\x1FW__\xFD[\x82\x82` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15aFIW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aF^W__\xFD[a#9\x84\x82\x85\x01aE\xC8V[_` \x82\x84\x03\x12\x15aFzW__\xFD[\x815a\x0B!\x81aD\xB3V[__`@\x83\x85\x03\x12\x15aF\x96W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14aF\xB5W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15aF\xCAW__\xFD[a\x0B!\x82aF\xA5V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\r7V[\x805a\xFF\xFF\x81\x16\x81\x14aF\xB5W__\xFD[_``\x82\x84\x03\x12\x15aG\x0BW__\xFD[aG\x13aD\x17V[\x90P\x815aG \x81aE\x82V[\x81RaG.` \x83\x01aF\xEAV[` \x82\x01RaG?`@\x83\x01aF\xEAV[`@\x82\x01R\x92\x91PPV[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x0C\xACW__\xFD[_\x82`\x1F\x83\x01\x12aGmW__\xFD[\x815aG{aD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aG\x9CW__\xFD[` \x85\x01[\x83\x81\x10\x15aE0W`@\x81\x88\x03\x12\x15aG\xB8W__\xFD[aG\xC0aD?V[\x815aG\xCB\x81aD\xB3V[\x81R` \x82\x015aG\xDB\x81aGJV[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aG\xA1V[____`\xC0\x85\x87\x03\x12\x15aH\x07W__\xFD[aH\x11\x86\x86aF\xFBV[\x93P``\x85\x015aH!\x81aGJV[\x92P`\x80\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aH;W__\xFD[aHG\x87\x82\x88\x01aG^V[\x92PP`\xA0\x85\x015aHX\x81aE\x82V[\x93\x96\x92\x95P\x90\x93PPV[__\x83`\x1F\x84\x01\x12aHsW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aH\x89W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aB\xA3W__\xFD[___`@\x84\x86\x03\x12\x15aH\xB2W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xC7W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aH\xD7W__\xFD[\x805aH\xE5aD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15aI\x06W__\xFD[` \x84\x01[\x83\x81\x10\x15aIFW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aI(W__\xFD[aI7\x8B` \x83\x89\x01\x01aD\xC7V[\x84RP` \x92\x83\x01\x92\x01aI\x0BV[P\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aIdW__\xFD[aIp\x86\x82\x87\x01aHcV[\x94\x97\x90\x96P\x93\x94PPPPV[_____`\xA0\x86\x88\x03\x12\x15aI\x91W__\xFD[\x855aI\x9C\x81aD\xB3V[\x94P` \x86\x015aI\xAC\x81aD\xB3V[\x93P`@\x86\x015aI\xBC\x81aD\xB3V[\x92P``\x86\x015\x91P`\x80\x86\x015aI\xD3\x81aD\xB3V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10aJ\x11WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91a3\xD2\x90\x84\x01\x82aI\xF5V[__`\x80\x83\x85\x03\x12\x15aJAW__\xFD[aJJ\x83aF\xA5V[\x91PaJY\x84` \x85\x01aF\xFBV[\x90P\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aJsW__\xFD[\x825aJ~\x81aD\xB3V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\x98W__\xFD[aJ\xA4\x85\x82\x86\x01aE\xC8V[\x91PP\x92P\x92\x90PV[___`\xA0\x84\x86\x03\x12\x15aJ\xC0W__\xFD[aJ\xCA\x85\x85aF\xFBV[\x92P``\x84\x015aJ\xDA\x81aGJV[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xF4W__\xFD[aK\0\x86\x82\x87\x01aG^V[\x91PP\x92P\x92P\x92V[_\x82`\x1F\x83\x01\x12aK\x19W__\xFD[\x815aK'aD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aKHW__\xFD[` \x85\x01[\x83\x81\x10\x15aE0W`@\x81\x88\x03\x12\x15aKdW__\xFD[aKlaD?V[aKu\x82aF\xA5V[\x81R` \x82\x015aK\x85\x81aD\xB3V[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aKMV[_____`\xA0\x86\x88\x03\x12\x15aK\xB2W__\xFD[\x855aK\xBD\x81aD\xB3V[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xDEW__\xFD[aK\xEA\x88\x82\x89\x01aK\nV[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x015\x93\x92PPPV[_`@\x82\x84\x03\x12\x15aL\x13W__\xFD[aL\x1BaD?V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12aLAW__\xFD[aLIaD?V[\x80`@\x84\x01\x85\x81\x11\x15aLZW__\xFD[\x84[\x81\x81\x10\x15aLtW\x805\x84R` \x93\x84\x01\x93\x01aL\\V[P\x90\x95\x94PPPPPV[_\x81\x83\x03a\x01\0\x81\x12\x15aL\x91W__\xFD[aL\x99aD\x17V[\x91PaL\xA5\x84\x84aL\x03V[\x82RaL\xB4\x84`@\x85\x01aL\x03V[` \x83\x01R`\x80`\x7F\x19\x82\x01\x12\x15aL\xCAW__\xFD[PaL\xD3aD?V[aL\xE0\x84`\x80\x85\x01aL2V[\x81RaL\xEF\x84`\xC0\x85\x01aL2V[` \x82\x01R`@\x82\x01R\x92\x91PPV[_``\x82\x84\x03\x12\x15aM\x0FW__\xFD[aM\x17aD\x17V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aM.W__\xFD[aM:\x84\x82\x85\x01aE\xC8V[\x82RP` \x82\x81\x015\x90\x82\x01R`@\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[_______a\x01\xA0\x88\x8A\x03\x12\x15aMpW__\xFD[\x875`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x85W__\xFD[aM\x91\x8A\x82\x8B\x01aHcV[\x90\x98P\x96PP` \x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xAFW__\xFD[aM\xBB\x8A\x82\x8B\x01aE\xC8V[\x95PPaM\xCB\x89`@\x8A\x01aL\x7FV[\x93Pa\x01@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xE6W__\xFD[aM\xF2\x8A\x82\x8B\x01aK\nV[\x93PPa\x01`\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN\x0EW__\xFD[aN\x1A\x8A\x82\x8B\x01aL\xFFV[\x92PPa\x01\x80\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN6W__\xFD[aNB\x8A\x82\x8B\x01aL\xFFV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_\x82`\x1F\x83\x01\x12aN`W__\xFD[\x815aNnaD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aN\x8FW__\xFD[` \x85\x01[\x83\x81\x10\x15aE0W\x805aN\xA7\x81aE\x82V[\x83R` \x92\x83\x01\x92\x01aN\x94V[__`@\x83\x85\x03\x12\x15aN\xC6W__\xFD[\x825aN\xD1\x81aD\xB3V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xEBW__\xFD[aJ\xA4\x85\x82\x86\x01aNQV[____a\x01`\x85\x87\x03\x12\x15aO\x0BW__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15aO W__\xFD[aO,\x87\x82\x88\x01aE\xC8V[\x94PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aOGW__\xFD[aOS\x87\x82\x88\x01aE\xC8V[\x93PPaOc\x86`@\x87\x01aL\x7FV[\x91Pa\x01@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO~W__\xFD[aO\x8A\x87\x82\x88\x01aL\xFFV[\x91PP\x92\x95\x91\x94P\x92PV[____``\x85\x87\x03\x12\x15aO\xA9W__\xFD[\x845aO\xB4\x81aD\xB3V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xCEW__\xFD[aO\xDA\x87\x82\x88\x01aNQV[\x93PP`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xF5W__\xFD[aP\x01\x87\x82\x88\x01aHcV[\x95\x98\x94\x97P\x95PPPPV[__`@\x83\x85\x03\x12\x15aP\x1EW__\xFD[\x825aP)\x81aE\x82V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aPCW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aPSW__\xFD[\x805aPaaD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aP\x82W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aP\xA4W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aP\x89V[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aLtW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aP\xCBV[` \x81\x01a\r7\x82\x84aI\xF5V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0B!` \x83\x01\x84aQ\x11V[_` \x82\x84\x03\x12\x15aQaW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0B!W__\xFD[_` \x82\x84\x03\x12\x15aQ\x80W__\xFD[\x81Qa\x0B!\x81aD\xB3V[_` \x82\x84\x03\x12\x15aQ\x9BW__\xFD[\x81Qa\x0B!\x81aE\x82V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\r7Wa\r7aQ\xA6V[__\x85\x85\x11\x15aQ\xDBW__\xFD[\x83\x86\x11\x15aQ\xE7W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[_`\xC0\x82\x01\x88\x83R`\x01\x80`\xA0\x1B\x03\x88\x16` \x84\x01R\x86`@\x84\x01R`\xC0``\x84\x01R\x80\x86Q\x80\x83R`\xE0\x85\x01\x91P` \x88\x01\x92P_[\x81\x81\x10\x15aRaW\x83Q\x80Q`\xFF\x16\x84R` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01aR+V[PP`\x80\x84\x01\x95\x90\x95RPP`\xA0\x01R\x94\x93PPPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q```@\x84\x01RaR\xA2`\xA0\x84\x01\x82aQ\x11V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[\x805`\x02\x81\x10aF\xB5W__\xFD[___a\x01@\x84\x86\x03\x12\x15aR\xE4W__\xFD[aR\xED\x84aR\xC3V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x07W__\xFD[aS\x13\x86\x82\x87\x01aE\xC8V[\x92PPaS#\x85`@\x86\x01aL\x7FV[\x90P\x92P\x92P\x92V[_____a\x01\x80\x86\x88\x03\x12\x15aSAW__\xFD[aSJ\x86aR\xC3V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aSdW__\xFD[aSp\x88\x82\x89\x01aE\xC8V[\x94PPaS\x80\x87`@\x88\x01aL\x7FV[\x92Pa\x01@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x9BW__\xFD[aS\xA7\x88\x82\x89\x01aK\nV[\x92PPa\x01`\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xC3W__\xFD[aS\xCF\x88\x82\x89\x01aL\xFFV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_`\x01\x82\x01aS\xEDWaS\xEDaQ\xA6V[P`\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R_aT\x1A``\x83\x01\x84aQ\x11V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15aT3W__\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x0B!W__\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aTkWaTkaTIV[P\x06\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\r7Wa\r7aQ\xA6V[_`@\x82\x01`\x01\x80`\xA0\x1B\x03\x85\x16\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aU>W\x86\x85\x03`_\x19\x01\x84R\x81Q\x80Qc\xFF\xFF\xFF\xFF\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15aU&W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90aT\xFBV[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aT\xBDV[P\x92\x97\x96PPPPPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aU\x99W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x88\x01R`@\x90\x96\x01\x95\x90\x91\x01\x90`\x01\x01aU]V[P\x93\x94\x93PPPPV[`\xFF\x84\x16\x81R`\x01`\x01``\x1B\x03\x83\x16` \x82\x01R```@\x82\x01R_aT\x1A``\x83\x01\x84aUKV[`\xFF\x85\x16\x81R`\x01`\x01``\x1B\x03\x84\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R_a\x17\x05`\x80\x83\x01\x84aUKV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x0B\x1E\x90\x83\x01\x84aQ\x11V[\x82\x81R`@` \x82\x01R_a\x0B\x1E`@\x83\x01\x84aQ\x11V[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x83\x01R\x83\x82\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x83\x01Q``\x80\x84\x01R\x80Q`\x80\x84\x01\x81\x90R_\x92\x91\x90\x91\x01\x90\x82\x90`\xA0\x85\x01\x90[\x80\x83\x10\x15aE0Wc\xFF\xFF\xFF\xFF\x84Q\x16\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92PaV\x83V[_` \x82\x84\x03\x12\x15aV\xBCW__\xFD[PQ\x91\x90PV[\x80_[`\x02\x81\x10\x15a\x0E\x9FW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aV\xC6V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R\x82Q\x80Q` \x80\x84\x01\x91\x90\x91R\x01Q`@\x82\x01Ra\x01`\x81\x01` \x84\x81\x01Q\x80Q``\x85\x01R\x90\x81\x01Q`\x80\x84\x01RP`@\x84\x01QaW4`\xA0\x84\x01\x82QaV\xC3V[` \x01QaWE`\xE0\x84\x01\x82aV\xC3V[P\x82Qa\x01 \x83\x01R` \x83\x01Qa\x01@\x83\x01Ra#9V[_\x82`\x1F\x83\x01\x12aWmW__\xFD[\x81QaW{aD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aW\x9CW__\xFD[` \x85\x01[\x83\x81\x10\x15aE0W\x80QaW\xB4\x81aGJV[\x83R` \x92\x83\x01\x92\x01aW\xA1V[__`@\x83\x85\x03\x12\x15aW\xD3W__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aW\xE8W__\xFD[aW\xF4\x85\x82\x86\x01aW^V[\x92PP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aX\x0FW__\xFD[aJ\xA4\x85\x82\x86\x01aW^V[_` \x82\x84\x03\x12\x15aX+W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aX@W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aXPW__\xFD[\x80QaX^aD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aX\x7FW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x17\x05W\x83QaX\x99\x81aE\x82V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aX\x86V[\x81\x81\x03\x81\x81\x11\x15a\r7Wa\r7aQ\xA6V[_a\xFF\xFF\x82\x16a\xFF\xFF\x81\x03aX\xD4WaX\xD4aQ\xA6V[`\x01\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15aX\xEDW__\xFD[\x81Qa\x0B!\x81aGJV[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a3\xD2Wa3\xD2aQ\xA6V[_`\x01`\x01``\x1B\x03\x83\x16\x80aY2WaY2aTIV[\x80`\x01`\x01``\x1B\x03\x84\x16\x04\x91PP\x92\x91PPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15aYmW__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B!W__\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xA2\xAD\xA9\x83M)a\x0B\xF3\xE5\xAC\xA5\x08\xA2\xB5\xA6\x1A+\x0C\x9F\x84n\x83\x83\xD3\x80\xCA\x01\x18\x1B\x0F\xDDdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610392575f3560e01c806368304835116101df578063a4d7871f11610109578063ca4f2d97116100a9578063ee31882111610079578063ee318821146109b2578063f2fde38b146109ba578063fabc1cbc146109cd578063fd39105a146109e0575f5ffd5b8063ca4f2d97146108ce578063ca8aa7c7146108e1578063d72d8dd614610908578063e65797ad14610910575f5ffd5b8063adcf73f7116100e4578063adcf73f714610862578063b2d8678d14610875578063c391425e14610887578063ca0de882146108a7575f5ffd5b8063a4d7871f14610833578063a50857bf14610846578063a96f783e14610859575f5ffd5b8063871ef0491161017f5780639b5d177b1161014f5780639b5d177b146107bf5780639d8e0c23146107d25780639e9923c2146107e55780639feab8591461080c575f5ffd5b8063871ef0491461075e578063886f1195146107715780638da5cb5b146107985780639aa1653d146107a0575f5ffd5b8063733b7507116101ba578063733b75071461072357806381f936d21461072b5780638281ab751461073857806384ca52131461074b575f5ffd5b806368304835146106e15780636e3b17db14610708578063715018a61461071b575f5ffd5b8063296bb064116102c0578063530b97a4116102605780635b0b829f116102305780635b0b829f1461068c5780635c975abb1461069f5780635df45946146106a75780636347c900146106ce575f5ffd5b8063530b97a4146106325780635865c60c14610645578063595c6a67146106655780635ac86ab71461066d575f5ffd5b80633998fdd31161029b5780633998fdd3146105c55780633c2a7f4c146105ec5780633eef3a511461060c5780635140a5481461061f575f5ffd5b8063296bb0641461058c57806329d1e0c31461059f5780632cdd1e86146105b2575f5ffd5b8063125e0584116103365780631478851f116103065780631478851f146104df5780631eb812da14610511578063249a0c421461055a57806328f61b3114610579575f5ffd5b8063125e05841461047257806313542a4e14610491578063136439dd146104b9578063143e5915146104cc575f5ffd5b8063054310e611610371578063054310e6146104085780630764cb93146104335780630cf4b7671461044c5780630d3f21341461045f575f5ffd5b8062cf2ab51461039657806303fd3492146103ab57806304ec6351146103dd575b5f5ffd5b6103a96103a436600461453a565b610a1b565b005b6103ca6103b936600461456b565b5f9081526098602052604090205490565b6040519081526020015b60405180910390f35b6103f06103eb366004614593565b610b10565b6040516001600160c01b0390911681526020016103d4565b609d5461041b906001600160a01b031681565b6040516001600160a01b0390911681526020016103d4565b60a15461041b906201000090046001600160a01b031681565b6103a961045a366004614639565b610b28565b6103a961046d36600461456b565b610bb9565b6103ca61048036600461466a565b609f6020525f908152604090205481565b6103ca61049f36600461466a565b6001600160a01b03165f9081526099602052604090205490565b6103a96104c736600461456b565b610bc6565b6103a96104da36600461466a565b610c9b565b6105016104ed36600461456b565b609a6020525f908152604090205460ff1681565b60405190151581526020016103d4565b61052461051f366004614685565b610caf565b60408051825163ffffffff908116825260208085015190911690820152918101516001600160c01b0316908201526060016103d4565b6103ca6105683660046146ba565b609b6020525f908152604090205481565b609e5461041b906001600160a01b031681565b61041b61059a36600461456b565b610d3d565b6103a96105ad36600461466a565b610dc6565b6103a96105c036600461466a565b610dd7565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b6105ff6105fa36600461466a565b610de8565b6040516103d491906146d3565b6103a961061a3660046147f4565b610e66565b6103a961062d3660046148a0565b610ea5565b6103a961064036600461497d565b6111ef565b61065861065336600461466a565b611415565b6040516103d49190614a15565b6103a9611487565b61050161067b3660046146ba565b6001805460ff9092161b9081161490565b6103a961069a366004614a30565b611536565b6001546103ca565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b61041b6106dc36600461456b565b611552565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b6103a9610716366004614a62565b61157a565b6103a961163c565b6103a961164d565b60a1546105019060ff1681565b6103a9610746366004614aae565b6116b1565b6103ca610759366004614b9e565b6116c6565b6103f061076c36600461456b565b61170f565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b61041b611719565b6096546107ad9060ff1681565b60405160ff90911681526020016103d4565b6103a96107cd366004614d59565b611731565b6103a96107e0366004614eb5565b611906565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b6103ca7f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de681565b6105016108413660046146ba565b61196e565b6103a9610854366004614ef7565b611978565b6103ca60a05481565b6103a9610870366004614f96565b611ba2565b60a15461050190610100900460ff1681565b61089a61089536600461500d565b611e05565b6040516103d491906150b2565b6103ca7f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a81565b6103a96108dc366004614639565b611e13565b61041b7f000000000000000000000000000000000000000000000000000000000000000081565b609c546103ca565b61097e61091e3660046146ba565b60408051606080820183525f808352602080840182905292840181905260ff9490941684526097825292829020825193840183525463ffffffff8116845261ffff600160201b8204811692850192909252600160301b9004169082015290565b60408051825163ffffffff16815260208084015161ffff9081169183019190915292820151909216908201526060016103d4565b6103a9611ea4565b6103a96109c836600461466a565b611f19565b6103a96109db36600461456b565b611f8f565b610a0e6109ee36600461466a565b6001600160a01b03165f9081526099602052604090206001015460ff1690565b6040516103d491906150ef565b600154600290600490811603610a445760405163840a48d560e01b815260040160405180910390fd5b5f5b8251811015610b0b575f838281518110610a6257610a626150fd565b6020908102919091018101516001600160a01b0381165f9081526099835260408082208151808301909252805482526001810154939550919390929083019060ff166002811115610ab557610ab56149e1565b6002811115610ac657610ac66149e1565b90525080519091505f610ad8826120a6565b90505f610aed826001600160c01b03166120b2565b9050610afa85858361217b565b505060019093019250610a46915050565b505050565b5f610b1e609885858561225d565b90505b9392505050565b6001335f9081526099602052604090206001015460ff166002811115610b5057610b506149e1565b14610b6e5760405163aba4733960e01b815260040160405180910390fd5b335f90815260996020526040908190205490517fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa90610bae90849061513f565b60405180910390a250565b610bc1612341565b60a055565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015610c28573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c4c9190615151565b610c6957604051631d77d47760e21b815260040160405180910390fd5b6001548181168114610c8e5760405163c61dca5d60e01b815260040160405180910390fd5b610c97826123a0565b5050565b610ca3612341565b610cac816123d7565b50565b604080516060810182525f80825260208201819052918101919091525f838152609860205260409020805483908110610cea57610cea6150fd565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160c01b03600160401b909304929092169082015290505b92915050565b6040516308f6629d60e31b8152600481018290525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906347b314e890602401602060405180830381865afa158015610da2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d379190615170565b610dce612341565b610cac81612401565b610ddf612341565b610cac8161246a565b604080518082019091525f8082526020820152610d37610e617f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de684604051602001610e469291909182526001600160a01b0316602082015260400190565b604051602081830303815290604052805190602001206124d3565b61251f565b610e6e612341565b60a15460ff16610e9157604051635b77901960e01b815260040160405180910390fd5b610e9f8484846001856125a9565b50505050565b600154600290600490811603610ece5760405163840a48d560e01b815260040160405180910390fd5b610f1383838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505060965460ff1691506129e79050565b5083518214610f355760405163aaad13f760e01b815260040160405180910390fd5b5f5b828110156111e8575f848483818110610f5257610f526150fd565b885192013560f81c92505f9188915084908110610f7157610f716150fd565b60209081029190910101516040516379a0849160e11b815260ff841660048201529091506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f341092290602401602060405180830381865afa158015610fe4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611008919061518b565b63ffffffff1681511461102e57604051638e5aeee760e01b815260040160405180910390fd5b5f805b825181101561118f575f83828151811061104d5761104d6150fd565b6020908102919091018101516001600160a01b0381165f9081526099835260408082208151808301909252805482526001810154939550919390929083019060ff1660028111156110a0576110a06149e1565b60028111156110b1576110b16149e1565b90525080519091505f6110c3826120a6565b905060016001600160c01b03821660ff8a161c8116146110f65760405163d053aa2160e01b815260040160405180910390fd5b856001600160a01b0316846001600160a01b0316116111285760405163ba50f91160e01b815260040160405180910390fd5b5061118283838d8b8e61113c8260016151ba565b92611149939291906151cd565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061217b92505050565b5090925050600101611031565b5060ff83165f818152609b6020908152604091829020439081905591519182527f46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db4910160405180910390a2505050806001019050610f37565b5050505050565b5f54610100900460ff161580801561120d57505f54600160ff909116105b806112265750303b15801561122657505f5460ff166001145b61128e5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156112af575f805461ff0019166101001790555b6112b886612a1b565b6112c185612401565b6112ca836123a0565b6112d38461246a565b6112dc826123d7565b609c8054600181810183555f8390527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c91820180546001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166001600160a01b03199283161790925584548084018655840180547f000000000000000000000000000000000000000000000000000000000000000084169083161790558454928301909455910180547f00000000000000000000000000000000000000000000000000000000000000009092169190921617905560a1805461010161ffff19909116179055801561140d575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050565b604080518082019091525f80825260208201526001600160a01b0382165f908152609960209081526040918290208251808401909352805483526001810154909183019060ff16600281111561146d5761146d6149e1565b600281111561147e5761147e6149e1565b90525092915050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa1580156114e9573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061150d9190615151565b61152a57604051631d77d47760e21b815260040160405180910390fd5b6115345f196123a0565b565b61153e612341565b8161154881612a6c565b610b0b8383612a95565b609c8181548110611561575f80fd5b5f918252602090912001546001600160a01b0316905081565b611582612b3a565b6001600160a01b0382165f908152609f60209081526040808320429055609990915281208054609654919290916115bd90859060ff166129e7565b90505f6115c9836120a6565b905060018085015460ff1660028111156115e5576115e56149e1565b1480156115fa57506001600160c01b03821615155b801561161857506116186001600160c01b0383811690831681161490565b1561140d576116278686612b65565b60a15460ff161561140d5761140d8686612e25565b611644612341565b6115345f612a1b565b611655612341565b60a15460ff1661167857604051635b77901960e01b815260040160405180910390fd5b60a1805461ff0019166101001790556040517fa4cd42920ed0d1372ba4051d4577279f236fbbe677a67f3f7d645e82425dd98d905f90a1565b6116b9612341565b610b0b8383835f5f6125a9565b5f6117057f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a8787878787604051602001610e46969594939291906151f4565b9695505050505050565b5f610d37826120a6565b5f61172c6064546001600160a01b031690565b905090565b600180545f91908116036117585760405163840a48d560e01b815260040160405180910390fd5b60a154610100900460ff161561178157604051631d76201f60e31b815260040160405180910390fd5b5f61178c3387612f94565b90506117d333828b8b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508d92508b91508a90506130c2565b6001335f9081526099602052604090206001015460ff1660028111156117fb576117fb6149e1565b146118fb5760408051808201825282815260016020808301828152335f908152609990925293902082518155925183820180549394939192909160ff19169083600281111561184c5761184c6149e1565b021790555050604051639926ee7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169150639926ee7d906118a19033908790600401615279565b5f604051808303815f87803b1580156118b8575f5ffd5b505af11580156118ca573d5f5f3e3d5ffd5b50506040518392503391507fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe905f90a35b505050505050505050565b61190e6132e8565b600180546002908116036119355760405163840a48d560e01b815260040160405180910390fd5b60a15460ff1661195857604051635b77901960e01b815260040160405180910390fd5b5f61196283613331565b9050610e9f8482612b65565b5f610d37826133d9565b600180545f919081160361199f5760405163840a48d560e01b815260040160405180910390fd5b60a154610100900460ff16156119c857604051631d76201f60e31b815260040160405180910390fd5b5f6119d33385612f94565b90505f6119e2338389896133f0565b5190505f5b8751811015611a70575f888281518110611a0357611a036150fd565b0160209081015160f81c5f8181526097909252604090912054845191925063ffffffff1690849084908110611a3a57611a3a6150fd565b602002602001015163ffffffff161115611a6757604051633cb89c9760e01b815260040160405180910390fd5b506001016119e7565b506001335f9081526099602052604090206001015460ff166002811115611a9957611a996149e1565b14611b995760408051808201825283815260016020808301828152335f908152609990925293902082518155925183820180549394939192909160ff191690836002811115611aea57611aea6149e1565b021790555050604051639926ee7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169150639926ee7d90611b3f9033908890600401615279565b5f604051808303815f87803b158015611b56575f5ffd5b505af1158015611b68573d5f5f3e3d5ffd5b50506040518492503391507fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe905f90a35b50505050505050565b611baa6132e8565b600180545f9190811603611bd15760405163840a48d560e01b815260040160405180910390fd5b60a15460ff16611bf457604051635b77901960e01b815260040160405180910390fd5b5f611bfe85613331565b90505f8080611c0f868801886152d1565b9250925092505f611c208a83612f94565b90505f846001811115611c3557611c356149e1565b03611cdc575f611c478b8388876133f0565b5190505f5b8651811015611cd5575f878281518110611c6857611c686150fd565b0160209081015160f81c5f8181526097909252604090912054845191925063ffffffff1690849084908110611c9f57611c9f6150fd565b602002602001015163ffffffff161115611ccc57604051633cb89c9760e01b815260040160405180910390fd5b50600101611c4c565b5050611d31565b6001846001811115611cf057611cf06149e1565b03611d18575f80611d03898b018b61532c565b94509450505050611cd58c84898886866130c2565b60405163354bb8ab60e01b815260040160405180910390fd5b60016001600160a01b038b165f9081526099602052604090206001015460ff166002811115611d6257611d626149e1565b14611df957604080518082018252828152600160208083018281526001600160a01b038f165f908152609990925293902082518155925183820180549394939192909160ff191690836002811115611dbc57611dbc6149e1565b0217905550506040518291506001600160a01b038c16907fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe905f90a35b50505050505050505050565b6060610b21609884846136d4565b60018054600290811603611e3a5760405163840a48d560e01b815260040160405180910390fd5b5f5b8251811015611e995760a15460ff161580611e745750611e74838281518110611e6757611e676150fd565b016020015160f81c6133d9565b611e915760405163b2e18e0560e01b815260040160405180910390fd5b600101611e3c565b50610c973383612b65565b611eac612341565b60a15460ff1615611ed05760405163b2e18e0560e01b815260040160405180910390fd5b609654611edf9060ff16613783565b60a25560a1805460ff191660011790556040517f0b88306ff4627121f5b3e5b1c5f88f6b1e42fd2c0478ef1c91662d49d1f07755905f90a1565b611f21612341565b6001600160a01b038116611f865760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401611285565b610cac81612a1b565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611feb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061200f9190615170565b6001600160a01b0316336001600160a01b0316146120405760405163794821ff60e01b815260040160405180910390fd5b600154801982198116146120675760405163c61dca5d60e01b815260040160405180910390fd5b600182905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020015b60405180910390a25050565b5f610d37609883613791565b60605f5f6120bf846137fb565b61ffff166001600160401b038111156120da576120da614403565b6040519080825280601f01601f191660200182016040528015612104576020820181803683370190505b5090505f805b82518210801561211b575061010081105b15612171576001811b935085841615612161578060f81b838381518110612144576121446150fd565b60200101906001600160f81b03191690815f1a9053508160010191505b61216a816153dc565b905061210a565b5090949350505050565b600182602001516002811115612193576121936149e1565b1461219d57505050565b81516040516333567f7f60e11b81525f906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906366acfefe906121f1908890869088906004016153f4565b6020604051808303815f875af115801561220d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122319190615423565b90506001600160c01b038116156111e8576111e885612258836001600160c01b03166120b2565b612b65565b5f83815260208590526040812080548291908490811061227f5761227f6150fd565b5f91825260209182902060408051606081018252929091015463ffffffff808216808552600160201b8304821695850195909552600160401b9091046001600160c01b031691830191909152909250851610156122ef57604051636cb19aff60e01b815260040160405180910390fd5b602081015163ffffffff1615806123155750806020015163ffffffff168463ffffffff16105b6123325760405163bbba60cb60e01b815260040160405180910390fd5b6040015190505b949350505050565b3361234a611719565b6001600160a01b0316146115345760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401611285565b600181905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d90602001610bae565b60a180546001600160a01b03909216620100000262010000600160b01b0319909216919091179055565b609d54604080516001600160a01b03928316815291831660208301527f315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c910160405180910390a1609d80546001600160a01b0319166001600160a01b0392909216919091179055565b609e54604080516001600160a01b03928316815291831660208301527f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9910160405180910390a1609e80546001600160a01b0319166001600160a01b0392909216919091179055565b5f610d376124df613825565b8360405161190160f01b602082015260228101839052604281018290525f9060620160405160208183030381529060405280519060200120905092915050565b604080518082019091525f80825260208201525f808061254c5f5160206159855f395f51905f528661545d565b90505b6125588161394b565b90935091505f5160206159855f395f51905f528283098303612590576040805180820190915290815260208101919091529392505050565b5f5160206159855f395f51905f5260018208905061254f565b60965460ff1660c081106125d057604051633cb89c9760e01b815260040160405180910390fd5b6125db816001615470565b6096805460ff191660ff92909216919091179055806125fa8188612a95565b60a15460ff1680156126125750612610816133d9565b155b156127bd576040805160018082528183019092525f91816020015b604080518082019091525f81526060602082015281526020019060019003908161262d5790505090505f86516001600160401b0381111561267057612670614403565b604051908082528060200260200182016040528015612699578160200160208202803683370190505b5090505f5b87518110156126f6578781815181106126b9576126b96150fd565b60200260200101515f01518282815181106126d6576126d66150fd565b6001600160a01b039092166020928302919091019091015260010161269e565b5060405180604001604052808460ff1663ffffffff16815260200182815250825f81518110612727576127276150fd565b602090810291909101015260a154604051630130fc2760e51b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169263261f84e09261278d9262010000909204909116908690600401615489565b5f604051808303815f87803b1580156127a4575f5ffd5b505af11580156127b6573d5f5f3e3d5ffd5b5050505050505b5f8460018111156127d0576127d06149e1565b0361285757604051633aea0b9d60e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906375d4173a906128259084908a908a906004016155a3565b5f604051808303815f87803b15801561283c575f5ffd5b505af115801561284e573d5f5f3e3d5ffd5b505050506128f0565b600184600181111561286b5761286b6149e1565b036128f057604051630662d3e160e51b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063cc5a7c20906128c29084908a9088908b906004016155cd565b5f604051808303815f87803b1580156128d9575f5ffd5b505af11580156128eb573d5f5f3e3d5ffd5b505050505b60405163136ca0f960e11b815260ff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906326d941f2906024015f604051808303815f87803b158015612951575f5ffd5b505af1158015612963573d5f5f3e3d5ffd5b505060405163136ca0f960e11b815260ff841660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692506326d941f291506024015f604051808303815f87803b1580156129c8575f5ffd5b505af11580156129da573d5f5f3e3d5ffd5b5050505050505050505050565b5f5f6129f2846139c7565b9050808360ff166001901b11610b215760405163ca95733360e01b815260040160405180910390fd5b606480546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b60965460ff90811690821610610cac57604051637310cff560e11b815260040160405180910390fd5b60ff82165f81815260976020908152604091829020845181548684018051888701805163ffffffff90951665ffffffffffff199094168417600160201b61ffff938416021767ffff0000000000001916600160301b95831695909502949094179094558551918252518316938101939093525116918101919091527f3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac9060600161209a565b609e546001600160a01b03163314611534576040516376d8ab1760e11b815260040160405180910390fd5b6001600160a01b0382165f90815260996020526040812080549091612b89826120a6565b905060018084015460ff166002811115612ba557612ba56149e1565b14612bc35760405163aba4733960e01b815260040160405180910390fd5b6096545f90612bd690869060ff166129e7565b90506001600160c01b038116612bff576040516368b6a87560e11b815260040160405180910390fd5b612c166001600160c01b0382811690841681161490565b612c335760405163d053aa2160e01b815260040160405180910390fd5b6001600160c01b0381811619831616612c4c8482613a87565b6001600160c01b038116612ca8576001600160a01b0387165f81815260996020526040808220600101805460ff19166002179055518692917f396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e491a35b60405163f4e24fe560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f4e24fe590612cf6908a908a90600401615603565b5f604051808303815f87803b158015612d0d575f5ffd5b505af1158015612d1f573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd9150612d719087908a90600401615626565b5f604051808303815f87803b158015612d88575f5ffd5b505af1158015612d9a573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd9150612dec9087908a90600401615626565b5f604051808303815f87803b158015612e03575f5ffd5b505af1158015612e15573d5f5f3e3d5ffd5b50505050611b9987858884613a93565b5f81516001600160401b03811115612e3f57612e3f614403565b604051908082528060200260200182016040528015612e68578160200160208202803683370190505b5090505f805b8351811015612ee7575f848281518110612e8a57612e8a6150fd565b016020015160f81c9050612e9d816133d9565b15612ede5760ff81168484612eb1816153dc565b955081518110612ec357612ec36150fd565b602002602001019063ffffffff16908163ffffffff16815250505b50600101612e6e565b508015610e9f57808252604080516060810182526001600160a01b03868116825260a154620100009004811660208301528183018590529151636e3492b560e01b81527f000000000000000000000000000000000000000000000000000000000000000090921691636e3492b591612f619160040161563e565b5f604051808303815f87803b158015612f78575f5ffd5b505af1158015612f8a573d5f5f3e3d5ffd5b5050505050505050565b6040516309aa152760e11b81526001600160a01b0383811660048301525f917f0000000000000000000000000000000000000000000000000000000000000000909116906313542a4e90602401602060405180830381865afa158015612ffc573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061302091906156ac565b90505f819003610d37577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bf79ce58848461306487610de8565b6040518463ffffffff1660e01b8152600401613082939291906156e5565b6020604051808303815f875af115801561309e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b2191906156ac565b83518251146130e45760405163aaad13f760e01b815260040160405180910390fd5b6130f086868484613b2b565b5f6130fd878787876133f0565b90505f5b8551811015612f8a575f60975f888481518110613120576131206150fd565b0160209081015160f81c82528181019290925260409081015f208151606081018352905463ffffffff811680835261ffff600160201b8304811695840195909552600160301b9091049093169181019190915284518051919350908490811061318b5761318b6150fd565b602002602001015163ffffffff1611156132df5761321f8783815181106131b4576131b46150fd565b602001015160f81c60f81b60f81c846040015184815181106131d8576131d86150fd565b60200260200101518b866020015186815181106131f7576131f76150fd565b6020026020010151898781518110613211576132116150fd565b602002602001015186613bd6565b6040805160018082528183019092525f91602082018180368337019050509050878381518110613251576132516150fd565b602001015160f81c60f81b815f8151811061326e5761326e6150fd565b60200101906001600160f81b03191690815f1a9053506132ab868481518110613299576132996150fd565b60200260200101516020015182612b65565b60a15460ff16156132dd576132dd8684815181106132cb576132cb6150fd565b60200260200101516020015182612e25565b505b50600101613101565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611534576040516323d871a560e01b815260040160405180910390fd5b60605f82516001600160401b0381111561334d5761334d614403565b6040519080825280601f01601f191660200182016040528015613377576020820181803683370190505b5090505f5b83518110156133d257838181518110613397576133976150fd565b602002602001015160f81b8282815181106133b4576133b46150fd565b60200101906001600160f81b03191690815f1a90535060010161337c565b5092915050565b60a2545f90610d37908360ff161c60019081161490565b61341460405180606001604052806060815260200160608152602001606081525090565b6096545f9061342790859060ff166129e7565b90505f613433866120a6565b90506001600160c01b03821661345c576040516313ca465760e01b815260040160405180910390fd5b8082166001600160c01b03161561348657604051630c6816cd60e01b815260040160405180910390fd5b60a0546001600160a01b0388165f908152609f60205260409020546001600160c01b03838116908516179142916134bd91906151ba565b106134db57604051631968677d60e11b815260040160405180910390fd5b6134e58782613a87565b867fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa86604051613515919061513f565b60405180910390a2604051631fd93ca960e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633fb279529061356b908b908a90600401615603565b5f604051808303815f87803b158015613582575f5ffd5b505af1158015613594573d5f5f3e3d5ffd5b5050604051632550477760e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169250632550477791506135e8908b908b908b906004016153f4565b5f604051808303815f875af1158015613603573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261362a91908101906157c2565b60408087019190915260208601919091525162bff04d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062bff04d90613685908a908a90600401615626565b5f604051808303815f875af11580156136a0573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526136c7919081019061581b565b8452505050949350505050565b60605f82516001600160401b038111156136f0576136f0614403565b604051908082528060200260200182016040528015613719578160200160208202803683370190505b5090505f5b835181101561377a5761374b868686848151811061373e5761373e6150fd565b6020026020010151613d4c565b82828151811061375d5761375d6150fd565b63ffffffff9092166020928302919091019091015260010161371e565b50949350505050565b5f610d37600180841b6158aa565b5f818152602083905260408120548082036137af575f915050610d37565b5f8381526020859052604090206137c76001836158aa565b815481106137d7576137d76150fd565b5f91825260209091200154600160401b90046001600160c01b03169150610d379050565b5f805b8215610d375761380f6001846158aa565b909216918061381d816158bd565b9150506137fe565b5f306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614801561387d57507f000000000000000000000000000000000000000000000000000000000000000046145b156138a757507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b5f80805f5160206159855f395f51905f5260035f5160206159855f395f51905f52865f5160206159855f395f51905f52888909090890505f6139bb827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f5160206159855f395f51905f52613e64565b91959194509092505050565b5f610100825111156139ec57604051637da54e4760e11b815260040160405180910390fd5b81515f036139fb57505f919050565b5f5f835f81518110613a0f57613a0f6150fd565b0160200151600160f89190911c81901b92505b8451811015613a7957848181518110613a3d57613a3d6150fd565b0160200151600160f89190911c1b9150828211613a6d57604051631019106960e31b815260040160405180910390fd5b91811791600101613a22565b50909392505050565b191690565b610c9760988383613edd565b5f613ab260a254836001600160c01b0316613a8290919063ffffffff16565b9050806111e8576040516351b27a6d60e11b81526001600160a01b0386811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da906024015f604051808303815f87803b158015613b19575f5ffd5b505af11580156118fb573d5f5f3e3d5ffd5b6020808201515f908152609a909152604090205460ff1615613b6057604051636fbefec360e11b815260040160405180910390fd5b4281604001511015613b8557604051630819bdcd60e01b815260040160405180910390fd5b602080820180515f908152609a909252604091829020805460ff19166001179055609d54905191830151610e9f926001600160a01b0390921691613bcf91889188918891906116c6565b8351614096565b6020808301516001600160a01b038082165f8181526099909452604090932054919290871603613c19576040516356168b4160e11b815260040160405180910390fd5b8760ff16845f015160ff1614613c4257604051638e5aeee760e01b815260040160405180910390fd5b604051635401ed2760e01b81526004810182905260ff891660248201525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635401ed2790604401602060405180830381865afa158015613cb0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cd491906158dd565b9050613ce081856140be565b6001600160601b0316866001600160601b031611613d1157604051634c44995d60e01b815260040160405180910390fd5b613d1b88856140e1565b6001600160601b0316816001600160601b0316106118fb5760405163b187e86960e01b815260040160405180910390fd5b5f81815260208490526040812054815b81811015613dcf576001613d7082846158aa565b613d7a91906158aa565b92508463ffffffff16865f8681526020019081526020015f208463ffffffff1681548110613daa57613daa6150fd565b5f9182526020909120015463ffffffff1611613dc7575050610b21565b600101613d5c565b5060405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a401611285565b5f5f613e6e6143c7565b613e766143e5565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280613eb357fe5b5082613ed25760405163d51edae360e01b815260040160405180910390fd5b505195945050505050565b5f8281526020849052604081205490819003613f81575f83815260208581526040808320815160608101835263ffffffff43811682528185018681526001600160c01b03808a16958401958652845460018101865594885295909620915191909201805495519351909416600160401b026001600160401b03938316600160201b0267ffffffffffffffff1990961691909216179390931716919091179055610e9f565b5f838152602085905260408120613f996001846158aa565b81548110613fa957613fa96150fd565b5f918252602090912001805490915063ffffffff438116911603613fea5780546001600160401b0316600160401b6001600160c01b038516021781556111e8565b805463ffffffff438116600160201b81810267ffffffff00000000199094169390931784555f8781526020898152604080832081516060810183529485528483018481526001600160c01b03808c1693870193845282546001810184559286529390942094519401805493519151909216600160401b026001600160401b0391861690960267ffffffffffffffff19909316939094169290921717919091169190911790555050505050565b6140a18383836140fa565b610b0b57604051638baa579f60e01b815260040160405180910390fd5b60208101515f90612710906140d79061ffff16856158f8565b610b21919061591a565b60408101515f90612710906140d79061ffff16856158f8565b5f5f5f614107858561423f565b90925090505f81600481111561411f5761411f6149e1565b14801561413d5750856001600160a01b0316826001600160a01b0316145b1561414d57600192505050610b21565b5f5f876001600160a01b0316631626ba7e60e01b8888604051602401614174929190615626565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516141b29190615947565b5f60405180830381855afa9150503d805f81146141ea576040519150601f19603f3d011682016040523d82523d5f602084013e6141ef565b606091505b5091509150818015614202575080516020145b801561423357508051630b135d3f60e11b90614227908301602090810190840161595d565b6001600160e01b031916145b98975050505050505050565b5f5f8251604103614273576020830151604084015160608501515f1a614267878285856142aa565b945094505050506142a3565b825160400361429c576020830151604084015161429186838361438f565b9350935050506142a3565b505f905060025b9250929050565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156142df57505f90506003614386565b8460ff16601b141580156142f757508460ff16601c14155b1561430757505f90506004614386565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015614358573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116614380575f60019250925050614386565b91505f90505b94509492505050565b5f806001600160ff1b038316816143ab60ff86901c601b6151ba565b90506143b9878288856142aa565b935093505050935093915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b038111828210171561443957614439614403565b60405290565b604080519081016001600160401b038111828210171561443957614439614403565b604051601f8201601f191681016001600160401b038111828210171561448957614489614403565b604052919050565b5f6001600160401b038211156144a9576144a9614403565b5060051b60200190565b6001600160a01b0381168114610cac575f5ffd5b5f82601f8301126144d6575f5ffd5b81356144e96144e482614491565b614461565b8082825260208201915060208360051b86010192508583111561450a575f5ffd5b602085015b83811015614530578035614522816144b3565b83526020928301920161450f565b5095945050505050565b5f6020828403121561454a575f5ffd5b81356001600160401b0381111561455f575f5ffd5b612339848285016144c7565b5f6020828403121561457b575f5ffd5b5035919050565b63ffffffff81168114610cac575f5ffd5b5f5f5f606084860312156145a5575f5ffd5b8335925060208401356145b781614582565b929592945050506040919091013590565b5f82601f8301126145d7575f5ffd5b8135602083015f5f6001600160401b038411156145f6576145f6614403565b50601f8301601f191660200161460b81614461565b91505082815285838301111561461f575f5ffd5b828260208301375f92810160200192909252509392505050565b5f60208284031215614649575f5ffd5b81356001600160401b0381111561465e575f5ffd5b612339848285016145c8565b5f6020828403121561467a575f5ffd5b8135610b21816144b3565b5f5f60408385031215614696575f5ffd5b50508035926020909101359150565b803560ff811681146146b5575f5ffd5b919050565b5f602082840312156146ca575f5ffd5b610b21826146a5565b815181526020808301519082015260408101610d37565b803561ffff811681146146b5575f5ffd5b5f6060828403121561470b575f5ffd5b614713614417565b9050813561472081614582565b815261472e602083016146ea565b602082015261473f604083016146ea565b604082015292915050565b6001600160601b0381168114610cac575f5ffd5b5f82601f83011261476d575f5ffd5b813561477b6144e482614491565b8082825260208201915060208360061b86010192508583111561479c575f5ffd5b602085015b8381101561453057604081880312156147b8575f5ffd5b6147c061443f565b81356147cb816144b3565b815260208201356147db8161474a565b60208281019190915290845292909201916040016147a1565b5f5f5f5f60c08587031215614807575f5ffd5b61481186866146fb565b935060608501356148218161474a565b925060808501356001600160401b0381111561483b575f5ffd5b6148478782880161475e565b92505060a085013561485881614582565b939692955090935050565b5f5f83601f840112614873575f5ffd5b5081356001600160401b03811115614889575f5ffd5b6020830191508360208285010111156142a3575f5ffd5b5f5f5f604084860312156148b2575f5ffd5b83356001600160401b038111156148c7575f5ffd5b8401601f810186136148d7575f5ffd5b80356148e56144e482614491565b8082825260208201915060208360051b850101925088831115614906575f5ffd5b602084015b838110156149465780356001600160401b03811115614928575f5ffd5b6149378b6020838901016144c7565b8452506020928301920161490b565b50955050505060208401356001600160401b03811115614964575f5ffd5b61497086828701614863565b9497909650939450505050565b5f5f5f5f5f60a08688031215614991575f5ffd5b853561499c816144b3565b945060208601356149ac816144b3565b935060408601356149bc816144b3565b92506060860135915060808601356149d3816144b3565b809150509295509295909350565b634e487b7160e01b5f52602160045260245ffd5b60038110614a1157634e487b7160e01b5f52602160045260245ffd5b9052565b8151815260208083015160408301916133d2908401826149f5565b5f5f60808385031215614a41575f5ffd5b614a4a836146a5565b9150614a5984602085016146fb565b90509250929050565b5f5f60408385031215614a73575f5ffd5b8235614a7e816144b3565b915060208301356001600160401b03811115614a98575f5ffd5b614aa4858286016145c8565b9150509250929050565b5f5f5f60a08486031215614ac0575f5ffd5b614aca85856146fb565b92506060840135614ada8161474a565b915060808401356001600160401b03811115614af4575f5ffd5b614b008682870161475e565b9150509250925092565b5f82601f830112614b19575f5ffd5b8135614b276144e482614491565b8082825260208201915060208360061b860101925085831115614b48575f5ffd5b602085015b838110156145305760408188031215614b64575f5ffd5b614b6c61443f565b614b75826146a5565b81526020820135614b85816144b3565b6020828101919091529084529290920191604001614b4d565b5f5f5f5f5f60a08688031215614bb2575f5ffd5b8535614bbd816144b3565b94506020860135935060408601356001600160401b03811115614bde575f5ffd5b614bea88828901614b0a565b9598949750949560608101359550608001359392505050565b5f60408284031215614c13575f5ffd5b614c1b61443f565b823581526020928301359281019290925250919050565b5f82601f830112614c41575f5ffd5b614c4961443f565b806040840185811115614c5a575f5ffd5b845b81811015614c74578035845260209384019301614c5c565b509095945050505050565b5f818303610100811215614c91575f5ffd5b614c99614417565b9150614ca58484614c03565b8252614cb48460408501614c03565b60208301526080607f1982011215614cca575f5ffd5b50614cd361443f565b614ce08460808501614c32565b8152614cef8460c08501614c32565b6020820152604082015292915050565b5f60608284031215614d0f575f5ffd5b614d17614417565b905081356001600160401b03811115614d2e575f5ffd5b614d3a848285016145c8565b8252506020828101359082015260409182013591810191909152919050565b5f5f5f5f5f5f5f6101a0888a031215614d70575f5ffd5b87356001600160401b03811115614d85575f5ffd5b614d918a828b01614863565b90985096505060208801356001600160401b03811115614daf575f5ffd5b614dbb8a828b016145c8565b955050614dcb8960408a01614c7f565b93506101408801356001600160401b03811115614de6575f5ffd5b614df28a828b01614b0a565b9350506101608801356001600160401b03811115614e0e575f5ffd5b614e1a8a828b01614cff565b9250506101808801356001600160401b03811115614e36575f5ffd5b614e428a828b01614cff565b91505092959891949750929550565b5f82601f830112614e60575f5ffd5b8135614e6e6144e482614491565b8082825260208201915060208360051b860101925085831115614e8f575f5ffd5b602085015b83811015614530578035614ea781614582565b835260209283019201614e94565b5f5f60408385031215614ec6575f5ffd5b8235614ed1816144b3565b915060208301356001600160401b03811115614eeb575f5ffd5b614aa485828601614e51565b5f5f5f5f6101608587031215614f0b575f5ffd5b84356001600160401b03811115614f20575f5ffd5b614f2c878288016145c8565b94505060208501356001600160401b03811115614f47575f5ffd5b614f53878288016145c8565b935050614f638660408701614c7f565b91506101408501356001600160401b03811115614f7e575f5ffd5b614f8a87828801614cff565b91505092959194509250565b5f5f5f5f60608587031215614fa9575f5ffd5b8435614fb4816144b3565b935060208501356001600160401b03811115614fce575f5ffd5b614fda87828801614e51565b93505060408501356001600160401b03811115614ff5575f5ffd5b61500187828801614863565b95989497509550505050565b5f5f6040838503121561501e575f5ffd5b823561502981614582565b915060208301356001600160401b03811115615043575f5ffd5b8301601f81018513615053575f5ffd5b80356150616144e482614491565b8082825260208201915060208360051b850101925087831115615082575f5ffd5b6020840193505b828410156150a4578335825260209384019390910190615089565b809450505050509250929050565b602080825282518282018190525f918401906040840190835b81811015614c7457835163ffffffff168352602093840193909201916001016150cb565b60208101610d3782846149f5565b634e487b7160e01b5f52603260045260245ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f610b216020830184615111565b5f60208284031215615161575f5ffd5b81518015158114610b21575f5ffd5b5f60208284031215615180575f5ffd5b8151610b21816144b3565b5f6020828403121561519b575f5ffd5b8151610b2181614582565b634e487b7160e01b5f52601160045260245ffd5b80820180821115610d3757610d376151a6565b5f5f858511156151db575f5ffd5b838611156151e7575f5ffd5b5050820193919092039150565b5f60c0820188835260018060a01b038816602084015286604084015260c0606084015280865180835260e0850191506020880192505f5b81811015615261578351805160ff1684526020908101516001600160a01b0316818501529093019260409092019160010161522b565b50506080840195909552505060a00152949350505050565b60018060a01b0383168152604060208201525f8251606060408401526152a260a0840182615111565b90506020840151606084015260408401516080840152809150509392505050565b8035600281106146b5575f5ffd5b5f5f5f61014084860312156152e4575f5ffd5b6152ed846152c3565b925060208401356001600160401b03811115615307575f5ffd5b615313868287016145c8565b9250506153238560408601614c7f565b90509250925092565b5f5f5f5f5f6101808688031215615341575f5ffd5b61534a866152c3565b945060208601356001600160401b03811115615364575f5ffd5b615370888289016145c8565b9450506153808760408801614c7f565b92506101408601356001600160401b0381111561539b575f5ffd5b6153a788828901614b0a565b9250506101608601356001600160401b038111156153c3575f5ffd5b6153cf88828901614cff565b9150509295509295909350565b5f600182016153ed576153ed6151a6565b5060010190565b60018060a01b0384168152826020820152606060408201525f61541a6060830184615111565b95945050505050565b5f60208284031215615433575f5ffd5b81516001600160c01b0381168114610b21575f5ffd5b634e487b7160e01b5f52601260045260245ffd5b5f8261546b5761546b615449565b500690565b60ff8181168382160190811115610d3757610d376151a6565b5f6040820160018060a01b03851683526040602084015280845180835260608501915060608160051b8601019250602086015f5b8281101561553e57868503605f190184528151805163ffffffff168652602090810151604082880181905281519088018190529101905f9060608801905b808310156155265783516001600160a01b0316825260209384019360019390930192909101906154fb565b509650505060209384019391909101906001016154bd565b5092979650505050505050565b5f8151808452602084019350602083015f5b8281101561559957815180516001600160a01b031687526020908101516001600160601b0316818801526040909601959091019060010161555d565b5093949350505050565b60ff841681526001600160601b0383166020820152606060408201525f61541a606083018461554b565b60ff851681526001600160601b038416602082015263ffffffff83166040820152608060608201525f611705608083018461554b565b6001600160a01b03831681526040602082018190525f90610b1e90830184615111565b828152604060208201525f610b1e6040830184615111565b602080825282516001600160a01b039081168383015283820151166040808401919091528301516060808401528051608084018190525f929190910190829060a08501905b808310156145305763ffffffff8451168252602082019150602084019350600183019250615683565b5f602082840312156156bc575f5ffd5b5051919050565b805f5b6002811015610e9f5781518452602093840193909101906001016156c6565b6001600160a01b0384168152825180516020808401919091520151604082015261016081016020848101518051606085015290810151608084015250604084015161573460a0840182516156c3565b6020015161574560e08401826156c3565b5082516101208301526020830151610140830152612339565b5f82601f83011261576d575f5ffd5b815161577b6144e482614491565b8082825260208201915060208360051b86010192508583111561579c575f5ffd5b602085015b838110156145305780516157b48161474a565b8352602092830192016157a1565b5f5f604083850312156157d3575f5ffd5b82516001600160401b038111156157e8575f5ffd5b6157f48582860161575e565b92505060208301516001600160401b0381111561580f575f5ffd5b614aa48582860161575e565b5f6020828403121561582b575f5ffd5b81516001600160401b03811115615840575f5ffd5b8201601f81018413615850575f5ffd5b805161585e6144e482614491565b8082825260208201915060208360051b85010192508683111561587f575f5ffd5b6020840193505b8284101561170557835161589981614582565b825260209384019390910190615886565b81810381811115610d3757610d376151a6565b5f61ffff821661ffff81036158d4576158d46151a6565b60010192915050565b5f602082840312156158ed575f5ffd5b8151610b218161474a565b6001600160601b0381811683821602908116908181146133d2576133d26151a6565b5f6001600160601b0383168061593257615932615449565b806001600160601b0384160491505092915050565b5f82518060208501845e5f920191825250919050565b5f6020828403121561596d575f5ffd5b81516001600160e01b031981168114610b21575f5ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220a2ada9834d29610bf3e5aca508a2b5a61a2b0c9f846e8383d380ca01181b0fdd64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x03\x92W_5`\xE0\x1C\x80ch0H5\x11a\x01\xDFW\x80c\xA4\xD7\x87\x1F\x11a\x01\tW\x80c\xCAO-\x97\x11a\0\xA9W\x80c\xEE1\x88!\x11a\0yW\x80c\xEE1\x88!\x14a\t\xB2W\x80c\xF2\xFD\xE3\x8B\x14a\t\xBAW\x80c\xFA\xBC\x1C\xBC\x14a\t\xCDW\x80c\xFD9\x10Z\x14a\t\xE0W__\xFD[\x80c\xCAO-\x97\x14a\x08\xCEW\x80c\xCA\x8A\xA7\xC7\x14a\x08\xE1W\x80c\xD7-\x8D\xD6\x14a\t\x08W\x80c\xE6W\x97\xAD\x14a\t\x10W__\xFD[\x80c\xAD\xCFs\xF7\x11a\0\xE4W\x80c\xAD\xCFs\xF7\x14a\x08bW\x80c\xB2\xD8g\x8D\x14a\x08uW\x80c\xC3\x91B^\x14a\x08\x87W\x80c\xCA\r\xE8\x82\x14a\x08\xA7W__\xFD[\x80c\xA4\xD7\x87\x1F\x14a\x083W\x80c\xA5\x08W\xBF\x14a\x08FW\x80c\xA9ox>\x14a\x08YW__\xFD[\x80c\x87\x1E\xF0I\x11a\x01\x7FW\x80c\x9B]\x17{\x11a\x01OW\x80c\x9B]\x17{\x14a\x07\xBFW\x80c\x9D\x8E\x0C#\x14a\x07\xD2W\x80c\x9E\x99#\xC2\x14a\x07\xE5W\x80c\x9F\xEA\xB8Y\x14a\x08\x0CW__\xFD[\x80c\x87\x1E\xF0I\x14a\x07^W\x80c\x88o\x11\x95\x14a\x07qW\x80c\x8D\xA5\xCB[\x14a\x07\x98W\x80c\x9A\xA1e=\x14a\x07\xA0W__\xFD[\x80cs;u\x07\x11a\x01\xBAW\x80cs;u\x07\x14a\x07#W\x80c\x81\xF96\xD2\x14a\x07+W\x80c\x82\x81\xABu\x14a\x078W\x80c\x84\xCAR\x13\x14a\x07KW__\xFD[\x80ch0H5\x14a\x06\xE1W\x80cn;\x17\xDB\x14a\x07\x08W\x80cqP\x18\xA6\x14a\x07\x1BW__\xFD[\x80c)k\xB0d\x11a\x02\xC0W\x80cS\x0B\x97\xA4\x11a\x02`W\x80c[\x0B\x82\x9F\x11a\x020W\x80c[\x0B\x82\x9F\x14a\x06\x8CW\x80c\\\x97Z\xBB\x14a\x06\x9FW\x80c]\xF4YF\x14a\x06\xA7W\x80ccG\xC9\0\x14a\x06\xCEW__\xFD[\x80cS\x0B\x97\xA4\x14a\x062W\x80cXe\xC6\x0C\x14a\x06EW\x80cY\\jg\x14a\x06eW\x80cZ\xC8j\xB7\x14a\x06mW__\xFD[\x80c9\x98\xFD\xD3\x11a\x02\x9BW\x80c9\x98\xFD\xD3\x14a\x05\xC5W\x80c<*\x7FL\x14a\x05\xECW\x80c>\xEF:Q\x14a\x06\x0CW\x80cQ@\xA5H\x14a\x06\x1FW__\xFD[\x80c)k\xB0d\x14a\x05\x8CW\x80c)\xD1\xE0\xC3\x14a\x05\x9FW\x80c,\xDD\x1E\x86\x14a\x05\xB2W__\xFD[\x80c\x12^\x05\x84\x11a\x036W\x80c\x14x\x85\x1F\x11a\x03\x06W\x80c\x14x\x85\x1F\x14a\x04\xDFW\x80c\x1E\xB8\x12\xDA\x14a\x05\x11W\x80c$\x9A\x0CB\x14a\x05ZW\x80c(\xF6\x1B1\x14a\x05yW__\xFD[\x80c\x12^\x05\x84\x14a\x04rW\x80c\x13T*N\x14a\x04\x91W\x80c\x13d9\xDD\x14a\x04\xB9W\x80c\x14>Y\x15\x14a\x04\xCCW__\xFD[\x80c\x05C\x10\xE6\x11a\x03qW\x80c\x05C\x10\xE6\x14a\x04\x08W\x80c\x07d\xCB\x93\x14a\x043W\x80c\x0C\xF4\xB7g\x14a\x04LW\x80c\r?!4\x14a\x04_W__\xFD[\x80b\xCF*\xB5\x14a\x03\x96W\x80c\x03\xFD4\x92\x14a\x03\xABW\x80c\x04\xECcQ\x14a\x03\xDDW[__\xFD[a\x03\xA9a\x03\xA46`\x04aE:V[a\n\x1BV[\0[a\x03\xCAa\x03\xB96`\x04aEkV[_\x90\x81R`\x98` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\xF0a\x03\xEB6`\x04aE\x93V[a\x0B\x10V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xD4V[`\x9DTa\x04\x1B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xD4V[`\xA1Ta\x04\x1B\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xA9a\x04Z6`\x04aF9V[a\x0B(V[a\x03\xA9a\x04m6`\x04aEkV[a\x0B\xB9V[a\x03\xCAa\x04\x806`\x04aFjV[`\x9F` R_\x90\x81R`@\x90 T\x81V[a\x03\xCAa\x04\x9F6`\x04aFjV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x99` R`@\x90 T\x90V[a\x03\xA9a\x04\xC76`\x04aEkV[a\x0B\xC6V[a\x03\xA9a\x04\xDA6`\x04aFjV[a\x0C\x9BV[a\x05\x01a\x04\xED6`\x04aEkV[`\x9A` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xD4V[a\x05$a\x05\x1F6`\x04aF\x85V[a\x0C\xAFV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x03\xD4V[a\x03\xCAa\x05h6`\x04aF\xBAV[`\x9B` R_\x90\x81R`@\x90 T\x81V[`\x9ETa\x04\x1B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x1Ba\x05\x9A6`\x04aEkV[a\r=V[a\x03\xA9a\x05\xAD6`\x04aFjV[a\r\xC6V[a\x03\xA9a\x05\xC06`\x04aFjV[a\r\xD7V[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xFFa\x05\xFA6`\x04aFjV[a\r\xE8V[`@Qa\x03\xD4\x91\x90aF\xD3V[a\x03\xA9a\x06\x1A6`\x04aG\xF4V[a\x0EfV[a\x03\xA9a\x06-6`\x04aH\xA0V[a\x0E\xA5V[a\x03\xA9a\x06@6`\x04aI}V[a\x11\xEFV[a\x06Xa\x06S6`\x04aFjV[a\x14\x15V[`@Qa\x03\xD4\x91\x90aJ\x15V[a\x03\xA9a\x14\x87V[a\x05\x01a\x06{6`\x04aF\xBAV[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x03\xA9a\x06\x9A6`\x04aJ0V[a\x156V[`\x01Ta\x03\xCAV[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\x1Ba\x06\xDC6`\x04aEkV[a\x15RV[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xA9a\x07\x166`\x04aJbV[a\x15zV[a\x03\xA9a\x16<V[a\x03\xA9a\x16MV[`\xA1Ta\x05\x01\x90`\xFF\x16\x81V[a\x03\xA9a\x07F6`\x04aJ\xAEV[a\x16\xB1V[a\x03\xCAa\x07Y6`\x04aK\x9EV[a\x16\xC6V[a\x03\xF0a\x07l6`\x04aEkV[a\x17\x0FV[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\x1Ba\x17\x19V[`\x96Ta\x07\xAD\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03\xD4V[a\x03\xA9a\x07\xCD6`\x04aMYV[a\x171V[a\x03\xA9a\x07\xE06`\x04aN\xB5V[a\x19\x06V[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xCA\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x05\x01a\x08A6`\x04aF\xBAV[a\x19nV[a\x03\xA9a\x08T6`\x04aN\xF7V[a\x19xV[a\x03\xCA`\xA0T\x81V[a\x03\xA9a\x08p6`\x04aO\x96V[a\x1B\xA2V[`\xA1Ta\x05\x01\x90a\x01\0\x90\x04`\xFF\x16\x81V[a\x08\x9Aa\x08\x956`\x04aP\rV[a\x1E\x05V[`@Qa\x03\xD4\x91\x90aP\xB2V[a\x03\xCA\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81V[a\x03\xA9a\x08\xDC6`\x04aF9V[a\x1E\x13V[a\x04\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x9CTa\x03\xCAV[a\t~a\t\x1E6`\x04aF\xBAV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x97\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x03\xD4V[a\x03\xA9a\x1E\xA4V[a\x03\xA9a\t\xC86`\x04aFjV[a\x1F\x19V[a\x03\xA9a\t\xDB6`\x04aEkV[a\x1F\x8FV[a\n\x0Ea\t\xEE6`\x04aFjV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x03\xD4\x91\x90aP\xEFV[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\nDW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82Q\x81\x10\x15a\x0B\x0BW_\x83\x82\x81Q\x81\x10a\nbWa\nbaP\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x99\x83R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x93\x95P\x91\x93\x90\x92\x90\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\n\xB5Wa\n\xB5aI\xE1V[`\x02\x81\x11\x15a\n\xC6Wa\n\xC6aI\xE1V[\x90RP\x80Q\x90\x91P_a\n\xD8\x82a \xA6V[\x90P_a\n\xED\x82`\x01`\x01`\xC0\x1B\x03\x16a \xB2V[\x90Pa\n\xFA\x85\x85\x83a!{V[PP`\x01\x90\x93\x01\x92Pa\nF\x91PPV[PPPV[_a\x0B\x1E`\x98\x85\x85\x85a\"]V[\x90P[\x93\x92PPPV[`\x013_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x0BPWa\x0BPaI\xE1V[\x14a\x0BnW`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x99` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\x0B\xAE\x90\x84\x90aQ?V[`@Q\x80\x91\x03\x90\xA2PV[a\x0B\xC1a#AV[`\xA0UV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CL\x91\x90aQQV[a\x0CiW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x81\x81\x16\x81\x14a\x0C\x8EW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\x97\x82a#\xA0V[PPV[a\x0C\xA3a#AV[a\x0C\xAC\x81a#\xD7V[PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R_\x83\x81R`\x98` R`@\x90 \x80T\x83\x90\x81\x10a\x0C\xEAWa\x0C\xEAaP\xFDV[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r7\x91\x90aQpV[a\r\xCEa#AV[a\x0C\xAC\x81a$\x01V[a\r\xDFa#AV[a\x0C\xAC\x81a$jV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\r7a\x0Ea\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x84`@Q` \x01a\x0EF\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a$\xD3V[a%\x1FV[a\x0Ena#AV[`\xA1T`\xFF\x16a\x0E\x91W`@Qc[w\x90\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\x9F\x84\x84\x84`\x01\x85a%\xA9V[PPPPV[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\x0E\xCEW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x13\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa)\xE7\x90PV[P\x83Q\x82\x14a\x0F5W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\x11\xE8W_\x84\x84\x83\x81\x81\x10a\x0FRWa\x0FRaP\xFDV[\x88Q\x92\x015`\xF8\x1C\x92P_\x91\x88\x91P\x84\x90\x81\x10a\x0FqWa\x0FqaP\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xE4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x08\x91\x90aQ\x8BV[c\xFF\xFF\xFF\xFF\x16\x81Q\x14a\x10.W`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80[\x82Q\x81\x10\x15a\x11\x8FW_\x83\x82\x81Q\x81\x10a\x10MWa\x10MaP\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x99\x83R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x93\x95P\x91\x93\x90\x92\x90\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x10\xA0Wa\x10\xA0aI\xE1V[`\x02\x81\x11\x15a\x10\xB1Wa\x10\xB1aI\xE1V[\x90RP\x80Q\x90\x91P_a\x10\xC3\x82a \xA6V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x8A\x16\x1C\x81\x16\x14a\x10\xF6W`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11a\x11(W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa\x11\x82\x83\x83\x8D\x8B\x8Ea\x11<\x82`\x01aQ\xBAV[\x92a\x11I\x93\x92\x91\x90aQ\xCDV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa!{\x92PPPV[P\x90\x92PP`\x01\x01a\x101V[P`\xFF\x83\x16_\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPP\x80`\x01\x01\x90Pa\x0F7V[PPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\rWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12&WP0;\x15\x80\x15a\x12&WP_T`\xFF\x16`\x01\x14[a\x12\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12\xAFW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12\xB8\x86a*\x1BV[a\x12\xC1\x85a$\x01V[a\x12\xCA\x83a#\xA0V[a\x12\xD3\x84a$jV[a\x12\xDC\x82a#\xD7V[`\x9C\x80T`\x01\x81\x81\x01\x83U_\x83\x90R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x91\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U\x84T\x80\x84\x01\x86U\x84\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x84T\x92\x83\x01\x90\x94U\x91\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x92\x16\x17\x90U`\xA1\x80Ta\x01\x01a\xFF\xFF\x19\x90\x91\x16\x17\x90U\x80\x15a\x14\rW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x99` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x14mWa\x14maI\xE1V[`\x02\x81\x11\x15a\x14~Wa\x14~aI\xE1V[\x90RP\x92\x91PPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xE9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\r\x91\x90aQQV[a\x15*W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x154_\x19a#\xA0V[V[a\x15>a#AV[\x81a\x15H\x81a*lV[a\x0B\x0B\x83\x83a*\x95V[`\x9C\x81\x81T\x81\x10a\x15aW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[a\x15\x82a+:V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 B\x90U`\x99\x90\x91R\x81 \x80T`\x96T\x91\x92\x90\x91a\x15\xBD\x90\x85\x90`\xFF\x16a)\xE7V[\x90P_a\x15\xC9\x83a \xA6V[\x90P`\x01\x80\x85\x01T`\xFF\x16`\x02\x81\x11\x15a\x15\xE5Wa\x15\xE5aI\xE1V[\x14\x80\x15a\x15\xFAWP`\x01`\x01`\xC0\x1B\x03\x82\x16\x15\x15[\x80\x15a\x16\x18WPa\x16\x18`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[\x15a\x14\rWa\x16'\x86\x86a+eV[`\xA1T`\xFF\x16\x15a\x14\rWa\x14\r\x86\x86a.%V[a\x16Da#AV[a\x154_a*\x1BV[a\x16Ua#AV[`\xA1T`\xFF\x16a\x16xW`@Qc[w\x90\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U`@Q\x7F\xA4\xCDB\x92\x0E\xD0\xD17+\xA4\x05\x1DEw'\x9F#o\xBB\xE6w\xA6\x7F?}d^\x82B]\xD9\x8D\x90_\x90\xA1V[a\x16\xB9a#AV[a\x0B\x0B\x83\x83\x83__a%\xA9V[_a\x17\x05\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87\x87\x87\x87\x87`@Q` \x01a\x0EF\x96\x95\x94\x93\x92\x91\x90aQ\xF4V[\x96\x95PPPPPPV[_a\r7\x82a \xA6V[_a\x17,`dT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\x01\x80T_\x91\x90\x81\x16\x03a\x17XW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1Ta\x01\0\x90\x04`\xFF\x16\x15a\x17\x81W`@Qc\x1Dv \x1F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x17\x8C3\x87a/\x94V[\x90Pa\x17\xD33\x82\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8D\x92P\x8B\x91P\x8A\x90Pa0\xC2V[`\x013_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x17\xFBWa\x17\xFBaI\xE1V[\x14a\x18\xFBW`@\x80Q\x80\x82\x01\x82R\x82\x81R`\x01` \x80\x83\x01\x82\x81R3_\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a\x18LWa\x18LaI\xE1V[\x02\x17\x90UPP`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pc\x99&\xEE}\x90a\x18\xA1\x903\x90\x87\x90`\x04\x01aRyV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18\xB8W__\xFD[PZ\xF1\x15\x80\x15a\x18\xCAW=__>=_\xFD[PP`@Q\x83\x92P3\x91P\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90_\x90\xA3[PPPPPPPPPV[a\x19\x0Ea2\xE8V[`\x01\x80T`\x02\x90\x81\x16\x03a\x195W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1T`\xFF\x16a\x19XW`@Qc[w\x90\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x19b\x83a31V[\x90Pa\x0E\x9F\x84\x82a+eV[_a\r7\x82a3\xD9V[`\x01\x80T_\x91\x90\x81\x16\x03a\x19\x9FW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1Ta\x01\0\x90\x04`\xFF\x16\x15a\x19\xC8W`@Qc\x1Dv \x1F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x19\xD33\x85a/\x94V[\x90P_a\x19\xE23\x83\x89\x89a3\xF0V[Q\x90P_[\x87Q\x81\x10\x15a\x1ApW_\x88\x82\x81Q\x81\x10a\x1A\x03Wa\x1A\x03aP\xFDV[\x01` \x90\x81\x01Q`\xF8\x1C_\x81\x81R`\x97\x90\x92R`@\x90\x91 T\x84Q\x91\x92Pc\xFF\xFF\xFF\xFF\x16\x90\x84\x90\x84\x90\x81\x10a\x1A:Wa\x1A:aP\xFDV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1AgW`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x19\xE7V[P`\x013_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x1A\x99Wa\x1A\x99aI\xE1V[\x14a\x1B\x99W`@\x80Q\x80\x82\x01\x82R\x83\x81R`\x01` \x80\x83\x01\x82\x81R3_\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a\x1A\xEAWa\x1A\xEAaI\xE1V[\x02\x17\x90UPP`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pc\x99&\xEE}\x90a\x1B?\x903\x90\x88\x90`\x04\x01aRyV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1BVW__\xFD[PZ\xF1\x15\x80\x15a\x1BhW=__>=_\xFD[PP`@Q\x84\x92P3\x91P\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90_\x90\xA3[PPPPPPPV[a\x1B\xAAa2\xE8V[`\x01\x80T_\x91\x90\x81\x16\x03a\x1B\xD1W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1T`\xFF\x16a\x1B\xF4W`@Qc[w\x90\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1B\xFE\x85a31V[\x90P_\x80\x80a\x1C\x0F\x86\x88\x01\x88aR\xD1V[\x92P\x92P\x92P_a\x1C \x8A\x83a/\x94V[\x90P_\x84`\x01\x81\x11\x15a\x1C5Wa\x1C5aI\xE1V[\x03a\x1C\xDCW_a\x1CG\x8B\x83\x88\x87a3\xF0V[Q\x90P_[\x86Q\x81\x10\x15a\x1C\xD5W_\x87\x82\x81Q\x81\x10a\x1ChWa\x1ChaP\xFDV[\x01` \x90\x81\x01Q`\xF8\x1C_\x81\x81R`\x97\x90\x92R`@\x90\x91 T\x84Q\x91\x92Pc\xFF\xFF\xFF\xFF\x16\x90\x84\x90\x84\x90\x81\x10a\x1C\x9FWa\x1C\x9FaP\xFDV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1C\xCCW`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x1CLV[PPa\x1D1V[`\x01\x84`\x01\x81\x11\x15a\x1C\xF0Wa\x1C\xF0aI\xE1V[\x03a\x1D\x18W_\x80a\x1D\x03\x89\x8B\x01\x8BaS,V[\x94P\x94PPPPa\x1C\xD5\x8C\x84\x89\x88\x86\x86a0\xC2V[`@Qc5K\xB8\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\x01`\xA0\x1B\x03\x8B\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x1DbWa\x1DbaI\xE1V[\x14a\x1D\xF9W`@\x80Q\x80\x82\x01\x82R\x82\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16_\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a\x1D\xBCWa\x1D\xBCaI\xE1V[\x02\x17\x90UPP`@Q\x82\x91P`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90_\x90\xA3[PPPPPPPPPPV[``a\x0B!`\x98\x84\x84a6\xD4V[`\x01\x80T`\x02\x90\x81\x16\x03a\x1E:W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82Q\x81\x10\x15a\x1E\x99W`\xA1T`\xFF\x16\x15\x80a\x1EtWPa\x1Et\x83\x82\x81Q\x81\x10a\x1EgWa\x1EgaP\xFDV[\x01` \x01Q`\xF8\x1Ca3\xD9V[a\x1E\x91W`@Qc\xB2\xE1\x8E\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\x1E<V[Pa\x0C\x973\x83a+eV[a\x1E\xACa#AV[`\xA1T`\xFF\x16\x15a\x1E\xD0W`@Qc\xB2\xE1\x8E\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x96Ta\x1E\xDF\x90`\xFF\x16a7\x83V[`\xA2U`\xA1\x80T`\xFF\x19\x16`\x01\x17\x90U`@Q\x7F\x0B\x880o\xF4bq!\xF5\xB3\xE5\xB1\xC5\xF8\x8Fk\x1EB\xFD,\x04x\xEF\x1C\x91f-I\xD1\xF0wU\x90_\x90\xA1V[a\x1F!a#AV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1F\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x12\x85V[a\x0C\xAC\x81a*\x1BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x0F\x91\x90aQpV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a @W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x80\x19\x82\x19\x81\x16\x14a gW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPV[_a\r7`\x98\x83a7\x91V[``__a \xBF\x84a7\xFBV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a \xDAWa \xDAaD\x03V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a!\x04W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_\x80[\x82Q\x82\x10\x80\x15a!\x1BWPa\x01\0\x81\x10[\x15a!qW`\x01\x81\x1B\x93P\x85\x84\x16\x15a!aW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a!DWa!DaP\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x81`\x01\x01\x91P[a!j\x81aS\xDCV[\x90Pa!\nV[P\x90\x94\x93PPPPV[`\x01\x82` \x01Q`\x02\x81\x11\x15a!\x93Wa!\x93aI\xE1V[\x14a!\x9DWPPPV[\x81Q`@Qc3V\x7F\x7F`\xE1\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cf\xAC\xFE\xFE\x90a!\xF1\x90\x88\x90\x86\x90\x88\x90`\x04\x01aS\xF4V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"1\x91\x90aT#V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15a\x11\xE8Wa\x11\xE8\x85a\"X\x83`\x01`\x01`\xC0\x1B\x03\x16a \xB2V[a+eV[_\x83\x81R` \x85\x90R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\"\x7FWa\"\x7FaP\xFDV[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\"\xEFW`@Qcl\xB1\x9A\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a#\x15WP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a#2W`@Qc\xBB\xBA`\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x01Q\x90P[\x94\x93PPPPV[3a#Ja\x17\x19V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x154W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x12\x85V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\x0B\xAEV[`\xA1\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_a\r7a$\xDFa8%V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R_\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a%L_Q` aY\x85_9_Q\x90_R\x86aT]V[\x90P[a%X\x81a9KV[\x90\x93P\x91P_Q` aY\x85_9_Q\x90_R\x82\x83\t\x83\x03a%\x90W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` aY\x85_9_Q\x90_R`\x01\x82\x08\x90Pa%OV[`\x96T`\xFF\x16`\xC0\x81\x10a%\xD0W`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a%\xDB\x81`\x01aTpV[`\x96\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a%\xFA\x81\x88a*\x95V[`\xA1T`\xFF\x16\x80\x15a&\x12WPa&\x10\x81a3\xD9V[\x15[\x15a'\xBDW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a&-W\x90PP\x90P_\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&pWa&paD\x03V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&\x99W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x87Q\x81\x10\x15a&\xF6W\x87\x81\x81Q\x81\x10a&\xB9Wa&\xB9aP\xFDV[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a&\xD6Wa&\xD6aP\xFDV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a&\x9EV[P`@Q\x80`@\x01`@R\x80\x84`\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x82_\x81Q\x81\x10a''Wa''aP\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xA1T`@Qc\x010\xFC'`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92c&\x1F\x84\xE0\x92a'\x8D\x92b\x01\0\0\x90\x92\x04\x90\x91\x16\x90\x86\x90`\x04\x01aT\x89V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'\xA4W__\xFD[PZ\xF1\x15\x80\x15a'\xB6W=__>=_\xFD[PPPPPP[_\x84`\x01\x81\x11\x15a'\xD0Wa'\xD0aI\xE1V[\x03a(WW`@Qc:\xEA\x0B\x9D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cu\xD4\x17:\x90a(%\x90\x84\x90\x8A\x90\x8A\x90`\x04\x01aU\xA3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(<W__\xFD[PZ\xF1\x15\x80\x15a(NW=__>=_\xFD[PPPPa(\xF0V[`\x01\x84`\x01\x81\x11\x15a(kWa(kaI\xE1V[\x03a(\xF0W`@Qc\x06b\xD3\xE1`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xCCZ| \x90a(\xC2\x90\x84\x90\x8A\x90\x88\x90\x8B\x90`\x04\x01aU\xCDV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(\xD9W__\xFD[PZ\xF1\x15\x80\x15a(\xEBW=__>=_\xFD[PPPP[`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c&\xD9A\xF2\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a)QW__\xFD[PZ\xF1\x15\x80\x15a)cW=__>=_\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a)\xC8W__\xFD[PZ\xF1\x15\x80\x15a)\xDAW=__>=_\xFD[PPPPPPPPPPPV[__a)\xF2\x84a9\xC7V[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x0B!W`@Qc\xCA\x95s3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\x96T`\xFF\x90\x81\x16\x90\x82\x16\x10a\x0C\xACW`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x16_\x81\x81R`\x97` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01a \x9AV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\x154W`@Qcv\xD8\xAB\x17`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x99` R`@\x81 \x80T\x90\x91a+\x89\x82a \xA6V[\x90P`\x01\x80\x84\x01T`\xFF\x16`\x02\x81\x11\x15a+\xA5Wa+\xA5aI\xE1V[\x14a+\xC3W`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x96T_\x90a+\xD6\x90\x86\x90`\xFF\x16a)\xE7V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a+\xFFW`@Qch\xB6\xA8u`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,\x16`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x90\x84\x16\x81\x16\x14\x90V[a,3W`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xC0\x1B\x03\x81\x81\x16\x19\x83\x16\x16a,L\x84\x82a:\x87V[`\x01`\x01`\xC0\x1B\x03\x81\x16a,\xA8W`\x01`\x01`\xA0\x1B\x03\x87\x16_\x81\x81R`\x99` R`@\x80\x82 `\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UQ\x86\x92\x91\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x91\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a,\xF6\x90\x8A\x90\x8A\x90`\x04\x01aV\x03V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\rW__\xFD[PZ\xF1\x15\x80\x15a-\x1FW=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa-q\x90\x87\x90\x8A\x90`\x04\x01aV&V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\x88W__\xFD[PZ\xF1\x15\x80\x15a-\x9AW=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa-\xEC\x90\x87\x90\x8A\x90`\x04\x01aV&V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a.\x03W__\xFD[PZ\xF1\x15\x80\x15a.\x15W=__>=_\xFD[PPPPa\x1B\x99\x87\x85\x88\x84a:\x93V[_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a.?Wa.?aD\x03V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a.hW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x83Q\x81\x10\x15a.\xE7W_\x84\x82\x81Q\x81\x10a.\x8AWa.\x8AaP\xFDV[\x01` \x01Q`\xF8\x1C\x90Pa.\x9D\x81a3\xD9V[\x15a.\xDEW`\xFF\x81\x16\x84\x84a.\xB1\x81aS\xDCV[\x95P\x81Q\x81\x10a.\xC3Wa.\xC3aP\xFDV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPP[P`\x01\x01a.nV[P\x80\x15a\x0E\x9FW\x80\x82R`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R`\xA1Tb\x01\0\0\x90\x04\x81\x16` \x83\x01R\x81\x83\x01\x85\x90R\x91Qcn4\x92\xB5`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cn4\x92\xB5\x91a/a\x91`\x04\x01aV>V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/xW__\xFD[PZ\xF1\x15\x80\x15a/\x8AW=__>=_\xFD[PPPPPPPPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xFCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0 \x91\x90aV\xACV[\x90P_\x81\x90\x03a\r7W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBFy\xCEX\x84\x84a0d\x87a\r\xE8V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\x82\x93\x92\x91\x90aV\xE5V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B!\x91\x90aV\xACV[\x83Q\x82Q\x14a0\xE4W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a0\xF0\x86\x86\x84\x84a;+V[_a0\xFD\x87\x87\x87\x87a3\xF0V[\x90P_[\x85Q\x81\x10\x15a/\x8AW_`\x97_\x88\x84\x81Q\x81\x10a1 Wa1 aP\xFDV[\x01` \x90\x81\x01Q`\xF8\x1C\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a1\x8BWa1\x8BaP\xFDV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a2\xDFWa2\x1F\x87\x83\x81Q\x81\x10a1\xB4Wa1\xB4aP\xFDV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a1\xD8Wa1\xD8aP\xFDV[` \x02` \x01\x01Q\x8B\x86` \x01Q\x86\x81Q\x81\x10a1\xF7Wa1\xF7aP\xFDV[` \x02` \x01\x01Q\x89\x87\x81Q\x81\x10a2\x11Wa2\x11aP\xFDV[` \x02` \x01\x01Q\x86a;\xD6V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a2QWa2QaP\xFDV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x81_\x81Q\x81\x10a2nWa2naP\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa2\xAB\x86\x84\x81Q\x81\x10a2\x99Wa2\x99aP\xFDV[` \x02` \x01\x01Q` \x01Q\x82a+eV[`\xA1T`\xFF\x16\x15a2\xDDWa2\xDD\x86\x84\x81Q\x81\x10a2\xCBWa2\xCBaP\xFDV[` \x02` \x01\x01Q` \x01Q\x82a.%V[P[P`\x01\x01a1\x01V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x154W`@Qc#\xD8q\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a3MWa3MaD\x03V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a3wW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a3\xD2W\x83\x81\x81Q\x81\x10a3\x97Wa3\x97aP\xFDV[` \x02` \x01\x01Q`\xF8\x1B\x82\x82\x81Q\x81\x10a3\xB4Wa3\xB4aP\xFDV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a3|V[P\x92\x91PPV[`\xA2T_\x90a\r7\x90\x83`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a4\x14`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\x96T_\x90a4'\x90\x85\x90`\xFF\x16a)\xE7V[\x90P_a43\x86a \xA6V[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a4\\W`@Qc\x13\xCAFW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a4\x86W`@Qc\x0Ch\x16\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA0T`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9F` R`@\x90 T`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17\x91B\x91a4\xBD\x91\x90aQ\xBAV[\x10a4\xDBW`@Qc\x19hg}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a4\xE5\x87\x82a:\x87V[\x86\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x86`@Qa5\x15\x91\x90aQ?V[`@Q\x80\x91\x03\x90\xA2`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a5k\x90\x8B\x90\x8A\x90`\x04\x01aV\x03V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5\x82W__\xFD[PZ\xF1\x15\x80\x15a5\x94W=__>=_\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa5\xE8\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aS\xF4V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a6\x03W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6*\x91\x90\x81\x01\x90aW\xC2V[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a6\x85\x90\x8A\x90\x8A\x90`\x04\x01aV&V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a6\xA0W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6\xC7\x91\x90\x81\x01\x90aX\x1BV[\x84RPPP\x94\x93PPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a6\xF0Wa6\xF0aD\x03V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7\x19W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a7zWa7K\x86\x86\x86\x84\x81Q\x81\x10a7>Wa7>aP\xFDV[` \x02` \x01\x01Qa=LV[\x82\x82\x81Q\x81\x10a7]Wa7]aP\xFDV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a7\x1EV[P\x94\x93PPPPV[_a\r7`\x01\x80\x84\x1BaX\xAAV[_\x81\x81R` \x83\x90R`@\x81 T\x80\x82\x03a7\xAFW_\x91PPa\r7V[_\x83\x81R` \x85\x90R`@\x90 a7\xC7`\x01\x83aX\xAAV[\x81T\x81\x10a7\xD7Wa7\xD7aP\xFDV[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x91Pa\r7\x90PV[_\x80[\x82\x15a\r7Wa8\x0F`\x01\x84aX\xAAV[\x90\x92\x16\x91\x80a8\x1D\x81aX\xBDV[\x91PPa7\xFEV[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a8}WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a8\xA7WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[_\x80\x80_Q` aY\x85_9_Q\x90_R`\x03_Q` aY\x85_9_Q\x90_R\x86_Q` aY\x85_9_Q\x90_R\x88\x89\t\t\x08\x90P_a9\xBB\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` aY\x85_9_Q\x90_Ra>dV[\x91\x95\x91\x94P\x90\x92PPPV[_a\x01\0\x82Q\x11\x15a9\xECW`@Qc}\xA5NG`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q_\x03a9\xFBWP_\x91\x90PV[__\x83_\x81Q\x81\x10a:\x0FWa:\x0FaP\xFDV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a:yW\x84\x81\x81Q\x81\x10a:=Wa:=aP\xFDV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a:mW`@Qc\x10\x19\x10i`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x81\x17\x91`\x01\x01a:\"V[P\x90\x93\x92PPPV[\x19\x16\x90V[a\x0C\x97`\x98\x83\x83a>\xDDV[_a:\xB2`\xA2T\x83`\x01`\x01`\xC0\x1B\x03\x16a:\x82\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80a\x11\xE8W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a;\x19W__\xFD[PZ\xF1\x15\x80\x15a\x18\xFBW=__>=_\xFD[` \x80\x82\x01Q_\x90\x81R`\x9A\x90\x91R`@\x90 T`\xFF\x16\x15a;`W`@Qco\xBE\xFE\xC3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x81`@\x01Q\x10\x15a;\x85W`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x82\x01\x80Q_\x90\x81R`\x9A\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9DT\x90Q\x91\x83\x01Qa\x0E\x9F\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a;\xCF\x91\x88\x91\x88\x91\x88\x91\x90a\x16\xC6V[\x83Qa@\x96V[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x81\x81R`\x99\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x03a<\x19W`@QcV\x16\x8BA`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87`\xFF\x16\x84_\x01Q`\xFF\x16\x14a<BW`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\xB0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xD4\x91\x90aX\xDDV[\x90Pa<\xE0\x81\x85a@\xBEV[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11a=\x11W`@QcLD\x99]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a=\x1B\x88\x85a@\xE1V[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a\x18\xFBW`@Qc\xB1\x87\xE8i`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R` \x84\x90R`@\x81 T\x81[\x81\x81\x10\x15a=\xCFW`\x01a=p\x82\x84aX\xAAV[a=z\x91\x90aX\xAAV[\x92P\x84c\xFF\xFF\xFF\xFF\x16\x86_\x86\x81R` \x01\x90\x81R` \x01_ \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a=\xAAWa=\xAAaP\xFDV[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a=\xC7WPPa\x0B!V[`\x01\x01a=\\V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x12\x85V[__a>naC\xC7V[a>vaC\xE5V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a>\xB3W\xFE[P\x82a>\xD2W`@Qc\xD5\x1E\xDA\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x95\x94PPPPPV[_\x82\x81R` \x84\x90R`@\x81 T\x90\x81\x90\x03a?\x81W_\x83\x81R` \x85\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\x0E\x9FV[_\x83\x81R` \x85\x90R`@\x81 a?\x99`\x01\x84aX\xAAV[\x81T\x81\x10a?\xA9Wa?\xA9aP\xFDV[_\x91\x82R` \x90\x91 \x01\x80T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a?\xEAW\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\x11\xE8V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U_\x87\x81R` \x89\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPPV[a@\xA1\x83\x83\x83a@\xFAV[a\x0B\x0BW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_\x90a'\x10\x90a@\xD7\x90a\xFF\xFF\x16\x85aX\xF8V[a\x0B!\x91\x90aY\x1AV[`@\x81\x01Q_\x90a'\x10\x90a@\xD7\x90a\xFF\xFF\x16\x85aX\xF8V[___aA\x07\x85\x85aB?V[\x90\x92P\x90P_\x81`\x04\x81\x11\x15aA\x1FWaA\x1FaI\xE1V[\x14\x80\x15aA=WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15aAMW`\x01\x92PPPa\x0B!V[__\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01aAt\x92\x91\x90aV&V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90QaA\xB2\x91\x90aYGV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14aA\xEAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aA\xEFV[``\x91P[P\x91P\x91P\x81\x80\x15aB\x02WP\x80Q` \x14[\x80\x15aB3WP\x80Qc\x0B\x13]?`\xE1\x1B\x90aB'\x90\x83\x01` \x90\x81\x01\x90\x84\x01aY]V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[__\x82Q`A\x03aBsW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1AaBg\x87\x82\x85\x85aB\xAAV[\x94P\x94PPPPaB\xA3V[\x82Q`@\x03aB\x9CW` \x83\x01Q`@\x84\x01QaB\x91\x86\x83\x83aC\x8FV[\x93P\x93PPPaB\xA3V[P_\x90P`\x02[\x92P\x92\x90PV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aB\xDFWP_\x90P`\x03aC\x86V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aB\xF7WP\x84`\xFF\x16`\x1C\x14\x15[\x15aC\x07WP_\x90P`\x04aC\x86V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aCXW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aC\x80W_`\x01\x92P\x92PPaC\x86V[\x91P_\x90P[\x94P\x94\x92PPPV[_\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aC\xAB`\xFF\x86\x90\x1C`\x1BaQ\xBAV[\x90PaC\xB9\x87\x82\x88\x85aB\xAAV[\x93P\x93PPP\x93P\x93\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD9WaD9aD\x03V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD9WaD9aD\x03V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD\x89WaD\x89aD\x03V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aD\xA9WaD\xA9aD\x03V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\xACW__\xFD[_\x82`\x1F\x83\x01\x12aD\xD6W__\xFD[\x815aD\xE9aD\xE4\x82aD\x91V[aDaV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aE\nW__\xFD[` \x85\x01[\x83\x81\x10\x15aE0W\x805aE\"\x81aD\xB3V[\x83R` \x92\x83\x01\x92\x01aE\x0FV[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15aEJW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aE_W__\xFD[a#9\x84\x82\x85\x01aD\xC7V[_` \x82\x84\x03\x12\x15aE{W__\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\xACW__\xFD[___``\x84\x86\x03\x12\x15aE\xA5W__\xFD[\x835\x92P` \x84\x015aE\xB7\x81aE\x82V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_\x82`\x1F\x83\x01\x12aE\xD7W__\xFD[\x815` \x83\x01__`\x01`\x01`@\x1B\x03\x84\x11\x15aE\xF6WaE\xF6aD\x03V[P`\x1F\x83\x01`\x1F\x19\x16` \x01aF\x0B\x81aDaV[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15aF\x1FW__\xFD[\x82\x82` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15aFIW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aF^W__\xFD[a#9\x84\x82\x85\x01aE\xC8V[_` \x82\x84\x03\x12\x15aFzW__\xFD[\x815a\x0B!\x81aD\xB3V[__`@\x83\x85\x03\x12\x15aF\x96W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14aF\xB5W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15aF\xCAW__\xFD[a\x0B!\x82aF\xA5V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\r7V[\x805a\xFF\xFF\x81\x16\x81\x14aF\xB5W__\xFD[_``\x82\x84\x03\x12\x15aG\x0BW__\xFD[aG\x13aD\x17V[\x90P\x815aG \x81aE\x82V[\x81RaG.` \x83\x01aF\xEAV[` \x82\x01RaG?`@\x83\x01aF\xEAV[`@\x82\x01R\x92\x91PPV[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x0C\xACW__\xFD[_\x82`\x1F\x83\x01\x12aGmW__\xFD[\x815aG{aD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aG\x9CW__\xFD[` \x85\x01[\x83\x81\x10\x15aE0W`@\x81\x88\x03\x12\x15aG\xB8W__\xFD[aG\xC0aD?V[\x815aG\xCB\x81aD\xB3V[\x81R` \x82\x015aG\xDB\x81aGJV[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aG\xA1V[____`\xC0\x85\x87\x03\x12\x15aH\x07W__\xFD[aH\x11\x86\x86aF\xFBV[\x93P``\x85\x015aH!\x81aGJV[\x92P`\x80\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aH;W__\xFD[aHG\x87\x82\x88\x01aG^V[\x92PP`\xA0\x85\x015aHX\x81aE\x82V[\x93\x96\x92\x95P\x90\x93PPV[__\x83`\x1F\x84\x01\x12aHsW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aH\x89W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aB\xA3W__\xFD[___`@\x84\x86\x03\x12\x15aH\xB2W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xC7W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aH\xD7W__\xFD[\x805aH\xE5aD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15aI\x06W__\xFD[` \x84\x01[\x83\x81\x10\x15aIFW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aI(W__\xFD[aI7\x8B` \x83\x89\x01\x01aD\xC7V[\x84RP` \x92\x83\x01\x92\x01aI\x0BV[P\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aIdW__\xFD[aIp\x86\x82\x87\x01aHcV[\x94\x97\x90\x96P\x93\x94PPPPV[_____`\xA0\x86\x88\x03\x12\x15aI\x91W__\xFD[\x855aI\x9C\x81aD\xB3V[\x94P` \x86\x015aI\xAC\x81aD\xB3V[\x93P`@\x86\x015aI\xBC\x81aD\xB3V[\x92P``\x86\x015\x91P`\x80\x86\x015aI\xD3\x81aD\xB3V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10aJ\x11WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91a3\xD2\x90\x84\x01\x82aI\xF5V[__`\x80\x83\x85\x03\x12\x15aJAW__\xFD[aJJ\x83aF\xA5V[\x91PaJY\x84` \x85\x01aF\xFBV[\x90P\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aJsW__\xFD[\x825aJ~\x81aD\xB3V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\x98W__\xFD[aJ\xA4\x85\x82\x86\x01aE\xC8V[\x91PP\x92P\x92\x90PV[___`\xA0\x84\x86\x03\x12\x15aJ\xC0W__\xFD[aJ\xCA\x85\x85aF\xFBV[\x92P``\x84\x015aJ\xDA\x81aGJV[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xF4W__\xFD[aK\0\x86\x82\x87\x01aG^V[\x91PP\x92P\x92P\x92V[_\x82`\x1F\x83\x01\x12aK\x19W__\xFD[\x815aK'aD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aKHW__\xFD[` \x85\x01[\x83\x81\x10\x15aE0W`@\x81\x88\x03\x12\x15aKdW__\xFD[aKlaD?V[aKu\x82aF\xA5V[\x81R` \x82\x015aK\x85\x81aD\xB3V[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aKMV[_____`\xA0\x86\x88\x03\x12\x15aK\xB2W__\xFD[\x855aK\xBD\x81aD\xB3V[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xDEW__\xFD[aK\xEA\x88\x82\x89\x01aK\nV[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x015\x93\x92PPPV[_`@\x82\x84\x03\x12\x15aL\x13W__\xFD[aL\x1BaD?V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12aLAW__\xFD[aLIaD?V[\x80`@\x84\x01\x85\x81\x11\x15aLZW__\xFD[\x84[\x81\x81\x10\x15aLtW\x805\x84R` \x93\x84\x01\x93\x01aL\\V[P\x90\x95\x94PPPPPV[_\x81\x83\x03a\x01\0\x81\x12\x15aL\x91W__\xFD[aL\x99aD\x17V[\x91PaL\xA5\x84\x84aL\x03V[\x82RaL\xB4\x84`@\x85\x01aL\x03V[` \x83\x01R`\x80`\x7F\x19\x82\x01\x12\x15aL\xCAW__\xFD[PaL\xD3aD?V[aL\xE0\x84`\x80\x85\x01aL2V[\x81RaL\xEF\x84`\xC0\x85\x01aL2V[` \x82\x01R`@\x82\x01R\x92\x91PPV[_``\x82\x84\x03\x12\x15aM\x0FW__\xFD[aM\x17aD\x17V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aM.W__\xFD[aM:\x84\x82\x85\x01aE\xC8V[\x82RP` \x82\x81\x015\x90\x82\x01R`@\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[_______a\x01\xA0\x88\x8A\x03\x12\x15aMpW__\xFD[\x875`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x85W__\xFD[aM\x91\x8A\x82\x8B\x01aHcV[\x90\x98P\x96PP` \x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xAFW__\xFD[aM\xBB\x8A\x82\x8B\x01aE\xC8V[\x95PPaM\xCB\x89`@\x8A\x01aL\x7FV[\x93Pa\x01@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xE6W__\xFD[aM\xF2\x8A\x82\x8B\x01aK\nV[\x93PPa\x01`\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN\x0EW__\xFD[aN\x1A\x8A\x82\x8B\x01aL\xFFV[\x92PPa\x01\x80\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN6W__\xFD[aNB\x8A\x82\x8B\x01aL\xFFV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_\x82`\x1F\x83\x01\x12aN`W__\xFD[\x815aNnaD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aN\x8FW__\xFD[` \x85\x01[\x83\x81\x10\x15aE0W\x805aN\xA7\x81aE\x82V[\x83R` \x92\x83\x01\x92\x01aN\x94V[__`@\x83\x85\x03\x12\x15aN\xC6W__\xFD[\x825aN\xD1\x81aD\xB3V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xEBW__\xFD[aJ\xA4\x85\x82\x86\x01aNQV[____a\x01`\x85\x87\x03\x12\x15aO\x0BW__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15aO W__\xFD[aO,\x87\x82\x88\x01aE\xC8V[\x94PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aOGW__\xFD[aOS\x87\x82\x88\x01aE\xC8V[\x93PPaOc\x86`@\x87\x01aL\x7FV[\x91Pa\x01@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO~W__\xFD[aO\x8A\x87\x82\x88\x01aL\xFFV[\x91PP\x92\x95\x91\x94P\x92PV[____``\x85\x87\x03\x12\x15aO\xA9W__\xFD[\x845aO\xB4\x81aD\xB3V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xCEW__\xFD[aO\xDA\x87\x82\x88\x01aNQV[\x93PP`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xF5W__\xFD[aP\x01\x87\x82\x88\x01aHcV[\x95\x98\x94\x97P\x95PPPPV[__`@\x83\x85\x03\x12\x15aP\x1EW__\xFD[\x825aP)\x81aE\x82V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aPCW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aPSW__\xFD[\x805aPaaD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aP\x82W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aP\xA4W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aP\x89V[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aLtW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aP\xCBV[` \x81\x01a\r7\x82\x84aI\xF5V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0B!` \x83\x01\x84aQ\x11V[_` \x82\x84\x03\x12\x15aQaW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0B!W__\xFD[_` \x82\x84\x03\x12\x15aQ\x80W__\xFD[\x81Qa\x0B!\x81aD\xB3V[_` \x82\x84\x03\x12\x15aQ\x9BW__\xFD[\x81Qa\x0B!\x81aE\x82V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\r7Wa\r7aQ\xA6V[__\x85\x85\x11\x15aQ\xDBW__\xFD[\x83\x86\x11\x15aQ\xE7W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[_`\xC0\x82\x01\x88\x83R`\x01\x80`\xA0\x1B\x03\x88\x16` \x84\x01R\x86`@\x84\x01R`\xC0``\x84\x01R\x80\x86Q\x80\x83R`\xE0\x85\x01\x91P` \x88\x01\x92P_[\x81\x81\x10\x15aRaW\x83Q\x80Q`\xFF\x16\x84R` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01aR+V[PP`\x80\x84\x01\x95\x90\x95RPP`\xA0\x01R\x94\x93PPPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q```@\x84\x01RaR\xA2`\xA0\x84\x01\x82aQ\x11V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[\x805`\x02\x81\x10aF\xB5W__\xFD[___a\x01@\x84\x86\x03\x12\x15aR\xE4W__\xFD[aR\xED\x84aR\xC3V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x07W__\xFD[aS\x13\x86\x82\x87\x01aE\xC8V[\x92PPaS#\x85`@\x86\x01aL\x7FV[\x90P\x92P\x92P\x92V[_____a\x01\x80\x86\x88\x03\x12\x15aSAW__\xFD[aSJ\x86aR\xC3V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aSdW__\xFD[aSp\x88\x82\x89\x01aE\xC8V[\x94PPaS\x80\x87`@\x88\x01aL\x7FV[\x92Pa\x01@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x9BW__\xFD[aS\xA7\x88\x82\x89\x01aK\nV[\x92PPa\x01`\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xC3W__\xFD[aS\xCF\x88\x82\x89\x01aL\xFFV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_`\x01\x82\x01aS\xEDWaS\xEDaQ\xA6V[P`\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R_aT\x1A``\x83\x01\x84aQ\x11V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15aT3W__\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x0B!W__\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aTkWaTkaTIV[P\x06\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\r7Wa\r7aQ\xA6V[_`@\x82\x01`\x01\x80`\xA0\x1B\x03\x85\x16\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aU>W\x86\x85\x03`_\x19\x01\x84R\x81Q\x80Qc\xFF\xFF\xFF\xFF\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15aU&W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90aT\xFBV[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aT\xBDV[P\x92\x97\x96PPPPPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aU\x99W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x88\x01R`@\x90\x96\x01\x95\x90\x91\x01\x90`\x01\x01aU]V[P\x93\x94\x93PPPPV[`\xFF\x84\x16\x81R`\x01`\x01``\x1B\x03\x83\x16` \x82\x01R```@\x82\x01R_aT\x1A``\x83\x01\x84aUKV[`\xFF\x85\x16\x81R`\x01`\x01``\x1B\x03\x84\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R_a\x17\x05`\x80\x83\x01\x84aUKV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x0B\x1E\x90\x83\x01\x84aQ\x11V[\x82\x81R`@` \x82\x01R_a\x0B\x1E`@\x83\x01\x84aQ\x11V[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x83\x01R\x83\x82\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x83\x01Q``\x80\x84\x01R\x80Q`\x80\x84\x01\x81\x90R_\x92\x91\x90\x91\x01\x90\x82\x90`\xA0\x85\x01\x90[\x80\x83\x10\x15aE0Wc\xFF\xFF\xFF\xFF\x84Q\x16\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92PaV\x83V[_` \x82\x84\x03\x12\x15aV\xBCW__\xFD[PQ\x91\x90PV[\x80_[`\x02\x81\x10\x15a\x0E\x9FW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aV\xC6V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R\x82Q\x80Q` \x80\x84\x01\x91\x90\x91R\x01Q`@\x82\x01Ra\x01`\x81\x01` \x84\x81\x01Q\x80Q``\x85\x01R\x90\x81\x01Q`\x80\x84\x01RP`@\x84\x01QaW4`\xA0\x84\x01\x82QaV\xC3V[` \x01QaWE`\xE0\x84\x01\x82aV\xC3V[P\x82Qa\x01 \x83\x01R` \x83\x01Qa\x01@\x83\x01Ra#9V[_\x82`\x1F\x83\x01\x12aWmW__\xFD[\x81QaW{aD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aW\x9CW__\xFD[` \x85\x01[\x83\x81\x10\x15aE0W\x80QaW\xB4\x81aGJV[\x83R` \x92\x83\x01\x92\x01aW\xA1V[__`@\x83\x85\x03\x12\x15aW\xD3W__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aW\xE8W__\xFD[aW\xF4\x85\x82\x86\x01aW^V[\x92PP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aX\x0FW__\xFD[aJ\xA4\x85\x82\x86\x01aW^V[_` \x82\x84\x03\x12\x15aX+W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aX@W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aXPW__\xFD[\x80QaX^aD\xE4\x82aD\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aX\x7FW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x17\x05W\x83QaX\x99\x81aE\x82V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aX\x86V[\x81\x81\x03\x81\x81\x11\x15a\r7Wa\r7aQ\xA6V[_a\xFF\xFF\x82\x16a\xFF\xFF\x81\x03aX\xD4WaX\xD4aQ\xA6V[`\x01\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15aX\xEDW__\xFD[\x81Qa\x0B!\x81aGJV[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a3\xD2Wa3\xD2aQ\xA6V[_`\x01`\x01``\x1B\x03\x83\x16\x80aY2WaY2aTIV[\x80`\x01`\x01``\x1B\x03\x84\x16\x04\x91PP\x92\x91PPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15aYmW__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B!W__\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xA2\xAD\xA9\x83M)a\x0B\xF3\xE5\xAC\xA5\x08\xA2\xB5\xA6\x1A+\x0C\x9F\x84n\x83\x83\xD3\x80\xCA\x01\x18\x1B\x0F\xDDdsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `AlreadyRegisteredForQuorums()` and selector `0x0c6816cd`.
    ```solidity
    error AlreadyRegisteredForQuorums();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyRegisteredForQuorums {}
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
        impl ::core::convert::From<AlreadyRegisteredForQuorums> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyRegisteredForQuorums) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyRegisteredForQuorums {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyRegisteredForQuorums {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyRegisteredForQuorums()";
            const SELECTOR: [u8; 4] = [12u8, 104u8, 22u8, 205u8];
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
    /**Custom error with signature `BitmapCannotBeZero()` and selector `0xd16d50ea`.
    ```solidity
    error BitmapCannotBeZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapCannotBeZero {}
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
        impl ::core::convert::From<BitmapCannotBeZero> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapCannotBeZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapCannotBeZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapCannotBeZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapCannotBeZero()";
            const SELECTOR: [u8; 4] = [209u8, 109u8, 80u8, 234u8];
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
    /**Custom error with signature `BitmapEmpty()` and selector `0x13ca4657`.
    ```solidity
    error BitmapEmpty();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapEmpty {}
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
        impl ::core::convert::From<BitmapEmpty> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapEmpty) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapEmpty {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapEmpty {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapEmpty()";
            const SELECTOR: [u8; 4] = [19u8, 202u8, 70u8, 87u8];
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
    /**Custom error with signature `BitmapUpdateIsAfterBlockNumber()` and selector `0x6cb19aff`.
    ```solidity
    error BitmapUpdateIsAfterBlockNumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapUpdateIsAfterBlockNumber {}
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
        impl ::core::convert::From<BitmapUpdateIsAfterBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapUpdateIsAfterBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapUpdateIsAfterBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapUpdateIsAfterBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapUpdateIsAfterBlockNumber()";
            const SELECTOR: [u8; 4] = [108u8, 177u8, 154u8, 255u8];
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
    /**Custom error with signature `BitmapValueTooLarge()` and selector `0xca957333`.
    ```solidity
    error BitmapValueTooLarge();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapValueTooLarge {}
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
        impl ::core::convert::From<BitmapValueTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapValueTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapValueTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapValueTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapValueTooLarge()";
            const SELECTOR: [u8; 4] = [202u8, 149u8, 115u8, 51u8];
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
    /**Custom error with signature `BytesArrayLengthTooLong()` and selector `0xfb4a9c8e`.
    ```solidity
    error BytesArrayLengthTooLong();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayLengthTooLong {}
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
        impl ::core::convert::From<BytesArrayLengthTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayLengthTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayLengthTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayLengthTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayLengthTooLong()";
            const SELECTOR: [u8; 4] = [251u8, 74u8, 156u8, 142u8];
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
    /**Custom error with signature `BytesArrayNotOrdered()` and selector `0x80c88348`.
    ```solidity
    error BytesArrayNotOrdered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayNotOrdered {}
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
        impl ::core::convert::From<BytesArrayNotOrdered> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayNotOrdered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayNotOrdered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayNotOrdered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayNotOrdered()";
            const SELECTOR: [u8; 4] = [128u8, 200u8, 131u8, 72u8];
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
    /**Custom error with signature `CannotChurnSelf()` and selector `0xac2d1682`.
    ```solidity
    error CannotChurnSelf();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotChurnSelf {}
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
        impl ::core::convert::From<CannotChurnSelf> for UnderlyingRustTuple<'_> {
            fn from(value: CannotChurnSelf) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotChurnSelf {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotChurnSelf {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotChurnSelf()";
            const SELECTOR: [u8; 4] = [172u8, 45u8, 22u8, 130u8];
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
    /**Custom error with signature `CannotKickOperatorAboveThreshold()` and selector `0xb187e869`.
    ```solidity
    error CannotKickOperatorAboveThreshold();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotKickOperatorAboveThreshold {}
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
        impl ::core::convert::From<CannotKickOperatorAboveThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: CannotKickOperatorAboveThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotKickOperatorAboveThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotKickOperatorAboveThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotKickOperatorAboveThreshold()";
            const SELECTOR: [u8; 4] = [177u8, 135u8, 232u8, 105u8];
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
    /**Custom error with signature `CannotReregisterYet()` and selector `0x32d0cefa`.
    ```solidity
    error CannotReregisterYet();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotReregisterYet {}
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
        impl ::core::convert::From<CannotReregisterYet> for UnderlyingRustTuple<'_> {
            fn from(value: CannotReregisterYet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotReregisterYet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotReregisterYet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotReregisterYet()";
            const SELECTOR: [u8; 4] = [50u8, 208u8, 206u8, 250u8];
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
    /**Custom error with signature `ChurnApproverSaltUsed()` and selector `0xdf7dfd86`.
    ```solidity
    error ChurnApproverSaltUsed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChurnApproverSaltUsed {}
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
        impl ::core::convert::From<ChurnApproverSaltUsed> for UnderlyingRustTuple<'_> {
            fn from(value: ChurnApproverSaltUsed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChurnApproverSaltUsed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ChurnApproverSaltUsed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ChurnApproverSaltUsed()";
            const SELECTOR: [u8; 4] = [223u8, 125u8, 253u8, 134u8];
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
    /**Custom error with signature `CurrentlyPaused()` and selector `0x840a48d5`.
    ```solidity
    error CurrentlyPaused();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CurrentlyPaused {}
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
        impl ::core::convert::From<CurrentlyPaused> for UnderlyingRustTuple<'_> {
            fn from(value: CurrentlyPaused) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CurrentlyPaused {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CurrentlyPaused {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CurrentlyPaused()";
            const SELECTOR: [u8; 4] = [132u8, 10u8, 72u8, 213u8];
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
    /**Custom error with signature `ExpModFailed()` and selector `0xd51edae3`.
    ```solidity
    error ExpModFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpModFailed {}
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
        impl ::core::convert::From<ExpModFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ExpModFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpModFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpModFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpModFailed()";
            const SELECTOR: [u8; 4] = [213u8, 30u8, 218u8, 227u8];
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
    /**Custom error with signature `InputAddressZero()` and selector `0x73632176`.
    ```solidity
    error InputAddressZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputAddressZero {}
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
        impl ::core::convert::From<InputAddressZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputAddressZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputAddressZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputAddressZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputAddressZero()";
            const SELECTOR: [u8; 4] = [115u8, 99u8, 33u8, 118u8];
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
    /**Custom error with signature `InputLengthMismatch()` and selector `0xaaad13f7`.
    ```solidity
    error InputLengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputLengthMismatch {}
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
        impl ::core::convert::From<InputLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputLengthMismatch()";
            const SELECTOR: [u8; 4] = [170u8, 173u8, 19u8, 247u8];
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
    /**Custom error with signature `InsufficientStakeForChurn()` and selector `0x4c44995d`.
    ```solidity
    error InsufficientStakeForChurn();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientStakeForChurn {}
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
        impl ::core::convert::From<InsufficientStakeForChurn> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientStakeForChurn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientStakeForChurn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientStakeForChurn {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientStakeForChurn()";
            const SELECTOR: [u8; 4] = [76u8, 68u8, 153u8, 93u8];
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
    /**Custom error with signature `InvalidNewPausedStatus()` and selector `0xc61dca5d`.
    ```solidity
    error InvalidNewPausedStatus();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNewPausedStatus {}
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
        impl ::core::convert::From<InvalidNewPausedStatus> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNewPausedStatus) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNewPausedStatus {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNewPausedStatus {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNewPausedStatus()";
            const SELECTOR: [u8; 4] = [198u8, 29u8, 202u8, 93u8];
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
    /**Custom error with signature `InvalidRegistrationType()` and selector `0x354bb8ab`.
    ```solidity
    error InvalidRegistrationType();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRegistrationType {}
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
        impl ::core::convert::From<InvalidRegistrationType> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRegistrationType) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRegistrationType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRegistrationType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRegistrationType()";
            const SELECTOR: [u8; 4] = [53u8, 75u8, 184u8, 171u8];
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
    /**Custom error with signature `M2QuorumsAlreadyDisabled()` and selector `0xebb100f8`.
    ```solidity
    error M2QuorumsAlreadyDisabled();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct M2QuorumsAlreadyDisabled {}
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
        impl ::core::convert::From<M2QuorumsAlreadyDisabled> for UnderlyingRustTuple<'_> {
            fn from(value: M2QuorumsAlreadyDisabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for M2QuorumsAlreadyDisabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for M2QuorumsAlreadyDisabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "M2QuorumsAlreadyDisabled()";
            const SELECTOR: [u8; 4] = [235u8, 177u8, 0u8, 248u8];
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
    /**Custom error with signature `MaxQuorumsReached()` and selector `0x3cb89c97`.
    ```solidity
    error MaxQuorumsReached();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxQuorumsReached {}
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
        impl ::core::convert::From<MaxQuorumsReached> for UnderlyingRustTuple<'_> {
            fn from(value: MaxQuorumsReached) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MaxQuorumsReached {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxQuorumsReached {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxQuorumsReached()";
            const SELECTOR: [u8; 4] = [60u8, 184u8, 156u8, 151u8];
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
    /**Custom error with signature `NextBitmapUpdateIsBeforeBlockNumber()` and selector `0xbbba60cb`.
    ```solidity
    error NextBitmapUpdateIsBeforeBlockNumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NextBitmapUpdateIsBeforeBlockNumber {}
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
        impl ::core::convert::From<NextBitmapUpdateIsBeforeBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: NextBitmapUpdateIsBeforeBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NextBitmapUpdateIsBeforeBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NextBitmapUpdateIsBeforeBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NextBitmapUpdateIsBeforeBlockNumber()";
            const SELECTOR: [u8; 4] = [187u8, 186u8, 96u8, 203u8];
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
    /**Custom error with signature `NotRegistered()` and selector `0xaba47339`.
    ```solidity
    error NotRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotRegistered {}
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
        impl ::core::convert::From<NotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: NotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotRegistered()";
            const SELECTOR: [u8; 4] = [171u8, 164u8, 115u8, 57u8];
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
    /**Custom error with signature `NotRegisteredForQuorum()` and selector `0xd053aa21`.
    ```solidity
    error NotRegisteredForQuorum();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotRegisteredForQuorum {}
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
        impl ::core::convert::From<NotRegisteredForQuorum> for UnderlyingRustTuple<'_> {
            fn from(value: NotRegisteredForQuorum) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotRegisteredForQuorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotRegisteredForQuorum {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotRegisteredForQuorum()";
            const SELECTOR: [u8; 4] = [208u8, 83u8, 170u8, 33u8];
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
    /**Custom error with signature `OnlyAllocationManager()` and selector `0x23d871a5`.
    ```solidity
    error OnlyAllocationManager();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyAllocationManager {}
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
        impl ::core::convert::From<OnlyAllocationManager> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyAllocationManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyAllocationManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyAllocationManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyAllocationManager()";
            const SELECTOR: [u8; 4] = [35u8, 216u8, 113u8, 165u8];
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
    /**Custom error with signature `OnlyEjector()` and selector `0xedb1562e`.
    ```solidity
    error OnlyEjector();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEjector {}
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
        impl ::core::convert::From<OnlyEjector> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEjector) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyEjector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEjector {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEjector()";
            const SELECTOR: [u8; 4] = [237u8, 177u8, 86u8, 46u8];
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
    /**Custom error with signature `OnlyPauser()` and selector `0x75df51dc`.
    ```solidity
    error OnlyPauser();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyPauser {}
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
        impl ::core::convert::From<OnlyPauser> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyPauser) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyPauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyPauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyPauser()";
            const SELECTOR: [u8; 4] = [117u8, 223u8, 81u8, 220u8];
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
    /**Custom error with signature `OnlyUnpauser()` and selector `0x794821ff`.
    ```solidity
    error OnlyUnpauser();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyUnpauser {}
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
        impl ::core::convert::From<OnlyUnpauser> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyUnpauser) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyUnpauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyUnpauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyUnpauser()";
            const SELECTOR: [u8; 4] = [121u8, 72u8, 33u8, 255u8];
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
    /**Custom error with signature `OperatorSetsAlreadyEnabled()` and selector `0xb2e18e05`.
    ```solidity
    error OperatorSetsAlreadyEnabled();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSetsAlreadyEnabled {}
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
        impl ::core::convert::From<OperatorSetsAlreadyEnabled> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSetsAlreadyEnabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSetsAlreadyEnabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorSetsAlreadyEnabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorSetsAlreadyEnabled()";
            const SELECTOR: [u8; 4] = [178u8, 225u8, 142u8, 5u8];
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
    /**Custom error with signature `OperatorSetsNotEnabled()` and selector `0x5b779019`.
    ```solidity
    error OperatorSetsNotEnabled();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSetsNotEnabled {}
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
        impl ::core::convert::From<OperatorSetsNotEnabled> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSetsNotEnabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSetsNotEnabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorSetsNotEnabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorSetsNotEnabled()";
            const SELECTOR: [u8; 4] = [91u8, 119u8, 144u8, 25u8];
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
    /**Custom error with signature `QuorumOperatorCountMismatch()` and selector `0x8e5aeee7`.
    ```solidity
    error QuorumOperatorCountMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumOperatorCountMismatch {}
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
        impl ::core::convert::From<QuorumOperatorCountMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumOperatorCountMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumOperatorCountMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumOperatorCountMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QuorumOperatorCountMismatch()";
            const SELECTOR: [u8; 4] = [142u8, 90u8, 238u8, 231u8];
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
    /**Custom error with signature `RegistryCoordinatorSignatureExpired()` and selector `0x9a15098d`.
    ```solidity
    error RegistryCoordinatorSignatureExpired();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RegistryCoordinatorSignatureExpired {}
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
        impl ::core::convert::From<RegistryCoordinatorSignatureExpired> for UnderlyingRustTuple<'_> {
            fn from(value: RegistryCoordinatorSignatureExpired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RegistryCoordinatorSignatureExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RegistryCoordinatorSignatureExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RegistryCoordinatorSignatureExpired()";
            const SELECTOR: [u8; 4] = [154u8, 21u8, 9u8, 141u8];
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
    /**Custom error with signature `SaltAlreadyUsed()` and selector `0x0ced3043`.
    ```solidity
    error SaltAlreadyUsed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SaltAlreadyUsed {}
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
        impl ::core::convert::From<SaltAlreadyUsed> for UnderlyingRustTuple<'_> {
            fn from(value: SaltAlreadyUsed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SaltAlreadyUsed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SaltAlreadyUsed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SaltAlreadyUsed()";
            const SELECTOR: [u8; 4] = [12u8, 237u8, 48u8, 67u8];
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
    /**Custom error with signature `SignatureExpired()` and selector `0x0819bdcd`.
    ```solidity
    error SignatureExpired();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureExpired {}
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
        impl ::core::convert::From<SignatureExpired> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureExpired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignatureExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignatureExpired()";
            const SELECTOR: [u8; 4] = [8u8, 25u8, 189u8, 205u8];
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
    /**Event with signature `ChurnApproverUpdated(address,address)` and selector `0x315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c`.
    ```solidity
    event ChurnApproverUpdated(address prevChurnApprover, address newChurnApprover);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChurnApproverUpdated {
        #[allow(missing_docs)]
        pub prevChurnApprover: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newChurnApprover: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ChurnApproverUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ChurnApproverUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 84u8, 87u8, 216u8, 168u8, 254u8, 96u8, 240u8, 74u8, 241u8, 124u8, 22u8,
                    226u8, 245u8, 165u8, 225u8, 219u8, 97u8, 43u8, 49u8, 100u8, 142u8, 88u8, 3u8,
                    3u8, 96u8, 117u8, 158u8, 248u8, 243u8, 82u8, 140u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevChurnApprover: data.0,
                    newChurnApprover: data.1,
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
                        &self.prevChurnApprover,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newChurnApprover,
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
        impl alloy_sol_types::private::IntoLogData for ChurnApproverUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChurnApproverUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChurnApproverUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EjectorUpdated(address,address)` and selector `0x8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9`.
    ```solidity
    event EjectorUpdated(address prevEjector, address newEjector);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EjectorUpdated {
        #[allow(missing_docs)]
        pub prevEjector: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newEjector: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for EjectorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EjectorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    143u8, 48u8, 171u8, 9u8, 244u8, 58u8, 108u8, 21u8, 125u8, 127u8, 206u8, 126u8,
                    10u8, 19u8, 192u8, 3u8, 4u8, 44u8, 28u8, 149u8, 232u8, 167u8, 46u8, 122u8,
                    20u8, 106u8, 33u8, 192u8, 202u8, 162u8, 77u8, 201u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevEjector: data.0,
                    newEjector: data.1,
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
                        &self.prevEjector,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newEjector,
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
        impl alloy_sol_types::private::IntoLogData for EjectorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EjectorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EjectorUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
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
    /**Event with signature `M2QuorumsDisabled()` and selector `0xa4cd42920ed0d1372ba4051d4577279f236fbbe677a67f3f7d645e82425dd98d`.
    ```solidity
    event M2QuorumsDisabled();
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct M2QuorumsDisabled {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for M2QuorumsDisabled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "M2QuorumsDisabled()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    164u8, 205u8, 66u8, 146u8, 14u8, 208u8, 209u8, 55u8, 43u8, 164u8, 5u8, 29u8,
                    69u8, 119u8, 39u8, 159u8, 35u8, 111u8, 187u8, 230u8, 119u8, 166u8, 127u8, 63u8,
                    125u8, 100u8, 94u8, 130u8, 66u8, 93u8, 217u8, 141u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
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
        impl alloy_sol_types::private::IntoLogData for M2QuorumsDisabled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&M2QuorumsDisabled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &M2QuorumsDisabled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorDeregistered(address,bytes32)` and selector `0x396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e4`.
    ```solidity
    event OperatorDeregistered(address indexed operator, bytes32 indexed operatorId);
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
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    57u8, 111u8, 220u8, 177u8, 128u8, 203u8, 15u8, 234u8, 38u8, 146u8, 129u8, 19u8,
                    251u8, 15u8, 209u8, 195u8, 84u8, 152u8, 99u8, 249u8, 205u8, 86u8, 62u8, 106u8,
                    24u8, 79u8, 29u8, 87u8, 129u8, 22u8, 200u8, 228u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    operatorId: topics.2,
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
                    self.operator.clone(),
                    self.operatorId.clone(),
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
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
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
    /**Event with signature `OperatorRegistered(address,bytes32)` and selector `0xe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe`.
    ```solidity
    event OperatorRegistered(address indexed operator, bytes32 indexed operatorId);
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
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    232u8, 230u8, 140u8, 239u8, 28u8, 58u8, 118u8, 30u8, 215u8, 190u8, 126u8,
                    132u8, 99u8, 163u8, 117u8, 242u8, 127u8, 123u8, 195u8, 53u8, 229u8, 24u8, 36u8,
                    34u8, 60u8, 172u8, 206u8, 99u8, 110u8, 197u8, 195u8, 254u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    operatorId: topics.2,
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
                    self.operator.clone(),
                    self.operatorId.clone(),
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
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
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
    /**Event with signature `OperatorSetParamsUpdated(uint8,(uint32,uint16,uint16))` and selector `0x3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac`.
    ```solidity
    event OperatorSetParamsUpdated(uint8 indexed quorumNumber, ISlashingRegistryCoordinator.OperatorSetParam operatorSetParams);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetParamsUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operatorSetParams:
            <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorSetParamsUpdated {
            type DataTuple<'a> = (ISlashingRegistryCoordinator::OperatorSetParam,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str =
                "OperatorSetParamsUpdated(uint8,(uint32,uint16,uint16))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    62u8, 230u8, 254u8, 141u8, 84u8, 97u8, 2u8, 68u8, 195u8, 233u8, 211u8, 192u8,
                    102u8, 174u8, 74u8, 238u8, 153u8, 120u8, 132u8, 170u8, 40u8, 241u8, 6u8, 22u8,
                    174u8, 130u8, 25u8, 37u8, 64u8, 19u8, 24u8, 172u8,
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
                    operatorSetParams: data.0,
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
                    <ISlashingRegistryCoordinator::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
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
        impl alloy_sol_types::private::IntoLogData for OperatorSetParamsUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetParamsUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSetParamsUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSetsEnabled()` and selector `0x0b88306ff4627121f5b3e5b1c5f88f6b1e42fd2c0478ef1c91662d49d1f07755`.
    ```solidity
    event OperatorSetsEnabled();
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetsEnabled {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorSetsEnabled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorSetsEnabled()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    11u8, 136u8, 48u8, 111u8, 244u8, 98u8, 113u8, 33u8, 245u8, 179u8, 229u8, 177u8,
                    197u8, 248u8, 143u8, 107u8, 30u8, 66u8, 253u8, 44u8, 4u8, 120u8, 239u8, 28u8,
                    145u8, 102u8, 45u8, 73u8, 209u8, 240u8, 119u8, 85u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
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
        impl alloy_sol_types::private::IntoLogData for OperatorSetsEnabled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetsEnabled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSetsEnabled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSocketUpdate(bytes32,string)` and selector `0xec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa`.
    ```solidity
    event OperatorSocketUpdate(bytes32 indexed operatorId, string socket);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSocketUpdate {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub socket: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for OperatorSocketUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorSocketUpdate(bytes32,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    236u8, 41u8, 99u8, 171u8, 33u8, 193u8, 229u8, 14u8, 30u8, 88u8, 42u8, 165u8,
                    66u8, 175u8, 46u8, 75u8, 247u8, 191u8, 56u8, 230u8, 225u8, 64u8, 60u8, 39u8,
                    180u8, 46u8, 28u8, 93u8, 110u8, 98u8, 30u8, 170u8,
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
                    socket: data.0,
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
                        &self.socket,
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
        impl alloy_sol_types::private::IntoLogData for OperatorSocketUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSocketUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSocketUpdate) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8,
                    208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8,
                    175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    /**Event with signature `Paused(address,uint256)` and selector `0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d`.
    ```solidity
    event Paused(address indexed account, uint256 newPausedStatus);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Paused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                    152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                    224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
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
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
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
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumBlockNumberUpdated(uint8,uint256)` and selector `0x46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db4`.
    ```solidity
    event QuorumBlockNumberUpdated(uint8 indexed quorumNumber, uint256 blocknumber);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumBlockNumberUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub blocknumber: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for QuorumBlockNumberUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "QuorumBlockNumberUpdated(uint8,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    70u8, 7u8, 125u8, 85u8, 51u8, 7u8, 99u8, 241u8, 98u8, 105u8, 253u8, 117u8,
                    229u8, 118u8, 22u8, 99u8, 244u8, 25u8, 45u8, 39u8, 145u8, 116u8, 124u8, 1u8,
                    137u8, 177u8, 106u8, 211u8, 29u8, 176u8, 125u8, 180u8,
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
                    blocknumber: data.0,
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
                        &self.blocknumber,
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
        impl alloy_sol_types::private::IntoLogData for QuorumBlockNumberUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumBlockNumberUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumBlockNumberUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Unpaused(address,uint256)` and selector `0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c`.
    ```solidity
    event Unpaused(address indexed account, uint256 newPausedStatus);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Unpaused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                    188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                    107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
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
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
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
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _serviceManager, address _stakeRegistry, address _blsApkRegistry, address _indexRegistry, address _allocationManager, address _pauserRegistry);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _serviceManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _stakeRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _blsApkRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _indexRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _pauserRegistry: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
                        value._serviceManager,
                        value._stakeRegistry,
                        value._blsApkRegistry,
                        value._indexRegistry,
                        value._allocationManager,
                        value._pauserRegistry,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _serviceManager: tuple.0,
                        _stakeRegistry: tuple.1,
                        _blsApkRegistry: tuple.2,
                        _indexRegistry: tuple.3,
                        _allocationManager: tuple.4,
                        _pauserRegistry: tuple.5,
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
                        &self._serviceManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._stakeRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._blsApkRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._indexRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                )
            }
        }
    };
    /**Function with signature `OPERATOR_CHURN_APPROVAL_TYPEHASH()` and selector `0xca0de882`.
    ```solidity
    function OPERATOR_CHURN_APPROVAL_TYPEHASH() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_CHURN_APPROVAL_TYPEHASHCall {}
    ///Container type for the return parameters of the [`OPERATOR_CHURN_APPROVAL_TYPEHASH()`](OPERATOR_CHURN_APPROVAL_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_CHURN_APPROVAL_TYPEHASHReturn {
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
            impl ::core::convert::From<OPERATOR_CHURN_APPROVAL_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_CHURN_APPROVAL_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_CHURN_APPROVAL_TYPEHASHCall {
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
            impl ::core::convert::From<OPERATOR_CHURN_APPROVAL_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_CHURN_APPROVAL_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_CHURN_APPROVAL_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_CHURN_APPROVAL_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = OPERATOR_CHURN_APPROVAL_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPERATOR_CHURN_APPROVAL_TYPEHASH()";
            const SELECTOR: [u8; 4] = [202u8, 13u8, 232u8, 130u8];
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
    /**Function with signature `PUBKEY_REGISTRATION_TYPEHASH()` and selector `0x9feab859`.
    ```solidity
    function PUBKEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PUBKEY_REGISTRATION_TYPEHASHCall {}
    ///Container type for the return parameters of the [`PUBKEY_REGISTRATION_TYPEHASH()`](PUBKEY_REGISTRATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PUBKEY_REGISTRATION_TYPEHASHReturn {
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
            impl ::core::convert::From<PUBKEY_REGISTRATION_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: PUBKEY_REGISTRATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PUBKEY_REGISTRATION_TYPEHASHCall {
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
            impl ::core::convert::From<PUBKEY_REGISTRATION_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: PUBKEY_REGISTRATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PUBKEY_REGISTRATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PUBKEY_REGISTRATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = PUBKEY_REGISTRATION_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PUBKEY_REGISTRATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [159u8, 234u8, 184u8, 89u8];
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
    /**Function with signature `accountIdentifier()` and selector `0x0764cb93`.
    ```solidity
    function accountIdentifier() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct accountIdentifierCall {}
    ///Container type for the return parameters of the [`accountIdentifier()`](accountIdentifierCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct accountIdentifierReturn {
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
            impl ::core::convert::From<accountIdentifierCall> for UnderlyingRustTuple<'_> {
                fn from(value: accountIdentifierCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for accountIdentifierCall {
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
            impl ::core::convert::From<accountIdentifierReturn> for UnderlyingRustTuple<'_> {
                fn from(value: accountIdentifierReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for accountIdentifierReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for accountIdentifierCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = accountIdentifierReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "accountIdentifier()";
            const SELECTOR: [u8; 4] = [7u8, 100u8, 203u8, 147u8];
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blsApkRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blsApkRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = blsApkRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)` and selector `0x84ca5213`.
    ```solidity
    function calculateOperatorChurnApprovalDigestHash(address registeringOperator, bytes32 registeringOperatorId, ISlashingRegistryCoordinator.OperatorKickParam[] memory operatorKickParams, bytes32 salt, uint256 expiry) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorChurnApprovalDigestHashCall {
        #[allow(missing_docs)]
        pub registeringOperator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub registeringOperatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub operatorKickParams: alloy::sol_types::private::Vec<
            <ISlashingRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)`](calculateOperatorChurnApprovalDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorChurnApprovalDigestHashReturn {
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
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<ISlashingRegistryCoordinator::OperatorKickParam>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    <ISlashingRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<calculateOperatorChurnApprovalDigestHashCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculateOperatorChurnApprovalDigestHashCall) -> Self {
                    (
                        value.registeringOperator,
                        value.registeringOperatorId,
                        value.operatorKickParams,
                        value.salt,
                        value.expiry,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculateOperatorChurnApprovalDigestHashCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registeringOperator: tuple.0,
                        registeringOperatorId: tuple.1,
                        operatorKickParams: tuple.2,
                        salt: tuple.3,
                        expiry: tuple.4,
                    }
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
            impl ::core::convert::From<calculateOperatorChurnApprovalDigestHashReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculateOperatorChurnApprovalDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculateOperatorChurnApprovalDigestHashReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateOperatorChurnApprovalDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<ISlashingRegistryCoordinator::OperatorKickParam>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateOperatorChurnApprovalDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)";
            const SELECTOR: [u8; 4] = [132u8, 202u8, 82u8, 19u8];
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
                        &self.registeringOperator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.registeringOperatorId,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        ISlashingRegistryCoordinator::OperatorKickParam,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorKickParams),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
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
    /**Function with signature `churnApprover()` and selector `0x054310e6`.
    ```solidity
    function churnApprover() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct churnApproverCall {}
    ///Container type for the return parameters of the [`churnApprover()`](churnApproverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct churnApproverReturn {
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
            impl ::core::convert::From<churnApproverCall> for UnderlyingRustTuple<'_> {
                fn from(value: churnApproverCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for churnApproverCall {
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
            impl ::core::convert::From<churnApproverReturn> for UnderlyingRustTuple<'_> {
                fn from(value: churnApproverReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for churnApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for churnApproverCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = churnApproverReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "churnApprover()";
            const SELECTOR: [u8; 4] = [5u8, 67u8, 16u8, 230u8];
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
    /**Function with signature `createSlashableStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[],uint32)` and selector `0x3eef3a51`.
    ```solidity
    function createSlashableStakeQuorum(ISlashingRegistryCoordinator.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory strategyParams, uint32 lookAheadPeriod) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSlashableStakeQuorumCall {
        #[allow(missing_docs)]
        pub operatorSetParams:
            <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub lookAheadPeriod: u32,
    }
    ///Container type for the return parameters of the [`createSlashableStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[],uint32)`](createSlashableStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSlashableStakeQuorumReturn {}
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
                ISlashingRegistryCoordinator::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::Vec<
                    <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
                >,
                u32,
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
            impl ::core::convert::From<createSlashableStakeQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: createSlashableStakeQuorumCall) -> Self {
                    (
                        value.operatorSetParams,
                        value.minimumStake,
                        value.strategyParams,
                        value.lookAheadPeriod,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createSlashableStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetParams: tuple.0,
                        minimumStake: tuple.1,
                        strategyParams: tuple.2,
                        lookAheadPeriod: tuple.3,
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
            impl ::core::convert::From<createSlashableStakeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createSlashableStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createSlashableStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createSlashableStakeQuorumCall {
            type Parameters<'a> = (
                ISlashingRegistryCoordinator::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createSlashableStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createSlashableStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[],uint32)";
            const SELECTOR: [u8; 4] = [62u8, 239u8, 58u8, 81u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISlashingRegistryCoordinator::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistry::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategyParams),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.lookAheadPeriod),
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
    /**Function with signature `createTotalDelegatedStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])` and selector `0x8281ab75`.
    ```solidity
    function createTotalDelegatedStakeQuorum(ISlashingRegistryCoordinator.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory strategyParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTotalDelegatedStakeQuorumCall {
        #[allow(missing_docs)]
        pub operatorSetParams:
            <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createTotalDelegatedStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])`](createTotalDelegatedStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTotalDelegatedStakeQuorumReturn {}
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
                ISlashingRegistryCoordinator::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::Vec<
                    <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createTotalDelegatedStakeQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: createTotalDelegatedStakeQuorumCall) -> Self {
                    (
                        value.operatorSetParams,
                        value.minimumStake,
                        value.strategyParams,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createTotalDelegatedStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetParams: tuple.0,
                        minimumStake: tuple.1,
                        strategyParams: tuple.2,
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
            impl ::core::convert::From<createTotalDelegatedStakeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createTotalDelegatedStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createTotalDelegatedStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createTotalDelegatedStakeQuorumCall {
            type Parameters<'a> = (
                ISlashingRegistryCoordinator::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createTotalDelegatedStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "createTotalDelegatedStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [130u8, 129u8, 171u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISlashingRegistryCoordinator::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistry::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategyParams),
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
    /**Function with signature `deregisterOperator(address,uint32[])` and selector `0x9d8e0c23`.
    ```solidity
    function deregisterOperator(address operator, uint32[] memory operatorSetIds) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperator_0Call {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`deregisterOperator(address,uint32[])`](deregisterOperator_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperator_0Return {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperator_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperator_0Call) -> Self {
                    (value.operator, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperator_0Call {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperator_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperator_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperator_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperator_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperator_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator(address,uint32[])";
            const SELECTOR: [u8; 4] = [157u8, 142u8, 12u8, 35u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `deregisterOperator(bytes)` and selector `0xca4f2d97`.
    ```solidity
    function deregisterOperator(bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperator_1Call {
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`deregisterOperator(bytes)`](deregisterOperator_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperator_1Return {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
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
            impl ::core::convert::From<deregisterOperator_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperator_1Call) -> Self {
                    (value.quorumNumbers,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperator_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumbers: tuple.0,
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
            impl ::core::convert::From<deregisterOperator_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperator_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperator_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperator_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperator_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator(bytes)";
            const SELECTOR: [u8; 4] = [202u8, 79u8, 45u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
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
    /**Function with signature `disableM2QuorumRegistration()` and selector `0x733b7507`.
    ```solidity
    function disableM2QuorumRegistration() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableM2QuorumRegistrationCall {}
    ///Container type for the return parameters of the [`disableM2QuorumRegistration()`](disableM2QuorumRegistrationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableM2QuorumRegistrationReturn {}
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
            impl ::core::convert::From<disableM2QuorumRegistrationCall> for UnderlyingRustTuple<'_> {
                fn from(value: disableM2QuorumRegistrationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableM2QuorumRegistrationCall {
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
            impl ::core::convert::From<disableM2QuorumRegistrationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: disableM2QuorumRegistrationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableM2QuorumRegistrationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disableM2QuorumRegistrationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableM2QuorumRegistrationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disableM2QuorumRegistration()";
            const SELECTOR: [u8; 4] = [115u8, 59u8, 117u8, 7u8];
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
    /**Function with signature `ejectOperator(address,bytes)` and selector `0x6e3b17db`.
    ```solidity
    function ejectOperator(address operator, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectOperatorCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`ejectOperator(address,bytes)`](ejectOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectOperatorReturn {}
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<ejectOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: ejectOperatorCall) -> Self {
                    (value.operator, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
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
            impl ::core::convert::From<ejectOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ejectOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ejectOperator(address,bytes)";
            const SELECTOR: [u8; 4] = [110u8, 59u8, 23u8, 219u8];
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
    /**Function with signature `ejectionCooldown()` and selector `0xa96f783e`.
    ```solidity
    function ejectionCooldown() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectionCooldownCall {}
    ///Container type for the return parameters of the [`ejectionCooldown()`](ejectionCooldownCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectionCooldownReturn {
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
            impl ::core::convert::From<ejectionCooldownCall> for UnderlyingRustTuple<'_> {
                fn from(value: ejectionCooldownCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectionCooldownCall {
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
            impl ::core::convert::From<ejectionCooldownReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ejectionCooldownReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectionCooldownReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectionCooldownCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectionCooldownReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ejectionCooldown()";
            const SELECTOR: [u8; 4] = [169u8, 111u8, 120u8, 62u8];
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
    /**Function with signature `ejector()` and selector `0x28f61b31`.
    ```solidity
    function ejector() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectorCall {}
    ///Container type for the return parameters of the [`ejector()`](ejectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectorReturn {
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
            impl ::core::convert::From<ejectorCall> for UnderlyingRustTuple<'_> {
                fn from(value: ejectorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectorCall {
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
            impl ::core::convert::From<ejectorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ejectorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ejector()";
            const SELECTOR: [u8; 4] = [40u8, 246u8, 27u8, 49u8];
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
    /**Function with signature `enableOperatorSets()` and selector `0xee318821`.
    ```solidity
    function enableOperatorSets() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableOperatorSetsCall {}
    ///Container type for the return parameters of the [`enableOperatorSets()`](enableOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableOperatorSetsReturn {}
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
            impl ::core::convert::From<enableOperatorSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: enableOperatorSetsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for enableOperatorSetsCall {
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
            impl ::core::convert::From<enableOperatorSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: enableOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for enableOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for enableOperatorSetsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = enableOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "enableOperatorSets()";
            const SELECTOR: [u8; 4] = [238u8, 49u8, 136u8, 33u8];
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
    /**Function with signature `getCurrentQuorumBitmap(bytes32)` and selector `0x871ef049`.
    ```solidity
    function getCurrentQuorumBitmap(bytes32 operatorId) external view returns (uint192);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentQuorumBitmapCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getCurrentQuorumBitmap(bytes32)`](getCurrentQuorumBitmapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentQuorumBitmapReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<getCurrentQuorumBitmapCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentQuorumBitmapCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentQuorumBitmapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
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
            impl ::core::convert::From<getCurrentQuorumBitmapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentQuorumBitmapReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentQuorumBitmapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentQuorumBitmapCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentQuorumBitmapReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentQuorumBitmap(bytes32)";
            const SELECTOR: [u8; 4] = [135u8, 30u8, 240u8, 73u8];
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
    /**Function with signature `getOperator(address)` and selector `0x5865c60c`.
    ```solidity
    function getOperator(address operator) external view returns (ISlashingRegistryCoordinator.OperatorInfo memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperator(address)`](getOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorReturn {
        #[allow(missing_docs)]
        pub _0: <ISlashingRegistryCoordinator::OperatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinator::OperatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinator::OperatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinator::OperatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperator(address)";
            const SELECTOR: [u8; 4] = [88u8, 101u8, 198u8, 12u8];
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
    /**Function with signature `getOperatorFromId(bytes32)` and selector `0x296bb064`.
    ```solidity
    function getOperatorFromId(bytes32 operatorId) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromIdCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getOperatorFromId(bytes32)`](getOperatorFromIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromIdReturn {
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
            impl ::core::convert::From<getOperatorFromIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromIdCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorFromIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
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
            impl ::core::convert::From<getOperatorFromIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorFromIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorFromIdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorFromIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorFromId(bytes32)";
            const SELECTOR: [u8; 4] = [41u8, 107u8, 176u8, 100u8];
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
    /**Function with signature `getOperatorId(address)` and selector `0x13542a4e`.
    ```solidity
    function getOperatorId(address operator) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorIdCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorId(address)`](getOperatorIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorIdReturn {
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
            impl ::core::convert::From<getOperatorIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorIdCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<getOperatorIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorIdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorId(address)";
            const SELECTOR: [u8; 4] = [19u8, 84u8, 42u8, 78u8];
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
    /**Function with signature `getOperatorSetParams(uint8)` and selector `0xe65797ad`.
    ```solidity
    function getOperatorSetParams(uint8 quorumNumber) external view returns (ISlashingRegistryCoordinator.OperatorSetParam memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetParamsCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getOperatorSetParams(uint8)`](getOperatorSetParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetParamsReturn {
        #[allow(missing_docs)]
        pub _0:
            <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorSetParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetParamsCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinator::OperatorSetParam,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorSetParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetParamsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetParamsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSetParamsReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinator::OperatorSetParam,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSetParams(uint8)";
            const SELECTOR: [u8; 4] = [230u8, 87u8, 151u8, 173u8];
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
    /**Function with signature `getOperatorStatus(address)` and selector `0xfd39105a`.
    ```solidity
    function getOperatorStatus(address operator) external view returns (ISlashingRegistryCoordinator.OperatorStatus);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStatusCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorStatus(address)`](getOperatorStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStatusReturn {
        #[allow(missing_docs)]
        pub _0:
            <ISlashingRegistryCoordinator::OperatorStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorStatusCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStatusCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinator::OperatorStatus,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinator::OperatorStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorStatusReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStatusReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorStatusCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorStatusReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinator::OperatorStatus,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorStatus(address)";
            const SELECTOR: [u8; 4] = [253u8, 57u8, 16u8, 90u8];
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
    /**Function with signature `getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)` and selector `0x04ec6351`.
    ```solidity
    function getQuorumBitmapAtBlockNumberByIndex(bytes32 operatorId, uint32 blockNumber, uint256 index) external view returns (uint192);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapAtBlockNumberByIndexCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub blockNumber: u32,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)`](getQuorumBitmapAtBlockNumberByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapAtBlockNumberByIndexReturn {
        #[allow(missing_docs)]
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getQuorumBitmapAtBlockNumberByIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapAtBlockNumberByIndexCall) -> Self {
                    (value.operatorId, value.blockNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapAtBlockNumberByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        blockNumber: tuple.1,
                        index: tuple.2,
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
            impl ::core::convert::From<getQuorumBitmapAtBlockNumberByIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapAtBlockNumberByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapAtBlockNumberByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapAtBlockNumberByIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapAtBlockNumberByIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)";
            const SELECTOR: [u8; 4] = [4u8, 236u8, 99u8, 81u8];
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
    /**Function with signature `getQuorumBitmapHistoryLength(bytes32)` and selector `0x03fd3492`.
    ```solidity
    function getQuorumBitmapHistoryLength(bytes32 operatorId) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapHistoryLengthCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapHistoryLength(bytes32)`](getQuorumBitmapHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapHistoryLengthReturn {
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
            impl ::core::convert::From<getQuorumBitmapHistoryLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapHistoryLengthCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
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
            impl ::core::convert::From<getQuorumBitmapHistoryLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapHistoryLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapHistoryLength(bytes32)";
            const SELECTOR: [u8; 4] = [3u8, 253u8, 52u8, 146u8];
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
    /**Function with signature `getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])` and selector `0xc391425e`.
    ```solidity
    function getQuorumBitmapIndicesAtBlockNumber(uint32 blockNumber, bytes32[] memory operatorIds) external view returns (uint32[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapIndicesAtBlockNumberCall {
        #[allow(missing_docs)]
        pub blockNumber: u32,
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])`](getQuorumBitmapIndicesAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapIndicesAtBlockNumberReturn {
        #[allow(missing_docs)]
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getQuorumBitmapIndicesAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapIndicesAtBlockNumberCall) -> Self {
                    (value.blockNumber, value.operatorIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapIndicesAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumber: tuple.0,
                        operatorIds: tuple.1,
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
            impl ::core::convert::From<getQuorumBitmapIndicesAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapIndicesAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapIndicesAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapIndicesAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapIndicesAtBlockNumberReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])";
            const SELECTOR: [u8; 4] = [195u8, 145u8, 66u8, 94u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIds),
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
    /**Function with signature `getQuorumBitmapUpdateByIndex(bytes32,uint256)` and selector `0x1eb812da`.
    ```solidity
    function getQuorumBitmapUpdateByIndex(bytes32 operatorId, uint256 index) external view returns (ISlashingRegistryCoordinator.QuorumBitmapUpdate memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapUpdateByIndexCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapUpdateByIndex(bytes32,uint256)`](getQuorumBitmapUpdateByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapUpdateByIndexReturn {
        #[allow(missing_docs)]
        pub _0: <ISlashingRegistryCoordinator::QuorumBitmapUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getQuorumBitmapUpdateByIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapUpdateByIndexCall) -> Self {
                    (value.operatorId, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapUpdateByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinator::QuorumBitmapUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinator::QuorumBitmapUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getQuorumBitmapUpdateByIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapUpdateByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapUpdateByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapUpdateByIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapUpdateByIndexReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinator::QuorumBitmapUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapUpdateByIndex(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [30u8, 184u8, 18u8, 218u8];
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
    /**Function with signature `indexRegistry()` and selector `0x9e9923c2`.
    ```solidity
    function indexRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct indexRegistryCall {}
    ///Container type for the return parameters of the [`indexRegistry()`](indexRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct indexRegistryReturn {
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
            impl ::core::convert::From<indexRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: indexRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for indexRegistryCall {
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
            impl ::core::convert::From<indexRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: indexRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for indexRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for indexRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = indexRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "indexRegistry()";
            const SELECTOR: [u8; 4] = [158u8, 153u8, 35u8, 194u8];
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
    /**Function with signature `initialize(address,address,address,uint256,address)` and selector `0x530b97a4`.
    ```solidity
    function initialize(address _initialOwner, address _churnApprover, address _ejector, uint256 _initialPausedStatus, address _accountIdentifier) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _churnApprover: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _ejector: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _accountIdentifier: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address,address,uint256,address)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value._initialOwner,
                        value._churnApprover,
                        value._ejector,
                        value._initialPausedStatus,
                        value._accountIdentifier,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _initialOwner: tuple.0,
                        _churnApprover: tuple.1,
                        _ejector: tuple.2,
                        _initialPausedStatus: tuple.3,
                        _accountIdentifier: tuple.4,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address,address,uint256,address)";
            const SELECTOR: [u8; 4] = [83u8, 11u8, 151u8, 164u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._churnApprover,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._ejector,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._initialPausedStatus,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._accountIdentifier,
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
    /**Function with signature `isChurnApproverSaltUsed(bytes32)` and selector `0x1478851f`.
    ```solidity
    function isChurnApproverSaltUsed(bytes32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChurnApproverSaltUsedCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isChurnApproverSaltUsed(bytes32)`](isChurnApproverSaltUsedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChurnApproverSaltUsedReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<isChurnApproverSaltUsedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isChurnApproverSaltUsedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isChurnApproverSaltUsedCall {
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
            impl ::core::convert::From<isChurnApproverSaltUsedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isChurnApproverSaltUsedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isChurnApproverSaltUsedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isChurnApproverSaltUsedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isChurnApproverSaltUsedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isChurnApproverSaltUsed(bytes32)";
            const SELECTOR: [u8; 4] = [20u8, 120u8, 133u8, 31u8];
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
    /**Function with signature `isM2Quorum(uint8)` and selector `0xa4d7871f`.
    ```solidity
    function isM2Quorum(uint8 quorumNumber) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isM2QuorumCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`isM2Quorum(uint8)`](isM2QuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isM2QuorumReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<isM2QuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: isM2QuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isM2QuorumCall {
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
            impl ::core::convert::From<isM2QuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isM2QuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isM2QuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isM2QuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isM2QuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isM2Quorum(uint8)";
            const SELECTOR: [u8; 4] = [164u8, 215u8, 135u8, 31u8];
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
    /**Function with signature `lastEjectionTimestamp(address)` and selector `0x125e0584`.
    ```solidity
    function lastEjectionTimestamp(address) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastEjectionTimestampCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`lastEjectionTimestamp(address)`](lastEjectionTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastEjectionTimestampReturn {
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
            impl ::core::convert::From<lastEjectionTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastEjectionTimestampCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastEjectionTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<lastEjectionTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lastEjectionTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastEjectionTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastEjectionTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastEjectionTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastEjectionTimestamp(address)";
            const SELECTOR: [u8; 4] = [18u8, 94u8, 5u8, 132u8];
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
    /**Function with signature `m2QuorumsDisabled()` and selector `0xb2d8678d`.
    ```solidity
    function m2QuorumsDisabled() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct m2QuorumsDisabledCall {}
    ///Container type for the return parameters of the [`m2QuorumsDisabled()`](m2QuorumsDisabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct m2QuorumsDisabledReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<m2QuorumsDisabledCall> for UnderlyingRustTuple<'_> {
                fn from(value: m2QuorumsDisabledCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for m2QuorumsDisabledCall {
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
            impl ::core::convert::From<m2QuorumsDisabledReturn> for UnderlyingRustTuple<'_> {
                fn from(value: m2QuorumsDisabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for m2QuorumsDisabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for m2QuorumsDisabledCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = m2QuorumsDisabledReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "m2QuorumsDisabled()";
            const SELECTOR: [u8; 4] = [178u8, 216u8, 103u8, 141u8];
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
    /**Function with signature `numRegistries()` and selector `0xd72d8dd6`.
    ```solidity
    function numRegistries() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numRegistriesCall {}
    ///Container type for the return parameters of the [`numRegistries()`](numRegistriesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numRegistriesReturn {
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
            impl ::core::convert::From<numRegistriesCall> for UnderlyingRustTuple<'_> {
                fn from(value: numRegistriesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numRegistriesCall {
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
            impl ::core::convert::From<numRegistriesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: numRegistriesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numRegistriesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for numRegistriesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = numRegistriesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "numRegistries()";
            const SELECTOR: [u8; 4] = [215u8, 45u8, 141u8, 214u8];
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
    /**Function with signature `operatorSetsEnabled()` and selector `0x81f936d2`.
    ```solidity
    function operatorSetsEnabled() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetsEnabledCall {}
    ///Container type for the return parameters of the [`operatorSetsEnabled()`](operatorSetsEnabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetsEnabledReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<operatorSetsEnabledCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetsEnabledCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorSetsEnabledCall {
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
            impl ::core::convert::From<operatorSetsEnabledReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetsEnabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorSetsEnabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorSetsEnabledCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSetsEnabledReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorSetsEnabled()";
            const SELECTOR: [u8; 4] = [129u8, 249u8, 54u8, 210u8];
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pause(uint256)` and selector `0x136439dd`.
    ```solidity
    function pause(uint256 newPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall {
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`pause(uint256)`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
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
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPausedStatus: tuple.0,
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
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause(uint256)";
            const SELECTOR: [u8; 4] = [19u8, 100u8, 57u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
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
    /**Function with signature `pauseAll()` and selector `0x595c6a67`.
    ```solidity
    function pauseAll() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllCall {}
    ///Container type for the return parameters of the [`pauseAll()`](pauseAllCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllReturn {}
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
            impl ::core::convert::From<pauseAllCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllCall {
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
            impl ::core::convert::From<pauseAllReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseAllCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauseAll()";
            const SELECTOR: [u8; 4] = [89u8, 92u8, 106u8, 103u8];
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
    /**Function with signature `paused(uint8)` and selector `0x5ac86ab7`.
    ```solidity
    function paused(uint8 index) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Call {
        #[allow(missing_docs)]
        pub index: u8,
    }
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<paused_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Call) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
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
            impl ::core::convert::From<paused_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_0Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused(uint8)";
            const SELECTOR: [u8; 4] = [90u8, 200u8, 106u8, 183u8];
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
    /**Function with signature `paused()` and selector `0x5c975abb`.
    ```solidity
    function paused() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Call {}
    ///Container type for the return parameters of the [`paused()`](paused_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Return {
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
            impl ::core::convert::From<paused_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Call {
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
            impl ::core::convert::From<paused_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
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
    /**Function with signature `pauserRegistry()` and selector `0x886f1195`.
    ```solidity
    function pauserRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryCall {}
    ///Container type for the return parameters of the [`pauserRegistry()`](pauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryReturn {
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
            impl ::core::convert::From<pauserRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryCall {
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
            impl ::core::convert::From<pauserRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauserRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauserRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauserRegistry()";
            const SELECTOR: [u8; 4] = [136u8, 111u8, 17u8, 149u8];
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
    /**Function with signature `pubkeyRegistrationMessageHash(address)` and selector `0x3c2a7f4c`.
    ```solidity
    function pubkeyRegistrationMessageHash(address operator) external view returns (BN254.G1Point memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyRegistrationMessageHashCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`pubkeyRegistrationMessageHash(address)`](pubkeyRegistrationMessageHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyRegistrationMessageHashReturn {
        #[allow(missing_docs)]
        pub _0: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<pubkeyRegistrationMessageHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyRegistrationMessageHashCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyRegistrationMessageHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BN254::G1Point,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<BN254::G1Point as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<pubkeyRegistrationMessageHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyRegistrationMessageHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyRegistrationMessageHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pubkeyRegistrationMessageHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyRegistrationMessageHashReturn;
            type ReturnTuple<'a> = (BN254::G1Point,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pubkeyRegistrationMessageHash(address)";
            const SELECTOR: [u8; 4] = [60u8, 42u8, 127u8, 76u8];
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
    /**Function with signature `quorumCount()` and selector `0x9aa1653d`.
    ```solidity
    function quorumCount() external view returns (uint8);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCountCall {}
    ///Container type for the return parameters of the [`quorumCount()`](quorumCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCountReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<quorumCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCountCall {
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
            impl ::core::convert::From<quorumCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorumCount()";
            const SELECTOR: [u8; 4] = [154u8, 161u8, 101u8, 61u8];
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
    /**Function with signature `quorumUpdateBlockNumber(uint8)` and selector `0x249a0c42`.
    ```solidity
    function quorumUpdateBlockNumber(uint8) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumUpdateBlockNumberCall {
        #[allow(missing_docs)]
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`quorumUpdateBlockNumber(uint8)`](quorumUpdateBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumUpdateBlockNumberReturn {
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
            impl ::core::convert::From<quorumUpdateBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumUpdateBlockNumberCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumUpdateBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<quorumUpdateBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumUpdateBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumUpdateBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumUpdateBlockNumberCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumUpdateBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorumUpdateBlockNumber(uint8)";
            const SELECTOR: [u8; 4] = [36u8, 154u8, 12u8, 66u8];
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
    /**Function with signature `registerOperator(bytes,string,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(bytes,bytes32,uint256))` and selector `0xa50857bf`.
    ```solidity
    function registerOperator(bytes memory quorumNumbers, string memory socket, IBLSApkRegistry.PubkeyRegistrationParams memory params, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperator_0Call {
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub socket: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub params:
            <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operatorSignature:
            <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperator(bytes,string,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(bytes,bytes32,uint256))`](registerOperator_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperator_0Return {}
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::String,
                IBLSApkRegistry::PubkeyRegistrationParams,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::String,
                <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerOperator_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperator_0Call) -> Self {
                    (
                        value.quorumNumbers,
                        value.socket,
                        value.params,
                        value.operatorSignature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperator_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumbers: tuple.0,
                        socket: tuple.1,
                        params: tuple.2,
                        operatorSignature: tuple.3,
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
            impl ::core::convert::From<registerOperator_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperator_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperator_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperator_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::String,
                IBLSApkRegistry::PubkeyRegistrationParams,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperator_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(bytes,string,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [165u8, 8u8, 87u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.socket,
                    ),
                    <IBLSApkRegistry::PubkeyRegistrationParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `registerOperator(address,uint32[],bytes)` and selector `0xadcf73f7`.
    ```solidity
    function registerOperator(address operator, uint32[] memory operatorSetIds, bytes memory data) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperator_1Call {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(address,uint32[],bytes)`](registerOperator_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperator_1Return {}
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
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
            impl ::core::convert::From<registerOperator_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperator_1Call) -> Self {
                    (value.operator, value.operatorSetIds, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperator_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSetIds: tuple.1,
                        data: tuple.2,
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
            impl ::core::convert::From<registerOperator_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperator_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperator_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperator_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperator_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(address,uint32[],bytes)";
            const SELECTOR: [u8; 4] = [173u8, 207u8, 115u8, 247u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
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
    /**Function with signature `registerOperatorWithChurn(bytes,string,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint8,address)[],(bytes,bytes32,uint256),(bytes,bytes32,uint256))` and selector `0x9b5d177b`.
    ```solidity
    function registerOperatorWithChurn(bytes memory quorumNumbers, string memory socket, IBLSApkRegistry.PubkeyRegistrationParams memory params, ISlashingRegistryCoordinator.OperatorKickParam[] memory operatorKickParams, ISignatureUtils.SignatureWithSaltAndExpiry memory churnApproverSignature, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithChurnCall {
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub socket: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub params: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operatorKickParams: alloy::sol_types::private::Vec<
            <ISlashingRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub churnApproverSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperatorWithChurn(bytes,string,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint8,address)[],(bytes,bytes32,uint256),(bytes,bytes32,uint256))`](registerOperatorWithChurnCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithChurnReturn {}
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::String,
                IBLSApkRegistry::PubkeyRegistrationParams,
                alloy::sol_types::sol_data::Array<ISlashingRegistryCoordinator::OperatorKickParam>,
                ISignatureUtils::SignatureWithSaltAndExpiry,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::String,
                <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <ISlashingRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
                >,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerOperatorWithChurnCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithChurnCall) -> Self {
                    (
                        value.quorumNumbers,
                        value.socket,
                        value.params,
                        value.operatorKickParams,
                        value.churnApproverSignature,
                        value.operatorSignature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorWithChurnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumbers: tuple.0,
                        socket: tuple.1,
                        params: tuple.2,
                        operatorKickParams: tuple.3,
                        churnApproverSignature: tuple.4,
                        operatorSignature: tuple.5,
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
            impl ::core::convert::From<registerOperatorWithChurnReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithChurnReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorWithChurnReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorWithChurnCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::String,
                IBLSApkRegistry::PubkeyRegistrationParams,
                alloy::sol_types::sol_data::Array<ISlashingRegistryCoordinator::OperatorKickParam>,
                ISignatureUtils::SignatureWithSaltAndExpiry,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorWithChurnReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorWithChurn(bytes,string,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint8,address)[],(bytes,bytes32,uint256),(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [155u8, 93u8, 23u8, 123u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.socket,
                    ),
                    <IBLSApkRegistry::PubkeyRegistrationParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        ISlashingRegistryCoordinator::OperatorKickParam,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorKickParams),
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.churnApproverSignature,
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `registries(uint256)` and selector `0x6347c900`.
    ```solidity
    function registries(uint256) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registriesCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`registries(uint256)`](registriesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registriesReturn {
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
            impl ::core::convert::From<registriesCall> for UnderlyingRustTuple<'_> {
                fn from(value: registriesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registriesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<registriesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registriesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registriesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registriesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registriesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registries(uint256)";
            const SELECTOR: [u8; 4] = [99u8, 71u8, 201u8, 0u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipCall {
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
            impl ::core::convert::From<renounceOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<serviceManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setAccountIdentifier(address)` and selector `0x143e5915`.
    ```solidity
    function setAccountIdentifier(address _accountIdentifier) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAccountIdentifierCall {
        #[allow(missing_docs)]
        pub _accountIdentifier: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setAccountIdentifier(address)`](setAccountIdentifierCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAccountIdentifierReturn {}
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
            impl ::core::convert::From<setAccountIdentifierCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAccountIdentifierCall) -> Self {
                    (value._accountIdentifier,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAccountIdentifierCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _accountIdentifier: tuple.0,
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
            impl ::core::convert::From<setAccountIdentifierReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAccountIdentifierReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAccountIdentifierReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAccountIdentifierCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAccountIdentifierReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAccountIdentifier(address)";
            const SELECTOR: [u8; 4] = [20u8, 62u8, 89u8, 21u8];
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
                        &self._accountIdentifier,
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
    /**Function with signature `setChurnApprover(address)` and selector `0x29d1e0c3`.
    ```solidity
    function setChurnApprover(address _churnApprover) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setChurnApproverCall {
        #[allow(missing_docs)]
        pub _churnApprover: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setChurnApprover(address)`](setChurnApproverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setChurnApproverReturn {}
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
            impl ::core::convert::From<setChurnApproverCall> for UnderlyingRustTuple<'_> {
                fn from(value: setChurnApproverCall) -> Self {
                    (value._churnApprover,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setChurnApproverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _churnApprover: tuple.0,
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
            impl ::core::convert::From<setChurnApproverReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setChurnApproverReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setChurnApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setChurnApproverCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setChurnApproverReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setChurnApprover(address)";
            const SELECTOR: [u8; 4] = [41u8, 209u8, 224u8, 195u8];
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
                        &self._churnApprover,
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
    /**Function with signature `setEjectionCooldown(uint256)` and selector `0x0d3f2134`.
    ```solidity
    function setEjectionCooldown(uint256 _ejectionCooldown) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectionCooldownCall {
        #[allow(missing_docs)]
        pub _ejectionCooldown: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setEjectionCooldown(uint256)`](setEjectionCooldownCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectionCooldownReturn {}
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
            impl ::core::convert::From<setEjectionCooldownCall> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectionCooldownCall) -> Self {
                    (value._ejectionCooldown,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectionCooldownCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ejectionCooldown: tuple.0,
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
            impl ::core::convert::From<setEjectionCooldownReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectionCooldownReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectionCooldownReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEjectionCooldownCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEjectionCooldownReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setEjectionCooldown(uint256)";
            const SELECTOR: [u8; 4] = [13u8, 63u8, 33u8, 52u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._ejectionCooldown,
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
    /**Function with signature `setEjector(address)` and selector `0x2cdd1e86`.
    ```solidity
    function setEjector(address _ejector) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectorCall {
        #[allow(missing_docs)]
        pub _ejector: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setEjector(address)`](setEjectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectorReturn {}
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
            impl ::core::convert::From<setEjectorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectorCall) -> Self {
                    (value._ejector,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _ejector: tuple.0 }
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
            impl ::core::convert::From<setEjectorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEjectorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEjectorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setEjector(address)";
            const SELECTOR: [u8; 4] = [44u8, 221u8, 30u8, 134u8];
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
                        &self._ejector,
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
    /**Function with signature `setOperatorSetParams(uint8,(uint32,uint16,uint16))` and selector `0x5b0b829f`.
    ```solidity
    function setOperatorSetParams(uint8 quorumNumber, ISlashingRegistryCoordinator.OperatorSetParam memory operatorSetParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorSetParamsCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operatorSetParams:
            <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setOperatorSetParams(uint8,(uint32,uint16,uint16))`](setOperatorSetParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorSetParamsReturn {}
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
                ISlashingRegistryCoordinator::OperatorSetParam,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<setOperatorSetParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorSetParamsCall) -> Self {
                    (value.quorumNumber, value.operatorSetParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorSetParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operatorSetParams: tuple.1,
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
            impl ::core::convert::From<setOperatorSetParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorSetParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorSetParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorSetParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                ISlashingRegistryCoordinator::OperatorSetParam,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorSetParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOperatorSetParams(uint8,(uint32,uint16,uint16))";
            const SELECTOR: [u8; 4] = [91u8, 11u8, 130u8, 159u8];
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
                    <ISlashingRegistryCoordinator::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `unpause(uint256)` and selector `0xfabc1cbc`.
    ```solidity
    function unpause(uint256 newPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`unpause(uint256)`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
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
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPausedStatus: tuple.0,
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
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause(uint256)";
            const SELECTOR: [u8; 4] = [250u8, 188u8, 28u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
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
    /**Function with signature `updateOperators(address[])` and selector `0x00cf2ab5`.
    ```solidity
    function updateOperators(address[] memory operators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsCall {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`updateOperators(address[])`](updateOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsReturn {}
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
            impl ::core::convert::From<updateOperatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsCall) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
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
            impl ::core::convert::From<updateOperatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperators(address[])";
            const SELECTOR: [u8; 4] = [0u8, 207u8, 42u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.operators
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
    /**Function with signature `updateOperatorsForQuorum(address[][],bytes)` and selector `0x5140a548`.
    ```solidity
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumCall {
        #[allow(missing_docs)]
        pub operatorsPerQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorsForQuorum(address[][],bytes)`](updateOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumReturn {}
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
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
            impl ::core::convert::From<updateOperatorsForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumCall) -> Self {
                    (value.operatorsPerQuorum, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorsPerQuorum: tuple.0,
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
            impl ::core::convert::From<updateOperatorsForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorsForQuorum(address[][],bytes)";
            const SELECTOR: [u8; 4] = [81u8, 64u8, 165u8, 72u8];
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
                        alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.operatorsPerQuorum
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
    /**Function with signature `updateSocket(string)` and selector `0x0cf4b767`.
    ```solidity
    function updateSocket(string memory socket) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSocketCall {
        #[allow(missing_docs)]
        pub socket: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateSocket(string)`](updateSocketCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSocketReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateSocketCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateSocketCall) -> Self {
                    (value.socket,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateSocketCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { socket: tuple.0 }
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
            impl ::core::convert::From<updateSocketReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateSocketReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateSocketReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateSocketCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateSocketReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateSocket(string)";
            const SELECTOR: [u8; 4] = [12u8, 244u8, 183u8, 103u8];
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
                        &self.socket,
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
    ///Container for all the [`RegistryCoordinator`](self) function calls.
    pub enum RegistryCoordinatorCalls {
        #[allow(missing_docs)]
        OPERATOR_CHURN_APPROVAL_TYPEHASH(OPERATOR_CHURN_APPROVAL_TYPEHASHCall),
        #[allow(missing_docs)]
        PUBKEY_REGISTRATION_TYPEHASH(PUBKEY_REGISTRATION_TYPEHASHCall),
        #[allow(missing_docs)]
        accountIdentifier(accountIdentifierCall),
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        blsApkRegistry(blsApkRegistryCall),
        #[allow(missing_docs)]
        calculateOperatorChurnApprovalDigestHash(calculateOperatorChurnApprovalDigestHashCall),
        #[allow(missing_docs)]
        churnApprover(churnApproverCall),
        #[allow(missing_docs)]
        createSlashableStakeQuorum(createSlashableStakeQuorumCall),
        #[allow(missing_docs)]
        createTotalDelegatedStakeQuorum(createTotalDelegatedStakeQuorumCall),
        #[allow(missing_docs)]
        deregisterOperator_0(deregisterOperator_0Call),
        #[allow(missing_docs)]
        deregisterOperator_1(deregisterOperator_1Call),
        #[allow(missing_docs)]
        disableM2QuorumRegistration(disableM2QuorumRegistrationCall),
        #[allow(missing_docs)]
        ejectOperator(ejectOperatorCall),
        #[allow(missing_docs)]
        ejectionCooldown(ejectionCooldownCall),
        #[allow(missing_docs)]
        ejector(ejectorCall),
        #[allow(missing_docs)]
        enableOperatorSets(enableOperatorSetsCall),
        #[allow(missing_docs)]
        getCurrentQuorumBitmap(getCurrentQuorumBitmapCall),
        #[allow(missing_docs)]
        getOperator(getOperatorCall),
        #[allow(missing_docs)]
        getOperatorFromId(getOperatorFromIdCall),
        #[allow(missing_docs)]
        getOperatorId(getOperatorIdCall),
        #[allow(missing_docs)]
        getOperatorSetParams(getOperatorSetParamsCall),
        #[allow(missing_docs)]
        getOperatorStatus(getOperatorStatusCall),
        #[allow(missing_docs)]
        getQuorumBitmapAtBlockNumberByIndex(getQuorumBitmapAtBlockNumberByIndexCall),
        #[allow(missing_docs)]
        getQuorumBitmapHistoryLength(getQuorumBitmapHistoryLengthCall),
        #[allow(missing_docs)]
        getQuorumBitmapIndicesAtBlockNumber(getQuorumBitmapIndicesAtBlockNumberCall),
        #[allow(missing_docs)]
        getQuorumBitmapUpdateByIndex(getQuorumBitmapUpdateByIndexCall),
        #[allow(missing_docs)]
        indexRegistry(indexRegistryCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isChurnApproverSaltUsed(isChurnApproverSaltUsedCall),
        #[allow(missing_docs)]
        isM2Quorum(isM2QuorumCall),
        #[allow(missing_docs)]
        lastEjectionTimestamp(lastEjectionTimestampCall),
        #[allow(missing_docs)]
        m2QuorumsDisabled(m2QuorumsDisabledCall),
        #[allow(missing_docs)]
        numRegistries(numRegistriesCall),
        #[allow(missing_docs)]
        operatorSetsEnabled(operatorSetsEnabledCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        pauseAll(pauseAllCall),
        #[allow(missing_docs)]
        paused_0(paused_0Call),
        #[allow(missing_docs)]
        paused_1(paused_1Call),
        #[allow(missing_docs)]
        pauserRegistry(pauserRegistryCall),
        #[allow(missing_docs)]
        pubkeyRegistrationMessageHash(pubkeyRegistrationMessageHashCall),
        #[allow(missing_docs)]
        quorumCount(quorumCountCall),
        #[allow(missing_docs)]
        quorumUpdateBlockNumber(quorumUpdateBlockNumberCall),
        #[allow(missing_docs)]
        registerOperator_0(registerOperator_0Call),
        #[allow(missing_docs)]
        registerOperator_1(registerOperator_1Call),
        #[allow(missing_docs)]
        registerOperatorWithChurn(registerOperatorWithChurnCall),
        #[allow(missing_docs)]
        registries(registriesCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        serviceManager(serviceManagerCall),
        #[allow(missing_docs)]
        setAccountIdentifier(setAccountIdentifierCall),
        #[allow(missing_docs)]
        setChurnApprover(setChurnApproverCall),
        #[allow(missing_docs)]
        setEjectionCooldown(setEjectionCooldownCall),
        #[allow(missing_docs)]
        setEjector(setEjectorCall),
        #[allow(missing_docs)]
        setOperatorSetParams(setOperatorSetParamsCall),
        #[allow(missing_docs)]
        stakeRegistry(stakeRegistryCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        updateOperators(updateOperatorsCall),
        #[allow(missing_docs)]
        updateOperatorsForQuorum(updateOperatorsForQuorumCall),
        #[allow(missing_docs)]
        updateSocket(updateSocketCall),
    }
    #[automatically_derived]
    impl RegistryCoordinatorCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 207u8, 42u8, 181u8],
            [3u8, 253u8, 52u8, 146u8],
            [4u8, 236u8, 99u8, 81u8],
            [5u8, 67u8, 16u8, 230u8],
            [7u8, 100u8, 203u8, 147u8],
            [12u8, 244u8, 183u8, 103u8],
            [13u8, 63u8, 33u8, 52u8],
            [18u8, 94u8, 5u8, 132u8],
            [19u8, 84u8, 42u8, 78u8],
            [19u8, 100u8, 57u8, 221u8],
            [20u8, 62u8, 89u8, 21u8],
            [20u8, 120u8, 133u8, 31u8],
            [30u8, 184u8, 18u8, 218u8],
            [36u8, 154u8, 12u8, 66u8],
            [40u8, 246u8, 27u8, 49u8],
            [41u8, 107u8, 176u8, 100u8],
            [41u8, 209u8, 224u8, 195u8],
            [44u8, 221u8, 30u8, 134u8],
            [57u8, 152u8, 253u8, 211u8],
            [60u8, 42u8, 127u8, 76u8],
            [62u8, 239u8, 58u8, 81u8],
            [81u8, 64u8, 165u8, 72u8],
            [83u8, 11u8, 151u8, 164u8],
            [88u8, 101u8, 198u8, 12u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [91u8, 11u8, 130u8, 159u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 244u8, 89u8, 70u8],
            [99u8, 71u8, 201u8, 0u8],
            [104u8, 48u8, 72u8, 53u8],
            [110u8, 59u8, 23u8, 219u8],
            [113u8, 80u8, 24u8, 166u8],
            [115u8, 59u8, 117u8, 7u8],
            [129u8, 249u8, 54u8, 210u8],
            [130u8, 129u8, 171u8, 117u8],
            [132u8, 202u8, 82u8, 19u8],
            [135u8, 30u8, 240u8, 73u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [154u8, 161u8, 101u8, 61u8],
            [155u8, 93u8, 23u8, 123u8],
            [157u8, 142u8, 12u8, 35u8],
            [158u8, 153u8, 35u8, 194u8],
            [159u8, 234u8, 184u8, 89u8],
            [164u8, 215u8, 135u8, 31u8],
            [165u8, 8u8, 87u8, 191u8],
            [169u8, 111u8, 120u8, 62u8],
            [173u8, 207u8, 115u8, 247u8],
            [178u8, 216u8, 103u8, 141u8],
            [195u8, 145u8, 66u8, 94u8],
            [202u8, 13u8, 232u8, 130u8],
            [202u8, 79u8, 45u8, 151u8],
            [202u8, 138u8, 167u8, 199u8],
            [215u8, 45u8, 141u8, 214u8],
            [230u8, 87u8, 151u8, 173u8],
            [238u8, 49u8, 136u8, 33u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 188u8, 28u8, 188u8],
            [253u8, 57u8, 16u8, 90u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RegistryCoordinatorCalls {
        const NAME: &'static str = "RegistryCoordinatorCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 60usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(_) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(_) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::accountIdentifier(_) => {
                    <accountIdentifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorChurnApprovalDigestHash(_) => {
                    <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::churnApprover(_) => {
                    <churnApproverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createSlashableStakeQuorum(_) => {
                    <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createTotalDelegatedStakeQuorum(_) => {
                    <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperator_0(_) => {
                    <deregisterOperator_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperator_1(_) => {
                    <deregisterOperator_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disableM2QuorumRegistration(_) => {
                    <disableM2QuorumRegistrationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ejectOperator(_) => {
                    <ejectOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ejectionCooldown(_) => {
                    <ejectionCooldownCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ejector(_) => <ejectorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::enableOperatorSets(_) => {
                    <enableOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentQuorumBitmap(_) => {
                    <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperator(_) => {
                    <getOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorFromId(_) => {
                    <getOperatorFromIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorId(_) => {
                    <getOperatorIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSetParams(_) => {
                    <getOperatorSetParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorStatus(_) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapAtBlockNumberByIndex(_) => {
                    <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapHistoryLength(_) => {
                    <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapIndicesAtBlockNumber(_) => {
                    <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapUpdateByIndex(_) => {
                    <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::indexRegistry(_) => {
                    <indexRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isChurnApproverSaltUsed(_) => {
                    <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isM2Quorum(_) => {
                    <isM2QuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastEjectionTimestamp(_) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::m2QuorumsDisabled(_) => {
                    <m2QuorumsDisabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::numRegistries(_) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorSetsEnabled(_) => {
                    <operatorSetsEnabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pubkeyRegistrationMessageHash(_) => {
                    <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::quorumCount(_) => {
                    <quorumCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::quorumUpdateBlockNumber(_) => {
                    <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator_0(_) => {
                    <registerOperator_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator_1(_) => {
                    <registerOperator_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperatorWithChurn(_) => {
                    <registerOperatorWithChurnCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registries(_) => {
                    <registriesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::serviceManager(_) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAccountIdentifier(_) => {
                    <setAccountIdentifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setChurnApprover(_) => {
                    <setChurnApproverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setEjectionCooldown(_) => {
                    <setEjectionCooldownCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setEjector(_) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorSetParams(_) => {
                    <setOperatorSetParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateOperators(_) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorsForQuorum(_) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateSocket(_) => {
                    <updateSocketCall as alloy_sol_types::SolCall>::SELECTOR
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
                -> alloy_sol_types::Result<RegistryCoordinatorCalls>] = &[
                {
                    fn updateOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <updateOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::updateOperators)
                    }
                    updateOperators
                },
                {
                    fn getQuorumBitmapHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RegistryCoordinatorCalls::getQuorumBitmapHistoryLength)
                    }
                    getQuorumBitmapHistoryLength
                },
                {
                    fn getQuorumBitmapAtBlockNumberByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RegistryCoordinatorCalls::getQuorumBitmapAtBlockNumberByIndex,
                            )
                    }
                    getQuorumBitmapAtBlockNumberByIndex
                },
                {
                    fn churnApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <churnApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::churnApprover)
                    }
                    churnApprover
                },
                {
                    fn accountIdentifier(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <accountIdentifierCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::accountIdentifier)
                    }
                    accountIdentifier
                },
                {
                    fn updateSocket(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <updateSocketCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::updateSocket)
                    }
                    updateSocket
                },
                {
                    fn setEjectionCooldown(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <setEjectionCooldownCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::setEjectionCooldown)
                    }
                    setEjectionCooldown
                },
                {
                    fn lastEjectionTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::lastEjectionTimestamp)
                    }
                    lastEjectionTimestamp
                },
                {
                    fn getOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <getOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::getOperatorId)
                    }
                    getOperatorId
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::pause)
                    }
                    pause
                },
                {
                    fn setAccountIdentifier(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <setAccountIdentifierCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::setAccountIdentifier)
                    }
                    setAccountIdentifier
                },
                {
                    fn isChurnApproverSaltUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::isChurnApproverSaltUsed)
                    }
                    isChurnApproverSaltUsed
                },
                {
                    fn getQuorumBitmapUpdateByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RegistryCoordinatorCalls::getQuorumBitmapUpdateByIndex)
                    }
                    getQuorumBitmapUpdateByIndex
                },
                {
                    fn quorumUpdateBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::quorumUpdateBlockNumber)
                    }
                    quorumUpdateBlockNumber
                },
                {
                    fn ejector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <ejectorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::ejector)
                    }
                    ejector
                },
                {
                    fn getOperatorFromId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <getOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::getOperatorFromId)
                    }
                    getOperatorFromId
                },
                {
                    fn setChurnApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <setChurnApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::setChurnApprover)
                    }
                    setChurnApprover
                },
                {
                    fn setEjector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <setEjectorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::setEjector)
                    }
                    setEjector
                },
                {
                    fn serviceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn pubkeyRegistrationMessageHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RegistryCoordinatorCalls::pubkeyRegistrationMessageHash)
                    }
                    pubkeyRegistrationMessageHash
                },
                {
                    fn createSlashableStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RegistryCoordinatorCalls::createSlashableStakeQuorum)
                    }
                    createSlashableStakeQuorum
                },
                {
                    fn updateOperatorsForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::updateOperatorsForQuorum)
                    }
                    updateOperatorsForQuorum
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <getOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::getOperator)
                    }
                    getOperator
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn setOperatorSetParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <setOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::setOperatorSetParams)
                    }
                    setOperatorSetParams
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn registries(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <registriesCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::registries)
                    }
                    registries
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn ejectOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <ejectOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::ejectOperator)
                    }
                    ejectOperator
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn disableM2QuorumRegistration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <disableM2QuorumRegistrationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RegistryCoordinatorCalls::disableM2QuorumRegistration)
                    }
                    disableM2QuorumRegistration
                },
                {
                    fn operatorSetsEnabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <operatorSetsEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::operatorSetsEnabled)
                    }
                    operatorSetsEnabled
                },
                {
                    fn createTotalDelegatedStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RegistryCoordinatorCalls::createTotalDelegatedStakeQuorum,
                            )
                    }
                    createTotalDelegatedStakeQuorum
                },
                {
                    fn calculateOperatorChurnApprovalDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RegistryCoordinatorCalls::calculateOperatorChurnApprovalDigestHash,
                            )
                    }
                    calculateOperatorChurnApprovalDigestHash
                },
                {
                    fn getCurrentQuorumBitmap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::getCurrentQuorumBitmap)
                    }
                    getCurrentQuorumBitmap
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::owner)
                    }
                    owner
                },
                {
                    fn quorumCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <quorumCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::quorumCount)
                    }
                    quorumCount
                },
                {
                    fn registerOperatorWithChurn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <registerOperatorWithChurnCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::registerOperatorWithChurn)
                    }
                    registerOperatorWithChurn
                },
                {
                    fn deregisterOperator_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <deregisterOperator_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::deregisterOperator_0)
                    }
                    deregisterOperator_0
                },
                {
                    fn indexRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <indexRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::indexRegistry)
                    }
                    indexRegistry
                },
                {
                    fn PUBKEY_REGISTRATION_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RegistryCoordinatorCalls::PUBKEY_REGISTRATION_TYPEHASH)
                    }
                    PUBKEY_REGISTRATION_TYPEHASH
                },
                {
                    fn isM2Quorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <isM2QuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::isM2Quorum)
                    }
                    isM2Quorum
                },
                {
                    fn registerOperator_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <registerOperator_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::registerOperator_0)
                    }
                    registerOperator_0
                },
                {
                    fn ejectionCooldown(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <ejectionCooldownCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::ejectionCooldown)
                    }
                    ejectionCooldown
                },
                {
                    fn registerOperator_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <registerOperator_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::registerOperator_1)
                    }
                    registerOperator_1
                },
                {
                    fn m2QuorumsDisabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <m2QuorumsDisabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::m2QuorumsDisabled)
                    }
                    m2QuorumsDisabled
                },
                {
                    fn getQuorumBitmapIndicesAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RegistryCoordinatorCalls::getQuorumBitmapIndicesAtBlockNumber,
                            )
                    }
                    getQuorumBitmapIndicesAtBlockNumber
                },
                {
                    fn OPERATOR_CHURN_APPROVAL_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RegistryCoordinatorCalls::OPERATOR_CHURN_APPROVAL_TYPEHASH,
                            )
                    }
                    OPERATOR_CHURN_APPROVAL_TYPEHASH
                },
                {
                    fn deregisterOperator_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <deregisterOperator_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::deregisterOperator_1)
                    }
                    deregisterOperator_1
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn numRegistries(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <numRegistriesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::numRegistries)
                    }
                    numRegistries
                },
                {
                    fn getOperatorSetParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <getOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::getOperatorSetParams)
                    }
                    getOperatorSetParams
                },
                {
                    fn enableOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <enableOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::enableOperatorSets)
                    }
                    enableOperatorSets
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorCalls::unpause)
                    }
                    unpause
                },
                {
                    fn getOperatorStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::getOperatorStatus)
                    }
                    getOperatorStatus
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
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(inner) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(inner) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::accountIdentifier(inner) => {
                    <accountIdentifierCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateOperatorChurnApprovalDigestHash(inner) => {
                    <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::churnApprover(inner) => {
                    <churnApproverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createSlashableStakeQuorum(inner) => {
                    <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createTotalDelegatedStakeQuorum(inner) => {
                    <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperator_0(inner) => {
                    <deregisterOperator_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperator_1(inner) => {
                    <deregisterOperator_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disableM2QuorumRegistration(inner) => {
                    <disableM2QuorumRegistrationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ejectOperator(inner) => {
                    <ejectOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ejectionCooldown(inner) => {
                    <ejectionCooldownCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ejector(inner) => {
                    <ejectorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::enableOperatorSets(inner) => {
                    <enableOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentQuorumBitmap(inner) => {
                    <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperator(inner) => {
                    <getOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorFromId(inner) => {
                    <getOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorId(inner) => {
                    <getOperatorIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSetParams(inner) => {
                    <getOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorStatus(inner) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapAtBlockNumberByIndex(inner) => {
                    <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapHistoryLength(inner) => {
                    <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapIndicesAtBlockNumber(inner) => {
                    <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapUpdateByIndex(inner) => {
                    <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::indexRegistry(inner) => {
                    <indexRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isChurnApproverSaltUsed(inner) => {
                    <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isM2Quorum(inner) => {
                    <isM2QuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lastEjectionTimestamp(inner) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::m2QuorumsDisabled(inner) => {
                    <m2QuorumsDisabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::numRegistries(inner) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorSetsEnabled(inner) => {
                    <operatorSetsEnabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pubkeyRegistrationMessageHash(inner) => {
                    <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::quorumCount(inner) => {
                    <quorumCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::quorumUpdateBlockNumber(inner) => {
                    <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperator_0(inner) => {
                    <registerOperator_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperator_1(inner) => {
                    <registerOperator_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperatorWithChurn(inner) => {
                    <registerOperatorWithChurnCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registries(inner) => {
                    <registriesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAccountIdentifier(inner) => {
                    <setAccountIdentifierCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setChurnApprover(inner) => {
                    <setChurnApproverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setEjectionCooldown(inner) => {
                    <setEjectionCooldownCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setEjector(inner) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setOperatorSetParams(inner) => {
                    <setOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateSocket(inner) => {
                    <updateSocketCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(inner) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(inner) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::accountIdentifier(inner) => {
                    <accountIdentifierCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateOperatorChurnApprovalDigestHash(inner) => {
                    <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::churnApprover(inner) => {
                    <churnApproverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createSlashableStakeQuorum(inner) => {
                    <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createTotalDelegatedStakeQuorum(inner) => {
                    <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperator_0(inner) => {
                    <deregisterOperator_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperator_1(inner) => {
                    <deregisterOperator_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disableM2QuorumRegistration(inner) => {
                    <disableM2QuorumRegistrationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ejectOperator(inner) => {
                    <ejectOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ejectionCooldown(inner) => {
                    <ejectionCooldownCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ejector(inner) => {
                    <ejectorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::enableOperatorSets(inner) => {
                    <enableOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentQuorumBitmap(inner) => {
                    <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperator(inner) => {
                    <getOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorFromId(inner) => {
                    <getOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorId(inner) => {
                    <getOperatorIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSetParams(inner) => {
                    <getOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorStatus(inner) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapAtBlockNumberByIndex(inner) => {
                    <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapHistoryLength(inner) => {
                    <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapIndicesAtBlockNumber(inner) => {
                    <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapUpdateByIndex(inner) => {
                    <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::indexRegistry(inner) => {
                    <indexRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isChurnApproverSaltUsed(inner) => {
                    <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isM2Quorum(inner) => {
                    <isM2QuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastEjectionTimestamp(inner) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::m2QuorumsDisabled(inner) => {
                    <m2QuorumsDisabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::numRegistries(inner) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorSetsEnabled(inner) => {
                    <operatorSetsEnabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pubkeyRegistrationMessageHash(inner) => {
                    <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::quorumCount(inner) => {
                    <quorumCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::quorumUpdateBlockNumber(inner) => {
                    <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperator_0(inner) => {
                    <registerOperator_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperator_1(inner) => {
                    <registerOperator_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperatorWithChurn(inner) => {
                    <registerOperatorWithChurnCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registries(inner) => {
                    <registriesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAccountIdentifier(inner) => {
                    <setAccountIdentifierCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setChurnApprover(inner) => {
                    <setChurnApproverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setEjectionCooldown(inner) => {
                    <setEjectionCooldownCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setEjector(inner) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperatorSetParams(inner) => {
                    <setOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateSocket(inner) => {
                    <updateSocketCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`RegistryCoordinator`](self) custom errors.
    pub enum RegistryCoordinatorErrors {
        #[allow(missing_docs)]
        AlreadyRegisteredForQuorums(AlreadyRegisteredForQuorums),
        #[allow(missing_docs)]
        BitmapCannotBeZero(BitmapCannotBeZero),
        #[allow(missing_docs)]
        BitmapEmpty(BitmapEmpty),
        #[allow(missing_docs)]
        BitmapUpdateIsAfterBlockNumber(BitmapUpdateIsAfterBlockNumber),
        #[allow(missing_docs)]
        BitmapValueTooLarge(BitmapValueTooLarge),
        #[allow(missing_docs)]
        BytesArrayLengthTooLong(BytesArrayLengthTooLong),
        #[allow(missing_docs)]
        BytesArrayNotOrdered(BytesArrayNotOrdered),
        #[allow(missing_docs)]
        CannotChurnSelf(CannotChurnSelf),
        #[allow(missing_docs)]
        CannotKickOperatorAboveThreshold(CannotKickOperatorAboveThreshold),
        #[allow(missing_docs)]
        CannotReregisterYet(CannotReregisterYet),
        #[allow(missing_docs)]
        ChurnApproverSaltUsed(ChurnApproverSaltUsed),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        ExpModFailed(ExpModFailed),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InputLengthMismatch(InputLengthMismatch),
        #[allow(missing_docs)]
        InsufficientStakeForChurn(InsufficientStakeForChurn),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidRegistrationType(InvalidRegistrationType),
        #[allow(missing_docs)]
        InvalidSignature(InvalidSignature),
        #[allow(missing_docs)]
        M2QuorumsAlreadyDisabled(M2QuorumsAlreadyDisabled),
        #[allow(missing_docs)]
        MaxQuorumsReached(MaxQuorumsReached),
        #[allow(missing_docs)]
        NextBitmapUpdateIsBeforeBlockNumber(NextBitmapUpdateIsBeforeBlockNumber),
        #[allow(missing_docs)]
        NotRegistered(NotRegistered),
        #[allow(missing_docs)]
        NotRegisteredForQuorum(NotRegisteredForQuorum),
        #[allow(missing_docs)]
        NotSorted(NotSorted),
        #[allow(missing_docs)]
        OnlyAllocationManager(OnlyAllocationManager),
        #[allow(missing_docs)]
        OnlyEjector(OnlyEjector),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        OperatorSetsAlreadyEnabled(OperatorSetsAlreadyEnabled),
        #[allow(missing_docs)]
        OperatorSetsNotEnabled(OperatorSetsNotEnabled),
        #[allow(missing_docs)]
        QuorumDoesNotExist(QuorumDoesNotExist),
        #[allow(missing_docs)]
        QuorumOperatorCountMismatch(QuorumOperatorCountMismatch),
        #[allow(missing_docs)]
        RegistryCoordinatorSignatureExpired(RegistryCoordinatorSignatureExpired),
        #[allow(missing_docs)]
        SaltAlreadyUsed(SaltAlreadyUsed),
        #[allow(missing_docs)]
        SignatureExpired(SignatureExpired),
    }
    #[automatically_derived]
    impl RegistryCoordinatorErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 25u8, 189u8, 205u8],
            [12u8, 104u8, 22u8, 205u8],
            [12u8, 237u8, 48u8, 67u8],
            [19u8, 202u8, 70u8, 87u8],
            [35u8, 216u8, 113u8, 165u8],
            [50u8, 208u8, 206u8, 250u8],
            [53u8, 75u8, 184u8, 171u8],
            [60u8, 184u8, 156u8, 151u8],
            [76u8, 68u8, 153u8, 93u8],
            [91u8, 119u8, 144u8, 25u8],
            [108u8, 177u8, 154u8, 255u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [128u8, 200u8, 131u8, 72u8],
            [132u8, 10u8, 72u8, 213u8],
            [139u8, 170u8, 87u8, 159u8],
            [142u8, 90u8, 238u8, 231u8],
            [154u8, 21u8, 9u8, 141u8],
            [170u8, 173u8, 19u8, 247u8],
            [171u8, 164u8, 115u8, 57u8],
            [172u8, 45u8, 22u8, 130u8],
            [177u8, 135u8, 232u8, 105u8],
            [178u8, 225u8, 142u8, 5u8],
            [186u8, 80u8, 249u8, 17u8],
            [187u8, 186u8, 96u8, 203u8],
            [198u8, 29u8, 202u8, 93u8],
            [202u8, 149u8, 115u8, 51u8],
            [208u8, 83u8, 170u8, 33u8],
            [209u8, 109u8, 80u8, 234u8],
            [213u8, 30u8, 218u8, 227u8],
            [223u8, 125u8, 253u8, 134u8],
            [230u8, 33u8, 159u8, 234u8],
            [235u8, 177u8, 0u8, 248u8],
            [237u8, 177u8, 86u8, 46u8],
            [251u8, 74u8, 156u8, 142u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RegistryCoordinatorErrors {
        const NAME: &'static str = "RegistryCoordinatorErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 36usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyRegisteredForQuorums(_) => {
                    <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BitmapCannotBeZero(_) => {
                    <BitmapCannotBeZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BitmapEmpty(_) => <BitmapEmpty as alloy_sol_types::SolError>::SELECTOR,
                Self::BitmapUpdateIsAfterBlockNumber(_) => {
                    <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BitmapValueTooLarge(_) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayLengthTooLong(_) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayNotOrdered(_) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotChurnSelf(_) => {
                    <CannotChurnSelf as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotKickOperatorAboveThreshold(_) => {
                    <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotReregisterYet(_) => {
                    <CannotReregisterYet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ChurnApproverSaltUsed(_) => {
                    <ChurnApproverSaltUsed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpModFailed(_) => <ExpModFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputLengthMismatch(_) => {
                    <InputLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientStakeForChurn(_) => {
                    <InsufficientStakeForChurn as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRegistrationType(_) => {
                    <InvalidRegistrationType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::M2QuorumsAlreadyDisabled(_) => {
                    <M2QuorumsAlreadyDisabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxQuorumsReached(_) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NextBitmapUpdateIsBeforeBlockNumber(_) => {
                    <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotRegistered(_) => <NotRegistered as alloy_sol_types::SolError>::SELECTOR,
                Self::NotRegisteredForQuorum(_) => {
                    <NotRegisteredForQuorum as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSorted(_) => <NotSorted as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyAllocationManager(_) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEjector(_) => <OnlyEjector as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorSetsAlreadyEnabled(_) => {
                    <OperatorSetsAlreadyEnabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorSetsNotEnabled(_) => {
                    <OperatorSetsNotEnabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumDoesNotExist(_) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumOperatorCountMismatch(_) => {
                    <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RegistryCoordinatorSignatureExpired(_) => {
                    <RegistryCoordinatorSignatureExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SaltAlreadyUsed(_) => {
                    <SaltAlreadyUsed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
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
                -> alloy_sol_types::Result<RegistryCoordinatorErrors>] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn AlreadyRegisteredForQuorums(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::AlreadyRegisteredForQuorums)
                    }
                    AlreadyRegisteredForQuorums
                },
                {
                    fn SaltAlreadyUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <SaltAlreadyUsed as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::SaltAlreadyUsed)
                    }
                    SaltAlreadyUsed
                },
                {
                    fn BitmapEmpty(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <BitmapEmpty as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorErrors::BitmapEmpty)
                    }
                    BitmapEmpty
                },
                {
                    fn OnlyAllocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <OnlyAllocationManager as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::OnlyAllocationManager)
                    }
                    OnlyAllocationManager
                },
                {
                    fn CannotReregisterYet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <CannotReregisterYet as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::CannotReregisterYet)
                    }
                    CannotReregisterYet
                },
                {
                    fn InvalidRegistrationType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <InvalidRegistrationType as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::InvalidRegistrationType)
                    }
                    InvalidRegistrationType
                },
                {
                    fn MaxQuorumsReached(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <MaxQuorumsReached as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::MaxQuorumsReached)
                    }
                    MaxQuorumsReached
                },
                {
                    fn InsufficientStakeForChurn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <InsufficientStakeForChurn as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::InsufficientStakeForChurn)
                    }
                    InsufficientStakeForChurn
                },
                {
                    fn OperatorSetsNotEnabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <OperatorSetsNotEnabled as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::OperatorSetsNotEnabled)
                    }
                    OperatorSetsNotEnabled
                },
                {
                    fn BitmapUpdateIsAfterBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RegistryCoordinatorErrors::BitmapUpdateIsAfterBlockNumber,
                            )
                    }
                    BitmapUpdateIsAfterBlockNumber
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::BytesArrayNotOrdered)
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn QuorumOperatorCountMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::QuorumOperatorCountMismatch)
                    }
                    QuorumOperatorCountMismatch
                },
                {
                    fn RegistryCoordinatorSignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <RegistryCoordinatorSignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RegistryCoordinatorErrors::RegistryCoordinatorSignatureExpired,
                            )
                    }
                    RegistryCoordinatorSignatureExpired
                },
                {
                    fn InputLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <InputLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::InputLengthMismatch)
                    }
                    InputLengthMismatch
                },
                {
                    fn NotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <NotRegistered as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorErrors::NotRegistered)
                    }
                    NotRegistered
                },
                {
                    fn CannotChurnSelf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <CannotChurnSelf as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::CannotChurnSelf)
                    }
                    CannotChurnSelf
                },
                {
                    fn CannotKickOperatorAboveThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RegistryCoordinatorErrors::CannotKickOperatorAboveThreshold,
                            )
                    }
                    CannotKickOperatorAboveThreshold
                },
                {
                    fn OperatorSetsAlreadyEnabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <OperatorSetsAlreadyEnabled as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::OperatorSetsAlreadyEnabled)
                    }
                    OperatorSetsAlreadyEnabled
                },
                {
                    fn NotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn NextBitmapUpdateIsBeforeBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RegistryCoordinatorErrors::NextBitmapUpdateIsBeforeBlockNumber,
                            )
                    }
                    NextBitmapUpdateIsBeforeBlockNumber
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn NotRegisteredForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <NotRegisteredForQuorum as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::NotRegisteredForQuorum)
                    }
                    NotRegisteredForQuorum
                },
                {
                    fn BitmapCannotBeZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <BitmapCannotBeZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::BitmapCannotBeZero)
                    }
                    BitmapCannotBeZero
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn ChurnApproverSaltUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <ChurnApproverSaltUsed as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::ChurnApproverSaltUsed)
                    }
                    ChurnApproverSaltUsed
                },
                {
                    fn QuorumDoesNotExist(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::QuorumDoesNotExist)
                    }
                    QuorumDoesNotExist
                },
                {
                    fn M2QuorumsAlreadyDisabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <M2QuorumsAlreadyDisabled as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::M2QuorumsAlreadyDisabled)
                    }
                    M2QuorumsAlreadyDisabled
                },
                {
                    fn OnlyEjector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <OnlyEjector as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RegistryCoordinatorErrors::OnlyEjector)
                    }
                    OnlyEjector
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::BytesArrayLengthTooLong)
                    }
                    BytesArrayLengthTooLong
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
                Self::AlreadyRegisteredForQuorums(inner) => {
                    <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BitmapCannotBeZero(inner) => {
                    <BitmapCannotBeZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BitmapEmpty(inner) => {
                    <BitmapEmpty as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BitmapUpdateIsAfterBlockNumber(inner) => {
                    <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotChurnSelf(inner) => {
                    <CannotChurnSelf as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotKickOperatorAboveThreshold(inner) => {
                    <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotReregisterYet(inner) => {
                    <CannotReregisterYet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ChurnApproverSaltUsed(inner) => {
                    <ChurnApproverSaltUsed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputLengthMismatch(inner) => {
                    <InputLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientStakeForChurn(inner) => {
                    <InsufficientStakeForChurn as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidRegistrationType(inner) => {
                    <InvalidRegistrationType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::M2QuorumsAlreadyDisabled(inner) => {
                    <M2QuorumsAlreadyDisabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxQuorumsReached(inner) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NextBitmapUpdateIsBeforeBlockNumber(inner) => {
                    <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotRegistered(inner) => {
                    <NotRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotRegisteredForQuorum(inner) => {
                    <NotRegisteredForQuorum as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyAllocationManager(inner) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEjector(inner) => {
                    <OnlyEjector as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorSetsAlreadyEnabled(inner) => {
                    <OperatorSetsAlreadyEnabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorSetsNotEnabled(inner) => {
                    <OperatorSetsNotEnabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumDoesNotExist(inner) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumOperatorCountMismatch(inner) => {
                    <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RegistryCoordinatorSignatureExpired(inner) => {
                    <RegistryCoordinatorSignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SaltAlreadyUsed(inner) => {
                    <SaltAlreadyUsed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyRegisteredForQuorums(inner) => {
                    <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapCannotBeZero(inner) => {
                    <BitmapCannotBeZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapEmpty(inner) => {
                    <BitmapEmpty as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapUpdateIsAfterBlockNumber(inner) => {
                    <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotChurnSelf(inner) => {
                    <CannotChurnSelf as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotKickOperatorAboveThreshold(inner) => {
                    <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotReregisterYet(inner) => {
                    <CannotReregisterYet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ChurnApproverSaltUsed(inner) => {
                    <ChurnApproverSaltUsed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputLengthMismatch(inner) => {
                    <InputLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientStakeForChurn(inner) => {
                    <InsufficientStakeForChurn as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidRegistrationType(inner) => {
                    <InvalidRegistrationType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::M2QuorumsAlreadyDisabled(inner) => {
                    <M2QuorumsAlreadyDisabled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxQuorumsReached(inner) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NextBitmapUpdateIsBeforeBlockNumber(inner) => {
                    <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotRegistered(inner) => {
                    <NotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotRegisteredForQuorum(inner) => {
                    <NotRegisteredForQuorum as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyAllocationManager(inner) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEjector(inner) => {
                    <OnlyEjector as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorSetsAlreadyEnabled(inner) => {
                    <OperatorSetsAlreadyEnabled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorSetsNotEnabled(inner) => {
                    <OperatorSetsNotEnabled as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::QuorumOperatorCountMismatch(inner) => {
                    <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RegistryCoordinatorSignatureExpired(inner) => {
                    <RegistryCoordinatorSignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SaltAlreadyUsed(inner) => {
                    <SaltAlreadyUsed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`RegistryCoordinator`](self) events.
    pub enum RegistryCoordinatorEvents {
        #[allow(missing_docs)]
        ChurnApproverUpdated(ChurnApproverUpdated),
        #[allow(missing_docs)]
        EjectorUpdated(EjectorUpdated),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        M2QuorumsDisabled(M2QuorumsDisabled),
        #[allow(missing_docs)]
        OperatorDeregistered(OperatorDeregistered),
        #[allow(missing_docs)]
        OperatorRegistered(OperatorRegistered),
        #[allow(missing_docs)]
        OperatorSetParamsUpdated(OperatorSetParamsUpdated),
        #[allow(missing_docs)]
        OperatorSetsEnabled(OperatorSetsEnabled),
        #[allow(missing_docs)]
        OperatorSocketUpdate(OperatorSocketUpdate),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        QuorumBlockNumberUpdated(QuorumBlockNumberUpdated),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl RegistryCoordinatorEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                11u8, 136u8, 48u8, 111u8, 244u8, 98u8, 113u8, 33u8, 245u8, 179u8, 229u8, 177u8,
                197u8, 248u8, 143u8, 107u8, 30u8, 66u8, 253u8, 44u8, 4u8, 120u8, 239u8, 28u8,
                145u8, 102u8, 45u8, 73u8, 209u8, 240u8, 119u8, 85u8,
            ],
            [
                49u8, 84u8, 87u8, 216u8, 168u8, 254u8, 96u8, 240u8, 74u8, 241u8, 124u8, 22u8,
                226u8, 245u8, 165u8, 225u8, 219u8, 97u8, 43u8, 49u8, 100u8, 142u8, 88u8, 3u8, 3u8,
                96u8, 117u8, 158u8, 248u8, 243u8, 82u8, 140u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                57u8, 111u8, 220u8, 177u8, 128u8, 203u8, 15u8, 234u8, 38u8, 146u8, 129u8, 19u8,
                251u8, 15u8, 209u8, 195u8, 84u8, 152u8, 99u8, 249u8, 205u8, 86u8, 62u8, 106u8,
                24u8, 79u8, 29u8, 87u8, 129u8, 22u8, 200u8, 228u8,
            ],
            [
                62u8, 230u8, 254u8, 141u8, 84u8, 97u8, 2u8, 68u8, 195u8, 233u8, 211u8, 192u8,
                102u8, 174u8, 74u8, 238u8, 153u8, 120u8, 132u8, 170u8, 40u8, 241u8, 6u8, 22u8,
                174u8, 130u8, 25u8, 37u8, 64u8, 19u8, 24u8, 172u8,
            ],
            [
                70u8, 7u8, 125u8, 85u8, 51u8, 7u8, 99u8, 241u8, 98u8, 105u8, 253u8, 117u8, 229u8,
                118u8, 22u8, 99u8, 244u8, 25u8, 45u8, 39u8, 145u8, 116u8, 124u8, 1u8, 137u8, 177u8,
                106u8, 211u8, 29u8, 176u8, 125u8, 180u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                143u8, 48u8, 171u8, 9u8, 244u8, 58u8, 108u8, 21u8, 125u8, 127u8, 206u8, 126u8,
                10u8, 19u8, 192u8, 3u8, 4u8, 44u8, 28u8, 149u8, 232u8, 167u8, 46u8, 122u8, 20u8,
                106u8, 33u8, 192u8, 202u8, 162u8, 77u8, 201u8,
            ],
            [
                164u8, 205u8, 66u8, 146u8, 14u8, 208u8, 209u8, 55u8, 43u8, 164u8, 5u8, 29u8, 69u8,
                119u8, 39u8, 159u8, 35u8, 111u8, 187u8, 230u8, 119u8, 166u8, 127u8, 63u8, 125u8,
                100u8, 94u8, 130u8, 66u8, 93u8, 217u8, 141u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                232u8, 230u8, 140u8, 239u8, 28u8, 58u8, 118u8, 30u8, 215u8, 190u8, 126u8, 132u8,
                99u8, 163u8, 117u8, 242u8, 127u8, 123u8, 195u8, 53u8, 229u8, 24u8, 36u8, 34u8,
                60u8, 172u8, 206u8, 99u8, 110u8, 197u8, 195u8, 254u8,
            ],
            [
                236u8, 41u8, 99u8, 171u8, 33u8, 193u8, 229u8, 14u8, 30u8, 88u8, 42u8, 165u8, 66u8,
                175u8, 46u8, 75u8, 247u8, 191u8, 56u8, 230u8, 225u8, 64u8, 60u8, 39u8, 180u8, 46u8,
                28u8, 93u8, 110u8, 98u8, 30u8, 170u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for RegistryCoordinatorEvents {
        const NAME: &'static str = "RegistryCoordinatorEvents";
        const COUNT: usize = 13usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ChurnApproverUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChurnApproverUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ChurnApproverUpdated)
                }
                Some(<EjectorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EjectorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::EjectorUpdated)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<M2QuorumsDisabled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <M2QuorumsDisabled as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::M2QuorumsDisabled)
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
                Some(<OperatorSetParamsUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSetParamsUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSetParamsUpdated)
                }
                Some(<OperatorSetsEnabled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSetsEnabled as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSetsEnabled)
                }
                Some(<OperatorSocketUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSocketUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSocketUpdate)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Paused)
                }
                Some(<QuorumBlockNumberUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumBlockNumberUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumBlockNumberUpdated)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Unpaused)
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
    impl alloy_sol_types::private::IntoLogData for RegistryCoordinatorEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChurnApproverUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EjectorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::M2QuorumsDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetParamsUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetsEnabled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSocketUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::QuorumBlockNumberUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChurnApproverUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EjectorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::M2QuorumsDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetParamsUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetsEnabled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSocketUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::QuorumBlockNumberUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`RegistryCoordinator`](self) contract instance.

    See the [wrapper's documentation](`RegistryCoordinatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> RegistryCoordinatorInstance<T, P, N> {
        RegistryCoordinatorInstance::<T, P, N>::new(address, provider)
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
        _serviceManager: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        _blsApkRegistry: alloy::sol_types::private::Address,
        _indexRegistry: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<RegistryCoordinatorInstance<T, P, N>>>
    {
        RegistryCoordinatorInstance::<T, P, N>::deploy(
            provider,
            _serviceManager,
            _stakeRegistry,
            _blsApkRegistry,
            _indexRegistry,
            _allocationManager,
            _pauserRegistry,
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
        _serviceManager: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        _blsApkRegistry: alloy::sol_types::private::Address,
        _indexRegistry: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        RegistryCoordinatorInstance::<T, P, N>::deploy_builder(
            provider,
            _serviceManager,
            _stakeRegistry,
            _blsApkRegistry,
            _indexRegistry,
            _allocationManager,
            _pauserRegistry,
        )
    }
    /**A [`RegistryCoordinator`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`RegistryCoordinator`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RegistryCoordinatorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for RegistryCoordinatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RegistryCoordinatorInstance")
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
        > RegistryCoordinatorInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`RegistryCoordinator`](self) contract instance.

        See the [wrapper's documentation](`RegistryCoordinatorInstance`) for more details.*/
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
            _serviceManager: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            _blsApkRegistry: alloy::sol_types::private::Address,
            _indexRegistry: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<RegistryCoordinatorInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _serviceManager,
                _stakeRegistry,
                _blsApkRegistry,
                _indexRegistry,
                _allocationManager,
                _pauserRegistry,
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
            _serviceManager: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            _blsApkRegistry: alloy::sol_types::private::Address,
            _indexRegistry: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _serviceManager,
                        _stakeRegistry,
                        _blsApkRegistry,
                        _indexRegistry,
                        _allocationManager,
                        _pauserRegistry,
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
    impl<T, P: ::core::clone::Clone, N> RegistryCoordinatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RegistryCoordinatorInstance<T, P, N> {
            RegistryCoordinatorInstance {
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
        > RegistryCoordinatorInstance<T, P, N>
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
        ///Creates a new call builder for the [`OPERATOR_CHURN_APPROVAL_TYPEHASH`] function.
        pub fn OPERATOR_CHURN_APPROVAL_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, OPERATOR_CHURN_APPROVAL_TYPEHASHCall, N>
        {
            self.call_builder(&OPERATOR_CHURN_APPROVAL_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`PUBKEY_REGISTRATION_TYPEHASH`] function.
        pub fn PUBKEY_REGISTRATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, PUBKEY_REGISTRATION_TYPEHASHCall, N> {
            self.call_builder(&PUBKEY_REGISTRATION_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`accountIdentifier`] function.
        pub fn accountIdentifier(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, accountIdentifierCall, N> {
            self.call_builder(&accountIdentifierCall {})
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`blsApkRegistry`] function.
        pub fn blsApkRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, blsApkRegistryCall, N> {
            self.call_builder(&blsApkRegistryCall {})
        }
        ///Creates a new call builder for the [`calculateOperatorChurnApprovalDigestHash`] function.
        pub fn calculateOperatorChurnApprovalDigestHash(
            &self,
            registeringOperator: alloy::sol_types::private::Address,
            registeringOperatorId: alloy::sol_types::private::FixedBytes<32>,
            operatorKickParams: alloy::sol_types::private::Vec<
                <ISlashingRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
            >,
            salt: alloy::sol_types::private::FixedBytes<32>,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateOperatorChurnApprovalDigestHashCall, N>
        {
            self.call_builder(&calculateOperatorChurnApprovalDigestHashCall {
                registeringOperator,
                registeringOperatorId,
                operatorKickParams,
                salt,
                expiry,
            })
        }
        ///Creates a new call builder for the [`churnApprover`] function.
        pub fn churnApprover(&self) -> alloy_contract::SolCallBuilder<T, &P, churnApproverCall, N> {
            self.call_builder(&churnApproverCall {})
        }
        ///Creates a new call builder for the [`createSlashableStakeQuorum`] function.
        pub fn createSlashableStakeQuorum(
            &self,
            operatorSetParams: <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
            lookAheadPeriod: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, createSlashableStakeQuorumCall, N> {
            self.call_builder(&createSlashableStakeQuorumCall {
                operatorSetParams,
                minimumStake,
                strategyParams,
                lookAheadPeriod,
            })
        }
        ///Creates a new call builder for the [`createTotalDelegatedStakeQuorum`] function.
        pub fn createTotalDelegatedStakeQuorum(
            &self,
            operatorSetParams: <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createTotalDelegatedStakeQuorumCall, N> {
            self.call_builder(&createTotalDelegatedStakeQuorumCall {
                operatorSetParams,
                minimumStake,
                strategyParams,
            })
        }
        ///Creates a new call builder for the [`deregisterOperator_0`] function.
        pub fn deregisterOperator_0(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperator_0Call, N> {
            self.call_builder(&deregisterOperator_0Call {
                operator,
                operatorSetIds,
            })
        }
        ///Creates a new call builder for the [`deregisterOperator_1`] function.
        pub fn deregisterOperator_1(
            &self,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperator_1Call, N> {
            self.call_builder(&deregisterOperator_1Call { quorumNumbers })
        }
        ///Creates a new call builder for the [`disableM2QuorumRegistration`] function.
        pub fn disableM2QuorumRegistration(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, disableM2QuorumRegistrationCall, N> {
            self.call_builder(&disableM2QuorumRegistrationCall {})
        }
        ///Creates a new call builder for the [`ejectOperator`] function.
        pub fn ejectOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, ejectOperatorCall, N> {
            self.call_builder(&ejectOperatorCall {
                operator,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`ejectionCooldown`] function.
        pub fn ejectionCooldown(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, ejectionCooldownCall, N> {
            self.call_builder(&ejectionCooldownCall {})
        }
        ///Creates a new call builder for the [`ejector`] function.
        pub fn ejector(&self) -> alloy_contract::SolCallBuilder<T, &P, ejectorCall, N> {
            self.call_builder(&ejectorCall {})
        }
        ///Creates a new call builder for the [`enableOperatorSets`] function.
        pub fn enableOperatorSets(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, enableOperatorSetsCall, N> {
            self.call_builder(&enableOperatorSetsCall {})
        }
        ///Creates a new call builder for the [`getCurrentQuorumBitmap`] function.
        pub fn getCurrentQuorumBitmap(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentQuorumBitmapCall, N> {
            self.call_builder(&getCurrentQuorumBitmapCall { operatorId })
        }
        ///Creates a new call builder for the [`getOperator`] function.
        pub fn getOperator(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorCall, N> {
            self.call_builder(&getOperatorCall { operator })
        }
        ///Creates a new call builder for the [`getOperatorFromId`] function.
        pub fn getOperatorFromId(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorFromIdCall, N> {
            self.call_builder(&getOperatorFromIdCall { operatorId })
        }
        ///Creates a new call builder for the [`getOperatorId`] function.
        pub fn getOperatorId(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorIdCall, N> {
            self.call_builder(&getOperatorIdCall { operator })
        }
        ///Creates a new call builder for the [`getOperatorSetParams`] function.
        pub fn getOperatorSetParams(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSetParamsCall, N> {
            self.call_builder(&getOperatorSetParamsCall { quorumNumber })
        }
        ///Creates a new call builder for the [`getOperatorStatus`] function.
        pub fn getOperatorStatus(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorStatusCall, N> {
            self.call_builder(&getOperatorStatusCall { operator })
        }
        ///Creates a new call builder for the [`getQuorumBitmapAtBlockNumberByIndex`] function.
        pub fn getQuorumBitmapAtBlockNumberByIndex(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            blockNumber: u32,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapAtBlockNumberByIndexCall, N>
        {
            self.call_builder(&getQuorumBitmapAtBlockNumberByIndexCall {
                operatorId,
                blockNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`getQuorumBitmapHistoryLength`] function.
        pub fn getQuorumBitmapHistoryLength(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapHistoryLengthCall, N> {
            self.call_builder(&getQuorumBitmapHistoryLengthCall { operatorId })
        }
        ///Creates a new call builder for the [`getQuorumBitmapIndicesAtBlockNumber`] function.
        pub fn getQuorumBitmapIndicesAtBlockNumber(
            &self,
            blockNumber: u32,
            operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapIndicesAtBlockNumberCall, N>
        {
            self.call_builder(&getQuorumBitmapIndicesAtBlockNumberCall {
                blockNumber,
                operatorIds,
            })
        }
        ///Creates a new call builder for the [`getQuorumBitmapUpdateByIndex`] function.
        pub fn getQuorumBitmapUpdateByIndex(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapUpdateByIndexCall, N> {
            self.call_builder(&getQuorumBitmapUpdateByIndexCall { operatorId, index })
        }
        ///Creates a new call builder for the [`indexRegistry`] function.
        pub fn indexRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, indexRegistryCall, N> {
            self.call_builder(&indexRegistryCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _initialOwner: alloy::sol_types::private::Address,
            _churnApprover: alloy::sol_types::private::Address,
            _ejector: alloy::sol_types::private::Address,
            _initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
            _accountIdentifier: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                _initialOwner,
                _churnApprover,
                _ejector,
                _initialPausedStatus,
                _accountIdentifier,
            })
        }
        ///Creates a new call builder for the [`isChurnApproverSaltUsed`] function.
        pub fn isChurnApproverSaltUsed(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isChurnApproverSaltUsedCall, N> {
            self.call_builder(&isChurnApproverSaltUsedCall { _0 })
        }
        ///Creates a new call builder for the [`isM2Quorum`] function.
        pub fn isM2Quorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, isM2QuorumCall, N> {
            self.call_builder(&isM2QuorumCall { quorumNumber })
        }
        ///Creates a new call builder for the [`lastEjectionTimestamp`] function.
        pub fn lastEjectionTimestamp(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastEjectionTimestampCall, N> {
            self.call_builder(&lastEjectionTimestampCall { _0 })
        }
        ///Creates a new call builder for the [`m2QuorumsDisabled`] function.
        pub fn m2QuorumsDisabled(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, m2QuorumsDisabledCall, N> {
            self.call_builder(&m2QuorumsDisabledCall {})
        }
        ///Creates a new call builder for the [`numRegistries`] function.
        pub fn numRegistries(&self) -> alloy_contract::SolCallBuilder<T, &P, numRegistriesCall, N> {
            self.call_builder(&numRegistriesCall {})
        }
        ///Creates a new call builder for the [`operatorSetsEnabled`] function.
        pub fn operatorSetsEnabled(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSetsEnabledCall, N> {
            self.call_builder(&operatorSetsEnabledCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauseCall, N> {
            self.call_builder(&pauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`pauseAll`] function.
        pub fn pauseAll(&self) -> alloy_contract::SolCallBuilder<T, &P, pauseAllCall, N> {
            self.call_builder(&pauseAllCall {})
        }
        ///Creates a new call builder for the [`paused_0`] function.
        pub fn paused_0(
            &self,
            index: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, paused_0Call, N> {
            self.call_builder(&paused_0Call { index })
        }
        ///Creates a new call builder for the [`paused_1`] function.
        pub fn paused_1(&self) -> alloy_contract::SolCallBuilder<T, &P, paused_1Call, N> {
            self.call_builder(&paused_1Call {})
        }
        ///Creates a new call builder for the [`pauserRegistry`] function.
        pub fn pauserRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauserRegistryCall, N> {
            self.call_builder(&pauserRegistryCall {})
        }
        ///Creates a new call builder for the [`pubkeyRegistrationMessageHash`] function.
        pub fn pubkeyRegistrationMessageHash(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, pubkeyRegistrationMessageHashCall, N> {
            self.call_builder(&pubkeyRegistrationMessageHashCall { operator })
        }
        ///Creates a new call builder for the [`quorumCount`] function.
        pub fn quorumCount(&self) -> alloy_contract::SolCallBuilder<T, &P, quorumCountCall, N> {
            self.call_builder(&quorumCountCall {})
        }
        ///Creates a new call builder for the [`quorumUpdateBlockNumber`] function.
        pub fn quorumUpdateBlockNumber(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, quorumUpdateBlockNumberCall, N> {
            self.call_builder(&quorumUpdateBlockNumberCall { _0 })
        }
        ///Creates a new call builder for the [`registerOperator_0`] function.
        pub fn registerOperator_0(
            &self,
            quorumNumbers: alloy::sol_types::private::Bytes,
            socket: alloy::sol_types::private::String,
            params: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperator_0Call, N> {
            self.call_builder(&registerOperator_0Call {
                quorumNumbers,
                socket,
                params,
                operatorSignature,
            })
        }
        ///Creates a new call builder for the [`registerOperator_1`] function.
        pub fn registerOperator_1(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperator_1Call, N> {
            self.call_builder(&registerOperator_1Call {
                operator,
                operatorSetIds,
                data,
            })
        }
        ///Creates a new call builder for the [`registerOperatorWithChurn`] function.
        pub fn registerOperatorWithChurn(
            &self,
            quorumNumbers: alloy::sol_types::private::Bytes,
            socket: alloy::sol_types::private::String,
            params: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
            operatorKickParams: alloy::sol_types::private::Vec<
                <ISlashingRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
            >,
            churnApproverSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorWithChurnCall, N> {
            self.call_builder(&registerOperatorWithChurnCall {
                quorumNumbers,
                socket,
                params,
                operatorKickParams,
                churnApproverSignature,
                operatorSignature,
            })
        }
        ///Creates a new call builder for the [`registries`] function.
        pub fn registries(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, registriesCall, N> {
            self.call_builder(&registriesCall { _0 })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`serviceManager`] function.
        pub fn serviceManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceManagerCall, N> {
            self.call_builder(&serviceManagerCall {})
        }
        ///Creates a new call builder for the [`setAccountIdentifier`] function.
        pub fn setAccountIdentifier(
            &self,
            _accountIdentifier: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAccountIdentifierCall, N> {
            self.call_builder(&setAccountIdentifierCall { _accountIdentifier })
        }
        ///Creates a new call builder for the [`setChurnApprover`] function.
        pub fn setChurnApprover(
            &self,
            _churnApprover: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setChurnApproverCall, N> {
            self.call_builder(&setChurnApproverCall { _churnApprover })
        }
        ///Creates a new call builder for the [`setEjectionCooldown`] function.
        pub fn setEjectionCooldown(
            &self,
            _ejectionCooldown: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setEjectionCooldownCall, N> {
            self.call_builder(&setEjectionCooldownCall { _ejectionCooldown })
        }
        ///Creates a new call builder for the [`setEjector`] function.
        pub fn setEjector(
            &self,
            _ejector: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setEjectorCall, N> {
            self.call_builder(&setEjectorCall { _ejector })
        }
        ///Creates a new call builder for the [`setOperatorSetParams`] function.
        pub fn setOperatorSetParams(
            &self,
            quorumNumber: u8,
            operatorSetParams: <ISlashingRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setOperatorSetParamsCall, N> {
            self.call_builder(&setOperatorSetParamsCall {
                quorumNumber,
                operatorSetParams,
            })
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`updateOperators`] function.
        pub fn updateOperators(
            &self,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsCall, N> {
            self.call_builder(&updateOperatorsCall { operators })
        }
        ///Creates a new call builder for the [`updateOperatorsForQuorum`] function.
        pub fn updateOperatorsForQuorum(
            &self,
            operatorsPerQuorum: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsForQuorumCall, N> {
            self.call_builder(&updateOperatorsForQuorumCall {
                operatorsPerQuorum,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`updateSocket`] function.
        pub fn updateSocket(
            &self,
            socket: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateSocketCall, N> {
            self.call_builder(&updateSocketCall { socket })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > RegistryCoordinatorInstance<T, P, N>
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
        ///Creates a new event filter for the [`ChurnApproverUpdated`] event.
        pub fn ChurnApproverUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ChurnApproverUpdated, N> {
            self.event_filter::<ChurnApproverUpdated>()
        }
        ///Creates a new event filter for the [`EjectorUpdated`] event.
        pub fn EjectorUpdated_filter(&self) -> alloy_contract::Event<T, &P, EjectorUpdated, N> {
            self.event_filter::<EjectorUpdated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`M2QuorumsDisabled`] event.
        pub fn M2QuorumsDisabled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, M2QuorumsDisabled, N> {
            self.event_filter::<M2QuorumsDisabled>()
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
        ///Creates a new event filter for the [`OperatorSetParamsUpdated`] event.
        pub fn OperatorSetParamsUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSetParamsUpdated, N> {
            self.event_filter::<OperatorSetParamsUpdated>()
        }
        ///Creates a new event filter for the [`OperatorSetsEnabled`] event.
        pub fn OperatorSetsEnabled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSetsEnabled, N> {
            self.event_filter::<OperatorSetsEnabled>()
        }
        ///Creates a new event filter for the [`OperatorSocketUpdate`] event.
        pub fn OperatorSocketUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSocketUpdate, N> {
            self.event_filter::<OperatorSocketUpdate>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<T, &P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`QuorumBlockNumberUpdated`] event.
        pub fn QuorumBlockNumberUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumBlockNumberUpdated, N> {
            self.event_filter::<QuorumBlockNumberUpdated>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
