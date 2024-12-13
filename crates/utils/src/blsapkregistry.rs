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
    ///0x60a060405234801561000f575f5ffd5b506040516136ee3803806136ee833981810160405281019061003191906101a9565b808073ffffffffffffffffffffffffffffffffffffffff1660808173ffffffffffffffffffffffffffffffffffffffff168152505061007461007b60201b60201c565b50506102a6565b5f60019054906101000a900460ff16156100ca576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100c190610254565b60405180910390fd5b60ff80165f5f9054906101000a900460ff1660ff1610156101385760ff5f5f6101000a81548160ff021916908360ff1602179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249860ff60405161012f919061028d565b60405180910390a15b565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6101678261013e565b9050919050565b5f6101788261015d565b9050919050565b6101888161016e565b8114610192575f5ffd5b50565b5f815190506101a38161017f565b92915050565b5f602082840312156101be576101bd61013a565b5b5f6101cb84828501610195565b91505092915050565b5f82825260208201905092915050565b7f496e697469616c697a61626c653a20636f6e747261637420697320696e6974695f8201527f616c697a696e6700000000000000000000000000000000000000000000000000602082015250565b5f61023e6027836101d4565b9150610249826101e4565b604082019050919050565b5f6020820190508181035f83015261026b81610232565b9050919050565b5f60ff82169050919050565b61028781610272565b82525050565b5f6020820190506102a05f83018461027e565b92915050565b6080516134296102c55f395f818161093201526111fc01526134295ff3fe608060405234801561000f575f5ffd5b5060043610610113575f3560e01c80636d14a987116100a0578063bf79ce581161006f578063bf79ce5814610352578063d5254a8c14610382578063de29fac0146103b2578063e8bb9ae6146103e2578063f4e24fe51461041257610113565b80636d14a987146102a05780637916cea6146102be5780637ff81a87146102f0578063a3db80e21461032157610113565b80633fb27952116100e75780633fb27952146101c457806347b314e8146101e05780635f61a88414610210578063605747d51461024057806368bccaac1461027057610113565b8062a1f4cb1461011757806313542a4e1461014857806326d941f214610178578063377ed99d14610194575b5f5ffd5b610131600480360381019061012c9190611d68565b61042e565b60405161013f929190611dab565b60405180910390f35b610162600480360381019061015d9190611d68565b61044e565b60405161016f9190611dea565b60405180910390f35b610192600480360381019061018d9190611e39565b610494565b005b6101ae60048036038101906101a99190611e39565b6105ec565b6040516101bb9190611e82565b60405180910390f35b6101de60048036038101906101d99190611fd7565b61060f565b005b6101fa60048036038101906101f5919061205b565b610676565b6040516102079190612095565b60405180910390f35b61022a60048036038101906102259190611e39565b6106af565b60405161023791906120ea565b60405180910390f35b61025a6004803603810190610255919061212d565b6106f3565b60405161026791906121f4565b60405180910390f35b61028a60048036038101906102859190612237565b6107b4565b6040516102979190612296565b60405180910390f35b6102a8610930565b6040516102b59190612095565b60405180910390f35b6102d860048036038101906102d3919061212d565b610954565b6040516102e7939291906122af565b60405180910390f35b61030a60048036038101906103059190611d68565b6109ba565b6040516103189291906122e4565b60405180910390f35b61033b60048036038101906103369190611e39565b610ab3565b604051610349929190611dab565b60405180910390f35b61036c6004803603810190610367919061234c565b610ad3565b6040516103799190611dea565b60405180910390f35b61039c600480360381019061039791906123fb565b610f37565b6040516103a99190612500565b60405180910390f35b6103cc60048036038101906103c79190611d68565b611146565b6040516103d99190611dea565b60405180910390f35b6103fc60048036038101906103f7919061205b565b61115b565b6040516104099190612095565b60405180910390f35b61042c60048036038101906104279190611fd7565b61118b565b005b6003602052805f5260405f205f91509050805f0154908060010154905082565b5f60015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20549050919050565b61049c6111fa565b5f60045f8360ff1660ff1681526020019081526020015f2080549050146104f8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104ef906125a0565b60405180910390fd5b60045f8260ff1660ff1681526020019081526020015f2060405180606001604052805f60401b67ffffffffffffffff191681526020014363ffffffff1681526020015f63ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c02179055506020820151815f0160186101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f01601c6101000a81548163ffffffff021916908363ffffffff160217905550505050565b5f60045f8360ff1660ff1681526020019081526020015f20805490509050919050565b6106176111fa565b5f610621836109ba565b50905061062e828261128a565b7f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e836106598561044e565b846040516106699392919061261e565b60405180910390a1505050565b5f60025f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050919050565b6106b7611bdf565b60055f8360ff1660ff1681526020019081526020015f206040518060400160405290815f82015481526020016001820154815250509050919050565b6106fb611bf7565b60045f8460ff1660ff1681526020019081526020015f2082815481106107245761072361265a565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900460401b67ffffffffffffffff191667ffffffffffffffff191681526020015f820160189054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f8201601c9054906101000a900463ffffffff1663ffffffff1663ffffffff1681525050905092915050565b5f5f60045f8660ff1660ff1681526020019081526020015f2083815481106107df576107de61265a565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900460401b67ffffffffffffffff191667ffffffffffffffff191681526020015f820160189054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f8201601c9054906101000a900463ffffffff1663ffffffff1663ffffffff16815250509050806020015163ffffffff168463ffffffff1610156108bc576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016108b3906126f7565b60405180910390fd5b5f816040015163ffffffff1614806108e35750806040015163ffffffff168463ffffffff16105b610922576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610919906127ab565b60405180910390fd5b805f01519150509392505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b6004602052815f5260405f20818154811061096d575f80fd5b905f5260205f20015f9150915050805f015f9054906101000a900460401b90805f0160189054906101000a900463ffffffff1690805f01601c9054906101000a900463ffffffff16905083565b6109c2611bdf565b5f5f60035f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060400160405290815f820154815260200160018201548152505090505f60015f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205490505f5f1b8103610aa6576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a9d90612839565b60405180910390fd5b8181935093505050915091565b6005602052805f5260405f205f91509050805f0154908060010154905082565b5f610adc6111fa565b5f610af984604001803603810190610af491906128a8565b61156b565b90507fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb58103610b5d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610b5490612943565b60405180910390fd5b5f5f1b60015f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205414610bde576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610bd5906129f7565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff1660025f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614610c7c576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610c7390612aab565b60405180910390fd5b5f7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001855f015f0135865f0160200135876040015f01358860400160200135896080015f018a6080016040018a5f01358b60200135604051602001610ce7989796959493929190612b02565b604051602081830303815290604052805190602001205f1c610d099190612bc0565b9050610db5610d58610d378388604001803603810190610d2991906128a8565b61158390919063ffffffff16565b875f01803603810190610d4a91906128a8565b61165790919063ffffffff16565b610d60611750565b610d9c610d7d85610d6f61181a565b61158390919063ffffffff16565b88803603810190610d8e91906128a8565b61165790919063ffffffff16565b88608001803603810190610db09190612ceb565b61183e565b610df4576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610deb90612dd2565b60405180910390fd5b8460400160035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181610e419190612eeb565b9050508160015f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508560025f8481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508573ffffffffffffffffffffffffffffffffffffffff167fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba38280418660400187608001604051610f23929190612fa3565b60405180910390a281925050509392505050565b60605f8484905067ffffffffffffffff811115610f5757610f56611eb3565b5b604051908082528060200260200182016040528015610f855781602001602082028036833780820191505090505b5090505f5f90505b8585905081101561113a575f868683818110610fac57610fab61265a565b5b9050013560f81c60f81b60f81c90505f60045f8360ff1660ff1681526020019081526020015f208054905090505f81148061102f575060045f8360ff1660ff1681526020019081526020015f205f8154811061100b5761100a61265a565b5b905f5260205f20015f0160189054906101000a900463ffffffff1663ffffffff1686105b1561106f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161106690613060565b60405180910390fd5b5f8190505b5f81111561112a578660045f8560ff1660ff1681526020019081526020015f206001836110a191906130ab565b815481106110b2576110b161265a565b5b905f5260205f20015f0160189054906101000a900463ffffffff1663ffffffff1611611117576001816110e591906130ab565b8585815181106110f8576110f761265a565b5b602002602001019063ffffffff16908163ffffffff168152505061112a565b8080611122906130de565b915050611074565b5050508080600101915050610f8d565b50809150509392505050565b6001602052805f5260405f205f915090505481565b6002602052805f5260405f205f915054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6111936111fa565b5f61119d836109ba565b5090506111b2826111ad83611b27565b61128a565b7ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e836111dd8561044e565b846040516111ed9392919061261e565b60405180910390a1505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611288576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161127f9061319b565b60405180910390fd5b565b611292611bdf565b5f5f90505b8351811015611565575f8482815181106112b4576112b361265a565b5b602001015160f81c60f81b60f81c90505f60045f8360ff1660ff1681526020019081526020015f208054905090505f8103611324576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161131b90613229565b60405180910390fd5b61136b8560055f8560ff1660ff1681526020019081526020015f206040518060400160405290815f820154815260200160018201548152505061165790919063ffffffff16565b93508360055f8460ff1660ff1681526020019081526020015f205f820151815f0155602082015181600101559050505f6113a48561156b565b90505f60045f8560ff1660ff1681526020019081526020015f206001846113cb91906130ab565b815481106113dc576113db61265a565b5b905f5260205f200190504363ffffffff16815f0160189054906101000a900463ffffffff1663ffffffff16036114435781815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c0217905550611554565b43815f01601c6101000a81548163ffffffff021916908363ffffffff16021790555060045f8560ff1660ff1681526020019081526020015f2060405180606001604052808467ffffffffffffffff191681526020014363ffffffff1681526020015f63ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c02179055506020820151815f0160186101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f01601c6101000a81548163ffffffff021916908363ffffffff16021790555050505b505050508080600101915050611297565b50505050565b5f81515f52816020015160205260405f209050919050565b61158b611bdf565b611593611c2c565b835f0151815f600381106115aa576115a961265a565b5b6020020181815250508360200151816001600381106115cc576115cb61265a565b5b60200201818152505082816002600381106115ea576115e961265a565b5b6020020181815250505f60408360608460076107d05a03fa9050805f810361160e57fe5b508061164f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161164690613291565b60405180910390fd5b505092915050565b61165f611bdf565b611667611c4e565b835f0151815f6004811061167e5761167d61265a565b5b6020020181815250508360200151816001600481106116a05761169f61265a565b5b602002018181525050825f0151816002600481106116c1576116c061265a565b5b6020020181815250508260200151816003600481106116e3576116e261265a565b5b6020020181815250505f60408360808460066107d05a03fa9050805f810361170757fe5b5080611748576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161173f906132f9565b60405180910390fd5b505092915050565b611758611c70565b604051806040016040528060405180604001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed815250815260200160405180604001604052807f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec81526020017f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d815250815250905090565b611822611bdf565b6040518060400160405280600181526020016002815250905090565b5f5f60405180604001604052808781526020018581525090505f6040518060400160405280878152602001858152509050611877611c96565b5f5f90505b6002811015611a95575f6006826118939190613317565b90508482600281106118a8576118a761265a565b5b60200201515f0151835f836118bd9190613358565b600c81106118ce576118cd61265a565b5b6020020181815250508482600281106118ea576118e961265a565b5b602002015160200151836001836119019190613358565b600c81106119125761191161265a565b5b60200201818152505083826002811061192e5761192d61265a565b5b60200201515f01515f600281106119485761194761265a565b5b60200201518360028361195b9190613358565b600c811061196c5761196b61265a565b5b6020020181815250508382600281106119885761198761265a565b5b60200201515f01516001600281106119a3576119a261265a565b5b6020020151836003836119b69190613358565b600c81106119c7576119c661265a565b5b6020020181815250508382600281106119e3576119e261265a565b5b6020020151602001515f600281106119fe576119fd61265a565b5b602002015183600483611a119190613358565b600c8110611a2257611a2161265a565b5b602002018181525050838260028110611a3e57611a3d61265a565b5b602002015160200151600160028110611a5a57611a5961265a565b5b602002015183600583611a6d9190613358565b600c8110611a7e57611a7d61265a565b5b60200201818152505050808060010191505061187c565b50611a9e611cb9565b5f6020826020600c028560086107d05a03fa9050805f8103611abc57fe5b5080611afd576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611af4906133d5565b60405180910390fd5b5f825f60018110611b1157611b1061265a565b5b6020020151141595505050505050949350505050565b611b2f611bdf565b5f825f0151148015611b4457505f8260200151145b15611b655760405180604001604052805f81526020015f8152509050611bda565b6040518060400160405280835f015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478460200151611ba99190612bc0565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611bd491906130ab565b81525090505b919050565b60405180604001604052805f81526020015f81525090565b60405180606001604052805f67ffffffffffffffff191681526020015f63ffffffff1681526020015f63ffffffff1681525090565b6040518060600160405280600390602082028036833780820191505090505090565b6040518060800160405280600490602082028036833780820191505090505090565b6040518060400160405280611c83611cdb565b8152602001611c90611cdb565b81525090565b604051806101800160405280600c90602082028036833780820191505090505090565b6040518060200160405280600190602082028036833780820191505090505090565b6040518060400160405280600290602082028036833780820191505090505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f611d3782611d0e565b9050919050565b611d4781611d2d565b8114611d51575f5ffd5b50565b5f81359050611d6281611d3e565b92915050565b5f60208284031215611d7d57611d7c611d06565b5b5f611d8a84828501611d54565b91505092915050565b5f819050919050565b611da581611d93565b82525050565b5f604082019050611dbe5f830185611d9c565b611dcb6020830184611d9c565b9392505050565b5f819050919050565b611de481611dd2565b82525050565b5f602082019050611dfd5f830184611ddb565b92915050565b5f60ff82169050919050565b611e1881611e03565b8114611e22575f5ffd5b50565b5f81359050611e3381611e0f565b92915050565b5f60208284031215611e4e57611e4d611d06565b5b5f611e5b84828501611e25565b91505092915050565b5f63ffffffff82169050919050565b611e7c81611e64565b82525050565b5f602082019050611e955f830184611e73565b92915050565b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b611ee982611ea3565b810181811067ffffffffffffffff82111715611f0857611f07611eb3565b5b80604052505050565b5f611f1a611cfd565b9050611f268282611ee0565b919050565b5f67ffffffffffffffff821115611f4557611f44611eb3565b5b611f4e82611ea3565b9050602081019050919050565b828183375f83830152505050565b5f611f7b611f7684611f2b565b611f11565b905082815260208101848484011115611f9757611f96611e9f565b5b611fa2848285611f5b565b509392505050565b5f82601f830112611fbe57611fbd611e9b565b5b8135611fce848260208601611f69565b91505092915050565b5f5f60408385031215611fed57611fec611d06565b5b5f611ffa85828601611d54565b925050602083013567ffffffffffffffff81111561201b5761201a611d0a565b5b61202785828601611faa565b9150509250929050565b61203a81611dd2565b8114612044575f5ffd5b50565b5f8135905061205581612031565b92915050565b5f602082840312156120705761206f611d06565b5b5f61207d84828501612047565b91505092915050565b61208f81611d2d565b82525050565b5f6020820190506120a85f830184612086565b92915050565b6120b781611d93565b82525050565b604082015f8201516120d15f8501826120ae565b5060208201516120e460208501826120ae565b50505050565b5f6040820190506120fd5f8301846120bd565b92915050565b61210c81611d93565b8114612116575f5ffd5b50565b5f8135905061212781612103565b92915050565b5f5f6040838503121561214357612142611d06565b5b5f61215085828601611e25565b925050602061216185828601612119565b9150509250929050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000082169050919050565b61219f8161216b565b82525050565b6121ae81611e64565b82525050565b606082015f8201516121c85f850182612196565b5060208201516121db60208501826121a5565b5060408201516121ee60408501826121a5565b50505050565b5f6060820190506122075f8301846121b4565b92915050565b61221681611e64565b8114612220575f5ffd5b50565b5f813590506122318161220d565b92915050565b5f5f5f6060848603121561224e5761224d611d06565b5b5f61225b86828701611e25565b935050602061226c86828701612223565b925050604061227d86828701612119565b9150509250925092565b6122908161216b565b82525050565b5f6020820190506122a95f830184612287565b92915050565b5f6060820190506122c25f830186612287565b6122cf6020830185611e73565b6122dc6040830184611e73565b949350505050565b5f6060820190506122f75f8301856120bd565b6123046040830184611ddb565b9392505050565b5f5ffd5b5f61010082840312156123255761232461230b565b5b81905092915050565b5f604082840312156123435761234261230b565b5b81905092915050565b5f5f5f610160848603121561236457612363611d06565b5b5f61237186828701611d54565b93505060206123828682870161230f565b9250506101206123948682870161232e565b9150509250925092565b5f5ffd5b5f5ffd5b5f5f83601f8401126123bb576123ba611e9b565b5b8235905067ffffffffffffffff8111156123d8576123d761239e565b5b6020830191508360018202830111156123f4576123f36123a2565b5b9250929050565b5f5f5f6040848603121561241257612411611d06565b5b5f84013567ffffffffffffffff81111561242f5761242e611d0a565b5b61243b868287016123a6565b9350935050602061244e86828701612119565b9150509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f61248c83836121a5565b60208301905092915050565b5f602082019050919050565b5f6124ae82612458565b6124b88185612462565b93506124c383612472565b805f5b838110156124f35781516124da8882612481565b97506124e583612498565b9250506001810190506124c6565b5085935050505092915050565b5f6020820190508181035f83015261251881846124a4565b905092915050565b5f82825260208201905092915050565b7f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a5f8201527f2071756f72756d20616c72656164792065786973747300000000000000000000602082015250565b5f61258a603683612520565b915061259582612530565b604082019050919050565b5f6020820190508181035f8301526125b78161257e565b9050919050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f6125f0826125be565b6125fa81856125c8565b935061260a8185602086016125d8565b61261381611ea3565b840191505092915050565b5f6060820190506126315f830186612086565b61263e6020830185611ddb565b818103604083015261265081846125e6565b9050949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f424c5341706b52656769737472792e67657441706b486173684174426c6f636b5f8201527f4e756d626572416e64496e6465783a20696e64657820746f6f20726563656e74602082015250565b5f6126e1604083612520565b91506126ec82612687565b604082019050919050565b5f6020820190508181035f83015261270e816126d5565b9050919050565b7f424c5341706b52656769737472792e67657441706b486173684174426c6f636b5f8201527f4e756d626572416e64496e6465783a206e6f74206c61746573742061706b207560208201527f7064617465000000000000000000000000000000000000000000000000000000604082015250565b5f612795604583612520565b91506127a082612715565b606082019050919050565b5f6020820190508181035f8301526127c281612789565b9050919050565b7f424c5341706b52656769737472792e676574526567697374657265645075626b5f8201527f65793a206f70657261746f72206973206e6f7420726567697374657265640000602082015250565b5f612823603e83612520565b915061282e826127c9565b604082019050919050565b5f6020820190508181035f83015261285081612817565b9050919050565b5f5ffd5b5f604082840312156128705761286f612857565b5b61287a6040611f11565b90505f61288984828501612119565b5f83015250602061289c84828501612119565b60208301525092915050565b5f604082840312156128bd576128bc611d06565b5b5f6128ca8482850161285b565b91505092915050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b6579602082015250565b5f61292d604083612520565b9150612938826128d3565b604082019050919050565b5f6020820190508181035f83015261295a81612921565b9050919050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a206f70657261746f7220616c7265616479207265676973746572656460208201527f207075626b657900000000000000000000000000000000000000000000000000604082015250565b5f6129e1604783612520565b91506129ec82612961565b606082019050919050565b5f6020820190508181035f830152612a0e816129d5565b9050919050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a207075626c6963206b657920616c726561647920726567697374657260208201527f6564000000000000000000000000000000000000000000000000000000000000604082015250565b5f612a95604283612520565b9150612aa082612a15565b606082019050919050565b5f6020820190508181035f830152612ac281612a89565b9050919050565b5f819050919050565b612ae3612ade82611d93565b612ac9565b82525050565b82818337505050565b612afe60408383612ae9565b5050565b5f612b0d828b612ad2565b602082019150612b1d828a612ad2565b602082019150612b2d8289612ad2565b602082019150612b3d8288612ad2565b602082019150612b4d8287612af2565b604082019150612b5d8286612af2565b604082019150612b6d8285612ad2565b602082019150612b7d8284612ad2565b6020820191508190509998505050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f612bca82611d93565b9150612bd583611d93565b925082612be557612be4612b93565b5b828206905092915050565b5f67ffffffffffffffff821115612c0a57612c09611eb3565b5b602082029050919050565b5f612c27612c2284612bf0565b611f11565b90508060208402830185811115612c4157612c406123a2565b5b835b81811015612c6a5780612c568882612119565b845260208401935050602081019050612c43565b5050509392505050565b5f82601f830112612c8857612c87611e9b565b5b6002612c95848285612c15565b91505092915050565b5f60808284031215612cb357612cb2612857565b5b612cbd6040611f11565b90505f612ccc84828501612c74565b5f830152506040612cdf84828501612c74565b60208301525092915050565b5f60808284031215612d0057612cff611d06565b5b5f612d0d84828501612c9e565b91505092915050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a2065697468657220746865204731207369676e61747572652069732060208201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060408201527f646f206e6f74206d617463680000000000000000000000000000000000000000606082015250565b5f612dbc606c83612520565b9150612dc782612d16565b608082019050919050565b5f6020820190508181035f830152612de981612db0565b9050919050565b5f8135612dfc81612103565b80915050919050565b5f815f1b9050919050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff612e3b84612e05565b9350801983169250808416831791505092915050565b5f819050919050565b5f612e74612e6f612e6a84611d93565b612e51565b611d93565b9050919050565b5f819050919050565b612e8d82612e5a565b612ea0612e9982612e7b565b8354612e10565b8255505050565b5f81015f830180612eb781612df0565b9050612ec38184612e84565b505050600181016020830180612ed881612df0565b9050612ee48184612e84565b5050505050565b612ef58282612ea7565b5050565b5f612f076020840184612119565b905092915050565b60408201612f1f5f830183612ef9565b612f2b5f8501826120ae565b50612f396020830183612ef9565b612f4660208501826120ae565b50505050565b5f82905092915050565b612f6260408383612ae9565b5050565b60808201612f765f830183612f4c565b612f825f850182612f56565b50612f906040830183612f4c565b612f9d6040850182612f56565b50505050565b5f60c082019050612fb65f830185612f0f565b612fc36040830184612f66565b9392505050565b7f424c5341706b52656769737472792e67657441706b496e64696365734174426c5f8201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f726560208201527f2074686520666972737420757064617465000000000000000000000000000000604082015250565b5f61304a605183612520565b915061305582612fca565b606082019050919050565b5f6020820190508181035f8301526130778161303e565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6130b582611d93565b91506130c083611d93565b92508282039050818111156130d8576130d761307e565b5b92915050565b5f6130e882611d93565b91505f82036130fa576130f961307e565b5b600182039050919050565b7f424c5341706b52656769737472792e5f636865636b5265676973747279436f6f5f8201527f7264696e61746f723a2063616c6c6572206973206e6f7420746865207265676960208201527f7374727920636f6f7264696e61746f7200000000000000000000000000000000604082015250565b5f613185605083612520565b915061319082613105565b606082019050919050565b5f6020820190508181035f8301526131b281613179565b9050919050565b7f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b5f8201527f5570646174653a2071756f72756d20646f6573206e6f74206578697374000000602082015250565b5f613213603d83612520565b915061321e826131b9565b604082019050919050565b5f6020820190508181035f83015261324081613207565b9050919050565b7f65632d6d756c2d6661696c6564000000000000000000000000000000000000005f82015250565b5f61327b600d83612520565b915061328682613247565b602082019050919050565b5f6020820190508181035f8301526132a88161326f565b9050919050565b7f65632d6164642d6661696c6564000000000000000000000000000000000000005f82015250565b5f6132e3600d83612520565b91506132ee826132af565b602082019050919050565b5f6020820190508181035f830152613310816132d7565b9050919050565b5f61332182611d93565b915061332c83611d93565b925082820261333a81611d93565b915082820484148315176133515761335061307e565b5b5092915050565b5f61336282611d93565b915061336d83611d93565b92508282019050808211156133855761338461307e565b5b92915050565b7f70616972696e672d6f70636f64652d6661696c656400000000000000000000005f82015250565b5f6133bf601583612520565b91506133ca8261338b565b602082019050919050565b5f6020820190508181035f8301526133ec816133b3565b905091905056fea26469706673582212201232bd7323c7c8671ff9a4dbf3eba545d7f2dd78e395ca0bae1118056a966a1a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa6\xEE8\x03\x80a6\xEE\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x01\xA9V[\x80\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\0ta\0{` \x1B` \x1CV[PPa\x02\xA6V[_`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\0\xCAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xC1\x90a\x02TV[`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10\x15a\x018W`\xFF__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\xFF`@Qa\x01/\x91\x90a\x02\x8DV[`@Q\x80\x91\x03\x90\xA1[V[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x01g\x82a\x01>V[\x90P\x91\x90PV[_a\x01x\x82a\x01]V[\x90P\x91\x90PV[a\x01\x88\x81a\x01nV[\x81\x14a\x01\x92W__\xFD[PV[_\x81Q\x90Pa\x01\xA3\x81a\x01\x7FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x01\xBEWa\x01\xBDa\x01:V[[_a\x01\xCB\x84\x82\x85\x01a\x01\x95V[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FInitializable: contract is initi_\x82\x01R\x7Falizing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x02>`'\x83a\x01\xD4V[\x91Pa\x02I\x82a\x01\xE4V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x02k\x81a\x022V[\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x02\x87\x81a\x02rV[\x82RPPV[_` \x82\x01\x90Pa\x02\xA0_\x83\x01\x84a\x02~V[\x92\x91PPV[`\x80Qa4)a\x02\xC5_9_\x81\x81a\t2\x01Ra\x11\xFC\x01Ra4)_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x13W_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\xA0W\x80c\xBFy\xCEX\x11a\0oW\x80c\xBFy\xCEX\x14a\x03RW\x80c\xD5%J\x8C\x14a\x03\x82W\x80c\xDE)\xFA\xC0\x14a\x03\xB2W\x80c\xE8\xBB\x9A\xE6\x14a\x03\xE2W\x80c\xF4\xE2O\xE5\x14a\x04\x12Wa\x01\x13V[\x80cm\x14\xA9\x87\x14a\x02\xA0W\x80cy\x16\xCE\xA6\x14a\x02\xBEW\x80c\x7F\xF8\x1A\x87\x14a\x02\xF0W\x80c\xA3\xDB\x80\xE2\x14a\x03!Wa\x01\x13V[\x80c?\xB2yR\x11a\0\xE7W\x80c?\xB2yR\x14a\x01\xC4W\x80cG\xB3\x14\xE8\x14a\x01\xE0W\x80c_a\xA8\x84\x14a\x02\x10W\x80c`WG\xD5\x14a\x02@W\x80ch\xBC\xCA\xAC\x14a\x02pWa\x01\x13V[\x80b\xA1\xF4\xCB\x14a\x01\x17W\x80c\x13T*N\x14a\x01HW\x80c&\xD9A\xF2\x14a\x01xW\x80c7~\xD9\x9D\x14a\x01\x94W[__\xFD[a\x011`\x04\x806\x03\x81\x01\x90a\x01,\x91\x90a\x1DhV[a\x04.V[`@Qa\x01?\x92\x91\x90a\x1D\xABV[`@Q\x80\x91\x03\x90\xF3[a\x01b`\x04\x806\x03\x81\x01\x90a\x01]\x91\x90a\x1DhV[a\x04NV[`@Qa\x01o\x91\x90a\x1D\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x01\x92`\x04\x806\x03\x81\x01\x90a\x01\x8D\x91\x90a\x1E9V[a\x04\x94V[\0[a\x01\xAE`\x04\x806\x03\x81\x01\x90a\x01\xA9\x91\x90a\x1E9V[a\x05\xECV[`@Qa\x01\xBB\x91\x90a\x1E\x82V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDE`\x04\x806\x03\x81\x01\x90a\x01\xD9\x91\x90a\x1F\xD7V[a\x06\x0FV[\0[a\x01\xFA`\x04\x806\x03\x81\x01\x90a\x01\xF5\x91\x90a [V[a\x06vV[`@Qa\x02\x07\x91\x90a \x95V[`@Q\x80\x91\x03\x90\xF3[a\x02*`\x04\x806\x03\x81\x01\x90a\x02%\x91\x90a\x1E9V[a\x06\xAFV[`@Qa\x027\x91\x90a \xEAV[`@Q\x80\x91\x03\x90\xF3[a\x02Z`\x04\x806\x03\x81\x01\x90a\x02U\x91\x90a!-V[a\x06\xF3V[`@Qa\x02g\x91\x90a!\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x02\x8A`\x04\x806\x03\x81\x01\x90a\x02\x85\x91\x90a\"7V[a\x07\xB4V[`@Qa\x02\x97\x91\x90a\"\x96V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA8a\t0V[`@Qa\x02\xB5\x91\x90a \x95V[`@Q\x80\x91\x03\x90\xF3[a\x02\xD8`\x04\x806\x03\x81\x01\x90a\x02\xD3\x91\x90a!-V[a\tTV[`@Qa\x02\xE7\x93\x92\x91\x90a\"\xAFV[`@Q\x80\x91\x03\x90\xF3[a\x03\n`\x04\x806\x03\x81\x01\x90a\x03\x05\x91\x90a\x1DhV[a\t\xBAV[`@Qa\x03\x18\x92\x91\x90a\"\xE4V[`@Q\x80\x91\x03\x90\xF3[a\x03;`\x04\x806\x03\x81\x01\x90a\x036\x91\x90a\x1E9V[a\n\xB3V[`@Qa\x03I\x92\x91\x90a\x1D\xABV[`@Q\x80\x91\x03\x90\xF3[a\x03l`\x04\x806\x03\x81\x01\x90a\x03g\x91\x90a#LV[a\n\xD3V[`@Qa\x03y\x91\x90a\x1D\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x03\x9C`\x04\x806\x03\x81\x01\x90a\x03\x97\x91\x90a#\xFBV[a\x0F7V[`@Qa\x03\xA9\x91\x90a%\0V[`@Q\x80\x91\x03\x90\xF3[a\x03\xCC`\x04\x806\x03\x81\x01\x90a\x03\xC7\x91\x90a\x1DhV[a\x11FV[`@Qa\x03\xD9\x91\x90a\x1D\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x03\xFC`\x04\x806\x03\x81\x01\x90a\x03\xF7\x91\x90a [V[a\x11[V[`@Qa\x04\t\x91\x90a \x95V[`@Q\x80\x91\x03\x90\xF3[a\x04,`\x04\x806\x03\x81\x01\x90a\x04'\x91\x90a\x1F\xD7V[a\x11\x8BV[\0[`\x03` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01T\x90P\x82V[_`\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[a\x04\x9Ca\x11\xFAV[_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14a\x04\xF8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xEF\x90a%\xA0V[`@Q\x80\x91\x03\x90\xFD[`\x04_\x82`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80_`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[a\x06\x17a\x11\xFAV[_a\x06!\x83a\t\xBAV[P\x90Pa\x06.\x82\x82a\x12\x8AV[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06Y\x85a\x04NV[\x84`@Qa\x06i\x93\x92\x91\x90a&\x1EV[`@Q\x80\x91\x03\x90\xA1PPPV[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[a\x06\xB7a\x1B\xDFV[`\x05_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[a\x06\xFBa\x1B\xF7V[`\x04_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x07$Wa\x07#a&ZV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[__`\x04_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x07\xDFWa\x07\xDEa&ZV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10\x15a\x08\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xB3\x90a&\xF7V[`@Q\x80\x91\x03\x90\xFD[_\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x14\x80a\x08\xE3WP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\t\"W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x19\x90a'\xABV[`@Q\x80\x91\x03\x90\xFD[\x80_\x01Q\x91PP\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\tmW_\x80\xFD[\x90_R` _ \x01_\x91P\x91PP\x80_\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1B\x90\x80_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x83V[a\t\xC2a\x1B\xDFV[__`\x03_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P_`\x01_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P__\x1B\x81\x03a\n\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x9D\x90a(9V[`@Q\x80\x91\x03\x90\xFD[\x81\x81\x93P\x93PPP\x91P\x91V[`\x05` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01T\x90P\x82V[_a\n\xDCa\x11\xFAV[_a\n\xF9\x84`@\x01\x806\x03\x81\x01\x90a\n\xF4\x91\x90a(\xA8V[a\x15kV[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x03a\x0B]W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0BT\x90a)CV[`@Q\x80\x91\x03\x90\xFD[__\x1B`\x01_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x14a\x0B\xDEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xD5\x90a)\xF7V[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C|W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Cs\x90a*\xABV[`@Q\x80\x91\x03\x90\xFD[_\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x85_\x01_\x015\x86_\x01` \x015\x87`@\x01_\x015\x88`@\x01` \x015\x89`\x80\x01_\x01\x8A`\x80\x01`@\x01\x8A_\x015\x8B` \x015`@Q` \x01a\x0C\xE7\x98\x97\x96\x95\x94\x93\x92\x91\x90a+\x02V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\r\t\x91\x90a+\xC0V[\x90Pa\r\xB5a\rXa\r7\x83\x88`@\x01\x806\x03\x81\x01\x90a\r)\x91\x90a(\xA8V[a\x15\x83\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87_\x01\x806\x03\x81\x01\x90a\rJ\x91\x90a(\xA8V[a\x16W\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r`a\x17PV[a\r\x9Ca\r}\x85a\roa\x18\x1AV[a\x15\x83\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88\x806\x03\x81\x01\x90a\r\x8E\x91\x90a(\xA8V[a\x16W\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\x80\x01\x806\x03\x81\x01\x90a\r\xB0\x91\x90a,\xEBV[a\x18>V[a\r\xF4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xEB\x90a-\xD2V[`@Q\x80\x91\x03\x90\xFD[\x84`@\x01`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a\x0EA\x91\x90a.\xEBV[\x90PP\x81`\x01_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x85`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x86`@\x01\x87`\x80\x01`@Qa\x0F#\x92\x91\x90a/\xA3V[`@Q\x80\x91\x03\x90\xA2\x81\x92PPP\x93\x92PPPV[``_\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FWWa\x0FVa\x1E\xB3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x85W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x85\x85\x90P\x81\x10\x15a\x11:W_\x86\x86\x83\x81\x81\x10a\x0F\xACWa\x0F\xABa&ZV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x14\x80a\x10/WP`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x81T\x81\x10a\x10\x0BWa\x10\na&ZV[[\x90_R` _ \x01_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x10oW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10f\x90a0`V[`@Q\x80\x91\x03\x90\xFD[_\x81\x90P[_\x81\x11\x15a\x11*W\x86`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a\x10\xA1\x91\x90a0\xABV[\x81T\x81\x10a\x10\xB2Wa\x10\xB1a&ZV[[\x90_R` _ \x01_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a\x11\x17W`\x01\x81a\x10\xE5\x91\x90a0\xABV[\x85\x85\x81Q\x81\x10a\x10\xF8Wa\x10\xF7a&ZV[[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x11*V[\x80\x80a\x11\"\x90a0\xDEV[\x91PPa\x10tV[PPP\x80\x80`\x01\x01\x91PPa\x0F\x8DV[P\x80\x91PP\x93\x92PPPV[`\x01` R\x80_R`@_ _\x91P\x90PT\x81V[`\x02` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x11\x93a\x11\xFAV[_a\x11\x9D\x83a\t\xBAV[P\x90Pa\x11\xB2\x82a\x11\xAD\x83a\x1B'V[a\x12\x8AV[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x11\xDD\x85a\x04NV[\x84`@Qa\x11\xED\x93\x92\x91\x90a&\x1EV[`@Q\x80\x91\x03\x90\xA1PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x12\x88W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\x7F\x90a1\x9BV[`@Q\x80\x91\x03\x90\xFD[V[a\x12\x92a\x1B\xDFV[__\x90P[\x83Q\x81\x10\x15a\x15eW_\x84\x82\x81Q\x81\x10a\x12\xB4Wa\x12\xB3a&ZV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a\x13$W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\x1B\x90a2)V[`@Q\x80\x91\x03\x90\xFD[a\x13k\x85`\x05_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x16W\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93P\x83`\x05_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP_a\x13\xA4\x85a\x15kV[\x90P_`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x84a\x13\xCB\x91\x90a0\xABV[\x81T\x81\x10a\x13\xDCWa\x13\xDBa&ZV[[\x90_R` _ \x01\x90PCc\xFF\xFF\xFF\xFF\x16\x81_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a\x14CW\x81\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UPa\x15TV[C\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[PPPP\x80\x80`\x01\x01\x91PPa\x12\x97V[PPPPV[_\x81Q_R\x81` \x01Q` R`@_ \x90P\x91\x90PV[a\x15\x8Ba\x1B\xDFV[a\x15\x93a\x1C,V[\x83_\x01Q\x81_`\x03\x81\x10a\x15\xAAWa\x15\xA9a&ZV[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x03\x81\x10a\x15\xCCWa\x15\xCBa&ZV[[` \x02\x01\x81\x81RPP\x82\x81`\x02`\x03\x81\x10a\x15\xEAWa\x15\xE9a&ZV[[` \x02\x01\x81\x81RPP_`@\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x16\x0EW\xFE[P\x80a\x16OW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16F\x90a2\x91V[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x16_a\x1B\xDFV[a\x16ga\x1CNV[\x83_\x01Q\x81_`\x04\x81\x10a\x16~Wa\x16}a&ZV[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x04\x81\x10a\x16\xA0Wa\x16\x9Fa&ZV[[` \x02\x01\x81\x81RPP\x82_\x01Q\x81`\x02`\x04\x81\x10a\x16\xC1Wa\x16\xC0a&ZV[[` \x02\x01\x81\x81RPP\x82` \x01Q\x81`\x03`\x04\x81\x10a\x16\xE3Wa\x16\xE2a&ZV[[` \x02\x01\x81\x81RPP_`@\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x17\x07W\xFE[P\x80a\x17HW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17?\x90a2\xF9V[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x17Xa\x1CpV[`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x81R` \x01\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D\x81RP\x81RP\x90P\x90V[a\x18\"a\x1B\xDFV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x02\x81RP\x90P\x90V[__`@Q\x80`@\x01`@R\x80\x87\x81R` \x01\x85\x81RP\x90P_`@Q\x80`@\x01`@R\x80\x87\x81R` \x01\x85\x81RP\x90Pa\x18wa\x1C\x96V[__\x90P[`\x02\x81\x10\x15a\x1A\x95W_`\x06\x82a\x18\x93\x91\x90a3\x17V[\x90P\x84\x82`\x02\x81\x10a\x18\xA8Wa\x18\xA7a&ZV[[` \x02\x01Q_\x01Q\x83_\x83a\x18\xBD\x91\x90a3XV[`\x0C\x81\x10a\x18\xCEWa\x18\xCDa&ZV[[` \x02\x01\x81\x81RPP\x84\x82`\x02\x81\x10a\x18\xEAWa\x18\xE9a&ZV[[` \x02\x01Q` \x01Q\x83`\x01\x83a\x19\x01\x91\x90a3XV[`\x0C\x81\x10a\x19\x12Wa\x19\x11a&ZV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x19.Wa\x19-a&ZV[[` \x02\x01Q_\x01Q_`\x02\x81\x10a\x19HWa\x19Ga&ZV[[` \x02\x01Q\x83`\x02\x83a\x19[\x91\x90a3XV[`\x0C\x81\x10a\x19lWa\x19ka&ZV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x19\x88Wa\x19\x87a&ZV[[` \x02\x01Q_\x01Q`\x01`\x02\x81\x10a\x19\xA3Wa\x19\xA2a&ZV[[` \x02\x01Q\x83`\x03\x83a\x19\xB6\x91\x90a3XV[`\x0C\x81\x10a\x19\xC7Wa\x19\xC6a&ZV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x19\xE3Wa\x19\xE2a&ZV[[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x19\xFEWa\x19\xFDa&ZV[[` \x02\x01Q\x83`\x04\x83a\x1A\x11\x91\x90a3XV[`\x0C\x81\x10a\x1A\"Wa\x1A!a&ZV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x1A>Wa\x1A=a&ZV[[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x1AZWa\x1AYa&ZV[[` \x02\x01Q\x83`\x05\x83a\x1Am\x91\x90a3XV[`\x0C\x81\x10a\x1A~Wa\x1A}a&ZV[[` \x02\x01\x81\x81RPPP\x80\x80`\x01\x01\x91PPa\x18|V[Pa\x1A\x9Ea\x1C\xB9V[_` \x82` `\x0C\x02\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x1A\xBCW\xFE[P\x80a\x1A\xFDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\xF4\x90a3\xD5V[`@Q\x80\x91\x03\x90\xFD[_\x82_`\x01\x81\x10a\x1B\x11Wa\x1B\x10a&ZV[[` \x02\x01Q\x14\x15\x95PPPPPP\x94\x93PPPPV[a\x1B/a\x1B\xDFV[_\x82_\x01Q\x14\x80\x15a\x1BDWP_\x82` \x01Q\x14[\x15a\x1BeW`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x1B\xDAV[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x1B\xA9\x91\x90a+\xC0V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1B\xD4\x91\x90a0\xABV[\x81RP\x90P[\x91\x90PV[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80a\x1C\x83a\x1C\xDBV[\x81R` \x01a\x1C\x90a\x1C\xDBV[\x81RP\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x1D7\x82a\x1D\x0EV[\x90P\x91\x90PV[a\x1DG\x81a\x1D-V[\x81\x14a\x1DQW__\xFD[PV[_\x815\x90Pa\x1Db\x81a\x1D>V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1D}Wa\x1D|a\x1D\x06V[[_a\x1D\x8A\x84\x82\x85\x01a\x1DTV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x1D\xA5\x81a\x1D\x93V[\x82RPPV[_`@\x82\x01\x90Pa\x1D\xBE_\x83\x01\x85a\x1D\x9CV[a\x1D\xCB` \x83\x01\x84a\x1D\x9CV[\x93\x92PPPV[_\x81\x90P\x91\x90PV[a\x1D\xE4\x81a\x1D\xD2V[\x82RPPV[_` \x82\x01\x90Pa\x1D\xFD_\x83\x01\x84a\x1D\xDBV[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x1E\x18\x81a\x1E\x03V[\x81\x14a\x1E\"W__\xFD[PV[_\x815\x90Pa\x1E3\x81a\x1E\x0FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1ENWa\x1EMa\x1D\x06V[[_a\x1E[\x84\x82\x85\x01a\x1E%V[\x91PP\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x1E|\x81a\x1EdV[\x82RPPV[_` \x82\x01\x90Pa\x1E\x95_\x83\x01\x84a\x1EsV[\x92\x91PPV[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x1E\xE9\x82a\x1E\xA3V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1F\x08Wa\x1F\x07a\x1E\xB3V[[\x80`@RPPPV[_a\x1F\x1Aa\x1C\xFDV[\x90Pa\x1F&\x82\x82a\x1E\xE0V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1FEWa\x1FDa\x1E\xB3V[[a\x1FN\x82a\x1E\xA3V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x1F{a\x1Fv\x84a\x1F+V[a\x1F\x11V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x1F\x97Wa\x1F\x96a\x1E\x9FV[[a\x1F\xA2\x84\x82\x85a\x1F[V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x1F\xBEWa\x1F\xBDa\x1E\x9BV[[\x815a\x1F\xCE\x84\x82` \x86\x01a\x1FiV[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x1F\xEDWa\x1F\xECa\x1D\x06V[[_a\x1F\xFA\x85\x82\x86\x01a\x1DTV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x1BWa \x1Aa\x1D\nV[[a '\x85\x82\x86\x01a\x1F\xAAV[\x91PP\x92P\x92\x90PV[a :\x81a\x1D\xD2V[\x81\x14a DW__\xFD[PV[_\x815\x90Pa U\x81a 1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a pWa oa\x1D\x06V[[_a }\x84\x82\x85\x01a GV[\x91PP\x92\x91PPV[a \x8F\x81a\x1D-V[\x82RPPV[_` \x82\x01\x90Pa \xA8_\x83\x01\x84a \x86V[\x92\x91PPV[a \xB7\x81a\x1D\x93V[\x82RPPV[`@\x82\x01_\x82\x01Qa \xD1_\x85\x01\x82a \xAEV[P` \x82\x01Qa \xE4` \x85\x01\x82a \xAEV[PPPPV[_`@\x82\x01\x90Pa \xFD_\x83\x01\x84a \xBDV[\x92\x91PPV[a!\x0C\x81a\x1D\x93V[\x81\x14a!\x16W__\xFD[PV[_\x815\x90Pa!'\x81a!\x03V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a!CWa!Ba\x1D\x06V[[_a!P\x85\x82\x86\x01a\x1E%V[\x92PP` a!a\x85\x82\x86\x01a!\x19V[\x91PP\x92P\x92\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a!\x9F\x81a!kV[\x82RPPV[a!\xAE\x81a\x1EdV[\x82RPPV[``\x82\x01_\x82\x01Qa!\xC8_\x85\x01\x82a!\x96V[P` \x82\x01Qa!\xDB` \x85\x01\x82a!\xA5V[P`@\x82\x01Qa!\xEE`@\x85\x01\x82a!\xA5V[PPPPV[_``\x82\x01\x90Pa\"\x07_\x83\x01\x84a!\xB4V[\x92\x91PPV[a\"\x16\x81a\x1EdV[\x81\x14a\" W__\xFD[PV[_\x815\x90Pa\"1\x81a\"\rV[\x92\x91PPV[___``\x84\x86\x03\x12\x15a\"NWa\"Ma\x1D\x06V[[_a\"[\x86\x82\x87\x01a\x1E%V[\x93PP` a\"l\x86\x82\x87\x01a\"#V[\x92PP`@a\"}\x86\x82\x87\x01a!\x19V[\x91PP\x92P\x92P\x92V[a\"\x90\x81a!kV[\x82RPPV[_` \x82\x01\x90Pa\"\xA9_\x83\x01\x84a\"\x87V[\x92\x91PPV[_``\x82\x01\x90Pa\"\xC2_\x83\x01\x86a\"\x87V[a\"\xCF` \x83\x01\x85a\x1EsV[a\"\xDC`@\x83\x01\x84a\x1EsV[\x94\x93PPPPV[_``\x82\x01\x90Pa\"\xF7_\x83\x01\x85a \xBDV[a#\x04`@\x83\x01\x84a\x1D\xDBV[\x93\x92PPPV[__\xFD[_a\x01\0\x82\x84\x03\x12\x15a#%Wa#$a#\x0BV[[\x81\x90P\x92\x91PPV[_`@\x82\x84\x03\x12\x15a#CWa#Ba#\x0BV[[\x81\x90P\x92\x91PPV[___a\x01`\x84\x86\x03\x12\x15a#dWa#ca\x1D\x06V[[_a#q\x86\x82\x87\x01a\x1DTV[\x93PP` a#\x82\x86\x82\x87\x01a#\x0FV[\x92PPa\x01 a#\x94\x86\x82\x87\x01a#.V[\x91PP\x92P\x92P\x92V[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a#\xBBWa#\xBAa\x1E\x9BV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xD8Wa#\xD7a#\x9EV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a#\xF4Wa#\xF3a#\xA2V[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a$\x12Wa$\x11a\x1D\x06V[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$/Wa$.a\x1D\nV[[a$;\x86\x82\x87\x01a#\xA6V[\x93P\x93PP` a$N\x86\x82\x87\x01a!\x19V[\x91PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a$\x8C\x83\x83a!\xA5V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a$\xAE\x82a$XV[a$\xB8\x81\x85a$bV[\x93Pa$\xC3\x83a$rV[\x80_[\x83\x81\x10\x15a$\xF3W\x81Qa$\xDA\x88\x82a$\x81V[\x97Pa$\xE5\x83a$\x98V[\x92PP`\x01\x81\x01\x90Pa$\xC6V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra%\x18\x81\x84a$\xA4V[\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBLSApkRegistry.initializeQuorum:_\x82\x01R\x7F quorum already exists\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a%\x8A`6\x83a% V[\x91Pa%\x95\x82a%0V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra%\xB7\x81a%~V[\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a%\xF0\x82a%\xBEV[a%\xFA\x81\x85a%\xC8V[\x93Pa&\n\x81\x85` \x86\x01a%\xD8V[a&\x13\x81a\x1E\xA3V[\x84\x01\x91PP\x92\x91PPV[_``\x82\x01\x90Pa&1_\x83\x01\x86a \x86V[a&>` \x83\x01\x85a\x1D\xDBV[\x81\x81\x03`@\x83\x01Ra&P\x81\x84a%\xE6V[\x90P\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FBLSApkRegistry.getApkHashAtBlock_\x82\x01R\x7FNumberAndIndex: index too recent` \x82\x01RPV[_a&\xE1`@\x83a% V[\x91Pa&\xEC\x82a&\x87V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra'\x0E\x81a&\xD5V[\x90P\x91\x90PV[\x7FBLSApkRegistry.getApkHashAtBlock_\x82\x01R\x7FNumberAndIndex: not latest apk u` \x82\x01R\x7Fpdate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a'\x95`E\x83a% V[\x91Pa'\xA0\x82a'\x15V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra'\xC2\x81a'\x89V[\x90P\x91\x90PV[\x7FBLSApkRegistry.getRegisteredPubk_\x82\x01R\x7Fey: operator is not registered\0\0` \x82\x01RPV[_a(#`>\x83a% V[\x91Pa(.\x82a'\xC9V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra(P\x81a(\x17V[\x90P\x91\x90PV[__\xFD[_`@\x82\x84\x03\x12\x15a(pWa(oa(WV[[a(z`@a\x1F\x11V[\x90P_a(\x89\x84\x82\x85\x01a!\x19V[_\x83\x01RP` a(\x9C\x84\x82\x85\x01a!\x19V[` \x83\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a(\xBDWa(\xBCa\x1D\x06V[[_a(\xCA\x84\x82\x85\x01a([V[\x91PP\x92\x91PPV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: cannot register zero pubkey` \x82\x01RPV[_a)-`@\x83a% V[\x91Pa)8\x82a(\xD3V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra)Z\x81a)!V[\x90P\x91\x90PV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: operator already registered` \x82\x01R\x7F pubkey\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a)\xE1`G\x83a% V[\x91Pa)\xEC\x82a)aV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra*\x0E\x81a)\xD5V[\x90P\x91\x90PV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: public key already register` \x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a*\x95`B\x83a% V[\x91Pa*\xA0\x82a*\x15V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra*\xC2\x81a*\x89V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a*\xE3a*\xDE\x82a\x1D\x93V[a*\xC9V[\x82RPPV[\x82\x81\x837PPPV[a*\xFE`@\x83\x83a*\xE9V[PPV[_a+\r\x82\x8Ba*\xD2V[` \x82\x01\x91Pa+\x1D\x82\x8Aa*\xD2V[` \x82\x01\x91Pa+-\x82\x89a*\xD2V[` \x82\x01\x91Pa+=\x82\x88a*\xD2V[` \x82\x01\x91Pa+M\x82\x87a*\xF2V[`@\x82\x01\x91Pa+]\x82\x86a*\xF2V[`@\x82\x01\x91Pa+m\x82\x85a*\xD2V[` \x82\x01\x91Pa+}\x82\x84a*\xD2V[` \x82\x01\x91P\x81\x90P\x99\x98PPPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a+\xCA\x82a\x1D\x93V[\x91Pa+\xD5\x83a\x1D\x93V[\x92P\x82a+\xE5Wa+\xE4a+\x93V[[\x82\x82\x06\x90P\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a,\nWa,\ta\x1E\xB3V[[` \x82\x02\x90P\x91\x90PV[_a,'a,\"\x84a+\xF0V[a\x1F\x11V[\x90P\x80` \x84\x02\x83\x01\x85\x81\x11\x15a,AWa,@a#\xA2V[[\x83[\x81\x81\x10\x15a,jW\x80a,V\x88\x82a!\x19V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa,CV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a,\x88Wa,\x87a\x1E\x9BV[[`\x02a,\x95\x84\x82\x85a,\x15V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a,\xB3Wa,\xB2a(WV[[a,\xBD`@a\x1F\x11V[\x90P_a,\xCC\x84\x82\x85\x01a,tV[_\x83\x01RP`@a,\xDF\x84\x82\x85\x01a,tV[` \x83\x01RP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a-\0Wa,\xFFa\x1D\x06V[[_a-\r\x84\x82\x85\x01a,\x9EV[\x91PP\x92\x91PPV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: either the G1 signature is ` \x82\x01R\x7Fwrong, or G1 and G2 private key `@\x82\x01R\x7Fdo not match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_a-\xBC`l\x83a% V[\x91Pa-\xC7\x82a-\x16V[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra-\xE9\x81a-\xB0V[\x90P\x91\x90PV[_\x815a-\xFC\x81a!\x03V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa.;\x84a.\x05V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a.ta.oa.j\x84a\x1D\x93V[a.QV[a\x1D\x93V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a.\x8D\x82a.ZV[a.\xA0a.\x99\x82a.{V[\x83Ta.\x10V[\x82UPPPV[_\x81\x01_\x83\x01\x80a.\xB7\x81a-\xF0V[\x90Pa.\xC3\x81\x84a.\x84V[PPP`\x01\x81\x01` \x83\x01\x80a.\xD8\x81a-\xF0V[\x90Pa.\xE4\x81\x84a.\x84V[PPPPPV[a.\xF5\x82\x82a.\xA7V[PPV[_a/\x07` \x84\x01\x84a!\x19V[\x90P\x92\x91PPV[`@\x82\x01a/\x1F_\x83\x01\x83a.\xF9V[a/+_\x85\x01\x82a \xAEV[Pa/9` \x83\x01\x83a.\xF9V[a/F` \x85\x01\x82a \xAEV[PPPPV[_\x82\x90P\x92\x91PPV[a/b`@\x83\x83a*\xE9V[PPV[`\x80\x82\x01a/v_\x83\x01\x83a/LV[a/\x82_\x85\x01\x82a/VV[Pa/\x90`@\x83\x01\x83a/LV[a/\x9D`@\x85\x01\x82a/VV[PPPPV[_`\xC0\x82\x01\x90Pa/\xB6_\x83\x01\x85a/\x0FV[a/\xC3`@\x83\x01\x84a/fV[\x93\x92PPPV[\x7FBLSApkRegistry.getApkIndicesAtBl_\x82\x01R\x7FockNumber: blockNumber is before` \x82\x01R\x7F the first update\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a0J`Q\x83a% V[\x91Pa0U\x82a/\xCAV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra0w\x81a0>V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a0\xB5\x82a\x1D\x93V[\x91Pa0\xC0\x83a\x1D\x93V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a0\xD8Wa0\xD7a0~V[[\x92\x91PPV[_a0\xE8\x82a\x1D\x93V[\x91P_\x82\x03a0\xFAWa0\xF9a0~V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FBLSApkRegistry._checkRegistryCoo_\x82\x01R\x7Frdinator: caller is not the regi` \x82\x01R\x7Fstry coordinator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a1\x85`P\x83a% V[\x91Pa1\x90\x82a1\x05V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra1\xB2\x81a1yV[\x90P\x91\x90PV[\x7FBLSApkRegistry._processQuorumApk_\x82\x01R\x7FUpdate: quorum does not exist\0\0\0` \x82\x01RPV[_a2\x13`=\x83a% V[\x91Pa2\x1E\x82a1\xB9V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra2@\x81a2\x07V[\x90P\x91\x90PV[\x7Fec-mul-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a2{`\r\x83a% V[\x91Pa2\x86\x82a2GV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra2\xA8\x81a2oV[\x90P\x91\x90PV[\x7Fec-add-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a2\xE3`\r\x83a% V[\x91Pa2\xEE\x82a2\xAFV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3\x10\x81a2\xD7V[\x90P\x91\x90PV[_a3!\x82a\x1D\x93V[\x91Pa3,\x83a\x1D\x93V[\x92P\x82\x82\x02a3:\x81a\x1D\x93V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a3QWa3Pa0~V[[P\x92\x91PPV[_a3b\x82a\x1D\x93V[\x91Pa3m\x83a\x1D\x93V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a3\x85Wa3\x84a0~V[[\x92\x91PPV[\x7Fpairing-opcode-failed\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a3\xBF`\x15\x83a% V[\x91Pa3\xCA\x82a3\x8BV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3\xEC\x81a3\xB3V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x122\xBDs#\xC7\xC8g\x1F\xF9\xA4\xDB\xF3\xEB\xA5E\xD7\xF2\xDDx\xE3\x95\xCA\x0B\xAE\x11\x18\x05j\x96j\x1AdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610113575f3560e01c80636d14a987116100a0578063bf79ce581161006f578063bf79ce5814610352578063d5254a8c14610382578063de29fac0146103b2578063e8bb9ae6146103e2578063f4e24fe51461041257610113565b80636d14a987146102a05780637916cea6146102be5780637ff81a87146102f0578063a3db80e21461032157610113565b80633fb27952116100e75780633fb27952146101c457806347b314e8146101e05780635f61a88414610210578063605747d51461024057806368bccaac1461027057610113565b8062a1f4cb1461011757806313542a4e1461014857806326d941f214610178578063377ed99d14610194575b5f5ffd5b610131600480360381019061012c9190611d68565b61042e565b60405161013f929190611dab565b60405180910390f35b610162600480360381019061015d9190611d68565b61044e565b60405161016f9190611dea565b60405180910390f35b610192600480360381019061018d9190611e39565b610494565b005b6101ae60048036038101906101a99190611e39565b6105ec565b6040516101bb9190611e82565b60405180910390f35b6101de60048036038101906101d99190611fd7565b61060f565b005b6101fa60048036038101906101f5919061205b565b610676565b6040516102079190612095565b60405180910390f35b61022a60048036038101906102259190611e39565b6106af565b60405161023791906120ea565b60405180910390f35b61025a6004803603810190610255919061212d565b6106f3565b60405161026791906121f4565b60405180910390f35b61028a60048036038101906102859190612237565b6107b4565b6040516102979190612296565b60405180910390f35b6102a8610930565b6040516102b59190612095565b60405180910390f35b6102d860048036038101906102d3919061212d565b610954565b6040516102e7939291906122af565b60405180910390f35b61030a60048036038101906103059190611d68565b6109ba565b6040516103189291906122e4565b60405180910390f35b61033b60048036038101906103369190611e39565b610ab3565b604051610349929190611dab565b60405180910390f35b61036c6004803603810190610367919061234c565b610ad3565b6040516103799190611dea565b60405180910390f35b61039c600480360381019061039791906123fb565b610f37565b6040516103a99190612500565b60405180910390f35b6103cc60048036038101906103c79190611d68565b611146565b6040516103d99190611dea565b60405180910390f35b6103fc60048036038101906103f7919061205b565b61115b565b6040516104099190612095565b60405180910390f35b61042c60048036038101906104279190611fd7565b61118b565b005b6003602052805f5260405f205f91509050805f0154908060010154905082565b5f60015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20549050919050565b61049c6111fa565b5f60045f8360ff1660ff1681526020019081526020015f2080549050146104f8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104ef906125a0565b60405180910390fd5b60045f8260ff1660ff1681526020019081526020015f2060405180606001604052805f60401b67ffffffffffffffff191681526020014363ffffffff1681526020015f63ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c02179055506020820151815f0160186101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f01601c6101000a81548163ffffffff021916908363ffffffff160217905550505050565b5f60045f8360ff1660ff1681526020019081526020015f20805490509050919050565b6106176111fa565b5f610621836109ba565b50905061062e828261128a565b7f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e836106598561044e565b846040516106699392919061261e565b60405180910390a1505050565b5f60025f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050919050565b6106b7611bdf565b60055f8360ff1660ff1681526020019081526020015f206040518060400160405290815f82015481526020016001820154815250509050919050565b6106fb611bf7565b60045f8460ff1660ff1681526020019081526020015f2082815481106107245761072361265a565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900460401b67ffffffffffffffff191667ffffffffffffffff191681526020015f820160189054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f8201601c9054906101000a900463ffffffff1663ffffffff1663ffffffff1681525050905092915050565b5f5f60045f8660ff1660ff1681526020019081526020015f2083815481106107df576107de61265a565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900460401b67ffffffffffffffff191667ffffffffffffffff191681526020015f820160189054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f8201601c9054906101000a900463ffffffff1663ffffffff1663ffffffff16815250509050806020015163ffffffff168463ffffffff1610156108bc576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016108b3906126f7565b60405180910390fd5b5f816040015163ffffffff1614806108e35750806040015163ffffffff168463ffffffff16105b610922576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610919906127ab565b60405180910390fd5b805f01519150509392505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b6004602052815f5260405f20818154811061096d575f80fd5b905f5260205f20015f9150915050805f015f9054906101000a900460401b90805f0160189054906101000a900463ffffffff1690805f01601c9054906101000a900463ffffffff16905083565b6109c2611bdf565b5f5f60035f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060400160405290815f820154815260200160018201548152505090505f60015f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205490505f5f1b8103610aa6576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a9d90612839565b60405180910390fd5b8181935093505050915091565b6005602052805f5260405f205f91509050805f0154908060010154905082565b5f610adc6111fa565b5f610af984604001803603810190610af491906128a8565b61156b565b90507fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb58103610b5d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610b5490612943565b60405180910390fd5b5f5f1b60015f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205414610bde576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610bd5906129f7565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff1660025f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614610c7c576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610c7390612aab565b60405180910390fd5b5f7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001855f015f0135865f0160200135876040015f01358860400160200135896080015f018a6080016040018a5f01358b60200135604051602001610ce7989796959493929190612b02565b604051602081830303815290604052805190602001205f1c610d099190612bc0565b9050610db5610d58610d378388604001803603810190610d2991906128a8565b61158390919063ffffffff16565b875f01803603810190610d4a91906128a8565b61165790919063ffffffff16565b610d60611750565b610d9c610d7d85610d6f61181a565b61158390919063ffffffff16565b88803603810190610d8e91906128a8565b61165790919063ffffffff16565b88608001803603810190610db09190612ceb565b61183e565b610df4576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610deb90612dd2565b60405180910390fd5b8460400160035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181610e419190612eeb565b9050508160015f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508560025f8481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508573ffffffffffffffffffffffffffffffffffffffff167fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba38280418660400187608001604051610f23929190612fa3565b60405180910390a281925050509392505050565b60605f8484905067ffffffffffffffff811115610f5757610f56611eb3565b5b604051908082528060200260200182016040528015610f855781602001602082028036833780820191505090505b5090505f5f90505b8585905081101561113a575f868683818110610fac57610fab61265a565b5b9050013560f81c60f81b60f81c90505f60045f8360ff1660ff1681526020019081526020015f208054905090505f81148061102f575060045f8360ff1660ff1681526020019081526020015f205f8154811061100b5761100a61265a565b5b905f5260205f20015f0160189054906101000a900463ffffffff1663ffffffff1686105b1561106f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161106690613060565b60405180910390fd5b5f8190505b5f81111561112a578660045f8560ff1660ff1681526020019081526020015f206001836110a191906130ab565b815481106110b2576110b161265a565b5b905f5260205f20015f0160189054906101000a900463ffffffff1663ffffffff1611611117576001816110e591906130ab565b8585815181106110f8576110f761265a565b5b602002602001019063ffffffff16908163ffffffff168152505061112a565b8080611122906130de565b915050611074565b5050508080600101915050610f8d565b50809150509392505050565b6001602052805f5260405f205f915090505481565b6002602052805f5260405f205f915054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6111936111fa565b5f61119d836109ba565b5090506111b2826111ad83611b27565b61128a565b7ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e836111dd8561044e565b846040516111ed9392919061261e565b60405180910390a1505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611288576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161127f9061319b565b60405180910390fd5b565b611292611bdf565b5f5f90505b8351811015611565575f8482815181106112b4576112b361265a565b5b602001015160f81c60f81b60f81c90505f60045f8360ff1660ff1681526020019081526020015f208054905090505f8103611324576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161131b90613229565b60405180910390fd5b61136b8560055f8560ff1660ff1681526020019081526020015f206040518060400160405290815f820154815260200160018201548152505061165790919063ffffffff16565b93508360055f8460ff1660ff1681526020019081526020015f205f820151815f0155602082015181600101559050505f6113a48561156b565b90505f60045f8560ff1660ff1681526020019081526020015f206001846113cb91906130ab565b815481106113dc576113db61265a565b5b905f5260205f200190504363ffffffff16815f0160189054906101000a900463ffffffff1663ffffffff16036114435781815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c0217905550611554565b43815f01601c6101000a81548163ffffffff021916908363ffffffff16021790555060045f8560ff1660ff1681526020019081526020015f2060405180606001604052808467ffffffffffffffff191681526020014363ffffffff1681526020015f63ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c02179055506020820151815f0160186101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f01601c6101000a81548163ffffffff021916908363ffffffff16021790555050505b505050508080600101915050611297565b50505050565b5f81515f52816020015160205260405f209050919050565b61158b611bdf565b611593611c2c565b835f0151815f600381106115aa576115a961265a565b5b6020020181815250508360200151816001600381106115cc576115cb61265a565b5b60200201818152505082816002600381106115ea576115e961265a565b5b6020020181815250505f60408360608460076107d05a03fa9050805f810361160e57fe5b508061164f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161164690613291565b60405180910390fd5b505092915050565b61165f611bdf565b611667611c4e565b835f0151815f6004811061167e5761167d61265a565b5b6020020181815250508360200151816001600481106116a05761169f61265a565b5b602002018181525050825f0151816002600481106116c1576116c061265a565b5b6020020181815250508260200151816003600481106116e3576116e261265a565b5b6020020181815250505f60408360808460066107d05a03fa9050805f810361170757fe5b5080611748576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161173f906132f9565b60405180910390fd5b505092915050565b611758611c70565b604051806040016040528060405180604001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed815250815260200160405180604001604052807f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec81526020017f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d815250815250905090565b611822611bdf565b6040518060400160405280600181526020016002815250905090565b5f5f60405180604001604052808781526020018581525090505f6040518060400160405280878152602001858152509050611877611c96565b5f5f90505b6002811015611a95575f6006826118939190613317565b90508482600281106118a8576118a761265a565b5b60200201515f0151835f836118bd9190613358565b600c81106118ce576118cd61265a565b5b6020020181815250508482600281106118ea576118e961265a565b5b602002015160200151836001836119019190613358565b600c81106119125761191161265a565b5b60200201818152505083826002811061192e5761192d61265a565b5b60200201515f01515f600281106119485761194761265a565b5b60200201518360028361195b9190613358565b600c811061196c5761196b61265a565b5b6020020181815250508382600281106119885761198761265a565b5b60200201515f01516001600281106119a3576119a261265a565b5b6020020151836003836119b69190613358565b600c81106119c7576119c661265a565b5b6020020181815250508382600281106119e3576119e261265a565b5b6020020151602001515f600281106119fe576119fd61265a565b5b602002015183600483611a119190613358565b600c8110611a2257611a2161265a565b5b602002018181525050838260028110611a3e57611a3d61265a565b5b602002015160200151600160028110611a5a57611a5961265a565b5b602002015183600583611a6d9190613358565b600c8110611a7e57611a7d61265a565b5b60200201818152505050808060010191505061187c565b50611a9e611cb9565b5f6020826020600c028560086107d05a03fa9050805f8103611abc57fe5b5080611afd576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611af4906133d5565b60405180910390fd5b5f825f60018110611b1157611b1061265a565b5b6020020151141595505050505050949350505050565b611b2f611bdf565b5f825f0151148015611b4457505f8260200151145b15611b655760405180604001604052805f81526020015f8152509050611bda565b6040518060400160405280835f015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478460200151611ba99190612bc0565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611bd491906130ab565b81525090505b919050565b60405180604001604052805f81526020015f81525090565b60405180606001604052805f67ffffffffffffffff191681526020015f63ffffffff1681526020015f63ffffffff1681525090565b6040518060600160405280600390602082028036833780820191505090505090565b6040518060800160405280600490602082028036833780820191505090505090565b6040518060400160405280611c83611cdb565b8152602001611c90611cdb565b81525090565b604051806101800160405280600c90602082028036833780820191505090505090565b6040518060200160405280600190602082028036833780820191505090505090565b6040518060400160405280600290602082028036833780820191505090505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f611d3782611d0e565b9050919050565b611d4781611d2d565b8114611d51575f5ffd5b50565b5f81359050611d6281611d3e565b92915050565b5f60208284031215611d7d57611d7c611d06565b5b5f611d8a84828501611d54565b91505092915050565b5f819050919050565b611da581611d93565b82525050565b5f604082019050611dbe5f830185611d9c565b611dcb6020830184611d9c565b9392505050565b5f819050919050565b611de481611dd2565b82525050565b5f602082019050611dfd5f830184611ddb565b92915050565b5f60ff82169050919050565b611e1881611e03565b8114611e22575f5ffd5b50565b5f81359050611e3381611e0f565b92915050565b5f60208284031215611e4e57611e4d611d06565b5b5f611e5b84828501611e25565b91505092915050565b5f63ffffffff82169050919050565b611e7c81611e64565b82525050565b5f602082019050611e955f830184611e73565b92915050565b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b611ee982611ea3565b810181811067ffffffffffffffff82111715611f0857611f07611eb3565b5b80604052505050565b5f611f1a611cfd565b9050611f268282611ee0565b919050565b5f67ffffffffffffffff821115611f4557611f44611eb3565b5b611f4e82611ea3565b9050602081019050919050565b828183375f83830152505050565b5f611f7b611f7684611f2b565b611f11565b905082815260208101848484011115611f9757611f96611e9f565b5b611fa2848285611f5b565b509392505050565b5f82601f830112611fbe57611fbd611e9b565b5b8135611fce848260208601611f69565b91505092915050565b5f5f60408385031215611fed57611fec611d06565b5b5f611ffa85828601611d54565b925050602083013567ffffffffffffffff81111561201b5761201a611d0a565b5b61202785828601611faa565b9150509250929050565b61203a81611dd2565b8114612044575f5ffd5b50565b5f8135905061205581612031565b92915050565b5f602082840312156120705761206f611d06565b5b5f61207d84828501612047565b91505092915050565b61208f81611d2d565b82525050565b5f6020820190506120a85f830184612086565b92915050565b6120b781611d93565b82525050565b604082015f8201516120d15f8501826120ae565b5060208201516120e460208501826120ae565b50505050565b5f6040820190506120fd5f8301846120bd565b92915050565b61210c81611d93565b8114612116575f5ffd5b50565b5f8135905061212781612103565b92915050565b5f5f6040838503121561214357612142611d06565b5b5f61215085828601611e25565b925050602061216185828601612119565b9150509250929050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000082169050919050565b61219f8161216b565b82525050565b6121ae81611e64565b82525050565b606082015f8201516121c85f850182612196565b5060208201516121db60208501826121a5565b5060408201516121ee60408501826121a5565b50505050565b5f6060820190506122075f8301846121b4565b92915050565b61221681611e64565b8114612220575f5ffd5b50565b5f813590506122318161220d565b92915050565b5f5f5f6060848603121561224e5761224d611d06565b5b5f61225b86828701611e25565b935050602061226c86828701612223565b925050604061227d86828701612119565b9150509250925092565b6122908161216b565b82525050565b5f6020820190506122a95f830184612287565b92915050565b5f6060820190506122c25f830186612287565b6122cf6020830185611e73565b6122dc6040830184611e73565b949350505050565b5f6060820190506122f75f8301856120bd565b6123046040830184611ddb565b9392505050565b5f5ffd5b5f61010082840312156123255761232461230b565b5b81905092915050565b5f604082840312156123435761234261230b565b5b81905092915050565b5f5f5f610160848603121561236457612363611d06565b5b5f61237186828701611d54565b93505060206123828682870161230f565b9250506101206123948682870161232e565b9150509250925092565b5f5ffd5b5f5ffd5b5f5f83601f8401126123bb576123ba611e9b565b5b8235905067ffffffffffffffff8111156123d8576123d761239e565b5b6020830191508360018202830111156123f4576123f36123a2565b5b9250929050565b5f5f5f6040848603121561241257612411611d06565b5b5f84013567ffffffffffffffff81111561242f5761242e611d0a565b5b61243b868287016123a6565b9350935050602061244e86828701612119565b9150509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f61248c83836121a5565b60208301905092915050565b5f602082019050919050565b5f6124ae82612458565b6124b88185612462565b93506124c383612472565b805f5b838110156124f35781516124da8882612481565b97506124e583612498565b9250506001810190506124c6565b5085935050505092915050565b5f6020820190508181035f83015261251881846124a4565b905092915050565b5f82825260208201905092915050565b7f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a5f8201527f2071756f72756d20616c72656164792065786973747300000000000000000000602082015250565b5f61258a603683612520565b915061259582612530565b604082019050919050565b5f6020820190508181035f8301526125b78161257e565b9050919050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f6125f0826125be565b6125fa81856125c8565b935061260a8185602086016125d8565b61261381611ea3565b840191505092915050565b5f6060820190506126315f830186612086565b61263e6020830185611ddb565b818103604083015261265081846125e6565b9050949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f424c5341706b52656769737472792e67657441706b486173684174426c6f636b5f8201527f4e756d626572416e64496e6465783a20696e64657820746f6f20726563656e74602082015250565b5f6126e1604083612520565b91506126ec82612687565b604082019050919050565b5f6020820190508181035f83015261270e816126d5565b9050919050565b7f424c5341706b52656769737472792e67657441706b486173684174426c6f636b5f8201527f4e756d626572416e64496e6465783a206e6f74206c61746573742061706b207560208201527f7064617465000000000000000000000000000000000000000000000000000000604082015250565b5f612795604583612520565b91506127a082612715565b606082019050919050565b5f6020820190508181035f8301526127c281612789565b9050919050565b7f424c5341706b52656769737472792e676574526567697374657265645075626b5f8201527f65793a206f70657261746f72206973206e6f7420726567697374657265640000602082015250565b5f612823603e83612520565b915061282e826127c9565b604082019050919050565b5f6020820190508181035f83015261285081612817565b9050919050565b5f5ffd5b5f604082840312156128705761286f612857565b5b61287a6040611f11565b90505f61288984828501612119565b5f83015250602061289c84828501612119565b60208301525092915050565b5f604082840312156128bd576128bc611d06565b5b5f6128ca8482850161285b565b91505092915050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b6579602082015250565b5f61292d604083612520565b9150612938826128d3565b604082019050919050565b5f6020820190508181035f83015261295a81612921565b9050919050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a206f70657261746f7220616c7265616479207265676973746572656460208201527f207075626b657900000000000000000000000000000000000000000000000000604082015250565b5f6129e1604783612520565b91506129ec82612961565b606082019050919050565b5f6020820190508181035f830152612a0e816129d5565b9050919050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a207075626c6963206b657920616c726561647920726567697374657260208201527f6564000000000000000000000000000000000000000000000000000000000000604082015250565b5f612a95604283612520565b9150612aa082612a15565b606082019050919050565b5f6020820190508181035f830152612ac281612a89565b9050919050565b5f819050919050565b612ae3612ade82611d93565b612ac9565b82525050565b82818337505050565b612afe60408383612ae9565b5050565b5f612b0d828b612ad2565b602082019150612b1d828a612ad2565b602082019150612b2d8289612ad2565b602082019150612b3d8288612ad2565b602082019150612b4d8287612af2565b604082019150612b5d8286612af2565b604082019150612b6d8285612ad2565b602082019150612b7d8284612ad2565b6020820191508190509998505050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f612bca82611d93565b9150612bd583611d93565b925082612be557612be4612b93565b5b828206905092915050565b5f67ffffffffffffffff821115612c0a57612c09611eb3565b5b602082029050919050565b5f612c27612c2284612bf0565b611f11565b90508060208402830185811115612c4157612c406123a2565b5b835b81811015612c6a5780612c568882612119565b845260208401935050602081019050612c43565b5050509392505050565b5f82601f830112612c8857612c87611e9b565b5b6002612c95848285612c15565b91505092915050565b5f60808284031215612cb357612cb2612857565b5b612cbd6040611f11565b90505f612ccc84828501612c74565b5f830152506040612cdf84828501612c74565b60208301525092915050565b5f60808284031215612d0057612cff611d06565b5b5f612d0d84828501612c9e565b91505092915050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a2065697468657220746865204731207369676e61747572652069732060208201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060408201527f646f206e6f74206d617463680000000000000000000000000000000000000000606082015250565b5f612dbc606c83612520565b9150612dc782612d16565b608082019050919050565b5f6020820190508181035f830152612de981612db0565b9050919050565b5f8135612dfc81612103565b80915050919050565b5f815f1b9050919050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff612e3b84612e05565b9350801983169250808416831791505092915050565b5f819050919050565b5f612e74612e6f612e6a84611d93565b612e51565b611d93565b9050919050565b5f819050919050565b612e8d82612e5a565b612ea0612e9982612e7b565b8354612e10565b8255505050565b5f81015f830180612eb781612df0565b9050612ec38184612e84565b505050600181016020830180612ed881612df0565b9050612ee48184612e84565b5050505050565b612ef58282612ea7565b5050565b5f612f076020840184612119565b905092915050565b60408201612f1f5f830183612ef9565b612f2b5f8501826120ae565b50612f396020830183612ef9565b612f4660208501826120ae565b50505050565b5f82905092915050565b612f6260408383612ae9565b5050565b60808201612f765f830183612f4c565b612f825f850182612f56565b50612f906040830183612f4c565b612f9d6040850182612f56565b50505050565b5f60c082019050612fb65f830185612f0f565b612fc36040830184612f66565b9392505050565b7f424c5341706b52656769737472792e67657441706b496e64696365734174426c5f8201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f726560208201527f2074686520666972737420757064617465000000000000000000000000000000604082015250565b5f61304a605183612520565b915061305582612fca565b606082019050919050565b5f6020820190508181035f8301526130778161303e565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6130b582611d93565b91506130c083611d93565b92508282039050818111156130d8576130d761307e565b5b92915050565b5f6130e882611d93565b91505f82036130fa576130f961307e565b5b600182039050919050565b7f424c5341706b52656769737472792e5f636865636b5265676973747279436f6f5f8201527f7264696e61746f723a2063616c6c6572206973206e6f7420746865207265676960208201527f7374727920636f6f7264696e61746f7200000000000000000000000000000000604082015250565b5f613185605083612520565b915061319082613105565b606082019050919050565b5f6020820190508181035f8301526131b281613179565b9050919050565b7f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b5f8201527f5570646174653a2071756f72756d20646f6573206e6f74206578697374000000602082015250565b5f613213603d83612520565b915061321e826131b9565b604082019050919050565b5f6020820190508181035f83015261324081613207565b9050919050565b7f65632d6d756c2d6661696c6564000000000000000000000000000000000000005f82015250565b5f61327b600d83612520565b915061328682613247565b602082019050919050565b5f6020820190508181035f8301526132a88161326f565b9050919050565b7f65632d6164642d6661696c6564000000000000000000000000000000000000005f82015250565b5f6132e3600d83612520565b91506132ee826132af565b602082019050919050565b5f6020820190508181035f830152613310816132d7565b9050919050565b5f61332182611d93565b915061332c83611d93565b925082820261333a81611d93565b915082820484148315176133515761335061307e565b5b5092915050565b5f61336282611d93565b915061336d83611d93565b92508282019050808211156133855761338461307e565b5b92915050565b7f70616972696e672d6f70636f64652d6661696c656400000000000000000000005f82015250565b5f6133bf601583612520565b91506133ca8261338b565b602082019050919050565b5f6020820190508181035f8301526133ec816133b3565b905091905056fea26469706673582212201232bd7323c7c8671ff9a4dbf3eba545d7f2dd78e395ca0bae1118056a966a1a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x13W_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\xA0W\x80c\xBFy\xCEX\x11a\0oW\x80c\xBFy\xCEX\x14a\x03RW\x80c\xD5%J\x8C\x14a\x03\x82W\x80c\xDE)\xFA\xC0\x14a\x03\xB2W\x80c\xE8\xBB\x9A\xE6\x14a\x03\xE2W\x80c\xF4\xE2O\xE5\x14a\x04\x12Wa\x01\x13V[\x80cm\x14\xA9\x87\x14a\x02\xA0W\x80cy\x16\xCE\xA6\x14a\x02\xBEW\x80c\x7F\xF8\x1A\x87\x14a\x02\xF0W\x80c\xA3\xDB\x80\xE2\x14a\x03!Wa\x01\x13V[\x80c?\xB2yR\x11a\0\xE7W\x80c?\xB2yR\x14a\x01\xC4W\x80cG\xB3\x14\xE8\x14a\x01\xE0W\x80c_a\xA8\x84\x14a\x02\x10W\x80c`WG\xD5\x14a\x02@W\x80ch\xBC\xCA\xAC\x14a\x02pWa\x01\x13V[\x80b\xA1\xF4\xCB\x14a\x01\x17W\x80c\x13T*N\x14a\x01HW\x80c&\xD9A\xF2\x14a\x01xW\x80c7~\xD9\x9D\x14a\x01\x94W[__\xFD[a\x011`\x04\x806\x03\x81\x01\x90a\x01,\x91\x90a\x1DhV[a\x04.V[`@Qa\x01?\x92\x91\x90a\x1D\xABV[`@Q\x80\x91\x03\x90\xF3[a\x01b`\x04\x806\x03\x81\x01\x90a\x01]\x91\x90a\x1DhV[a\x04NV[`@Qa\x01o\x91\x90a\x1D\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x01\x92`\x04\x806\x03\x81\x01\x90a\x01\x8D\x91\x90a\x1E9V[a\x04\x94V[\0[a\x01\xAE`\x04\x806\x03\x81\x01\x90a\x01\xA9\x91\x90a\x1E9V[a\x05\xECV[`@Qa\x01\xBB\x91\x90a\x1E\x82V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDE`\x04\x806\x03\x81\x01\x90a\x01\xD9\x91\x90a\x1F\xD7V[a\x06\x0FV[\0[a\x01\xFA`\x04\x806\x03\x81\x01\x90a\x01\xF5\x91\x90a [V[a\x06vV[`@Qa\x02\x07\x91\x90a \x95V[`@Q\x80\x91\x03\x90\xF3[a\x02*`\x04\x806\x03\x81\x01\x90a\x02%\x91\x90a\x1E9V[a\x06\xAFV[`@Qa\x027\x91\x90a \xEAV[`@Q\x80\x91\x03\x90\xF3[a\x02Z`\x04\x806\x03\x81\x01\x90a\x02U\x91\x90a!-V[a\x06\xF3V[`@Qa\x02g\x91\x90a!\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x02\x8A`\x04\x806\x03\x81\x01\x90a\x02\x85\x91\x90a\"7V[a\x07\xB4V[`@Qa\x02\x97\x91\x90a\"\x96V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA8a\t0V[`@Qa\x02\xB5\x91\x90a \x95V[`@Q\x80\x91\x03\x90\xF3[a\x02\xD8`\x04\x806\x03\x81\x01\x90a\x02\xD3\x91\x90a!-V[a\tTV[`@Qa\x02\xE7\x93\x92\x91\x90a\"\xAFV[`@Q\x80\x91\x03\x90\xF3[a\x03\n`\x04\x806\x03\x81\x01\x90a\x03\x05\x91\x90a\x1DhV[a\t\xBAV[`@Qa\x03\x18\x92\x91\x90a\"\xE4V[`@Q\x80\x91\x03\x90\xF3[a\x03;`\x04\x806\x03\x81\x01\x90a\x036\x91\x90a\x1E9V[a\n\xB3V[`@Qa\x03I\x92\x91\x90a\x1D\xABV[`@Q\x80\x91\x03\x90\xF3[a\x03l`\x04\x806\x03\x81\x01\x90a\x03g\x91\x90a#LV[a\n\xD3V[`@Qa\x03y\x91\x90a\x1D\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x03\x9C`\x04\x806\x03\x81\x01\x90a\x03\x97\x91\x90a#\xFBV[a\x0F7V[`@Qa\x03\xA9\x91\x90a%\0V[`@Q\x80\x91\x03\x90\xF3[a\x03\xCC`\x04\x806\x03\x81\x01\x90a\x03\xC7\x91\x90a\x1DhV[a\x11FV[`@Qa\x03\xD9\x91\x90a\x1D\xEAV[`@Q\x80\x91\x03\x90\xF3[a\x03\xFC`\x04\x806\x03\x81\x01\x90a\x03\xF7\x91\x90a [V[a\x11[V[`@Qa\x04\t\x91\x90a \x95V[`@Q\x80\x91\x03\x90\xF3[a\x04,`\x04\x806\x03\x81\x01\x90a\x04'\x91\x90a\x1F\xD7V[a\x11\x8BV[\0[`\x03` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01T\x90P\x82V[_`\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[a\x04\x9Ca\x11\xFAV[_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14a\x04\xF8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xEF\x90a%\xA0V[`@Q\x80\x91\x03\x90\xFD[`\x04_\x82`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80_`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[a\x06\x17a\x11\xFAV[_a\x06!\x83a\t\xBAV[P\x90Pa\x06.\x82\x82a\x12\x8AV[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06Y\x85a\x04NV[\x84`@Qa\x06i\x93\x92\x91\x90a&\x1EV[`@Q\x80\x91\x03\x90\xA1PPPV[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[a\x06\xB7a\x1B\xDFV[`\x05_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[a\x06\xFBa\x1B\xF7V[`\x04_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x07$Wa\x07#a&ZV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[__`\x04_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x07\xDFWa\x07\xDEa&ZV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10\x15a\x08\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xB3\x90a&\xF7V[`@Q\x80\x91\x03\x90\xFD[_\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x14\x80a\x08\xE3WP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\t\"W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x19\x90a'\xABV[`@Q\x80\x91\x03\x90\xFD[\x80_\x01Q\x91PP\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\tmW_\x80\xFD[\x90_R` _ \x01_\x91P\x91PP\x80_\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1B\x90\x80_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x83V[a\t\xC2a\x1B\xDFV[__`\x03_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P_`\x01_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P__\x1B\x81\x03a\n\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x9D\x90a(9V[`@Q\x80\x91\x03\x90\xFD[\x81\x81\x93P\x93PPP\x91P\x91V[`\x05` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01T\x90P\x82V[_a\n\xDCa\x11\xFAV[_a\n\xF9\x84`@\x01\x806\x03\x81\x01\x90a\n\xF4\x91\x90a(\xA8V[a\x15kV[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x03a\x0B]W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0BT\x90a)CV[`@Q\x80\x91\x03\x90\xFD[__\x1B`\x01_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x14a\x0B\xDEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xD5\x90a)\xF7V[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C|W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Cs\x90a*\xABV[`@Q\x80\x91\x03\x90\xFD[_\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x85_\x01_\x015\x86_\x01` \x015\x87`@\x01_\x015\x88`@\x01` \x015\x89`\x80\x01_\x01\x8A`\x80\x01`@\x01\x8A_\x015\x8B` \x015`@Q` \x01a\x0C\xE7\x98\x97\x96\x95\x94\x93\x92\x91\x90a+\x02V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\r\t\x91\x90a+\xC0V[\x90Pa\r\xB5a\rXa\r7\x83\x88`@\x01\x806\x03\x81\x01\x90a\r)\x91\x90a(\xA8V[a\x15\x83\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87_\x01\x806\x03\x81\x01\x90a\rJ\x91\x90a(\xA8V[a\x16W\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r`a\x17PV[a\r\x9Ca\r}\x85a\roa\x18\x1AV[a\x15\x83\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88\x806\x03\x81\x01\x90a\r\x8E\x91\x90a(\xA8V[a\x16W\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\x80\x01\x806\x03\x81\x01\x90a\r\xB0\x91\x90a,\xEBV[a\x18>V[a\r\xF4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xEB\x90a-\xD2V[`@Q\x80\x91\x03\x90\xFD[\x84`@\x01`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a\x0EA\x91\x90a.\xEBV[\x90PP\x81`\x01_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x85`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x86`@\x01\x87`\x80\x01`@Qa\x0F#\x92\x91\x90a/\xA3V[`@Q\x80\x91\x03\x90\xA2\x81\x92PPP\x93\x92PPPV[``_\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FWWa\x0FVa\x1E\xB3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x85W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x85\x85\x90P\x81\x10\x15a\x11:W_\x86\x86\x83\x81\x81\x10a\x0F\xACWa\x0F\xABa&ZV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x14\x80a\x10/WP`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x81T\x81\x10a\x10\x0BWa\x10\na&ZV[[\x90_R` _ \x01_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x10oW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10f\x90a0`V[`@Q\x80\x91\x03\x90\xFD[_\x81\x90P[_\x81\x11\x15a\x11*W\x86`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a\x10\xA1\x91\x90a0\xABV[\x81T\x81\x10a\x10\xB2Wa\x10\xB1a&ZV[[\x90_R` _ \x01_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a\x11\x17W`\x01\x81a\x10\xE5\x91\x90a0\xABV[\x85\x85\x81Q\x81\x10a\x10\xF8Wa\x10\xF7a&ZV[[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x11*V[\x80\x80a\x11\"\x90a0\xDEV[\x91PPa\x10tV[PPP\x80\x80`\x01\x01\x91PPa\x0F\x8DV[P\x80\x91PP\x93\x92PPPV[`\x01` R\x80_R`@_ _\x91P\x90PT\x81V[`\x02` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x11\x93a\x11\xFAV[_a\x11\x9D\x83a\t\xBAV[P\x90Pa\x11\xB2\x82a\x11\xAD\x83a\x1B'V[a\x12\x8AV[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x11\xDD\x85a\x04NV[\x84`@Qa\x11\xED\x93\x92\x91\x90a&\x1EV[`@Q\x80\x91\x03\x90\xA1PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x12\x88W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\x7F\x90a1\x9BV[`@Q\x80\x91\x03\x90\xFD[V[a\x12\x92a\x1B\xDFV[__\x90P[\x83Q\x81\x10\x15a\x15eW_\x84\x82\x81Q\x81\x10a\x12\xB4Wa\x12\xB3a&ZV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a\x13$W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\x1B\x90a2)V[`@Q\x80\x91\x03\x90\xFD[a\x13k\x85`\x05_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x16W\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93P\x83`\x05_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP_a\x13\xA4\x85a\x15kV[\x90P_`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x84a\x13\xCB\x91\x90a0\xABV[\x81T\x81\x10a\x13\xDCWa\x13\xDBa&ZV[[\x90_R` _ \x01\x90PCc\xFF\xFF\xFF\xFF\x16\x81_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a\x14CW\x81\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UPa\x15TV[C\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[PPPP\x80\x80`\x01\x01\x91PPa\x12\x97V[PPPPV[_\x81Q_R\x81` \x01Q` R`@_ \x90P\x91\x90PV[a\x15\x8Ba\x1B\xDFV[a\x15\x93a\x1C,V[\x83_\x01Q\x81_`\x03\x81\x10a\x15\xAAWa\x15\xA9a&ZV[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x03\x81\x10a\x15\xCCWa\x15\xCBa&ZV[[` \x02\x01\x81\x81RPP\x82\x81`\x02`\x03\x81\x10a\x15\xEAWa\x15\xE9a&ZV[[` \x02\x01\x81\x81RPP_`@\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x16\x0EW\xFE[P\x80a\x16OW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16F\x90a2\x91V[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x16_a\x1B\xDFV[a\x16ga\x1CNV[\x83_\x01Q\x81_`\x04\x81\x10a\x16~Wa\x16}a&ZV[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x04\x81\x10a\x16\xA0Wa\x16\x9Fa&ZV[[` \x02\x01\x81\x81RPP\x82_\x01Q\x81`\x02`\x04\x81\x10a\x16\xC1Wa\x16\xC0a&ZV[[` \x02\x01\x81\x81RPP\x82` \x01Q\x81`\x03`\x04\x81\x10a\x16\xE3Wa\x16\xE2a&ZV[[` \x02\x01\x81\x81RPP_`@\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x17\x07W\xFE[P\x80a\x17HW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17?\x90a2\xF9V[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x17Xa\x1CpV[`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x81R` \x01\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D\x81RP\x81RP\x90P\x90V[a\x18\"a\x1B\xDFV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x02\x81RP\x90P\x90V[__`@Q\x80`@\x01`@R\x80\x87\x81R` \x01\x85\x81RP\x90P_`@Q\x80`@\x01`@R\x80\x87\x81R` \x01\x85\x81RP\x90Pa\x18wa\x1C\x96V[__\x90P[`\x02\x81\x10\x15a\x1A\x95W_`\x06\x82a\x18\x93\x91\x90a3\x17V[\x90P\x84\x82`\x02\x81\x10a\x18\xA8Wa\x18\xA7a&ZV[[` \x02\x01Q_\x01Q\x83_\x83a\x18\xBD\x91\x90a3XV[`\x0C\x81\x10a\x18\xCEWa\x18\xCDa&ZV[[` \x02\x01\x81\x81RPP\x84\x82`\x02\x81\x10a\x18\xEAWa\x18\xE9a&ZV[[` \x02\x01Q` \x01Q\x83`\x01\x83a\x19\x01\x91\x90a3XV[`\x0C\x81\x10a\x19\x12Wa\x19\x11a&ZV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x19.Wa\x19-a&ZV[[` \x02\x01Q_\x01Q_`\x02\x81\x10a\x19HWa\x19Ga&ZV[[` \x02\x01Q\x83`\x02\x83a\x19[\x91\x90a3XV[`\x0C\x81\x10a\x19lWa\x19ka&ZV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x19\x88Wa\x19\x87a&ZV[[` \x02\x01Q_\x01Q`\x01`\x02\x81\x10a\x19\xA3Wa\x19\xA2a&ZV[[` \x02\x01Q\x83`\x03\x83a\x19\xB6\x91\x90a3XV[`\x0C\x81\x10a\x19\xC7Wa\x19\xC6a&ZV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x19\xE3Wa\x19\xE2a&ZV[[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x19\xFEWa\x19\xFDa&ZV[[` \x02\x01Q\x83`\x04\x83a\x1A\x11\x91\x90a3XV[`\x0C\x81\x10a\x1A\"Wa\x1A!a&ZV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x1A>Wa\x1A=a&ZV[[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x1AZWa\x1AYa&ZV[[` \x02\x01Q\x83`\x05\x83a\x1Am\x91\x90a3XV[`\x0C\x81\x10a\x1A~Wa\x1A}a&ZV[[` \x02\x01\x81\x81RPPP\x80\x80`\x01\x01\x91PPa\x18|V[Pa\x1A\x9Ea\x1C\xB9V[_` \x82` `\x0C\x02\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x1A\xBCW\xFE[P\x80a\x1A\xFDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\xF4\x90a3\xD5V[`@Q\x80\x91\x03\x90\xFD[_\x82_`\x01\x81\x10a\x1B\x11Wa\x1B\x10a&ZV[[` \x02\x01Q\x14\x15\x95PPPPPP\x94\x93PPPPV[a\x1B/a\x1B\xDFV[_\x82_\x01Q\x14\x80\x15a\x1BDWP_\x82` \x01Q\x14[\x15a\x1BeW`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x1B\xDAV[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x1B\xA9\x91\x90a+\xC0V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1B\xD4\x91\x90a0\xABV[\x81RP\x90P[\x91\x90PV[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80a\x1C\x83a\x1C\xDBV[\x81R` \x01a\x1C\x90a\x1C\xDBV[\x81RP\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x1D7\x82a\x1D\x0EV[\x90P\x91\x90PV[a\x1DG\x81a\x1D-V[\x81\x14a\x1DQW__\xFD[PV[_\x815\x90Pa\x1Db\x81a\x1D>V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1D}Wa\x1D|a\x1D\x06V[[_a\x1D\x8A\x84\x82\x85\x01a\x1DTV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x1D\xA5\x81a\x1D\x93V[\x82RPPV[_`@\x82\x01\x90Pa\x1D\xBE_\x83\x01\x85a\x1D\x9CV[a\x1D\xCB` \x83\x01\x84a\x1D\x9CV[\x93\x92PPPV[_\x81\x90P\x91\x90PV[a\x1D\xE4\x81a\x1D\xD2V[\x82RPPV[_` \x82\x01\x90Pa\x1D\xFD_\x83\x01\x84a\x1D\xDBV[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x1E\x18\x81a\x1E\x03V[\x81\x14a\x1E\"W__\xFD[PV[_\x815\x90Pa\x1E3\x81a\x1E\x0FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1ENWa\x1EMa\x1D\x06V[[_a\x1E[\x84\x82\x85\x01a\x1E%V[\x91PP\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x1E|\x81a\x1EdV[\x82RPPV[_` \x82\x01\x90Pa\x1E\x95_\x83\x01\x84a\x1EsV[\x92\x91PPV[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x1E\xE9\x82a\x1E\xA3V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1F\x08Wa\x1F\x07a\x1E\xB3V[[\x80`@RPPPV[_a\x1F\x1Aa\x1C\xFDV[\x90Pa\x1F&\x82\x82a\x1E\xE0V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1FEWa\x1FDa\x1E\xB3V[[a\x1FN\x82a\x1E\xA3V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x1F{a\x1Fv\x84a\x1F+V[a\x1F\x11V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x1F\x97Wa\x1F\x96a\x1E\x9FV[[a\x1F\xA2\x84\x82\x85a\x1F[V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x1F\xBEWa\x1F\xBDa\x1E\x9BV[[\x815a\x1F\xCE\x84\x82` \x86\x01a\x1FiV[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x1F\xEDWa\x1F\xECa\x1D\x06V[[_a\x1F\xFA\x85\x82\x86\x01a\x1DTV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x1BWa \x1Aa\x1D\nV[[a '\x85\x82\x86\x01a\x1F\xAAV[\x91PP\x92P\x92\x90PV[a :\x81a\x1D\xD2V[\x81\x14a DW__\xFD[PV[_\x815\x90Pa U\x81a 1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a pWa oa\x1D\x06V[[_a }\x84\x82\x85\x01a GV[\x91PP\x92\x91PPV[a \x8F\x81a\x1D-V[\x82RPPV[_` \x82\x01\x90Pa \xA8_\x83\x01\x84a \x86V[\x92\x91PPV[a \xB7\x81a\x1D\x93V[\x82RPPV[`@\x82\x01_\x82\x01Qa \xD1_\x85\x01\x82a \xAEV[P` \x82\x01Qa \xE4` \x85\x01\x82a \xAEV[PPPPV[_`@\x82\x01\x90Pa \xFD_\x83\x01\x84a \xBDV[\x92\x91PPV[a!\x0C\x81a\x1D\x93V[\x81\x14a!\x16W__\xFD[PV[_\x815\x90Pa!'\x81a!\x03V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a!CWa!Ba\x1D\x06V[[_a!P\x85\x82\x86\x01a\x1E%V[\x92PP` a!a\x85\x82\x86\x01a!\x19V[\x91PP\x92P\x92\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a!\x9F\x81a!kV[\x82RPPV[a!\xAE\x81a\x1EdV[\x82RPPV[``\x82\x01_\x82\x01Qa!\xC8_\x85\x01\x82a!\x96V[P` \x82\x01Qa!\xDB` \x85\x01\x82a!\xA5V[P`@\x82\x01Qa!\xEE`@\x85\x01\x82a!\xA5V[PPPPV[_``\x82\x01\x90Pa\"\x07_\x83\x01\x84a!\xB4V[\x92\x91PPV[a\"\x16\x81a\x1EdV[\x81\x14a\" W__\xFD[PV[_\x815\x90Pa\"1\x81a\"\rV[\x92\x91PPV[___``\x84\x86\x03\x12\x15a\"NWa\"Ma\x1D\x06V[[_a\"[\x86\x82\x87\x01a\x1E%V[\x93PP` a\"l\x86\x82\x87\x01a\"#V[\x92PP`@a\"}\x86\x82\x87\x01a!\x19V[\x91PP\x92P\x92P\x92V[a\"\x90\x81a!kV[\x82RPPV[_` \x82\x01\x90Pa\"\xA9_\x83\x01\x84a\"\x87V[\x92\x91PPV[_``\x82\x01\x90Pa\"\xC2_\x83\x01\x86a\"\x87V[a\"\xCF` \x83\x01\x85a\x1EsV[a\"\xDC`@\x83\x01\x84a\x1EsV[\x94\x93PPPPV[_``\x82\x01\x90Pa\"\xF7_\x83\x01\x85a \xBDV[a#\x04`@\x83\x01\x84a\x1D\xDBV[\x93\x92PPPV[__\xFD[_a\x01\0\x82\x84\x03\x12\x15a#%Wa#$a#\x0BV[[\x81\x90P\x92\x91PPV[_`@\x82\x84\x03\x12\x15a#CWa#Ba#\x0BV[[\x81\x90P\x92\x91PPV[___a\x01`\x84\x86\x03\x12\x15a#dWa#ca\x1D\x06V[[_a#q\x86\x82\x87\x01a\x1DTV[\x93PP` a#\x82\x86\x82\x87\x01a#\x0FV[\x92PPa\x01 a#\x94\x86\x82\x87\x01a#.V[\x91PP\x92P\x92P\x92V[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a#\xBBWa#\xBAa\x1E\x9BV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xD8Wa#\xD7a#\x9EV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a#\xF4Wa#\xF3a#\xA2V[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a$\x12Wa$\x11a\x1D\x06V[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$/Wa$.a\x1D\nV[[a$;\x86\x82\x87\x01a#\xA6V[\x93P\x93PP` a$N\x86\x82\x87\x01a!\x19V[\x91PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a$\x8C\x83\x83a!\xA5V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a$\xAE\x82a$XV[a$\xB8\x81\x85a$bV[\x93Pa$\xC3\x83a$rV[\x80_[\x83\x81\x10\x15a$\xF3W\x81Qa$\xDA\x88\x82a$\x81V[\x97Pa$\xE5\x83a$\x98V[\x92PP`\x01\x81\x01\x90Pa$\xC6V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra%\x18\x81\x84a$\xA4V[\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBLSApkRegistry.initializeQuorum:_\x82\x01R\x7F quorum already exists\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a%\x8A`6\x83a% V[\x91Pa%\x95\x82a%0V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra%\xB7\x81a%~V[\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a%\xF0\x82a%\xBEV[a%\xFA\x81\x85a%\xC8V[\x93Pa&\n\x81\x85` \x86\x01a%\xD8V[a&\x13\x81a\x1E\xA3V[\x84\x01\x91PP\x92\x91PPV[_``\x82\x01\x90Pa&1_\x83\x01\x86a \x86V[a&>` \x83\x01\x85a\x1D\xDBV[\x81\x81\x03`@\x83\x01Ra&P\x81\x84a%\xE6V[\x90P\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FBLSApkRegistry.getApkHashAtBlock_\x82\x01R\x7FNumberAndIndex: index too recent` \x82\x01RPV[_a&\xE1`@\x83a% V[\x91Pa&\xEC\x82a&\x87V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra'\x0E\x81a&\xD5V[\x90P\x91\x90PV[\x7FBLSApkRegistry.getApkHashAtBlock_\x82\x01R\x7FNumberAndIndex: not latest apk u` \x82\x01R\x7Fpdate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a'\x95`E\x83a% V[\x91Pa'\xA0\x82a'\x15V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra'\xC2\x81a'\x89V[\x90P\x91\x90PV[\x7FBLSApkRegistry.getRegisteredPubk_\x82\x01R\x7Fey: operator is not registered\0\0` \x82\x01RPV[_a(#`>\x83a% V[\x91Pa(.\x82a'\xC9V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra(P\x81a(\x17V[\x90P\x91\x90PV[__\xFD[_`@\x82\x84\x03\x12\x15a(pWa(oa(WV[[a(z`@a\x1F\x11V[\x90P_a(\x89\x84\x82\x85\x01a!\x19V[_\x83\x01RP` a(\x9C\x84\x82\x85\x01a!\x19V[` \x83\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a(\xBDWa(\xBCa\x1D\x06V[[_a(\xCA\x84\x82\x85\x01a([V[\x91PP\x92\x91PPV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: cannot register zero pubkey` \x82\x01RPV[_a)-`@\x83a% V[\x91Pa)8\x82a(\xD3V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra)Z\x81a)!V[\x90P\x91\x90PV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: operator already registered` \x82\x01R\x7F pubkey\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a)\xE1`G\x83a% V[\x91Pa)\xEC\x82a)aV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra*\x0E\x81a)\xD5V[\x90P\x91\x90PV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: public key already register` \x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a*\x95`B\x83a% V[\x91Pa*\xA0\x82a*\x15V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra*\xC2\x81a*\x89V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a*\xE3a*\xDE\x82a\x1D\x93V[a*\xC9V[\x82RPPV[\x82\x81\x837PPPV[a*\xFE`@\x83\x83a*\xE9V[PPV[_a+\r\x82\x8Ba*\xD2V[` \x82\x01\x91Pa+\x1D\x82\x8Aa*\xD2V[` \x82\x01\x91Pa+-\x82\x89a*\xD2V[` \x82\x01\x91Pa+=\x82\x88a*\xD2V[` \x82\x01\x91Pa+M\x82\x87a*\xF2V[`@\x82\x01\x91Pa+]\x82\x86a*\xF2V[`@\x82\x01\x91Pa+m\x82\x85a*\xD2V[` \x82\x01\x91Pa+}\x82\x84a*\xD2V[` \x82\x01\x91P\x81\x90P\x99\x98PPPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a+\xCA\x82a\x1D\x93V[\x91Pa+\xD5\x83a\x1D\x93V[\x92P\x82a+\xE5Wa+\xE4a+\x93V[[\x82\x82\x06\x90P\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a,\nWa,\ta\x1E\xB3V[[` \x82\x02\x90P\x91\x90PV[_a,'a,\"\x84a+\xF0V[a\x1F\x11V[\x90P\x80` \x84\x02\x83\x01\x85\x81\x11\x15a,AWa,@a#\xA2V[[\x83[\x81\x81\x10\x15a,jW\x80a,V\x88\x82a!\x19V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa,CV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a,\x88Wa,\x87a\x1E\x9BV[[`\x02a,\x95\x84\x82\x85a,\x15V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a,\xB3Wa,\xB2a(WV[[a,\xBD`@a\x1F\x11V[\x90P_a,\xCC\x84\x82\x85\x01a,tV[_\x83\x01RP`@a,\xDF\x84\x82\x85\x01a,tV[` \x83\x01RP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a-\0Wa,\xFFa\x1D\x06V[[_a-\r\x84\x82\x85\x01a,\x9EV[\x91PP\x92\x91PPV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: either the G1 signature is ` \x82\x01R\x7Fwrong, or G1 and G2 private key `@\x82\x01R\x7Fdo not match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_a-\xBC`l\x83a% V[\x91Pa-\xC7\x82a-\x16V[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra-\xE9\x81a-\xB0V[\x90P\x91\x90PV[_\x815a-\xFC\x81a!\x03V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa.;\x84a.\x05V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a.ta.oa.j\x84a\x1D\x93V[a.QV[a\x1D\x93V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a.\x8D\x82a.ZV[a.\xA0a.\x99\x82a.{V[\x83Ta.\x10V[\x82UPPPV[_\x81\x01_\x83\x01\x80a.\xB7\x81a-\xF0V[\x90Pa.\xC3\x81\x84a.\x84V[PPP`\x01\x81\x01` \x83\x01\x80a.\xD8\x81a-\xF0V[\x90Pa.\xE4\x81\x84a.\x84V[PPPPPV[a.\xF5\x82\x82a.\xA7V[PPV[_a/\x07` \x84\x01\x84a!\x19V[\x90P\x92\x91PPV[`@\x82\x01a/\x1F_\x83\x01\x83a.\xF9V[a/+_\x85\x01\x82a \xAEV[Pa/9` \x83\x01\x83a.\xF9V[a/F` \x85\x01\x82a \xAEV[PPPPV[_\x82\x90P\x92\x91PPV[a/b`@\x83\x83a*\xE9V[PPV[`\x80\x82\x01a/v_\x83\x01\x83a/LV[a/\x82_\x85\x01\x82a/VV[Pa/\x90`@\x83\x01\x83a/LV[a/\x9D`@\x85\x01\x82a/VV[PPPPV[_`\xC0\x82\x01\x90Pa/\xB6_\x83\x01\x85a/\x0FV[a/\xC3`@\x83\x01\x84a/fV[\x93\x92PPPV[\x7FBLSApkRegistry.getApkIndicesAtBl_\x82\x01R\x7FockNumber: blockNumber is before` \x82\x01R\x7F the first update\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a0J`Q\x83a% V[\x91Pa0U\x82a/\xCAV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra0w\x81a0>V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a0\xB5\x82a\x1D\x93V[\x91Pa0\xC0\x83a\x1D\x93V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a0\xD8Wa0\xD7a0~V[[\x92\x91PPV[_a0\xE8\x82a\x1D\x93V[\x91P_\x82\x03a0\xFAWa0\xF9a0~V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FBLSApkRegistry._checkRegistryCoo_\x82\x01R\x7Frdinator: caller is not the regi` \x82\x01R\x7Fstry coordinator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a1\x85`P\x83a% V[\x91Pa1\x90\x82a1\x05V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra1\xB2\x81a1yV[\x90P\x91\x90PV[\x7FBLSApkRegistry._processQuorumApk_\x82\x01R\x7FUpdate: quorum does not exist\0\0\0` \x82\x01RPV[_a2\x13`=\x83a% V[\x91Pa2\x1E\x82a1\xB9V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra2@\x81a2\x07V[\x90P\x91\x90PV[\x7Fec-mul-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a2{`\r\x83a% V[\x91Pa2\x86\x82a2GV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra2\xA8\x81a2oV[\x90P\x91\x90PV[\x7Fec-add-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a2\xE3`\r\x83a% V[\x91Pa2\xEE\x82a2\xAFV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3\x10\x81a2\xD7V[\x90P\x91\x90PV[_a3!\x82a\x1D\x93V[\x91Pa3,\x83a\x1D\x93V[\x92P\x82\x82\x02a3:\x81a\x1D\x93V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a3QWa3Pa0~V[[P\x92\x91PPV[_a3b\x82a\x1D\x93V[\x91Pa3m\x83a\x1D\x93V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a3\x85Wa3\x84a0~V[[\x92\x91PPV[\x7Fpairing-opcode-failed\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a3\xBF`\x15\x83a% V[\x91Pa3\xCA\x82a3\x8BV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3\xEC\x81a3\xB3V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x122\xBDs#\xC7\xC8g\x1F\xF9\xA4\xDB\xF3\xEB\xA5E\xD7\xF2\xDDx\xE3\x95\xCA\x0B\xAE\x11\x18\x05j\x96j\x1AdsolcC\0\x08\x1B\x003",
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
