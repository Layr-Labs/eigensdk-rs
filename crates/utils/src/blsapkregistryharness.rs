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
    clippy::style
)]
pub mod BLSApkRegistryHarness {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405234801561000f575f5ffd5b5060405161384a38038061384a833981810160405281019061003191906101ab565b80808073ffffffffffffffffffffffffffffffffffffffff1660808173ffffffffffffffffffffffffffffffffffffffff168152505061007561007d60201b60201c565b5050506102a8565b5f60019054906101000a900460ff16156100cc576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100c390610256565b60405180910390fd5b60ff80165f5f9054906101000a900460ff1660ff16101561013a5760ff5f5f6101000a81548160ff021916908360ff1602179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249860ff604051610131919061028f565b60405180910390a15b565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61016982610140565b9050919050565b5f61017a8261015f565b9050919050565b61018a81610170565b8114610194575f5ffd5b50565b5f815190506101a581610181565b92915050565b5f602082840312156101c0576101bf61013c565b5b5f6101cd84828501610197565b91505092915050565b5f82825260208201905092915050565b7f496e697469616c697a61626c653a20636f6e747261637420697320696e6974695f8201527f616c697a696e6700000000000000000000000000000000000000000000000000602082015250565b5f6102406027836101d6565b915061024b826101e6565b604082019050919050565b5f6020820190508181035f83015261026d81610234565b9050919050565b5f60ff82169050919050565b61028981610274565b82525050565b5f6020820190506102a25f830184610280565b92915050565b6080516135836102c75f395f8181610959015261131801526135835ff3fe608060405234801561000f575f5ffd5b506004361061011e575f3560e01c80636d14a987116100ab578063d5254a8c1161006f578063d5254a8c1461038d578063de29fac0146103bd578063df4d09e0146103ed578063e8bb9ae614610409578063f4e24fe5146104395761011e565b80636d14a987146102ab5780637916cea6146102c95780637ff81a87146102fb578063a3db80e21461032c578063bf79ce581461035d5761011e565b80633fb27952116100f25780633fb27952146101cf57806347b314e8146101eb5780635f61a8841461021b578063605747d51461024b57806368bccaac1461027b5761011e565b8062a1f4cb1461012257806313542a4e1461015357806326d941f214610183578063377ed99d1461019f575b5f5ffd5b61013c60048036038101906101379190611e84565b610455565b60405161014a929190611ec7565b60405180910390f35b61016d60048036038101906101689190611e84565b610475565b60405161017a9190611f06565b60405180910390f35b61019d60048036038101906101989190611f55565b6104bb565b005b6101b960048036038101906101b49190611f55565b610613565b6040516101c69190611f9e565b60405180910390f35b6101e960048036038101906101e491906120f3565b610636565b005b61020560048036038101906102009190612177565b61069d565b60405161021291906121b1565b60405180910390f35b61023560048036038101906102309190611f55565b6106d6565b6040516102429190612206565b60405180910390f35b61026560048036038101906102609190612249565b61071a565b6040516102729190612310565b60405180910390f35b61029560048036038101906102909190612353565b6107db565b6040516102a291906123b2565b60405180910390f35b6102b3610957565b6040516102c091906121b1565b60405180910390f35b6102e360048036038101906102de9190612249565b61097b565b6040516102f2939291906123cb565b60405180910390f35b61031560048036038101906103109190611e84565b6109e1565b604051610323929190612400565b60405180910390f35b61034660048036038101906103419190611f55565b610ada565b604051610354929190611ec7565b60405180910390f35b61037760048036038101906103729190612468565b610afa565b6040516103849190611f06565b60405180910390f35b6103a760048036038101906103a29190612517565b610f5e565b6040516103b4919061261c565b60405180910390f35b6103d760048036038101906103d29190611e84565b61116d565b6040516103e49190611f06565b60405180910390f35b6104076004803603810190610402919061268d565b611182565b005b610423600480360381019061041e9190612177565b611277565b60405161043091906121b1565b60405180910390f35b610453600480360381019061044e91906120f3565b6112a7565b005b6003602052805f5260405f205f91509050805f0154908060010154905082565b5f60015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20549050919050565b6104c3611316565b5f60045f8360ff1660ff1681526020019081526020015f20805490501461051f576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016105169061274b565b60405180910390fd5b60045f8260ff1660ff1681526020019081526020015f2060405180606001604052805f60401b67ffffffffffffffff191681526020014363ffffffff1681526020015f63ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c02179055506020820151815f0160186101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f01601c6101000a81548163ffffffff021916908363ffffffff160217905550505050565b5f60045f8360ff1660ff1681526020019081526020015f20805490509050919050565b61063e611316565b5f610648836109e1565b50905061065582826113a6565b7f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e8361068085610475565b84604051610690939291906127c9565b60405180910390a1505050565b5f60025f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050919050565b6106de611cfb565b60055f8360ff1660ff1681526020019081526020015f206040518060400160405290815f82015481526020016001820154815250509050919050565b610722611d13565b60045f8460ff1660ff1681526020019081526020015f20828154811061074b5761074a612805565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900460401b67ffffffffffffffff191667ffffffffffffffff191681526020015f820160189054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f8201601c9054906101000a900463ffffffff1663ffffffff1663ffffffff1681525050905092915050565b5f5f60045f8660ff1660ff1681526020019081526020015f20838154811061080657610805612805565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900460401b67ffffffffffffffff191667ffffffffffffffff191681526020015f820160189054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f8201601c9054906101000a900463ffffffff1663ffffffff1663ffffffff16815250509050806020015163ffffffff168463ffffffff1610156108e3576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016108da906128a2565b60405180910390fd5b5f816040015163ffffffff16148061090a5750806040015163ffffffff168463ffffffff16105b610949576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161094090612956565b60405180910390fd5b805f01519150509392505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b6004602052815f5260405f208181548110610994575f80fd5b905f5260205f20015f9150915050805f015f9054906101000a900460401b90805f0160189054906101000a900463ffffffff1690805f01601c9054906101000a900463ffffffff16905083565b6109e9611cfb565b5f5f60035f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060400160405290815f820154815260200160018201548152505090505f60015f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205490505f5f1b8103610acd576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610ac4906129e4565b60405180910390fd5b8181935093505050915091565b6005602052805f5260405f205f91509050805f0154908060010154905082565b5f610b03611316565b5f610b2084604001803603810190610b1b9190612a02565b611687565b90507fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb58103610b84576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610b7b90612a9d565b60405180910390fd5b5f5f1b60015f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205414610c05576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610bfc90612b51565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff1660025f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614610ca3576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610c9a90612c05565b60405180910390fd5b5f7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001855f015f0135865f0160200135876040015f01358860400160200135896080015f018a6080016040018a5f01358b60200135604051602001610d0e989796959493929190612c5c565b604051602081830303815290604052805190602001205f1c610d309190612d1a565b9050610ddc610d7f610d5e8388604001803603810190610d509190612a02565b61169f90919063ffffffff16565b875f01803603810190610d719190612a02565b61177390919063ffffffff16565b610d8761186c565b610dc3610da485610d96611936565b61169f90919063ffffffff16565b88803603810190610db59190612a02565b61177390919063ffffffff16565b88608001803603810190610dd79190612e45565b61195a565b610e1b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e1290612f2c565b60405180910390fd5b8460400160035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181610e689190613045565b9050508160015f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508560025f8481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508573ffffffffffffffffffffffffffffffffffffffff167fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba38280418660400187608001604051610f4a9291906130fd565b60405180910390a281925050509392505050565b60605f8484905067ffffffffffffffff811115610f7e57610f7d611fcf565b5b604051908082528060200260200182016040528015610fac5781602001602082028036833780820191505090505b5090505f5f90505b85859050811015611161575f868683818110610fd357610fd2612805565b5b9050013560f81c60f81b60f81c90505f60045f8360ff1660ff1681526020019081526020015f208054905090505f811480611056575060045f8360ff1660ff1681526020019081526020015f205f8154811061103257611031612805565b5b905f5260205f20015f0160189054906101000a900463ffffffff1663ffffffff1686105b15611096576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161108d906131ba565b60405180910390fd5b5f8190505b5f811115611151578660045f8560ff1660ff1681526020019081526020015f206001836110c89190613205565b815481106110d9576110d8612805565b5b905f5260205f20015f0160189054906101000a900463ffffffff1663ffffffff161161113e5760018161110c9190613205565b85858151811061111f5761111e612805565b5b602002602001019063ffffffff16908163ffffffff1681525050611151565b808061114990613238565b91505061109b565b5050508080600101915050610fb4565b50809150509392505050565b6001602052805f5260405f205f915090505481565b5f61118c82611687565b90508060015f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508260025f8381526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508160035f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f015560208201518160010155905050505050565b6002602052805f5260405f205f915054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6112af611316565b5f6112b9836109e1565b5090506112ce826112c983611c43565b6113a6565b7ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e836112f985610475565b84604051611309939291906127c9565b60405180910390a1505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146113a4576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161139b906132f5565b60405180910390fd5b565b6113ae611cfb565b5f5f90505b8351811015611681575f8482815181106113d0576113cf612805565b5b602001015160f81c60f81b60f81c90505f60045f8360ff1660ff1681526020019081526020015f208054905090505f8103611440576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161143790613383565b60405180910390fd5b6114878560055f8560ff1660ff1681526020019081526020015f206040518060400160405290815f820154815260200160018201548152505061177390919063ffffffff16565b93508360055f8460ff1660ff1681526020019081526020015f205f820151815f0155602082015181600101559050505f6114c085611687565b90505f60045f8560ff1660ff1681526020019081526020015f206001846114e79190613205565b815481106114f8576114f7612805565b5b905f5260205f200190504363ffffffff16815f0160189054906101000a900463ffffffff1663ffffffff160361155f5781815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c0217905550611670565b43815f01601c6101000a81548163ffffffff021916908363ffffffff16021790555060045f8560ff1660ff1681526020019081526020015f2060405180606001604052808467ffffffffffffffff191681526020014363ffffffff1681526020015f63ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c02179055506020820151815f0160186101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f01601c6101000a81548163ffffffff021916908363ffffffff16021790555050505b5050505080806001019150506113b3565b50505050565b5f81515f52816020015160205260405f209050919050565b6116a7611cfb565b6116af611d48565b835f0151815f600381106116c6576116c5612805565b5b6020020181815250508360200151816001600381106116e8576116e7612805565b5b602002018181525050828160026003811061170657611705612805565b5b6020020181815250505f60408360608460076107d05a03fa9050805f810361172a57fe5b508061176b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611762906133eb565b60405180910390fd5b505092915050565b61177b611cfb565b611783611d6a565b835f0151815f6004811061179a57611799612805565b5b6020020181815250508360200151816001600481106117bc576117bb612805565b5b602002018181525050825f0151816002600481106117dd576117dc612805565b5b6020020181815250508260200151816003600481106117ff576117fe612805565b5b6020020181815250505f60408360808460066107d05a03fa9050805f810361182357fe5b5080611864576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161185b90613453565b60405180910390fd5b505092915050565b611874611d8c565b604051806040016040528060405180604001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed815250815260200160405180604001604052807f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec81526020017f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d815250815250905090565b61193e611cfb565b6040518060400160405280600181526020016002815250905090565b5f5f60405180604001604052808781526020018581525090505f6040518060400160405280878152602001858152509050611993611db2565b5f5f90505b6002811015611bb1575f6006826119af9190613471565b90508482600281106119c4576119c3612805565b5b60200201515f0151835f836119d991906134b2565b600c81106119ea576119e9612805565b5b602002018181525050848260028110611a0657611a05612805565b5b60200201516020015183600183611a1d91906134b2565b600c8110611a2e57611a2d612805565b5b602002018181525050838260028110611a4a57611a49612805565b5b60200201515f01515f60028110611a6457611a63612805565b5b602002015183600283611a7791906134b2565b600c8110611a8857611a87612805565b5b602002018181525050838260028110611aa457611aa3612805565b5b60200201515f0151600160028110611abf57611abe612805565b5b602002015183600383611ad291906134b2565b600c8110611ae357611ae2612805565b5b602002018181525050838260028110611aff57611afe612805565b5b6020020151602001515f60028110611b1a57611b19612805565b5b602002015183600483611b2d91906134b2565b600c8110611b3e57611b3d612805565b5b602002018181525050838260028110611b5a57611b59612805565b5b602002015160200151600160028110611b7657611b75612805565b5b602002015183600583611b8991906134b2565b600c8110611b9a57611b99612805565b5b602002018181525050508080600101915050611998565b50611bba611dd5565b5f6020826020600c028560086107d05a03fa9050805f8103611bd857fe5b5080611c19576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611c109061352f565b60405180910390fd5b5f825f60018110611c2d57611c2c612805565b5b6020020151141595505050505050949350505050565b611c4b611cfb565b5f825f0151148015611c6057505f8260200151145b15611c815760405180604001604052805f81526020015f8152509050611cf6565b6040518060400160405280835f015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478460200151611cc59190612d1a565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611cf09190613205565b81525090505b919050565b60405180604001604052805f81526020015f81525090565b60405180606001604052805f67ffffffffffffffff191681526020015f63ffffffff1681526020015f63ffffffff1681525090565b6040518060600160405280600390602082028036833780820191505090505090565b6040518060800160405280600490602082028036833780820191505090505090565b6040518060400160405280611d9f611df7565b8152602001611dac611df7565b81525090565b604051806101800160405280600c90602082028036833780820191505090505090565b6040518060200160405280600190602082028036833780820191505090505090565b6040518060400160405280600290602082028036833780820191505090505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f611e5382611e2a565b9050919050565b611e6381611e49565b8114611e6d575f5ffd5b50565b5f81359050611e7e81611e5a565b92915050565b5f60208284031215611e9957611e98611e22565b5b5f611ea684828501611e70565b91505092915050565b5f819050919050565b611ec181611eaf565b82525050565b5f604082019050611eda5f830185611eb8565b611ee76020830184611eb8565b9392505050565b5f819050919050565b611f0081611eee565b82525050565b5f602082019050611f195f830184611ef7565b92915050565b5f60ff82169050919050565b611f3481611f1f565b8114611f3e575f5ffd5b50565b5f81359050611f4f81611f2b565b92915050565b5f60208284031215611f6a57611f69611e22565b5b5f611f7784828501611f41565b91505092915050565b5f63ffffffff82169050919050565b611f9881611f80565b82525050565b5f602082019050611fb15f830184611f8f565b92915050565b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61200582611fbf565b810181811067ffffffffffffffff8211171561202457612023611fcf565b5b80604052505050565b5f612036611e19565b90506120428282611ffc565b919050565b5f67ffffffffffffffff82111561206157612060611fcf565b5b61206a82611fbf565b9050602081019050919050565b828183375f83830152505050565b5f61209761209284612047565b61202d565b9050828152602081018484840111156120b3576120b2611fbb565b5b6120be848285612077565b509392505050565b5f82601f8301126120da576120d9611fb7565b5b81356120ea848260208601612085565b91505092915050565b5f5f6040838503121561210957612108611e22565b5b5f61211685828601611e70565b925050602083013567ffffffffffffffff81111561213757612136611e26565b5b612143858286016120c6565b9150509250929050565b61215681611eee565b8114612160575f5ffd5b50565b5f813590506121718161214d565b92915050565b5f6020828403121561218c5761218b611e22565b5b5f61219984828501612163565b91505092915050565b6121ab81611e49565b82525050565b5f6020820190506121c45f8301846121a2565b92915050565b6121d381611eaf565b82525050565b604082015f8201516121ed5f8501826121ca565b50602082015161220060208501826121ca565b50505050565b5f6040820190506122195f8301846121d9565b92915050565b61222881611eaf565b8114612232575f5ffd5b50565b5f813590506122438161221f565b92915050565b5f5f6040838503121561225f5761225e611e22565b5b5f61226c85828601611f41565b925050602061227d85828601612235565b9150509250929050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000082169050919050565b6122bb81612287565b82525050565b6122ca81611f80565b82525050565b606082015f8201516122e45f8501826122b2565b5060208201516122f760208501826122c1565b50604082015161230a60408501826122c1565b50505050565b5f6060820190506123235f8301846122d0565b92915050565b61233281611f80565b811461233c575f5ffd5b50565b5f8135905061234d81612329565b92915050565b5f5f5f6060848603121561236a57612369611e22565b5b5f61237786828701611f41565b93505060206123888682870161233f565b925050604061239986828701612235565b9150509250925092565b6123ac81612287565b82525050565b5f6020820190506123c55f8301846123a3565b92915050565b5f6060820190506123de5f8301866123a3565b6123eb6020830185611f8f565b6123f86040830184611f8f565b949350505050565b5f6060820190506124135f8301856121d9565b6124206040830184611ef7565b9392505050565b5f5ffd5b5f610100828403121561244157612440612427565b5b81905092915050565b5f6040828403121561245f5761245e612427565b5b81905092915050565b5f5f5f61016084860312156124805761247f611e22565b5b5f61248d86828701611e70565b935050602061249e8682870161242b565b9250506101206124b08682870161244a565b9150509250925092565b5f5ffd5b5f5ffd5b5f5f83601f8401126124d7576124d6611fb7565b5b8235905067ffffffffffffffff8111156124f4576124f36124ba565b5b6020830191508360018202830111156125105761250f6124be565b5b9250929050565b5f5f5f6040848603121561252e5761252d611e22565b5b5f84013567ffffffffffffffff81111561254b5761254a611e26565b5b612557868287016124c2565b9350935050602061256a86828701612235565b9150509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6125a883836122c1565b60208301905092915050565b5f602082019050919050565b5f6125ca82612574565b6125d4818561257e565b93506125df8361258e565b805f5b8381101561260f5781516125f6888261259d565b9750612601836125b4565b9250506001810190506125e2565b5085935050505092915050565b5f6020820190508181035f83015261263481846125c0565b905092915050565b5f5ffd5b5f604082840312156126555761265461263c565b5b61265f604061202d565b90505f61266e84828501612235565b5f83015250602061268184828501612235565b60208301525092915050565b5f5f606083850312156126a3576126a2611e22565b5b5f6126b085828601611e70565b92505060206126c185828601612640565b9150509250929050565b5f82825260208201905092915050565b7f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a5f8201527f2071756f72756d20616c72656164792065786973747300000000000000000000602082015250565b5f6127356036836126cb565b9150612740826126db565b604082019050919050565b5f6020820190508181035f83015261276281612729565b9050919050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f61279b82612769565b6127a58185612773565b93506127b5818560208601612783565b6127be81611fbf565b840191505092915050565b5f6060820190506127dc5f8301866121a2565b6127e96020830185611ef7565b81810360408301526127fb8184612791565b9050949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f424c5341706b52656769737472792e67657441706b486173684174426c6f636b5f8201527f4e756d626572416e64496e6465783a20696e64657820746f6f20726563656e74602082015250565b5f61288c6040836126cb565b915061289782612832565b604082019050919050565b5f6020820190508181035f8301526128b981612880565b9050919050565b7f424c5341706b52656769737472792e67657441706b486173684174426c6f636b5f8201527f4e756d626572416e64496e6465783a206e6f74206c61746573742061706b207560208201527f7064617465000000000000000000000000000000000000000000000000000000604082015250565b5f6129406045836126cb565b915061294b826128c0565b606082019050919050565b5f6020820190508181035f83015261296d81612934565b9050919050565b7f424c5341706b52656769737472792e676574526567697374657265645075626b5f8201527f65793a206f70657261746f72206973206e6f7420726567697374657265640000602082015250565b5f6129ce603e836126cb565b91506129d982612974565b604082019050919050565b5f6020820190508181035f8301526129fb816129c2565b9050919050565b5f60408284031215612a1757612a16611e22565b5b5f612a2484828501612640565b91505092915050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b6579602082015250565b5f612a876040836126cb565b9150612a9282612a2d565b604082019050919050565b5f6020820190508181035f830152612ab481612a7b565b9050919050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a206f70657261746f7220616c7265616479207265676973746572656460208201527f207075626b657900000000000000000000000000000000000000000000000000604082015250565b5f612b3b6047836126cb565b9150612b4682612abb565b606082019050919050565b5f6020820190508181035f830152612b6881612b2f565b9050919050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a207075626c6963206b657920616c726561647920726567697374657260208201527f6564000000000000000000000000000000000000000000000000000000000000604082015250565b5f612bef6042836126cb565b9150612bfa82612b6f565b606082019050919050565b5f6020820190508181035f830152612c1c81612be3565b9050919050565b5f819050919050565b612c3d612c3882611eaf565b612c23565b82525050565b82818337505050565b612c5860408383612c43565b5050565b5f612c67828b612c2c565b602082019150612c77828a612c2c565b602082019150612c878289612c2c565b602082019150612c978288612c2c565b602082019150612ca78287612c4c565b604082019150612cb78286612c4c565b604082019150612cc78285612c2c565b602082019150612cd78284612c2c565b6020820191508190509998505050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f612d2482611eaf565b9150612d2f83611eaf565b925082612d3f57612d3e612ced565b5b828206905092915050565b5f67ffffffffffffffff821115612d6457612d63611fcf565b5b602082029050919050565b5f612d81612d7c84612d4a565b61202d565b90508060208402830185811115612d9b57612d9a6124be565b5b835b81811015612dc45780612db08882612235565b845260208401935050602081019050612d9d565b5050509392505050565b5f82601f830112612de257612de1611fb7565b5b6002612def848285612d6f565b91505092915050565b5f60808284031215612e0d57612e0c61263c565b5b612e17604061202d565b90505f612e2684828501612dce565b5f830152506040612e3984828501612dce565b60208301525092915050565b5f60808284031215612e5a57612e59611e22565b5b5f612e6784828501612df8565b91505092915050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a2065697468657220746865204731207369676e61747572652069732060208201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060408201527f646f206e6f74206d617463680000000000000000000000000000000000000000606082015250565b5f612f16606c836126cb565b9150612f2182612e70565b608082019050919050565b5f6020820190508181035f830152612f4381612f0a565b9050919050565b5f8135612f568161221f565b80915050919050565b5f815f1b9050919050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff612f9584612f5f565b9350801983169250808416831791505092915050565b5f819050919050565b5f612fce612fc9612fc484611eaf565b612fab565b611eaf565b9050919050565b5f819050919050565b612fe782612fb4565b612ffa612ff382612fd5565b8354612f6a565b8255505050565b5f81015f83018061301181612f4a565b905061301d8184612fde565b50505060018101602083018061303281612f4a565b905061303e8184612fde565b5050505050565b61304f8282613001565b5050565b5f6130616020840184612235565b905092915050565b604082016130795f830183613053565b6130855f8501826121ca565b506130936020830183613053565b6130a060208501826121ca565b50505050565b5f82905092915050565b6130bc60408383612c43565b5050565b608082016130d05f8301836130a6565b6130dc5f8501826130b0565b506130ea60408301836130a6565b6130f760408501826130b0565b50505050565b5f60c0820190506131105f830185613069565b61311d60408301846130c0565b9392505050565b7f424c5341706b52656769737472792e67657441706b496e64696365734174426c5f8201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f726560208201527f2074686520666972737420757064617465000000000000000000000000000000604082015250565b5f6131a46051836126cb565b91506131af82613124565b606082019050919050565b5f6020820190508181035f8301526131d181613198565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61320f82611eaf565b915061321a83611eaf565b9250828203905081811115613232576132316131d8565b5b92915050565b5f61324282611eaf565b91505f8203613254576132536131d8565b5b600182039050919050565b7f424c5341706b52656769737472792e5f636865636b5265676973747279436f6f5f8201527f7264696e61746f723a2063616c6c6572206973206e6f7420746865207265676960208201527f7374727920636f6f7264696e61746f7200000000000000000000000000000000604082015250565b5f6132df6050836126cb565b91506132ea8261325f565b606082019050919050565b5f6020820190508181035f83015261330c816132d3565b9050919050565b7f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b5f8201527f5570646174653a2071756f72756d20646f6573206e6f74206578697374000000602082015250565b5f61336d603d836126cb565b915061337882613313565b604082019050919050565b5f6020820190508181035f83015261339a81613361565b9050919050565b7f65632d6d756c2d6661696c6564000000000000000000000000000000000000005f82015250565b5f6133d5600d836126cb565b91506133e0826133a1565b602082019050919050565b5f6020820190508181035f830152613402816133c9565b9050919050565b7f65632d6164642d6661696c6564000000000000000000000000000000000000005f82015250565b5f61343d600d836126cb565b915061344882613409565b602082019050919050565b5f6020820190508181035f83015261346a81613431565b9050919050565b5f61347b82611eaf565b915061348683611eaf565b925082820261349481611eaf565b915082820484148315176134ab576134aa6131d8565b5b5092915050565b5f6134bc82611eaf565b91506134c783611eaf565b92508282019050808211156134df576134de6131d8565b5b92915050565b7f70616972696e672d6f70636f64652d6661696c656400000000000000000000005f82015250565b5f6135196015836126cb565b9150613524826134e5565b602082019050919050565b5f6020820190508181035f8301526135468161350d565b905091905056fea264697066735822122028f026356b26e283c1cd4e7b77123df74834f754e8ac1588d20e68445660228d64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa8J8\x03\x80a8J\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x01\xABV[\x80\x80\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\0ua\0}` \x1B` \x1CV[PPPa\x02\xA8V[_`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\0\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xC3\x90a\x02VV[`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10\x15a\x01:W`\xFF__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\xFF`@Qa\x011\x91\x90a\x02\x8FV[`@Q\x80\x91\x03\x90\xA1[V[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x01i\x82a\x01@V[\x90P\x91\x90PV[_a\x01z\x82a\x01_V[\x90P\x91\x90PV[a\x01\x8A\x81a\x01pV[\x81\x14a\x01\x94W__\xFD[PV[_\x81Q\x90Pa\x01\xA5\x81a\x01\x81V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x01\xC0Wa\x01\xBFa\x01<V[[_a\x01\xCD\x84\x82\x85\x01a\x01\x97V[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FInitializable: contract is initi_\x82\x01R\x7Falizing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x02@`'\x83a\x01\xD6V[\x91Pa\x02K\x82a\x01\xE6V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x02m\x81a\x024V[\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x02\x89\x81a\x02tV[\x82RPPV[_` \x82\x01\x90Pa\x02\xA2_\x83\x01\x84a\x02\x80V[\x92\x91PPV[`\x80Qa5\x83a\x02\xC7_9_\x81\x81a\tY\x01Ra\x13\x18\x01Ra5\x83_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x1EW_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\xABW\x80c\xD5%J\x8C\x11a\0oW\x80c\xD5%J\x8C\x14a\x03\x8DW\x80c\xDE)\xFA\xC0\x14a\x03\xBDW\x80c\xDFM\t\xE0\x14a\x03\xEDW\x80c\xE8\xBB\x9A\xE6\x14a\x04\tW\x80c\xF4\xE2O\xE5\x14a\x049Wa\x01\x1EV[\x80cm\x14\xA9\x87\x14a\x02\xABW\x80cy\x16\xCE\xA6\x14a\x02\xC9W\x80c\x7F\xF8\x1A\x87\x14a\x02\xFBW\x80c\xA3\xDB\x80\xE2\x14a\x03,W\x80c\xBFy\xCEX\x14a\x03]Wa\x01\x1EV[\x80c?\xB2yR\x11a\0\xF2W\x80c?\xB2yR\x14a\x01\xCFW\x80cG\xB3\x14\xE8\x14a\x01\xEBW\x80c_a\xA8\x84\x14a\x02\x1BW\x80c`WG\xD5\x14a\x02KW\x80ch\xBC\xCA\xAC\x14a\x02{Wa\x01\x1EV[\x80b\xA1\xF4\xCB\x14a\x01\"W\x80c\x13T*N\x14a\x01SW\x80c&\xD9A\xF2\x14a\x01\x83W\x80c7~\xD9\x9D\x14a\x01\x9FW[__\xFD[a\x01<`\x04\x806\x03\x81\x01\x90a\x017\x91\x90a\x1E\x84V[a\x04UV[`@Qa\x01J\x92\x91\x90a\x1E\xC7V[`@Q\x80\x91\x03\x90\xF3[a\x01m`\x04\x806\x03\x81\x01\x90a\x01h\x91\x90a\x1E\x84V[a\x04uV[`@Qa\x01z\x91\x90a\x1F\x06V[`@Q\x80\x91\x03\x90\xF3[a\x01\x9D`\x04\x806\x03\x81\x01\x90a\x01\x98\x91\x90a\x1FUV[a\x04\xBBV[\0[a\x01\xB9`\x04\x806\x03\x81\x01\x90a\x01\xB4\x91\x90a\x1FUV[a\x06\x13V[`@Qa\x01\xC6\x91\x90a\x1F\x9EV[`@Q\x80\x91\x03\x90\xF3[a\x01\xE9`\x04\x806\x03\x81\x01\x90a\x01\xE4\x91\x90a \xF3V[a\x066V[\0[a\x02\x05`\x04\x806\x03\x81\x01\x90a\x02\0\x91\x90a!wV[a\x06\x9DV[`@Qa\x02\x12\x91\x90a!\xB1V[`@Q\x80\x91\x03\x90\xF3[a\x025`\x04\x806\x03\x81\x01\x90a\x020\x91\x90a\x1FUV[a\x06\xD6V[`@Qa\x02B\x91\x90a\"\x06V[`@Q\x80\x91\x03\x90\xF3[a\x02e`\x04\x806\x03\x81\x01\x90a\x02`\x91\x90a\"IV[a\x07\x1AV[`@Qa\x02r\x91\x90a#\x10V[`@Q\x80\x91\x03\x90\xF3[a\x02\x95`\x04\x806\x03\x81\x01\x90a\x02\x90\x91\x90a#SV[a\x07\xDBV[`@Qa\x02\xA2\x91\x90a#\xB2V[`@Q\x80\x91\x03\x90\xF3[a\x02\xB3a\tWV[`@Qa\x02\xC0\x91\x90a!\xB1V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE3`\x04\x806\x03\x81\x01\x90a\x02\xDE\x91\x90a\"IV[a\t{V[`@Qa\x02\xF2\x93\x92\x91\x90a#\xCBV[`@Q\x80\x91\x03\x90\xF3[a\x03\x15`\x04\x806\x03\x81\x01\x90a\x03\x10\x91\x90a\x1E\x84V[a\t\xE1V[`@Qa\x03#\x92\x91\x90a$\0V[`@Q\x80\x91\x03\x90\xF3[a\x03F`\x04\x806\x03\x81\x01\x90a\x03A\x91\x90a\x1FUV[a\n\xDAV[`@Qa\x03T\x92\x91\x90a\x1E\xC7V[`@Q\x80\x91\x03\x90\xF3[a\x03w`\x04\x806\x03\x81\x01\x90a\x03r\x91\x90a$hV[a\n\xFAV[`@Qa\x03\x84\x91\x90a\x1F\x06V[`@Q\x80\x91\x03\x90\xF3[a\x03\xA7`\x04\x806\x03\x81\x01\x90a\x03\xA2\x91\x90a%\x17V[a\x0F^V[`@Qa\x03\xB4\x91\x90a&\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x03\xD7`\x04\x806\x03\x81\x01\x90a\x03\xD2\x91\x90a\x1E\x84V[a\x11mV[`@Qa\x03\xE4\x91\x90a\x1F\x06V[`@Q\x80\x91\x03\x90\xF3[a\x04\x07`\x04\x806\x03\x81\x01\x90a\x04\x02\x91\x90a&\x8DV[a\x11\x82V[\0[a\x04#`\x04\x806\x03\x81\x01\x90a\x04\x1E\x91\x90a!wV[a\x12wV[`@Qa\x040\x91\x90a!\xB1V[`@Q\x80\x91\x03\x90\xF3[a\x04S`\x04\x806\x03\x81\x01\x90a\x04N\x91\x90a \xF3V[a\x12\xA7V[\0[`\x03` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01T\x90P\x82V[_`\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[a\x04\xC3a\x13\x16V[_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14a\x05\x1FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\x16\x90a'KV[`@Q\x80\x91\x03\x90\xFD[`\x04_\x82`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80_`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[a\x06>a\x13\x16V[_a\x06H\x83a\t\xE1V[P\x90Pa\x06U\x82\x82a\x13\xA6V[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06\x80\x85a\x04uV[\x84`@Qa\x06\x90\x93\x92\x91\x90a'\xC9V[`@Q\x80\x91\x03\x90\xA1PPPV[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[a\x06\xDEa\x1C\xFBV[`\x05_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[a\x07\"a\x1D\x13V[`\x04_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x07KWa\x07Ja(\x05V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[__`\x04_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x08\x06Wa\x08\x05a(\x05V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10\x15a\x08\xE3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xDA\x90a(\xA2V[`@Q\x80\x91\x03\x90\xFD[_\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x14\x80a\t\nWP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\tIW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t@\x90a)VV[`@Q\x80\x91\x03\x90\xFD[\x80_\x01Q\x91PP\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\t\x94W_\x80\xFD[\x90_R` _ \x01_\x91P\x91PP\x80_\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1B\x90\x80_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x83V[a\t\xE9a\x1C\xFBV[__`\x03_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P_`\x01_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P__\x1B\x81\x03a\n\xCDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xC4\x90a)\xE4V[`@Q\x80\x91\x03\x90\xFD[\x81\x81\x93P\x93PPP\x91P\x91V[`\x05` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01T\x90P\x82V[_a\x0B\x03a\x13\x16V[_a\x0B \x84`@\x01\x806\x03\x81\x01\x90a\x0B\x1B\x91\x90a*\x02V[a\x16\x87V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x03a\x0B\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B{\x90a*\x9DV[`@Q\x80\x91\x03\x90\xFD[__\x1B`\x01_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x14a\x0C\x05W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xFC\x90a+QV[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\xA3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x9A\x90a,\x05V[`@Q\x80\x91\x03\x90\xFD[_\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x85_\x01_\x015\x86_\x01` \x015\x87`@\x01_\x015\x88`@\x01` \x015\x89`\x80\x01_\x01\x8A`\x80\x01`@\x01\x8A_\x015\x8B` \x015`@Q` \x01a\r\x0E\x98\x97\x96\x95\x94\x93\x92\x91\x90a,\\V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\r0\x91\x90a-\x1AV[\x90Pa\r\xDCa\r\x7Fa\r^\x83\x88`@\x01\x806\x03\x81\x01\x90a\rP\x91\x90a*\x02V[a\x16\x9F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87_\x01\x806\x03\x81\x01\x90a\rq\x91\x90a*\x02V[a\x17s\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\x87a\x18lV[a\r\xC3a\r\xA4\x85a\r\x96a\x196V[a\x16\x9F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88\x806\x03\x81\x01\x90a\r\xB5\x91\x90a*\x02V[a\x17s\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\x80\x01\x806\x03\x81\x01\x90a\r\xD7\x91\x90a.EV[a\x19ZV[a\x0E\x1BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\x12\x90a/,V[`@Q\x80\x91\x03\x90\xFD[\x84`@\x01`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a\x0Eh\x91\x90a0EV[\x90PP\x81`\x01_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x85`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x86`@\x01\x87`\x80\x01`@Qa\x0FJ\x92\x91\x90a0\xFDV[`@Q\x80\x91\x03\x90\xA2\x81\x92PPP\x93\x92PPPV[``_\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F~Wa\x0F}a\x1F\xCFV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\xACW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x85\x85\x90P\x81\x10\x15a\x11aW_\x86\x86\x83\x81\x81\x10a\x0F\xD3Wa\x0F\xD2a(\x05V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x14\x80a\x10VWP`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x81T\x81\x10a\x102Wa\x101a(\x05V[[\x90_R` _ \x01_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x10\x96W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x8D\x90a1\xBAV[`@Q\x80\x91\x03\x90\xFD[_\x81\x90P[_\x81\x11\x15a\x11QW\x86`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a\x10\xC8\x91\x90a2\x05V[\x81T\x81\x10a\x10\xD9Wa\x10\xD8a(\x05V[[\x90_R` _ \x01_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a\x11>W`\x01\x81a\x11\x0C\x91\x90a2\x05V[\x85\x85\x81Q\x81\x10a\x11\x1FWa\x11\x1Ea(\x05V[[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x11QV[\x80\x80a\x11I\x90a28V[\x91PPa\x10\x9BV[PPP\x80\x80`\x01\x01\x91PPa\x0F\xB4V[P\x80\x91PP\x93\x92PPPV[`\x01` R\x80_R`@_ _\x91P\x90PT\x81V[_a\x11\x8C\x82a\x16\x87V[\x90P\x80`\x01_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\x03_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PPPPPV[`\x02` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x12\xAFa\x13\x16V[_a\x12\xB9\x83a\t\xE1V[P\x90Pa\x12\xCE\x82a\x12\xC9\x83a\x1CCV[a\x13\xA6V[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x12\xF9\x85a\x04uV[\x84`@Qa\x13\t\x93\x92\x91\x90a'\xC9V[`@Q\x80\x91\x03\x90\xA1PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x13\xA4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\x9B\x90a2\xF5V[`@Q\x80\x91\x03\x90\xFD[V[a\x13\xAEa\x1C\xFBV[__\x90P[\x83Q\x81\x10\x15a\x16\x81W_\x84\x82\x81Q\x81\x10a\x13\xD0Wa\x13\xCFa(\x05V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a\x14@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x147\x90a3\x83V[`@Q\x80\x91\x03\x90\xFD[a\x14\x87\x85`\x05_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x17s\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93P\x83`\x05_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP_a\x14\xC0\x85a\x16\x87V[\x90P_`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x84a\x14\xE7\x91\x90a2\x05V[\x81T\x81\x10a\x14\xF8Wa\x14\xF7a(\x05V[[\x90_R` _ \x01\x90PCc\xFF\xFF\xFF\xFF\x16\x81_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a\x15_W\x81\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UPa\x16pV[C\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[PPPP\x80\x80`\x01\x01\x91PPa\x13\xB3V[PPPPV[_\x81Q_R\x81` \x01Q` R`@_ \x90P\x91\x90PV[a\x16\xA7a\x1C\xFBV[a\x16\xAFa\x1DHV[\x83_\x01Q\x81_`\x03\x81\x10a\x16\xC6Wa\x16\xC5a(\x05V[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x03\x81\x10a\x16\xE8Wa\x16\xE7a(\x05V[[` \x02\x01\x81\x81RPP\x82\x81`\x02`\x03\x81\x10a\x17\x06Wa\x17\x05a(\x05V[[` \x02\x01\x81\x81RPP_`@\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x17*W\xFE[P\x80a\x17kW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17b\x90a3\xEBV[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x17{a\x1C\xFBV[a\x17\x83a\x1DjV[\x83_\x01Q\x81_`\x04\x81\x10a\x17\x9AWa\x17\x99a(\x05V[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x04\x81\x10a\x17\xBCWa\x17\xBBa(\x05V[[` \x02\x01\x81\x81RPP\x82_\x01Q\x81`\x02`\x04\x81\x10a\x17\xDDWa\x17\xDCa(\x05V[[` \x02\x01\x81\x81RPP\x82` \x01Q\x81`\x03`\x04\x81\x10a\x17\xFFWa\x17\xFEa(\x05V[[` \x02\x01\x81\x81RPP_`@\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x18#W\xFE[P\x80a\x18dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18[\x90a4SV[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x18ta\x1D\x8CV[`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x81R` \x01\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D\x81RP\x81RP\x90P\x90V[a\x19>a\x1C\xFBV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x02\x81RP\x90P\x90V[__`@Q\x80`@\x01`@R\x80\x87\x81R` \x01\x85\x81RP\x90P_`@Q\x80`@\x01`@R\x80\x87\x81R` \x01\x85\x81RP\x90Pa\x19\x93a\x1D\xB2V[__\x90P[`\x02\x81\x10\x15a\x1B\xB1W_`\x06\x82a\x19\xAF\x91\x90a4qV[\x90P\x84\x82`\x02\x81\x10a\x19\xC4Wa\x19\xC3a(\x05V[[` \x02\x01Q_\x01Q\x83_\x83a\x19\xD9\x91\x90a4\xB2V[`\x0C\x81\x10a\x19\xEAWa\x19\xE9a(\x05V[[` \x02\x01\x81\x81RPP\x84\x82`\x02\x81\x10a\x1A\x06Wa\x1A\x05a(\x05V[[` \x02\x01Q` \x01Q\x83`\x01\x83a\x1A\x1D\x91\x90a4\xB2V[`\x0C\x81\x10a\x1A.Wa\x1A-a(\x05V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x1AJWa\x1AIa(\x05V[[` \x02\x01Q_\x01Q_`\x02\x81\x10a\x1AdWa\x1Aca(\x05V[[` \x02\x01Q\x83`\x02\x83a\x1Aw\x91\x90a4\xB2V[`\x0C\x81\x10a\x1A\x88Wa\x1A\x87a(\x05V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x1A\xA4Wa\x1A\xA3a(\x05V[[` \x02\x01Q_\x01Q`\x01`\x02\x81\x10a\x1A\xBFWa\x1A\xBEa(\x05V[[` \x02\x01Q\x83`\x03\x83a\x1A\xD2\x91\x90a4\xB2V[`\x0C\x81\x10a\x1A\xE3Wa\x1A\xE2a(\x05V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x1A\xFFWa\x1A\xFEa(\x05V[[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x1B\x1AWa\x1B\x19a(\x05V[[` \x02\x01Q\x83`\x04\x83a\x1B-\x91\x90a4\xB2V[`\x0C\x81\x10a\x1B>Wa\x1B=a(\x05V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x1BZWa\x1BYa(\x05V[[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x1BvWa\x1Bua(\x05V[[` \x02\x01Q\x83`\x05\x83a\x1B\x89\x91\x90a4\xB2V[`\x0C\x81\x10a\x1B\x9AWa\x1B\x99a(\x05V[[` \x02\x01\x81\x81RPPP\x80\x80`\x01\x01\x91PPa\x19\x98V[Pa\x1B\xBAa\x1D\xD5V[_` \x82` `\x0C\x02\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x1B\xD8W\xFE[P\x80a\x1C\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\x10\x90a5/V[`@Q\x80\x91\x03\x90\xFD[_\x82_`\x01\x81\x10a\x1C-Wa\x1C,a(\x05V[[` \x02\x01Q\x14\x15\x95PPPPPP\x94\x93PPPPV[a\x1CKa\x1C\xFBV[_\x82_\x01Q\x14\x80\x15a\x1C`WP_\x82` \x01Q\x14[\x15a\x1C\x81W`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x1C\xF6V[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x1C\xC5\x91\x90a-\x1AV[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1C\xF0\x91\x90a2\x05V[\x81RP\x90P[\x91\x90PV[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80a\x1D\x9Fa\x1D\xF7V[\x81R` \x01a\x1D\xACa\x1D\xF7V[\x81RP\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x1ES\x82a\x1E*V[\x90P\x91\x90PV[a\x1Ec\x81a\x1EIV[\x81\x14a\x1EmW__\xFD[PV[_\x815\x90Pa\x1E~\x81a\x1EZV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1E\x99Wa\x1E\x98a\x1E\"V[[_a\x1E\xA6\x84\x82\x85\x01a\x1EpV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x1E\xC1\x81a\x1E\xAFV[\x82RPPV[_`@\x82\x01\x90Pa\x1E\xDA_\x83\x01\x85a\x1E\xB8V[a\x1E\xE7` \x83\x01\x84a\x1E\xB8V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[a\x1F\0\x81a\x1E\xEEV[\x82RPPV[_` \x82\x01\x90Pa\x1F\x19_\x83\x01\x84a\x1E\xF7V[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x1F4\x81a\x1F\x1FV[\x81\x14a\x1F>W__\xFD[PV[_\x815\x90Pa\x1FO\x81a\x1F+V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1FjWa\x1Fia\x1E\"V[[_a\x1Fw\x84\x82\x85\x01a\x1FAV[\x91PP\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x1F\x98\x81a\x1F\x80V[\x82RPPV[_` \x82\x01\x90Pa\x1F\xB1_\x83\x01\x84a\x1F\x8FV[\x92\x91PPV[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a \x05\x82a\x1F\xBFV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a $Wa #a\x1F\xCFV[[\x80`@RPPPV[_a 6a\x1E\x19V[\x90Pa B\x82\x82a\x1F\xFCV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a aWa `a\x1F\xCFV[[a j\x82a\x1F\xBFV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a \x97a \x92\x84a GV[a -V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a \xB3Wa \xB2a\x1F\xBBV[[a \xBE\x84\x82\x85a wV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a \xDAWa \xD9a\x1F\xB7V[[\x815a \xEA\x84\x82` \x86\x01a \x85V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a!\tWa!\x08a\x1E\"V[[_a!\x16\x85\x82\x86\x01a\x1EpV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!7Wa!6a\x1E&V[[a!C\x85\x82\x86\x01a \xC6V[\x91PP\x92P\x92\x90PV[a!V\x81a\x1E\xEEV[\x81\x14a!`W__\xFD[PV[_\x815\x90Pa!q\x81a!MV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a!\x8CWa!\x8Ba\x1E\"V[[_a!\x99\x84\x82\x85\x01a!cV[\x91PP\x92\x91PPV[a!\xAB\x81a\x1EIV[\x82RPPV[_` \x82\x01\x90Pa!\xC4_\x83\x01\x84a!\xA2V[\x92\x91PPV[a!\xD3\x81a\x1E\xAFV[\x82RPPV[`@\x82\x01_\x82\x01Qa!\xED_\x85\x01\x82a!\xCAV[P` \x82\x01Qa\"\0` \x85\x01\x82a!\xCAV[PPPPV[_`@\x82\x01\x90Pa\"\x19_\x83\x01\x84a!\xD9V[\x92\x91PPV[a\"(\x81a\x1E\xAFV[\x81\x14a\"2W__\xFD[PV[_\x815\x90Pa\"C\x81a\"\x1FV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\"_Wa\"^a\x1E\"V[[_a\"l\x85\x82\x86\x01a\x1FAV[\x92PP` a\"}\x85\x82\x86\x01a\"5V[\x91PP\x92P\x92\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\"\xBB\x81a\"\x87V[\x82RPPV[a\"\xCA\x81a\x1F\x80V[\x82RPPV[``\x82\x01_\x82\x01Qa\"\xE4_\x85\x01\x82a\"\xB2V[P` \x82\x01Qa\"\xF7` \x85\x01\x82a\"\xC1V[P`@\x82\x01Qa#\n`@\x85\x01\x82a\"\xC1V[PPPPV[_``\x82\x01\x90Pa##_\x83\x01\x84a\"\xD0V[\x92\x91PPV[a#2\x81a\x1F\x80V[\x81\x14a#<W__\xFD[PV[_\x815\x90Pa#M\x81a#)V[\x92\x91PPV[___``\x84\x86\x03\x12\x15a#jWa#ia\x1E\"V[[_a#w\x86\x82\x87\x01a\x1FAV[\x93PP` a#\x88\x86\x82\x87\x01a#?V[\x92PP`@a#\x99\x86\x82\x87\x01a\"5V[\x91PP\x92P\x92P\x92V[a#\xAC\x81a\"\x87V[\x82RPPV[_` \x82\x01\x90Pa#\xC5_\x83\x01\x84a#\xA3V[\x92\x91PPV[_``\x82\x01\x90Pa#\xDE_\x83\x01\x86a#\xA3V[a#\xEB` \x83\x01\x85a\x1F\x8FV[a#\xF8`@\x83\x01\x84a\x1F\x8FV[\x94\x93PPPPV[_``\x82\x01\x90Pa$\x13_\x83\x01\x85a!\xD9V[a$ `@\x83\x01\x84a\x1E\xF7V[\x93\x92PPPV[__\xFD[_a\x01\0\x82\x84\x03\x12\x15a$AWa$@a$'V[[\x81\x90P\x92\x91PPV[_`@\x82\x84\x03\x12\x15a$_Wa$^a$'V[[\x81\x90P\x92\x91PPV[___a\x01`\x84\x86\x03\x12\x15a$\x80Wa$\x7Fa\x1E\"V[[_a$\x8D\x86\x82\x87\x01a\x1EpV[\x93PP` a$\x9E\x86\x82\x87\x01a$+V[\x92PPa\x01 a$\xB0\x86\x82\x87\x01a$JV[\x91PP\x92P\x92P\x92V[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a$\xD7Wa$\xD6a\x1F\xB7V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xF4Wa$\xF3a$\xBAV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a%\x10Wa%\x0Fa$\xBEV[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a%.Wa%-a\x1E\"V[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%KWa%Ja\x1E&V[[a%W\x86\x82\x87\x01a$\xC2V[\x93P\x93PP` a%j\x86\x82\x87\x01a\"5V[\x91PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a%\xA8\x83\x83a\"\xC1V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a%\xCA\x82a%tV[a%\xD4\x81\x85a%~V[\x93Pa%\xDF\x83a%\x8EV[\x80_[\x83\x81\x10\x15a&\x0FW\x81Qa%\xF6\x88\x82a%\x9DV[\x97Pa&\x01\x83a%\xB4V[\x92PP`\x01\x81\x01\x90Pa%\xE2V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra&4\x81\x84a%\xC0V[\x90P\x92\x91PPV[__\xFD[_`@\x82\x84\x03\x12\x15a&UWa&Ta&<V[[a&_`@a -V[\x90P_a&n\x84\x82\x85\x01a\"5V[_\x83\x01RP` a&\x81\x84\x82\x85\x01a\"5V[` \x83\x01RP\x92\x91PPV[__``\x83\x85\x03\x12\x15a&\xA3Wa&\xA2a\x1E\"V[[_a&\xB0\x85\x82\x86\x01a\x1EpV[\x92PP` a&\xC1\x85\x82\x86\x01a&@V[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBLSApkRegistry.initializeQuorum:_\x82\x01R\x7F quorum already exists\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a'5`6\x83a&\xCBV[\x91Pa'@\x82a&\xDBV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra'b\x81a')V[\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a'\x9B\x82a'iV[a'\xA5\x81\x85a'sV[\x93Pa'\xB5\x81\x85` \x86\x01a'\x83V[a'\xBE\x81a\x1F\xBFV[\x84\x01\x91PP\x92\x91PPV[_``\x82\x01\x90Pa'\xDC_\x83\x01\x86a!\xA2V[a'\xE9` \x83\x01\x85a\x1E\xF7V[\x81\x81\x03`@\x83\x01Ra'\xFB\x81\x84a'\x91V[\x90P\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FBLSApkRegistry.getApkHashAtBlock_\x82\x01R\x7FNumberAndIndex: index too recent` \x82\x01RPV[_a(\x8C`@\x83a&\xCBV[\x91Pa(\x97\x82a(2V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra(\xB9\x81a(\x80V[\x90P\x91\x90PV[\x7FBLSApkRegistry.getApkHashAtBlock_\x82\x01R\x7FNumberAndIndex: not latest apk u` \x82\x01R\x7Fpdate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a)@`E\x83a&\xCBV[\x91Pa)K\x82a(\xC0V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra)m\x81a)4V[\x90P\x91\x90PV[\x7FBLSApkRegistry.getRegisteredPubk_\x82\x01R\x7Fey: operator is not registered\0\0` \x82\x01RPV[_a)\xCE`>\x83a&\xCBV[\x91Pa)\xD9\x82a)tV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra)\xFB\x81a)\xC2V[\x90P\x91\x90PV[_`@\x82\x84\x03\x12\x15a*\x17Wa*\x16a\x1E\"V[[_a*$\x84\x82\x85\x01a&@V[\x91PP\x92\x91PPV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: cannot register zero pubkey` \x82\x01RPV[_a*\x87`@\x83a&\xCBV[\x91Pa*\x92\x82a*-V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra*\xB4\x81a*{V[\x90P\x91\x90PV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: operator already registered` \x82\x01R\x7F pubkey\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a+;`G\x83a&\xCBV[\x91Pa+F\x82a*\xBBV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra+h\x81a+/V[\x90P\x91\x90PV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: public key already register` \x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a+\xEF`B\x83a&\xCBV[\x91Pa+\xFA\x82a+oV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra,\x1C\x81a+\xE3V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a,=a,8\x82a\x1E\xAFV[a,#V[\x82RPPV[\x82\x81\x837PPPV[a,X`@\x83\x83a,CV[PPV[_a,g\x82\x8Ba,,V[` \x82\x01\x91Pa,w\x82\x8Aa,,V[` \x82\x01\x91Pa,\x87\x82\x89a,,V[` \x82\x01\x91Pa,\x97\x82\x88a,,V[` \x82\x01\x91Pa,\xA7\x82\x87a,LV[`@\x82\x01\x91Pa,\xB7\x82\x86a,LV[`@\x82\x01\x91Pa,\xC7\x82\x85a,,V[` \x82\x01\x91Pa,\xD7\x82\x84a,,V[` \x82\x01\x91P\x81\x90P\x99\x98PPPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a-$\x82a\x1E\xAFV[\x91Pa-/\x83a\x1E\xAFV[\x92P\x82a-?Wa->a,\xEDV[[\x82\x82\x06\x90P\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a-dWa-ca\x1F\xCFV[[` \x82\x02\x90P\x91\x90PV[_a-\x81a-|\x84a-JV[a -V[\x90P\x80` \x84\x02\x83\x01\x85\x81\x11\x15a-\x9BWa-\x9Aa$\xBEV[[\x83[\x81\x81\x10\x15a-\xC4W\x80a-\xB0\x88\x82a\"5V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa-\x9DV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a-\xE2Wa-\xE1a\x1F\xB7V[[`\x02a-\xEF\x84\x82\x85a-oV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a.\rWa.\x0Ca&<V[[a.\x17`@a -V[\x90P_a.&\x84\x82\x85\x01a-\xCEV[_\x83\x01RP`@a.9\x84\x82\x85\x01a-\xCEV[` \x83\x01RP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a.ZWa.Ya\x1E\"V[[_a.g\x84\x82\x85\x01a-\xF8V[\x91PP\x92\x91PPV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: either the G1 signature is ` \x82\x01R\x7Fwrong, or G1 and G2 private key `@\x82\x01R\x7Fdo not match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_a/\x16`l\x83a&\xCBV[\x91Pa/!\x82a.pV[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra/C\x81a/\nV[\x90P\x91\x90PV[_\x815a/V\x81a\"\x1FV[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa/\x95\x84a/_V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a/\xCEa/\xC9a/\xC4\x84a\x1E\xAFV[a/\xABV[a\x1E\xAFV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a/\xE7\x82a/\xB4V[a/\xFAa/\xF3\x82a/\xD5V[\x83Ta/jV[\x82UPPPV[_\x81\x01_\x83\x01\x80a0\x11\x81a/JV[\x90Pa0\x1D\x81\x84a/\xDEV[PPP`\x01\x81\x01` \x83\x01\x80a02\x81a/JV[\x90Pa0>\x81\x84a/\xDEV[PPPPPV[a0O\x82\x82a0\x01V[PPV[_a0a` \x84\x01\x84a\"5V[\x90P\x92\x91PPV[`@\x82\x01a0y_\x83\x01\x83a0SV[a0\x85_\x85\x01\x82a!\xCAV[Pa0\x93` \x83\x01\x83a0SV[a0\xA0` \x85\x01\x82a!\xCAV[PPPPV[_\x82\x90P\x92\x91PPV[a0\xBC`@\x83\x83a,CV[PPV[`\x80\x82\x01a0\xD0_\x83\x01\x83a0\xA6V[a0\xDC_\x85\x01\x82a0\xB0V[Pa0\xEA`@\x83\x01\x83a0\xA6V[a0\xF7`@\x85\x01\x82a0\xB0V[PPPPV[_`\xC0\x82\x01\x90Pa1\x10_\x83\x01\x85a0iV[a1\x1D`@\x83\x01\x84a0\xC0V[\x93\x92PPPV[\x7FBLSApkRegistry.getApkIndicesAtBl_\x82\x01R\x7FockNumber: blockNumber is before` \x82\x01R\x7F the first update\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a1\xA4`Q\x83a&\xCBV[\x91Pa1\xAF\x82a1$V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra1\xD1\x81a1\x98V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a2\x0F\x82a\x1E\xAFV[\x91Pa2\x1A\x83a\x1E\xAFV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a22Wa21a1\xD8V[[\x92\x91PPV[_a2B\x82a\x1E\xAFV[\x91P_\x82\x03a2TWa2Sa1\xD8V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FBLSApkRegistry._checkRegistryCoo_\x82\x01R\x7Frdinator: caller is not the regi` \x82\x01R\x7Fstry coordinator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a2\xDF`P\x83a&\xCBV[\x91Pa2\xEA\x82a2_V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3\x0C\x81a2\xD3V[\x90P\x91\x90PV[\x7FBLSApkRegistry._processQuorumApk_\x82\x01R\x7FUpdate: quorum does not exist\0\0\0` \x82\x01RPV[_a3m`=\x83a&\xCBV[\x91Pa3x\x82a3\x13V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3\x9A\x81a3aV[\x90P\x91\x90PV[\x7Fec-mul-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a3\xD5`\r\x83a&\xCBV[\x91Pa3\xE0\x82a3\xA1V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra4\x02\x81a3\xC9V[\x90P\x91\x90PV[\x7Fec-add-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a4=`\r\x83a&\xCBV[\x91Pa4H\x82a4\tV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra4j\x81a41V[\x90P\x91\x90PV[_a4{\x82a\x1E\xAFV[\x91Pa4\x86\x83a\x1E\xAFV[\x92P\x82\x82\x02a4\x94\x81a\x1E\xAFV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a4\xABWa4\xAAa1\xD8V[[P\x92\x91PPV[_a4\xBC\x82a\x1E\xAFV[\x91Pa4\xC7\x83a\x1E\xAFV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a4\xDFWa4\xDEa1\xD8V[[\x92\x91PPV[\x7Fpairing-opcode-failed\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a5\x19`\x15\x83a&\xCBV[\x91Pa5$\x82a4\xE5V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra5F\x81a5\rV[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 (\xF0&5k&\xE2\x83\xC1\xCDN{w\x12=\xF7H4\xF7T\xE8\xAC\x15\x88\xD2\x0EhDV`\"\x8DdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061011e575f3560e01c80636d14a987116100ab578063d5254a8c1161006f578063d5254a8c1461038d578063de29fac0146103bd578063df4d09e0146103ed578063e8bb9ae614610409578063f4e24fe5146104395761011e565b80636d14a987146102ab5780637916cea6146102c95780637ff81a87146102fb578063a3db80e21461032c578063bf79ce581461035d5761011e565b80633fb27952116100f25780633fb27952146101cf57806347b314e8146101eb5780635f61a8841461021b578063605747d51461024b57806368bccaac1461027b5761011e565b8062a1f4cb1461012257806313542a4e1461015357806326d941f214610183578063377ed99d1461019f575b5f5ffd5b61013c60048036038101906101379190611e84565b610455565b60405161014a929190611ec7565b60405180910390f35b61016d60048036038101906101689190611e84565b610475565b60405161017a9190611f06565b60405180910390f35b61019d60048036038101906101989190611f55565b6104bb565b005b6101b960048036038101906101b49190611f55565b610613565b6040516101c69190611f9e565b60405180910390f35b6101e960048036038101906101e491906120f3565b610636565b005b61020560048036038101906102009190612177565b61069d565b60405161021291906121b1565b60405180910390f35b61023560048036038101906102309190611f55565b6106d6565b6040516102429190612206565b60405180910390f35b61026560048036038101906102609190612249565b61071a565b6040516102729190612310565b60405180910390f35b61029560048036038101906102909190612353565b6107db565b6040516102a291906123b2565b60405180910390f35b6102b3610957565b6040516102c091906121b1565b60405180910390f35b6102e360048036038101906102de9190612249565b61097b565b6040516102f2939291906123cb565b60405180910390f35b61031560048036038101906103109190611e84565b6109e1565b604051610323929190612400565b60405180910390f35b61034660048036038101906103419190611f55565b610ada565b604051610354929190611ec7565b60405180910390f35b61037760048036038101906103729190612468565b610afa565b6040516103849190611f06565b60405180910390f35b6103a760048036038101906103a29190612517565b610f5e565b6040516103b4919061261c565b60405180910390f35b6103d760048036038101906103d29190611e84565b61116d565b6040516103e49190611f06565b60405180910390f35b6104076004803603810190610402919061268d565b611182565b005b610423600480360381019061041e9190612177565b611277565b60405161043091906121b1565b60405180910390f35b610453600480360381019061044e91906120f3565b6112a7565b005b6003602052805f5260405f205f91509050805f0154908060010154905082565b5f60015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20549050919050565b6104c3611316565b5f60045f8360ff1660ff1681526020019081526020015f20805490501461051f576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016105169061274b565b60405180910390fd5b60045f8260ff1660ff1681526020019081526020015f2060405180606001604052805f60401b67ffffffffffffffff191681526020014363ffffffff1681526020015f63ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c02179055506020820151815f0160186101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f01601c6101000a81548163ffffffff021916908363ffffffff160217905550505050565b5f60045f8360ff1660ff1681526020019081526020015f20805490509050919050565b61063e611316565b5f610648836109e1565b50905061065582826113a6565b7f73a2b7fb844724b971802ae9b15db094d4b7192df9d7350e14eb466b9b22eb4e8361068085610475565b84604051610690939291906127c9565b60405180910390a1505050565b5f60025f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050919050565b6106de611cfb565b60055f8360ff1660ff1681526020019081526020015f206040518060400160405290815f82015481526020016001820154815250509050919050565b610722611d13565b60045f8460ff1660ff1681526020019081526020015f20828154811061074b5761074a612805565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900460401b67ffffffffffffffff191667ffffffffffffffff191681526020015f820160189054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f8201601c9054906101000a900463ffffffff1663ffffffff1663ffffffff1681525050905092915050565b5f5f60045f8660ff1660ff1681526020019081526020015f20838154811061080657610805612805565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900460401b67ffffffffffffffff191667ffffffffffffffff191681526020015f820160189054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f8201601c9054906101000a900463ffffffff1663ffffffff1663ffffffff16815250509050806020015163ffffffff168463ffffffff1610156108e3576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016108da906128a2565b60405180910390fd5b5f816040015163ffffffff16148061090a5750806040015163ffffffff168463ffffffff16105b610949576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161094090612956565b60405180910390fd5b805f01519150509392505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b6004602052815f5260405f208181548110610994575f80fd5b905f5260205f20015f9150915050805f015f9054906101000a900460401b90805f0160189054906101000a900463ffffffff1690805f01601c9054906101000a900463ffffffff16905083565b6109e9611cfb565b5f5f60035f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060400160405290815f820154815260200160018201548152505090505f60015f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205490505f5f1b8103610acd576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610ac4906129e4565b60405180910390fd5b8181935093505050915091565b6005602052805f5260405f205f91509050805f0154908060010154905082565b5f610b03611316565b5f610b2084604001803603810190610b1b9190612a02565b611687565b90507fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb58103610b84576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610b7b90612a9d565b60405180910390fd5b5f5f1b60015f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205414610c05576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610bfc90612b51565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff1660025f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614610ca3576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610c9a90612c05565b60405180910390fd5b5f7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001855f015f0135865f0160200135876040015f01358860400160200135896080015f018a6080016040018a5f01358b60200135604051602001610d0e989796959493929190612c5c565b604051602081830303815290604052805190602001205f1c610d309190612d1a565b9050610ddc610d7f610d5e8388604001803603810190610d509190612a02565b61169f90919063ffffffff16565b875f01803603810190610d719190612a02565b61177390919063ffffffff16565b610d8761186c565b610dc3610da485610d96611936565b61169f90919063ffffffff16565b88803603810190610db59190612a02565b61177390919063ffffffff16565b88608001803603810190610dd79190612e45565b61195a565b610e1b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e1290612f2c565b60405180910390fd5b8460400160035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181610e689190613045565b9050508160015f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508560025f8481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508573ffffffffffffffffffffffffffffffffffffffff167fe3fb6613af2e8930cf85d47fcf6db10192224a64c6cbe8023e0eee1ba38280418660400187608001604051610f4a9291906130fd565b60405180910390a281925050509392505050565b60605f8484905067ffffffffffffffff811115610f7e57610f7d611fcf565b5b604051908082528060200260200182016040528015610fac5781602001602082028036833780820191505090505b5090505f5f90505b85859050811015611161575f868683818110610fd357610fd2612805565b5b9050013560f81c60f81b60f81c90505f60045f8360ff1660ff1681526020019081526020015f208054905090505f811480611056575060045f8360ff1660ff1681526020019081526020015f205f8154811061103257611031612805565b5b905f5260205f20015f0160189054906101000a900463ffffffff1663ffffffff1686105b15611096576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161108d906131ba565b60405180910390fd5b5f8190505b5f811115611151578660045f8560ff1660ff1681526020019081526020015f206001836110c89190613205565b815481106110d9576110d8612805565b5b905f5260205f20015f0160189054906101000a900463ffffffff1663ffffffff161161113e5760018161110c9190613205565b85858151811061111f5761111e612805565b5b602002602001019063ffffffff16908163ffffffff1681525050611151565b808061114990613238565b91505061109b565b5050508080600101915050610fb4565b50809150509392505050565b6001602052805f5260405f205f915090505481565b5f61118c82611687565b90508060015f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508260025f8381526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508160035f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f015560208201518160010155905050505050565b6002602052805f5260405f205f915054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6112af611316565b5f6112b9836109e1565b5090506112ce826112c983611c43565b6113a6565b7ff843ecd53a563675e62107be1494fdde4a3d49aeedaf8d88c616d85346e3500e836112f985610475565b84604051611309939291906127c9565b60405180910390a1505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146113a4576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161139b906132f5565b60405180910390fd5b565b6113ae611cfb565b5f5f90505b8351811015611681575f8482815181106113d0576113cf612805565b5b602001015160f81c60f81b60f81c90505f60045f8360ff1660ff1681526020019081526020015f208054905090505f8103611440576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161143790613383565b60405180910390fd5b6114878560055f8560ff1660ff1681526020019081526020015f206040518060400160405290815f820154815260200160018201548152505061177390919063ffffffff16565b93508360055f8460ff1660ff1681526020019081526020015f205f820151815f0155602082015181600101559050505f6114c085611687565b90505f60045f8560ff1660ff1681526020019081526020015f206001846114e79190613205565b815481106114f8576114f7612805565b5b905f5260205f200190504363ffffffff16815f0160189054906101000a900463ffffffff1663ffffffff160361155f5781815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c0217905550611670565b43815f01601c6101000a81548163ffffffff021916908363ffffffff16021790555060045f8560ff1660ff1681526020019081526020015f2060405180606001604052808467ffffffffffffffff191681526020014363ffffffff1681526020015f63ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548177ffffffffffffffffffffffffffffffffffffffffffffffff021916908360401c02179055506020820151815f0160186101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f01601c6101000a81548163ffffffff021916908363ffffffff16021790555050505b5050505080806001019150506113b3565b50505050565b5f81515f52816020015160205260405f209050919050565b6116a7611cfb565b6116af611d48565b835f0151815f600381106116c6576116c5612805565b5b6020020181815250508360200151816001600381106116e8576116e7612805565b5b602002018181525050828160026003811061170657611705612805565b5b6020020181815250505f60408360608460076107d05a03fa9050805f810361172a57fe5b508061176b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611762906133eb565b60405180910390fd5b505092915050565b61177b611cfb565b611783611d6a565b835f0151815f6004811061179a57611799612805565b5b6020020181815250508360200151816001600481106117bc576117bb612805565b5b602002018181525050825f0151816002600481106117dd576117dc612805565b5b6020020181815250508260200151816003600481106117ff576117fe612805565b5b6020020181815250505f60408360808460066107d05a03fa9050805f810361182357fe5b5080611864576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161185b90613453565b60405180910390fd5b505092915050565b611874611d8c565b604051806040016040528060405180604001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed815250815260200160405180604001604052807f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec81526020017f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d815250815250905090565b61193e611cfb565b6040518060400160405280600181526020016002815250905090565b5f5f60405180604001604052808781526020018581525090505f6040518060400160405280878152602001858152509050611993611db2565b5f5f90505b6002811015611bb1575f6006826119af9190613471565b90508482600281106119c4576119c3612805565b5b60200201515f0151835f836119d991906134b2565b600c81106119ea576119e9612805565b5b602002018181525050848260028110611a0657611a05612805565b5b60200201516020015183600183611a1d91906134b2565b600c8110611a2e57611a2d612805565b5b602002018181525050838260028110611a4a57611a49612805565b5b60200201515f01515f60028110611a6457611a63612805565b5b602002015183600283611a7791906134b2565b600c8110611a8857611a87612805565b5b602002018181525050838260028110611aa457611aa3612805565b5b60200201515f0151600160028110611abf57611abe612805565b5b602002015183600383611ad291906134b2565b600c8110611ae357611ae2612805565b5b602002018181525050838260028110611aff57611afe612805565b5b6020020151602001515f60028110611b1a57611b19612805565b5b602002015183600483611b2d91906134b2565b600c8110611b3e57611b3d612805565b5b602002018181525050838260028110611b5a57611b59612805565b5b602002015160200151600160028110611b7657611b75612805565b5b602002015183600583611b8991906134b2565b600c8110611b9a57611b99612805565b5b602002018181525050508080600101915050611998565b50611bba611dd5565b5f6020826020600c028560086107d05a03fa9050805f8103611bd857fe5b5080611c19576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611c109061352f565b60405180910390fd5b5f825f60018110611c2d57611c2c612805565b5b6020020151141595505050505050949350505050565b611c4b611cfb565b5f825f0151148015611c6057505f8260200151145b15611c815760405180604001604052805f81526020015f8152509050611cf6565b6040518060400160405280835f015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478460200151611cc59190612d1a565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611cf09190613205565b81525090505b919050565b60405180604001604052805f81526020015f81525090565b60405180606001604052805f67ffffffffffffffff191681526020015f63ffffffff1681526020015f63ffffffff1681525090565b6040518060600160405280600390602082028036833780820191505090505090565b6040518060800160405280600490602082028036833780820191505090505090565b6040518060400160405280611d9f611df7565b8152602001611dac611df7565b81525090565b604051806101800160405280600c90602082028036833780820191505090505090565b6040518060200160405280600190602082028036833780820191505090505090565b6040518060400160405280600290602082028036833780820191505090505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f611e5382611e2a565b9050919050565b611e6381611e49565b8114611e6d575f5ffd5b50565b5f81359050611e7e81611e5a565b92915050565b5f60208284031215611e9957611e98611e22565b5b5f611ea684828501611e70565b91505092915050565b5f819050919050565b611ec181611eaf565b82525050565b5f604082019050611eda5f830185611eb8565b611ee76020830184611eb8565b9392505050565b5f819050919050565b611f0081611eee565b82525050565b5f602082019050611f195f830184611ef7565b92915050565b5f60ff82169050919050565b611f3481611f1f565b8114611f3e575f5ffd5b50565b5f81359050611f4f81611f2b565b92915050565b5f60208284031215611f6a57611f69611e22565b5b5f611f7784828501611f41565b91505092915050565b5f63ffffffff82169050919050565b611f9881611f80565b82525050565b5f602082019050611fb15f830184611f8f565b92915050565b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61200582611fbf565b810181811067ffffffffffffffff8211171561202457612023611fcf565b5b80604052505050565b5f612036611e19565b90506120428282611ffc565b919050565b5f67ffffffffffffffff82111561206157612060611fcf565b5b61206a82611fbf565b9050602081019050919050565b828183375f83830152505050565b5f61209761209284612047565b61202d565b9050828152602081018484840111156120b3576120b2611fbb565b5b6120be848285612077565b509392505050565b5f82601f8301126120da576120d9611fb7565b5b81356120ea848260208601612085565b91505092915050565b5f5f6040838503121561210957612108611e22565b5b5f61211685828601611e70565b925050602083013567ffffffffffffffff81111561213757612136611e26565b5b612143858286016120c6565b9150509250929050565b61215681611eee565b8114612160575f5ffd5b50565b5f813590506121718161214d565b92915050565b5f6020828403121561218c5761218b611e22565b5b5f61219984828501612163565b91505092915050565b6121ab81611e49565b82525050565b5f6020820190506121c45f8301846121a2565b92915050565b6121d381611eaf565b82525050565b604082015f8201516121ed5f8501826121ca565b50602082015161220060208501826121ca565b50505050565b5f6040820190506122195f8301846121d9565b92915050565b61222881611eaf565b8114612232575f5ffd5b50565b5f813590506122438161221f565b92915050565b5f5f6040838503121561225f5761225e611e22565b5b5f61226c85828601611f41565b925050602061227d85828601612235565b9150509250929050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000082169050919050565b6122bb81612287565b82525050565b6122ca81611f80565b82525050565b606082015f8201516122e45f8501826122b2565b5060208201516122f760208501826122c1565b50604082015161230a60408501826122c1565b50505050565b5f6060820190506123235f8301846122d0565b92915050565b61233281611f80565b811461233c575f5ffd5b50565b5f8135905061234d81612329565b92915050565b5f5f5f6060848603121561236a57612369611e22565b5b5f61237786828701611f41565b93505060206123888682870161233f565b925050604061239986828701612235565b9150509250925092565b6123ac81612287565b82525050565b5f6020820190506123c55f8301846123a3565b92915050565b5f6060820190506123de5f8301866123a3565b6123eb6020830185611f8f565b6123f86040830184611f8f565b949350505050565b5f6060820190506124135f8301856121d9565b6124206040830184611ef7565b9392505050565b5f5ffd5b5f610100828403121561244157612440612427565b5b81905092915050565b5f6040828403121561245f5761245e612427565b5b81905092915050565b5f5f5f61016084860312156124805761247f611e22565b5b5f61248d86828701611e70565b935050602061249e8682870161242b565b9250506101206124b08682870161244a565b9150509250925092565b5f5ffd5b5f5ffd5b5f5f83601f8401126124d7576124d6611fb7565b5b8235905067ffffffffffffffff8111156124f4576124f36124ba565b5b6020830191508360018202830111156125105761250f6124be565b5b9250929050565b5f5f5f6040848603121561252e5761252d611e22565b5b5f84013567ffffffffffffffff81111561254b5761254a611e26565b5b612557868287016124c2565b9350935050602061256a86828701612235565b9150509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6125a883836122c1565b60208301905092915050565b5f602082019050919050565b5f6125ca82612574565b6125d4818561257e565b93506125df8361258e565b805f5b8381101561260f5781516125f6888261259d565b9750612601836125b4565b9250506001810190506125e2565b5085935050505092915050565b5f6020820190508181035f83015261263481846125c0565b905092915050565b5f5ffd5b5f604082840312156126555761265461263c565b5b61265f604061202d565b90505f61266e84828501612235565b5f83015250602061268184828501612235565b60208301525092915050565b5f5f606083850312156126a3576126a2611e22565b5b5f6126b085828601611e70565b92505060206126c185828601612640565b9150509250929050565b5f82825260208201905092915050565b7f424c5341706b52656769737472792e696e697469616c697a6551756f72756d3a5f8201527f2071756f72756d20616c72656164792065786973747300000000000000000000602082015250565b5f6127356036836126cb565b9150612740826126db565b604082019050919050565b5f6020820190508181035f83015261276281612729565b9050919050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f61279b82612769565b6127a58185612773565b93506127b5818560208601612783565b6127be81611fbf565b840191505092915050565b5f6060820190506127dc5f8301866121a2565b6127e96020830185611ef7565b81810360408301526127fb8184612791565b9050949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f424c5341706b52656769737472792e67657441706b486173684174426c6f636b5f8201527f4e756d626572416e64496e6465783a20696e64657820746f6f20726563656e74602082015250565b5f61288c6040836126cb565b915061289782612832565b604082019050919050565b5f6020820190508181035f8301526128b981612880565b9050919050565b7f424c5341706b52656769737472792e67657441706b486173684174426c6f636b5f8201527f4e756d626572416e64496e6465783a206e6f74206c61746573742061706b207560208201527f7064617465000000000000000000000000000000000000000000000000000000604082015250565b5f6129406045836126cb565b915061294b826128c0565b606082019050919050565b5f6020820190508181035f83015261296d81612934565b9050919050565b7f424c5341706b52656769737472792e676574526567697374657265645075626b5f8201527f65793a206f70657261746f72206973206e6f7420726567697374657265640000602082015250565b5f6129ce603e836126cb565b91506129d982612974565b604082019050919050565b5f6020820190508181035f8301526129fb816129c2565b9050919050565b5f60408284031215612a1757612a16611e22565b5b5f612a2484828501612640565b91505092915050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a2063616e6e6f74207265676973746572207a65726f207075626b6579602082015250565b5f612a876040836126cb565b9150612a9282612a2d565b604082019050919050565b5f6020820190508181035f830152612ab481612a7b565b9050919050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a206f70657261746f7220616c7265616479207265676973746572656460208201527f207075626b657900000000000000000000000000000000000000000000000000604082015250565b5f612b3b6047836126cb565b9150612b4682612abb565b606082019050919050565b5f6020820190508181035f830152612b6881612b2f565b9050919050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a207075626c6963206b657920616c726561647920726567697374657260208201527f6564000000000000000000000000000000000000000000000000000000000000604082015250565b5f612bef6042836126cb565b9150612bfa82612b6f565b606082019050919050565b5f6020820190508181035f830152612c1c81612be3565b9050919050565b5f819050919050565b612c3d612c3882611eaf565b612c23565b82525050565b82818337505050565b612c5860408383612c43565b5050565b5f612c67828b612c2c565b602082019150612c77828a612c2c565b602082019150612c878289612c2c565b602082019150612c978288612c2c565b602082019150612ca78287612c4c565b604082019150612cb78286612c4c565b604082019150612cc78285612c2c565b602082019150612cd78284612c2c565b6020820191508190509998505050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f612d2482611eaf565b9150612d2f83611eaf565b925082612d3f57612d3e612ced565b5b828206905092915050565b5f67ffffffffffffffff821115612d6457612d63611fcf565b5b602082029050919050565b5f612d81612d7c84612d4a565b61202d565b90508060208402830185811115612d9b57612d9a6124be565b5b835b81811015612dc45780612db08882612235565b845260208401935050602081019050612d9d565b5050509392505050565b5f82601f830112612de257612de1611fb7565b5b6002612def848285612d6f565b91505092915050565b5f60808284031215612e0d57612e0c61263c565b5b612e17604061202d565b90505f612e2684828501612dce565b5f830152506040612e3984828501612dce565b60208301525092915050565b5f60808284031215612e5a57612e59611e22565b5b5f612e6784828501612df8565b91505092915050565b7f424c5341706b52656769737472792e7265676973746572424c535075626c69635f8201527f4b65793a2065697468657220746865204731207369676e61747572652069732060208201527f77726f6e672c206f7220473120616e642047322070726976617465206b65792060408201527f646f206e6f74206d617463680000000000000000000000000000000000000000606082015250565b5f612f16606c836126cb565b9150612f2182612e70565b608082019050919050565b5f6020820190508181035f830152612f4381612f0a565b9050919050565b5f8135612f568161221f565b80915050919050565b5f815f1b9050919050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff612f9584612f5f565b9350801983169250808416831791505092915050565b5f819050919050565b5f612fce612fc9612fc484611eaf565b612fab565b611eaf565b9050919050565b5f819050919050565b612fe782612fb4565b612ffa612ff382612fd5565b8354612f6a565b8255505050565b5f81015f83018061301181612f4a565b905061301d8184612fde565b50505060018101602083018061303281612f4a565b905061303e8184612fde565b5050505050565b61304f8282613001565b5050565b5f6130616020840184612235565b905092915050565b604082016130795f830183613053565b6130855f8501826121ca565b506130936020830183613053565b6130a060208501826121ca565b50505050565b5f82905092915050565b6130bc60408383612c43565b5050565b608082016130d05f8301836130a6565b6130dc5f8501826130b0565b506130ea60408301836130a6565b6130f760408501826130b0565b50505050565b5f60c0820190506131105f830185613069565b61311d60408301846130c0565b9392505050565b7f424c5341706b52656769737472792e67657441706b496e64696365734174426c5f8201527f6f636b4e756d6265723a20626c6f636b4e756d626572206973206265666f726560208201527f2074686520666972737420757064617465000000000000000000000000000000604082015250565b5f6131a46051836126cb565b91506131af82613124565b606082019050919050565b5f6020820190508181035f8301526131d181613198565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61320f82611eaf565b915061321a83611eaf565b9250828203905081811115613232576132316131d8565b5b92915050565b5f61324282611eaf565b91505f8203613254576132536131d8565b5b600182039050919050565b7f424c5341706b52656769737472792e5f636865636b5265676973747279436f6f5f8201527f7264696e61746f723a2063616c6c6572206973206e6f7420746865207265676960208201527f7374727920636f6f7264696e61746f7200000000000000000000000000000000604082015250565b5f6132df6050836126cb565b91506132ea8261325f565b606082019050919050565b5f6020820190508181035f83015261330c816132d3565b9050919050565b7f424c5341706b52656769737472792e5f70726f6365737351756f72756d41706b5f8201527f5570646174653a2071756f72756d20646f6573206e6f74206578697374000000602082015250565b5f61336d603d836126cb565b915061337882613313565b604082019050919050565b5f6020820190508181035f83015261339a81613361565b9050919050565b7f65632d6d756c2d6661696c6564000000000000000000000000000000000000005f82015250565b5f6133d5600d836126cb565b91506133e0826133a1565b602082019050919050565b5f6020820190508181035f830152613402816133c9565b9050919050565b7f65632d6164642d6661696c6564000000000000000000000000000000000000005f82015250565b5f61343d600d836126cb565b915061344882613409565b602082019050919050565b5f6020820190508181035f83015261346a81613431565b9050919050565b5f61347b82611eaf565b915061348683611eaf565b925082820261349481611eaf565b915082820484148315176134ab576134aa6131d8565b5b5092915050565b5f6134bc82611eaf565b91506134c783611eaf565b92508282019050808211156134df576134de6131d8565b5b92915050565b7f70616972696e672d6f70636f64652d6661696c656400000000000000000000005f82015250565b5f6135196015836126cb565b9150613524826134e5565b602082019050919050565b5f6020820190508181035f8301526135468161350d565b905091905056fea264697066735822122028f026356b26e283c1cd4e7b77123df74834f754e8ac1588d20e68445660228d64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x1EW_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\xABW\x80c\xD5%J\x8C\x11a\0oW\x80c\xD5%J\x8C\x14a\x03\x8DW\x80c\xDE)\xFA\xC0\x14a\x03\xBDW\x80c\xDFM\t\xE0\x14a\x03\xEDW\x80c\xE8\xBB\x9A\xE6\x14a\x04\tW\x80c\xF4\xE2O\xE5\x14a\x049Wa\x01\x1EV[\x80cm\x14\xA9\x87\x14a\x02\xABW\x80cy\x16\xCE\xA6\x14a\x02\xC9W\x80c\x7F\xF8\x1A\x87\x14a\x02\xFBW\x80c\xA3\xDB\x80\xE2\x14a\x03,W\x80c\xBFy\xCEX\x14a\x03]Wa\x01\x1EV[\x80c?\xB2yR\x11a\0\xF2W\x80c?\xB2yR\x14a\x01\xCFW\x80cG\xB3\x14\xE8\x14a\x01\xEBW\x80c_a\xA8\x84\x14a\x02\x1BW\x80c`WG\xD5\x14a\x02KW\x80ch\xBC\xCA\xAC\x14a\x02{Wa\x01\x1EV[\x80b\xA1\xF4\xCB\x14a\x01\"W\x80c\x13T*N\x14a\x01SW\x80c&\xD9A\xF2\x14a\x01\x83W\x80c7~\xD9\x9D\x14a\x01\x9FW[__\xFD[a\x01<`\x04\x806\x03\x81\x01\x90a\x017\x91\x90a\x1E\x84V[a\x04UV[`@Qa\x01J\x92\x91\x90a\x1E\xC7V[`@Q\x80\x91\x03\x90\xF3[a\x01m`\x04\x806\x03\x81\x01\x90a\x01h\x91\x90a\x1E\x84V[a\x04uV[`@Qa\x01z\x91\x90a\x1F\x06V[`@Q\x80\x91\x03\x90\xF3[a\x01\x9D`\x04\x806\x03\x81\x01\x90a\x01\x98\x91\x90a\x1FUV[a\x04\xBBV[\0[a\x01\xB9`\x04\x806\x03\x81\x01\x90a\x01\xB4\x91\x90a\x1FUV[a\x06\x13V[`@Qa\x01\xC6\x91\x90a\x1F\x9EV[`@Q\x80\x91\x03\x90\xF3[a\x01\xE9`\x04\x806\x03\x81\x01\x90a\x01\xE4\x91\x90a \xF3V[a\x066V[\0[a\x02\x05`\x04\x806\x03\x81\x01\x90a\x02\0\x91\x90a!wV[a\x06\x9DV[`@Qa\x02\x12\x91\x90a!\xB1V[`@Q\x80\x91\x03\x90\xF3[a\x025`\x04\x806\x03\x81\x01\x90a\x020\x91\x90a\x1FUV[a\x06\xD6V[`@Qa\x02B\x91\x90a\"\x06V[`@Q\x80\x91\x03\x90\xF3[a\x02e`\x04\x806\x03\x81\x01\x90a\x02`\x91\x90a\"IV[a\x07\x1AV[`@Qa\x02r\x91\x90a#\x10V[`@Q\x80\x91\x03\x90\xF3[a\x02\x95`\x04\x806\x03\x81\x01\x90a\x02\x90\x91\x90a#SV[a\x07\xDBV[`@Qa\x02\xA2\x91\x90a#\xB2V[`@Q\x80\x91\x03\x90\xF3[a\x02\xB3a\tWV[`@Qa\x02\xC0\x91\x90a!\xB1V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE3`\x04\x806\x03\x81\x01\x90a\x02\xDE\x91\x90a\"IV[a\t{V[`@Qa\x02\xF2\x93\x92\x91\x90a#\xCBV[`@Q\x80\x91\x03\x90\xF3[a\x03\x15`\x04\x806\x03\x81\x01\x90a\x03\x10\x91\x90a\x1E\x84V[a\t\xE1V[`@Qa\x03#\x92\x91\x90a$\0V[`@Q\x80\x91\x03\x90\xF3[a\x03F`\x04\x806\x03\x81\x01\x90a\x03A\x91\x90a\x1FUV[a\n\xDAV[`@Qa\x03T\x92\x91\x90a\x1E\xC7V[`@Q\x80\x91\x03\x90\xF3[a\x03w`\x04\x806\x03\x81\x01\x90a\x03r\x91\x90a$hV[a\n\xFAV[`@Qa\x03\x84\x91\x90a\x1F\x06V[`@Q\x80\x91\x03\x90\xF3[a\x03\xA7`\x04\x806\x03\x81\x01\x90a\x03\xA2\x91\x90a%\x17V[a\x0F^V[`@Qa\x03\xB4\x91\x90a&\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x03\xD7`\x04\x806\x03\x81\x01\x90a\x03\xD2\x91\x90a\x1E\x84V[a\x11mV[`@Qa\x03\xE4\x91\x90a\x1F\x06V[`@Q\x80\x91\x03\x90\xF3[a\x04\x07`\x04\x806\x03\x81\x01\x90a\x04\x02\x91\x90a&\x8DV[a\x11\x82V[\0[a\x04#`\x04\x806\x03\x81\x01\x90a\x04\x1E\x91\x90a!wV[a\x12wV[`@Qa\x040\x91\x90a!\xB1V[`@Q\x80\x91\x03\x90\xF3[a\x04S`\x04\x806\x03\x81\x01\x90a\x04N\x91\x90a \xF3V[a\x12\xA7V[\0[`\x03` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01T\x90P\x82V[_`\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[a\x04\xC3a\x13\x16V[_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14a\x05\x1FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\x16\x90a'KV[`@Q\x80\x91\x03\x90\xFD[`\x04_\x82`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80_`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[a\x06>a\x13\x16V[_a\x06H\x83a\t\xE1V[P\x90Pa\x06U\x82\x82a\x13\xA6V[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06\x80\x85a\x04uV[\x84`@Qa\x06\x90\x93\x92\x91\x90a'\xC9V[`@Q\x80\x91\x03\x90\xA1PPPV[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[a\x06\xDEa\x1C\xFBV[`\x05_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[a\x07\"a\x1D\x13V[`\x04_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x07KWa\x07Ja(\x05V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[__`\x04_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x08\x06Wa\x08\x05a(\x05V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_\x82\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10\x15a\x08\xE3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xDA\x90a(\xA2V[`@Q\x80\x91\x03\x90\xFD[_\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x14\x80a\t\nWP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\tIW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t@\x90a)VV[`@Q\x80\x91\x03\x90\xFD[\x80_\x01Q\x91PP\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\t\x94W_\x80\xFD[\x90_R` _ \x01_\x91P\x91PP\x80_\x01_\x90T\x90a\x01\0\n\x90\x04`@\x1B\x90\x80_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x83V[a\t\xE9a\x1C\xFBV[__`\x03_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P_`\x01_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P__\x1B\x81\x03a\n\xCDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xC4\x90a)\xE4V[`@Q\x80\x91\x03\x90\xFD[\x81\x81\x93P\x93PPP\x91P\x91V[`\x05` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01T\x90P\x82V[_a\x0B\x03a\x13\x16V[_a\x0B \x84`@\x01\x806\x03\x81\x01\x90a\x0B\x1B\x91\x90a*\x02V[a\x16\x87V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x03a\x0B\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B{\x90a*\x9DV[`@Q\x80\x91\x03\x90\xFD[__\x1B`\x01_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x14a\x0C\x05W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xFC\x90a+QV[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\xA3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x9A\x90a,\x05V[`@Q\x80\x91\x03\x90\xFD[_\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x85_\x01_\x015\x86_\x01` \x015\x87`@\x01_\x015\x88`@\x01` \x015\x89`\x80\x01_\x01\x8A`\x80\x01`@\x01\x8A_\x015\x8B` \x015`@Q` \x01a\r\x0E\x98\x97\x96\x95\x94\x93\x92\x91\x90a,\\V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\r0\x91\x90a-\x1AV[\x90Pa\r\xDCa\r\x7Fa\r^\x83\x88`@\x01\x806\x03\x81\x01\x90a\rP\x91\x90a*\x02V[a\x16\x9F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87_\x01\x806\x03\x81\x01\x90a\rq\x91\x90a*\x02V[a\x17s\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r\x87a\x18lV[a\r\xC3a\r\xA4\x85a\r\x96a\x196V[a\x16\x9F\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88\x806\x03\x81\x01\x90a\r\xB5\x91\x90a*\x02V[a\x17s\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\x80\x01\x806\x03\x81\x01\x90a\r\xD7\x91\x90a.EV[a\x19ZV[a\x0E\x1BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\x12\x90a/,V[`@Q\x80\x91\x03\x90\xFD[\x84`@\x01`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a\x0Eh\x91\x90a0EV[\x90PP\x81`\x01_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x85`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x86`@\x01\x87`\x80\x01`@Qa\x0FJ\x92\x91\x90a0\xFDV[`@Q\x80\x91\x03\x90\xA2\x81\x92PPP\x93\x92PPPV[``_\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F~Wa\x0F}a\x1F\xCFV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\xACW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x85\x85\x90P\x81\x10\x15a\x11aW_\x86\x86\x83\x81\x81\x10a\x0F\xD3Wa\x0F\xD2a(\x05V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x14\x80a\x10VWP`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x81T\x81\x10a\x102Wa\x101a(\x05V[[\x90_R` _ \x01_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x10\x96W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x8D\x90a1\xBAV[`@Q\x80\x91\x03\x90\xFD[_\x81\x90P[_\x81\x11\x15a\x11QW\x86`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a\x10\xC8\x91\x90a2\x05V[\x81T\x81\x10a\x10\xD9Wa\x10\xD8a(\x05V[[\x90_R` _ \x01_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a\x11>W`\x01\x81a\x11\x0C\x91\x90a2\x05V[\x85\x85\x81Q\x81\x10a\x11\x1FWa\x11\x1Ea(\x05V[[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x11QV[\x80\x80a\x11I\x90a28V[\x91PPa\x10\x9BV[PPP\x80\x80`\x01\x01\x91PPa\x0F\xB4V[P\x80\x91PP\x93\x92PPPV[`\x01` R\x80_R`@_ _\x91P\x90PT\x81V[_a\x11\x8C\x82a\x16\x87V[\x90P\x80`\x01_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\x03_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PPPPPV[`\x02` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x12\xAFa\x13\x16V[_a\x12\xB9\x83a\t\xE1V[P\x90Pa\x12\xCE\x82a\x12\xC9\x83a\x1CCV[a\x13\xA6V[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x12\xF9\x85a\x04uV[\x84`@Qa\x13\t\x93\x92\x91\x90a'\xC9V[`@Q\x80\x91\x03\x90\xA1PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x13\xA4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\x9B\x90a2\xF5V[`@Q\x80\x91\x03\x90\xFD[V[a\x13\xAEa\x1C\xFBV[__\x90P[\x83Q\x81\x10\x15a\x16\x81W_\x84\x82\x81Q\x81\x10a\x13\xD0Wa\x13\xCFa(\x05V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x04_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a\x14@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x147\x90a3\x83V[`@Q\x80\x91\x03\x90\xFD[a\x14\x87\x85`\x05_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x17s\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93P\x83`\x05_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP_a\x14\xC0\x85a\x16\x87V[\x90P_`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x84a\x14\xE7\x91\x90a2\x05V[\x81T\x81\x10a\x14\xF8Wa\x14\xF7a(\x05V[[\x90_R` _ \x01\x90PCc\xFF\xFF\xFF\xFF\x16\x81_\x01`\x18\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a\x15_W\x81\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UPa\x16pV[C\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x04_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`@\x1C\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[PPPP\x80\x80`\x01\x01\x91PPa\x13\xB3V[PPPPV[_\x81Q_R\x81` \x01Q` R`@_ \x90P\x91\x90PV[a\x16\xA7a\x1C\xFBV[a\x16\xAFa\x1DHV[\x83_\x01Q\x81_`\x03\x81\x10a\x16\xC6Wa\x16\xC5a(\x05V[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x03\x81\x10a\x16\xE8Wa\x16\xE7a(\x05V[[` \x02\x01\x81\x81RPP\x82\x81`\x02`\x03\x81\x10a\x17\x06Wa\x17\x05a(\x05V[[` \x02\x01\x81\x81RPP_`@\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x17*W\xFE[P\x80a\x17kW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17b\x90a3\xEBV[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x17{a\x1C\xFBV[a\x17\x83a\x1DjV[\x83_\x01Q\x81_`\x04\x81\x10a\x17\x9AWa\x17\x99a(\x05V[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x04\x81\x10a\x17\xBCWa\x17\xBBa(\x05V[[` \x02\x01\x81\x81RPP\x82_\x01Q\x81`\x02`\x04\x81\x10a\x17\xDDWa\x17\xDCa(\x05V[[` \x02\x01\x81\x81RPP\x82` \x01Q\x81`\x03`\x04\x81\x10a\x17\xFFWa\x17\xFEa(\x05V[[` \x02\x01\x81\x81RPP_`@\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x18#W\xFE[P\x80a\x18dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18[\x90a4SV[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x18ta\x1D\x8CV[`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x81R` \x01\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D\x81RP\x81RP\x90P\x90V[a\x19>a\x1C\xFBV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x02\x81RP\x90P\x90V[__`@Q\x80`@\x01`@R\x80\x87\x81R` \x01\x85\x81RP\x90P_`@Q\x80`@\x01`@R\x80\x87\x81R` \x01\x85\x81RP\x90Pa\x19\x93a\x1D\xB2V[__\x90P[`\x02\x81\x10\x15a\x1B\xB1W_`\x06\x82a\x19\xAF\x91\x90a4qV[\x90P\x84\x82`\x02\x81\x10a\x19\xC4Wa\x19\xC3a(\x05V[[` \x02\x01Q_\x01Q\x83_\x83a\x19\xD9\x91\x90a4\xB2V[`\x0C\x81\x10a\x19\xEAWa\x19\xE9a(\x05V[[` \x02\x01\x81\x81RPP\x84\x82`\x02\x81\x10a\x1A\x06Wa\x1A\x05a(\x05V[[` \x02\x01Q` \x01Q\x83`\x01\x83a\x1A\x1D\x91\x90a4\xB2V[`\x0C\x81\x10a\x1A.Wa\x1A-a(\x05V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x1AJWa\x1AIa(\x05V[[` \x02\x01Q_\x01Q_`\x02\x81\x10a\x1AdWa\x1Aca(\x05V[[` \x02\x01Q\x83`\x02\x83a\x1Aw\x91\x90a4\xB2V[`\x0C\x81\x10a\x1A\x88Wa\x1A\x87a(\x05V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x1A\xA4Wa\x1A\xA3a(\x05V[[` \x02\x01Q_\x01Q`\x01`\x02\x81\x10a\x1A\xBFWa\x1A\xBEa(\x05V[[` \x02\x01Q\x83`\x03\x83a\x1A\xD2\x91\x90a4\xB2V[`\x0C\x81\x10a\x1A\xE3Wa\x1A\xE2a(\x05V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x1A\xFFWa\x1A\xFEa(\x05V[[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x1B\x1AWa\x1B\x19a(\x05V[[` \x02\x01Q\x83`\x04\x83a\x1B-\x91\x90a4\xB2V[`\x0C\x81\x10a\x1B>Wa\x1B=a(\x05V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x1BZWa\x1BYa(\x05V[[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x1BvWa\x1Bua(\x05V[[` \x02\x01Q\x83`\x05\x83a\x1B\x89\x91\x90a4\xB2V[`\x0C\x81\x10a\x1B\x9AWa\x1B\x99a(\x05V[[` \x02\x01\x81\x81RPPP\x80\x80`\x01\x01\x91PPa\x19\x98V[Pa\x1B\xBAa\x1D\xD5V[_` \x82` `\x0C\x02\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x1B\xD8W\xFE[P\x80a\x1C\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\x10\x90a5/V[`@Q\x80\x91\x03\x90\xFD[_\x82_`\x01\x81\x10a\x1C-Wa\x1C,a(\x05V[[` \x02\x01Q\x14\x15\x95PPPPPP\x94\x93PPPPV[a\x1CKa\x1C\xFBV[_\x82_\x01Q\x14\x80\x15a\x1C`WP_\x82` \x01Q\x14[\x15a\x1C\x81W`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x1C\xF6V[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x1C\xC5\x91\x90a-\x1AV[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1C\xF0\x91\x90a2\x05V[\x81RP\x90P[\x91\x90PV[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80a\x1D\x9Fa\x1D\xF7V[\x81R` \x01a\x1D\xACa\x1D\xF7V[\x81RP\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x1ES\x82a\x1E*V[\x90P\x91\x90PV[a\x1Ec\x81a\x1EIV[\x81\x14a\x1EmW__\xFD[PV[_\x815\x90Pa\x1E~\x81a\x1EZV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1E\x99Wa\x1E\x98a\x1E\"V[[_a\x1E\xA6\x84\x82\x85\x01a\x1EpV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x1E\xC1\x81a\x1E\xAFV[\x82RPPV[_`@\x82\x01\x90Pa\x1E\xDA_\x83\x01\x85a\x1E\xB8V[a\x1E\xE7` \x83\x01\x84a\x1E\xB8V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[a\x1F\0\x81a\x1E\xEEV[\x82RPPV[_` \x82\x01\x90Pa\x1F\x19_\x83\x01\x84a\x1E\xF7V[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x1F4\x81a\x1F\x1FV[\x81\x14a\x1F>W__\xFD[PV[_\x815\x90Pa\x1FO\x81a\x1F+V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1FjWa\x1Fia\x1E\"V[[_a\x1Fw\x84\x82\x85\x01a\x1FAV[\x91PP\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x1F\x98\x81a\x1F\x80V[\x82RPPV[_` \x82\x01\x90Pa\x1F\xB1_\x83\x01\x84a\x1F\x8FV[\x92\x91PPV[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a \x05\x82a\x1F\xBFV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a $Wa #a\x1F\xCFV[[\x80`@RPPPV[_a 6a\x1E\x19V[\x90Pa B\x82\x82a\x1F\xFCV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a aWa `a\x1F\xCFV[[a j\x82a\x1F\xBFV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a \x97a \x92\x84a GV[a -V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a \xB3Wa \xB2a\x1F\xBBV[[a \xBE\x84\x82\x85a wV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a \xDAWa \xD9a\x1F\xB7V[[\x815a \xEA\x84\x82` \x86\x01a \x85V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a!\tWa!\x08a\x1E\"V[[_a!\x16\x85\x82\x86\x01a\x1EpV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!7Wa!6a\x1E&V[[a!C\x85\x82\x86\x01a \xC6V[\x91PP\x92P\x92\x90PV[a!V\x81a\x1E\xEEV[\x81\x14a!`W__\xFD[PV[_\x815\x90Pa!q\x81a!MV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a!\x8CWa!\x8Ba\x1E\"V[[_a!\x99\x84\x82\x85\x01a!cV[\x91PP\x92\x91PPV[a!\xAB\x81a\x1EIV[\x82RPPV[_` \x82\x01\x90Pa!\xC4_\x83\x01\x84a!\xA2V[\x92\x91PPV[a!\xD3\x81a\x1E\xAFV[\x82RPPV[`@\x82\x01_\x82\x01Qa!\xED_\x85\x01\x82a!\xCAV[P` \x82\x01Qa\"\0` \x85\x01\x82a!\xCAV[PPPPV[_`@\x82\x01\x90Pa\"\x19_\x83\x01\x84a!\xD9V[\x92\x91PPV[a\"(\x81a\x1E\xAFV[\x81\x14a\"2W__\xFD[PV[_\x815\x90Pa\"C\x81a\"\x1FV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\"_Wa\"^a\x1E\"V[[_a\"l\x85\x82\x86\x01a\x1FAV[\x92PP` a\"}\x85\x82\x86\x01a\"5V[\x91PP\x92P\x92\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\"\xBB\x81a\"\x87V[\x82RPPV[a\"\xCA\x81a\x1F\x80V[\x82RPPV[``\x82\x01_\x82\x01Qa\"\xE4_\x85\x01\x82a\"\xB2V[P` \x82\x01Qa\"\xF7` \x85\x01\x82a\"\xC1V[P`@\x82\x01Qa#\n`@\x85\x01\x82a\"\xC1V[PPPPV[_``\x82\x01\x90Pa##_\x83\x01\x84a\"\xD0V[\x92\x91PPV[a#2\x81a\x1F\x80V[\x81\x14a#<W__\xFD[PV[_\x815\x90Pa#M\x81a#)V[\x92\x91PPV[___``\x84\x86\x03\x12\x15a#jWa#ia\x1E\"V[[_a#w\x86\x82\x87\x01a\x1FAV[\x93PP` a#\x88\x86\x82\x87\x01a#?V[\x92PP`@a#\x99\x86\x82\x87\x01a\"5V[\x91PP\x92P\x92P\x92V[a#\xAC\x81a\"\x87V[\x82RPPV[_` \x82\x01\x90Pa#\xC5_\x83\x01\x84a#\xA3V[\x92\x91PPV[_``\x82\x01\x90Pa#\xDE_\x83\x01\x86a#\xA3V[a#\xEB` \x83\x01\x85a\x1F\x8FV[a#\xF8`@\x83\x01\x84a\x1F\x8FV[\x94\x93PPPPV[_``\x82\x01\x90Pa$\x13_\x83\x01\x85a!\xD9V[a$ `@\x83\x01\x84a\x1E\xF7V[\x93\x92PPPV[__\xFD[_a\x01\0\x82\x84\x03\x12\x15a$AWa$@a$'V[[\x81\x90P\x92\x91PPV[_`@\x82\x84\x03\x12\x15a$_Wa$^a$'V[[\x81\x90P\x92\x91PPV[___a\x01`\x84\x86\x03\x12\x15a$\x80Wa$\x7Fa\x1E\"V[[_a$\x8D\x86\x82\x87\x01a\x1EpV[\x93PP` a$\x9E\x86\x82\x87\x01a$+V[\x92PPa\x01 a$\xB0\x86\x82\x87\x01a$JV[\x91PP\x92P\x92P\x92V[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a$\xD7Wa$\xD6a\x1F\xB7V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xF4Wa$\xF3a$\xBAV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a%\x10Wa%\x0Fa$\xBEV[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a%.Wa%-a\x1E\"V[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%KWa%Ja\x1E&V[[a%W\x86\x82\x87\x01a$\xC2V[\x93P\x93PP` a%j\x86\x82\x87\x01a\"5V[\x91PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a%\xA8\x83\x83a\"\xC1V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a%\xCA\x82a%tV[a%\xD4\x81\x85a%~V[\x93Pa%\xDF\x83a%\x8EV[\x80_[\x83\x81\x10\x15a&\x0FW\x81Qa%\xF6\x88\x82a%\x9DV[\x97Pa&\x01\x83a%\xB4V[\x92PP`\x01\x81\x01\x90Pa%\xE2V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra&4\x81\x84a%\xC0V[\x90P\x92\x91PPV[__\xFD[_`@\x82\x84\x03\x12\x15a&UWa&Ta&<V[[a&_`@a -V[\x90P_a&n\x84\x82\x85\x01a\"5V[_\x83\x01RP` a&\x81\x84\x82\x85\x01a\"5V[` \x83\x01RP\x92\x91PPV[__``\x83\x85\x03\x12\x15a&\xA3Wa&\xA2a\x1E\"V[[_a&\xB0\x85\x82\x86\x01a\x1EpV[\x92PP` a&\xC1\x85\x82\x86\x01a&@V[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBLSApkRegistry.initializeQuorum:_\x82\x01R\x7F quorum already exists\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a'5`6\x83a&\xCBV[\x91Pa'@\x82a&\xDBV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra'b\x81a')V[\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a'\x9B\x82a'iV[a'\xA5\x81\x85a'sV[\x93Pa'\xB5\x81\x85` \x86\x01a'\x83V[a'\xBE\x81a\x1F\xBFV[\x84\x01\x91PP\x92\x91PPV[_``\x82\x01\x90Pa'\xDC_\x83\x01\x86a!\xA2V[a'\xE9` \x83\x01\x85a\x1E\xF7V[\x81\x81\x03`@\x83\x01Ra'\xFB\x81\x84a'\x91V[\x90P\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FBLSApkRegistry.getApkHashAtBlock_\x82\x01R\x7FNumberAndIndex: index too recent` \x82\x01RPV[_a(\x8C`@\x83a&\xCBV[\x91Pa(\x97\x82a(2V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra(\xB9\x81a(\x80V[\x90P\x91\x90PV[\x7FBLSApkRegistry.getApkHashAtBlock_\x82\x01R\x7FNumberAndIndex: not latest apk u` \x82\x01R\x7Fpdate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a)@`E\x83a&\xCBV[\x91Pa)K\x82a(\xC0V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra)m\x81a)4V[\x90P\x91\x90PV[\x7FBLSApkRegistry.getRegisteredPubk_\x82\x01R\x7Fey: operator is not registered\0\0` \x82\x01RPV[_a)\xCE`>\x83a&\xCBV[\x91Pa)\xD9\x82a)tV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra)\xFB\x81a)\xC2V[\x90P\x91\x90PV[_`@\x82\x84\x03\x12\x15a*\x17Wa*\x16a\x1E\"V[[_a*$\x84\x82\x85\x01a&@V[\x91PP\x92\x91PPV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: cannot register zero pubkey` \x82\x01RPV[_a*\x87`@\x83a&\xCBV[\x91Pa*\x92\x82a*-V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra*\xB4\x81a*{V[\x90P\x91\x90PV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: operator already registered` \x82\x01R\x7F pubkey\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a+;`G\x83a&\xCBV[\x91Pa+F\x82a*\xBBV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra+h\x81a+/V[\x90P\x91\x90PV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: public key already register` \x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a+\xEF`B\x83a&\xCBV[\x91Pa+\xFA\x82a+oV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra,\x1C\x81a+\xE3V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a,=a,8\x82a\x1E\xAFV[a,#V[\x82RPPV[\x82\x81\x837PPPV[a,X`@\x83\x83a,CV[PPV[_a,g\x82\x8Ba,,V[` \x82\x01\x91Pa,w\x82\x8Aa,,V[` \x82\x01\x91Pa,\x87\x82\x89a,,V[` \x82\x01\x91Pa,\x97\x82\x88a,,V[` \x82\x01\x91Pa,\xA7\x82\x87a,LV[`@\x82\x01\x91Pa,\xB7\x82\x86a,LV[`@\x82\x01\x91Pa,\xC7\x82\x85a,,V[` \x82\x01\x91Pa,\xD7\x82\x84a,,V[` \x82\x01\x91P\x81\x90P\x99\x98PPPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a-$\x82a\x1E\xAFV[\x91Pa-/\x83a\x1E\xAFV[\x92P\x82a-?Wa->a,\xEDV[[\x82\x82\x06\x90P\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a-dWa-ca\x1F\xCFV[[` \x82\x02\x90P\x91\x90PV[_a-\x81a-|\x84a-JV[a -V[\x90P\x80` \x84\x02\x83\x01\x85\x81\x11\x15a-\x9BWa-\x9Aa$\xBEV[[\x83[\x81\x81\x10\x15a-\xC4W\x80a-\xB0\x88\x82a\"5V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa-\x9DV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a-\xE2Wa-\xE1a\x1F\xB7V[[`\x02a-\xEF\x84\x82\x85a-oV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a.\rWa.\x0Ca&<V[[a.\x17`@a -V[\x90P_a.&\x84\x82\x85\x01a-\xCEV[_\x83\x01RP`@a.9\x84\x82\x85\x01a-\xCEV[` \x83\x01RP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a.ZWa.Ya\x1E\"V[[_a.g\x84\x82\x85\x01a-\xF8V[\x91PP\x92\x91PPV[\x7FBLSApkRegistry.registerBLSPublic_\x82\x01R\x7FKey: either the G1 signature is ` \x82\x01R\x7Fwrong, or G1 and G2 private key `@\x82\x01R\x7Fdo not match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_a/\x16`l\x83a&\xCBV[\x91Pa/!\x82a.pV[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra/C\x81a/\nV[\x90P\x91\x90PV[_\x815a/V\x81a\"\x1FV[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa/\x95\x84a/_V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a/\xCEa/\xC9a/\xC4\x84a\x1E\xAFV[a/\xABV[a\x1E\xAFV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a/\xE7\x82a/\xB4V[a/\xFAa/\xF3\x82a/\xD5V[\x83Ta/jV[\x82UPPPV[_\x81\x01_\x83\x01\x80a0\x11\x81a/JV[\x90Pa0\x1D\x81\x84a/\xDEV[PPP`\x01\x81\x01` \x83\x01\x80a02\x81a/JV[\x90Pa0>\x81\x84a/\xDEV[PPPPPV[a0O\x82\x82a0\x01V[PPV[_a0a` \x84\x01\x84a\"5V[\x90P\x92\x91PPV[`@\x82\x01a0y_\x83\x01\x83a0SV[a0\x85_\x85\x01\x82a!\xCAV[Pa0\x93` \x83\x01\x83a0SV[a0\xA0` \x85\x01\x82a!\xCAV[PPPPV[_\x82\x90P\x92\x91PPV[a0\xBC`@\x83\x83a,CV[PPV[`\x80\x82\x01a0\xD0_\x83\x01\x83a0\xA6V[a0\xDC_\x85\x01\x82a0\xB0V[Pa0\xEA`@\x83\x01\x83a0\xA6V[a0\xF7`@\x85\x01\x82a0\xB0V[PPPPV[_`\xC0\x82\x01\x90Pa1\x10_\x83\x01\x85a0iV[a1\x1D`@\x83\x01\x84a0\xC0V[\x93\x92PPPV[\x7FBLSApkRegistry.getApkIndicesAtBl_\x82\x01R\x7FockNumber: blockNumber is before` \x82\x01R\x7F the first update\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a1\xA4`Q\x83a&\xCBV[\x91Pa1\xAF\x82a1$V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra1\xD1\x81a1\x98V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a2\x0F\x82a\x1E\xAFV[\x91Pa2\x1A\x83a\x1E\xAFV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a22Wa21a1\xD8V[[\x92\x91PPV[_a2B\x82a\x1E\xAFV[\x91P_\x82\x03a2TWa2Sa1\xD8V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FBLSApkRegistry._checkRegistryCoo_\x82\x01R\x7Frdinator: caller is not the regi` \x82\x01R\x7Fstry coordinator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a2\xDF`P\x83a&\xCBV[\x91Pa2\xEA\x82a2_V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3\x0C\x81a2\xD3V[\x90P\x91\x90PV[\x7FBLSApkRegistry._processQuorumApk_\x82\x01R\x7FUpdate: quorum does not exist\0\0\0` \x82\x01RPV[_a3m`=\x83a&\xCBV[\x91Pa3x\x82a3\x13V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3\x9A\x81a3aV[\x90P\x91\x90PV[\x7Fec-mul-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a3\xD5`\r\x83a&\xCBV[\x91Pa3\xE0\x82a3\xA1V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra4\x02\x81a3\xC9V[\x90P\x91\x90PV[\x7Fec-add-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a4=`\r\x83a&\xCBV[\x91Pa4H\x82a4\tV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra4j\x81a41V[\x90P\x91\x90PV[_a4{\x82a\x1E\xAFV[\x91Pa4\x86\x83a\x1E\xAFV[\x92P\x82\x82\x02a4\x94\x81a\x1E\xAFV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a4\xABWa4\xAAa1\xD8V[[P\x92\x91PPV[_a4\xBC\x82a\x1E\xAFV[\x91Pa4\xC7\x83a\x1E\xAFV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a4\xDFWa4\xDEa1\xD8V[[\x92\x91PPV[\x7Fpairing-opcode-failed\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a5\x19`\x15\x83a&\xCBV[\x91Pa5$\x82a4\xE5V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra5F\x81a5\rV[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 (\xF0&5k&\xE2\x83\xC1\xCDN{w\x12=\xF7H4\xF7T\xE8\xAC\x15\x88\xD2\x0EhDV`\"\x8DdsolcC\0\x08\x1B\x003",
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                BN254::G1Point,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<setBLSPublicKeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBLSPublicKeyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBLSPublicKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBLSPublicKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, BN254::G1Point);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBLSPublicKeyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls>] = &[
                {
                    fn operatorToPubkey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSApkRegistryHarnessCalls> {
                        <operatorToPubkeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <getApkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <apkHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                        <currentApkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
                            )
                            .map(BLSApkRegistryHarnessCalls::deregisterOperator)
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
        BLSApkRegistryHarnessInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _registryCoordinator)
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
    pub struct BLSApkRegistryHarnessInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BLSApkRegistryHarnessInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BLSApkRegistryHarnessInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BLSApkRegistryHarnessInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BLSApkRegistryHarness`](self) contract instance.

See the [wrapper's documentation](`BLSApkRegistryHarnessInstance`) for more details.*/
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
    > BLSApkRegistryHarnessInstance<T, P, N> {
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
    > BLSApkRegistryHarnessInstance<T, P, N> {
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
