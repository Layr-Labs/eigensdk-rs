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
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<24>,
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
        impl ::core::convert::From<ApkUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: ApkUpdate) -> Self {
                (value.apkHash, value.updateBlockNumber, value.nextUpdateBlockNumber)
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
        impl alloy_sol_types::SolType for ApkUpdate {
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
        impl alloy_sol_types::SolStruct for ApkUpdate {
            const NAME: &'static str = "ApkUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ApkUpdate(bytes24 apkHash,uint32 updateBlockNumber,uint32 nextUpdateBlockNumber)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
        impl ::core::convert::From<PubkeyRegistrationParams>
        for UnderlyingRustTuple<'_> {
            fn from(value: PubkeyRegistrationParams) -> Self {
                (value.pubkeyRegistrationSignature, value.pubkeyG1, value.pubkeyG2)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for PubkeyRegistrationParams {
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
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(
                        &self.pubkeyG1,
                    ),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(
                        &self.pubkeyG2,
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
        impl alloy_sol_types::SolType for PubkeyRegistrationParams {
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
        impl alloy_sol_types::SolStruct for PubkeyRegistrationParams {
            const NAME: &'static str = "PubkeyRegistrationParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PubkeyRegistrationParams(BN254.G1Point pubkeyRegistrationSignature,BN254.G1Point pubkeyG1,BN254.G2Point pubkeyG2)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(3);
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
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pubkeyRegistrationSignature,
                        )
                        .0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pubkeyG1,
                        )
                        .0,
                    <BN254::G2Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pubkeyG2,
                        )
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
            f.debug_tuple("IBLSApkRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IBLSApkRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IBLSApkRegistry`](self) contract instance.

See the [wrapper's documentation](`IBLSApkRegistryInstance`) for more details.*/
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
    > IBLSApkRegistryInstance<T, P, N> {
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
    > IBLSApkRegistryInstance<T, P, N> {
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
    ///0x60a060405234801561000f575f5ffd5b50604051611fd6380380611fd683398101604081905261002e91610108565b6001600160a01b0381166080528061004461004b565b5050610135565b5f54610100900460ff16156100b65760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff9081161015610106575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b5f60208284031215610118575f5ffd5b81516001600160a01b038116811461012e575f5ffd5b9392505050565b608051611e6d6101695f395f818161030301528181610457015281816105ae015281816109a70152610ff30152611e6d5ff3fe608060405234801561000f575f5ffd5b5060043610610110575f3560e01c80636d14a9871161009e578063bf79ce581161006e578063bf79ce58146103bf578063d5254a8c146103d2578063de29fac0146103f2578063e8bb9ae614610411578063f4e24fe514610439575f5ffd5b80636d14a987146102fe5780637916cea6146103255780637ff81a8714610366578063a3db80e214610399575f5ffd5b80633fb27952116100e45780633fb27952146101d657806347b314e8146101e95780635f61a88414610229578063605747d51461028357806368bccaac146102d1575f5ffd5b8062a1f4cb1461011457806313542a4e1461015457806326d941f21461018a578063377ed99d1461019f575b5f5ffd5b61013a610122366004611899565b60036020525f90815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b61017c610162366004611899565b6001600160a01b03165f9081526001602052604090205490565b60405190815260200161014b565b61019d6101983660046118c9565b61044c565b005b6101c16101ad3660046118c9565b60ff165f9081526004602052604090205490565b60405163ffffffff909116815260200161014b565b61019d6101e4366004611950565b6105a3565b6102116101f73660046119f5565b5f908152600260205260409020546001600160a01b031690565b6040516001600160a01b03909116815260200161014b565b6102766102373660046118c9565b604080518082019091525f80825260208201525060ff165f90815260056020908152604091829020825180840190935280548352600101549082015290565b60405161014b9190611a0c565b610296610291366004611a23565b61065f565b60408051825167ffffffffffffffff1916815260208084015163ffffffff90811691830191909152928201519092169082015260600161014b565b6102e46102df366004611a4b565b6106f0565b60405167ffffffffffffffff19909116815260200161014b565b6102117f000000000000000000000000000000000000000000000000000000000000000081565b610338610333366004611a23565b610889565b6040805167ffffffffffffffff19909416845263ffffffff928316602085015291169082015260600161014b565b610379610374366004611899565b6108d0565b60408051835181526020938401519381019390935282015260600161014b565b61013a6103a73660046118c9565b60056020525f90815260409020805460019091015482565b61017c6103cd366004611a8f565b61099b565b6103e56103e0366004611ae7565b610de1565b60405161014b9190611b59565b61017c610400366004611899565b60016020525f908152604090205481565b61021161041f3660046119f5565b60026020525f90815260409020546001600160a01b031681565b61019d610447366004611950565b610fe8565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461049d5760405162461bcd60e51b815260040161049490611ba1565b60405180910390fd5b60ff81165f908152600460205260409020541561051b5760405162461bcd60e51b815260206004820152603660248201527f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a6044820152752071756f72756d20616c72656164792065786973747360501b6064820152608401610494565b60ff165f908152600460209081526040808320815160608101835284815263ffffffff4381168286019081528285018781528454600181018655948852959096209151919092018054955194518316600160e01b026001600160e01b0395909316600160c01b026001600160e01b03199096169190931c179390931791909116919091179055565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146105eb5760405162461bcd60e51b815260040161049490611ba1565b5f6105f5836108d0565b509050610602828261108f565b7f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e83610642856001600160a01b03165f9081526001602052604090205490565b8460405161065293929190611c15565b60405180910390a1505050565b604080516060810182525f808252602080830182905282840182905260ff86168252600490529190912080548390811061069b5761069b611c60565b5f91825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b8204811694830194909452600160e01b90049092169082015290505b92915050565b60ff83165f90815260046020526040812080548291908490811061071657610716611c60565b5f91825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b82048116948301859052600160e01b9091048116928201929092529250851610156107dc5760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a20696e64657820746f6f20726563656e7400006064820152608401610494565b604081015163ffffffff1615806108025750806040015163ffffffff168463ffffffff16105b6108805760405162461bcd60e51b815260206004820152604360248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a206e6f74206c61746573742061706b2075706460648201526261746560e81b608482015260a401610494565b51949350505050565b6004602052815f5260405f2081815481106108a2575f80fd5b5f91825260209091200154604081901b925063ffffffff600160c01b820481169250600160e01b9091041683565b604080518082019091525f80825260208201526001600160a01b0382165f818152600360209081526040808320815180830183528154815260019182015481850152948452909152812054909190806109915760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e676574526567697374657265645075626b60448201527f65793a206f70657261746f72206973206e6f74207265676973746572656400006064820152608401610494565b9094909350915050565b5f336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146109e45760405162461bcd60e51b815260040161049490611ba1565b5f610a106109fa36869003860160408701611c74565b80515f9081526020918201519091526040902090565b90507fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb58103610a96576040805162461bcd60e51b81526020600482015260248101919091525f516020611e185f395f51905f5260448201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b65796064820152608401610494565b6001600160a01b0385165f9081526001602052604090205415610b1e5760405162461bcd60e51b815260206004820152604760248201525f516020611e185f395f51905f5260448201527f4b65793a206f70657261746f7220616c72656164792072656769737465726564606482015266207075626b657960c81b608482015260a401610494565b5f818152600260205260409020546001600160a01b031615610ba05760405162461bcd60e51b815260206004820152604260248201525f516020611e185f395f51905f5260448201527f4b65793a207075626c6963206b657920616c7265616479207265676973746572606482015261195960f21b608482015260a401610494565b604080515f917f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000191610bf8918835916020808b0135928b01359160608c01359160808d019160c08e01918d35918e8201359101611ca5565b604051602081830303815290604052805190602001205f1c610c1a9190611ce7565b9050610cb3610c53610c3e83610c38368a90038a0160408b01611c74565b906112cb565b610c4d36899003890189611c74565b9061135a565b610c5b6113ed565b610c9c610c8d85610c386040805180820182525f80825260209182015281518083019092526001825260029082015290565b610c4d368a90038a018a611c74565b610cae368a90038a0160808b01611d48565b6114ad565b610d4d5760405162461bcd60e51b815260206004820152606c60248201525f516020611e185f395f51905f5260448201527f4b65793a2065697468657220746865204731207369676e61747572652069732060648201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060848201526b0c8de40dcdee840dac2e8c6d60a31b60a482015260c401610494565b6001600160a01b0386165f8181526003602090815260408083208982018035825560608b013560019283015590835281842087905586845260029092529182902080546001600160a01b0319168417905590517fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba382804191610dd09160808a0190611d87565b60405180910390a250949350505050565b60605f8367ffffffffffffffff811115610dfd57610dfd6118e2565b604051908082528060200260200182016040528015610e26578160200160208202803683370190505b5090505f5b84811015610fdf575f868683818110610e4657610e46611c60565b919091013560f81c5f818152600460205260409020549092509050801580610ea6575060ff82165f9081526004602052604081208054909190610e8b57610e8b611c60565b5f91825260209091200154600160c01b900463ffffffff1686105b15610f335760405162461bcd60e51b815260206004820152605160248201527f424c5341706b52656769737472792e67657441706b496e64696365734174426c60448201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f7265606482015270207468652066697273742075706461746560781b608482015260a401610494565b805b8015610fd45760ff83165f9081526004602052604090208790610f59600184611dc5565b81548110610f6957610f69611c60565b5f91825260209091200154600160c01b900463ffffffff1611610fc257610f91600182611dc5565b858581518110610fa357610fa3611c60565b602002602001019063ffffffff16908163ffffffff1681525050610fd4565b80610fcc81611dd8565b915050610f35565b505050600101610e2b565b50949350505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146110305760405162461bcd60e51b815260040161049490611ba1565b5f61103a836108d0565b50905061104f8261104a8361170b565b61108f565b7ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e83610642856001600160a01b03165f9081526001602052604090205490565b604080518082019091525f80825260208201525f5b83518110156112c5575f8482815181106110c0576110c0611c60565b0160209081015160f81c5f81815260049092526040822054909250908190036111515760405162461bcd60e51b815260206004820152603d60248201527f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b60448201527f5570646174653a2071756f72756d20646f6573206e6f742065786973740000006064820152608401610494565b60ff82165f908152600560209081526040918290208251808401909352805483526001015490820152611184908661135a565b60ff83165f8181526005602090815260408083208551808255868401805160019384015590855251835281842094845260049092528220939750919290916111cc9085611dc5565b815481106111dc576111dc611c60565b5f918252602090912001805490915063ffffffff438116600160c01b909204160361121a5780546001600160c01b031916604083901c1781556112b5565b805463ffffffff438116600160e01b8181026001600160e01b0394851617855560ff88165f908152600460209081526040808320815160608101835267ffffffffffffffff198b16815280840196875280830185815282546001810184559286529390942093519301805495519251871690940291909516600160c01b026001600160e01b0319949094169190941c17919091179092161790555b5050600190920191506110a49050565b50505050565b604080518082019091525f80825260208201526112e66117c7565b835181526020808501519082015260408082018490525f908360608460076107d05a03fa9050808061131457fe5b50806113525760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610494565b505092915050565b604080518082019091525f80825260208201526113756117e5565b835181526020808501518183015283516040808401919091529084015160608301525f908360808460066107d05a03fa905080806113af57fe5b50806113525760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610494565b6113f5611803565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820182528581526020808201859052825180840190935285835282018390525f916114db611828565b5f5b6002811015611692575f6114f2826006611ded565b905084826002811061150657611506611c60565b60200201515183611517835f611e04565b600c811061152757611527611c60565b602002015284826002811061153e5761153e611c60565b602002015160200151838260016115559190611e04565b600c811061156557611565611c60565b602002015283826002811061157c5761157c611c60565b602002015151518361158f836002611e04565b600c811061159f5761159f611c60565b60200201528382600281106115b6576115b6611c60565b60200201515160016020020151836115cf836003611e04565b600c81106115df576115df611c60565b60200201528382600281106115f6576115f6611c60565b6020020151602001515f6002811061161057611610611c60565b602002015183611621836004611e04565b600c811061163157611631611c60565b602002015283826002811061164857611648611c60565b60200201516020015160016002811061166357611663611c60565b602002015183611674836005611e04565b600c811061168457611684611c60565b6020020152506001016114dd565b5061169b611847565b5f6020826101808560086107d05a03fa905080806116b557fe5b50806116fb5760405162461bcd60e51b81526020600482015260156024820152741c185a5c9a5b99cb5bdc18dbd9194b59985a5b1959605a1b6044820152606401610494565b5051151598975050505050505050565b604080518082019091525f8082526020820152815115801561172f57506020820151155b1561174c575050604080518082019091525f808252602082015290565b6040518060400160405280835f015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4784602001516117909190611ce7565b6117ba907f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611dc5565b905292915050565b919050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280611816611865565b8152602001611823611865565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b80356001600160a01b03811681146117c2575f5ffd5b5f602082840312156118a9575f5ffd5b6118b282611883565b9392505050565b803560ff811681146117c2575f5ffd5b5f602082840312156118d9575f5ffd5b6118b2826118b9565b634e487b7160e01b5f52604160045260245ffd5b6040805190810167ffffffffffffffff81118282101715611919576119196118e2565b60405290565b604051601f8201601f1916810167ffffffffffffffff81118282101715611948576119486118e2565b604052919050565b5f5f60408385031215611961575f5ffd5b61196a83611883565b9150602083013567ffffffffffffffff811115611985575f5ffd5b8301601f81018513611995575f5ffd5b803567ffffffffffffffff8111156119af576119af6118e2565b6119c2601f8201601f191660200161191f565b8181528660208385010111156119d6575f5ffd5b816020840160208301375f602083830101528093505050509250929050565b5f60208284031215611a05575f5ffd5b5035919050565b8151815260208083015190820152604081016106ea565b5f5f60408385031215611a34575f5ffd5b611a3d836118b9565b946020939093013593505050565b5f5f5f60608486031215611a5d575f5ffd5b611a66846118b9565b9250602084013563ffffffff81168114611a7e575f5ffd5b929592945050506040919091013590565b5f5f5f838503610160811215611aa3575f5ffd5b611aac85611883565b9350610100601f1982011215611ac0575f5ffd5b602085019250604061011f1982011215611ad8575f5ffd5b50610120840190509250925092565b5f5f5f60408486031215611af9575f5ffd5b833567ffffffffffffffff811115611b0f575f5ffd5b8401601f81018613611b1f575f5ffd5b803567ffffffffffffffff811115611b35575f5ffd5b866020828401011115611b46575f5ffd5b6020918201979096509401359392505050565b602080825282518282018190525f918401906040840190835b81811015611b9657835163ffffffff16835260209384019390920191600101611b72565b509095945050505050565b6020808252604e908201527f424c5341706b52656769737472792e6f6e6c795265676973747279436f6f726460408201527f696e61746f723a2063616c6c6572206973206e6f74207468652072656769737460608201526d393c9031b7b7b93234b730ba37b960911b608082015260a00190565b60018060a01b0384168152826020820152606060408201525f82518060608401528060208501608085015e5f608082850101526080601f19601f830116840101915050949350505050565b634e487b7160e01b5f52603260045260245ffd5b5f6040828403128015611c85575f5ffd5b50611c8e6118f6565b823581526020928301359281019290925250919050565b888152876020820152866040820152856060820152604085608083013760408460c0830137610100810192909252610120820152610140019695505050505050565b5f82611d0157634e487b7160e01b5f52601260045260245ffd5b500690565b5f82601f830112611d15575f5ffd5b611d1d6118f6565b806040840185811115611d2e575f5ffd5b845b81811015611b96578035845260209384019301611d30565b5f6080828403128015611d59575f5ffd5b50611d626118f6565b611d6c8484611d06565b8152611d7b8460408501611d06565b60208201529392505050565b823581526020808401359082015260c0810160408381840137604080840160808401379392505050565b634e487b7160e01b5f52601160045260245ffd5b818103818111156106ea576106ea611db1565b5f81611de657611de6611db1565b505f190190565b80820281158282048414176106ea576106ea611db1565b808201808211156106ea576106ea611db156fe424c5341706b52656769737472792e7265676973746572424c535075626c6963a2646970667358221220c19864f239c758ed49daf065941962208584aef050e2a28b0c61f4f4b2a159dd64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x1F\xD68\x03\x80a\x1F\xD6\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\x08V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R\x80a\0Da\0KV[PPa\x015V[_Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x10\x15a\x01\x06W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[_` \x82\x84\x03\x12\x15a\x01\x18W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01.W__\xFD[\x93\x92PPPV[`\x80Qa\x1Ema\x01i_9_\x81\x81a\x03\x03\x01R\x81\x81a\x04W\x01R\x81\x81a\x05\xAE\x01R\x81\x81a\t\xA7\x01Ra\x0F\xF3\x01Ra\x1Em_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x10W_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\x9EW\x80c\xBFy\xCEX\x11a\0nW\x80c\xBFy\xCEX\x14a\x03\xBFW\x80c\xD5%J\x8C\x14a\x03\xD2W\x80c\xDE)\xFA\xC0\x14a\x03\xF2W\x80c\xE8\xBB\x9A\xE6\x14a\x04\x11W\x80c\xF4\xE2O\xE5\x14a\x049W__\xFD[\x80cm\x14\xA9\x87\x14a\x02\xFEW\x80cy\x16\xCE\xA6\x14a\x03%W\x80c\x7F\xF8\x1A\x87\x14a\x03fW\x80c\xA3\xDB\x80\xE2\x14a\x03\x99W__\xFD[\x80c?\xB2yR\x11a\0\xE4W\x80c?\xB2yR\x14a\x01\xD6W\x80cG\xB3\x14\xE8\x14a\x01\xE9W\x80c_a\xA8\x84\x14a\x02)W\x80c`WG\xD5\x14a\x02\x83W\x80ch\xBC\xCA\xAC\x14a\x02\xD1W__\xFD[\x80b\xA1\xF4\xCB\x14a\x01\x14W\x80c\x13T*N\x14a\x01TW\x80c&\xD9A\xF2\x14a\x01\x8AW\x80c7~\xD9\x9D\x14a\x01\x9FW[__\xFD[a\x01:a\x01\"6`\x04a\x18\x99V[`\x03` R_\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01|a\x01b6`\x04a\x18\x99V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01KV[a\x01\x9Da\x01\x986`\x04a\x18\xC9V[a\x04LV[\0[a\x01\xC1a\x01\xAD6`\x04a\x18\xC9V[`\xFF\x16_\x90\x81R`\x04` R`@\x90 T\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01KV[a\x01\x9Da\x01\xE46`\x04a\x19PV[a\x05\xA3V[a\x02\x11a\x01\xF76`\x04a\x19\xF5V[_\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01KV[a\x02va\x0276`\x04a\x18\xC9V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`\xFF\x16_\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`@Qa\x01K\x91\x90a\x1A\x0CV[a\x02\x96a\x02\x916`\x04a\x1A#V[a\x06_V[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x01KV[a\x02\xE4a\x02\xDF6`\x04a\x1AKV[a\x06\xF0V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R` \x01a\x01KV[a\x02\x11\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x038a\x0336`\x04a\x1A#V[a\x08\x89V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01KV[a\x03ya\x03t6`\x04a\x18\x99V[a\x08\xD0V[`@\x80Q\x83Q\x81R` \x93\x84\x01Q\x93\x81\x01\x93\x90\x93R\x82\x01R``\x01a\x01KV[a\x01:a\x03\xA76`\x04a\x18\xC9V[`\x05` R_\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[a\x01|a\x03\xCD6`\x04a\x1A\x8FV[a\t\x9BV[a\x03\xE5a\x03\xE06`\x04a\x1A\xE7V[a\r\xE1V[`@Qa\x01K\x91\x90a\x1BYV[a\x01|a\x04\x006`\x04a\x18\x99V[`\x01` R_\x90\x81R`@\x90 T\x81V[a\x02\x11a\x04\x1F6`\x04a\x19\xF5V[`\x02` R_\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x9Da\x04G6`\x04a\x19PV[a\x0F\xE8V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x94\x90a\x1B\xA1V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16_\x90\x81R`\x04` R`@\x90 T\x15a\x05\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBLSApkRegistry.initializeQuorum:`D\x82\x01Ru quorum already exists`P\x1B`d\x82\x01R`\x84\x01a\x04\x94V[`\xFF\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x84\x81Rc\xFF\xFF\xFF\xFFC\x81\x16\x82\x86\x01\x90\x81R\x82\x85\x01\x87\x81R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x94Q\x83\x16`\x01`\xE0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x95\x90\x93\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x96\x16\x91\x90\x93\x1C\x17\x93\x90\x93\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x94\x90a\x1B\xA1V[_a\x05\xF5\x83a\x08\xD0V[P\x90Pa\x06\x02\x82\x82a\x10\x8FV[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06B\x85`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01` R`@\x90 T\x90V[\x84`@Qa\x06R\x93\x92\x91\x90a\x1C\x15V[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x04\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x06\x9BWa\x06\x9Ba\x1C`V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\xFF\x83\x16_\x90\x81R`\x04` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x07\x16Wa\x07\x16a\x1C`V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x85\x90R`\x01`\xE0\x1B\x90\x91\x04\x81\x16\x92\x82\x01\x92\x90\x92R\x92P\x85\x16\x10\x15a\x07\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: index too recent\0\0`d\x82\x01R`\x84\x01a\x04\x94V[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x08\x02WP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x08\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: not latest apk upd`d\x82\x01Rbate`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x04\x94V[Q\x94\x93PPPPV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x08\xA2W_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`@\x81\x90\x1B\x92Pc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x92P`\x01`\xE0\x1B\x90\x91\x04\x16\x83V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x91\x82\x01T\x81\x85\x01R\x94\x84R\x90\x91R\x81 T\x90\x91\x90\x80a\t\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry.getRegisteredPubk`D\x82\x01R\x7Fey: operator is not registered\0\0`d\x82\x01R`\x84\x01a\x04\x94V[\x90\x94\x90\x93P\x91PPV[_3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x94\x90a\x1B\xA1V[_a\n\x10a\t\xFA6\x86\x90\x03\x86\x01`@\x87\x01a\x1CtV[\x80Q_\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x03a\n\x96W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R_Q` a\x1E\x18_9_Q\x90_R`D\x82\x01R\x7FKey: cannot register zero pubkey`d\x82\x01R`\x84\x01a\x04\x94V[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x0B\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R_Q` a\x1E\x18_9_Q\x90_R`D\x82\x01R\x7FKey: operator already registered`d\x82\x01Rf pubkey`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x04\x94V[_\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R_Q` a\x1E\x18_9_Q\x90_R`D\x82\x01R\x7FKey: public key already register`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x04\x94V[`@\x80Q_\x91\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x91a\x0B\xF8\x91\x885\x91` \x80\x8B\x015\x92\x8B\x015\x91``\x8C\x015\x91`\x80\x8D\x01\x91`\xC0\x8E\x01\x91\x8D5\x91\x8E\x82\x015\x91\x01a\x1C\xA5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x0C\x1A\x91\x90a\x1C\xE7V[\x90Pa\x0C\xB3a\x0CSa\x0C>\x83a\x0C86\x8A\x90\x03\x8A\x01`@\x8B\x01a\x1CtV[\x90a\x12\xCBV[a\x0CM6\x89\x90\x03\x89\x01\x89a\x1CtV[\x90a\x13ZV[a\x0C[a\x13\xEDV[a\x0C\x9Ca\x0C\x8D\x85a\x0C8`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[a\x0CM6\x8A\x90\x03\x8A\x01\x8Aa\x1CtV[a\x0C\xAE6\x8A\x90\x03\x8A\x01`\x80\x8B\x01a\x1DHV[a\x14\xADV[a\rMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R_Q` a\x1E\x18_9_Q\x90_R`D\x82\x01R\x7FKey: either the G1 signature is `d\x82\x01R\x7Fwrong, or G1 and G2 private key `\x84\x82\x01Rk\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x01a\x04\x94V[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x89\x82\x01\x805\x82U``\x8B\x015`\x01\x92\x83\x01U\x90\x83R\x81\x84 \x87\x90U\x86\x84R`\x02\x90\x92R\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90U\x90Q\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x91a\r\xD0\x91`\x80\x8A\x01\x90a\x1D\x87V[`@Q\x80\x91\x03\x90\xA2P\x94\x93PPPPV[``_\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xFDWa\r\xFDa\x18\xE2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E&W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\x0F\xDFW_\x86\x86\x83\x81\x81\x10a\x0EFWa\x0EFa\x1C`V[\x91\x90\x91\x015`\xF8\x1C_\x81\x81R`\x04` R`@\x90 T\x90\x92P\x90P\x80\x15\x80a\x0E\xA6WP`\xFF\x82\x16_\x90\x81R`\x04` R`@\x81 \x80T\x90\x91\x90a\x0E\x8BWa\x0E\x8Ba\x1C`V[_\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x0F3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FBLSApkRegistry.getApkIndicesAtBl`D\x82\x01R\x7FockNumber: blockNumber is before`d\x82\x01Rp the first update`x\x1B`\x84\x82\x01R`\xA4\x01a\x04\x94V[\x80[\x80\x15a\x0F\xD4W`\xFF\x83\x16_\x90\x81R`\x04` R`@\x90 \x87\x90a\x0FY`\x01\x84a\x1D\xC5V[\x81T\x81\x10a\x0FiWa\x0Fia\x1C`V[_\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11a\x0F\xC2Wa\x0F\x91`\x01\x82a\x1D\xC5V[\x85\x85\x81Q\x81\x10a\x0F\xA3Wa\x0F\xA3a\x1C`V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x0F\xD4V[\x80a\x0F\xCC\x81a\x1D\xD8V[\x91PPa\x0F5V[PPP`\x01\x01a\x0E+V[P\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x100W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x94\x90a\x1B\xA1V[_a\x10:\x83a\x08\xD0V[P\x90Pa\x10O\x82a\x10J\x83a\x17\x0BV[a\x10\x8FV[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x06B\x85`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_[\x83Q\x81\x10\x15a\x12\xC5W_\x84\x82\x81Q\x81\x10a\x10\xC0Wa\x10\xC0a\x1C`V[\x01` \x90\x81\x01Q`\xF8\x1C_\x81\x81R`\x04\x90\x92R`@\x82 T\x90\x92P\x90\x81\x90\x03a\x11QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBLSApkRegistry._processQuorumApk`D\x82\x01R\x7FUpdate: quorum does not exist\0\0\0`d\x82\x01R`\x84\x01a\x04\x94V[`\xFF\x82\x16_\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01Ra\x11\x84\x90\x86a\x13ZV[`\xFF\x83\x16_\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x85Q\x80\x82U\x86\x84\x01\x80Q`\x01\x93\x84\x01U\x90\x85RQ\x83R\x81\x84 \x94\x84R`\x04\x90\x92R\x82 \x93\x97P\x91\x92\x90\x91a\x11\xCC\x90\x85a\x1D\xC5V[\x81T\x81\x10a\x11\xDCWa\x11\xDCa\x1C`V[_\x91\x82R` \x90\x91 \x01\x80T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16`\x01`\xC0\x1B\x90\x92\x04\x16\x03a\x12\x1AW\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`@\x83\x90\x1C\x17\x81Ua\x12\xB5V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01`\xE0\x1B\x81\x81\x02`\x01`\x01`\xE0\x1B\x03\x94\x85\x16\x17\x85U`\xFF\x88\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x8B\x16\x81R\x80\x84\x01\x96\x87R\x80\x83\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x93Q\x93\x01\x80T\x95Q\x92Q\x87\x16\x90\x94\x02\x91\x90\x95\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x94\x90\x94\x16\x91\x90\x94\x1C\x17\x91\x90\x91\x17\x90\x92\x16\x17\x90U[PP`\x01\x90\x92\x01\x91Pa\x10\xA4\x90PV[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x12\xE6a\x17\xC7V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R_\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x13\x14W\xFE[P\x80a\x13RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04\x94V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x13ua\x17\xE5V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R_\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x13\xAFW\xFE[P\x80a\x13RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04\x94V[a\x13\xF5a\x18\x03V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R_\x91a\x14\xDBa\x18(V[_[`\x02\x81\x10\x15a\x16\x92W_a\x14\xF2\x82`\x06a\x1D\xEDV[\x90P\x84\x82`\x02\x81\x10a\x15\x06Wa\x15\x06a\x1C`V[` \x02\x01QQ\x83a\x15\x17\x83_a\x1E\x04V[`\x0C\x81\x10a\x15'Wa\x15'a\x1C`V[` \x02\x01R\x84\x82`\x02\x81\x10a\x15>Wa\x15>a\x1C`V[` \x02\x01Q` \x01Q\x83\x82`\x01a\x15U\x91\x90a\x1E\x04V[`\x0C\x81\x10a\x15eWa\x15ea\x1C`V[` \x02\x01R\x83\x82`\x02\x81\x10a\x15|Wa\x15|a\x1C`V[` \x02\x01QQQ\x83a\x15\x8F\x83`\x02a\x1E\x04V[`\x0C\x81\x10a\x15\x9FWa\x15\x9Fa\x1C`V[` \x02\x01R\x83\x82`\x02\x81\x10a\x15\xB6Wa\x15\xB6a\x1C`V[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x15\xCF\x83`\x03a\x1E\x04V[`\x0C\x81\x10a\x15\xDFWa\x15\xDFa\x1C`V[` \x02\x01R\x83\x82`\x02\x81\x10a\x15\xF6Wa\x15\xF6a\x1C`V[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x16\x10Wa\x16\x10a\x1C`V[` \x02\x01Q\x83a\x16!\x83`\x04a\x1E\x04V[`\x0C\x81\x10a\x161Wa\x161a\x1C`V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16HWa\x16Ha\x1C`V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x16cWa\x16ca\x1C`V[` \x02\x01Q\x83a\x16t\x83`\x05a\x1E\x04V[`\x0C\x81\x10a\x16\x84Wa\x16\x84a\x1C`V[` \x02\x01RP`\x01\x01a\x14\xDDV[Pa\x16\x9Ba\x18GV[_` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x16\xB5W\xFE[P\x80a\x16\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x04\x94V[PQ\x15\x15\x98\x97PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x17/WP` \x82\x01Q\x15[\x15a\x17LWPP`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x17\x90\x91\x90a\x1C\xE7V[a\x17\xBA\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1D\xC5V[\x90R\x92\x91PPV[\x91\x90PV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x18\x16a\x18eV[\x81R` \x01a\x18#a\x18eV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x17\xC2W__\xFD[_` \x82\x84\x03\x12\x15a\x18\xA9W__\xFD[a\x18\xB2\x82a\x18\x83V[\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x17\xC2W__\xFD[_` \x82\x84\x03\x12\x15a\x18\xD9W__\xFD[a\x18\xB2\x82a\x18\xB9V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\x19Wa\x19\x19a\x18\xE2V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19HWa\x19Ha\x18\xE2V[`@R\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x19aW__\xFD[a\x19j\x83a\x18\x83V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\x85W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x19\x95W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xAFWa\x19\xAFa\x18\xE2V[a\x19\xC2`\x1F\x82\x01`\x1F\x19\x16` \x01a\x19\x1FV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a\x19\xD6W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x1A\x05W__\xFD[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x06\xEAV[__`@\x83\x85\x03\x12\x15a\x1A4W__\xFD[a\x1A=\x83a\x18\xB9V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\x1A]W__\xFD[a\x1Af\x84a\x18\xB9V[\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A~W__\xFD[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[___\x83\x85\x03a\x01`\x81\x12\x15a\x1A\xA3W__\xFD[a\x1A\xAC\x85a\x18\x83V[\x93Pa\x01\0`\x1F\x19\x82\x01\x12\x15a\x1A\xC0W__\xFD[` \x85\x01\x92P`@a\x01\x1F\x19\x82\x01\x12\x15a\x1A\xD8W__\xFD[Pa\x01 \x84\x01\x90P\x92P\x92P\x92V[___`@\x84\x86\x03\x12\x15a\x1A\xF9W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\x0FW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x1B\x1FW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B5W__\xFD[\x86` \x82\x84\x01\x01\x11\x15a\x1BFW__\xFD[` \x91\x82\x01\x97\x90\x96P\x94\x015\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x1B\x96W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1BrV[P\x90\x95\x94PPPPPV[` \x80\x82R`N\x90\x82\x01R\x7FBLSApkRegistry.onlyRegistryCoord`@\x82\x01R\x7Finator: caller is not the regist``\x82\x01Rm9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x91\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R_\x82Q\x80``\x84\x01R\x80` \x85\x01`\x80\x85\x01^_`\x80\x82\x85\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x94\x93PPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`@\x82\x84\x03\x12\x80\x15a\x1C\x85W__\xFD[Pa\x1C\x8Ea\x18\xF6V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01R`@\x85`\x80\x83\x017`@\x84`\xC0\x83\x017a\x01\0\x81\x01\x92\x90\x92Ra\x01 \x82\x01Ra\x01@\x01\x96\x95PPPPPPV[_\x82a\x1D\x01WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[_\x82`\x1F\x83\x01\x12a\x1D\x15W__\xFD[a\x1D\x1Da\x18\xF6V[\x80`@\x84\x01\x85\x81\x11\x15a\x1D.W__\xFD[\x84[\x81\x81\x10\x15a\x1B\x96W\x805\x84R` \x93\x84\x01\x93\x01a\x1D0V[_`\x80\x82\x84\x03\x12\x80\x15a\x1DYW__\xFD[Pa\x1Dba\x18\xF6V[a\x1Dl\x84\x84a\x1D\x06V[\x81Ra\x1D{\x84`@\x85\x01a\x1D\x06V[` \x82\x01R\x93\x92PPPV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`\xC0\x81\x01`@\x83\x81\x84\x017`@\x80\x84\x01`\x80\x84\x017\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\xEAWa\x06\xEAa\x1D\xB1V[_\x81a\x1D\xE6Wa\x1D\xE6a\x1D\xB1V[P_\x19\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xEAWa\x06\xEAa\x1D\xB1V[\x80\x82\x01\x80\x82\x11\x15a\x06\xEAWa\x06\xEAa\x1D\xB1V\xFEBLSApkRegistry.registerBLSPublic\xA2dipfsX\"\x12 \xC1\x98d\xF29\xC7X\xEDI\xDA\xF0e\x94\x19b \x85\x84\xAE\xF0P\xE2\xA2\x8B\x0Ca\xF4\xF4\xB2\xA1Y\xDDdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610110575f3560e01c80636d14a9871161009e578063bf79ce581161006e578063bf79ce58146103bf578063d5254a8c146103d2578063de29fac0146103f2578063e8bb9ae614610411578063f4e24fe514610439575f5ffd5b80636d14a987146102fe5780637916cea6146103255780637ff81a8714610366578063a3db80e214610399575f5ffd5b80633fb27952116100e45780633fb27952146101d657806347b314e8146101e95780635f61a88414610229578063605747d51461028357806368bccaac146102d1575f5ffd5b8062a1f4cb1461011457806313542a4e1461015457806326d941f21461018a578063377ed99d1461019f575b5f5ffd5b61013a610122366004611899565b60036020525f90815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b61017c610162366004611899565b6001600160a01b03165f9081526001602052604090205490565b60405190815260200161014b565b61019d6101983660046118c9565b61044c565b005b6101c16101ad3660046118c9565b60ff165f9081526004602052604090205490565b60405163ffffffff909116815260200161014b565b61019d6101e4366004611950565b6105a3565b6102116101f73660046119f5565b5f908152600260205260409020546001600160a01b031690565b6040516001600160a01b03909116815260200161014b565b6102766102373660046118c9565b604080518082019091525f80825260208201525060ff165f90815260056020908152604091829020825180840190935280548352600101549082015290565b60405161014b9190611a0c565b610296610291366004611a23565b61065f565b60408051825167ffffffffffffffff1916815260208084015163ffffffff90811691830191909152928201519092169082015260600161014b565b6102e46102df366004611a4b565b6106f0565b60405167ffffffffffffffff19909116815260200161014b565b6102117f000000000000000000000000000000000000000000000000000000000000000081565b610338610333366004611a23565b610889565b6040805167ffffffffffffffff19909416845263ffffffff928316602085015291169082015260600161014b565b610379610374366004611899565b6108d0565b60408051835181526020938401519381019390935282015260600161014b565b61013a6103a73660046118c9565b60056020525f90815260409020805460019091015482565b61017c6103cd366004611a8f565b61099b565b6103e56103e0366004611ae7565b610de1565b60405161014b9190611b59565b61017c610400366004611899565b60016020525f908152604090205481565b61021161041f3660046119f5565b60026020525f90815260409020546001600160a01b031681565b61019d610447366004611950565b610fe8565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461049d5760405162461bcd60e51b815260040161049490611ba1565b60405180910390fd5b60ff81165f908152600460205260409020541561051b5760405162461bcd60e51b815260206004820152603660248201527f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a6044820152752071756f72756d20616c72656164792065786973747360501b6064820152608401610494565b60ff165f908152600460209081526040808320815160608101835284815263ffffffff4381168286019081528285018781528454600181018655948852959096209151919092018054955194518316600160e01b026001600160e01b0395909316600160c01b026001600160e01b03199096169190931c179390931791909116919091179055565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146105eb5760405162461bcd60e51b815260040161049490611ba1565b5f6105f5836108d0565b509050610602828261108f565b7f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e83610642856001600160a01b03165f9081526001602052604090205490565b8460405161065293929190611c15565b60405180910390a1505050565b604080516060810182525f808252602080830182905282840182905260ff86168252600490529190912080548390811061069b5761069b611c60565b5f91825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b8204811694830194909452600160e01b90049092169082015290505b92915050565b60ff83165f90815260046020526040812080548291908490811061071657610716611c60565b5f91825260209182902060408051606081018252919092015467ffffffffffffffff1981841b16825263ffffffff600160c01b82048116948301859052600160e01b9091048116928201929092529250851610156107dc5760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a20696e64657820746f6f20726563656e7400006064820152608401610494565b604081015163ffffffff1615806108025750806040015163ffffffff168463ffffffff16105b6108805760405162461bcd60e51b815260206004820152604360248201527f424c5341706b52656769737472792e5f76616c696461746541706b486173684160448201527f74426c6f636b4e756d6265723a206e6f74206c61746573742061706b2075706460648201526261746560e81b608482015260a401610494565b51949350505050565b6004602052815f5260405f2081815481106108a2575f80fd5b5f91825260209091200154604081901b925063ffffffff600160c01b820481169250600160e01b9091041683565b604080518082019091525f80825260208201526001600160a01b0382165f818152600360209081526040808320815180830183528154815260019182015481850152948452909152812054909190806109915760405162461bcd60e51b815260206004820152603e60248201527f424c5341706b52656769737472792e676574526567697374657265645075626b60448201527f65793a206f70657261746f72206973206e6f74207265676973746572656400006064820152608401610494565b9094909350915050565b5f336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146109e45760405162461bcd60e51b815260040161049490611ba1565b5f610a106109fa36869003860160408701611c74565b80515f9081526020918201519091526040902090565b90507fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb58103610a96576040805162461bcd60e51b81526020600482015260248101919091525f516020611e185f395f51905f5260448201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b65796064820152608401610494565b6001600160a01b0385165f9081526001602052604090205415610b1e5760405162461bcd60e51b815260206004820152604760248201525f516020611e185f395f51905f5260448201527f4b65793a206f70657261746f7220616c72656164792072656769737465726564606482015266207075626b657960c81b608482015260a401610494565b5f818152600260205260409020546001600160a01b031615610ba05760405162461bcd60e51b815260206004820152604260248201525f516020611e185f395f51905f5260448201527f4b65793a207075626c6963206b657920616c7265616479207265676973746572606482015261195960f21b608482015260a401610494565b604080515f917f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000191610bf8918835916020808b0135928b01359160608c01359160808d019160c08e01918d35918e8201359101611ca5565b604051602081830303815290604052805190602001205f1c610c1a9190611ce7565b9050610cb3610c53610c3e83610c38368a90038a0160408b01611c74565b906112cb565b610c4d36899003890189611c74565b9061135a565b610c5b6113ed565b610c9c610c8d85610c386040805180820182525f80825260209182015281518083019092526001825260029082015290565b610c4d368a90038a018a611c74565b610cae368a90038a0160808b01611d48565b6114ad565b610d4d5760405162461bcd60e51b815260206004820152606c60248201525f516020611e185f395f51905f5260448201527f4b65793a2065697468657220746865204731207369676e61747572652069732060648201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060848201526b0c8de40dcdee840dac2e8c6d60a31b60a482015260c401610494565b6001600160a01b0386165f8181526003602090815260408083208982018035825560608b013560019283015590835281842087905586845260029092529182902080546001600160a01b0319168417905590517fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba382804191610dd09160808a0190611d87565b60405180910390a250949350505050565b60605f8367ffffffffffffffff811115610dfd57610dfd6118e2565b604051908082528060200260200182016040528015610e26578160200160208202803683370190505b5090505f5b84811015610fdf575f868683818110610e4657610e46611c60565b919091013560f81c5f818152600460205260409020549092509050801580610ea6575060ff82165f9081526004602052604081208054909190610e8b57610e8b611c60565b5f91825260209091200154600160c01b900463ffffffff1686105b15610f335760405162461bcd60e51b815260206004820152605160248201527f424c5341706b52656769737472792e67657441706b496e64696365734174426c60448201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f7265606482015270207468652066697273742075706461746560781b608482015260a401610494565b805b8015610fd45760ff83165f9081526004602052604090208790610f59600184611dc5565b81548110610f6957610f69611c60565b5f91825260209091200154600160c01b900463ffffffff1611610fc257610f91600182611dc5565b858581518110610fa357610fa3611c60565b602002602001019063ffffffff16908163ffffffff1681525050610fd4565b80610fcc81611dd8565b915050610f35565b505050600101610e2b565b50949350505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146110305760405162461bcd60e51b815260040161049490611ba1565b5f61103a836108d0565b50905061104f8261104a8361170b565b61108f565b7ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e83610642856001600160a01b03165f9081526001602052604090205490565b604080518082019091525f80825260208201525f5b83518110156112c5575f8482815181106110c0576110c0611c60565b0160209081015160f81c5f81815260049092526040822054909250908190036111515760405162461bcd60e51b815260206004820152603d60248201527f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b60448201527f5570646174653a2071756f72756d20646f6573206e6f742065786973740000006064820152608401610494565b60ff82165f908152600560209081526040918290208251808401909352805483526001015490820152611184908661135a565b60ff83165f8181526005602090815260408083208551808255868401805160019384015590855251835281842094845260049092528220939750919290916111cc9085611dc5565b815481106111dc576111dc611c60565b5f918252602090912001805490915063ffffffff438116600160c01b909204160361121a5780546001600160c01b031916604083901c1781556112b5565b805463ffffffff438116600160e01b8181026001600160e01b0394851617855560ff88165f908152600460209081526040808320815160608101835267ffffffffffffffff198b16815280840196875280830185815282546001810184559286529390942093519301805495519251871690940291909516600160c01b026001600160e01b0319949094169190941c17919091179092161790555b5050600190920191506110a49050565b50505050565b604080518082019091525f80825260208201526112e66117c7565b835181526020808501519082015260408082018490525f908360608460076107d05a03fa9050808061131457fe5b50806113525760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610494565b505092915050565b604080518082019091525f80825260208201526113756117e5565b835181526020808501518183015283516040808401919091529084015160608301525f908360808460066107d05a03fa905080806113af57fe5b50806113525760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610494565b6113f5611803565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820182528581526020808201859052825180840190935285835282018390525f916114db611828565b5f5b6002811015611692575f6114f2826006611ded565b905084826002811061150657611506611c60565b60200201515183611517835f611e04565b600c811061152757611527611c60565b602002015284826002811061153e5761153e611c60565b602002015160200151838260016115559190611e04565b600c811061156557611565611c60565b602002015283826002811061157c5761157c611c60565b602002015151518361158f836002611e04565b600c811061159f5761159f611c60565b60200201528382600281106115b6576115b6611c60565b60200201515160016020020151836115cf836003611e04565b600c81106115df576115df611c60565b60200201528382600281106115f6576115f6611c60565b6020020151602001515f6002811061161057611610611c60565b602002015183611621836004611e04565b600c811061163157611631611c60565b602002015283826002811061164857611648611c60565b60200201516020015160016002811061166357611663611c60565b602002015183611674836005611e04565b600c811061168457611684611c60565b6020020152506001016114dd565b5061169b611847565b5f6020826101808560086107d05a03fa905080806116b557fe5b50806116fb5760405162461bcd60e51b81526020600482015260156024820152741c185a5c9a5b99cb5bdc18dbd9194b59985a5b1959605a1b6044820152606401610494565b5051151598975050505050505050565b604080518082019091525f8082526020820152815115801561172f57506020820151155b1561174c575050604080518082019091525f808252602082015290565b6040518060400160405280835f015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4784602001516117909190611ce7565b6117ba907f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611dc5565b905292915050565b919050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280611816611865565b8152602001611823611865565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b80356001600160a01b03811681146117c2575f5ffd5b5f602082840312156118a9575f5ffd5b6118b282611883565b9392505050565b803560ff811681146117c2575f5ffd5b5f602082840312156118d9575f5ffd5b6118b2826118b9565b634e487b7160e01b5f52604160045260245ffd5b6040805190810167ffffffffffffffff81118282101715611919576119196118e2565b60405290565b604051601f8201601f1916810167ffffffffffffffff81118282101715611948576119486118e2565b604052919050565b5f5f60408385031215611961575f5ffd5b61196a83611883565b9150602083013567ffffffffffffffff811115611985575f5ffd5b8301601f81018513611995575f5ffd5b803567ffffffffffffffff8111156119af576119af6118e2565b6119c2601f8201601f191660200161191f565b8181528660208385010111156119d6575f5ffd5b816020840160208301375f602083830101528093505050509250929050565b5f60208284031215611a05575f5ffd5b5035919050565b8151815260208083015190820152604081016106ea565b5f5f60408385031215611a34575f5ffd5b611a3d836118b9565b946020939093013593505050565b5f5f5f60608486031215611a5d575f5ffd5b611a66846118b9565b9250602084013563ffffffff81168114611a7e575f5ffd5b929592945050506040919091013590565b5f5f5f838503610160811215611aa3575f5ffd5b611aac85611883565b9350610100601f1982011215611ac0575f5ffd5b602085019250604061011f1982011215611ad8575f5ffd5b50610120840190509250925092565b5f5f5f60408486031215611af9575f5ffd5b833567ffffffffffffffff811115611b0f575f5ffd5b8401601f81018613611b1f575f5ffd5b803567ffffffffffffffff811115611b35575f5ffd5b866020828401011115611b46575f5ffd5b6020918201979096509401359392505050565b602080825282518282018190525f918401906040840190835b81811015611b9657835163ffffffff16835260209384019390920191600101611b72565b509095945050505050565b6020808252604e908201527f424c5341706b52656769737472792e6f6e6c795265676973747279436f6f726460408201527f696e61746f723a2063616c6c6572206973206e6f74207468652072656769737460608201526d393c9031b7b7b93234b730ba37b960911b608082015260a00190565b60018060a01b0384168152826020820152606060408201525f82518060608401528060208501608085015e5f608082850101526080601f19601f830116840101915050949350505050565b634e487b7160e01b5f52603260045260245ffd5b5f6040828403128015611c85575f5ffd5b50611c8e6118f6565b823581526020928301359281019290925250919050565b888152876020820152866040820152856060820152604085608083013760408460c0830137610100810192909252610120820152610140019695505050505050565b5f82611d0157634e487b7160e01b5f52601260045260245ffd5b500690565b5f82601f830112611d15575f5ffd5b611d1d6118f6565b806040840185811115611d2e575f5ffd5b845b81811015611b96578035845260209384019301611d30565b5f6080828403128015611d59575f5ffd5b50611d626118f6565b611d6c8484611d06565b8152611d7b8460408501611d06565b60208201529392505050565b823581526020808401359082015260c0810160408381840137604080840160808401379392505050565b634e487b7160e01b5f52601160045260245ffd5b818103818111156106ea576106ea611db1565b5f81611de657611de6611db1565b505f190190565b80820281158282048414176106ea576106ea611db1565b808201808211156106ea576106ea611db156fe424c5341706b52656769737472792e7265676973746572424c535075626c6963a2646970667358221220c19864f239c758ed49daf065941962208584aef050e2a28b0c61f4f4b2a159dd64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x10W_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\x9EW\x80c\xBFy\xCEX\x11a\0nW\x80c\xBFy\xCEX\x14a\x03\xBFW\x80c\xD5%J\x8C\x14a\x03\xD2W\x80c\xDE)\xFA\xC0\x14a\x03\xF2W\x80c\xE8\xBB\x9A\xE6\x14a\x04\x11W\x80c\xF4\xE2O\xE5\x14a\x049W__\xFD[\x80cm\x14\xA9\x87\x14a\x02\xFEW\x80cy\x16\xCE\xA6\x14a\x03%W\x80c\x7F\xF8\x1A\x87\x14a\x03fW\x80c\xA3\xDB\x80\xE2\x14a\x03\x99W__\xFD[\x80c?\xB2yR\x11a\0\xE4W\x80c?\xB2yR\x14a\x01\xD6W\x80cG\xB3\x14\xE8\x14a\x01\xE9W\x80c_a\xA8\x84\x14a\x02)W\x80c`WG\xD5\x14a\x02\x83W\x80ch\xBC\xCA\xAC\x14a\x02\xD1W__\xFD[\x80b\xA1\xF4\xCB\x14a\x01\x14W\x80c\x13T*N\x14a\x01TW\x80c&\xD9A\xF2\x14a\x01\x8AW\x80c7~\xD9\x9D\x14a\x01\x9FW[__\xFD[a\x01:a\x01\"6`\x04a\x18\x99V[`\x03` R_\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01|a\x01b6`\x04a\x18\x99V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01KV[a\x01\x9Da\x01\x986`\x04a\x18\xC9V[a\x04LV[\0[a\x01\xC1a\x01\xAD6`\x04a\x18\xC9V[`\xFF\x16_\x90\x81R`\x04` R`@\x90 T\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01KV[a\x01\x9Da\x01\xE46`\x04a\x19PV[a\x05\xA3V[a\x02\x11a\x01\xF76`\x04a\x19\xF5V[_\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01KV[a\x02va\x0276`\x04a\x18\xC9V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`\xFF\x16_\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`@Qa\x01K\x91\x90a\x1A\x0CV[a\x02\x96a\x02\x916`\x04a\x1A#V[a\x06_V[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x01KV[a\x02\xE4a\x02\xDF6`\x04a\x1AKV[a\x06\xF0V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R` \x01a\x01KV[a\x02\x11\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x038a\x0336`\x04a\x1A#V[a\x08\x89V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01KV[a\x03ya\x03t6`\x04a\x18\x99V[a\x08\xD0V[`@\x80Q\x83Q\x81R` \x93\x84\x01Q\x93\x81\x01\x93\x90\x93R\x82\x01R``\x01a\x01KV[a\x01:a\x03\xA76`\x04a\x18\xC9V[`\x05` R_\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[a\x01|a\x03\xCD6`\x04a\x1A\x8FV[a\t\x9BV[a\x03\xE5a\x03\xE06`\x04a\x1A\xE7V[a\r\xE1V[`@Qa\x01K\x91\x90a\x1BYV[a\x01|a\x04\x006`\x04a\x18\x99V[`\x01` R_\x90\x81R`@\x90 T\x81V[a\x02\x11a\x04\x1F6`\x04a\x19\xF5V[`\x02` R_\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x9Da\x04G6`\x04a\x19PV[a\x0F\xE8V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x94\x90a\x1B\xA1V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16_\x90\x81R`\x04` R`@\x90 T\x15a\x05\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBLSApkRegistry.initializeQuorum:`D\x82\x01Ru quorum already exists`P\x1B`d\x82\x01R`\x84\x01a\x04\x94V[`\xFF\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x84\x81Rc\xFF\xFF\xFF\xFFC\x81\x16\x82\x86\x01\x90\x81R\x82\x85\x01\x87\x81R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x94Q\x83\x16`\x01`\xE0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x95\x90\x93\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x96\x16\x91\x90\x93\x1C\x17\x93\x90\x93\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x94\x90a\x1B\xA1V[_a\x05\xF5\x83a\x08\xD0V[P\x90Pa\x06\x02\x82\x82a\x10\x8FV[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06B\x85`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01` R`@\x90 T\x90V[\x84`@Qa\x06R\x93\x92\x91\x90a\x1C\x15V[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x04\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x06\x9BWa\x06\x9Ba\x1C`V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\xFF\x83\x16_\x90\x81R`\x04` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x07\x16Wa\x07\x16a\x1C`V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x85\x90R`\x01`\xE0\x1B\x90\x91\x04\x81\x16\x92\x82\x01\x92\x90\x92R\x92P\x85\x16\x10\x15a\x07\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: index too recent\0\0`d\x82\x01R`\x84\x01a\x04\x94V[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x08\x02WP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x08\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: not latest apk upd`d\x82\x01Rbate`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x04\x94V[Q\x94\x93PPPPV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x08\xA2W_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`@\x81\x90\x1B\x92Pc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x92P`\x01`\xE0\x1B\x90\x91\x04\x16\x83V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x91\x82\x01T\x81\x85\x01R\x94\x84R\x90\x91R\x81 T\x90\x91\x90\x80a\t\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry.getRegisteredPubk`D\x82\x01R\x7Fey: operator is not registered\0\0`d\x82\x01R`\x84\x01a\x04\x94V[\x90\x94\x90\x93P\x91PPV[_3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x94\x90a\x1B\xA1V[_a\n\x10a\t\xFA6\x86\x90\x03\x86\x01`@\x87\x01a\x1CtV[\x80Q_\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x03a\n\x96W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R_Q` a\x1E\x18_9_Q\x90_R`D\x82\x01R\x7FKey: cannot register zero pubkey`d\x82\x01R`\x84\x01a\x04\x94V[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x0B\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R_Q` a\x1E\x18_9_Q\x90_R`D\x82\x01R\x7FKey: operator already registered`d\x82\x01Rf pubkey`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x04\x94V[_\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R_Q` a\x1E\x18_9_Q\x90_R`D\x82\x01R\x7FKey: public key already register`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x04\x94V[`@\x80Q_\x91\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x91a\x0B\xF8\x91\x885\x91` \x80\x8B\x015\x92\x8B\x015\x91``\x8C\x015\x91`\x80\x8D\x01\x91`\xC0\x8E\x01\x91\x8D5\x91\x8E\x82\x015\x91\x01a\x1C\xA5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x0C\x1A\x91\x90a\x1C\xE7V[\x90Pa\x0C\xB3a\x0CSa\x0C>\x83a\x0C86\x8A\x90\x03\x8A\x01`@\x8B\x01a\x1CtV[\x90a\x12\xCBV[a\x0CM6\x89\x90\x03\x89\x01\x89a\x1CtV[\x90a\x13ZV[a\x0C[a\x13\xEDV[a\x0C\x9Ca\x0C\x8D\x85a\x0C8`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[a\x0CM6\x8A\x90\x03\x8A\x01\x8Aa\x1CtV[a\x0C\xAE6\x8A\x90\x03\x8A\x01`\x80\x8B\x01a\x1DHV[a\x14\xADV[a\rMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R_Q` a\x1E\x18_9_Q\x90_R`D\x82\x01R\x7FKey: either the G1 signature is `d\x82\x01R\x7Fwrong, or G1 and G2 private key `\x84\x82\x01Rk\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x01a\x04\x94V[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x89\x82\x01\x805\x82U``\x8B\x015`\x01\x92\x83\x01U\x90\x83R\x81\x84 \x87\x90U\x86\x84R`\x02\x90\x92R\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90U\x90Q\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x91a\r\xD0\x91`\x80\x8A\x01\x90a\x1D\x87V[`@Q\x80\x91\x03\x90\xA2P\x94\x93PPPPV[``_\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xFDWa\r\xFDa\x18\xE2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E&W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\x0F\xDFW_\x86\x86\x83\x81\x81\x10a\x0EFWa\x0EFa\x1C`V[\x91\x90\x91\x015`\xF8\x1C_\x81\x81R`\x04` R`@\x90 T\x90\x92P\x90P\x80\x15\x80a\x0E\xA6WP`\xFF\x82\x16_\x90\x81R`\x04` R`@\x81 \x80T\x90\x91\x90a\x0E\x8BWa\x0E\x8Ba\x1C`V[_\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x0F3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FBLSApkRegistry.getApkIndicesAtBl`D\x82\x01R\x7FockNumber: blockNumber is before`d\x82\x01Rp the first update`x\x1B`\x84\x82\x01R`\xA4\x01a\x04\x94V[\x80[\x80\x15a\x0F\xD4W`\xFF\x83\x16_\x90\x81R`\x04` R`@\x90 \x87\x90a\x0FY`\x01\x84a\x1D\xC5V[\x81T\x81\x10a\x0FiWa\x0Fia\x1C`V[_\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11a\x0F\xC2Wa\x0F\x91`\x01\x82a\x1D\xC5V[\x85\x85\x81Q\x81\x10a\x0F\xA3Wa\x0F\xA3a\x1C`V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x0F\xD4V[\x80a\x0F\xCC\x81a\x1D\xD8V[\x91PPa\x0F5V[PPP`\x01\x01a\x0E+V[P\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x100W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\x94\x90a\x1B\xA1V[_a\x10:\x83a\x08\xD0V[P\x90Pa\x10O\x82a\x10J\x83a\x17\x0BV[a\x10\x8FV[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x06B\x85`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_[\x83Q\x81\x10\x15a\x12\xC5W_\x84\x82\x81Q\x81\x10a\x10\xC0Wa\x10\xC0a\x1C`V[\x01` \x90\x81\x01Q`\xF8\x1C_\x81\x81R`\x04\x90\x92R`@\x82 T\x90\x92P\x90\x81\x90\x03a\x11QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBLSApkRegistry._processQuorumApk`D\x82\x01R\x7FUpdate: quorum does not exist\0\0\0`d\x82\x01R`\x84\x01a\x04\x94V[`\xFF\x82\x16_\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01Ra\x11\x84\x90\x86a\x13ZV[`\xFF\x83\x16_\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x85Q\x80\x82U\x86\x84\x01\x80Q`\x01\x93\x84\x01U\x90\x85RQ\x83R\x81\x84 \x94\x84R`\x04\x90\x92R\x82 \x93\x97P\x91\x92\x90\x91a\x11\xCC\x90\x85a\x1D\xC5V[\x81T\x81\x10a\x11\xDCWa\x11\xDCa\x1C`V[_\x91\x82R` \x90\x91 \x01\x80T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16`\x01`\xC0\x1B\x90\x92\x04\x16\x03a\x12\x1AW\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`@\x83\x90\x1C\x17\x81Ua\x12\xB5V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01`\xE0\x1B\x81\x81\x02`\x01`\x01`\xE0\x1B\x03\x94\x85\x16\x17\x85U`\xFF\x88\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x8B\x16\x81R\x80\x84\x01\x96\x87R\x80\x83\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x93Q\x93\x01\x80T\x95Q\x92Q\x87\x16\x90\x94\x02\x91\x90\x95\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x94\x90\x94\x16\x91\x90\x94\x1C\x17\x91\x90\x91\x17\x90\x92\x16\x17\x90U[PP`\x01\x90\x92\x01\x91Pa\x10\xA4\x90PV[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x12\xE6a\x17\xC7V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R_\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x13\x14W\xFE[P\x80a\x13RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04\x94V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x13ua\x17\xE5V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R_\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x13\xAFW\xFE[P\x80a\x13RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04\x94V[a\x13\xF5a\x18\x03V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R_\x91a\x14\xDBa\x18(V[_[`\x02\x81\x10\x15a\x16\x92W_a\x14\xF2\x82`\x06a\x1D\xEDV[\x90P\x84\x82`\x02\x81\x10a\x15\x06Wa\x15\x06a\x1C`V[` \x02\x01QQ\x83a\x15\x17\x83_a\x1E\x04V[`\x0C\x81\x10a\x15'Wa\x15'a\x1C`V[` \x02\x01R\x84\x82`\x02\x81\x10a\x15>Wa\x15>a\x1C`V[` \x02\x01Q` \x01Q\x83\x82`\x01a\x15U\x91\x90a\x1E\x04V[`\x0C\x81\x10a\x15eWa\x15ea\x1C`V[` \x02\x01R\x83\x82`\x02\x81\x10a\x15|Wa\x15|a\x1C`V[` \x02\x01QQQ\x83a\x15\x8F\x83`\x02a\x1E\x04V[`\x0C\x81\x10a\x15\x9FWa\x15\x9Fa\x1C`V[` \x02\x01R\x83\x82`\x02\x81\x10a\x15\xB6Wa\x15\xB6a\x1C`V[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x15\xCF\x83`\x03a\x1E\x04V[`\x0C\x81\x10a\x15\xDFWa\x15\xDFa\x1C`V[` \x02\x01R\x83\x82`\x02\x81\x10a\x15\xF6Wa\x15\xF6a\x1C`V[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x16\x10Wa\x16\x10a\x1C`V[` \x02\x01Q\x83a\x16!\x83`\x04a\x1E\x04V[`\x0C\x81\x10a\x161Wa\x161a\x1C`V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16HWa\x16Ha\x1C`V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x16cWa\x16ca\x1C`V[` \x02\x01Q\x83a\x16t\x83`\x05a\x1E\x04V[`\x0C\x81\x10a\x16\x84Wa\x16\x84a\x1C`V[` \x02\x01RP`\x01\x01a\x14\xDDV[Pa\x16\x9Ba\x18GV[_` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x16\xB5W\xFE[P\x80a\x16\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x04\x94V[PQ\x15\x15\x98\x97PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x17/WP` \x82\x01Q\x15[\x15a\x17LWPP`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x17\x90\x91\x90a\x1C\xE7V[a\x17\xBA\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1D\xC5V[\x90R\x92\x91PPV[\x91\x90PV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x18\x16a\x18eV[\x81R` \x01a\x18#a\x18eV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x17\xC2W__\xFD[_` \x82\x84\x03\x12\x15a\x18\xA9W__\xFD[a\x18\xB2\x82a\x18\x83V[\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x17\xC2W__\xFD[_` \x82\x84\x03\x12\x15a\x18\xD9W__\xFD[a\x18\xB2\x82a\x18\xB9V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\x19Wa\x19\x19a\x18\xE2V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19HWa\x19Ha\x18\xE2V[`@R\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x19aW__\xFD[a\x19j\x83a\x18\x83V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\x85W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x19\x95W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xAFWa\x19\xAFa\x18\xE2V[a\x19\xC2`\x1F\x82\x01`\x1F\x19\x16` \x01a\x19\x1FV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a\x19\xD6W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x1A\x05W__\xFD[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x06\xEAV[__`@\x83\x85\x03\x12\x15a\x1A4W__\xFD[a\x1A=\x83a\x18\xB9V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\x1A]W__\xFD[a\x1Af\x84a\x18\xB9V[\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A~W__\xFD[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[___\x83\x85\x03a\x01`\x81\x12\x15a\x1A\xA3W__\xFD[a\x1A\xAC\x85a\x18\x83V[\x93Pa\x01\0`\x1F\x19\x82\x01\x12\x15a\x1A\xC0W__\xFD[` \x85\x01\x92P`@a\x01\x1F\x19\x82\x01\x12\x15a\x1A\xD8W__\xFD[Pa\x01 \x84\x01\x90P\x92P\x92P\x92V[___`@\x84\x86\x03\x12\x15a\x1A\xF9W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\x0FW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x1B\x1FW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B5W__\xFD[\x86` \x82\x84\x01\x01\x11\x15a\x1BFW__\xFD[` \x91\x82\x01\x97\x90\x96P\x94\x015\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x1B\x96W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1BrV[P\x90\x95\x94PPPPPV[` \x80\x82R`N\x90\x82\x01R\x7FBLSApkRegistry.onlyRegistryCoord`@\x82\x01R\x7Finator: caller is not the regist``\x82\x01Rm9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x91\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R_\x82Q\x80``\x84\x01R\x80` \x85\x01`\x80\x85\x01^_`\x80\x82\x85\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x94\x93PPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`@\x82\x84\x03\x12\x80\x15a\x1C\x85W__\xFD[Pa\x1C\x8Ea\x18\xF6V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01R`@\x85`\x80\x83\x017`@\x84`\xC0\x83\x017a\x01\0\x81\x01\x92\x90\x92Ra\x01 \x82\x01Ra\x01@\x01\x96\x95PPPPPPV[_\x82a\x1D\x01WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[_\x82`\x1F\x83\x01\x12a\x1D\x15W__\xFD[a\x1D\x1Da\x18\xF6V[\x80`@\x84\x01\x85\x81\x11\x15a\x1D.W__\xFD[\x84[\x81\x81\x10\x15a\x1B\x96W\x805\x84R` \x93\x84\x01\x93\x01a\x1D0V[_`\x80\x82\x84\x03\x12\x80\x15a\x1DYW__\xFD[Pa\x1Dba\x18\xF6V[a\x1Dl\x84\x84a\x1D\x06V[\x81Ra\x1D{\x84`@\x85\x01a\x1D\x06V[` \x82\x01R\x93\x92PPPV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`\xC0\x81\x01`@\x83\x81\x84\x017`@\x80\x84\x01`\x80\x84\x017\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\xEAWa\x06\xEAa\x1D\xB1V[_\x81a\x1D\xE6Wa\x1D\xE6a\x1D\xB1V[P_\x19\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xEAWa\x06\xEAa\x1D\xB1V[\x80\x82\x01\x80\x82\x11\x15a\x06\xEAWa\x06\xEAa\x1D\xB1V\xFEBLSApkRegistry.registerBLSPublic\xA2dipfsX\"\x12 \xC1\x98d\xF29\xC7X\xEDI\xDA\xF0e\x94\x19b \x85\x84\xAE\xF0P\xE2\xA2\x8B\x0Ca\xF4\xF4\xB2\xA1Y\xDDdsolcC\0\x08\x1B\x003",
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "NewPubkeyRegistration(address,(uint256,uint256),(uint256[2],uint256[2]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                227u8,
                251u8,
                102u8,
                19u8,
                175u8,
                46u8,
                137u8,
                48u8,
                207u8,
                133u8,
                212u8,
                127u8,
                207u8,
                109u8,
                177u8,
                1u8,
                146u8,
                34u8,
                74u8,
                100u8,
                198u8,
                203u8,
                232u8,
                2u8,
                62u8,
                14u8,
                238u8,
                27u8,
                163u8,
                130u8,
                128u8,
                65u8,
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
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(
                        &self.pubkeyG1,
                    ),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(
                        &self.pubkeyG2,
                    ),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorAddedToQuorums(address,bytes32,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                115u8,
                162u8,
                183u8,
                251u8,
                132u8,
                71u8,
                36u8,
                185u8,
                113u8,
                128u8,
                42u8,
                233u8,
                177u8,
                93u8,
                176u8,
                148u8,
                212u8,
                183u8,
                25u8,
                45u8,
                249u8,
                215u8,
                53u8,
                14u8,
                20u8,
                235u8,
                70u8,
                107u8,
                155u8,
                34u8,
                235u8,
                78u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorRemovedFromQuorums(address,bytes32,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                248u8,
                67u8,
                236u8,
                213u8,
                58u8,
                86u8,
                54u8,
                117u8,
                230u8,
                33u8,
                7u8,
                190u8,
                20u8,
                148u8,
                253u8,
                222u8,
                74u8,
                61u8,
                73u8,
                174u8,
                237u8,
                175u8,
                141u8,
                136u8,
                198u8,
                22u8,
                216u8,
                83u8,
                70u8,
                227u8,
                80u8,
                14u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            fn from(
                this: &OperatorRemovedFromQuorums,
            ) -> alloy_sol_types::private::LogData {
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
            type UnderlyingRustTuple<'a> = (
                u8,
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
            impl ::core::convert::From<apkHistoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: apkHistoryCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for apkHistoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<24>,
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
            impl ::core::convert::From<apkHistoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: apkHistoryReturn) -> Self {
                    (value.apkHash, value.updateBlockNumber, value.nextUpdateBlockNumber)
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = apkHistoryReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<24>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
            impl ::core::convert::From<currentApkReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentApkReturn) -> Self {
                    (value.X, value.Y)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentApkReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { X: tuple.0, Y: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentApkCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentApkReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        8,
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
            impl ::core::convert::From<deregisterOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.operator, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorCall {
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
            impl ::core::convert::From<deregisterOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<getApkCall> for UnderlyingRustTuple<'_> {
                fn from(value: getApkCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApkCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BN254::G1Point,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApkReturn;
            type ReturnTuple<'a> = (BN254::G1Point,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<getApkHashAtBlockNumberAndIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getApkHashAtBlockNumberAndIndexCall) -> Self {
                    (value.quorumNumber, value.blockNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getApkHashAtBlockNumberAndIndexCall {
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
            impl ::core::convert::From<getApkHashAtBlockNumberAndIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getApkHashAtBlockNumberAndIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getApkHashAtBlockNumberAndIndexReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApkHashAtBlockNumberAndIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<24>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<getApkHistoryLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getApkHistoryLengthCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getApkHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
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
            impl ::core::convert::From<getApkHistoryLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getApkHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getApkHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getApkHistoryLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApkHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<getApkIndicesAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getApkIndicesAtBlockNumberCall) -> Self {
                    (value.quorumNumbers, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getApkIndicesAtBlockNumberCall {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u32>,);
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
            impl ::core::convert::From<getApkIndicesAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getApkIndicesAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getApkIndicesAtBlockNumberReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApkIndicesAtBlockNumberReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
            type UnderlyingRustTuple<'a> = (
                u8,
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
            impl ::core::convert::From<getApkUpdateAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getApkUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getApkUpdateAtIndexCall {
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
            type UnderlyingRustTuple<'a> = (
                <IBLSApkRegistry::ApkUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getApkUpdateAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getApkUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getApkUpdateAtIndexReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApkUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IBLSApkRegistry::ApkUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<getOperatorFromPubkeyHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromPubkeyHashCall) -> Self {
                    (value.pubkeyHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorFromPubkeyHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pubkeyHash: tuple.0 }
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
            impl ::core::convert::From<getOperatorFromPubkeyHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromPubkeyHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorFromPubkeyHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorFromPubkeyHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorFromPubkeyHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<getRegisteredPubkeyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRegisteredPubkeyCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRegisteredPubkeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                BN254::G1Point,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getRegisteredPubkeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRegisteredPubkeyReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRegisteredPubkeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRegisteredPubkeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRegisteredPubkeyReturn;
            type ReturnTuple<'a> = (
                BN254::G1Point,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<initializeQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
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
            impl ::core::convert::From<initializeQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<operatorToPubkeyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorToPubkeyCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorToPubkeyCall {
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
            impl ::core::convert::From<operatorToPubkeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorToPubkeyReturn) -> Self {
                    (value.X, value.Y)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorToPubkeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { X: tuple.0, Y: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorToPubkeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorToPubkeyReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<operatorToPubkeyHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorToPubkeyHashCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorToPubkeyHashCall {
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
            impl ::core::convert::From<operatorToPubkeyHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorToPubkeyHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorToPubkeyHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorToPubkeyHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorToPubkeyHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<pubkeyHashToOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyHashToOperatorCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pubkeyHashToOperatorCall {
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
            impl ::core::convert::From<pubkeyHashToOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyHashToOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pubkeyHashToOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pubkeyHashToOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyHashToOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        pub params: <IBLSApkRegistry::PubkeyRegistrationParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerBLSPublicKeyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerBLSPublicKeyCall) -> Self {
                    (value.operator, value.params, value.pubkeyRegistrationMessageHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerBLSPublicKeyCall {
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
            impl ::core::convert::From<registerBLSPublicKeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerBLSPublicKeyReturn) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerBLSPublicKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorId: tuple.0 }
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerBLSPublicKeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<registerOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.operator, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorCall {
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
            impl ::core::convert::From<registerOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                Self::apkHistory(_) => {
                    <apkHistoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentApk(_) => {
                    <currentApkCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::getOperatorId(_) => {
                    <getOperatorIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
            ) -> alloy_sol_types::Result<BLSApkRegistryCalls>] = &[
                {
                    fn operatorToPubkey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryCalls> {
                        <operatorToPubkeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <getApkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <apkHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                        <currentApkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
                            )
                            .map(BLSApkRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
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
                115u8,
                162u8,
                183u8,
                251u8,
                132u8,
                71u8,
                36u8,
                185u8,
                113u8,
                128u8,
                42u8,
                233u8,
                177u8,
                93u8,
                176u8,
                148u8,
                212u8,
                183u8,
                25u8,
                45u8,
                249u8,
                215u8,
                53u8,
                14u8,
                20u8,
                235u8,
                70u8,
                107u8,
                155u8,
                34u8,
                235u8,
                78u8,
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
                227u8,
                251u8,
                102u8,
                19u8,
                175u8,
                46u8,
                137u8,
                48u8,
                207u8,
                133u8,
                212u8,
                127u8,
                207u8,
                109u8,
                177u8,
                1u8,
                146u8,
                34u8,
                74u8,
                100u8,
                198u8,
                203u8,
                232u8,
                2u8,
                62u8,
                14u8,
                238u8,
                27u8,
                163u8,
                130u8,
                128u8,
                65u8,
            ],
            [
                248u8,
                67u8,
                236u8,
                213u8,
                58u8,
                86u8,
                54u8,
                117u8,
                230u8,
                33u8,
                7u8,
                190u8,
                20u8,
                148u8,
                253u8,
                222u8,
                74u8,
                61u8,
                73u8,
                174u8,
                237u8,
                175u8,
                141u8,
                136u8,
                198u8,
                22u8,
                216u8,
                83u8,
                70u8,
                227u8,
                80u8,
                14u8,
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
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <NewPubkeyRegistration as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <NewPubkeyRegistration as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NewPubkeyRegistration)
                }
                Some(
                    <OperatorAddedToQuorums as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorAddedToQuorums as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorAddedToQuorums)
                }
                Some(
                    <OperatorRemovedFromQuorums as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorRemovedFromQuorums as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorRemovedFromQuorums)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<BLSApkRegistryInstance<T, P, N>>,
    > {
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
            f.debug_tuple("BLSApkRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BLSApkRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BLSApkRegistry`](self) contract instance.

See the [wrapper's documentation](`BLSApkRegistryInstance`) for more details.*/
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
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _registryCoordinator,
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
    > BLSApkRegistryInstance<T, P, N> {
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
            self.call_builder(
                &deregisterOperatorCall {
                    operator,
                    quorumNumbers,
                },
            )
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
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getApkHashAtBlockNumberAndIndexCall,
            N,
        > {
            self.call_builder(
                &getApkHashAtBlockNumberAndIndexCall {
                    quorumNumber,
                    blockNumber,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`getApkHistoryLength`] function.
        pub fn getApkHistoryLength(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getApkHistoryLengthCall, N> {
            self.call_builder(
                &getApkHistoryLengthCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getApkIndicesAtBlockNumber`] function.
        pub fn getApkIndicesAtBlockNumber(
            &self,
            quorumNumbers: alloy::sol_types::private::Bytes,
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getApkIndicesAtBlockNumberCall, N> {
            self.call_builder(
                &getApkIndicesAtBlockNumberCall {
                    quorumNumbers,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getApkUpdateAtIndex`] function.
        pub fn getApkUpdateAtIndex(
            &self,
            quorumNumber: u8,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getApkUpdateAtIndexCall, N> {
            self.call_builder(
                &getApkUpdateAtIndexCall {
                    quorumNumber,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorFromPubkeyHash`] function.
        pub fn getOperatorFromPubkeyHash(
            &self,
            pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorFromPubkeyHashCall, N> {
            self.call_builder(
                &getOperatorFromPubkeyHashCall {
                    pubkeyHash,
                },
            )
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
            self.call_builder(
                &getRegisteredPubkeyCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`initializeQuorum`] function.
        pub fn initializeQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeQuorumCall, N> {
            self.call_builder(
                &initializeQuorumCall {
                    quorumNumber,
                },
            )
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
            self.call_builder(
                &registerBLSPublicKeyCall {
                    operator,
                    params,
                    pubkeyRegistrationMessageHash,
                },
            )
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(
                &registerOperatorCall {
                    operator,
                    quorumNumbers,
                },
            )
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
    > BLSApkRegistryInstance<T, P, N> {
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
