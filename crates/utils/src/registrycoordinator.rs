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
library IRegistryCoordinator {
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
    clippy::style
)]
pub mod IRegistryCoordinator {
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
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
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
        pub quorumNumber: u8,
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
        pub maxOperatorCount: u32,
        pub kickBIPsOfOperatorStake: u16,
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
        pub updateBlockNumber: u32,
        pub nextUpdateBlockNumber: u32,
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
    /**Creates a new wrapper around an on-chain [`IRegistryCoordinator`](self) contract instance.

    See the [wrapper's documentation](`IRegistryCoordinatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRegistryCoordinatorInstance<T, P, N> {
        IRegistryCoordinatorInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRegistryCoordinator`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IRegistryCoordinator`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRegistryCoordinatorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRegistryCoordinatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRegistryCoordinatorInstance")
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
        > IRegistryCoordinatorInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IRegistryCoordinator`](self) contract instance.

        See the [wrapper's documentation](`IRegistryCoordinatorInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IRegistryCoordinatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRegistryCoordinatorInstance<T, P, N> {
            IRegistryCoordinatorInstance {
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
        > IRegistryCoordinatorInstance<T, P, N>
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
        > IRegistryCoordinatorInstance<T, P, N>
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
    clippy::style
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
library IStakeRegistry {
    struct StrategyParams { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
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

library IRegistryCoordinator {
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

library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

library IStakeRegistry {
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }
}

interface RegistryCoordinator {
    type StakeType is uint8;

    error AlreadyRegisteredForQuorums();
    error BitmapCannotBeZero();
    error BitmapEmpty();
    error CannotChurnSelf();
    error CannotKickOperatorAboveThreshold();
    error CannotReregisterYet();
    error ChurnApproverSaltUsed();
    error CurrentlyPaused();
    error InputAddressZero();
    error InputLengthMismatch();
    error InsufficientStakeForChurn();
    error InvalidNewPausedStatus();
    error InvalidSignature();
    error MaxQuorumsReached();
    error NotRegistered();
    error NotRegisteredForQuorum();
    error NotSorted();
    error OnlyAllocationManager();
    error OnlyEjector();
    error OnlyPauser();
    error OnlyUnpauser();
    error OperatorSetsEnabled();
    error OperatorSetsNotEnabled();
    error OperatorSetsNotSupported();
    error QuorumDoesNotExist();
    error QuorumOperatorCountMismatch();
    error RegistryCoordinatorSignatureExpired();
    error SaltAlreadyUsed();
    error SignatureExpired();

    event ChurnApproverUpdated(address prevChurnApprover, address newChurnApprover);
    event EjectorUpdated(address prevEjector, address newEjector);
    event Initialized(uint8 version);
    event OperatorDeregistered(address indexed operator, bytes32 indexed operatorId);
    event OperatorRegistered(address indexed operator, bytes32 indexed operatorId);
    event OperatorSetParamsUpdated(uint8 indexed quorumNumber, IRegistryCoordinator.OperatorSetParam operatorSetParams);
    event OperatorSocketUpdate(bytes32 indexed operatorId, string socket);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event QuorumBlockNumberUpdated(uint8 indexed quorumNumber, uint256 blocknumber);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _serviceManager, address _stakeRegistry, address _blsApkRegistry, address _indexRegistry, address _avsDirectory, address _pauserRegistry);

    function OPERATOR_CHURN_APPROVAL_TYPEHASH() external view returns (bytes32);
    function PUBKEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
    function avsDirectory() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function calculateOperatorChurnApprovalDigestHash(address registeringOperator, bytes32 registeringOperatorId, IRegistryCoordinator.OperatorKickParam[] memory operatorKickParams, bytes32 salt, uint256 expiry) external view returns (bytes32);
    function churnApprover() external view returns (address);
    function createSlashableStakeQuorum(IRegistryCoordinator.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory strategyParams, uint32 lookAheadPeriod) external;
    function createTotalDelegatedStakeQuorum(IRegistryCoordinator.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory strategyParams) external;
    function deregisterOperator(address operator, uint32[] memory operatorSetIds) external;
    function deregisterOperator(bytes memory quorumNumbers) external;
    function ejectOperator(address operator, bytes memory quorumNumbers) external;
    function ejectionCooldown() external view returns (uint256);
    function ejector() external view returns (address);
    function enableOperatorSets() external;
    function getCurrentQuorumBitmap(bytes32 operatorId) external view returns (uint192);
    function getOperator(address operator) external view returns (IRegistryCoordinator.OperatorInfo memory);
    function getOperatorFromId(bytes32 operatorId) external view returns (address);
    function getOperatorId(address operator) external view returns (bytes32);
    function getOperatorSetParams(uint8 quorumNumber) external view returns (IRegistryCoordinator.OperatorSetParam memory);
    function getOperatorStatus(address operator) external view returns (IRegistryCoordinator.OperatorStatus);
    function getQuorumBitmapAtBlockNumberByIndex(bytes32 operatorId, uint32 blockNumber, uint256 index) external view returns (uint192);
    function getQuorumBitmapHistoryLength(bytes32 operatorId) external view returns (uint256);
    function getQuorumBitmapIndicesAtBlockNumber(uint32 blockNumber, bytes32[] memory operatorIds) external view returns (uint32[] memory);
    function getQuorumBitmapUpdateByIndex(bytes32 operatorId, uint256 index) external view returns (IRegistryCoordinator.QuorumBitmapUpdate memory);
    function indexRegistry() external view returns (address);
    function initialize(address _initialOwner, address _churnApprover, address _ejector, uint256 _initialPausedStatus, IRegistryCoordinator.OperatorSetParam[] memory _operatorSetParams, uint96[] memory _minimumStakes, IStakeRegistry.StrategyParams[][] memory _strategyParams, StakeType[] memory _stakeTypes, uint32[] memory _lookAheadPeriods) external;
    function isChurnApproverSaltUsed(bytes32) external view returns (bool);
    function isM2Quorum(uint8) external view returns (bool);
    function isOperatorSetAVS() external view returns (bool);
    function isUsingOperatorSets() external view returns (bool);
    function lastEjectionTimestamp(address) external view returns (uint256);
    function numRegistries() external view returns (uint256);
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
    function registerOperatorWithChurn(bytes memory quorumNumbers, string memory socket, IBLSApkRegistry.PubkeyRegistrationParams memory params, IRegistryCoordinator.OperatorKickParam[] memory operatorKickParams, ISignatureUtils.SignatureWithSaltAndExpiry memory churnApproverSignature, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function registries(uint256) external view returns (address);
    function renounceOwnership() external;
    function serviceManager() external view returns (address);
    function setChurnApprover(address _churnApprover) external;
    function setEjectionCooldown(uint256 _ejectionCooldown) external;
    function setEjector(address _ejector) external;
    function setOperatorSetParams(uint8 quorumNumber, IRegistryCoordinator.OperatorSetParam memory operatorSetParams) external;
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
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "contract IAVSDirectory"
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
        "internalType": "struct IRegistryCoordinator.OperatorKickParam[]",
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
        "internalType": "struct IRegistryCoordinator.OperatorSetParam",
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
        "internalType": "struct IRegistryCoordinator.OperatorSetParam",
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
        "internalType": "struct IRegistryCoordinator.OperatorInfo",
        "components": [
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IRegistryCoordinator.OperatorStatus"
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
        "internalType": "struct IRegistryCoordinator.OperatorSetParam",
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
        "internalType": "enum IRegistryCoordinator.OperatorStatus"
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
        "internalType": "struct IRegistryCoordinator.QuorumBitmapUpdate",
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
        "name": "_operatorSetParams",
        "type": "tuple[]",
        "internalType": "struct IRegistryCoordinator.OperatorSetParam[]",
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
        "name": "_minimumStakes",
        "type": "uint96[]",
        "internalType": "uint96[]"
      },
      {
        "name": "_strategyParams",
        "type": "tuple[][]",
        "internalType": "struct IStakeRegistry.StrategyParams[][]",
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
        "name": "_stakeTypes",
        "type": "uint8[]",
        "internalType": "enum StakeType[]"
      },
      {
        "name": "_lookAheadPeriods",
        "type": "uint32[]",
        "internalType": "uint32[]"
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
        "name": "",
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
    "name": "isOperatorSetAVS",
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
    "name": "isUsingOperatorSets",
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
        "internalType": "struct IRegistryCoordinator.OperatorKickParam[]",
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
        "internalType": "struct IRegistryCoordinator.OperatorSetParam",
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
        "internalType": "struct IRegistryCoordinator.OperatorSetParam",
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
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MaxQuorumsReached",
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
    "name": "OperatorSetsEnabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorSetsNotEnabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorSetsNotSupported",
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
    clippy::style
)]
pub mod RegistryCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61020080604052346103b85760c081615d16803803809161002082856103bc565b8339810103126103b85780516001600160a01b03811681036103b85760208201516001600160a01b03811681036103b8576040830151906001600160a01b03821682036103b8576060840151926001600160a01b03841684036103b8576080850151946001600160a01b03861686036103b85760a001516001600160a01b0381168082036103b8576040516100b66040826103bc565b6016815260208101907f4156535265676973747279436f6f7264696e61746f72000000000000000000008252604051916100f16040846103bc565b6006835260208301916576302e302e3160d01b8352519020915190208160e05280610100524660a0526040519060208201925f516020615cf65f395f51905f528452604083015260608201524660808201523060a082015260a0815261015860c0826103bc565b5190206080523060c0525f516020615cf65f395f51905f5261012052156103a95761014052610160526101a052610180526101c0526101e0525f5460ff8160081c166103545760ff8082161061031a575b60405161590290816103f48239608051816150ba015260a05181615171015260c05181615084015260e051816151090152610100518161512f015261012051816150e601526101405181818161095a015281816112a801528181611cd80152612872015261016051818181610c1b0152818161105001528181611b0f0152818161201601528181612382015281816126ea01528181613fdc015281816144150152614c02015261018051818181610b2f01528181610f1e015281816113e6015281816130a101528181613e30015281816142c6015281816145de01528181614a440152614f3301526101a051818181610e57015281816114b201528181611a490152818161306f0152818161371401528181613d6801528181613ee80152818161432b01528181614acd0152614d9301526101c051818181610ebd015281816120ee01528181612bae015281816130d301528181613dcf0152818161438d0152614b3701526101e051816114f60152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6101a9565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b6339b190bb60e11b5f5260045ffd5b5f80fd5b601f909101601f19168101906001600160401b038211908210176103df57604052565b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8062cf2ab51461039e57806303fd34921461039957806304ec635114610394578063054310e61461038f5780630cf4b7671461038a5780630d3f213414610385578063125e05841461038057806313542a4e1461037b578063136439dd146103765780631478851f146103715780631eb812da1461036c578063249a0c421461036757806328f61b3114610362578063296bb0641461035d57806329d1e0c3146103585780632cdd1e86146103535780633998fdd31461034e5780633c2a7f4c146103495780633eef3a51146103445780635140a5481461033f5780635865c60c1461033a578063595c6a67146103355780635ac86ab7146103305780635b0b829f1461032b5780635c975abb146103265780635df45946146103215780636347c9001461031c57806368304835146103175780636b3aa72e146103125780636e3b17db1461030d578063715018a6146103085780637fc3f886146103035780638281ab75146102fe57806384ca5213146102f9578063871ef049146102f4578063886f1195146102ef5780638da5cb5b146102ea5780639aa1653d146102e55780639b5d177b146102e05780639d8e0c23146102db5780639e9923c2146102d65780639feab859146102d1578063a4d7871f146102cc578063a50857bf146102c7578063a96f783e146102c2578063adcf73f7146102bd578063bd33ee24146102a9578063c391425e146102b8578063ca0de882146102b3578063ca4f2d97146102ae578063cabbb17f146102a9578063d72d8dd6146102a4578063e65797ad1461029f578063ee3188211461029a578063f2fde38b14610295578063fabc1cbc146102905763fd39105a1461028b575f80fd5b61292b565b612849565b6127b8565b6126d0565b61263c565b61261f565b612439565b612578565b61253e565b61249a565b6122be565b6122a1565b61218e565b612157565b61211d565b6120d9565b611f49565b611e83565b611d2f565b611d07565b611cc3565b611c93565b611c37565b6119b3565b6118dc565b61162c565b611525565b6114e1565b61149d565b611443565b6113d1565b6113b4565b61131f565b6112f0565b61127d565b611211565b611120565b610d8b565b610c4a565b610c06565b610bd9565b610bac565b610af9565b610ad1565b610a9f565b610a17565b6109e8565b61092a565b6108ef565b6108b4565b610893565b6107f3565b61075c565b6105ef565b6105b7565b6104ed565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176103d257604052565b6103a3565b606081019081106001600160401b038211176103d257604052565b90601f801991011681019081106001600160401b038211176103d257604052565b604051906104226040836103f2565b565b604051906104226060836103f2565b6001600160401b0381116103d25760051b60200190565b6001600160a01b0381160361045b57565b5f80fd5b600435906104228261044a565b602435906104228261044a565b604435906104228261044a565b9080601f8301121561045b57813561049d81610433565b926104ab60405194856103f2565b81845260208085019260051b82010192831161045b57602001905b8282106104d35750505090565b6020809183356104e28161044a565b8152019101906104c6565b3461045b57602036600319011261045b576004356001600160401b03811161045b5761051d903690600401610486565b61053461052e600480600154161490565b15612971565b5f5b81518110156105b5576001906105af6001600160a01b036105578386612994565b5116805f52609960205260405f2061058860ff8660405193610578856103b7565b80548552015416602083016129a8565b6105a96105a4610598835161503d565b6001600160c01b031690565b6135e8565b916136ca565b01610536565b005b3461045b57602036600319011261045b576004355f526098602052602060405f2054604051908152f35b63ffffffff81160361045b57565b3461045b57606036600319011261045b5760243561062f610629600435610615846105e1565b604435905f52609860205260405f20611429565b50612a92565b63ffffffff8082511692169182106106b35760408161067761069f9461065f602061068596015163ffffffff1690565b9063ffffffff8216159182156106a3575b50506137a0565b01516001600160c01b031690565b6040516001600160c01b0390911681529081906020820190565b0390f35b63ffffffff161190505f80610670565b60405162461bcd60e51b815260206004820152606560248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d61704174426c6f636b4e756d6265724279496e6465783a2071756f72756d4260648201527f69746d61705570646174652069732066726f6d20616674657220626c6f636b4e6084820152643ab6b132b960d91b60a482015260c490fd5b5f91031261045b57565b3461045b575f36600319011261045b57609d546040516001600160a01b039091168152602090f35b6001600160401b0381116103d257601f01601f191660200190565b9291926107ab82610784565b916107b960405193846103f2565b82948184528183011161045b578281602093845f960137010152565b9080601f8301121561045b578160206107f09335910161079f565b90565b3461045b57602036600319011261045b576004356001600160401b03811161045b576108239036906004016107d5565b335f52609960205260ff600160405f20015416600381101561088e57600161084b91146129da565b335f5260996020527fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa61088960405f20549260405191829182612a14565b0390a2005b6111e6565b3461045b57602036600319011261045b576004356108af613847565b60a055005b3461045b57602036600319011261045b576004356108d18161044a565b60018060a01b03165f52609f602052602060405f2054604051908152f35b3461045b57602036600319011261045b5760043561090c8161044a565b60018060a01b03165f526099602052602060405f2054604051908152f35b3461045b57602036600319011261045b5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156109e3576105b5926109a0915f916109b4575b50612a48565b6109af60015482811614612a5e565b6138d3565b6109d6915060203d6020116109dc575b6109ce81836103f2565b810190612a25565b5f61099a565b503d6109c4565b612a3d565b3461045b57602036600319011261045b576004355f52609a602052602060ff60405f2054166040519015158152f35b3461045b57604036600319011261045b576060610a4f610629602435600435610a3e612a74565b505f52609860205260405f20611429565b6040519063ffffffff815116825263ffffffff6020820151166020830152604060018060c01b03910151166040820152f35b6004359060ff8216820361045b57565b359060ff8216820361045b57565b3461045b57602036600319011261045b5760ff610aba610a81565b165f52609b602052602060405f2054604051908152f35b3461045b575f36600319011261045b57609e546040516001600160a01b039091168152602090f35b3461045b57602036600319011261045b576040516308f6629d60e31b815260048035908201526020816024816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa80156109e35761069f915f91610b7d575b506040516001600160a01b0390911681529081906020820190565b610b9f915060203d602011610ba5575b610b9781836103f2565b810190612ac3565b5f610b62565b503d610b8d565b3461045b57602036600319011261045b576105b5600435610bcc8161044a565b610bd4613847565b613905565b3461045b57602036600319011261045b576105b5600435610bf98161044a565b610c01613847565b613963565b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461045b57602036600319011261045b576040610c71600435610c6c8161044a565b612af0565b610c878251809260208091805184520151910152565bf35b359061ffff8216820361045b57565b606090600319011261045b5760405190610cb1826103d7565b81600435610cbe816105e1565b815260243561ffff8116810361045b5760208201526044359061ffff8216820361045b5760400152565b6001600160601b0381160361045b57565b81601f8201121561045b57803590610d1082610433565b92610d1e60405194856103f2565b82845260208085019360061b8301019181831161045b57602001925b828410610d48575050505090565b60408483031261045b5760206040918251610d62816103b7565b8635610d6d8161044a565b815282870135610d7c81610ce8565b83820152815201930192610d3a565b3461045b5760c036600319011261045b57610da536610c98565b606435610db181610ce8565b6084356001600160401b03811161045b57610dd0903690600401610cf9565b9060a43591610dde836105e1565b610de6613847565b610df460ff60a15416612b56565b60965460ff16938490610e2f90610e0d60c08410613474565b610e29610e1988613b2c565b60ff1660ff196096541617609655565b866140fb565b60a15460ff16806110c7575b610fb9575b50610e4b6001613014565b610e556001613014565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b57610eab935f809460405196879586948593630662d3e160e51b85528b60048601613cb0565b03925af180156109e357610fa5575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163136ca0f960e11b815260ff83166004820152905f908290602490829084905af180156109e357610f91575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163136ca0f960e11b815260ff83166004820152905f908290602490829084905af180156109e357610f7d57005b80610f8b5f6105b5936103f2565b80610752565b80610f8b5f610f9f936103f2565b5f610f1b565b80610f8b5f610fb3936103f2565b5f610eba565b92610fc2613b3e565b92610fcd8351613b8b565b935f5b84518110156110195780611013610ffa610fec60019489612994565b51516001600160a01b031690565b611004838a612994565b6001600160a01b039091169052565b01610fd0565b5091949093611035611029610413565b63ffffffff9093168352565b602082015261104382612987565b5261104d81612987565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163847d634f60e01b8152915f9183918290849082906110a49060048301613bb3565b03925af180156109e35715610e405780610f8b5f6110c1936103f2565b5f610e40565b506110ee6110ea6110e38760ff165f5260a260205260405f2090565b5460ff1690565b1590565b610e3b565b9181601f8401121561045b578235916001600160401b03831161045b576020838186019501011161045b57565b3461045b57604036600319011261045b576004356001600160401b03811161045b573660238201121561045b57806004013561115b81610433565b9161116960405193846103f2565b8183526024602084019260051b8201019036821161045b5760248101925b8284106111b757602435856001600160401b03821161045b576111b16105b59236906004016110f3565b91612b6c565b83356001600160401b03811161045b576020916111db839260243691870101610486565b815201930192611187565b634e487b7160e01b5f52602160045260245ffd5b6003111561088e57565b90600382101561088e5752565b3461045b57602036600319011261045b5760043561122e8161044a565b611236612ad8565b5060018060a01b03165f52609960205260405f2061125e60ff600160405193610578856103b7565b604051809161069f602060408401928051855201516020840190611204565b3461045b575f36600319011261045b5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156109e3576112e8915f916109b45750612a48565b6105b561389f565b3461045b57602036600319011261045b576020600160ff61130f610a81565b161b806001541614604051908152f35b3461045b57608036600319011261045b57611338610a81565b606036602319011261045b57604051611350816103d7565b60243561135c816105e1565b815260443561ffff8116810361045b57602082015260643561ffff8116810361045b57604082015261138c613847565b60ff6096541660ff831610156113a5576105b5916140fb565b637310cff560e11b5f5260045ffd5b3461045b575f36600319011261045b576020600154604051908152f35b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b634e487b7160e01b5f52603260045260245ffd5b805482101561143e575f5260205f2001905f90565b611415565b3461045b57602036600319011261045b57600435609c5481101561045b57609c5f527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c01546040516001600160a01b039091168152602090f35b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461045b57604036600319011261045b576004356115428161044a565b6024356001600160401b03811161045b576115619036906004016107d5565b609e546001600160a01b0316330361161d576001600160a01b0382165f908152609f6020908152604080832042905560999091529020805460016115c9816115c06115ba6105986115b460965460ff1690565b89614074565b9461503d565b94015460ff1690565b6115d2816111fa565b14918261160a575b826115f1575b50506115e857005b6105b59161420d565b81166001600160c01b0390811691161490505f806115e0565b6001600160c01b038216151592506115da565b6376d8ab1760e11b5f5260045ffd5b3461045b575f36600319011261045b57611644613847565b606480546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b81601f8201121561045b5780359061169e82610433565b926116ac60405194856103f2565b8284526020606081860194028301019181831161045b57602001925b8284106116d6575050505090565b60608483031261045b5760206060916040516116f1816103d7565b86356116fc816105e1565b8152611709838801610c89565b8382015261171960408801610c89565b60408201528152019301926116c8565b9080601f8301121561045b57813561174081610433565b9261174e60405194856103f2565b81845260208085019260051b82010192831161045b57602001905b8282106117765750505090565b60208091833561178581610ce8565b815201910190611769565b9080601f8301121561045b5781356117a781610433565b926117b560405194856103f2565b81845260208085019260051b8201019183831161045b5760208201905b8382106117e157505050505090565b81356001600160401b03811161045b5760209161180387848094880101610cf9565b8152019101906117d2565b9080601f8301121561045b5781359061182682610433565b9261183460405194856103f2565b82845260208085019360051b82010191821161045b57602001915b81831061185c5750505090565b8235600281101561045b5781526020928301920161184f565b9080601f8301121561045b57813561188c81610433565b9261189a60405194856103f2565b81845260208085019260051b82010192831161045b57602001905b8282106118c25750505090565b6020809183356118d1816105e1565b8152019101906118b5565b3461045b5761012036600319011261045b576118f661045f565b6118fe61046c565b90611907610479565b6064356084356001600160401b03811161045b57611929903690600401611687565b60a4356001600160401b03811161045b57611948903690600401611729565b9060c4356001600160401b03811161045b57611968903690600401611790565b9260e4356001600160401b03811161045b5761198890369060040161180e565b9461010435976001600160401b03891161045b576119ad6105b5993690600401611875565b97612edd565b3461045b5760a036600319011261045b576119cd36610c98565b6064356119d981610ce8565b6084356001600160401b03811161045b576119f8903690600401610cf9565b90611a01613847565b60965460ff16928390611a2c90611a1a60c08410613474565b611a26610e1987613b2c565b856140fb565b60a15460ff1680611b86575b611a9d575b50611a475f613014565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561045b57610eab925f9283604051809681958294633aea0b9d60e11b84528a60048501613ce5565b919092611aa8613b3e565b93611ab38351613b8b565b945f5b8451811015611ae25780611adc611ad2610fec60019489612994565b611004838b612994565b01611ab6565b50919493909293611af4611029610413565b6020820152611b0282612987565b52611b0c81612987565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163847d634f60e01b8152915f918391829084908290611b639060048301613bb3565b03925af180156109e35715611a3d5780610f8b5f611b80936103f2565b5f611a3d565b50611ba26110ea6110e38660ff165f5260a260205260405f2090565b611a38565b81601f8201121561045b57803590611bbe82610433565b92611bcc60405194856103f2565b82845260208085019360061b8301019181831161045b57602001925b828410611bf6575050505090565b60408483031261045b5760206040918251611c10816103b7565b611c1987610a91565b815282870135611c288161044a565b83820152815201930192611be8565b3461045b5760a036600319011261045b57600435611c548161044a565b60243590604435906001600160401b03821161045b57602092611c7e611c8b933690600401611ba7565b6064359160843593613197565b604051908152f35b3461045b57602036600319011261045b576020611cb160043561503d565b6040516001600160c01b039091168152f35b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461045b575f36600319011261045b576064546040516001600160a01b039091168152602090f35b3461045b575f36600319011261045b57602060ff60965416604051908152f35b919082604091031261045b57604051611d67816103b7565b6020808294803584520135910152565b9080601f8301121561045b5760405191611d926040846103f2565b82906040810192831161045b57905b828210611dae5750505090565b8135815260209182019101611da1565b9061010060431983011261045b5760405191611dd9836103d7565b82611de5826044611d4f565b8152611df2826084611d4f565b6020820152608060c31983011261045b57604090611e2a825193611e15856103b7565b611e208160c4611d77565b8552610104611d77565b60208401520152565b91909160608184031261045b5760405190611e4d826103d7565b81938135916001600160401b03831161045b57611e7060409392849383016107d5565b8452602081013560208501520135910152565b3461045b576101a036600319011261045b576004356001600160401b03811161045b57611eb49036906004016110f3565b906024356001600160401b03811161045b57611ed49036906004016107d5565b611edd36611dbe565b610144356001600160401b03811161045b57611efd903690600401611ba7565b90610164356001600160401b03811161045b57611f1e903690600401611e33565b9261018435956001600160401b03871161045b57611f436105b5973690600401611e33565b95613252565b3461045b57604036600319011261045b57600435611f668161044a565b6024356001600160401b03811161045b57611f85903690600401611875565b90611f9661052e6001808054161490565b611faa611fa560ff60a1541690565b612b56565b5f5b82518110156120005780611ffa611ff56110ea6110e3611fe4611fde611fd46001988b612994565b5163ffffffff1690565b60ff1690565b60ff165f5260a260205260405f2090565b613405565b01611fac565b5060405163ca8aa7c760e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156109e357612062915f916120ba575b506001600160a01b0316331461341b565b61206c8251613431565b5f5b83518110156120b0578061209d61208d611fde611fd460019589612994565b60f81b6001600160f81b03191690565b5f1a6120a98285613463565b530161206e565b506105b59161420d565b6120d3915060203d602011610ba557610b9781836103f2565b5f612051565b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461045b575f36600319011261045b5760206040517f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de68152f35b3461045b57602036600319011261045b5760ff612172610a81565b165f5260a2602052602060ff60405f2054166040519015158152f35b3461045b5761016036600319011261045b576004356001600160401b03811161045b576121bf9036906004016107d5565b6024356001600160401b03811161045b576121de9036906004016107d5565b6121e736611dbe565b61014435916001600160401b03831161045b578361223561220f61223b953690600401611e33565b9361222061052e6001808054161490565b61222f60ff60a15416156133bd565b336145bc565b3361493c565b51905f5b81518110156105b5578061229b61225860019385613463565b5160f81c63ffffffff6122928161228781612273888c612994565b51169460ff165f52609760205260405f2090565b541663ffffffff1690565b91161115613474565b0161223f565b3461045b575f36600319011261045b57602060a054604051908152f35b3461045b57606036600319011261045b576004356122db8161044a565b6024356001600160401b03811161045b576122fa903690600401611875565b906044356001600160401b03811161045b5761231a9036906004016107d5565b9061232b61052e6001808054161490565b61233a611fa560ff60a1541690565b5f5b835181101561236a5780612364611ff56110ea6110e3611fe4611fde611fd46001988c612994565b0161233c565b5060405163ca8aa7c760e01b815290916020826004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156109e3576123d56123e4926123ed945f916120ba57506001600160a01b0316331461341b565b602080825183010191016134f9565b839291926145bc565b906123f88451613431565b915f5b855181101561242d578061241a61208d611fde611fd46001958b612994565b5f1a6124268287613463565b53016123fb565b50916105b59284614e5d565b3461045b575f36600319011261045b57602060ff60a154166040519015158152f35b60206040818301928281528451809452019201905f5b81811061247e5750505090565b825163ffffffff16845260209384019390920191600101612471565b3461045b57604036600319011261045b576004356124b7816105e1565b602435906001600160401b03821161045b573660238301121561045b578160040135916124e383610433565b926124f160405194856103f2565b8084526024602085019160051b8301019136831161045b57602401905b82821061252e5761069f6125228686614fb9565b6040519182918261245b565b813581526020918201910161250e565b3461045b575f36600319011261045b5760206040517f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a8152f35b3461045b57602036600319011261045b576004356001600160401b03811161045b576125a89036906004016107d5565b6125b961052e600280600154161490565b5f60ff60a15416158015915b835181101561261557806125db60019286613463565b5160f81c83856125f6575b6125f091506133bd565b016125c5565b505f5260a26020526125f061261060405f2060ff90541690565b6125e6565b6105b5843361420d565b3461045b575f36600319011261045b576020609c54604051908152f35b3461045b57602036600319011261045b5760ff612657610a81565b61265f612a74565b50165f52609760205261069f60405f2061ffff6040519161267f836103d7565b5463ffffffff81168352818160201c16602084015260301c16604082015260405191829182919091604061ffff81606084019563ffffffff8151168552826020820151166020860152015116910152565b3461045b575f36600319011261045b576126e8613847565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b57604051630f25f16160e41b8152306004820152905f908290602490829084905af180156109e3576127a4575b505f5b612757611fde60965460ff1690565b60ff821610156127925760018161278b61277e60ff9460ff165f5260a260205260405f2090565b805460ff19166001179055565b0116612748565b6105b5600160ff1960a154161760a155565b80610f8b5f6127b2936103f2565b5f612745565b3461045b57602036600319011261045b576004356127d58161044a565b6127dd613847565b6001600160a01b038116156127f5576105b5906144c3565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461045b57602036600319011261045b5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156109e3575f9161290c575b506001600160a01b031633036128fd576128cb600154198219811614612a5e565b806001556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b612925915060203d602011610ba557610b9781836103f2565b5f6128aa565b3461045b57602036600319011261045b576004356129488161044a565b60018060a01b03165f526099602052602060ff600160405f20015416610c876040518092611204565b1561297857565b63840a48d560e01b5f5260045ffd5b80511561143e5760200190565b805182101561143e5760209160051b010190565b600382101561088e5752565b906104226040516129c4816103b7565b602060ff600183968054855201541691016129a8565b156129e157565b63aba4733960e01b5f5260045ffd5b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060206107f09281815201906129f0565b9081602091031261045b5751801515810361045b5790565b6040513d5f823e3d90fd5b15612a4f57565b631d77d47760e21b5f5260045ffd5b15612a6557565b63c61dca5d60e01b5f5260045ffd5b60405190612a81826103d7565b5f6040838281528260208201520152565b90604051612a9f816103d7565b604081935463ffffffff8116835263ffffffff8160201c166020840152811c910152565b9081602091031261045b57516107f08161044a565b60405190612ae5826103b7565b5f6020838281520152565b612b516107f091612aff612ad8565b50604080517f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de6602082019081526001600160a01b0390931681830152908152612b496060826103f2565b5190206139c1565b613a0e565b15612b5d57565b635b77901960e01b5f5260045ffd5b909291612b8061052e600480600154161490565b612b9f612b8f60965460ff1690565b612b9a36848861079f565b614074565b50612bac81835114612db5565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316935f5b828110612be857505050509050565b612c0d612c07612bf9838686612dcb565b356001600160f81b03191690565b60f81c90565b92612c188286612994565b5180516040516379a0849160e11b815260ff87166004820152919791906020826024818d5afa9182156109e357612c5e9263ffffffff915f91612d87575b501614612dec565b5f97885b88518a1015612d1b57600190612d13612c8b612c7e8d8d612994565b516001600160a01b031690565b91612cee612cb1612cac8560018060a01b03165f52609960205260405f2090565b6129b4565b91612cd9612cd48d612cc6610598875161503d565b60ff600192161c1660011490565b612e02565b858060a01b0316858060a01b03851611612e18565b612d0c612d05612cfd8a612e42565b8a8a8d612e62565b369161079f565b90836136ca565b990198612c62565b5096509650929060019194929443612d3e8260ff165f52609b60205260405f2090565b557f46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db460ff60405192169180612d7843829190602083019252565b0390a201949394929092612bd9565b612da8915060203d8111612dae575b612da081836103f2565b810190612dd7565b5f612c56565b503d612d96565b15612dbc57565b63aaad13f760e01b5f5260045ffd5b9082101561143e570190565b9081602091031261045b57516107f0816105e1565b15612df357565b638e5aeee760e01b5f5260045ffd5b15612e0957565b63d053aa2160e01b5f5260045ffd5b15612e1f57565b63ba50f91160e01b5f5260045ffd5b634e487b7160e01b5f52601160045260245ffd5b9060018201809211612e5057565b612e2e565b91908201809211612e5057565b9093929384831161045b57841161045b578101920390565b15612e8157565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b97959391612f27979593915f5499612f0d60ff8c60081c16151515809c81612fa1575b8115612f81575b50612e7a565b8a612f1e600160ff195f5416175f55565b612f6a5761302b565b612f2d57565b612f3b61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1565b612f7c61010061ff00195f5416175f55565b61302b565b303b15915081612f93575b505f612f07565b60ff1660011490505f612f8c565b600160ff8216109150612f00565b609c54600160401b8110156103d25760018101609c55609c5481101561143e57609c5f527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c0180546001600160a01b0319166001600160a01b03909216919091179055565b6002111561088e57565b51600281101561088e5790565b926109af610c0192610bd461306a969c9b9a99989c8d89519051809114908161318c575b5080613181575b80613176575b61306590612db5565b6144c3565b61309c7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612faf565b6130ce7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612faf565b6131007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612faf565b5f5b815181101561316d578061316761311b60019385612994565b51613136613129848c612994565b516001600160601b031690565b6131408488612994565b5161315361314e868b612994565b61301e565b91613161611fd4878d612994565b93613d0d565b01613102565b50505050509050565b508a518c511461305c565b5089518b5114613056565b90508a51145f61304f565b919493909260405192602084019460e08501917f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a875260018060a01b03166040860152606085015260c060808501528651809152602061010085019701905f5b818110613224575050506107f09495612b4992849260a084015260c083015203601f1981018352826103f2565b8251805160ff168a526020908101516001600160a01b0316818b0152604090990198909201916001016131f7565b9461329e6132956132b095989493969861327261052e6001808054161490565b6132896132846110ea60ff60a1541690565b6133bd565b61222f888b5114612db5565b918883336146be565b6132a936868861079f565b903361493c565b925f5b8281106132c1575050505050565b806132f16132ec6132db612c07612bf9600196898b612dcb565b60ff165f52609760205260405f2090565b6133d3565b6132ff611fd4838951612994565b63ffffffff61331e613315845163ffffffff1690565b63ffffffff1690565b91161161332d575b50016132b3565b61337990613342612c07612bf985898b612dcb565b6133536131298560408c0151612994565b906133656131298660208d0151612994565b906133708689612994565b51923391614d07565b6133b761339a602061338b8487612994565b5101516001600160a01b031690565b6133b1612d056133a985612e42565b85898b612e62565b9061420d565b5f613326565b156133c457565b630b88306f60e01b5f5260045ffd5b906040516133e0816103d7565b604061ffff82945463ffffffff81168452818160201c16602085015260301c16910152565b1561340c57565b63fd2c1f4d60e01b5f5260045ffd5b1561342257565b6323d871a560e01b5f5260045ffd5b9061343b82610784565b61344860405191826103f2565b8281528092613459601f1991610784565b0190602036910137565b90815181101561143e570160200190565b1561347b57565b633cb89c9760e01b5f5260045ffd5b919082604091031261045b576040516134a2816103b7565b6020808294805184520151910152565b9080601f8301121561045b57604051916134cd6040846103f2565b82906040810192831161045b57905b8282106134e95750505090565b81518152602091820191016134dc565b91909180830390610120821261045b5780516001600160401b03811161045b57810184601f8201121561045b57805161353181610784565b9161353f60405193846103f2565b818352866020838301011161045b57815f9260208093018386015e8301015293610100601f1984011261045b5760806040519361357b856103d7565b613588836020860161348a565b8552613597836060860161348a565b6020860152609f19011261045b576135cd9060e0604051936135b8856103b7565b6135c58360a083016134b2565b8552016134b2565b6020820152604082015290565b5f198114612e505760010190565b5f81805b61366257506135fe9061ffff16613431565b5f5f5b8251821080613657575b15613650576001811b8416613629575b613624906135da565b613601565b9060016136249160ff60f81b8460f81b165f1a6136468287613463565b530191905061361b565b5050905090565b50610100811061360b565b5f198101818111612e505761ffff9116911661ffff8114612e505760010190806135ec565b9081602091031261045b57516001600160c01b038116810361045b5790565b6107f0939260609260018060a01b03168252602082015281604082015201906129f0565b9190600160208201516136dc816111fa565b6136e5816111fa565b0361379b57516040516333567f7f60e11b8152916020918391829161370f919087600485016136a6565b03815f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af19081156109e3575f9161376c575b506001600160c01b03169081613760575050565b6133b1610422926135e8565b61378e915060203d602011613794575b61378681836103f2565b810190613687565b5f61374c565b503d61377c565b505050565b156137a757565b60405162461bcd60e51b815260206004820152606660248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d61704174426c6f636b4e756d6265724279496e6465783a2071756f72756d4260648201527f69746d61705570646174652069732066726f6d206265666f726520626c6f636b608482015265273ab6b132b960d11b60a482015260c490fd5b6064546001600160a01b0316330361385b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b5f196001556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806001556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b609d54604080516001600160a01b038084168252841660208201529192917f315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c9190a16001600160a01b03166001600160a01b03199190911617609d55565b609e54604080516001600160a01b038084168252841660208201529192917f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc99190a16001600160a01b03166001600160a01b03199190911617609e55565b6139c9615081565b9060405190602082019261190160f01b845260228301526042820152604281526139f46062826103f2565b51902090565b634e487b7160e01b5f52601260045260245ffd5b5f5160206158ad5f395f51905f5290613a25612ad8565b505f919006602060c0835b613b25575f935f5160206158ad5f395f51905f5260038186818180090908604051613a5b85826103f2565b84368237848185604051613a6f82826103f2565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f5160206158ad5f395f51905f5260a082015260056107cf195a01fa8015613b2a57613ad9906155b7565b5191613b25575f5160206158ad5f395f51905f5282800914613b1057505f5160206158ad5f395f51905f5260015f94089293613a30565b92935050613b1c610413565b92835282015290565b6139fa565bfe5b60ff60019116019060ff8211612e5057565b60408051909190613b4f83826103f2565b6001815291601f1901825f5b828110613b6757505050565b602090604051613b76816103b7565b5f815260608382015282828501015201613b5b565b90613b9582610433565b613ba260405191826103f2565b8281528092613459601f1991610433565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310613be557505050505090565b9091929394603f1982820301835285516020606081604085019363ffffffff81511686520151936040838201528451809452019201905f905b808210613c3d5750505060208060019297019301930191939290613bd6565b82516001600160a01b0316845260209384019390920191600190910190613c1e565b90602080835192838152019201905f5b818110613c7c5750505090565b825180516001600160a01b031685526020908101516001600160601b03168186015260409094019390920191600101613c6f565b906107f094936001600160601b0360809460ff63ffffffff941685521660208401521660408201528160608201520190613c5f565b6001600160601b036107f0949360ff6060941683521660208201528160408201520190613c5f565b93909192613d1d60965460ff1690565b94613d4460ff871691613d3260c08410613474565b613d3e610e1989613b2c565b876140fb565b60a15460ff1680614053575b613f66575b50613d5f81613014565b80613ec95750507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561045b57613dbc925f9283604051809681958294633aea0b9d60e11b84528a60048501613ce5565b03925af180156109e357613eb5575b505b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163136ca0f960e11b815260ff83166004820152905f908290602490829084905af180156109e357613ea1575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163136ca0f960e11b815260ff90921660048301525f908290818381602481015b03925af180156109e357613e935750565b80610f8b5f610422936103f2565b80610f8b5f613eaf936103f2565b5f613e2d565b80610f8b5f613ec3936103f2565b5f613dcb565b80613ed8600192959395613014565b14613ee6575b505050613dcd565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b57613f3c935f809460405196879586948593630662d3e160e51b85528b60048601613cb0565b03925af180156109e357613f52575b8080613ede565b80610f8b5f613f60936103f2565b5f613f4b565b9592909491613f73613b3e565b95613f7e8651613b8b565b965f5b8751811015613fad5780613fa7613f9d610fec6001948c612994565b611004838d612994565b01613f81565b509193969790929497613fc1611029610413565b6020820152613fcf82612987565b52613fd981612987565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163847d634f60e01b8152915f9183918290849082906140309060048301613bb3565b03925af180156109e35715613d555780610f8b5f61404d936103f2565b5f613d55565b5061406f6110ea6110e38860ff165f5260a260205260405f2090565b613d50565b90600161408260ff93615219565b928392161b11156140905790565b60405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608490fd5b6141b960ff7f3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac921692835f52609760205260405f2061415163ffffffff835116829063ffffffff1663ffffffff19825416179055565b6020820151815465ffff0000000067ffff000000000000604086015160301b169260201b169067ffffffff0000000019161717905560405191829182919091604061ffff81606084019563ffffffff8151168552826020820151166020860152015116910152565b0390a2565b156141c557565b6368b6a87560e11b5f5260045ffd5b6001600160a01b0390911681526040602082018190526107f0929101906129f0565b6040906107f09392815281602082015201906129f0565b6001600160a01b0381165f9081526099602052604090209060018254920161424a600161423b835460ff1690565b614244816111fa565b146129da565b6142aa61426561059861425f60965460ff1690565b87614074565b61426e8561503d565b6001600160c01b03909116906142858215156141be565b61429b8282166001600160c01b03168314612e02565b9019166001600160c01b031690565b6142b4818561530e565b6001600160c01b031615614408575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b57835f91614319938360405180968195829463f4e24fe560e01b8452600484016141d4565b03925af180156109e3576143f4575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b575f604051809263bd29b8cd60e01b825281838161437b8989600484016141f6565b03925af180156109e3576143e0575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561045b57613e82925f928360405180968195829463bd29b8cd60e01b8452600484016141f6565b80610f8b5f6143ee936103f2565b5f61438a565b80610f8b5f614402936103f2565b5f614328565b805460ff191660021790557f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b576040516351b27a6d60e11b81526001600160a01b0383166004820152905f908290602490829084905af180156109e3576144af575b50816001600160a01b0382167f396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e45f80a35f6142c3565b80610f8b5f6144bd936103f2565b5f614479565b606480546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b9081602091031261045b575190565b905f905b6002821061452b57505050565b602080600192855181520193019101909161451e565b610120906145ab60206040610422969897959861016085019960018060a01b0316855261457b838601825160208091805184520151910152565b8083015180516060870152602001516080860152015161459f60a08501825161451a565b015160e083019061451a565b019060208091805184520151910152565b6040516309aa152760e11b81526001600160a01b0382811660048301529091907f000000000000000000000000000000000000000000000000000000000000000016602083602481845afa9283156109e3575f9361469d575b508215614623575050905090565b60209250614653935f61463584612af0565b6040516317ef39cb60e31b8152968795869485939160048501614541565b03925af19081156109e3575f9161466e575b50805f80613650565b614690915060203d602011614696575b61468881836103f2565b81019061450b565b5f614665565b503d61467e565b6146b791935060203d6020116146965761468881836103f2565b915f614615565b919290602082019283515f52609a60205260ff60405f20541661473957604083018051421161472a57610422956147229386515f52609a60205261470c60405f20600160ff19825416179055565b609d546001600160a01b03169651925193613197565b905191615428565b630819bdcd60e01b5f5260045ffd5b636fbefec360e11b5f5260045ffd5b60405190614755826103d7565b60606040838281528260208201520152565b1561476e57565b6313ca465760e01b5f5260045ffd5b1561478457565b630c6816cd60e01b5f5260045ffd5b1561479a57565b631968677d60e11b5f5260045ffd5b60016020918351815501910151600381101561088e5760ff80198354169116179055565b9060018060a01b0316815260406020820152608060406147f8845160608386015260a08501906129f0565b9360208101516060850152015191015290565b9080601f8301121561045b57815161482281610433565b9261483060405194856103f2565b81845260208085019260051b82010192831161045b57602001905b8282106148585750505090565b60208091835161486781610ce8565b81520191019061484b565b91909160408184031261045b5780516001600160401b03811161045b578361489b91830161480b565b9260208201516001600160401b03811161045b576107f0920161480b565b60208183031261045b578051906001600160401b03821161045b57019080601f8301121561045b5781516148ec81610433565b926148fa60405194856103f2565b81845260208085019260051b82010192831161045b57602001905b8282106149225750505090565b602080918351614931816105e1565b815201910190614915565b90919293827fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa614a0761496d614748565b976149fb61498961059861498360965460ff1690565b8b614074565b6149928661503d565b6001600160c01b03909116906149a9821515614767565b60018060c01b03166149c36149be8284161590565b61477d565b6001600160a01b0389165f908152609f602052604090206149f4906149ed905b5460a05490612e55565b4211614793565b178561530e565b60405191829182612a14565b0390a26001614a3281614a2a8560018060a01b03165f52609960205260405f2090565b015460ff1690565b614a3b816111fa565b03614bcf575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b575f6040518092631fd93ca960e11b8252818381614a948a89600484016141d4565b03925af180156109e35784925f928592614bbb575b50614ac86040519687938493632550477760e01b8552600485016136a6565b0381837f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af19182156109e357614b32935f9384918591614b97575b5060408701526020860152604051938492839262bff04d60e01b8452600484016141f6565b0381837f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af19081156109e3575f91614b75575b50815290565b614b9191503d805f833e614b8981836103f2565b8101906148b9565b5f614b6f565b9050614bb591503d8086833e614bad81836103f2565b810190614872565b5f614b0d565b80610f8b85614bc9936103f2565b5f614aa9565b614c00614bda610413565b848152600160208201526001600160a01b0384165f9081526099602052604090206147a9565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b57604051639926ee7d60e01b8152915f918391829084908290614c579089600484016147cd565b03925af180156109e357614c9c575b50816001600160a01b0382167fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe5f80a35f614a41565b80610f8b5f614caa936103f2565b5f614c66565b15614cb757565b6356168b4160e11b5f5260045ffd5b9081602091031261045b57516107f081610ce8565b15614ce257565b634c44995d60e01b5f5260045ffd5b15614cf857565b63b187e86960e01b5f5260045ffd5b60209192614d64614d57614d8f989697614d50614d2c8783015160018060a01b031690565b6001600160a01b039081165f81815260996020526040902054969091161415614cb0565b5160ff1690565b60ff808516911614612dec565b604051635401ed2760e01b8152600481019190915260ff909116602482015294859081906044820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9283156109e357610422945f94614e13575b5082614e0b92614e06614df2936001600160601b03614dfe614df282998b61546c565b6001600160601b031690565b911611614cdb565b61548f565b911610614cf1565b614df291945092614e0b92614e06614e4a6001600160601b039660203d602011614e56575b614e4281836103f2565b810190614cc6565b96935050925092614dcf565b503d614e38565b817fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa614eff614e8a614748565b966149fb614ea6610598614ea060965460ff1690565b8a614074565b614eaf8661503d565b6001600160c01b0390911690614ec6821515614767565b60018060c01b0316614edb6149be8284161590565b6001600160a01b0388165f908152609f602052604090206149f4906149ed906149e3565b0390a26001614f2281614a2a8460018060a01b03165f52609960205260405f2090565b614f2b816111fa565b03614f83575b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b575f6040518092631fd93ca960e11b8252818381614a948a89600484016141d4565b614fb4614f8e610413565b838152600160208201526001600160a01b0383165f9081526099602052604090206147a9565b614f31565b9190805190614fc782610433565b91614fd560405193846103f2565b808352614fe4601f1991610433565b013660208401375f5b8151811015615028578061500f61500660019385612994565b518760986154ad565b63ffffffff61501e8387612994565b9116905201614fed565b5090925050565b5f19810191908211612e5057565b805f52609860205260405f20549081155f146150595750505f90565b5f52609860205260405f20905f198101908111612e505761507991611429565b505460401c90565b307f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316148061516e575b156150dc577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f000000000000000000000000000000000000000000000000000000000000000082527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a081526139f460c0826103f2565b507f000000000000000000000000000000000000000000000000000000000000000046146150b3565b1561519e57565b60405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a490fd5b90610100825111615297578151156152925761525561524b612c0761523d85612987565b516001600160f81b03191690565b60ff600191161b90565b6001905b835182101561528d5760019061527861524b612c0761523d8689613463565b90615284818311615197565b17910190615259565b925050565b5f9150565b60a460405162461bcd60e51b815260206004820152604460248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b6084820152fd5b90615321825f52609860205260405f2090565b548061536e575061533d610422925f52609860205260405f2090565b615369615348610424565b4363ffffffff168152925b5f60208501526001600160c01b03166040840152565b615603565b9161539963ffffffff9361539361538d845f52609860205260405f2090565b9161502f565b90611429565b50906153a9825463ffffffff1690565b438516941684036153d457506104229250906001600160401b0382549181199060401b169116179055565b815467ffffffff000000001916602085901b67ffffffff00000000161790915561042292919061536990615410905f52609860205260405f2090565b9161535361541c610424565b63ffffffff9095168552565b9061543392916156ca565b1561543a57565b638baa579f60e01b5f5260045ffd5b906001600160601b03809116911602906001600160601b038216918203612e5057565b61548a6001600160601b039161ffff60206127109501511690615449565b160490565b61548a6001600160601b039161ffff60406127109501511690615449565b9190815f528260205260405f2054925f5b8481106155565760405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a490fd5b808503858111612e505761331561556c9161502f565b61559561558a826155858887905f5260205260405f2090565b611429565b505463ffffffff1690565b63ffffffff808616911611156155ae57506001016154be565b94505050505090565b156155be57565b60405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606490fd5b8054600160401b8110156103d25761562091600182018155611429565b61565e57815160208084015160409485015163ffffffff909316911b67ffffffff00000000161767ffffffffffffffff199190931b16919091179055565b634e487b7160e01b5f525f60045260245ffd5b6005111561088e57565b3d156156a5573d9061568c82610784565b9161569a60405193846103f2565b82523d5f602084013e565b606090565b9081602091031261045b57516001600160e01b03198116810361045b5790565b9190916156d78284615795565b6156e081615671565b15908161577f575b50615777575f9261571561572385946040519283916020830195630b135d3f60e11b8752602484016141f6565b03601f1981018352826103f2565b51915afa61572f61567b565b8161576b575b8161573e575090565b8051630b135d3f60e11b92506001600160e01b031991615766918101602090810191016156aa565b161490565b80516020149150615735565b505050600190565b6001600160a01b0383811691161490505f6156e8565b8151604181036157c15750906157bd91602082015190606060408401519301515f1a90615803565b9091565b6040036157fa5760406020830151920151918260ff1c91601b8301809311612e50576157bd936001600160ff1b03169260ff1690615803565b50505f90600290565b9291907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083116158a15760ff16601b81141580615896575b61588b576020935f93604051938493608085019385528785015260408401526060830152838052039060015afa156109e3575f516001600160a01b0381161561588357905f90565b505f90600190565b505050505f90600490565b50601c81141561583b565b505050505f9060039056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220b99f5cdf10f1afbec60a96a467376aca78e0010a34f26b7ed32db3a82257596d64736f6c634300081b00338b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x02\0\x80`@R4a\x03\xB8W`\xC0\x81a]\x16\x808\x03\x80\x91a\0 \x82\x85a\x03\xBCV[\x839\x81\x01\x03\x12a\x03\xB8W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\xB8W` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\xB8W`@\x83\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\xB8W``\x84\x01Q\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x84\x03a\x03\xB8W`\x80\x85\x01Q\x94`\x01`\x01`\xA0\x1B\x03\x86\x16\x86\x03a\x03\xB8W`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x82\x03a\x03\xB8W`@Qa\0\xB6`@\x82a\x03\xBCV[`\x16\x81R` \x81\x01\x90\x7FAVSRegistryCoordinator\0\0\0\0\0\0\0\0\0\0\x82R`@Q\x91a\0\xF1`@\x84a\x03\xBCV[`\x06\x83R` \x83\x01\x91ev0.0.1`\xD0\x1B\x83RQ\x90 \x91Q\x90 \x81`\xE0R\x80a\x01\0RF`\xA0R`@Q\x90` \x82\x01\x92_Q` a\\\xF6_9_Q\x90_R\x84R`@\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x01X`\xC0\x82a\x03\xBCV[Q\x90 `\x80R0`\xC0R_Q` a\\\xF6_9_Q\x90_Ra\x01 R\x15a\x03\xA9Wa\x01@Ra\x01`Ra\x01\xA0Ra\x01\x80Ra\x01\xC0Ra\x01\xE0R_T`\xFF\x81`\x08\x1C\x16a\x03TW`\xFF\x80\x82\x16\x10a\x03\x1AW[`@QaY\x02\x90\x81a\x03\xF4\x829`\x80Q\x81aP\xBA\x01R`\xA0Q\x81aQq\x01R`\xC0Q\x81aP\x84\x01R`\xE0Q\x81aQ\t\x01Ra\x01\0Q\x81aQ/\x01Ra\x01 Q\x81aP\xE6\x01Ra\x01@Q\x81\x81\x81a\tZ\x01R\x81\x81a\x12\xA8\x01R\x81\x81a\x1C\xD8\x01Ra(r\x01Ra\x01`Q\x81\x81\x81a\x0C\x1B\x01R\x81\x81a\x10P\x01R\x81\x81a\x1B\x0F\x01R\x81\x81a \x16\x01R\x81\x81a#\x82\x01R\x81\x81a&\xEA\x01R\x81\x81a?\xDC\x01R\x81\x81aD\x15\x01RaL\x02\x01Ra\x01\x80Q\x81\x81\x81a\x0B/\x01R\x81\x81a\x0F\x1E\x01R\x81\x81a\x13\xE6\x01R\x81\x81a0\xA1\x01R\x81\x81a>0\x01R\x81\x81aB\xC6\x01R\x81\x81aE\xDE\x01R\x81\x81aJD\x01RaO3\x01Ra\x01\xA0Q\x81\x81\x81a\x0EW\x01R\x81\x81a\x14\xB2\x01R\x81\x81a\x1AI\x01R\x81\x81a0o\x01R\x81\x81a7\x14\x01R\x81\x81a=h\x01R\x81\x81a>\xE8\x01R\x81\x81aC+\x01R\x81\x81aJ\xCD\x01RaM\x93\x01Ra\x01\xC0Q\x81\x81\x81a\x0E\xBD\x01R\x81\x81a \xEE\x01R\x81\x81a+\xAE\x01R\x81\x81a0\xD3\x01R\x81\x81a=\xCF\x01R\x81\x81aC\x8D\x01RaK7\x01Ra\x01\xE0Q\x81a\x14\xF6\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\x01\xA9V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[_\x80\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x03\xDFW`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\xCF*\xB5\x14a\x03\x9EW\x80c\x03\xFD4\x92\x14a\x03\x99W\x80c\x04\xECcQ\x14a\x03\x94W\x80c\x05C\x10\xE6\x14a\x03\x8FW\x80c\x0C\xF4\xB7g\x14a\x03\x8AW\x80c\r?!4\x14a\x03\x85W\x80c\x12^\x05\x84\x14a\x03\x80W\x80c\x13T*N\x14a\x03{W\x80c\x13d9\xDD\x14a\x03vW\x80c\x14x\x85\x1F\x14a\x03qW\x80c\x1E\xB8\x12\xDA\x14a\x03lW\x80c$\x9A\x0CB\x14a\x03gW\x80c(\xF6\x1B1\x14a\x03bW\x80c)k\xB0d\x14a\x03]W\x80c)\xD1\xE0\xC3\x14a\x03XW\x80c,\xDD\x1E\x86\x14a\x03SW\x80c9\x98\xFD\xD3\x14a\x03NW\x80c<*\x7FL\x14a\x03IW\x80c>\xEF:Q\x14a\x03DW\x80cQ@\xA5H\x14a\x03?W\x80cXe\xC6\x0C\x14a\x03:W\x80cY\\jg\x14a\x035W\x80cZ\xC8j\xB7\x14a\x030W\x80c[\x0B\x82\x9F\x14a\x03+W\x80c\\\x97Z\xBB\x14a\x03&W\x80c]\xF4YF\x14a\x03!W\x80ccG\xC9\0\x14a\x03\x1CW\x80ch0H5\x14a\x03\x17W\x80ck:\xA7.\x14a\x03\x12W\x80cn;\x17\xDB\x14a\x03\rW\x80cqP\x18\xA6\x14a\x03\x08W\x80c\x7F\xC3\xF8\x86\x14a\x03\x03W\x80c\x82\x81\xABu\x14a\x02\xFEW\x80c\x84\xCAR\x13\x14a\x02\xF9W\x80c\x87\x1E\xF0I\x14a\x02\xF4W\x80c\x88o\x11\x95\x14a\x02\xEFW\x80c\x8D\xA5\xCB[\x14a\x02\xEAW\x80c\x9A\xA1e=\x14a\x02\xE5W\x80c\x9B]\x17{\x14a\x02\xE0W\x80c\x9D\x8E\x0C#\x14a\x02\xDBW\x80c\x9E\x99#\xC2\x14a\x02\xD6W\x80c\x9F\xEA\xB8Y\x14a\x02\xD1W\x80c\xA4\xD7\x87\x1F\x14a\x02\xCCW\x80c\xA5\x08W\xBF\x14a\x02\xC7W\x80c\xA9ox>\x14a\x02\xC2W\x80c\xAD\xCFs\xF7\x14a\x02\xBDW\x80c\xBD3\xEE$\x14a\x02\xA9W\x80c\xC3\x91B^\x14a\x02\xB8W\x80c\xCA\r\xE8\x82\x14a\x02\xB3W\x80c\xCAO-\x97\x14a\x02\xAEW\x80c\xCA\xBB\xB1\x7F\x14a\x02\xA9W\x80c\xD7-\x8D\xD6\x14a\x02\xA4W\x80c\xE6W\x97\xAD\x14a\x02\x9FW\x80c\xEE1\x88!\x14a\x02\x9AW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x95W\x80c\xFA\xBC\x1C\xBC\x14a\x02\x90Wc\xFD9\x10Z\x14a\x02\x8BW_\x80\xFD[a)+V[a(IV[a'\xB8V[a&\xD0V[a&<V[a&\x1FV[a$9V[a%xV[a%>V[a$\x9AV[a\"\xBEV[a\"\xA1V[a!\x8EV[a!WV[a!\x1DV[a \xD9V[a\x1FIV[a\x1E\x83V[a\x1D/V[a\x1D\x07V[a\x1C\xC3V[a\x1C\x93V[a\x1C7V[a\x19\xB3V[a\x18\xDCV[a\x16,V[a\x15%V[a\x14\xE1V[a\x14\x9DV[a\x14CV[a\x13\xD1V[a\x13\xB4V[a\x13\x1FV[a\x12\xF0V[a\x12}V[a\x12\x11V[a\x11 V[a\r\x8BV[a\x0CJV[a\x0C\x06V[a\x0B\xD9V[a\x0B\xACV[a\n\xF9V[a\n\xD1V[a\n\x9FV[a\n\x17V[a\t\xE8V[a\t*V[a\x08\xEFV[a\x08\xB4V[a\x08\x93V[a\x07\xF3V[a\x07\\V[a\x05\xEFV[a\x05\xB7V[a\x04\xEDV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xD2W`@RV[a\x03\xA3V[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xD2W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xD2W`@RV[`@Q\x90a\x04\"`@\x83a\x03\xF2V[V[`@Q\x90a\x04\"``\x83a\x03\xF2V[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xD2W`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04[WV[_\x80\xFD[`\x045\x90a\x04\"\x82a\x04JV[`$5\x90a\x04\"\x82a\x04JV[`D5\x90a\x04\"\x82a\x04JV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x815a\x04\x9D\x81a\x043V[\x92a\x04\xAB`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04[W` \x01\x90[\x82\x82\x10a\x04\xD3WPPP\x90V[` \x80\x91\x835a\x04\xE2\x81a\x04JV[\x81R\x01\x91\x01\x90a\x04\xC6V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x05\x1D\x906\x90`\x04\x01a\x04\x86V[a\x054a\x05.`\x04\x80`\x01T\x16\x14\x90V[\x15a)qV[_[\x81Q\x81\x10\x15a\x05\xB5W`\x01\x90a\x05\xAF`\x01`\x01`\xA0\x1B\x03a\x05W\x83\x86a)\x94V[Q\x16\x80_R`\x99` R`@_ a\x05\x88`\xFF\x86`@Q\x93a\x05x\x85a\x03\xB7V[\x80T\x85R\x01T\x16` \x83\x01a)\xA8V[a\x05\xA9a\x05\xA4a\x05\x98\x83QaP=V[`\x01`\x01`\xC0\x1B\x03\x16\x90V[a5\xE8V[\x91a6\xCAV[\x01a\x056V[\0[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045_R`\x98` R` `@_ T`@Q\x90\x81R\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x04[WV[4a\x04[W``6`\x03\x19\x01\x12a\x04[W`$5a\x06/a\x06)`\x045a\x06\x15\x84a\x05\xE1V[`D5\x90_R`\x98` R`@_ a\x14)V[Pa*\x92V[c\xFF\xFF\xFF\xFF\x80\x82Q\x16\x92\x16\x91\x82\x10a\x06\xB3W`@\x81a\x06wa\x06\x9F\x94a\x06_` a\x06\x85\x96\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90c\xFF\xFF\xFF\xFF\x82\x16\x15\x91\x82\x15a\x06\xA3W[PPa7\xA0V[\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[c\xFF\xFF\xFF\xFF\x16\x11\x90P_\x80a\x06pV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from after blockN`\x84\x82\x01Rd:\xB6\xB12\xB9`\xD9\x1B`\xA4\x82\x01R`\xC4\x90\xFD[_\x91\x03\x12a\x04[WV[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`\x9DT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xD2W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xAB\x82a\x07\x84V[\x91a\x07\xB9`@Q\x93\x84a\x03\xF2V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x04[W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x81` a\x07\xF0\x935\x91\x01a\x07\x9FV[\x90V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x08#\x906\x90`\x04\x01a\x07\xD5V[3_R`\x99` R`\xFF`\x01`@_ \x01T\x16`\x03\x81\x10\x15a\x08\x8EW`\x01a\x08K\x91\x14a)\xDAV[3_R`\x99` R\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAAa\x08\x89`@_ T\x92`@Q\x91\x82\x91\x82a*\x14V[\x03\x90\xA2\0[a\x11\xE6V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a\x08\xAFa8GV[`\xA0U\0[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a\x08\xD1\x81a\x04JV[`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R` `@_ T`@Q\x90\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a\t\x0C\x81a\x04JV[`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R` `@_ T`@Q\x90\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\t\xE3Wa\x05\xB5\x92a\t\xA0\x91_\x91a\t\xB4W[Pa*HV[a\t\xAF`\x01T\x82\x81\x16\x14a*^V[a8\xD3V[a\t\xD6\x91P` =` \x11a\t\xDCW[a\t\xCE\x81\x83a\x03\xF2V[\x81\x01\x90a*%V[_a\t\x9AV[P=a\t\xC4V[a*=V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045_R`\x9A` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04[W`@6`\x03\x19\x01\x12a\x04[W``a\nOa\x06)`$5`\x045a\n>a*tV[P_R`\x98` R`@_ a\x14)V[`@Q\x90c\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@`\x01\x80`\xC0\x1B\x03\x91\x01Q\x16`@\x82\x01R\xF3[`\x045\x90`\xFF\x82\x16\x82\x03a\x04[WV[5\x90`\xFF\x82\x16\x82\x03a\x04[WV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\xFFa\n\xBAa\n\x81V[\x16_R`\x9B` R` `@_ T`@Q\x90\x81R\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`\x9ET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x805\x90\x82\x01R` \x81`$\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\t\xE3Wa\x06\x9F\x91_\x91a\x0B}W[P`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[a\x0B\x9F\x91P` =` \x11a\x0B\xA5W[a\x0B\x97\x81\x83a\x03\xF2V[\x81\x01\x90a*\xC3V[_a\x0BbV[P=a\x0B\x8DV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[Wa\x05\xB5`\x045a\x0B\xCC\x81a\x04JV[a\x0B\xD4a8GV[a9\x05V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[Wa\x05\xB5`\x045a\x0B\xF9\x81a\x04JV[a\x0C\x01a8GV[a9cV[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`@a\x0Cq`\x045a\x0Cl\x81a\x04JV[a*\xF0V[a\x0C\x87\x82Q\x80\x92` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\xF3[5\x90a\xFF\xFF\x82\x16\x82\x03a\x04[WV[``\x90`\x03\x19\x01\x12a\x04[W`@Q\x90a\x0C\xB1\x82a\x03\xD7V[\x81`\x045a\x0C\xBE\x81a\x05\xE1V[\x81R`$5a\xFF\xFF\x81\x16\x81\x03a\x04[W` \x82\x01R`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x04[W`@\x01RV[`\x01`\x01``\x1B\x03\x81\x16\x03a\x04[WV[\x81`\x1F\x82\x01\x12\x15a\x04[W\x805\x90a\r\x10\x82a\x043V[\x92a\r\x1E`@Q\x94\x85a\x03\xF2V[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x04[W` \x01\x92[\x82\x84\x10a\rHWPPPP\x90V[`@\x84\x83\x03\x12a\x04[W` `@\x91\x82Qa\rb\x81a\x03\xB7V[\x865a\rm\x81a\x04JV[\x81R\x82\x87\x015a\r|\x81a\x0C\xE8V[\x83\x82\x01R\x81R\x01\x93\x01\x92a\r:V[4a\x04[W`\xC06`\x03\x19\x01\x12a\x04[Wa\r\xA56a\x0C\x98V[`d5a\r\xB1\x81a\x0C\xE8V[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\r\xD0\x906\x90`\x04\x01a\x0C\xF9V[\x90`\xA45\x91a\r\xDE\x83a\x05\xE1V[a\r\xE6a8GV[a\r\xF4`\xFF`\xA1T\x16a+VV[`\x96T`\xFF\x16\x93\x84\x90a\x0E/\x90a\x0E\r`\xC0\x84\x10a4tV[a\x0E)a\x0E\x19\x88a;,V[`\xFF\x16`\xFF\x19`\x96T\x16\x17`\x96UV[\x86a@\xFBV[`\xA1T`\xFF\x16\x80a\x10\xC7W[a\x0F\xB9W[Pa\x0EK`\x01a0\x14V[a\x0EU`\x01a0\x14V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[Wa\x0E\xAB\x93_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x06b\xD3\xE1`\xE5\x1B\x85R\x8B`\x04\x86\x01a<\xB0V[\x03\x92Z\xF1\x80\x15a\t\xE3Wa\x0F\xA5W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xE3Wa\x0F\x91W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xE3Wa\x0F}W\0[\x80a\x0F\x8B_a\x05\xB5\x93a\x03\xF2V[\x80a\x07RV[\x80a\x0F\x8B_a\x0F\x9F\x93a\x03\xF2V[_a\x0F\x1BV[\x80a\x0F\x8B_a\x0F\xB3\x93a\x03\xF2V[_a\x0E\xBAV[\x92a\x0F\xC2a;>V[\x92a\x0F\xCD\x83Qa;\x8BV[\x93_[\x84Q\x81\x10\x15a\x10\x19W\x80a\x10\x13a\x0F\xFAa\x0F\xEC`\x01\x94\x89a)\x94V[QQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x10\x04\x83\x8Aa)\x94V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\x0F\xD0V[P\x91\x94\x90\x93a\x105a\x10)a\x04\x13V[c\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[` \x82\x01Ra\x10C\x82a)\x87V[Ra\x10M\x81a)\x87V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x84}cO`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x10\xA4\x90`\x04\x83\x01a;\xB3V[\x03\x92Z\xF1\x80\x15a\t\xE3W\x15a\x0E@W\x80a\x0F\x8B_a\x10\xC1\x93a\x03\xF2V[_a\x0E@V[Pa\x10\xEEa\x10\xEAa\x10\xE3\x87`\xFF\x16_R`\xA2` R`@_ \x90V[T`\xFF\x16\x90V[\x15\x90V[a\x0E;V[\x91\x81`\x1F\x84\x01\x12\x15a\x04[W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x04[W` \x83\x81\x86\x01\x95\x01\x01\x11a\x04[WV[4a\x04[W`@6`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[W6`#\x82\x01\x12\x15a\x04[W\x80`\x04\x015a\x11[\x81a\x043V[\x91a\x11i`@Q\x93\x84a\x03\xF2V[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x04[W`$\x81\x01\x92[\x82\x84\x10a\x11\xB7W`$5\x85`\x01`\x01`@\x1B\x03\x82\x11a\x04[Wa\x11\xB1a\x05\xB5\x926\x90`\x04\x01a\x10\xF3V[\x91a+lV[\x835`\x01`\x01`@\x1B\x03\x81\x11a\x04[W` \x91a\x11\xDB\x83\x92`$6\x91\x87\x01\x01a\x04\x86V[\x81R\x01\x93\x01\x92a\x11\x87V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x11\x15a\x08\x8EWV[\x90`\x03\x82\x10\x15a\x08\x8EWRV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a\x12.\x81a\x04JV[a\x126a*\xD8V[P`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ a\x12^`\xFF`\x01`@Q\x93a\x05x\x85a\x03\xB7V[`@Q\x80\x91a\x06\x9F` `@\x84\x01\x92\x80Q\x85R\x01Q` \x84\x01\x90a\x12\x04V[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\xE3Wa\x12\xE8\x91_\x91a\t\xB4WPa*HV[a\x05\xB5a8\x9FV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W` `\x01`\xFFa\x13\x0Fa\n\x81V[\x16\x1B\x80`\x01T\x16\x14`@Q\x90\x81R\xF3[4a\x04[W`\x806`\x03\x19\x01\x12a\x04[Wa\x138a\n\x81V[``6`#\x19\x01\x12a\x04[W`@Qa\x13P\x81a\x03\xD7V[`$5a\x13\\\x81a\x05\xE1V[\x81R`D5a\xFF\xFF\x81\x16\x81\x03a\x04[W` \x82\x01R`d5a\xFF\xFF\x81\x16\x81\x03a\x04[W`@\x82\x01Ra\x13\x8Ca8GV[`\xFF`\x96T\x16`\xFF\x83\x16\x10\x15a\x13\xA5Wa\x05\xB5\x91a@\xFBV[cs\x10\xCF\xF5`\xE1\x1B_R`\x04_\xFD[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `\x01T`@Q\x90\x81R\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x82\x10\x15a\x14>W_R` _ \x01\x90_\x90V[a\x14\x15V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`\x9CT\x81\x10\x15a\x04[W`\x9C_R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04[W`@6`\x03\x19\x01\x12a\x04[W`\x045a\x15B\x81a\x04JV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x15a\x906\x90`\x04\x01a\x07\xD5V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x03a\x16\x1DW`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 B\x90U`\x99\x90\x91R\x90 \x80T`\x01a\x15\xC9\x81a\x15\xC0a\x15\xBAa\x05\x98a\x15\xB4`\x96T`\xFF\x16\x90V[\x89a@tV[\x94aP=V[\x94\x01T`\xFF\x16\x90V[a\x15\xD2\x81a\x11\xFAV[\x14\x91\x82a\x16\nW[\x82a\x15\xF1W[PPa\x15\xE8W\0[a\x05\xB5\x91aB\rV[\x81\x16`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x91\x16\x14\x90P_\x80a\x15\xE0V[`\x01`\x01`\xC0\x1B\x03\x82\x16\x15\x15\x92Pa\x15\xDAV[cv\xD8\xAB\x17`\xE1\x1B_R`\x04_\xFD[4a\x04[W_6`\x03\x19\x01\x12a\x04[Wa\x16Da8GV[`d\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[\x81`\x1F\x82\x01\x12\x15a\x04[W\x805\x90a\x16\x9E\x82a\x043V[\x92a\x16\xAC`@Q\x94\x85a\x03\xF2V[\x82\x84R` ``\x81\x86\x01\x94\x02\x83\x01\x01\x91\x81\x83\x11a\x04[W` \x01\x92[\x82\x84\x10a\x16\xD6WPPPP\x90V[``\x84\x83\x03\x12a\x04[W` ``\x91`@Qa\x16\xF1\x81a\x03\xD7V[\x865a\x16\xFC\x81a\x05\xE1V[\x81Ra\x17\t\x83\x88\x01a\x0C\x89V[\x83\x82\x01Ra\x17\x19`@\x88\x01a\x0C\x89V[`@\x82\x01R\x81R\x01\x93\x01\x92a\x16\xC8V[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x815a\x17@\x81a\x043V[\x92a\x17N`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04[W` \x01\x90[\x82\x82\x10a\x17vWPPP\x90V[` \x80\x91\x835a\x17\x85\x81a\x0C\xE8V[\x81R\x01\x91\x01\x90a\x17iV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x815a\x17\xA7\x81a\x043V[\x92a\x17\xB5`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x04[W` \x82\x01\x90[\x83\x82\x10a\x17\xE1WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x04[W` \x91a\x18\x03\x87\x84\x80\x94\x88\x01\x01a\x0C\xF9V[\x81R\x01\x91\x01\x90a\x17\xD2V[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x815\x90a\x18&\x82a\x043V[\x92a\x184`@Q\x94\x85a\x03\xF2V[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a\x04[W` \x01\x91[\x81\x83\x10a\x18\\WPPP\x90V[\x825`\x02\x81\x10\x15a\x04[W\x81R` \x92\x83\x01\x92\x01a\x18OV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x815a\x18\x8C\x81a\x043V[\x92a\x18\x9A`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04[W` \x01\x90[\x82\x82\x10a\x18\xC2WPPP\x90V[` \x80\x91\x835a\x18\xD1\x81a\x05\xE1V[\x81R\x01\x91\x01\x90a\x18\xB5V[4a\x04[Wa\x01 6`\x03\x19\x01\x12a\x04[Wa\x18\xF6a\x04_V[a\x18\xFEa\x04lV[\x90a\x19\x07a\x04yV[`d5`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x19)\x906\x90`\x04\x01a\x16\x87V[`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x19H\x906\x90`\x04\x01a\x17)V[\x90`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x19h\x906\x90`\x04\x01a\x17\x90V[\x92`\xE45`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x19\x88\x906\x90`\x04\x01a\x18\x0EV[\x94a\x01\x045\x97`\x01`\x01`@\x1B\x03\x89\x11a\x04[Wa\x19\xADa\x05\xB5\x996\x90`\x04\x01a\x18uV[\x97a.\xDDV[4a\x04[W`\xA06`\x03\x19\x01\x12a\x04[Wa\x19\xCD6a\x0C\x98V[`d5a\x19\xD9\x81a\x0C\xE8V[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x19\xF8\x906\x90`\x04\x01a\x0C\xF9V[\x90a\x1A\x01a8GV[`\x96T`\xFF\x16\x92\x83\x90a\x1A,\x90a\x1A\x1A`\xC0\x84\x10a4tV[a\x1A&a\x0E\x19\x87a;,V[\x85a@\xFBV[`\xA1T`\xFF\x16\x80a\x1B\x86W[a\x1A\x9DW[Pa\x1AG_a0\x14V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x04[Wa\x0E\xAB\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c:\xEA\x0B\x9D`\xE1\x1B\x84R\x8A`\x04\x85\x01a<\xE5V[\x91\x90\x92a\x1A\xA8a;>V[\x93a\x1A\xB3\x83Qa;\x8BV[\x94_[\x84Q\x81\x10\x15a\x1A\xE2W\x80a\x1A\xDCa\x1A\xD2a\x0F\xEC`\x01\x94\x89a)\x94V[a\x10\x04\x83\x8Ba)\x94V[\x01a\x1A\xB6V[P\x91\x94\x93\x90\x92\x93a\x1A\xF4a\x10)a\x04\x13V[` \x82\x01Ra\x1B\x02\x82a)\x87V[Ra\x1B\x0C\x81a)\x87V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x84}cO`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x1Bc\x90`\x04\x83\x01a;\xB3V[\x03\x92Z\xF1\x80\x15a\t\xE3W\x15a\x1A=W\x80a\x0F\x8B_a\x1B\x80\x93a\x03\xF2V[_a\x1A=V[Pa\x1B\xA2a\x10\xEAa\x10\xE3\x86`\xFF\x16_R`\xA2` R`@_ \x90V[a\x1A8V[\x81`\x1F\x82\x01\x12\x15a\x04[W\x805\x90a\x1B\xBE\x82a\x043V[\x92a\x1B\xCC`@Q\x94\x85a\x03\xF2V[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x04[W` \x01\x92[\x82\x84\x10a\x1B\xF6WPPPP\x90V[`@\x84\x83\x03\x12a\x04[W` `@\x91\x82Qa\x1C\x10\x81a\x03\xB7V[a\x1C\x19\x87a\n\x91V[\x81R\x82\x87\x015a\x1C(\x81a\x04JV[\x83\x82\x01R\x81R\x01\x93\x01\x92a\x1B\xE8V[4a\x04[W`\xA06`\x03\x19\x01\x12a\x04[W`\x045a\x1CT\x81a\x04JV[`$5\x90`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04[W` \x92a\x1C~a\x1C\x8B\x936\x90`\x04\x01a\x1B\xA7V[`d5\x91`\x845\x93a1\x97V[`@Q\x90\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W` a\x1C\xB1`\x045aP=V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`dT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `\xFF`\x96T\x16`@Q\x90\x81R\xF3[\x91\x90\x82`@\x91\x03\x12a\x04[W`@Qa\x1Dg\x81a\x03\xB7V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W`@Q\x91a\x1D\x92`@\x84a\x03\xF2V[\x82\x90`@\x81\x01\x92\x83\x11a\x04[W\x90[\x82\x82\x10a\x1D\xAEWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x1D\xA1V[\x90a\x01\0`C\x19\x83\x01\x12a\x04[W`@Q\x91a\x1D\xD9\x83a\x03\xD7V[\x82a\x1D\xE5\x82`Da\x1DOV[\x81Ra\x1D\xF2\x82`\x84a\x1DOV[` \x82\x01R`\x80`\xC3\x19\x83\x01\x12a\x04[W`@\x90a\x1E*\x82Q\x93a\x1E\x15\x85a\x03\xB7V[a\x1E \x81`\xC4a\x1DwV[\x85Ra\x01\x04a\x1DwV[` \x84\x01R\x01RV[\x91\x90\x91``\x81\x84\x03\x12a\x04[W`@Q\x90a\x1EM\x82a\x03\xD7V[\x81\x93\x815\x91`\x01`\x01`@\x1B\x03\x83\x11a\x04[Wa\x1Ep`@\x93\x92\x84\x93\x83\x01a\x07\xD5V[\x84R` \x81\x015` \x85\x01R\x015\x91\x01RV[4a\x04[Wa\x01\xA06`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x1E\xB4\x906\x90`\x04\x01a\x10\xF3V[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x1E\xD4\x906\x90`\x04\x01a\x07\xD5V[a\x1E\xDD6a\x1D\xBEV[a\x01D5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x1E\xFD\x906\x90`\x04\x01a\x1B\xA7V[\x90a\x01d5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x1F\x1E\x906\x90`\x04\x01a\x1E3V[\x92a\x01\x845\x95`\x01`\x01`@\x1B\x03\x87\x11a\x04[Wa\x1FCa\x05\xB5\x976\x90`\x04\x01a\x1E3V[\x95a2RV[4a\x04[W`@6`\x03\x19\x01\x12a\x04[W`\x045a\x1Ff\x81a\x04JV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x1F\x85\x906\x90`\x04\x01a\x18uV[\x90a\x1F\x96a\x05.`\x01\x80\x80T\x16\x14\x90V[a\x1F\xAAa\x1F\xA5`\xFF`\xA1T\x16\x90V[a+VV[_[\x82Q\x81\x10\x15a \0W\x80a\x1F\xFAa\x1F\xF5a\x10\xEAa\x10\xE3a\x1F\xE4a\x1F\xDEa\x1F\xD4`\x01\x98\x8Ba)\x94V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`\xFF\x16\x90V[`\xFF\x16_R`\xA2` R`@_ \x90V[a4\x05V[\x01a\x1F\xACV[P`@Qc\xCA\x8A\xA7\xC7`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\xE3Wa b\x91_\x91a \xBAW[P`\x01`\x01`\xA0\x1B\x03\x163\x14a4\x1BV[a l\x82Qa41V[_[\x83Q\x81\x10\x15a \xB0W\x80a \x9Da \x8Da\x1F\xDEa\x1F\xD4`\x01\x95\x89a)\x94V[`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[_\x1Aa \xA9\x82\x85a4cV[S\x01a nV[Pa\x05\xB5\x91aB\rV[a \xD3\x91P` =` \x11a\x0B\xA5Wa\x0B\x97\x81\x83a\x03\xF2V[_a QV[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `@Q\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\xFFa!ra\n\x81V[\x16_R`\xA2` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04[Wa\x01`6`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa!\xBF\x906\x90`\x04\x01a\x07\xD5V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa!\xDE\x906\x90`\x04\x01a\x07\xD5V[a!\xE76a\x1D\xBEV[a\x01D5\x91`\x01`\x01`@\x1B\x03\x83\x11a\x04[W\x83a\"5a\"\x0Fa\";\x956\x90`\x04\x01a\x1E3V[\x93a\" a\x05.`\x01\x80\x80T\x16\x14\x90V[a\"/`\xFF`\xA1T\x16\x15a3\xBDV[3aE\xBCV[3aI<V[Q\x90_[\x81Q\x81\x10\x15a\x05\xB5W\x80a\"\x9Ba\"X`\x01\x93\x85a4cV[Q`\xF8\x1Cc\xFF\xFF\xFF\xFFa\"\x92\x81a\"\x87\x81a\"s\x88\x8Ca)\x94V[Q\x16\x94`\xFF\x16_R`\x97` R`@_ \x90V[T\x16c\xFF\xFF\xFF\xFF\x16\x90V[\x91\x16\x11\x15a4tV[\x01a\"?V[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `\xA0T`@Q\x90\x81R\xF3[4a\x04[W``6`\x03\x19\x01\x12a\x04[W`\x045a\"\xDB\x81a\x04JV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\"\xFA\x906\x90`\x04\x01a\x18uV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa#\x1A\x906\x90`\x04\x01a\x07\xD5V[\x90a#+a\x05.`\x01\x80\x80T\x16\x14\x90V[a#:a\x1F\xA5`\xFF`\xA1T\x16\x90V[_[\x83Q\x81\x10\x15a#jW\x80a#da\x1F\xF5a\x10\xEAa\x10\xE3a\x1F\xE4a\x1F\xDEa\x1F\xD4`\x01\x98\x8Ca)\x94V[\x01a#<V[P`@Qc\xCA\x8A\xA7\xC7`\xE0\x1B\x81R\x90\x91` \x82`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\xE3Wa#\xD5a#\xE4\x92a#\xED\x94_\x91a \xBAWP`\x01`\x01`\xA0\x1B\x03\x163\x14a4\x1BV[` \x80\x82Q\x83\x01\x01\x91\x01a4\xF9V[\x83\x92\x91\x92aE\xBCV[\x90a#\xF8\x84Qa41V[\x91_[\x85Q\x81\x10\x15a$-W\x80a$\x1Aa \x8Da\x1F\xDEa\x1F\xD4`\x01\x95\x8Ba)\x94V[_\x1Aa$&\x82\x87a4cV[S\x01a#\xFBV[P\x91a\x05\xB5\x92\x84aN]V[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `\xFF`\xA1T\x16`@Q\x90\x15\x15\x81R\xF3[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a$~WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a$qV[4a\x04[W`@6`\x03\x19\x01\x12a\x04[W`\x045a$\xB7\x81a\x05\xE1V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04[W6`#\x83\x01\x12\x15a\x04[W\x81`\x04\x015\x91a$\xE3\x83a\x043V[\x92a$\xF1`@Q\x94\x85a\x03\xF2V[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x04[W`$\x01\x90[\x82\x82\x10a%.Wa\x06\x9Fa%\"\x86\x86aO\xB9V[`@Q\x91\x82\x91\x82a$[V[\x815\x81R` \x91\x82\x01\x91\x01a%\x0EV[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `@Q\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa%\xA8\x906\x90`\x04\x01a\x07\xD5V[a%\xB9a\x05.`\x02\x80`\x01T\x16\x14\x90V[_`\xFF`\xA1T\x16\x15\x80\x15\x91[\x83Q\x81\x10\x15a&\x15W\x80a%\xDB`\x01\x92\x86a4cV[Q`\xF8\x1C\x83\x85a%\xF6W[a%\xF0\x91Pa3\xBDV[\x01a%\xC5V[P_R`\xA2` Ra%\xF0a&\x10`@_ `\xFF\x90T\x16\x90V[a%\xE6V[a\x05\xB5\x843aB\rV[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `\x9CT`@Q\x90\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\xFFa&Wa\n\x81V[a&_a*tV[P\x16_R`\x97` Ra\x06\x9F`@_ a\xFF\xFF`@Q\x91a&\x7F\x83a\x03\xD7V[Tc\xFF\xFF\xFF\xFF\x81\x16\x83R\x81\x81` \x1C\x16` \x84\x01R`0\x1C\x16`@\x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91`@a\xFF\xFF\x81``\x84\x01\x95c\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[4a\x04[W_6`\x03\x19\x01\x12a\x04[Wa&\xE8a8GV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x0F%\xF1a`\xE4\x1B\x81R0`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xE3Wa'\xA4W[P_[a'Wa\x1F\xDE`\x96T`\xFF\x16\x90V[`\xFF\x82\x16\x10\x15a'\x92W`\x01\x81a'\x8Ba'~`\xFF\x94`\xFF\x16_R`\xA2` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[\x01\x16a'HV[a\x05\xB5`\x01`\xFF\x19`\xA1T\x16\x17`\xA1UV[\x80a\x0F\x8B_a'\xB2\x93a\x03\xF2V[_a'EV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a'\xD5\x81a\x04JV[a'\xDDa8GV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a'\xF5Wa\x05\xB5\x90aD\xC3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\xE3W_\x91a)\x0CW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a(\xFDWa(\xCB`\x01T\x19\x82\x19\x81\x16\x14a*^V[\x80`\x01U`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a)%\x91P` =` \x11a\x0B\xA5Wa\x0B\x97\x81\x83a\x03\xF2V[_a(\xAAV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a)H\x81a\x04JV[`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R` `\xFF`\x01`@_ \x01T\x16a\x0C\x87`@Q\x80\x92a\x12\x04V[\x15a)xWV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x80Q\x15a\x14>W` \x01\x90V[\x80Q\x82\x10\x15a\x14>W` \x91`\x05\x1B\x01\x01\x90V[`\x03\x82\x10\x15a\x08\x8EWRV[\x90a\x04\"`@Qa)\xC4\x81a\x03\xB7V[` `\xFF`\x01\x83\x96\x80T\x85R\x01T\x16\x91\x01a)\xA8V[\x15a)\xE1WV[c\xAB\xA4s9`\xE0\x1B_R`\x04_\xFD[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x07\xF0\x92\x81\x81R\x01\x90a)\xF0V[\x90\x81` \x91\x03\x12a\x04[WQ\x80\x15\x15\x81\x03a\x04[W\x90V[`@Q=_\x82>=\x90\xFD[\x15a*OWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a*eWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[`@Q\x90a*\x81\x82a\x03\xD7V[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x90`@Qa*\x9F\x81a\x03\xD7V[`@\x81\x93Tc\xFF\xFF\xFF\xFF\x81\x16\x83Rc\xFF\xFF\xFF\xFF\x81` \x1C\x16` \x84\x01R\x81\x1C\x91\x01RV[\x90\x81` \x91\x03\x12a\x04[WQa\x07\xF0\x81a\x04JV[`@Q\x90a*\xE5\x82a\x03\xB7V[_` \x83\x82\x81R\x01RV[a+Qa\x07\xF0\x91a*\xFFa*\xD8V[P`@\x80Q\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6` \x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x81\x83\x01R\x90\x81Ra+I``\x82a\x03\xF2V[Q\x90 a9\xC1V[a:\x0EV[\x15a+]WV[c[w\x90\x19`\xE0\x1B_R`\x04_\xFD[\x90\x92\x91a+\x80a\x05.`\x04\x80`\x01T\x16\x14\x90V[a+\x9Fa+\x8F`\x96T`\xFF\x16\x90V[a+\x9A6\x84\x88a\x07\x9FV[a@tV[Pa+\xAC\x81\x83Q\x14a-\xB5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x93_[\x82\x81\x10a+\xE8WPPPP\x90PV[a,\ra,\x07a+\xF9\x83\x86\x86a-\xCBV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[\x92a,\x18\x82\x86a)\x94V[Q\x80Q`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x87\x16`\x04\x82\x01R\x91\x97\x91\x90` \x82`$\x81\x8DZ\xFA\x91\x82\x15a\t\xE3Wa,^\x92c\xFF\xFF\xFF\xFF\x91_\x91a-\x87W[P\x16\x14a-\xECV[_\x97\x88[\x88Q\x8A\x10\x15a-\x1BW`\x01\x90a-\x13a,\x8Ba,~\x8D\x8Da)\x94V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91a,\xEEa,\xB1a,\xAC\x85`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[a)\xB4V[\x91a,\xD9a,\xD4\x8Da,\xC6a\x05\x98\x87QaP=V[`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a.\x02V[\x85\x80`\xA0\x1B\x03\x16\x85\x80`\xA0\x1B\x03\x85\x16\x11a.\x18V[a-\x0Ca-\x05a,\xFD\x8Aa.BV[\x8A\x8A\x8Da.bV[6\x91a\x07\x9FV[\x90\x83a6\xCAV[\x99\x01\x98a,bV[P\x96P\x96P\x92\x90`\x01\x91\x94\x92\x94Ca->\x82`\xFF\x16_R`\x9B` R`@_ \x90V[U\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4`\xFF`@Q\x92\x16\x91\x80a-xC\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA2\x01\x94\x93\x94\x92\x90\x92a+\xD9V[a-\xA8\x91P` =\x81\x11a-\xAEW[a-\xA0\x81\x83a\x03\xF2V[\x81\x01\x90a-\xD7V[_a,VV[P=a-\x96V[\x15a-\xBCWV[c\xAA\xAD\x13\xF7`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a\x14>W\x01\x90V[\x90\x81` \x91\x03\x12a\x04[WQa\x07\xF0\x81a\x05\xE1V[\x15a-\xF3WV[c\x8EZ\xEE\xE7`\xE0\x1B_R`\x04_\xFD[\x15a.\tWV[c\xD0S\xAA!`\xE0\x1B_R`\x04_\xFD[\x15a.\x1FWV[c\xBAP\xF9\x11`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a.PWV[a..V[\x91\x90\x82\x01\x80\x92\x11a.PWV[\x90\x93\x92\x93\x84\x83\x11a\x04[W\x84\x11a\x04[W\x81\x01\x92\x03\x90V[\x15a.\x81WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[\x97\x95\x93\x91a/'\x97\x95\x93\x91_T\x99a/\r`\xFF\x8C`\x08\x1C\x16\x15\x15\x15\x80\x9C\x81a/\xA1W[\x81\x15a/\x81W[Pa.zV[\x8Aa/\x1E`\x01`\xFF\x19_T\x16\x17_UV[a/jWa0+V[a/-WV[a/;a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1V[a/|a\x01\0a\xFF\0\x19_T\x16\x17_UV[a0+V[0;\x15\x91P\x81a/\x93W[P_a/\x07V[`\xFF\x16`\x01\x14\x90P_a/\x8CV[`\x01`\xFF\x82\x16\x10\x91Pa/\0V[`\x9CT`\x01`@\x1B\x81\x10\x15a\x03\xD2W`\x01\x81\x01`\x9CU`\x9CT\x81\x10\x15a\x14>W`\x9C_R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x02\x11\x15a\x08\x8EWV[Q`\x02\x81\x10\x15a\x08\x8EW\x90V[\x92a\t\xAFa\x0C\x01\x92a\x0B\xD4a0j\x96\x9C\x9B\x9A\x99\x98\x9C\x8D\x89Q\x90Q\x80\x91\x14\x90\x81a1\x8CW[P\x80a1\x81W[\x80a1vW[a0e\x90a-\xB5V[aD\xC3V[a0\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a/\xAFV[a0\xCE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a/\xAFV[a1\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a/\xAFV[_[\x81Q\x81\x10\x15a1mW\x80a1ga1\x1B`\x01\x93\x85a)\x94V[Qa16a1)\x84\x8Ca)\x94V[Q`\x01`\x01``\x1B\x03\x16\x90V[a1@\x84\x88a)\x94V[Qa1Sa1N\x86\x8Ba)\x94V[a0\x1EV[\x91a1aa\x1F\xD4\x87\x8Da)\x94V[\x93a=\rV[\x01a1\x02V[PPPPP\x90PV[P\x8AQ\x8CQ\x14a0\\V[P\x89Q\x8BQ\x14a0VV[\x90P\x8AQ\x14_a0OV[\x91\x94\x93\x90\x92`@Q\x92` \x84\x01\x94`\xE0\x85\x01\x91\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87R`\x01\x80`\xA0\x1B\x03\x16`@\x86\x01R``\x85\x01R`\xC0`\x80\x85\x01R\x86Q\x80\x91R` a\x01\0\x85\x01\x97\x01\x90_[\x81\x81\x10a2$WPPPa\x07\xF0\x94\x95a+I\x92\x84\x92`\xA0\x84\x01R`\xC0\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xF2V[\x82Q\x80Q`\xFF\x16\x8AR` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x8B\x01R`@\x90\x99\x01\x98\x90\x92\x01\x91`\x01\x01a1\xF7V[\x94a2\x9Ea2\x95a2\xB0\x95\x98\x94\x93\x96\x98a2ra\x05.`\x01\x80\x80T\x16\x14\x90V[a2\x89a2\x84a\x10\xEA`\xFF`\xA1T\x16\x90V[a3\xBDV[a\"/\x88\x8BQ\x14a-\xB5V[\x91\x88\x833aF\xBEV[a2\xA96\x86\x88a\x07\x9FV[\x903aI<V[\x92_[\x82\x81\x10a2\xC1WPPPPPV[\x80a2\xF1a2\xECa2\xDBa,\x07a+\xF9`\x01\x96\x89\x8Ba-\xCBV[`\xFF\x16_R`\x97` R`@_ \x90V[a3\xD3V[a2\xFFa\x1F\xD4\x83\x89Qa)\x94V[c\xFF\xFF\xFF\xFFa3\x1Ea3\x15\x84Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[\x91\x16\x11a3-W[P\x01a2\xB3V[a3y\x90a3Ba,\x07a+\xF9\x85\x89\x8Ba-\xCBV[a3Sa1)\x85`@\x8C\x01Qa)\x94V[\x90a3ea1)\x86` \x8D\x01Qa)\x94V[\x90a3p\x86\x89a)\x94V[Q\x923\x91aM\x07V[a3\xB7a3\x9A` a3\x8B\x84\x87a)\x94V[Q\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a3\xB1a-\x05a3\xA9\x85a.BV[\x85\x89\x8Ba.bV[\x90aB\rV[_a3&V[\x15a3\xC4WV[c\x0B\x880o`\xE0\x1B_R`\x04_\xFD[\x90`@Qa3\xE0\x81a\x03\xD7V[`@a\xFF\xFF\x82\x94Tc\xFF\xFF\xFF\xFF\x81\x16\x84R\x81\x81` \x1C\x16` \x85\x01R`0\x1C\x16\x91\x01RV[\x15a4\x0CWV[c\xFD,\x1FM`\xE0\x1B_R`\x04_\xFD[\x15a4\"WV[c#\xD8q\xA5`\xE0\x1B_R`\x04_\xFD[\x90a4;\x82a\x07\x84V[a4H`@Q\x91\x82a\x03\xF2V[\x82\x81R\x80\x92a4Y`\x1F\x19\x91a\x07\x84V[\x01\x90` 6\x91\x017V[\x90\x81Q\x81\x10\x15a\x14>W\x01` \x01\x90V[\x15a4{WV[c<\xB8\x9C\x97`\xE0\x1B_R`\x04_\xFD[\x91\x90\x82`@\x91\x03\x12a\x04[W`@Qa4\xA2\x81a\x03\xB7V[` \x80\x82\x94\x80Q\x84R\x01Q\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W`@Q\x91a4\xCD`@\x84a\x03\xF2V[\x82\x90`@\x81\x01\x92\x83\x11a\x04[W\x90[\x82\x82\x10a4\xE9WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a4\xDCV[\x91\x90\x91\x80\x83\x03\x90a\x01 \x82\x12a\x04[W\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x04[W\x81\x01\x84`\x1F\x82\x01\x12\x15a\x04[W\x80Qa51\x81a\x07\x84V[\x91a5?`@Q\x93\x84a\x03\xF2V[\x81\x83R\x86` \x83\x83\x01\x01\x11a\x04[W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x93a\x01\0`\x1F\x19\x84\x01\x12a\x04[W`\x80`@Q\x93a5{\x85a\x03\xD7V[a5\x88\x83` \x86\x01a4\x8AV[\x85Ra5\x97\x83``\x86\x01a4\x8AV[` \x86\x01R`\x9F\x19\x01\x12a\x04[Wa5\xCD\x90`\xE0`@Q\x93a5\xB8\x85a\x03\xB7V[a5\xC5\x83`\xA0\x83\x01a4\xB2V[\x85R\x01a4\xB2V[` \x82\x01R`@\x82\x01R\x90V[_\x19\x81\x14a.PW`\x01\x01\x90V[_\x81\x80[a6bWPa5\xFE\x90a\xFF\xFF\x16a41V[__[\x82Q\x82\x10\x80a6WW[\x15a6PW`\x01\x81\x1B\x84\x16a6)W[a6$\x90a5\xDAV[a6\x01V[\x90`\x01a6$\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa6F\x82\x87a4cV[S\x01\x91\x90Pa6\x1BV[PP\x90P\x90V[Pa\x01\0\x81\x10a6\x0BV[_\x19\x81\x01\x81\x81\x11a.PWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a.PW`\x01\x01\x90\x80a5\xECV[\x90\x81` \x91\x03\x12a\x04[WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x04[W\x90V[a\x07\xF0\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a)\xF0V[\x91\x90`\x01` \x82\x01Qa6\xDC\x81a\x11\xFAV[a6\xE5\x81a\x11\xFAV[\x03a7\x9BWQ`@Qc3V\x7F\x7F`\xE1\x1B\x81R\x91` \x91\x83\x91\x82\x91a7\x0F\x91\x90\x87`\x04\x85\x01a6\xA6V[\x03\x81_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x90\x81\x15a\t\xE3W_\x91a7lW[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81a7`WPPV[a3\xB1a\x04\"\x92a5\xE8V[a7\x8E\x91P` =` \x11a7\x94W[a7\x86\x81\x83a\x03\xF2V[\x81\x01\x90a6\x87V[_a7LV[P=a7|V[PPPV[\x15a7\xA7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from before block`\x84\x82\x01Re':\xB6\xB12\xB9`\xD1\x1B`\xA4\x82\x01R`\xC4\x90\xFD[`dT`\x01`\x01`\xA0\x1B\x03\x163\x03a8[WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[_\x19`\x01U`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`\x01U`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`\x9DUV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`\x9EUV[a9\xC9aP\x81V[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra9\xF4`b\x82a\x03\xF2V[Q\x90 \x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_Q` aX\xAD_9_Q\x90_R\x90a:%a*\xD8V[P_\x91\x90\x06` `\xC0\x83[a;%W_\x93_Q` aX\xAD_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa:[\x85\x82a\x03\xF2V[\x846\x827\x84\x81\x85`@Qa:o\x82\x82a\x03\xF2V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aX\xAD_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a;*Wa:\xD9\x90aU\xB7V[Q\x91a;%W_Q` aX\xAD_9_Q\x90_R\x82\x80\t\x14a;\x10WP_Q` aX\xAD_9_Q\x90_R`\x01_\x94\x08\x92\x93a:0V[\x92\x93PPa;\x1Ca\x04\x13V[\x92\x83R\x82\x01R\x90V[a9\xFAV[\xFE[`\xFF`\x01\x91\x16\x01\x90`\xFF\x82\x11a.PWV[`@\x80Q\x90\x91\x90a;O\x83\x82a\x03\xF2V[`\x01\x81R\x91`\x1F\x19\x01\x82_[\x82\x81\x10a;gWPPPV[` \x90`@Qa;v\x81a\x03\xB7V[_\x81R``\x83\x82\x01R\x82\x82\x85\x01\x01R\x01a;[V[\x90a;\x95\x82a\x043V[a;\xA2`@Q\x91\x82a\x03\xF2V[\x82\x81R\x80\x92a4Y`\x1F\x19\x91a\x043V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a;\xE5WPPPPP\x90V[\x90\x91\x92\x93\x94`?\x19\x82\x82\x03\x01\x83R\x85Q` ``\x81`@\x85\x01\x93c\xFF\xFF\xFF\xFF\x81Q\x16\x86R\x01Q\x93`@\x83\x82\x01R\x84Q\x80\x94R\x01\x92\x01\x90_\x90[\x80\x82\x10a<=WPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a;\xD6V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x90\x91\x01\x90a<\x1EV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a<|WPPP\x90V[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x86\x01R`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a<oV[\x90a\x07\xF0\x94\x93`\x01`\x01``\x1B\x03`\x80\x94`\xFFc\xFF\xFF\xFF\xFF\x94\x16\x85R\x16` \x84\x01R\x16`@\x82\x01R\x81``\x82\x01R\x01\x90a<_V[`\x01`\x01``\x1B\x03a\x07\xF0\x94\x93`\xFF``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a<_V[\x93\x90\x91\x92a=\x1D`\x96T`\xFF\x16\x90V[\x94a=D`\xFF\x87\x16\x91a=2`\xC0\x84\x10a4tV[a=>a\x0E\x19\x89a;,V[\x87a@\xFBV[`\xA1T`\xFF\x16\x80a@SW[a?fW[Pa=_\x81a0\x14V[\x80a>\xC9WPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x04[Wa=\xBC\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c:\xEA\x0B\x9D`\xE1\x1B\x84R\x8A`\x04\x85\x01a<\xE5V[\x03\x92Z\xF1\x80\x15a\t\xE3Wa>\xB5W[P[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xE3Wa>\xA1W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x90\x92\x16`\x04\x83\x01R_\x90\x82\x90\x81\x83\x81`$\x81\x01[\x03\x92Z\xF1\x80\x15a\t\xE3Wa>\x93WPV[\x80a\x0F\x8B_a\x04\"\x93a\x03\xF2V[\x80a\x0F\x8B_a>\xAF\x93a\x03\xF2V[_a>-V[\x80a\x0F\x8B_a>\xC3\x93a\x03\xF2V[_a=\xCBV[\x80a>\xD8`\x01\x92\x95\x93\x95a0\x14V[\x14a>\xE6W[PPPa=\xCDV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[Wa?<\x93_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x06b\xD3\xE1`\xE5\x1B\x85R\x8B`\x04\x86\x01a<\xB0V[\x03\x92Z\xF1\x80\x15a\t\xE3Wa?RW[\x80\x80a>\xDEV[\x80a\x0F\x8B_a?`\x93a\x03\xF2V[_a?KV[\x95\x92\x90\x94\x91a?sa;>V[\x95a?~\x86Qa;\x8BV[\x96_[\x87Q\x81\x10\x15a?\xADW\x80a?\xA7a?\x9Da\x0F\xEC`\x01\x94\x8Ca)\x94V[a\x10\x04\x83\x8Da)\x94V[\x01a?\x81V[P\x91\x93\x96\x97\x90\x92\x94\x97a?\xC1a\x10)a\x04\x13V[` \x82\x01Ra?\xCF\x82a)\x87V[Ra?\xD9\x81a)\x87V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x84}cO`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a@0\x90`\x04\x83\x01a;\xB3V[\x03\x92Z\xF1\x80\x15a\t\xE3W\x15a=UW\x80a\x0F\x8B_a@M\x93a\x03\xF2V[_a=UV[Pa@oa\x10\xEAa\x10\xE3\x88`\xFF\x16_R`\xA2` R`@_ \x90V[a=PV[\x90`\x01a@\x82`\xFF\x93aR\x19V[\x92\x83\x92\x16\x1B\x11\x15a@\x90W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x90\xFD[aA\xB9`\xFF\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x92\x16\x92\x83_R`\x97` R`@_ aAQc\xFF\xFF\xFF\xFF\x83Q\x16\x82\x90c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[` \x82\x01Q\x81Te\xFF\xFF\0\0\0\0g\xFF\xFF\0\0\0\0\0\0`@\x86\x01Q`0\x1B\x16\x92` \x1B\x16\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x17\x17\x90U`@Q\x91\x82\x91\x82\x91\x90\x91`@a\xFF\xFF\x81``\x84\x01\x95c\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[\x03\x90\xA2V[\x15aA\xC5WV[ch\xB6\xA8u`\xE1\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra\x07\xF0\x92\x91\x01\x90a)\xF0V[`@\x90a\x07\xF0\x93\x92\x81R\x81` \x82\x01R\x01\x90a)\xF0V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x99` R`@\x90 \x90`\x01\x82T\x92\x01aBJ`\x01aB;\x83T`\xFF\x16\x90V[aBD\x81a\x11\xFAV[\x14a)\xDAV[aB\xAAaBea\x05\x98aB_`\x96T`\xFF\x16\x90V[\x87a@tV[aBn\x85aP=V[`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x90aB\x85\x82\x15\x15aA\xBEV[aB\x9B\x82\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x83\x14a.\x02V[\x90\x19\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[aB\xB4\x81\x85aS\x0EV[`\x01`\x01`\xC0\x1B\x03\x16\x15aD\x08W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W\x83_\x91aC\x19\x93\x83`@Q\x80\x96\x81\x95\x82\x94c\xF4\xE2O\xE5`\xE0\x1B\x84R`\x04\x84\x01aA\xD4V[\x03\x92Z\xF1\x80\x15a\t\xE3WaC\xF4W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W_`@Q\x80\x92c\xBD)\xB8\xCD`\xE0\x1B\x82R\x81\x83\x81aC{\x89\x89`\x04\x84\x01aA\xF6V[\x03\x92Z\xF1\x80\x15a\t\xE3WaC\xE0W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x04[Wa>\x82\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xBD)\xB8\xCD`\xE0\x1B\x84R`\x04\x84\x01aA\xF6V[\x80a\x0F\x8B_aC\xEE\x93a\x03\xF2V[_aC\x8AV[\x80a\x0F\x8B_aD\x02\x93a\x03\xF2V[_aC(V[\x80T`\xFF\x19\x16`\x02\x17\x90U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xE3WaD\xAFW[P\x81`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4_\x80\xA3_aB\xC3V[\x80a\x0F\x8B_aD\xBD\x93a\x03\xF2V[_aDyV[`d\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x90\x81` \x91\x03\x12a\x04[WQ\x90V[\x90_\x90[`\x02\x82\x10aE+WPPPV[` \x80`\x01\x92\x85Q\x81R\x01\x93\x01\x91\x01\x90\x91aE\x1EV[a\x01 \x90aE\xAB` `@a\x04\"\x96\x98\x97\x95\x98a\x01`\x85\x01\x99`\x01\x80`\xA0\x1B\x03\x16\x85RaE{\x83\x86\x01\x82Q` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\x80\x83\x01Q\x80Q``\x87\x01R` \x01Q`\x80\x86\x01R\x01QaE\x9F`\xA0\x85\x01\x82QaE\x1AV[\x01Q`\xE0\x83\x01\x90aE\x1AV[\x01\x90` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x90\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x83`$\x81\x84Z\xFA\x92\x83\x15a\t\xE3W_\x93aF\x9DW[P\x82\x15aF#WPP\x90P\x90V[` \x92PaFS\x93_aF5\x84a*\xF0V[`@Qc\x17\xEF9\xCB`\xE3\x1B\x81R\x96\x87\x95\x86\x94\x85\x93\x91`\x04\x85\x01aEAV[\x03\x92Z\xF1\x90\x81\x15a\t\xE3W_\x91aFnW[P\x80_\x80a6PV[aF\x90\x91P` =` \x11aF\x96W[aF\x88\x81\x83a\x03\xF2V[\x81\x01\x90aE\x0BV[_aFeV[P=aF~V[aF\xB7\x91\x93P` =` \x11aF\x96WaF\x88\x81\x83a\x03\xF2V[\x91_aF\x15V[\x91\x92\x90` \x82\x01\x92\x83Q_R`\x9A` R`\xFF`@_ T\x16aG9W`@\x83\x01\x80QB\x11aG*Wa\x04\"\x95aG\"\x93\x86Q_R`\x9A` RaG\x0C`@_ `\x01`\xFF\x19\x82T\x16\x17\x90UV[`\x9DT`\x01`\x01`\xA0\x1B\x03\x16\x96Q\x92Q\x93a1\x97V[\x90Q\x91aT(V[c\x08\x19\xBD\xCD`\xE0\x1B_R`\x04_\xFD[co\xBE\xFE\xC3`\xE1\x1B_R`\x04_\xFD[`@Q\x90aGU\x82a\x03\xD7V[```@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x15aGnWV[c\x13\xCAFW`\xE0\x1B_R`\x04_\xFD[\x15aG\x84WV[c\x0Ch\x16\xCD`\xE0\x1B_R`\x04_\xFD[\x15aG\x9AWV[c\x19hg}`\xE1\x1B_R`\x04_\xFD[`\x01` \x91\x83Q\x81U\x01\x91\x01Q`\x03\x81\x10\x15a\x08\x8EW`\xFF\x80\x19\x83T\x16\x91\x16\x17\x90UV[\x90`\x01\x80`\xA0\x1B\x03\x16\x81R`@` \x82\x01R`\x80`@aG\xF8\x84Q``\x83\x86\x01R`\xA0\x85\x01\x90a)\xF0V[\x93` \x81\x01Q``\x85\x01R\x01Q\x91\x01R\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x81QaH\"\x81a\x043V[\x92aH0`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04[W` \x01\x90[\x82\x82\x10aHXWPPP\x90V[` \x80\x91\x83QaHg\x81a\x0C\xE8V[\x81R\x01\x91\x01\x90aHKV[\x91\x90\x91`@\x81\x84\x03\x12a\x04[W\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x04[W\x83aH\x9B\x91\x83\x01aH\x0BV[\x92` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x07\xF0\x92\x01aH\x0BV[` \x81\x83\x03\x12a\x04[W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04[W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x81QaH\xEC\x81a\x043V[\x92aH\xFA`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04[W` \x01\x90[\x82\x82\x10aI\"WPPP\x90V[` \x80\x91\x83QaI1\x81a\x05\xE1V[\x81R\x01\x91\x01\x90aI\x15V[\x90\x91\x92\x93\x82\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAAaJ\x07aImaGHV[\x97aI\xFBaI\x89a\x05\x98aI\x83`\x96T`\xFF\x16\x90V[\x8Ba@tV[aI\x92\x86aP=V[`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x90aI\xA9\x82\x15\x15aGgV[`\x01\x80`\xC0\x1B\x03\x16aI\xC3aI\xBE\x82\x84\x16\x15\x90V[aG}V[`\x01`\x01`\xA0\x1B\x03\x89\x16_\x90\x81R`\x9F` R`@\x90 aI\xF4\x90aI\xED\x90[T`\xA0T\x90a.UV[B\x11aG\x93V[\x17\x85aS\x0EV[`@Q\x91\x82\x91\x82a*\x14V[\x03\x90\xA2`\x01aJ2\x81aJ*\x85`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[\x01T`\xFF\x16\x90V[aJ;\x81a\x11\xFAV[\x03aK\xCFW[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W_`@Q\x80\x92c\x1F\xD9<\xA9`\xE1\x1B\x82R\x81\x83\x81aJ\x94\x8A\x89`\x04\x84\x01aA\xD4V[\x03\x92Z\xF1\x80\x15a\t\xE3W\x84\x92_\x92\x85\x92aK\xBBW[PaJ\xC8`@Q\x96\x87\x93\x84\x93c%PGw`\xE0\x1B\x85R`\x04\x85\x01a6\xA6V[\x03\x81\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x91\x82\x15a\t\xE3WaK2\x93_\x93\x84\x91\x85\x91aK\x97W[P`@\x87\x01R` \x86\x01R`@Q\x93\x84\x92\x83\x92b\xBF\xF0M`\xE0\x1B\x84R`\x04\x84\x01aA\xF6V[\x03\x81\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x90\x81\x15a\t\xE3W_\x91aKuW[P\x81R\x90V[aK\x91\x91P=\x80_\x83>aK\x89\x81\x83a\x03\xF2V[\x81\x01\x90aH\xB9V[_aKoV[\x90PaK\xB5\x91P=\x80\x86\x83>aK\xAD\x81\x83a\x03\xF2V[\x81\x01\x90aHrV[_aK\rV[\x80a\x0F\x8B\x85aK\xC9\x93a\x03\xF2V[_aJ\xA9V[aL\0aK\xDAa\x04\x13V[\x84\x81R`\x01` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\x99` R`@\x90 aG\xA9V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x99&\xEE}`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90aLW\x90\x89`\x04\x84\x01aG\xCDV[\x03\x92Z\xF1\x80\x15a\t\xE3WaL\x9CW[P\x81`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE_\x80\xA3_aJAV[\x80a\x0F\x8B_aL\xAA\x93a\x03\xF2V[_aLfV[\x15aL\xB7WV[cV\x16\x8BA`\xE1\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x04[WQa\x07\xF0\x81a\x0C\xE8V[\x15aL\xE2WV[cLD\x99]`\xE0\x1B_R`\x04_\xFD[\x15aL\xF8WV[c\xB1\x87\xE8i`\xE0\x1B_R`\x04_\xFD[` \x91\x92aMdaMWaM\x8F\x98\x96\x97aMPaM,\x87\x83\x01Q`\x01\x80`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x99` R`@\x90 T\x96\x90\x91\x16\x14\x15aL\xB0V[Q`\xFF\x16\x90V[`\xFF\x80\x85\x16\x91\x16\x14a-\xECV[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x90\x91\x16`$\x82\x01R\x94\x85\x90\x81\x90`D\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\t\xE3Wa\x04\"\x94_\x94aN\x13W[P\x82aN\x0B\x92aN\x06aM\xF2\x93`\x01`\x01``\x1B\x03aM\xFEaM\xF2\x82\x99\x8BaTlV[`\x01`\x01``\x1B\x03\x16\x90V[\x91\x16\x11aL\xDBV[aT\x8FV[\x91\x16\x10aL\xF1V[aM\xF2\x91\x94P\x92aN\x0B\x92aN\x06aNJ`\x01`\x01``\x1B\x03\x96` =` \x11aNVW[aNB\x81\x83a\x03\xF2V[\x81\x01\x90aL\xC6V[\x96\x93PP\x92P\x92aM\xCFV[P=aN8V[\x81\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAAaN\xFFaN\x8AaGHV[\x96aI\xFBaN\xA6a\x05\x98aN\xA0`\x96T`\xFF\x16\x90V[\x8Aa@tV[aN\xAF\x86aP=V[`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x90aN\xC6\x82\x15\x15aGgV[`\x01\x80`\xC0\x1B\x03\x16aN\xDBaI\xBE\x82\x84\x16\x15\x90V[`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9F` R`@\x90 aI\xF4\x90aI\xED\x90aI\xE3V[\x03\x90\xA2`\x01aO\"\x81aJ*\x84`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[aO+\x81a\x11\xFAV[\x03aO\x83W[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W_`@Q\x80\x92c\x1F\xD9<\xA9`\xE1\x1B\x82R\x81\x83\x81aJ\x94\x8A\x89`\x04\x84\x01aA\xD4V[aO\xB4aO\x8Ea\x04\x13V[\x83\x81R`\x01` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x99` R`@\x90 aG\xA9V[aO1V[\x91\x90\x80Q\x90aO\xC7\x82a\x043V[\x91aO\xD5`@Q\x93\x84a\x03\xF2V[\x80\x83RaO\xE4`\x1F\x19\x91a\x043V[\x016` \x84\x017_[\x81Q\x81\x10\x15aP(W\x80aP\x0FaP\x06`\x01\x93\x85a)\x94V[Q\x87`\x98aT\xADV[c\xFF\xFF\xFF\xFFaP\x1E\x83\x87a)\x94V[\x91\x16\x90R\x01aO\xEDV[P\x90\x92PPV[_\x19\x81\x01\x91\x90\x82\x11a.PWV[\x80_R`\x98` R`@_ T\x90\x81\x15_\x14aPYWPP_\x90V[_R`\x98` R`@_ \x90_\x19\x81\x01\x90\x81\x11a.PWaPy\x91a\x14)V[PT`@\x1C\x90V[0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80aQnW[\x15aP\xDCW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra9\xF4`\xC0\x82a\x03\xF2V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14aP\xB3V[\x15aQ\x9EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90a\x01\0\x82Q\x11aR\x97W\x81Q\x15aR\x92WaRUaRKa,\x07aR=\x85a)\x87V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xFF`\x01\x91\x16\x1B\x90V[`\x01\x90[\x83Q\x82\x10\x15aR\x8DW`\x01\x90aRxaRKa,\x07aR=\x86\x89a4cV[\x90aR\x84\x81\x83\x11aQ\x97V[\x17\x91\x01\x90aRYV[\x92PPV[_\x91PV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R\xFD[\x90aS!\x82_R`\x98` R`@_ \x90V[T\x80aSnWPaS=a\x04\"\x92_R`\x98` R`@_ \x90V[aSiaSHa\x04$V[Cc\xFF\xFF\xFF\xFF\x16\x81R\x92[_` \x85\x01R`\x01`\x01`\xC0\x1B\x03\x16`@\x84\x01RV[aV\x03V[\x91aS\x99c\xFF\xFF\xFF\xFF\x93aS\x93aS\x8D\x84_R`\x98` R`@_ \x90V[\x91aP/V[\x90a\x14)V[P\x90aS\xA9\x82Tc\xFF\xFF\xFF\xFF\x16\x90V[C\x85\x16\x94\x16\x84\x03aS\xD4WPa\x04\"\x92P\x90`\x01`\x01`@\x1B\x03\x82T\x91\x81\x19\x90`@\x1B\x16\x91\x16\x17\x90UV[\x81Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x85\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x91Ua\x04\"\x92\x91\x90aSi\x90aT\x10\x90_R`\x98` R`@_ \x90V[\x91aSSaT\x1Ca\x04$V[c\xFF\xFF\xFF\xFF\x90\x95\x16\x85RV[\x90aT3\x92\x91aV\xCAV[\x15aT:WV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a.PWV[aT\x8A`\x01`\x01``\x1B\x03\x91a\xFF\xFF` a'\x10\x95\x01Q\x16\x90aTIV[\x16\x04\x90V[aT\x8A`\x01`\x01``\x1B\x03\x91a\xFF\xFF`@a'\x10\x95\x01Q\x16\x90aTIV[\x91\x90\x81_R\x82` R`@_ T\x92_[\x84\x81\x10aUVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[\x80\x85\x03\x85\x81\x11a.PWa3\x15aUl\x91aP/V[aU\x95aU\x8A\x82aU\x85\x88\x87\x90_R` R`@_ \x90V[a\x14)V[PTc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x11\x15aU\xAEWP`\x01\x01aT\xBEV[\x94PPPPP\x90V[\x15aU\xBEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x03\xD2WaV \x91`\x01\x82\x01\x81Ua\x14)V[aV^W\x81Q` \x80\x84\x01Q`@\x94\x85\x01Qc\xFF\xFF\xFF\xFF\x90\x93\x16\x91\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x90\x93\x1B\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[`\x05\x11\x15a\x08\x8EWV[=\x15aV\xA5W=\x90aV\x8C\x82a\x07\x84V[\x91aV\x9A`@Q\x93\x84a\x03\xF2V[\x82R=_` \x84\x01>V[``\x90V[\x90\x81` \x91\x03\x12a\x04[WQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x04[W\x90V[\x91\x90\x91aV\xD7\x82\x84aW\x95V[aV\xE0\x81aVqV[\x15\x90\x81aW\x7FW[PaWwW_\x92aW\x15aW#\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01aA\xF6V[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xF2V[Q\x91Z\xFAaW/aV{V[\x81aWkW[\x81aW>WP\x90V[\x80Qc\x0B\x13]?`\xE1\x1B\x92P`\x01`\x01`\xE0\x1B\x03\x19\x91aWf\x91\x81\x01` \x90\x81\x01\x91\x01aV\xAAV[\x16\x14\x90V[\x80Q` \x14\x91PaW5V[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_aV\xE8V[\x81Q`A\x81\x03aW\xC1WP\x90aW\xBD\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aX\x03V[\x90\x91V[`@\x03aW\xFAW`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a.PWaW\xBD\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90aX\x03V[PP_\x90`\x02\x90V[\x92\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11aX\xA1W`\xFF\x16`\x1B\x81\x14\x15\x80aX\x96W[aX\x8BW` \x93_\x93`@Q\x93\x84\x93`\x80\x85\x01\x93\x85R\x87\x85\x01R`@\x84\x01R``\x83\x01R\x83\x80R\x03\x90`\x01Z\xFA\x15a\t\xE3W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aX\x83W\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x81\x14\x15aX;V[PPPP_\x90`\x03\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xB9\x9F\\\xDF\x10\xF1\xAF\xBE\xC6\n\x96\xA4g7j\xCAx\xE0\x01\n4\xF2k~\xD3-\xB3\xA8\"WYmdsolcC\0\x08\x1B\x003\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8062cf2ab51461039e57806303fd34921461039957806304ec635114610394578063054310e61461038f5780630cf4b7671461038a5780630d3f213414610385578063125e05841461038057806313542a4e1461037b578063136439dd146103765780631478851f146103715780631eb812da1461036c578063249a0c421461036757806328f61b3114610362578063296bb0641461035d57806329d1e0c3146103585780632cdd1e86146103535780633998fdd31461034e5780633c2a7f4c146103495780633eef3a51146103445780635140a5481461033f5780635865c60c1461033a578063595c6a67146103355780635ac86ab7146103305780635b0b829f1461032b5780635c975abb146103265780635df45946146103215780636347c9001461031c57806368304835146103175780636b3aa72e146103125780636e3b17db1461030d578063715018a6146103085780637fc3f886146103035780638281ab75146102fe57806384ca5213146102f9578063871ef049146102f4578063886f1195146102ef5780638da5cb5b146102ea5780639aa1653d146102e55780639b5d177b146102e05780639d8e0c23146102db5780639e9923c2146102d65780639feab859146102d1578063a4d7871f146102cc578063a50857bf146102c7578063a96f783e146102c2578063adcf73f7146102bd578063bd33ee24146102a9578063c391425e146102b8578063ca0de882146102b3578063ca4f2d97146102ae578063cabbb17f146102a9578063d72d8dd6146102a4578063e65797ad1461029f578063ee3188211461029a578063f2fde38b14610295578063fabc1cbc146102905763fd39105a1461028b575f80fd5b61292b565b612849565b6127b8565b6126d0565b61263c565b61261f565b612439565b612578565b61253e565b61249a565b6122be565b6122a1565b61218e565b612157565b61211d565b6120d9565b611f49565b611e83565b611d2f565b611d07565b611cc3565b611c93565b611c37565b6119b3565b6118dc565b61162c565b611525565b6114e1565b61149d565b611443565b6113d1565b6113b4565b61131f565b6112f0565b61127d565b611211565b611120565b610d8b565b610c4a565b610c06565b610bd9565b610bac565b610af9565b610ad1565b610a9f565b610a17565b6109e8565b61092a565b6108ef565b6108b4565b610893565b6107f3565b61075c565b6105ef565b6105b7565b6104ed565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176103d257604052565b6103a3565b606081019081106001600160401b038211176103d257604052565b90601f801991011681019081106001600160401b038211176103d257604052565b604051906104226040836103f2565b565b604051906104226060836103f2565b6001600160401b0381116103d25760051b60200190565b6001600160a01b0381160361045b57565b5f80fd5b600435906104228261044a565b602435906104228261044a565b604435906104228261044a565b9080601f8301121561045b57813561049d81610433565b926104ab60405194856103f2565b81845260208085019260051b82010192831161045b57602001905b8282106104d35750505090565b6020809183356104e28161044a565b8152019101906104c6565b3461045b57602036600319011261045b576004356001600160401b03811161045b5761051d903690600401610486565b61053461052e600480600154161490565b15612971565b5f5b81518110156105b5576001906105af6001600160a01b036105578386612994565b5116805f52609960205260405f2061058860ff8660405193610578856103b7565b80548552015416602083016129a8565b6105a96105a4610598835161503d565b6001600160c01b031690565b6135e8565b916136ca565b01610536565b005b3461045b57602036600319011261045b576004355f526098602052602060405f2054604051908152f35b63ffffffff81160361045b57565b3461045b57606036600319011261045b5760243561062f610629600435610615846105e1565b604435905f52609860205260405f20611429565b50612a92565b63ffffffff8082511692169182106106b35760408161067761069f9461065f602061068596015163ffffffff1690565b9063ffffffff8216159182156106a3575b50506137a0565b01516001600160c01b031690565b6040516001600160c01b0390911681529081906020820190565b0390f35b63ffffffff161190505f80610670565b60405162461bcd60e51b815260206004820152606560248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d61704174426c6f636b4e756d6265724279496e6465783a2071756f72756d4260648201527f69746d61705570646174652069732066726f6d20616674657220626c6f636b4e6084820152643ab6b132b960d91b60a482015260c490fd5b5f91031261045b57565b3461045b575f36600319011261045b57609d546040516001600160a01b039091168152602090f35b6001600160401b0381116103d257601f01601f191660200190565b9291926107ab82610784565b916107b960405193846103f2565b82948184528183011161045b578281602093845f960137010152565b9080601f8301121561045b578160206107f09335910161079f565b90565b3461045b57602036600319011261045b576004356001600160401b03811161045b576108239036906004016107d5565b335f52609960205260ff600160405f20015416600381101561088e57600161084b91146129da565b335f5260996020527fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa61088960405f20549260405191829182612a14565b0390a2005b6111e6565b3461045b57602036600319011261045b576004356108af613847565b60a055005b3461045b57602036600319011261045b576004356108d18161044a565b60018060a01b03165f52609f602052602060405f2054604051908152f35b3461045b57602036600319011261045b5760043561090c8161044a565b60018060a01b03165f526099602052602060405f2054604051908152f35b3461045b57602036600319011261045b5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156109e3576105b5926109a0915f916109b4575b50612a48565b6109af60015482811614612a5e565b6138d3565b6109d6915060203d6020116109dc575b6109ce81836103f2565b810190612a25565b5f61099a565b503d6109c4565b612a3d565b3461045b57602036600319011261045b576004355f52609a602052602060ff60405f2054166040519015158152f35b3461045b57604036600319011261045b576060610a4f610629602435600435610a3e612a74565b505f52609860205260405f20611429565b6040519063ffffffff815116825263ffffffff6020820151166020830152604060018060c01b03910151166040820152f35b6004359060ff8216820361045b57565b359060ff8216820361045b57565b3461045b57602036600319011261045b5760ff610aba610a81565b165f52609b602052602060405f2054604051908152f35b3461045b575f36600319011261045b57609e546040516001600160a01b039091168152602090f35b3461045b57602036600319011261045b576040516308f6629d60e31b815260048035908201526020816024816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa80156109e35761069f915f91610b7d575b506040516001600160a01b0390911681529081906020820190565b610b9f915060203d602011610ba5575b610b9781836103f2565b810190612ac3565b5f610b62565b503d610b8d565b3461045b57602036600319011261045b576105b5600435610bcc8161044a565b610bd4613847565b613905565b3461045b57602036600319011261045b576105b5600435610bf98161044a565b610c01613847565b613963565b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461045b57602036600319011261045b576040610c71600435610c6c8161044a565b612af0565b610c878251809260208091805184520151910152565bf35b359061ffff8216820361045b57565b606090600319011261045b5760405190610cb1826103d7565b81600435610cbe816105e1565b815260243561ffff8116810361045b5760208201526044359061ffff8216820361045b5760400152565b6001600160601b0381160361045b57565b81601f8201121561045b57803590610d1082610433565b92610d1e60405194856103f2565b82845260208085019360061b8301019181831161045b57602001925b828410610d48575050505090565b60408483031261045b5760206040918251610d62816103b7565b8635610d6d8161044a565b815282870135610d7c81610ce8565b83820152815201930192610d3a565b3461045b5760c036600319011261045b57610da536610c98565b606435610db181610ce8565b6084356001600160401b03811161045b57610dd0903690600401610cf9565b9060a43591610dde836105e1565b610de6613847565b610df460ff60a15416612b56565b60965460ff16938490610e2f90610e0d60c08410613474565b610e29610e1988613b2c565b60ff1660ff196096541617609655565b866140fb565b60a15460ff16806110c7575b610fb9575b50610e4b6001613014565b610e556001613014565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b57610eab935f809460405196879586948593630662d3e160e51b85528b60048601613cb0565b03925af180156109e357610fa5575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163136ca0f960e11b815260ff83166004820152905f908290602490829084905af180156109e357610f91575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163136ca0f960e11b815260ff83166004820152905f908290602490829084905af180156109e357610f7d57005b80610f8b5f6105b5936103f2565b80610752565b80610f8b5f610f9f936103f2565b5f610f1b565b80610f8b5f610fb3936103f2565b5f610eba565b92610fc2613b3e565b92610fcd8351613b8b565b935f5b84518110156110195780611013610ffa610fec60019489612994565b51516001600160a01b031690565b611004838a612994565b6001600160a01b039091169052565b01610fd0565b5091949093611035611029610413565b63ffffffff9093168352565b602082015261104382612987565b5261104d81612987565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163847d634f60e01b8152915f9183918290849082906110a49060048301613bb3565b03925af180156109e35715610e405780610f8b5f6110c1936103f2565b5f610e40565b506110ee6110ea6110e38760ff165f5260a260205260405f2090565b5460ff1690565b1590565b610e3b565b9181601f8401121561045b578235916001600160401b03831161045b576020838186019501011161045b57565b3461045b57604036600319011261045b576004356001600160401b03811161045b573660238201121561045b57806004013561115b81610433565b9161116960405193846103f2565b8183526024602084019260051b8201019036821161045b5760248101925b8284106111b757602435856001600160401b03821161045b576111b16105b59236906004016110f3565b91612b6c565b83356001600160401b03811161045b576020916111db839260243691870101610486565b815201930192611187565b634e487b7160e01b5f52602160045260245ffd5b6003111561088e57565b90600382101561088e5752565b3461045b57602036600319011261045b5760043561122e8161044a565b611236612ad8565b5060018060a01b03165f52609960205260405f2061125e60ff600160405193610578856103b7565b604051809161069f602060408401928051855201516020840190611204565b3461045b575f36600319011261045b5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156109e3576112e8915f916109b45750612a48565b6105b561389f565b3461045b57602036600319011261045b576020600160ff61130f610a81565b161b806001541614604051908152f35b3461045b57608036600319011261045b57611338610a81565b606036602319011261045b57604051611350816103d7565b60243561135c816105e1565b815260443561ffff8116810361045b57602082015260643561ffff8116810361045b57604082015261138c613847565b60ff6096541660ff831610156113a5576105b5916140fb565b637310cff560e11b5f5260045ffd5b3461045b575f36600319011261045b576020600154604051908152f35b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b634e487b7160e01b5f52603260045260245ffd5b805482101561143e575f5260205f2001905f90565b611415565b3461045b57602036600319011261045b57600435609c5481101561045b57609c5f527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c01546040516001600160a01b039091168152602090f35b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461045b57604036600319011261045b576004356115428161044a565b6024356001600160401b03811161045b576115619036906004016107d5565b609e546001600160a01b0316330361161d576001600160a01b0382165f908152609f6020908152604080832042905560999091529020805460016115c9816115c06115ba6105986115b460965460ff1690565b89614074565b9461503d565b94015460ff1690565b6115d2816111fa565b14918261160a575b826115f1575b50506115e857005b6105b59161420d565b81166001600160c01b0390811691161490505f806115e0565b6001600160c01b038216151592506115da565b6376d8ab1760e11b5f5260045ffd5b3461045b575f36600319011261045b57611644613847565b606480546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b81601f8201121561045b5780359061169e82610433565b926116ac60405194856103f2565b8284526020606081860194028301019181831161045b57602001925b8284106116d6575050505090565b60608483031261045b5760206060916040516116f1816103d7565b86356116fc816105e1565b8152611709838801610c89565b8382015261171960408801610c89565b60408201528152019301926116c8565b9080601f8301121561045b57813561174081610433565b9261174e60405194856103f2565b81845260208085019260051b82010192831161045b57602001905b8282106117765750505090565b60208091833561178581610ce8565b815201910190611769565b9080601f8301121561045b5781356117a781610433565b926117b560405194856103f2565b81845260208085019260051b8201019183831161045b5760208201905b8382106117e157505050505090565b81356001600160401b03811161045b5760209161180387848094880101610cf9565b8152019101906117d2565b9080601f8301121561045b5781359061182682610433565b9261183460405194856103f2565b82845260208085019360051b82010191821161045b57602001915b81831061185c5750505090565b8235600281101561045b5781526020928301920161184f565b9080601f8301121561045b57813561188c81610433565b9261189a60405194856103f2565b81845260208085019260051b82010192831161045b57602001905b8282106118c25750505090565b6020809183356118d1816105e1565b8152019101906118b5565b3461045b5761012036600319011261045b576118f661045f565b6118fe61046c565b90611907610479565b6064356084356001600160401b03811161045b57611929903690600401611687565b60a4356001600160401b03811161045b57611948903690600401611729565b9060c4356001600160401b03811161045b57611968903690600401611790565b9260e4356001600160401b03811161045b5761198890369060040161180e565b9461010435976001600160401b03891161045b576119ad6105b5993690600401611875565b97612edd565b3461045b5760a036600319011261045b576119cd36610c98565b6064356119d981610ce8565b6084356001600160401b03811161045b576119f8903690600401610cf9565b90611a01613847565b60965460ff16928390611a2c90611a1a60c08410613474565b611a26610e1987613b2c565b856140fb565b60a15460ff1680611b86575b611a9d575b50611a475f613014565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561045b57610eab925f9283604051809681958294633aea0b9d60e11b84528a60048501613ce5565b919092611aa8613b3e565b93611ab38351613b8b565b945f5b8451811015611ae25780611adc611ad2610fec60019489612994565b611004838b612994565b01611ab6565b50919493909293611af4611029610413565b6020820152611b0282612987565b52611b0c81612987565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163847d634f60e01b8152915f918391829084908290611b639060048301613bb3565b03925af180156109e35715611a3d5780610f8b5f611b80936103f2565b5f611a3d565b50611ba26110ea6110e38660ff165f5260a260205260405f2090565b611a38565b81601f8201121561045b57803590611bbe82610433565b92611bcc60405194856103f2565b82845260208085019360061b8301019181831161045b57602001925b828410611bf6575050505090565b60408483031261045b5760206040918251611c10816103b7565b611c1987610a91565b815282870135611c288161044a565b83820152815201930192611be8565b3461045b5760a036600319011261045b57600435611c548161044a565b60243590604435906001600160401b03821161045b57602092611c7e611c8b933690600401611ba7565b6064359160843593613197565b604051908152f35b3461045b57602036600319011261045b576020611cb160043561503d565b6040516001600160c01b039091168152f35b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461045b575f36600319011261045b576064546040516001600160a01b039091168152602090f35b3461045b575f36600319011261045b57602060ff60965416604051908152f35b919082604091031261045b57604051611d67816103b7565b6020808294803584520135910152565b9080601f8301121561045b5760405191611d926040846103f2565b82906040810192831161045b57905b828210611dae5750505090565b8135815260209182019101611da1565b9061010060431983011261045b5760405191611dd9836103d7565b82611de5826044611d4f565b8152611df2826084611d4f565b6020820152608060c31983011261045b57604090611e2a825193611e15856103b7565b611e208160c4611d77565b8552610104611d77565b60208401520152565b91909160608184031261045b5760405190611e4d826103d7565b81938135916001600160401b03831161045b57611e7060409392849383016107d5565b8452602081013560208501520135910152565b3461045b576101a036600319011261045b576004356001600160401b03811161045b57611eb49036906004016110f3565b906024356001600160401b03811161045b57611ed49036906004016107d5565b611edd36611dbe565b610144356001600160401b03811161045b57611efd903690600401611ba7565b90610164356001600160401b03811161045b57611f1e903690600401611e33565b9261018435956001600160401b03871161045b57611f436105b5973690600401611e33565b95613252565b3461045b57604036600319011261045b57600435611f668161044a565b6024356001600160401b03811161045b57611f85903690600401611875565b90611f9661052e6001808054161490565b611faa611fa560ff60a1541690565b612b56565b5f5b82518110156120005780611ffa611ff56110ea6110e3611fe4611fde611fd46001988b612994565b5163ffffffff1690565b60ff1690565b60ff165f5260a260205260405f2090565b613405565b01611fac565b5060405163ca8aa7c760e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156109e357612062915f916120ba575b506001600160a01b0316331461341b565b61206c8251613431565b5f5b83518110156120b0578061209d61208d611fde611fd460019589612994565b60f81b6001600160f81b03191690565b5f1a6120a98285613463565b530161206e565b506105b59161420d565b6120d3915060203d602011610ba557610b9781836103f2565b5f612051565b3461045b575f36600319011261045b576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461045b575f36600319011261045b5760206040517f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de68152f35b3461045b57602036600319011261045b5760ff612172610a81565b165f5260a2602052602060ff60405f2054166040519015158152f35b3461045b5761016036600319011261045b576004356001600160401b03811161045b576121bf9036906004016107d5565b6024356001600160401b03811161045b576121de9036906004016107d5565b6121e736611dbe565b61014435916001600160401b03831161045b578361223561220f61223b953690600401611e33565b9361222061052e6001808054161490565b61222f60ff60a15416156133bd565b336145bc565b3361493c565b51905f5b81518110156105b5578061229b61225860019385613463565b5160f81c63ffffffff6122928161228781612273888c612994565b51169460ff165f52609760205260405f2090565b541663ffffffff1690565b91161115613474565b0161223f565b3461045b575f36600319011261045b57602060a054604051908152f35b3461045b57606036600319011261045b576004356122db8161044a565b6024356001600160401b03811161045b576122fa903690600401611875565b906044356001600160401b03811161045b5761231a9036906004016107d5565b9061232b61052e6001808054161490565b61233a611fa560ff60a1541690565b5f5b835181101561236a5780612364611ff56110ea6110e3611fe4611fde611fd46001988c612994565b0161233c565b5060405163ca8aa7c760e01b815290916020826004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156109e3576123d56123e4926123ed945f916120ba57506001600160a01b0316331461341b565b602080825183010191016134f9565b839291926145bc565b906123f88451613431565b915f5b855181101561242d578061241a61208d611fde611fd46001958b612994565b5f1a6124268287613463565b53016123fb565b50916105b59284614e5d565b3461045b575f36600319011261045b57602060ff60a154166040519015158152f35b60206040818301928281528451809452019201905f5b81811061247e5750505090565b825163ffffffff16845260209384019390920191600101612471565b3461045b57604036600319011261045b576004356124b7816105e1565b602435906001600160401b03821161045b573660238301121561045b578160040135916124e383610433565b926124f160405194856103f2565b8084526024602085019160051b8301019136831161045b57602401905b82821061252e5761069f6125228686614fb9565b6040519182918261245b565b813581526020918201910161250e565b3461045b575f36600319011261045b5760206040517f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a8152f35b3461045b57602036600319011261045b576004356001600160401b03811161045b576125a89036906004016107d5565b6125b961052e600280600154161490565b5f60ff60a15416158015915b835181101561261557806125db60019286613463565b5160f81c83856125f6575b6125f091506133bd565b016125c5565b505f5260a26020526125f061261060405f2060ff90541690565b6125e6565b6105b5843361420d565b3461045b575f36600319011261045b576020609c54604051908152f35b3461045b57602036600319011261045b5760ff612657610a81565b61265f612a74565b50165f52609760205261069f60405f2061ffff6040519161267f836103d7565b5463ffffffff81168352818160201c16602084015260301c16604082015260405191829182919091604061ffff81606084019563ffffffff8151168552826020820151166020860152015116910152565b3461045b575f36600319011261045b576126e8613847565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b57604051630f25f16160e41b8152306004820152905f908290602490829084905af180156109e3576127a4575b505f5b612757611fde60965460ff1690565b60ff821610156127925760018161278b61277e60ff9460ff165f5260a260205260405f2090565b805460ff19166001179055565b0116612748565b6105b5600160ff1960a154161760a155565b80610f8b5f6127b2936103f2565b5f612745565b3461045b57602036600319011261045b576004356127d58161044a565b6127dd613847565b6001600160a01b038116156127f5576105b5906144c3565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461045b57602036600319011261045b5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156109e3575f9161290c575b506001600160a01b031633036128fd576128cb600154198219811614612a5e565b806001556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b612925915060203d602011610ba557610b9781836103f2565b5f6128aa565b3461045b57602036600319011261045b576004356129488161044a565b60018060a01b03165f526099602052602060ff600160405f20015416610c876040518092611204565b1561297857565b63840a48d560e01b5f5260045ffd5b80511561143e5760200190565b805182101561143e5760209160051b010190565b600382101561088e5752565b906104226040516129c4816103b7565b602060ff600183968054855201541691016129a8565b156129e157565b63aba4733960e01b5f5260045ffd5b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060206107f09281815201906129f0565b9081602091031261045b5751801515810361045b5790565b6040513d5f823e3d90fd5b15612a4f57565b631d77d47760e21b5f5260045ffd5b15612a6557565b63c61dca5d60e01b5f5260045ffd5b60405190612a81826103d7565b5f6040838281528260208201520152565b90604051612a9f816103d7565b604081935463ffffffff8116835263ffffffff8160201c166020840152811c910152565b9081602091031261045b57516107f08161044a565b60405190612ae5826103b7565b5f6020838281520152565b612b516107f091612aff612ad8565b50604080517f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de6602082019081526001600160a01b0390931681830152908152612b496060826103f2565b5190206139c1565b613a0e565b15612b5d57565b635b77901960e01b5f5260045ffd5b909291612b8061052e600480600154161490565b612b9f612b8f60965460ff1690565b612b9a36848861079f565b614074565b50612bac81835114612db5565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316935f5b828110612be857505050509050565b612c0d612c07612bf9838686612dcb565b356001600160f81b03191690565b60f81c90565b92612c188286612994565b5180516040516379a0849160e11b815260ff87166004820152919791906020826024818d5afa9182156109e357612c5e9263ffffffff915f91612d87575b501614612dec565b5f97885b88518a1015612d1b57600190612d13612c8b612c7e8d8d612994565b516001600160a01b031690565b91612cee612cb1612cac8560018060a01b03165f52609960205260405f2090565b6129b4565b91612cd9612cd48d612cc6610598875161503d565b60ff600192161c1660011490565b612e02565b858060a01b0316858060a01b03851611612e18565b612d0c612d05612cfd8a612e42565b8a8a8d612e62565b369161079f565b90836136ca565b990198612c62565b5096509650929060019194929443612d3e8260ff165f52609b60205260405f2090565b557f46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db460ff60405192169180612d7843829190602083019252565b0390a201949394929092612bd9565b612da8915060203d8111612dae575b612da081836103f2565b810190612dd7565b5f612c56565b503d612d96565b15612dbc57565b63aaad13f760e01b5f5260045ffd5b9082101561143e570190565b9081602091031261045b57516107f0816105e1565b15612df357565b638e5aeee760e01b5f5260045ffd5b15612e0957565b63d053aa2160e01b5f5260045ffd5b15612e1f57565b63ba50f91160e01b5f5260045ffd5b634e487b7160e01b5f52601160045260245ffd5b9060018201809211612e5057565b612e2e565b91908201809211612e5057565b9093929384831161045b57841161045b578101920390565b15612e8157565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b97959391612f27979593915f5499612f0d60ff8c60081c16151515809c81612fa1575b8115612f81575b50612e7a565b8a612f1e600160ff195f5416175f55565b612f6a5761302b565b612f2d57565b612f3b61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1565b612f7c61010061ff00195f5416175f55565b61302b565b303b15915081612f93575b505f612f07565b60ff1660011490505f612f8c565b600160ff8216109150612f00565b609c54600160401b8110156103d25760018101609c55609c5481101561143e57609c5f527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c0180546001600160a01b0319166001600160a01b03909216919091179055565b6002111561088e57565b51600281101561088e5790565b926109af610c0192610bd461306a969c9b9a99989c8d89519051809114908161318c575b5080613181575b80613176575b61306590612db5565b6144c3565b61309c7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612faf565b6130ce7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612faf565b6131007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612faf565b5f5b815181101561316d578061316761311b60019385612994565b51613136613129848c612994565b516001600160601b031690565b6131408488612994565b5161315361314e868b612994565b61301e565b91613161611fd4878d612994565b93613d0d565b01613102565b50505050509050565b508a518c511461305c565b5089518b5114613056565b90508a51145f61304f565b919493909260405192602084019460e08501917f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a875260018060a01b03166040860152606085015260c060808501528651809152602061010085019701905f5b818110613224575050506107f09495612b4992849260a084015260c083015203601f1981018352826103f2565b8251805160ff168a526020908101516001600160a01b0316818b0152604090990198909201916001016131f7565b9461329e6132956132b095989493969861327261052e6001808054161490565b6132896132846110ea60ff60a1541690565b6133bd565b61222f888b5114612db5565b918883336146be565b6132a936868861079f565b903361493c565b925f5b8281106132c1575050505050565b806132f16132ec6132db612c07612bf9600196898b612dcb565b60ff165f52609760205260405f2090565b6133d3565b6132ff611fd4838951612994565b63ffffffff61331e613315845163ffffffff1690565b63ffffffff1690565b91161161332d575b50016132b3565b61337990613342612c07612bf985898b612dcb565b6133536131298560408c0151612994565b906133656131298660208d0151612994565b906133708689612994565b51923391614d07565b6133b761339a602061338b8487612994565b5101516001600160a01b031690565b6133b1612d056133a985612e42565b85898b612e62565b9061420d565b5f613326565b156133c457565b630b88306f60e01b5f5260045ffd5b906040516133e0816103d7565b604061ffff82945463ffffffff81168452818160201c16602085015260301c16910152565b1561340c57565b63fd2c1f4d60e01b5f5260045ffd5b1561342257565b6323d871a560e01b5f5260045ffd5b9061343b82610784565b61344860405191826103f2565b8281528092613459601f1991610784565b0190602036910137565b90815181101561143e570160200190565b1561347b57565b633cb89c9760e01b5f5260045ffd5b919082604091031261045b576040516134a2816103b7565b6020808294805184520151910152565b9080601f8301121561045b57604051916134cd6040846103f2565b82906040810192831161045b57905b8282106134e95750505090565b81518152602091820191016134dc565b91909180830390610120821261045b5780516001600160401b03811161045b57810184601f8201121561045b57805161353181610784565b9161353f60405193846103f2565b818352866020838301011161045b57815f9260208093018386015e8301015293610100601f1984011261045b5760806040519361357b856103d7565b613588836020860161348a565b8552613597836060860161348a565b6020860152609f19011261045b576135cd9060e0604051936135b8856103b7565b6135c58360a083016134b2565b8552016134b2565b6020820152604082015290565b5f198114612e505760010190565b5f81805b61366257506135fe9061ffff16613431565b5f5f5b8251821080613657575b15613650576001811b8416613629575b613624906135da565b613601565b9060016136249160ff60f81b8460f81b165f1a6136468287613463565b530191905061361b565b5050905090565b50610100811061360b565b5f198101818111612e505761ffff9116911661ffff8114612e505760010190806135ec565b9081602091031261045b57516001600160c01b038116810361045b5790565b6107f0939260609260018060a01b03168252602082015281604082015201906129f0565b9190600160208201516136dc816111fa565b6136e5816111fa565b0361379b57516040516333567f7f60e11b8152916020918391829161370f919087600485016136a6565b03815f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af19081156109e3575f9161376c575b506001600160c01b03169081613760575050565b6133b1610422926135e8565b61378e915060203d602011613794575b61378681836103f2565b810190613687565b5f61374c565b503d61377c565b505050565b156137a757565b60405162461bcd60e51b815260206004820152606660248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d61704174426c6f636b4e756d6265724279496e6465783a2071756f72756d4260648201527f69746d61705570646174652069732066726f6d206265666f726520626c6f636b608482015265273ab6b132b960d11b60a482015260c490fd5b6064546001600160a01b0316330361385b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b5f196001556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806001556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b609d54604080516001600160a01b038084168252841660208201529192917f315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c9190a16001600160a01b03166001600160a01b03199190911617609d55565b609e54604080516001600160a01b038084168252841660208201529192917f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc99190a16001600160a01b03166001600160a01b03199190911617609e55565b6139c9615081565b9060405190602082019261190160f01b845260228301526042820152604281526139f46062826103f2565b51902090565b634e487b7160e01b5f52601260045260245ffd5b5f5160206158ad5f395f51905f5290613a25612ad8565b505f919006602060c0835b613b25575f935f5160206158ad5f395f51905f5260038186818180090908604051613a5b85826103f2565b84368237848185604051613a6f82826103f2565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f5160206158ad5f395f51905f5260a082015260056107cf195a01fa8015613b2a57613ad9906155b7565b5191613b25575f5160206158ad5f395f51905f5282800914613b1057505f5160206158ad5f395f51905f5260015f94089293613a30565b92935050613b1c610413565b92835282015290565b6139fa565bfe5b60ff60019116019060ff8211612e5057565b60408051909190613b4f83826103f2565b6001815291601f1901825f5b828110613b6757505050565b602090604051613b76816103b7565b5f815260608382015282828501015201613b5b565b90613b9582610433565b613ba260405191826103f2565b8281528092613459601f1991610433565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310613be557505050505090565b9091929394603f1982820301835285516020606081604085019363ffffffff81511686520151936040838201528451809452019201905f905b808210613c3d5750505060208060019297019301930191939290613bd6565b82516001600160a01b0316845260209384019390920191600190910190613c1e565b90602080835192838152019201905f5b818110613c7c5750505090565b825180516001600160a01b031685526020908101516001600160601b03168186015260409094019390920191600101613c6f565b906107f094936001600160601b0360809460ff63ffffffff941685521660208401521660408201528160608201520190613c5f565b6001600160601b036107f0949360ff6060941683521660208201528160408201520190613c5f565b93909192613d1d60965460ff1690565b94613d4460ff871691613d3260c08410613474565b613d3e610e1989613b2c565b876140fb565b60a15460ff1680614053575b613f66575b50613d5f81613014565b80613ec95750507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561045b57613dbc925f9283604051809681958294633aea0b9d60e11b84528a60048501613ce5565b03925af180156109e357613eb5575b505b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163136ca0f960e11b815260ff83166004820152905f908290602490829084905af180156109e357613ea1575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163136ca0f960e11b815260ff90921660048301525f908290818381602481015b03925af180156109e357613e935750565b80610f8b5f610422936103f2565b80610f8b5f613eaf936103f2565b5f613e2d565b80610f8b5f613ec3936103f2565b5f613dcb565b80613ed8600192959395613014565b14613ee6575b505050613dcd565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b57613f3c935f809460405196879586948593630662d3e160e51b85528b60048601613cb0565b03925af180156109e357613f52575b8080613ede565b80610f8b5f613f60936103f2565b5f613f4b565b9592909491613f73613b3e565b95613f7e8651613b8b565b965f5b8751811015613fad5780613fa7613f9d610fec6001948c612994565b611004838d612994565b01613f81565b509193969790929497613fc1611029610413565b6020820152613fcf82612987565b52613fd981612987565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b5760405163847d634f60e01b8152915f9183918290849082906140309060048301613bb3565b03925af180156109e35715613d555780610f8b5f61404d936103f2565b5f613d55565b5061406f6110ea6110e38860ff165f5260a260205260405f2090565b613d50565b90600161408260ff93615219565b928392161b11156140905790565b60405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608490fd5b6141b960ff7f3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac921692835f52609760205260405f2061415163ffffffff835116829063ffffffff1663ffffffff19825416179055565b6020820151815465ffff0000000067ffff000000000000604086015160301b169260201b169067ffffffff0000000019161717905560405191829182919091604061ffff81606084019563ffffffff8151168552826020820151166020860152015116910152565b0390a2565b156141c557565b6368b6a87560e11b5f5260045ffd5b6001600160a01b0390911681526040602082018190526107f0929101906129f0565b6040906107f09392815281602082015201906129f0565b6001600160a01b0381165f9081526099602052604090209060018254920161424a600161423b835460ff1690565b614244816111fa565b146129da565b6142aa61426561059861425f60965460ff1690565b87614074565b61426e8561503d565b6001600160c01b03909116906142858215156141be565b61429b8282166001600160c01b03168314612e02565b9019166001600160c01b031690565b6142b4818561530e565b6001600160c01b031615614408575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b57835f91614319938360405180968195829463f4e24fe560e01b8452600484016141d4565b03925af180156109e3576143f4575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b575f604051809263bd29b8cd60e01b825281838161437b8989600484016141f6565b03925af180156109e3576143e0575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561045b57613e82925f928360405180968195829463bd29b8cd60e01b8452600484016141f6565b80610f8b5f6143ee936103f2565b5f61438a565b80610f8b5f614402936103f2565b5f614328565b805460ff191660021790557f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b576040516351b27a6d60e11b81526001600160a01b0383166004820152905f908290602490829084905af180156109e3576144af575b50816001600160a01b0382167f396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e45f80a35f6142c3565b80610f8b5f6144bd936103f2565b5f614479565b606480546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b9081602091031261045b575190565b905f905b6002821061452b57505050565b602080600192855181520193019101909161451e565b610120906145ab60206040610422969897959861016085019960018060a01b0316855261457b838601825160208091805184520151910152565b8083015180516060870152602001516080860152015161459f60a08501825161451a565b015160e083019061451a565b019060208091805184520151910152565b6040516309aa152760e11b81526001600160a01b0382811660048301529091907f000000000000000000000000000000000000000000000000000000000000000016602083602481845afa9283156109e3575f9361469d575b508215614623575050905090565b60209250614653935f61463584612af0565b6040516317ef39cb60e31b8152968795869485939160048501614541565b03925af19081156109e3575f9161466e575b50805f80613650565b614690915060203d602011614696575b61468881836103f2565b81019061450b565b5f614665565b503d61467e565b6146b791935060203d6020116146965761468881836103f2565b915f614615565b919290602082019283515f52609a60205260ff60405f20541661473957604083018051421161472a57610422956147229386515f52609a60205261470c60405f20600160ff19825416179055565b609d546001600160a01b03169651925193613197565b905191615428565b630819bdcd60e01b5f5260045ffd5b636fbefec360e11b5f5260045ffd5b60405190614755826103d7565b60606040838281528260208201520152565b1561476e57565b6313ca465760e01b5f5260045ffd5b1561478457565b630c6816cd60e01b5f5260045ffd5b1561479a57565b631968677d60e11b5f5260045ffd5b60016020918351815501910151600381101561088e5760ff80198354169116179055565b9060018060a01b0316815260406020820152608060406147f8845160608386015260a08501906129f0565b9360208101516060850152015191015290565b9080601f8301121561045b57815161482281610433565b9261483060405194856103f2565b81845260208085019260051b82010192831161045b57602001905b8282106148585750505090565b60208091835161486781610ce8565b81520191019061484b565b91909160408184031261045b5780516001600160401b03811161045b578361489b91830161480b565b9260208201516001600160401b03811161045b576107f0920161480b565b60208183031261045b578051906001600160401b03821161045b57019080601f8301121561045b5781516148ec81610433565b926148fa60405194856103f2565b81845260208085019260051b82010192831161045b57602001905b8282106149225750505090565b602080918351614931816105e1565b815201910190614915565b90919293827fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa614a0761496d614748565b976149fb61498961059861498360965460ff1690565b8b614074565b6149928661503d565b6001600160c01b03909116906149a9821515614767565b60018060c01b03166149c36149be8284161590565b61477d565b6001600160a01b0389165f908152609f602052604090206149f4906149ed905b5460a05490612e55565b4211614793565b178561530e565b60405191829182612a14565b0390a26001614a3281614a2a8560018060a01b03165f52609960205260405f2090565b015460ff1690565b614a3b816111fa565b03614bcf575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b575f6040518092631fd93ca960e11b8252818381614a948a89600484016141d4565b03925af180156109e35784925f928592614bbb575b50614ac86040519687938493632550477760e01b8552600485016136a6565b0381837f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af19182156109e357614b32935f9384918591614b97575b5060408701526020860152604051938492839262bff04d60e01b8452600484016141f6565b0381837f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af19081156109e3575f91614b75575b50815290565b614b9191503d805f833e614b8981836103f2565b8101906148b9565b5f614b6f565b9050614bb591503d8086833e614bad81836103f2565b810190614872565b5f614b0d565b80610f8b85614bc9936103f2565b5f614aa9565b614c00614bda610413565b848152600160208201526001600160a01b0384165f9081526099602052604090206147a9565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b57604051639926ee7d60e01b8152915f918391829084908290614c579089600484016147cd565b03925af180156109e357614c9c575b50816001600160a01b0382167fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe5f80a35f614a41565b80610f8b5f614caa936103f2565b5f614c66565b15614cb757565b6356168b4160e11b5f5260045ffd5b9081602091031261045b57516107f081610ce8565b15614ce257565b634c44995d60e01b5f5260045ffd5b15614cf857565b63b187e86960e01b5f5260045ffd5b60209192614d64614d57614d8f989697614d50614d2c8783015160018060a01b031690565b6001600160a01b039081165f81815260996020526040902054969091161415614cb0565b5160ff1690565b60ff808516911614612dec565b604051635401ed2760e01b8152600481019190915260ff909116602482015294859081906044820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9283156109e357610422945f94614e13575b5082614e0b92614e06614df2936001600160601b03614dfe614df282998b61546c565b6001600160601b031690565b911611614cdb565b61548f565b911610614cf1565b614df291945092614e0b92614e06614e4a6001600160601b039660203d602011614e56575b614e4281836103f2565b810190614cc6565b96935050925092614dcf565b503d614e38565b817fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa614eff614e8a614748565b966149fb614ea6610598614ea060965460ff1690565b8a614074565b614eaf8661503d565b6001600160c01b0390911690614ec6821515614767565b60018060c01b0316614edb6149be8284161590565b6001600160a01b0388165f908152609f602052604090206149f4906149ed906149e3565b0390a26001614f2281614a2a8460018060a01b03165f52609960205260405f2090565b614f2b816111fa565b03614f83575b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561045b575f6040518092631fd93ca960e11b8252818381614a948a89600484016141d4565b614fb4614f8e610413565b838152600160208201526001600160a01b0383165f9081526099602052604090206147a9565b614f31565b9190805190614fc782610433565b91614fd560405193846103f2565b808352614fe4601f1991610433565b013660208401375f5b8151811015615028578061500f61500660019385612994565b518760986154ad565b63ffffffff61501e8387612994565b9116905201614fed565b5090925050565b5f19810191908211612e5057565b805f52609860205260405f20549081155f146150595750505f90565b5f52609860205260405f20905f198101908111612e505761507991611429565b505460401c90565b307f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316148061516e575b156150dc577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f000000000000000000000000000000000000000000000000000000000000000082527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a081526139f460c0826103f2565b507f000000000000000000000000000000000000000000000000000000000000000046146150b3565b1561519e57565b60405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a490fd5b90610100825111615297578151156152925761525561524b612c0761523d85612987565b516001600160f81b03191690565b60ff600191161b90565b6001905b835182101561528d5760019061527861524b612c0761523d8689613463565b90615284818311615197565b17910190615259565b925050565b5f9150565b60a460405162461bcd60e51b815260206004820152604460248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b6084820152fd5b90615321825f52609860205260405f2090565b548061536e575061533d610422925f52609860205260405f2090565b615369615348610424565b4363ffffffff168152925b5f60208501526001600160c01b03166040840152565b615603565b9161539963ffffffff9361539361538d845f52609860205260405f2090565b9161502f565b90611429565b50906153a9825463ffffffff1690565b438516941684036153d457506104229250906001600160401b0382549181199060401b169116179055565b815467ffffffff000000001916602085901b67ffffffff00000000161790915561042292919061536990615410905f52609860205260405f2090565b9161535361541c610424565b63ffffffff9095168552565b9061543392916156ca565b1561543a57565b638baa579f60e01b5f5260045ffd5b906001600160601b03809116911602906001600160601b038216918203612e5057565b61548a6001600160601b039161ffff60206127109501511690615449565b160490565b61548a6001600160601b039161ffff60406127109501511690615449565b9190815f528260205260405f2054925f5b8481106155565760405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a490fd5b808503858111612e505761331561556c9161502f565b61559561558a826155858887905f5260205260405f2090565b611429565b505463ffffffff1690565b63ffffffff808616911611156155ae57506001016154be565b94505050505090565b156155be57565b60405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606490fd5b8054600160401b8110156103d25761562091600182018155611429565b61565e57815160208084015160409485015163ffffffff909316911b67ffffffff00000000161767ffffffffffffffff199190931b16919091179055565b634e487b7160e01b5f525f60045260245ffd5b6005111561088e57565b3d156156a5573d9061568c82610784565b9161569a60405193846103f2565b82523d5f602084013e565b606090565b9081602091031261045b57516001600160e01b03198116810361045b5790565b9190916156d78284615795565b6156e081615671565b15908161577f575b50615777575f9261571561572385946040519283916020830195630b135d3f60e11b8752602484016141f6565b03601f1981018352826103f2565b51915afa61572f61567b565b8161576b575b8161573e575090565b8051630b135d3f60e11b92506001600160e01b031991615766918101602090810191016156aa565b161490565b80516020149150615735565b505050600190565b6001600160a01b0383811691161490505f6156e8565b8151604181036157c15750906157bd91602082015190606060408401519301515f1a90615803565b9091565b6040036157fa5760406020830151920151918260ff1c91601b8301809311612e50576157bd936001600160ff1b03169260ff1690615803565b50505f90600290565b9291907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083116158a15760ff16601b81141580615896575b61588b576020935f93604051938493608085019385528785015260408401526060830152838052039060015afa156109e3575f516001600160a01b0381161561588357905f90565b505f90600190565b505050505f90600490565b50601c81141561583b565b505050505f9060039056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220b99f5cdf10f1afbec60a96a467376aca78e0010a34f26b7ed32db3a82257596d64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\xCF*\xB5\x14a\x03\x9EW\x80c\x03\xFD4\x92\x14a\x03\x99W\x80c\x04\xECcQ\x14a\x03\x94W\x80c\x05C\x10\xE6\x14a\x03\x8FW\x80c\x0C\xF4\xB7g\x14a\x03\x8AW\x80c\r?!4\x14a\x03\x85W\x80c\x12^\x05\x84\x14a\x03\x80W\x80c\x13T*N\x14a\x03{W\x80c\x13d9\xDD\x14a\x03vW\x80c\x14x\x85\x1F\x14a\x03qW\x80c\x1E\xB8\x12\xDA\x14a\x03lW\x80c$\x9A\x0CB\x14a\x03gW\x80c(\xF6\x1B1\x14a\x03bW\x80c)k\xB0d\x14a\x03]W\x80c)\xD1\xE0\xC3\x14a\x03XW\x80c,\xDD\x1E\x86\x14a\x03SW\x80c9\x98\xFD\xD3\x14a\x03NW\x80c<*\x7FL\x14a\x03IW\x80c>\xEF:Q\x14a\x03DW\x80cQ@\xA5H\x14a\x03?W\x80cXe\xC6\x0C\x14a\x03:W\x80cY\\jg\x14a\x035W\x80cZ\xC8j\xB7\x14a\x030W\x80c[\x0B\x82\x9F\x14a\x03+W\x80c\\\x97Z\xBB\x14a\x03&W\x80c]\xF4YF\x14a\x03!W\x80ccG\xC9\0\x14a\x03\x1CW\x80ch0H5\x14a\x03\x17W\x80ck:\xA7.\x14a\x03\x12W\x80cn;\x17\xDB\x14a\x03\rW\x80cqP\x18\xA6\x14a\x03\x08W\x80c\x7F\xC3\xF8\x86\x14a\x03\x03W\x80c\x82\x81\xABu\x14a\x02\xFEW\x80c\x84\xCAR\x13\x14a\x02\xF9W\x80c\x87\x1E\xF0I\x14a\x02\xF4W\x80c\x88o\x11\x95\x14a\x02\xEFW\x80c\x8D\xA5\xCB[\x14a\x02\xEAW\x80c\x9A\xA1e=\x14a\x02\xE5W\x80c\x9B]\x17{\x14a\x02\xE0W\x80c\x9D\x8E\x0C#\x14a\x02\xDBW\x80c\x9E\x99#\xC2\x14a\x02\xD6W\x80c\x9F\xEA\xB8Y\x14a\x02\xD1W\x80c\xA4\xD7\x87\x1F\x14a\x02\xCCW\x80c\xA5\x08W\xBF\x14a\x02\xC7W\x80c\xA9ox>\x14a\x02\xC2W\x80c\xAD\xCFs\xF7\x14a\x02\xBDW\x80c\xBD3\xEE$\x14a\x02\xA9W\x80c\xC3\x91B^\x14a\x02\xB8W\x80c\xCA\r\xE8\x82\x14a\x02\xB3W\x80c\xCAO-\x97\x14a\x02\xAEW\x80c\xCA\xBB\xB1\x7F\x14a\x02\xA9W\x80c\xD7-\x8D\xD6\x14a\x02\xA4W\x80c\xE6W\x97\xAD\x14a\x02\x9FW\x80c\xEE1\x88!\x14a\x02\x9AW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x95W\x80c\xFA\xBC\x1C\xBC\x14a\x02\x90Wc\xFD9\x10Z\x14a\x02\x8BW_\x80\xFD[a)+V[a(IV[a'\xB8V[a&\xD0V[a&<V[a&\x1FV[a$9V[a%xV[a%>V[a$\x9AV[a\"\xBEV[a\"\xA1V[a!\x8EV[a!WV[a!\x1DV[a \xD9V[a\x1FIV[a\x1E\x83V[a\x1D/V[a\x1D\x07V[a\x1C\xC3V[a\x1C\x93V[a\x1C7V[a\x19\xB3V[a\x18\xDCV[a\x16,V[a\x15%V[a\x14\xE1V[a\x14\x9DV[a\x14CV[a\x13\xD1V[a\x13\xB4V[a\x13\x1FV[a\x12\xF0V[a\x12}V[a\x12\x11V[a\x11 V[a\r\x8BV[a\x0CJV[a\x0C\x06V[a\x0B\xD9V[a\x0B\xACV[a\n\xF9V[a\n\xD1V[a\n\x9FV[a\n\x17V[a\t\xE8V[a\t*V[a\x08\xEFV[a\x08\xB4V[a\x08\x93V[a\x07\xF3V[a\x07\\V[a\x05\xEFV[a\x05\xB7V[a\x04\xEDV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xD2W`@RV[a\x03\xA3V[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xD2W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xD2W`@RV[`@Q\x90a\x04\"`@\x83a\x03\xF2V[V[`@Q\x90a\x04\"``\x83a\x03\xF2V[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xD2W`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04[WV[_\x80\xFD[`\x045\x90a\x04\"\x82a\x04JV[`$5\x90a\x04\"\x82a\x04JV[`D5\x90a\x04\"\x82a\x04JV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x815a\x04\x9D\x81a\x043V[\x92a\x04\xAB`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04[W` \x01\x90[\x82\x82\x10a\x04\xD3WPPP\x90V[` \x80\x91\x835a\x04\xE2\x81a\x04JV[\x81R\x01\x91\x01\x90a\x04\xC6V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x05\x1D\x906\x90`\x04\x01a\x04\x86V[a\x054a\x05.`\x04\x80`\x01T\x16\x14\x90V[\x15a)qV[_[\x81Q\x81\x10\x15a\x05\xB5W`\x01\x90a\x05\xAF`\x01`\x01`\xA0\x1B\x03a\x05W\x83\x86a)\x94V[Q\x16\x80_R`\x99` R`@_ a\x05\x88`\xFF\x86`@Q\x93a\x05x\x85a\x03\xB7V[\x80T\x85R\x01T\x16` \x83\x01a)\xA8V[a\x05\xA9a\x05\xA4a\x05\x98\x83QaP=V[`\x01`\x01`\xC0\x1B\x03\x16\x90V[a5\xE8V[\x91a6\xCAV[\x01a\x056V[\0[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045_R`\x98` R` `@_ T`@Q\x90\x81R\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x04[WV[4a\x04[W``6`\x03\x19\x01\x12a\x04[W`$5a\x06/a\x06)`\x045a\x06\x15\x84a\x05\xE1V[`D5\x90_R`\x98` R`@_ a\x14)V[Pa*\x92V[c\xFF\xFF\xFF\xFF\x80\x82Q\x16\x92\x16\x91\x82\x10a\x06\xB3W`@\x81a\x06wa\x06\x9F\x94a\x06_` a\x06\x85\x96\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90c\xFF\xFF\xFF\xFF\x82\x16\x15\x91\x82\x15a\x06\xA3W[PPa7\xA0V[\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[c\xFF\xFF\xFF\xFF\x16\x11\x90P_\x80a\x06pV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from after blockN`\x84\x82\x01Rd:\xB6\xB12\xB9`\xD9\x1B`\xA4\x82\x01R`\xC4\x90\xFD[_\x91\x03\x12a\x04[WV[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`\x9DT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xD2W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xAB\x82a\x07\x84V[\x91a\x07\xB9`@Q\x93\x84a\x03\xF2V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x04[W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x81` a\x07\xF0\x935\x91\x01a\x07\x9FV[\x90V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x08#\x906\x90`\x04\x01a\x07\xD5V[3_R`\x99` R`\xFF`\x01`@_ \x01T\x16`\x03\x81\x10\x15a\x08\x8EW`\x01a\x08K\x91\x14a)\xDAV[3_R`\x99` R\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAAa\x08\x89`@_ T\x92`@Q\x91\x82\x91\x82a*\x14V[\x03\x90\xA2\0[a\x11\xE6V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a\x08\xAFa8GV[`\xA0U\0[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a\x08\xD1\x81a\x04JV[`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R` `@_ T`@Q\x90\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a\t\x0C\x81a\x04JV[`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R` `@_ T`@Q\x90\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\t\xE3Wa\x05\xB5\x92a\t\xA0\x91_\x91a\t\xB4W[Pa*HV[a\t\xAF`\x01T\x82\x81\x16\x14a*^V[a8\xD3V[a\t\xD6\x91P` =` \x11a\t\xDCW[a\t\xCE\x81\x83a\x03\xF2V[\x81\x01\x90a*%V[_a\t\x9AV[P=a\t\xC4V[a*=V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045_R`\x9A` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04[W`@6`\x03\x19\x01\x12a\x04[W``a\nOa\x06)`$5`\x045a\n>a*tV[P_R`\x98` R`@_ a\x14)V[`@Q\x90c\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@`\x01\x80`\xC0\x1B\x03\x91\x01Q\x16`@\x82\x01R\xF3[`\x045\x90`\xFF\x82\x16\x82\x03a\x04[WV[5\x90`\xFF\x82\x16\x82\x03a\x04[WV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\xFFa\n\xBAa\n\x81V[\x16_R`\x9B` R` `@_ T`@Q\x90\x81R\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`\x9ET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x805\x90\x82\x01R` \x81`$\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\t\xE3Wa\x06\x9F\x91_\x91a\x0B}W[P`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[a\x0B\x9F\x91P` =` \x11a\x0B\xA5W[a\x0B\x97\x81\x83a\x03\xF2V[\x81\x01\x90a*\xC3V[_a\x0BbV[P=a\x0B\x8DV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[Wa\x05\xB5`\x045a\x0B\xCC\x81a\x04JV[a\x0B\xD4a8GV[a9\x05V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[Wa\x05\xB5`\x045a\x0B\xF9\x81a\x04JV[a\x0C\x01a8GV[a9cV[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`@a\x0Cq`\x045a\x0Cl\x81a\x04JV[a*\xF0V[a\x0C\x87\x82Q\x80\x92` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\xF3[5\x90a\xFF\xFF\x82\x16\x82\x03a\x04[WV[``\x90`\x03\x19\x01\x12a\x04[W`@Q\x90a\x0C\xB1\x82a\x03\xD7V[\x81`\x045a\x0C\xBE\x81a\x05\xE1V[\x81R`$5a\xFF\xFF\x81\x16\x81\x03a\x04[W` \x82\x01R`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x04[W`@\x01RV[`\x01`\x01``\x1B\x03\x81\x16\x03a\x04[WV[\x81`\x1F\x82\x01\x12\x15a\x04[W\x805\x90a\r\x10\x82a\x043V[\x92a\r\x1E`@Q\x94\x85a\x03\xF2V[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x04[W` \x01\x92[\x82\x84\x10a\rHWPPPP\x90V[`@\x84\x83\x03\x12a\x04[W` `@\x91\x82Qa\rb\x81a\x03\xB7V[\x865a\rm\x81a\x04JV[\x81R\x82\x87\x015a\r|\x81a\x0C\xE8V[\x83\x82\x01R\x81R\x01\x93\x01\x92a\r:V[4a\x04[W`\xC06`\x03\x19\x01\x12a\x04[Wa\r\xA56a\x0C\x98V[`d5a\r\xB1\x81a\x0C\xE8V[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\r\xD0\x906\x90`\x04\x01a\x0C\xF9V[\x90`\xA45\x91a\r\xDE\x83a\x05\xE1V[a\r\xE6a8GV[a\r\xF4`\xFF`\xA1T\x16a+VV[`\x96T`\xFF\x16\x93\x84\x90a\x0E/\x90a\x0E\r`\xC0\x84\x10a4tV[a\x0E)a\x0E\x19\x88a;,V[`\xFF\x16`\xFF\x19`\x96T\x16\x17`\x96UV[\x86a@\xFBV[`\xA1T`\xFF\x16\x80a\x10\xC7W[a\x0F\xB9W[Pa\x0EK`\x01a0\x14V[a\x0EU`\x01a0\x14V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[Wa\x0E\xAB\x93_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x06b\xD3\xE1`\xE5\x1B\x85R\x8B`\x04\x86\x01a<\xB0V[\x03\x92Z\xF1\x80\x15a\t\xE3Wa\x0F\xA5W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xE3Wa\x0F\x91W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xE3Wa\x0F}W\0[\x80a\x0F\x8B_a\x05\xB5\x93a\x03\xF2V[\x80a\x07RV[\x80a\x0F\x8B_a\x0F\x9F\x93a\x03\xF2V[_a\x0F\x1BV[\x80a\x0F\x8B_a\x0F\xB3\x93a\x03\xF2V[_a\x0E\xBAV[\x92a\x0F\xC2a;>V[\x92a\x0F\xCD\x83Qa;\x8BV[\x93_[\x84Q\x81\x10\x15a\x10\x19W\x80a\x10\x13a\x0F\xFAa\x0F\xEC`\x01\x94\x89a)\x94V[QQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x10\x04\x83\x8Aa)\x94V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\x0F\xD0V[P\x91\x94\x90\x93a\x105a\x10)a\x04\x13V[c\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[` \x82\x01Ra\x10C\x82a)\x87V[Ra\x10M\x81a)\x87V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x84}cO`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x10\xA4\x90`\x04\x83\x01a;\xB3V[\x03\x92Z\xF1\x80\x15a\t\xE3W\x15a\x0E@W\x80a\x0F\x8B_a\x10\xC1\x93a\x03\xF2V[_a\x0E@V[Pa\x10\xEEa\x10\xEAa\x10\xE3\x87`\xFF\x16_R`\xA2` R`@_ \x90V[T`\xFF\x16\x90V[\x15\x90V[a\x0E;V[\x91\x81`\x1F\x84\x01\x12\x15a\x04[W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x04[W` \x83\x81\x86\x01\x95\x01\x01\x11a\x04[WV[4a\x04[W`@6`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[W6`#\x82\x01\x12\x15a\x04[W\x80`\x04\x015a\x11[\x81a\x043V[\x91a\x11i`@Q\x93\x84a\x03\xF2V[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x04[W`$\x81\x01\x92[\x82\x84\x10a\x11\xB7W`$5\x85`\x01`\x01`@\x1B\x03\x82\x11a\x04[Wa\x11\xB1a\x05\xB5\x926\x90`\x04\x01a\x10\xF3V[\x91a+lV[\x835`\x01`\x01`@\x1B\x03\x81\x11a\x04[W` \x91a\x11\xDB\x83\x92`$6\x91\x87\x01\x01a\x04\x86V[\x81R\x01\x93\x01\x92a\x11\x87V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x11\x15a\x08\x8EWV[\x90`\x03\x82\x10\x15a\x08\x8EWRV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a\x12.\x81a\x04JV[a\x126a*\xD8V[P`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ a\x12^`\xFF`\x01`@Q\x93a\x05x\x85a\x03\xB7V[`@Q\x80\x91a\x06\x9F` `@\x84\x01\x92\x80Q\x85R\x01Q` \x84\x01\x90a\x12\x04V[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\xE3Wa\x12\xE8\x91_\x91a\t\xB4WPa*HV[a\x05\xB5a8\x9FV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W` `\x01`\xFFa\x13\x0Fa\n\x81V[\x16\x1B\x80`\x01T\x16\x14`@Q\x90\x81R\xF3[4a\x04[W`\x806`\x03\x19\x01\x12a\x04[Wa\x138a\n\x81V[``6`#\x19\x01\x12a\x04[W`@Qa\x13P\x81a\x03\xD7V[`$5a\x13\\\x81a\x05\xE1V[\x81R`D5a\xFF\xFF\x81\x16\x81\x03a\x04[W` \x82\x01R`d5a\xFF\xFF\x81\x16\x81\x03a\x04[W`@\x82\x01Ra\x13\x8Ca8GV[`\xFF`\x96T\x16`\xFF\x83\x16\x10\x15a\x13\xA5Wa\x05\xB5\x91a@\xFBV[cs\x10\xCF\xF5`\xE1\x1B_R`\x04_\xFD[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `\x01T`@Q\x90\x81R\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x82\x10\x15a\x14>W_R` _ \x01\x90_\x90V[a\x14\x15V[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`\x9CT\x81\x10\x15a\x04[W`\x9C_R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04[W`@6`\x03\x19\x01\x12a\x04[W`\x045a\x15B\x81a\x04JV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x15a\x906\x90`\x04\x01a\x07\xD5V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x03a\x16\x1DW`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 B\x90U`\x99\x90\x91R\x90 \x80T`\x01a\x15\xC9\x81a\x15\xC0a\x15\xBAa\x05\x98a\x15\xB4`\x96T`\xFF\x16\x90V[\x89a@tV[\x94aP=V[\x94\x01T`\xFF\x16\x90V[a\x15\xD2\x81a\x11\xFAV[\x14\x91\x82a\x16\nW[\x82a\x15\xF1W[PPa\x15\xE8W\0[a\x05\xB5\x91aB\rV[\x81\x16`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x91\x16\x14\x90P_\x80a\x15\xE0V[`\x01`\x01`\xC0\x1B\x03\x82\x16\x15\x15\x92Pa\x15\xDAV[cv\xD8\xAB\x17`\xE1\x1B_R`\x04_\xFD[4a\x04[W_6`\x03\x19\x01\x12a\x04[Wa\x16Da8GV[`d\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[\x81`\x1F\x82\x01\x12\x15a\x04[W\x805\x90a\x16\x9E\x82a\x043V[\x92a\x16\xAC`@Q\x94\x85a\x03\xF2V[\x82\x84R` ``\x81\x86\x01\x94\x02\x83\x01\x01\x91\x81\x83\x11a\x04[W` \x01\x92[\x82\x84\x10a\x16\xD6WPPPP\x90V[``\x84\x83\x03\x12a\x04[W` ``\x91`@Qa\x16\xF1\x81a\x03\xD7V[\x865a\x16\xFC\x81a\x05\xE1V[\x81Ra\x17\t\x83\x88\x01a\x0C\x89V[\x83\x82\x01Ra\x17\x19`@\x88\x01a\x0C\x89V[`@\x82\x01R\x81R\x01\x93\x01\x92a\x16\xC8V[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x815a\x17@\x81a\x043V[\x92a\x17N`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04[W` \x01\x90[\x82\x82\x10a\x17vWPPP\x90V[` \x80\x91\x835a\x17\x85\x81a\x0C\xE8V[\x81R\x01\x91\x01\x90a\x17iV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x815a\x17\xA7\x81a\x043V[\x92a\x17\xB5`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x04[W` \x82\x01\x90[\x83\x82\x10a\x17\xE1WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x04[W` \x91a\x18\x03\x87\x84\x80\x94\x88\x01\x01a\x0C\xF9V[\x81R\x01\x91\x01\x90a\x17\xD2V[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x815\x90a\x18&\x82a\x043V[\x92a\x184`@Q\x94\x85a\x03\xF2V[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a\x04[W` \x01\x91[\x81\x83\x10a\x18\\WPPP\x90V[\x825`\x02\x81\x10\x15a\x04[W\x81R` \x92\x83\x01\x92\x01a\x18OV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x815a\x18\x8C\x81a\x043V[\x92a\x18\x9A`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04[W` \x01\x90[\x82\x82\x10a\x18\xC2WPPP\x90V[` \x80\x91\x835a\x18\xD1\x81a\x05\xE1V[\x81R\x01\x91\x01\x90a\x18\xB5V[4a\x04[Wa\x01 6`\x03\x19\x01\x12a\x04[Wa\x18\xF6a\x04_V[a\x18\xFEa\x04lV[\x90a\x19\x07a\x04yV[`d5`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x19)\x906\x90`\x04\x01a\x16\x87V[`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x19H\x906\x90`\x04\x01a\x17)V[\x90`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x19h\x906\x90`\x04\x01a\x17\x90V[\x92`\xE45`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x19\x88\x906\x90`\x04\x01a\x18\x0EV[\x94a\x01\x045\x97`\x01`\x01`@\x1B\x03\x89\x11a\x04[Wa\x19\xADa\x05\xB5\x996\x90`\x04\x01a\x18uV[\x97a.\xDDV[4a\x04[W`\xA06`\x03\x19\x01\x12a\x04[Wa\x19\xCD6a\x0C\x98V[`d5a\x19\xD9\x81a\x0C\xE8V[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x19\xF8\x906\x90`\x04\x01a\x0C\xF9V[\x90a\x1A\x01a8GV[`\x96T`\xFF\x16\x92\x83\x90a\x1A,\x90a\x1A\x1A`\xC0\x84\x10a4tV[a\x1A&a\x0E\x19\x87a;,V[\x85a@\xFBV[`\xA1T`\xFF\x16\x80a\x1B\x86W[a\x1A\x9DW[Pa\x1AG_a0\x14V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x04[Wa\x0E\xAB\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c:\xEA\x0B\x9D`\xE1\x1B\x84R\x8A`\x04\x85\x01a<\xE5V[\x91\x90\x92a\x1A\xA8a;>V[\x93a\x1A\xB3\x83Qa;\x8BV[\x94_[\x84Q\x81\x10\x15a\x1A\xE2W\x80a\x1A\xDCa\x1A\xD2a\x0F\xEC`\x01\x94\x89a)\x94V[a\x10\x04\x83\x8Ba)\x94V[\x01a\x1A\xB6V[P\x91\x94\x93\x90\x92\x93a\x1A\xF4a\x10)a\x04\x13V[` \x82\x01Ra\x1B\x02\x82a)\x87V[Ra\x1B\x0C\x81a)\x87V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x84}cO`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x1Bc\x90`\x04\x83\x01a;\xB3V[\x03\x92Z\xF1\x80\x15a\t\xE3W\x15a\x1A=W\x80a\x0F\x8B_a\x1B\x80\x93a\x03\xF2V[_a\x1A=V[Pa\x1B\xA2a\x10\xEAa\x10\xE3\x86`\xFF\x16_R`\xA2` R`@_ \x90V[a\x1A8V[\x81`\x1F\x82\x01\x12\x15a\x04[W\x805\x90a\x1B\xBE\x82a\x043V[\x92a\x1B\xCC`@Q\x94\x85a\x03\xF2V[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x04[W` \x01\x92[\x82\x84\x10a\x1B\xF6WPPPP\x90V[`@\x84\x83\x03\x12a\x04[W` `@\x91\x82Qa\x1C\x10\x81a\x03\xB7V[a\x1C\x19\x87a\n\x91V[\x81R\x82\x87\x015a\x1C(\x81a\x04JV[\x83\x82\x01R\x81R\x01\x93\x01\x92a\x1B\xE8V[4a\x04[W`\xA06`\x03\x19\x01\x12a\x04[W`\x045a\x1CT\x81a\x04JV[`$5\x90`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04[W` \x92a\x1C~a\x1C\x8B\x936\x90`\x04\x01a\x1B\xA7V[`d5\x91`\x845\x93a1\x97V[`@Q\x90\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W` a\x1C\xB1`\x045aP=V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`dT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `\xFF`\x96T\x16`@Q\x90\x81R\xF3[\x91\x90\x82`@\x91\x03\x12a\x04[W`@Qa\x1Dg\x81a\x03\xB7V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W`@Q\x91a\x1D\x92`@\x84a\x03\xF2V[\x82\x90`@\x81\x01\x92\x83\x11a\x04[W\x90[\x82\x82\x10a\x1D\xAEWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x1D\xA1V[\x90a\x01\0`C\x19\x83\x01\x12a\x04[W`@Q\x91a\x1D\xD9\x83a\x03\xD7V[\x82a\x1D\xE5\x82`Da\x1DOV[\x81Ra\x1D\xF2\x82`\x84a\x1DOV[` \x82\x01R`\x80`\xC3\x19\x83\x01\x12a\x04[W`@\x90a\x1E*\x82Q\x93a\x1E\x15\x85a\x03\xB7V[a\x1E \x81`\xC4a\x1DwV[\x85Ra\x01\x04a\x1DwV[` \x84\x01R\x01RV[\x91\x90\x91``\x81\x84\x03\x12a\x04[W`@Q\x90a\x1EM\x82a\x03\xD7V[\x81\x93\x815\x91`\x01`\x01`@\x1B\x03\x83\x11a\x04[Wa\x1Ep`@\x93\x92\x84\x93\x83\x01a\x07\xD5V[\x84R` \x81\x015` \x85\x01R\x015\x91\x01RV[4a\x04[Wa\x01\xA06`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x1E\xB4\x906\x90`\x04\x01a\x10\xF3V[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x1E\xD4\x906\x90`\x04\x01a\x07\xD5V[a\x1E\xDD6a\x1D\xBEV[a\x01D5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x1E\xFD\x906\x90`\x04\x01a\x1B\xA7V[\x90a\x01d5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x1F\x1E\x906\x90`\x04\x01a\x1E3V[\x92a\x01\x845\x95`\x01`\x01`@\x1B\x03\x87\x11a\x04[Wa\x1FCa\x05\xB5\x976\x90`\x04\x01a\x1E3V[\x95a2RV[4a\x04[W`@6`\x03\x19\x01\x12a\x04[W`\x045a\x1Ff\x81a\x04JV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x1F\x85\x906\x90`\x04\x01a\x18uV[\x90a\x1F\x96a\x05.`\x01\x80\x80T\x16\x14\x90V[a\x1F\xAAa\x1F\xA5`\xFF`\xA1T\x16\x90V[a+VV[_[\x82Q\x81\x10\x15a \0W\x80a\x1F\xFAa\x1F\xF5a\x10\xEAa\x10\xE3a\x1F\xE4a\x1F\xDEa\x1F\xD4`\x01\x98\x8Ba)\x94V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`\xFF\x16\x90V[`\xFF\x16_R`\xA2` R`@_ \x90V[a4\x05V[\x01a\x1F\xACV[P`@Qc\xCA\x8A\xA7\xC7`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\xE3Wa b\x91_\x91a \xBAW[P`\x01`\x01`\xA0\x1B\x03\x163\x14a4\x1BV[a l\x82Qa41V[_[\x83Q\x81\x10\x15a \xB0W\x80a \x9Da \x8Da\x1F\xDEa\x1F\xD4`\x01\x95\x89a)\x94V[`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[_\x1Aa \xA9\x82\x85a4cV[S\x01a nV[Pa\x05\xB5\x91aB\rV[a \xD3\x91P` =` \x11a\x0B\xA5Wa\x0B\x97\x81\x83a\x03\xF2V[_a QV[4a\x04[W_6`\x03\x19\x01\x12a\x04[W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `@Q\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\xFFa!ra\n\x81V[\x16_R`\xA2` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04[Wa\x01`6`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa!\xBF\x906\x90`\x04\x01a\x07\xD5V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa!\xDE\x906\x90`\x04\x01a\x07\xD5V[a!\xE76a\x1D\xBEV[a\x01D5\x91`\x01`\x01`@\x1B\x03\x83\x11a\x04[W\x83a\"5a\"\x0Fa\";\x956\x90`\x04\x01a\x1E3V[\x93a\" a\x05.`\x01\x80\x80T\x16\x14\x90V[a\"/`\xFF`\xA1T\x16\x15a3\xBDV[3aE\xBCV[3aI<V[Q\x90_[\x81Q\x81\x10\x15a\x05\xB5W\x80a\"\x9Ba\"X`\x01\x93\x85a4cV[Q`\xF8\x1Cc\xFF\xFF\xFF\xFFa\"\x92\x81a\"\x87\x81a\"s\x88\x8Ca)\x94V[Q\x16\x94`\xFF\x16_R`\x97` R`@_ \x90V[T\x16c\xFF\xFF\xFF\xFF\x16\x90V[\x91\x16\x11\x15a4tV[\x01a\"?V[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `\xA0T`@Q\x90\x81R\xF3[4a\x04[W``6`\x03\x19\x01\x12a\x04[W`\x045a\"\xDB\x81a\x04JV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\"\xFA\x906\x90`\x04\x01a\x18uV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa#\x1A\x906\x90`\x04\x01a\x07\xD5V[\x90a#+a\x05.`\x01\x80\x80T\x16\x14\x90V[a#:a\x1F\xA5`\xFF`\xA1T\x16\x90V[_[\x83Q\x81\x10\x15a#jW\x80a#da\x1F\xF5a\x10\xEAa\x10\xE3a\x1F\xE4a\x1F\xDEa\x1F\xD4`\x01\x98\x8Ca)\x94V[\x01a#<V[P`@Qc\xCA\x8A\xA7\xC7`\xE0\x1B\x81R\x90\x91` \x82`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\xE3Wa#\xD5a#\xE4\x92a#\xED\x94_\x91a \xBAWP`\x01`\x01`\xA0\x1B\x03\x163\x14a4\x1BV[` \x80\x82Q\x83\x01\x01\x91\x01a4\xF9V[\x83\x92\x91\x92aE\xBCV[\x90a#\xF8\x84Qa41V[\x91_[\x85Q\x81\x10\x15a$-W\x80a$\x1Aa \x8Da\x1F\xDEa\x1F\xD4`\x01\x95\x8Ba)\x94V[_\x1Aa$&\x82\x87a4cV[S\x01a#\xFBV[P\x91a\x05\xB5\x92\x84aN]V[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `\xFF`\xA1T\x16`@Q\x90\x15\x15\x81R\xF3[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a$~WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a$qV[4a\x04[W`@6`\x03\x19\x01\x12a\x04[W`\x045a$\xB7\x81a\x05\xE1V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04[W6`#\x83\x01\x12\x15a\x04[W\x81`\x04\x015\x91a$\xE3\x83a\x043V[\x92a$\xF1`@Q\x94\x85a\x03\xF2V[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x04[W`$\x01\x90[\x82\x82\x10a%.Wa\x06\x9Fa%\"\x86\x86aO\xB9V[`@Q\x91\x82\x91\x82a$[V[\x815\x81R` \x91\x82\x01\x91\x01a%\x0EV[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `@Q\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa%\xA8\x906\x90`\x04\x01a\x07\xD5V[a%\xB9a\x05.`\x02\x80`\x01T\x16\x14\x90V[_`\xFF`\xA1T\x16\x15\x80\x15\x91[\x83Q\x81\x10\x15a&\x15W\x80a%\xDB`\x01\x92\x86a4cV[Q`\xF8\x1C\x83\x85a%\xF6W[a%\xF0\x91Pa3\xBDV[\x01a%\xC5V[P_R`\xA2` Ra%\xF0a&\x10`@_ `\xFF\x90T\x16\x90V[a%\xE6V[a\x05\xB5\x843aB\rV[4a\x04[W_6`\x03\x19\x01\x12a\x04[W` `\x9CT`@Q\x90\x81R\xF3[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\xFFa&Wa\n\x81V[a&_a*tV[P\x16_R`\x97` Ra\x06\x9F`@_ a\xFF\xFF`@Q\x91a&\x7F\x83a\x03\xD7V[Tc\xFF\xFF\xFF\xFF\x81\x16\x83R\x81\x81` \x1C\x16` \x84\x01R`0\x1C\x16`@\x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91`@a\xFF\xFF\x81``\x84\x01\x95c\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[4a\x04[W_6`\x03\x19\x01\x12a\x04[Wa&\xE8a8GV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x0F%\xF1a`\xE4\x1B\x81R0`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xE3Wa'\xA4W[P_[a'Wa\x1F\xDE`\x96T`\xFF\x16\x90V[`\xFF\x82\x16\x10\x15a'\x92W`\x01\x81a'\x8Ba'~`\xFF\x94`\xFF\x16_R`\xA2` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[\x01\x16a'HV[a\x05\xB5`\x01`\xFF\x19`\xA1T\x16\x17`\xA1UV[\x80a\x0F\x8B_a'\xB2\x93a\x03\xF2V[_a'EV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a'\xD5\x81a\x04JV[a'\xDDa8GV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a'\xF5Wa\x05\xB5\x90aD\xC3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\xE3W_\x91a)\x0CW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a(\xFDWa(\xCB`\x01T\x19\x82\x19\x81\x16\x14a*^V[\x80`\x01U`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a)%\x91P` =` \x11a\x0B\xA5Wa\x0B\x97\x81\x83a\x03\xF2V[_a(\xAAV[4a\x04[W` 6`\x03\x19\x01\x12a\x04[W`\x045a)H\x81a\x04JV[`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R` `\xFF`\x01`@_ \x01T\x16a\x0C\x87`@Q\x80\x92a\x12\x04V[\x15a)xWV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x80Q\x15a\x14>W` \x01\x90V[\x80Q\x82\x10\x15a\x14>W` \x91`\x05\x1B\x01\x01\x90V[`\x03\x82\x10\x15a\x08\x8EWRV[\x90a\x04\"`@Qa)\xC4\x81a\x03\xB7V[` `\xFF`\x01\x83\x96\x80T\x85R\x01T\x16\x91\x01a)\xA8V[\x15a)\xE1WV[c\xAB\xA4s9`\xE0\x1B_R`\x04_\xFD[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x07\xF0\x92\x81\x81R\x01\x90a)\xF0V[\x90\x81` \x91\x03\x12a\x04[WQ\x80\x15\x15\x81\x03a\x04[W\x90V[`@Q=_\x82>=\x90\xFD[\x15a*OWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a*eWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[`@Q\x90a*\x81\x82a\x03\xD7V[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x90`@Qa*\x9F\x81a\x03\xD7V[`@\x81\x93Tc\xFF\xFF\xFF\xFF\x81\x16\x83Rc\xFF\xFF\xFF\xFF\x81` \x1C\x16` \x84\x01R\x81\x1C\x91\x01RV[\x90\x81` \x91\x03\x12a\x04[WQa\x07\xF0\x81a\x04JV[`@Q\x90a*\xE5\x82a\x03\xB7V[_` \x83\x82\x81R\x01RV[a+Qa\x07\xF0\x91a*\xFFa*\xD8V[P`@\x80Q\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6` \x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x81\x83\x01R\x90\x81Ra+I``\x82a\x03\xF2V[Q\x90 a9\xC1V[a:\x0EV[\x15a+]WV[c[w\x90\x19`\xE0\x1B_R`\x04_\xFD[\x90\x92\x91a+\x80a\x05.`\x04\x80`\x01T\x16\x14\x90V[a+\x9Fa+\x8F`\x96T`\xFF\x16\x90V[a+\x9A6\x84\x88a\x07\x9FV[a@tV[Pa+\xAC\x81\x83Q\x14a-\xB5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x93_[\x82\x81\x10a+\xE8WPPPP\x90PV[a,\ra,\x07a+\xF9\x83\x86\x86a-\xCBV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[\x92a,\x18\x82\x86a)\x94V[Q\x80Q`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x87\x16`\x04\x82\x01R\x91\x97\x91\x90` \x82`$\x81\x8DZ\xFA\x91\x82\x15a\t\xE3Wa,^\x92c\xFF\xFF\xFF\xFF\x91_\x91a-\x87W[P\x16\x14a-\xECV[_\x97\x88[\x88Q\x8A\x10\x15a-\x1BW`\x01\x90a-\x13a,\x8Ba,~\x8D\x8Da)\x94V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91a,\xEEa,\xB1a,\xAC\x85`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[a)\xB4V[\x91a,\xD9a,\xD4\x8Da,\xC6a\x05\x98\x87QaP=V[`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a.\x02V[\x85\x80`\xA0\x1B\x03\x16\x85\x80`\xA0\x1B\x03\x85\x16\x11a.\x18V[a-\x0Ca-\x05a,\xFD\x8Aa.BV[\x8A\x8A\x8Da.bV[6\x91a\x07\x9FV[\x90\x83a6\xCAV[\x99\x01\x98a,bV[P\x96P\x96P\x92\x90`\x01\x91\x94\x92\x94Ca->\x82`\xFF\x16_R`\x9B` R`@_ \x90V[U\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4`\xFF`@Q\x92\x16\x91\x80a-xC\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA2\x01\x94\x93\x94\x92\x90\x92a+\xD9V[a-\xA8\x91P` =\x81\x11a-\xAEW[a-\xA0\x81\x83a\x03\xF2V[\x81\x01\x90a-\xD7V[_a,VV[P=a-\x96V[\x15a-\xBCWV[c\xAA\xAD\x13\xF7`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a\x14>W\x01\x90V[\x90\x81` \x91\x03\x12a\x04[WQa\x07\xF0\x81a\x05\xE1V[\x15a-\xF3WV[c\x8EZ\xEE\xE7`\xE0\x1B_R`\x04_\xFD[\x15a.\tWV[c\xD0S\xAA!`\xE0\x1B_R`\x04_\xFD[\x15a.\x1FWV[c\xBAP\xF9\x11`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a.PWV[a..V[\x91\x90\x82\x01\x80\x92\x11a.PWV[\x90\x93\x92\x93\x84\x83\x11a\x04[W\x84\x11a\x04[W\x81\x01\x92\x03\x90V[\x15a.\x81WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[\x97\x95\x93\x91a/'\x97\x95\x93\x91_T\x99a/\r`\xFF\x8C`\x08\x1C\x16\x15\x15\x15\x80\x9C\x81a/\xA1W[\x81\x15a/\x81W[Pa.zV[\x8Aa/\x1E`\x01`\xFF\x19_T\x16\x17_UV[a/jWa0+V[a/-WV[a/;a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1V[a/|a\x01\0a\xFF\0\x19_T\x16\x17_UV[a0+V[0;\x15\x91P\x81a/\x93W[P_a/\x07V[`\xFF\x16`\x01\x14\x90P_a/\x8CV[`\x01`\xFF\x82\x16\x10\x91Pa/\0V[`\x9CT`\x01`@\x1B\x81\x10\x15a\x03\xD2W`\x01\x81\x01`\x9CU`\x9CT\x81\x10\x15a\x14>W`\x9C_R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x02\x11\x15a\x08\x8EWV[Q`\x02\x81\x10\x15a\x08\x8EW\x90V[\x92a\t\xAFa\x0C\x01\x92a\x0B\xD4a0j\x96\x9C\x9B\x9A\x99\x98\x9C\x8D\x89Q\x90Q\x80\x91\x14\x90\x81a1\x8CW[P\x80a1\x81W[\x80a1vW[a0e\x90a-\xB5V[aD\xC3V[a0\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a/\xAFV[a0\xCE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a/\xAFV[a1\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a/\xAFV[_[\x81Q\x81\x10\x15a1mW\x80a1ga1\x1B`\x01\x93\x85a)\x94V[Qa16a1)\x84\x8Ca)\x94V[Q`\x01`\x01``\x1B\x03\x16\x90V[a1@\x84\x88a)\x94V[Qa1Sa1N\x86\x8Ba)\x94V[a0\x1EV[\x91a1aa\x1F\xD4\x87\x8Da)\x94V[\x93a=\rV[\x01a1\x02V[PPPPP\x90PV[P\x8AQ\x8CQ\x14a0\\V[P\x89Q\x8BQ\x14a0VV[\x90P\x8AQ\x14_a0OV[\x91\x94\x93\x90\x92`@Q\x92` \x84\x01\x94`\xE0\x85\x01\x91\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87R`\x01\x80`\xA0\x1B\x03\x16`@\x86\x01R``\x85\x01R`\xC0`\x80\x85\x01R\x86Q\x80\x91R` a\x01\0\x85\x01\x97\x01\x90_[\x81\x81\x10a2$WPPPa\x07\xF0\x94\x95a+I\x92\x84\x92`\xA0\x84\x01R`\xC0\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xF2V[\x82Q\x80Q`\xFF\x16\x8AR` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x8B\x01R`@\x90\x99\x01\x98\x90\x92\x01\x91`\x01\x01a1\xF7V[\x94a2\x9Ea2\x95a2\xB0\x95\x98\x94\x93\x96\x98a2ra\x05.`\x01\x80\x80T\x16\x14\x90V[a2\x89a2\x84a\x10\xEA`\xFF`\xA1T\x16\x90V[a3\xBDV[a\"/\x88\x8BQ\x14a-\xB5V[\x91\x88\x833aF\xBEV[a2\xA96\x86\x88a\x07\x9FV[\x903aI<V[\x92_[\x82\x81\x10a2\xC1WPPPPPV[\x80a2\xF1a2\xECa2\xDBa,\x07a+\xF9`\x01\x96\x89\x8Ba-\xCBV[`\xFF\x16_R`\x97` R`@_ \x90V[a3\xD3V[a2\xFFa\x1F\xD4\x83\x89Qa)\x94V[c\xFF\xFF\xFF\xFFa3\x1Ea3\x15\x84Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[\x91\x16\x11a3-W[P\x01a2\xB3V[a3y\x90a3Ba,\x07a+\xF9\x85\x89\x8Ba-\xCBV[a3Sa1)\x85`@\x8C\x01Qa)\x94V[\x90a3ea1)\x86` \x8D\x01Qa)\x94V[\x90a3p\x86\x89a)\x94V[Q\x923\x91aM\x07V[a3\xB7a3\x9A` a3\x8B\x84\x87a)\x94V[Q\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a3\xB1a-\x05a3\xA9\x85a.BV[\x85\x89\x8Ba.bV[\x90aB\rV[_a3&V[\x15a3\xC4WV[c\x0B\x880o`\xE0\x1B_R`\x04_\xFD[\x90`@Qa3\xE0\x81a\x03\xD7V[`@a\xFF\xFF\x82\x94Tc\xFF\xFF\xFF\xFF\x81\x16\x84R\x81\x81` \x1C\x16` \x85\x01R`0\x1C\x16\x91\x01RV[\x15a4\x0CWV[c\xFD,\x1FM`\xE0\x1B_R`\x04_\xFD[\x15a4\"WV[c#\xD8q\xA5`\xE0\x1B_R`\x04_\xFD[\x90a4;\x82a\x07\x84V[a4H`@Q\x91\x82a\x03\xF2V[\x82\x81R\x80\x92a4Y`\x1F\x19\x91a\x07\x84V[\x01\x90` 6\x91\x017V[\x90\x81Q\x81\x10\x15a\x14>W\x01` \x01\x90V[\x15a4{WV[c<\xB8\x9C\x97`\xE0\x1B_R`\x04_\xFD[\x91\x90\x82`@\x91\x03\x12a\x04[W`@Qa4\xA2\x81a\x03\xB7V[` \x80\x82\x94\x80Q\x84R\x01Q\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W`@Q\x91a4\xCD`@\x84a\x03\xF2V[\x82\x90`@\x81\x01\x92\x83\x11a\x04[W\x90[\x82\x82\x10a4\xE9WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a4\xDCV[\x91\x90\x91\x80\x83\x03\x90a\x01 \x82\x12a\x04[W\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x04[W\x81\x01\x84`\x1F\x82\x01\x12\x15a\x04[W\x80Qa51\x81a\x07\x84V[\x91a5?`@Q\x93\x84a\x03\xF2V[\x81\x83R\x86` \x83\x83\x01\x01\x11a\x04[W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x93a\x01\0`\x1F\x19\x84\x01\x12a\x04[W`\x80`@Q\x93a5{\x85a\x03\xD7V[a5\x88\x83` \x86\x01a4\x8AV[\x85Ra5\x97\x83``\x86\x01a4\x8AV[` \x86\x01R`\x9F\x19\x01\x12a\x04[Wa5\xCD\x90`\xE0`@Q\x93a5\xB8\x85a\x03\xB7V[a5\xC5\x83`\xA0\x83\x01a4\xB2V[\x85R\x01a4\xB2V[` \x82\x01R`@\x82\x01R\x90V[_\x19\x81\x14a.PW`\x01\x01\x90V[_\x81\x80[a6bWPa5\xFE\x90a\xFF\xFF\x16a41V[__[\x82Q\x82\x10\x80a6WW[\x15a6PW`\x01\x81\x1B\x84\x16a6)W[a6$\x90a5\xDAV[a6\x01V[\x90`\x01a6$\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa6F\x82\x87a4cV[S\x01\x91\x90Pa6\x1BV[PP\x90P\x90V[Pa\x01\0\x81\x10a6\x0BV[_\x19\x81\x01\x81\x81\x11a.PWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a.PW`\x01\x01\x90\x80a5\xECV[\x90\x81` \x91\x03\x12a\x04[WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x04[W\x90V[a\x07\xF0\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a)\xF0V[\x91\x90`\x01` \x82\x01Qa6\xDC\x81a\x11\xFAV[a6\xE5\x81a\x11\xFAV[\x03a7\x9BWQ`@Qc3V\x7F\x7F`\xE1\x1B\x81R\x91` \x91\x83\x91\x82\x91a7\x0F\x91\x90\x87`\x04\x85\x01a6\xA6V[\x03\x81_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x90\x81\x15a\t\xE3W_\x91a7lW[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81a7`WPPV[a3\xB1a\x04\"\x92a5\xE8V[a7\x8E\x91P` =` \x11a7\x94W[a7\x86\x81\x83a\x03\xF2V[\x81\x01\x90a6\x87V[_a7LV[P=a7|V[PPPV[\x15a7\xA7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from before block`\x84\x82\x01Re':\xB6\xB12\xB9`\xD1\x1B`\xA4\x82\x01R`\xC4\x90\xFD[`dT`\x01`\x01`\xA0\x1B\x03\x163\x03a8[WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[_\x19`\x01U`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`\x01U`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`\x9DUV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`\x9EUV[a9\xC9aP\x81V[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra9\xF4`b\x82a\x03\xF2V[Q\x90 \x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_Q` aX\xAD_9_Q\x90_R\x90a:%a*\xD8V[P_\x91\x90\x06` `\xC0\x83[a;%W_\x93_Q` aX\xAD_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa:[\x85\x82a\x03\xF2V[\x846\x827\x84\x81\x85`@Qa:o\x82\x82a\x03\xF2V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aX\xAD_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a;*Wa:\xD9\x90aU\xB7V[Q\x91a;%W_Q` aX\xAD_9_Q\x90_R\x82\x80\t\x14a;\x10WP_Q` aX\xAD_9_Q\x90_R`\x01_\x94\x08\x92\x93a:0V[\x92\x93PPa;\x1Ca\x04\x13V[\x92\x83R\x82\x01R\x90V[a9\xFAV[\xFE[`\xFF`\x01\x91\x16\x01\x90`\xFF\x82\x11a.PWV[`@\x80Q\x90\x91\x90a;O\x83\x82a\x03\xF2V[`\x01\x81R\x91`\x1F\x19\x01\x82_[\x82\x81\x10a;gWPPPV[` \x90`@Qa;v\x81a\x03\xB7V[_\x81R``\x83\x82\x01R\x82\x82\x85\x01\x01R\x01a;[V[\x90a;\x95\x82a\x043V[a;\xA2`@Q\x91\x82a\x03\xF2V[\x82\x81R\x80\x92a4Y`\x1F\x19\x91a\x043V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a;\xE5WPPPPP\x90V[\x90\x91\x92\x93\x94`?\x19\x82\x82\x03\x01\x83R\x85Q` ``\x81`@\x85\x01\x93c\xFF\xFF\xFF\xFF\x81Q\x16\x86R\x01Q\x93`@\x83\x82\x01R\x84Q\x80\x94R\x01\x92\x01\x90_\x90[\x80\x82\x10a<=WPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a;\xD6V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x90\x91\x01\x90a<\x1EV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a<|WPPP\x90V[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x86\x01R`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a<oV[\x90a\x07\xF0\x94\x93`\x01`\x01``\x1B\x03`\x80\x94`\xFFc\xFF\xFF\xFF\xFF\x94\x16\x85R\x16` \x84\x01R\x16`@\x82\x01R\x81``\x82\x01R\x01\x90a<_V[`\x01`\x01``\x1B\x03a\x07\xF0\x94\x93`\xFF``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a<_V[\x93\x90\x91\x92a=\x1D`\x96T`\xFF\x16\x90V[\x94a=D`\xFF\x87\x16\x91a=2`\xC0\x84\x10a4tV[a=>a\x0E\x19\x89a;,V[\x87a@\xFBV[`\xA1T`\xFF\x16\x80a@SW[a?fW[Pa=_\x81a0\x14V[\x80a>\xC9WPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x04[Wa=\xBC\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c:\xEA\x0B\x9D`\xE1\x1B\x84R\x8A`\x04\x85\x01a<\xE5V[\x03\x92Z\xF1\x80\x15a\t\xE3Wa>\xB5W[P[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xE3Wa>\xA1W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x90\x92\x16`\x04\x83\x01R_\x90\x82\x90\x81\x83\x81`$\x81\x01[\x03\x92Z\xF1\x80\x15a\t\xE3Wa>\x93WPV[\x80a\x0F\x8B_a\x04\"\x93a\x03\xF2V[\x80a\x0F\x8B_a>\xAF\x93a\x03\xF2V[_a>-V[\x80a\x0F\x8B_a>\xC3\x93a\x03\xF2V[_a=\xCBV[\x80a>\xD8`\x01\x92\x95\x93\x95a0\x14V[\x14a>\xE6W[PPPa=\xCDV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[Wa?<\x93_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x06b\xD3\xE1`\xE5\x1B\x85R\x8B`\x04\x86\x01a<\xB0V[\x03\x92Z\xF1\x80\x15a\t\xE3Wa?RW[\x80\x80a>\xDEV[\x80a\x0F\x8B_a?`\x93a\x03\xF2V[_a?KV[\x95\x92\x90\x94\x91a?sa;>V[\x95a?~\x86Qa;\x8BV[\x96_[\x87Q\x81\x10\x15a?\xADW\x80a?\xA7a?\x9Da\x0F\xEC`\x01\x94\x8Ca)\x94V[a\x10\x04\x83\x8Da)\x94V[\x01a?\x81V[P\x91\x93\x96\x97\x90\x92\x94\x97a?\xC1a\x10)a\x04\x13V[` \x82\x01Ra?\xCF\x82a)\x87V[Ra?\xD9\x81a)\x87V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x84}cO`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a@0\x90`\x04\x83\x01a;\xB3V[\x03\x92Z\xF1\x80\x15a\t\xE3W\x15a=UW\x80a\x0F\x8B_a@M\x93a\x03\xF2V[_a=UV[Pa@oa\x10\xEAa\x10\xE3\x88`\xFF\x16_R`\xA2` R`@_ \x90V[a=PV[\x90`\x01a@\x82`\xFF\x93aR\x19V[\x92\x83\x92\x16\x1B\x11\x15a@\x90W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x90\xFD[aA\xB9`\xFF\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x92\x16\x92\x83_R`\x97` R`@_ aAQc\xFF\xFF\xFF\xFF\x83Q\x16\x82\x90c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[` \x82\x01Q\x81Te\xFF\xFF\0\0\0\0g\xFF\xFF\0\0\0\0\0\0`@\x86\x01Q`0\x1B\x16\x92` \x1B\x16\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x17\x17\x90U`@Q\x91\x82\x91\x82\x91\x90\x91`@a\xFF\xFF\x81``\x84\x01\x95c\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[\x03\x90\xA2V[\x15aA\xC5WV[ch\xB6\xA8u`\xE1\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra\x07\xF0\x92\x91\x01\x90a)\xF0V[`@\x90a\x07\xF0\x93\x92\x81R\x81` \x82\x01R\x01\x90a)\xF0V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x99` R`@\x90 \x90`\x01\x82T\x92\x01aBJ`\x01aB;\x83T`\xFF\x16\x90V[aBD\x81a\x11\xFAV[\x14a)\xDAV[aB\xAAaBea\x05\x98aB_`\x96T`\xFF\x16\x90V[\x87a@tV[aBn\x85aP=V[`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x90aB\x85\x82\x15\x15aA\xBEV[aB\x9B\x82\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x83\x14a.\x02V[\x90\x19\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[aB\xB4\x81\x85aS\x0EV[`\x01`\x01`\xC0\x1B\x03\x16\x15aD\x08W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W\x83_\x91aC\x19\x93\x83`@Q\x80\x96\x81\x95\x82\x94c\xF4\xE2O\xE5`\xE0\x1B\x84R`\x04\x84\x01aA\xD4V[\x03\x92Z\xF1\x80\x15a\t\xE3WaC\xF4W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W_`@Q\x80\x92c\xBD)\xB8\xCD`\xE0\x1B\x82R\x81\x83\x81aC{\x89\x89`\x04\x84\x01aA\xF6V[\x03\x92Z\xF1\x80\x15a\t\xE3WaC\xE0W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x04[Wa>\x82\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xBD)\xB8\xCD`\xE0\x1B\x84R`\x04\x84\x01aA\xF6V[\x80a\x0F\x8B_aC\xEE\x93a\x03\xF2V[_aC\x8AV[\x80a\x0F\x8B_aD\x02\x93a\x03\xF2V[_aC(V[\x80T`\xFF\x19\x16`\x02\x17\x90U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\t\xE3WaD\xAFW[P\x81`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4_\x80\xA3_aB\xC3V[\x80a\x0F\x8B_aD\xBD\x93a\x03\xF2V[_aDyV[`d\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x90\x81` \x91\x03\x12a\x04[WQ\x90V[\x90_\x90[`\x02\x82\x10aE+WPPPV[` \x80`\x01\x92\x85Q\x81R\x01\x93\x01\x91\x01\x90\x91aE\x1EV[a\x01 \x90aE\xAB` `@a\x04\"\x96\x98\x97\x95\x98a\x01`\x85\x01\x99`\x01\x80`\xA0\x1B\x03\x16\x85RaE{\x83\x86\x01\x82Q` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\x80\x83\x01Q\x80Q``\x87\x01R` \x01Q`\x80\x86\x01R\x01QaE\x9F`\xA0\x85\x01\x82QaE\x1AV[\x01Q`\xE0\x83\x01\x90aE\x1AV[\x01\x90` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x90\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x83`$\x81\x84Z\xFA\x92\x83\x15a\t\xE3W_\x93aF\x9DW[P\x82\x15aF#WPP\x90P\x90V[` \x92PaFS\x93_aF5\x84a*\xF0V[`@Qc\x17\xEF9\xCB`\xE3\x1B\x81R\x96\x87\x95\x86\x94\x85\x93\x91`\x04\x85\x01aEAV[\x03\x92Z\xF1\x90\x81\x15a\t\xE3W_\x91aFnW[P\x80_\x80a6PV[aF\x90\x91P` =` \x11aF\x96W[aF\x88\x81\x83a\x03\xF2V[\x81\x01\x90aE\x0BV[_aFeV[P=aF~V[aF\xB7\x91\x93P` =` \x11aF\x96WaF\x88\x81\x83a\x03\xF2V[\x91_aF\x15V[\x91\x92\x90` \x82\x01\x92\x83Q_R`\x9A` R`\xFF`@_ T\x16aG9W`@\x83\x01\x80QB\x11aG*Wa\x04\"\x95aG\"\x93\x86Q_R`\x9A` RaG\x0C`@_ `\x01`\xFF\x19\x82T\x16\x17\x90UV[`\x9DT`\x01`\x01`\xA0\x1B\x03\x16\x96Q\x92Q\x93a1\x97V[\x90Q\x91aT(V[c\x08\x19\xBD\xCD`\xE0\x1B_R`\x04_\xFD[co\xBE\xFE\xC3`\xE1\x1B_R`\x04_\xFD[`@Q\x90aGU\x82a\x03\xD7V[```@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x15aGnWV[c\x13\xCAFW`\xE0\x1B_R`\x04_\xFD[\x15aG\x84WV[c\x0Ch\x16\xCD`\xE0\x1B_R`\x04_\xFD[\x15aG\x9AWV[c\x19hg}`\xE1\x1B_R`\x04_\xFD[`\x01` \x91\x83Q\x81U\x01\x91\x01Q`\x03\x81\x10\x15a\x08\x8EW`\xFF\x80\x19\x83T\x16\x91\x16\x17\x90UV[\x90`\x01\x80`\xA0\x1B\x03\x16\x81R`@` \x82\x01R`\x80`@aG\xF8\x84Q``\x83\x86\x01R`\xA0\x85\x01\x90a)\xF0V[\x93` \x81\x01Q``\x85\x01R\x01Q\x91\x01R\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x81QaH\"\x81a\x043V[\x92aH0`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04[W` \x01\x90[\x82\x82\x10aHXWPPP\x90V[` \x80\x91\x83QaHg\x81a\x0C\xE8V[\x81R\x01\x91\x01\x90aHKV[\x91\x90\x91`@\x81\x84\x03\x12a\x04[W\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x04[W\x83aH\x9B\x91\x83\x01aH\x0BV[\x92` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x04[Wa\x07\xF0\x92\x01aH\x0BV[` \x81\x83\x03\x12a\x04[W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04[W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x04[W\x81QaH\xEC\x81a\x043V[\x92aH\xFA`@Q\x94\x85a\x03\xF2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04[W` \x01\x90[\x82\x82\x10aI\"WPPP\x90V[` \x80\x91\x83QaI1\x81a\x05\xE1V[\x81R\x01\x91\x01\x90aI\x15V[\x90\x91\x92\x93\x82\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAAaJ\x07aImaGHV[\x97aI\xFBaI\x89a\x05\x98aI\x83`\x96T`\xFF\x16\x90V[\x8Ba@tV[aI\x92\x86aP=V[`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x90aI\xA9\x82\x15\x15aGgV[`\x01\x80`\xC0\x1B\x03\x16aI\xC3aI\xBE\x82\x84\x16\x15\x90V[aG}V[`\x01`\x01`\xA0\x1B\x03\x89\x16_\x90\x81R`\x9F` R`@\x90 aI\xF4\x90aI\xED\x90[T`\xA0T\x90a.UV[B\x11aG\x93V[\x17\x85aS\x0EV[`@Q\x91\x82\x91\x82a*\x14V[\x03\x90\xA2`\x01aJ2\x81aJ*\x85`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[\x01T`\xFF\x16\x90V[aJ;\x81a\x11\xFAV[\x03aK\xCFW[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W_`@Q\x80\x92c\x1F\xD9<\xA9`\xE1\x1B\x82R\x81\x83\x81aJ\x94\x8A\x89`\x04\x84\x01aA\xD4V[\x03\x92Z\xF1\x80\x15a\t\xE3W\x84\x92_\x92\x85\x92aK\xBBW[PaJ\xC8`@Q\x96\x87\x93\x84\x93c%PGw`\xE0\x1B\x85R`\x04\x85\x01a6\xA6V[\x03\x81\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x91\x82\x15a\t\xE3WaK2\x93_\x93\x84\x91\x85\x91aK\x97W[P`@\x87\x01R` \x86\x01R`@Q\x93\x84\x92\x83\x92b\xBF\xF0M`\xE0\x1B\x84R`\x04\x84\x01aA\xF6V[\x03\x81\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x90\x81\x15a\t\xE3W_\x91aKuW[P\x81R\x90V[aK\x91\x91P=\x80_\x83>aK\x89\x81\x83a\x03\xF2V[\x81\x01\x90aH\xB9V[_aKoV[\x90PaK\xB5\x91P=\x80\x86\x83>aK\xAD\x81\x83a\x03\xF2V[\x81\x01\x90aHrV[_aK\rV[\x80a\x0F\x8B\x85aK\xC9\x93a\x03\xF2V[_aJ\xA9V[aL\0aK\xDAa\x04\x13V[\x84\x81R`\x01` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\x99` R`@\x90 aG\xA9V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W`@Qc\x99&\xEE}`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90aLW\x90\x89`\x04\x84\x01aG\xCDV[\x03\x92Z\xF1\x80\x15a\t\xE3WaL\x9CW[P\x81`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE_\x80\xA3_aJAV[\x80a\x0F\x8B_aL\xAA\x93a\x03\xF2V[_aLfV[\x15aL\xB7WV[cV\x16\x8BA`\xE1\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x04[WQa\x07\xF0\x81a\x0C\xE8V[\x15aL\xE2WV[cLD\x99]`\xE0\x1B_R`\x04_\xFD[\x15aL\xF8WV[c\xB1\x87\xE8i`\xE0\x1B_R`\x04_\xFD[` \x91\x92aMdaMWaM\x8F\x98\x96\x97aMPaM,\x87\x83\x01Q`\x01\x80`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x99` R`@\x90 T\x96\x90\x91\x16\x14\x15aL\xB0V[Q`\xFF\x16\x90V[`\xFF\x80\x85\x16\x91\x16\x14a-\xECV[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x90\x91\x16`$\x82\x01R\x94\x85\x90\x81\x90`D\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\t\xE3Wa\x04\"\x94_\x94aN\x13W[P\x82aN\x0B\x92aN\x06aM\xF2\x93`\x01`\x01``\x1B\x03aM\xFEaM\xF2\x82\x99\x8BaTlV[`\x01`\x01``\x1B\x03\x16\x90V[\x91\x16\x11aL\xDBV[aT\x8FV[\x91\x16\x10aL\xF1V[aM\xF2\x91\x94P\x92aN\x0B\x92aN\x06aNJ`\x01`\x01``\x1B\x03\x96` =` \x11aNVW[aNB\x81\x83a\x03\xF2V[\x81\x01\x90aL\xC6V[\x96\x93PP\x92P\x92aM\xCFV[P=aN8V[\x81\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAAaN\xFFaN\x8AaGHV[\x96aI\xFBaN\xA6a\x05\x98aN\xA0`\x96T`\xFF\x16\x90V[\x8Aa@tV[aN\xAF\x86aP=V[`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x90aN\xC6\x82\x15\x15aGgV[`\x01\x80`\xC0\x1B\x03\x16aN\xDBaI\xBE\x82\x84\x16\x15\x90V[`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9F` R`@\x90 aI\xF4\x90aI\xED\x90aI\xE3V[\x03\x90\xA2`\x01aO\"\x81aJ*\x84`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[aO+\x81a\x11\xFAV[\x03aO\x83W[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04[W_`@Q\x80\x92c\x1F\xD9<\xA9`\xE1\x1B\x82R\x81\x83\x81aJ\x94\x8A\x89`\x04\x84\x01aA\xD4V[aO\xB4aO\x8Ea\x04\x13V[\x83\x81R`\x01` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x99` R`@\x90 aG\xA9V[aO1V[\x91\x90\x80Q\x90aO\xC7\x82a\x043V[\x91aO\xD5`@Q\x93\x84a\x03\xF2V[\x80\x83RaO\xE4`\x1F\x19\x91a\x043V[\x016` \x84\x017_[\x81Q\x81\x10\x15aP(W\x80aP\x0FaP\x06`\x01\x93\x85a)\x94V[Q\x87`\x98aT\xADV[c\xFF\xFF\xFF\xFFaP\x1E\x83\x87a)\x94V[\x91\x16\x90R\x01aO\xEDV[P\x90\x92PPV[_\x19\x81\x01\x91\x90\x82\x11a.PWV[\x80_R`\x98` R`@_ T\x90\x81\x15_\x14aPYWPP_\x90V[_R`\x98` R`@_ \x90_\x19\x81\x01\x90\x81\x11a.PWaPy\x91a\x14)V[PT`@\x1C\x90V[0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80aQnW[\x15aP\xDCW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra9\xF4`\xC0\x82a\x03\xF2V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14aP\xB3V[\x15aQ\x9EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90a\x01\0\x82Q\x11aR\x97W\x81Q\x15aR\x92WaRUaRKa,\x07aR=\x85a)\x87V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xFF`\x01\x91\x16\x1B\x90V[`\x01\x90[\x83Q\x82\x10\x15aR\x8DW`\x01\x90aRxaRKa,\x07aR=\x86\x89a4cV[\x90aR\x84\x81\x83\x11aQ\x97V[\x17\x91\x01\x90aRYV[\x92PPV[_\x91PV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R\xFD[\x90aS!\x82_R`\x98` R`@_ \x90V[T\x80aSnWPaS=a\x04\"\x92_R`\x98` R`@_ \x90V[aSiaSHa\x04$V[Cc\xFF\xFF\xFF\xFF\x16\x81R\x92[_` \x85\x01R`\x01`\x01`\xC0\x1B\x03\x16`@\x84\x01RV[aV\x03V[\x91aS\x99c\xFF\xFF\xFF\xFF\x93aS\x93aS\x8D\x84_R`\x98` R`@_ \x90V[\x91aP/V[\x90a\x14)V[P\x90aS\xA9\x82Tc\xFF\xFF\xFF\xFF\x16\x90V[C\x85\x16\x94\x16\x84\x03aS\xD4WPa\x04\"\x92P\x90`\x01`\x01`@\x1B\x03\x82T\x91\x81\x19\x90`@\x1B\x16\x91\x16\x17\x90UV[\x81Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x85\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x91Ua\x04\"\x92\x91\x90aSi\x90aT\x10\x90_R`\x98` R`@_ \x90V[\x91aSSaT\x1Ca\x04$V[c\xFF\xFF\xFF\xFF\x90\x95\x16\x85RV[\x90aT3\x92\x91aV\xCAV[\x15aT:WV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a.PWV[aT\x8A`\x01`\x01``\x1B\x03\x91a\xFF\xFF` a'\x10\x95\x01Q\x16\x90aTIV[\x16\x04\x90V[aT\x8A`\x01`\x01``\x1B\x03\x91a\xFF\xFF`@a'\x10\x95\x01Q\x16\x90aTIV[\x91\x90\x81_R\x82` R`@_ T\x92_[\x84\x81\x10aUVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[\x80\x85\x03\x85\x81\x11a.PWa3\x15aUl\x91aP/V[aU\x95aU\x8A\x82aU\x85\x88\x87\x90_R` R`@_ \x90V[a\x14)V[PTc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x11\x15aU\xAEWP`\x01\x01aT\xBEV[\x94PPPPP\x90V[\x15aU\xBEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x03\xD2WaV \x91`\x01\x82\x01\x81Ua\x14)V[aV^W\x81Q` \x80\x84\x01Q`@\x94\x85\x01Qc\xFF\xFF\xFF\xFF\x90\x93\x16\x91\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x90\x93\x1B\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[`\x05\x11\x15a\x08\x8EWV[=\x15aV\xA5W=\x90aV\x8C\x82a\x07\x84V[\x91aV\x9A`@Q\x93\x84a\x03\xF2V[\x82R=_` \x84\x01>V[``\x90V[\x90\x81` \x91\x03\x12a\x04[WQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x04[W\x90V[\x91\x90\x91aV\xD7\x82\x84aW\x95V[aV\xE0\x81aVqV[\x15\x90\x81aW\x7FW[PaWwW_\x92aW\x15aW#\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01aA\xF6V[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xF2V[Q\x91Z\xFAaW/aV{V[\x81aWkW[\x81aW>WP\x90V[\x80Qc\x0B\x13]?`\xE1\x1B\x92P`\x01`\x01`\xE0\x1B\x03\x19\x91aWf\x91\x81\x01` \x90\x81\x01\x91\x01aV\xAAV[\x16\x14\x90V[\x80Q` \x14\x91PaW5V[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_aV\xE8V[\x81Q`A\x81\x03aW\xC1WP\x90aW\xBD\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aX\x03V[\x90\x91V[`@\x03aW\xFAW`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a.PWaW\xBD\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90aX\x03V[PP_\x90`\x02\x90V[\x92\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11aX\xA1W`\xFF\x16`\x1B\x81\x14\x15\x80aX\x96W[aX\x8BW` \x93_\x93`@Q\x93\x84\x93`\x80\x85\x01\x93\x85R\x87\x85\x01R`@\x84\x01R``\x83\x01R\x83\x80R\x03\x90`\x01Z\xFA\x15a\t\xE3W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aX\x83W\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x81\x14\x15aX;V[PPPP_\x90`\x03\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xB9\x9F\\\xDF\x10\xF1\xAF\xBE\xC6\n\x96\xA4g7j\xCAx\xE0\x01\n4\xF2k~\xD3-\xB3\xA8\"WYmdsolcC\0\x08\x1B\x003",
    );
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
    /**Custom error with signature `OperatorSetsEnabled()` and selector `0x0b88306f`.
    ```solidity
    error OperatorSetsEnabled();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
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
        impl ::core::convert::From<OperatorSetsEnabled> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSetsEnabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSetsEnabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorSetsEnabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorSetsEnabled()";
            const SELECTOR: [u8; 4] = [11u8, 136u8, 48u8, 111u8];
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
    /**Custom error with signature `OperatorSetsNotSupported()` and selector `0xfd2c1f4d`.
    ```solidity
    error OperatorSetsNotSupported();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSetsNotSupported {}
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
        impl ::core::convert::From<OperatorSetsNotSupported> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSetsNotSupported) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSetsNotSupported {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorSetsNotSupported {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorSetsNotSupported()";
            const SELECTOR: [u8; 4] = [253u8, 44u8, 31u8, 77u8];
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
    event OperatorSetParamsUpdated(uint8 indexed quorumNumber, IRegistryCoordinator.OperatorSetParam operatorSetParams);
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
            <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            type DataTuple<'a> = (IRegistryCoordinator::OperatorSetParam,);
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
                    <IRegistryCoordinator::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
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
    constructor(address _serviceManager, address _stakeRegistry, address _blsApkRegistry, address _indexRegistry, address _avsDirectory, address _pauserRegistry);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _serviceManager: alloy::sol_types::private::Address,
        pub _stakeRegistry: alloy::sol_types::private::Address,
        pub _blsApkRegistry: alloy::sol_types::private::Address,
        pub _indexRegistry: alloy::sol_types::private::Address,
        pub _avsDirectory: alloy::sol_types::private::Address,
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
                        value._avsDirectory,
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
                        _avsDirectory: tuple.4,
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
                        &self._avsDirectory,
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
    function calculateOperatorChurnApprovalDigestHash(address registeringOperator, bytes32 registeringOperatorId, IRegistryCoordinator.OperatorKickParam[] memory operatorKickParams, bytes32 salt, uint256 expiry) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorChurnApprovalDigestHashCall {
        pub registeringOperator: alloy::sol_types::private::Address,
        pub registeringOperatorId: alloy::sol_types::private::FixedBytes<32>,
        pub operatorKickParams: alloy::sol_types::private::Vec<
            <IRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
        >,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)`](calculateOperatorChurnApprovalDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorChurnApprovalDigestHashReturn {
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
                alloy::sol_types::sol_data::Array<IRegistryCoordinator::OperatorKickParam>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    <IRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<IRegistryCoordinator::OperatorKickParam>,
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
                        IRegistryCoordinator::OperatorKickParam,
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
    function createSlashableStakeQuorum(IRegistryCoordinator.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory strategyParams, uint32 lookAheadPeriod) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSlashableStakeQuorumCall {
        pub operatorSetParams:
            <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        pub strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
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
                IRegistryCoordinator::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
                IRegistryCoordinator::OperatorSetParam,
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
                    <IRegistryCoordinator::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
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
    function createTotalDelegatedStakeQuorum(IRegistryCoordinator.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory strategyParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTotalDelegatedStakeQuorumCall {
        pub operatorSetParams:
            <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
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
                IRegistryCoordinator::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
                IRegistryCoordinator::OperatorSetParam,
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
                    <IRegistryCoordinator::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
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
        pub operator: alloy::sol_types::private::Address,
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
    /**Function with signature `ejectOperator(address,bytes)` and selector `0x6e3b17db`.
    ```solidity
    function ejectOperator(address operator, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectOperatorCall {
        pub operator: alloy::sol_types::private::Address,
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
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getCurrentQuorumBitmap(bytes32)`](getCurrentQuorumBitmapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentQuorumBitmapReturn {
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
    function getOperator(address operator) external view returns (IRegistryCoordinator.OperatorInfo memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperator(address)`](getOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorReturn {
        pub _0: <IRegistryCoordinator::OperatorInfo as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (IRegistryCoordinator::OperatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRegistryCoordinator::OperatorInfo as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
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
            type ReturnTuple<'a> = (IRegistryCoordinator::OperatorInfo,);
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
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getOperatorFromId(bytes32)`](getOperatorFromIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromIdReturn {
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
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorId(address)`](getOperatorIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorIdReturn {
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
    function getOperatorSetParams(uint8 quorumNumber) external view returns (IRegistryCoordinator.OperatorSetParam memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetParamsCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getOperatorSetParams(uint8)`](getOperatorSetParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetParamsReturn {
        pub _0: <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (IRegistryCoordinator::OperatorSetParam,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
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
            type ReturnTuple<'a> = (IRegistryCoordinator::OperatorSetParam,);
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
    function getOperatorStatus(address operator) external view returns (IRegistryCoordinator.OperatorStatus);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStatusCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorStatus(address)`](getOperatorStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStatusReturn {
        pub _0: <IRegistryCoordinator::OperatorStatus as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (IRegistryCoordinator::OperatorStatus,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRegistryCoordinator::OperatorStatus as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
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
            type ReturnTuple<'a> = (IRegistryCoordinator::OperatorStatus,);
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
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub blockNumber: u32,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)`](getQuorumBitmapAtBlockNumberByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapAtBlockNumberByIndexReturn {
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
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapHistoryLength(bytes32)`](getQuorumBitmapHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapHistoryLengthReturn {
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
        pub blockNumber: u32,
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])`](getQuorumBitmapIndicesAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapIndicesAtBlockNumberReturn {
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
    function getQuorumBitmapUpdateByIndex(bytes32 operatorId, uint256 index) external view returns (IRegistryCoordinator.QuorumBitmapUpdate memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapUpdateByIndexCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapUpdateByIndex(bytes32,uint256)`](getQuorumBitmapUpdateByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapUpdateByIndexReturn {
        pub _0: <IRegistryCoordinator::QuorumBitmapUpdate as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (IRegistryCoordinator::QuorumBitmapUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRegistryCoordinator::QuorumBitmapUpdate as alloy::sol_types::SolType>::RustType,
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
            type ReturnTuple<'a> = (IRegistryCoordinator::QuorumBitmapUpdate,);
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
    /**Function with signature `initialize(address,address,address,uint256,(uint32,uint16,uint16)[],uint96[],(address,uint96)[][],uint8[],uint32[])` and selector `0x7fc3f886`.
    ```solidity
    function initialize(address _initialOwner, address _churnApprover, address _ejector, uint256 _initialPausedStatus, IRegistryCoordinator.OperatorSetParam[] memory _operatorSetParams, uint96[] memory _minimumStakes, IStakeRegistry.StrategyParams[][] memory _strategyParams, StakeType[] memory _stakeTypes, uint32[] memory _lookAheadPeriods) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _initialOwner: alloy::sol_types::private::Address,
        pub _churnApprover: alloy::sol_types::private::Address,
        pub _ejector: alloy::sol_types::private::Address,
        pub _initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        pub _operatorSetParams: alloy::sol_types::private::Vec<
            <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        >,
        pub _minimumStakes:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
        pub _strategyParams: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        >,
        pub _stakeTypes:
            alloy::sol_types::private::Vec<<StakeType as alloy::sol_types::SolType>::RustType>,
        pub _lookAheadPeriods: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`initialize(address,address,address,uint256,(uint32,uint16,uint16)[],uint96[],(address,uint96)[][],uint8[],uint32[])`](initializeCall) function.
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
                alloy::sol_types::sol_data::Array<IRegistryCoordinator::OperatorSetParam>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
                >,
                alloy::sol_types::sol_data::Array<StakeType>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
                    >,
                >,
                alloy::sol_types::private::Vec<<StakeType as alloy::sol_types::SolType>::RustType>,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value._initialOwner,
                        value._churnApprover,
                        value._ejector,
                        value._initialPausedStatus,
                        value._operatorSetParams,
                        value._minimumStakes,
                        value._strategyParams,
                        value._stakeTypes,
                        value._lookAheadPeriods,
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
                        _operatorSetParams: tuple.4,
                        _minimumStakes: tuple.5,
                        _strategyParams: tuple.6,
                        _stakeTypes: tuple.7,
                        _lookAheadPeriods: tuple.8,
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
                alloy::sol_types::sol_data::Array<IRegistryCoordinator::OperatorSetParam>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
                >,
                alloy::sol_types::sol_data::Array<StakeType>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address,address,uint256,(uint32,uint16,uint16)[],uint96[],(address,uint96)[][],uint8[],uint32[])";
            const SELECTOR: [u8; 4] = [127u8, 195u8, 248u8, 134u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._initialPausedStatus),
                    <alloy::sol_types::sol_data::Array<
                        IRegistryCoordinator::OperatorSetParam,
                    > as alloy_sol_types::SolType>::tokenize(&self._operatorSetParams),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self._minimumStakes),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
                    > as alloy_sol_types::SolType>::tokenize(&self._strategyParams),
                    <alloy::sol_types::sol_data::Array<
                        StakeType,
                    > as alloy_sol_types::SolType>::tokenize(&self._stakeTypes),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self._lookAheadPeriods),
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
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isChurnApproverSaltUsed(bytes32)`](isChurnApproverSaltUsedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChurnApproverSaltUsedReturn {
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
    function isM2Quorum(uint8) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isM2QuorumCall {
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`isM2Quorum(uint8)`](isM2QuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isM2QuorumReturn {
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isM2QuorumCall {
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
    /**Function with signature `isOperatorSetAVS()` and selector `0xcabbb17f`.
    ```solidity
    function isOperatorSetAVS() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetAVSCall {}
    ///Container type for the return parameters of the [`isOperatorSetAVS()`](isOperatorSetAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetAVSReturn {
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
            impl ::core::convert::From<isOperatorSetAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetAVSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetAVSCall {
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
            impl ::core::convert::From<isOperatorSetAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetAVSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorSetAVSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSetAVSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorSetAVS()";
            const SELECTOR: [u8; 4] = [202u8, 187u8, 177u8, 127u8];
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
    /**Function with signature `isUsingOperatorSets()` and selector `0xbd33ee24`.
    ```solidity
    function isUsingOperatorSets() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUsingOperatorSetsCall {}
    ///Container type for the return parameters of the [`isUsingOperatorSets()`](isUsingOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUsingOperatorSetsReturn {
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
            impl ::core::convert::From<isUsingOperatorSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: isUsingOperatorSetsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isUsingOperatorSetsCall {
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
            impl ::core::convert::From<isUsingOperatorSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isUsingOperatorSetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isUsingOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isUsingOperatorSetsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isUsingOperatorSetsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isUsingOperatorSets()";
            const SELECTOR: [u8; 4] = [189u8, 51u8, 238u8, 36u8];
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
    /**Function with signature `lastEjectionTimestamp(address)` and selector `0x125e0584`.
    ```solidity
    function lastEjectionTimestamp(address) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastEjectionTimestampCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`lastEjectionTimestamp(address)`](lastEjectionTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastEjectionTimestampReturn {
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
        pub index: u8,
    }
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
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
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`pubkeyRegistrationMessageHash(address)`](pubkeyRegistrationMessageHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyRegistrationMessageHashReturn {
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
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`quorumUpdateBlockNumber(uint8)`](quorumUpdateBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumUpdateBlockNumberReturn {
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
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        pub socket: alloy::sol_types::private::String,
        pub params:
            <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
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
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
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
    function registerOperatorWithChurn(bytes memory quorumNumbers, string memory socket, IBLSApkRegistry.PubkeyRegistrationParams memory params, IRegistryCoordinator.OperatorKickParam[] memory operatorKickParams, ISignatureUtils.SignatureWithSaltAndExpiry memory churnApproverSignature, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithChurnCall {
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        pub socket: alloy::sol_types::private::String,
        pub params:
            <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
        pub operatorKickParams: alloy::sol_types::private::Vec<
            <IRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
        >,
        pub churnApproverSignature:
            <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        pub operatorSignature:
            <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<IRegistryCoordinator::OperatorKickParam>,
                ISignatureUtils::SignatureWithSaltAndExpiry,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::String,
                <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <IRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<IRegistryCoordinator::OperatorKickParam>,
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
                        IRegistryCoordinator::OperatorKickParam,
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
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`registries(uint256)`](registriesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registriesReturn {
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
    /**Function with signature `setChurnApprover(address)` and selector `0x29d1e0c3`.
    ```solidity
    function setChurnApprover(address _churnApprover) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setChurnApproverCall {
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
    function setOperatorSetParams(uint8 quorumNumber, IRegistryCoordinator.OperatorSetParam memory operatorSetParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorSetParamsCall {
        pub quorumNumber: u8,
        pub operatorSetParams:
            <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
                IRegistryCoordinator::OperatorSetParam,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
                IRegistryCoordinator::OperatorSetParam,
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <IRegistryCoordinator::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
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
        pub operatorsPerQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
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
        OPERATOR_CHURN_APPROVAL_TYPEHASH(OPERATOR_CHURN_APPROVAL_TYPEHASHCall),
        PUBKEY_REGISTRATION_TYPEHASH(PUBKEY_REGISTRATION_TYPEHASHCall),
        avsDirectory(avsDirectoryCall),
        blsApkRegistry(blsApkRegistryCall),
        calculateOperatorChurnApprovalDigestHash(calculateOperatorChurnApprovalDigestHashCall),
        churnApprover(churnApproverCall),
        createSlashableStakeQuorum(createSlashableStakeQuorumCall),
        createTotalDelegatedStakeQuorum(createTotalDelegatedStakeQuorumCall),
        deregisterOperator_0(deregisterOperator_0Call),
        deregisterOperator_1(deregisterOperator_1Call),
        ejectOperator(ejectOperatorCall),
        ejectionCooldown(ejectionCooldownCall),
        ejector(ejectorCall),
        enableOperatorSets(enableOperatorSetsCall),
        getCurrentQuorumBitmap(getCurrentQuorumBitmapCall),
        getOperator(getOperatorCall),
        getOperatorFromId(getOperatorFromIdCall),
        getOperatorId(getOperatorIdCall),
        getOperatorSetParams(getOperatorSetParamsCall),
        getOperatorStatus(getOperatorStatusCall),
        getQuorumBitmapAtBlockNumberByIndex(getQuorumBitmapAtBlockNumberByIndexCall),
        getQuorumBitmapHistoryLength(getQuorumBitmapHistoryLengthCall),
        getQuorumBitmapIndicesAtBlockNumber(getQuorumBitmapIndicesAtBlockNumberCall),
        getQuorumBitmapUpdateByIndex(getQuorumBitmapUpdateByIndexCall),
        indexRegistry(indexRegistryCall),
        initialize(initializeCall),
        isChurnApproverSaltUsed(isChurnApproverSaltUsedCall),
        isM2Quorum(isM2QuorumCall),
        isOperatorSetAVS(isOperatorSetAVSCall),
        isUsingOperatorSets(isUsingOperatorSetsCall),
        lastEjectionTimestamp(lastEjectionTimestampCall),
        numRegistries(numRegistriesCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        pubkeyRegistrationMessageHash(pubkeyRegistrationMessageHashCall),
        quorumCount(quorumCountCall),
        quorumUpdateBlockNumber(quorumUpdateBlockNumberCall),
        registerOperator_0(registerOperator_0Call),
        registerOperator_1(registerOperator_1Call),
        registerOperatorWithChurn(registerOperatorWithChurnCall),
        registries(registriesCall),
        renounceOwnership(renounceOwnershipCall),
        serviceManager(serviceManagerCall),
        setChurnApprover(setChurnApproverCall),
        setEjectionCooldown(setEjectionCooldownCall),
        setEjector(setEjectorCall),
        setOperatorSetParams(setOperatorSetParamsCall),
        stakeRegistry(stakeRegistryCall),
        transferOwnership(transferOwnershipCall),
        unpause(unpauseCall),
        updateOperators(updateOperatorsCall),
        updateOperatorsForQuorum(updateOperatorsForQuorumCall),
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
            [12u8, 244u8, 183u8, 103u8],
            [13u8, 63u8, 33u8, 52u8],
            [18u8, 94u8, 5u8, 132u8],
            [19u8, 84u8, 42u8, 78u8],
            [19u8, 100u8, 57u8, 221u8],
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
            [88u8, 101u8, 198u8, 12u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [91u8, 11u8, 130u8, 159u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 244u8, 89u8, 70u8],
            [99u8, 71u8, 201u8, 0u8],
            [104u8, 48u8, 72u8, 53u8],
            [107u8, 58u8, 167u8, 46u8],
            [110u8, 59u8, 23u8, 219u8],
            [113u8, 80u8, 24u8, 166u8],
            [127u8, 195u8, 248u8, 134u8],
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
            [189u8, 51u8, 238u8, 36u8],
            [195u8, 145u8, 66u8, 94u8],
            [202u8, 13u8, 232u8, 130u8],
            [202u8, 79u8, 45u8, 151u8],
            [202u8, 187u8, 177u8, 127u8],
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
        const COUNT: usize = 57usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(_) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(_) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::isOperatorSetAVS(_) => {
                    <isOperatorSetAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isUsingOperatorSets(_) => {
                    <isUsingOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastEjectionTimestamp(_) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::numRegistries(_) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::avsDirectory)
                    }
                    avsDirectory
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
                    fn isUsingOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <isUsingOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::isUsingOperatorSets)
                    }
                    isUsingOperatorSets
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
                    fn isOperatorSetAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorCalls> {
                        <isOperatorSetAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorCalls::isOperatorSetAVS)
                    }
                    isOperatorSetAVS
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
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
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
                Self::isOperatorSetAVS(inner) => {
                    <isOperatorSetAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isUsingOperatorSets(inner) => {
                    <isUsingOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastEjectionTimestamp(inner) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::numRegistries(inner) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isOperatorSetAVS(inner) => {
                    <isOperatorSetAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isUsingOperatorSets(inner) => {
                    <isUsingOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::numRegistries(inner) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        AlreadyRegisteredForQuorums(AlreadyRegisteredForQuorums),
        BitmapCannotBeZero(BitmapCannotBeZero),
        BitmapEmpty(BitmapEmpty),
        CannotChurnSelf(CannotChurnSelf),
        CannotKickOperatorAboveThreshold(CannotKickOperatorAboveThreshold),
        CannotReregisterYet(CannotReregisterYet),
        ChurnApproverSaltUsed(ChurnApproverSaltUsed),
        CurrentlyPaused(CurrentlyPaused),
        InputAddressZero(InputAddressZero),
        InputLengthMismatch(InputLengthMismatch),
        InsufficientStakeForChurn(InsufficientStakeForChurn),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        InvalidSignature(InvalidSignature),
        MaxQuorumsReached(MaxQuorumsReached),
        NotRegistered(NotRegistered),
        NotRegisteredForQuorum(NotRegisteredForQuorum),
        NotSorted(NotSorted),
        OnlyAllocationManager(OnlyAllocationManager),
        OnlyEjector(OnlyEjector),
        OnlyPauser(OnlyPauser),
        OnlyUnpauser(OnlyUnpauser),
        OperatorSetsEnabled(OperatorSetsEnabled),
        OperatorSetsNotEnabled(OperatorSetsNotEnabled),
        OperatorSetsNotSupported(OperatorSetsNotSupported),
        QuorumDoesNotExist(QuorumDoesNotExist),
        QuorumOperatorCountMismatch(QuorumOperatorCountMismatch),
        RegistryCoordinatorSignatureExpired(RegistryCoordinatorSignatureExpired),
        SaltAlreadyUsed(SaltAlreadyUsed),
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
            [11u8, 136u8, 48u8, 111u8],
            [12u8, 104u8, 22u8, 205u8],
            [12u8, 237u8, 48u8, 67u8],
            [19u8, 202u8, 70u8, 87u8],
            [35u8, 216u8, 113u8, 165u8],
            [50u8, 208u8, 206u8, 250u8],
            [60u8, 184u8, 156u8, 151u8],
            [76u8, 68u8, 153u8, 93u8],
            [91u8, 119u8, 144u8, 25u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [132u8, 10u8, 72u8, 213u8],
            [139u8, 170u8, 87u8, 159u8],
            [142u8, 90u8, 238u8, 231u8],
            [154u8, 21u8, 9u8, 141u8],
            [170u8, 173u8, 19u8, 247u8],
            [171u8, 164u8, 115u8, 57u8],
            [172u8, 45u8, 22u8, 130u8],
            [177u8, 135u8, 232u8, 105u8],
            [186u8, 80u8, 249u8, 17u8],
            [198u8, 29u8, 202u8, 93u8],
            [208u8, 83u8, 170u8, 33u8],
            [209u8, 109u8, 80u8, 234u8],
            [223u8, 125u8, 253u8, 134u8],
            [230u8, 33u8, 159u8, 234u8],
            [237u8, 177u8, 86u8, 46u8],
            [253u8, 44u8, 31u8, 77u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RegistryCoordinatorErrors {
        const NAME: &'static str = "RegistryCoordinatorErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
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
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxQuorumsReached(_) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::SELECTOR
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
                Self::OperatorSetsEnabled(_) => {
                    <OperatorSetsEnabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorSetsNotEnabled(_) => {
                    <OperatorSetsNotEnabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorSetsNotSupported(_) => {
                    <OperatorSetsNotSupported as alloy_sol_types::SolError>::SELECTOR
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
                    fn OperatorSetsEnabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <OperatorSetsEnabled as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::OperatorSetsEnabled)
                    }
                    OperatorSetsEnabled
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
                    fn OperatorSetsNotSupported(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RegistryCoordinatorErrors> {
                        <OperatorSetsNotSupported as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RegistryCoordinatorErrors::OperatorSetsNotSupported)
                    }
                    OperatorSetsNotSupported
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
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxQuorumsReached(inner) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::OperatorSetsEnabled(inner) => {
                    <OperatorSetsEnabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorSetsNotEnabled(inner) => {
                    <OperatorSetsNotEnabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorSetsNotSupported(inner) => {
                    <OperatorSetsNotSupported as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OperatorSetsEnabled(inner) => {
                    <OperatorSetsEnabled as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OperatorSetsNotSupported(inner) => {
                    <OperatorSetsNotSupported as alloy_sol_types::SolError>::abi_encode_raw(
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
        ChurnApproverUpdated(ChurnApproverUpdated),
        EjectorUpdated(EjectorUpdated),
        Initialized(Initialized),
        OperatorDeregistered(OperatorDeregistered),
        OperatorRegistered(OperatorRegistered),
        OperatorSetParamsUpdated(OperatorSetParamsUpdated),
        OperatorSocketUpdate(OperatorSocketUpdate),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        QuorumBlockNumberUpdated(QuorumBlockNumberUpdated),
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
        const COUNT: usize = 11usize;
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
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetParamsUpdated(inner) => {
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
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetParamsUpdated(inner) => {
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
        _avsDirectory: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<RegistryCoordinatorInstance<T, P, N>>>
    {
        RegistryCoordinatorInstance::<T, P, N>::deploy(
            provider,
            _serviceManager,
            _stakeRegistry,
            _blsApkRegistry,
            _indexRegistry,
            _avsDirectory,
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
        _avsDirectory: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        RegistryCoordinatorInstance::<T, P, N>::deploy_builder(
            provider,
            _serviceManager,
            _stakeRegistry,
            _blsApkRegistry,
            _indexRegistry,
            _avsDirectory,
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
            _avsDirectory: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<RegistryCoordinatorInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _serviceManager,
                _stakeRegistry,
                _blsApkRegistry,
                _indexRegistry,
                _avsDirectory,
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
            _avsDirectory: alloy::sol_types::private::Address,
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
                        _avsDirectory,
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
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(&self) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
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
                <IRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
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
            operatorSetParams: <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            operatorSetParams: <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            _operatorSetParams: alloy::sol_types::private::Vec<
                <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
            >,
            _minimumStakes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U96,
            >,
            _strategyParams: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<
                    <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
                >,
            >,
            _stakeTypes: alloy::sol_types::private::Vec<
                <StakeType as alloy::sol_types::SolType>::RustType,
            >,
            _lookAheadPeriods: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                _initialOwner,
                _churnApprover,
                _ejector,
                _initialPausedStatus,
                _operatorSetParams,
                _minimumStakes,
                _strategyParams,
                _stakeTypes,
                _lookAheadPeriods,
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
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, isM2QuorumCall, N> {
            self.call_builder(&isM2QuorumCall { _0 })
        }
        ///Creates a new call builder for the [`isOperatorSetAVS`] function.
        pub fn isOperatorSetAVS(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorSetAVSCall, N> {
            self.call_builder(&isOperatorSetAVSCall {})
        }
        ///Creates a new call builder for the [`isUsingOperatorSets`] function.
        pub fn isUsingOperatorSets(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, isUsingOperatorSetsCall, N> {
            self.call_builder(&isUsingOperatorSetsCall {})
        }
        ///Creates a new call builder for the [`lastEjectionTimestamp`] function.
        pub fn lastEjectionTimestamp(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastEjectionTimestampCall, N> {
            self.call_builder(&lastEjectionTimestampCall { _0 })
        }
        ///Creates a new call builder for the [`numRegistries`] function.
        pub fn numRegistries(&self) -> alloy_contract::SolCallBuilder<T, &P, numRegistriesCall, N> {
            self.call_builder(&numRegistriesCall {})
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
                <IRegistryCoordinator::OperatorKickParam as alloy::sol_types::SolType>::RustType,
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
            operatorSetParams: <IRegistryCoordinator::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
