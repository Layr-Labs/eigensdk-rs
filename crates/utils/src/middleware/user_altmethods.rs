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
pub mod User_AltMethods {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60806040526007805460ff199081166001908117909255600b80549091169091179055601c80546001600160a01b031916737109709ecfa91a80626ff3989d68f67f5b1dd12d17905560006034553480156200005a57600080fd5b5060405162005013380380620050138339810160408190526200007d9162000b68565b8282826000339050806001600160a01b0316636d14a9876040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000c4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000ea919062000c5f565b602080546001600160a01b0319166001600160a01b039283161781556040805163359d539760e11b8152905192841692636b3aa72e926004808401939192918290030181865afa15801562000143573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000169919062000c5f565b601f80546001600160a01b0319166001600160a01b039283161790556020805460408051633998fdd360e01b815290519190931692633998fdd39260048083019391928290030181865afa158015620001c6573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001ec919062000c5f565b602180546001600160a01b0319166001600160a01b039283161790556020805460408051632efa2ca360e11b815290519190931692635df459469260048083019391928290030181865afa15801562000249573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200026f919062000c5f565b602280546001600160a01b0319166001600160a01b039283161790556020805460408051636830483560e01b81529051919093169263683048359260048083019391928290030181865afa158015620002cc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620002f2919062000c5f565b602380546001600160a01b0319166001600160a01b039283161790556020805460408051634f4c91e160e11b815290519190931692639e9923c29260048083019391928290030181865afa1580156200034f573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000375919062000c5f565b602480546001600160a01b0319166001600160a01b039283161790556023546040805163df5cf72360e01b81529051919092169163df5cf7239160048083019260209291908290030181865afa158015620003d4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620003fa919062000c5f565b601d80546001600160a01b0319166001600160a01b0392909216918217905560408051630736e1c760e31b815290516339b70e38916004808201926020929091908290030181865afa15801562000455573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200047b919062000c5f565b601e80546001600160a01b0319166001600160a01b039283161790556021546040805163359d539760e11b815290519190921691636b3aa72e9160048083019260209291908290030181865afa158015620004da573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000500919062000c5f565b601f60006101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316633dfb40e06040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000565573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200058b919062000c5f565b602560006101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316632dbcb04c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015620005f0573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000616919062000c86565b602681905550806001600160a01b031663054310e66040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200065b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000681919062000c5f565b602780546001600160a01b0319166001600160a01b03929092169190911790558351620006b6906028906020870190620008a8565b50602a83905581518051602b908155602091820151602c55818401518051602d5590910151602e556040830151805184929190602f90620006fb908290600262000937565b50602082015162000713906002808401919062000937565b5050602054604051630f0a9fd360e21b8152306004820152600094506001600160a01b039091169250633c2a7f4c91506024016040805180830381865afa15801562000763573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000789919062000ca0565b9050620007a7602a5482620007f160201b620022a81790919060201c565b8051602b55602090810151602c5560408051808201909152602d548152602e5481830152620007df916200233f62000891821b17901c565b6029555062000cfc9650505050505050565b60408051808201909152600080825260208201526200080f62000967565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa9050808015620008445762000846565bfe5b5080620008895760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b604482015260640160405180910390fd5b505092915050565b805160009081526020918201519091526040902090565b828054620008b69062000cbf565b90600052602060002090601f016020900481019282620008da576000855562000925565b82601f10620008f557805160ff191683800117855562000925565b8280016001018555821562000925579182015b828111156200092557825182559160200191906001019062000908565b506200093392915062000985565b5090565b82600281019282156200092557916020028201828111156200092557825182559160200191906001019062000908565b60405180606001604052806003906020820280368337509192915050565b5b8082111562000933576000815560010162000986565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b0381118282101715620009d757620009d76200099c565b60405290565b604051601f8201601f191681016001600160401b038111828210171562000a085762000a086200099c565b604052919050565b60006040828403121562000a2357600080fd5b604080519081016001600160401b038111828210171562000a485762000a486200099c565b604052825181526020928301519281019290925250919050565b600082601f83011262000a7457600080fd5b62000a7e620009b2565b80604084018581111562000a9157600080fd5b845b8181101562000aad57805184526020938401930162000a93565b509095945050505050565b600081830361010081121562000acd57600080fd5b604051606081016001600160401b038111828210171562000af25762000af26200099c565b60405291508162000b04858562000a10565b815262000b15856040860162000a10565b60208201526080607f198301121562000b2d57600080fd5b62000b37620009b2565b915062000b48856080860162000a62565b825262000b598560c0860162000a62565b60208301526040015292915050565b6000806000610140848603121562000b7f57600080fd5b83516001600160401b038082111562000b9757600080fd5b818601915086601f83011262000bac57600080fd5b81518181111562000bc15762000bc16200099c565b6020915062000bd9601f8201601f19168301620009dd565b818152888383860101111562000bee57600080fd5b60005b8281101562000c0e57848101840151828201850152830162000bf1565b8281111562000c205760008484840101525b508096505050808601519350505062000c3d856040860162000ab8565b90509250925092565b6001600160a01b038116811462000c5c57600080fd5b50565b60006020828403121562000c7257600080fd5b815162000c7f8162000c46565b9392505050565b60006020828403121562000c9957600080fd5b5051919050565b60006040828403121562000cb357600080fd5b62000c7f838362000a10565b600181811c9082168062000cd457607f821691505b6020821081141562000cf657634e487b7160e01b600052602260045260246000fd5b50919050565b6143078062000d0c6000396000f3fe608060405234801561001057600080fd5b50600436106101425760003560e01c806385226c81116100b8578063b5508aa91161007c578063b5508aa9146102a6578063ba414fa6146102ae578063bf68b816146102c6578063ca4f2d97146102cf578063e20c9f71146102e2578063fa7626d4146102ea57600080fd5b806385226c8114610223578063916a17c614610238578063a3f4df7e14610240578063a5f6cc1a14610255578063afa1c7371461026857600080fd5b80633f7286f41161010a5780633f7286f4146101b4578063505377e2146101bc57806365eda8e5146101c457806366d9a9a0146101da5780636d336f58146101ef5780638231b54c1461020257600080fd5b80631626ba7e146101475780631ed7831c146101785780632a34ade81461018d5780632ade3880146101975780633e5e3c23146101ac575b600080fd5b61015a6101553660046130dc565b6102f7565b6040516001600160e01b031990911681526020015b60405180910390f35b610180610326565b60405161016f9190613165565b610195610388565b005b61019f61052d565b60405161016f919061320e565b61018061066f565b6101806106cf565b61019561072f565b6101cc610bc2565b60405161016f929190613342565b6101e2610e6b565b60405161016f9190613370565b6101956101fd3660046134c1565b610f51565b6102156102103660046135ca565b611236565b60405190815260200161016f565b61022b6113f5565b60405161016f919061360b565b6101e26114c5565b6102486115ab565b60405161016f919061366d565b610195610263366004613680565b611639565b604080518082018252600080825260209182015281518083018352602d54808252602e5491830191825283519081529051918101919091520161016f565b61022b611dec565b6102b6611ebc565b604051901515815260200161016f565b61021560295481565b6101956102dd3660046135ca565b611fe9565b610180612248565b6007546102b69060ff1681565b60008281526033602052604081205460ff161561031c5750630b135d3f60e11b610320565b5060005b92915050565b6060601480548060200260200160405190810160405280929190818152602001828054801561037e57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610360575b5050505050905090565b601c546001600160a01b0316631f7b4f306103a4436001613760565b6040518263ffffffff1660e01b81526004016103c291815260200190565b600060405180830381600087803b1580156103dc57600080fd5b505af11580156103f0573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610449573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061046d9190613778565b506104ac6040518060400160405280601981526020017f726567697374657241734f70657261746f722028636f72652900000000000000815250612356565b6040805160608101825230815260006020820181905281830152601d549151630f589e5960e01b815290916001600160a01b031690630f589e59906104f8908490602890600401613843565b600060405180830381600087803b15801561051257600080fd5b505af1158015610526573d6000803e3d6000fd5b5050505050565b6060601b805480602002602001604051908101604052809291908181526020016000905b8282101561066657600084815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b8282101561064f5783829060005260206000200180546105c290613791565b80601f01602080910402602001604051908101604052809291908181526020018280546105ee90613791565b801561063b5780601f106106105761010080835404028352916020019161063b565b820191906000526020600020905b81548152906001019060200180831161061e57829003601f168201915b5050505050815260200190600101906105a3565b505050508152505081526020019060010190610551565b50505050905090565b6060601680548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b6060601580548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b601c546001600160a01b0316631f7b4f3061074b436001613760565b6040518263ffffffff1660e01b815260040161076991815260200190565b600060405180830381600087803b15801561078357600080fd5b505af1158015610797573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156107f0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108149190613778565b506108366040518060600160405280602781526020016142ab60279139612356565b60006108ca6001602060009054906101000a90046001600160a01b03166001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610890573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108b49190613894565b60ff166001901b6108c591906138af565b61239e565b9050600081516001600160401b038111156108e7576108e761306f565b60405190808252806020026020018201604052801561091a57816020015b60608152602001906001900390816109055790505b50905060005b8251811015610b5957600083828151811061093d5761093d6138c6565b016020015160248054604051638902624560e01b815260f89390931c600484018190524363ffffffff16928401929092529092506000916001600160a01b0390911690638902624590604401600060405180830381865afa1580156109a6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526109ce91908101906138dc565b905080516001600160401b038111156109e9576109e961306f565b604051908082528060200260200182016040528015610a12578160200160208202803683370190505b50848481518110610a2557610a256138c6565b602002602001018190525060005b8151811015610b215760225482516001600160a01b03909116906347b314e890849084908110610a6557610a656138c6565b60200260200101516040518263ffffffff1660e01b8152600401610a8b91815260200190565b602060405180830381865afa158015610aa8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610acc919061396c565b858581518110610ade57610ade6138c6565b60200260200101518281518110610af757610af76138c6565b6001600160a01b039092166020928302919091019091015280610b1981613989565b915050610a33565b50610b44848481518110610b3757610b376138c6565b602002602001015161246a565b50508080610b5190613989565b915050610920565b50602054604051630a2814a960e31b81526001600160a01b0390911690635140a54890610b8c90849086906004016139a4565b600060405180830381600087803b158015610ba657600080fd5b505af1158015610bba573d6000803e3d6000fd5b505050505050565b601c5460609081906001600160a01b0316631f7b4f30610be3436001613760565b6040518263ffffffff1660e01b8152600401610c0191815260200190565b600060405180830381600087803b158015610c1b57600080fd5b505af1158015610c2f573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610c88573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cac9190613778565b50610ce36040518060400160405280601581526020017465786974456967656e6c617965722028636f72652960581b815250612356565b601d546040516367c0439f60e11b815230600482015260009182916001600160a01b039091169063cf80873e90602401600060405180830381865afa158015610d30573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610d589190810190613a9d565b60408051600180825281830190925292945090925060009190816020015b60408051606080820183528082526020820152600091810191909152815260200190600190039081610d765790505090506040518060600160405280848152602001838152602001306001600160a01b031681525081600081518110610dde57610dde6138c6565b6020908102919091010152601d546040516306ec6e8160e11b81526001600160a01b0390911690630dd8dd0290610e19908490600401613b57565b6000604051808303816000875af1158015610e38573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610e6091908101906138dc565b509193509150509091565b60606019805480602002602001604051908101604052809291908181526020016000905b828210156106665760008481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015610f3957602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411610efb5790505b50505050508152505081526020019060010190610e8f565b601c546001600160a01b0316631f7b4f30610f6d436001613760565b6040518263ffffffff1660e01b8152600401610f8b91815260200190565b600060405180830381600087803b158015610fa557600080fd5b505af1158015610fb9573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015611012573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110369190613778565b506110756040518060400160405280601c81526020017f6465706f736974496e746f456967656e4c617965722028636f72652900000000815250612356565b60005b8251811015611231576000838281518110611095576110956138c6565b6020026020010151905060008383815181106110b3576110b36138c6565b602002602001015190506000826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa1580156110fd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611121919061396c565b601e5460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303816000875af1158015611176573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061119a9190613bf1565b50601e546040516373d0285560e11b81526001600160a01b0385811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303816000875af11580156111f6573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061121a9190613778565b50505050808061122990613989565b915050611078565b505050565b601c546000906001600160a01b0316631f7b4f30611255436001613760565b6040518263ffffffff1660e01b815260040161127391815260200190565b600060405180830381600087803b15801561128d57600080fd5b505af11580156112a1573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156112fa573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061131e9190613778565b506113526040518060400160405280601081526020016f3932b3b4b9ba32b927b832b930ba37b960811b815250848461259b565b6020546001600160a01b031663a50857bf84846028602b611371612620565b6040518663ffffffff1660e01b8152600401611391959493929190613cd6565b600060405180830381600087803b1580156113ab57600080fd5b505af11580156113bf573d6000803e3d6000fd5b5050604080518082018252602d54808252602e5460209283019081526000918252519091522091506113ee9050565b9392505050565b60606018805480602002602001604051908101604052809291908181526020016000905b8282101561066657838290600052602060002001805461143890613791565b80601f016020809104026020016040519081016040528092919081815260200182805461146490613791565b80156114b15780601f10611486576101008083540402835291602001916114b1565b820191906000526020600020905b81548152906001019060200180831161149457829003601f168201915b505050505081526020019060010190611419565b6060601a805480602002602001604051908101604052809291908181526020016000905b828210156106665760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561159357602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116115555790505b505050505081525050815260200190600101906114e9565b602880546115b890613791565b80601f01602080910402602001604051908101604052809291908181526020018280546115e490613791565b80156116315780601f1061160657610100808354040283529160200191611631565b820191906000526020600020905b81548152906001019060200180831161161457829003601f168201915b505050505081565b601c546001600160a01b0316631f7b4f30611655436001613760565b6040518263ffffffff1660e01b815260040161167391815260200190565b600060405180830381600087803b15801561168d57600080fd5b505af11580156116a1573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156116fa573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061171e9190613778565b506117f96040518060400160405280601981526020017f72656769737465724f70657261746f7257697468436875726e0000000000000081525087878080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525050604080516020808b0282810182019093528a82529093508a92508991829185019084908082843760009201919091525050604080516020601f8a01819004810282018101909252888152925088915087908190840183828082843760009201919091525061272892505050565b600061183a87878080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506129ab92505050565b9050600061187d84848080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506129ab92505050565b604080516060810190915260358082529192506118a591899188916142186020830139612b38565b6118d38183166001600160c01b0316156040518060600160405280603e815260200161424d603e9139612b6e565b60006118ec6001600160c01b038381169085161761239e565b9050600081516001600160401b038111156119095761190961306f565b60405190808252806020026020018201604052801561194e57816020015b60408051808201909152600080825260208201528152602001906001900390816119275790505b5090506000805b83516119618284613760565b1015611b7c57818b14156119bc5760408051808201909152600080825260208201528361198e8385613760565b8151811061199e5761199e6138c6565b602002602001018190525080806119b490613989565b915050611955565b80871480611a0f57508787828181106119d7576119d76138c6565b909101356001600160f81b03191690508c8c848181106119f9576119f96138c6565b9050013560f81c60f81b6001600160f81b031916105b15611aaa5760405180604001604052808d8d85818110611a3157611a316138c6565b919091013560f81c8252506020018b8b85818110611a5157611a516138c6565b9050602002016020810190611a669190613d2d565b6001600160a01b0316905283611a7c8385613760565b81518110611a8c57611a8c6138c6565b60200260200101819052508180611aa290613989565b925050611955565b8b8b83818110611abc57611abc6138c6565b909101356001600160f81b0319169050888883818110611ade57611ade6138c6565b9050013560f81c60f81b6001600160f81b0319161015611b175760408051808201909152600080825260208201528361198e8385613760565b60405162461bcd60e51b815260206004820152602f60248201527f557365722e72656769737465724f70657261746f7257697468436875726e3a2060448201526e1b585b199bdc9b5959081a5b9c1d5d608a1b60648201526084015b60405180910390fd5b6000603460008154611b8d90613989565b91829055506040805160208101929092526bffffffffffffffffffffffff193060601b169082015260540160408051601f1981840301815290829052805160209182012090546029546384ca521360e01b8452919350600019926000926001600160a01b03909216916384ca521391611c109130918b9089908990600401613d91565b602060405180830381865afa158015611c2d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c519190613778565b601c546026546040516338d07aa960e21b8152929350600092839283926001600160a01b039091169163e341eaa491611c97918890600401918252602082015260400190565b606060405180830381865afa158015611cb4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611cd89190613dcc565b604080516041808252608082019092529396509194509250600091906020820181803683370190505090508260208201528160408201528360f81b8160018351611d2291906138af565b81518110611d3257611d326138c6565b60200101906001600160f81b031916908160001a9053506040805160608101825282815260208082018a905291810188905290546001600160a01b0316639b5d177b8d6028602b8f86611d83612620565b6040518763ffffffff1660e01b8152600401611da496959493929190613e01565b600060405180830381600087803b158015611dbe57600080fd5b505af1158015611dd2573d6000803e3d6000fd5b505050505050505050505050505050505050505050505050565b60606017805480602002602001604051908101604052809291908181526020016000905b82821015610666578382906000526020600020018054611e2f90613791565b80601f0160208091040260200160405190810160405280929190818152602001828054611e5b90613791565b8015611ea85780601f10611e7d57610100808354040283529160200191611ea8565b820191906000526020600020905b815481529060010190602001808311611e8b57829003601f168201915b505050505081526020019060010190611e10565b600754600090610100900460ff1615611ede5750600754610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611fe45760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091611f6c917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001613e82565b60408051601f1981840301815290829052611f8691613eb3565b6000604051808303816000865af19150503d8060008114611fc3576040519150601f19603f3d011682016040523d82523d6000602084013e611fc8565b606091505b5091505080806020019051810190611fe09190613bf1565b9150505b919050565b601c546001600160a01b0316631f7b4f30612005436001613760565b6040518263ffffffff1660e01b815260040161202391815260200190565b600060405180830381600087803b15801561203d57600080fd5b505af1158015612051573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156120aa573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906120ce9190613778565b5061210f6040518060400160405280601a81526020017f646572656769737465724f70657261746f722028656a65637429000000000000815250838361259b565b60208054604080516328f61b3160e01b815290516000936001600160a01b03909316926328f61b3192600480820193918290030181865afa158015612158573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061217c919061396c565b601c5460405163ca669fa760e01b81526001600160a01b03808416600483015292935091169063ca669fa790602401600060405180830381600087803b1580156121c557600080fd5b505af11580156121d9573d6000803e3d6000fd5b5050602054604051636e3b17db60e01b81526001600160a01b039091169250636e3b17db915061221190309087908790600401613ecf565b600060405180830381600087803b15801561222b57600080fd5b505af115801561223f573d6000803e3d6000fd5b50505050505050565b6060601380548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b60408051808201909152600080825260208201526122c4613051565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa90508080156122f7576122f9565bfe5b50806123375760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401611b73565b505092915050565b805160009081526020918201519091526040902090565b6000805160206141f8833981519152602882604051602001612379929190613ef4565b60408051601f19818403018152908290526123939161366d565b60405180910390a150565b60606000806123ac84612ba1565b61ffff166001600160401b038111156123c7576123c761306f565b6040519080825280601f01601f1916602001820160405280156123f1576020820181803683370190505b5090506000805b825182108015612409575061010081105b15612460576001811b935085841615612450578060f81b838381518110612432576124326138c6565b60200101906001600160f81b031916908160001a9053508160010191505b61245981613989565b90506123f8565b5090949350505050565b60015b815181101561259757600082828151811061248a5761248a6138c6565b6020026020010151905060006001836124a391906138af565b90505b816001600160a01b03168482815181106124c2576124c26138c6565b60200260200101516001600160a01b03161115612545578381815181106124eb576124eb6138c6565b6020026020010151848260016125019190613760565b81518110612511576125116138c6565b6001600160a01b03909216602092830291909101909101528061253357612545565b8061253d81613f84565b9150506124a6565b8184612552836001613760565b81518110612562576125626138c6565b60200260200101906001600160a01b031690816001600160a01b0316815250505050808061258f90613989565b91505061246d565b5050565b60008051602061428b8339815191526028846040516020016125be929190613ef4565b60408051601f198184030181526020601f860181900481028401810190925284835291612605918690869081908401838280828437600092019190915250612bcc92505050565b604051612613929190613f9b565b60405180910390a1505050565b60408051606080820183528082526000602083018190528284018190528351918201818152608083019094529192819081526020016034600081548092919061266890613989565b909155508152600019602091820152601f54602154918301516040808501519051631420c19160e31b81523060048201526001600160a01b0394851660248201526044810192909252606482015292935060009291169063a1060c8890608401602060405180830381865afa1580156126e5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906127099190613778565b6000908152603360205260409020805460ff1916600117905550919050565b6000805160206141f883398151915260288560405160200161274b929190613ef4565b60408051601f19818403018152908290526127659161366d565b60405180910390a160008051602061428b83398151915261278582612bcc565b6040516127929190613fc0565b60405180910390a160008051602061428b8339815191526127b284612bcc565b6040516127bf9190613ffb565b60405180910390a16040805180820190915260018152605b60f81b602082015260005b835181101561295c57600184516127f991906138af565b8114156128a75781848281518110612813576128136138c6565b60200260200101516001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa158015612858573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526128809190810190614033565b6040516020016128919291906140a0565b604051602081830303815290604052915061294a565b818482815181106128ba576128ba6138c6565b60200260200101516001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa1580156128ff573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526129279190810190614033565b6040516020016129389291906140cf565b60405160208183030381529060405291505b8061295481613989565b9150506127e2565b508060405160200161296e919061410b565b604051602081830303815290604052905060008051602061428b8339815191528160405161299c9190614130565b60405180910390a15050505050565b600061010082511115612a345760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a401611b73565b8151612a4257506000919050565b60008083600081518110612a5857612a586138c6565b0160200151600160f89190911c81901b92505b8451811015612b2f57848181518110612a8657612a866138c6565b0160200151600160f89190911c1b9150828211612b1b5760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a401611b73565b91811791612b2881613989565b9050612a6b565b50909392505050565b8183146112315760008051602061428b83398151915281604051612b5c9190614168565b60405180910390a16112318383612cc6565b816125975760008051602061428b83398151915281604051612b909190614168565b60405180910390a161259782612ddb565b6000805b821561032057612bb66001846138af565b9092169180612bc481614197565b915050612ba5565b6040805180820190915260018152605b60f81b602082015260609060005b8351811015612c9d5760018451612c0191906138af565b811415612c525781612c2b858381518110612c1e57612c1e6138c6565b016020015160f81c612e40565b604051602001612c3c9291906140a0565b6040516020818303038152906040529150612c8b565b81612c68858381518110612c1e57612c1e6138c6565b604051602001612c799291906140cf565b60405160208183030381529060405291505b80612c9581613989565b915050612bea565b5080604051602001612caf919061410b565b60408051601f198184030181529190529392505050565b808214612597576000805160206141f8833981519152604051612d259060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808080808081319599d60b21b60608201526020810184905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160408051818152600a81830152690808080808149a59da1d60b21b60608201526020810183905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a1612597612f45565b80612e3d576000805160206141f8833981519152604051612e2d9060208082526017908201527f4572726f723a20417373657274696f6e204661696c6564000000000000000000604082015260600190565b60405180910390a1612e3d612f45565b50565b606081612e645750506040805180820190915260018152600360fc1b602082015290565b8160005b8115612e8e5780612e7881613989565b9150612e879050600a836141cf565b9150612e68565b6000816001600160401b03811115612ea857612ea861306f565b6040519080825280601f01601f191660200182016040528015612ed2576020820181803683370190505b5090505b8415612f3d57612ee76001836138af565b9150612ef4600a866141e3565b612eff906030613760565b60f81b818381518110612f1457612f146138c6565b60200101906001600160f81b031916908160001a905350612f36600a866141cf565b9450612ed6565b949350505050565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156130405760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f1981840301815290829052612fdf9291602001613e82565b60408051601f1981840301815290829052612ff991613eb3565b6000604051808303816000865af19150503d8060008114613036576040519150601f19603f3d011682016040523d82523d6000602084013e61303b565b606091505b505050505b6007805461ff001916610100179055565b60405180606001604052806003906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b03811182821017156130ad576130ad61306f565b604052919050565b60006001600160401b038211156130ce576130ce61306f565b50601f01601f191660200190565b600080604083850312156130ef57600080fd5b8235915060208301356001600160401b0381111561310c57600080fd5b8301601f8101851361311d57600080fd5b803561313061312b826130b5565b613085565b81815286602083850101111561314557600080fd5b816020840160208301376000602083830101528093505050509250929050565b6020808252825182820181905260009190848201906040850190845b818110156131a65783516001600160a01b031683529284019291840191600101613181565b50909695505050505050565b60005b838110156131cd5781810151838201526020016131b5565b838111156131dc576000848401525b50505050565b600081518084526131fa8160208601602086016131b2565b601f01601f19169290920160200192915050565b602080825282518282018190526000919060409081850190600581811b8701840188860187805b858110156132be57603f198b8503018752825180516001600160a01b031685528901518985018990528051898601819052908a0190606081881b870181019190870190855b818110156132a857605f198985030183526132968486516131e2565b948e01949350918d019160010161327a565b505050978a019794505091880191600101613235565b50919a9950505050505050505050565b600081518084526020808501945080840160005b838110156133075781516001600160a01b0316875295820195908201906001016132e2565b509495945050505050565b600081518084526020808501945080840160005b8381101561330757815187529582019590820190600101613326565b60408152600061335560408301856132ce565b82810360208401526133678185613312565b95945050505050565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b8481101561341457898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b808310156133ff5783516001600160e01b0319168252928b019260019290920191908b01906133d5565b50978a01979550505091870191600101613398565b50919998505050505050505050565b60006001600160401b0382111561343c5761343c61306f565b5060051b60200190565b6001600160a01b0381168114612e3d57600080fd5b600082601f83011261346c57600080fd5b8135602061347c61312b83613423565b82815260059290921b8401810191818101908684111561349b57600080fd5b8286015b848110156134b6578035835291830191830161349f565b509695505050505050565b600080604083850312156134d457600080fd5b82356001600160401b03808211156134eb57600080fd5b818501915085601f8301126134ff57600080fd5b8135602061350f61312b83613423565b82815260059290921b8401810191818101908984111561352e57600080fd5b948201945b8386101561355557853561354681613446565b82529482019490820190613533565b9650508601359250508082111561356b57600080fd5b506135788582860161345b565b9150509250929050565b60008083601f84011261359457600080fd5b5081356001600160401b038111156135ab57600080fd5b6020830191508360208285010111156135c357600080fd5b9250929050565b600080602083850312156135dd57600080fd5b82356001600160401b038111156135f357600080fd5b6135ff85828601613582565b90969095509350505050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b8281101561366057603f1988860301845261364e8583516131e2565b94509285019290850190600101613632565b5092979650505050505050565b6020815260006113ee60208301846131e2565b6000806000806000806060878903121561369957600080fd5b86356001600160401b03808211156136b057600080fd5b6136bc8a838b01613582565b909850965060208901359150808211156136d557600080fd5b818901915089601f8301126136e957600080fd5b8135818111156136f857600080fd5b8a60208260051b850101111561370d57600080fd5b60208301965080955050604089013591508082111561372b57600080fd5b5061373889828a01613582565b979a9699509497509295939492505050565b634e487b7160e01b600052601160045260246000fd5b600082198211156137735761377361374a565b500190565b60006020828403121561378a57600080fd5b5051919050565b600181811c908216806137a557607f821691505b602082108114156137c657634e487b7160e01b600052602260045260246000fd5b50919050565b600081546137d981613791565b8085526020600183811680156137f6576001811461380a57613838565b60ff19851688840152604088019550613838565b866000528260002060005b858110156138305781548a8201860152908301908401613815565b890184019650505b505050505092915050565b600060018060a01b038085511683528060208601511660208401525063ffffffff604085015116604083015260806060830152612f3d60808301846137cc565b805160ff81168114611fe457600080fd5b6000602082840312156138a657600080fd5b6113ee82613883565b6000828210156138c1576138c161374a565b500390565b634e487b7160e01b600052603260045260246000fd5b600060208083850312156138ef57600080fd5b82516001600160401b0381111561390557600080fd5b8301601f8101851361391657600080fd5b805161392461312b82613423565b81815260059190911b8201830190838101908783111561394357600080fd5b928401925b8284101561396157835182529284019290840190613948565b979650505050505050565b60006020828403121561397e57600080fd5b81516113ee81613446565b600060001982141561399d5761399d61374a565b5060010190565b6000604082016040835280855180835260608501915060608160051b860101925060208088016000805b84811015613a2b57888703605f19018652825180518089529085019085890190845b81811015613a155783516001600160a01b0316835292870192918701916001016139f0565b50909850505094830194918301916001016139ce565b5050508584038187015250505061336781856131e2565b600082601f830112613a5357600080fd5b81516020613a6361312b83613423565b82815260059290921b84018101918181019086841115613a8257600080fd5b8286015b848110156134b65780518352918301918301613a86565b60008060408385031215613ab057600080fd5b82516001600160401b0380821115613ac757600080fd5b818501915085601f830112613adb57600080fd5b81516020613aeb61312b83613423565b82815260059290921b84018101918181019089841115613b0a57600080fd5b948201945b83861015613b31578551613b2281613446565b82529482019490820190613b0f565b91880151919650909350505080821115613b4a57600080fd5b5061357885828601613a42565b60006020808301818452808551808352604092508286019150828160051b87010184880160005b83811015613be357603f19898403018552815160608151818652613ba4828701826132ce565b915050888201518582038a870152613bbc8282613312565b928901516001600160a01b0316958901959095525094870194925090860190600101613b7e565b509098975050505050505050565b600060208284031215613c0357600080fd5b815180151581146113ee57600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b8060005b60028110156131dc578154845260209093019260019182019101613c40565b80548252600181015460208301526002810154604083015260038101546060830152613c916080830160048301613c3c565b61259760c0830160068301613c3c565b6000815160608452613cb660608501826131e2565b905060208301516020850152604083015160408501528091505092915050565b6000610160808352613ceb818401888a613c13565b90508281036020840152613cff81876137cc565b9050613d0e6040840186613c5f565b828103610140840152613d218185613ca1565b98975050505050505050565b600060208284031215613d3f57600080fd5b81356113ee81613446565b600081518084526020808501945080840160005b83811015613307578151805160ff1688528301516001600160a01b03168388015260409096019590820190600101613d5e565b60018060a01b038616815284602082015260a060408201526000613db860a0830186613d4a565b606083019490945250608001529392505050565b600080600060608486031215613de157600080fd5b613dea84613883565b925060208401519150604084015190509250925092565b60006101a0808352613e158184018a6131e2565b90508281036020840152613e2981896137cc565b9050613e386040840188613c5f565b828103610140840152613e4b8187613d4a565b9050828103610160840152613e608186613ca1565b9050828103610180840152613e758185613ca1565b9998505050505050505050565b6001600160e01b0319831681528151600090613ea58160048501602087016131b2565b919091016004019392505050565b60008251613ec58184602087016131b2565b9190910192915050565b6001600160a01b03841681526040602082018190526000906133679083018486613c13565b6000808454613f0281613791565b60018281168015613f1a5760018114613f2b57613f5a565b60ff19841687528287019450613f5a565b8860005260208060002060005b85811015613f515781548a820152908401908201613f38565b50505082870194505b50601760f91b845286519250613f768382860160208a016131b2565b919092010195945050505050565b600081613f9357613f9361374a565b506000190190565b604081526000613fae60408301856131e2565b828103602084015261336781856131e2565b6040815260116040820152702d207374616e6461726451756f72756d7360781b60608201526080602082015260006113ee60808301846131e2565b60408152600e60408201526d2d20636875726e51756f72756d7360901b60608201526080602082015260006113ee60808301846131e2565b60006020828403121561404557600080fd5b81516001600160401b0381111561405b57600080fd5b8201601f8101841361406c57600080fd5b805161407a61312b826130b5565b81815285602083850101111561408f57600080fd5b6133678260208301602086016131b2565b600083516140b28184602088016131b2565b8351908301906140c68183602088016131b2565b01949350505050565b600083516140e18184602088016131b2565b8351908301906140f58183602088016131b2565b61016160f51b9101908152600201949350505050565b6000825161411d8184602087016131b2565b605d60f81b920191825250600101919050565b60408152600e60408201526d2d20636875726e5461726765747360901b60608201526080602082015260006113ee60808301846131e2565b60408152600560408201526422b93937b960d91b60608201526080602082015260006113ee60808301846131e2565b600061ffff808316818114156141af576141af61374a565b6001019392505050565b634e487b7160e01b600052601260045260246000fd5b6000826141de576141de6141b9565b500490565b6000826141f2576141f26141b9565b50069056fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50557365722e72656769737465724f70657261746f7257697468436875726e3a20696e707574206c656e677468206d69736d61746368557365722e72656769737465724f70657261746f7257697468436875726e3a20696e7075742071756f72756d73206861766520636f6d6d6f6e2062697473280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf35837570646174655374616b657320287570646174654f70657261746f7273466f7251756f72756d29a26469706673582212205b671206c61d23c42c96339fe5ec70fbc6dd8924b67cbec0bb85c21812db98c364736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x07\x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U`\x0B\x80T\x90\x91\x16\x90\x91\x17\x90U`\x1C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x17\x90U`\0`4U4\x80\x15b\0\0ZW`\0\x80\xFD[P`@Qb\0P\x138\x03\x80b\0P\x13\x839\x81\x01`@\x81\x90Rb\0\0}\x91b\0\x0BhV[\x82\x82\x82`\x003\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16cm\x14\xA9\x87`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xEA\x91\x90b\0\x0C_V[` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x81U`@\x80Qc5\x9DS\x97`\xE1\x1B\x81R\x90Q\x92\x84\x16\x92ck:\xA7.\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x01CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01i\x91\x90b\0\x0C_V[`\x1F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U` \x80T`@\x80Qc9\x98\xFD\xD3`\xE0\x1B\x81R\x90Q\x91\x90\x93\x16\x92c9\x98\xFD\xD3\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x01\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xEC\x91\x90b\0\x0C_V[`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U` \x80T`@\x80Qc.\xFA,\xA3`\xE1\x1B\x81R\x90Q\x91\x90\x93\x16\x92c]\xF4YF\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x02IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02o\x91\x90b\0\x0C_V[`\"\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U` \x80T`@\x80Qch0H5`\xE0\x1B\x81R\x90Q\x91\x90\x93\x16\x92ch0H5\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x02\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02\xF2\x91\x90b\0\x0C_V[`#\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U` \x80T`@\x80QcOL\x91\xE1`\xE1\x1B\x81R\x90Q\x91\x90\x93\x16\x92c\x9E\x99#\xC2\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x03OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x03u\x91\x90b\0\x0C_V[`$\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`#T`@\x80Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90Q\x91\x90\x92\x16\x91c\xDF\\\xF7#\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x03\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x03\xFA\x91\x90b\0\x0C_V[`\x1D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80Qc\x076\xE1\xC7`\xE3\x1B\x81R\x90Qc9\xB7\x0E8\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x04UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x04{\x91\x90b\0\x0C_V[`\x1E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`!T`@\x80Qc5\x9DS\x97`\xE1\x1B\x81R\x90Q\x91\x90\x92\x16\x91ck:\xA7.\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x04\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x05\0\x91\x90b\0\x0C_V[`\x1F`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c=\xFB@\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x05\x8B\x91\x90b\0\x0C_V[`%`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c-\xBC\xB0L`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06\x16\x91\x90b\0\x0C\x86V[`&\x81\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x05C\x10\xE6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06\x81\x91\x90b\0\x0C_V[`'\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x83Qb\0\x06\xB6\x90`(\x90` \x87\x01\x90b\0\x08\xA8V[P`*\x83\x90U\x81Q\x80Q`+\x90\x81U` \x91\x82\x01Q`,U\x81\x84\x01Q\x80Q`-U\x90\x91\x01Q`.U`@\x83\x01Q\x80Q\x84\x92\x91\x90`/\x90b\0\x06\xFB\x90\x82\x90`\x02b\0\t7V[P` \x82\x01Qb\0\x07\x13\x90`\x02\x80\x84\x01\x91\x90b\0\t7V[PP` T`@Qc\x0F\n\x9F\xD3`\xE2\x1B\x81R0`\x04\x82\x01R`\0\x94P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc<*\x7FL\x91P`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x07cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x07\x89\x91\x90b\0\x0C\xA0V[\x90Pb\0\x07\xA7`*T\x82b\0\x07\xF1` \x1Bb\0\"\xA8\x17\x90\x91\x90` \x1CV[\x80Q`+U` \x90\x81\x01Q`,U`@\x80Q\x80\x82\x01\x90\x91R`-T\x81R`.T\x81\x83\x01Rb\0\x07\xDF\x91b\0#?b\0\x08\x91\x82\x1B\x17\x90\x1CV[`)UPb\0\x0C\xFC\x96PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Rb\0\x08\x0Fb\0\tgV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15b\0\x08DWb\0\x08FV[\xFE[P\x80b\0\x08\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x82\x80Tb\0\x08\xB6\x90b\0\x0C\xBFV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0\x08\xDAW`\0\x85Ub\0\t%V[\x82`\x1F\x10b\0\x08\xF5W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0\t%V[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0\t%W\x91\x82\x01[\x82\x81\x11\x15b\0\t%W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\t\x08V[Pb\0\t3\x92\x91Pb\0\t\x85V[P\x90V[\x82`\x02\x81\x01\x92\x82\x15b\0\t%W\x91` \x02\x82\x01\x82\x81\x11\x15b\0\t%W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\t\x08V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15b\0\t3W`\0\x81U`\x01\x01b\0\t\x86V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\t\xD7Wb\0\t\xD7b\0\t\x9CV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\n\x08Wb\0\n\x08b\0\t\x9CV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15b\0\n#W`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\nHWb\0\nHb\0\t\x9CV[`@R\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12b\0\ntW`\0\x80\xFD[b\0\n~b\0\t\xB2V[\x80`@\x84\x01\x85\x81\x11\x15b\0\n\x91W`\0\x80\xFD[\x84[\x81\x81\x10\x15b\0\n\xADW\x80Q\x84R` \x93\x84\x01\x93\x01b\0\n\x93V[P\x90\x95\x94PPPPPV[`\0\x81\x83\x03a\x01\0\x81\x12\x15b\0\n\xCDW`\0\x80\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\n\xF2Wb\0\n\xF2b\0\t\x9CV[`@R\x91P\x81b\0\x0B\x04\x85\x85b\0\n\x10V[\x81Rb\0\x0B\x15\x85`@\x86\x01b\0\n\x10V[` \x82\x01R`\x80`\x7F\x19\x83\x01\x12\x15b\0\x0B-W`\0\x80\xFD[b\0\x0B7b\0\t\xB2V[\x91Pb\0\x0BH\x85`\x80\x86\x01b\0\nbV[\x82Rb\0\x0BY\x85`\xC0\x86\x01b\0\nbV[` \x83\x01R`@\x01R\x92\x91PPV[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15b\0\x0B\x7FW`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x0B\x97W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12b\0\x0B\xACW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x0B\xC1Wb\0\x0B\xC1b\0\t\x9CV[` \x91Pb\0\x0B\xD9`\x1F\x82\x01`\x1F\x19\x16\x83\x01b\0\t\xDDV[\x81\x81R\x88\x83\x83\x86\x01\x01\x11\x15b\0\x0B\xEEW`\0\x80\xFD[`\0[\x82\x81\x10\x15b\0\x0C\x0EW\x84\x81\x01\x84\x01Q\x82\x82\x01\x85\x01R\x83\x01b\0\x0B\xF1V[\x82\x81\x11\x15b\0\x0C W`\0\x84\x84\x84\x01\x01R[P\x80\x96PPP\x80\x86\x01Q\x93PPPb\0\x0C=\x85`@\x86\x01b\0\n\xB8V[\x90P\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x0C\\W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15b\0\x0CrW`\0\x80\xFD[\x81Qb\0\x0C\x7F\x81b\0\x0CFV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15b\0\x0C\x99W`\0\x80\xFD[PQ\x91\x90PV[`\0`@\x82\x84\x03\x12\x15b\0\x0C\xB3W`\0\x80\xFD[b\0\x0C\x7F\x83\x83b\0\n\x10V[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x0C\xD4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15b\0\x0C\xF6WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[aC\x07\x80b\0\r\x0C`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\xB8W\x80c\xB5P\x8A\xA9\x11a\0|W\x80c\xB5P\x8A\xA9\x14a\x02\xA6W\x80c\xBAAO\xA6\x14a\x02\xAEW\x80c\xBFh\xB8\x16\x14a\x02\xC6W\x80c\xCAO-\x97\x14a\x02\xCFW\x80c\xE2\x0C\x9Fq\x14a\x02\xE2W\x80c\xFAv&\xD4\x14a\x02\xEAW`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x02#W\x80c\x91j\x17\xC6\x14a\x028W\x80c\xA3\xF4\xDF~\x14a\x02@W\x80c\xA5\xF6\xCC\x1A\x14a\x02UW\x80c\xAF\xA1\xC77\x14a\x02hW`\0\x80\xFD[\x80c?r\x86\xF4\x11a\x01\nW\x80c?r\x86\xF4\x14a\x01\xB4W\x80cPSw\xE2\x14a\x01\xBCW\x80ce\xED\xA8\xE5\x14a\x01\xC4W\x80cf\xD9\xA9\xA0\x14a\x01\xDAW\x80cm3oX\x14a\x01\xEFW\x80c\x821\xB5L\x14a\x02\x02W`\0\x80\xFD[\x80c\x16&\xBA~\x14a\x01GW\x80c\x1E\xD7\x83\x1C\x14a\x01xW\x80c*4\xAD\xE8\x14a\x01\x8DW\x80c*\xDE8\x80\x14a\x01\x97W\x80c>^<#\x14a\x01\xACW[`\0\x80\xFD[a\x01Za\x01U6`\x04a0\xDCV[a\x02\xF7V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x80a\x03&V[`@Qa\x01o\x91\x90a1eV[a\x01\x95a\x03\x88V[\0[a\x01\x9Fa\x05-V[`@Qa\x01o\x91\x90a2\x0EV[a\x01\x80a\x06oV[a\x01\x80a\x06\xCFV[a\x01\x95a\x07/V[a\x01\xCCa\x0B\xC2V[`@Qa\x01o\x92\x91\x90a3BV[a\x01\xE2a\x0EkV[`@Qa\x01o\x91\x90a3pV[a\x01\x95a\x01\xFD6`\x04a4\xC1V[a\x0FQV[a\x02\x15a\x02\x106`\x04a5\xCAV[a\x126V[`@Q\x90\x81R` \x01a\x01oV[a\x02+a\x13\xF5V[`@Qa\x01o\x91\x90a6\x0BV[a\x01\xE2a\x14\xC5V[a\x02Ha\x15\xABV[`@Qa\x01o\x91\x90a6mV[a\x01\x95a\x02c6`\x04a6\x80V[a\x169V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`-T\x80\x82R`.T\x91\x83\x01\x91\x82R\x83Q\x90\x81R\x90Q\x91\x81\x01\x91\x90\x91R\x01a\x01oV[a\x02+a\x1D\xECV[a\x02\xB6a\x1E\xBCV[`@Q\x90\x15\x15\x81R` \x01a\x01oV[a\x02\x15`)T\x81V[a\x01\x95a\x02\xDD6`\x04a5\xCAV[a\x1F\xE9V[a\x01\x80a\"HV[`\x07Ta\x02\xB6\x90`\xFF\x16\x81V[`\0\x82\x81R`3` R`@\x81 T`\xFF\x16\x15a\x03\x1CWPc\x0B\x13]?`\xE1\x1Ba\x03 V[P`\0[\x92\x91PPV[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`W[PPPPP\x90P\x90V[`\x1CT`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x03\xA4C`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xC2\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xDCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xF0W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04m\x91\x90a7xV[Pa\x04\xAC`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FregisterAsOperator (core)\0\0\0\0\0\0\0\x81RPa#VV[`@\x80Q``\x81\x01\x82R0\x81R`\0` \x82\x01\x81\x90R\x81\x83\x01R`\x1DT\x91Qc\x0FX\x9EY`\xE0\x1B\x81R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x0FX\x9EY\x90a\x04\xF8\x90\x84\x90`(\x90`\x04\x01a8CV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05&W=`\0\x80>=`\0\xFD[PPPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06fW`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x06OW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x05\xC2\x90a7\x91V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xEE\x90a7\x91V[\x80\x15a\x06;W\x80`\x1F\x10a\x06\x10Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06;V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x1EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x05\xA3V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x05QV[PPPP\x90P\x90V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[`\x1CT`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x07KC`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07i\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x83W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x97W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x14\x91\x90a7xV[Pa\x086`@Q\x80``\x01`@R\x80`'\x81R` \x01aB\xAB`'\x919a#VV[`\0a\x08\xCA`\x01` `\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xB4\x91\x90a8\x94V[`\xFF\x16`\x01\x90\x1Ba\x08\xC5\x91\x90a8\xAFV[a#\x9EV[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xE7Wa\x08\xE7a0oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x1AW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\x05W\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x0BYW`\0\x83\x82\x81Q\x81\x10a\t=Wa\t=a8\xC6V[\x01` \x01Q`$\x80T`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x93\x90\x93\x1C`\x04\x84\x01\x81\x90RCc\xFF\xFF\xFF\xFF\x16\x92\x84\x01\x92\x90\x92R\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xCE\x91\x90\x81\x01\x90a8\xDCV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xE9Wa\t\xE9a0oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\x12W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x84\x84\x81Q\x81\x10a\n%Wa\n%a8\xC6V[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x0B!W`\"T\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG\xB3\x14\xE8\x90\x84\x90\x84\x90\x81\x10a\neWa\nea8\xC6V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x8B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xCC\x91\x90a9lV[\x85\x85\x81Q\x81\x10a\n\xDEWa\n\xDEa8\xC6V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\n\xF7Wa\n\xF7a8\xC6V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0B\x19\x81a9\x89V[\x91PPa\n3V[Pa\x0BD\x84\x84\x81Q\x81\x10a\x0B7Wa\x0B7a8\xC6V[` \x02` \x01\x01Qa$jV[PP\x80\x80a\x0BQ\x90a9\x89V[\x91PPa\t V[P` T`@Qc\n(\x14\xA9`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cQ@\xA5H\x90a\x0B\x8C\x90\x84\x90\x86\x90`\x04\x01a9\xA4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xBAW=`\0\x80>=`\0\xFD[PPPPPPV[`\x1CT``\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x0B\xE3C`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x01\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\x1BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C/W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAC\x91\x90a7xV[Pa\x0C\xE3`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01texitEigenlayer (core)`X\x1B\x81RPa#VV[`\x1DT`@Qcg\xC0C\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCF\x80\x87>\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\rX\x91\x90\x81\x01\x90a:\x9DV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90\x92P`\0\x91\x90\x81` \x01[`@\x80Q``\x80\x82\x01\x83R\x80\x82R` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\rvW\x90PP\x90P`@Q\x80``\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x010`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x81`\0\x81Q\x81\x10a\r\xDEWa\r\xDEa8\xC6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x1DT`@Qc\x06\xECn\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\r\xD8\xDD\x02\x90a\x0E\x19\x90\x84\x90`\x04\x01a;WV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E`\x91\x90\x81\x01\x90a8\xDCV[P\x91\x93P\x91PP\x90\x91V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06fW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0F9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0E\xFBW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0E\x8FV[`\x1CT`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x0FmC`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x8B\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xA5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xB9W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x10\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x106\x91\x90a7xV[Pa\x10u`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FdepositIntoEigenLayer (core)\0\0\0\0\x81RPa#VV[`\0[\x82Q\x81\x10\x15a\x121W`\0\x83\x82\x81Q\x81\x10a\x10\x95Wa\x10\x95a8\xC6V[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x10\xB3Wa\x10\xB3a8\xC6V[` \x02` \x01\x01Q\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11!\x91\x90a9lV[`\x1ET`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11vW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x9A\x91\x90a;\xF1V[P`\x1ET`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x1A\x91\x90a7xV[PPPP\x80\x80a\x12)\x90a9\x89V[\x91PPa\x10xV[PPPV[`\x1CT`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x12UC`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12s\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\x8DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xA1W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x1E\x91\x90a7xV[Pa\x13R`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o92\xB3\xB4\xB9\xBA2\xB9'\xB82\xB90\xBA7\xB9`\x81\x1B\x81RP\x84\x84a%\x9BV[` T`\x01`\x01`\xA0\x1B\x03\x16c\xA5\x08W\xBF\x84\x84`(`+a\x13qa& V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x91\x95\x94\x93\x92\x91\x90a<\xD6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xABW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xBFW=`\0\x80>=`\0\xFD[PP`@\x80Q\x80\x82\x01\x82R`-T\x80\x82R`.T` \x92\x83\x01\x90\x81R`\0\x91\x82RQ\x90\x91R \x91Pa\x13\xEE\x90PV[\x93\x92PPPV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06fW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x148\x90a7\x91V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14d\x90a7\x91V[\x80\x15a\x14\xB1W\x80`\x1F\x10a\x14\x86Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\xB1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\x94W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x14\x19V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06fW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x15\x93W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x15UW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x14\xE9V[`(\x80Ta\x15\xB8\x90a7\x91V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\xE4\x90a7\x91V[\x80\x15a\x161W\x80`\x1F\x10a\x16\x06Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x161V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\x14W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x1CT`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x16UC`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16s\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x8DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xA1W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x16\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x1E\x91\x90a7xV[Pa\x17\xF9`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FregisterOperatorWithChurn\0\0\0\0\0\0\0\x81RP\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` \x80\x8B\x02\x82\x81\x01\x82\x01\x90\x93R\x8A\x82R\x90\x93P\x8A\x92P\x89\x91\x82\x91\x85\x01\x90\x84\x90\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa'(\x92PPPV[`\0a\x18:\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\xAB\x92PPPV[\x90P`\0a\x18}\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\xAB\x92PPPV[`@\x80Q``\x81\x01\x90\x91R`5\x80\x82R\x91\x92Pa\x18\xA5\x91\x89\x91\x88\x91aB\x18` \x83\x019a+8V[a\x18\xD3\x81\x83\x16`\x01`\x01`\xC0\x1B\x03\x16\x15`@Q\x80``\x01`@R\x80`>\x81R` \x01aBM`>\x919a+nV[`\0a\x18\xEC`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17a#\x9EV[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\tWa\x19\ta0oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19NW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19'W\x90P[P\x90P`\0\x80[\x83Qa\x19a\x82\x84a7`V[\x10\x15a\x1B|W\x81\x8B\x14\x15a\x19\xBCW`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x83a\x19\x8E\x83\x85a7`V[\x81Q\x81\x10a\x19\x9EWa\x19\x9Ea8\xC6V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x19\xB4\x90a9\x89V[\x91PPa\x19UV[\x80\x87\x14\x80a\x1A\x0FWP\x87\x87\x82\x81\x81\x10a\x19\xD7Wa\x19\xD7a8\xC6V[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P\x8C\x8C\x84\x81\x81\x10a\x19\xF9Wa\x19\xF9a8\xC6V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x10[\x15a\x1A\xAAW`@Q\x80`@\x01`@R\x80\x8D\x8D\x85\x81\x81\x10a\x1A1Wa\x1A1a8\xC6V[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x01\x8B\x8B\x85\x81\x81\x10a\x1AQWa\x1AQa8\xC6V[\x90P` \x02\x01` \x81\x01\x90a\x1Af\x91\x90a=-V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x83a\x1A|\x83\x85a7`V[\x81Q\x81\x10a\x1A\x8CWa\x1A\x8Ca8\xC6V[` \x02` \x01\x01\x81\x90RP\x81\x80a\x1A\xA2\x90a9\x89V[\x92PPa\x19UV[\x8B\x8B\x83\x81\x81\x10a\x1A\xBCWa\x1A\xBCa8\xC6V[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P\x88\x88\x83\x81\x81\x10a\x1A\xDEWa\x1A\xDEa8\xC6V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x10\x15a\x1B\x17W`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x83a\x19\x8E\x83\x85a7`V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FUser.registerOperatorWithChurn: `D\x82\x01Rn\x1BX[\x19\x9B\xDC\x9BYY\x08\x1A[\x9C\x1D]`\x8A\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0`4`\0\x81Ta\x1B\x8D\x90a9\x89V[\x91\x82\x90UP`@\x80Q` \x81\x01\x92\x90\x92Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16\x90\x82\x01R`T\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x90T`)Tc\x84\xCAR\x13`\xE0\x1B\x84R\x91\x93P`\0\x19\x92`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x84\xCAR\x13\x91a\x1C\x10\x910\x91\x8B\x90\x89\x90\x89\x90`\x04\x01a=\x91V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CQ\x91\x90a7xV[`\x1CT`&T`@Qc8\xD0z\xA9`\xE2\x1B\x81R\x92\x93P`\0\x92\x83\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xE3A\xEA\xA4\x91a\x1C\x97\x91\x88\x90`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xD8\x91\x90a=\xCCV[`@\x80Q`A\x80\x82R`\x80\x82\x01\x90\x92R\x93\x96P\x91\x94P\x92P`\0\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x82` \x82\x01R\x81`@\x82\x01R\x83`\xF8\x1B\x81`\x01\x83Qa\x1D\"\x91\x90a8\xAFV[\x81Q\x81\x10a\x1D2Wa\x1D2a8\xC6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`@\x80Q``\x81\x01\x82R\x82\x81R` \x80\x82\x01\x8A\x90R\x91\x81\x01\x88\x90R\x90T`\x01`\x01`\xA0\x1B\x03\x16c\x9B]\x17{\x8D`(`+\x8F\x86a\x1D\x83a& V[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\xA4\x96\x95\x94\x93\x92\x91\x90a>\x01V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\xBEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D\xD2W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPPPPPPPPPPV[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06fW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1E/\x90a7\x91V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E[\x90a7\x91V[\x80\x15a\x1E\xA8W\x80`\x1F\x10a\x1E}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1E\x10V[`\x07T`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a\x1E\xDEWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1F\xE4W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x1Fl\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a>\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1F\x86\x91a>\xB3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1F\xC3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1F\xC8V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x1F\xE0\x91\x90a;\xF1V[\x91PP[\x91\x90PV[`\x1CT`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a \x05C`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a #\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a =W`\0\x80\xFD[PZ\xF1\x15\x80\x15a QW=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a \xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xCE\x91\x90a7xV[Pa!\x0F`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FderegisterOperator (eject)\0\0\0\0\0\0\x81RP\x83\x83a%\x9BV[` \x80T`@\x80Qc(\xF6\x1B1`\xE0\x1B\x81R\x90Q`\0\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c(\xF6\x1B1\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!|\x91\x90a9lV[`\x1CT`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xC5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\xD9W=`\0\x80>=`\0\xFD[PP` T`@Qcn;\x17\xDB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcn;\x17\xDB\x91Pa\"\x11\x900\x90\x87\x90\x87\x90`\x04\x01a>\xCFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\"+W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"?W=`\0\x80>=`\0\xFD[PPPPPPPV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\"\xC4a0QV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\"\xF7Wa\"\xF9V[\xFE[P\x80a#7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x1BsV[PP\x92\x91PPV[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[`\0\x80Q` aA\xF8\x839\x81Q\x91R`(\x82`@Q` \x01a#y\x92\x91\x90a>\xF4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra#\x93\x91a6mV[`@Q\x80\x91\x03\x90\xA1PV[```\0\x80a#\xAC\x84a+\xA1V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xC7Wa#\xC7a0oV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#\xF1W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a$\tWPa\x01\0\x81\x10[\x15a$`W`\x01\x81\x1B\x93P\x85\x84\x16\x15a$PW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a$2Wa$2a8\xC6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a$Y\x81a9\x89V[\x90Pa#\xF8V[P\x90\x94\x93PPPPV[`\x01[\x81Q\x81\x10\x15a%\x97W`\0\x82\x82\x81Q\x81\x10a$\x8AWa$\x8Aa8\xC6V[` \x02` \x01\x01Q\x90P`\0`\x01\x83a$\xA3\x91\x90a8\xAFV[\x90P[\x81`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a$\xC2Wa$\xC2a8\xC6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a%EW\x83\x81\x81Q\x81\x10a$\xEBWa$\xEBa8\xC6V[` \x02` \x01\x01Q\x84\x82`\x01a%\x01\x91\x90a7`V[\x81Q\x81\x10a%\x11Wa%\x11a8\xC6V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a%3Wa%EV[\x80a%=\x81a?\x84V[\x91PPa$\xA6V[\x81\x84a%R\x83`\x01a7`V[\x81Q\x81\x10a%bWa%ba8\xC6V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPPP\x80\x80a%\x8F\x90a9\x89V[\x91PPa$mV[PPV[`\0\x80Q` aB\x8B\x839\x81Q\x91R`(\x84`@Q` \x01a%\xBE\x92\x91\x90a>\xF4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` `\x1F\x86\x01\x81\x90\x04\x81\x02\x84\x01\x81\x01\x90\x92R\x84\x83R\x91a&\x05\x91\x86\x90\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa+\xCC\x92PPPV[`@Qa&\x13\x92\x91\x90a?\x9BV[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x80\x82\x01\x83R\x80\x82R`\0` \x83\x01\x81\x90R\x82\x84\x01\x81\x90R\x83Q\x91\x82\x01\x81\x81R`\x80\x83\x01\x90\x94R\x91\x92\x81\x90\x81R` \x01`4`\0\x81T\x80\x92\x91\x90a&h\x90a9\x89V[\x90\x91UP\x81R`\0\x19` \x91\x82\x01R`\x1FT`!T\x91\x83\x01Q`@\x80\x85\x01Q\x90Qc\x14 \xC1\x91`\xE3\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`$\x82\x01R`D\x81\x01\x92\x90\x92R`d\x82\x01R\x92\x93P`\0\x92\x91\x16\x90c\xA1\x06\x0C\x88\x90`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\t\x91\x90a7xV[`\0\x90\x81R`3` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UP\x91\x90PV[`\0\x80Q` aA\xF8\x839\x81Q\x91R`(\x85`@Q` \x01a'K\x92\x91\x90a>\xF4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra'e\x91a6mV[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` aB\x8B\x839\x81Q\x91Ra'\x85\x82a+\xCCV[`@Qa'\x92\x91\x90a?\xC0V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` aB\x8B\x839\x81Q\x91Ra'\xB2\x84a+\xCCV[`@Qa'\xBF\x91\x90a?\xFBV[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`[`\xF8\x1B` \x82\x01R`\0[\x83Q\x81\x10\x15a)\\W`\x01\x84Qa'\xF9\x91\x90a8\xAFV[\x81\x14\x15a(\xA7W\x81\x84\x82\x81Q\x81\x10a(\x13Wa(\x13a8\xC6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(\x80\x91\x90\x81\x01\x90a@3V[`@Q` \x01a(\x91\x92\x91\x90a@\xA0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa)JV[\x81\x84\x82\x81Q\x81\x10a(\xBAWa(\xBAa8\xC6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra)'\x91\x90\x81\x01\x90a@3V[`@Q` \x01a)8\x92\x91\x90a@\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P[\x80a)T\x81a9\x89V[\x91PPa'\xE2V[P\x80`@Q` \x01a)n\x91\x90aA\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80Q` aB\x8B\x839\x81Q\x91R\x81`@Qa)\x9C\x91\x90aA0V[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\0a\x01\0\x82Q\x11\x15a*4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x1BsV[\x81Qa*BWP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a*XWa*Xa8\xC6V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a+/W\x84\x81\x81Q\x81\x10a*\x86Wa*\x86a8\xC6V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a+\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x1BsV[\x91\x81\x17\x91a+(\x81a9\x89V[\x90Pa*kV[P\x90\x93\x92PPPV[\x81\x83\x14a\x121W`\0\x80Q` aB\x8B\x839\x81Q\x91R\x81`@Qa+\\\x91\x90aAhV[`@Q\x80\x91\x03\x90\xA1a\x121\x83\x83a,\xC6V[\x81a%\x97W`\0\x80Q` aB\x8B\x839\x81Q\x91R\x81`@Qa+\x90\x91\x90aAhV[`@Q\x80\x91\x03\x90\xA1a%\x97\x82a-\xDBV[`\0\x80[\x82\x15a\x03 Wa+\xB6`\x01\x84a8\xAFV[\x90\x92\x16\x91\x80a+\xC4\x81aA\x97V[\x91PPa+\xA5V[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`[`\xF8\x1B` \x82\x01R``\x90`\0[\x83Q\x81\x10\x15a,\x9DW`\x01\x84Qa,\x01\x91\x90a8\xAFV[\x81\x14\x15a,RW\x81a,+\x85\x83\x81Q\x81\x10a,\x1EWa,\x1Ea8\xC6V[\x01` \x01Q`\xF8\x1Ca.@V[`@Q` \x01a,<\x92\x91\x90a@\xA0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa,\x8BV[\x81a,h\x85\x83\x81Q\x81\x10a,\x1EWa,\x1Ea8\xC6V[`@Q` \x01a,y\x92\x91\x90a@\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P[\x80a,\x95\x81a9\x89V[\x91PPa+\xEAV[P\x80`@Q` \x01a,\xAF\x91\x90aA\x0BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[\x80\x82\x14a%\x97W`\0\x80Q` aA\xF8\x839\x81Q\x91R`@Qa-%\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x08\x13\x19Y\x9D`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x14\x9AY\xDA\x1D`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a%\x97a/EV[\x80a.=W`\0\x80Q` aA\xF8\x839\x81Q\x91R`@Qa.-\x90` \x80\x82R`\x17\x90\x82\x01R\x7FError: Assertion Failed\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1a.=a/EV[PV[``\x81a.dWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[\x81\x15a.\x8EW\x80a.x\x81a9\x89V[\x91Pa.\x87\x90P`\n\x83aA\xCFV[\x91Pa.hV[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xA8Wa.\xA8a0oV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a.\xD2W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a/=Wa.\xE7`\x01\x83a8\xAFV[\x91Pa.\xF4`\n\x86aA\xE3V[a.\xFF\x90`0a7`V[`\xF8\x1B\x81\x83\x81Q\x81\x10a/\x14Wa/\x14a8\xC6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SPa/6`\n\x86aA\xCFV[\x94Pa.\xD6V[\x94\x93PPPPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a0@W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra/\xDF\x92\x91` \x01a>\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra/\xF9\x91a>\xB3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a06W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a0;V[``\x91P[PPPP[`\x07\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a0\xADWa0\xADa0oV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a0\xCEWa0\xCEa0oV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a0\xEFW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\x0CW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a1\x1DW`\0\x80\xFD[\x805a10a1+\x82a0\xB5V[a0\x85V[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a1EW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a1\xA6W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a1\x81V[P\x90\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15a1\xCDW\x81\x81\x01Q\x83\x82\x01R` \x01a1\xB5V[\x83\x81\x11\x15a1\xDCW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra1\xFA\x81` \x86\x01` \x86\x01a1\xB2V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a2\xBEW`?\x19\x8B\x85\x03\x01\x87R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a2\xA8W`_\x19\x89\x85\x03\x01\x83Ra2\x96\x84\x86Qa1\xE2V[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a2zV[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a25V[P\x91\x9A\x99PPPPPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a3\x07W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a2\xE2V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a3\x07W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a3&V[`@\x81R`\0a3U`@\x83\x01\x85a2\xCEV[\x82\x81\x03` \x84\x01Ra3g\x81\x85a3\x12V[\x95\x94PPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a4\x14W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a3\xFFW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a3\xD5V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a3\x98V[P\x91\x99\x98PPPPPPPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a4<Wa4<a0oV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a.=W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a4lW`\0\x80\xFD[\x815` a4|a1+\x83a4#V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a4\x9BW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a4\xB6W\x805\x83R\x91\x83\x01\x91\x83\x01a4\x9FV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a4\xD4W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a4\xEBW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a4\xFFW`\0\x80\xFD[\x815` a5\x0Fa1+\x83a4#V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a5.W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a5UW\x855a5F\x81a4FV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a53V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a5kW`\0\x80\xFD[Pa5x\x85\x82\x86\x01a4[V[\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a5\x94W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xABW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a5\xC3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a5\xDDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xF3W`\0\x80\xFD[a5\xFF\x85\x82\x86\x01a5\x82V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a6`W`?\x19\x88\x86\x03\x01\x84Ra6N\x85\x83Qa1\xE2V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a62V[P\x92\x97\x96PPPPPPPV[` \x81R`\0a\x13\xEE` \x83\x01\x84a1\xE2V[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a6\x99W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a6\xB0W`\0\x80\xFD[a6\xBC\x8A\x83\x8B\x01a5\x82V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a6\xD5W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a6\xE9W`\0\x80\xFD[\x815\x81\x81\x11\x15a6\xF8W`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15a7\rW`\0\x80\xFD[` \x83\x01\x96P\x80\x95PP`@\x89\x015\x91P\x80\x82\x11\x15a7+W`\0\x80\xFD[Pa78\x89\x82\x8A\x01a5\x82V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a7sWa7sa7JV[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a7\x8AW`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a7\xA5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a7\xC6WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x81Ta7\xD9\x81a7\x91V[\x80\x85R` `\x01\x83\x81\x16\x80\x15a7\xF6W`\x01\x81\x14a8\nWa88V[`\xFF\x19\x85\x16\x88\x84\x01R`@\x88\x01\x95Pa88V[\x86`\0R\x82`\0 `\0[\x85\x81\x10\x15a80W\x81T\x8A\x82\x01\x86\x01R\x90\x83\x01\x90\x84\x01a8\x15V[\x89\x01\x84\x01\x96PP[PPPPP\x92\x91PPV[`\0`\x01\x80`\xA0\x1B\x03\x80\x85Q\x16\x83R\x80` \x86\x01Q\x16` \x84\x01RPc\xFF\xFF\xFF\xFF`@\x85\x01Q\x16`@\x83\x01R`\x80``\x83\x01Ra/=`\x80\x83\x01\x84a7\xCCV[\x80Q`\xFF\x81\x16\x81\x14a\x1F\xE4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a8\xA6W`\0\x80\xFD[a\x13\xEE\x82a8\x83V[`\0\x82\x82\x10\x15a8\xC1Wa8\xC1a7JV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a8\xEFW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a9\x05W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a9\x16W`\0\x80\xFD[\x80Qa9$a1+\x82a4#V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a9CW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a9aW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a9HV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a9~W`\0\x80\xFD[\x81Qa\x13\xEE\x81a4FV[`\0`\0\x19\x82\x14\x15a9\x9DWa9\x9Da7JV[P`\x01\x01\x90V[`\0`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x80\x88\x01`\0\x80[\x84\x81\x10\x15a:+W\x88\x87\x03`_\x19\x01\x86R\x82Q\x80Q\x80\x89R\x90\x85\x01\x90\x85\x89\x01\x90\x84[\x81\x81\x10\x15a:\x15W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x87\x01\x92\x91\x87\x01\x91`\x01\x01a9\xF0V[P\x90\x98PPP\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a9\xCEV[PPP\x85\x84\x03\x81\x87\x01RPPPa3g\x81\x85a1\xE2V[`\0\x82`\x1F\x83\x01\x12a:SW`\0\x80\xFD[\x81Q` a:ca1+\x83a4#V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a:\x82W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a4\xB6W\x80Q\x83R\x91\x83\x01\x91\x83\x01a:\x86V[`\0\x80`@\x83\x85\x03\x12\x15a:\xB0W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\xC7W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a:\xDBW`\0\x80\xFD[\x81Q` a:\xEBa1+\x83a4#V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a;\nW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a;1W\x85Qa;\"\x81a4FV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a;\x0FV[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15a;JW`\0\x80\xFD[Pa5x\x85\x82\x86\x01a:BV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a;\xE3W`?\x19\x89\x84\x03\x01\x85R\x81Q``\x81Q\x81\x86Ra;\xA4\x82\x87\x01\x82a2\xCEV[\x91PP\x88\x82\x01Q\x85\x82\x03\x8A\x87\x01Ra;\xBC\x82\x82a3\x12V[\x92\x89\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x95\x89\x01\x95\x90\x95RP\x94\x87\x01\x94\x92P\x90\x86\x01\x90`\x01\x01a;~V[P\x90\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a<\x03W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\xEEW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x80`\0[`\x02\x81\x10\x15a1\xDCW\x81T\x84R` \x90\x93\x01\x92`\x01\x91\x82\x01\x91\x01a<@V[\x80T\x82R`\x01\x81\x01T` \x83\x01R`\x02\x81\x01T`@\x83\x01R`\x03\x81\x01T``\x83\x01Ra<\x91`\x80\x83\x01`\x04\x83\x01a<<V[a%\x97`\xC0\x83\x01`\x06\x83\x01a<<V[`\0\x81Q``\x84Ra<\xB6``\x85\x01\x82a1\xE2V[\x90P` \x83\x01Q` \x85\x01R`@\x83\x01Q`@\x85\x01R\x80\x91PP\x92\x91PPV[`\0a\x01`\x80\x83Ra<\xEB\x81\x84\x01\x88\x8Aa<\x13V[\x90P\x82\x81\x03` \x84\x01Ra<\xFF\x81\x87a7\xCCV[\x90Pa=\x0E`@\x84\x01\x86a<_V[\x82\x81\x03a\x01@\x84\x01Ra=!\x81\x85a<\xA1V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a=?W`\0\x80\xFD[\x815a\x13\xEE\x81a4FV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a3\x07W\x81Q\x80Q`\xFF\x16\x88R\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a=^V[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0a=\xB8`\xA0\x83\x01\x86a=JV[``\x83\x01\x94\x90\x94RP`\x80\x01R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a=\xE1W`\0\x80\xFD[a=\xEA\x84a8\x83V[\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0a\x01\xA0\x80\x83Ra>\x15\x81\x84\x01\x8Aa1\xE2V[\x90P\x82\x81\x03` \x84\x01Ra>)\x81\x89a7\xCCV[\x90Pa>8`@\x84\x01\x88a<_V[\x82\x81\x03a\x01@\x84\x01Ra>K\x81\x87a=JV[\x90P\x82\x81\x03a\x01`\x84\x01Ra>`\x81\x86a<\xA1V[\x90P\x82\x81\x03a\x01\x80\x84\x01Ra>u\x81\x85a<\xA1V[\x99\x98PPPPPPPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a>\xA5\x81`\x04\x85\x01` \x87\x01a1\xB2V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa>\xC5\x81\x84` \x87\x01a1\xB2V[\x91\x90\x91\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a3g\x90\x83\x01\x84\x86a<\x13V[`\0\x80\x84Ta?\x02\x81a7\x91V[`\x01\x82\x81\x16\x80\x15a?\x1AW`\x01\x81\x14a?+Wa?ZV[`\xFF\x19\x84\x16\x87R\x82\x87\x01\x94Pa?ZV[\x88`\0R` \x80`\0 `\0[\x85\x81\x10\x15a?QW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a?8V[PPP\x82\x87\x01\x94P[P`\x17`\xF9\x1B\x84R\x86Q\x92Pa?v\x83\x82\x86\x01` \x8A\x01a1\xB2V[\x91\x90\x92\x01\x01\x95\x94PPPPPV[`\0\x81a?\x93Wa?\x93a7JV[P`\0\x19\x01\x90V[`@\x81R`\0a?\xAE`@\x83\x01\x85a1\xE2V[\x82\x81\x03` \x84\x01Ra3g\x81\x85a1\xE2V[`@\x81R`\x11`@\x82\x01Rp- standardQuorums`x\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x13\xEE`\x80\x83\x01\x84a1\xE2V[`@\x81R`\x0E`@\x82\x01Rm- churnQuorums`\x90\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x13\xEE`\x80\x83\x01\x84a1\xE2V[`\0` \x82\x84\x03\x12\x15a@EW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a@[W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a@lW`\0\x80\xFD[\x80Qa@za1+\x82a0\xB5V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a@\x8FW`\0\x80\xFD[a3g\x82` \x83\x01` \x86\x01a1\xB2V[`\0\x83Qa@\xB2\x81\x84` \x88\x01a1\xB2V[\x83Q\x90\x83\x01\x90a@\xC6\x81\x83` \x88\x01a1\xB2V[\x01\x94\x93PPPPV[`\0\x83Qa@\xE1\x81\x84` \x88\x01a1\xB2V[\x83Q\x90\x83\x01\x90a@\xF5\x81\x83` \x88\x01a1\xB2V[a\x01a`\xF5\x1B\x91\x01\x90\x81R`\x02\x01\x94\x93PPPPV[`\0\x82QaA\x1D\x81\x84` \x87\x01a1\xB2V[`]`\xF8\x1B\x92\x01\x91\x82RP`\x01\x01\x91\x90PV[`@\x81R`\x0E`@\x82\x01Rm- churnTargets`\x90\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x13\xEE`\x80\x83\x01\x84a1\xE2V[`@\x81R`\x05`@\x82\x01Rd\"\xB997\xB9`\xD9\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x13\xEE`\x80\x83\x01\x84a1\xE2V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aA\xAFWaA\xAFa7JV[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aA\xDEWaA\xDEaA\xB9V[P\x04\x90V[`\0\x82aA\xF2WaA\xF2aA\xB9V[P\x06\x90V\xFEA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FPUser.registerOperatorWithChurn: input length mismatchUser.registerOperatorWithChurn: input quorums have common bits(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83updateStakes (updateOperatorsForQuorum)\xA2dipfsX\"\x12 [g\x12\x06\xC6\x1D#\xC4,\x963\x9F\xE5\xECp\xFB\xC6\xDD\x89$\xB6|\xBE\xC0\xBB\x85\xC2\x18\x12\xDB\x98\xC3dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106101425760003560e01c806385226c81116100b8578063b5508aa91161007c578063b5508aa9146102a6578063ba414fa6146102ae578063bf68b816146102c6578063ca4f2d97146102cf578063e20c9f71146102e2578063fa7626d4146102ea57600080fd5b806385226c8114610223578063916a17c614610238578063a3f4df7e14610240578063a5f6cc1a14610255578063afa1c7371461026857600080fd5b80633f7286f41161010a5780633f7286f4146101b4578063505377e2146101bc57806365eda8e5146101c457806366d9a9a0146101da5780636d336f58146101ef5780638231b54c1461020257600080fd5b80631626ba7e146101475780631ed7831c146101785780632a34ade81461018d5780632ade3880146101975780633e5e3c23146101ac575b600080fd5b61015a6101553660046130dc565b6102f7565b6040516001600160e01b031990911681526020015b60405180910390f35b610180610326565b60405161016f9190613165565b610195610388565b005b61019f61052d565b60405161016f919061320e565b61018061066f565b6101806106cf565b61019561072f565b6101cc610bc2565b60405161016f929190613342565b6101e2610e6b565b60405161016f9190613370565b6101956101fd3660046134c1565b610f51565b6102156102103660046135ca565b611236565b60405190815260200161016f565b61022b6113f5565b60405161016f919061360b565b6101e26114c5565b6102486115ab565b60405161016f919061366d565b610195610263366004613680565b611639565b604080518082018252600080825260209182015281518083018352602d54808252602e5491830191825283519081529051918101919091520161016f565b61022b611dec565b6102b6611ebc565b604051901515815260200161016f565b61021560295481565b6101956102dd3660046135ca565b611fe9565b610180612248565b6007546102b69060ff1681565b60008281526033602052604081205460ff161561031c5750630b135d3f60e11b610320565b5060005b92915050565b6060601480548060200260200160405190810160405280929190818152602001828054801561037e57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610360575b5050505050905090565b601c546001600160a01b0316631f7b4f306103a4436001613760565b6040518263ffffffff1660e01b81526004016103c291815260200190565b600060405180830381600087803b1580156103dc57600080fd5b505af11580156103f0573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610449573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061046d9190613778565b506104ac6040518060400160405280601981526020017f726567697374657241734f70657261746f722028636f72652900000000000000815250612356565b6040805160608101825230815260006020820181905281830152601d549151630f589e5960e01b815290916001600160a01b031690630f589e59906104f8908490602890600401613843565b600060405180830381600087803b15801561051257600080fd5b505af1158015610526573d6000803e3d6000fd5b5050505050565b6060601b805480602002602001604051908101604052809291908181526020016000905b8282101561066657600084815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b8282101561064f5783829060005260206000200180546105c290613791565b80601f01602080910402602001604051908101604052809291908181526020018280546105ee90613791565b801561063b5780601f106106105761010080835404028352916020019161063b565b820191906000526020600020905b81548152906001019060200180831161061e57829003601f168201915b5050505050815260200190600101906105a3565b505050508152505081526020019060010190610551565b50505050905090565b6060601680548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b6060601580548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b601c546001600160a01b0316631f7b4f3061074b436001613760565b6040518263ffffffff1660e01b815260040161076991815260200190565b600060405180830381600087803b15801561078357600080fd5b505af1158015610797573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156107f0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108149190613778565b506108366040518060600160405280602781526020016142ab60279139612356565b60006108ca6001602060009054906101000a90046001600160a01b03166001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610890573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108b49190613894565b60ff166001901b6108c591906138af565b61239e565b9050600081516001600160401b038111156108e7576108e761306f565b60405190808252806020026020018201604052801561091a57816020015b60608152602001906001900390816109055790505b50905060005b8251811015610b5957600083828151811061093d5761093d6138c6565b016020015160248054604051638902624560e01b815260f89390931c600484018190524363ffffffff16928401929092529092506000916001600160a01b0390911690638902624590604401600060405180830381865afa1580156109a6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526109ce91908101906138dc565b905080516001600160401b038111156109e9576109e961306f565b604051908082528060200260200182016040528015610a12578160200160208202803683370190505b50848481518110610a2557610a256138c6565b602002602001018190525060005b8151811015610b215760225482516001600160a01b03909116906347b314e890849084908110610a6557610a656138c6565b60200260200101516040518263ffffffff1660e01b8152600401610a8b91815260200190565b602060405180830381865afa158015610aa8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610acc919061396c565b858581518110610ade57610ade6138c6565b60200260200101518281518110610af757610af76138c6565b6001600160a01b039092166020928302919091019091015280610b1981613989565b915050610a33565b50610b44848481518110610b3757610b376138c6565b602002602001015161246a565b50508080610b5190613989565b915050610920565b50602054604051630a2814a960e31b81526001600160a01b0390911690635140a54890610b8c90849086906004016139a4565b600060405180830381600087803b158015610ba657600080fd5b505af1158015610bba573d6000803e3d6000fd5b505050505050565b601c5460609081906001600160a01b0316631f7b4f30610be3436001613760565b6040518263ffffffff1660e01b8152600401610c0191815260200190565b600060405180830381600087803b158015610c1b57600080fd5b505af1158015610c2f573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610c88573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cac9190613778565b50610ce36040518060400160405280601581526020017465786974456967656e6c617965722028636f72652960581b815250612356565b601d546040516367c0439f60e11b815230600482015260009182916001600160a01b039091169063cf80873e90602401600060405180830381865afa158015610d30573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610d589190810190613a9d565b60408051600180825281830190925292945090925060009190816020015b60408051606080820183528082526020820152600091810191909152815260200190600190039081610d765790505090506040518060600160405280848152602001838152602001306001600160a01b031681525081600081518110610dde57610dde6138c6565b6020908102919091010152601d546040516306ec6e8160e11b81526001600160a01b0390911690630dd8dd0290610e19908490600401613b57565b6000604051808303816000875af1158015610e38573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610e6091908101906138dc565b509193509150509091565b60606019805480602002602001604051908101604052809291908181526020016000905b828210156106665760008481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015610f3957602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411610efb5790505b50505050508152505081526020019060010190610e8f565b601c546001600160a01b0316631f7b4f30610f6d436001613760565b6040518263ffffffff1660e01b8152600401610f8b91815260200190565b600060405180830381600087803b158015610fa557600080fd5b505af1158015610fb9573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015611012573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110369190613778565b506110756040518060400160405280601c81526020017f6465706f736974496e746f456967656e4c617965722028636f72652900000000815250612356565b60005b8251811015611231576000838281518110611095576110956138c6565b6020026020010151905060008383815181106110b3576110b36138c6565b602002602001015190506000826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa1580156110fd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611121919061396c565b601e5460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303816000875af1158015611176573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061119a9190613bf1565b50601e546040516373d0285560e11b81526001600160a01b0385811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303816000875af11580156111f6573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061121a9190613778565b50505050808061122990613989565b915050611078565b505050565b601c546000906001600160a01b0316631f7b4f30611255436001613760565b6040518263ffffffff1660e01b815260040161127391815260200190565b600060405180830381600087803b15801561128d57600080fd5b505af11580156112a1573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156112fa573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061131e9190613778565b506113526040518060400160405280601081526020016f3932b3b4b9ba32b927b832b930ba37b960811b815250848461259b565b6020546001600160a01b031663a50857bf84846028602b611371612620565b6040518663ffffffff1660e01b8152600401611391959493929190613cd6565b600060405180830381600087803b1580156113ab57600080fd5b505af11580156113bf573d6000803e3d6000fd5b5050604080518082018252602d54808252602e5460209283019081526000918252519091522091506113ee9050565b9392505050565b60606018805480602002602001604051908101604052809291908181526020016000905b8282101561066657838290600052602060002001805461143890613791565b80601f016020809104026020016040519081016040528092919081815260200182805461146490613791565b80156114b15780601f10611486576101008083540402835291602001916114b1565b820191906000526020600020905b81548152906001019060200180831161149457829003601f168201915b505050505081526020019060010190611419565b6060601a805480602002602001604051908101604052809291908181526020016000905b828210156106665760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561159357602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116115555790505b505050505081525050815260200190600101906114e9565b602880546115b890613791565b80601f01602080910402602001604051908101604052809291908181526020018280546115e490613791565b80156116315780601f1061160657610100808354040283529160200191611631565b820191906000526020600020905b81548152906001019060200180831161161457829003601f168201915b505050505081565b601c546001600160a01b0316631f7b4f30611655436001613760565b6040518263ffffffff1660e01b815260040161167391815260200190565b600060405180830381600087803b15801561168d57600080fd5b505af11580156116a1573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156116fa573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061171e9190613778565b506117f96040518060400160405280601981526020017f72656769737465724f70657261746f7257697468436875726e0000000000000081525087878080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525050604080516020808b0282810182019093528a82529093508a92508991829185019084908082843760009201919091525050604080516020601f8a01819004810282018101909252888152925088915087908190840183828082843760009201919091525061272892505050565b600061183a87878080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506129ab92505050565b9050600061187d84848080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506129ab92505050565b604080516060810190915260358082529192506118a591899188916142186020830139612b38565b6118d38183166001600160c01b0316156040518060600160405280603e815260200161424d603e9139612b6e565b60006118ec6001600160c01b038381169085161761239e565b9050600081516001600160401b038111156119095761190961306f565b60405190808252806020026020018201604052801561194e57816020015b60408051808201909152600080825260208201528152602001906001900390816119275790505b5090506000805b83516119618284613760565b1015611b7c57818b14156119bc5760408051808201909152600080825260208201528361198e8385613760565b8151811061199e5761199e6138c6565b602002602001018190525080806119b490613989565b915050611955565b80871480611a0f57508787828181106119d7576119d76138c6565b909101356001600160f81b03191690508c8c848181106119f9576119f96138c6565b9050013560f81c60f81b6001600160f81b031916105b15611aaa5760405180604001604052808d8d85818110611a3157611a316138c6565b919091013560f81c8252506020018b8b85818110611a5157611a516138c6565b9050602002016020810190611a669190613d2d565b6001600160a01b0316905283611a7c8385613760565b81518110611a8c57611a8c6138c6565b60200260200101819052508180611aa290613989565b925050611955565b8b8b83818110611abc57611abc6138c6565b909101356001600160f81b0319169050888883818110611ade57611ade6138c6565b9050013560f81c60f81b6001600160f81b0319161015611b175760408051808201909152600080825260208201528361198e8385613760565b60405162461bcd60e51b815260206004820152602f60248201527f557365722e72656769737465724f70657261746f7257697468436875726e3a2060448201526e1b585b199bdc9b5959081a5b9c1d5d608a1b60648201526084015b60405180910390fd5b6000603460008154611b8d90613989565b91829055506040805160208101929092526bffffffffffffffffffffffff193060601b169082015260540160408051601f1981840301815290829052805160209182012090546029546384ca521360e01b8452919350600019926000926001600160a01b03909216916384ca521391611c109130918b9089908990600401613d91565b602060405180830381865afa158015611c2d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c519190613778565b601c546026546040516338d07aa960e21b8152929350600092839283926001600160a01b039091169163e341eaa491611c97918890600401918252602082015260400190565b606060405180830381865afa158015611cb4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611cd89190613dcc565b604080516041808252608082019092529396509194509250600091906020820181803683370190505090508260208201528160408201528360f81b8160018351611d2291906138af565b81518110611d3257611d326138c6565b60200101906001600160f81b031916908160001a9053506040805160608101825282815260208082018a905291810188905290546001600160a01b0316639b5d177b8d6028602b8f86611d83612620565b6040518763ffffffff1660e01b8152600401611da496959493929190613e01565b600060405180830381600087803b158015611dbe57600080fd5b505af1158015611dd2573d6000803e3d6000fd5b505050505050505050505050505050505050505050505050565b60606017805480602002602001604051908101604052809291908181526020016000905b82821015610666578382906000526020600020018054611e2f90613791565b80601f0160208091040260200160405190810160405280929190818152602001828054611e5b90613791565b8015611ea85780601f10611e7d57610100808354040283529160200191611ea8565b820191906000526020600020905b815481529060010190602001808311611e8b57829003601f168201915b505050505081526020019060010190611e10565b600754600090610100900460ff1615611ede5750600754610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611fe45760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091611f6c917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001613e82565b60408051601f1981840301815290829052611f8691613eb3565b6000604051808303816000865af19150503d8060008114611fc3576040519150601f19603f3d011682016040523d82523d6000602084013e611fc8565b606091505b5091505080806020019051810190611fe09190613bf1565b9150505b919050565b601c546001600160a01b0316631f7b4f30612005436001613760565b6040518263ffffffff1660e01b815260040161202391815260200190565b600060405180830381600087803b15801561203d57600080fd5b505af1158015612051573d6000803e3d6000fd5b50505050602560009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156120aa573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906120ce9190613778565b5061210f6040518060400160405280601a81526020017f646572656769737465724f70657261746f722028656a65637429000000000000815250838361259b565b60208054604080516328f61b3160e01b815290516000936001600160a01b03909316926328f61b3192600480820193918290030181865afa158015612158573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061217c919061396c565b601c5460405163ca669fa760e01b81526001600160a01b03808416600483015292935091169063ca669fa790602401600060405180830381600087803b1580156121c557600080fd5b505af11580156121d9573d6000803e3d6000fd5b5050602054604051636e3b17db60e01b81526001600160a01b039091169250636e3b17db915061221190309087908790600401613ecf565b600060405180830381600087803b15801561222b57600080fd5b505af115801561223f573d6000803e3d6000fd5b50505050505050565b6060601380548060200260200160405190810160405280929190818152602001828054801561037e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610360575050505050905090565b60408051808201909152600080825260208201526122c4613051565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa90508080156122f7576122f9565bfe5b50806123375760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401611b73565b505092915050565b805160009081526020918201519091526040902090565b6000805160206141f8833981519152602882604051602001612379929190613ef4565b60408051601f19818403018152908290526123939161366d565b60405180910390a150565b60606000806123ac84612ba1565b61ffff166001600160401b038111156123c7576123c761306f565b6040519080825280601f01601f1916602001820160405280156123f1576020820181803683370190505b5090506000805b825182108015612409575061010081105b15612460576001811b935085841615612450578060f81b838381518110612432576124326138c6565b60200101906001600160f81b031916908160001a9053508160010191505b61245981613989565b90506123f8565b5090949350505050565b60015b815181101561259757600082828151811061248a5761248a6138c6565b6020026020010151905060006001836124a391906138af565b90505b816001600160a01b03168482815181106124c2576124c26138c6565b60200260200101516001600160a01b03161115612545578381815181106124eb576124eb6138c6565b6020026020010151848260016125019190613760565b81518110612511576125116138c6565b6001600160a01b03909216602092830291909101909101528061253357612545565b8061253d81613f84565b9150506124a6565b8184612552836001613760565b81518110612562576125626138c6565b60200260200101906001600160a01b031690816001600160a01b0316815250505050808061258f90613989565b91505061246d565b5050565b60008051602061428b8339815191526028846040516020016125be929190613ef4565b60408051601f198184030181526020601f860181900481028401810190925284835291612605918690869081908401838280828437600092019190915250612bcc92505050565b604051612613929190613f9b565b60405180910390a1505050565b60408051606080820183528082526000602083018190528284018190528351918201818152608083019094529192819081526020016034600081548092919061266890613989565b909155508152600019602091820152601f54602154918301516040808501519051631420c19160e31b81523060048201526001600160a01b0394851660248201526044810192909252606482015292935060009291169063a1060c8890608401602060405180830381865afa1580156126e5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906127099190613778565b6000908152603360205260409020805460ff1916600117905550919050565b6000805160206141f883398151915260288560405160200161274b929190613ef4565b60408051601f19818403018152908290526127659161366d565b60405180910390a160008051602061428b83398151915261278582612bcc565b6040516127929190613fc0565b60405180910390a160008051602061428b8339815191526127b284612bcc565b6040516127bf9190613ffb565b60405180910390a16040805180820190915260018152605b60f81b602082015260005b835181101561295c57600184516127f991906138af565b8114156128a75781848281518110612813576128136138c6565b60200260200101516001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa158015612858573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526128809190810190614033565b6040516020016128919291906140a0565b604051602081830303815290604052915061294a565b818482815181106128ba576128ba6138c6565b60200260200101516001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa1580156128ff573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526129279190810190614033565b6040516020016129389291906140cf565b60405160208183030381529060405291505b8061295481613989565b9150506127e2565b508060405160200161296e919061410b565b604051602081830303815290604052905060008051602061428b8339815191528160405161299c9190614130565b60405180910390a15050505050565b600061010082511115612a345760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a401611b73565b8151612a4257506000919050565b60008083600081518110612a5857612a586138c6565b0160200151600160f89190911c81901b92505b8451811015612b2f57848181518110612a8657612a866138c6565b0160200151600160f89190911c1b9150828211612b1b5760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a401611b73565b91811791612b2881613989565b9050612a6b565b50909392505050565b8183146112315760008051602061428b83398151915281604051612b5c9190614168565b60405180910390a16112318383612cc6565b816125975760008051602061428b83398151915281604051612b909190614168565b60405180910390a161259782612ddb565b6000805b821561032057612bb66001846138af565b9092169180612bc481614197565b915050612ba5565b6040805180820190915260018152605b60f81b602082015260609060005b8351811015612c9d5760018451612c0191906138af565b811415612c525781612c2b858381518110612c1e57612c1e6138c6565b016020015160f81c612e40565b604051602001612c3c9291906140a0565b6040516020818303038152906040529150612c8b565b81612c68858381518110612c1e57612c1e6138c6565b604051602001612c799291906140cf565b60405160208183030381529060405291505b80612c9581613989565b915050612bea565b5080604051602001612caf919061410b565b60408051601f198184030181529190529392505050565b808214612597576000805160206141f8833981519152604051612d259060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808080808081319599d60b21b60608201526020810184905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160408051818152600a81830152690808080808149a59da1d60b21b60608201526020810183905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a1612597612f45565b80612e3d576000805160206141f8833981519152604051612e2d9060208082526017908201527f4572726f723a20417373657274696f6e204661696c6564000000000000000000604082015260600190565b60405180910390a1612e3d612f45565b50565b606081612e645750506040805180820190915260018152600360fc1b602082015290565b8160005b8115612e8e5780612e7881613989565b9150612e879050600a836141cf565b9150612e68565b6000816001600160401b03811115612ea857612ea861306f565b6040519080825280601f01601f191660200182016040528015612ed2576020820181803683370190505b5090505b8415612f3d57612ee76001836138af565b9150612ef4600a866141e3565b612eff906030613760565b60f81b818381518110612f1457612f146138c6565b60200101906001600160f81b031916908160001a905350612f36600a866141cf565b9450612ed6565b949350505050565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156130405760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f1981840301815290829052612fdf9291602001613e82565b60408051601f1981840301815290829052612ff991613eb3565b6000604051808303816000865af19150503d8060008114613036576040519150601f19603f3d011682016040523d82523d6000602084013e61303b565b606091505b505050505b6007805461ff001916610100179055565b60405180606001604052806003906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b03811182821017156130ad576130ad61306f565b604052919050565b60006001600160401b038211156130ce576130ce61306f565b50601f01601f191660200190565b600080604083850312156130ef57600080fd5b8235915060208301356001600160401b0381111561310c57600080fd5b8301601f8101851361311d57600080fd5b803561313061312b826130b5565b613085565b81815286602083850101111561314557600080fd5b816020840160208301376000602083830101528093505050509250929050565b6020808252825182820181905260009190848201906040850190845b818110156131a65783516001600160a01b031683529284019291840191600101613181565b50909695505050505050565b60005b838110156131cd5781810151838201526020016131b5565b838111156131dc576000848401525b50505050565b600081518084526131fa8160208601602086016131b2565b601f01601f19169290920160200192915050565b602080825282518282018190526000919060409081850190600581811b8701840188860187805b858110156132be57603f198b8503018752825180516001600160a01b031685528901518985018990528051898601819052908a0190606081881b870181019190870190855b818110156132a857605f198985030183526132968486516131e2565b948e01949350918d019160010161327a565b505050978a019794505091880191600101613235565b50919a9950505050505050505050565b600081518084526020808501945080840160005b838110156133075781516001600160a01b0316875295820195908201906001016132e2565b509495945050505050565b600081518084526020808501945080840160005b8381101561330757815187529582019590820190600101613326565b60408152600061335560408301856132ce565b82810360208401526133678185613312565b95945050505050565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b8481101561341457898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b808310156133ff5783516001600160e01b0319168252928b019260019290920191908b01906133d5565b50978a01979550505091870191600101613398565b50919998505050505050505050565b60006001600160401b0382111561343c5761343c61306f565b5060051b60200190565b6001600160a01b0381168114612e3d57600080fd5b600082601f83011261346c57600080fd5b8135602061347c61312b83613423565b82815260059290921b8401810191818101908684111561349b57600080fd5b8286015b848110156134b6578035835291830191830161349f565b509695505050505050565b600080604083850312156134d457600080fd5b82356001600160401b03808211156134eb57600080fd5b818501915085601f8301126134ff57600080fd5b8135602061350f61312b83613423565b82815260059290921b8401810191818101908984111561352e57600080fd5b948201945b8386101561355557853561354681613446565b82529482019490820190613533565b9650508601359250508082111561356b57600080fd5b506135788582860161345b565b9150509250929050565b60008083601f84011261359457600080fd5b5081356001600160401b038111156135ab57600080fd5b6020830191508360208285010111156135c357600080fd5b9250929050565b600080602083850312156135dd57600080fd5b82356001600160401b038111156135f357600080fd5b6135ff85828601613582565b90969095509350505050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b8281101561366057603f1988860301845261364e8583516131e2565b94509285019290850190600101613632565b5092979650505050505050565b6020815260006113ee60208301846131e2565b6000806000806000806060878903121561369957600080fd5b86356001600160401b03808211156136b057600080fd5b6136bc8a838b01613582565b909850965060208901359150808211156136d557600080fd5b818901915089601f8301126136e957600080fd5b8135818111156136f857600080fd5b8a60208260051b850101111561370d57600080fd5b60208301965080955050604089013591508082111561372b57600080fd5b5061373889828a01613582565b979a9699509497509295939492505050565b634e487b7160e01b600052601160045260246000fd5b600082198211156137735761377361374a565b500190565b60006020828403121561378a57600080fd5b5051919050565b600181811c908216806137a557607f821691505b602082108114156137c657634e487b7160e01b600052602260045260246000fd5b50919050565b600081546137d981613791565b8085526020600183811680156137f6576001811461380a57613838565b60ff19851688840152604088019550613838565b866000528260002060005b858110156138305781548a8201860152908301908401613815565b890184019650505b505050505092915050565b600060018060a01b038085511683528060208601511660208401525063ffffffff604085015116604083015260806060830152612f3d60808301846137cc565b805160ff81168114611fe457600080fd5b6000602082840312156138a657600080fd5b6113ee82613883565b6000828210156138c1576138c161374a565b500390565b634e487b7160e01b600052603260045260246000fd5b600060208083850312156138ef57600080fd5b82516001600160401b0381111561390557600080fd5b8301601f8101851361391657600080fd5b805161392461312b82613423565b81815260059190911b8201830190838101908783111561394357600080fd5b928401925b8284101561396157835182529284019290840190613948565b979650505050505050565b60006020828403121561397e57600080fd5b81516113ee81613446565b600060001982141561399d5761399d61374a565b5060010190565b6000604082016040835280855180835260608501915060608160051b860101925060208088016000805b84811015613a2b57888703605f19018652825180518089529085019085890190845b81811015613a155783516001600160a01b0316835292870192918701916001016139f0565b50909850505094830194918301916001016139ce565b5050508584038187015250505061336781856131e2565b600082601f830112613a5357600080fd5b81516020613a6361312b83613423565b82815260059290921b84018101918181019086841115613a8257600080fd5b8286015b848110156134b65780518352918301918301613a86565b60008060408385031215613ab057600080fd5b82516001600160401b0380821115613ac757600080fd5b818501915085601f830112613adb57600080fd5b81516020613aeb61312b83613423565b82815260059290921b84018101918181019089841115613b0a57600080fd5b948201945b83861015613b31578551613b2281613446565b82529482019490820190613b0f565b91880151919650909350505080821115613b4a57600080fd5b5061357885828601613a42565b60006020808301818452808551808352604092508286019150828160051b87010184880160005b83811015613be357603f19898403018552815160608151818652613ba4828701826132ce565b915050888201518582038a870152613bbc8282613312565b928901516001600160a01b0316958901959095525094870194925090860190600101613b7e565b509098975050505050505050565b600060208284031215613c0357600080fd5b815180151581146113ee57600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b8060005b60028110156131dc578154845260209093019260019182019101613c40565b80548252600181015460208301526002810154604083015260038101546060830152613c916080830160048301613c3c565b61259760c0830160068301613c3c565b6000815160608452613cb660608501826131e2565b905060208301516020850152604083015160408501528091505092915050565b6000610160808352613ceb818401888a613c13565b90508281036020840152613cff81876137cc565b9050613d0e6040840186613c5f565b828103610140840152613d218185613ca1565b98975050505050505050565b600060208284031215613d3f57600080fd5b81356113ee81613446565b600081518084526020808501945080840160005b83811015613307578151805160ff1688528301516001600160a01b03168388015260409096019590820190600101613d5e565b60018060a01b038616815284602082015260a060408201526000613db860a0830186613d4a565b606083019490945250608001529392505050565b600080600060608486031215613de157600080fd5b613dea84613883565b925060208401519150604084015190509250925092565b60006101a0808352613e158184018a6131e2565b90508281036020840152613e2981896137cc565b9050613e386040840188613c5f565b828103610140840152613e4b8187613d4a565b9050828103610160840152613e608186613ca1565b9050828103610180840152613e758185613ca1565b9998505050505050505050565b6001600160e01b0319831681528151600090613ea58160048501602087016131b2565b919091016004019392505050565b60008251613ec58184602087016131b2565b9190910192915050565b6001600160a01b03841681526040602082018190526000906133679083018486613c13565b6000808454613f0281613791565b60018281168015613f1a5760018114613f2b57613f5a565b60ff19841687528287019450613f5a565b8860005260208060002060005b85811015613f515781548a820152908401908201613f38565b50505082870194505b50601760f91b845286519250613f768382860160208a016131b2565b919092010195945050505050565b600081613f9357613f9361374a565b506000190190565b604081526000613fae60408301856131e2565b828103602084015261336781856131e2565b6040815260116040820152702d207374616e6461726451756f72756d7360781b60608201526080602082015260006113ee60808301846131e2565b60408152600e60408201526d2d20636875726e51756f72756d7360901b60608201526080602082015260006113ee60808301846131e2565b60006020828403121561404557600080fd5b81516001600160401b0381111561405b57600080fd5b8201601f8101841361406c57600080fd5b805161407a61312b826130b5565b81815285602083850101111561408f57600080fd5b6133678260208301602086016131b2565b600083516140b28184602088016131b2565b8351908301906140c68183602088016131b2565b01949350505050565b600083516140e18184602088016131b2565b8351908301906140f58183602088016131b2565b61016160f51b9101908152600201949350505050565b6000825161411d8184602087016131b2565b605d60f81b920191825250600101919050565b60408152600e60408201526d2d20636875726e5461726765747360901b60608201526080602082015260006113ee60808301846131e2565b60408152600560408201526422b93937b960d91b60608201526080602082015260006113ee60808301846131e2565b600061ffff808316818114156141af576141af61374a565b6001019392505050565b634e487b7160e01b600052601260045260246000fd5b6000826141de576141de6141b9565b500490565b6000826141f2576141f26141b9565b50069056fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50557365722e72656769737465724f70657261746f7257697468436875726e3a20696e707574206c656e677468206d69736d61746368557365722e72656769737465724f70657261746f7257697468436875726e3a20696e7075742071756f72756d73206861766520636f6d6d6f6e2062697473280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf35837570646174655374616b657320287570646174654f70657261746f7273466f7251756f72756d29a26469706673582212205b671206c61d23c42c96339fe5ec70fbc6dd8924b67cbec0bb85c21812db98c364736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\xB8W\x80c\xB5P\x8A\xA9\x11a\0|W\x80c\xB5P\x8A\xA9\x14a\x02\xA6W\x80c\xBAAO\xA6\x14a\x02\xAEW\x80c\xBFh\xB8\x16\x14a\x02\xC6W\x80c\xCAO-\x97\x14a\x02\xCFW\x80c\xE2\x0C\x9Fq\x14a\x02\xE2W\x80c\xFAv&\xD4\x14a\x02\xEAW`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x02#W\x80c\x91j\x17\xC6\x14a\x028W\x80c\xA3\xF4\xDF~\x14a\x02@W\x80c\xA5\xF6\xCC\x1A\x14a\x02UW\x80c\xAF\xA1\xC77\x14a\x02hW`\0\x80\xFD[\x80c?r\x86\xF4\x11a\x01\nW\x80c?r\x86\xF4\x14a\x01\xB4W\x80cPSw\xE2\x14a\x01\xBCW\x80ce\xED\xA8\xE5\x14a\x01\xC4W\x80cf\xD9\xA9\xA0\x14a\x01\xDAW\x80cm3oX\x14a\x01\xEFW\x80c\x821\xB5L\x14a\x02\x02W`\0\x80\xFD[\x80c\x16&\xBA~\x14a\x01GW\x80c\x1E\xD7\x83\x1C\x14a\x01xW\x80c*4\xAD\xE8\x14a\x01\x8DW\x80c*\xDE8\x80\x14a\x01\x97W\x80c>^<#\x14a\x01\xACW[`\0\x80\xFD[a\x01Za\x01U6`\x04a0\xDCV[a\x02\xF7V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x80a\x03&V[`@Qa\x01o\x91\x90a1eV[a\x01\x95a\x03\x88V[\0[a\x01\x9Fa\x05-V[`@Qa\x01o\x91\x90a2\x0EV[a\x01\x80a\x06oV[a\x01\x80a\x06\xCFV[a\x01\x95a\x07/V[a\x01\xCCa\x0B\xC2V[`@Qa\x01o\x92\x91\x90a3BV[a\x01\xE2a\x0EkV[`@Qa\x01o\x91\x90a3pV[a\x01\x95a\x01\xFD6`\x04a4\xC1V[a\x0FQV[a\x02\x15a\x02\x106`\x04a5\xCAV[a\x126V[`@Q\x90\x81R` \x01a\x01oV[a\x02+a\x13\xF5V[`@Qa\x01o\x91\x90a6\x0BV[a\x01\xE2a\x14\xC5V[a\x02Ha\x15\xABV[`@Qa\x01o\x91\x90a6mV[a\x01\x95a\x02c6`\x04a6\x80V[a\x169V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`-T\x80\x82R`.T\x91\x83\x01\x91\x82R\x83Q\x90\x81R\x90Q\x91\x81\x01\x91\x90\x91R\x01a\x01oV[a\x02+a\x1D\xECV[a\x02\xB6a\x1E\xBCV[`@Q\x90\x15\x15\x81R` \x01a\x01oV[a\x02\x15`)T\x81V[a\x01\x95a\x02\xDD6`\x04a5\xCAV[a\x1F\xE9V[a\x01\x80a\"HV[`\x07Ta\x02\xB6\x90`\xFF\x16\x81V[`\0\x82\x81R`3` R`@\x81 T`\xFF\x16\x15a\x03\x1CWPc\x0B\x13]?`\xE1\x1Ba\x03 V[P`\0[\x92\x91PPV[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`W[PPPPP\x90P\x90V[`\x1CT`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x03\xA4C`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xC2\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xDCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xF0W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04m\x91\x90a7xV[Pa\x04\xAC`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FregisterAsOperator (core)\0\0\0\0\0\0\0\x81RPa#VV[`@\x80Q``\x81\x01\x82R0\x81R`\0` \x82\x01\x81\x90R\x81\x83\x01R`\x1DT\x91Qc\x0FX\x9EY`\xE0\x1B\x81R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x0FX\x9EY\x90a\x04\xF8\x90\x84\x90`(\x90`\x04\x01a8CV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05&W=`\0\x80>=`\0\xFD[PPPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06fW`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x06OW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x05\xC2\x90a7\x91V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xEE\x90a7\x91V[\x80\x15a\x06;W\x80`\x1F\x10a\x06\x10Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06;V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x1EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x05\xA3V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x05QV[PPPP\x90P\x90V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[`\x1CT`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x07KC`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07i\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x83W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x97W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x14\x91\x90a7xV[Pa\x086`@Q\x80``\x01`@R\x80`'\x81R` \x01aB\xAB`'\x919a#VV[`\0a\x08\xCA`\x01` `\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xB4\x91\x90a8\x94V[`\xFF\x16`\x01\x90\x1Ba\x08\xC5\x91\x90a8\xAFV[a#\x9EV[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xE7Wa\x08\xE7a0oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x1AW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\x05W\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x0BYW`\0\x83\x82\x81Q\x81\x10a\t=Wa\t=a8\xC6V[\x01` \x01Q`$\x80T`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x93\x90\x93\x1C`\x04\x84\x01\x81\x90RCc\xFF\xFF\xFF\xFF\x16\x92\x84\x01\x92\x90\x92R\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xCE\x91\x90\x81\x01\x90a8\xDCV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xE9Wa\t\xE9a0oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\x12W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x84\x84\x81Q\x81\x10a\n%Wa\n%a8\xC6V[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x0B!W`\"T\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG\xB3\x14\xE8\x90\x84\x90\x84\x90\x81\x10a\neWa\nea8\xC6V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x8B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xCC\x91\x90a9lV[\x85\x85\x81Q\x81\x10a\n\xDEWa\n\xDEa8\xC6V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\n\xF7Wa\n\xF7a8\xC6V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0B\x19\x81a9\x89V[\x91PPa\n3V[Pa\x0BD\x84\x84\x81Q\x81\x10a\x0B7Wa\x0B7a8\xC6V[` \x02` \x01\x01Qa$jV[PP\x80\x80a\x0BQ\x90a9\x89V[\x91PPa\t V[P` T`@Qc\n(\x14\xA9`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cQ@\xA5H\x90a\x0B\x8C\x90\x84\x90\x86\x90`\x04\x01a9\xA4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xBAW=`\0\x80>=`\0\xFD[PPPPPPV[`\x1CT``\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x0B\xE3C`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x01\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\x1BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C/W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAC\x91\x90a7xV[Pa\x0C\xE3`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01texitEigenlayer (core)`X\x1B\x81RPa#VV[`\x1DT`@Qcg\xC0C\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCF\x80\x87>\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\rX\x91\x90\x81\x01\x90a:\x9DV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90\x92P`\0\x91\x90\x81` \x01[`@\x80Q``\x80\x82\x01\x83R\x80\x82R` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\rvW\x90PP\x90P`@Q\x80``\x01`@R\x80\x84\x81R` \x01\x83\x81R` \x010`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x81`\0\x81Q\x81\x10a\r\xDEWa\r\xDEa8\xC6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x1DT`@Qc\x06\xECn\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\r\xD8\xDD\x02\x90a\x0E\x19\x90\x84\x90`\x04\x01a;WV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E`\x91\x90\x81\x01\x90a8\xDCV[P\x91\x93P\x91PP\x90\x91V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06fW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0F9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0E\xFBW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0E\x8FV[`\x1CT`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x0FmC`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x8B\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xA5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xB9W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x10\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x106\x91\x90a7xV[Pa\x10u`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7FdepositIntoEigenLayer (core)\0\0\0\0\x81RPa#VV[`\0[\x82Q\x81\x10\x15a\x121W`\0\x83\x82\x81Q\x81\x10a\x10\x95Wa\x10\x95a8\xC6V[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x10\xB3Wa\x10\xB3a8\xC6V[` \x02` \x01\x01Q\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11!\x91\x90a9lV[`\x1ET`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11vW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x9A\x91\x90a;\xF1V[P`\x1ET`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x1A\x91\x90a7xV[PPPP\x80\x80a\x12)\x90a9\x89V[\x91PPa\x10xV[PPPV[`\x1CT`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x12UC`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12s\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\x8DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xA1W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x1E\x91\x90a7xV[Pa\x13R`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o92\xB3\xB4\xB9\xBA2\xB9'\xB82\xB90\xBA7\xB9`\x81\x1B\x81RP\x84\x84a%\x9BV[` T`\x01`\x01`\xA0\x1B\x03\x16c\xA5\x08W\xBF\x84\x84`(`+a\x13qa& V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x91\x95\x94\x93\x92\x91\x90a<\xD6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xABW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xBFW=`\0\x80>=`\0\xFD[PP`@\x80Q\x80\x82\x01\x82R`-T\x80\x82R`.T` \x92\x83\x01\x90\x81R`\0\x91\x82RQ\x90\x91R \x91Pa\x13\xEE\x90PV[\x93\x92PPPV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06fW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x148\x90a7\x91V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14d\x90a7\x91V[\x80\x15a\x14\xB1W\x80`\x1F\x10a\x14\x86Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\xB1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\x94W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x14\x19V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06fW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x15\x93W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x15UW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x14\xE9V[`(\x80Ta\x15\xB8\x90a7\x91V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\xE4\x90a7\x91V[\x80\x15a\x161W\x80`\x1F\x10a\x16\x06Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x161V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\x14W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x1CT`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a\x16UC`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16s\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x8DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xA1W=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x16\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x1E\x91\x90a7xV[Pa\x17\xF9`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FregisterOperatorWithChurn\0\0\0\0\0\0\0\x81RP\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` \x80\x8B\x02\x82\x81\x01\x82\x01\x90\x93R\x8A\x82R\x90\x93P\x8A\x92P\x89\x91\x82\x91\x85\x01\x90\x84\x90\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa'(\x92PPPV[`\0a\x18:\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\xAB\x92PPPV[\x90P`\0a\x18}\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\xAB\x92PPPV[`@\x80Q``\x81\x01\x90\x91R`5\x80\x82R\x91\x92Pa\x18\xA5\x91\x89\x91\x88\x91aB\x18` \x83\x019a+8V[a\x18\xD3\x81\x83\x16`\x01`\x01`\xC0\x1B\x03\x16\x15`@Q\x80``\x01`@R\x80`>\x81R` \x01aBM`>\x919a+nV[`\0a\x18\xEC`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17a#\x9EV[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\tWa\x19\ta0oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19NW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19'W\x90P[P\x90P`\0\x80[\x83Qa\x19a\x82\x84a7`V[\x10\x15a\x1B|W\x81\x8B\x14\x15a\x19\xBCW`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x83a\x19\x8E\x83\x85a7`V[\x81Q\x81\x10a\x19\x9EWa\x19\x9Ea8\xC6V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x19\xB4\x90a9\x89V[\x91PPa\x19UV[\x80\x87\x14\x80a\x1A\x0FWP\x87\x87\x82\x81\x81\x10a\x19\xD7Wa\x19\xD7a8\xC6V[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P\x8C\x8C\x84\x81\x81\x10a\x19\xF9Wa\x19\xF9a8\xC6V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x10[\x15a\x1A\xAAW`@Q\x80`@\x01`@R\x80\x8D\x8D\x85\x81\x81\x10a\x1A1Wa\x1A1a8\xC6V[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x01\x8B\x8B\x85\x81\x81\x10a\x1AQWa\x1AQa8\xC6V[\x90P` \x02\x01` \x81\x01\x90a\x1Af\x91\x90a=-V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x83a\x1A|\x83\x85a7`V[\x81Q\x81\x10a\x1A\x8CWa\x1A\x8Ca8\xC6V[` \x02` \x01\x01\x81\x90RP\x81\x80a\x1A\xA2\x90a9\x89V[\x92PPa\x19UV[\x8B\x8B\x83\x81\x81\x10a\x1A\xBCWa\x1A\xBCa8\xC6V[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P\x88\x88\x83\x81\x81\x10a\x1A\xDEWa\x1A\xDEa8\xC6V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x10\x15a\x1B\x17W`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x83a\x19\x8E\x83\x85a7`V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FUser.registerOperatorWithChurn: `D\x82\x01Rn\x1BX[\x19\x9B\xDC\x9BYY\x08\x1A[\x9C\x1D]`\x8A\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0`4`\0\x81Ta\x1B\x8D\x90a9\x89V[\x91\x82\x90UP`@\x80Q` \x81\x01\x92\x90\x92Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16\x90\x82\x01R`T\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x90T`)Tc\x84\xCAR\x13`\xE0\x1B\x84R\x91\x93P`\0\x19\x92`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x84\xCAR\x13\x91a\x1C\x10\x910\x91\x8B\x90\x89\x90\x89\x90`\x04\x01a=\x91V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CQ\x91\x90a7xV[`\x1CT`&T`@Qc8\xD0z\xA9`\xE2\x1B\x81R\x92\x93P`\0\x92\x83\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xE3A\xEA\xA4\x91a\x1C\x97\x91\x88\x90`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xD8\x91\x90a=\xCCV[`@\x80Q`A\x80\x82R`\x80\x82\x01\x90\x92R\x93\x96P\x91\x94P\x92P`\0\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x82` \x82\x01R\x81`@\x82\x01R\x83`\xF8\x1B\x81`\x01\x83Qa\x1D\"\x91\x90a8\xAFV[\x81Q\x81\x10a\x1D2Wa\x1D2a8\xC6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`@\x80Q``\x81\x01\x82R\x82\x81R` \x80\x82\x01\x8A\x90R\x91\x81\x01\x88\x90R\x90T`\x01`\x01`\xA0\x1B\x03\x16c\x9B]\x17{\x8D`(`+\x8F\x86a\x1D\x83a& V[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\xA4\x96\x95\x94\x93\x92\x91\x90a>\x01V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\xBEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D\xD2W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPPPPPPPPPPV[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06fW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1E/\x90a7\x91V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E[\x90a7\x91V[\x80\x15a\x1E\xA8W\x80`\x1F\x10a\x1E}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1E\x10V[`\x07T`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a\x1E\xDEWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1F\xE4W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x1Fl\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a>\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1F\x86\x91a>\xB3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1F\xC3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1F\xC8V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x1F\xE0\x91\x90a;\xF1V[\x91PP[\x91\x90PV[`\x1CT`\x01`\x01`\xA0\x1B\x03\x16c\x1F{O0a \x05C`\x01a7`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a #\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a =W`\0\x80\xFD[PZ\xF1\x15\x80\x15a QW=`\0\x80>=`\0\xFD[PPPP`%`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a \xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xCE\x91\x90a7xV[Pa!\x0F`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FderegisterOperator (eject)\0\0\0\0\0\0\x81RP\x83\x83a%\x9BV[` \x80T`@\x80Qc(\xF6\x1B1`\xE0\x1B\x81R\x90Q`\0\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c(\xF6\x1B1\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!|\x91\x90a9lV[`\x1CT`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xC5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\xD9W=`\0\x80>=`\0\xFD[PP` T`@Qcn;\x17\xDB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcn;\x17\xDB\x91Pa\"\x11\x900\x90\x87\x90\x87\x90`\x04\x01a>\xCFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\"+W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"?W=`\0\x80>=`\0\xFD[PPPPPPPV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03`WPPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\"\xC4a0QV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\"\xF7Wa\"\xF9V[\xFE[P\x80a#7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x1BsV[PP\x92\x91PPV[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[`\0\x80Q` aA\xF8\x839\x81Q\x91R`(\x82`@Q` \x01a#y\x92\x91\x90a>\xF4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra#\x93\x91a6mV[`@Q\x80\x91\x03\x90\xA1PV[```\0\x80a#\xAC\x84a+\xA1V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xC7Wa#\xC7a0oV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#\xF1W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a$\tWPa\x01\0\x81\x10[\x15a$`W`\x01\x81\x1B\x93P\x85\x84\x16\x15a$PW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a$2Wa$2a8\xC6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a$Y\x81a9\x89V[\x90Pa#\xF8V[P\x90\x94\x93PPPPV[`\x01[\x81Q\x81\x10\x15a%\x97W`\0\x82\x82\x81Q\x81\x10a$\x8AWa$\x8Aa8\xC6V[` \x02` \x01\x01Q\x90P`\0`\x01\x83a$\xA3\x91\x90a8\xAFV[\x90P[\x81`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a$\xC2Wa$\xC2a8\xC6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a%EW\x83\x81\x81Q\x81\x10a$\xEBWa$\xEBa8\xC6V[` \x02` \x01\x01Q\x84\x82`\x01a%\x01\x91\x90a7`V[\x81Q\x81\x10a%\x11Wa%\x11a8\xC6V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a%3Wa%EV[\x80a%=\x81a?\x84V[\x91PPa$\xA6V[\x81\x84a%R\x83`\x01a7`V[\x81Q\x81\x10a%bWa%ba8\xC6V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPPP\x80\x80a%\x8F\x90a9\x89V[\x91PPa$mV[PPV[`\0\x80Q` aB\x8B\x839\x81Q\x91R`(\x84`@Q` \x01a%\xBE\x92\x91\x90a>\xF4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` `\x1F\x86\x01\x81\x90\x04\x81\x02\x84\x01\x81\x01\x90\x92R\x84\x83R\x91a&\x05\x91\x86\x90\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa+\xCC\x92PPPV[`@Qa&\x13\x92\x91\x90a?\x9BV[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x80\x82\x01\x83R\x80\x82R`\0` \x83\x01\x81\x90R\x82\x84\x01\x81\x90R\x83Q\x91\x82\x01\x81\x81R`\x80\x83\x01\x90\x94R\x91\x92\x81\x90\x81R` \x01`4`\0\x81T\x80\x92\x91\x90a&h\x90a9\x89V[\x90\x91UP\x81R`\0\x19` \x91\x82\x01R`\x1FT`!T\x91\x83\x01Q`@\x80\x85\x01Q\x90Qc\x14 \xC1\x91`\xE3\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`$\x82\x01R`D\x81\x01\x92\x90\x92R`d\x82\x01R\x92\x93P`\0\x92\x91\x16\x90c\xA1\x06\x0C\x88\x90`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\t\x91\x90a7xV[`\0\x90\x81R`3` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UP\x91\x90PV[`\0\x80Q` aA\xF8\x839\x81Q\x91R`(\x85`@Q` \x01a'K\x92\x91\x90a>\xF4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra'e\x91a6mV[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` aB\x8B\x839\x81Q\x91Ra'\x85\x82a+\xCCV[`@Qa'\x92\x91\x90a?\xC0V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` aB\x8B\x839\x81Q\x91Ra'\xB2\x84a+\xCCV[`@Qa'\xBF\x91\x90a?\xFBV[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`[`\xF8\x1B` \x82\x01R`\0[\x83Q\x81\x10\x15a)\\W`\x01\x84Qa'\xF9\x91\x90a8\xAFV[\x81\x14\x15a(\xA7W\x81\x84\x82\x81Q\x81\x10a(\x13Wa(\x13a8\xC6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(\x80\x91\x90\x81\x01\x90a@3V[`@Q` \x01a(\x91\x92\x91\x90a@\xA0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa)JV[\x81\x84\x82\x81Q\x81\x10a(\xBAWa(\xBAa8\xC6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra)'\x91\x90\x81\x01\x90a@3V[`@Q` \x01a)8\x92\x91\x90a@\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P[\x80a)T\x81a9\x89V[\x91PPa'\xE2V[P\x80`@Q` \x01a)n\x91\x90aA\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80Q` aB\x8B\x839\x81Q\x91R\x81`@Qa)\x9C\x91\x90aA0V[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\0a\x01\0\x82Q\x11\x15a*4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x1BsV[\x81Qa*BWP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a*XWa*Xa8\xC6V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a+/W\x84\x81\x81Q\x81\x10a*\x86Wa*\x86a8\xC6V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a+\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x1BsV[\x91\x81\x17\x91a+(\x81a9\x89V[\x90Pa*kV[P\x90\x93\x92PPPV[\x81\x83\x14a\x121W`\0\x80Q` aB\x8B\x839\x81Q\x91R\x81`@Qa+\\\x91\x90aAhV[`@Q\x80\x91\x03\x90\xA1a\x121\x83\x83a,\xC6V[\x81a%\x97W`\0\x80Q` aB\x8B\x839\x81Q\x91R\x81`@Qa+\x90\x91\x90aAhV[`@Q\x80\x91\x03\x90\xA1a%\x97\x82a-\xDBV[`\0\x80[\x82\x15a\x03 Wa+\xB6`\x01\x84a8\xAFV[\x90\x92\x16\x91\x80a+\xC4\x81aA\x97V[\x91PPa+\xA5V[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`[`\xF8\x1B` \x82\x01R``\x90`\0[\x83Q\x81\x10\x15a,\x9DW`\x01\x84Qa,\x01\x91\x90a8\xAFV[\x81\x14\x15a,RW\x81a,+\x85\x83\x81Q\x81\x10a,\x1EWa,\x1Ea8\xC6V[\x01` \x01Q`\xF8\x1Ca.@V[`@Q` \x01a,<\x92\x91\x90a@\xA0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa,\x8BV[\x81a,h\x85\x83\x81Q\x81\x10a,\x1EWa,\x1Ea8\xC6V[`@Q` \x01a,y\x92\x91\x90a@\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P[\x80a,\x95\x81a9\x89V[\x91PPa+\xEAV[P\x80`@Q` \x01a,\xAF\x91\x90aA\x0BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x93\x92PPPV[\x80\x82\x14a%\x97W`\0\x80Q` aA\xF8\x839\x81Q\x91R`@Qa-%\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x08\x13\x19Y\x9D`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x14\x9AY\xDA\x1D`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a%\x97a/EV[\x80a.=W`\0\x80Q` aA\xF8\x839\x81Q\x91R`@Qa.-\x90` \x80\x82R`\x17\x90\x82\x01R\x7FError: Assertion Failed\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1a.=a/EV[PV[``\x81a.dWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[\x81\x15a.\x8EW\x80a.x\x81a9\x89V[\x91Pa.\x87\x90P`\n\x83aA\xCFV[\x91Pa.hV[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xA8Wa.\xA8a0oV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a.\xD2W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a/=Wa.\xE7`\x01\x83a8\xAFV[\x91Pa.\xF4`\n\x86aA\xE3V[a.\xFF\x90`0a7`V[`\xF8\x1B\x81\x83\x81Q\x81\x10a/\x14Wa/\x14a8\xC6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SPa/6`\n\x86aA\xCFV[\x94Pa.\xD6V[\x94\x93PPPPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a0@W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra/\xDF\x92\x91` \x01a>\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra/\xF9\x91a>\xB3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a06W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a0;V[``\x91P[PPPP[`\x07\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a0\xADWa0\xADa0oV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a0\xCEWa0\xCEa0oV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a0\xEFW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\x0CW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a1\x1DW`\0\x80\xFD[\x805a10a1+\x82a0\xB5V[a0\x85V[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a1EW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a1\xA6W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a1\x81V[P\x90\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15a1\xCDW\x81\x81\x01Q\x83\x82\x01R` \x01a1\xB5V[\x83\x81\x11\x15a1\xDCW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra1\xFA\x81` \x86\x01` \x86\x01a1\xB2V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a2\xBEW`?\x19\x8B\x85\x03\x01\x87R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a2\xA8W`_\x19\x89\x85\x03\x01\x83Ra2\x96\x84\x86Qa1\xE2V[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a2zV[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a25V[P\x91\x9A\x99PPPPPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a3\x07W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a2\xE2V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a3\x07W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a3&V[`@\x81R`\0a3U`@\x83\x01\x85a2\xCEV[\x82\x81\x03` \x84\x01Ra3g\x81\x85a3\x12V[\x95\x94PPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a4\x14W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a3\xFFW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a3\xD5V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a3\x98V[P\x91\x99\x98PPPPPPPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a4<Wa4<a0oV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a.=W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a4lW`\0\x80\xFD[\x815` a4|a1+\x83a4#V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a4\x9BW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a4\xB6W\x805\x83R\x91\x83\x01\x91\x83\x01a4\x9FV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a4\xD4W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a4\xEBW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a4\xFFW`\0\x80\xFD[\x815` a5\x0Fa1+\x83a4#V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a5.W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a5UW\x855a5F\x81a4FV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a53V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a5kW`\0\x80\xFD[Pa5x\x85\x82\x86\x01a4[V[\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a5\x94W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xABW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a5\xC3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a5\xDDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xF3W`\0\x80\xFD[a5\xFF\x85\x82\x86\x01a5\x82V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a6`W`?\x19\x88\x86\x03\x01\x84Ra6N\x85\x83Qa1\xE2V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a62V[P\x92\x97\x96PPPPPPPV[` \x81R`\0a\x13\xEE` \x83\x01\x84a1\xE2V[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a6\x99W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a6\xB0W`\0\x80\xFD[a6\xBC\x8A\x83\x8B\x01a5\x82V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a6\xD5W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a6\xE9W`\0\x80\xFD[\x815\x81\x81\x11\x15a6\xF8W`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15a7\rW`\0\x80\xFD[` \x83\x01\x96P\x80\x95PP`@\x89\x015\x91P\x80\x82\x11\x15a7+W`\0\x80\xFD[Pa78\x89\x82\x8A\x01a5\x82V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a7sWa7sa7JV[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a7\x8AW`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a7\xA5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a7\xC6WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x81Ta7\xD9\x81a7\x91V[\x80\x85R` `\x01\x83\x81\x16\x80\x15a7\xF6W`\x01\x81\x14a8\nWa88V[`\xFF\x19\x85\x16\x88\x84\x01R`@\x88\x01\x95Pa88V[\x86`\0R\x82`\0 `\0[\x85\x81\x10\x15a80W\x81T\x8A\x82\x01\x86\x01R\x90\x83\x01\x90\x84\x01a8\x15V[\x89\x01\x84\x01\x96PP[PPPPP\x92\x91PPV[`\0`\x01\x80`\xA0\x1B\x03\x80\x85Q\x16\x83R\x80` \x86\x01Q\x16` \x84\x01RPc\xFF\xFF\xFF\xFF`@\x85\x01Q\x16`@\x83\x01R`\x80``\x83\x01Ra/=`\x80\x83\x01\x84a7\xCCV[\x80Q`\xFF\x81\x16\x81\x14a\x1F\xE4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a8\xA6W`\0\x80\xFD[a\x13\xEE\x82a8\x83V[`\0\x82\x82\x10\x15a8\xC1Wa8\xC1a7JV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a8\xEFW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a9\x05W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a9\x16W`\0\x80\xFD[\x80Qa9$a1+\x82a4#V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a9CW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a9aW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a9HV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a9~W`\0\x80\xFD[\x81Qa\x13\xEE\x81a4FV[`\0`\0\x19\x82\x14\x15a9\x9DWa9\x9Da7JV[P`\x01\x01\x90V[`\0`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x80\x88\x01`\0\x80[\x84\x81\x10\x15a:+W\x88\x87\x03`_\x19\x01\x86R\x82Q\x80Q\x80\x89R\x90\x85\x01\x90\x85\x89\x01\x90\x84[\x81\x81\x10\x15a:\x15W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x87\x01\x92\x91\x87\x01\x91`\x01\x01a9\xF0V[P\x90\x98PPP\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a9\xCEV[PPP\x85\x84\x03\x81\x87\x01RPPPa3g\x81\x85a1\xE2V[`\0\x82`\x1F\x83\x01\x12a:SW`\0\x80\xFD[\x81Q` a:ca1+\x83a4#V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a:\x82W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a4\xB6W\x80Q\x83R\x91\x83\x01\x91\x83\x01a:\x86V[`\0\x80`@\x83\x85\x03\x12\x15a:\xB0W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\xC7W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a:\xDBW`\0\x80\xFD[\x81Q` a:\xEBa1+\x83a4#V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a;\nW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a;1W\x85Qa;\"\x81a4FV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a;\x0FV[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15a;JW`\0\x80\xFD[Pa5x\x85\x82\x86\x01a:BV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a;\xE3W`?\x19\x89\x84\x03\x01\x85R\x81Q``\x81Q\x81\x86Ra;\xA4\x82\x87\x01\x82a2\xCEV[\x91PP\x88\x82\x01Q\x85\x82\x03\x8A\x87\x01Ra;\xBC\x82\x82a3\x12V[\x92\x89\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x95\x89\x01\x95\x90\x95RP\x94\x87\x01\x94\x92P\x90\x86\x01\x90`\x01\x01a;~V[P\x90\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a<\x03W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\xEEW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x80`\0[`\x02\x81\x10\x15a1\xDCW\x81T\x84R` \x90\x93\x01\x92`\x01\x91\x82\x01\x91\x01a<@V[\x80T\x82R`\x01\x81\x01T` \x83\x01R`\x02\x81\x01T`@\x83\x01R`\x03\x81\x01T``\x83\x01Ra<\x91`\x80\x83\x01`\x04\x83\x01a<<V[a%\x97`\xC0\x83\x01`\x06\x83\x01a<<V[`\0\x81Q``\x84Ra<\xB6``\x85\x01\x82a1\xE2V[\x90P` \x83\x01Q` \x85\x01R`@\x83\x01Q`@\x85\x01R\x80\x91PP\x92\x91PPV[`\0a\x01`\x80\x83Ra<\xEB\x81\x84\x01\x88\x8Aa<\x13V[\x90P\x82\x81\x03` \x84\x01Ra<\xFF\x81\x87a7\xCCV[\x90Pa=\x0E`@\x84\x01\x86a<_V[\x82\x81\x03a\x01@\x84\x01Ra=!\x81\x85a<\xA1V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a=?W`\0\x80\xFD[\x815a\x13\xEE\x81a4FV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a3\x07W\x81Q\x80Q`\xFF\x16\x88R\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a=^V[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0a=\xB8`\xA0\x83\x01\x86a=JV[``\x83\x01\x94\x90\x94RP`\x80\x01R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a=\xE1W`\0\x80\xFD[a=\xEA\x84a8\x83V[\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0a\x01\xA0\x80\x83Ra>\x15\x81\x84\x01\x8Aa1\xE2V[\x90P\x82\x81\x03` \x84\x01Ra>)\x81\x89a7\xCCV[\x90Pa>8`@\x84\x01\x88a<_V[\x82\x81\x03a\x01@\x84\x01Ra>K\x81\x87a=JV[\x90P\x82\x81\x03a\x01`\x84\x01Ra>`\x81\x86a<\xA1V[\x90P\x82\x81\x03a\x01\x80\x84\x01Ra>u\x81\x85a<\xA1V[\x99\x98PPPPPPPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a>\xA5\x81`\x04\x85\x01` \x87\x01a1\xB2V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa>\xC5\x81\x84` \x87\x01a1\xB2V[\x91\x90\x91\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a3g\x90\x83\x01\x84\x86a<\x13V[`\0\x80\x84Ta?\x02\x81a7\x91V[`\x01\x82\x81\x16\x80\x15a?\x1AW`\x01\x81\x14a?+Wa?ZV[`\xFF\x19\x84\x16\x87R\x82\x87\x01\x94Pa?ZV[\x88`\0R` \x80`\0 `\0[\x85\x81\x10\x15a?QW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a?8V[PPP\x82\x87\x01\x94P[P`\x17`\xF9\x1B\x84R\x86Q\x92Pa?v\x83\x82\x86\x01` \x8A\x01a1\xB2V[\x91\x90\x92\x01\x01\x95\x94PPPPPV[`\0\x81a?\x93Wa?\x93a7JV[P`\0\x19\x01\x90V[`@\x81R`\0a?\xAE`@\x83\x01\x85a1\xE2V[\x82\x81\x03` \x84\x01Ra3g\x81\x85a1\xE2V[`@\x81R`\x11`@\x82\x01Rp- standardQuorums`x\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x13\xEE`\x80\x83\x01\x84a1\xE2V[`@\x81R`\x0E`@\x82\x01Rm- churnQuorums`\x90\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x13\xEE`\x80\x83\x01\x84a1\xE2V[`\0` \x82\x84\x03\x12\x15a@EW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a@[W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a@lW`\0\x80\xFD[\x80Qa@za1+\x82a0\xB5V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a@\x8FW`\0\x80\xFD[a3g\x82` \x83\x01` \x86\x01a1\xB2V[`\0\x83Qa@\xB2\x81\x84` \x88\x01a1\xB2V[\x83Q\x90\x83\x01\x90a@\xC6\x81\x83` \x88\x01a1\xB2V[\x01\x94\x93PPPPV[`\0\x83Qa@\xE1\x81\x84` \x88\x01a1\xB2V[\x83Q\x90\x83\x01\x90a@\xF5\x81\x83` \x88\x01a1\xB2V[a\x01a`\xF5\x1B\x91\x01\x90\x81R`\x02\x01\x94\x93PPPPV[`\0\x82QaA\x1D\x81\x84` \x87\x01a1\xB2V[`]`\xF8\x1B\x92\x01\x91\x82RP`\x01\x01\x91\x90PV[`@\x81R`\x0E`@\x82\x01Rm- churnTargets`\x90\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x13\xEE`\x80\x83\x01\x84a1\xE2V[`@\x81R`\x05`@\x82\x01Rd\"\xB997\xB9`\xD9\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x13\xEE`\x80\x83\x01\x84a1\xE2V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aA\xAFWaA\xAFa7JV[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aA\xDEWaA\xDEaA\xB9V[P\x04\x90V[`\0\x82aA\xF2WaA\xF2aA\xB9V[P\x06\x90V\xFEA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FPUser.registerOperatorWithChurn: input length mismatchUser.registerOperatorWithChurn: input quorums have common bits(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83updateStakes (updateOperatorsForQuorum)\xA2dipfsX\"\x12 [g\x12\x06\xC6\x1D#\xC4,\x963\x9F\xE5\xECp\xFB\xC6\xDD\x89$\xB6|\xBE\xC0\xBB\x85\xC2\x18\x12\xDB\x98\xC3dsolcC\0\x08\x0C\x003",
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
    ///Container for all the [`User_AltMethods`](self) function calls.
    pub enum User_AltMethodsCalls {
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
    impl User_AltMethodsCalls {
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
    impl alloy_sol_types::SolInterface for User_AltMethodsCalls {
        const NAME: &'static str = "User_AltMethodsCalls";
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
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<User_AltMethodsCalls>] = &[
                {
                    fn isValidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <isValidSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::isValidSignature)
                    }
                    isValidSignature
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
                    fn updateStakes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <updateStakesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::updateStakes)
                    }
                    updateStakes
                },
                {
                    fn exitEigenlayer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <exitEigenlayerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::exitEigenlayer)
                    }
                    exitEigenlayer
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
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::registerOperator)
                    }
                    registerOperator
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
                    fn registerOperatorWithChurn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <registerOperatorWithChurnCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::registerOperatorWithChurn)
                    }
                    registerOperatorWithChurn
                },
                {
                    fn pubkeyG1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <pubkeyG1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(User_AltMethodsCalls::pubkeyG1)
                    }
                    pubkeyG1
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
                    fn operatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <operatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(User_AltMethodsCalls::operatorId)
                    }
                    operatorId
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::deregisterOperator)
                    }
                    deregisterOperator
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
        _privKey: alloy::sol_types::private::primitives::aliases::U256,
        _pubkeyParams: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<User_AltMethodsInstance<T, P, N>>>
    {
        User_AltMethodsInstance::<T, P, N>::deploy(provider, name, _privKey, _pubkeyParams)
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
        User_AltMethodsInstance::<T, P, N>::deploy_builder(provider, name, _privKey, _pubkeyParams)
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
            _privKey: alloy::sol_types::private::primitives::aliases::U256,
            _pubkeyParams: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::Result<User_AltMethodsInstance<T, P, N>> {
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
