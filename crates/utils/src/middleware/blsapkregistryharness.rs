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
    struct ApkUpdate { bytes24 apkHash; uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; }
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
    struct ApkUpdate { bytes24 apkHash; uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ApkUpdate {
        pub apkHash: alloy::sol_types::private::FixedBytes<24>,
        pub updateBlockNumber: u32,
        pub nextUpdateBlockNumber: u32,
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
            alloy::sol_types::sol_data::FixedBytes<24>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<24>, u32, u32);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ApkUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: ApkUpdate) -> Self {
                (
                    value.apkHash,
                    value.updateBlockNumber,
                    value.nextUpdateBlockNumber,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ApkUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    apkHash: tuple.0,
                    updateBlockNumber: tuple.1,
                    nextUpdateBlockNumber: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ApkUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ApkUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.apkHash),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.updateBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.nextUpdateBlockNumber),
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
        impl alloy_sol_types::SolType for ApkUpdate {
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
        impl alloy_sol_types::SolStruct for ApkUpdate {
            const NAME: &'static str = "ApkUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ApkUpdate(bytes24 apkHash,uint32 updateBlockNumber,uint32 nextUpdateBlockNumber)",
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
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.apkHash)
                        .0,
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
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ApkUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.apkHash,
                    )
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
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedBytes<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.apkHash,
                    out,
                );
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
    struct ApkUpdate {
        bytes24 apkHash;
        uint32 updateBlockNumber;
        uint32 nextUpdateBlockNumber;
    }
    struct PubkeyRegistrationParams {
        BN254.G1Point pubkeyRegistrationSignature;
        BN254.G1Point pubkeyG1;
        BN254.G2Point pubkeyG2;
    }
}

interface BLSApkRegistryHarness {
    event Initialized(uint8 version);
    event NewPubkeyRegistration(address indexed operator, BN254.G1Point pubkeyG1, BN254.G2Point pubkeyG2);
    event OperatorAddedToQuorums(address operator, bytes32 operatorId, bytes quorumNumbers);
    event OperatorRemovedFromQuorums(address operator, bytes32 operatorId, bytes quorumNumbers);

    constructor(address _registryCoordinator);

    function apkHistory(uint8, uint256) external view returns (bytes24 apkHash, uint32 updateBlockNumber, uint32 nextUpdateBlockNumber);
    function currentApk(uint8) external view returns (uint256 X, uint256 Y);
    function deregisterOperator(address operator, bytes memory quorumNumbers) external;
    function getApk(uint8 quorumNumber) external view returns (BN254.G1Point memory);
    function getApkHashAtBlockNumberAndIndex(uint8 quorumNumber, uint32 blockNumber, uint256 index) external view returns (bytes24);
    function getApkHistoryLength(uint8 quorumNumber) external view returns (uint32);
    function getApkIndicesAtBlockNumber(bytes memory quorumNumbers, uint256 blockNumber) external view returns (uint32[] memory);
    function getApkUpdateAtIndex(uint8 quorumNumber, uint256 index) external view returns (IBLSApkRegistry.ApkUpdate memory);
    function getOperatorFromPubkeyHash(bytes32 pubkeyHash) external view returns (address);
    function getOperatorId(address operator) external view returns (bytes32);
    function getRegisteredPubkey(address operator) external view returns (BN254.G1Point memory, bytes32);
    function initializeQuorum(uint8 quorumNumber) external;
    function operatorToPubkey(address) external view returns (uint256 X, uint256 Y);
    function operatorToPubkeyHash(address) external view returns (bytes32);
    function pubkeyHashToOperator(bytes32) external view returns (address);
    function registerBLSPublicKey(address operator, IBLSApkRegistry.PubkeyRegistrationParams memory params, BN254.G1Point memory pubkeyRegistrationMessageHash) external returns (bytes32 operatorId);
    function registerOperator(address operator, bytes memory quorumNumbers) external;
    function registryCoordinator() external view returns (address);
    function setBLSPublicKey(address account, BN254.G1Point memory pk) external;
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
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "apkHistory",
    "inputs": [
      {
        "name": "",
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
        "name": "apkHash",
        "type": "bytes24",
        "internalType": "bytes24"
      },
      {
        "name": "updateBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "nextUpdateBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "currentApk",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
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
    ],
    "stateMutability": "view"
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
    "name": "getApk",
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
    "name": "getApkHashAtBlockNumberAndIndex",
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
        "type": "bytes24",
        "internalType": "bytes24"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getApkHistoryLength",
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
    "name": "getApkIndicesAtBlockNumber",
    "inputs": [
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "blockNumber",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "getApkUpdateAtIndex",
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
        "internalType": "struct IBLSApkRegistry.ApkUpdate",
        "components": [
          {
            "name": "apkHash",
            "type": "bytes24",
            "internalType": "bytes24"
          },
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorFromPubkeyHash",
    "inputs": [
      {
        "name": "pubkeyHash",
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
    "name": "getRegisteredPubkey",
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
    "name": "initializeQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "operatorToPubkey",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
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
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorToPubkeyHash",
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
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pubkeyHashToOperator",
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
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerBLSPublicKey",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
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
        "name": "pubkeyRegistrationMessageHash",
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
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
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
    "name": "setBLSPublicKey",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "pk",
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
    "name": "NewPubkeyRegistration",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "pubkeyG1",
        "type": "tuple",
        "indexed": false,
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
        "indexed": false,
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
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorAddedToQuorums",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRemovedFromQuorums",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
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
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BLSApkRegistryHarness {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523480156200001157600080fd5b50604051620020fc380380620020fc833981016040819052620000349162000118565b6001600160a01b03811660805280806200004d62000056565b5050506200014a565b600054610100900460ff1615620000c35760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff908116101562000116576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6000602082840312156200012b57600080fd5b81516001600160a01b03811681146200014357600080fd5b9392505050565b608051611f8f6200016d6000396000818161031a015261104b0152611f8f6000f3fe608060405234801561001057600080fd5b50600436106101205760003560e01c80636d14a987116100ad578063d5254a8c11610071578063d5254a8c146103ea578063de29fac01461040a578063df4d09e01461042a578063e8bb9ae614610494578063f4e24fe5146104bd57600080fd5b80636d14a987146103155780637916cea61461033c5780637ff81a871461037d578063a3db80e2146103b0578063bf79ce58146103d757600080fd5b80633fb27952116100f45780633fb27952146101ea57806347b314e8146101fd5780635f61a8841461023e578063605747d51461029a57806368bccaac146102e857600080fd5b8062a1f4cb1461012557806313542a4e1461016657806326d941f21461019d578063377ed99d146101b2575b600080fd5b61014c61013336600461192c565b6003602052600090815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b61018f61017436600461192c565b6001600160a01b031660009081526001602052604090205490565b60405190815260200161015d565b6101b06101ab36600461195f565b6104d0565b005b6101d56101c036600461195f565b60ff1660009081526004602052604090205490565b60405163ffffffff909116815260200161015d565b6101b06101f83660046119ea565b6105e5565b61022661020b366004611a90565b6000908152600260205260409020546001600160a01b031690565b6040516001600160a01b03909116815260200161015d565b61028d61024c36600461195f565b60408051808201909152600080825260208201525060ff16600090815260056020908152604091829020825180840190935280548352600101549082015290565b60405161015d9190611aa9565b6102ad6102a8366004611ac0565b610663565b60408051825167ffffffffffffffff1916815260208084015163ffffffff90811691830191909152928201519092169082015260600161015d565b6102fb6102f6366004611aea565b6106f6565b60405167ffffffffffffffff19909116815260200161015d565b6102267f000000000000000000000000000000000000000000000000000000000000000081565b61034f61034a366004611ac0565b610891565b6040805167ffffffffffffffff19909416845263ffffffff928316602085015291169082015260600161015d565b61039061038b36600461192c565b6108dc565b60408051835181526020938401519381019390935282015260600161015d565b61014c6103be36600461195f565b6005602052600090815260409020805460019091015482565b61018f6103e5366004611b32565b6109a9565b6103fd6103f8366004611b8f565b610dbd565b60405161015d9190611c07565b61018f61041836600461192c565b60016020526000908152604090205481565b6101b0610438366004611c81565b8051600090815260208083018051825260408084206001600160a01b039690961680855260018085528286208890559685526002845281852080546001600160a01b031916821790558452600390925291209151825551910155565b6102266104a2366004611a90565b6002602052600090815260409020546001600160a01b031681565b6101b06104cb3660046119ea565b610fd7565b6104d8611040565b60ff81166000908152600460205260409020541561055c5760405162461bcd60e51b815260206004820152603660248201527f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a6044820152752071756f72756d20616c72656164792065786973747360501b60648201526084015b60405180910390fd5b60ff166000908152600460209081526040808320815160608101835284815263ffffffff4381168286019081528285018781528454600181018655948852959096209151919092018054955194518316600160e01b026001600160e01b0395909316600160c01b026001600160e01b03199096169190931c179390931791909116919091179055565b6105ed611040565b60006105f8836108dc565b50905061060582826110f7565b7f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e83610646856001600160a01b031660009081526001602052604090205490565b8460405161065693929190611cb5565b60405180910390a1505050565b604080516060810182526000808252602080830182905282840182905260ff8616825260049052919091208054839081106106a0576106a0611d21565b600091825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b8204811694830194909452600160e01b90049092169082015290505b92915050565b60ff8316600090815260046020526040812080548291908490811061071d5761071d611d21565b600091825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b82048116948301859052600160e01b9091048116928201929092529250851610156107e45760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a20696e64657820746f6f20726563656e7400006064820152608401610553565b604081015163ffffffff16158061080a5750806040015163ffffffff168463ffffffff16105b6108885760405162461bcd60e51b815260206004820152604360248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a206e6f74206c61746573742061706b2075706460648201526261746560e81b608482015260a401610553565b51949350505050565b600460205281600052604060002081815481106108ad57600080fd5b600091825260209091200154604081901b925063ffffffff600160c01b820481169250600160e01b9091041683565b60408051808201909152600080825260208201526001600160a01b03821660008181526003602090815260408083208151808301835281548152600191820154818501529484529091528120549091908061099f5760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e676574526567697374657265645075626b60448201527f65793a206f70657261746f72206973206e6f74207265676973746572656400006064820152608401610553565b9094909350915050565b60006109b3611040565b60006109e16109ca36869003860160408701611d37565b805160009081526020918201519091526040902090565b90507fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5811415610a69576040805162461bcd60e51b8152602060048201526024810191909152600080516020611f3a83398151915260448201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b65796064820152608401610553565b6001600160a01b03851660009081526001602052604090205415610af35760405162461bcd60e51b81526020600482015260476024820152600080516020611f3a83398151915260448201527f4b65793a206f70657261746f7220616c72656164792072656769737465726564606482015266207075626b657960c81b608482015260a401610553565b6000818152600260205260409020546001600160a01b031615610b775760405162461bcd60e51b81526020600482015260426024820152600080516020611f3a83398151915260448201527f4b65793a207075626c6963206b657920616c7265616479207265676973746572606482015261195960f21b608482015260a401610553565b604080516000917f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000191610bd0918835916020808b0135928b01359160608c01359160808d019160c08e01918d35918e8201359101611d53565b6040516020818303038152906040528051906020012060001c610bf39190611d9e565b9050610c8d610c2c610c1783610c11368a90038a0160408b01611d37565b90611342565b610c2636899003890189611d37565b906113d9565b610c3461146d565b610c76610c6785610c11604080518082018252600080825260209182015281518083019092526001825260029082015290565b610c26368a90038a018a611d37565b610c88368a90038a0160808b01611e30565b61152d565b610d285760405162461bcd60e51b815260206004820152606c6024820152600080516020611f3a83398151915260448201527f4b65793a2065697468657220746865204731207369676e61747572652069732060648201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060848201526b0c8de40dcdee840dac2e8c6d60a31b60a482015260c401610553565b6001600160a01b03861660008181526003602090815260408083208982018035825560608b013560019283015590835281842087905586845260029092529182902080546001600160a01b0319168417905590517fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba382804191610dac9160808a0190611e6f565b60405180910390a250949350505050565b606060008367ffffffffffffffff811115610dda57610dda61197a565b604051908082528060200260200182016040528015610e03578160200160208202803683370190505b50905060005b84811015610fce576000868683818110610e2557610e25611d21565b919091013560f81c6000818152600460205260409020549092509050801580610e88575060ff821660009081526004602052604081208054909190610e6c57610e6c611d21565b600091825260209091200154600160c01b900463ffffffff1686105b15610f155760405162461bcd60e51b815260206004820152605160248201527f424c5341706b52656769737472792e67657441706b496e64696365734174426c60448201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f7265606482015270207468652066697273742075706461746560781b608482015260a401610553565b805b8015610fb85760ff831660009081526004602052604090208790610f3c600184611eb9565b81548110610f4c57610f4c611d21565b600091825260209091200154600160c01b900463ffffffff1611610fa657610f75600182611eb9565b858581518110610f8757610f87611d21565b602002602001019063ffffffff16908163ffffffff1681525050610fb8565b80610fb081611ed0565b915050610f17565b5050508080610fc690611ee7565b915050610e09565b50949350505050565b610fdf611040565b6000610fea836108dc565b509050610fff82610ffa8361179a565b6110f7565b7ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e83610646856001600160a01b031660009081526001602052604090205490565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146110f55760405162461bcd60e51b815260206004820152604e60248201527f424c5341706b52656769737472792e6f6e6c795265676973747279436f6f726460448201527f696e61746f723a2063616c6c6572206973206e6f74207468652072656769737460648201526d393c9031b7b7b93234b730ba37b960911b608482015260a401610553565b565b604080518082019091526000808252602082015260005b835181101561133c57600084828151811061112b5761112b611d21565b0160209081015160f81c60008181526004909252604090912054909150806111bb5760405162461bcd60e51b815260206004820152603d60248201527f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b60448201527f5570646174653a2071756f72756d20646f6573206e6f742065786973740000006064820152608401610553565b60ff821660009081526005602090815260409182902082518084019093528054835260010154908201526111ef90866113d9565b60ff831660008181526005602090815260408083208551808255868401805160019384015590855251835281842094845260049092528220939750919290916112389085611eb9565b8154811061124857611248611d21565b600091825260209091200180549091504363ffffffff908116600160c01b9092041614156112895780546001600160c01b031916604083901c178155611325565b805463ffffffff438116600160e01b8181026001600160e01b0394851617855560ff88166000908152600460209081526040808320815160608101835267ffffffffffffffff198b16815280840196875280830185815282546001810184559286529390942093519301805495519251871690940291909516600160c01b026001600160e01b0319949094169190941c17919091179092161790555b50505050808061133490611ee7565b91505061110e565b50505050565b604080518082019091526000808252602082015261135e611859565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa905080801561139157611393565bfe5b50806113d15760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610553565b505092915050565b60408051808201909152600080825260208201526113f5611877565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa90508080156113915750806113d15760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610553565b611475611895565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b60408051808201825285815260208082018590528251808401909352858352820183905260009161155c6118ba565b60005b6002811015611721576000611575826006611f02565b905084826002811061158957611589611d21565b6020020151518361159b836000611f21565b600c81106115ab576115ab611d21565b60200201528482600281106115c2576115c2611d21565b602002015160200151838260016115d99190611f21565b600c81106115e9576115e9611d21565b602002015283826002811061160057611600611d21565b6020020151515183611613836002611f21565b600c811061162357611623611d21565b602002015283826002811061163a5761163a611d21565b6020020151516001602002015183611653836003611f21565b600c811061166357611663611d21565b602002015283826002811061167a5761167a611d21565b60200201516020015160006002811061169557611695611d21565b6020020151836116a6836004611f21565b600c81106116b6576116b6611d21565b60200201528382600281106116cd576116cd611d21565b6020020151602001516001600281106116e8576116e8611d21565b6020020151836116f9836005611f21565b600c811061170957611709611d21565b6020020152508061171981611ee7565b91505061155f565b5061172a6118d9565b60006020826101808560086107d05a03fa905080801561139157508061178a5760405162461bcd60e51b81526020600482015260156024820152741c185a5c9a5b99cb5bdc18dbd9194b59985a5b1959605a1b6044820152606401610553565b5051151598975050505050505050565b604080518082019091526000808252602082015281511580156117bf57506020820151155b156117dd575050604080518082019091526000808252602082015290565b6040518060400160405280836000015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4784602001516118229190611d9e565b61184c907f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611eb9565b905292915050565b919050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806118a86118f7565b81526020016118b56118f7565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b80356001600160a01b038116811461185457600080fd5b60006020828403121561193e57600080fd5b61194782611915565b9392505050565b803560ff8116811461185457600080fd5b60006020828403121561197157600080fd5b6119478261194e565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff811182821017156119b3576119b361197a565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156119e2576119e261197a565b604052919050565b600080604083850312156119fd57600080fd5b611a0683611915565b915060208084013567ffffffffffffffff80821115611a2457600080fd5b818601915086601f830112611a3857600080fd5b813581811115611a4a57611a4a61197a565b611a5c601f8201601f191685016119b9565b91508082528784828501011115611a7257600080fd5b80848401858401376000848284010152508093505050509250929050565b600060208284031215611aa257600080fd5b5035919050565b8151815260208083015190820152604081016106f0565b60008060408385031215611ad357600080fd5b611adc8361194e565b946020939093013593505050565b600080600060608486031215611aff57600080fd5b611b088461194e565b9250602084013563ffffffff81168114611b2157600080fd5b929592945050506040919091013590565b6000806000838503610160811215611b4957600080fd5b611b5285611915565b9350610100601f1982011215611b6757600080fd5b602085019250604061011f1982011215611b8057600080fd5b50610120840190509250925092565b600080600060408486031215611ba457600080fd5b833567ffffffffffffffff80821115611bbc57600080fd5b818601915086601f830112611bd057600080fd5b813581811115611bdf57600080fd5b876020828501011115611bf157600080fd5b6020928301989097509590910135949350505050565b6020808252825182820181905260009190848201906040850190845b81811015611c4557835163ffffffff1683529284019291840191600101611c23565b50909695505050505050565b600060408284031215611c6357600080fd5b611c6b611990565b9050813581526020820135602082015292915050565b60008060608385031215611c9457600080fd5b611c9d83611915565b9150611cac8460208501611c51565b90509250929050565b60018060a01b038416815260006020848184015260606040840152835180606085015260005b81811015611cf757858101830151858201608001528201611cdb565b81811115611d09576000608083870101525b50601f01601f19169290920160800195945050505050565b634e487b7160e01b600052603260045260246000fd5b600060408284031215611d4957600080fd5b6119478383611c51565b8881528760208201528660408201528560608201526040856080830137600060c082016000815260408682375050610100810192909252610120820152610140019695505050505050565b600082611dbb57634e487b7160e01b600052601260045260246000fd5b500690565b600082601f830112611dd157600080fd5b6040516040810181811067ffffffffffffffff82111715611df457611df461197a565b8060405250806040840185811115611e0b57600080fd5b845b81811015611e25578035835260209283019201611e0d565b509195945050505050565b600060808284031215611e4257600080fd5b611e4a611990565b611e548484611dc0565b8152611e638460408501611dc0565b60208201529392505050565b823581526020808401359082015260c081016040838184013760808201600081526040808501823750600081529392505050565b634e487b7160e01b600052601160045260246000fd5b600082821015611ecb57611ecb611ea3565b500390565b600081611edf57611edf611ea3565b506000190190565b6000600019821415611efb57611efb611ea3565b5060010190565b6000816000190483118215151615611f1c57611f1c611ea3565b500290565b60008219821115611f3457611f34611ea3565b50019056fe424c5341706b52656769737472792e7265676973746572424c535075626c6963a2646970667358221220eea33c8e1c7c9150fe692d53bc83e71bbbea74b773567bc363507193cdd5aeaa64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0 \xFC8\x03\x80b\0 \xFC\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R\x80\x80b\0\0Mb\0\0VV[PPPb\0\x01JV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\x16W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15b\0\x01+W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01CW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x1F\x8Fb\0\x01m`\09`\0\x81\x81a\x03\x1A\x01Ra\x10K\x01Ra\x1F\x8F`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01 W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\xADW\x80c\xD5%J\x8C\x11a\0qW\x80c\xD5%J\x8C\x14a\x03\xEAW\x80c\xDE)\xFA\xC0\x14a\x04\nW\x80c\xDFM\t\xE0\x14a\x04*W\x80c\xE8\xBB\x9A\xE6\x14a\x04\x94W\x80c\xF4\xE2O\xE5\x14a\x04\xBDW`\0\x80\xFD[\x80cm\x14\xA9\x87\x14a\x03\x15W\x80cy\x16\xCE\xA6\x14a\x03<W\x80c\x7F\xF8\x1A\x87\x14a\x03}W\x80c\xA3\xDB\x80\xE2\x14a\x03\xB0W\x80c\xBFy\xCEX\x14a\x03\xD7W`\0\x80\xFD[\x80c?\xB2yR\x11a\0\xF4W\x80c?\xB2yR\x14a\x01\xEAW\x80cG\xB3\x14\xE8\x14a\x01\xFDW\x80c_a\xA8\x84\x14a\x02>W\x80c`WG\xD5\x14a\x02\x9AW\x80ch\xBC\xCA\xAC\x14a\x02\xE8W`\0\x80\xFD[\x80b\xA1\xF4\xCB\x14a\x01%W\x80c\x13T*N\x14a\x01fW\x80c&\xD9A\xF2\x14a\x01\x9DW\x80c7~\xD9\x9D\x14a\x01\xB2W[`\0\x80\xFD[a\x01La\x0136`\x04a\x19,V[`\x03` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Fa\x01t6`\x04a\x19,V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01]V[a\x01\xB0a\x01\xAB6`\x04a\x19_V[a\x04\xD0V[\0[a\x01\xD5a\x01\xC06`\x04a\x19_V[`\xFF\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01]V[a\x01\xB0a\x01\xF86`\x04a\x19\xEAV[a\x05\xE5V[a\x02&a\x02\x0B6`\x04a\x1A\x90V[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01]V[a\x02\x8Da\x02L6`\x04a\x19_V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\xFF\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`@Qa\x01]\x91\x90a\x1A\xA9V[a\x02\xADa\x02\xA86`\x04a\x1A\xC0V[a\x06cV[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x01]V[a\x02\xFBa\x02\xF66`\x04a\x1A\xEAV[a\x06\xF6V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R` \x01a\x01]V[a\x02&\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Oa\x03J6`\x04a\x1A\xC0V[a\x08\x91V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01]V[a\x03\x90a\x03\x8B6`\x04a\x19,V[a\x08\xDCV[`@\x80Q\x83Q\x81R` \x93\x84\x01Q\x93\x81\x01\x93\x90\x93R\x82\x01R``\x01a\x01]V[a\x01La\x03\xBE6`\x04a\x19_V[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[a\x01\x8Fa\x03\xE56`\x04a\x1B2V[a\t\xA9V[a\x03\xFDa\x03\xF86`\x04a\x1B\x8FV[a\r\xBDV[`@Qa\x01]\x91\x90a\x1C\x07V[a\x01\x8Fa\x04\x186`\x04a\x19,V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xB0a\x0486`\x04a\x1C\x81V[\x80Q`\0\x90\x81R` \x80\x83\x01\x80Q\x82R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16\x80\x85R`\x01\x80\x85R\x82\x86 \x88\x90U\x96\x85R`\x02\x84R\x81\x85 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x82\x17\x90U\x84R`\x03\x90\x92R\x91 \x91Q\x82UQ\x91\x01UV[a\x02&a\x04\xA26`\x04a\x1A\x90V[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xB0a\x04\xCB6`\x04a\x19\xEAV[a\x0F\xD7V[a\x04\xD8a\x10@V[`\xFF\x81\x16`\0\x90\x81R`\x04` R`@\x90 T\x15a\x05\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBLSApkRegistry.initializeQuorum:`D\x82\x01Ru quorum already exists`P\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\xFF\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x84\x81Rc\xFF\xFF\xFF\xFFC\x81\x16\x82\x86\x01\x90\x81R\x82\x85\x01\x87\x81R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x94Q\x83\x16`\x01`\xE0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x95\x90\x93\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x96\x16\x91\x90\x93\x1C\x17\x93\x90\x93\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UV[a\x05\xEDa\x10@V[`\0a\x05\xF8\x83a\x08\xDCV[P\x90Pa\x06\x05\x82\x82a\x10\xF7V[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06F\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[\x84`@Qa\x06V\x93\x92\x91\x90a\x1C\xB5V[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x04\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x06\xA0Wa\x06\xA0a\x1D!V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x07\x1DWa\x07\x1Da\x1D!V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x85\x90R`\x01`\xE0\x1B\x90\x91\x04\x81\x16\x92\x82\x01\x92\x90\x92R\x92P\x85\x16\x10\x15a\x07\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: index too recent\0\0`d\x82\x01R`\x84\x01a\x05SV[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x08\nWP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x08\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: not latest apk upd`d\x82\x01Rbate`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x05SV[Q\x94\x93PPPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x08\xADW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`@\x81\x90\x1B\x92Pc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x92P`\x01`\xE0\x1B\x90\x91\x04\x16\x83V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x91\x82\x01T\x81\x85\x01R\x94\x84R\x90\x91R\x81 T\x90\x91\x90\x80a\t\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry.getRegisteredPubk`D\x82\x01R\x7Fey: operator is not registered\0\0`d\x82\x01R`\x84\x01a\x05SV[\x90\x94\x90\x93P\x91PPV[`\0a\t\xB3a\x10@V[`\0a\t\xE1a\t\xCA6\x86\x90\x03\x86\x01`@\x87\x01a\x1D7V[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x14\x15a\niW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` a\x1F:\x839\x81Q\x91R`D\x82\x01R\x7FKey: cannot register zero pubkey`d\x82\x01R`\x84\x01a\x05SV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\n\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a\x1F:\x839\x81Q\x91R`D\x82\x01R\x7FKey: operator already registered`d\x82\x01Rf pubkey`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05SV[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0BwW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a\x1F:\x839\x81Q\x91R`D\x82\x01R\x7FKey: public key already register`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x05SV[`@\x80Q`\0\x91\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x91a\x0B\xD0\x91\x885\x91` \x80\x8B\x015\x92\x8B\x015\x91``\x8C\x015\x91`\x80\x8D\x01\x91`\xC0\x8E\x01\x91\x8D5\x91\x8E\x82\x015\x91\x01a\x1DSV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x0B\xF3\x91\x90a\x1D\x9EV[\x90Pa\x0C\x8Da\x0C,a\x0C\x17\x83a\x0C\x116\x8A\x90\x03\x8A\x01`@\x8B\x01a\x1D7V[\x90a\x13BV[a\x0C&6\x89\x90\x03\x89\x01\x89a\x1D7V[\x90a\x13\xD9V[a\x0C4a\x14mV[a\x0Cva\x0Cg\x85a\x0C\x11`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[a\x0C&6\x8A\x90\x03\x8A\x01\x8Aa\x1D7V[a\x0C\x886\x8A\x90\x03\x8A\x01`\x80\x8B\x01a\x1E0V[a\x15-V[a\r(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R`\0\x80Q` a\x1F:\x839\x81Q\x91R`D\x82\x01R\x7FKey: either the G1 signature is `d\x82\x01R\x7Fwrong, or G1 and G2 private key `\x84\x82\x01Rk\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x01a\x05SV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x89\x82\x01\x805\x82U``\x8B\x015`\x01\x92\x83\x01U\x90\x83R\x81\x84 \x87\x90U\x86\x84R`\x02\x90\x92R\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90U\x90Q\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x91a\r\xAC\x91`\x80\x8A\x01\x90a\x1EoV[`@Q\x80\x91\x03\x90\xA2P\x94\x93PPPPV[```\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xDAWa\r\xDAa\x19zV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x03W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x0F\xCEW`\0\x86\x86\x83\x81\x81\x10a\x0E%Wa\x0E%a\x1D!V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x04` R`@\x90 T\x90\x92P\x90P\x80\x15\x80a\x0E\x88WP`\xFF\x82\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x90\x91\x90a\x0ElWa\x0Ela\x1D!V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x0F\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FBLSApkRegistry.getApkIndicesAtBl`D\x82\x01R\x7FockNumber: blockNumber is before`d\x82\x01Rp the first update`x\x1B`\x84\x82\x01R`\xA4\x01a\x05SV[\x80[\x80\x15a\x0F\xB8W`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x90 \x87\x90a\x0F<`\x01\x84a\x1E\xB9V[\x81T\x81\x10a\x0FLWa\x0FLa\x1D!V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11a\x0F\xA6Wa\x0Fu`\x01\x82a\x1E\xB9V[\x85\x85\x81Q\x81\x10a\x0F\x87Wa\x0F\x87a\x1D!V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x0F\xB8V[\x80a\x0F\xB0\x81a\x1E\xD0V[\x91PPa\x0F\x17V[PPP\x80\x80a\x0F\xC6\x90a\x1E\xE7V[\x91PPa\x0E\tV[P\x94\x93PPPPV[a\x0F\xDFa\x10@V[`\0a\x0F\xEA\x83a\x08\xDCV[P\x90Pa\x0F\xFF\x82a\x0F\xFA\x83a\x17\x9AV[a\x10\xF7V[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x06F\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R\x7FBLSApkRegistry.onlyRegistryCoord`D\x82\x01R\x7Finator: caller is not the regist`d\x82\x01Rm9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x91\x1B`\x84\x82\x01R`\xA4\x01a\x05SV[V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[\x83Q\x81\x10\x15a\x13<W`\0\x84\x82\x81Q\x81\x10a\x11+Wa\x11+a\x1D!V[\x01` \x90\x81\x01Q`\xF8\x1C`\0\x81\x81R`\x04\x90\x92R`@\x90\x91 T\x90\x91P\x80a\x11\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBLSApkRegistry._processQuorumApk`D\x82\x01R\x7FUpdate: quorum does not exist\0\0\0`d\x82\x01R`\x84\x01a\x05SV[`\xFF\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01Ra\x11\xEF\x90\x86a\x13\xD9V[`\xFF\x83\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x85Q\x80\x82U\x86\x84\x01\x80Q`\x01\x93\x84\x01U\x90\x85RQ\x83R\x81\x84 \x94\x84R`\x04\x90\x92R\x82 \x93\x97P\x91\x92\x90\x91a\x128\x90\x85a\x1E\xB9V[\x81T\x81\x10a\x12HWa\x12Ha\x1D!V[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16`\x01`\xC0\x1B\x90\x92\x04\x16\x14\x15a\x12\x89W\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`@\x83\x90\x1C\x17\x81Ua\x13%V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01`\xE0\x1B\x81\x81\x02`\x01`\x01`\xE0\x1B\x03\x94\x85\x16\x17\x85U`\xFF\x88\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x8B\x16\x81R\x80\x84\x01\x96\x87R\x80\x83\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x93Q\x93\x01\x80T\x95Q\x92Q\x87\x16\x90\x94\x02\x91\x90\x95\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x94\x90\x94\x16\x91\x90\x94\x1C\x17\x91\x90\x91\x17\x90\x92\x16\x17\x90U[PPPP\x80\x80a\x134\x90a\x1E\xE7V[\x91PPa\x11\x0EV[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13^a\x18YV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\x91Wa\x13\x93V[\xFE[P\x80a\x13\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05SV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13\xF5a\x18wV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\x91WP\x80a\x13\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05SV[a\x14ua\x18\x95V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R`\0\x91a\x15\\a\x18\xBAV[`\0[`\x02\x81\x10\x15a\x17!W`\0a\x15u\x82`\x06a\x1F\x02V[\x90P\x84\x82`\x02\x81\x10a\x15\x89Wa\x15\x89a\x1D!V[` \x02\x01QQ\x83a\x15\x9B\x83`\0a\x1F!V[`\x0C\x81\x10a\x15\xABWa\x15\xABa\x1D!V[` \x02\x01R\x84\x82`\x02\x81\x10a\x15\xC2Wa\x15\xC2a\x1D!V[` \x02\x01Q` \x01Q\x83\x82`\x01a\x15\xD9\x91\x90a\x1F!V[`\x0C\x81\x10a\x15\xE9Wa\x15\xE9a\x1D!V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\0Wa\x16\0a\x1D!V[` \x02\x01QQQ\x83a\x16\x13\x83`\x02a\x1F!V[`\x0C\x81\x10a\x16#Wa\x16#a\x1D!V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16:Wa\x16:a\x1D!V[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x16S\x83`\x03a\x1F!V[`\x0C\x81\x10a\x16cWa\x16ca\x1D!V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16zWa\x16za\x1D!V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a\x16\x95Wa\x16\x95a\x1D!V[` \x02\x01Q\x83a\x16\xA6\x83`\x04a\x1F!V[`\x0C\x81\x10a\x16\xB6Wa\x16\xB6a\x1D!V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\xCDWa\x16\xCDa\x1D!V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x16\xE8Wa\x16\xE8a\x1D!V[` \x02\x01Q\x83a\x16\xF9\x83`\x05a\x1F!V[`\x0C\x81\x10a\x17\tWa\x17\ta\x1D!V[` \x02\x01RP\x80a\x17\x19\x81a\x1E\xE7V[\x91PPa\x15_V[Pa\x17*a\x18\xD9V[`\0` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\x91WP\x80a\x17\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x05SV[PQ\x15\x15\x98\x97PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x17\xBFWP` \x82\x01Q\x15[\x15a\x17\xDDWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x18\"\x91\x90a\x1D\x9EV[a\x18L\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1E\xB9V[\x90R\x92\x91PPV[\x91\x90PV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x18\xA8a\x18\xF7V[\x81R` \x01a\x18\xB5a\x18\xF7V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18TW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19>W`\0\x80\xFD[a\x19G\x82a\x19\x15V[\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x18TW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19qW`\0\x80\xFD[a\x19G\x82a\x19NV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\xB3Wa\x19\xB3a\x19zV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\xE2Wa\x19\xE2a\x19zV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\xFDW`\0\x80\xFD[a\x1A\x06\x83a\x19\x15V[\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A$W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1A8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1AJWa\x1AJa\x19zV[a\x1A\\`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x19\xB9V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x1ArW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1A\xA2W`\0\x80\xFD[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x06\xF0V[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xD3W`\0\x80\xFD[a\x1A\xDC\x83a\x19NV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1A\xFFW`\0\x80\xFD[a\x1B\x08\x84a\x19NV[\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1B!W`\0\x80\xFD[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x83\x85\x03a\x01`\x81\x12\x15a\x1BIW`\0\x80\xFD[a\x1BR\x85a\x19\x15V[\x93Pa\x01\0`\x1F\x19\x82\x01\x12\x15a\x1BgW`\0\x80\xFD[` \x85\x01\x92P`@a\x01\x1F\x19\x82\x01\x12\x15a\x1B\x80W`\0\x80\xFD[Pa\x01 \x84\x01\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1B\xA4W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\xBCW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1B\xD0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1B\xDFW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x1B\xF1W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1CEW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1C#V[P\x90\x96\x95PPPPPPV[`\0`@\x82\x84\x03\x12\x15a\x1CcW`\0\x80\xFD[a\x1Cka\x19\x90V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15a\x1C\x94W`\0\x80\xFD[a\x1C\x9D\x83a\x19\x15V[\x91Pa\x1C\xAC\x84` \x85\x01a\x1CQV[\x90P\x92P\x92\x90PV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R`\0` \x84\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x1C\xF7W\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x1C\xDBV[\x81\x81\x11\x15a\x1D\tW`\0`\x80\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`\x80\x01\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x1DIW`\0\x80\xFD[a\x19G\x83\x83a\x1CQV[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01R`@\x85`\x80\x83\x017`\0`\xC0\x82\x01`\0\x81R`@\x86\x827PPa\x01\0\x81\x01\x92\x90\x92Ra\x01 \x82\x01Ra\x01@\x01\x96\x95PPPPPPV[`\0\x82a\x1D\xBBWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x82`\x1F\x83\x01\x12a\x1D\xD1W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1D\xF4Wa\x1D\xF4a\x19zV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a\x1E\x0BW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x1E%W\x805\x83R` \x92\x83\x01\x92\x01a\x1E\rV[P\x91\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x1EBW`\0\x80\xFD[a\x1EJa\x19\x90V[a\x1ET\x84\x84a\x1D\xC0V[\x81Ra\x1Ec\x84`@\x85\x01a\x1D\xC0V[` \x82\x01R\x93\x92PPPV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`\xC0\x81\x01`@\x83\x81\x84\x017`\x80\x82\x01`\0\x81R`@\x80\x85\x01\x827P`\0\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1E\xCBWa\x1E\xCBa\x1E\xA3V[P\x03\x90V[`\0\x81a\x1E\xDFWa\x1E\xDFa\x1E\xA3V[P`\0\x19\x01\x90V[`\0`\0\x19\x82\x14\x15a\x1E\xFBWa\x1E\xFBa\x1E\xA3V[P`\x01\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1F\x1CWa\x1F\x1Ca\x1E\xA3V[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x1F4Wa\x1F4a\x1E\xA3V[P\x01\x90V\xFEBLSApkRegistry.registerBLSPublic\xA2dipfsX\"\x12 \xEE\xA3<\x8E\x1C|\x91P\xFEi-S\xBC\x83\xE7\x1B\xBB\xEAt\xB7sV{\xC3cPq\x93\xCD\xD5\xAE\xAAdsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106101205760003560e01c80636d14a987116100ad578063d5254a8c11610071578063d5254a8c146103ea578063de29fac01461040a578063df4d09e01461042a578063e8bb9ae614610494578063f4e24fe5146104bd57600080fd5b80636d14a987146103155780637916cea61461033c5780637ff81a871461037d578063a3db80e2146103b0578063bf79ce58146103d757600080fd5b80633fb27952116100f45780633fb27952146101ea57806347b314e8146101fd5780635f61a8841461023e578063605747d51461029a57806368bccaac146102e857600080fd5b8062a1f4cb1461012557806313542a4e1461016657806326d941f21461019d578063377ed99d146101b2575b600080fd5b61014c61013336600461192c565b6003602052600090815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b61018f61017436600461192c565b6001600160a01b031660009081526001602052604090205490565b60405190815260200161015d565b6101b06101ab36600461195f565b6104d0565b005b6101d56101c036600461195f565b60ff1660009081526004602052604090205490565b60405163ffffffff909116815260200161015d565b6101b06101f83660046119ea565b6105e5565b61022661020b366004611a90565b6000908152600260205260409020546001600160a01b031690565b6040516001600160a01b03909116815260200161015d565b61028d61024c36600461195f565b60408051808201909152600080825260208201525060ff16600090815260056020908152604091829020825180840190935280548352600101549082015290565b60405161015d9190611aa9565b6102ad6102a8366004611ac0565b610663565b60408051825167ffffffffffffffff1916815260208084015163ffffffff90811691830191909152928201519092169082015260600161015d565b6102fb6102f6366004611aea565b6106f6565b60405167ffffffffffffffff19909116815260200161015d565b6102267f000000000000000000000000000000000000000000000000000000000000000081565b61034f61034a366004611ac0565b610891565b6040805167ffffffffffffffff19909416845263ffffffff928316602085015291169082015260600161015d565b61039061038b36600461192c565b6108dc565b60408051835181526020938401519381019390935282015260600161015d565b61014c6103be36600461195f565b6005602052600090815260409020805460019091015482565b61018f6103e5366004611b32565b6109a9565b6103fd6103f8366004611b8f565b610dbd565b60405161015d9190611c07565b61018f61041836600461192c565b60016020526000908152604090205481565b6101b0610438366004611c81565b8051600090815260208083018051825260408084206001600160a01b039690961680855260018085528286208890559685526002845281852080546001600160a01b031916821790558452600390925291209151825551910155565b6102266104a2366004611a90565b6002602052600090815260409020546001600160a01b031681565b6101b06104cb3660046119ea565b610fd7565b6104d8611040565b60ff81166000908152600460205260409020541561055c5760405162461bcd60e51b815260206004820152603660248201527f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a6044820152752071756f72756d20616c72656164792065786973747360501b60648201526084015b60405180910390fd5b60ff166000908152600460209081526040808320815160608101835284815263ffffffff4381168286019081528285018781528454600181018655948852959096209151919092018054955194518316600160e01b026001600160e01b0395909316600160c01b026001600160e01b03199096169190931c179390931791909116919091179055565b6105ed611040565b60006105f8836108dc565b50905061060582826110f7565b7f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e83610646856001600160a01b031660009081526001602052604090205490565b8460405161065693929190611cb5565b60405180910390a1505050565b604080516060810182526000808252602080830182905282840182905260ff8616825260049052919091208054839081106106a0576106a0611d21565b600091825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b8204811694830194909452600160e01b90049092169082015290505b92915050565b60ff8316600090815260046020526040812080548291908490811061071d5761071d611d21565b600091825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b82048116948301859052600160e01b9091048116928201929092529250851610156107e45760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a20696e64657820746f6f20726563656e7400006064820152608401610553565b604081015163ffffffff16158061080a5750806040015163ffffffff168463ffffffff16105b6108885760405162461bcd60e51b815260206004820152604360248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a206e6f74206c61746573742061706b2075706460648201526261746560e81b608482015260a401610553565b51949350505050565b600460205281600052604060002081815481106108ad57600080fd5b600091825260209091200154604081901b925063ffffffff600160c01b820481169250600160e01b9091041683565b60408051808201909152600080825260208201526001600160a01b03821660008181526003602090815260408083208151808301835281548152600191820154818501529484529091528120549091908061099f5760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e676574526567697374657265645075626b60448201527f65793a206f70657261746f72206973206e6f74207265676973746572656400006064820152608401610553565b9094909350915050565b60006109b3611040565b60006109e16109ca36869003860160408701611d37565b805160009081526020918201519091526040902090565b90507fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5811415610a69576040805162461bcd60e51b8152602060048201526024810191909152600080516020611f3a83398151915260448201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b65796064820152608401610553565b6001600160a01b03851660009081526001602052604090205415610af35760405162461bcd60e51b81526020600482015260476024820152600080516020611f3a83398151915260448201527f4b65793a206f70657261746f7220616c72656164792072656769737465726564606482015266207075626b657960c81b608482015260a401610553565b6000818152600260205260409020546001600160a01b031615610b775760405162461bcd60e51b81526020600482015260426024820152600080516020611f3a83398151915260448201527f4b65793a207075626c6963206b657920616c7265616479207265676973746572606482015261195960f21b608482015260a401610553565b604080516000917f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000191610bd0918835916020808b0135928b01359160608c01359160808d019160c08e01918d35918e8201359101611d53565b6040516020818303038152906040528051906020012060001c610bf39190611d9e565b9050610c8d610c2c610c1783610c11368a90038a0160408b01611d37565b90611342565b610c2636899003890189611d37565b906113d9565b610c3461146d565b610c76610c6785610c11604080518082018252600080825260209182015281518083019092526001825260029082015290565b610c26368a90038a018a611d37565b610c88368a90038a0160808b01611e30565b61152d565b610d285760405162461bcd60e51b815260206004820152606c6024820152600080516020611f3a83398151915260448201527f4b65793a2065697468657220746865204731207369676e61747572652069732060648201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060848201526b0c8de40dcdee840dac2e8c6d60a31b60a482015260c401610553565b6001600160a01b03861660008181526003602090815260408083208982018035825560608b013560019283015590835281842087905586845260029092529182902080546001600160a01b0319168417905590517fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba382804191610dac9160808a0190611e6f565b60405180910390a250949350505050565b606060008367ffffffffffffffff811115610dda57610dda61197a565b604051908082528060200260200182016040528015610e03578160200160208202803683370190505b50905060005b84811015610fce576000868683818110610e2557610e25611d21565b919091013560f81c6000818152600460205260409020549092509050801580610e88575060ff821660009081526004602052604081208054909190610e6c57610e6c611d21565b600091825260209091200154600160c01b900463ffffffff1686105b15610f155760405162461bcd60e51b815260206004820152605160248201527f424c5341706b52656769737472792e67657441706b496e64696365734174426c60448201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f7265606482015270207468652066697273742075706461746560781b608482015260a401610553565b805b8015610fb85760ff831660009081526004602052604090208790610f3c600184611eb9565b81548110610f4c57610f4c611d21565b600091825260209091200154600160c01b900463ffffffff1611610fa657610f75600182611eb9565b858581518110610f8757610f87611d21565b602002602001019063ffffffff16908163ffffffff1681525050610fb8565b80610fb081611ed0565b915050610f17565b5050508080610fc690611ee7565b915050610e09565b50949350505050565b610fdf611040565b6000610fea836108dc565b509050610fff82610ffa8361179a565b6110f7565b7ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e83610646856001600160a01b031660009081526001602052604090205490565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146110f55760405162461bcd60e51b815260206004820152604e60248201527f424c5341706b52656769737472792e6f6e6c795265676973747279436f6f726460448201527f696e61746f723a2063616c6c6572206973206e6f74207468652072656769737460648201526d393c9031b7b7b93234b730ba37b960911b608482015260a401610553565b565b604080518082019091526000808252602082015260005b835181101561133c57600084828151811061112b5761112b611d21565b0160209081015160f81c60008181526004909252604090912054909150806111bb5760405162461bcd60e51b815260206004820152603d60248201527f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b60448201527f5570646174653a2071756f72756d20646f6573206e6f742065786973740000006064820152608401610553565b60ff821660009081526005602090815260409182902082518084019093528054835260010154908201526111ef90866113d9565b60ff831660008181526005602090815260408083208551808255868401805160019384015590855251835281842094845260049092528220939750919290916112389085611eb9565b8154811061124857611248611d21565b600091825260209091200180549091504363ffffffff908116600160c01b9092041614156112895780546001600160c01b031916604083901c178155611325565b805463ffffffff438116600160e01b8181026001600160e01b0394851617855560ff88166000908152600460209081526040808320815160608101835267ffffffffffffffff198b16815280840196875280830185815282546001810184559286529390942093519301805495519251871690940291909516600160c01b026001600160e01b0319949094169190941c17919091179092161790555b50505050808061133490611ee7565b91505061110e565b50505050565b604080518082019091526000808252602082015261135e611859565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa905080801561139157611393565bfe5b50806113d15760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610553565b505092915050565b60408051808201909152600080825260208201526113f5611877565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa90508080156113915750806113d15760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610553565b611475611895565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b60408051808201825285815260208082018590528251808401909352858352820183905260009161155c6118ba565b60005b6002811015611721576000611575826006611f02565b905084826002811061158957611589611d21565b6020020151518361159b836000611f21565b600c81106115ab576115ab611d21565b60200201528482600281106115c2576115c2611d21565b602002015160200151838260016115d99190611f21565b600c81106115e9576115e9611d21565b602002015283826002811061160057611600611d21565b6020020151515183611613836002611f21565b600c811061162357611623611d21565b602002015283826002811061163a5761163a611d21565b6020020151516001602002015183611653836003611f21565b600c811061166357611663611d21565b602002015283826002811061167a5761167a611d21565b60200201516020015160006002811061169557611695611d21565b6020020151836116a6836004611f21565b600c81106116b6576116b6611d21565b60200201528382600281106116cd576116cd611d21565b6020020151602001516001600281106116e8576116e8611d21565b6020020151836116f9836005611f21565b600c811061170957611709611d21565b6020020152508061171981611ee7565b91505061155f565b5061172a6118d9565b60006020826101808560086107d05a03fa905080801561139157508061178a5760405162461bcd60e51b81526020600482015260156024820152741c185a5c9a5b99cb5bdc18dbd9194b59985a5b1959605a1b6044820152606401610553565b5051151598975050505050505050565b604080518082019091526000808252602082015281511580156117bf57506020820151155b156117dd575050604080518082019091526000808252602082015290565b6040518060400160405280836000015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4784602001516118229190611d9e565b61184c907f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611eb9565b905292915050565b919050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806118a86118f7565b81526020016118b56118f7565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b80356001600160a01b038116811461185457600080fd5b60006020828403121561193e57600080fd5b61194782611915565b9392505050565b803560ff8116811461185457600080fd5b60006020828403121561197157600080fd5b6119478261194e565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff811182821017156119b3576119b361197a565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156119e2576119e261197a565b604052919050565b600080604083850312156119fd57600080fd5b611a0683611915565b915060208084013567ffffffffffffffff80821115611a2457600080fd5b818601915086601f830112611a3857600080fd5b813581811115611a4a57611a4a61197a565b611a5c601f8201601f191685016119b9565b91508082528784828501011115611a7257600080fd5b80848401858401376000848284010152508093505050509250929050565b600060208284031215611aa257600080fd5b5035919050565b8151815260208083015190820152604081016106f0565b60008060408385031215611ad357600080fd5b611adc8361194e565b946020939093013593505050565b600080600060608486031215611aff57600080fd5b611b088461194e565b9250602084013563ffffffff81168114611b2157600080fd5b929592945050506040919091013590565b6000806000838503610160811215611b4957600080fd5b611b5285611915565b9350610100601f1982011215611b6757600080fd5b602085019250604061011f1982011215611b8057600080fd5b50610120840190509250925092565b600080600060408486031215611ba457600080fd5b833567ffffffffffffffff80821115611bbc57600080fd5b818601915086601f830112611bd057600080fd5b813581811115611bdf57600080fd5b876020828501011115611bf157600080fd5b6020928301989097509590910135949350505050565b6020808252825182820181905260009190848201906040850190845b81811015611c4557835163ffffffff1683529284019291840191600101611c23565b50909695505050505050565b600060408284031215611c6357600080fd5b611c6b611990565b9050813581526020820135602082015292915050565b60008060608385031215611c9457600080fd5b611c9d83611915565b9150611cac8460208501611c51565b90509250929050565b60018060a01b038416815260006020848184015260606040840152835180606085015260005b81811015611cf757858101830151858201608001528201611cdb565b81811115611d09576000608083870101525b50601f01601f19169290920160800195945050505050565b634e487b7160e01b600052603260045260246000fd5b600060408284031215611d4957600080fd5b6119478383611c51565b8881528760208201528660408201528560608201526040856080830137600060c082016000815260408682375050610100810192909252610120820152610140019695505050505050565b600082611dbb57634e487b7160e01b600052601260045260246000fd5b500690565b600082601f830112611dd157600080fd5b6040516040810181811067ffffffffffffffff82111715611df457611df461197a565b8060405250806040840185811115611e0b57600080fd5b845b81811015611e25578035835260209283019201611e0d565b509195945050505050565b600060808284031215611e4257600080fd5b611e4a611990565b611e548484611dc0565b8152611e638460408501611dc0565b60208201529392505050565b823581526020808401359082015260c081016040838184013760808201600081526040808501823750600081529392505050565b634e487b7160e01b600052601160045260246000fd5b600082821015611ecb57611ecb611ea3565b500390565b600081611edf57611edf611ea3565b506000190190565b6000600019821415611efb57611efb611ea3565b5060010190565b6000816000190483118215151615611f1c57611f1c611ea3565b500290565b60008219821115611f3457611f34611ea3565b50019056fe424c5341706b52656769737472792e7265676973746572424c535075626c6963a2646970667358221220eea33c8e1c7c9150fe692d53bc83e71bbbea74b773567bc363507193cdd5aeaa64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01 W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\xADW\x80c\xD5%J\x8C\x11a\0qW\x80c\xD5%J\x8C\x14a\x03\xEAW\x80c\xDE)\xFA\xC0\x14a\x04\nW\x80c\xDFM\t\xE0\x14a\x04*W\x80c\xE8\xBB\x9A\xE6\x14a\x04\x94W\x80c\xF4\xE2O\xE5\x14a\x04\xBDW`\0\x80\xFD[\x80cm\x14\xA9\x87\x14a\x03\x15W\x80cy\x16\xCE\xA6\x14a\x03<W\x80c\x7F\xF8\x1A\x87\x14a\x03}W\x80c\xA3\xDB\x80\xE2\x14a\x03\xB0W\x80c\xBFy\xCEX\x14a\x03\xD7W`\0\x80\xFD[\x80c?\xB2yR\x11a\0\xF4W\x80c?\xB2yR\x14a\x01\xEAW\x80cG\xB3\x14\xE8\x14a\x01\xFDW\x80c_a\xA8\x84\x14a\x02>W\x80c`WG\xD5\x14a\x02\x9AW\x80ch\xBC\xCA\xAC\x14a\x02\xE8W`\0\x80\xFD[\x80b\xA1\xF4\xCB\x14a\x01%W\x80c\x13T*N\x14a\x01fW\x80c&\xD9A\xF2\x14a\x01\x9DW\x80c7~\xD9\x9D\x14a\x01\xB2W[`\0\x80\xFD[a\x01La\x0136`\x04a\x19,V[`\x03` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Fa\x01t6`\x04a\x19,V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01]V[a\x01\xB0a\x01\xAB6`\x04a\x19_V[a\x04\xD0V[\0[a\x01\xD5a\x01\xC06`\x04a\x19_V[`\xFF\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01]V[a\x01\xB0a\x01\xF86`\x04a\x19\xEAV[a\x05\xE5V[a\x02&a\x02\x0B6`\x04a\x1A\x90V[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01]V[a\x02\x8Da\x02L6`\x04a\x19_V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\xFF\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`@Qa\x01]\x91\x90a\x1A\xA9V[a\x02\xADa\x02\xA86`\x04a\x1A\xC0V[a\x06cV[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x01]V[a\x02\xFBa\x02\xF66`\x04a\x1A\xEAV[a\x06\xF6V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R` \x01a\x01]V[a\x02&\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Oa\x03J6`\x04a\x1A\xC0V[a\x08\x91V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01]V[a\x03\x90a\x03\x8B6`\x04a\x19,V[a\x08\xDCV[`@\x80Q\x83Q\x81R` \x93\x84\x01Q\x93\x81\x01\x93\x90\x93R\x82\x01R``\x01a\x01]V[a\x01La\x03\xBE6`\x04a\x19_V[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[a\x01\x8Fa\x03\xE56`\x04a\x1B2V[a\t\xA9V[a\x03\xFDa\x03\xF86`\x04a\x1B\x8FV[a\r\xBDV[`@Qa\x01]\x91\x90a\x1C\x07V[a\x01\x8Fa\x04\x186`\x04a\x19,V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xB0a\x0486`\x04a\x1C\x81V[\x80Q`\0\x90\x81R` \x80\x83\x01\x80Q\x82R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16\x80\x85R`\x01\x80\x85R\x82\x86 \x88\x90U\x96\x85R`\x02\x84R\x81\x85 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x82\x17\x90U\x84R`\x03\x90\x92R\x91 \x91Q\x82UQ\x91\x01UV[a\x02&a\x04\xA26`\x04a\x1A\x90V[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xB0a\x04\xCB6`\x04a\x19\xEAV[a\x0F\xD7V[a\x04\xD8a\x10@V[`\xFF\x81\x16`\0\x90\x81R`\x04` R`@\x90 T\x15a\x05\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBLSApkRegistry.initializeQuorum:`D\x82\x01Ru quorum already exists`P\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\xFF\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x84\x81Rc\xFF\xFF\xFF\xFFC\x81\x16\x82\x86\x01\x90\x81R\x82\x85\x01\x87\x81R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x94Q\x83\x16`\x01`\xE0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x95\x90\x93\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x96\x16\x91\x90\x93\x1C\x17\x93\x90\x93\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UV[a\x05\xEDa\x10@V[`\0a\x05\xF8\x83a\x08\xDCV[P\x90Pa\x06\x05\x82\x82a\x10\xF7V[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06F\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[\x84`@Qa\x06V\x93\x92\x91\x90a\x1C\xB5V[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x04\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x06\xA0Wa\x06\xA0a\x1D!V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x07\x1DWa\x07\x1Da\x1D!V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x85\x90R`\x01`\xE0\x1B\x90\x91\x04\x81\x16\x92\x82\x01\x92\x90\x92R\x92P\x85\x16\x10\x15a\x07\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: index too recent\0\0`d\x82\x01R`\x84\x01a\x05SV[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x08\nWP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x08\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: not latest apk upd`d\x82\x01Rbate`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x05SV[Q\x94\x93PPPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x08\xADW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`@\x81\x90\x1B\x92Pc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x92P`\x01`\xE0\x1B\x90\x91\x04\x16\x83V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x91\x82\x01T\x81\x85\x01R\x94\x84R\x90\x91R\x81 T\x90\x91\x90\x80a\t\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry.getRegisteredPubk`D\x82\x01R\x7Fey: operator is not registered\0\0`d\x82\x01R`\x84\x01a\x05SV[\x90\x94\x90\x93P\x91PPV[`\0a\t\xB3a\x10@V[`\0a\t\xE1a\t\xCA6\x86\x90\x03\x86\x01`@\x87\x01a\x1D7V[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x14\x15a\niW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` a\x1F:\x839\x81Q\x91R`D\x82\x01R\x7FKey: cannot register zero pubkey`d\x82\x01R`\x84\x01a\x05SV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\n\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a\x1F:\x839\x81Q\x91R`D\x82\x01R\x7FKey: operator already registered`d\x82\x01Rf pubkey`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05SV[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0BwW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a\x1F:\x839\x81Q\x91R`D\x82\x01R\x7FKey: public key already register`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x05SV[`@\x80Q`\0\x91\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x91a\x0B\xD0\x91\x885\x91` \x80\x8B\x015\x92\x8B\x015\x91``\x8C\x015\x91`\x80\x8D\x01\x91`\xC0\x8E\x01\x91\x8D5\x91\x8E\x82\x015\x91\x01a\x1DSV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x0B\xF3\x91\x90a\x1D\x9EV[\x90Pa\x0C\x8Da\x0C,a\x0C\x17\x83a\x0C\x116\x8A\x90\x03\x8A\x01`@\x8B\x01a\x1D7V[\x90a\x13BV[a\x0C&6\x89\x90\x03\x89\x01\x89a\x1D7V[\x90a\x13\xD9V[a\x0C4a\x14mV[a\x0Cva\x0Cg\x85a\x0C\x11`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[a\x0C&6\x8A\x90\x03\x8A\x01\x8Aa\x1D7V[a\x0C\x886\x8A\x90\x03\x8A\x01`\x80\x8B\x01a\x1E0V[a\x15-V[a\r(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R`\0\x80Q` a\x1F:\x839\x81Q\x91R`D\x82\x01R\x7FKey: either the G1 signature is `d\x82\x01R\x7Fwrong, or G1 and G2 private key `\x84\x82\x01Rk\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x01a\x05SV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x89\x82\x01\x805\x82U``\x8B\x015`\x01\x92\x83\x01U\x90\x83R\x81\x84 \x87\x90U\x86\x84R`\x02\x90\x92R\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90U\x90Q\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x91a\r\xAC\x91`\x80\x8A\x01\x90a\x1EoV[`@Q\x80\x91\x03\x90\xA2P\x94\x93PPPPV[```\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xDAWa\r\xDAa\x19zV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x03W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x0F\xCEW`\0\x86\x86\x83\x81\x81\x10a\x0E%Wa\x0E%a\x1D!V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x04` R`@\x90 T\x90\x92P\x90P\x80\x15\x80a\x0E\x88WP`\xFF\x82\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x90\x91\x90a\x0ElWa\x0Ela\x1D!V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x0F\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FBLSApkRegistry.getApkIndicesAtBl`D\x82\x01R\x7FockNumber: blockNumber is before`d\x82\x01Rp the first update`x\x1B`\x84\x82\x01R`\xA4\x01a\x05SV[\x80[\x80\x15a\x0F\xB8W`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x90 \x87\x90a\x0F<`\x01\x84a\x1E\xB9V[\x81T\x81\x10a\x0FLWa\x0FLa\x1D!V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11a\x0F\xA6Wa\x0Fu`\x01\x82a\x1E\xB9V[\x85\x85\x81Q\x81\x10a\x0F\x87Wa\x0F\x87a\x1D!V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x0F\xB8V[\x80a\x0F\xB0\x81a\x1E\xD0V[\x91PPa\x0F\x17V[PPP\x80\x80a\x0F\xC6\x90a\x1E\xE7V[\x91PPa\x0E\tV[P\x94\x93PPPPV[a\x0F\xDFa\x10@V[`\0a\x0F\xEA\x83a\x08\xDCV[P\x90Pa\x0F\xFF\x82a\x0F\xFA\x83a\x17\x9AV[a\x10\xF7V[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x06F\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R\x7FBLSApkRegistry.onlyRegistryCoord`D\x82\x01R\x7Finator: caller is not the regist`d\x82\x01Rm9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x91\x1B`\x84\x82\x01R`\xA4\x01a\x05SV[V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[\x83Q\x81\x10\x15a\x13<W`\0\x84\x82\x81Q\x81\x10a\x11+Wa\x11+a\x1D!V[\x01` \x90\x81\x01Q`\xF8\x1C`\0\x81\x81R`\x04\x90\x92R`@\x90\x91 T\x90\x91P\x80a\x11\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBLSApkRegistry._processQuorumApk`D\x82\x01R\x7FUpdate: quorum does not exist\0\0\0`d\x82\x01R`\x84\x01a\x05SV[`\xFF\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01Ra\x11\xEF\x90\x86a\x13\xD9V[`\xFF\x83\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x85Q\x80\x82U\x86\x84\x01\x80Q`\x01\x93\x84\x01U\x90\x85RQ\x83R\x81\x84 \x94\x84R`\x04\x90\x92R\x82 \x93\x97P\x91\x92\x90\x91a\x128\x90\x85a\x1E\xB9V[\x81T\x81\x10a\x12HWa\x12Ha\x1D!V[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16`\x01`\xC0\x1B\x90\x92\x04\x16\x14\x15a\x12\x89W\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`@\x83\x90\x1C\x17\x81Ua\x13%V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01`\xE0\x1B\x81\x81\x02`\x01`\x01`\xE0\x1B\x03\x94\x85\x16\x17\x85U`\xFF\x88\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x8B\x16\x81R\x80\x84\x01\x96\x87R\x80\x83\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x93Q\x93\x01\x80T\x95Q\x92Q\x87\x16\x90\x94\x02\x91\x90\x95\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x94\x90\x94\x16\x91\x90\x94\x1C\x17\x91\x90\x91\x17\x90\x92\x16\x17\x90U[PPPP\x80\x80a\x134\x90a\x1E\xE7V[\x91PPa\x11\x0EV[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13^a\x18YV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\x91Wa\x13\x93V[\xFE[P\x80a\x13\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05SV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13\xF5a\x18wV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\x91WP\x80a\x13\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05SV[a\x14ua\x18\x95V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R`\0\x91a\x15\\a\x18\xBAV[`\0[`\x02\x81\x10\x15a\x17!W`\0a\x15u\x82`\x06a\x1F\x02V[\x90P\x84\x82`\x02\x81\x10a\x15\x89Wa\x15\x89a\x1D!V[` \x02\x01QQ\x83a\x15\x9B\x83`\0a\x1F!V[`\x0C\x81\x10a\x15\xABWa\x15\xABa\x1D!V[` \x02\x01R\x84\x82`\x02\x81\x10a\x15\xC2Wa\x15\xC2a\x1D!V[` \x02\x01Q` \x01Q\x83\x82`\x01a\x15\xD9\x91\x90a\x1F!V[`\x0C\x81\x10a\x15\xE9Wa\x15\xE9a\x1D!V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\0Wa\x16\0a\x1D!V[` \x02\x01QQQ\x83a\x16\x13\x83`\x02a\x1F!V[`\x0C\x81\x10a\x16#Wa\x16#a\x1D!V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16:Wa\x16:a\x1D!V[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x16S\x83`\x03a\x1F!V[`\x0C\x81\x10a\x16cWa\x16ca\x1D!V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16zWa\x16za\x1D!V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a\x16\x95Wa\x16\x95a\x1D!V[` \x02\x01Q\x83a\x16\xA6\x83`\x04a\x1F!V[`\x0C\x81\x10a\x16\xB6Wa\x16\xB6a\x1D!V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\xCDWa\x16\xCDa\x1D!V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x16\xE8Wa\x16\xE8a\x1D!V[` \x02\x01Q\x83a\x16\xF9\x83`\x05a\x1F!V[`\x0C\x81\x10a\x17\tWa\x17\ta\x1D!V[` \x02\x01RP\x80a\x17\x19\x81a\x1E\xE7V[\x91PPa\x15_V[Pa\x17*a\x18\xD9V[`\0` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\x91WP\x80a\x17\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x05SV[PQ\x15\x15\x98\x97PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x17\xBFWP` \x82\x01Q\x15[\x15a\x17\xDDWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x18\"\x91\x90a\x1D\x9EV[a\x18L\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1E\xB9V[\x90R\x92\x91PPV[\x91\x90PV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x18\xA8a\x18\xF7V[\x81R` \x01a\x18\xB5a\x18\xF7V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18TW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19>W`\0\x80\xFD[a\x19G\x82a\x19\x15V[\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x18TW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19qW`\0\x80\xFD[a\x19G\x82a\x19NV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\xB3Wa\x19\xB3a\x19zV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\xE2Wa\x19\xE2a\x19zV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\xFDW`\0\x80\xFD[a\x1A\x06\x83a\x19\x15V[\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A$W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1A8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1AJWa\x1AJa\x19zV[a\x1A\\`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x19\xB9V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x1ArW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1A\xA2W`\0\x80\xFD[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x06\xF0V[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xD3W`\0\x80\xFD[a\x1A\xDC\x83a\x19NV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1A\xFFW`\0\x80\xFD[a\x1B\x08\x84a\x19NV[\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1B!W`\0\x80\xFD[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x83\x85\x03a\x01`\x81\x12\x15a\x1BIW`\0\x80\xFD[a\x1BR\x85a\x19\x15V[\x93Pa\x01\0`\x1F\x19\x82\x01\x12\x15a\x1BgW`\0\x80\xFD[` \x85\x01\x92P`@a\x01\x1F\x19\x82\x01\x12\x15a\x1B\x80W`\0\x80\xFD[Pa\x01 \x84\x01\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1B\xA4W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\xBCW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1B\xD0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1B\xDFW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x1B\xF1W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1CEW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1C#V[P\x90\x96\x95PPPPPPV[`\0`@\x82\x84\x03\x12\x15a\x1CcW`\0\x80\xFD[a\x1Cka\x19\x90V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15a\x1C\x94W`\0\x80\xFD[a\x1C\x9D\x83a\x19\x15V[\x91Pa\x1C\xAC\x84` \x85\x01a\x1CQV[\x90P\x92P\x92\x90PV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R`\0` \x84\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x1C\xF7W\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x1C\xDBV[\x81\x81\x11\x15a\x1D\tW`\0`\x80\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`\x80\x01\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x1DIW`\0\x80\xFD[a\x19G\x83\x83a\x1CQV[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01R`@\x85`\x80\x83\x017`\0`\xC0\x82\x01`\0\x81R`@\x86\x827PPa\x01\0\x81\x01\x92\x90\x92Ra\x01 \x82\x01Ra\x01@\x01\x96\x95PPPPPPV[`\0\x82a\x1D\xBBWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x82`\x1F\x83\x01\x12a\x1D\xD1W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1D\xF4Wa\x1D\xF4a\x19zV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a\x1E\x0BW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x1E%W\x805\x83R` \x92\x83\x01\x92\x01a\x1E\rV[P\x91\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x1EBW`\0\x80\xFD[a\x1EJa\x19\x90V[a\x1ET\x84\x84a\x1D\xC0V[\x81Ra\x1Ec\x84`@\x85\x01a\x1D\xC0V[` \x82\x01R\x93\x92PPPV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`\xC0\x81\x01`@\x83\x81\x84\x017`\x80\x82\x01`\0\x81R`@\x80\x85\x01\x827P`\0\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1E\xCBWa\x1E\xCBa\x1E\xA3V[P\x03\x90V[`\0\x81a\x1E\xDFWa\x1E\xDFa\x1E\xA3V[P`\0\x19\x01\x90V[`\0`\0\x19\x82\x14\x15a\x1E\xFBWa\x1E\xFBa\x1E\xA3V[P`\x01\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1F\x1CWa\x1F\x1Ca\x1E\xA3V[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x1F4Wa\x1F4a\x1E\xA3V[P\x01\x90V\xFEBLSApkRegistry.registerBLSPublic\xA2dipfsX\"\x12 \xEE\xA3<\x8E\x1C|\x91P\xFEi-S\xBC\x83\xE7\x1B\xBB\xEAt\xB7sV{\xC3cPq\x93\xCD\xD5\xAE\xAAdsolcC\0\x08\x0C\x003",
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
    /**Event with signature `NewPubkeyRegistration(address,(uint256,uint256),(uint256[2],uint256[2]))` and selector `0xe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba3828041`.
    ```solidity
    event NewPubkeyRegistration(address indexed operator, BN254.G1Point pubkeyG1, BN254.G2Point pubkeyG2);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewPubkeyRegistration {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NewPubkeyRegistration {
            type DataTuple<'a> = (BN254::G1Point, BN254::G2Point);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "NewPubkeyRegistration(address,(uint256,uint256),(uint256[2],uint256[2]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    227u8, 251u8, 102u8, 19u8, 175u8, 46u8, 137u8, 48u8, 207u8, 133u8, 212u8,
                    127u8, 207u8, 109u8, 177u8, 1u8, 146u8, 34u8, 74u8, 100u8, 198u8, 203u8, 232u8,
                    2u8, 62u8, 14u8, 238u8, 27u8, 163u8, 130u8, 128u8, 65u8,
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
                    pubkeyG1: data.0,
                    pubkeyG2: data.1,
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
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.pubkeyG1),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.pubkeyG2),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewPubkeyRegistration {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewPubkeyRegistration> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewPubkeyRegistration) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorAddedToQuorums(address,bytes32,bytes)` and selector `0x73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e`.
    ```solidity
    event OperatorAddedToQuorums(address operator, bytes32 operatorId, bytes quorumNumbers);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorAddedToQuorums {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for OperatorAddedToQuorums {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorAddedToQuorums(address,bytes32,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    115u8, 162u8, 183u8, 251u8, 132u8, 71u8, 36u8, 185u8, 113u8, 128u8, 42u8,
                    233u8, 177u8, 93u8, 176u8, 148u8, 212u8, 183u8, 25u8, 45u8, 249u8, 215u8, 53u8,
                    14u8, 20u8, 235u8, 70u8, 107u8, 155u8, 34u8, 235u8, 78u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: data.0,
                    operatorId: data.1,
                    quorumNumbers: data.2,
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
        impl alloy_sol_types::private::IntoLogData for OperatorAddedToQuorums {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorAddedToQuorums> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorAddedToQuorums) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRemovedFromQuorums(address,bytes32,bytes)` and selector `0xf843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e`.
    ```solidity
    event OperatorRemovedFromQuorums(address operator, bytes32 operatorId, bytes quorumNumbers);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorRemovedFromQuorums {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for OperatorRemovedFromQuorums {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorRemovedFromQuorums(address,bytes32,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    248u8, 67u8, 236u8, 213u8, 58u8, 86u8, 54u8, 117u8, 230u8, 33u8, 7u8, 190u8,
                    20u8, 148u8, 253u8, 222u8, 74u8, 61u8, 73u8, 174u8, 237u8, 175u8, 141u8, 136u8,
                    198u8, 22u8, 216u8, 83u8, 70u8, 227u8, 80u8, 14u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: data.0,
                    operatorId: data.1,
                    quorumNumbers: data.2,
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
        impl alloy_sol_types::private::IntoLogData for OperatorRemovedFromQuorums {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRemovedFromQuorums> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRemovedFromQuorums) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _registryCoordinator);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _registryCoordinator: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._registryCoordinator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                        &self._registryCoordinator,
                    ),
                )
            }
        }
    };
    /**Function with signature `apkHistory(uint8,uint256)` and selector `0x7916cea6`.
    ```solidity
    function apkHistory(uint8, uint256) external view returns (bytes24 apkHash, uint32 updateBlockNumber, uint32 nextUpdateBlockNumber);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct apkHistoryCall {
        pub _0: u8,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`apkHistory(uint8,uint256)`](apkHistoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct apkHistoryReturn {
        pub apkHash: alloy::sol_types::private::FixedBytes<24>,
        pub updateBlockNumber: u32,
        pub nextUpdateBlockNumber: u32,
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
            impl ::core::convert::From<apkHistoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: apkHistoryCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for apkHistoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<24>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<24>, u32, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<apkHistoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: apkHistoryReturn) -> Self {
                    (
                        value.apkHash,
                        value.updateBlockNumber,
                        value.nextUpdateBlockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for apkHistoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        apkHash: tuple.0,
                        updateBlockNumber: tuple.1,
                        nextUpdateBlockNumber: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for apkHistoryCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = apkHistoryReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<24>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "apkHistory(uint8,uint256)";
            const SELECTOR: [u8; 4] = [121u8, 22u8, 206u8, 166u8];
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
    /**Function with signature `currentApk(uint8)` and selector `0xa3db80e2`.
    ```solidity
    function currentApk(uint8) external view returns (uint256 X, uint256 Y);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentApkCall {
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`currentApk(uint8)`](currentApkCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentApkReturn {
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
            impl ::core::convert::From<currentApkCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentApkCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentApkCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
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
            impl ::core::convert::From<currentApkReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentApkReturn) -> Self {
                    (value.X, value.Y)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentApkReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        X: tuple.0,
                        Y: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentApkCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentApkReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentApk(uint8)";
            const SELECTOR: [u8; 4] = [163u8, 219u8, 128u8, 226u8];
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
    /**Function with signature `deregisterOperator(address,bytes)` and selector `0xf4e24fe5`.
    ```solidity
    function deregisterOperator(address operator, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`deregisterOperator(address,bytes)`](deregisterOperatorCall) function.
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
            impl ::core::convert::From<deregisterOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.operator, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorCall {
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator(address,bytes)";
            const SELECTOR: [u8; 4] = [244u8, 226u8, 79u8, 229u8];
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
    /**Function with signature `getApk(uint8)` and selector `0x5f61a884`.
    ```solidity
    function getApk(uint8 quorumNumber) external view returns (BN254.G1Point memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApkCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getApk(uint8)`](getApkCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApkReturn {
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
            impl ::core::convert::From<getApkCall> for UnderlyingRustTuple<'_> {
                fn from(value: getApkCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
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
            impl ::core::convert::From<getApkReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getApkReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getApkCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApkReturn;
            type ReturnTuple<'a> = (BN254::G1Point,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getApk(uint8)";
            const SELECTOR: [u8; 4] = [95u8, 97u8, 168u8, 132u8];
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
    /**Function with signature `getApkHashAtBlockNumberAndIndex(uint8,uint32,uint256)` and selector `0x68bccaac`.
    ```solidity
    function getApkHashAtBlockNumberAndIndex(uint8 quorumNumber, uint32 blockNumber, uint256 index) external view returns (bytes24);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApkHashAtBlockNumberAndIndexCall {
        pub quorumNumber: u8,
        pub blockNumber: u32,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getApkHashAtBlockNumberAndIndex(uint8,uint32,uint256)`](getApkHashAtBlockNumberAndIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApkHashAtBlockNumberAndIndexReturn {
        pub _0: alloy::sol_types::private::FixedBytes<24>,
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
            impl ::core::convert::From<getApkHashAtBlockNumberAndIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getApkHashAtBlockNumberAndIndexCall) -> Self {
                    (value.quorumNumber, value.blockNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkHashAtBlockNumberAndIndexCall {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<24>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<24>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getApkHashAtBlockNumberAndIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getApkHashAtBlockNumberAndIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkHashAtBlockNumberAndIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getApkHashAtBlockNumberAndIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApkHashAtBlockNumberAndIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<24>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getApkHashAtBlockNumberAndIndex(uint8,uint32,uint256)";
            const SELECTOR: [u8; 4] = [104u8, 188u8, 202u8, 172u8];
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
    /**Function with signature `getApkHistoryLength(uint8)` and selector `0x377ed99d`.
    ```solidity
    function getApkHistoryLength(uint8 quorumNumber) external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApkHistoryLengthCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getApkHistoryLength(uint8)`](getApkHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApkHistoryLengthReturn {
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
            impl ::core::convert::From<getApkHistoryLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: getApkHistoryLengthCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkHistoryLengthCall {
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
            impl ::core::convert::From<getApkHistoryLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getApkHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getApkHistoryLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApkHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getApkHistoryLength(uint8)";
            const SELECTOR: [u8; 4] = [55u8, 126u8, 217u8, 157u8];
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
    /**Function with signature `getApkIndicesAtBlockNumber(bytes,uint256)` and selector `0xd5254a8c`.
    ```solidity
    function getApkIndicesAtBlockNumber(bytes memory quorumNumbers, uint256 blockNumber) external view returns (uint32[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApkIndicesAtBlockNumberCall {
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getApkIndicesAtBlockNumber(bytes,uint256)`](getApkIndicesAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApkIndicesAtBlockNumberReturn {
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<getApkIndicesAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getApkIndicesAtBlockNumberCall) -> Self {
                    (value.quorumNumbers, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkIndicesAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumbers: tuple.0,
                        blockNumber: tuple.1,
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
            impl ::core::convert::From<getApkIndicesAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getApkIndicesAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkIndicesAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getApkIndicesAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApkIndicesAtBlockNumberReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getApkIndicesAtBlockNumber(bytes,uint256)";
            const SELECTOR: [u8; 4] = [213u8, 37u8, 74u8, 140u8];
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
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
    /**Function with signature `getApkUpdateAtIndex(uint8,uint256)` and selector `0x605747d5`.
    ```solidity
    function getApkUpdateAtIndex(uint8 quorumNumber, uint256 index) external view returns (IBLSApkRegistry.ApkUpdate memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApkUpdateAtIndexCall {
        pub quorumNumber: u8,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getApkUpdateAtIndex(uint8,uint256)`](getApkUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApkUpdateAtIndexReturn {
        pub _0: <IBLSApkRegistry::ApkUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getApkUpdateAtIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getApkUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkUpdateAtIndexCall {
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
            type UnderlyingSolTuple<'a> = (IBLSApkRegistry::ApkUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IBLSApkRegistry::ApkUpdate as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getApkUpdateAtIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getApkUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getApkUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApkUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IBLSApkRegistry::ApkUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getApkUpdateAtIndex(uint8,uint256)";
            const SELECTOR: [u8; 4] = [96u8, 87u8, 71u8, 213u8];
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
    /**Function with signature `getOperatorFromPubkeyHash(bytes32)` and selector `0x47b314e8`.
    ```solidity
    function getOperatorFromPubkeyHash(bytes32 pubkeyHash) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromPubkeyHashCall {
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getOperatorFromPubkeyHash(bytes32)`](getOperatorFromPubkeyHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromPubkeyHashReturn {
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
            impl ::core::convert::From<getOperatorFromPubkeyHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromPubkeyHashCall) -> Self {
                    (value.pubkeyHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorFromPubkeyHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pubkeyHash: tuple.0,
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
            impl ::core::convert::From<getOperatorFromPubkeyHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromPubkeyHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorFromPubkeyHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorFromPubkeyHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorFromPubkeyHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorFromPubkeyHash(bytes32)";
            const SELECTOR: [u8; 4] = [71u8, 179u8, 20u8, 232u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.pubkeyHash),
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
    /**Function with signature `getRegisteredPubkey(address)` and selector `0x7ff81a87`.
    ```solidity
    function getRegisteredPubkey(address operator) external view returns (BN254.G1Point memory, bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRegisteredPubkeyCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getRegisteredPubkey(address)`](getRegisteredPubkeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRegisteredPubkeyReturn {
        pub _0: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getRegisteredPubkeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRegisteredPubkeyCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRegisteredPubkeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (BN254::G1Point, alloy::sol_types::sol_data::FixedBytes<32>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getRegisteredPubkeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRegisteredPubkeyReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRegisteredPubkeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRegisteredPubkeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRegisteredPubkeyReturn;
            type ReturnTuple<'a> = (BN254::G1Point, alloy::sol_types::sol_data::FixedBytes<32>);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRegisteredPubkey(address)";
            const SELECTOR: [u8; 4] = [127u8, 248u8, 26u8, 135u8];
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
    /**Function with signature `initializeQuorum(uint8)` and selector `0x26d941f2`.
    ```solidity
    function initializeQuorum(uint8 quorumNumber) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeQuorumCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`initializeQuorum(uint8)`](initializeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeQuorumReturn {}
    #[allow(
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
            impl ::core::convert::From<initializeQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
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
            impl ::core::convert::From<initializeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeQuorum(uint8)";
            const SELECTOR: [u8; 4] = [38u8, 217u8, 65u8, 242u8];
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
    /**Function with signature `operatorToPubkey(address)` and selector `0x00a1f4cb`.
    ```solidity
    function operatorToPubkey(address) external view returns (uint256 X, uint256 Y);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorToPubkeyCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorToPubkey(address)`](operatorToPubkeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorToPubkeyReturn {
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
            impl ::core::convert::From<operatorToPubkeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorToPubkeyCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorToPubkeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
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
            impl ::core::convert::From<operatorToPubkeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorToPubkeyReturn) -> Self {
                    (value.X, value.Y)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorToPubkeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        X: tuple.0,
                        Y: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorToPubkeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorToPubkeyReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorToPubkey(address)";
            const SELECTOR: [u8; 4] = [0u8, 161u8, 244u8, 203u8];
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
    /**Function with signature `operatorToPubkeyHash(address)` and selector `0xde29fac0`.
    ```solidity
    function operatorToPubkeyHash(address) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorToPubkeyHashCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorToPubkeyHash(address)`](operatorToPubkeyHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorToPubkeyHashReturn {
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
            impl ::core::convert::From<operatorToPubkeyHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorToPubkeyHashCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorToPubkeyHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<operatorToPubkeyHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorToPubkeyHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorToPubkeyHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorToPubkeyHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorToPubkeyHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorToPubkeyHash(address)";
            const SELECTOR: [u8; 4] = [222u8, 41u8, 250u8, 192u8];
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
    /**Function with signature `pubkeyHashToOperator(bytes32)` and selector `0xe8bb9ae6`.
    ```solidity
    function pubkeyHashToOperator(bytes32) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyHashToOperatorCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`pubkeyHashToOperator(bytes32)`](pubkeyHashToOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyHashToOperatorReturn {
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
            impl ::core::convert::From<pubkeyHashToOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyHashToOperatorCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyHashToOperatorCall {
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
            impl ::core::convert::From<pubkeyHashToOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyHashToOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyHashToOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pubkeyHashToOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyHashToOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pubkeyHashToOperator(bytes32)";
            const SELECTOR: [u8; 4] = [232u8, 187u8, 154u8, 230u8];
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
    /**Function with signature `registerBLSPublicKey(address,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint256,uint256))` and selector `0xbf79ce58`.
    ```solidity
    function registerBLSPublicKey(address operator, IBLSApkRegistry.PubkeyRegistrationParams memory params, BN254.G1Point memory pubkeyRegistrationMessageHash) external returns (bytes32 operatorId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerBLSPublicKeyCall {
        pub operator: alloy::sol_types::private::Address,
        pub params:
            <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
        pub pubkeyRegistrationMessageHash: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerBLSPublicKey(address,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint256,uint256))`](registerBLSPublicKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerBLSPublicKeyReturn {
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
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                IBLSApkRegistry::PubkeyRegistrationParams,
                BN254::G1Point,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerBLSPublicKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerBLSPublicKeyCall) -> Self {
                    (
                        value.operator,
                        value.params,
                        value.pubkeyRegistrationMessageHash,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerBLSPublicKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        params: tuple.1,
                        pubkeyRegistrationMessageHash: tuple.2,
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
            impl ::core::convert::From<registerBLSPublicKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerBLSPublicKeyReturn) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerBLSPublicKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerBLSPublicKeyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                IBLSApkRegistry::PubkeyRegistrationParams,
                BN254::G1Point,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerBLSPublicKeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerBLSPublicKey(address,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint256,uint256))";
            const SELECTOR: [u8; 4] = [191u8, 121u8, 206u8, 88u8];
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
                    <IBLSApkRegistry::PubkeyRegistrationParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(
                        &self.pubkeyRegistrationMessageHash,
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
    /**Function with signature `registerOperator(address,bytes)` and selector `0x3fb27952`.
    ```solidity
    function registerOperator(address operator, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(address,bytes)`](registerOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorReturn {}
    #[allow(
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
            impl ::core::convert::From<registerOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.operator, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorCall {
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
            impl ::core::convert::From<registerOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(address,bytes)";
            const SELECTOR: [u8; 4] = [63u8, 178u8, 121u8, 82u8];
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
    /**Function with signature `setBLSPublicKey(address,(uint256,uint256))` and selector `0xdf4d09e0`.
    ```solidity
    function setBLSPublicKey(address account, BN254.G1Point memory pk) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBLSPublicKeyCall {
        pub account: alloy::sol_types::private::Address,
        pub pk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setBLSPublicKey(address,(uint256,uint256))`](setBLSPublicKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBLSPublicKeyReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address, BN254::G1Point);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<setBLSPublicKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: setBLSPublicKeyCall) -> Self {
                    (value.account, value.pk)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBLSPublicKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        pk: tuple.1,
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
            impl ::core::convert::From<setBLSPublicKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setBLSPublicKeyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBLSPublicKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBLSPublicKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, BN254::G1Point);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBLSPublicKeyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBLSPublicKey(address,(uint256,uint256))";
            const SELECTOR: [u8; 4] = [223u8, 77u8, 9u8, 224u8];
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
                        &self.account,
                    ),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.pk),
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
    ///Container for all the [`BLSApkRegistryHarness`](self) function calls.
    pub enum BLSApkRegistryHarnessCalls {
        apkHistory(apkHistoryCall),
        currentApk(currentApkCall),
        deregisterOperator(deregisterOperatorCall),
        getApk(getApkCall),
        getApkHashAtBlockNumberAndIndex(getApkHashAtBlockNumberAndIndexCall),
        getApkHistoryLength(getApkHistoryLengthCall),
        getApkIndicesAtBlockNumber(getApkIndicesAtBlockNumberCall),
        getApkUpdateAtIndex(getApkUpdateAtIndexCall),
        getOperatorFromPubkeyHash(getOperatorFromPubkeyHashCall),
        getOperatorId(getOperatorIdCall),
        getRegisteredPubkey(getRegisteredPubkeyCall),
        initializeQuorum(initializeQuorumCall),
        operatorToPubkey(operatorToPubkeyCall),
        operatorToPubkeyHash(operatorToPubkeyHashCall),
        pubkeyHashToOperator(pubkeyHashToOperatorCall),
        registerBLSPublicKey(registerBLSPublicKeyCall),
        registerOperator(registerOperatorCall),
        registryCoordinator(registryCoordinatorCall),
        setBLSPublicKey(setBLSPublicKeyCall),
    }
    #[automatically_derived]
    impl BLSApkRegistryHarnessCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 161u8, 244u8, 203u8],
            [19u8, 84u8, 42u8, 78u8],
            [38u8, 217u8, 65u8, 242u8],
            [55u8, 126u8, 217u8, 157u8],
            [63u8, 178u8, 121u8, 82u8],
            [71u8, 179u8, 20u8, 232u8],
            [95u8, 97u8, 168u8, 132u8],
            [96u8, 87u8, 71u8, 213u8],
            [104u8, 188u8, 202u8, 172u8],
            [109u8, 20u8, 169u8, 135u8],
            [121u8, 22u8, 206u8, 166u8],
            [127u8, 248u8, 26u8, 135u8],
            [163u8, 219u8, 128u8, 226u8],
            [191u8, 121u8, 206u8, 88u8],
            [213u8, 37u8, 74u8, 140u8],
            [222u8, 41u8, 250u8, 192u8],
            [223u8, 77u8, 9u8, 224u8],
            [232u8, 187u8, 154u8, 230u8],
            [244u8, 226u8, 79u8, 229u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BLSApkRegistryHarnessCalls {
        const NAME: &'static str = "BLSApkRegistryHarnessCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 19usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::apkHistory(_) => <apkHistoryCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::currentApk(_) => <currentApkCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getApk(_) => <getApkCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getApkHashAtBlockNumberAndIndex(_) => {
                    <getApkHashAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getApkHistoryLength(_) => {
                    <getApkHistoryLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getApkIndicesAtBlockNumber(_) => {
                    <getApkIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getApkUpdateAtIndex(_) => {
                    <getApkUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorFromPubkeyHash(_) => {
                    <getOperatorFromPubkeyHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorId(_) => <getOperatorIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getRegisteredPubkey(_) => {
                    <getRegisteredPubkeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeQuorum(_) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorToPubkey(_) => {
                    <operatorToPubkeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorToPubkeyHash(_) => {
                    <operatorToPubkeyHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pubkeyHashToOperator(_) => {
                    <pubkeyHashToOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerBLSPublicKey(_) => {
                    <registerBLSPublicKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator(_) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setBLSPublicKey(_) => {
                    <setBLSPublicKeyCall as alloy_sol_types::SolCall>::SELECTOR
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
                -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls>] = &[
                {
                    fn operatorToPubkey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <operatorToPubkeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::operatorToPubkey)
                    }
                    operatorToPubkey
                },
                {
                    fn getOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <getOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::getOperatorId)
                    }
                    getOperatorId
                },
                {
                    fn initializeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <initializeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::initializeQuorum)
                    }
                    initializeQuorum
                },
                {
                    fn getApkHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <getApkHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::getApkHistoryLength)
                    }
                    getApkHistoryLength
                },
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn getOperatorFromPubkeyHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <getOperatorFromPubkeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::getOperatorFromPubkeyHash)
                    }
                    getOperatorFromPubkeyHash
                },
                {
                    fn getApk(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <getApkCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(BLSApkRegistryHarnessCalls::getApk)
                    }
                    getApk
                },
                {
                    fn getApkUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <getApkUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::getApkUpdateAtIndex)
                    }
                    getApkUpdateAtIndex
                },
                {
                    fn getApkHashAtBlockNumberAndIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <getApkHashAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                BLSApkRegistryHarnessCalls::getApkHashAtBlockNumberAndIndex,
                            )
                    }
                    getApkHashAtBlockNumberAndIndex
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn apkHistory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <apkHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(BLSApkRegistryHarnessCalls::apkHistory)
                    }
                    apkHistory
                },
                {
                    fn getRegisteredPubkey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <getRegisteredPubkeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::getRegisteredPubkey)
                    }
                    getRegisteredPubkey
                },
                {
                    fn currentApk(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <currentApkCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(BLSApkRegistryHarnessCalls::currentApk)
                    }
                    currentApk
                },
                {
                    fn registerBLSPublicKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <registerBLSPublicKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::registerBLSPublicKey)
                    }
                    registerBLSPublicKey
                },
                {
                    fn getApkIndicesAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <getApkIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BLSApkRegistryHarnessCalls::getApkIndicesAtBlockNumber)
                    }
                    getApkIndicesAtBlockNumber
                },
                {
                    fn operatorToPubkeyHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <operatorToPubkeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::operatorToPubkeyHash)
                    }
                    operatorToPubkeyHash
                },
                {
                    fn setBLSPublicKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <setBLSPublicKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::setBLSPublicKey)
                    }
                    setBLSPublicKey
                },
                {
                    fn pubkeyHashToOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <pubkeyHashToOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::pubkeyHashToOperator)
                    }
                    pubkeyHashToOperator
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryHarnessCalls::deregisterOperator)
                    }
                    deregisterOperator
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
                Self::apkHistory(inner) => {
                    <apkHistoryCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::currentApk(inner) => {
                    <currentApkCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getApk(inner) => {
                    <getApkCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getApkHashAtBlockNumberAndIndex(inner) => {
                    <getApkHashAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getApkHistoryLength(inner) => {
                    <getApkHistoryLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getApkIndicesAtBlockNumber(inner) => {
                    <getApkIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getApkUpdateAtIndex(inner) => {
                    <getApkUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorFromPubkeyHash(inner) => {
                    <getOperatorFromPubkeyHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorId(inner) => {
                    <getOperatorIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRegisteredPubkey(inner) => {
                    <getRegisteredPubkeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeQuorum(inner) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorToPubkey(inner) => {
                    <operatorToPubkeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorToPubkeyHash(inner) => {
                    <operatorToPubkeyHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pubkeyHashToOperator(inner) => {
                    <pubkeyHashToOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerBLSPublicKey(inner) => {
                    <registerBLSPublicKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setBLSPublicKey(inner) => {
                    <setBLSPublicKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::apkHistory(inner) => {
                    <apkHistoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentApk(inner) => {
                    <currentApkCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getApk(inner) => {
                    <getApkCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getApkHashAtBlockNumberAndIndex(inner) => {
                    <getApkHashAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getApkHistoryLength(inner) => {
                    <getApkHistoryLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getApkIndicesAtBlockNumber(inner) => {
                    <getApkIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getApkUpdateAtIndex(inner) => {
                    <getApkUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorFromPubkeyHash(inner) => {
                    <getOperatorFromPubkeyHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getRegisteredPubkey(inner) => {
                    <getRegisteredPubkeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeQuorum(inner) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorToPubkey(inner) => {
                    <operatorToPubkeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorToPubkeyHash(inner) => {
                    <operatorToPubkeyHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pubkeyHashToOperator(inner) => {
                    <pubkeyHashToOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerBLSPublicKey(inner) => {
                    <registerBLSPublicKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setBLSPublicKey(inner) => {
                    <setBLSPublicKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`BLSApkRegistryHarness`](self) events.
    pub enum BLSApkRegistryHarnessEvents {
        Initialized(Initialized),
        NewPubkeyRegistration(NewPubkeyRegistration),
        OperatorAddedToQuorums(OperatorAddedToQuorums),
        OperatorRemovedFromQuorums(OperatorRemovedFromQuorums),
    }
    #[automatically_derived]
    impl BLSApkRegistryHarnessEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                115u8, 162u8, 183u8, 251u8, 132u8, 71u8, 36u8, 185u8, 113u8, 128u8, 42u8, 233u8,
                177u8, 93u8, 176u8, 148u8, 212u8, 183u8, 25u8, 45u8, 249u8, 215u8, 53u8, 14u8,
                20u8, 235u8, 70u8, 107u8, 155u8, 34u8, 235u8, 78u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                227u8, 251u8, 102u8, 19u8, 175u8, 46u8, 137u8, 48u8, 207u8, 133u8, 212u8, 127u8,
                207u8, 109u8, 177u8, 1u8, 146u8, 34u8, 74u8, 100u8, 198u8, 203u8, 232u8, 2u8, 62u8,
                14u8, 238u8, 27u8, 163u8, 130u8, 128u8, 65u8,
            ],
            [
                248u8, 67u8, 236u8, 213u8, 58u8, 86u8, 54u8, 117u8, 230u8, 33u8, 7u8, 190u8, 20u8,
                148u8, 253u8, 222u8, 74u8, 61u8, 73u8, 174u8, 237u8, 175u8, 141u8, 136u8, 198u8,
                22u8, 216u8, 83u8, 70u8, 227u8, 80u8, 14u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for BLSApkRegistryHarnessEvents {
        const NAME: &'static str = "BLSApkRegistryHarnessEvents";
        const COUNT: usize = 4usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<NewPubkeyRegistration as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewPubkeyRegistration as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NewPubkeyRegistration)
                }
                Some(<OperatorAddedToQuorums as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorAddedToQuorums as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorAddedToQuorums)
                }
                Some(<OperatorRemovedFromQuorums as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorRemovedFromQuorums as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorRemovedFromQuorums)
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
    impl alloy_sol_types::private::IntoLogData for BLSApkRegistryHarnessEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewPubkeyRegistration(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorAddedToQuorums(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRemovedFromQuorums(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewPubkeyRegistration(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorAddedToQuorums(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRemovedFromQuorums(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BLSApkRegistryHarness`](self) contract instance.

    See the [wrapper's documentation](`BLSApkRegistryHarnessInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BLSApkRegistryHarnessInstance<T, P, N> {
        BLSApkRegistryHarnessInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<BLSApkRegistryHarnessInstance<T, P, N>>,
    > {
        BLSApkRegistryHarnessInstance::<T, P, N>::deploy(provider, _registryCoordinator)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        BLSApkRegistryHarnessInstance::<T, P, N>::deploy_builder(provider, _registryCoordinator)
    }
    /**A [`BLSApkRegistryHarness`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BLSApkRegistryHarness`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BLSApkRegistryHarnessInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BLSApkRegistryHarnessInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BLSApkRegistryHarnessInstance")
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
        > BLSApkRegistryHarnessInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BLSApkRegistryHarness`](self) contract instance.

        See the [wrapper's documentation](`BLSApkRegistryHarnessInstance`) for more details.*/
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
            _registryCoordinator: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<BLSApkRegistryHarnessInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _registryCoordinator);
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
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _registryCoordinator,
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
    impl<T, P: ::core::clone::Clone, N> BLSApkRegistryHarnessInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BLSApkRegistryHarnessInstance<T, P, N> {
            BLSApkRegistryHarnessInstance {
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
        > BLSApkRegistryHarnessInstance<T, P, N>
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
        ///Creates a new call builder for the [`apkHistory`] function.
        pub fn apkHistory(
            &self,
            _0: u8,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, apkHistoryCall, N> {
            self.call_builder(&apkHistoryCall { _0, _1 })
        }
        ///Creates a new call builder for the [`currentApk`] function.
        pub fn currentApk(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, currentApkCall, N> {
            self.call_builder(&currentApkCall { _0 })
        }
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(&deregisterOperatorCall {
                operator,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`getApk`] function.
        pub fn getApk(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getApkCall, N> {
            self.call_builder(&getApkCall { quorumNumber })
        }
        ///Creates a new call builder for the [`getApkHashAtBlockNumberAndIndex`] function.
        pub fn getApkHashAtBlockNumberAndIndex(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getApkHashAtBlockNumberAndIndexCall, N> {
            self.call_builder(&getApkHashAtBlockNumberAndIndexCall {
                quorumNumber,
                blockNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`getApkHistoryLength`] function.
        pub fn getApkHistoryLength(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getApkHistoryLengthCall, N> {
            self.call_builder(&getApkHistoryLengthCall { quorumNumber })
        }
        ///Creates a new call builder for the [`getApkIndicesAtBlockNumber`] function.
        pub fn getApkIndicesAtBlockNumber(
            &self,
            quorumNumbers: alloy::sol_types::private::Bytes,
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getApkIndicesAtBlockNumberCall, N> {
            self.call_builder(&getApkIndicesAtBlockNumberCall {
                quorumNumbers,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getApkUpdateAtIndex`] function.
        pub fn getApkUpdateAtIndex(
            &self,
            quorumNumber: u8,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getApkUpdateAtIndexCall, N> {
            self.call_builder(&getApkUpdateAtIndexCall {
                quorumNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`getOperatorFromPubkeyHash`] function.
        pub fn getOperatorFromPubkeyHash(
            &self,
            pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorFromPubkeyHashCall, N> {
            self.call_builder(&getOperatorFromPubkeyHashCall { pubkeyHash })
        }
        ///Creates a new call builder for the [`getOperatorId`] function.
        pub fn getOperatorId(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorIdCall, N> {
            self.call_builder(&getOperatorIdCall { operator })
        }
        ///Creates a new call builder for the [`getRegisteredPubkey`] function.
        pub fn getRegisteredPubkey(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRegisteredPubkeyCall, N> {
            self.call_builder(&getRegisteredPubkeyCall { operator })
        }
        ///Creates a new call builder for the [`initializeQuorum`] function.
        pub fn initializeQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeQuorumCall, N> {
            self.call_builder(&initializeQuorumCall { quorumNumber })
        }
        ///Creates a new call builder for the [`operatorToPubkey`] function.
        pub fn operatorToPubkey(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorToPubkeyCall, N> {
            self.call_builder(&operatorToPubkeyCall { _0 })
        }
        ///Creates a new call builder for the [`operatorToPubkeyHash`] function.
        pub fn operatorToPubkeyHash(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorToPubkeyHashCall, N> {
            self.call_builder(&operatorToPubkeyHashCall { _0 })
        }
        ///Creates a new call builder for the [`pubkeyHashToOperator`] function.
        pub fn pubkeyHashToOperator(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, pubkeyHashToOperatorCall, N> {
            self.call_builder(&pubkeyHashToOperatorCall { _0 })
        }
        ///Creates a new call builder for the [`registerBLSPublicKey`] function.
        pub fn registerBLSPublicKey(
            &self,
            operator: alloy::sol_types::private::Address,
            params: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
            pubkeyRegistrationMessageHash: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerBLSPublicKeyCall, N> {
            self.call_builder(&registerBLSPublicKeyCall {
                operator,
                params,
                pubkeyRegistrationMessageHash,
            })
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(&registerOperatorCall {
                operator,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`setBLSPublicKey`] function.
        pub fn setBLSPublicKey(
            &self,
            account: alloy::sol_types::private::Address,
            pk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setBLSPublicKeyCall, N> {
            self.call_builder(&setBLSPublicKeyCall { account, pk })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > BLSApkRegistryHarnessInstance<T, P, N>
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
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`NewPubkeyRegistration`] event.
        pub fn NewPubkeyRegistration_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, NewPubkeyRegistration, N> {
            self.event_filter::<NewPubkeyRegistration>()
        }
        ///Creates a new event filter for the [`OperatorAddedToQuorums`] event.
        pub fn OperatorAddedToQuorums_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorAddedToQuorums, N> {
            self.event_filter::<OperatorAddedToQuorums>()
        }
        ///Creates a new event filter for the [`OperatorRemovedFromQuorums`] event.
        pub fn OperatorRemovedFromQuorums_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRemovedFromQuorums, N> {
            self.event_filter::<OperatorRemovedFromQuorums>()
        }
    }
}
