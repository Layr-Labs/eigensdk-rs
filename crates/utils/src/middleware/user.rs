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
    clippy::style
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
    clippy::style
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
        pub pubkeyRegistrationSignature: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub pubkeyG1: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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

    constructor(string name, uint256 _privKey, IBLSApkRegistry.PubkeyRegistrationParams _pubkeyParams);

    function IS_TEST() external view returns (bool);
    function NAME() external view returns (string memory);
    function depositIntoEigenlayer(address[] memory strategies, uint256[] memory tokenBalances) external;
    function deregisterOperator(bytes memory quorums) external;
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function exitEigenlayer() external returns (address[] memory, uint256[] memory);
    function failed() external returns (bool);
    function isValidSignature(bytes32 digestHash, bytes memory) external view returns (bytes4);
    function operatorId() external view returns (bytes32);
    function pubkeyG1() external view returns (BN254.G1Point memory);
    function registerAsOperator() external;
    function registerOperator(bytes memory quorums) external returns (bytes32);
    function registerOperatorWithChurn(bytes memory churnQuorums, address[] memory churnTargets, bytes memory standardQuorums) external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function updateStakes() external;
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
      },
      {
        "name": "_privKey",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_pubkeyParams",
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
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "deregisterOperator",
    "inputs": [
      {
        "name": "quorums",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "exitEigenlayer",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
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
    "name": "isValidSignature",
    "inputs": [
      {
        "name": "digestHash",
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
    "name": "operatorId",
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
    "name": "pubkeyG1",
    "inputs": [],
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
    "name": "registerAsOperator",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerOperator",
    "inputs": [
      {
        "name": "quorums",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerOperatorWithChurn",
    "inputs": [
      {
        "name": "churnQuorums",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "churnTargets",
        "type": "address[]",
        "internalType": "contract User[]"
      },
      {
        "name": "standardQuorums",
        "type": "bytes",
        "internalType": "bytes"
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
    "type": "function",
    "name": "updateStakes",
    "inputs": [],
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
    ///0x60806040526007805460ff199081166001908117909255600b80549091169091179055601c80546001600160a01b031916737109709ecfa91a80626ff3989d68f67f5b1dd12d17905560006034553480156200005a57600080fd5b50604051620046f4380380620046f48339810160408190526200007d9162000b62565b6000339050806001600160a01b0316636d14a9876040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000c1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000e7919062000c59565b602080546001600160a01b0319166001600160a01b039283161781556040805163359d539760e11b8152905192841692636b3aa72e926004808401939192918290030181865afa15801562000140573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000166919062000c59565b601f80546001600160a01b0319166001600160a01b039283161790556020805460408051633998fdd360e01b815290519190931692633998fdd39260048083019391928290030181865afa158015620001c3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001e9919062000c59565b602180546001600160a01b0319166001600160a01b039283161790556020805460408051632efa2ca360e11b815290519190931692635df459469260048083019391928290030181865afa15801562000246573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200026c919062000c59565b602280546001600160a01b0319166001600160a01b039283161790556020805460408051636830483560e01b81529051919093169263683048359260048083019391928290030181865afa158015620002c9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620002ef919062000c59565b602380546001600160a01b0319166001600160a01b039283161790556020805460408051634f4c91e160e11b815290519190931692639e9923c29260048083019391928290030181865afa1580156200034c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000372919062000c59565b602480546001600160a01b0319166001600160a01b039283161790556023546040805163df5cf72360e01b81529051919092169163df5cf7239160048083019260209291908290030181865afa158015620003d1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620003f7919062000c59565b601d80546001600160a01b0319166001600160a01b0392909216918217905560408051630736e1c760e31b815290516339b70e38916004808201926020929091908290030181865afa15801562000452573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000478919062000c59565b601e80546001600160a01b0319166001600160a01b039283161790556021546040805163359d539760e11b815290519190921691636b3aa72e9160048083019260209291908290030181865afa158015620004d7573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620004fd919062000c59565b601f60006101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316633dfb40e06040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000562573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000588919062000c59565b602560006101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316632dbcb04c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015620005ed573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000613919062000c80565b602681905550806001600160a01b031663054310e66040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000658573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200067e919062000c59565b602780546001600160a01b0319166001600160a01b03929092169190911790558351620006b3906028906020870190620008a2565b50602a83905581518051602b908155602091820151602c55818401518051602d5590910151602e556040830151805184929190602f90620006f8908290600262000931565b50602082015162000710906002808401919062000931565b5050602054604051630f0a9fd360e21b8152306004820152600094506001600160a01b039091169250633c2a7f4c91506024016040805180830381865afa15801562000760573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000786919062000c9a565b9050620007a4602a5482620007eb60201b62001bcd1790919060201c565b8051602b55602090810151602c5560408051808201909152602d548152602e5481830152620007dc9162001c646200088b821b17901c565b6029555062000cf69350505050565b60408051808201909152600080825260208201526200080962000961565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa90508080156200083e5762000840565bfe5b5080620008835760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b604482015260640160405180910390fd5b505092915050565b805160009081526020918201519091526040902090565b828054620008b09062000cb9565b90600052602060002090601f016020900481019282620008d457600085556200091f565b82601f10620008ef57805160ff19168380011785556200091f565b828001600101855582156200091f579182015b828111156200091f57825182559160200191906001019062000902565b506200092d9291506200097f565b5090565b82600281019282156200091f57916020028201828111156200091f57825182559160200191906001019062000902565b60405180606001604052806003906020820280368337509192915050565b5b808211156200092d576000815560010162000980565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b0381118282101715620009d157620009d162000996565b60405290565b604051601f8201601f191681016001600160401b038111828210171562000a025762000a0262000996565b604052919050565b60006040828403121562000a1d57600080fd5b604080519081016001600160401b038111828210171562000a425762000a4262000996565b604052825181526020928301519281019290925250919050565b600082601f83011262000a6e57600080fd5b62000a78620009ac565b80604084018581111562000a8b57600080fd5b845b8181101562000aa757805184526020938401930162000a8d565b509095945050505050565b600081830361010081121562000ac757600080fd5b604051606081016001600160401b038111828210171562000aec5762000aec62000996565b60405291508162000afe858562000a0a565b815262000b0f856040860162000a0a565b60208201526080607f198301121562000b2757600080fd5b62000b31620009ac565b915062000b42856080860162000a5c565b825262000b538560c0860162000a5c565b60208301526040015292915050565b6000806000610140848603121562000b7957600080fd5b83516001600160401b038082111562000b9157600080fd5b818601915086601f83011262000ba657600080fd5b81518181111562000bbb5762000bbb62000996565b6020915062000bd3601f8201601f19168301620009d7565b818152888383860101111562000be857600080fd5b60005b8281101562000c0857848101840151828201850152830162000beb565b8281111562000c1a5760008484840101525b508096505050808601519350505062000c37856040860162000ab2565b90509250925092565b6001600160a01b038116811462000c5657600080fd5b50565b60006020828403121562000c6c57600080fd5b815162000c798162000c40565b9392505050565b60006020828403121562000c9357600080fd5b5051919050565b60006040828403121562000cad57600080fd5b62000c79838362000a0a565b600181811c9082168062000cce57607f821691505b6020821081141562000cf057634e487b7160e01b600052602260045260246000fd5b50919050565b6139ee8062000d066000396000f3fe608060405234801561001057600080fd5b50600436106101425760003560e01c806385226c81116100b8578063b5508aa91161007c578063b5508aa9146102a6578063ba414fa6146102ae578063bf68b816146102c6578063ca4f2d97146102cf578063e20c9f71146102e2578063fa7626d4146102ea57600080fd5b806385226c8114610223578063916a17c614610238578063a3f4df7e14610240578063a5f6cc1a14610255578063afa1c7371461026857600080fd5b80633f7286f41161010a5780633f7286f4146101b4578063505377e2146101bc57806365eda8e5146101c457806366d9a9a0146101da5780636d336f58146101ef5780638231b54c1461020257600080fd5b80631626ba7e146101475780631ed7831c146101785780632a34ade81461018d5780632ade3880146101975780633e5e3c23146101ac575b600080fd5b61015a6101553660046128d4565b6102f7565b6040516001600160e01b031990911681526020015b60405180910390f35b610180610326565b60405161016f919061295d565b610195610388565b005b61019f6104c1565b60405161016f9190612a06565b610180610603565b610180610663565b6101956106c3565b6101cc6107ee565b60405161016f929190612b3a565b6101e2610a29565b60405161016f9190612b68565b6101956101fd366004612cb9565b610b0f565b610215610210366004612dc2565b610d88565b60405190815260200161016f565b61022b610ece565b60405161016f9190612e03565b6101e2610f9e565b610248611084565b60405161016f9190612e65565b610195610263366004612e78565b611112565b604080518082018252600080825260209182015281518083018352602d54808252602e5491830191825283519081529051918101919091520161016f565b61022b611859565b6102b6611929565b604051901515815260200161016f565b61021560295481565b6101956102dd366004612dc2565b611a56565b610180611b6d565b6007546102b69060ff1681565b60008281526033602052604081205460ff161561031c5750630b135d3f60e11b610320565b5060005b92915050565b6060601480548060200260200160405190810160405280929190818152602001828054801561037e57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610360575b5050505050905090565b602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156103dd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104019190612f42565b506104406040518060400160405280601981526020017f726567697374657241734f70657261746f722028636f72652900000000000000815250611c7b565b6040805160608101825230815260006020820181905281830152601d549151630f589e5960e01b815290916001600160a01b031690630f589e599061048c90849060289060040161300d565b600060405180830381600087803b1580156104a657600080fd5b505af11580156104ba573d6000803e3d6000fd5b5050505050565b6060601b805480602002602001604051908101604052809291908181526020016000905b828210156105fa57600084815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b828210156105e357838290600052602060002001805461055690612f5b565b80601f016020809104026020016040519081016040528092919081815260200182805461058290612f5b565b80156105cf5780601f106105a4576101008083540402835291602001916105cf565b820191906000526020600020905b8154815290600101906020018083116105b257829003601f168201915b505050505081526020019060010190610537565b5050505081525050815260200190600101906104e5565b50505050905090565b6060601680548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b6060601580548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610718573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061073c9190612f42565b5061077b6040518060400160405280601e81526020017f7570646174655374616b657320287570646174654f70657261746f7273290000815250611c7b565b6040805160018082528183019092526000916020808301908036833701905050905030816000815181106107b1576107b161304d565b6001600160a01b039283166020918202929092018101919091525460405162cf2ab560e01b815291169062cf2ab59061048c90849060040161295d565b606080602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610846573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061086a9190612f42565b506108a16040518060400160405280601581526020017465786974456967656e6c617965722028636f72652960581b815250611c7b565b601d546040516367c0439f60e11b815230600482015260009182916001600160a01b039091169063cf80873e90602401600060405180830381865afa1580156108ee573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261091691908101906130be565b60408051600180825281830190925292945090925060009190816020015b604080516060808201835280825260208201526000918101919091528152602001906001900390816109345790505090506040518060600160405280848152602001838152602001306001600160a01b03168152508160008151811061099c5761099c61304d565b6020908102919091010152601d546040516306ec6e8160e11b81526001600160a01b0390911690630dd8dd02906109d7908490600401613178565b6000604051808303816000875af11580156109f6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610a1e9190810190613212565b509193509150509091565b60606019805480602002602001604051908101604052809291908181526020016000905b828210156105fa5760008481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015610af757602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411610ab95790505b50505050508152505081526020019060010190610a4d565b602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610b64573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b889190612f42565b50610bc76040518060400160405280601c81526020017f6465706f736974496e746f456967656e4c617965722028636f72652900000000815250611c7b565b60005b8251811015610d83576000838281518110610be757610be761304d565b602002602001015190506000838381518110610c0557610c0561304d565b602002602001015190506000826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c4f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c7391906132a2565b601e5460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303816000875af1158015610cc8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cec91906132bf565b50601e546040516373d0285560e11b81526001600160a01b0385811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303816000875af1158015610d48573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d6c9190612f42565b505050508080610d7b906132f7565b915050610bca565b505050565b602554604080516301504d8f60e41b815290516000926001600160a01b031691631504d8f0916004808301926020929190829003018187875af1158015610dd3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610df79190612f42565b50610e2b6040518060400160405280601081526020016f3932b3b4b9ba32b927b832b930ba37b960811b8152508484611cc3565b6020546001600160a01b031663a50857bf84846028602b610e4a611d48565b6040518663ffffffff1660e01b8152600401610e6a9594939291906133d5565b600060405180830381600087803b158015610e8457600080fd5b505af1158015610e98573d6000803e3d6000fd5b5050604080518082018252602d54808252602e546020928301908152600091825251909152209150610ec79050565b9392505050565b60606018805480602002602001604051908101604052809291908181526020016000905b828210156105fa578382906000526020600020018054610f1190612f5b565b80601f0160208091040260200160405190810160405280929190818152602001828054610f3d90612f5b565b8015610f8a5780601f10610f5f57610100808354040283529160200191610f8a565b820191906000526020600020905b815481529060010190602001808311610f6d57829003601f168201915b505050505081526020019060010190610ef2565b6060601a805480602002602001604051908101604052809291908181526020016000905b828210156105fa5760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561106c57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161102e5790505b50505050508152505081526020019060010190610fc2565b6028805461109190612f5b565b80601f01602080910402602001604051908101604052809291908181526020018280546110bd90612f5b565b801561110a5780601f106110df5761010080835404028352916020019161110a565b820191906000526020600020905b8154815290600101906020018083116110ed57829003601f168201915b505050505081565b602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015611167573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061118b9190612f42565b506112666040518060400160405280601981526020017f72656769737465724f70657261746f7257697468436875726e0000000000000081525087878080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525050604080516020808b0282810182019093528a82529093508a92508991829185019084908082843760009201919091525050604080516020601f8a018190048102820181019092528881529250889150879081908401838280828437600092019190915250611e5092505050565b60006112a787878080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506120d392505050565b905060006112ea84848080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506120d392505050565b6040805160608101909152603580825291925061131291899188916139266020830139612260565b6113408183166001600160c01b0316156040518060600160405280603e815260200161395b603e9139612296565b60006113596001600160c01b03838116908516176122cd565b9050600081516001600160401b0381111561137657611376612867565b6040519080825280602002602001820160405280156113bb57816020015b60408051808201909152600080825260208201528152602001906001900390816113945790505b5090506000805b83516113ce828461342c565b10156115e957818b1415611429576040805180820190915260008082526020820152836113fb838561342c565b8151811061140b5761140b61304d565b60200260200101819052508080611421906132f7565b9150506113c2565b8087148061147c57508787828181106114445761144461304d565b909101356001600160f81b03191690508c8c848181106114665761146661304d565b9050013560f81c60f81b6001600160f81b031916105b156115175760405180604001604052808d8d8581811061149e5761149e61304d565b919091013560f81c8252506020018b8b858181106114be576114be61304d565b90506020020160208101906114d39190613444565b6001600160a01b03169052836114e9838561342c565b815181106114f9576114f961304d565b6020026020010181905250818061150f906132f7565b9250506113c2565b8b8b838181106115295761152961304d565b909101356001600160f81b031916905088888381811061154b5761154b61304d565b9050013560f81c60f81b6001600160f81b0319161015611584576040805180820190915260008082526020820152836113fb838561342c565b60405162461bcd60e51b815260206004820152602f60248201527f557365722e72656769737465724f70657261746f7257697468436875726e3a2060448201526e1b585b199bdc9b5959081a5b9c1d5d608a1b60648201526084015b60405180910390fd5b60006034600081546115fa906132f7565b91829055506040805160208101929092526bffffffffffffffffffffffff193060601b169082015260540160408051601f1981840301815290829052805160209182012090546029546384ca521360e01b8452919350600019926000926001600160a01b03909216916384ca52139161167d9130918b90899089906004016134a8565b602060405180830381865afa15801561169a573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116be9190612f42565b601c546026546040516338d07aa960e21b8152929350600092839283926001600160a01b039091169163e341eaa491611704918890600401918252602082015260400190565b606060405180830381865afa158015611721573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061174591906134e3565b604080516041808252608082019092529396509194509250600091906020820181803683370190505090508260208201528160408201528360f81b816001835161178f9190613520565b8151811061179f5761179f61304d565b60200101906001600160f81b031916908160001a9053506040805160608101825282815260208082018a905291810188905290546001600160a01b0316639b5d177b8d6028602b8f866117f0611d48565b6040518763ffffffff1660e01b815260040161181196959493929190613537565b600060405180830381600087803b15801561182b57600080fd5b505af115801561183f573d6000803e3d6000fd5b505050505050505050505050505050505050505050505050565b60606017805480602002602001604051908101604052809291908181526020016000905b828210156105fa57838290600052602060002001805461189c90612f5b565b80601f01602080910402602001604051908101604052809291908181526020018280546118c890612f5b565b80156119155780601f106118ea57610100808354040283529160200191611915565b820191906000526020600020905b8154815290600101906020018083116118f857829003601f168201915b50505050508152602001906001019061187d565b600754600090610100900460ff161561194b5750600754610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611a515760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b828401528251808303840181526060830190935260009290916119d9917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc4916080016135b8565b60408051601f19818403018152908290526119f3916135e9565b6000604051808303816000865af19150503d8060008114611a30576040519150601f19603f3d011682016040523d82523d6000602084013e611a35565b606091505b5091505080806020019051810190611a4d91906132bf565b9150505b919050565b602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015611aab573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611acf9190612f42565b50611b05604051806040016040528060128152602001713232b932b3b4b9ba32b927b832b930ba37b960711b8152508383611cc3565b60205460405163ca4f2d9760e01b81526001600160a01b039091169063ca4f2d9790611b379085908590600401613605565b600060405180830381600087803b158015611b5157600080fd5b505af1158015611b65573d6000803e3d6000fd5b505050505050565b6060601380548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b6040805180820190915260008082526020820152611be9612849565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa9050808015611c1c57611c1e565bfe5b5080611c5c5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b60448201526064016115e0565b505092915050565b805160009081526020918201519091526040902090565b600080516020613906833981519152602882604051602001611c9e929190613619565b60408051601f1981840301815290829052611cb891612e65565b60405180910390a150565b600080516020613999833981519152602884604051602001611ce6929190613619565b60408051601f198184030181526020601f860181900481028401810190925284835291611d2d91869086908190840183828082843760009201919091525061239992505050565b604051611d3b9291906136a9565b60405180910390a1505050565b604080516060808201835280825260006020830181905282840181905283519182018181526080830190945291928190815260200160346000815480929190611d90906132f7565b909155508152600019602091820152601f54602154918301516040808501519051631420c19160e31b81523060048201526001600160a01b0394851660248201526044810192909252606482015292935060009291169063a1060c8890608401602060405180830381865afa158015611e0d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e319190612f42565b6000908152603360205260409020805460ff1916600117905550919050565b600080516020613906833981519152602885604051602001611e73929190613619565b60408051601f1981840301815290829052611e8d91612e65565b60405180910390a1600080516020613999833981519152611ead82612399565b604051611eba91906136ce565b60405180910390a1600080516020613999833981519152611eda84612399565b604051611ee79190613709565b60405180910390a16040805180820190915260018152605b60f81b602082015260005b83518110156120845760018451611f219190613520565b811415611fcf5781848281518110611f3b57611f3b61304d565b60200260200101516001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa158015611f80573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611fa89190810190613741565b604051602001611fb99291906137ae565b6040516020818303038152906040529150612072565b81848281518110611fe257611fe261304d565b60200260200101516001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa158015612027573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261204f9190810190613741565b6040516020016120609291906137dd565b60405160208183030381529060405291505b8061207c816132f7565b915050611f0a565b50806040516020016120969190613819565b6040516020818303038152906040529050600080516020613999833981519152816040516120c4919061383e565b60405180910390a15050505050565b60006101008251111561215c5760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a4016115e0565b815161216a57506000919050565b600080836000815181106121805761218061304d565b0160200151600160f89190911c81901b92505b8451811015612257578481815181106121ae576121ae61304d565b0160200151600160f89190911c1b91508282116122435760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a4016115e0565b91811791612250816132f7565b9050612193565b50909392505050565b818314610d8357600080516020613999833981519152816040516122849190613876565b60405180910390a1610d838383612493565b816122c957600080516020613999833981519152816040516122b89190613876565b60405180910390a16122c9826125a8565b5050565b60606000806122db8461260d565b61ffff166001600160401b038111156122f6576122f6612867565b6040519080825280601f01601f191660200182016040528015612320576020820181803683370190505b5090506000805b825182108015612338575061010081105b1561238f576001811b93508584161561237f578060f81b8383815181106123615761236161304d565b60200101906001600160f81b031916908160001a9053508160010191505b612388816132f7565b9050612327565b5090949350505050565b6040805180820190915260018152605b60f81b602082015260609060005b835181101561246a57600184516123ce9190613520565b81141561241f57816123f88583815181106123eb576123eb61304d565b016020015160f81c612638565b6040516020016124099291906137ae565b6040516020818303038152906040529150612458565b816124358583815181106123eb576123eb61304d565b6040516020016124469291906137dd565b60405160208183030381529060405291505b80612462816132f7565b9150506123b7565b508060405160200161247c9190613819565b60408051601f198184030181529190529392505050565b8082146122c9576000805160206139068339815191526040516124f29060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808080808081319599d60b21b60608201526020810184905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160408051818152600a81830152690808080808149a59da1d60b21b60608201526020810183905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a16122c961273d565b8061260a576000805160206139068339815191526040516125fa9060208082526017908201527f4572726f723a20417373657274696f6e204661696c6564000000000000000000604082015260600190565b60405180910390a161260a61273d565b50565b6000805b821561032057612622600184613520565b9092169180612630816138a5565b915050612611565b60608161265c5750506040805180820190915260018152600360fc1b602082015290565b8160005b81156126865780612670816132f7565b915061267f9050600a836138dd565b9150612660565b6000816001600160401b038111156126a0576126a0612867565b6040519080825280601f01601f1916602001820160405280156126ca576020820181803683370190505b5090505b8415612735576126df600183613520565b91506126ec600a866138f1565b6126f790603061342c565b60f81b81838151811061270c5761270c61304d565b60200101906001600160f81b031916908160001a90535061272e600a866138dd565b94506126ce565b949350505050565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156128385760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f19818403018152908290526127d792916020016135b8565b60408051601f19818403018152908290526127f1916135e9565b6000604051808303816000865af19150503d806000811461282e576040519150601f19603f3d011682016040523d82523d6000602084013e612833565b606091505b505050505b6007805461ff001916610100179055565b60405180606001604052806003906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b03811182821017156128a5576128a5612867565b604052919050565b60006001600160401b038211156128c6576128c6612867565b50601f01601f191660200190565b600080604083850312156128e757600080fd5b8235915060208301356001600160401b0381111561290457600080fd5b8301601f8101851361291557600080fd5b8035612928612923826128ad565b61287d565b81815286602083850101111561293d57600080fd5b816020840160208301376000602083830101528093505050509250929050565b6020808252825182820181905260009190848201906040850190845b8181101561299e5783516001600160a01b031683529284019291840191600101612979565b50909695505050505050565b60005b838110156129c55781810151838201526020016129ad565b838111156129d4576000848401525b50505050565b600081518084526129f28160208601602086016129aa565b601f01601f19169290920160200192915050565b602080825282518282018190526000919060409081850190600581811b8701840188860187805b85811015612ab657603f198b8503018752825180516001600160a01b031685528901518985018990528051898601819052908a0190606081881b870181019190870190855b81811015612aa057605f19898503018352612a8e8486516129da565b948e01949350918d0191600101612a72565b505050978a019794505091880191600101612a2d565b50919a9950505050505050505050565b600081518084526020808501945080840160005b83811015612aff5781516001600160a01b031687529582019590820190600101612ada565b509495945050505050565b600081518084526020808501945080840160005b83811015612aff57815187529582019590820190600101612b1e565b604081526000612b4d6040830185612ac6565b8281036020840152612b5f8185612b0a565b95945050505050565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b84811015612c0c57898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b80831015612bf75783516001600160e01b0319168252928b019260019290920191908b0190612bcd565b50978a01979550505091870191600101612b90565b50919998505050505050505050565b60006001600160401b03821115612c3457612c34612867565b5060051b60200190565b6001600160a01b038116811461260a57600080fd5b600082601f830112612c6457600080fd5b81356020612c7461292383612c1b565b82815260059290921b84018101918181019086841115612c9357600080fd5b8286015b84811015612cae5780358352918301918301612c97565b509695505050505050565b60008060408385031215612ccc57600080fd5b82356001600160401b0380821115612ce357600080fd5b818501915085601f830112612cf757600080fd5b81356020612d0761292383612c1b565b82815260059290921b84018101918181019089841115612d2657600080fd5b948201945b83861015612d4d578535612d3e81612c3e565b82529482019490820190612d2b565b96505086013592505080821115612d6357600080fd5b50612d7085828601612c53565b9150509250929050565b60008083601f840112612d8c57600080fd5b5081356001600160401b03811115612da357600080fd5b602083019150836020828501011115612dbb57600080fd5b9250929050565b60008060208385031215612dd557600080fd5b82356001600160401b03811115612deb57600080fd5b612df785828601612d7a565b90969095509350505050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b82811015612e5857603f19888603018452612e468583516129da565b94509285019290850190600101612e2a565b5092979650505050505050565b602081526000610ec760208301846129da565b60008060008060008060608789031215612e9157600080fd5b86356001600160401b0380821115612ea857600080fd5b612eb48a838b01612d7a565b90985096506020890135915080821115612ecd57600080fd5b818901915089601f830112612ee157600080fd5b813581811115612ef057600080fd5b8a60208260051b8501011115612f0557600080fd5b602083019650809550506040890135915080821115612f2357600080fd5b50612f3089828a01612d7a565b979a9699509497509295939492505050565b600060208284031215612f5457600080fd5b5051919050565b600181811c90821680612f6f57607f821691505b60208210811415612f9057634e487b7160e01b600052602260045260246000fd5b50919050565b60008154612fa381612f5b565b808552602060018381168015612fc05760018114612fd457613002565b60ff19851688840152604088019550613002565b866000528260002060005b85811015612ffa5781548a8201860152908301908401612fdf565b890184019650505b505050505092915050565b600060018060a01b038085511683528060208601511660208401525063ffffffff6040850151166040830152608060608301526127356080830184612f96565b634e487b7160e01b600052603260045260246000fd5b600082601f83011261307457600080fd5b8151602061308461292383612c1b565b82815260059290921b840181019181810190868411156130a357600080fd5b8286015b84811015612cae57805183529183019183016130a7565b600080604083850312156130d157600080fd5b82516001600160401b03808211156130e857600080fd5b818501915085601f8301126130fc57600080fd5b8151602061310c61292383612c1b565b82815260059290921b8401810191818101908984111561312b57600080fd5b948201945b8386101561315257855161314381612c3e565b82529482019490820190613130565b9188015191965090935050508082111561316b57600080fd5b50612d7085828601613063565b60006020808301818452808551808352604092508286019150828160051b87010184880160005b8381101561320457603f198984030185528151606081518186526131c582870182612ac6565b915050888201518582038a8701526131dd8282612b0a565b928901516001600160a01b031695890195909552509487019492509086019060010161319f565b509098975050505050505050565b6000602080838503121561322557600080fd5b82516001600160401b0381111561323b57600080fd5b8301601f8101851361324c57600080fd5b805161325a61292382612c1b565b81815260059190911b8201830190838101908783111561327957600080fd5b928401925b828410156132975783518252928401929084019061327e565b979650505050505050565b6000602082840312156132b457600080fd5b8151610ec781612c3e565b6000602082840312156132d157600080fd5b81518015158114610ec757600080fd5b634e487b7160e01b600052601160045260246000fd5b600060001982141561330b5761330b6132e1565b5060010190565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b8060005b60028110156129d457815484526020909301926001918201910161333f565b80548252600181015460208301526002810154604083015260038101546060830152613390608083016004830161333b565b6122c960c083016006830161333b565b60008151606084526133b560608501826129da565b905060208301516020850152604083015160408501528091505092915050565b60006101608083526133ea818401888a613312565b905082810360208401526133fe8187612f96565b905061340d604084018661335e565b82810361014084015261342081856133a0565b98975050505050505050565b6000821982111561343f5761343f6132e1565b500190565b60006020828403121561345657600080fd5b8135610ec781612c3e565b600081518084526020808501945080840160005b83811015612aff578151805160ff1688528301516001600160a01b03168388015260409096019590820190600101613475565b60018060a01b038616815284602082015260a0604082015260006134cf60a0830186613461565b606083019490945250608001529392505050565b6000806000606084860312156134f857600080fd5b835160ff8116811461350957600080fd5b602085015160409095015190969495509392505050565b600082821015613532576135326132e1565b500390565b60006101a080835261354b8184018a6129da565b9050828103602084015261355f8189612f96565b905061356e604084018861335e565b8281036101408401526135818187613461565b905082810361016084015261359681866133a0565b90508281036101808401526135ab81856133a0565b9998505050505050505050565b6001600160e01b03198316815281516000906135db8160048501602087016129aa565b919091016004019392505050565b600082516135fb8184602087016129aa565b9190910192915050565b602081526000612735602083018486613312565b600080845461362781612f5b565b6001828116801561363f57600181146136505761367f565b60ff1984168752828701945061367f565b8860005260208060002060005b858110156136765781548a82015290840190820161365d565b50505082870194505b50601760f91b84528651925061369b8382860160208a016129aa565b919092010195945050505050565b6040815260006136bc60408301856129da565b8281036020840152612b5f81856129da565b6040815260116040820152702d207374616e6461726451756f72756d7360781b6060820152608060208201526000610ec760808301846129da565b60408152600e60408201526d2d20636875726e51756f72756d7360901b6060820152608060208201526000610ec760808301846129da565b60006020828403121561375357600080fd5b81516001600160401b0381111561376957600080fd5b8201601f8101841361377a57600080fd5b8051613788612923826128ad565b81815285602083850101111561379d57600080fd5b612b5f8260208301602086016129aa565b600083516137c08184602088016129aa565b8351908301906137d48183602088016129aa565b01949350505050565b600083516137ef8184602088016129aa565b8351908301906138038183602088016129aa565b61016160f51b9101908152600201949350505050565b6000825161382b8184602087016129aa565b605d60f81b920191825250600101919050565b60408152600e60408201526d2d20636875726e5461726765747360901b6060820152608060208201526000610ec760808301846129da565b60408152600560408201526422b93937b960d91b6060820152608060208201526000610ec760808301846129da565b600061ffff808316818114156138bd576138bd6132e1565b6001019392505050565b634e487b7160e01b600052601260045260246000fd5b6000826138ec576138ec6138c7565b500490565b600082613900576139006138c7565b50069056fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50557365722e72656769737465724f70657261746f7257697468436875726e3a20696e707574206c656e677468206d69736d61746368557365722e72656769737465724f70657261746f7257697468436875726e3a20696e7075742071756f72756d73206861766520636f6d6d6f6e2062697473280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583a26469706673582212205412fccab6a31a52e0c0507762d4453fac4ba8d9b9d0ba236dc54acba88d3ec464736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x07\x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U`\x0B\x80T\x90\x91\x16\x90\x91\x17\x90U`\x1C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x17\x90U`\0`4U4\x80\x15b\0\0ZW`\0\x80\xFD[P`@Qb\0F\xF48\x03\x80b\0F\xF4\x839\x81\x01`@\x81\x90Rb\0\0}\x91b\0\x0BbV[`\x003\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16cm\x14\xA9\x87`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xE7\x91\x90b\0\x0CYV[` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x81U`@\x80Qc5\x9DS\x97`\xE1\x1B\x81R\x90Q\x92\x84\x16\x92ck:\xA7.\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x01@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01f\x91\x90b\0\x0CYV[`\x1F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U` \x80T`@\x80Qc9\x98\xFD\xD3`\xE0\x1B\x81R\x90Q\x91\x90\x93\x16\x92c9\x98\xFD\xD3\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x01\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xE9\x91\x90b\0\x0CYV[`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U` \x80T`@\x80Qc.\xFA,\xA3`\xE1\x1B\x81R\x90Q\x91\x90\x93\x16\x92c]\xF4YF\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x02FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02l\x91\x90b\0\x0CYV[`\"\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U` \x80T`@\x80Qch0H5`\xE0\x1B\x81R\x90Q\x91\x90\x93\x16\x92ch0H5\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x02\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02\xEF\x91\x90b\0\x0CYV[`#\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U` \x80T`@\x80QcOL\x91\xE1`\xE1\x1B\x81R\x90Q\x91\x90\x93\x16\x92c\x9E\x99#\xC2\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x03LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x03r\x91\x90b\0\x0CYV[`$\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`#T`@\x80Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90Q\x91\x90\x92\x16\x91c\xDF\\\xF7#\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x03\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x03\xF7\x91\x90b\0\x0CYV[`\x1D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80Qc\x076\xE1\xC7`\xE3\x1B\x81R\x90Qc9\xB7\x0E8\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x04RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x04x\x91\x90b\0\x0CYV[`\x1E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`!T`@\x80Qc5\x9DS\x97`\xE1\x1B\x81R\x90Q\x91\x90\x92\x16\x91ck:\xA7.\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x04\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x04\xFD\x91\x90b\0\x0CYV[`\x1F`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c=\xFB@\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05bW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x05\x88\x91\x90b\0\x0CYV[`%`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c-\xBC\xB0L`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06\x13\x91\x90b\0\x0C\x80V[`&\x81\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x05C\x10\xE6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06~\x91\x90b\0\x0CYV[`'\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x83Qb\0\x06\xB3\x90`(\x90` \x87\x01\x90b\0\x08\xA2V[P`*\x83\x90U\x81Q\x80Q`+\x90\x81U` \x91\x82\x01Q`,U\x81\x84\x01Q\x80Q`-U\x90\x91\x01Q`.U`@\x83\x01Q\x80Q\x84\x92\x91\x90`/\x90b\0\x06\xF8\x90\x82\x90`\x02b\0\t1V[P` \x82\x01Qb\0\x07\x10\x90`\x02\x80\x84\x01\x91\x90b\0\t1V[PP` T`@Qc\x0F\n\x9F\xD3`\xE2\x1B\x81R0`\x04\x82\x01R`\0\x94P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc<*\x7FL\x91P`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x07`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x07\x86\x91\x90b\0\x0C\x9AV[\x90Pb\0\x07\xA4`*T\x82b\0\x07\xEB` \x1Bb\0\x1B\xCD\x17\x90\x91\x90` \x1CV[\x80Q`+U` \x90\x81\x01Q`,U`@\x80Q\x80\x82\x01\x90\x91R`-T\x81R`.T\x81\x83\x01Rb\0\x07\xDC\x91b\0\x1Cdb\0\x08\x8B\x82\x1B\x17\x90\x1CV[`)UPb\0\x0C\xF6\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Rb\0\x08\tb\0\taV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15b\0\x08>Wb\0\x08@V[\xFE[P\x80b\0\x08\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x82\x80Tb\0\x08\xB0\x90b\0\x0C\xB9V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0\x08\xD4W`\0\x85Ub\0\t\x1FV[\x82`\x1F\x10b\0\x08\xEFW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0\t\x1FV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0\t\x1FW\x91\x82\x01[\x82\x81\x11\x15b\0\t\x1FW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\t\x02V[Pb\0\t-\x92\x91Pb\0\t\x7FV[P\x90V[\x82`\x02\x81\x01\x92\x82\x15b\0\t\x1FW\x91` \x02\x82\x01\x82\x81\x11\x15b\0\t\x1FW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\t\x02V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15b\0\t-W`\0\x81U`\x01\x01b\0\t\x80V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\t\xD1Wb\0\t\xD1b\0\t\x96V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\n\x02Wb\0\n\x02b\0\t\x96V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15b\0\n\x1DW`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\nBWb\0\nBb\0\t\x96V[`@R\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12b\0\nnW`\0\x80\xFD[b\0\nxb\0\t\xACV[\x80`@\x84\x01\x85\x81\x11\x15b\0\n\x8BW`\0\x80\xFD[\x84[\x81\x81\x10\x15b\0\n\xA7W\x80Q\x84R` \x93\x84\x01\x93\x01b\0\n\x8DV[P\x90\x95\x94PPPPPV[`\0\x81\x83\x03a\x01\0\x81\x12\x15b\0\n\xC7W`\0\x80\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\n\xECWb\0\n\xECb\0\t\x96V[`@R\x91P\x81b\0\n\xFE\x85\x85b\0\n\nV[\x81Rb\0\x0B\x0F\x85`@\x86\x01b\0\n\nV[` \x82\x01R`\x80`\x7F\x19\x83\x01\x12\x15b\0\x0B'W`\0\x80\xFD[b\0\x0B1b\0\t\xACV[\x91Pb\0\x0BB\x85`\x80\x86\x01b\0\n\\V[\x82Rb\0\x0BS\x85`\xC0\x86\x01b\0\n\\V[` \x83\x01R`@\x01R\x92\x91PPV[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15b\0\x0ByW`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x0B\x91W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12b\0\x0B\xA6W`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x0B\xBBWb\0\x0B\xBBb\0\t\x96V[` \x91Pb\0\x0B\xD3`\x1F\x82\x01`\x1F\x19\x16\x83\x01b\0\t\xD7V[\x81\x81R\x88\x83\x83\x86\x01\x01\x11\x15b\0\x0B\xE8W`\0\x80\xFD[`\0[\x82\x81\x10\x15b\0\x0C\x08W\x84\x81\x01\x84\x01Q\x82\x82\x01\x85\x01R\x83\x01b\0\x0B\xEBV[\x82\x81\x11\x15b\0\x0C\x1AW`\0\x84\x84\x84\x01\x01R[P\x80\x96PPP\x80\x86\x01Q\x93PPPb\0\x0C7\x85`@\x86\x01b\0\n\xB2V[\x90P\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x0CVW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15b\0\x0ClW`\0\x80\xFD[\x81Qb\0\x0Cy\x81b\0\x0C@V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15b\0\x0C\x93W`\0\x80\xFD[PQ\x91\x90PV[`\0`@\x82\x84\x03\x12\x15b\0\x0C\xADW`\0\x80\xFD[b\0\x0Cy\x83\x83b\0\n\nV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x0C\xCEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15b\0\x0C\xF0WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[a9\xEE\x80b\0\r\x06`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\xB8W\x80c\xB5P\x8A\xA9\x11a\0|W\x80c\xB5P\x8A\xA9\x14a\x02\xA6W\x80c\xBAAO\xA6\x14a\x02\xAEW\x80c\xBFh\xB8\x16\x14a\x02\xC6W\x80c\xCAO-\x97\x14a\x02\xCFW\x80c\xE2\x0C\x9Fq\x14a\x02\xE2W\x80c\xFAv&\xD4\x14a\x02\xEAW`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x02#W\x80c\x91j\x17\xC6\x14a\x028W\x80c\xA3\xF4\xDF~\x14a\x02@W\x80c\xA5\xF6\xCC\x1A\x14a\x02UW\x80c\xAF\xA1\xC77\x14a\x02hW`\0\x80\xFD[\x80c?r\x86\xF4\x11a\x01\nW\x80c?r\x86\xF4\x14a\x01\xB4W\x80cPSw\xE2\x14a\x01\xBCW\x80ce\xED\xA8\xE5\x14a\x01\xC4W\x80cf\xD9\xA9\xA0\x14a\x01\xDAW\x80cm3oX\x14a\x01\xEFW\x80c\x821\xB5L\x14a\x02\x02W`\0\x80\xFD[\x80c\x16&\xBA~\x14a\x01GW\x80c\x1E\xD7\x83\x1C\x14a\x01xW\x80c*4\xAD\xE8\x14a\x01\x8DW\x80c*\xDE8\x80\x14a\x01\x97W\x80c>^<#\x14a\x01\xACW[`\0\x80\xFD[a\x01Za\x01U6`\x04a(\xD4V[a\x02\xF7V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x80a\x03&V[`@Qa\x01o\x91\x90a)]V[a\x01\x95a\x03\x88V[\0[a\x01\x9Fa\x04\xC1V[`@Qa\x01o\x91\x90a*\x06V[a\x01\x80a\x06\x03V[a\x01\x80a\x06cV[a\x01\x95a\x06\xC3V[a\x01\xCCa\x07\xEEV[`@Qa\x01o\x92\x91\x90a+:V[a\x01\xE2a\n)V[`@Qa\x01o\x91\x90a+hV[a\x01\x95a\x01\xFD6`\x04a,\xB9V[a\x0B\x0FV[a\x02\x15a\x02\x106`\x04a-\xC2V[a\r\x88V[`@Q\x90\x81R` \x01a\x01oV[a\x02+a\x0E\xCEV[`@Qa\x01o\x91\x90a.\x03V[a\x01\xE2a\x0F\x9EV[a\x02Ha\x10\x84V[`@Qa\x01o\x91\x90a.eV[a\x01\x95a\x02c6`\x04a.xV[a\x11\x12V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`-T\x80\x82R`.T\x91\x83\x01\x91\x82R\x83Q\x90\x81R\x90Q\x91\x81\x01\x91\x90\x91R\x01a\x01oV[a\x02+a\x18YV[a\x02\xB6a\x19)V[`@Q\x90\x15\x15\x81R` \x01a\x01oV[a\x02\x15`)T\x81V[a\x01\x95a\x02\xDD6`\x04a-\xC2V[a\x1AVV[a\x01\x80a\x1BmV[`\x07Ta\x02\xB6\x90`\xFF\x16\x81V[`\0\x82\x81R`3` R`@\x81 T`\xFF\x16\x15a\x03\x1CWPc\x0B\x13]?`\xE1\x1Ba\x03 V[P`\0[\x92\x91PPV[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`W[PPPPP\x90P\x90V[`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x01\x91\x90a/BV[Pa\x04@`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FregisterAsOperator (core)\0\0\0\0\0\0\0\x81RPa\x1C{V[`@\x80Q``\x81\x01\x82R0\x81R`\0` \x82\x01\x81\x90R\x81\x83\x01R`\x1DT\x91Qc\x0FX\x9EY`\xE0\x1B\x81R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x0FX\x9EY\x90a\x04\x8C\x90\x84\x90`(\x90`\x04\x01a0\rV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xBAW=`\0\x80>=`\0\xFD[PPPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFAW`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x05\xE3W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x05V\x90a/[V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x82\x90a/[V[\x80\x15a\x05\xCFW\x80`\x1F\x10a\x05\xA4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xCFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xB2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x057V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04\xE5V[PPPP\x90P\x90V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07<\x91\x90a/BV[Pa\x07{`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7FupdateStakes (updateOperators)\0\0\x81RPa\x1C{V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P0\x81`\0\x81Q\x81\x10a\x07\xB1Wa\x07\xB1a0MV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91RT`@Qb\xCF*\xB5`\xE0\x1B\x81R\x91\x16\x90b\xCF*\xB5\x90a\x04\x8C\x90\x84\x90`\x04\x01a)]V[``\x80`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08j\x91\x90a/BV[Pa\x08\xA1`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01texitEigenlayer (core)`X\x1B\x81RPa\x1C{V[`\x1DT`@Qcg\xC0C\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCF\x80\x87>\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\x16\x91\x90\x81\x01\x90a0\xBEV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90\x92P`\0\x91\x90\x81` \x01[`@\x80Q``\x80\x82\x01\x83R\x80\x82R` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t4W\x90PP\x90P`@Q\x80``\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x010`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x81`\0\x81Q\x81\x10a\t\x9CWa\t\x9Ca0MV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x1DT`@Qc\x06\xECn\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\r\xD8\xDD\x02\x90a\t\xD7\x90\x84\x90`\x04\x01a1xV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\x1E\x91\x90\x81\x01\x90a2\x12V[P\x91\x93P\x91PP\x90\x91V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFAW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\n\xF7W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\n\xB9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\nMV[`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0BdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x88\x91\x90a/BV[Pa\x0B\xC7`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FdepositIntoEigenLayer (core)\0\0\0\0\x81RPa\x1C{V[`\0[\x82Q\x81\x10\x15a\r\x83W`\0\x83\x82\x81Q\x81\x10a\x0B\xE7Wa\x0B\xE7a0MV[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x0C\x05Wa\x0C\x05a0MV[` \x02` \x01\x01Q\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0COW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cs\x91\x90a2\xA2V[`\x1ET`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xEC\x91\x90a2\xBFV[P`\x1ET`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\rHW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rl\x91\x90a/BV[PPPP\x80\x80a\r{\x90a2\xF7V[\x91PPa\x0B\xCAV[PPPV[`%T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x87\x87Z\xF1\x15\x80\x15a\r\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF7\x91\x90a/BV[Pa\x0E+`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o92\xB3\xB4\xB9\xBA2\xB9'\xB82\xB90\xBA7\xB9`\x81\x1B\x81RP\x84\x84a\x1C\xC3V[` T`\x01`\x01`\xA0\x1B\x03\x16c\xA5\x08W\xBF\x84\x84`(`+a\x0EJa\x1DHV[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ej\x95\x94\x93\x92\x91\x90a3\xD5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x84W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x98W=`\0\x80>=`\0\xFD[PP`@\x80Q\x80\x82\x01\x82R`-T\x80\x82R`.T` \x92\x83\x01\x90\x81R`\0\x91\x82RQ\x90\x91R \x91Pa\x0E\xC7\x90PV[\x93\x92PPPV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFAW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x0F\x11\x90a/[V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F=\x90a/[V[\x80\x15a\x0F\x8AW\x80`\x1F\x10a\x0F_Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\x8AV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0FmW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0E\xF2V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFAW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x10lW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x10.W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0F\xC2V[`(\x80Ta\x10\x91\x90a/[V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xBD\x90a/[V[\x80\x15a\x11\nW\x80`\x1F\x10a\x10\xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\nV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xEDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x8B\x91\x90a/BV[Pa\x12f`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FregisterOperatorWithChurn\0\0\0\0\0\0\0\x81RP\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` \x80\x8B\x02\x82\x81\x01\x82\x01\x90\x93R\x8A\x82R\x90\x93P\x8A\x92P\x89\x91\x82\x91\x85\x01\x90\x84\x90\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1EP\x92PPPV[`\0a\x12\xA7\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa \xD3\x92PPPV[\x90P`\0a\x12\xEA\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa \xD3\x92PPPV[`@\x80Q``\x81\x01\x90\x91R`5\x80\x82R\x91\x92Pa\x13\x12\x91\x89\x91\x88\x91a9&` \x83\x019a\"`V[a\x13@\x81\x83\x16`\x01`\x01`\xC0\x1B\x03\x16\x15`@Q\x80``\x01`@R\x80`>\x81R` \x01a9[`>\x919a\"\x96V[`\0a\x13Y`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17a\"\xCDV[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13vWa\x13va(gV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\xBBW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\x94W\x90P[P\x90P`\0\x80[\x83Qa\x13\xCE\x82\x84a4,V[\x10\x15a\x15\xE9W\x81\x8B\x14\x15a\x14)W`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x83a\x13\xFB\x83\x85a4,V[\x81Q\x81\x10a\x14\x0BWa\x14\x0Ba0MV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x14!\x90a2\xF7V[\x91PPa\x13\xC2V[\x80\x87\x14\x80a\x14|WP\x87\x87\x82\x81\x81\x10a\x14DWa\x14Da0MV[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P\x8C\x8C\x84\x81\x81\x10a\x14fWa\x14fa0MV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x10[\x15a\x15\x17W`@Q\x80`@\x01`@R\x80\x8D\x8D\x85\x81\x81\x10a\x14\x9EWa\x14\x9Ea0MV[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x01\x8B\x8B\x85\x81\x81\x10a\x14\xBEWa\x14\xBEa0MV[\x90P` \x02\x01` \x81\x01\x90a\x14\xD3\x91\x90a4DV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x83a\x14\xE9\x83\x85a4,V[\x81Q\x81\x10a\x14\xF9Wa\x14\xF9a0MV[` \x02` \x01\x01\x81\x90RP\x81\x80a\x15\x0F\x90a2\xF7V[\x92PPa\x13\xC2V[\x8B\x8B\x83\x81\x81\x10a\x15)Wa\x15)a0MV[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P\x88\x88\x83\x81\x81\x10a\x15KWa\x15Ka0MV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x10\x15a\x15\x84W`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x83a\x13\xFB\x83\x85a4,V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FUser.registerOperatorWithChurn: `D\x82\x01Rn\x1BX[\x19\x9B\xDC\x9BYY\x08\x1A[\x9C\x1D]`\x8A\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0`4`\0\x81Ta\x15\xFA\x90a2\xF7V[\x91\x82\x90UP`@\x80Q` \x81\x01\x92\x90\x92Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16\x90\x82\x01R`T\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x90T`)Tc\x84\xCAR\x13`\xE0\x1B\x84R\x91\x93P`\0\x19\x92`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x84\xCAR\x13\x91a\x16}\x910\x91\x8B\x90\x89\x90\x89\x90`\x04\x01a4\xA8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xBE\x91\x90a/BV[`\x1CT`&T`@Qc8\xD0z\xA9`\xE2\x1B\x81R\x92\x93P`\0\x92\x83\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xE3A\xEA\xA4\x91a\x17\x04\x91\x88\x90`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17E\x91\x90a4\xE3V[`@\x80Q`A\x80\x82R`\x80\x82\x01\x90\x92R\x93\x96P\x91\x94P\x92P`\0\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x82` \x82\x01R\x81`@\x82\x01R\x83`\xF8\x1B\x81`\x01\x83Qa\x17\x8F\x91\x90a5 V[\x81Q\x81\x10a\x17\x9FWa\x17\x9Fa0MV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`@\x80Q``\x81\x01\x82R\x82\x81R` \x80\x82\x01\x8A\x90R\x91\x81\x01\x88\x90R\x90T`\x01`\x01`\xA0\x1B\x03\x16c\x9B]\x17{\x8D`(`+\x8F\x86a\x17\xF0a\x1DHV[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x11\x96\x95\x94\x93\x92\x91\x90a57V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18+W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18?W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPPPPPPPPPPV[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFAW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x18\x9C\x90a/[V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18\xC8\x90a/[V[\x80\x15a\x19\x15W\x80`\x1F\x10a\x18\xEAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x19\x15V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18\xF8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x18}V[`\x07T`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a\x19KWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1AQW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x19\xD9\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a5\xB8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x19\xF3\x91a5\xE9V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1A0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1A5V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x1AM\x91\x90a2\xBFV[\x91PP[\x91\x90PV[`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1A\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xCF\x91\x90a/BV[Pa\x1B\x05`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q22\xB92\xB3\xB4\xB9\xBA2\xB9'\xB82\xB90\xBA7\xB9`q\x1B\x81RP\x83\x83a\x1C\xC3V[` T`@Qc\xCAO-\x97`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCAO-\x97\x90a\x1B7\x90\x85\x90\x85\x90`\x04\x01a6\x05V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1BQW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1BeW=`\0\x80>=`\0\xFD[PPPPPPV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x1B\xE9a(IV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x1C\x1CWa\x1C\x1EV[\xFE[P\x80a\x1C\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x15\xE0V[PP\x92\x91PPV[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[`\0\x80Q` a9\x06\x839\x81Q\x91R`(\x82`@Q` \x01a\x1C\x9E\x92\x91\x90a6\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1C\xB8\x91a.eV[`@Q\x80\x91\x03\x90\xA1PV[`\0\x80Q` a9\x99\x839\x81Q\x91R`(\x84`@Q` \x01a\x1C\xE6\x92\x91\x90a6\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` `\x1F\x86\x01\x81\x90\x04\x81\x02\x84\x01\x81\x01\x90\x92R\x84\x83R\x91a\x1D-\x91\x86\x90\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa#\x99\x92PPPV[`@Qa\x1D;\x92\x91\x90a6\xA9V[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x80\x82\x01\x83R\x80\x82R`\0` \x83\x01\x81\x90R\x82\x84\x01\x81\x90R\x83Q\x91\x82\x01\x81\x81R`\x80\x83\x01\x90\x94R\x91\x92\x81\x90\x81R` \x01`4`\0\x81T\x80\x92\x91\x90a\x1D\x90\x90a2\xF7V[\x90\x91UP\x81R`\0\x19` \x91\x82\x01R`\x1FT`!T\x91\x83\x01Q`@\x80\x85\x01Q\x90Qc\x14 \xC1\x91`\xE3\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`$\x82\x01R`D\x81\x01\x92\x90\x92R`d\x82\x01R\x92\x93P`\0\x92\x91\x16\x90c\xA1\x06\x0C\x88\x90`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E1\x91\x90a/BV[`\0\x90\x81R`3` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UP\x91\x90PV[`\0\x80Q` a9\x06\x839\x81Q\x91R`(\x85`@Q` \x01a\x1Es\x92\x91\x90a6\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\x8D\x91a.eV[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a9\x99\x839\x81Q\x91Ra\x1E\xAD\x82a#\x99V[`@Qa\x1E\xBA\x91\x90a6\xCEV[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a9\x99\x839\x81Q\x91Ra\x1E\xDA\x84a#\x99V[`@Qa\x1E\xE7\x91\x90a7\tV[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`[`\xF8\x1B` \x82\x01R`\0[\x83Q\x81\x10\x15a \x84W`\x01\x84Qa\x1F!\x91\x90a5 V[\x81\x14\x15a\x1F\xCFW\x81\x84\x82\x81Q\x81\x10a\x1F;Wa\x1F;a0MV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\xA8\x91\x90\x81\x01\x90a7AV[`@Q` \x01a\x1F\xB9\x92\x91\x90a7\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa rV[\x81\x84\x82\x81Q\x81\x10a\x1F\xE2Wa\x1F\xE2a0MV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a 'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra O\x91\x90\x81\x01\x90a7AV[`@Q` \x01a `\x92\x91\x90a7\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P[\x80a |\x81a2\xF7V[\x91PPa\x1F\nV[P\x80`@Q` \x01a \x96\x91\x90a8\x19V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80Q` a9\x99\x839\x81Q\x91R\x81`@Qa \xC4\x91\x90a8>V[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\0a\x01\0\x82Q\x11\x15a!\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x15\xE0V[\x81Qa!jWP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a!\x80Wa!\x80a0MV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a\"WW\x84\x81\x81Q\x81\x10a!\xAEWa!\xAEa0MV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a\"CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x15\xE0V[\x91\x81\x17\x91a\"P\x81a2\xF7V[\x90Pa!\x93V[P\x90\x93\x92PPPV[\x81\x83\x14a\r\x83W`\0\x80Q` a9\x99\x839\x81Q\x91R\x81`@Qa\"\x84\x91\x90a8vV[`@Q\x80\x91\x03\x90\xA1a\r\x83\x83\x83a$\x93V[\x81a\"\xC9W`\0\x80Q` a9\x99\x839\x81Q\x91R\x81`@Qa\"\xB8\x91\x90a8vV[`@Q\x80\x91\x03\x90\xA1a\"\xC9\x82a%\xA8V[PPV[```\0\x80a\"\xDB\x84a&\rV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xF6Wa\"\xF6a(gV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a# W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a#8WPa\x01\0\x81\x10[\x15a#\x8FW`\x01\x81\x1B\x93P\x85\x84\x16\x15a#\x7FW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a#aWa#aa0MV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a#\x88\x81a2\xF7V[\x90Pa#'V[P\x90\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`[`\xF8\x1B` \x82\x01R``\x90`\0[\x83Q\x81\x10\x15a$jW`\x01\x84Qa#\xCE\x91\x90a5 V[\x81\x14\x15a$\x1FW\x81a#\xF8\x85\x83\x81Q\x81\x10a#\xEBWa#\xEBa0MV[\x01` \x01Q`\xF8\x1Ca&8V[`@Q` \x01a$\t\x92\x91\x90a7\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa$XV[\x81a$5\x85\x83\x81Q\x81\x10a#\xEBWa#\xEBa0MV[`@Q` \x01a$F\x92\x91\x90a7\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P[\x80a$b\x81a2\xF7V[\x91PPa#\xB7V[P\x80`@Q` \x01a$|\x91\x90a8\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[\x80\x82\x14a\"\xC9W`\0\x80Q` a9\x06\x839\x81Q\x91R`@Qa$\xF2\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x08\x13\x19Y\x9D`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x14\x9AY\xDA\x1D`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a\"\xC9a'=V[\x80a&\nW`\0\x80Q` a9\x06\x839\x81Q\x91R`@Qa%\xFA\x90` \x80\x82R`\x17\x90\x82\x01R\x7FError: Assertion Failed\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1a&\na'=V[PV[`\0\x80[\x82\x15a\x03 Wa&\"`\x01\x84a5 V[\x90\x92\x16\x91\x80a&0\x81a8\xA5V[\x91PPa&\x11V[``\x81a&\\WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[\x81\x15a&\x86W\x80a&p\x81a2\xF7V[\x91Pa&\x7F\x90P`\n\x83a8\xDDV[\x91Pa&`V[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xA0Wa&\xA0a(gV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a&\xCAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a'5Wa&\xDF`\x01\x83a5 V[\x91Pa&\xEC`\n\x86a8\xF1V[a&\xF7\x90`0a4,V[`\xF8\x1B\x81\x83\x81Q\x81\x10a'\x0CWa'\x0Ca0MV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SPa'.`\n\x86a8\xDDV[\x94Pa&\xCEV[\x94\x93PPPPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a(8W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra'\xD7\x92\x91` \x01a5\xB8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra'\xF1\x91a5\xE9V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a(.W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a(3V[``\x91P[PPPP[`\x07\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a(\xA5Wa(\xA5a(gV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a(\xC6Wa(\xC6a(gV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a(\xE7W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)\x04W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a)\x15W`\0\x80\xFD[\x805a)(a)#\x82a(\xADV[a(}V[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a)=W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a)\x9EW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a)yV[P\x90\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15a)\xC5W\x81\x81\x01Q\x83\x82\x01R` \x01a)\xADV[\x83\x81\x11\x15a)\xD4W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra)\xF2\x81` \x86\x01` \x86\x01a)\xAAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a*\xB6W`?\x19\x8B\x85\x03\x01\x87R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a*\xA0W`_\x19\x89\x85\x03\x01\x83Ra*\x8E\x84\x86Qa)\xDAV[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a*rV[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a*-V[P\x91\x9A\x99PPPPPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\xFFW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a*\xDAV[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\xFFW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a+\x1EV[`@\x81R`\0a+M`@\x83\x01\x85a*\xC6V[\x82\x81\x03` \x84\x01Ra+_\x81\x85a+\nV[\x95\x94PPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a,\x0CW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a+\xF7W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a+\xCDV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a+\x90V[P\x91\x99\x98PPPPPPPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a,4Wa,4a(gV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a&\nW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a,dW`\0\x80\xFD[\x815` a,ta)#\x83a,\x1BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a,\x93W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a,\xAEW\x805\x83R\x91\x83\x01\x91\x83\x01a,\x97V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a,\xCCW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a,\xE3W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a,\xF7W`\0\x80\xFD[\x815` a-\x07a)#\x83a,\x1BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a-&W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a-MW\x855a->\x81a,>V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a-+V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a-cW`\0\x80\xFD[Pa-p\x85\x82\x86\x01a,SV[\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a-\x8CW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xA3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a-\xBBW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a-\xD5W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xEBW`\0\x80\xFD[a-\xF7\x85\x82\x86\x01a-zV[\x90\x96\x90\x95P\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a.XW`?\x19\x88\x86\x03\x01\x84Ra.F\x85\x83Qa)\xDAV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a.*V[P\x92\x97\x96PPPPPPPV[` \x81R`\0a\x0E\xC7` \x83\x01\x84a)\xDAV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a.\x91W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a.\xA8W`\0\x80\xFD[a.\xB4\x8A\x83\x8B\x01a-zV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a.\xCDW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a.\xE1W`\0\x80\xFD[\x815\x81\x81\x11\x15a.\xF0W`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15a/\x05W`\0\x80\xFD[` \x83\x01\x96P\x80\x95PP`@\x89\x015\x91P\x80\x82\x11\x15a/#W`\0\x80\xFD[Pa/0\x89\x82\x8A\x01a-zV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0` \x82\x84\x03\x12\x15a/TW`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a/oW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a/\x90WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x81Ta/\xA3\x81a/[V[\x80\x85R` `\x01\x83\x81\x16\x80\x15a/\xC0W`\x01\x81\x14a/\xD4Wa0\x02V[`\xFF\x19\x85\x16\x88\x84\x01R`@\x88\x01\x95Pa0\x02V[\x86`\0R\x82`\0 `\0[\x85\x81\x10\x15a/\xFAW\x81T\x8A\x82\x01\x86\x01R\x90\x83\x01\x90\x84\x01a/\xDFV[\x89\x01\x84\x01\x96PP[PPPPP\x92\x91PPV[`\0`\x01\x80`\xA0\x1B\x03\x80\x85Q\x16\x83R\x80` \x86\x01Q\x16` \x84\x01RPc\xFF\xFF\xFF\xFF`@\x85\x01Q\x16`@\x83\x01R`\x80``\x83\x01Ra'5`\x80\x83\x01\x84a/\x96V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a0tW`\0\x80\xFD[\x81Q` a0\x84a)#\x83a,\x1BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a0\xA3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a,\xAEW\x80Q\x83R\x91\x83\x01\x91\x83\x01a0\xA7V[`\0\x80`@\x83\x85\x03\x12\x15a0\xD1W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a0\xE8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a0\xFCW`\0\x80\xFD[\x81Q` a1\x0Ca)#\x83a,\x1BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a1+W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a1RW\x85Qa1C\x81a,>V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a10V[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15a1kW`\0\x80\xFD[Pa-p\x85\x82\x86\x01a0cV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a2\x04W`?\x19\x89\x84\x03\x01\x85R\x81Q``\x81Q\x81\x86Ra1\xC5\x82\x87\x01\x82a*\xC6V[\x91PP\x88\x82\x01Q\x85\x82\x03\x8A\x87\x01Ra1\xDD\x82\x82a+\nV[\x92\x89\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x95\x89\x01\x95\x90\x95RP\x94\x87\x01\x94\x92P\x90\x86\x01\x90`\x01\x01a1\x9FV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a2%W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a2;W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a2LW`\0\x80\xFD[\x80Qa2Za)#\x82a,\x1BV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a2yW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a2\x97W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a2~V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a2\xB4W`\0\x80\xFD[\x81Qa\x0E\xC7\x81a,>V[`\0` \x82\x84\x03\x12\x15a2\xD1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0E\xC7W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a3\x0BWa3\x0Ba2\xE1V[P`\x01\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x80`\0[`\x02\x81\x10\x15a)\xD4W\x81T\x84R` \x90\x93\x01\x92`\x01\x91\x82\x01\x91\x01a3?V[\x80T\x82R`\x01\x81\x01T` \x83\x01R`\x02\x81\x01T`@\x83\x01R`\x03\x81\x01T``\x83\x01Ra3\x90`\x80\x83\x01`\x04\x83\x01a3;V[a\"\xC9`\xC0\x83\x01`\x06\x83\x01a3;V[`\0\x81Q``\x84Ra3\xB5``\x85\x01\x82a)\xDAV[\x90P` \x83\x01Q` \x85\x01R`@\x83\x01Q`@\x85\x01R\x80\x91PP\x92\x91PPV[`\0a\x01`\x80\x83Ra3\xEA\x81\x84\x01\x88\x8Aa3\x12V[\x90P\x82\x81\x03` \x84\x01Ra3\xFE\x81\x87a/\x96V[\x90Pa4\r`@\x84\x01\x86a3^V[\x82\x81\x03a\x01@\x84\x01Ra4 \x81\x85a3\xA0V[\x98\x97PPPPPPPPV[`\0\x82\x19\x82\x11\x15a4?Wa4?a2\xE1V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a4VW`\0\x80\xFD[\x815a\x0E\xC7\x81a,>V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\xFFW\x81Q\x80Q`\xFF\x16\x88R\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a4uV[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0a4\xCF`\xA0\x83\x01\x86a4aV[``\x83\x01\x94\x90\x94RP`\x80\x01R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a4\xF8W`\0\x80\xFD[\x83Q`\xFF\x81\x16\x81\x14a5\tW`\0\x80\xFD[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[`\0\x82\x82\x10\x15a52Wa52a2\xE1V[P\x03\x90V[`\0a\x01\xA0\x80\x83Ra5K\x81\x84\x01\x8Aa)\xDAV[\x90P\x82\x81\x03` \x84\x01Ra5_\x81\x89a/\x96V[\x90Pa5n`@\x84\x01\x88a3^V[\x82\x81\x03a\x01@\x84\x01Ra5\x81\x81\x87a4aV[\x90P\x82\x81\x03a\x01`\x84\x01Ra5\x96\x81\x86a3\xA0V[\x90P\x82\x81\x03a\x01\x80\x84\x01Ra5\xAB\x81\x85a3\xA0V[\x99\x98PPPPPPPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a5\xDB\x81`\x04\x85\x01` \x87\x01a)\xAAV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa5\xFB\x81\x84` \x87\x01a)\xAAV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a'5` \x83\x01\x84\x86a3\x12V[`\0\x80\x84Ta6'\x81a/[V[`\x01\x82\x81\x16\x80\x15a6?W`\x01\x81\x14a6PWa6\x7FV[`\xFF\x19\x84\x16\x87R\x82\x87\x01\x94Pa6\x7FV[\x88`\0R` \x80`\0 `\0[\x85\x81\x10\x15a6vW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a6]V[PPP\x82\x87\x01\x94P[P`\x17`\xF9\x1B\x84R\x86Q\x92Pa6\x9B\x83\x82\x86\x01` \x8A\x01a)\xAAV[\x91\x90\x92\x01\x01\x95\x94PPPPPV[`@\x81R`\0a6\xBC`@\x83\x01\x85a)\xDAV[\x82\x81\x03` \x84\x01Ra+_\x81\x85a)\xDAV[`@\x81R`\x11`@\x82\x01Rp- standardQuorums`x\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x0E\xC7`\x80\x83\x01\x84a)\xDAV[`@\x81R`\x0E`@\x82\x01Rm- churnQuorums`\x90\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x0E\xC7`\x80\x83\x01\x84a)\xDAV[`\0` \x82\x84\x03\x12\x15a7SW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a7iW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a7zW`\0\x80\xFD[\x80Qa7\x88a)#\x82a(\xADV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a7\x9DW`\0\x80\xFD[a+_\x82` \x83\x01` \x86\x01a)\xAAV[`\0\x83Qa7\xC0\x81\x84` \x88\x01a)\xAAV[\x83Q\x90\x83\x01\x90a7\xD4\x81\x83` \x88\x01a)\xAAV[\x01\x94\x93PPPPV[`\0\x83Qa7\xEF\x81\x84` \x88\x01a)\xAAV[\x83Q\x90\x83\x01\x90a8\x03\x81\x83` \x88\x01a)\xAAV[a\x01a`\xF5\x1B\x91\x01\x90\x81R`\x02\x01\x94\x93PPPPV[`\0\x82Qa8+\x81\x84` \x87\x01a)\xAAV[`]`\xF8\x1B\x92\x01\x91\x82RP`\x01\x01\x91\x90PV[`@\x81R`\x0E`@\x82\x01Rm- churnTargets`\x90\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x0E\xC7`\x80\x83\x01\x84a)\xDAV[`@\x81R`\x05`@\x82\x01Rd\"\xB997\xB9`\xD9\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x0E\xC7`\x80\x83\x01\x84a)\xDAV[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a8\xBDWa8\xBDa2\xE1V[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a8\xECWa8\xECa8\xC7V[P\x04\x90V[`\0\x82a9\0Wa9\0a8\xC7V[P\x06\x90V\xFEA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FPUser.registerOperatorWithChurn: input length mismatchUser.registerOperatorWithChurn: input quorums have common bits(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\xA2dipfsX\"\x12 T\x12\xFC\xCA\xB6\xA3\x1AR\xE0\xC0Pwb\xD4E?\xACK\xA8\xD9\xB9\xD0\xBA#m\xC5J\xCB\xA8\x8D>\xC4dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106101425760003560e01c806385226c81116100b8578063b5508aa91161007c578063b5508aa9146102a6578063ba414fa6146102ae578063bf68b816146102c6578063ca4f2d97146102cf578063e20c9f71146102e2578063fa7626d4146102ea57600080fd5b806385226c8114610223578063916a17c614610238578063a3f4df7e14610240578063a5f6cc1a14610255578063afa1c7371461026857600080fd5b80633f7286f41161010a5780633f7286f4146101b4578063505377e2146101bc57806365eda8e5146101c457806366d9a9a0146101da5780636d336f58146101ef5780638231b54c1461020257600080fd5b80631626ba7e146101475780631ed7831c146101785780632a34ade81461018d5780632ade3880146101975780633e5e3c23146101ac575b600080fd5b61015a6101553660046128d4565b6102f7565b6040516001600160e01b031990911681526020015b60405180910390f35b610180610326565b60405161016f919061295d565b610195610388565b005b61019f6104c1565b60405161016f9190612a06565b610180610603565b610180610663565b6101956106c3565b6101cc6107ee565b60405161016f929190612b3a565b6101e2610a29565b60405161016f9190612b68565b6101956101fd366004612cb9565b610b0f565b610215610210366004612dc2565b610d88565b60405190815260200161016f565b61022b610ece565b60405161016f9190612e03565b6101e2610f9e565b610248611084565b60405161016f9190612e65565b610195610263366004612e78565b611112565b604080518082018252600080825260209182015281518083018352602d54808252602e5491830191825283519081529051918101919091520161016f565b61022b611859565b6102b6611929565b604051901515815260200161016f565b61021560295481565b6101956102dd366004612dc2565b611a56565b610180611b6d565b6007546102b69060ff1681565b60008281526033602052604081205460ff161561031c5750630b135d3f60e11b610320565b5060005b92915050565b6060601480548060200260200160405190810160405280929190818152602001828054801561037e57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610360575b5050505050905090565b602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156103dd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104019190612f42565b506104406040518060400160405280601981526020017f726567697374657241734f70657261746f722028636f72652900000000000000815250611c7b565b6040805160608101825230815260006020820181905281830152601d549151630f589e5960e01b815290916001600160a01b031690630f589e599061048c90849060289060040161300d565b600060405180830381600087803b1580156104a657600080fd5b505af11580156104ba573d6000803e3d6000fd5b5050505050565b6060601b805480602002602001604051908101604052809291908181526020016000905b828210156105fa57600084815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b828210156105e357838290600052602060002001805461055690612f5b565b80601f016020809104026020016040519081016040528092919081815260200182805461058290612f5b565b80156105cf5780601f106105a4576101008083540402835291602001916105cf565b820191906000526020600020905b8154815290600101906020018083116105b257829003601f168201915b505050505081526020019060010190610537565b5050505081525050815260200190600101906104e5565b50505050905090565b6060601680548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b6060601580548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610718573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061073c9190612f42565b5061077b6040518060400160405280601e81526020017f7570646174655374616b657320287570646174654f70657261746f7273290000815250611c7b565b6040805160018082528183019092526000916020808301908036833701905050905030816000815181106107b1576107b161304d565b6001600160a01b039283166020918202929092018101919091525460405162cf2ab560e01b815291169062cf2ab59061048c90849060040161295d565b606080602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610846573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061086a9190612f42565b506108a16040518060400160405280601581526020017465786974456967656e6c617965722028636f72652960581b815250611c7b565b601d546040516367c0439f60e11b815230600482015260009182916001600160a01b039091169063cf80873e90602401600060405180830381865afa1580156108ee573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261091691908101906130be565b60408051600180825281830190925292945090925060009190816020015b604080516060808201835280825260208201526000918101919091528152602001906001900390816109345790505090506040518060600160405280848152602001838152602001306001600160a01b03168152508160008151811061099c5761099c61304d565b6020908102919091010152601d546040516306ec6e8160e11b81526001600160a01b0390911690630dd8dd02906109d7908490600401613178565b6000604051808303816000875af11580156109f6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610a1e9190810190613212565b509193509150509091565b60606019805480602002602001604051908101604052809291908181526020016000905b828210156105fa5760008481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015610af757602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411610ab95790505b50505050508152505081526020019060010190610a4d565b602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610b64573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b889190612f42565b50610bc76040518060400160405280601c81526020017f6465706f736974496e746f456967656e4c617965722028636f72652900000000815250611c7b565b60005b8251811015610d83576000838281518110610be757610be761304d565b602002602001015190506000838381518110610c0557610c0561304d565b602002602001015190506000826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c4f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c7391906132a2565b601e5460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303816000875af1158015610cc8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cec91906132bf565b50601e546040516373d0285560e11b81526001600160a01b0385811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303816000875af1158015610d48573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d6c9190612f42565b505050508080610d7b906132f7565b915050610bca565b505050565b602554604080516301504d8f60e41b815290516000926001600160a01b031691631504d8f0916004808301926020929190829003018187875af1158015610dd3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610df79190612f42565b50610e2b6040518060400160405280601081526020016f3932b3b4b9ba32b927b832b930ba37b960811b8152508484611cc3565b6020546001600160a01b031663a50857bf84846028602b610e4a611d48565b6040518663ffffffff1660e01b8152600401610e6a9594939291906133d5565b600060405180830381600087803b158015610e8457600080fd5b505af1158015610e98573d6000803e3d6000fd5b5050604080518082018252602d54808252602e546020928301908152600091825251909152209150610ec79050565b9392505050565b60606018805480602002602001604051908101604052809291908181526020016000905b828210156105fa578382906000526020600020018054610f1190612f5b565b80601f0160208091040260200160405190810160405280929190818152602001828054610f3d90612f5b565b8015610f8a5780601f10610f5f57610100808354040283529160200191610f8a565b820191906000526020600020905b815481529060010190602001808311610f6d57829003601f168201915b505050505081526020019060010190610ef2565b6060601a805480602002602001604051908101604052809291908181526020016000905b828210156105fa5760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561106c57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161102e5790505b50505050508152505081526020019060010190610fc2565b6028805461109190612f5b565b80601f01602080910402602001604051908101604052809291908181526020018280546110bd90612f5b565b801561110a5780601f106110df5761010080835404028352916020019161110a565b820191906000526020600020905b8154815290600101906020018083116110ed57829003601f168201915b505050505081565b602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015611167573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061118b9190612f42565b506112666040518060400160405280601981526020017f72656769737465724f70657261746f7257697468436875726e0000000000000081525087878080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525050604080516020808b0282810182019093528a82529093508a92508991829185019084908082843760009201919091525050604080516020601f8a018190048102820181019092528881529250889150879081908401838280828437600092019190915250611e5092505050565b60006112a787878080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506120d392505050565b905060006112ea84848080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506120d392505050565b6040805160608101909152603580825291925061131291899188916139266020830139612260565b6113408183166001600160c01b0316156040518060600160405280603e815260200161395b603e9139612296565b60006113596001600160c01b03838116908516176122cd565b9050600081516001600160401b0381111561137657611376612867565b6040519080825280602002602001820160405280156113bb57816020015b60408051808201909152600080825260208201528152602001906001900390816113945790505b5090506000805b83516113ce828461342c565b10156115e957818b1415611429576040805180820190915260008082526020820152836113fb838561342c565b8151811061140b5761140b61304d565b60200260200101819052508080611421906132f7565b9150506113c2565b8087148061147c57508787828181106114445761144461304d565b909101356001600160f81b03191690508c8c848181106114665761146661304d565b9050013560f81c60f81b6001600160f81b031916105b156115175760405180604001604052808d8d8581811061149e5761149e61304d565b919091013560f81c8252506020018b8b858181106114be576114be61304d565b90506020020160208101906114d39190613444565b6001600160a01b03169052836114e9838561342c565b815181106114f9576114f961304d565b6020026020010181905250818061150f906132f7565b9250506113c2565b8b8b838181106115295761152961304d565b909101356001600160f81b031916905088888381811061154b5761154b61304d565b9050013560f81c60f81b6001600160f81b0319161015611584576040805180820190915260008082526020820152836113fb838561342c565b60405162461bcd60e51b815260206004820152602f60248201527f557365722e72656769737465724f70657261746f7257697468436875726e3a2060448201526e1b585b199bdc9b5959081a5b9c1d5d608a1b60648201526084015b60405180910390fd5b60006034600081546115fa906132f7565b91829055506040805160208101929092526bffffffffffffffffffffffff193060601b169082015260540160408051601f1981840301815290829052805160209182012090546029546384ca521360e01b8452919350600019926000926001600160a01b03909216916384ca52139161167d9130918b90899089906004016134a8565b602060405180830381865afa15801561169a573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116be9190612f42565b601c546026546040516338d07aa960e21b8152929350600092839283926001600160a01b039091169163e341eaa491611704918890600401918252602082015260400190565b606060405180830381865afa158015611721573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061174591906134e3565b604080516041808252608082019092529396509194509250600091906020820181803683370190505090508260208201528160408201528360f81b816001835161178f9190613520565b8151811061179f5761179f61304d565b60200101906001600160f81b031916908160001a9053506040805160608101825282815260208082018a905291810188905290546001600160a01b0316639b5d177b8d6028602b8f866117f0611d48565b6040518763ffffffff1660e01b815260040161181196959493929190613537565b600060405180830381600087803b15801561182b57600080fd5b505af115801561183f573d6000803e3d6000fd5b505050505050505050505050505050505050505050505050565b60606017805480602002602001604051908101604052809291908181526020016000905b828210156105fa57838290600052602060002001805461189c90612f5b565b80601f01602080910402602001604051908101604052809291908181526020018280546118c890612f5b565b80156119155780601f106118ea57610100808354040283529160200191611915565b820191906000526020600020905b8154815290600101906020018083116118f857829003601f168201915b50505050508152602001906001019061187d565b600754600090610100900460ff161561194b5750600754610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611a515760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b828401528251808303840181526060830190935260009290916119d9917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc4916080016135b8565b60408051601f19818403018152908290526119f3916135e9565b6000604051808303816000865af19150503d8060008114611a30576040519150601f19603f3d011682016040523d82523d6000602084013e611a35565b606091505b5091505080806020019051810190611a4d91906132bf565b9150505b919050565b602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015611aab573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611acf9190612f42565b50611b05604051806040016040528060128152602001713232b932b3b4b9ba32b927b832b930ba37b960711b8152508383611cc3565b60205460405163ca4f2d9760e01b81526001600160a01b039091169063ca4f2d9790611b379085908590600401613605565b600060405180830381600087803b158015611b5157600080fd5b505af1158015611b65573d6000803e3d6000fd5b505050505050565b6060601380548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b6040805180820190915260008082526020820152611be9612849565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa9050808015611c1c57611c1e565bfe5b5080611c5c5760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b60448201526064016115e0565b505092915050565b805160009081526020918201519091526040902090565b600080516020613906833981519152602882604051602001611c9e929190613619565b60408051601f1981840301815290829052611cb891612e65565b60405180910390a150565b600080516020613999833981519152602884604051602001611ce6929190613619565b60408051601f198184030181526020601f860181900481028401810190925284835291611d2d91869086908190840183828082843760009201919091525061239992505050565b604051611d3b9291906136a9565b60405180910390a1505050565b604080516060808201835280825260006020830181905282840181905283519182018181526080830190945291928190815260200160346000815480929190611d90906132f7565b909155508152600019602091820152601f54602154918301516040808501519051631420c19160e31b81523060048201526001600160a01b0394851660248201526044810192909252606482015292935060009291169063a1060c8890608401602060405180830381865afa158015611e0d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e319190612f42565b6000908152603360205260409020805460ff1916600117905550919050565b600080516020613906833981519152602885604051602001611e73929190613619565b60408051601f1981840301815290829052611e8d91612e65565b60405180910390a1600080516020613999833981519152611ead82612399565b604051611eba91906136ce565b60405180910390a1600080516020613999833981519152611eda84612399565b604051611ee79190613709565b60405180910390a16040805180820190915260018152605b60f81b602082015260005b83518110156120845760018451611f219190613520565b811415611fcf5781848281518110611f3b57611f3b61304d565b60200260200101516001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa158015611f80573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611fa89190810190613741565b604051602001611fb99291906137ae565b6040516020818303038152906040529150612072565b81848281518110611fe257611fe261304d565b60200260200101516001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa158015612027573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261204f9190810190613741565b6040516020016120609291906137dd565b60405160208183030381529060405291505b8061207c816132f7565b915050611f0a565b50806040516020016120969190613819565b6040516020818303038152906040529050600080516020613999833981519152816040516120c4919061383e565b60405180910390a15050505050565b60006101008251111561215c5760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a4016115e0565b815161216a57506000919050565b600080836000815181106121805761218061304d565b0160200151600160f89190911c81901b92505b8451811015612257578481815181106121ae576121ae61304d565b0160200151600160f89190911c1b91508282116122435760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a4016115e0565b91811791612250816132f7565b9050612193565b50909392505050565b818314610d8357600080516020613999833981519152816040516122849190613876565b60405180910390a1610d838383612493565b816122c957600080516020613999833981519152816040516122b89190613876565b60405180910390a16122c9826125a8565b5050565b60606000806122db8461260d565b61ffff166001600160401b038111156122f6576122f6612867565b6040519080825280601f01601f191660200182016040528015612320576020820181803683370190505b5090506000805b825182108015612338575061010081105b1561238f576001811b93508584161561237f578060f81b8383815181106123615761236161304d565b60200101906001600160f81b031916908160001a9053508160010191505b612388816132f7565b9050612327565b5090949350505050565b6040805180820190915260018152605b60f81b602082015260609060005b835181101561246a57600184516123ce9190613520565b81141561241f57816123f88583815181106123eb576123eb61304d565b016020015160f81c612638565b6040516020016124099291906137ae565b6040516020818303038152906040529150612458565b816124358583815181106123eb576123eb61304d565b6040516020016124469291906137dd565b60405160208183030381529060405291505b80612462816132f7565b9150506123b7565b508060405160200161247c9190613819565b60408051601f198184030181529190529392505050565b8082146122c9576000805160206139068339815191526040516124f29060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808080808081319599d60b21b60608201526020810184905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160408051818152600a81830152690808080808149a59da1d60b21b60608201526020810183905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a16122c961273d565b8061260a576000805160206139068339815191526040516125fa9060208082526017908201527f4572726f723a20417373657274696f6e204661696c6564000000000000000000604082015260600190565b60405180910390a161260a61273d565b50565b6000805b821561032057612622600184613520565b9092169180612630816138a5565b915050612611565b60608161265c5750506040805180820190915260018152600360fc1b602082015290565b8160005b81156126865780612670816132f7565b915061267f9050600a836138dd565b9150612660565b6000816001600160401b038111156126a0576126a0612867565b6040519080825280601f01601f1916602001820160405280156126ca576020820181803683370190505b5090505b8415612735576126df600183613520565b91506126ec600a866138f1565b6126f790603061342c565b60f81b81838151811061270c5761270c61304d565b60200101906001600160f81b031916908160001a90535061272e600a866138dd565b94506126ce565b949350505050565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156128385760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f19818403018152908290526127d792916020016135b8565b60408051601f19818403018152908290526127f1916135e9565b6000604051808303816000865af19150503d806000811461282e576040519150601f19603f3d011682016040523d82523d6000602084013e612833565b606091505b505050505b6007805461ff001916610100179055565b60405180606001604052806003906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b03811182821017156128a5576128a5612867565b604052919050565b60006001600160401b038211156128c6576128c6612867565b50601f01601f191660200190565b600080604083850312156128e757600080fd5b8235915060208301356001600160401b0381111561290457600080fd5b8301601f8101851361291557600080fd5b8035612928612923826128ad565b61287d565b81815286602083850101111561293d57600080fd5b816020840160208301376000602083830101528093505050509250929050565b6020808252825182820181905260009190848201906040850190845b8181101561299e5783516001600160a01b031683529284019291840191600101612979565b50909695505050505050565b60005b838110156129c55781810151838201526020016129ad565b838111156129d4576000848401525b50505050565b600081518084526129f28160208601602086016129aa565b601f01601f19169290920160200192915050565b602080825282518282018190526000919060409081850190600581811b8701840188860187805b85811015612ab657603f198b8503018752825180516001600160a01b031685528901518985018990528051898601819052908a0190606081881b870181019190870190855b81811015612aa057605f19898503018352612a8e8486516129da565b948e01949350918d0191600101612a72565b505050978a019794505091880191600101612a2d565b50919a9950505050505050505050565b600081518084526020808501945080840160005b83811015612aff5781516001600160a01b031687529582019590820190600101612ada565b509495945050505050565b600081518084526020808501945080840160005b83811015612aff57815187529582019590820190600101612b1e565b604081526000612b4d6040830185612ac6565b8281036020840152612b5f8185612b0a565b95945050505050565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b84811015612c0c57898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b80831015612bf75783516001600160e01b0319168252928b019260019290920191908b0190612bcd565b50978a01979550505091870191600101612b90565b50919998505050505050505050565b60006001600160401b03821115612c3457612c34612867565b5060051b60200190565b6001600160a01b038116811461260a57600080fd5b600082601f830112612c6457600080fd5b81356020612c7461292383612c1b565b82815260059290921b84018101918181019086841115612c9357600080fd5b8286015b84811015612cae5780358352918301918301612c97565b509695505050505050565b60008060408385031215612ccc57600080fd5b82356001600160401b0380821115612ce357600080fd5b818501915085601f830112612cf757600080fd5b81356020612d0761292383612c1b565b82815260059290921b84018101918181019089841115612d2657600080fd5b948201945b83861015612d4d578535612d3e81612c3e565b82529482019490820190612d2b565b96505086013592505080821115612d6357600080fd5b50612d7085828601612c53565b9150509250929050565b60008083601f840112612d8c57600080fd5b5081356001600160401b03811115612da357600080fd5b602083019150836020828501011115612dbb57600080fd5b9250929050565b60008060208385031215612dd557600080fd5b82356001600160401b03811115612deb57600080fd5b612df785828601612d7a565b90969095509350505050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b82811015612e5857603f19888603018452612e468583516129da565b94509285019290850190600101612e2a565b5092979650505050505050565b602081526000610ec760208301846129da565b60008060008060008060608789031215612e9157600080fd5b86356001600160401b0380821115612ea857600080fd5b612eb48a838b01612d7a565b90985096506020890135915080821115612ecd57600080fd5b818901915089601f830112612ee157600080fd5b813581811115612ef057600080fd5b8a60208260051b8501011115612f0557600080fd5b602083019650809550506040890135915080821115612f2357600080fd5b50612f3089828a01612d7a565b979a9699509497509295939492505050565b600060208284031215612f5457600080fd5b5051919050565b600181811c90821680612f6f57607f821691505b60208210811415612f9057634e487b7160e01b600052602260045260246000fd5b50919050565b60008154612fa381612f5b565b808552602060018381168015612fc05760018114612fd457613002565b60ff19851688840152604088019550613002565b866000528260002060005b85811015612ffa5781548a8201860152908301908401612fdf565b890184019650505b505050505092915050565b600060018060a01b038085511683528060208601511660208401525063ffffffff6040850151166040830152608060608301526127356080830184612f96565b634e487b7160e01b600052603260045260246000fd5b600082601f83011261307457600080fd5b8151602061308461292383612c1b565b82815260059290921b840181019181810190868411156130a357600080fd5b8286015b84811015612cae57805183529183019183016130a7565b600080604083850312156130d157600080fd5b82516001600160401b03808211156130e857600080fd5b818501915085601f8301126130fc57600080fd5b8151602061310c61292383612c1b565b82815260059290921b8401810191818101908984111561312b57600080fd5b948201945b8386101561315257855161314381612c3e565b82529482019490820190613130565b9188015191965090935050508082111561316b57600080fd5b50612d7085828601613063565b60006020808301818452808551808352604092508286019150828160051b87010184880160005b8381101561320457603f198984030185528151606081518186526131c582870182612ac6565b915050888201518582038a8701526131dd8282612b0a565b928901516001600160a01b031695890195909552509487019492509086019060010161319f565b509098975050505050505050565b6000602080838503121561322557600080fd5b82516001600160401b0381111561323b57600080fd5b8301601f8101851361324c57600080fd5b805161325a61292382612c1b565b81815260059190911b8201830190838101908783111561327957600080fd5b928401925b828410156132975783518252928401929084019061327e565b979650505050505050565b6000602082840312156132b457600080fd5b8151610ec781612c3e565b6000602082840312156132d157600080fd5b81518015158114610ec757600080fd5b634e487b7160e01b600052601160045260246000fd5b600060001982141561330b5761330b6132e1565b5060010190565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b8060005b60028110156129d457815484526020909301926001918201910161333f565b80548252600181015460208301526002810154604083015260038101546060830152613390608083016004830161333b565b6122c960c083016006830161333b565b60008151606084526133b560608501826129da565b905060208301516020850152604083015160408501528091505092915050565b60006101608083526133ea818401888a613312565b905082810360208401526133fe8187612f96565b905061340d604084018661335e565b82810361014084015261342081856133a0565b98975050505050505050565b6000821982111561343f5761343f6132e1565b500190565b60006020828403121561345657600080fd5b8135610ec781612c3e565b600081518084526020808501945080840160005b83811015612aff578151805160ff1688528301516001600160a01b03168388015260409096019590820190600101613475565b60018060a01b038616815284602082015260a0604082015260006134cf60a0830186613461565b606083019490945250608001529392505050565b6000806000606084860312156134f857600080fd5b835160ff8116811461350957600080fd5b602085015160409095015190969495509392505050565b600082821015613532576135326132e1565b500390565b60006101a080835261354b8184018a6129da565b9050828103602084015261355f8189612f96565b905061356e604084018861335e565b8281036101408401526135818187613461565b905082810361016084015261359681866133a0565b90508281036101808401526135ab81856133a0565b9998505050505050505050565b6001600160e01b03198316815281516000906135db8160048501602087016129aa565b919091016004019392505050565b600082516135fb8184602087016129aa565b9190910192915050565b602081526000612735602083018486613312565b600080845461362781612f5b565b6001828116801561363f57600181146136505761367f565b60ff1984168752828701945061367f565b8860005260208060002060005b858110156136765781548a82015290840190820161365d565b50505082870194505b50601760f91b84528651925061369b8382860160208a016129aa565b919092010195945050505050565b6040815260006136bc60408301856129da565b8281036020840152612b5f81856129da565b6040815260116040820152702d207374616e6461726451756f72756d7360781b6060820152608060208201526000610ec760808301846129da565b60408152600e60408201526d2d20636875726e51756f72756d7360901b6060820152608060208201526000610ec760808301846129da565b60006020828403121561375357600080fd5b81516001600160401b0381111561376957600080fd5b8201601f8101841361377a57600080fd5b8051613788612923826128ad565b81815285602083850101111561379d57600080fd5b612b5f8260208301602086016129aa565b600083516137c08184602088016129aa565b8351908301906137d48183602088016129aa565b01949350505050565b600083516137ef8184602088016129aa565b8351908301906138038183602088016129aa565b61016160f51b9101908152600201949350505050565b6000825161382b8184602087016129aa565b605d60f81b920191825250600101919050565b60408152600e60408201526d2d20636875726e5461726765747360901b6060820152608060208201526000610ec760808301846129da565b60408152600560408201526422b93937b960d91b6060820152608060208201526000610ec760808301846129da565b600061ffff808316818114156138bd576138bd6132e1565b6001019392505050565b634e487b7160e01b600052601260045260246000fd5b6000826138ec576138ec6138c7565b500490565b600082613900576139006138c7565b50069056fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50557365722e72656769737465724f70657261746f7257697468436875726e3a20696e707574206c656e677468206d69736d61746368557365722e72656769737465724f70657261746f7257697468436875726e3a20696e7075742071756f72756d73206861766520636f6d6d6f6e2062697473280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583a26469706673582212205412fccab6a31a52e0c0507762d4453fac4ba8d9b9d0ba236dc54acba88d3ec464736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\xB8W\x80c\xB5P\x8A\xA9\x11a\0|W\x80c\xB5P\x8A\xA9\x14a\x02\xA6W\x80c\xBAAO\xA6\x14a\x02\xAEW\x80c\xBFh\xB8\x16\x14a\x02\xC6W\x80c\xCAO-\x97\x14a\x02\xCFW\x80c\xE2\x0C\x9Fq\x14a\x02\xE2W\x80c\xFAv&\xD4\x14a\x02\xEAW`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x02#W\x80c\x91j\x17\xC6\x14a\x028W\x80c\xA3\xF4\xDF~\x14a\x02@W\x80c\xA5\xF6\xCC\x1A\x14a\x02UW\x80c\xAF\xA1\xC77\x14a\x02hW`\0\x80\xFD[\x80c?r\x86\xF4\x11a\x01\nW\x80c?r\x86\xF4\x14a\x01\xB4W\x80cPSw\xE2\x14a\x01\xBCW\x80ce\xED\xA8\xE5\x14a\x01\xC4W\x80cf\xD9\xA9\xA0\x14a\x01\xDAW\x80cm3oX\x14a\x01\xEFW\x80c\x821\xB5L\x14a\x02\x02W`\0\x80\xFD[\x80c\x16&\xBA~\x14a\x01GW\x80c\x1E\xD7\x83\x1C\x14a\x01xW\x80c*4\xAD\xE8\x14a\x01\x8DW\x80c*\xDE8\x80\x14a\x01\x97W\x80c>^<#\x14a\x01\xACW[`\0\x80\xFD[a\x01Za\x01U6`\x04a(\xD4V[a\x02\xF7V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x80a\x03&V[`@Qa\x01o\x91\x90a)]V[a\x01\x95a\x03\x88V[\0[a\x01\x9Fa\x04\xC1V[`@Qa\x01o\x91\x90a*\x06V[a\x01\x80a\x06\x03V[a\x01\x80a\x06cV[a\x01\x95a\x06\xC3V[a\x01\xCCa\x07\xEEV[`@Qa\x01o\x92\x91\x90a+:V[a\x01\xE2a\n)V[`@Qa\x01o\x91\x90a+hV[a\x01\x95a\x01\xFD6`\x04a,\xB9V[a\x0B\x0FV[a\x02\x15a\x02\x106`\x04a-\xC2V[a\r\x88V[`@Q\x90\x81R` \x01a\x01oV[a\x02+a\x0E\xCEV[`@Qa\x01o\x91\x90a.\x03V[a\x01\xE2a\x0F\x9EV[a\x02Ha\x10\x84V[`@Qa\x01o\x91\x90a.eV[a\x01\x95a\x02c6`\x04a.xV[a\x11\x12V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`-T\x80\x82R`.T\x91\x83\x01\x91\x82R\x83Q\x90\x81R\x90Q\x91\x81\x01\x91\x90\x91R\x01a\x01oV[a\x02+a\x18YV[a\x02\xB6a\x19)V[`@Q\x90\x15\x15\x81R` \x01a\x01oV[a\x02\x15`)T\x81V[a\x01\x95a\x02\xDD6`\x04a-\xC2V[a\x1AVV[a\x01\x80a\x1BmV[`\x07Ta\x02\xB6\x90`\xFF\x16\x81V[`\0\x82\x81R`3` R`@\x81 T`\xFF\x16\x15a\x03\x1CWPc\x0B\x13]?`\xE1\x1Ba\x03 V[P`\0[\x92\x91PPV[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`W[PPPPP\x90P\x90V[`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x01\x91\x90a/BV[Pa\x04@`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FregisterAsOperator (core)\0\0\0\0\0\0\0\x81RPa\x1C{V[`@\x80Q``\x81\x01\x82R0\x81R`\0` \x82\x01\x81\x90R\x81\x83\x01R`\x1DT\x91Qc\x0FX\x9EY`\xE0\x1B\x81R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x0FX\x9EY\x90a\x04\x8C\x90\x84\x90`(\x90`\x04\x01a0\rV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xBAW=`\0\x80>=`\0\xFD[PPPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFAW`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x05\xE3W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x05V\x90a/[V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x82\x90a/[V[\x80\x15a\x05\xCFW\x80`\x1F\x10a\x05\xA4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xCFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xB2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x057V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04\xE5V[PPPP\x90P\x90V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07<\x91\x90a/BV[Pa\x07{`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7FupdateStakes (updateOperators)\0\0\x81RPa\x1C{V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P0\x81`\0\x81Q\x81\x10a\x07\xB1Wa\x07\xB1a0MV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91RT`@Qb\xCF*\xB5`\xE0\x1B\x81R\x91\x16\x90b\xCF*\xB5\x90a\x04\x8C\x90\x84\x90`\x04\x01a)]V[``\x80`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08j\x91\x90a/BV[Pa\x08\xA1`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01texitEigenlayer (core)`X\x1B\x81RPa\x1C{V[`\x1DT`@Qcg\xC0C\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCF\x80\x87>\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\x16\x91\x90\x81\x01\x90a0\xBEV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90\x92P`\0\x91\x90\x81` \x01[`@\x80Q``\x80\x82\x01\x83R\x80\x82R` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t4W\x90PP\x90P`@Q\x80``\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x010`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x81`\0\x81Q\x81\x10a\t\x9CWa\t\x9Ca0MV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x1DT`@Qc\x06\xECn\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\r\xD8\xDD\x02\x90a\t\xD7\x90\x84\x90`\x04\x01a1xV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\x1E\x91\x90\x81\x01\x90a2\x12V[P\x91\x93P\x91PP\x90\x91V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFAW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\n\xF7W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\n\xB9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\nMV[`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0BdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x88\x91\x90a/BV[Pa\x0B\xC7`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FdepositIntoEigenLayer (core)\0\0\0\0\x81RPa\x1C{V[`\0[\x82Q\x81\x10\x15a\r\x83W`\0\x83\x82\x81Q\x81\x10a\x0B\xE7Wa\x0B\xE7a0MV[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x0C\x05Wa\x0C\x05a0MV[` \x02` \x01\x01Q\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0COW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cs\x91\x90a2\xA2V[`\x1ET`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xEC\x91\x90a2\xBFV[P`\x1ET`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\rHW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rl\x91\x90a/BV[PPPP\x80\x80a\r{\x90a2\xF7V[\x91PPa\x0B\xCAV[PPPV[`%T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x87\x87Z\xF1\x15\x80\x15a\r\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF7\x91\x90a/BV[Pa\x0E+`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o92\xB3\xB4\xB9\xBA2\xB9'\xB82\xB90\xBA7\xB9`\x81\x1B\x81RP\x84\x84a\x1C\xC3V[` T`\x01`\x01`\xA0\x1B\x03\x16c\xA5\x08W\xBF\x84\x84`(`+a\x0EJa\x1DHV[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ej\x95\x94\x93\x92\x91\x90a3\xD5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x84W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x98W=`\0\x80>=`\0\xFD[PP`@\x80Q\x80\x82\x01\x82R`-T\x80\x82R`.T` \x92\x83\x01\x90\x81R`\0\x91\x82RQ\x90\x91R \x91Pa\x0E\xC7\x90PV[\x93\x92PPPV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFAW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x0F\x11\x90a/[V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F=\x90a/[V[\x80\x15a\x0F\x8AW\x80`\x1F\x10a\x0F_Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\x8AV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0FmW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0E\xF2V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFAW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x10lW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x10.W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0F\xC2V[`(\x80Ta\x10\x91\x90a/[V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xBD\x90a/[V[\x80\x15a\x11\nW\x80`\x1F\x10a\x10\xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\nV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xEDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x8B\x91\x90a/BV[Pa\x12f`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FregisterOperatorWithChurn\0\0\0\0\0\0\0\x81RP\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` \x80\x8B\x02\x82\x81\x01\x82\x01\x90\x93R\x8A\x82R\x90\x93P\x8A\x92P\x89\x91\x82\x91\x85\x01\x90\x84\x90\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1EP\x92PPPV[`\0a\x12\xA7\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa \xD3\x92PPPV[\x90P`\0a\x12\xEA\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa \xD3\x92PPPV[`@\x80Q``\x81\x01\x90\x91R`5\x80\x82R\x91\x92Pa\x13\x12\x91\x89\x91\x88\x91a9&` \x83\x019a\"`V[a\x13@\x81\x83\x16`\x01`\x01`\xC0\x1B\x03\x16\x15`@Q\x80``\x01`@R\x80`>\x81R` \x01a9[`>\x919a\"\x96V[`\0a\x13Y`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17a\"\xCDV[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13vWa\x13va(gV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\xBBW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\x94W\x90P[P\x90P`\0\x80[\x83Qa\x13\xCE\x82\x84a4,V[\x10\x15a\x15\xE9W\x81\x8B\x14\x15a\x14)W`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x83a\x13\xFB\x83\x85a4,V[\x81Q\x81\x10a\x14\x0BWa\x14\x0Ba0MV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x14!\x90a2\xF7V[\x91PPa\x13\xC2V[\x80\x87\x14\x80a\x14|WP\x87\x87\x82\x81\x81\x10a\x14DWa\x14Da0MV[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P\x8C\x8C\x84\x81\x81\x10a\x14fWa\x14fa0MV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x10[\x15a\x15\x17W`@Q\x80`@\x01`@R\x80\x8D\x8D\x85\x81\x81\x10a\x14\x9EWa\x14\x9Ea0MV[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x01\x8B\x8B\x85\x81\x81\x10a\x14\xBEWa\x14\xBEa0MV[\x90P` \x02\x01` \x81\x01\x90a\x14\xD3\x91\x90a4DV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x83a\x14\xE9\x83\x85a4,V[\x81Q\x81\x10a\x14\xF9Wa\x14\xF9a0MV[` \x02` \x01\x01\x81\x90RP\x81\x80a\x15\x0F\x90a2\xF7V[\x92PPa\x13\xC2V[\x8B\x8B\x83\x81\x81\x10a\x15)Wa\x15)a0MV[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P\x88\x88\x83\x81\x81\x10a\x15KWa\x15Ka0MV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x10\x15a\x15\x84W`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x83a\x13\xFB\x83\x85a4,V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FUser.registerOperatorWithChurn: `D\x82\x01Rn\x1BX[\x19\x9B\xDC\x9BYY\x08\x1A[\x9C\x1D]`\x8A\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0`4`\0\x81Ta\x15\xFA\x90a2\xF7V[\x91\x82\x90UP`@\x80Q` \x81\x01\x92\x90\x92Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16\x90\x82\x01R`T\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x90T`)Tc\x84\xCAR\x13`\xE0\x1B\x84R\x91\x93P`\0\x19\x92`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x84\xCAR\x13\x91a\x16}\x910\x91\x8B\x90\x89\x90\x89\x90`\x04\x01a4\xA8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xBE\x91\x90a/BV[`\x1CT`&T`@Qc8\xD0z\xA9`\xE2\x1B\x81R\x92\x93P`\0\x92\x83\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xE3A\xEA\xA4\x91a\x17\x04\x91\x88\x90`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17E\x91\x90a4\xE3V[`@\x80Q`A\x80\x82R`\x80\x82\x01\x90\x92R\x93\x96P\x91\x94P\x92P`\0\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x82` \x82\x01R\x81`@\x82\x01R\x83`\xF8\x1B\x81`\x01\x83Qa\x17\x8F\x91\x90a5 V[\x81Q\x81\x10a\x17\x9FWa\x17\x9Fa0MV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`@\x80Q``\x81\x01\x82R\x82\x81R` \x80\x82\x01\x8A\x90R\x91\x81\x01\x88\x90R\x90T`\x01`\x01`\xA0\x1B\x03\x16c\x9B]\x17{\x8D`(`+\x8F\x86a\x17\xF0a\x1DHV[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x11\x96\x95\x94\x93\x92\x91\x90a57V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18+W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18?W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPPPPPPPPPPV[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFAW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x18\x9C\x90a/[V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18\xC8\x90a/[V[\x80\x15a\x19\x15W\x80`\x1F\x10a\x18\xEAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x19\x15V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18\xF8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x18}V[`\x07T`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a\x19KWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1AQW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x19\xD9\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a5\xB8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x19\xF3\x91a5\xE9V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1A0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1A5V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x1AM\x91\x90a2\xBFV[\x91PP[\x91\x90PV[`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1A\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xCF\x91\x90a/BV[Pa\x1B\x05`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q22\xB92\xB3\xB4\xB9\xBA2\xB9'\xB82\xB90\xBA7\xB9`q\x1B\x81RP\x83\x83a\x1C\xC3V[` T`@Qc\xCAO-\x97`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCAO-\x97\x90a\x1B7\x90\x85\x90\x85\x90`\x04\x01a6\x05V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1BQW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1BeW=`\0\x80>=`\0\xFD[PPPPPPV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x1B\xE9a(IV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x1C\x1CWa\x1C\x1EV[\xFE[P\x80a\x1C\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x15\xE0V[PP\x92\x91PPV[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[`\0\x80Q` a9\x06\x839\x81Q\x91R`(\x82`@Q` \x01a\x1C\x9E\x92\x91\x90a6\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1C\xB8\x91a.eV[`@Q\x80\x91\x03\x90\xA1PV[`\0\x80Q` a9\x99\x839\x81Q\x91R`(\x84`@Q` \x01a\x1C\xE6\x92\x91\x90a6\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` `\x1F\x86\x01\x81\x90\x04\x81\x02\x84\x01\x81\x01\x90\x92R\x84\x83R\x91a\x1D-\x91\x86\x90\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa#\x99\x92PPPV[`@Qa\x1D;\x92\x91\x90a6\xA9V[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x80\x82\x01\x83R\x80\x82R`\0` \x83\x01\x81\x90R\x82\x84\x01\x81\x90R\x83Q\x91\x82\x01\x81\x81R`\x80\x83\x01\x90\x94R\x91\x92\x81\x90\x81R` \x01`4`\0\x81T\x80\x92\x91\x90a\x1D\x90\x90a2\xF7V[\x90\x91UP\x81R`\0\x19` \x91\x82\x01R`\x1FT`!T\x91\x83\x01Q`@\x80\x85\x01Q\x90Qc\x14 \xC1\x91`\xE3\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`$\x82\x01R`D\x81\x01\x92\x90\x92R`d\x82\x01R\x92\x93P`\0\x92\x91\x16\x90c\xA1\x06\x0C\x88\x90`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E1\x91\x90a/BV[`\0\x90\x81R`3` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UP\x91\x90PV[`\0\x80Q` a9\x06\x839\x81Q\x91R`(\x85`@Q` \x01a\x1Es\x92\x91\x90a6\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\x8D\x91a.eV[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a9\x99\x839\x81Q\x91Ra\x1E\xAD\x82a#\x99V[`@Qa\x1E\xBA\x91\x90a6\xCEV[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a9\x99\x839\x81Q\x91Ra\x1E\xDA\x84a#\x99V[`@Qa\x1E\xE7\x91\x90a7\tV[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`[`\xF8\x1B` \x82\x01R`\0[\x83Q\x81\x10\x15a \x84W`\x01\x84Qa\x1F!\x91\x90a5 V[\x81\x14\x15a\x1F\xCFW\x81\x84\x82\x81Q\x81\x10a\x1F;Wa\x1F;a0MV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\xA8\x91\x90\x81\x01\x90a7AV[`@Q` \x01a\x1F\xB9\x92\x91\x90a7\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa rV[\x81\x84\x82\x81Q\x81\x10a\x1F\xE2Wa\x1F\xE2a0MV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a 'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra O\x91\x90\x81\x01\x90a7AV[`@Q` \x01a `\x92\x91\x90a7\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P[\x80a |\x81a2\xF7V[\x91PPa\x1F\nV[P\x80`@Q` \x01a \x96\x91\x90a8\x19V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80Q` a9\x99\x839\x81Q\x91R\x81`@Qa \xC4\x91\x90a8>V[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\0a\x01\0\x82Q\x11\x15a!\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x15\xE0V[\x81Qa!jWP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a!\x80Wa!\x80a0MV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a\"WW\x84\x81\x81Q\x81\x10a!\xAEWa!\xAEa0MV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a\"CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x15\xE0V[\x91\x81\x17\x91a\"P\x81a2\xF7V[\x90Pa!\x93V[P\x90\x93\x92PPPV[\x81\x83\x14a\r\x83W`\0\x80Q` a9\x99\x839\x81Q\x91R\x81`@Qa\"\x84\x91\x90a8vV[`@Q\x80\x91\x03\x90\xA1a\r\x83\x83\x83a$\x93V[\x81a\"\xC9W`\0\x80Q` a9\x99\x839\x81Q\x91R\x81`@Qa\"\xB8\x91\x90a8vV[`@Q\x80\x91\x03\x90\xA1a\"\xC9\x82a%\xA8V[PPV[```\0\x80a\"\xDB\x84a&\rV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xF6Wa\"\xF6a(gV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a# W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a#8WPa\x01\0\x81\x10[\x15a#\x8FW`\x01\x81\x1B\x93P\x85\x84\x16\x15a#\x7FW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a#aWa#aa0MV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a#\x88\x81a2\xF7V[\x90Pa#'V[P\x90\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`[`\xF8\x1B` \x82\x01R``\x90`\0[\x83Q\x81\x10\x15a$jW`\x01\x84Qa#\xCE\x91\x90a5 V[\x81\x14\x15a$\x1FW\x81a#\xF8\x85\x83\x81Q\x81\x10a#\xEBWa#\xEBa0MV[\x01` \x01Q`\xF8\x1Ca&8V[`@Q` \x01a$\t\x92\x91\x90a7\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa$XV[\x81a$5\x85\x83\x81Q\x81\x10a#\xEBWa#\xEBa0MV[`@Q` \x01a$F\x92\x91\x90a7\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P[\x80a$b\x81a2\xF7V[\x91PPa#\xB7V[P\x80`@Q` \x01a$|\x91\x90a8\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[\x80\x82\x14a\"\xC9W`\0\x80Q` a9\x06\x839\x81Q\x91R`@Qa$\xF2\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x08\x13\x19Y\x9D`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x14\x9AY\xDA\x1D`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a\"\xC9a'=V[\x80a&\nW`\0\x80Q` a9\x06\x839\x81Q\x91R`@Qa%\xFA\x90` \x80\x82R`\x17\x90\x82\x01R\x7FError: Assertion Failed\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1a&\na'=V[PV[`\0\x80[\x82\x15a\x03 Wa&\"`\x01\x84a5 V[\x90\x92\x16\x91\x80a&0\x81a8\xA5V[\x91PPa&\x11V[``\x81a&\\WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[\x81\x15a&\x86W\x80a&p\x81a2\xF7V[\x91Pa&\x7F\x90P`\n\x83a8\xDDV[\x91Pa&`V[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xA0Wa&\xA0a(gV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a&\xCAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a'5Wa&\xDF`\x01\x83a5 V[\x91Pa&\xEC`\n\x86a8\xF1V[a&\xF7\x90`0a4,V[`\xF8\x1B\x81\x83\x81Q\x81\x10a'\x0CWa'\x0Ca0MV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SPa'.`\n\x86a8\xDDV[\x94Pa&\xCEV[\x94\x93PPPPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a(8W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra'\xD7\x92\x91` \x01a5\xB8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra'\xF1\x91a5\xE9V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a(.W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a(3V[``\x91P[PPPP[`\x07\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a(\xA5Wa(\xA5a(gV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a(\xC6Wa(\xC6a(gV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a(\xE7W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)\x04W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a)\x15W`\0\x80\xFD[\x805a)(a)#\x82a(\xADV[a(}V[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a)=W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a)\x9EW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a)yV[P\x90\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15a)\xC5W\x81\x81\x01Q\x83\x82\x01R` \x01a)\xADV[\x83\x81\x11\x15a)\xD4W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra)\xF2\x81` \x86\x01` \x86\x01a)\xAAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a*\xB6W`?\x19\x8B\x85\x03\x01\x87R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a*\xA0W`_\x19\x89\x85\x03\x01\x83Ra*\x8E\x84\x86Qa)\xDAV[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a*rV[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a*-V[P\x91\x9A\x99PPPPPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\xFFW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a*\xDAV[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\xFFW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a+\x1EV[`@\x81R`\0a+M`@\x83\x01\x85a*\xC6V[\x82\x81\x03` \x84\x01Ra+_\x81\x85a+\nV[\x95\x94PPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a,\x0CW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a+\xF7W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a+\xCDV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a+\x90V[P\x91\x99\x98PPPPPPPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a,4Wa,4a(gV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a&\nW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a,dW`\0\x80\xFD[\x815` a,ta)#\x83a,\x1BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a,\x93W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a,\xAEW\x805\x83R\x91\x83\x01\x91\x83\x01a,\x97V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a,\xCCW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a,\xE3W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a,\xF7W`\0\x80\xFD[\x815` a-\x07a)#\x83a,\x1BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a-&W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a-MW\x855a->\x81a,>V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a-+V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a-cW`\0\x80\xFD[Pa-p\x85\x82\x86\x01a,SV[\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a-\x8CW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xA3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a-\xBBW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a-\xD5W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xEBW`\0\x80\xFD[a-\xF7\x85\x82\x86\x01a-zV[\x90\x96\x90\x95P\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a.XW`?\x19\x88\x86\x03\x01\x84Ra.F\x85\x83Qa)\xDAV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a.*V[P\x92\x97\x96PPPPPPPV[` \x81R`\0a\x0E\xC7` \x83\x01\x84a)\xDAV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a.\x91W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a.\xA8W`\0\x80\xFD[a.\xB4\x8A\x83\x8B\x01a-zV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a.\xCDW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a.\xE1W`\0\x80\xFD[\x815\x81\x81\x11\x15a.\xF0W`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15a/\x05W`\0\x80\xFD[` \x83\x01\x96P\x80\x95PP`@\x89\x015\x91P\x80\x82\x11\x15a/#W`\0\x80\xFD[Pa/0\x89\x82\x8A\x01a-zV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0` \x82\x84\x03\x12\x15a/TW`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a/oW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a/\x90WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x81Ta/\xA3\x81a/[V[\x80\x85R` `\x01\x83\x81\x16\x80\x15a/\xC0W`\x01\x81\x14a/\xD4Wa0\x02V[`\xFF\x19\x85\x16\x88\x84\x01R`@\x88\x01\x95Pa0\x02V[\x86`\0R\x82`\0 `\0[\x85\x81\x10\x15a/\xFAW\x81T\x8A\x82\x01\x86\x01R\x90\x83\x01\x90\x84\x01a/\xDFV[\x89\x01\x84\x01\x96PP[PPPPP\x92\x91PPV[`\0`\x01\x80`\xA0\x1B\x03\x80\x85Q\x16\x83R\x80` \x86\x01Q\x16` \x84\x01RPc\xFF\xFF\xFF\xFF`@\x85\x01Q\x16`@\x83\x01R`\x80``\x83\x01Ra'5`\x80\x83\x01\x84a/\x96V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a0tW`\0\x80\xFD[\x81Q` a0\x84a)#\x83a,\x1BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a0\xA3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a,\xAEW\x80Q\x83R\x91\x83\x01\x91\x83\x01a0\xA7V[`\0\x80`@\x83\x85\x03\x12\x15a0\xD1W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a0\xE8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a0\xFCW`\0\x80\xFD[\x81Q` a1\x0Ca)#\x83a,\x1BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a1+W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a1RW\x85Qa1C\x81a,>V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a10V[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15a1kW`\0\x80\xFD[Pa-p\x85\x82\x86\x01a0cV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a2\x04W`?\x19\x89\x84\x03\x01\x85R\x81Q``\x81Q\x81\x86Ra1\xC5\x82\x87\x01\x82a*\xC6V[\x91PP\x88\x82\x01Q\x85\x82\x03\x8A\x87\x01Ra1\xDD\x82\x82a+\nV[\x92\x89\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x95\x89\x01\x95\x90\x95RP\x94\x87\x01\x94\x92P\x90\x86\x01\x90`\x01\x01a1\x9FV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a2%W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a2;W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a2LW`\0\x80\xFD[\x80Qa2Za)#\x82a,\x1BV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a2yW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a2\x97W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a2~V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a2\xB4W`\0\x80\xFD[\x81Qa\x0E\xC7\x81a,>V[`\0` \x82\x84\x03\x12\x15a2\xD1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0E\xC7W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a3\x0BWa3\x0Ba2\xE1V[P`\x01\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x80`\0[`\x02\x81\x10\x15a)\xD4W\x81T\x84R` \x90\x93\x01\x92`\x01\x91\x82\x01\x91\x01a3?V[\x80T\x82R`\x01\x81\x01T` \x83\x01R`\x02\x81\x01T`@\x83\x01R`\x03\x81\x01T``\x83\x01Ra3\x90`\x80\x83\x01`\x04\x83\x01a3;V[a\"\xC9`\xC0\x83\x01`\x06\x83\x01a3;V[`\0\x81Q``\x84Ra3\xB5``\x85\x01\x82a)\xDAV[\x90P` \x83\x01Q` \x85\x01R`@\x83\x01Q`@\x85\x01R\x80\x91PP\x92\x91PPV[`\0a\x01`\x80\x83Ra3\xEA\x81\x84\x01\x88\x8Aa3\x12V[\x90P\x82\x81\x03` \x84\x01Ra3\xFE\x81\x87a/\x96V[\x90Pa4\r`@\x84\x01\x86a3^V[\x82\x81\x03a\x01@\x84\x01Ra4 \x81\x85a3\xA0V[\x98\x97PPPPPPPPV[`\0\x82\x19\x82\x11\x15a4?Wa4?a2\xE1V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a4VW`\0\x80\xFD[\x815a\x0E\xC7\x81a,>V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\xFFW\x81Q\x80Q`\xFF\x16\x88R\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a4uV[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0a4\xCF`\xA0\x83\x01\x86a4aV[``\x83\x01\x94\x90\x94RP`\x80\x01R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a4\xF8W`\0\x80\xFD[\x83Q`\xFF\x81\x16\x81\x14a5\tW`\0\x80\xFD[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[`\0\x82\x82\x10\x15a52Wa52a2\xE1V[P\x03\x90V[`\0a\x01\xA0\x80\x83Ra5K\x81\x84\x01\x8Aa)\xDAV[\x90P\x82\x81\x03` \x84\x01Ra5_\x81\x89a/\x96V[\x90Pa5n`@\x84\x01\x88a3^V[\x82\x81\x03a\x01@\x84\x01Ra5\x81\x81\x87a4aV[\x90P\x82\x81\x03a\x01`\x84\x01Ra5\x96\x81\x86a3\xA0V[\x90P\x82\x81\x03a\x01\x80\x84\x01Ra5\xAB\x81\x85a3\xA0V[\x99\x98PPPPPPPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a5\xDB\x81`\x04\x85\x01` \x87\x01a)\xAAV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa5\xFB\x81\x84` \x87\x01a)\xAAV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a'5` \x83\x01\x84\x86a3\x12V[`\0\x80\x84Ta6'\x81a/[V[`\x01\x82\x81\x16\x80\x15a6?W`\x01\x81\x14a6PWa6\x7FV[`\xFF\x19\x84\x16\x87R\x82\x87\x01\x94Pa6\x7FV[\x88`\0R` \x80`\0 `\0[\x85\x81\x10\x15a6vW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a6]V[PPP\x82\x87\x01\x94P[P`\x17`\xF9\x1B\x84R\x86Q\x92Pa6\x9B\x83\x82\x86\x01` \x8A\x01a)\xAAV[\x91\x90\x92\x01\x01\x95\x94PPPPPV[`@\x81R`\0a6\xBC`@\x83\x01\x85a)\xDAV[\x82\x81\x03` \x84\x01Ra+_\x81\x85a)\xDAV[`@\x81R`\x11`@\x82\x01Rp- standardQuorums`x\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x0E\xC7`\x80\x83\x01\x84a)\xDAV[`@\x81R`\x0E`@\x82\x01Rm- churnQuorums`\x90\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x0E\xC7`\x80\x83\x01\x84a)\xDAV[`\0` \x82\x84\x03\x12\x15a7SW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a7iW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a7zW`\0\x80\xFD[\x80Qa7\x88a)#\x82a(\xADV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a7\x9DW`\0\x80\xFD[a+_\x82` \x83\x01` \x86\x01a)\xAAV[`\0\x83Qa7\xC0\x81\x84` \x88\x01a)\xAAV[\x83Q\x90\x83\x01\x90a7\xD4\x81\x83` \x88\x01a)\xAAV[\x01\x94\x93PPPPV[`\0\x83Qa7\xEF\x81\x84` \x88\x01a)\xAAV[\x83Q\x90\x83\x01\x90a8\x03\x81\x83` \x88\x01a)\xAAV[a\x01a`\xF5\x1B\x91\x01\x90\x81R`\x02\x01\x94\x93PPPPV[`\0\x82Qa8+\x81\x84` \x87\x01a)\xAAV[`]`\xF8\x1B\x92\x01\x91\x82RP`\x01\x01\x91\x90PV[`@\x81R`\x0E`@\x82\x01Rm- churnTargets`\x90\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x0E\xC7`\x80\x83\x01\x84a)\xDAV[`@\x81R`\x05`@\x82\x01Rd\"\xB997\xB9`\xD9\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x0E\xC7`\x80\x83\x01\x84a)\xDAV[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a8\xBDWa8\xBDa2\xE1V[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a8\xECWa8\xECa8\xC7V[P\x04\x90V[`\0\x82a9\0Wa9\0a8\xC7V[P\x06\x90V\xFEA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FPUser.registerOperatorWithChurn: input length mismatchUser.registerOperatorWithChurn: input quorums have common bits(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\xA2dipfsX\"\x12 T\x12\xFC\xCA\xB6\xA3\x1AR\xE0\xC0Pwb\xD4E?\xACK\xA8\xD9\xB9\xD0\xBA#m\xC5J\xCB\xA8\x8D>\xC4dsolcC\0\x08\x0C\x003",
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
    constructor(string name, uint256 _privKey, IBLSApkRegistry.PubkeyRegistrationParams _pubkeyParams);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub name: alloy::sol_types::private::String,
        pub _privKey: alloy::sol_types::private::primitives::aliases::U256,
        pub _pubkeyParams:
            <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                IBLSApkRegistry::PubkeyRegistrationParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
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
                    (value.name, value._privKey, value._pubkeyParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        name: tuple.0,
                        _privKey: tuple.1,
                        _pubkeyParams: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                IBLSApkRegistry::PubkeyRegistrationParams,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._privKey),
                    <IBLSApkRegistry::PubkeyRegistrationParams as alloy_sol_types::SolType>::tokenize(
                        &self._pubkeyParams,
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
    /**Function with signature `deregisterOperator(bytes)` and selector `0xca4f2d97`.
    ```solidity
    function deregisterOperator(bytes memory quorums) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {
        pub quorums: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`deregisterOperator(bytes)`](deregisterOperatorCall) function.
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
            impl ::core::convert::From<deregisterOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.quorums,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorums: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
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
                        &self.quorums,
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
    /**Function with signature `exitEigenlayer()` and selector `0x65eda8e5`.
    ```solidity
    function exitEigenlayer() external returns (address[] memory, uint256[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct exitEigenlayerCall {}
    ///Container type for the return parameters of the [`exitEigenlayer()`](exitEigenlayerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct exitEigenlayerReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub _1:
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
            impl ::core::convert::From<exitEigenlayerCall> for UnderlyingRustTuple<'_> {
                fn from(value: exitEigenlayerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exitEigenlayerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<exitEigenlayerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: exitEigenlayerReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exitEigenlayerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for exitEigenlayerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = exitEigenlayerReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "exitEigenlayer()";
            const SELECTOR: [u8; 4] = [101u8, 237u8, 168u8, 229u8];
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
    /**Function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`.
    ```solidity
    function isValidSignature(bytes32 digestHash, bytes memory) external view returns (bytes4);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureCall {
        pub digestHash: alloy::sol_types::private::FixedBytes<32>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureCall> for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureCall) -> Self {
                    (value.digestHash, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        digestHash: tuple.0,
                        _1: tuple.1,
                    }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidSignatureReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidSignatureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    > as alloy_sol_types::SolType>::tokenize(&self.digestHash),
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `operatorId()` and selector `0xbf68b816`.
    ```solidity
    function operatorId() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorIdCall {}
    ///Container type for the return parameters of the [`operatorId()`](operatorIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorIdReturn {
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
            impl ::core::convert::From<operatorIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorIdCall {
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
            impl ::core::convert::From<operatorIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorId()";
            const SELECTOR: [u8; 4] = [191u8, 104u8, 184u8, 22u8];
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
    /**Function with signature `pubkeyG1()` and selector `0xafa1c737`.
    ```solidity
    function pubkeyG1() external view returns (BN254.G1Point memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyG1Call {}
    ///Container type for the return parameters of the [`pubkeyG1()`](pubkeyG1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyG1Return {
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
            impl ::core::convert::From<pubkeyG1Call> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyG1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyG1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<pubkeyG1Return> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyG1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyG1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pubkeyG1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyG1Return;
            type ReturnTuple<'a> = (BN254::G1Point,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pubkeyG1()";
            const SELECTOR: [u8; 4] = [175u8, 161u8, 199u8, 55u8];
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
    /**Function with signature `registerOperator(bytes)` and selector `0x8231b54c`.
    ```solidity
    function registerOperator(bytes memory quorums) external returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        pub quorums: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(bytes)`](registerOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorReturn {
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
            impl ::core::convert::From<registerOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.quorums,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorums: tuple.0 }
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
            impl ::core::convert::From<registerOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(bytes)";
            const SELECTOR: [u8; 4] = [130u8, 49u8, 181u8, 76u8];
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
                        &self.quorums,
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
    /**Function with signature `registerOperatorWithChurn(bytes,address[],bytes)` and selector `0xa5f6cc1a`.
    ```solidity
    function registerOperatorWithChurn(bytes memory churnQuorums, address[] memory churnTargets, bytes memory standardQuorums) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithChurnCall {
        pub churnQuorums: alloy::sol_types::private::Bytes,
        pub churnTargets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub standardQuorums: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperatorWithChurn(bytes,address[],bytes)`](registerOperatorWithChurnCall) function.
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<registerOperatorWithChurnCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithChurnCall) -> Self {
                    (
                        value.churnQuorums,
                        value.churnTargets,
                        value.standardQuorums,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorWithChurnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        churnQuorums: tuple.0,
                        churnTargets: tuple.1,
                        standardQuorums: tuple.2,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorWithChurnReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorWithChurn(bytes,address[],bytes)";
            const SELECTOR: [u8; 4] = [165u8, 246u8, 204u8, 26u8];
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
                        &self.churnQuorums,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.churnTargets),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.standardQuorums,
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
    /**Function with signature `updateStakes()` and selector `0x505377e2`.
    ```solidity
    function updateStakes() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateStakesCall {}
    ///Container type for the return parameters of the [`updateStakes()`](updateStakesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateStakesReturn {}
    #[allow(
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
            impl ::core::convert::From<updateStakesCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateStakesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateStakesCall {
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
            impl ::core::convert::From<updateStakesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateStakesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateStakesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateStakesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateStakesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateStakes()";
            const SELECTOR: [u8; 4] = [80u8, 83u8, 119u8, 226u8];
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
    ///Container for all the [`User`](self) function calls.
    pub enum UserCalls {
        IS_TEST(IS_TESTCall),
        NAME(NAMECall),
        depositIntoEigenlayer(depositIntoEigenlayerCall),
        deregisterOperator(deregisterOperatorCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSenders(excludeSendersCall),
        exitEigenlayer(exitEigenlayerCall),
        failed(failedCall),
        isValidSignature(isValidSignatureCall),
        operatorId(operatorIdCall),
        pubkeyG1(pubkeyG1Call),
        registerAsOperator(registerAsOperatorCall),
        registerOperator(registerOperatorCall),
        registerOperatorWithChurn(registerOperatorWithChurnCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetInterfaces(targetInterfacesCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
        updateStakes(updateStakesCall),
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
            [22u8, 38u8, 186u8, 126u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 52u8, 173u8, 232u8],
            [42u8, 222u8, 56u8, 128u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [80u8, 83u8, 119u8, 226u8],
            [101u8, 237u8, 168u8, 229u8],
            [102u8, 217u8, 169u8, 160u8],
            [109u8, 51u8, 111u8, 88u8],
            [130u8, 49u8, 181u8, 76u8],
            [133u8, 34u8, 108u8, 129u8],
            [145u8, 106u8, 23u8, 198u8],
            [163u8, 244u8, 223u8, 126u8],
            [165u8, 246u8, 204u8, 26u8],
            [175u8, 161u8, 199u8, 55u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [191u8, 104u8, 184u8, 22u8],
            [202u8, 79u8, 45u8, 151u8],
            [226u8, 12u8, 159u8, 113u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for UserCalls {
        const NAME: &'static str = "UserCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 22usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::NAME(_) => <NAMECall as alloy_sol_types::SolCall>::SELECTOR,
                Self::depositIntoEigenlayer(_) => {
                    <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::exitEigenlayer(_) => {
                    <exitEigenlayerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isValidSignature(_) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorId(_) => <operatorIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pubkeyG1(_) => <pubkeyG1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::registerAsOperator(_) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator(_) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperatorWithChurn(_) => {
                    <registerOperatorWithChurnCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::updateStakes(_) => <updateStakesCall as alloy_sol_types::SolCall>::SELECTOR,
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
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<UserCalls>] = &[
                {
                    fn isValidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <isValidSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::isValidSignature)
                    }
                    isValidSignature
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn registerAsOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
                        )
                        .map(UserCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
                        )
                        .map(UserCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn updateStakes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <updateStakesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::updateStakes)
                    }
                    updateStakes
                },
                {
                    fn exitEigenlayer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <exitEigenlayerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::exitEigenlayer)
                    }
                    exitEigenlayer
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn depositIntoEigenlayer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::depositIntoEigenlayer)
                    }
                    depositIntoEigenlayer
                },
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn NAME(data: &[u8], validate: bool) -> alloy_sol_types::Result<UserCalls> {
                        <NAMECall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(UserCalls::NAME)
                    }
                    NAME
                },
                {
                    fn registerOperatorWithChurn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <registerOperatorWithChurnCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::registerOperatorWithChurn)
                    }
                    registerOperatorWithChurn
                },
                {
                    fn pubkeyG1(data: &[u8], validate: bool) -> alloy_sol_types::Result<UserCalls> {
                        <pubkeyG1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(UserCalls::pubkeyG1)
                    }
                    pubkeyG1
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(data: &[u8], validate: bool) -> alloy_sol_types::Result<UserCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(UserCalls::failed)
                    }
                    failed
                },
                {
                    fn operatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <operatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(UserCalls::operatorId)
                    }
                    operatorId
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(UserCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn IS_TEST(data: &[u8], validate: bool) -> alloy_sol_types::Result<UserCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(UserCalls::IS_TEST)
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
                Self::depositIntoEigenlayer(inner) => {
                    <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::exitEigenlayer(inner) => {
                    <exitEigenlayerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::operatorId(inner) => {
                    <operatorIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pubkeyG1(inner) => {
                    <pubkeyG1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperatorWithChurn(inner) => {
                    <registerOperatorWithChurnCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::updateStakes(inner) => {
                    <updateStakesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::depositIntoEigenlayer(inner) => {
                    <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::exitEigenlayer(inner) => {
                    <exitEigenlayerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::operatorId(inner) => {
                    <operatorIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pubkeyG1(inner) => {
                    <pubkeyG1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperatorWithChurn(inner) => {
                    <registerOperatorWithChurnCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
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
                Self::updateStakes(inner) => {
                    <updateStakesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
    impl alloy_sol_types::private::IntoLogData for UserEvents {
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
    /**Creates a new wrapper around an on-chain [`User`](self) contract instance.

    See the [wrapper's documentation](`UserInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> UserInstance<T, P, N> {
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
        _privKey: alloy::sol_types::private::primitives::aliases::U256,
        _pubkeyParams: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<UserInstance<T, P, N>>> {
        UserInstance::<T, P, N>::deploy(provider, name, _privKey, _pubkeyParams)
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
        _privKey: alloy::sol_types::private::primitives::aliases::U256,
        _pubkeyParams: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        UserInstance::<T, P, N>::deploy_builder(provider, name, _privKey, _pubkeyParams)
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
        > UserInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`User`](self) contract instance.

        See the [wrapper's documentation](`UserInstance`) for more details.*/
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
            _privKey: alloy::sol_types::private::primitives::aliases::U256,
            _pubkeyParams: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::Result<UserInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, name, _privKey, _pubkeyParams);
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
            _privKey: alloy::sol_types::private::primitives::aliases::U256,
            _pubkeyParams: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        name,
                        _privKey,
                        _pubkeyParams,
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
        > UserInstance<T, P, N>
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
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
            quorums: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(&deregisterOperatorCall { quorums })
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
        ///Creates a new call builder for the [`exitEigenlayer`] function.
        pub fn exitEigenlayer(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, exitEigenlayerCall, N> {
            self.call_builder(&exitEigenlayerCall {})
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<T, &P, failedCall, N> {
            self.call_builder(&failedCall {})
        }
        ///Creates a new call builder for the [`isValidSignature`] function.
        pub fn isValidSignature(
            &self,
            digestHash: alloy::sol_types::private::FixedBytes<32>,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidSignatureCall, N> {
            self.call_builder(&isValidSignatureCall { digestHash, _1 })
        }
        ///Creates a new call builder for the [`operatorId`] function.
        pub fn operatorId(&self) -> alloy_contract::SolCallBuilder<T, &P, operatorIdCall, N> {
            self.call_builder(&operatorIdCall {})
        }
        ///Creates a new call builder for the [`pubkeyG1`] function.
        pub fn pubkeyG1(&self) -> alloy_contract::SolCallBuilder<T, &P, pubkeyG1Call, N> {
            self.call_builder(&pubkeyG1Call {})
        }
        ///Creates a new call builder for the [`registerAsOperator`] function.
        pub fn registerAsOperator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerAsOperatorCall, N> {
            self.call_builder(&registerAsOperatorCall {})
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            quorums: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(&registerOperatorCall { quorums })
        }
        ///Creates a new call builder for the [`registerOperatorWithChurn`] function.
        pub fn registerOperatorWithChurn(
            &self,
            churnQuorums: alloy::sol_types::private::Bytes,
            churnTargets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            standardQuorums: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorWithChurnCall, N> {
            self.call_builder(&registerOperatorWithChurnCall {
                churnQuorums,
                churnTargets,
                standardQuorums,
            })
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
        ///Creates a new call builder for the [`updateStakes`] function.
        pub fn updateStakes(&self) -> alloy_contract::SolCallBuilder<T, &P, updateStakesCall, N> {
            self.call_builder(&updateStakesCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > UserInstance<T, P, N>
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
