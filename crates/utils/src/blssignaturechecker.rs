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
library IBLSSignatureChecker {
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
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

interface BLSSignatureChecker {
    event StaleStakesForbiddenUpdate(bool value);

    constructor(address _registryCoordinator);

    function blsApkRegistry() external view returns (address);
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureChecker.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureChecker.QuorumStakeTotals memory, bytes32);
    function delegation() external view returns (address);
    function registryCoordinator() external view returns (address);
    function setStaleStakesForbidden(bool value) external;
    function stakeRegistry() external view returns (address);
    function staleStakesForbidden() external view returns (bool);
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
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
    clippy::style
)]
pub mod BLSSignatureChecker {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101008060405234610171576020816124ba8038038091610020828561020d565b83398101031261017157516001600160a01b0381169081810361017157608052604051636830483560e01b8152602081600481855afa90811561017d575f916101ca575b5060a052604051632efa2ca360e11b815290602090829060049082905afa90811561017d575f91610188575b5060c05260a05160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa90811561017d575f91610137575b5060e052604051612275908161024582396080518181816102c701528181610406015281816110e4015281816111d701526117bd015260a0518181816103c20152818161150a0152611668015260c05181818161037e0152611446015260e05181818161080b01526112f90152f35b90506020813d602011610175575b816101526020938361020d565b8101031261017157516001600160a01b0381168103610171575f6100c8565b5f80fd5b3d9150610145565b6040513d5f823e3d90fd5b90506020813d6020116101c2575b816101a36020938361020d565b8101031261017157516001600160a01b0381168103610171575f610090565b3d9150610196565b90506020813d602011610205575b816101e56020938361020d565b8101031261017157516001600160a01b0381168103610171576004610064565b3d91506101d8565b601f909101601f19168101906001600160401b0382119082101761023057604052565b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8063171f1d5b14610094578063416c7e5e1461008f5780635df459461461008a57806368304835146100855780636d14a987146100805780636efb46361461007b578063b98d0908146100765763df5cf72314610071575f80fd5b6107f6565b6107d5565b61073c565b6103f1565b6103ad565b610369565b610295565b346100e0576101203660031901126100e05760406100d06004356100b736610168565b6100c036610228565b906100ca36610190565b92610878565b8251911515825215156020820152f35b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761011357604052565b6100e4565b90601f801991011681019081106001600160401b0382111761011357604052565b6040519061014961010083610118565b565b60405190610149604083610118565b906101496040519283610118565b60409060231901126100e05760405190610181826100f8565b60243582526044356020830152565b60409060e31901126100e057604051906101a9826100f8565b60e4358252610104356020830152565b91908260409103126100e0576040516101d1816100f8565b6020808294803584520135910152565b9080601f830112156100e057604051916101fc604084610118565b8290604081019283116100e057905b8282106102185750505090565b813581526020918201910161020b565b9060806063198301126100e057604051610241816100f8565b602061025c82946102538160646101e1565b845260a46101e1565b910152565b91906080838203126100e057602061025c6040519261027f846100f8565b6040849661028d83826101e1565b8652016101e1565b346100e05760203660031901126100e05760043580151581036100e057604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610364575f9161031e575b61031c83610317336001600160a01b03861614610969565b611dbf565b005b90506020813d60201161035c575b8161033960209383610118565b810103126100e05751906001600160a01b03821682036100e057906103176102ff565b3d915061032c565b61095e565b346100e0575f3660031901126100e0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346100e0575f3660031901126100e0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346100e0575f3660031901126100e0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b63ffffffff8116036100e057565b6044359061014982610435565b6001600160401b0381116101135760051b60200190565b9080601f830112156100e057813561047e81610450565b9261048c6040519485610118565b81845260208085019260051b8201019283116100e057602001905b8282106104b45750505090565b6020809183356104c381610435565b8152019101906104a7565b81601f820112156100e05780356104e481610450565b926104f26040519485610118565b81845260208085019260061b840101928184116100e057602001915b83831061051c575050505090565b602060409161052b84866101b9565b81520192019161050e565b9080601f830112156100e057813561054d81610450565b9261055b6040519485610118565b81845260208085019260051b820101918383116100e05760208201905b83821061058757505050505090565b81356001600160401b0381116100e0576020916105a987848094880101610467565b815201910190610578565b919091610180818403126100e0576105ca610139565b9281356001600160401b0381116100e057816105e7918401610467565b845260208201356001600160401b0381116100e057816106089184016104ce565b602085015260408201356001600160401b0381116100e0578161062c9184016104ce565b604085015261063e8160608401610261565b60608501526106508160e084016101b9565b60808501526101208201356001600160401b0381116100e05781610675918401610467565b60a08501526101408201356001600160401b0381116100e0578161069a918401610467565b60c08501526101608201356001600160401b0381116100e0576106bd9201610536565b60e0830152565b90602080835192838152019201905f5b8181106106e15750505090565b82516001600160601b03168452602093840193909201916001016106d4565b929190610737602091604086528261072382516040808a015260808901906106c4565b910151868203603f190160608801526106c4565b930152565b346100e05760803660031901126100e0576004356024356001600160401b0381116100e057366023820112156100e05780600401356001600160401b0381116100e05736602482840101116100e057610793610443565b90606435936001600160401b0385116100e05760246107b96107c19636906004016105b4565b940190611007565b906107d160405192839283610700565b0390f35b346100e0575f3660031901126100e057602060ff5f54166040519015158152f35b346100e0575f3660031901126100e0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b634e487b7160e01b5f52603260045260245ffd5b90600281101561085f5760051b0190565b61083a565b634e487b7160e01b5f52601260045260245ffd5b61095461093161095a9561092b61092485875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e08401526101008301526108fb81610120840103601f198101835282610118565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b8096611966565b906119ac565b9261092b610946610940611a34565b94611b2b565b9161094f611c47565b611966565b91611c91565b9091565b6040513d5f823e3d90fd5b1561097057565b60405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a490fd5b60405190610a0e826100f8565b60606020838281520152565b15610a2157565b60405162461bcd60e51b815260206004820152603760248201525f5160206122205f395f51905f5260448201527f7265733a20656d7074792071756f72756d20696e7075740000000000000000006064820152608490fd5b15610a8057565b60405162461bcd60e51b815260206004820152604160248201525f5160206122205f395f51905f5260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a490fd5b15610ae957565b60a460405162461bcd60e51b815260206004820152604460248201525f5160206122205f395f51905f5260448201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b6084820152fd5b15610b5457565b60405162461bcd60e51b815260206004820152603c60248201525f5160206122205f395f51905f5260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608490fd5b90610bb682610450565b610bc36040519182610118565b8281528092610bd4601f1991610450565b0190602036910137565b908160209103126100e0575160ff811681036100e05790565b9291926001600160401b0382116101135760405191610c20601f8201601f191660200184610118565b8294818452818301116100e0578281602093845f960137010152565b805182101561085f5760209160051b010190565b634e487b7160e01b5f52601160045260245ffd5b5f19810191908211610c7257565b610c50565b15610c7e57565b608460405162461bcd60e51b815260206004820152604060248201525f5160206122205f395f51905f5260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152fd5b908160209103126100e057516001600160c01b03811681036100e05790565b908160209103126100e05751610d0981610435565b90565b9082101561085f570190565b908160209103126100e0575190565b9060018201809211610c7257565b9060028201809211610c7257565b9060038201809211610c7257565b9060048201809211610c7257565b9060058201809211610c7257565b91908201809211610c7257565b15610d8157565b60405162461bcd60e51b815260206004820152606660248201525f5160206122205f395f51905f5260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c490fd5b908160209103126100e0575167ffffffffffffffff19811681036100e05790565b15610e3657565b60405162461bcd60e51b815260206004820152606160248201525f5160206122205f395f51905f5260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c490fd5b908160209103126100e057516001600160601b03811681036100e05790565b906001600160601b03809116911603906001600160601b038211610c7257565b15610f0457565b60405162461bcd60e51b815260206004820152604360248201525f5160206122205f395f51905f5260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a490fd5b15610f6f57565b60405162461bcd60e51b815260206004820152603960248201525f5160206122205f395f51905f5260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608490fd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b818110610ff15750505090565b8251845260209384019390920191600101610fe4565b949392909193611015610a01565b50611021851515610a1a565b604084015151851480611914575b80611906575b806118f8575b61104490610a79565b61105660208501515185515114610ae2565b61106d63ffffffff431663ffffffff841610610b4d565b61107561014b565b5f81525f602082015292611087610a01565b61109087610bac565b602082015261109e87610bac565b81526110a8610a01565b926110b7602088015151610bac565b84526110c7602088015151610bac565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561036457611130915f916118c9575b5061112b368b87610bf7565b611dfb565b985f965b602089015180518910156112bc576020886111a16111978c61118f8f96868e611174611161868095610c3c565b5180515f526020015160205260405f2090565b6111818484840151610c3c565b5282611289575b0151610c3c565b519551610c3c565b5163ffffffff1690565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9182156103645761092b8a61124e8f6112478f8460208f9261123e936112368460019e6112549e5f9161125c575b508f8060c01b03169251610c3c565b520151610c3c565b51938d51610c3c565b5116611e82565b90611eb3565b970196611134565b61127c9150863d8111611282575b6112748183610118565b810190610cd5565b5f611227565b503d61126a565b6112b76112998484840151610c3c565b516112b0848401516112aa87610c64565b90610c3c565b5110610c77565b611188565b509095979496506112d1919893929950611f99565b916112dd5f5460ff1690565b9081156118c1576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610364575f91611892575b5091905b5f925b81841061139c5750505050509261137561137061136961139695856113889860806060602099015192015192610878565b9190610efd565b610f68565b0151604051928391602083019586610fc7565b03601f198101835282610118565b51902090565b92989596909399919794878b888c888d61177c575b6111978260a06114056113ff6113f18461140d976113eb6113dd6111618f9c604060209f9e0151610c3c565b67ffffffffffffffff191690565b9b610d0c565b356001600160f81b03191690565b60f81c90565b970151610c3c565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa908115610364576114d16111978f958f906114c98f978f96848f6114c360c0966114bc848f60209f906111886113f1996040936113ff9c5f9161174e575b5067ffffffffffffffff19918216911614610e2f565b51906119ac565b9c610d0c565b960151610c3c565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156103645761155e918c8f925f9261172a575b50602061155092930151610c3c565b906001600160601b03169052565b61158b8c6115508c611584611577826020860151610c3c565b516001600160601b031690565b9251610c3c565b5f985f5b60208a015151811015611711578b8d6115cd896115c06113ff6113f1868f896115b89151610c3c565b519487610d0c565b60ff161c60019081161490565b6115dc575b505060010161158f565b8a8a611664859f948f968661161e8f9360e061161561119795602061160d6113ff6113f1839f6116249c8991610d0c565b9a0151610c3c565b519b0151610c3c565b51610c3c565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610364578f6116d0908f936001959486955f926116db575b506116ca611550929351936116c56115778487610c3c565b610edd565b92610c3c565b019a90508b8d6115d2565b61155092506117036116ca9160203d811161170a575b6116fb8183610118565b810190610ebe565b92506116ad565b503d6116f1565b5093919796996001919699509a94929a01929190611338565b6115509250611747602091823d811161170a576116fb8183610118565b9250611541565b602061176f92503d8111611775575b6117678183610118565b810190610e0e565b5f6114a6565b503d61175d565b6117b9945061179692506113ff916113f191602095610d0c565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156103645760208961140d8f938f60a08f976113ff6113f18f8f906113eb6113dd6111618f60408b96918f88936111979f61183d90611843936114059f5f92611859575b5063ffffffff809116931690610d6d565b11610d7a565b50505050505097505050505050929350506113b1565b602063ffffffff9293508291611883913d811161188b575b61187b8183610118565b810190610d18565b92915061182c565b503d611871565b6118b4915060203d6020116118ba575b6118ac8183610118565b810190610cf4565b5f611331565b503d6118a2565b5f9190611335565b6118eb915060203d6020116118f1575b6118e38183610118565b810190610bde565b5f61111f565b503d6118d9565b5060e084015151851461103b565b5060c0840151518514611035565b5060a084015151851461102f565b6040519061192f826100f8565b5f6020838281520152565b6040519061018061194b8184610118565b368337565b6040519061195f602083610118565b6020368337565b91906040906060611975611922565b94859260208551926119878585610118565b8436853780518452015160208301528482015260076107cf195a01fa156119aa57565bfe5b6020929160806040926119bd611922565b958693818651936119ce8686610118565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa80156119aa57156119ff57565b60405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606490fd5b604051611a40816100f8565b6040908151611a4f8382610118565b8236823781526020825191611a648484610118565b8336843701528051611a768282610118565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6020820152815190611acc8383610118565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152611b2183519384610118565b8252602082015290565b5f5160206122005f395f51905f5290611b42611922565b505f919006602060c0835b611c42575f935f5160206122005f395f51905f5260038186818180090908604051611b788582610118565b84368237848185604051611b8c8282610118565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f5160206122005f395f51905f5260a082015260056107cf195a01fa80156119aa57611bf6906121b3565b5191611c42575f5160206122005f395f51905f5282800914611c2d57505f5160206122005f395f51905f5260015f94089293611b4d565b92935050611c3961014b565b92835282015290565b610864565b611c4f611922565b50604051611c5c816100f8565b600181526002602082015290565b90600682029180830460061490151715610c7257565b90600c81101561085f5760051b0190565b93929091611c9f604061015a565b9485526020850152611cb1604061015a565b9182526020820152611cc161193a565b925f5b60028110611cee57505050602061018092611cdd611950565b93849160086201d4c0fa9151151590565b80611cfa600192611c6a565b611d04828561084e565b5151611d108289611c80565b526020611d1d838661084e565b510151611d32611d2c83610d27565b89611c80565b52611d3d828661084e565b515151611d4c611d2c83610d35565b52611d62611d5a838761084e565b515160200190565b51611d6f611d2c83610d43565b526020611d7c838761084e565b51015151611d8c611d2c83610d51565b52611db8611db2611dab6020611da2868a61084e565b51015160200190565b5192610d5f565b88611c80565b5201611cc4565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff195f541660ff8216175f55604051908152a1565b906001611e0960ff936120be565b928392161b1115611e175790565b60405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608490fd5b805f915b611e8e575090565b5f198101818111610c725761ffff9116911661ffff8114610c72576001019080611e86565b90611ebc611922565b5061ffff811690610200821015611f615760018214611f5c57611edd61014b565b5f81525f602082015292906001905f925b61ffff8316851015611f0257505050505090565b600161ffff831660ff86161c811614611f3c575b6001611f32611f278360ff946119ac565b9460011b61fffe1690565b9401169291611eee565b946001611f32611f27611f518960ff956119ac565b989350505050611f16565b505090565b60405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606490fd5b611fa1611922565b50805190811580612012575b15611fce575050604051611fc2604082610118565b5f81525f602082015290565b60205f5160206122005f395f51905f52910151065f5160206122005f395f51905f52035f5160206122005f395f51905f528111610c725760405191611b21836100f8565b50602081015115611fad565b80511561085f5760200190565b90815181101561085f570160200190565b1561204357565b60405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a490fd5b9061010082511161213c57815115612137576120fa6120f06113ff6120e28561201e565b516001600160f81b03191690565b60ff600191161b90565b6001905b83518210156121325760019061211d6120f06113ff6120e2868961202b565b9061212981831161203c565b179101906120fe565b925050565b5f9150565b60a460405162461bcd60e51b815260206004820152604460248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b6084820152fd5b156121ba57565b60405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606490fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a2646970667358221220a7aa0fa1db70a8e363f6a47e43b2c8884660df82047eabf495c89edf277d636564736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0\x80`@R4a\x01qW` \x81a$\xBA\x808\x03\x80\x91a\0 \x82\x85a\x02\rV[\x839\x81\x01\x03\x12a\x01qWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\x01qW`\x80R`@Qch0H5`\xE0\x1B\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x01}W_\x91a\x01\xCAW[P`\xA0R`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\x01}W_\x91a\x01\x88W[P`\xC0R`\xA0Q`@Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x01}W_\x91a\x017W[P`\xE0R`@Qa\"u\x90\x81a\x02E\x829`\x80Q\x81\x81\x81a\x02\xC7\x01R\x81\x81a\x04\x06\x01R\x81\x81a\x10\xE4\x01R\x81\x81a\x11\xD7\x01Ra\x17\xBD\x01R`\xA0Q\x81\x81\x81a\x03\xC2\x01R\x81\x81a\x15\n\x01Ra\x16h\x01R`\xC0Q\x81\x81\x81a\x03~\x01Ra\x14F\x01R`\xE0Q\x81\x81\x81a\x08\x0B\x01Ra\x12\xF9\x01R\xF3[\x90P` \x81=` \x11a\x01uW[\x81a\x01R` \x93\x83a\x02\rV[\x81\x01\x03\x12a\x01qWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01qW_a\0\xC8V[_\x80\xFD[=\x91Pa\x01EV[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x01\xC2W[\x81a\x01\xA3` \x93\x83a\x02\rV[\x81\x01\x03\x12a\x01qWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01qW_a\0\x90V[=\x91Pa\x01\x96V[\x90P` \x81=` \x11a\x02\x05W[\x81a\x01\xE5` \x93\x83a\x02\rV[\x81\x01\x03\x12a\x01qWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01qW`\x04a\0dV[=\x91Pa\x01\xD8V[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x020W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x17\x1F\x1D[\x14a\0\x94W\x80cAl~^\x14a\0\x8FW\x80c]\xF4YF\x14a\0\x8AW\x80ch0H5\x14a\0\x85W\x80cm\x14\xA9\x87\x14a\0\x80W\x80cn\xFBF6\x14a\0{W\x80c\xB9\x8D\t\x08\x14a\0vWc\xDF\\\xF7#\x14a\0qW_\x80\xFD[a\x07\xF6V[a\x07\xD5V[a\x07<V[a\x03\xF1V[a\x03\xADV[a\x03iV[a\x02\x95V[4a\0\xE0Wa\x01 6`\x03\x19\x01\x12a\0\xE0W`@a\0\xD0`\x045a\0\xB76a\x01hV[a\0\xC06a\x02(V[\x90a\0\xCA6a\x01\x90V[\x92a\x08xV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\x13W`@RV[a\0\xE4V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\x13W`@RV[`@Q\x90a\x01Ia\x01\0\x83a\x01\x18V[V[`@Q\x90a\x01I`@\x83a\x01\x18V[\x90a\x01I`@Q\x92\x83a\x01\x18V[`@\x90`#\x19\x01\x12a\0\xE0W`@Q\x90a\x01\x81\x82a\0\xF8V[`$5\x82R`D5` \x83\x01RV[`@\x90`\xE3\x19\x01\x12a\0\xE0W`@Q\x90a\x01\xA9\x82a\0\xF8V[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\0\xE0W`@Qa\x01\xD1\x81a\0\xF8V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\0\xE0W`@Q\x91a\x01\xFC`@\x84a\x01\x18V[\x82\x90`@\x81\x01\x92\x83\x11a\0\xE0W\x90[\x82\x82\x10a\x02\x18WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x02\x0BV[\x90`\x80`c\x19\x83\x01\x12a\0\xE0W`@Qa\x02A\x81a\0\xF8V[` a\x02\\\x82\x94a\x02S\x81`da\x01\xE1V[\x84R`\xA4a\x01\xE1V[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\0\xE0W` a\x02\\`@Q\x92a\x02\x7F\x84a\0\xF8V[`@\x84\x96a\x02\x8D\x83\x82a\x01\xE1V[\x86R\x01a\x01\xE1V[4a\0\xE0W` 6`\x03\x19\x01\x12a\0\xE0W`\x045\x80\x15\x15\x81\x03a\0\xE0W`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03dW_\x91a\x03\x1EW[a\x03\x1C\x83a\x03\x173`\x01`\x01`\xA0\x1B\x03\x86\x16\x14a\tiV[a\x1D\xBFV[\0[\x90P` \x81=` \x11a\x03\\W[\x81a\x039` \x93\x83a\x01\x18V[\x81\x01\x03\x12a\0\xE0WQ\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xE0W\x90a\x03\x17a\x02\xFFV[=\x91Pa\x03,V[a\t^V[4a\0\xE0W_6`\x03\x19\x01\x12a\0\xE0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\0\xE0W_6`\x03\x19\x01\x12a\0\xE0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\0\xE0W_6`\x03\x19\x01\x12a\0\xE0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\0\xE0WV[`D5\x90a\x01I\x82a\x045V[`\x01`\x01`@\x1B\x03\x81\x11a\x01\x13W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\0\xE0W\x815a\x04~\x81a\x04PV[\x92a\x04\x8C`@Q\x94\x85a\x01\x18V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\0\xE0W` \x01\x90[\x82\x82\x10a\x04\xB4WPPP\x90V[` \x80\x91\x835a\x04\xC3\x81a\x045V[\x81R\x01\x91\x01\x90a\x04\xA7V[\x81`\x1F\x82\x01\x12\x15a\0\xE0W\x805a\x04\xE4\x81a\x04PV[\x92a\x04\xF2`@Q\x94\x85a\x01\x18V[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\0\xE0W` \x01\x91[\x83\x83\x10a\x05\x1CWPPPP\x90V[` `@\x91a\x05+\x84\x86a\x01\xB9V[\x81R\x01\x92\x01\x91a\x05\x0EV[\x90\x80`\x1F\x83\x01\x12\x15a\0\xE0W\x815a\x05M\x81a\x04PV[\x92a\x05[`@Q\x94\x85a\x01\x18V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\0\xE0W` \x82\x01\x90[\x83\x82\x10a\x05\x87WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W` \x91a\x05\xA9\x87\x84\x80\x94\x88\x01\x01a\x04gV[\x81R\x01\x91\x01\x90a\x05xV[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\0\xE0Wa\x05\xCAa\x019V[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W\x81a\x05\xE7\x91\x84\x01a\x04gV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W\x81a\x06\x08\x91\x84\x01a\x04\xCEV[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W\x81a\x06,\x91\x84\x01a\x04\xCEV[`@\x85\x01Ra\x06>\x81``\x84\x01a\x02aV[``\x85\x01Ra\x06P\x81`\xE0\x84\x01a\x01\xB9V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W\x81a\x06u\x91\x84\x01a\x04gV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W\x81a\x06\x9A\x91\x84\x01a\x04gV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0Wa\x06\xBD\x92\x01a\x056V[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xE1WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\xD4V[\x92\x91\x90a\x077` \x91`@\x86R\x82a\x07#\x82Q`@\x80\x8A\x01R`\x80\x89\x01\x90a\x06\xC4V[\x91\x01Q\x86\x82\x03`?\x19\x01``\x88\x01Ra\x06\xC4V[\x93\x01RV[4a\0\xE0W`\x806`\x03\x19\x01\x12a\0\xE0W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W6`#\x82\x01\x12\x15a\0\xE0W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W6`$\x82\x84\x01\x01\x11a\0\xE0Wa\x07\x93a\x04CV[\x90`d5\x93`\x01`\x01`@\x1B\x03\x85\x11a\0\xE0W`$a\x07\xB9a\x07\xC1\x966\x90`\x04\x01a\x05\xB4V[\x94\x01\x90a\x10\x07V[\x90a\x07\xD1`@Q\x92\x83\x92\x83a\x07\0V[\x03\x90\xF3[4a\0\xE0W_6`\x03\x19\x01\x12a\0\xE0W` `\xFF_T\x16`@Q\x90\x15\x15\x81R\xF3[4a\0\xE0W_6`\x03\x19\x01\x12a\0\xE0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x08_W`\x05\x1B\x01\x90V[a\x08:V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\tTa\t1a\tZ\x95a\t+a\t$\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x08\xFB\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x01\x18V[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96a\x19fV[\x90a\x19\xACV[\x92a\t+a\tFa\t@a\x1A4V[\x94a\x1B+V[\x91a\tOa\x1CGV[a\x19fV[\x91a\x1C\x91V[\x90\x91V[`@Q=_\x82>=\x90\xFD[\x15a\tpWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[`@Q\x90a\n\x0E\x82a\0\xF8V[``` \x83\x82\x81R\x01RV[\x15a\n!WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a\n\x80WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a\n\xE9WV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R\xFD[\x15a\x0BTWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x90a\x0B\xB6\x82a\x04PV[a\x0B\xC3`@Q\x91\x82a\x01\x18V[\x82\x81R\x80\x92a\x0B\xD4`\x1F\x19\x91a\x04PV[\x01\x90` 6\x91\x017V[\x90\x81` \x91\x03\x12a\0\xE0WQ`\xFF\x81\x16\x81\x03a\0\xE0W\x90V[\x92\x91\x92`\x01`\x01`@\x1B\x03\x82\x11a\x01\x13W`@Q\x91a\x0C `\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01\x18V[\x82\x94\x81\x84R\x81\x83\x01\x11a\0\xE0W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x80Q\x82\x10\x15a\x08_W` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x19\x81\x01\x91\x90\x82\x11a\x0CrWV[a\x0CPV[\x15a\x0C~WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R\xFD[\x90\x81` \x91\x03\x12a\0\xE0WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\0\xE0W\x90V[\x90\x81` \x91\x03\x12a\0\xE0WQa\r\t\x81a\x045V[\x90V[\x90\x82\x10\x15a\x08_W\x01\x90V[\x90\x81` \x91\x03\x12a\0\xE0WQ\x90V[\x90`\x01\x82\x01\x80\x92\x11a\x0CrWV[\x90`\x02\x82\x01\x80\x92\x11a\x0CrWV[\x90`\x03\x82\x01\x80\x92\x11a\x0CrWV[\x90`\x04\x82\x01\x80\x92\x11a\x0CrWV[\x90`\x05\x82\x01\x80\x92\x11a\x0CrWV[\x91\x90\x82\x01\x80\x92\x11a\x0CrWV[\x15a\r\x81WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90\x81` \x91\x03\x12a\0\xE0WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\0\xE0W\x90V[\x15a\x0E6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90\x81` \x91\x03\x12a\0\xE0WQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\0\xE0W\x90V[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a\x0CrWV[\x15a\x0F\x04WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a\x0FoWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a\x0F\xF1WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0F\xE4V[\x94\x93\x92\x90\x91\x93a\x10\x15a\n\x01V[Pa\x10!\x85\x15\x15a\n\x1AV[`@\x84\x01QQ\x85\x14\x80a\x19\x14W[\x80a\x19\x06W[\x80a\x18\xF8W[a\x10D\x90a\nyV[a\x10V` \x85\x01QQ\x85QQ\x14a\n\xE2V[a\x10mc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a\x0BMV[a\x10ua\x01KV[_\x81R_` \x82\x01R\x92a\x10\x87a\n\x01V[a\x10\x90\x87a\x0B\xACV[` \x82\x01Ra\x10\x9E\x87a\x0B\xACV[\x81Ra\x10\xA8a\n\x01V[\x92a\x10\xB7` \x88\x01QQa\x0B\xACV[\x84Ra\x10\xC7` \x88\x01QQa\x0B\xACV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03dWa\x110\x91_\x91a\x18\xC9W[Pa\x11+6\x8B\x87a\x0B\xF7V[a\x1D\xFBV[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a\x12\xBCW` \x88a\x11\xA1a\x11\x97\x8Ca\x11\x8F\x8F\x96\x86\x8Ea\x11ta\x11a\x86\x80\x95a\x0C<V[Q\x80Q_R` \x01Q` R`@_ \x90V[a\x11\x81\x84\x84\x84\x01Qa\x0C<V[R\x82a\x12\x89W[\x01Qa\x0C<V[Q\x95Qa\x0C<V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x03dWa\t+\x8Aa\x12N\x8Fa\x12G\x8F\x84` \x8F\x92a\x12>\x93a\x126\x84`\x01\x9Ea\x12T\x9E_\x91a\x12\\W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\x0C<V[R\x01Qa\x0C<V[Q\x93\x8DQa\x0C<V[Q\x16a\x1E\x82V[\x90a\x1E\xB3V[\x97\x01\x96a\x114V[a\x12|\x91P\x86=\x81\x11a\x12\x82W[a\x12t\x81\x83a\x01\x18V[\x81\x01\x90a\x0C\xD5V[_a\x12'V[P=a\x12jV[a\x12\xB7a\x12\x99\x84\x84\x84\x01Qa\x0C<V[Qa\x12\xB0\x84\x84\x01Qa\x12\xAA\x87a\x0CdV[\x90a\x0C<V[Q\x10a\x0CwV[a\x11\x88V[P\x90\x95\x97\x94\x96Pa\x12\xD1\x91\x98\x93\x92\x99Pa\x1F\x99V[\x91a\x12\xDD_T`\xFF\x16\x90V[\x90\x81\x15a\x18\xC1W`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03dW_\x91a\x18\x92W[P\x91\x90[_\x92[\x81\x84\x10a\x13\x9CWPPPPP\x92a\x13ua\x13pa\x13ia\x13\x96\x95\x85a\x13\x88\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x08xV[\x91\x90a\x0E\xFDV[a\x0FhV[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a\x0F\xC7V[\x03`\x1F\x19\x81\x01\x83R\x82a\x01\x18V[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da\x17|W[a\x11\x97\x82`\xA0a\x14\x05a\x13\xFFa\x13\xF1\x84a\x14\r\x97a\x13\xEBa\x13\xDDa\x11a\x8F\x9C`@` \x9F\x9E\x01Qa\x0C<V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba\r\x0CV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[\x97\x01Qa\x0C<V[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03dWa\x14\xD1a\x11\x97\x8F\x95\x8F\x90a\x14\xC9\x8F\x97\x8F\x96\x84\x8Fa\x14\xC3`\xC0\x96a\x14\xBC\x84\x8F` \x9F\x90a\x11\x88a\x13\xF1\x99`@\x93a\x13\xFF\x9C_\x91a\x17NW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a\x0E/V[Q\x90a\x19\xACV[\x9Ca\r\x0CV[\x96\x01Qa\x0C<V[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03dWa\x15^\x91\x8C\x8F\x92_\x92a\x17*W[P` a\x15P\x92\x93\x01Qa\x0C<V[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a\x15\x8B\x8Ca\x15P\x8Ca\x15\x84a\x15w\x82` \x86\x01Qa\x0C<V[Q`\x01`\x01``\x1B\x03\x16\x90V[\x92Qa\x0C<V[_\x98_[` \x8A\x01QQ\x81\x10\x15a\x17\x11W\x8B\x8Da\x15\xCD\x89a\x15\xC0a\x13\xFFa\x13\xF1\x86\x8F\x89a\x15\xB8\x91Qa\x0C<V[Q\x94\x87a\r\x0CV[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a\x15\xDCW[PP`\x01\x01a\x15\x8FV[\x8A\x8Aa\x16d\x85\x9F\x94\x8F\x96\x86a\x16\x1E\x8F\x93`\xE0a\x16\x15a\x11\x97\x95` a\x16\ra\x13\xFFa\x13\xF1\x83\x9Fa\x16$\x9C\x89\x91a\r\x0CV[\x9A\x01Qa\x0C<V[Q\x9B\x01Qa\x0C<V[Qa\x0C<V[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03dW\x8Fa\x16\xD0\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a\x16\xDBW[Pa\x16\xCAa\x15P\x92\x93Q\x93a\x16\xC5a\x15w\x84\x87a\x0C<V[a\x0E\xDDV[\x92a\x0C<V[\x01\x9A\x90P\x8B\x8Da\x15\xD2V[a\x15P\x92Pa\x17\x03a\x16\xCA\x91` =\x81\x11a\x17\nW[a\x16\xFB\x81\x83a\x01\x18V[\x81\x01\x90a\x0E\xBEV[\x92Pa\x16\xADV[P=a\x16\xF1V[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a\x138V[a\x15P\x92Pa\x17G` \x91\x82=\x81\x11a\x17\nWa\x16\xFB\x81\x83a\x01\x18V[\x92Pa\x15AV[` a\x17o\x92P=\x81\x11a\x17uW[a\x17g\x81\x83a\x01\x18V[\x81\x01\x90a\x0E\x0EV[_a\x14\xA6V[P=a\x17]V[a\x17\xB9\x94Pa\x17\x96\x92Pa\x13\xFF\x91a\x13\xF1\x91` \x95a\r\x0CV[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03dW` \x89a\x14\r\x8F\x93\x8F`\xA0\x8F\x97a\x13\xFFa\x13\xF1\x8F\x8F\x90a\x13\xEBa\x13\xDDa\x11a\x8F`@\x8B\x96\x91\x8F\x88\x93a\x11\x97\x9Fa\x18=\x90a\x18C\x93a\x14\x05\x9F_\x92a\x18YW[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a\rmV[\x11a\rzV[PPPPPP\x97PPPPPP\x92\x93PPa\x13\xB1V[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a\x18\x83\x91=\x81\x11a\x18\x8BW[a\x18{\x81\x83a\x01\x18V[\x81\x01\x90a\r\x18V[\x92\x91Pa\x18,V[P=a\x18qV[a\x18\xB4\x91P` =` \x11a\x18\xBAW[a\x18\xAC\x81\x83a\x01\x18V[\x81\x01\x90a\x0C\xF4V[_a\x131V[P=a\x18\xA2V[_\x91\x90a\x135V[a\x18\xEB\x91P` =` \x11a\x18\xF1W[a\x18\xE3\x81\x83a\x01\x18V[\x81\x01\x90a\x0B\xDEV[_a\x11\x1FV[P=a\x18\xD9V[P`\xE0\x84\x01QQ\x85\x14a\x10;V[P`\xC0\x84\x01QQ\x85\x14a\x105V[P`\xA0\x84\x01QQ\x85\x14a\x10/V[`@Q\x90a\x19/\x82a\0\xF8V[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80a\x19K\x81\x84a\x01\x18V[6\x837V[`@Q\x90a\x19_` \x83a\x01\x18V[` 6\x837V[\x91\x90`@\x90``a\x19ua\x19\"V[\x94\x85\x92` \x85Q\x92a\x19\x87\x85\x85a\x01\x18V[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15a\x19\xAAWV[\xFE[` \x92\x91`\x80`@\x92a\x19\xBDa\x19\"V[\x95\x86\x93\x81\x86Q\x93a\x19\xCE\x86\x86a\x01\x18V[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15a\x19\xAAW\x15a\x19\xFFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x90\xFD[`@Qa\x1A@\x81a\0\xF8V[`@\x90\x81Qa\x1AO\x83\x82a\x01\x18V[\x826\x827\x81R` \x82Q\x91a\x1Ad\x84\x84a\x01\x18V[\x836\x847\x01R\x80Qa\x1Av\x82\x82a\x01\x18V[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90a\x1A\xCC\x83\x83a\x01\x18V[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01Ra\x1B!\x83Q\x93\x84a\x01\x18V[\x82R` \x82\x01R\x90V[_Q` a\"\0_9_Q\x90_R\x90a\x1BBa\x19\"V[P_\x91\x90\x06` `\xC0\x83[a\x1CBW_\x93_Q` a\"\0_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa\x1Bx\x85\x82a\x01\x18V[\x846\x827\x84\x81\x85`@Qa\x1B\x8C\x82\x82a\x01\x18V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` a\"\0_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a\x19\xAAWa\x1B\xF6\x90a!\xB3V[Q\x91a\x1CBW_Q` a\"\0_9_Q\x90_R\x82\x80\t\x14a\x1C-WP_Q` a\"\0_9_Q\x90_R`\x01_\x94\x08\x92\x93a\x1BMV[\x92\x93PPa\x1C9a\x01KV[\x92\x83R\x82\x01R\x90V[a\x08dV[a\x1COa\x19\"V[P`@Qa\x1C\\\x81a\0\xF8V[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a\x0CrWV[\x90`\x0C\x81\x10\x15a\x08_W`\x05\x1B\x01\x90V[\x93\x92\x90\x91a\x1C\x9F`@a\x01ZV[\x94\x85R` \x85\x01Ra\x1C\xB1`@a\x01ZV[\x91\x82R` \x82\x01Ra\x1C\xC1a\x19:V[\x92_[`\x02\x81\x10a\x1C\xEEWPPP` a\x01\x80\x92a\x1C\xDDa\x19PV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80a\x1C\xFA`\x01\x92a\x1CjV[a\x1D\x04\x82\x85a\x08NV[QQa\x1D\x10\x82\x89a\x1C\x80V[R` a\x1D\x1D\x83\x86a\x08NV[Q\x01Qa\x1D2a\x1D,\x83a\r'V[\x89a\x1C\x80V[Ra\x1D=\x82\x86a\x08NV[QQQa\x1DLa\x1D,\x83a\r5V[Ra\x1Dba\x1DZ\x83\x87a\x08NV[QQ` \x01\x90V[Qa\x1Doa\x1D,\x83a\rCV[R` a\x1D|\x83\x87a\x08NV[Q\x01QQa\x1D\x8Ca\x1D,\x83a\rQV[Ra\x1D\xB8a\x1D\xB2a\x1D\xAB` a\x1D\xA2\x86\x8Aa\x08NV[Q\x01Q` \x01\x90V[Q\x92a\r_V[\x88a\x1C\x80V[R\x01a\x1C\xC4V[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19_T\x16`\xFF\x82\x16\x17_U`@Q\x90\x81R\xA1V[\x90`\x01a\x1E\t`\xFF\x93a \xBEV[\x92\x83\x92\x16\x1B\x11\x15a\x1E\x17W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x90\xFD[\x80_\x91[a\x1E\x8EWP\x90V[_\x19\x81\x01\x81\x81\x11a\x0CrWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x0CrW`\x01\x01\x90\x80a\x1E\x86V[\x90a\x1E\xBCa\x19\"V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15a\x1FaW`\x01\x82\x14a\x1F\\Wa\x1E\xDDa\x01KV[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15a\x1F\x02WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14a\x1F<W[`\x01a\x1F2a\x1F'\x83`\xFF\x94a\x19\xACV[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91a\x1E\xEEV[\x94`\x01a\x1F2a\x1F'a\x1FQ\x89`\xFF\x95a\x19\xACV[\x98\x93PPPPa\x1F\x16V[PP\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x1F\xA1a\x19\"V[P\x80Q\x90\x81\x15\x80a \x12W[\x15a\x1F\xCEWPP`@Qa\x1F\xC2`@\x82a\x01\x18V[_\x81R_` \x82\x01R\x90V[` _Q` a\"\0_9_Q\x90_R\x91\x01Q\x06_Q` a\"\0_9_Q\x90_R\x03_Q` a\"\0_9_Q\x90_R\x81\x11a\x0CrW`@Q\x91a\x1B!\x83a\0\xF8V[P` \x81\x01Q\x15a\x1F\xADV[\x80Q\x15a\x08_W` \x01\x90V[\x90\x81Q\x81\x10\x15a\x08_W\x01` \x01\x90V[\x15a CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90a\x01\0\x82Q\x11a!<W\x81Q\x15a!7Wa \xFAa \xF0a\x13\xFFa \xE2\x85a \x1EV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xFF`\x01\x91\x16\x1B\x90V[`\x01\x90[\x83Q\x82\x10\x15a!2W`\x01\x90a!\x1Da \xF0a\x13\xFFa \xE2\x86\x89a +V[\x90a!)\x81\x83\x11a <V[\x17\x91\x01\x90a \xFEV[\x92PPV[_\x91PV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R\xFD[\x15a!\xBAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \xA7\xAA\x0F\xA1\xDBp\xA8\xE3c\xF6\xA4~C\xB2\xC8\x88F`\xDF\x82\x04~\xAB\xF4\x95\xC8\x9E\xDF'}cedsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8063171f1d5b14610094578063416c7e5e1461008f5780635df459461461008a57806368304835146100855780636d14a987146100805780636efb46361461007b578063b98d0908146100765763df5cf72314610071575f80fd5b6107f6565b6107d5565b61073c565b6103f1565b6103ad565b610369565b610295565b346100e0576101203660031901126100e05760406100d06004356100b736610168565b6100c036610228565b906100ca36610190565b92610878565b8251911515825215156020820152f35b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761011357604052565b6100e4565b90601f801991011681019081106001600160401b0382111761011357604052565b6040519061014961010083610118565b565b60405190610149604083610118565b906101496040519283610118565b60409060231901126100e05760405190610181826100f8565b60243582526044356020830152565b60409060e31901126100e057604051906101a9826100f8565b60e4358252610104356020830152565b91908260409103126100e0576040516101d1816100f8565b6020808294803584520135910152565b9080601f830112156100e057604051916101fc604084610118565b8290604081019283116100e057905b8282106102185750505090565b813581526020918201910161020b565b9060806063198301126100e057604051610241816100f8565b602061025c82946102538160646101e1565b845260a46101e1565b910152565b91906080838203126100e057602061025c6040519261027f846100f8565b6040849661028d83826101e1565b8652016101e1565b346100e05760203660031901126100e05760043580151581036100e057604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610364575f9161031e575b61031c83610317336001600160a01b03861614610969565b611dbf565b005b90506020813d60201161035c575b8161033960209383610118565b810103126100e05751906001600160a01b03821682036100e057906103176102ff565b3d915061032c565b61095e565b346100e0575f3660031901126100e0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346100e0575f3660031901126100e0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346100e0575f3660031901126100e0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b63ffffffff8116036100e057565b6044359061014982610435565b6001600160401b0381116101135760051b60200190565b9080601f830112156100e057813561047e81610450565b9261048c6040519485610118565b81845260208085019260051b8201019283116100e057602001905b8282106104b45750505090565b6020809183356104c381610435565b8152019101906104a7565b81601f820112156100e05780356104e481610450565b926104f26040519485610118565b81845260208085019260061b840101928184116100e057602001915b83831061051c575050505090565b602060409161052b84866101b9565b81520192019161050e565b9080601f830112156100e057813561054d81610450565b9261055b6040519485610118565b81845260208085019260051b820101918383116100e05760208201905b83821061058757505050505090565b81356001600160401b0381116100e0576020916105a987848094880101610467565b815201910190610578565b919091610180818403126100e0576105ca610139565b9281356001600160401b0381116100e057816105e7918401610467565b845260208201356001600160401b0381116100e057816106089184016104ce565b602085015260408201356001600160401b0381116100e0578161062c9184016104ce565b604085015261063e8160608401610261565b60608501526106508160e084016101b9565b60808501526101208201356001600160401b0381116100e05781610675918401610467565b60a08501526101408201356001600160401b0381116100e0578161069a918401610467565b60c08501526101608201356001600160401b0381116100e0576106bd9201610536565b60e0830152565b90602080835192838152019201905f5b8181106106e15750505090565b82516001600160601b03168452602093840193909201916001016106d4565b929190610737602091604086528261072382516040808a015260808901906106c4565b910151868203603f190160608801526106c4565b930152565b346100e05760803660031901126100e0576004356024356001600160401b0381116100e057366023820112156100e05780600401356001600160401b0381116100e05736602482840101116100e057610793610443565b90606435936001600160401b0385116100e05760246107b96107c19636906004016105b4565b940190611007565b906107d160405192839283610700565b0390f35b346100e0575f3660031901126100e057602060ff5f54166040519015158152f35b346100e0575f3660031901126100e0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b634e487b7160e01b5f52603260045260245ffd5b90600281101561085f5760051b0190565b61083a565b634e487b7160e01b5f52601260045260245ffd5b61095461093161095a9561092b61092485875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e08401526101008301526108fb81610120840103601f198101835282610118565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b8096611966565b906119ac565b9261092b610946610940611a34565b94611b2b565b9161094f611c47565b611966565b91611c91565b9091565b6040513d5f823e3d90fd5b1561097057565b60405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a490fd5b60405190610a0e826100f8565b60606020838281520152565b15610a2157565b60405162461bcd60e51b815260206004820152603760248201525f5160206122205f395f51905f5260448201527f7265733a20656d7074792071756f72756d20696e7075740000000000000000006064820152608490fd5b15610a8057565b60405162461bcd60e51b815260206004820152604160248201525f5160206122205f395f51905f5260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a490fd5b15610ae957565b60a460405162461bcd60e51b815260206004820152604460248201525f5160206122205f395f51905f5260448201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b6084820152fd5b15610b5457565b60405162461bcd60e51b815260206004820152603c60248201525f5160206122205f395f51905f5260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608490fd5b90610bb682610450565b610bc36040519182610118565b8281528092610bd4601f1991610450565b0190602036910137565b908160209103126100e0575160ff811681036100e05790565b9291926001600160401b0382116101135760405191610c20601f8201601f191660200184610118565b8294818452818301116100e0578281602093845f960137010152565b805182101561085f5760209160051b010190565b634e487b7160e01b5f52601160045260245ffd5b5f19810191908211610c7257565b610c50565b15610c7e57565b608460405162461bcd60e51b815260206004820152604060248201525f5160206122205f395f51905f5260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152fd5b908160209103126100e057516001600160c01b03811681036100e05790565b908160209103126100e05751610d0981610435565b90565b9082101561085f570190565b908160209103126100e0575190565b9060018201809211610c7257565b9060028201809211610c7257565b9060038201809211610c7257565b9060048201809211610c7257565b9060058201809211610c7257565b91908201809211610c7257565b15610d8157565b60405162461bcd60e51b815260206004820152606660248201525f5160206122205f395f51905f5260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c490fd5b908160209103126100e0575167ffffffffffffffff19811681036100e05790565b15610e3657565b60405162461bcd60e51b815260206004820152606160248201525f5160206122205f395f51905f5260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c490fd5b908160209103126100e057516001600160601b03811681036100e05790565b906001600160601b03809116911603906001600160601b038211610c7257565b15610f0457565b60405162461bcd60e51b815260206004820152604360248201525f5160206122205f395f51905f5260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a490fd5b15610f6f57565b60405162461bcd60e51b815260206004820152603960248201525f5160206122205f395f51905f5260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608490fd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b818110610ff15750505090565b8251845260209384019390920191600101610fe4565b949392909193611015610a01565b50611021851515610a1a565b604084015151851480611914575b80611906575b806118f8575b61104490610a79565b61105660208501515185515114610ae2565b61106d63ffffffff431663ffffffff841610610b4d565b61107561014b565b5f81525f602082015292611087610a01565b61109087610bac565b602082015261109e87610bac565b81526110a8610a01565b926110b7602088015151610bac565b84526110c7602088015151610bac565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561036457611130915f916118c9575b5061112b368b87610bf7565b611dfb565b985f965b602089015180518910156112bc576020886111a16111978c61118f8f96868e611174611161868095610c3c565b5180515f526020015160205260405f2090565b6111818484840151610c3c565b5282611289575b0151610c3c565b519551610c3c565b5163ffffffff1690565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9182156103645761092b8a61124e8f6112478f8460208f9261123e936112368460019e6112549e5f9161125c575b508f8060c01b03169251610c3c565b520151610c3c565b51938d51610c3c565b5116611e82565b90611eb3565b970196611134565b61127c9150863d8111611282575b6112748183610118565b810190610cd5565b5f611227565b503d61126a565b6112b76112998484840151610c3c565b516112b0848401516112aa87610c64565b90610c3c565b5110610c77565b611188565b509095979496506112d1919893929950611f99565b916112dd5f5460ff1690565b9081156118c1576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610364575f91611892575b5091905b5f925b81841061139c5750505050509261137561137061136961139695856113889860806060602099015192015192610878565b9190610efd565b610f68565b0151604051928391602083019586610fc7565b03601f198101835282610118565b51902090565b92989596909399919794878b888c888d61177c575b6111978260a06114056113ff6113f18461140d976113eb6113dd6111618f9c604060209f9e0151610c3c565b67ffffffffffffffff191690565b9b610d0c565b356001600160f81b03191690565b60f81c90565b970151610c3c565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa908115610364576114d16111978f958f906114c98f978f96848f6114c360c0966114bc848f60209f906111886113f1996040936113ff9c5f9161174e575b5067ffffffffffffffff19918216911614610e2f565b51906119ac565b9c610d0c565b960151610c3c565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156103645761155e918c8f925f9261172a575b50602061155092930151610c3c565b906001600160601b03169052565b61158b8c6115508c611584611577826020860151610c3c565b516001600160601b031690565b9251610c3c565b5f985f5b60208a015151811015611711578b8d6115cd896115c06113ff6113f1868f896115b89151610c3c565b519487610d0c565b60ff161c60019081161490565b6115dc575b505060010161158f565b8a8a611664859f948f968661161e8f9360e061161561119795602061160d6113ff6113f1839f6116249c8991610d0c565b9a0151610c3c565b519b0151610c3c565b51610c3c565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610364578f6116d0908f936001959486955f926116db575b506116ca611550929351936116c56115778487610c3c565b610edd565b92610c3c565b019a90508b8d6115d2565b61155092506117036116ca9160203d811161170a575b6116fb8183610118565b810190610ebe565b92506116ad565b503d6116f1565b5093919796996001919699509a94929a01929190611338565b6115509250611747602091823d811161170a576116fb8183610118565b9250611541565b602061176f92503d8111611775575b6117678183610118565b810190610e0e565b5f6114a6565b503d61175d565b6117b9945061179692506113ff916113f191602095610d0c565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156103645760208961140d8f938f60a08f976113ff6113f18f8f906113eb6113dd6111618f60408b96918f88936111979f61183d90611843936114059f5f92611859575b5063ffffffff809116931690610d6d565b11610d7a565b50505050505097505050505050929350506113b1565b602063ffffffff9293508291611883913d811161188b575b61187b8183610118565b810190610d18565b92915061182c565b503d611871565b6118b4915060203d6020116118ba575b6118ac8183610118565b810190610cf4565b5f611331565b503d6118a2565b5f9190611335565b6118eb915060203d6020116118f1575b6118e38183610118565b810190610bde565b5f61111f565b503d6118d9565b5060e084015151851461103b565b5060c0840151518514611035565b5060a084015151851461102f565b6040519061192f826100f8565b5f6020838281520152565b6040519061018061194b8184610118565b368337565b6040519061195f602083610118565b6020368337565b91906040906060611975611922565b94859260208551926119878585610118565b8436853780518452015160208301528482015260076107cf195a01fa156119aa57565bfe5b6020929160806040926119bd611922565b958693818651936119ce8686610118565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa80156119aa57156119ff57565b60405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606490fd5b604051611a40816100f8565b6040908151611a4f8382610118565b8236823781526020825191611a648484610118565b8336843701528051611a768282610118565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6020820152815190611acc8383610118565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152611b2183519384610118565b8252602082015290565b5f5160206122005f395f51905f5290611b42611922565b505f919006602060c0835b611c42575f935f5160206122005f395f51905f5260038186818180090908604051611b788582610118565b84368237848185604051611b8c8282610118565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f5160206122005f395f51905f5260a082015260056107cf195a01fa80156119aa57611bf6906121b3565b5191611c42575f5160206122005f395f51905f5282800914611c2d57505f5160206122005f395f51905f5260015f94089293611b4d565b92935050611c3961014b565b92835282015290565b610864565b611c4f611922565b50604051611c5c816100f8565b600181526002602082015290565b90600682029180830460061490151715610c7257565b90600c81101561085f5760051b0190565b93929091611c9f604061015a565b9485526020850152611cb1604061015a565b9182526020820152611cc161193a565b925f5b60028110611cee57505050602061018092611cdd611950565b93849160086201d4c0fa9151151590565b80611cfa600192611c6a565b611d04828561084e565b5151611d108289611c80565b526020611d1d838661084e565b510151611d32611d2c83610d27565b89611c80565b52611d3d828661084e565b515151611d4c611d2c83610d35565b52611d62611d5a838761084e565b515160200190565b51611d6f611d2c83610d43565b526020611d7c838761084e565b51015151611d8c611d2c83610d51565b52611db8611db2611dab6020611da2868a61084e565b51015160200190565b5192610d5f565b88611c80565b5201611cc4565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff195f541660ff8216175f55604051908152a1565b906001611e0960ff936120be565b928392161b1115611e175790565b60405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608490fd5b805f915b611e8e575090565b5f198101818111610c725761ffff9116911661ffff8114610c72576001019080611e86565b90611ebc611922565b5061ffff811690610200821015611f615760018214611f5c57611edd61014b565b5f81525f602082015292906001905f925b61ffff8316851015611f0257505050505090565b600161ffff831660ff86161c811614611f3c575b6001611f32611f278360ff946119ac565b9460011b61fffe1690565b9401169291611eee565b946001611f32611f27611f518960ff956119ac565b989350505050611f16565b505090565b60405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606490fd5b611fa1611922565b50805190811580612012575b15611fce575050604051611fc2604082610118565b5f81525f602082015290565b60205f5160206122005f395f51905f52910151065f5160206122005f395f51905f52035f5160206122005f395f51905f528111610c725760405191611b21836100f8565b50602081015115611fad565b80511561085f5760200190565b90815181101561085f570160200190565b1561204357565b60405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a490fd5b9061010082511161213c57815115612137576120fa6120f06113ff6120e28561201e565b516001600160f81b03191690565b60ff600191161b90565b6001905b83518210156121325760019061211d6120f06113ff6120e2868961202b565b9061212981831161203c565b179101906120fe565b925050565b5f9150565b60a460405162461bcd60e51b815260206004820152604460248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b6084820152fd5b156121ba57565b60405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606490fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a2646970667358221220a7aa0fa1db70a8e363f6a47e43b2c8884660df82047eabf495c89edf277d636564736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x17\x1F\x1D[\x14a\0\x94W\x80cAl~^\x14a\0\x8FW\x80c]\xF4YF\x14a\0\x8AW\x80ch0H5\x14a\0\x85W\x80cm\x14\xA9\x87\x14a\0\x80W\x80cn\xFBF6\x14a\0{W\x80c\xB9\x8D\t\x08\x14a\0vWc\xDF\\\xF7#\x14a\0qW_\x80\xFD[a\x07\xF6V[a\x07\xD5V[a\x07<V[a\x03\xF1V[a\x03\xADV[a\x03iV[a\x02\x95V[4a\0\xE0Wa\x01 6`\x03\x19\x01\x12a\0\xE0W`@a\0\xD0`\x045a\0\xB76a\x01hV[a\0\xC06a\x02(V[\x90a\0\xCA6a\x01\x90V[\x92a\x08xV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\x13W`@RV[a\0\xE4V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\x13W`@RV[`@Q\x90a\x01Ia\x01\0\x83a\x01\x18V[V[`@Q\x90a\x01I`@\x83a\x01\x18V[\x90a\x01I`@Q\x92\x83a\x01\x18V[`@\x90`#\x19\x01\x12a\0\xE0W`@Q\x90a\x01\x81\x82a\0\xF8V[`$5\x82R`D5` \x83\x01RV[`@\x90`\xE3\x19\x01\x12a\0\xE0W`@Q\x90a\x01\xA9\x82a\0\xF8V[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\0\xE0W`@Qa\x01\xD1\x81a\0\xF8V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\0\xE0W`@Q\x91a\x01\xFC`@\x84a\x01\x18V[\x82\x90`@\x81\x01\x92\x83\x11a\0\xE0W\x90[\x82\x82\x10a\x02\x18WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x02\x0BV[\x90`\x80`c\x19\x83\x01\x12a\0\xE0W`@Qa\x02A\x81a\0\xF8V[` a\x02\\\x82\x94a\x02S\x81`da\x01\xE1V[\x84R`\xA4a\x01\xE1V[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\0\xE0W` a\x02\\`@Q\x92a\x02\x7F\x84a\0\xF8V[`@\x84\x96a\x02\x8D\x83\x82a\x01\xE1V[\x86R\x01a\x01\xE1V[4a\0\xE0W` 6`\x03\x19\x01\x12a\0\xE0W`\x045\x80\x15\x15\x81\x03a\0\xE0W`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03dW_\x91a\x03\x1EW[a\x03\x1C\x83a\x03\x173`\x01`\x01`\xA0\x1B\x03\x86\x16\x14a\tiV[a\x1D\xBFV[\0[\x90P` \x81=` \x11a\x03\\W[\x81a\x039` \x93\x83a\x01\x18V[\x81\x01\x03\x12a\0\xE0WQ\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xE0W\x90a\x03\x17a\x02\xFFV[=\x91Pa\x03,V[a\t^V[4a\0\xE0W_6`\x03\x19\x01\x12a\0\xE0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\0\xE0W_6`\x03\x19\x01\x12a\0\xE0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\0\xE0W_6`\x03\x19\x01\x12a\0\xE0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\0\xE0WV[`D5\x90a\x01I\x82a\x045V[`\x01`\x01`@\x1B\x03\x81\x11a\x01\x13W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\0\xE0W\x815a\x04~\x81a\x04PV[\x92a\x04\x8C`@Q\x94\x85a\x01\x18V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\0\xE0W` \x01\x90[\x82\x82\x10a\x04\xB4WPPP\x90V[` \x80\x91\x835a\x04\xC3\x81a\x045V[\x81R\x01\x91\x01\x90a\x04\xA7V[\x81`\x1F\x82\x01\x12\x15a\0\xE0W\x805a\x04\xE4\x81a\x04PV[\x92a\x04\xF2`@Q\x94\x85a\x01\x18V[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\0\xE0W` \x01\x91[\x83\x83\x10a\x05\x1CWPPPP\x90V[` `@\x91a\x05+\x84\x86a\x01\xB9V[\x81R\x01\x92\x01\x91a\x05\x0EV[\x90\x80`\x1F\x83\x01\x12\x15a\0\xE0W\x815a\x05M\x81a\x04PV[\x92a\x05[`@Q\x94\x85a\x01\x18V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\0\xE0W` \x82\x01\x90[\x83\x82\x10a\x05\x87WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W` \x91a\x05\xA9\x87\x84\x80\x94\x88\x01\x01a\x04gV[\x81R\x01\x91\x01\x90a\x05xV[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\0\xE0Wa\x05\xCAa\x019V[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W\x81a\x05\xE7\x91\x84\x01a\x04gV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W\x81a\x06\x08\x91\x84\x01a\x04\xCEV[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W\x81a\x06,\x91\x84\x01a\x04\xCEV[`@\x85\x01Ra\x06>\x81``\x84\x01a\x02aV[``\x85\x01Ra\x06P\x81`\xE0\x84\x01a\x01\xB9V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W\x81a\x06u\x91\x84\x01a\x04gV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W\x81a\x06\x9A\x91\x84\x01a\x04gV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0Wa\x06\xBD\x92\x01a\x056V[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xE1WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\xD4V[\x92\x91\x90a\x077` \x91`@\x86R\x82a\x07#\x82Q`@\x80\x8A\x01R`\x80\x89\x01\x90a\x06\xC4V[\x91\x01Q\x86\x82\x03`?\x19\x01``\x88\x01Ra\x06\xC4V[\x93\x01RV[4a\0\xE0W`\x806`\x03\x19\x01\x12a\0\xE0W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W6`#\x82\x01\x12\x15a\0\xE0W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\0\xE0W6`$\x82\x84\x01\x01\x11a\0\xE0Wa\x07\x93a\x04CV[\x90`d5\x93`\x01`\x01`@\x1B\x03\x85\x11a\0\xE0W`$a\x07\xB9a\x07\xC1\x966\x90`\x04\x01a\x05\xB4V[\x94\x01\x90a\x10\x07V[\x90a\x07\xD1`@Q\x92\x83\x92\x83a\x07\0V[\x03\x90\xF3[4a\0\xE0W_6`\x03\x19\x01\x12a\0\xE0W` `\xFF_T\x16`@Q\x90\x15\x15\x81R\xF3[4a\0\xE0W_6`\x03\x19\x01\x12a\0\xE0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x08_W`\x05\x1B\x01\x90V[a\x08:V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\tTa\t1a\tZ\x95a\t+a\t$\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x08\xFB\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x01\x18V[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96a\x19fV[\x90a\x19\xACV[\x92a\t+a\tFa\t@a\x1A4V[\x94a\x1B+V[\x91a\tOa\x1CGV[a\x19fV[\x91a\x1C\x91V[\x90\x91V[`@Q=_\x82>=\x90\xFD[\x15a\tpWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[`@Q\x90a\n\x0E\x82a\0\xF8V[``` \x83\x82\x81R\x01RV[\x15a\n!WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a\n\x80WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a\n\xE9WV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R\xFD[\x15a\x0BTWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x90a\x0B\xB6\x82a\x04PV[a\x0B\xC3`@Q\x91\x82a\x01\x18V[\x82\x81R\x80\x92a\x0B\xD4`\x1F\x19\x91a\x04PV[\x01\x90` 6\x91\x017V[\x90\x81` \x91\x03\x12a\0\xE0WQ`\xFF\x81\x16\x81\x03a\0\xE0W\x90V[\x92\x91\x92`\x01`\x01`@\x1B\x03\x82\x11a\x01\x13W`@Q\x91a\x0C `\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01\x18V[\x82\x94\x81\x84R\x81\x83\x01\x11a\0\xE0W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x80Q\x82\x10\x15a\x08_W` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x19\x81\x01\x91\x90\x82\x11a\x0CrWV[a\x0CPV[\x15a\x0C~WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R\xFD[\x90\x81` \x91\x03\x12a\0\xE0WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\0\xE0W\x90V[\x90\x81` \x91\x03\x12a\0\xE0WQa\r\t\x81a\x045V[\x90V[\x90\x82\x10\x15a\x08_W\x01\x90V[\x90\x81` \x91\x03\x12a\0\xE0WQ\x90V[\x90`\x01\x82\x01\x80\x92\x11a\x0CrWV[\x90`\x02\x82\x01\x80\x92\x11a\x0CrWV[\x90`\x03\x82\x01\x80\x92\x11a\x0CrWV[\x90`\x04\x82\x01\x80\x92\x11a\x0CrWV[\x90`\x05\x82\x01\x80\x92\x11a\x0CrWV[\x91\x90\x82\x01\x80\x92\x11a\x0CrWV[\x15a\r\x81WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90\x81` \x91\x03\x12a\0\xE0WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\0\xE0W\x90V[\x15a\x0E6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x90\x81` \x91\x03\x12a\0\xE0WQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\0\xE0W\x90V[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a\x0CrWV[\x15a\x0F\x04WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a\x0FoWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R_Q` a\" _9_Q\x90_R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a\x0F\xF1WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0F\xE4V[\x94\x93\x92\x90\x91\x93a\x10\x15a\n\x01V[Pa\x10!\x85\x15\x15a\n\x1AV[`@\x84\x01QQ\x85\x14\x80a\x19\x14W[\x80a\x19\x06W[\x80a\x18\xF8W[a\x10D\x90a\nyV[a\x10V` \x85\x01QQ\x85QQ\x14a\n\xE2V[a\x10mc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a\x0BMV[a\x10ua\x01KV[_\x81R_` \x82\x01R\x92a\x10\x87a\n\x01V[a\x10\x90\x87a\x0B\xACV[` \x82\x01Ra\x10\x9E\x87a\x0B\xACV[\x81Ra\x10\xA8a\n\x01V[\x92a\x10\xB7` \x88\x01QQa\x0B\xACV[\x84Ra\x10\xC7` \x88\x01QQa\x0B\xACV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03dWa\x110\x91_\x91a\x18\xC9W[Pa\x11+6\x8B\x87a\x0B\xF7V[a\x1D\xFBV[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a\x12\xBCW` \x88a\x11\xA1a\x11\x97\x8Ca\x11\x8F\x8F\x96\x86\x8Ea\x11ta\x11a\x86\x80\x95a\x0C<V[Q\x80Q_R` \x01Q` R`@_ \x90V[a\x11\x81\x84\x84\x84\x01Qa\x0C<V[R\x82a\x12\x89W[\x01Qa\x0C<V[Q\x95Qa\x0C<V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x03dWa\t+\x8Aa\x12N\x8Fa\x12G\x8F\x84` \x8F\x92a\x12>\x93a\x126\x84`\x01\x9Ea\x12T\x9E_\x91a\x12\\W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\x0C<V[R\x01Qa\x0C<V[Q\x93\x8DQa\x0C<V[Q\x16a\x1E\x82V[\x90a\x1E\xB3V[\x97\x01\x96a\x114V[a\x12|\x91P\x86=\x81\x11a\x12\x82W[a\x12t\x81\x83a\x01\x18V[\x81\x01\x90a\x0C\xD5V[_a\x12'V[P=a\x12jV[a\x12\xB7a\x12\x99\x84\x84\x84\x01Qa\x0C<V[Qa\x12\xB0\x84\x84\x01Qa\x12\xAA\x87a\x0CdV[\x90a\x0C<V[Q\x10a\x0CwV[a\x11\x88V[P\x90\x95\x97\x94\x96Pa\x12\xD1\x91\x98\x93\x92\x99Pa\x1F\x99V[\x91a\x12\xDD_T`\xFF\x16\x90V[\x90\x81\x15a\x18\xC1W`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03dW_\x91a\x18\x92W[P\x91\x90[_\x92[\x81\x84\x10a\x13\x9CWPPPPP\x92a\x13ua\x13pa\x13ia\x13\x96\x95\x85a\x13\x88\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x08xV[\x91\x90a\x0E\xFDV[a\x0FhV[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a\x0F\xC7V[\x03`\x1F\x19\x81\x01\x83R\x82a\x01\x18V[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da\x17|W[a\x11\x97\x82`\xA0a\x14\x05a\x13\xFFa\x13\xF1\x84a\x14\r\x97a\x13\xEBa\x13\xDDa\x11a\x8F\x9C`@` \x9F\x9E\x01Qa\x0C<V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba\r\x0CV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[\x97\x01Qa\x0C<V[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03dWa\x14\xD1a\x11\x97\x8F\x95\x8F\x90a\x14\xC9\x8F\x97\x8F\x96\x84\x8Fa\x14\xC3`\xC0\x96a\x14\xBC\x84\x8F` \x9F\x90a\x11\x88a\x13\xF1\x99`@\x93a\x13\xFF\x9C_\x91a\x17NW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a\x0E/V[Q\x90a\x19\xACV[\x9Ca\r\x0CV[\x96\x01Qa\x0C<V[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03dWa\x15^\x91\x8C\x8F\x92_\x92a\x17*W[P` a\x15P\x92\x93\x01Qa\x0C<V[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a\x15\x8B\x8Ca\x15P\x8Ca\x15\x84a\x15w\x82` \x86\x01Qa\x0C<V[Q`\x01`\x01``\x1B\x03\x16\x90V[\x92Qa\x0C<V[_\x98_[` \x8A\x01QQ\x81\x10\x15a\x17\x11W\x8B\x8Da\x15\xCD\x89a\x15\xC0a\x13\xFFa\x13\xF1\x86\x8F\x89a\x15\xB8\x91Qa\x0C<V[Q\x94\x87a\r\x0CV[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a\x15\xDCW[PP`\x01\x01a\x15\x8FV[\x8A\x8Aa\x16d\x85\x9F\x94\x8F\x96\x86a\x16\x1E\x8F\x93`\xE0a\x16\x15a\x11\x97\x95` a\x16\ra\x13\xFFa\x13\xF1\x83\x9Fa\x16$\x9C\x89\x91a\r\x0CV[\x9A\x01Qa\x0C<V[Q\x9B\x01Qa\x0C<V[Qa\x0C<V[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03dW\x8Fa\x16\xD0\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a\x16\xDBW[Pa\x16\xCAa\x15P\x92\x93Q\x93a\x16\xC5a\x15w\x84\x87a\x0C<V[a\x0E\xDDV[\x92a\x0C<V[\x01\x9A\x90P\x8B\x8Da\x15\xD2V[a\x15P\x92Pa\x17\x03a\x16\xCA\x91` =\x81\x11a\x17\nW[a\x16\xFB\x81\x83a\x01\x18V[\x81\x01\x90a\x0E\xBEV[\x92Pa\x16\xADV[P=a\x16\xF1V[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a\x138V[a\x15P\x92Pa\x17G` \x91\x82=\x81\x11a\x17\nWa\x16\xFB\x81\x83a\x01\x18V[\x92Pa\x15AV[` a\x17o\x92P=\x81\x11a\x17uW[a\x17g\x81\x83a\x01\x18V[\x81\x01\x90a\x0E\x0EV[_a\x14\xA6V[P=a\x17]V[a\x17\xB9\x94Pa\x17\x96\x92Pa\x13\xFF\x91a\x13\xF1\x91` \x95a\r\x0CV[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03dW` \x89a\x14\r\x8F\x93\x8F`\xA0\x8F\x97a\x13\xFFa\x13\xF1\x8F\x8F\x90a\x13\xEBa\x13\xDDa\x11a\x8F`@\x8B\x96\x91\x8F\x88\x93a\x11\x97\x9Fa\x18=\x90a\x18C\x93a\x14\x05\x9F_\x92a\x18YW[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a\rmV[\x11a\rzV[PPPPPP\x97PPPPPP\x92\x93PPa\x13\xB1V[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a\x18\x83\x91=\x81\x11a\x18\x8BW[a\x18{\x81\x83a\x01\x18V[\x81\x01\x90a\r\x18V[\x92\x91Pa\x18,V[P=a\x18qV[a\x18\xB4\x91P` =` \x11a\x18\xBAW[a\x18\xAC\x81\x83a\x01\x18V[\x81\x01\x90a\x0C\xF4V[_a\x131V[P=a\x18\xA2V[_\x91\x90a\x135V[a\x18\xEB\x91P` =` \x11a\x18\xF1W[a\x18\xE3\x81\x83a\x01\x18V[\x81\x01\x90a\x0B\xDEV[_a\x11\x1FV[P=a\x18\xD9V[P`\xE0\x84\x01QQ\x85\x14a\x10;V[P`\xC0\x84\x01QQ\x85\x14a\x105V[P`\xA0\x84\x01QQ\x85\x14a\x10/V[`@Q\x90a\x19/\x82a\0\xF8V[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80a\x19K\x81\x84a\x01\x18V[6\x837V[`@Q\x90a\x19_` \x83a\x01\x18V[` 6\x837V[\x91\x90`@\x90``a\x19ua\x19\"V[\x94\x85\x92` \x85Q\x92a\x19\x87\x85\x85a\x01\x18V[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15a\x19\xAAWV[\xFE[` \x92\x91`\x80`@\x92a\x19\xBDa\x19\"V[\x95\x86\x93\x81\x86Q\x93a\x19\xCE\x86\x86a\x01\x18V[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15a\x19\xAAW\x15a\x19\xFFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x90\xFD[`@Qa\x1A@\x81a\0\xF8V[`@\x90\x81Qa\x1AO\x83\x82a\x01\x18V[\x826\x827\x81R` \x82Q\x91a\x1Ad\x84\x84a\x01\x18V[\x836\x847\x01R\x80Qa\x1Av\x82\x82a\x01\x18V[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90a\x1A\xCC\x83\x83a\x01\x18V[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01Ra\x1B!\x83Q\x93\x84a\x01\x18V[\x82R` \x82\x01R\x90V[_Q` a\"\0_9_Q\x90_R\x90a\x1BBa\x19\"V[P_\x91\x90\x06` `\xC0\x83[a\x1CBW_\x93_Q` a\"\0_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa\x1Bx\x85\x82a\x01\x18V[\x846\x827\x84\x81\x85`@Qa\x1B\x8C\x82\x82a\x01\x18V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` a\"\0_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a\x19\xAAWa\x1B\xF6\x90a!\xB3V[Q\x91a\x1CBW_Q` a\"\0_9_Q\x90_R\x82\x80\t\x14a\x1C-WP_Q` a\"\0_9_Q\x90_R`\x01_\x94\x08\x92\x93a\x1BMV[\x92\x93PPa\x1C9a\x01KV[\x92\x83R\x82\x01R\x90V[a\x08dV[a\x1COa\x19\"V[P`@Qa\x1C\\\x81a\0\xF8V[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a\x0CrWV[\x90`\x0C\x81\x10\x15a\x08_W`\x05\x1B\x01\x90V[\x93\x92\x90\x91a\x1C\x9F`@a\x01ZV[\x94\x85R` \x85\x01Ra\x1C\xB1`@a\x01ZV[\x91\x82R` \x82\x01Ra\x1C\xC1a\x19:V[\x92_[`\x02\x81\x10a\x1C\xEEWPPP` a\x01\x80\x92a\x1C\xDDa\x19PV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80a\x1C\xFA`\x01\x92a\x1CjV[a\x1D\x04\x82\x85a\x08NV[QQa\x1D\x10\x82\x89a\x1C\x80V[R` a\x1D\x1D\x83\x86a\x08NV[Q\x01Qa\x1D2a\x1D,\x83a\r'V[\x89a\x1C\x80V[Ra\x1D=\x82\x86a\x08NV[QQQa\x1DLa\x1D,\x83a\r5V[Ra\x1Dba\x1DZ\x83\x87a\x08NV[QQ` \x01\x90V[Qa\x1Doa\x1D,\x83a\rCV[R` a\x1D|\x83\x87a\x08NV[Q\x01QQa\x1D\x8Ca\x1D,\x83a\rQV[Ra\x1D\xB8a\x1D\xB2a\x1D\xAB` a\x1D\xA2\x86\x8Aa\x08NV[Q\x01Q` \x01\x90V[Q\x92a\r_V[\x88a\x1C\x80V[R\x01a\x1C\xC4V[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19_T\x16`\xFF\x82\x16\x17_U`@Q\x90\x81R\xA1V[\x90`\x01a\x1E\t`\xFF\x93a \xBEV[\x92\x83\x92\x16\x1B\x11\x15a\x1E\x17W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x90\xFD[\x80_\x91[a\x1E\x8EWP\x90V[_\x19\x81\x01\x81\x81\x11a\x0CrWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x0CrW`\x01\x01\x90\x80a\x1E\x86V[\x90a\x1E\xBCa\x19\"V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15a\x1FaW`\x01\x82\x14a\x1F\\Wa\x1E\xDDa\x01KV[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15a\x1F\x02WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14a\x1F<W[`\x01a\x1F2a\x1F'\x83`\xFF\x94a\x19\xACV[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91a\x1E\xEEV[\x94`\x01a\x1F2a\x1F'a\x1FQ\x89`\xFF\x95a\x19\xACV[\x98\x93PPPPa\x1F\x16V[PP\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x1F\xA1a\x19\"V[P\x80Q\x90\x81\x15\x80a \x12W[\x15a\x1F\xCEWPP`@Qa\x1F\xC2`@\x82a\x01\x18V[_\x81R_` \x82\x01R\x90V[` _Q` a\"\0_9_Q\x90_R\x91\x01Q\x06_Q` a\"\0_9_Q\x90_R\x03_Q` a\"\0_9_Q\x90_R\x81\x11a\x0CrW`@Q\x91a\x1B!\x83a\0\xF8V[P` \x81\x01Q\x15a\x1F\xADV[\x80Q\x15a\x08_W` \x01\x90V[\x90\x81Q\x81\x10\x15a\x08_W\x01` \x01\x90V[\x15a CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x90a\x01\0\x82Q\x11a!<W\x81Q\x15a!7Wa \xFAa \xF0a\x13\xFFa \xE2\x85a \x1EV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xFF`\x01\x91\x16\x1B\x90V[`\x01\x90[\x83Q\x82\x10\x15a!2W`\x01\x90a!\x1Da \xF0a\x13\xFFa \xE2\x86\x89a +V[\x90a!)\x81\x83\x11a <V[\x17\x91\x01\x90a \xFEV[\x92PPV[_\x91PV[`\xA4`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R\xFD[\x15a!\xBAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \xA7\xAA\x0F\xA1\xDBp\xA8\xE3c\xF6\xA4~C\xB2\xC8\x88F`\xDF\x82\x04~\xAB\xF4\x95\xC8\x9E\xDF'}cedsolcC\0\x08\x1B\x003",
    );
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
    ///Container for all the [`BLSSignatureChecker`](self) function calls.
    pub enum BLSSignatureCheckerCalls {
        blsApkRegistry(blsApkRegistryCall),
        checkSignatures(checkSignaturesCall),
        delegation(delegationCall),
        registryCoordinator(registryCoordinatorCall),
        setStaleStakesForbidden(setStaleStakesForbiddenCall),
        stakeRegistry(stakeRegistryCall),
        staleStakesForbidden(staleStakesForbiddenCall),
        trySignatureAndApkVerification(trySignatureAndApkVerificationCall),
    }
    #[automatically_derived]
    impl BLSSignatureCheckerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [23u8, 31u8, 29u8, 91u8],
            [65u8, 108u8, 126u8, 94u8],
            [93u8, 244u8, 89u8, 70u8],
            [104u8, 48u8, 72u8, 53u8],
            [109u8, 20u8, 169u8, 135u8],
            [110u8, 251u8, 70u8, 54u8],
            [185u8, 141u8, 9u8, 8u8],
            [223u8, 92u8, 247u8, 35u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BLSSignatureCheckerCalls {
        const NAME: &'static str = "BLSSignatureCheckerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 8usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkSignatures(_) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setStaleStakesForbidden(_) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::staleStakesForbidden(_) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::trySignatureAndApkVerification(_) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<BLSSignatureCheckerCalls>] = &[
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSSignatureCheckerCalls> {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                BLSSignatureCheckerCalls::trySignatureAndApkVerification,
                            )
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn setStaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSSignatureCheckerCalls> {
                        <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BLSSignatureCheckerCalls::setStaleStakesForbidden)
                    }
                    setStaleStakesForbidden
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSSignatureCheckerCalls> {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BLSSignatureCheckerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSSignatureCheckerCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BLSSignatureCheckerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSSignatureCheckerCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BLSSignatureCheckerCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSSignatureCheckerCalls> {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BLSSignatureCheckerCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn staleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSSignatureCheckerCalls> {
                        <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BLSSignatureCheckerCalls::staleStakesForbidden)
                    }
                    staleStakesForbidden
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BLSSignatureCheckerCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BLSSignatureCheckerCalls::delegation)
                    }
                    delegation
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
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setStaleStakesForbidden(inner) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
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
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setStaleStakesForbidden(inner) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`BLSSignatureChecker`](self) events.
    pub enum BLSSignatureCheckerEvents {
        StaleStakesForbiddenUpdate(StaleStakesForbiddenUpdate),
    }
    #[automatically_derived]
    impl BLSSignatureCheckerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
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
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for BLSSignatureCheckerEvents {
        const NAME: &'static str = "BLSSignatureCheckerEvents";
        const COUNT: usize = 1usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
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
    impl alloy_sol_types::private::IntoLogData for BLSSignatureCheckerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BLSSignatureChecker`](self) contract instance.

See the [wrapper's documentation](`BLSSignatureCheckerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BLSSignatureCheckerInstance<T, P, N> {
        BLSSignatureCheckerInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<BLSSignatureCheckerInstance<T, P, N>>,
    > {
        BLSSignatureCheckerInstance::<T, P, N>::deploy(provider, _registryCoordinator)
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
        BLSSignatureCheckerInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _registryCoordinator)
    }
    /**A [`BLSSignatureChecker`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BLSSignatureChecker`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BLSSignatureCheckerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BLSSignatureCheckerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BLSSignatureCheckerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BLSSignatureCheckerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BLSSignatureChecker`](self) contract instance.

See the [wrapper's documentation](`BLSSignatureCheckerInstance`) for more details.*/
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
        ) -> alloy_contract::Result<BLSSignatureCheckerInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> BLSSignatureCheckerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BLSSignatureCheckerInstance<T, P, N> {
            BLSSignatureCheckerInstance {
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
    > BLSSignatureCheckerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
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
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BLSSignatureCheckerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`StaleStakesForbiddenUpdate`] event.
        pub fn StaleStakesForbiddenUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StaleStakesForbiddenUpdate, N> {
            self.event_filter::<StaleStakesForbiddenUpdate>()
        }
    }
}
