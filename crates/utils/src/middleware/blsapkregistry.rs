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
    struct ApkUpdate { bytes24 apkHash; uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; }
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

interface BLSApkRegistry {
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
    clippy::style
)]
pub mod BLSApkRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
<<<<<<< HEAD:crates/utils/src/blsapkregistry.rs
    ///0x60a03461011a57601f611ce538819003918201601f19168301916001600160401b0383118484101761011e5780849260209460405283398101031261011a57516001600160a01b0381169081900361011a576080525f5460ff8160081c166100c55760ff8082161061008b575b604051611bb29081610133823960805181818161070f01526113b60152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f61006c565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8062a1f4cb1461012957806313542a4e146100e857806326d941f214610124578063377ed99d1461011f5780633fb279521461011a57806347b314e8146100e35780635f61a88414610115578063605747d51461011057806368bccaac1461010b5780636d14a987146101065780637916cea6146101015780637ff81a87146100fc578063a3db80e2146100f7578063bf79ce58146100f2578063d5254a8c146100ed578063de29fac0146100e8578063e8bb9ae6146100e35763f4e24fe5146100de575f80fd5b610b85565b6104a5565b6101b1565b610b18565b61087d565b610832565b6107f1565b61077e565b6106fa565b6105c0565b610533565b6104d1565b61041e565b6102d0565b6101f9565b61015c565b600435906001600160a01b038216820361014457565b5f80fd5b35906001600160a01b038216820361014457565b34610144576020366003190112610144576001600160a01b0361017d61012e565b165f52600360205260405f2060018154910154906101ad6040519283928360209093929193604081019481520152565b0390f35b34610144576020366003190112610144576001600160a01b036101d261012e565b165f526001602052602060405f2054604051908152f35b6004359060ff8216820361014457565b34610144576020366003190112610144576102126101e9565b61021a6113b4565b60ff81165f52600460205260405f205461026c5761024661026a9160ff165f52600460205260405f2090565b61024e61037b565b5f81524363ffffffff166020820152905b5f6040830152610bd8565b005b60405162461bcd60e51b815260206004820152603660248201527f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a6044820152752071756f72756d20616c72656164792065786973747360501b6064820152608490fd5b346101445760203660031901126101445760ff6102eb6101e9565b165f526004602052602063ffffffff60405f205416604051908152f35b634e487b7160e01b5f52604160045260245ffd5b6040810190811067ffffffffffffffff82111761033857604052565b610308565b6060810190811067ffffffffffffffff82111761033857604052565b90601f8019910116810190811067ffffffffffffffff82111761033857604052565b6040519061038a606083610359565b565b9061038a6040519283610359565b906040600319830112610144576103b16004610148565b9160243567ffffffffffffffff811161014457816023820112156101445780600401359067ffffffffffffffff821161033857604051926103fc601f8401601f191660200185610359565b8284526024838301011161014457815f92602460209301838601378301015290565b34610144577f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e61044d3661039a565b906104566113b4565b61046961046282610d7c565b50836114ed565b60018060a01b0381165f52600160205261048e60405f20549260405193849384610c4d565b0390a1005b60209060031901126101445760043590565b34610144576104b336610493565b5f526002602052602060018060a01b0360405f205416604051908152f35b346101445760203660031901126101445760ff6104ec6101e9565b6104f4610c8d565b50165f5260056020526040805f2060018251916105108361031c565b80548352015460208201526105318251809260208091805184520151910152565bf35b34610144576040366003190112610144576105886105826105526101e9565b60ff602435915f604080516105668161033d565b8281528260208201520152165f52600460205260405f20610769565b50610cc3565b604051809163ffffffff6040606084019267ffffffffffffffff19815116855282602082015116602086015201511660408301520390f35b34610144576060366003190112610144576105d96101e9565b6024359063ffffffff82168092036101445761058261060f9160ff6105fd60443590565b91165f52600460205260405f20610769565b9063ffffffff602083015116811061069057816106556106649261063d60406101ad96015163ffffffff1690565b9063ffffffff821615918215610680575b5050610cfc565b5167ffffffffffffffff191690565b60405167ffffffffffffffff1990911681529081906020820190565b63ffffffff161190505f8061064e565b608460405162461bcd60e51b815260206004820152604060248201527f424c5341706b52656769737472792e67657441706b486173684174426c6f636b60448201527f4e756d626572416e64496e6465783a20696e64657820746f6f20726563656e746064820152fd5b34610144575f366003190112610144576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b634e487b7160e01b5f52603260045260245ffd5b805415610764575f5260205f20905f90565b61073e565b8054821015610764575f5260205f2001905f90565b34610144576040366003190112610144576107976101e9565b60ff60243591165f52600460205260405f20908154811015610144576107bc91610769565b50546040805182821b67ffffffffffffffff1916815260c083901c63ffffffff16602082015260e09290921c90820152606090f35b3461014457602036600319011261014457606061081461080f61012e565b610d7c565b61082b604051809360208091805184520151910152565b6040820152f35b346101445760203660031901126101445760ff61084d6101e9565b165f52600560205260405f2060018154910154906101ad6040519283928360209093929193604081019481520152565b34610144576101603660031901126101445761089761012e565b61010036602319011261014457604036610123190112610144576101ad906108bd6113b4565b6108db6108c936610e37565b80515f526020015160205260405f2090565b906109087fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5831415610eb2565b6001600160a01b0381165f90815260016020526040902061092a905415610f10565b5f8281526002602052604090205461094b906001600160a01b031615610f7f565b604051610a1290610a0d906109b790602081019061098e8161098061014435610124356084356064356044356024358a610fe9565b03601f198101835282610359565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b6109db6109c336610e5f565b6109d5836109d036610e37565b6116b5565b906116fb565b906109fd6109e7611783565b916109d56109f436610e87565b916109d061187a565b90610a073661106d565b9261195f565b6110a6565b6001600160a01b0381165f908152600360205260409020610a3c9060643581556001608435910155565b6001600160a01b0381165f908152600160205260409020829055610a8b81610a6c845f52600260205260405f2090565b80546001600160a01b0319166001600160a01b03909216919091179055565b6040516001600160a01b03909116907fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba38280419080610ac681611140565b0390a26040519081529081906020820190565b60206040818301928281528451809452019201905f5b818110610afc5750505090565b825163ffffffff16845260209384019390920191600101610aef565b346101445760403660031901126101445760043567ffffffffffffffff8111610144573660238201121561014457806004013567ffffffffffffffff8111610144573660248284010111610144576101ad91610b7991602480359201611201565b60405191829182610ad9565b34610144577ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e610bb43661039a565b90610bbd6113b4565b610469610bd2610bcc83610d7c565b50611a9c565b836114ed565b80546801000000000000000081101561033857610bfa91600182018155610769565b610c3a578151602083015160409384015163ffffffff60c01b60c09290921b919091169190931c1760e09290921b6001600160e01b031916919091179055565b634e487b7160e01b5f525f60045260245ffd5b919260809360209260018060a01b0316845282840152606060408401528051918291826060860152018484015e5f828201840152601f01601f1916010190565b60405190610c9a8261031c565b5f6020838281520152565b90604051610cb28161031c565b602060018294805484520154910152565b90604051610cd08161033d565b604081935467ffffffffffffffff1981831b16835263ffffffff8160c01c16602084015260e01c910152565b15610d0357565b60405162461bcd60e51b815260206004820152604560248201527f424c5341706b52656769737472792e67657441706b486173684174426c6f636b60448201527f4e756d626572416e64496e6465783a206e6f74206c61746573742061706b2075606482015264706461746560d81b608482015260a490fd5b610d84610c8d565b5060018060a01b031690815f52600360205260405f2091600160405193610daa8561031c565b80548552015460208401525f52600160205260405f2054918215610dcc579190565b60405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e676574526567697374657265645075626b60448201527f65793a206f70657261746f72206973206e6f74207265676973746572656400006064820152608490fd5b60409060631901126101445760405190610e508261031c565b60643582526084356020830152565b60409060231901126101445760405190610e788261031c565b60243582526044356020830152565b6040906101231901126101445760405190610ea18261031c565b610124358252610144356020830152565b15610eb957565b608460405162461bcd60e51b815260206004820152604060248201525f516020611b5d5f395f51905f5260448201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b65796064820152fd5b15610f1757565b60405162461bcd60e51b815260206004820152604760248201525f516020611b5d5f395f51905f5260448201527f4b65793a206f70657261746f7220616c72656164792072656769737465726564606482015266207075626b657960c81b608482015260a490fd5b15610f8657565b60405162461bcd60e51b815260206004820152604260248201525f516020611b5d5f395f51905f5260448201527f4b65793a207075626c6963206b657920616c7265616479207265676973746572606482015261195960f21b608482015260a490fd5b949290916101409694928652602086015260408501526060840152604060a46080850137604060e460c08501376101008301526101208201520190565b9080601f830112156101445760405191611041604084610359565b82906040810192831161014457905b82821061105d5750505090565b8135815260209182019101611050565b90608060a319830112610144576040516110868161031c565b60206110a182946110988160a4611026565b845260e4611026565b910152565b156110ad57565b60405162461bcd60e51b815260206004820152606c60248201525f516020611b5d5f395f51905f5260448201527f4b65793a2065697468657220746865204731207369676e61747572652069732060648201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060848201526b0c8de40dcdee840dac2e8c6d60a31b60a482015260c490fd5b90604060e4608060c0850194606435815260843560208201528360a4818301370137565b67ffffffffffffffff81116103385760051b60200190565b9061118682611164565b6111936040519182610359565b82815280926111a4601f1991611164565b0190602036910137565b90821015610764570190565b634e487b7160e01b5f52601160045260245ffd5b80156111da575f190190565b6111ba565b5f198101919082116111da57565b80518210156107645760209160051b010190565b91909161120d8361117c565b925f5b81811061121e575050505090565b61124361123d61122f8385876111ae565b356001600160f81b03191690565b60f81c90565b6112588160ff165f52600460205260405f2090565b5480158015611389575b61130457805b611277575b5050600101611210565b8563ffffffff6112b56112a76112988660ff165f52600460205260405f2090565b6112a1866111df565b90610769565b505460c01c63ffffffff1690565b1611156112cb576112c5906111ce565b80611268565b60019291506112e86112df6112fd926111df565b63ffffffff1690565b6112f283896111ed565b9063ffffffff169052565b905f61126d565b60405162461bcd60e51b815260206004820152605160248201527f424c5341706b52656769737472792e67657441706b496e64696365734174426c60448201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f7265606482015270207468652066697273742075706461746560781b608482015260a490fd5b506113ad6112df6112a76113a88560ff165f52600460205260405f2090565b610752565b8610611262565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036113e657565b60405162461bcd60e51b815260206004820152605060248201527f424c5341706b52656769737472792e5f636865636b5265676973747279436f6f60448201527f7264696e61746f723a2063616c6c6572206973206e6f7420746865207265676960648201526f39ba393c9031b7b7b93234b730ba37b960811b608482015260a490fd5b908151811015610764570160200190565b1561148257565b60405162461bcd60e51b815260206004820152603d60248201527f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b60448201527f5570646174653a2071756f72756d20646f6573206e6f742065786973740000006064820152608490fd5b91906114f7610c8d565b504363ffffffff16905f5b845181101561168257808361152d61123d61151f6001958a61146a565b516001600160f81b03191690565b6115428160ff165f52600460205260405f2090565b549061154f82151561147b565b6115d56115b66115a861157e896115796115748760ff165f52600560205260405f2090565b610ca5565b6116fb565b6108c9816115978760ff165f52600560205260405f2090565b906020600191805184550151910155565b67ffffffffffffffff191690565b926112a16115cf8460ff165f52600460205260405f2090565b916111df565b5090836115ed6112df845463ffffffff9060c01c1690565b03611616575061161092509060401c67ffffffffffffffff60c01b825416179055565b01611502565b81546001600160e01b031660e09490941b6001600160e01b03191693909317905561167d916116509060ff165f52600460205260405f2090565b61166c61165b61037b565b67ffffffffffffffff199093168352565b63ffffffff8716602083015261025f565b611610565b5050509050565b6040519061018061169a8184610359565b368337565b604051906116ae602083610359565b6020368337565b919060409060606116c4610c8d565b94859260208551926116d68585610359565b8436853780518452015160208301528482015260076107cf195a01fa156116f957565bfe5b60209291608060409261170c610c8d565b9586938186519361171d8686610359565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa80156116f9571561174e57565b60405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606490fd5b60405161178f8161031c565b604090815161179e8382610359565b82368237815260208251916117b38484610359565b83368437015280516117c58282610359565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061181b8383610359565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d602083015261187083519384610359565b8252602082015290565b611882610c8d565b5060405161188f8161031c565b600181526002602082015290565b906006820291808304600614901517156111da57565b9060028110156107645760051b0190565b90600182018092116111da57565b90600282018092116111da57565b90600382018092116111da57565b90600482018092116111da57565b90600582018092116111da57565b90600c8110156107645760051b0190565b1561192257565b60405162461bcd60e51b81526020600482015260156024820152741c185a5c9a5b99cb5bdc18dbd9194b59985a5b1959605a1b6044820152606490fd5b92909161196c604061038c565b938452602084015261197e604061038c565b918252602082015261198e611689565b915f5b600281106119cb575050506020610180916119aa61169f565b92839160086107cf195a01fa80156116f9576119c59061191b565b51151590565b806119d760019261189d565b6119e182856118b3565b51516119ed828861190a565b5260206119fa83866118b3565b510151611a0f611a09836118c4565b8861190a565b52611a1a82866118b3565b515151611a29611a09836118d2565b52611a3f611a3783876118b3565b515160200190565b51611a4c611a09836118e0565b526020611a5983876118b3565b51015151611a69611a09836118ee565b52611a95611a8f611a886020611a7f868a6118b3565b51015160200190565b51926118fc565b8761190a565b5201611991565b611aa4610c8d565b50805190811580611b50575b15611ad1575050604051611ac5604082610359565b5f81525f602082015290565b60207f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47910151067f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47037f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4781116111da5760405191611870604084610359565b50602081015115611ab056fe424c5341706b52656769737472792e7265676973746572424c535075626c6963a26469706673582212202624d7cdb25e6d39e98062d9d507ac66c332152af6fd28180e86f1cc3f64648564736f6c634300081b0033
=======
    ///0x60a06040523480156200001157600080fd5b506040516200210b3803806200210b833981016040819052620000349162000116565b6001600160a01b038116608052806200004c62000054565b505062000148565b600054610100900460ff1615620000c15760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff908116101562000114576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6000602082840312156200012957600080fd5b81516001600160a01b03811681146200014157600080fd5b9392505050565b608051611f8b620001806000396000818161030f01528181610466015281816105bf015281816109c501526110310152611f8b6000f3fe608060405234801561001057600080fd5b50600436106101155760003560e01c80636d14a987116100a2578063bf79ce5811610071578063bf79ce58146103cc578063d5254a8c146103df578063de29fac0146103ff578063e8bb9ae61461041f578063f4e24fe51461044857600080fd5b80636d14a9871461030a5780637916cea6146103315780637ff81a8714610372578063a3db80e2146103a557600080fd5b80633fb27952116100e95780633fb27952146101df57806347b314e8146101f25780635f61a88414610233578063605747d51461028f57806368bccaac146102dd57600080fd5b8062a1f4cb1461011a57806313542a4e1461015b57806326d941f214610192578063377ed99d146101a7575b600080fd5b610141610128366004611904565b6003602052600090815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b610184610169366004611904565b6001600160a01b031660009081526001602052604090205490565b604051908152602001610152565b6101a56101a0366004611937565b61045b565b005b6101ca6101b5366004611937565b60ff1660009081526004602052604090205490565b60405163ffffffff9091168152602001610152565b6101a56101ed3660046119c2565b6105b4565b61021b610200366004611a68565b6000908152600260205260409020546001600160a01b031690565b6040516001600160a01b039091168152602001610152565b610282610241366004611937565b60408051808201909152600080825260208201525060ff16600090815260056020908152604091829020825180840190935280548352600101549082015290565b6040516101529190611a81565b6102a261029d366004611a98565b610672565b60408051825167ffffffffffffffff1916815260208084015163ffffffff908116918301919091529282015190921690820152606001610152565b6102f06102eb366004611ac2565b610705565b60405167ffffffffffffffff199091168152602001610152565b61021b7f000000000000000000000000000000000000000000000000000000000000000081565b61034461033f366004611a98565b6108a0565b6040805167ffffffffffffffff19909416845263ffffffff9283166020850152911690820152606001610152565b610385610380366004611904565b6108eb565b604080518351815260209384015193810193909352820152606001610152565b6101416103b3366004611937565b6005602052600090815260409020805460019091015482565b6101846103da366004611b0a565b6109b8565b6103f26103ed366004611b67565b610e0c565b6040516101529190611bdf565b61018461040d366004611904565b60016020526000908152604090205481565b61021b61042d366004611a68565b6002602052600090815260409020546001600160a01b031681565b6101a56104563660046119c2565b611026565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146104ac5760405162461bcd60e51b81526004016104a390611c29565b60405180910390fd5b60ff81166000908152600460205260409020541561052b5760405162461bcd60e51b815260206004820152603660248201527f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a6044820152752071756f72756d20616c72656164792065786973747360501b60648201526084016104a3565b60ff166000908152600460209081526040808320815160608101835284815263ffffffff4381168286019081528285018781528454600181018655948852959096209151919092018054955194518316600160e01b026001600160e01b0395909316600160c01b026001600160e01b03199096169190931c179390931791909116919091179055565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146105fc5760405162461bcd60e51b81526004016104a390611c29565b6000610607836108eb565b50905061061482826110cf565b7f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e83610655856001600160a01b031660009081526001602052604090205490565b8460405161066593929190611c9d565b60405180910390a1505050565b604080516060810182526000808252602080830182905282840182905260ff8616825260049052919091208054839081106106af576106af611d09565b600091825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b8204811694830194909452600160e01b90049092169082015290505b92915050565b60ff8316600090815260046020526040812080548291908490811061072c5761072c611d09565b600091825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b82048116948301859052600160e01b9091048116928201929092529250851610156107f35760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a20696e64657820746f6f20726563656e74000060648201526084016104a3565b604081015163ffffffff1615806108195750806040015163ffffffff168463ffffffff16105b6108975760405162461bcd60e51b815260206004820152604360248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a206e6f74206c61746573742061706b2075706460648201526261746560e81b608482015260a4016104a3565b51949350505050565b600460205281600052604060002081815481106108bc57600080fd5b600091825260209091200154604081901b925063ffffffff600160c01b820481169250600160e01b9091041683565b60408051808201909152600080825260208201526001600160a01b0382166000818152600360209081526040808320815180830183528154815260019182015481850152948452909152812054909190806109ae5760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e676574526567697374657265645075626b60448201527f65793a206f70657261746f72206973206e6f742072656769737465726564000060648201526084016104a3565b9094909350915050565b6000336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a025760405162461bcd60e51b81526004016104a390611c29565b6000610a30610a1936869003860160408701611d1f565b805160009081526020918201519091526040902090565b90507fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5811415610ab8576040805162461bcd60e51b8152602060048201526024810191909152600080516020611f3683398151915260448201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b657960648201526084016104a3565b6001600160a01b03851660009081526001602052604090205415610b425760405162461bcd60e51b81526020600482015260476024820152600080516020611f3683398151915260448201527f4b65793a206f70657261746f7220616c72656164792072656769737465726564606482015266207075626b657960c81b608482015260a4016104a3565b6000818152600260205260409020546001600160a01b031615610bc65760405162461bcd60e51b81526020600482015260426024820152600080516020611f3683398151915260448201527f4b65793a207075626c6963206b657920616c7265616479207265676973746572606482015261195960f21b608482015260a4016104a3565b604080516000917f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000191610c1f918835916020808b0135928b01359160608c01359160808d019160c08e01918d35918e8201359101611d51565b6040516020818303038152906040528051906020012060001c610c429190611d9c565b9050610cdc610c7b610c6683610c60368a90038a0160408b01611d1f565b9061131a565b610c7536899003890189611d1f565b906113b1565b610c83611445565b610cc5610cb685610c60604080518082018252600080825260209182015281518083019092526001825260029082015290565b610c75368a90038a018a611d1f565b610cd7368a90038a0160808b01611e0e565b611505565b610d775760405162461bcd60e51b815260206004820152606c6024820152600080516020611f3683398151915260448201527f4b65793a2065697468657220746865204731207369676e61747572652069732060648201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060848201526b0c8de40dcdee840dac2e8c6d60a31b60a482015260c4016104a3565b6001600160a01b03861660008181526003602090815260408083208982018035825560608b013560019283015590835281842087905586845260029092529182902080546001600160a01b0319168417905590517fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba382804191610dfb9160808a0190611e6b565b60405180910390a250949350505050565b606060008367ffffffffffffffff811115610e2957610e29611952565b604051908082528060200260200182016040528015610e52578160200160208202803683370190505b50905060005b8481101561101d576000868683818110610e7457610e74611d09565b919091013560f81c6000818152600460205260409020549092509050801580610ed7575060ff821660009081526004602052604081208054909190610ebb57610ebb611d09565b600091825260209091200154600160c01b900463ffffffff1686105b15610f645760405162461bcd60e51b815260206004820152605160248201527f424c5341706b52656769737472792e67657441706b496e64696365734174426c60448201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f7265606482015270207468652066697273742075706461746560781b608482015260a4016104a3565b805b80156110075760ff831660009081526004602052604090208790610f8b600184611eb5565b81548110610f9b57610f9b611d09565b600091825260209091200154600160c01b900463ffffffff1611610ff557610fc4600182611eb5565b858581518110610fd657610fd6611d09565b602002602001019063ffffffff16908163ffffffff1681525050611007565b80610fff81611ecc565b915050610f66565b505050808061101590611ee3565b915050610e58565b50949350505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461106e5760405162461bcd60e51b81526004016104a390611c29565b6000611079836108eb565b50905061108e8261108983611772565b6110cf565b7ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e83610655856001600160a01b031660009081526001602052604090205490565b604080518082019091526000808252602082015260005b835181101561131457600084828151811061110357611103611d09565b0160209081015160f81c60008181526004909252604090912054909150806111935760405162461bcd60e51b815260206004820152603d60248201527f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b60448201527f5570646174653a2071756f72756d20646f6573206e6f7420657869737400000060648201526084016104a3565b60ff821660009081526005602090815260409182902082518084019093528054835260010154908201526111c790866113b1565b60ff831660008181526005602090815260408083208551808255868401805160019384015590855251835281842094845260049092528220939750919290916112109085611eb5565b8154811061122057611220611d09565b600091825260209091200180549091504363ffffffff908116600160c01b9092041614156112615780546001600160c01b031916604083901c1781556112fd565b805463ffffffff438116600160e01b8181026001600160e01b0394851617855560ff88166000908152600460209081526040808320815160608101835267ffffffffffffffff198b16815280840196875280830185815282546001810184559286529390942093519301805495519251871690940291909516600160c01b026001600160e01b0319949094169190941c17919091179092161790555b50505050808061130c90611ee3565b9150506110e6565b50505050565b6040805180820190915260008082526020820152611336611831565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa90508080156113695761136b565bfe5b50806113a95760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b60448201526064016104a3565b505092915050565b60408051808201909152600080825260208201526113cd61184f565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa90508080156113695750806113a95760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b60448201526064016104a3565b61144d61186d565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b604080518082018252858152602080820185905282518084019093528583528201839052600091611534611892565b60005b60028110156116f957600061154d826006611efe565b905084826002811061156157611561611d09565b60200201515183611573836000611f1d565b600c811061158357611583611d09565b602002015284826002811061159a5761159a611d09565b602002015160200151838260016115b19190611f1d565b600c81106115c1576115c1611d09565b60200201528382600281106115d8576115d8611d09565b60200201515151836115eb836002611f1d565b600c81106115fb576115fb611d09565b602002015283826002811061161257611612611d09565b602002015151600160200201518361162b836003611f1d565b600c811061163b5761163b611d09565b602002015283826002811061165257611652611d09565b60200201516020015160006002811061166d5761166d611d09565b60200201518361167e836004611f1d565b600c811061168e5761168e611d09565b60200201528382600281106116a5576116a5611d09565b6020020151602001516001600281106116c0576116c0611d09565b6020020151836116d1836005611f1d565b600c81106116e1576116e1611d09565b602002015250806116f181611ee3565b915050611537565b506117026118b1565b60006020826101808560086107d05a03fa90508080156113695750806117625760405162461bcd60e51b81526020600482015260156024820152741c185a5c9a5b99cb5bdc18dbd9194b59985a5b1959605a1b60448201526064016104a3565b5051151598975050505050505050565b6040805180820190915260008082526020820152815115801561179757506020820151155b156117b5575050604080518082019091526000808252602082015290565b6040518060400160405280836000015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4784602001516117fa9190611d9c565b611824907f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611eb5565b905292915050565b919050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806118806118cf565b815260200161188d6118cf565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b80356001600160a01b038116811461182c57600080fd5b60006020828403121561191657600080fd5b61191f826118ed565b9392505050565b803560ff8116811461182c57600080fd5b60006020828403121561194957600080fd5b61191f82611926565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff8111828210171561198b5761198b611952565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156119ba576119ba611952565b604052919050565b600080604083850312156119d557600080fd5b6119de836118ed565b915060208084013567ffffffffffffffff808211156119fc57600080fd5b818601915086601f830112611a1057600080fd5b813581811115611a2257611a22611952565b611a34601f8201601f19168501611991565b91508082528784828501011115611a4a57600080fd5b80848401858401376000848284010152508093505050509250929050565b600060208284031215611a7a57600080fd5b5035919050565b8151815260208083015190820152604081016106ff565b60008060408385031215611aab57600080fd5b611ab483611926565b946020939093013593505050565b600080600060608486031215611ad757600080fd5b611ae084611926565b9250602084013563ffffffff81168114611af957600080fd5b929592945050506040919091013590565b6000806000838503610160811215611b2157600080fd5b611b2a856118ed565b9350610100601f1982011215611b3f57600080fd5b602085019250604061011f1982011215611b5857600080fd5b50610120840190509250925092565b600080600060408486031215611b7c57600080fd5b833567ffffffffffffffff80821115611b9457600080fd5b818601915086601f830112611ba857600080fd5b813581811115611bb757600080fd5b876020828501011115611bc957600080fd5b6020928301989097509590910135949350505050565b6020808252825182820181905260009190848201906040850190845b81811015611c1d57835163ffffffff1683529284019291840191600101611bfb565b50909695505050505050565b6020808252604e908201527f424c5341706b52656769737472792e6f6e6c795265676973747279436f6f726460408201527f696e61746f723a2063616c6c6572206973206e6f74207468652072656769737460608201526d393c9031b7b7b93234b730ba37b960911b608082015260a00190565b60018060a01b038416815260006020848184015260606040840152835180606085015260005b81811015611cdf57858101830151858201608001528201611cc3565b81811115611cf1576000608083870101525b50601f01601f19169290920160800195945050505050565b634e487b7160e01b600052603260045260246000fd5b600060408284031215611d3157600080fd5b611d39611968565b82358152602083013560208201528091505092915050565b8881528760208201528660408201528560608201526040856080830137600060c082016000815260408682375050610100810192909252610120820152610140019695505050505050565b600082611db957634e487b7160e01b600052601260045260246000fd5b500690565b600082601f830112611dcf57600080fd5b611dd7611968565b806040840185811115611de957600080fd5b845b81811015611e03578035845260209384019301611deb565b509095945050505050565b600060808284031215611e2057600080fd5b6040516040810181811067ffffffffffffffff82111715611e4357611e43611952565b604052611e508484611dbe565b8152611e5f8460408501611dbe565b60208201529392505050565b823581526020808401359082015260c081016040838184013760808201600081526040808501823750600081529392505050565b634e487b7160e01b600052601160045260246000fd5b600082821015611ec757611ec7611e9f565b500390565b600081611edb57611edb611e9f565b506000190190565b6000600019821415611ef757611ef7611e9f565b5060010190565b6000816000190483118215151615611f1857611f18611e9f565b500290565b60008219821115611f3057611f30611e9f565b50019056fe424c5341706b52656769737472792e7265676973746572424c535075626c6963a264697066735822122050dd2ab396214965939c9bae094a62350c112ffe5817e69d31274d001e7dcf2164736f6c634300080c0033
>>>>>>> dev:crates/utils/src/middleware/blsapkregistry.rs
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
<<<<<<< HEAD:crates/utils/src/blsapkregistry.rs
        b"`\xA04a\x01\x1AW`\x1Fa\x1C\xE58\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\x1EW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\x01\x1AWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x01\x1AW`\x80R_T`\xFF\x81`\x08\x1C\x16a\0\xC5W`\xFF\x80\x82\x16\x10a\0\x8BW[`@Qa\x1B\xB2\x90\x81a\x013\x829`\x80Q\x81\x81\x81a\x07\x0F\x01Ra\x13\xB6\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0lV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\xA1\xF4\xCB\x14a\x01)W\x80c\x13T*N\x14a\0\xE8W\x80c&\xD9A\xF2\x14a\x01$W\x80c7~\xD9\x9D\x14a\x01\x1FW\x80c?\xB2yR\x14a\x01\x1AW\x80cG\xB3\x14\xE8\x14a\0\xE3W\x80c_a\xA8\x84\x14a\x01\x15W\x80c`WG\xD5\x14a\x01\x10W\x80ch\xBC\xCA\xAC\x14a\x01\x0BW\x80cm\x14\xA9\x87\x14a\x01\x06W\x80cy\x16\xCE\xA6\x14a\x01\x01W\x80c\x7F\xF8\x1A\x87\x14a\0\xFCW\x80c\xA3\xDB\x80\xE2\x14a\0\xF7W\x80c\xBFy\xCEX\x14a\0\xF2W\x80c\xD5%J\x8C\x14a\0\xEDW\x80c\xDE)\xFA\xC0\x14a\0\xE8W\x80c\xE8\xBB\x9A\xE6\x14a\0\xE3Wc\xF4\xE2O\xE5\x14a\0\xDEW_\x80\xFD[a\x0B\x85V[a\x04\xA5V[a\x01\xB1V[a\x0B\x18V[a\x08}V[a\x082V[a\x07\xF1V[a\x07~V[a\x06\xFAV[a\x05\xC0V[a\x053V[a\x04\xD1V[a\x04\x1EV[a\x02\xD0V[a\x01\xF9V[a\x01\\V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01DWV[_\x80\xFD[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01DWV[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW`\x01`\x01`\xA0\x1B\x03a\x01}a\x01.V[\x16_R`\x03` R`@_ `\x01\x81T\x91\x01T\x90a\x01\xAD`@Q\x92\x83\x92\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xF3[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW`\x01`\x01`\xA0\x1B\x03a\x01\xD2a\x01.V[\x16_R`\x01` R` `@_ T`@Q\x90\x81R\xF3[`\x045\x90`\xFF\x82\x16\x82\x03a\x01DWV[4a\x01DW` 6`\x03\x19\x01\x12a\x01DWa\x02\x12a\x01\xE9V[a\x02\x1Aa\x13\xB4V[`\xFF\x81\x16_R`\x04` R`@_ Ta\x02lWa\x02Fa\x02j\x91`\xFF\x16_R`\x04` R`@_ \x90V[a\x02Na\x03{V[_\x81RCc\xFF\xFF\xFF\xFF\x16` \x82\x01R\x90[_`@\x83\x01Ra\x0B\xD8V[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBLSApkRegistry.initializeQuorum:`D\x82\x01Ru quorum already exists`P\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW`\xFFa\x02\xEBa\x01\xE9V[\x16_R`\x04` R` c\xFF\xFF\xFF\xFF`@_ T\x16`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x038W`@RV[a\x03\x08V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x038W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x038W`@RV[`@Q\x90a\x03\x8A``\x83a\x03YV[V[\x90a\x03\x8A`@Q\x92\x83a\x03YV[\x90`@`\x03\x19\x83\x01\x12a\x01DWa\x03\xB1`\x04a\x01HV[\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01DW\x81`#\x82\x01\x12\x15a\x01DW\x80`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x038W`@Q\x92a\x03\xFC`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x03YV[\x82\x84R`$\x83\x83\x01\x01\x11a\x01DW\x81_\x92`$` \x93\x01\x83\x86\x017\x83\x01\x01R\x90V[4a\x01DW\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBNa\x04M6a\x03\x9AV[\x90a\x04Va\x13\xB4V[a\x04ia\x04b\x82a\r|V[P\x83a\x14\xEDV[`\x01\x80`\xA0\x1B\x03\x81\x16_R`\x01` Ra\x04\x8E`@_ T\x92`@Q\x93\x84\x93\x84a\x0CMV[\x03\x90\xA1\0[` \x90`\x03\x19\x01\x12a\x01DW`\x045\x90V[4a\x01DWa\x04\xB36a\x04\x93V[_R`\x02` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW`\xFFa\x04\xECa\x01\xE9V[a\x04\xF4a\x0C\x8DV[P\x16_R`\x05` R`@\x80_ `\x01\x82Q\x91a\x05\x10\x83a\x03\x1CV[\x80T\x83R\x01T` \x82\x01Ra\x051\x82Q\x80\x92` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\xF3[4a\x01DW`@6`\x03\x19\x01\x12a\x01DWa\x05\x88a\x05\x82a\x05Ra\x01\xE9V[`\xFF`$5\x91_`@\x80Qa\x05f\x81a\x03=V[\x82\x81R\x82` \x82\x01R\x01R\x16_R`\x04` R`@_ a\x07iV[Pa\x0C\xC3V[`@Q\x80\x91c\xFF\xFF\xFF\xFF`@``\x84\x01\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x01Q\x16`@\x83\x01R\x03\x90\xF3[4a\x01DW``6`\x03\x19\x01\x12a\x01DWa\x05\xD9a\x01\xE9V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x01DWa\x05\x82a\x06\x0F\x91`\xFFa\x05\xFD`D5\x90V[\x91\x16_R`\x04` R`@_ a\x07iV[\x90c\xFF\xFF\xFF\xFF` \x83\x01Q\x16\x81\x10a\x06\x90W\x81a\x06Ua\x06d\x92a\x06=`@a\x01\xAD\x96\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90c\xFF\xFF\xFF\xFF\x82\x16\x15\x91\x82\x15a\x06\x80W[PPa\x0C\xFCV[Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[c\xFF\xFF\xFF\xFF\x16\x11\x90P_\x80a\x06NV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FBLSApkRegistry.getApkHashAtBlock`D\x82\x01R\x7FNumberAndIndex: index too recent`d\x82\x01R\xFD[4a\x01DW_6`\x03\x19\x01\x12a\x01DW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x15a\x07dW_R` _ \x90_\x90V[a\x07>V[\x80T\x82\x10\x15a\x07dW_R` _ \x01\x90_\x90V[4a\x01DW`@6`\x03\x19\x01\x12a\x01DWa\x07\x97a\x01\xE9V[`\xFF`$5\x91\x16_R`\x04` R`@_ \x90\x81T\x81\x10\x15a\x01DWa\x07\xBC\x91a\x07iV[PT`@\x80Q\x82\x82\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R`\xC0\x83\x90\x1Cc\xFF\xFF\xFF\xFF\x16` \x82\x01R`\xE0\x92\x90\x92\x1C\x90\x82\x01R``\x90\xF3[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW``a\x08\x14a\x08\x0Fa\x01.V[a\r|V[a\x08+`@Q\x80\x93` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[`@\x82\x01R\xF3[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW`\xFFa\x08Ma\x01\xE9V[\x16_R`\x05` R`@_ `\x01\x81T\x91\x01T\x90a\x01\xAD`@Q\x92\x83\x92\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[4a\x01DWa\x01`6`\x03\x19\x01\x12a\x01DWa\x08\x97a\x01.V[a\x01\x006`#\x19\x01\x12a\x01DW`@6a\x01#\x19\x01\x12a\x01DWa\x01\xAD\x90a\x08\xBDa\x13\xB4V[a\x08\xDBa\x08\xC96a\x0E7V[\x80Q_R` \x01Q` R`@_ \x90V[\x90a\t\x08\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x83\x14\x15a\x0E\xB2V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01` R`@\x90 a\t*\x90T\x15a\x0F\x10V[_\x82\x81R`\x02` R`@\x90 Ta\tK\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\x7FV[`@Qa\n\x12\x90a\n\r\x90a\t\xB7\x90` \x81\x01\x90a\t\x8E\x81a\t\x80a\x01D5a\x01$5`\x845`d5`D5`$5\x8Aa\x0F\xE9V[\x03`\x1F\x19\x81\x01\x83R\x82a\x03YV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[a\t\xDBa\t\xC36a\x0E_V[a\t\xD5\x83a\t\xD06a\x0E7V[a\x16\xB5V[\x90a\x16\xFBV[\x90a\t\xFDa\t\xE7a\x17\x83V[\x91a\t\xD5a\t\xF46a\x0E\x87V[\x91a\t\xD0a\x18zV[\x90a\n\x076a\x10mV[\x92a\x19_V[a\x10\xA6V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 a\n<\x90`d5\x81U`\x01`\x845\x91\x01UV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01` R`@\x90 \x82\x90Ua\n\x8B\x81a\nl\x84_R`\x02` R`@_ \x90V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x90\x80a\n\xC6\x81a\x11@V[\x03\x90\xA2`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\n\xFCWPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\xEFV[4a\x01DW`@6`\x03\x19\x01\x12a\x01DW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01DW6`#\x82\x01\x12\x15a\x01DW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01DW6`$\x82\x84\x01\x01\x11a\x01DWa\x01\xAD\x91a\x0By\x91`$\x805\x92\x01a\x12\x01V[`@Q\x91\x82\x91\x82a\n\xD9V[4a\x01DW\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0Ea\x0B\xB46a\x03\x9AV[\x90a\x0B\xBDa\x13\xB4V[a\x04ia\x0B\xD2a\x0B\xCC\x83a\r|V[Pa\x1A\x9CV[\x83a\x14\xEDV[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x038Wa\x0B\xFA\x91`\x01\x82\x01\x81Ua\x07iV[a\x0C:W\x81Q` \x83\x01Q`@\x93\x84\x01Qc\xFF\xFF\xFF\xFF`\xC0\x1B`\xC0\x92\x90\x92\x1B\x91\x90\x91\x16\x91\x90\x93\x1C\x17`\xE0\x92\x90\x92\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x91\x92`\x80\x93` \x92`\x01\x80`\xA0\x1B\x03\x16\x84R\x82\x84\x01R```@\x84\x01R\x80Q\x91\x82\x91\x82``\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q\x90a\x0C\x9A\x82a\x03\x1CV[_` \x83\x82\x81R\x01RV[\x90`@Qa\x0C\xB2\x81a\x03\x1CV[` `\x01\x82\x94\x80T\x84R\x01T\x91\x01RV[\x90`@Qa\x0C\xD0\x81a\x03=V[`@\x81\x93Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x83\x1B\x16\x83Rc\xFF\xFF\xFF\xFF\x81`\xC0\x1C\x16` \x84\x01R`\xE0\x1C\x91\x01RV[\x15a\r\x03WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FBLSApkRegistry.getApkHashAtBlock`D\x82\x01R\x7FNumberAndIndex: not latest apk u`d\x82\x01Rdpdate`\xD8\x1B`\x84\x82\x01R`\xA4\x90\xFD[a\r\x84a\x0C\x8DV[P`\x01\x80`\xA0\x1B\x03\x16\x90\x81_R`\x03` R`@_ \x91`\x01`@Q\x93a\r\xAA\x85a\x03\x1CV[\x80T\x85R\x01T` \x84\x01R_R`\x01` R`@_ T\x91\x82\x15a\r\xCCW\x91\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry.getRegisteredPubk`D\x82\x01R\x7Fey: operator is not registered\0\0`d\x82\x01R`\x84\x90\xFD[`@\x90`c\x19\x01\x12a\x01DW`@Q\x90a\x0EP\x82a\x03\x1CV[`d5\x82R`\x845` \x83\x01RV[`@\x90`#\x19\x01\x12a\x01DW`@Q\x90a\x0Ex\x82a\x03\x1CV[`$5\x82R`D5` \x83\x01RV[`@\x90a\x01#\x19\x01\x12a\x01DW`@Q\x90a\x0E\xA1\x82a\x03\x1CV[a\x01$5\x82Ra\x01D5` \x83\x01RV[\x15a\x0E\xB9WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R_Q` a\x1B]_9_Q\x90_R`D\x82\x01R\x7FKey: cannot register zero pubkey`d\x82\x01R\xFD[\x15a\x0F\x17WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R_Q` a\x1B]_9_Q\x90_R`D\x82\x01R\x7FKey: operator already registered`d\x82\x01Rf pubkey`\xC8\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a\x0F\x86WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R_Q` a\x1B]_9_Q\x90_R`D\x82\x01R\x7FKey: public key already register`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x94\x92\x90\x91a\x01@\x96\x94\x92\x86R` \x86\x01R`@\x85\x01R``\x84\x01R`@`\xA4`\x80\x85\x017`@`\xE4`\xC0\x85\x017a\x01\0\x83\x01Ra\x01 \x82\x01R\x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x01DW`@Q\x91a\x10A`@\x84a\x03YV[\x82\x90`@\x81\x01\x92\x83\x11a\x01DW\x90[\x82\x82\x10a\x10]WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x10PV[\x90`\x80`\xA3\x19\x83\x01\x12a\x01DW`@Qa\x10\x86\x81a\x03\x1CV[` a\x10\xA1\x82\x94a\x10\x98\x81`\xA4a\x10&V[\x84R`\xE4a\x10&V[\x91\x01RV[\x15a\x10\xADWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R_Q` a\x1B]_9_Q\x90_R`D\x82\x01R\x7FKey: either the G1 signature is `d\x82\x01R\x7Fwrong, or G1 and G2 private key `\x84\x82\x01Rk\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90`@`\xE4`\x80`\xC0\x85\x01\x94`d5\x81R`\x845` \x82\x01R\x83`\xA4\x81\x83\x017\x017V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x038W`\x05\x1B` \x01\x90V[\x90a\x11\x86\x82a\x11dV[a\x11\x93`@Q\x91\x82a\x03YV[\x82\x81R\x80\x92a\x11\xA4`\x1F\x19\x91a\x11dV[\x01\x90` 6\x91\x017V[\x90\x82\x10\x15a\x07dW\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x15a\x11\xDAW_\x19\x01\x90V[a\x11\xBAV[_\x19\x81\x01\x91\x90\x82\x11a\x11\xDAWV[\x80Q\x82\x10\x15a\x07dW` \x91`\x05\x1B\x01\x01\x90V[\x91\x90\x91a\x12\r\x83a\x11|V[\x92_[\x81\x81\x10a\x12\x1EWPPPP\x90V[a\x12Ca\x12=a\x12/\x83\x85\x87a\x11\xAEV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[a\x12X\x81`\xFF\x16_R`\x04` R`@_ \x90V[T\x80\x15\x80\x15a\x13\x89W[a\x13\x04W\x80[a\x12wW[PP`\x01\x01a\x12\x10V[\x85c\xFF\xFF\xFF\xFFa\x12\xB5a\x12\xA7a\x12\x98\x86`\xFF\x16_R`\x04` R`@_ \x90V[a\x12\xA1\x86a\x11\xDFV[\x90a\x07iV[PT`\xC0\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[\x16\x11\x15a\x12\xCBWa\x12\xC5\x90a\x11\xCEV[\x80a\x12hV[`\x01\x92\x91Pa\x12\xE8a\x12\xDFa\x12\xFD\x92a\x11\xDFV[c\xFF\xFF\xFF\xFF\x16\x90V[a\x12\xF2\x83\x89a\x11\xEDV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x90_a\x12mV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FBLSApkRegistry.getApkIndicesAtBl`D\x82\x01R\x7FockNumber: blockNumber is before`d\x82\x01Rp the first update`x\x1B`\x84\x82\x01R`\xA4\x90\xFD[Pa\x13\xADa\x12\xDFa\x12\xA7a\x13\xA8\x85`\xFF\x16_R`\x04` R`@_ \x90V[a\x07RV[\x86\x10a\x12bV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x13\xE6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FBLSApkRegistry._checkRegistryCoo`D\x82\x01R\x7Frdinator: caller is not the regi`d\x82\x01Ro9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90\x81Q\x81\x10\x15a\x07dW\x01` \x01\x90V[\x15a\x14\x82WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBLSApkRegistry._processQuorumApk`D\x82\x01R\x7FUpdate: quorum does not exist\0\0\0`d\x82\x01R`\x84\x90\xFD[\x91\x90a\x14\xF7a\x0C\x8DV[PCc\xFF\xFF\xFF\xFF\x16\x90_[\x84Q\x81\x10\x15a\x16\x82W\x80\x83a\x15-a\x12=a\x15\x1F`\x01\x95\x8Aa\x14jV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[a\x15B\x81`\xFF\x16_R`\x04` R`@_ \x90V[T\x90a\x15O\x82\x15\x15a\x14{V[a\x15\xD5a\x15\xB6a\x15\xA8a\x15~\x89a\x15ya\x15t\x87`\xFF\x16_R`\x05` R`@_ \x90V[a\x0C\xA5V[a\x16\xFBV[a\x08\xC9\x81a\x15\x97\x87`\xFF\x16_R`\x05` R`@_ \x90V[\x90` `\x01\x91\x80Q\x84U\x01Q\x91\x01UV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x92a\x12\xA1a\x15\xCF\x84`\xFF\x16_R`\x04` R`@_ \x90V[\x91a\x11\xDFV[P\x90\x83a\x15\xEDa\x12\xDF\x84Tc\xFF\xFF\xFF\xFF\x90`\xC0\x1C\x16\x90V[\x03a\x16\x16WPa\x16\x10\x92P\x90`@\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xC0\x1B\x82T\x16\x17\x90UV[\x01a\x15\x02V[\x81T`\x01`\x01`\xE0\x1B\x03\x16`\xE0\x94\x90\x94\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x93\x90\x93\x17\x90Ua\x16}\x91a\x16P\x90`\xFF\x16_R`\x04` R`@_ \x90V[a\x16la\x16[a\x03{V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x83RV[c\xFF\xFF\xFF\xFF\x87\x16` \x83\x01Ra\x02_V[a\x16\x10V[PPP\x90PV[`@Q\x90a\x01\x80a\x16\x9A\x81\x84a\x03YV[6\x837V[`@Q\x90a\x16\xAE` \x83a\x03YV[` 6\x837V[\x91\x90`@\x90``a\x16\xC4a\x0C\x8DV[\x94\x85\x92` \x85Q\x92a\x16\xD6\x85\x85a\x03YV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15a\x16\xF9WV[\xFE[` \x92\x91`\x80`@\x92a\x17\x0Ca\x0C\x8DV[\x95\x86\x93\x81\x86Q\x93a\x17\x1D\x86\x86a\x03YV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15a\x16\xF9W\x15a\x17NWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x90\xFD[`@Qa\x17\x8F\x81a\x03\x1CV[`@\x90\x81Qa\x17\x9E\x83\x82a\x03YV[\x826\x827\x81R` \x82Q\x91a\x17\xB3\x84\x84a\x03YV[\x836\x847\x01R\x80Qa\x17\xC5\x82\x82a\x03YV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90a\x18\x1B\x83\x83a\x03YV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01Ra\x18p\x83Q\x93\x84a\x03YV[\x82R` \x82\x01R\x90V[a\x18\x82a\x0C\x8DV[P`@Qa\x18\x8F\x81a\x03\x1CV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a\x11\xDAWV[\x90`\x02\x81\x10\x15a\x07dW`\x05\x1B\x01\x90V[\x90`\x01\x82\x01\x80\x92\x11a\x11\xDAWV[\x90`\x02\x82\x01\x80\x92\x11a\x11\xDAWV[\x90`\x03\x82\x01\x80\x92\x11a\x11\xDAWV[\x90`\x04\x82\x01\x80\x92\x11a\x11\xDAWV[\x90`\x05\x82\x01\x80\x92\x11a\x11\xDAWV[\x90`\x0C\x81\x10\x15a\x07dW`\x05\x1B\x01\x90V[\x15a\x19\"WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x90\xFD[\x92\x90\x91a\x19l`@a\x03\x8CV[\x93\x84R` \x84\x01Ra\x19~`@a\x03\x8CV[\x91\x82R` \x82\x01Ra\x19\x8Ea\x16\x89V[\x91_[`\x02\x81\x10a\x19\xCBWPPP` a\x01\x80\x91a\x19\xAAa\x16\x9FV[\x92\x83\x91`\x08a\x07\xCF\x19Z\x01\xFA\x80\x15a\x16\xF9Wa\x19\xC5\x90a\x19\x1BV[Q\x15\x15\x90V[\x80a\x19\xD7`\x01\x92a\x18\x9DV[a\x19\xE1\x82\x85a\x18\xB3V[QQa\x19\xED\x82\x88a\x19\nV[R` a\x19\xFA\x83\x86a\x18\xB3V[Q\x01Qa\x1A\x0Fa\x1A\t\x83a\x18\xC4V[\x88a\x19\nV[Ra\x1A\x1A\x82\x86a\x18\xB3V[QQQa\x1A)a\x1A\t\x83a\x18\xD2V[Ra\x1A?a\x1A7\x83\x87a\x18\xB3V[QQ` \x01\x90V[Qa\x1ALa\x1A\t\x83a\x18\xE0V[R` a\x1AY\x83\x87a\x18\xB3V[Q\x01QQa\x1Aia\x1A\t\x83a\x18\xEEV[Ra\x1A\x95a\x1A\x8Fa\x1A\x88` a\x1A\x7F\x86\x8Aa\x18\xB3V[Q\x01Q` \x01\x90V[Q\x92a\x18\xFCV[\x87a\x19\nV[R\x01a\x19\x91V[a\x1A\xA4a\x0C\x8DV[P\x80Q\x90\x81\x15\x80a\x1BPW[\x15a\x1A\xD1WPP`@Qa\x1A\xC5`@\x82a\x03YV[_\x81R_` \x82\x01R\x90V[` \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x91\x01Q\x06\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x03\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x81\x11a\x11\xDAW`@Q\x91a\x18p`@\x84a\x03YV[P` \x81\x01Q\x15a\x1A\xB0V\xFEBLSApkRegistry.registerBLSPublic\xA2dipfsX\"\x12 &$\xD7\xCD\xB2^m9\xE9\x80b\xD9\xD5\x07\xACf\xC32\x15*\xF6\xFD(\x18\x0E\x86\xF1\xCC?dd\x85dsolcC\0\x08\x1B\x003",
=======
        b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0!\x0B8\x03\x80b\0!\x0B\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\x16V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R\x80b\0\0Lb\0\0TV[PPb\0\x01HV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\x14W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15b\0\x01)W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01AW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x1F\x8Bb\0\x01\x80`\09`\0\x81\x81a\x03\x0F\x01R\x81\x81a\x04f\x01R\x81\x81a\x05\xBF\x01R\x81\x81a\t\xC5\x01Ra\x101\x01Ra\x1F\x8B`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x15W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\xA2W\x80c\xBFy\xCEX\x11a\0qW\x80c\xBFy\xCEX\x14a\x03\xCCW\x80c\xD5%J\x8C\x14a\x03\xDFW\x80c\xDE)\xFA\xC0\x14a\x03\xFFW\x80c\xE8\xBB\x9A\xE6\x14a\x04\x1FW\x80c\xF4\xE2O\xE5\x14a\x04HW`\0\x80\xFD[\x80cm\x14\xA9\x87\x14a\x03\nW\x80cy\x16\xCE\xA6\x14a\x031W\x80c\x7F\xF8\x1A\x87\x14a\x03rW\x80c\xA3\xDB\x80\xE2\x14a\x03\xA5W`\0\x80\xFD[\x80c?\xB2yR\x11a\0\xE9W\x80c?\xB2yR\x14a\x01\xDFW\x80cG\xB3\x14\xE8\x14a\x01\xF2W\x80c_a\xA8\x84\x14a\x023W\x80c`WG\xD5\x14a\x02\x8FW\x80ch\xBC\xCA\xAC\x14a\x02\xDDW`\0\x80\xFD[\x80b\xA1\xF4\xCB\x14a\x01\x1AW\x80c\x13T*N\x14a\x01[W\x80c&\xD9A\xF2\x14a\x01\x92W\x80c7~\xD9\x9D\x14a\x01\xA7W[`\0\x80\xFD[a\x01Aa\x01(6`\x04a\x19\x04V[`\x03` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x84a\x01i6`\x04a\x19\x04V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01RV[a\x01\xA5a\x01\xA06`\x04a\x197V[a\x04[V[\0[a\x01\xCAa\x01\xB56`\x04a\x197V[`\xFF\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01RV[a\x01\xA5a\x01\xED6`\x04a\x19\xC2V[a\x05\xB4V[a\x02\x1Ba\x02\x006`\x04a\x1AhV[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01RV[a\x02\x82a\x02A6`\x04a\x197V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\xFF\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`@Qa\x01R\x91\x90a\x1A\x81V[a\x02\xA2a\x02\x9D6`\x04a\x1A\x98V[a\x06rV[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x01RV[a\x02\xF0a\x02\xEB6`\x04a\x1A\xC2V[a\x07\x05V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R` \x01a\x01RV[a\x02\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Da\x03?6`\x04a\x1A\x98V[a\x08\xA0V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01RV[a\x03\x85a\x03\x806`\x04a\x19\x04V[a\x08\xEBV[`@\x80Q\x83Q\x81R` \x93\x84\x01Q\x93\x81\x01\x93\x90\x93R\x82\x01R``\x01a\x01RV[a\x01Aa\x03\xB36`\x04a\x197V[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[a\x01\x84a\x03\xDA6`\x04a\x1B\nV[a\t\xB8V[a\x03\xF2a\x03\xED6`\x04a\x1BgV[a\x0E\x0CV[`@Qa\x01R\x91\x90a\x1B\xDFV[a\x01\x84a\x04\r6`\x04a\x19\x04V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x02\x1Ba\x04-6`\x04a\x1AhV[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xA5a\x04V6`\x04a\x19\xC2V[a\x10&V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xA3\x90a\x1C)V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16`\0\x90\x81R`\x04` R`@\x90 T\x15a\x05+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBLSApkRegistry.initializeQuorum:`D\x82\x01Ru quorum already exists`P\x1B`d\x82\x01R`\x84\x01a\x04\xA3V[`\xFF\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x84\x81Rc\xFF\xFF\xFF\xFFC\x81\x16\x82\x86\x01\x90\x81R\x82\x85\x01\x87\x81R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x94Q\x83\x16`\x01`\xE0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x95\x90\x93\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x96\x16\x91\x90\x93\x1C\x17\x93\x90\x93\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xA3\x90a\x1C)V[`\0a\x06\x07\x83a\x08\xEBV[P\x90Pa\x06\x14\x82\x82a\x10\xCFV[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06U\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[\x84`@Qa\x06e\x93\x92\x91\x90a\x1C\x9DV[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x04\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x06\xAFWa\x06\xAFa\x1D\tV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x07,Wa\x07,a\x1D\tV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x85\x90R`\x01`\xE0\x1B\x90\x91\x04\x81\x16\x92\x82\x01\x92\x90\x92R\x92P\x85\x16\x10\x15a\x07\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: index too recent\0\0`d\x82\x01R`\x84\x01a\x04\xA3V[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x08\x19WP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x08\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: not latest apk upd`d\x82\x01Rbate`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x04\xA3V[Q\x94\x93PPPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x08\xBCW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`@\x81\x90\x1B\x92Pc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x92P`\x01`\xE0\x1B\x90\x91\x04\x16\x83V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x91\x82\x01T\x81\x85\x01R\x94\x84R\x90\x91R\x81 T\x90\x91\x90\x80a\t\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry.getRegisteredPubk`D\x82\x01R\x7Fey: operator is not registered\0\0`d\x82\x01R`\x84\x01a\x04\xA3V[\x90\x94\x90\x93P\x91PPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xA3\x90a\x1C)V[`\0a\n0a\n\x196\x86\x90\x03\x86\x01`@\x87\x01a\x1D\x1FV[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x14\x15a\n\xB8W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` a\x1F6\x839\x81Q\x91R`D\x82\x01R\x7FKey: cannot register zero pubkey`d\x82\x01R`\x84\x01a\x04\xA3V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x0BBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a\x1F6\x839\x81Q\x91R`D\x82\x01R\x7FKey: operator already registered`d\x82\x01Rf pubkey`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x04\xA3V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a\x1F6\x839\x81Q\x91R`D\x82\x01R\x7FKey: public key already register`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x04\xA3V[`@\x80Q`\0\x91\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x91a\x0C\x1F\x91\x885\x91` \x80\x8B\x015\x92\x8B\x015\x91``\x8C\x015\x91`\x80\x8D\x01\x91`\xC0\x8E\x01\x91\x8D5\x91\x8E\x82\x015\x91\x01a\x1DQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x0CB\x91\x90a\x1D\x9CV[\x90Pa\x0C\xDCa\x0C{a\x0Cf\x83a\x0C`6\x8A\x90\x03\x8A\x01`@\x8B\x01a\x1D\x1FV[\x90a\x13\x1AV[a\x0Cu6\x89\x90\x03\x89\x01\x89a\x1D\x1FV[\x90a\x13\xB1V[a\x0C\x83a\x14EV[a\x0C\xC5a\x0C\xB6\x85a\x0C``@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[a\x0Cu6\x8A\x90\x03\x8A\x01\x8Aa\x1D\x1FV[a\x0C\xD76\x8A\x90\x03\x8A\x01`\x80\x8B\x01a\x1E\x0EV[a\x15\x05V[a\rwW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R`\0\x80Q` a\x1F6\x839\x81Q\x91R`D\x82\x01R\x7FKey: either the G1 signature is `d\x82\x01R\x7Fwrong, or G1 and G2 private key `\x84\x82\x01Rk\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x01a\x04\xA3V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x89\x82\x01\x805\x82U``\x8B\x015`\x01\x92\x83\x01U\x90\x83R\x81\x84 \x87\x90U\x86\x84R`\x02\x90\x92R\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90U\x90Q\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x91a\r\xFB\x91`\x80\x8A\x01\x90a\x1EkV[`@Q\x80\x91\x03\x90\xA2P\x94\x93PPPPV[```\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E)Wa\x0E)a\x19RV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0ERW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x10\x1DW`\0\x86\x86\x83\x81\x81\x10a\x0EtWa\x0Eta\x1D\tV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x04` R`@\x90 T\x90\x92P\x90P\x80\x15\x80a\x0E\xD7WP`\xFF\x82\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x90\x91\x90a\x0E\xBBWa\x0E\xBBa\x1D\tV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x0FdW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FBLSApkRegistry.getApkIndicesAtBl`D\x82\x01R\x7FockNumber: blockNumber is before`d\x82\x01Rp the first update`x\x1B`\x84\x82\x01R`\xA4\x01a\x04\xA3V[\x80[\x80\x15a\x10\x07W`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x90 \x87\x90a\x0F\x8B`\x01\x84a\x1E\xB5V[\x81T\x81\x10a\x0F\x9BWa\x0F\x9Ba\x1D\tV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11a\x0F\xF5Wa\x0F\xC4`\x01\x82a\x1E\xB5V[\x85\x85\x81Q\x81\x10a\x0F\xD6Wa\x0F\xD6a\x1D\tV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x10\x07V[\x80a\x0F\xFF\x81a\x1E\xCCV[\x91PPa\x0FfV[PPP\x80\x80a\x10\x15\x90a\x1E\xE3V[\x91PPa\x0EXV[P\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xA3\x90a\x1C)V[`\0a\x10y\x83a\x08\xEBV[P\x90Pa\x10\x8E\x82a\x10\x89\x83a\x17rV[a\x10\xCFV[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x06U\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[\x83Q\x81\x10\x15a\x13\x14W`\0\x84\x82\x81Q\x81\x10a\x11\x03Wa\x11\x03a\x1D\tV[\x01` \x90\x81\x01Q`\xF8\x1C`\0\x81\x81R`\x04\x90\x92R`@\x90\x91 T\x90\x91P\x80a\x11\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBLSApkRegistry._processQuorumApk`D\x82\x01R\x7FUpdate: quorum does not exist\0\0\0`d\x82\x01R`\x84\x01a\x04\xA3V[`\xFF\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01Ra\x11\xC7\x90\x86a\x13\xB1V[`\xFF\x83\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x85Q\x80\x82U\x86\x84\x01\x80Q`\x01\x93\x84\x01U\x90\x85RQ\x83R\x81\x84 \x94\x84R`\x04\x90\x92R\x82 \x93\x97P\x91\x92\x90\x91a\x12\x10\x90\x85a\x1E\xB5V[\x81T\x81\x10a\x12 Wa\x12 a\x1D\tV[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16`\x01`\xC0\x1B\x90\x92\x04\x16\x14\x15a\x12aW\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`@\x83\x90\x1C\x17\x81Ua\x12\xFDV[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01`\xE0\x1B\x81\x81\x02`\x01`\x01`\xE0\x1B\x03\x94\x85\x16\x17\x85U`\xFF\x88\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x8B\x16\x81R\x80\x84\x01\x96\x87R\x80\x83\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x93Q\x93\x01\x80T\x95Q\x92Q\x87\x16\x90\x94\x02\x91\x90\x95\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x94\x90\x94\x16\x91\x90\x94\x1C\x17\x91\x90\x91\x17\x90\x92\x16\x17\x90U[PPPP\x80\x80a\x13\x0C\x90a\x1E\xE3V[\x91PPa\x10\xE6V[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x136a\x181V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13iWa\x13kV[\xFE[P\x80a\x13\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04\xA3V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13\xCDa\x18OV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13iWP\x80a\x13\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04\xA3V[a\x14Ma\x18mV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R`\0\x91a\x154a\x18\x92V[`\0[`\x02\x81\x10\x15a\x16\xF9W`\0a\x15M\x82`\x06a\x1E\xFEV[\x90P\x84\x82`\x02\x81\x10a\x15aWa\x15aa\x1D\tV[` \x02\x01QQ\x83a\x15s\x83`\0a\x1F\x1DV[`\x0C\x81\x10a\x15\x83Wa\x15\x83a\x1D\tV[` \x02\x01R\x84\x82`\x02\x81\x10a\x15\x9AWa\x15\x9Aa\x1D\tV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x15\xB1\x91\x90a\x1F\x1DV[`\x0C\x81\x10a\x15\xC1Wa\x15\xC1a\x1D\tV[` \x02\x01R\x83\x82`\x02\x81\x10a\x15\xD8Wa\x15\xD8a\x1D\tV[` \x02\x01QQQ\x83a\x15\xEB\x83`\x02a\x1F\x1DV[`\x0C\x81\x10a\x15\xFBWa\x15\xFBa\x1D\tV[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\x12Wa\x16\x12a\x1D\tV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x16+\x83`\x03a\x1F\x1DV[`\x0C\x81\x10a\x16;Wa\x16;a\x1D\tV[` \x02\x01R\x83\x82`\x02\x81\x10a\x16RWa\x16Ra\x1D\tV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a\x16mWa\x16ma\x1D\tV[` \x02\x01Q\x83a\x16~\x83`\x04a\x1F\x1DV[`\x0C\x81\x10a\x16\x8EWa\x16\x8Ea\x1D\tV[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\xA5Wa\x16\xA5a\x1D\tV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x16\xC0Wa\x16\xC0a\x1D\tV[` \x02\x01Q\x83a\x16\xD1\x83`\x05a\x1F\x1DV[`\x0C\x81\x10a\x16\xE1Wa\x16\xE1a\x1D\tV[` \x02\x01RP\x80a\x16\xF1\x81a\x1E\xE3V[\x91PPa\x157V[Pa\x17\x02a\x18\xB1V[`\0` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13iWP\x80a\x17bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x04\xA3V[PQ\x15\x15\x98\x97PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x17\x97WP` \x82\x01Q\x15[\x15a\x17\xB5WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x17\xFA\x91\x90a\x1D\x9CV[a\x18$\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1E\xB5V[\x90R\x92\x91PPV[\x91\x90PV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x18\x80a\x18\xCFV[\x81R` \x01a\x18\x8Da\x18\xCFV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18,W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19\x16W`\0\x80\xFD[a\x19\x1F\x82a\x18\xEDV[\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x18,W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19IW`\0\x80\xFD[a\x19\x1F\x82a\x19&V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\x8BWa\x19\x8Ba\x19RV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\xBAWa\x19\xBAa\x19RV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\xD5W`\0\x80\xFD[a\x19\xDE\x83a\x18\xEDV[\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x19\xFCW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1A\x10W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1A\"Wa\x1A\"a\x19RV[a\x1A4`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x19\x91V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x1AJW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1AzW`\0\x80\xFD[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x06\xFFV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xABW`\0\x80\xFD[a\x1A\xB4\x83a\x19&V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1A\xD7W`\0\x80\xFD[a\x1A\xE0\x84a\x19&V[\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\xF9W`\0\x80\xFD[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x83\x85\x03a\x01`\x81\x12\x15a\x1B!W`\0\x80\xFD[a\x1B*\x85a\x18\xEDV[\x93Pa\x01\0`\x1F\x19\x82\x01\x12\x15a\x1B?W`\0\x80\xFD[` \x85\x01\x92P`@a\x01\x1F\x19\x82\x01\x12\x15a\x1BXW`\0\x80\xFD[Pa\x01 \x84\x01\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1B|W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\x94W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1B\xA8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1B\xB7W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x1B\xC9W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1C\x1DW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1B\xFBV[P\x90\x96\x95PPPPPPV[` \x80\x82R`N\x90\x82\x01R\x7FBLSApkRegistry.onlyRegistryCoord`@\x82\x01R\x7Finator: caller is not the regist``\x82\x01Rm9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x91\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R`\0` \x84\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x1C\xDFW\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x1C\xC3V[\x81\x81\x11\x15a\x1C\xF1W`\0`\x80\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`\x80\x01\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x1D1W`\0\x80\xFD[a\x1D9a\x19hV[\x825\x81R` \x83\x015` \x82\x01R\x80\x91PP\x92\x91PPV[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01R`@\x85`\x80\x83\x017`\0`\xC0\x82\x01`\0\x81R`@\x86\x827PPa\x01\0\x81\x01\x92\x90\x92Ra\x01 \x82\x01Ra\x01@\x01\x96\x95PPPPPPV[`\0\x82a\x1D\xB9WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x82`\x1F\x83\x01\x12a\x1D\xCFW`\0\x80\xFD[a\x1D\xD7a\x19hV[\x80`@\x84\x01\x85\x81\x11\x15a\x1D\xE9W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x1E\x03W\x805\x84R` \x93\x84\x01\x93\x01a\x1D\xEBV[P\x90\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x1E W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1ECWa\x1ECa\x19RV[`@Ra\x1EP\x84\x84a\x1D\xBEV[\x81Ra\x1E_\x84`@\x85\x01a\x1D\xBEV[` \x82\x01R\x93\x92PPPV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`\xC0\x81\x01`@\x83\x81\x84\x017`\x80\x82\x01`\0\x81R`@\x80\x85\x01\x827P`\0\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1E\xC7Wa\x1E\xC7a\x1E\x9FV[P\x03\x90V[`\0\x81a\x1E\xDBWa\x1E\xDBa\x1E\x9FV[P`\0\x19\x01\x90V[`\0`\0\x19\x82\x14\x15a\x1E\xF7Wa\x1E\xF7a\x1E\x9FV[P`\x01\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1F\x18Wa\x1F\x18a\x1E\x9FV[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x1F0Wa\x1F0a\x1E\x9FV[P\x01\x90V\xFEBLSApkRegistry.registerBLSPublic\xA2dipfsX\"\x12 P\xDD*\xB3\x96!Ie\x93\x9C\x9B\xAE\tJb5\x0C\x11/\xFEX\x17\xE6\x9D1'M\0\x1E}\xCF!dsolcC\0\x08\x0C\x003",
>>>>>>> dev:crates/utils/src/middleware/blsapkregistry.rs
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
<<<<<<< HEAD:crates/utils/src/blsapkregistry.rs
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8062a1f4cb1461012957806313542a4e146100e857806326d941f214610124578063377ed99d1461011f5780633fb279521461011a57806347b314e8146100e35780635f61a88414610115578063605747d51461011057806368bccaac1461010b5780636d14a987146101065780637916cea6146101015780637ff81a87146100fc578063a3db80e2146100f7578063bf79ce58146100f2578063d5254a8c146100ed578063de29fac0146100e8578063e8bb9ae6146100e35763f4e24fe5146100de575f80fd5b610b85565b6104a5565b6101b1565b610b18565b61087d565b610832565b6107f1565b61077e565b6106fa565b6105c0565b610533565b6104d1565b61041e565b6102d0565b6101f9565b61015c565b600435906001600160a01b038216820361014457565b5f80fd5b35906001600160a01b038216820361014457565b34610144576020366003190112610144576001600160a01b0361017d61012e565b165f52600360205260405f2060018154910154906101ad6040519283928360209093929193604081019481520152565b0390f35b34610144576020366003190112610144576001600160a01b036101d261012e565b165f526001602052602060405f2054604051908152f35b6004359060ff8216820361014457565b34610144576020366003190112610144576102126101e9565b61021a6113b4565b60ff81165f52600460205260405f205461026c5761024661026a9160ff165f52600460205260405f2090565b61024e61037b565b5f81524363ffffffff166020820152905b5f6040830152610bd8565b005b60405162461bcd60e51b815260206004820152603660248201527f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a6044820152752071756f72756d20616c72656164792065786973747360501b6064820152608490fd5b346101445760203660031901126101445760ff6102eb6101e9565b165f526004602052602063ffffffff60405f205416604051908152f35b634e487b7160e01b5f52604160045260245ffd5b6040810190811067ffffffffffffffff82111761033857604052565b610308565b6060810190811067ffffffffffffffff82111761033857604052565b90601f8019910116810190811067ffffffffffffffff82111761033857604052565b6040519061038a606083610359565b565b9061038a6040519283610359565b906040600319830112610144576103b16004610148565b9160243567ffffffffffffffff811161014457816023820112156101445780600401359067ffffffffffffffff821161033857604051926103fc601f8401601f191660200185610359565b8284526024838301011161014457815f92602460209301838601378301015290565b34610144577f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e61044d3661039a565b906104566113b4565b61046961046282610d7c565b50836114ed565b60018060a01b0381165f52600160205261048e60405f20549260405193849384610c4d565b0390a1005b60209060031901126101445760043590565b34610144576104b336610493565b5f526002602052602060018060a01b0360405f205416604051908152f35b346101445760203660031901126101445760ff6104ec6101e9565b6104f4610c8d565b50165f5260056020526040805f2060018251916105108361031c565b80548352015460208201526105318251809260208091805184520151910152565bf35b34610144576040366003190112610144576105886105826105526101e9565b60ff602435915f604080516105668161033d565b8281528260208201520152165f52600460205260405f20610769565b50610cc3565b604051809163ffffffff6040606084019267ffffffffffffffff19815116855282602082015116602086015201511660408301520390f35b34610144576060366003190112610144576105d96101e9565b6024359063ffffffff82168092036101445761058261060f9160ff6105fd60443590565b91165f52600460205260405f20610769565b9063ffffffff602083015116811061069057816106556106649261063d60406101ad96015163ffffffff1690565b9063ffffffff821615918215610680575b5050610cfc565b5167ffffffffffffffff191690565b60405167ffffffffffffffff1990911681529081906020820190565b63ffffffff161190505f8061064e565b608460405162461bcd60e51b815260206004820152604060248201527f424c5341706b52656769737472792e67657441706b486173684174426c6f636b60448201527f4e756d626572416e64496e6465783a20696e64657820746f6f20726563656e746064820152fd5b34610144575f366003190112610144576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b634e487b7160e01b5f52603260045260245ffd5b805415610764575f5260205f20905f90565b61073e565b8054821015610764575f5260205f2001905f90565b34610144576040366003190112610144576107976101e9565b60ff60243591165f52600460205260405f20908154811015610144576107bc91610769565b50546040805182821b67ffffffffffffffff1916815260c083901c63ffffffff16602082015260e09290921c90820152606090f35b3461014457602036600319011261014457606061081461080f61012e565b610d7c565b61082b604051809360208091805184520151910152565b6040820152f35b346101445760203660031901126101445760ff61084d6101e9565b165f52600560205260405f2060018154910154906101ad6040519283928360209093929193604081019481520152565b34610144576101603660031901126101445761089761012e565b61010036602319011261014457604036610123190112610144576101ad906108bd6113b4565b6108db6108c936610e37565b80515f526020015160205260405f2090565b906109087fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5831415610eb2565b6001600160a01b0381165f90815260016020526040902061092a905415610f10565b5f8281526002602052604090205461094b906001600160a01b031615610f7f565b604051610a1290610a0d906109b790602081019061098e8161098061014435610124356084356064356044356024358a610fe9565b03601f198101835282610359565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b6109db6109c336610e5f565b6109d5836109d036610e37565b6116b5565b906116fb565b906109fd6109e7611783565b916109d56109f436610e87565b916109d061187a565b90610a073661106d565b9261195f565b6110a6565b6001600160a01b0381165f908152600360205260409020610a3c9060643581556001608435910155565b6001600160a01b0381165f908152600160205260409020829055610a8b81610a6c845f52600260205260405f2090565b80546001600160a01b0319166001600160a01b03909216919091179055565b6040516001600160a01b03909116907fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba38280419080610ac681611140565b0390a26040519081529081906020820190565b60206040818301928281528451809452019201905f5b818110610afc5750505090565b825163ffffffff16845260209384019390920191600101610aef565b346101445760403660031901126101445760043567ffffffffffffffff8111610144573660238201121561014457806004013567ffffffffffffffff8111610144573660248284010111610144576101ad91610b7991602480359201611201565b60405191829182610ad9565b34610144577ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e610bb43661039a565b90610bbd6113b4565b610469610bd2610bcc83610d7c565b50611a9c565b836114ed565b80546801000000000000000081101561033857610bfa91600182018155610769565b610c3a578151602083015160409384015163ffffffff60c01b60c09290921b919091169190931c1760e09290921b6001600160e01b031916919091179055565b634e487b7160e01b5f525f60045260245ffd5b919260809360209260018060a01b0316845282840152606060408401528051918291826060860152018484015e5f828201840152601f01601f1916010190565b60405190610c9a8261031c565b5f6020838281520152565b90604051610cb28161031c565b602060018294805484520154910152565b90604051610cd08161033d565b604081935467ffffffffffffffff1981831b16835263ffffffff8160c01c16602084015260e01c910152565b15610d0357565b60405162461bcd60e51b815260206004820152604560248201527f424c5341706b52656769737472792e67657441706b486173684174426c6f636b60448201527f4e756d626572416e64496e6465783a206e6f74206c61746573742061706b2075606482015264706461746560d81b608482015260a490fd5b610d84610c8d565b5060018060a01b031690815f52600360205260405f2091600160405193610daa8561031c565b80548552015460208401525f52600160205260405f2054918215610dcc579190565b60405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e676574526567697374657265645075626b60448201527f65793a206f70657261746f72206973206e6f74207265676973746572656400006064820152608490fd5b60409060631901126101445760405190610e508261031c565b60643582526084356020830152565b60409060231901126101445760405190610e788261031c565b60243582526044356020830152565b6040906101231901126101445760405190610ea18261031c565b610124358252610144356020830152565b15610eb957565b608460405162461bcd60e51b815260206004820152604060248201525f516020611b5d5f395f51905f5260448201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b65796064820152fd5b15610f1757565b60405162461bcd60e51b815260206004820152604760248201525f516020611b5d5f395f51905f5260448201527f4b65793a206f70657261746f7220616c72656164792072656769737465726564606482015266207075626b657960c81b608482015260a490fd5b15610f8657565b60405162461bcd60e51b815260206004820152604260248201525f516020611b5d5f395f51905f5260448201527f4b65793a207075626c6963206b657920616c7265616479207265676973746572606482015261195960f21b608482015260a490fd5b949290916101409694928652602086015260408501526060840152604060a46080850137604060e460c08501376101008301526101208201520190565b9080601f830112156101445760405191611041604084610359565b82906040810192831161014457905b82821061105d5750505090565b8135815260209182019101611050565b90608060a319830112610144576040516110868161031c565b60206110a182946110988160a4611026565b845260e4611026565b910152565b156110ad57565b60405162461bcd60e51b815260206004820152606c60248201525f516020611b5d5f395f51905f5260448201527f4b65793a2065697468657220746865204731207369676e61747572652069732060648201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060848201526b0c8de40dcdee840dac2e8c6d60a31b60a482015260c490fd5b90604060e4608060c0850194606435815260843560208201528360a4818301370137565b67ffffffffffffffff81116103385760051b60200190565b9061118682611164565b6111936040519182610359565b82815280926111a4601f1991611164565b0190602036910137565b90821015610764570190565b634e487b7160e01b5f52601160045260245ffd5b80156111da575f190190565b6111ba565b5f198101919082116111da57565b80518210156107645760209160051b010190565b91909161120d8361117c565b925f5b81811061121e575050505090565b61124361123d61122f8385876111ae565b356001600160f81b03191690565b60f81c90565b6112588160ff165f52600460205260405f2090565b5480158015611389575b61130457805b611277575b5050600101611210565b8563ffffffff6112b56112a76112988660ff165f52600460205260405f2090565b6112a1866111df565b90610769565b505460c01c63ffffffff1690565b1611156112cb576112c5906111ce565b80611268565b60019291506112e86112df6112fd926111df565b63ffffffff1690565b6112f283896111ed565b9063ffffffff169052565b905f61126d565b60405162461bcd60e51b815260206004820152605160248201527f424c5341706b52656769737472792e67657441706b496e64696365734174426c60448201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f7265606482015270207468652066697273742075706461746560781b608482015260a490fd5b506113ad6112df6112a76113a88560ff165f52600460205260405f2090565b610752565b8610611262565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036113e657565b60405162461bcd60e51b815260206004820152605060248201527f424c5341706b52656769737472792e5f636865636b5265676973747279436f6f60448201527f7264696e61746f723a2063616c6c6572206973206e6f7420746865207265676960648201526f39ba393c9031b7b7b93234b730ba37b960811b608482015260a490fd5b908151811015610764570160200190565b1561148257565b60405162461bcd60e51b815260206004820152603d60248201527f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b60448201527f5570646174653a2071756f72756d20646f6573206e6f742065786973740000006064820152608490fd5b91906114f7610c8d565b504363ffffffff16905f5b845181101561168257808361152d61123d61151f6001958a61146a565b516001600160f81b03191690565b6115428160ff165f52600460205260405f2090565b549061154f82151561147b565b6115d56115b66115a861157e896115796115748760ff165f52600560205260405f2090565b610ca5565b6116fb565b6108c9816115978760ff165f52600560205260405f2090565b906020600191805184550151910155565b67ffffffffffffffff191690565b926112a16115cf8460ff165f52600460205260405f2090565b916111df565b5090836115ed6112df845463ffffffff9060c01c1690565b03611616575061161092509060401c67ffffffffffffffff60c01b825416179055565b01611502565b81546001600160e01b031660e09490941b6001600160e01b03191693909317905561167d916116509060ff165f52600460205260405f2090565b61166c61165b61037b565b67ffffffffffffffff199093168352565b63ffffffff8716602083015261025f565b611610565b5050509050565b6040519061018061169a8184610359565b368337565b604051906116ae602083610359565b6020368337565b919060409060606116c4610c8d565b94859260208551926116d68585610359565b8436853780518452015160208301528482015260076107cf195a01fa156116f957565bfe5b60209291608060409261170c610c8d565b9586938186519361171d8686610359565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa80156116f9571561174e57565b60405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606490fd5b60405161178f8161031c565b604090815161179e8382610359565b82368237815260208251916117b38484610359565b83368437015280516117c58282610359565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061181b8383610359565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d602083015261187083519384610359565b8252602082015290565b611882610c8d565b5060405161188f8161031c565b600181526002602082015290565b906006820291808304600614901517156111da57565b9060028110156107645760051b0190565b90600182018092116111da57565b90600282018092116111da57565b90600382018092116111da57565b90600482018092116111da57565b90600582018092116111da57565b90600c8110156107645760051b0190565b1561192257565b60405162461bcd60e51b81526020600482015260156024820152741c185a5c9a5b99cb5bdc18dbd9194b59985a5b1959605a1b6044820152606490fd5b92909161196c604061038c565b938452602084015261197e604061038c565b918252602082015261198e611689565b915f5b600281106119cb575050506020610180916119aa61169f565b92839160086107cf195a01fa80156116f9576119c59061191b565b51151590565b806119d760019261189d565b6119e182856118b3565b51516119ed828861190a565b5260206119fa83866118b3565b510151611a0f611a09836118c4565b8861190a565b52611a1a82866118b3565b515151611a29611a09836118d2565b52611a3f611a3783876118b3565b515160200190565b51611a4c611a09836118e0565b526020611a5983876118b3565b51015151611a69611a09836118ee565b52611a95611a8f611a886020611a7f868a6118b3565b51015160200190565b51926118fc565b8761190a565b5201611991565b611aa4610c8d565b50805190811580611b50575b15611ad1575050604051611ac5604082610359565b5f81525f602082015290565b60207f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47910151067f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47037f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4781116111da5760405191611870604084610359565b50602081015115611ab056fe424c5341706b52656769737472792e7265676973746572424c535075626c6963a26469706673582212202624d7cdb25e6d39e98062d9d507ac66c332152af6fd28180e86f1cc3f64648564736f6c634300081b0033
=======
    ///0x608060405234801561001057600080fd5b50600436106101155760003560e01c80636d14a987116100a2578063bf79ce5811610071578063bf79ce58146103cc578063d5254a8c146103df578063de29fac0146103ff578063e8bb9ae61461041f578063f4e24fe51461044857600080fd5b80636d14a9871461030a5780637916cea6146103315780637ff81a8714610372578063a3db80e2146103a557600080fd5b80633fb27952116100e95780633fb27952146101df57806347b314e8146101f25780635f61a88414610233578063605747d51461028f57806368bccaac146102dd57600080fd5b8062a1f4cb1461011a57806313542a4e1461015b57806326d941f214610192578063377ed99d146101a7575b600080fd5b610141610128366004611904565b6003602052600090815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b610184610169366004611904565b6001600160a01b031660009081526001602052604090205490565b604051908152602001610152565b6101a56101a0366004611937565b61045b565b005b6101ca6101b5366004611937565b60ff1660009081526004602052604090205490565b60405163ffffffff9091168152602001610152565b6101a56101ed3660046119c2565b6105b4565b61021b610200366004611a68565b6000908152600260205260409020546001600160a01b031690565b6040516001600160a01b039091168152602001610152565b610282610241366004611937565b60408051808201909152600080825260208201525060ff16600090815260056020908152604091829020825180840190935280548352600101549082015290565b6040516101529190611a81565b6102a261029d366004611a98565b610672565b60408051825167ffffffffffffffff1916815260208084015163ffffffff908116918301919091529282015190921690820152606001610152565b6102f06102eb366004611ac2565b610705565b60405167ffffffffffffffff199091168152602001610152565b61021b7f000000000000000000000000000000000000000000000000000000000000000081565b61034461033f366004611a98565b6108a0565b6040805167ffffffffffffffff19909416845263ffffffff9283166020850152911690820152606001610152565b610385610380366004611904565b6108eb565b604080518351815260209384015193810193909352820152606001610152565b6101416103b3366004611937565b6005602052600090815260409020805460019091015482565b6101846103da366004611b0a565b6109b8565b6103f26103ed366004611b67565b610e0c565b6040516101529190611bdf565b61018461040d366004611904565b60016020526000908152604090205481565b61021b61042d366004611a68565b6002602052600090815260409020546001600160a01b031681565b6101a56104563660046119c2565b611026565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146104ac5760405162461bcd60e51b81526004016104a390611c29565b60405180910390fd5b60ff81166000908152600460205260409020541561052b5760405162461bcd60e51b815260206004820152603660248201527f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a6044820152752071756f72756d20616c72656164792065786973747360501b60648201526084016104a3565b60ff166000908152600460209081526040808320815160608101835284815263ffffffff4381168286019081528285018781528454600181018655948852959096209151919092018054955194518316600160e01b026001600160e01b0395909316600160c01b026001600160e01b03199096169190931c179390931791909116919091179055565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146105fc5760405162461bcd60e51b81526004016104a390611c29565b6000610607836108eb565b50905061061482826110cf565b7f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e83610655856001600160a01b031660009081526001602052604090205490565b8460405161066593929190611c9d565b60405180910390a1505050565b604080516060810182526000808252602080830182905282840182905260ff8616825260049052919091208054839081106106af576106af611d09565b600091825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b8204811694830194909452600160e01b90049092169082015290505b92915050565b60ff8316600090815260046020526040812080548291908490811061072c5761072c611d09565b600091825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b82048116948301859052600160e01b9091048116928201929092529250851610156107f35760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a20696e64657820746f6f20726563656e74000060648201526084016104a3565b604081015163ffffffff1615806108195750806040015163ffffffff168463ffffffff16105b6108975760405162461bcd60e51b815260206004820152604360248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a206e6f74206c61746573742061706b2075706460648201526261746560e81b608482015260a4016104a3565b51949350505050565b600460205281600052604060002081815481106108bc57600080fd5b600091825260209091200154604081901b925063ffffffff600160c01b820481169250600160e01b9091041683565b60408051808201909152600080825260208201526001600160a01b0382166000818152600360209081526040808320815180830183528154815260019182015481850152948452909152812054909190806109ae5760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e676574526567697374657265645075626b60448201527f65793a206f70657261746f72206973206e6f742072656769737465726564000060648201526084016104a3565b9094909350915050565b6000336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a025760405162461bcd60e51b81526004016104a390611c29565b6000610a30610a1936869003860160408701611d1f565b805160009081526020918201519091526040902090565b90507fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5811415610ab8576040805162461bcd60e51b8152602060048201526024810191909152600080516020611f3683398151915260448201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b657960648201526084016104a3565b6001600160a01b03851660009081526001602052604090205415610b425760405162461bcd60e51b81526020600482015260476024820152600080516020611f3683398151915260448201527f4b65793a206f70657261746f7220616c72656164792072656769737465726564606482015266207075626b657960c81b608482015260a4016104a3565b6000818152600260205260409020546001600160a01b031615610bc65760405162461bcd60e51b81526020600482015260426024820152600080516020611f3683398151915260448201527f4b65793a207075626c6963206b657920616c7265616479207265676973746572606482015261195960f21b608482015260a4016104a3565b604080516000917f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000191610c1f918835916020808b0135928b01359160608c01359160808d019160c08e01918d35918e8201359101611d51565b6040516020818303038152906040528051906020012060001c610c429190611d9c565b9050610cdc610c7b610c6683610c60368a90038a0160408b01611d1f565b9061131a565b610c7536899003890189611d1f565b906113b1565b610c83611445565b610cc5610cb685610c60604080518082018252600080825260209182015281518083019092526001825260029082015290565b610c75368a90038a018a611d1f565b610cd7368a90038a0160808b01611e0e565b611505565b610d775760405162461bcd60e51b815260206004820152606c6024820152600080516020611f3683398151915260448201527f4b65793a2065697468657220746865204731207369676e61747572652069732060648201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060848201526b0c8de40dcdee840dac2e8c6d60a31b60a482015260c4016104a3565b6001600160a01b03861660008181526003602090815260408083208982018035825560608b013560019283015590835281842087905586845260029092529182902080546001600160a01b0319168417905590517fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba382804191610dfb9160808a0190611e6b565b60405180910390a250949350505050565b606060008367ffffffffffffffff811115610e2957610e29611952565b604051908082528060200260200182016040528015610e52578160200160208202803683370190505b50905060005b8481101561101d576000868683818110610e7457610e74611d09565b919091013560f81c6000818152600460205260409020549092509050801580610ed7575060ff821660009081526004602052604081208054909190610ebb57610ebb611d09565b600091825260209091200154600160c01b900463ffffffff1686105b15610f645760405162461bcd60e51b815260206004820152605160248201527f424c5341706b52656769737472792e67657441706b496e64696365734174426c60448201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f7265606482015270207468652066697273742075706461746560781b608482015260a4016104a3565b805b80156110075760ff831660009081526004602052604090208790610f8b600184611eb5565b81548110610f9b57610f9b611d09565b600091825260209091200154600160c01b900463ffffffff1611610ff557610fc4600182611eb5565b858581518110610fd657610fd6611d09565b602002602001019063ffffffff16908163ffffffff1681525050611007565b80610fff81611ecc565b915050610f66565b505050808061101590611ee3565b915050610e58565b50949350505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461106e5760405162461bcd60e51b81526004016104a390611c29565b6000611079836108eb565b50905061108e8261108983611772565b6110cf565b7ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e83610655856001600160a01b031660009081526001602052604090205490565b604080518082019091526000808252602082015260005b835181101561131457600084828151811061110357611103611d09565b0160209081015160f81c60008181526004909252604090912054909150806111935760405162461bcd60e51b815260206004820152603d60248201527f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b60448201527f5570646174653a2071756f72756d20646f6573206e6f7420657869737400000060648201526084016104a3565b60ff821660009081526005602090815260409182902082518084019093528054835260010154908201526111c790866113b1565b60ff831660008181526005602090815260408083208551808255868401805160019384015590855251835281842094845260049092528220939750919290916112109085611eb5565b8154811061122057611220611d09565b600091825260209091200180549091504363ffffffff908116600160c01b9092041614156112615780546001600160c01b031916604083901c1781556112fd565b805463ffffffff438116600160e01b8181026001600160e01b0394851617855560ff88166000908152600460209081526040808320815160608101835267ffffffffffffffff198b16815280840196875280830185815282546001810184559286529390942093519301805495519251871690940291909516600160c01b026001600160e01b0319949094169190941c17919091179092161790555b50505050808061130c90611ee3565b9150506110e6565b50505050565b6040805180820190915260008082526020820152611336611831565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa90508080156113695761136b565bfe5b50806113a95760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b60448201526064016104a3565b505092915050565b60408051808201909152600080825260208201526113cd61184f565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa90508080156113695750806113a95760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b60448201526064016104a3565b61144d61186d565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b604080518082018252858152602080820185905282518084019093528583528201839052600091611534611892565b60005b60028110156116f957600061154d826006611efe565b905084826002811061156157611561611d09565b60200201515183611573836000611f1d565b600c811061158357611583611d09565b602002015284826002811061159a5761159a611d09565b602002015160200151838260016115b19190611f1d565b600c81106115c1576115c1611d09565b60200201528382600281106115d8576115d8611d09565b60200201515151836115eb836002611f1d565b600c81106115fb576115fb611d09565b602002015283826002811061161257611612611d09565b602002015151600160200201518361162b836003611f1d565b600c811061163b5761163b611d09565b602002015283826002811061165257611652611d09565b60200201516020015160006002811061166d5761166d611d09565b60200201518361167e836004611f1d565b600c811061168e5761168e611d09565b60200201528382600281106116a5576116a5611d09565b6020020151602001516001600281106116c0576116c0611d09565b6020020151836116d1836005611f1d565b600c81106116e1576116e1611d09565b602002015250806116f181611ee3565b915050611537565b506117026118b1565b60006020826101808560086107d05a03fa90508080156113695750806117625760405162461bcd60e51b81526020600482015260156024820152741c185a5c9a5b99cb5bdc18dbd9194b59985a5b1959605a1b60448201526064016104a3565b5051151598975050505050505050565b6040805180820190915260008082526020820152815115801561179757506020820151155b156117b5575050604080518082019091526000808252602082015290565b6040518060400160405280836000015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4784602001516117fa9190611d9c565b611824907f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611eb5565b905292915050565b919050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806118806118cf565b815260200161188d6118cf565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b80356001600160a01b038116811461182c57600080fd5b60006020828403121561191657600080fd5b61191f826118ed565b9392505050565b803560ff8116811461182c57600080fd5b60006020828403121561194957600080fd5b61191f82611926565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff8111828210171561198b5761198b611952565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156119ba576119ba611952565b604052919050565b600080604083850312156119d557600080fd5b6119de836118ed565b915060208084013567ffffffffffffffff808211156119fc57600080fd5b818601915086601f830112611a1057600080fd5b813581811115611a2257611a22611952565b611a34601f8201601f19168501611991565b91508082528784828501011115611a4a57600080fd5b80848401858401376000848284010152508093505050509250929050565b600060208284031215611a7a57600080fd5b5035919050565b8151815260208083015190820152604081016106ff565b60008060408385031215611aab57600080fd5b611ab483611926565b946020939093013593505050565b600080600060608486031215611ad757600080fd5b611ae084611926565b9250602084013563ffffffff81168114611af957600080fd5b929592945050506040919091013590565b6000806000838503610160811215611b2157600080fd5b611b2a856118ed565b9350610100601f1982011215611b3f57600080fd5b602085019250604061011f1982011215611b5857600080fd5b50610120840190509250925092565b600080600060408486031215611b7c57600080fd5b833567ffffffffffffffff80821115611b9457600080fd5b818601915086601f830112611ba857600080fd5b813581811115611bb757600080fd5b876020828501011115611bc957600080fd5b6020928301989097509590910135949350505050565b6020808252825182820181905260009190848201906040850190845b81811015611c1d57835163ffffffff1683529284019291840191600101611bfb565b50909695505050505050565b6020808252604e908201527f424c5341706b52656769737472792e6f6e6c795265676973747279436f6f726460408201527f696e61746f723a2063616c6c6572206973206e6f74207468652072656769737460608201526d393c9031b7b7b93234b730ba37b960911b608082015260a00190565b60018060a01b038416815260006020848184015260606040840152835180606085015260005b81811015611cdf57858101830151858201608001528201611cc3565b81811115611cf1576000608083870101525b50601f01601f19169290920160800195945050505050565b634e487b7160e01b600052603260045260246000fd5b600060408284031215611d3157600080fd5b611d39611968565b82358152602083013560208201528091505092915050565b8881528760208201528660408201528560608201526040856080830137600060c082016000815260408682375050610100810192909252610120820152610140019695505050505050565b600082611db957634e487b7160e01b600052601260045260246000fd5b500690565b600082601f830112611dcf57600080fd5b611dd7611968565b806040840185811115611de957600080fd5b845b81811015611e03578035845260209384019301611deb565b509095945050505050565b600060808284031215611e2057600080fd5b6040516040810181811067ffffffffffffffff82111715611e4357611e43611952565b604052611e508484611dbe565b8152611e5f8460408501611dbe565b60208201529392505050565b823581526020808401359082015260c081016040838184013760808201600081526040808501823750600081529392505050565b634e487b7160e01b600052601160045260246000fd5b600082821015611ec757611ec7611e9f565b500390565b600081611edb57611edb611e9f565b506000190190565b6000600019821415611ef757611ef7611e9f565b5060010190565b6000816000190483118215151615611f1857611f18611e9f565b500290565b60008219821115611f3057611f30611e9f565b50019056fe424c5341706b52656769737472792e7265676973746572424c535075626c6963a264697066735822122050dd2ab396214965939c9bae094a62350c112ffe5817e69d31274d001e7dcf2164736f6c634300080c0033
>>>>>>> dev:crates/utils/src/middleware/blsapkregistry.rs
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
<<<<<<< HEAD:crates/utils/src/blsapkregistry.rs
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\xA1\xF4\xCB\x14a\x01)W\x80c\x13T*N\x14a\0\xE8W\x80c&\xD9A\xF2\x14a\x01$W\x80c7~\xD9\x9D\x14a\x01\x1FW\x80c?\xB2yR\x14a\x01\x1AW\x80cG\xB3\x14\xE8\x14a\0\xE3W\x80c_a\xA8\x84\x14a\x01\x15W\x80c`WG\xD5\x14a\x01\x10W\x80ch\xBC\xCA\xAC\x14a\x01\x0BW\x80cm\x14\xA9\x87\x14a\x01\x06W\x80cy\x16\xCE\xA6\x14a\x01\x01W\x80c\x7F\xF8\x1A\x87\x14a\0\xFCW\x80c\xA3\xDB\x80\xE2\x14a\0\xF7W\x80c\xBFy\xCEX\x14a\0\xF2W\x80c\xD5%J\x8C\x14a\0\xEDW\x80c\xDE)\xFA\xC0\x14a\0\xE8W\x80c\xE8\xBB\x9A\xE6\x14a\0\xE3Wc\xF4\xE2O\xE5\x14a\0\xDEW_\x80\xFD[a\x0B\x85V[a\x04\xA5V[a\x01\xB1V[a\x0B\x18V[a\x08}V[a\x082V[a\x07\xF1V[a\x07~V[a\x06\xFAV[a\x05\xC0V[a\x053V[a\x04\xD1V[a\x04\x1EV[a\x02\xD0V[a\x01\xF9V[a\x01\\V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01DWV[_\x80\xFD[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01DWV[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW`\x01`\x01`\xA0\x1B\x03a\x01}a\x01.V[\x16_R`\x03` R`@_ `\x01\x81T\x91\x01T\x90a\x01\xAD`@Q\x92\x83\x92\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xF3[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW`\x01`\x01`\xA0\x1B\x03a\x01\xD2a\x01.V[\x16_R`\x01` R` `@_ T`@Q\x90\x81R\xF3[`\x045\x90`\xFF\x82\x16\x82\x03a\x01DWV[4a\x01DW` 6`\x03\x19\x01\x12a\x01DWa\x02\x12a\x01\xE9V[a\x02\x1Aa\x13\xB4V[`\xFF\x81\x16_R`\x04` R`@_ Ta\x02lWa\x02Fa\x02j\x91`\xFF\x16_R`\x04` R`@_ \x90V[a\x02Na\x03{V[_\x81RCc\xFF\xFF\xFF\xFF\x16` \x82\x01R\x90[_`@\x83\x01Ra\x0B\xD8V[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBLSApkRegistry.initializeQuorum:`D\x82\x01Ru quorum already exists`P\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW`\xFFa\x02\xEBa\x01\xE9V[\x16_R`\x04` R` c\xFF\xFF\xFF\xFF`@_ T\x16`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x038W`@RV[a\x03\x08V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x038W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x038W`@RV[`@Q\x90a\x03\x8A``\x83a\x03YV[V[\x90a\x03\x8A`@Q\x92\x83a\x03YV[\x90`@`\x03\x19\x83\x01\x12a\x01DWa\x03\xB1`\x04a\x01HV[\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01DW\x81`#\x82\x01\x12\x15a\x01DW\x80`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x038W`@Q\x92a\x03\xFC`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x03YV[\x82\x84R`$\x83\x83\x01\x01\x11a\x01DW\x81_\x92`$` \x93\x01\x83\x86\x017\x83\x01\x01R\x90V[4a\x01DW\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBNa\x04M6a\x03\x9AV[\x90a\x04Va\x13\xB4V[a\x04ia\x04b\x82a\r|V[P\x83a\x14\xEDV[`\x01\x80`\xA0\x1B\x03\x81\x16_R`\x01` Ra\x04\x8E`@_ T\x92`@Q\x93\x84\x93\x84a\x0CMV[\x03\x90\xA1\0[` \x90`\x03\x19\x01\x12a\x01DW`\x045\x90V[4a\x01DWa\x04\xB36a\x04\x93V[_R`\x02` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW`\xFFa\x04\xECa\x01\xE9V[a\x04\xF4a\x0C\x8DV[P\x16_R`\x05` R`@\x80_ `\x01\x82Q\x91a\x05\x10\x83a\x03\x1CV[\x80T\x83R\x01T` \x82\x01Ra\x051\x82Q\x80\x92` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\xF3[4a\x01DW`@6`\x03\x19\x01\x12a\x01DWa\x05\x88a\x05\x82a\x05Ra\x01\xE9V[`\xFF`$5\x91_`@\x80Qa\x05f\x81a\x03=V[\x82\x81R\x82` \x82\x01R\x01R\x16_R`\x04` R`@_ a\x07iV[Pa\x0C\xC3V[`@Q\x80\x91c\xFF\xFF\xFF\xFF`@``\x84\x01\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x01Q\x16`@\x83\x01R\x03\x90\xF3[4a\x01DW``6`\x03\x19\x01\x12a\x01DWa\x05\xD9a\x01\xE9V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x01DWa\x05\x82a\x06\x0F\x91`\xFFa\x05\xFD`D5\x90V[\x91\x16_R`\x04` R`@_ a\x07iV[\x90c\xFF\xFF\xFF\xFF` \x83\x01Q\x16\x81\x10a\x06\x90W\x81a\x06Ua\x06d\x92a\x06=`@a\x01\xAD\x96\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90c\xFF\xFF\xFF\xFF\x82\x16\x15\x91\x82\x15a\x06\x80W[PPa\x0C\xFCV[Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[c\xFF\xFF\xFF\xFF\x16\x11\x90P_\x80a\x06NV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FBLSApkRegistry.getApkHashAtBlock`D\x82\x01R\x7FNumberAndIndex: index too recent`d\x82\x01R\xFD[4a\x01DW_6`\x03\x19\x01\x12a\x01DW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x15a\x07dW_R` _ \x90_\x90V[a\x07>V[\x80T\x82\x10\x15a\x07dW_R` _ \x01\x90_\x90V[4a\x01DW`@6`\x03\x19\x01\x12a\x01DWa\x07\x97a\x01\xE9V[`\xFF`$5\x91\x16_R`\x04` R`@_ \x90\x81T\x81\x10\x15a\x01DWa\x07\xBC\x91a\x07iV[PT`@\x80Q\x82\x82\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R`\xC0\x83\x90\x1Cc\xFF\xFF\xFF\xFF\x16` \x82\x01R`\xE0\x92\x90\x92\x1C\x90\x82\x01R``\x90\xF3[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW``a\x08\x14a\x08\x0Fa\x01.V[a\r|V[a\x08+`@Q\x80\x93` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[`@\x82\x01R\xF3[4a\x01DW` 6`\x03\x19\x01\x12a\x01DW`\xFFa\x08Ma\x01\xE9V[\x16_R`\x05` R`@_ `\x01\x81T\x91\x01T\x90a\x01\xAD`@Q\x92\x83\x92\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[4a\x01DWa\x01`6`\x03\x19\x01\x12a\x01DWa\x08\x97a\x01.V[a\x01\x006`#\x19\x01\x12a\x01DW`@6a\x01#\x19\x01\x12a\x01DWa\x01\xAD\x90a\x08\xBDa\x13\xB4V[a\x08\xDBa\x08\xC96a\x0E7V[\x80Q_R` \x01Q` R`@_ \x90V[\x90a\t\x08\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x83\x14\x15a\x0E\xB2V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01` R`@\x90 a\t*\x90T\x15a\x0F\x10V[_\x82\x81R`\x02` R`@\x90 Ta\tK\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\x7FV[`@Qa\n\x12\x90a\n\r\x90a\t\xB7\x90` \x81\x01\x90a\t\x8E\x81a\t\x80a\x01D5a\x01$5`\x845`d5`D5`$5\x8Aa\x0F\xE9V[\x03`\x1F\x19\x81\x01\x83R\x82a\x03YV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[a\t\xDBa\t\xC36a\x0E_V[a\t\xD5\x83a\t\xD06a\x0E7V[a\x16\xB5V[\x90a\x16\xFBV[\x90a\t\xFDa\t\xE7a\x17\x83V[\x91a\t\xD5a\t\xF46a\x0E\x87V[\x91a\t\xD0a\x18zV[\x90a\n\x076a\x10mV[\x92a\x19_V[a\x10\xA6V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 a\n<\x90`d5\x81U`\x01`\x845\x91\x01UV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01` R`@\x90 \x82\x90Ua\n\x8B\x81a\nl\x84_R`\x02` R`@_ \x90V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x90\x80a\n\xC6\x81a\x11@V[\x03\x90\xA2`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\n\xFCWPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\xEFV[4a\x01DW`@6`\x03\x19\x01\x12a\x01DW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01DW6`#\x82\x01\x12\x15a\x01DW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01DW6`$\x82\x84\x01\x01\x11a\x01DWa\x01\xAD\x91a\x0By\x91`$\x805\x92\x01a\x12\x01V[`@Q\x91\x82\x91\x82a\n\xD9V[4a\x01DW\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0Ea\x0B\xB46a\x03\x9AV[\x90a\x0B\xBDa\x13\xB4V[a\x04ia\x0B\xD2a\x0B\xCC\x83a\r|V[Pa\x1A\x9CV[\x83a\x14\xEDV[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x038Wa\x0B\xFA\x91`\x01\x82\x01\x81Ua\x07iV[a\x0C:W\x81Q` \x83\x01Q`@\x93\x84\x01Qc\xFF\xFF\xFF\xFF`\xC0\x1B`\xC0\x92\x90\x92\x1B\x91\x90\x91\x16\x91\x90\x93\x1C\x17`\xE0\x92\x90\x92\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x91\x92`\x80\x93` \x92`\x01\x80`\xA0\x1B\x03\x16\x84R\x82\x84\x01R```@\x84\x01R\x80Q\x91\x82\x91\x82``\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q\x90a\x0C\x9A\x82a\x03\x1CV[_` \x83\x82\x81R\x01RV[\x90`@Qa\x0C\xB2\x81a\x03\x1CV[` `\x01\x82\x94\x80T\x84R\x01T\x91\x01RV[\x90`@Qa\x0C\xD0\x81a\x03=V[`@\x81\x93Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x83\x1B\x16\x83Rc\xFF\xFF\xFF\xFF\x81`\xC0\x1C\x16` \x84\x01R`\xE0\x1C\x91\x01RV[\x15a\r\x03WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FBLSApkRegistry.getApkHashAtBlock`D\x82\x01R\x7FNumberAndIndex: not latest apk u`d\x82\x01Rdpdate`\xD8\x1B`\x84\x82\x01R`\xA4\x90\xFD[a\r\x84a\x0C\x8DV[P`\x01\x80`\xA0\x1B\x03\x16\x90\x81_R`\x03` R`@_ \x91`\x01`@Q\x93a\r\xAA\x85a\x03\x1CV[\x80T\x85R\x01T` \x84\x01R_R`\x01` R`@_ T\x91\x82\x15a\r\xCCW\x91\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry.getRegisteredPubk`D\x82\x01R\x7Fey: operator is not registered\0\0`d\x82\x01R`\x84\x90\xFD[`@\x90`c\x19\x01\x12a\x01DW`@Q\x90a\x0EP\x82a\x03\x1CV[`d5\x82R`\x845` \x83\x01RV[`@\x90`#\x19\x01\x12a\x01DW`@Q\x90a\x0Ex\x82a\x03\x1CV[`$5\x82R`D5` \x83\x01RV[`@\x90a\x01#\x19\x01\x12a\x01DW`@Q\x90a\x0E\xA1\x82a\x03\x1CV[a\x01$5\x82Ra\x01D5` \x83\x01RV[\x15a\x0E\xB9WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R_Q` a\x1B]_9_Q\x90_R`D\x82\x01R\x7FKey: cannot register zero pubkey`d\x82\x01R\xFD[\x15a\x0F\x17WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R_Q` a\x1B]_9_Q\x90_R`D\x82\x01R\x7FKey: operator already registered`d\x82\x01Rf pubkey`\xC8\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a\x0F\x86WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R_Q` a\x1B]_9_Q\x90_R`D\x82\x01R\x7FKey: public key already register`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x94\x92\x90\x91a\x01@\x96\x94\x92\x86R` \x86\x01R`@\x85\x01R``\x84\x01R`@`\xA4`\x80\x85\x017`@`\xE4`\xC0\x85\x017a\x01\0\x83\x01Ra\x01 \x82\x01R\x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x01DW`@Q\x91a\x10A`@\x84a\x03YV[\x82\x90`@\x81\x01\x92\x83\x11a\x01DW\x90[\x82\x82\x10a\x10]WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x10PV[\x90`\x80`\xA3\x19\x83\x01\x12a\x01DW`@Qa\x10\x86\x81a\x03\x1CV[` a\x10\xA1\x82\x94a\x10\x98\x81`\xA4a\x10&V[\x84R`\xE4a\x10&V[\x91\x01RV[\x15a\x10\xADWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R_Q` a\x1B]_9_Q\x90_R`D\x82\x01R\x7FKey: either the G1 signature is `d\x82\x01R\x7Fwrong, or G1 and G2 private key `\x84\x82\x01Rk\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90`@`\xE4`\x80`\xC0\x85\x01\x94`d5\x81R`\x845` \x82\x01R\x83`\xA4\x81\x83\x017\x017V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x038W`\x05\x1B` \x01\x90V[\x90a\x11\x86\x82a\x11dV[a\x11\x93`@Q\x91\x82a\x03YV[\x82\x81R\x80\x92a\x11\xA4`\x1F\x19\x91a\x11dV[\x01\x90` 6\x91\x017V[\x90\x82\x10\x15a\x07dW\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x15a\x11\xDAW_\x19\x01\x90V[a\x11\xBAV[_\x19\x81\x01\x91\x90\x82\x11a\x11\xDAWV[\x80Q\x82\x10\x15a\x07dW` \x91`\x05\x1B\x01\x01\x90V[\x91\x90\x91a\x12\r\x83a\x11|V[\x92_[\x81\x81\x10a\x12\x1EWPPPP\x90V[a\x12Ca\x12=a\x12/\x83\x85\x87a\x11\xAEV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[a\x12X\x81`\xFF\x16_R`\x04` R`@_ \x90V[T\x80\x15\x80\x15a\x13\x89W[a\x13\x04W\x80[a\x12wW[PP`\x01\x01a\x12\x10V[\x85c\xFF\xFF\xFF\xFFa\x12\xB5a\x12\xA7a\x12\x98\x86`\xFF\x16_R`\x04` R`@_ \x90V[a\x12\xA1\x86a\x11\xDFV[\x90a\x07iV[PT`\xC0\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[\x16\x11\x15a\x12\xCBWa\x12\xC5\x90a\x11\xCEV[\x80a\x12hV[`\x01\x92\x91Pa\x12\xE8a\x12\xDFa\x12\xFD\x92a\x11\xDFV[c\xFF\xFF\xFF\xFF\x16\x90V[a\x12\xF2\x83\x89a\x11\xEDV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x90_a\x12mV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FBLSApkRegistry.getApkIndicesAtBl`D\x82\x01R\x7FockNumber: blockNumber is before`d\x82\x01Rp the first update`x\x1B`\x84\x82\x01R`\xA4\x90\xFD[Pa\x13\xADa\x12\xDFa\x12\xA7a\x13\xA8\x85`\xFF\x16_R`\x04` R`@_ \x90V[a\x07RV[\x86\x10a\x12bV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x13\xE6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FBLSApkRegistry._checkRegistryCoo`D\x82\x01R\x7Frdinator: caller is not the regi`d\x82\x01Ro9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90\x81Q\x81\x10\x15a\x07dW\x01` \x01\x90V[\x15a\x14\x82WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBLSApkRegistry._processQuorumApk`D\x82\x01R\x7FUpdate: quorum does not exist\0\0\0`d\x82\x01R`\x84\x90\xFD[\x91\x90a\x14\xF7a\x0C\x8DV[PCc\xFF\xFF\xFF\xFF\x16\x90_[\x84Q\x81\x10\x15a\x16\x82W\x80\x83a\x15-a\x12=a\x15\x1F`\x01\x95\x8Aa\x14jV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[a\x15B\x81`\xFF\x16_R`\x04` R`@_ \x90V[T\x90a\x15O\x82\x15\x15a\x14{V[a\x15\xD5a\x15\xB6a\x15\xA8a\x15~\x89a\x15ya\x15t\x87`\xFF\x16_R`\x05` R`@_ \x90V[a\x0C\xA5V[a\x16\xFBV[a\x08\xC9\x81a\x15\x97\x87`\xFF\x16_R`\x05` R`@_ \x90V[\x90` `\x01\x91\x80Q\x84U\x01Q\x91\x01UV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x92a\x12\xA1a\x15\xCF\x84`\xFF\x16_R`\x04` R`@_ \x90V[\x91a\x11\xDFV[P\x90\x83a\x15\xEDa\x12\xDF\x84Tc\xFF\xFF\xFF\xFF\x90`\xC0\x1C\x16\x90V[\x03a\x16\x16WPa\x16\x10\x92P\x90`@\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xC0\x1B\x82T\x16\x17\x90UV[\x01a\x15\x02V[\x81T`\x01`\x01`\xE0\x1B\x03\x16`\xE0\x94\x90\x94\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x93\x90\x93\x17\x90Ua\x16}\x91a\x16P\x90`\xFF\x16_R`\x04` R`@_ \x90V[a\x16la\x16[a\x03{V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x83RV[c\xFF\xFF\xFF\xFF\x87\x16` \x83\x01Ra\x02_V[a\x16\x10V[PPP\x90PV[`@Q\x90a\x01\x80a\x16\x9A\x81\x84a\x03YV[6\x837V[`@Q\x90a\x16\xAE` \x83a\x03YV[` 6\x837V[\x91\x90`@\x90``a\x16\xC4a\x0C\x8DV[\x94\x85\x92` \x85Q\x92a\x16\xD6\x85\x85a\x03YV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15a\x16\xF9WV[\xFE[` \x92\x91`\x80`@\x92a\x17\x0Ca\x0C\x8DV[\x95\x86\x93\x81\x86Q\x93a\x17\x1D\x86\x86a\x03YV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15a\x16\xF9W\x15a\x17NWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x90\xFD[`@Qa\x17\x8F\x81a\x03\x1CV[`@\x90\x81Qa\x17\x9E\x83\x82a\x03YV[\x826\x827\x81R` \x82Q\x91a\x17\xB3\x84\x84a\x03YV[\x836\x847\x01R\x80Qa\x17\xC5\x82\x82a\x03YV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90a\x18\x1B\x83\x83a\x03YV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01Ra\x18p\x83Q\x93\x84a\x03YV[\x82R` \x82\x01R\x90V[a\x18\x82a\x0C\x8DV[P`@Qa\x18\x8F\x81a\x03\x1CV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a\x11\xDAWV[\x90`\x02\x81\x10\x15a\x07dW`\x05\x1B\x01\x90V[\x90`\x01\x82\x01\x80\x92\x11a\x11\xDAWV[\x90`\x02\x82\x01\x80\x92\x11a\x11\xDAWV[\x90`\x03\x82\x01\x80\x92\x11a\x11\xDAWV[\x90`\x04\x82\x01\x80\x92\x11a\x11\xDAWV[\x90`\x05\x82\x01\x80\x92\x11a\x11\xDAWV[\x90`\x0C\x81\x10\x15a\x07dW`\x05\x1B\x01\x90V[\x15a\x19\"WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x90\xFD[\x92\x90\x91a\x19l`@a\x03\x8CV[\x93\x84R` \x84\x01Ra\x19~`@a\x03\x8CV[\x91\x82R` \x82\x01Ra\x19\x8Ea\x16\x89V[\x91_[`\x02\x81\x10a\x19\xCBWPPP` a\x01\x80\x91a\x19\xAAa\x16\x9FV[\x92\x83\x91`\x08a\x07\xCF\x19Z\x01\xFA\x80\x15a\x16\xF9Wa\x19\xC5\x90a\x19\x1BV[Q\x15\x15\x90V[\x80a\x19\xD7`\x01\x92a\x18\x9DV[a\x19\xE1\x82\x85a\x18\xB3V[QQa\x19\xED\x82\x88a\x19\nV[R` a\x19\xFA\x83\x86a\x18\xB3V[Q\x01Qa\x1A\x0Fa\x1A\t\x83a\x18\xC4V[\x88a\x19\nV[Ra\x1A\x1A\x82\x86a\x18\xB3V[QQQa\x1A)a\x1A\t\x83a\x18\xD2V[Ra\x1A?a\x1A7\x83\x87a\x18\xB3V[QQ` \x01\x90V[Qa\x1ALa\x1A\t\x83a\x18\xE0V[R` a\x1AY\x83\x87a\x18\xB3V[Q\x01QQa\x1Aia\x1A\t\x83a\x18\xEEV[Ra\x1A\x95a\x1A\x8Fa\x1A\x88` a\x1A\x7F\x86\x8Aa\x18\xB3V[Q\x01Q` \x01\x90V[Q\x92a\x18\xFCV[\x87a\x19\nV[R\x01a\x19\x91V[a\x1A\xA4a\x0C\x8DV[P\x80Q\x90\x81\x15\x80a\x1BPW[\x15a\x1A\xD1WPP`@Qa\x1A\xC5`@\x82a\x03YV[_\x81R_` \x82\x01R\x90V[` \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x91\x01Q\x06\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x03\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x81\x11a\x11\xDAW`@Q\x91a\x18p`@\x84a\x03YV[P` \x81\x01Q\x15a\x1A\xB0V\xFEBLSApkRegistry.registerBLSPublic\xA2dipfsX\"\x12 &$\xD7\xCD\xB2^m9\xE9\x80b\xD9\xD5\x07\xACf\xC32\x15*\xF6\xFD(\x18\x0E\x86\xF1\xCC?dd\x85dsolcC\0\x08\x1B\x003",
=======
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x15W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\xA2W\x80c\xBFy\xCEX\x11a\0qW\x80c\xBFy\xCEX\x14a\x03\xCCW\x80c\xD5%J\x8C\x14a\x03\xDFW\x80c\xDE)\xFA\xC0\x14a\x03\xFFW\x80c\xE8\xBB\x9A\xE6\x14a\x04\x1FW\x80c\xF4\xE2O\xE5\x14a\x04HW`\0\x80\xFD[\x80cm\x14\xA9\x87\x14a\x03\nW\x80cy\x16\xCE\xA6\x14a\x031W\x80c\x7F\xF8\x1A\x87\x14a\x03rW\x80c\xA3\xDB\x80\xE2\x14a\x03\xA5W`\0\x80\xFD[\x80c?\xB2yR\x11a\0\xE9W\x80c?\xB2yR\x14a\x01\xDFW\x80cG\xB3\x14\xE8\x14a\x01\xF2W\x80c_a\xA8\x84\x14a\x023W\x80c`WG\xD5\x14a\x02\x8FW\x80ch\xBC\xCA\xAC\x14a\x02\xDDW`\0\x80\xFD[\x80b\xA1\xF4\xCB\x14a\x01\x1AW\x80c\x13T*N\x14a\x01[W\x80c&\xD9A\xF2\x14a\x01\x92W\x80c7~\xD9\x9D\x14a\x01\xA7W[`\0\x80\xFD[a\x01Aa\x01(6`\x04a\x19\x04V[`\x03` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x84a\x01i6`\x04a\x19\x04V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01RV[a\x01\xA5a\x01\xA06`\x04a\x197V[a\x04[V[\0[a\x01\xCAa\x01\xB56`\x04a\x197V[`\xFF\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01RV[a\x01\xA5a\x01\xED6`\x04a\x19\xC2V[a\x05\xB4V[a\x02\x1Ba\x02\x006`\x04a\x1AhV[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01RV[a\x02\x82a\x02A6`\x04a\x197V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\xFF\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`@Qa\x01R\x91\x90a\x1A\x81V[a\x02\xA2a\x02\x9D6`\x04a\x1A\x98V[a\x06rV[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x01RV[a\x02\xF0a\x02\xEB6`\x04a\x1A\xC2V[a\x07\x05V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R` \x01a\x01RV[a\x02\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Da\x03?6`\x04a\x1A\x98V[a\x08\xA0V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01RV[a\x03\x85a\x03\x806`\x04a\x19\x04V[a\x08\xEBV[`@\x80Q\x83Q\x81R` \x93\x84\x01Q\x93\x81\x01\x93\x90\x93R\x82\x01R``\x01a\x01RV[a\x01Aa\x03\xB36`\x04a\x197V[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[a\x01\x84a\x03\xDA6`\x04a\x1B\nV[a\t\xB8V[a\x03\xF2a\x03\xED6`\x04a\x1BgV[a\x0E\x0CV[`@Qa\x01R\x91\x90a\x1B\xDFV[a\x01\x84a\x04\r6`\x04a\x19\x04V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x02\x1Ba\x04-6`\x04a\x1AhV[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xA5a\x04V6`\x04a\x19\xC2V[a\x10&V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xA3\x90a\x1C)V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16`\0\x90\x81R`\x04` R`@\x90 T\x15a\x05+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBLSApkRegistry.initializeQuorum:`D\x82\x01Ru quorum already exists`P\x1B`d\x82\x01R`\x84\x01a\x04\xA3V[`\xFF\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x84\x81Rc\xFF\xFF\xFF\xFFC\x81\x16\x82\x86\x01\x90\x81R\x82\x85\x01\x87\x81R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x94Q\x83\x16`\x01`\xE0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x95\x90\x93\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x96\x16\x91\x90\x93\x1C\x17\x93\x90\x93\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xA3\x90a\x1C)V[`\0a\x06\x07\x83a\x08\xEBV[P\x90Pa\x06\x14\x82\x82a\x10\xCFV[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06U\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[\x84`@Qa\x06e\x93\x92\x91\x90a\x1C\x9DV[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x04\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x06\xAFWa\x06\xAFa\x1D\tV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x07,Wa\x07,a\x1D\tV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x85\x90R`\x01`\xE0\x1B\x90\x91\x04\x81\x16\x92\x82\x01\x92\x90\x92R\x92P\x85\x16\x10\x15a\x07\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: index too recent\0\0`d\x82\x01R`\x84\x01a\x04\xA3V[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x08\x19WP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x08\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: not latest apk upd`d\x82\x01Rbate`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x04\xA3V[Q\x94\x93PPPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x08\xBCW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`@\x81\x90\x1B\x92Pc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x92P`\x01`\xE0\x1B\x90\x91\x04\x16\x83V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x91\x82\x01T\x81\x85\x01R\x94\x84R\x90\x91R\x81 T\x90\x91\x90\x80a\t\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry.getRegisteredPubk`D\x82\x01R\x7Fey: operator is not registered\0\0`d\x82\x01R`\x84\x01a\x04\xA3V[\x90\x94\x90\x93P\x91PPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xA3\x90a\x1C)V[`\0a\n0a\n\x196\x86\x90\x03\x86\x01`@\x87\x01a\x1D\x1FV[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x14\x15a\n\xB8W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` a\x1F6\x839\x81Q\x91R`D\x82\x01R\x7FKey: cannot register zero pubkey`d\x82\x01R`\x84\x01a\x04\xA3V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x0BBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a\x1F6\x839\x81Q\x91R`D\x82\x01R\x7FKey: operator already registered`d\x82\x01Rf pubkey`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x04\xA3V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a\x1F6\x839\x81Q\x91R`D\x82\x01R\x7FKey: public key already register`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x04\xA3V[`@\x80Q`\0\x91\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x91a\x0C\x1F\x91\x885\x91` \x80\x8B\x015\x92\x8B\x015\x91``\x8C\x015\x91`\x80\x8D\x01\x91`\xC0\x8E\x01\x91\x8D5\x91\x8E\x82\x015\x91\x01a\x1DQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x0CB\x91\x90a\x1D\x9CV[\x90Pa\x0C\xDCa\x0C{a\x0Cf\x83a\x0C`6\x8A\x90\x03\x8A\x01`@\x8B\x01a\x1D\x1FV[\x90a\x13\x1AV[a\x0Cu6\x89\x90\x03\x89\x01\x89a\x1D\x1FV[\x90a\x13\xB1V[a\x0C\x83a\x14EV[a\x0C\xC5a\x0C\xB6\x85a\x0C``@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[a\x0Cu6\x8A\x90\x03\x8A\x01\x8Aa\x1D\x1FV[a\x0C\xD76\x8A\x90\x03\x8A\x01`\x80\x8B\x01a\x1E\x0EV[a\x15\x05V[a\rwW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R`\0\x80Q` a\x1F6\x839\x81Q\x91R`D\x82\x01R\x7FKey: either the G1 signature is `d\x82\x01R\x7Fwrong, or G1 and G2 private key `\x84\x82\x01Rk\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x01a\x04\xA3V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x89\x82\x01\x805\x82U``\x8B\x015`\x01\x92\x83\x01U\x90\x83R\x81\x84 \x87\x90U\x86\x84R`\x02\x90\x92R\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90U\x90Q\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x91a\r\xFB\x91`\x80\x8A\x01\x90a\x1EkV[`@Q\x80\x91\x03\x90\xA2P\x94\x93PPPPV[```\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E)Wa\x0E)a\x19RV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0ERW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x10\x1DW`\0\x86\x86\x83\x81\x81\x10a\x0EtWa\x0Eta\x1D\tV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x04` R`@\x90 T\x90\x92P\x90P\x80\x15\x80a\x0E\xD7WP`\xFF\x82\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x90\x91\x90a\x0E\xBBWa\x0E\xBBa\x1D\tV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x0FdW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FBLSApkRegistry.getApkIndicesAtBl`D\x82\x01R\x7FockNumber: blockNumber is before`d\x82\x01Rp the first update`x\x1B`\x84\x82\x01R`\xA4\x01a\x04\xA3V[\x80[\x80\x15a\x10\x07W`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x90 \x87\x90a\x0F\x8B`\x01\x84a\x1E\xB5V[\x81T\x81\x10a\x0F\x9BWa\x0F\x9Ba\x1D\tV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11a\x0F\xF5Wa\x0F\xC4`\x01\x82a\x1E\xB5V[\x85\x85\x81Q\x81\x10a\x0F\xD6Wa\x0F\xD6a\x1D\tV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x10\x07V[\x80a\x0F\xFF\x81a\x1E\xCCV[\x91PPa\x0FfV[PPP\x80\x80a\x10\x15\x90a\x1E\xE3V[\x91PPa\x0EXV[P\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xA3\x90a\x1C)V[`\0a\x10y\x83a\x08\xEBV[P\x90Pa\x10\x8E\x82a\x10\x89\x83a\x17rV[a\x10\xCFV[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x06U\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[\x83Q\x81\x10\x15a\x13\x14W`\0\x84\x82\x81Q\x81\x10a\x11\x03Wa\x11\x03a\x1D\tV[\x01` \x90\x81\x01Q`\xF8\x1C`\0\x81\x81R`\x04\x90\x92R`@\x90\x91 T\x90\x91P\x80a\x11\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBLSApkRegistry._processQuorumApk`D\x82\x01R\x7FUpdate: quorum does not exist\0\0\0`d\x82\x01R`\x84\x01a\x04\xA3V[`\xFF\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01Ra\x11\xC7\x90\x86a\x13\xB1V[`\xFF\x83\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x85Q\x80\x82U\x86\x84\x01\x80Q`\x01\x93\x84\x01U\x90\x85RQ\x83R\x81\x84 \x94\x84R`\x04\x90\x92R\x82 \x93\x97P\x91\x92\x90\x91a\x12\x10\x90\x85a\x1E\xB5V[\x81T\x81\x10a\x12 Wa\x12 a\x1D\tV[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16`\x01`\xC0\x1B\x90\x92\x04\x16\x14\x15a\x12aW\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`@\x83\x90\x1C\x17\x81Ua\x12\xFDV[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01`\xE0\x1B\x81\x81\x02`\x01`\x01`\xE0\x1B\x03\x94\x85\x16\x17\x85U`\xFF\x88\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x8B\x16\x81R\x80\x84\x01\x96\x87R\x80\x83\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x93Q\x93\x01\x80T\x95Q\x92Q\x87\x16\x90\x94\x02\x91\x90\x95\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x94\x90\x94\x16\x91\x90\x94\x1C\x17\x91\x90\x91\x17\x90\x92\x16\x17\x90U[PPPP\x80\x80a\x13\x0C\x90a\x1E\xE3V[\x91PPa\x10\xE6V[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x136a\x181V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13iWa\x13kV[\xFE[P\x80a\x13\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04\xA3V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13\xCDa\x18OV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13iWP\x80a\x13\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04\xA3V[a\x14Ma\x18mV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R`\0\x91a\x154a\x18\x92V[`\0[`\x02\x81\x10\x15a\x16\xF9W`\0a\x15M\x82`\x06a\x1E\xFEV[\x90P\x84\x82`\x02\x81\x10a\x15aWa\x15aa\x1D\tV[` \x02\x01QQ\x83a\x15s\x83`\0a\x1F\x1DV[`\x0C\x81\x10a\x15\x83Wa\x15\x83a\x1D\tV[` \x02\x01R\x84\x82`\x02\x81\x10a\x15\x9AWa\x15\x9Aa\x1D\tV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x15\xB1\x91\x90a\x1F\x1DV[`\x0C\x81\x10a\x15\xC1Wa\x15\xC1a\x1D\tV[` \x02\x01R\x83\x82`\x02\x81\x10a\x15\xD8Wa\x15\xD8a\x1D\tV[` \x02\x01QQQ\x83a\x15\xEB\x83`\x02a\x1F\x1DV[`\x0C\x81\x10a\x15\xFBWa\x15\xFBa\x1D\tV[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\x12Wa\x16\x12a\x1D\tV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x16+\x83`\x03a\x1F\x1DV[`\x0C\x81\x10a\x16;Wa\x16;a\x1D\tV[` \x02\x01R\x83\x82`\x02\x81\x10a\x16RWa\x16Ra\x1D\tV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a\x16mWa\x16ma\x1D\tV[` \x02\x01Q\x83a\x16~\x83`\x04a\x1F\x1DV[`\x0C\x81\x10a\x16\x8EWa\x16\x8Ea\x1D\tV[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\xA5Wa\x16\xA5a\x1D\tV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x16\xC0Wa\x16\xC0a\x1D\tV[` \x02\x01Q\x83a\x16\xD1\x83`\x05a\x1F\x1DV[`\x0C\x81\x10a\x16\xE1Wa\x16\xE1a\x1D\tV[` \x02\x01RP\x80a\x16\xF1\x81a\x1E\xE3V[\x91PPa\x157V[Pa\x17\x02a\x18\xB1V[`\0` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13iWP\x80a\x17bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x04\xA3V[PQ\x15\x15\x98\x97PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x17\x97WP` \x82\x01Q\x15[\x15a\x17\xB5WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x17\xFA\x91\x90a\x1D\x9CV[a\x18$\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1E\xB5V[\x90R\x92\x91PPV[\x91\x90PV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x18\x80a\x18\xCFV[\x81R` \x01a\x18\x8Da\x18\xCFV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18,W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19\x16W`\0\x80\xFD[a\x19\x1F\x82a\x18\xEDV[\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x18,W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19IW`\0\x80\xFD[a\x19\x1F\x82a\x19&V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\x8BWa\x19\x8Ba\x19RV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\xBAWa\x19\xBAa\x19RV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\xD5W`\0\x80\xFD[a\x19\xDE\x83a\x18\xEDV[\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x19\xFCW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1A\x10W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1A\"Wa\x1A\"a\x19RV[a\x1A4`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x19\x91V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x1AJW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1AzW`\0\x80\xFD[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x06\xFFV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xABW`\0\x80\xFD[a\x1A\xB4\x83a\x19&V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1A\xD7W`\0\x80\xFD[a\x1A\xE0\x84a\x19&V[\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\xF9W`\0\x80\xFD[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x83\x85\x03a\x01`\x81\x12\x15a\x1B!W`\0\x80\xFD[a\x1B*\x85a\x18\xEDV[\x93Pa\x01\0`\x1F\x19\x82\x01\x12\x15a\x1B?W`\0\x80\xFD[` \x85\x01\x92P`@a\x01\x1F\x19\x82\x01\x12\x15a\x1BXW`\0\x80\xFD[Pa\x01 \x84\x01\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1B|W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\x94W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1B\xA8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1B\xB7W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x1B\xC9W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1C\x1DW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1B\xFBV[P\x90\x96\x95PPPPPPV[` \x80\x82R`N\x90\x82\x01R\x7FBLSApkRegistry.onlyRegistryCoord`@\x82\x01R\x7Finator: caller is not the regist``\x82\x01Rm9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x91\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R`\0` \x84\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x1C\xDFW\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x1C\xC3V[\x81\x81\x11\x15a\x1C\xF1W`\0`\x80\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`\x80\x01\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x1D1W`\0\x80\xFD[a\x1D9a\x19hV[\x825\x81R` \x83\x015` \x82\x01R\x80\x91PP\x92\x91PPV[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01R`@\x85`\x80\x83\x017`\0`\xC0\x82\x01`\0\x81R`@\x86\x827PPa\x01\0\x81\x01\x92\x90\x92Ra\x01 \x82\x01Ra\x01@\x01\x96\x95PPPPPPV[`\0\x82a\x1D\xB9WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x82`\x1F\x83\x01\x12a\x1D\xCFW`\0\x80\xFD[a\x1D\xD7a\x19hV[\x80`@\x84\x01\x85\x81\x11\x15a\x1D\xE9W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x1E\x03W\x805\x84R` \x93\x84\x01\x93\x01a\x1D\xEBV[P\x90\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x1E W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1ECWa\x1ECa\x19RV[`@Ra\x1EP\x84\x84a\x1D\xBEV[\x81Ra\x1E_\x84`@\x85\x01a\x1D\xBEV[` \x82\x01R\x93\x92PPPV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`\xC0\x81\x01`@\x83\x81\x84\x017`\x80\x82\x01`\0\x81R`@\x80\x85\x01\x827P`\0\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1E\xC7Wa\x1E\xC7a\x1E\x9FV[P\x03\x90V[`\0\x81a\x1E\xDBWa\x1E\xDBa\x1E\x9FV[P`\0\x19\x01\x90V[`\0`\0\x19\x82\x14\x15a\x1E\xF7Wa\x1E\xF7a\x1E\x9FV[P`\x01\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1F\x18Wa\x1F\x18a\x1E\x9FV[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x1F0Wa\x1F0a\x1E\x9FV[P\x01\x90V\xFEBLSApkRegistry.registerBLSPublic\xA2dipfsX\"\x12 P\xDD*\xB3\x96!Ie\x93\x9C\x9B\xAE\tJb5\x0C\x11/\xFEX\x17\xE6\x9D1'M\0\x1E}\xCF!dsolcC\0\x08\x0C\x003",
>>>>>>> dev:crates/utils/src/middleware/blsapkregistry.rs
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
    ///Container for all the [`BLSApkRegistry`](self) function calls.
    pub enum BLSApkRegistryCalls {
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
    }
    #[automatically_derived]
    impl BLSApkRegistryCalls {
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
            [232u8, 187u8, 154u8, 230u8],
            [244u8, 226u8, 79u8, 229u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BLSApkRegistryCalls {
        const NAME: &'static str = "BLSApkRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 18usize;
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
                -> alloy_sol_types::Result<BLSApkRegistryCalls>] = &[
                {
                    fn operatorToPubkey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <operatorToPubkeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::operatorToPubkey)
                    }
                    operatorToPubkey
                },
                {
                    fn getOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <getOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::getOperatorId)
                    }
                    getOperatorId
                },
                {
                    fn initializeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <initializeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::initializeQuorum)
                    }
                    initializeQuorum
                },
                {
                    fn getApkHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <getApkHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::getApkHistoryLength)
                    }
                    getApkHistoryLength
                },
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn getOperatorFromPubkeyHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <getOperatorFromPubkeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::getOperatorFromPubkeyHash)
                    }
                    getOperatorFromPubkeyHash
                },
                {
                    fn getApk(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <getApkCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(BLSApkRegistryCalls::getApk)
                    }
                    getApk
                },
                {
                    fn getApkUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <getApkUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::getApkUpdateAtIndex)
                    }
                    getApkUpdateAtIndex
                },
                {
                    fn getApkHashAtBlockNumberAndIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <getApkHashAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BLSApkRegistryCalls::getApkHashAtBlockNumberAndIndex)
                    }
                    getApkHashAtBlockNumberAndIndex
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn apkHistory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <apkHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(BLSApkRegistryCalls::apkHistory)
                    }
                    apkHistory
                },
                {
                    fn getRegisteredPubkey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <getRegisteredPubkeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::getRegisteredPubkey)
                    }
                    getRegisteredPubkey
                },
                {
                    fn currentApk(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <currentApkCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(BLSApkRegistryCalls::currentApk)
                    }
                    currentApk
                },
                {
                    fn registerBLSPublicKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <registerBLSPublicKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::registerBLSPublicKey)
                    }
                    registerBLSPublicKey
                },
                {
                    fn getApkIndicesAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <getApkIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BLSApkRegistryCalls::getApkIndicesAtBlockNumber)
                    }
                    getApkIndicesAtBlockNumber
                },
                {
                    fn operatorToPubkeyHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <operatorToPubkeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::operatorToPubkeyHash)
                    }
                    operatorToPubkeyHash
                },
                {
                    fn pubkeyHashToOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <pubkeyHashToOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::pubkeyHashToOperator)
                    }
                    pubkeyHashToOperator
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BLSApkRegistryCalls::deregisterOperator)
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
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
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
            }
        }
    }
    ///Container for all the [`BLSApkRegistry`](self) events.
    pub enum BLSApkRegistryEvents {
        Initialized(Initialized),
        NewPubkeyRegistration(NewPubkeyRegistration),
        OperatorAddedToQuorums(OperatorAddedToQuorums),
        OperatorRemovedFromQuorums(OperatorRemovedFromQuorums),
    }
    #[automatically_derived]
    impl BLSApkRegistryEvents {
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
    impl alloy_sol_types::SolEventInterface for BLSApkRegistryEvents {
        const NAME: &'static str = "BLSApkRegistryEvents";
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
    impl alloy_sol_types::private::IntoLogData for BLSApkRegistryEvents {
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
    /**Creates a new wrapper around an on-chain [`BLSApkRegistry`](self) contract instance.

    See the [wrapper's documentation](`BLSApkRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BLSApkRegistryInstance<T, P, N> {
        BLSApkRegistryInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<BLSApkRegistryInstance<T, P, N>>>
    {
        BLSApkRegistryInstance::<T, P, N>::deploy(provider, _registryCoordinator)
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
        BLSApkRegistryInstance::<T, P, N>::deploy_builder(provider, _registryCoordinator)
    }
    /**A [`BLSApkRegistry`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BLSApkRegistry`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BLSApkRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BLSApkRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BLSApkRegistryInstance")
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
        > BLSApkRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BLSApkRegistry`](self) contract instance.

        See the [wrapper's documentation](`BLSApkRegistryInstance`) for more details.*/
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
        ) -> alloy_contract::Result<BLSApkRegistryInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> BLSApkRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BLSApkRegistryInstance<T, P, N> {
            BLSApkRegistryInstance {
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
        > BLSApkRegistryInstance<T, P, N>
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
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > BLSApkRegistryInstance<T, P, N>
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
