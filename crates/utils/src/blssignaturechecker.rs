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
    ///0x610100604052348015610010575f5ffd5b506040516140e43803806140e4833981810160405281019061003291906102bd565b8073ffffffffffffffffffffffffffffffffffffffff1660808173ffffffffffffffffffffffffffffffffffffffff16815250508073ffffffffffffffffffffffffffffffffffffffff1663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100d39190610323565b73ffffffffffffffffffffffffffffffffffffffff1660a08173ffffffffffffffffffffffffffffffffffffffff16815250508073ffffffffffffffffffffffffffffffffffffffff16635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa15801561014f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101739190610389565b73ffffffffffffffffffffffffffffffffffffffff1660c08173ffffffffffffffffffffffffffffffffffffffff168152505060a05173ffffffffffffffffffffffffffffffffffffffff1663df5cf7236040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101f1573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061021591906103ef565b73ffffffffffffffffffffffffffffffffffffffff1660e08173ffffffffffffffffffffffffffffffffffffffff16815250505061041a565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61027b82610252565b9050919050565b5f61028c82610271565b9050919050565b61029c81610282565b81146102a6575f5ffd5b50565b5f815190506102b781610293565b92915050565b5f602082840312156102d2576102d161024e565b5b5f6102df848285016102a9565b91505092915050565b5f6102f282610271565b9050919050565b610302816102e8565b811461030c575f5ffd5b50565b5f8151905061031d816102f9565b92915050565b5f602082840312156103385761033761024e565b5b5f6103458482850161030f565b91505092915050565b5f61035882610271565b9050919050565b6103688161034e565b8114610372575f5ffd5b50565b5f815190506103838161035f565b92915050565b5f6020828403121561039e5761039d61024e565b5b5f6103ab84828501610375565b91505092915050565b5f6103be82610271565b9050919050565b6103ce816103b4565b81146103d8575f5ffd5b50565b5f815190506103e9816103c5565b92915050565b5f602082840312156104045761040361024e565b5b5f610411848285016103db565b91505092915050565b60805160a05160c05160e051613c6661047e5f395f6111d101525f81816104090152610bf501525f818161042d01528181610d7e0152610f5801525f818161030301528181610451015281816107a1015281816109270152610af30152613c665ff3fe608060405234801561000f575f5ffd5b5060043610610086575f3560e01c80636d14a987116100595780636d14a987146101135780636efb463614610131578063b98d090814610162578063df5cf7231461018057610086565b8063171f1d5b1461008a578063416c7e5e146100bb5780635df45946146100d757806368304835146100f5575b5f5ffd5b6100a4600480360381019061009f9190612206565b61019e565b6040516100b2929190612285565b60405180910390f35b6100d560048036038101906100d091906122d6565b610301565b005b6100df610407565b6040516100ec919061237b565b60405180910390f35b6100fd61042b565b60405161010a91906123b4565b60405180910390f35b61011b61044f565b60405161012891906123ed565b60405180910390f35b61014b60048036038101906101469190612868565b610473565b604051610159929190612a26565b60405180910390f35b61016a6111be565b6040516101779190612a54565b60405180910390f35b6101886111cf565b6040516101959190612a8d565b60405180910390f35b5f5f5f7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187875f01518860200151885f01515f600281106101e2576101e1612aa6565b5b6020020151895f01516001600281106101fe576101fd612aa6565b5b60200201518a602001515f6002811061021a57610219612aa6565b5b60200201518b6020015160016002811061023757610236612aa6565b5b60200201518b5f01518c6020015160405160200161025d99989796959493929190612b13565b604051602081830303815290604052805190602001205f1c61027f9190612be2565b90506102ef6102a961029a83896111f390919063ffffffff16565b866112c790919063ffffffff16565b6102b16113c0565b6102e56102ce856102c061148a565b6111f390919063ffffffff16565b6102d78c6114ae565b6112c790919063ffffffff16565b886201d4c06115b9565b80935081945050505094509492505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561036a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061038e9190612c4d565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146103fb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016103f290612d1e565b60405180910390fd5b61040481611859565b50565b7f000000000000000000000000000000000000000000000000000000000000000081565b7f000000000000000000000000000000000000000000000000000000000000000081565b7f000000000000000000000000000000000000000000000000000000000000000081565b61047b611e70565b5f5f86869050036104c1576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104b890612dac565b60405180910390fd5b826040015151868690501480156104df57508260a001515186869050145b80156104f257508260c001515186869050145b801561050557508260e001515186869050145b610544576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161053b90612e60565b60405180910390fd5b825f0151518360200151511461058f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161058690612f14565b60405180910390fd5b4363ffffffff168463ffffffff16106105dd576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016105d490612fa2565b60405180910390fd5b5f60405180604001604052805f81526020015f81525090506105fd611e70565b8787905067ffffffffffffffff81111561061a57610619612007565b5b6040519080825280602002602001820160405280156106485781602001602082028036833780820191505090505b5081602001819052508787905067ffffffffffffffff81111561066e5761066d612007565b5b60405190808252806020026020018201604052801561069c5781602001602082028036833780820191505090505b50815f01819052506106ac611e8a565b85602001515167ffffffffffffffff8111156106cb576106ca612007565b5b6040519080825280602002602001820160405280156106f95781602001602082028036833780820191505090505b50815f018190525085602001515167ffffffffffffffff8111156107205761071f612007565b5b60405190808252806020026020018201604052801561074e5781602001602082028036833780820191505090505b5081602001819052505f6108318a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610808573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061082c9190612ff6565b6118ab565b90505f5f90505b876020015151811015610ab15761086c8860200151828151811061085f5761085e612aa6565b5b602002602001015161190b565b8360200151828151811061088357610882612aa6565b5b6020026020010181815250505f81146109255782602001516001826108a8919061304e565b815181106108b9576108b8612aa6565b5b60200260200101515f1c836020015182815181106108da576108d9612aa6565b5b60200260200101515f1c11610924576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161091b906130f1565b60405180910390fd5b5b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166304ec63518460200151838151811061097857610977612aa6565b5b60200260200101518b8b5f0151858151811061099757610996612aa6565b5b60200260200101516040518463ffffffff1660e01b81526004016109bd9392919061314e565b602060405180830381865afa1580156109d8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109fc91906131d0565b77ffffffffffffffffffffffffffffffffffffffffffffffff16835f01518281518110610a2c57610a2b612aa6565b5b602002602001018181525050610aa2610a93610a6684865f01518581518110610a5857610a57612aa6565b5b602002602001015116611923565b8a602001518481518110610a7d57610a7c612aa6565b5b602002602001015161195e90919063ffffffff16565b866112c790919063ffffffff16565b94508080600101915050610838565b5050610abc83611a4c565b92505f5f5f9054906101000a900460ff1690505f5f90505f5f90505b8b8b90508110156110db578215610bf3578963ffffffff16827f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663249a0c428f8f86818110610b4057610b3f612aa6565b5b9050013560f81c60f81b60f81c6040518263ffffffff1660e01b8152600401610b69919061320a565b602060405180830381865afa158015610b84573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ba89190613237565b610bb29190613262565b11610bf2576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610be990613351565b60405180910390fd5b5b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166368bccaac8d8d84818110610c4257610c41612aa6565b5b9050013560f81c60f81b60f81c8c8c60a001518581518110610c6757610c66612aa6565b5b60200260200101516040518463ffffffff1660e01b8152600401610c8d9392919061336f565b602060405180830381865afa158015610ca8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ccc91906133f9565b67ffffffffffffffff1916610cfe8a604001518381518110610cf157610cf0612aa6565b5b602002602001015161190b565b67ffffffffffffffff191614610d49576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d40906134e0565b60405180910390fd5b610d7a89604001518281518110610d6357610d62612aa6565b5b6020026020010151876112c790919063ffffffff16565b95507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663c8294c568d8d84818110610dcb57610dca612aa6565b5b9050013560f81c60f81b60f81c8c8c60c001518581518110610df057610def612aa6565b5b60200260200101516040518463ffffffff1660e01b8152600401610e169392919061336f565b602060405180830381865afa158015610e31573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e559190613528565b85602001518281518110610e6c57610e6b612aa6565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff168152505084602001518181518110610ead57610eac612aa6565b5b6020026020010151855f01518281518110610ecb57610eca612aa6565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff16815250505f5f90505f5f90505b8a60200151518110156110cc57610f51865f01518281518110610f2457610f23612aa6565b5b60200260200101518f8f86818110610f3f57610f3e612aa6565b5b9050013560f81c60f81b60f81c611b04565b156110bf577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f2be94ae8f8f86818110610fa557610fa4612aa6565b5b9050013560f81c60f81b60f81c8e89602001518581518110610fca57610fc9612aa6565b5b60200260200101518f60e001518881518110610fe957610fe8612aa6565b5b6020026020010151878151811061100357611002612aa6565b5b60200260200101516040518563ffffffff1660e01b815260040161102a9493929190613553565b602060405180830381865afa158015611045573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110699190613528565b875f0151848151811061107f5761107e612aa6565b5b602002602001018181516110939190613596565b9150906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff16815250508160010191505b8080600101915050610efe565b50508080600101915050610ad8565b5050505f5f6110f48c868a606001518b6080015161019e565b9150915081611138576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161112f9061366b565b60405180910390fd5b80611178576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161116f906136f9565b60405180910390fd5b50505f8782602001516040516020016111929291906137fc565b604051602081830303815290604052805190602001209050828195509550505050509550959350505050565b5f5f9054906101000a900460ff1681565b7f000000000000000000000000000000000000000000000000000000000000000081565b6111fb611ea4565b611203611ebc565b835f0151815f6003811061121a57611219612aa6565b5b60200201818152505083602001518160016003811061123c5761123b612aa6565b5b602002018181525050828160026003811061125a57611259612aa6565b5b6020020181815250505f60408360608460076107d05a03fa9050805f810361127e57fe5b50806112bf576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016112b69061386d565b60405180910390fd5b505092915050565b6112cf611ea4565b6112d7611ede565b835f0151815f600481106112ee576112ed612aa6565b5b6020020181815250508360200151816001600481106113105761130f612aa6565b5b602002018181525050825f01518160026004811061133157611330612aa6565b5b60200201818152505082602001518160036004811061135357611352612aa6565b5b6020020181815250505f60408360808460066107d05a03fa9050805f810361137757fe5b50806113b8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016113af906138d5565b60405180910390fd5b505092915050565b6113c8611f00565b604051806040016040528060405180604001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed815250815260200160405180604001604052807f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec81526020017f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d815250815250905090565b611492611ea4565b6040518060400160405280600181526020016002815250905090565b6114b6611ea4565b5f5f90505f5f90505f7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47855f1c6114ed9190612be2565b90505b6001156115995761150081611b1a565b80935081945050507f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478061153757611536612bb5565b5b828309830361155f5760405180604001604052808281526020018381525093505050506115b4565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478061158e5761158d612bb5565b5b6001820890506114f0565b60405180604001604052805f81526020015f81525093505050505b919050565b5f5f5f60405180604001604052808981526020018781525090505f60405180604001604052808981526020018781525090506115f3611f26565b5f5f90505b6002811015611811575f60068261160f91906138f3565b905084826002811061162457611623612aa6565b5b60200201515f0151835f836116399190613262565b600c811061164a57611649612aa6565b5b60200201818152505084826002811061166657611665612aa6565b5b6020020151602001518360018361167d9190613262565b600c811061168e5761168d612aa6565b5b6020020181815250508382600281106116aa576116a9612aa6565b5b60200201515f01515f600281106116c4576116c3612aa6565b5b6020020151836002836116d79190613262565b600c81106116e8576116e7612aa6565b5b60200201818152505083826002811061170457611703612aa6565b5b60200201515f015160016002811061171f5761171e612aa6565b5b6020020151836003836117329190613262565b600c811061174357611742612aa6565b5b60200201818152505083826002811061175f5761175e612aa6565b5b6020020151602001515f6002811061177a57611779612aa6565b5b60200201518360048361178d9190613262565b600c811061179e5761179d612aa6565b5b6020020181815250508382600281106117ba576117b9612aa6565b5b6020020151602001516001600281106117d6576117d5612aa6565b5b6020020151836005836117e99190613262565b600c81106117fa576117f9612aa6565b5b6020020181815250505080806001019150506115f8565b5061181a611f49565b5f6020826020600c028560088cfa9050805f835f6001811061183f5761183e612aa6565b5b602002015114159650965050505050509550959350505050565b805f5f6101000a81548160ff0219169083151502179055507f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc816040516118a09190612a54565b60405180910390a150565b5f5f6118b684611c0f565b9050808360ff166001901b11611901576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016118f8906139a4565b60405180910390fd5b8091505092915050565b5f81515f52816020015160205260405f209050919050565b5f5f5f90505b5f8311156119555760018361193e919061304e565b83169250808061194d906139cf565b915050611929565b80915050919050565b611966611ea4565b6102008261ffff16106119ae576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016119a590613a42565b60405180910390fd5b60018261ffff16036119c257829050611a46565b5f60405180604001604052805f81526020015f81525090505f8490505f600190505f5f90505b8161ffff168661ffff1610611a3e576001808260ff168861ffff16901c1661ffff1603611a1c57611a1984846112c7565b93505b611a2683846112c7565b925060018261ffff16901b91508060010190506119e8565b839450505050505b92915050565b611a54611ea4565b5f825f0151148015611a6957505f8260200151145b15611a8a5760405180604001604052805f81526020015f8152509050611aff565b6040518060400160405280835f015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478460200151611ace9190612be2565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611af9919061304e565b81525090505b919050565b5f60018260ff1684901c16600114905092915050565b5f5f5f7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611b4c57611b4b612bb5565b5b60037f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611b7d57611b7c612bb5565b5b867f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611bad57611bac612bb5565b5b888909090890505f611c00827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f527f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611d29565b90508181935093505050915091565b5f61010082511115611c56576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611c4d90613af6565b60405180910390fd5b5f825103611c66575f9050611d24565b5f5f835f81518110611c7b57611c7a612aa6565b5b602001015160f81c60f81b60f81c60ff166001901b91505f600190505b8451811015611d1d57848181518110611cb457611cb3612aa6565b5b602001015160f81c60f81b60f81c60ff166001901b9150828211611d0d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611d0490613baa565b60405180910390fd5b8183179250806001019050611c98565b5081925050505b919050565b5f5f611d33611f49565b611d3b611f6b565b6020815f60068110611d5057611d4f612aa6565b5b602002018181525050602081600160068110611d6f57611d6e612aa6565b5b602002018181525050602081600260068110611d8e57611d8d612aa6565b5b6020020181815250508681600360068110611dac57611dab612aa6565b5b6020020181815250508581600460068110611dca57611dc9612aa6565b5b6020020181815250508481600560068110611de857611de7612aa6565b5b60200201818152505060208260c08360056107d05a03fa9250825f8103611e0b57fe5b5082611e4c576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611e4390613c12565b60405180910390fd5b815f60018110611e5f57611e5e612aa6565b5b602002015193505050509392505050565b604051806040016040528060608152602001606081525090565b604051806040016040528060608152602001606081525090565b60405180604001604052805f81526020015f81525090565b6040518060600160405280600390602082028036833780820191505090505090565b6040518060800160405280600490602082028036833780820191505090505090565b6040518060400160405280611f13611f8d565b8152602001611f20611f8d565b81525090565b604051806101800160405280600c90602082028036833780820191505090505090565b6040518060200160405280600190602082028036833780820191505090505090565b6040518060c00160405280600690602082028036833780820191505090505090565b6040518060400160405280600290602082028036833780820191505090505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b611fd281611fc0565b8114611fdc575f5ffd5b50565b5f81359050611fed81611fc9565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61203d82611ff7565b810181811067ffffffffffffffff8211171561205c5761205b612007565b5b80604052505050565b5f61206e611faf565b905061207a8282612034565b919050565b5f5ffd5b5f819050919050565b61209581612083565b811461209f575f5ffd5b50565b5f813590506120b08161208c565b92915050565b5f604082840312156120cb576120ca611ff3565b5b6120d56040612065565b90505f6120e4848285016120a2565b5f8301525060206120f7848285016120a2565b60208301525092915050565b5f5ffd5b5f67ffffffffffffffff82111561212157612120612007565b5b602082029050919050565b5f5ffd5b5f61214261213d84612107565b612065565b9050806020840283018581111561215c5761215b61212c565b5b835b81811015612185578061217188826120a2565b84526020840193505060208101905061215e565b5050509392505050565b5f82601f8301126121a3576121a2612103565b5b60026121b0848285612130565b91505092915050565b5f608082840312156121ce576121cd611ff3565b5b6121d86040612065565b90505f6121e78482850161218f565b5f8301525060406121fa8482850161218f565b60208301525092915050565b5f5f5f5f610120858703121561221f5761221e611fb8565b5b5f61222c87828801611fdf565b945050602061223d878288016120b6565b935050606061224e878288016121b9565b92505060e061225f878288016120b6565b91505092959194509250565b5f8115159050919050565b61227f8161226b565b82525050565b5f6040820190506122985f830185612276565b6122a56020830184612276565b9392505050565b6122b58161226b565b81146122bf575f5ffd5b50565b5f813590506122d0816122ac565b92915050565b5f602082840312156122eb576122ea611fb8565b5b5f6122f8848285016122c2565b91505092915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f819050919050565b5f61234361233e61233984612301565b612320565b612301565b9050919050565b5f61235482612329565b9050919050565b5f6123658261234a565b9050919050565b6123758161235b565b82525050565b5f60208201905061238e5f83018461236c565b92915050565b5f61239e8261234a565b9050919050565b6123ae81612394565b82525050565b5f6020820190506123c75f8301846123a5565b92915050565b5f6123d78261234a565b9050919050565b6123e7816123cd565b82525050565b5f6020820190506124005f8301846123de565b92915050565b5f5ffd5b5f5f83601f84011261241f5761241e612103565b5b8235905067ffffffffffffffff81111561243c5761243b612406565b5b6020830191508360018202830111156124585761245761212c565b5b9250929050565b5f63ffffffff82169050919050565b6124778161245f565b8114612481575f5ffd5b50565b5f813590506124928161246e565b92915050565b5f67ffffffffffffffff8211156124b2576124b1612007565b5b602082029050602081019050919050565b5f6124d56124d084612498565b612065565b905080838252602082019050602084028301858111156124f8576124f761212c565b5b835b81811015612521578061250d8882612484565b8452602084019350506020810190506124fa565b5050509392505050565b5f82601f83011261253f5761253e612103565b5b813561254f8482602086016124c3565b91505092915050565b5f67ffffffffffffffff82111561257257612571612007565b5b602082029050602081019050919050565b5f61259561259084612558565b612065565b905080838252602082019050604084028301858111156125b8576125b761212c565b5b835b818110156125e157806125cd88826120b6565b8452602084019350506040810190506125ba565b5050509392505050565b5f82601f8301126125ff576125fe612103565b5b813561260f848260208601612583565b91505092915050565b5f67ffffffffffffffff82111561263257612631612007565b5b602082029050602081019050919050565b5f61265561265084612618565b612065565b905080838252602082019050602084028301858111156126785761267761212c565b5b835b818110156126bf57803567ffffffffffffffff81111561269d5761269c612103565b5b8086016126aa898261252b565b8552602085019450505060208101905061267a565b5050509392505050565b5f82601f8301126126dd576126dc612103565b5b81356126ed848260208601612643565b91505092915050565b5f610180828403121561270c5761270b611ff3565b5b612717610100612065565b90505f82013567ffffffffffffffff8111156127365761273561207f565b5b6127428482850161252b565b5f83015250602082013567ffffffffffffffff8111156127655761276461207f565b5b612771848285016125eb565b602083015250604082013567ffffffffffffffff8111156127955761279461207f565b5b6127a1848285016125eb565b60408301525060606127b5848285016121b9565b60608301525060e06127c9848285016120b6565b60808301525061012082013567ffffffffffffffff8111156127ee576127ed61207f565b5b6127fa8482850161252b565b60a08301525061014082013567ffffffffffffffff81111561281f5761281e61207f565b5b61282b8482850161252b565b60c08301525061016082013567ffffffffffffffff8111156128505761284f61207f565b5b61285c848285016126c9565b60e08301525092915050565b5f5f5f5f5f6080868803121561288157612880611fb8565b5b5f61288e88828901611fdf565b955050602086013567ffffffffffffffff8111156128af576128ae611fbc565b5b6128bb8882890161240a565b945094505060406128ce88828901612484565b925050606086013567ffffffffffffffff8111156128ef576128ee611fbc565b5b6128fb888289016126f6565b9150509295509295909350565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6bffffffffffffffffffffffff82169050919050565b61295181612931565b82525050565b5f6129628383612948565b60208301905092915050565b5f602082019050919050565b5f61298482612908565b61298e8185612912565b935061299983612922565b805f5b838110156129c95781516129b08882612957565b97506129bb8361296e565b92505060018101905061299c565b5085935050505092915050565b5f604083015f8301518482035f8601526129f0828261297a565b91505060208301518482036020860152612a0a828261297a565b9150508091505092915050565b612a2081611fc0565b82525050565b5f6040820190508181035f830152612a3e81856129d6565b9050612a4d6020830184612a17565b9392505050565b5f602082019050612a675f830184612276565b92915050565b5f612a778261234a565b9050919050565b612a8781612a6d565b82525050565b5f602082019050612aa05f830184612a7e565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b612aed612ae882611fc0565b612ad3565b82525050565b5f819050919050565b612b0d612b0882612083565b612af3565b82525050565b5f612b1e828c612adc565b602082019150612b2e828b612afc565b602082019150612b3e828a612afc565b602082019150612b4e8289612afc565b602082019150612b5e8288612afc565b602082019150612b6e8287612afc565b602082019150612b7e8286612afc565b602082019150612b8e8285612afc565b602082019150612b9e8284612afc565b6020820191508190509a9950505050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f612bec82612083565b9150612bf783612083565b925082612c0757612c06612bb5565b5b828206905092915050565b5f612c1c82612301565b9050919050565b612c2c81612c12565b8114612c36575f5ffd5b50565b5f81519050612c4781612c23565b92915050565b5f60208284031215612c6257612c61611fb8565b5b5f612c6f84828501612c39565b91505092915050565b5f82825260208201905092915050565b7f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e615f8201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560208201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000604082015250565b5f612d08605c83612c78565b9150612d1382612c88565b606082019050919050565b5f6020820190508181035f830152612d3581612cfc565b9050919050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a20656d7074792071756f72756d20696e707574000000000000000000602082015250565b5f612d96603783612c78565b9150612da182612d3c565b604082019050919050565b5f6020820190508181035f830152612dc381612d8a565b9050919050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d61746360208201527f6800000000000000000000000000000000000000000000000000000000000000604082015250565b5f612e4a604183612c78565b9150612e5582612dca565b606082019050919050565b5f6020820190508181035f830152612e7781612e3e565b9050919050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d60208201527f6174636800000000000000000000000000000000000000000000000000000000604082015250565b5f612efe604483612c78565b9150612f0982612e7e565b606082019050919050565b5f6020820190508181035f830152612f2b81612ef2565b9050919050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a20696e76616c6964207265666572656e636520626c6f636b00000000602082015250565b5f612f8c603c83612c78565b9150612f9782612f32565b604082019050919050565b5f6020820190508181035f830152612fb981612f80565b9050919050565b5f60ff82169050919050565b612fd581612fc0565b8114612fdf575f5ffd5b50565b5f81519050612ff081612fcc565b92915050565b5f6020828403121561300b5761300a611fb8565b5b5f61301884828501612fe2565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61305882612083565b915061306383612083565b925082820390508181111561307b5761307a613021565b5b92915050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f72746564602082015250565b5f6130db604083612c78565b91506130e682613081565b604082019050919050565b5f6020820190508181035f830152613108816130cf565b9050919050565b6131188161245f565b82525050565b5f61313861313361312e8461245f565b612320565b612083565b9050919050565b6131488161311e565b82525050565b5f6060820190506131615f830186612a17565b61316e602083018561310f565b61317b604083018461313f565b949350505050565b5f77ffffffffffffffffffffffffffffffffffffffffffffffff82169050919050565b6131af81613183565b81146131b9575f5ffd5b50565b5f815190506131ca816131a6565b92915050565b5f602082840312156131e5576131e4611fb8565b5b5f6131f2848285016131bc565b91505092915050565b61320481612fc0565b82525050565b5f60208201905061321d5f8301846131fb565b92915050565b5f815190506132318161208c565b92915050565b5f6020828403121561324c5761324b611fb8565b5b5f61325984828501613223565b91505092915050565b5f61326c82612083565b915061327783612083565b925082820190508082111561328f5761328e613021565b5b92915050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a205374616b6552656769737472792075706461746573206d7573742060208201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060408201527f77696e646f770000000000000000000000000000000000000000000000000000606082015250565b5f61333b606683612c78565b915061334682613295565b608082019050919050565b5f6020820190508181035f8301526133688161332f565b9050919050565b5f6060820190506133825f8301866131fb565b61338f602083018561310f565b61339c604083018461313f565b949350505050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000082169050919050565b6133d8816133a4565b81146133e2575f5ffd5b50565b5f815190506133f3816133cf565b92915050565b5f6020828403121561340e5761340d611fb8565b5b5f61341b848285016133e5565b91505092915050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460208201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d20617060408201527f6b00000000000000000000000000000000000000000000000000000000000000606082015250565b5f6134ca606183612c78565b91506134d582613424565b608082019050919050565b5f6020820190508181035f8301526134f7816134be565b9050919050565b61350781612931565b8114613511575f5ffd5b50565b5f81519050613522816134fe565b92915050565b5f6020828403121561353d5761353c611fb8565b5b5f61354a84828501613514565b91505092915050565b5f6080820190506135665f8301876131fb565b613573602083018661310f565b6135806040830185612a17565b61358d606083018461313f565b95945050505050565b5f6135a082612931565b91506135ab83612931565b925082820390506bffffffffffffffffffffffff8111156135cf576135ce613021565b5b92915050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c2066616960208201527f6c65640000000000000000000000000000000000000000000000000000000000604082015250565b5f613655604383612c78565b9150613660826135d5565b606082019050919050565b5f6020820190508181035f83015261368281613649565b9050919050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a207369676e617475726520697320696e76616c696400000000000000602082015250565b5f6136e3603983612c78565b91506136ee82613689565b604082019050919050565b5f6020820190508181035f830152613710816136d7565b9050919050565b5f8160e01b9050919050565b5f61372d82613717565b9050919050565b6137456137408261245f565b613723565b82525050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b61377781611fc0565b82525050565b5f613788838361376e565b60208301905092915050565b5f602082019050919050565b5f6137aa8261374b565b6137b48185613755565b93506137bf8361375f565b805f5b838110156137ef5781516137d6888261377d565b97506137e183613794565b9250506001810190506137c2565b5085935050505092915050565b5f6138078285613734565b60048201915061381782846137a0565b91508190509392505050565b7f65632d6d756c2d6661696c6564000000000000000000000000000000000000005f82015250565b5f613857600d83612c78565b915061386282613823565b602082019050919050565b5f6020820190508181035f8301526138848161384b565b9050919050565b7f65632d6164642d6661696c6564000000000000000000000000000000000000005f82015250565b5f6138bf600d83612c78565b91506138ca8261388b565b602082019050919050565b5f6020820190508181035f8301526138ec816138b3565b9050919050565b5f6138fd82612083565b915061390883612083565b925082820261391681612083565b9150828204841483151761392d5761392c613021565b5b5092915050565b7f4269746d61705574696c732e6f72646572656442797465734172726179546f425f8201527f69746d61703a206269746d61702065786365656473206d61782076616c756500602082015250565b5f61398e603f83612c78565b915061399982613934565b604082019050919050565b5f6020820190508181035f8301526139bb81613982565b9050919050565b5f61ffff82169050919050565b5f6139d9826139c2565b915061ffff82036139ed576139ec613021565b5b600182019050919050565b7f7363616c61722d746f6f2d6c61726765000000000000000000000000000000005f82015250565b5f613a2c601083612c78565b9150613a37826139f8565b602082019050919050565b5f6020820190508181035f830152613a5981613a20565b9050919050565b7f4269746d61705574696c732e6f72646572656442797465734172726179546f425f8201527f69746d61703a206f7264657265644279746573417272617920697320746f6f2060208201527f6c6f6e6700000000000000000000000000000000000000000000000000000000604082015250565b5f613ae0604483612c78565b9150613aeb82613a60565b606082019050919050565b5f6020820190508181035f830152613b0d81613ad4565b9050919050565b7f4269746d61705574696c732e6f72646572656442797465734172726179546f425f8201527f69746d61703a206f72646572656442797465734172726179206973206e6f742060208201527f6f72646572656400000000000000000000000000000000000000000000000000604082015250565b5f613b94604783612c78565b9150613b9f82613b14565b606082019050919050565b5f6020820190508181035f830152613bc181613b88565b9050919050565b7f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000005f82015250565b5f613bfc601a83612c78565b9150613c0782613bc8565b602082019050919050565b5f6020820190508181035f830152613c2981613bf0565b905091905056fea2646970667358221220da57750388419c15a96c4365ae9aa5daa1f25f5bcde1cad99fb33e93239e026864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@Qa@\xE48\x03\x80a@\xE4\x839\x81\x81\x01`@R\x81\x01\x90a\x002\x91\x90a\x02\xBDV[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD3\x91\x90a\x03#V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01s\x91\x90a\x03\x89V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\xA0Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDF\\\xF7#`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x15\x91\x90a\x03\xEFV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPa\x04\x1AV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x02{\x82a\x02RV[\x90P\x91\x90PV[_a\x02\x8C\x82a\x02qV[\x90P\x91\x90PV[a\x02\x9C\x81a\x02\x82V[\x81\x14a\x02\xA6W__\xFD[PV[_\x81Q\x90Pa\x02\xB7\x81a\x02\x93V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x02\xD2Wa\x02\xD1a\x02NV[[_a\x02\xDF\x84\x82\x85\x01a\x02\xA9V[\x91PP\x92\x91PPV[_a\x02\xF2\x82a\x02qV[\x90P\x91\x90PV[a\x03\x02\x81a\x02\xE8V[\x81\x14a\x03\x0CW__\xFD[PV[_\x81Q\x90Pa\x03\x1D\x81a\x02\xF9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x038Wa\x037a\x02NV[[_a\x03E\x84\x82\x85\x01a\x03\x0FV[\x91PP\x92\x91PPV[_a\x03X\x82a\x02qV[\x90P\x91\x90PV[a\x03h\x81a\x03NV[\x81\x14a\x03rW__\xFD[PV[_\x81Q\x90Pa\x03\x83\x81a\x03_V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x03\x9EWa\x03\x9Da\x02NV[[_a\x03\xAB\x84\x82\x85\x01a\x03uV[\x91PP\x92\x91PPV[_a\x03\xBE\x82a\x02qV[\x90P\x91\x90PV[a\x03\xCE\x81a\x03\xB4V[\x81\x14a\x03\xD8W__\xFD[PV[_\x81Q\x90Pa\x03\xE9\x81a\x03\xC5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x04\x04Wa\x04\x03a\x02NV[[_a\x04\x11\x84\x82\x85\x01a\x03\xDBV[\x91PP\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa<fa\x04~_9_a\x11\xD1\x01R_\x81\x81a\x04\t\x01Ra\x0B\xF5\x01R_\x81\x81a\x04-\x01R\x81\x81a\r~\x01Ra\x0FX\x01R_\x81\x81a\x03\x03\x01R\x81\x81a\x04Q\x01R\x81\x81a\x07\xA1\x01R\x81\x81a\t'\x01Ra\n\xF3\x01Ra<f_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x86W_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0YW\x80cm\x14\xA9\x87\x14a\x01\x13W\x80cn\xFBF6\x14a\x011W\x80c\xB9\x8D\t\x08\x14a\x01bW\x80c\xDF\\\xF7#\x14a\x01\x80Wa\0\x86V[\x80c\x17\x1F\x1D[\x14a\0\x8AW\x80cAl~^\x14a\0\xBBW\x80c]\xF4YF\x14a\0\xD7W\x80ch0H5\x14a\0\xF5W[__\xFD[a\0\xA4`\x04\x806\x03\x81\x01\x90a\0\x9F\x91\x90a\"\x06V[a\x01\x9EV[`@Qa\0\xB2\x92\x91\x90a\"\x85V[`@Q\x80\x91\x03\x90\xF3[a\0\xD5`\x04\x806\x03\x81\x01\x90a\0\xD0\x91\x90a\"\xD6V[a\x03\x01V[\0[a\0\xDFa\x04\x07V[`@Qa\0\xEC\x91\x90a#{V[`@Q\x80\x91\x03\x90\xF3[a\0\xFDa\x04+V[`@Qa\x01\n\x91\x90a#\xB4V[`@Q\x80\x91\x03\x90\xF3[a\x01\x1Ba\x04OV[`@Qa\x01(\x91\x90a#\xEDV[`@Q\x80\x91\x03\x90\xF3[a\x01K`\x04\x806\x03\x81\x01\x90a\x01F\x91\x90a(hV[a\x04sV[`@Qa\x01Y\x92\x91\x90a*&V[`@Q\x80\x91\x03\x90\xF3[a\x01ja\x11\xBEV[`@Qa\x01w\x91\x90a*TV[`@Q\x80\x91\x03\x90\xF3[a\x01\x88a\x11\xCFV[`@Qa\x01\x95\x91\x90a*\x8DV[`@Q\x80\x91\x03\x90\xF3[___\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87_\x01Q\x88` \x01Q\x88_\x01Q_`\x02\x81\x10a\x01\xE2Wa\x01\xE1a*\xA6V[[` \x02\x01Q\x89_\x01Q`\x01`\x02\x81\x10a\x01\xFEWa\x01\xFDa*\xA6V[[` \x02\x01Q\x8A` \x01Q_`\x02\x81\x10a\x02\x1AWa\x02\x19a*\xA6V[[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x027Wa\x026a*\xA6V[[` \x02\x01Q\x8B_\x01Q\x8C` \x01Q`@Q` \x01a\x02]\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a+\x13V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x02\x7F\x91\x90a+\xE2V[\x90Pa\x02\xEFa\x02\xA9a\x02\x9A\x83\x89a\x11\xF3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x12\xC7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02\xB1a\x13\xC0V[a\x02\xE5a\x02\xCE\x85a\x02\xC0a\x14\x8AV[a\x11\xF3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02\xD7\x8Ca\x14\xAEV[a\x12\xC7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88b\x01\xD4\xC0a\x15\xB9V[\x80\x93P\x81\x94PPPP\x94P\x94\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03jW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x8E\x91\x90a,MV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x03\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\xF2\x90a-\x1EV[`@Q\x80\x91\x03\x90\xFD[a\x04\x04\x81a\x18YV[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04{a\x1EpV[__\x86\x86\x90P\x03a\x04\xC1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xB8\x90a-\xACV[`@Q\x80\x91\x03\x90\xFD[\x82`@\x01QQ\x86\x86\x90P\x14\x80\x15a\x04\xDFWP\x82`\xA0\x01QQ\x86\x86\x90P\x14[\x80\x15a\x04\xF2WP\x82`\xC0\x01QQ\x86\x86\x90P\x14[\x80\x15a\x05\x05WP\x82`\xE0\x01QQ\x86\x86\x90P\x14[a\x05DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05;\x90a.`V[`@Q\x80\x91\x03\x90\xFD[\x82_\x01QQ\x83` \x01QQ\x14a\x05\x8FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\x86\x90a/\x14V[`@Q\x80\x91\x03\x90\xFD[Cc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10a\x05\xDDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xD4\x90a/\xA2V[`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x05\xFDa\x1EpV[\x87\x87\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x1AWa\x06\x19a \x07V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06HW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81` \x01\x81\x90RP\x87\x87\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06nWa\x06ma \x07V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\x9CW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81_\x01\x81\x90RPa\x06\xACa\x1E\x8AV[\x85` \x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xCBWa\x06\xCAa \x07V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xF9W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81_\x01\x81\x90RP\x85` \x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07 Wa\x07\x1Fa \x07V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07NW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81` \x01\x81\x90RP_a\x081\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08,\x91\x90a/\xF6V[a\x18\xABV[\x90P__\x90P[\x87` \x01QQ\x81\x10\x15a\n\xB1Wa\x08l\x88` \x01Q\x82\x81Q\x81\x10a\x08_Wa\x08^a*\xA6V[[` \x02` \x01\x01Qa\x19\x0BV[\x83` \x01Q\x82\x81Q\x81\x10a\x08\x83Wa\x08\x82a*\xA6V[[` \x02` \x01\x01\x81\x81RPP_\x81\x14a\t%W\x82` \x01Q`\x01\x82a\x08\xA8\x91\x90a0NV[\x81Q\x81\x10a\x08\xB9Wa\x08\xB8a*\xA6V[[` \x02` \x01\x01Q_\x1C\x83` \x01Q\x82\x81Q\x81\x10a\x08\xDAWa\x08\xD9a*\xA6V[[` \x02` \x01\x01Q_\x1C\x11a\t$W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x1B\x90a0\xF1V[`@Q\x80\x91\x03\x90\xFD[[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x04\xECcQ\x84` \x01Q\x83\x81Q\x81\x10a\txWa\twa*\xA6V[[` \x02` \x01\x01Q\x8B\x8B_\x01Q\x85\x81Q\x81\x10a\t\x97Wa\t\x96a*\xA6V[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xBD\x93\x92\x91\x90a1NV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xFC\x91\x90a1\xD0V[w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83_\x01Q\x82\x81Q\x81\x10a\n,Wa\n+a*\xA6V[[` \x02` \x01\x01\x81\x81RPPa\n\xA2a\n\x93a\nf\x84\x86_\x01Q\x85\x81Q\x81\x10a\nXWa\nWa*\xA6V[[` \x02` \x01\x01Q\x16a\x19#V[\x8A` \x01Q\x84\x81Q\x81\x10a\n}Wa\n|a*\xA6V[[` \x02` \x01\x01Qa\x19^\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x12\xC7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P\x80\x80`\x01\x01\x91PPa\x088V[PPa\n\xBC\x83a\x1ALV[\x92P___\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P__\x90P__\x90P[\x8B\x8B\x90P\x81\x10\x15a\x10\xDBW\x82\x15a\x0B\xF3W\x89c\xFF\xFF\xFF\xFF\x16\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c$\x9A\x0CB\x8F\x8F\x86\x81\x81\x10a\x0B@Wa\x0B?a*\xA6V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Bi\x91\x90a2\nV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xA8\x91\x90a27V[a\x0B\xB2\x91\x90a2bV[\x11a\x0B\xF2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xE9\x90a3QV[`@Q\x80\x91\x03\x90\xFD[[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xBC\xCA\xAC\x8D\x8D\x84\x81\x81\x10a\x0CBWa\x0CAa*\xA6V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xA0\x01Q\x85\x81Q\x81\x10a\x0CgWa\x0Cfa*\xA6V[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x8D\x93\x92\x91\x90a3oV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xCC\x91\x90a3\xF9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16a\x0C\xFE\x8A`@\x01Q\x83\x81Q\x81\x10a\x0C\xF1Wa\x0C\xF0a*\xA6V[[` \x02` \x01\x01Qa\x19\x0BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\rIW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r@\x90a4\xE0V[`@Q\x80\x91\x03\x90\xFD[a\rz\x89`@\x01Q\x82\x81Q\x81\x10a\rcWa\rba*\xA6V[[` \x02` \x01\x01Q\x87a\x12\xC7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC8)LV\x8D\x8D\x84\x81\x81\x10a\r\xCBWa\r\xCAa*\xA6V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xC0\x01Q\x85\x81Q\x81\x10a\r\xF0Wa\r\xEFa*\xA6V[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x16\x93\x92\x91\x90a3oV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EU\x91\x90a5(V[\x85` \x01Q\x82\x81Q\x81\x10a\x0ElWa\x0Eka*\xA6V[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x84` \x01Q\x81\x81Q\x81\x10a\x0E\xADWa\x0E\xACa*\xA6V[[` \x02` \x01\x01Q\x85_\x01Q\x82\x81Q\x81\x10a\x0E\xCBWa\x0E\xCAa*\xA6V[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP__\x90P__\x90P[\x8A` \x01QQ\x81\x10\x15a\x10\xCCWa\x0FQ\x86_\x01Q\x82\x81Q\x81\x10a\x0F$Wa\x0F#a*\xA6V[[` \x02` \x01\x01Q\x8F\x8F\x86\x81\x81\x10a\x0F?Wa\x0F>a*\xA6V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1Ca\x1B\x04V[\x15a\x10\xBFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF2\xBE\x94\xAE\x8F\x8F\x86\x81\x81\x10a\x0F\xA5Wa\x0F\xA4a*\xA6V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8E\x89` \x01Q\x85\x81Q\x81\x10a\x0F\xCAWa\x0F\xC9a*\xA6V[[` \x02` \x01\x01Q\x8F`\xE0\x01Q\x88\x81Q\x81\x10a\x0F\xE9Wa\x0F\xE8a*\xA6V[[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x10\x03Wa\x10\x02a*\xA6V[[` \x02` \x01\x01Q`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10*\x94\x93\x92\x91\x90a5SV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10i\x91\x90a5(V[\x87_\x01Q\x84\x81Q\x81\x10a\x10\x7FWa\x10~a*\xA6V[[` \x02` \x01\x01\x81\x81Qa\x10\x93\x91\x90a5\x96V[\x91P\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81`\x01\x01\x91P[\x80\x80`\x01\x01\x91PPa\x0E\xFEV[PP\x80\x80`\x01\x01\x91PPa\n\xD8V[PPP__a\x10\xF4\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x01\x9EV[\x91P\x91P\x81a\x118W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11/\x90a6kV[`@Q\x80\x91\x03\x90\xFD[\x80a\x11xW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11o\x90a6\xF9V[`@Q\x80\x91\x03\x90\xFD[PP_\x87\x82` \x01Q`@Q` \x01a\x11\x92\x92\x91\x90a7\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x82\x81\x95P\x95PPPPP\x95P\x95\x93PPPPV[__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x11\xFBa\x1E\xA4V[a\x12\x03a\x1E\xBCV[\x83_\x01Q\x81_`\x03\x81\x10a\x12\x1AWa\x12\x19a*\xA6V[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x03\x81\x10a\x12<Wa\x12;a*\xA6V[[` \x02\x01\x81\x81RPP\x82\x81`\x02`\x03\x81\x10a\x12ZWa\x12Ya*\xA6V[[` \x02\x01\x81\x81RPP_`@\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x12~W\xFE[P\x80a\x12\xBFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xB6\x90a8mV[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x12\xCFa\x1E\xA4V[a\x12\xD7a\x1E\xDEV[\x83_\x01Q\x81_`\x04\x81\x10a\x12\xEEWa\x12\xEDa*\xA6V[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x04\x81\x10a\x13\x10Wa\x13\x0Fa*\xA6V[[` \x02\x01\x81\x81RPP\x82_\x01Q\x81`\x02`\x04\x81\x10a\x131Wa\x130a*\xA6V[[` \x02\x01\x81\x81RPP\x82` \x01Q\x81`\x03`\x04\x81\x10a\x13SWa\x13Ra*\xA6V[[` \x02\x01\x81\x81RPP_`@\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x13wW\xFE[P\x80a\x13\xB8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\xAF\x90a8\xD5V[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x13\xC8a\x1F\0V[`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x81R` \x01\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D\x81RP\x81RP\x90P\x90V[a\x14\x92a\x1E\xA4V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x02\x81RP\x90P\x90V[a\x14\xB6a\x1E\xA4V[__\x90P__\x90P_\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x85_\x1Ca\x14\xED\x91\x90a+\xE2V[\x90P[`\x01\x15a\x15\x99Wa\x15\0\x81a\x1B\x1AV[\x80\x93P\x81\x94PPP\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x157Wa\x156a+\xB5V[[\x82\x83\t\x83\x03a\x15_W`@Q\x80`@\x01`@R\x80\x82\x81R` \x01\x83\x81RP\x93PPPPa\x15\xB4V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x15\x8EWa\x15\x8Da+\xB5V[[`\x01\x82\x08\x90Pa\x14\xF0V[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x93PPPP[\x91\x90PV[___`@Q\x80`@\x01`@R\x80\x89\x81R` \x01\x87\x81RP\x90P_`@Q\x80`@\x01`@R\x80\x89\x81R` \x01\x87\x81RP\x90Pa\x15\xF3a\x1F&V[__\x90P[`\x02\x81\x10\x15a\x18\x11W_`\x06\x82a\x16\x0F\x91\x90a8\xF3V[\x90P\x84\x82`\x02\x81\x10a\x16$Wa\x16#a*\xA6V[[` \x02\x01Q_\x01Q\x83_\x83a\x169\x91\x90a2bV[`\x0C\x81\x10a\x16JWa\x16Ia*\xA6V[[` \x02\x01\x81\x81RPP\x84\x82`\x02\x81\x10a\x16fWa\x16ea*\xA6V[[` \x02\x01Q` \x01Q\x83`\x01\x83a\x16}\x91\x90a2bV[`\x0C\x81\x10a\x16\x8EWa\x16\x8Da*\xA6V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x16\xAAWa\x16\xA9a*\xA6V[[` \x02\x01Q_\x01Q_`\x02\x81\x10a\x16\xC4Wa\x16\xC3a*\xA6V[[` \x02\x01Q\x83`\x02\x83a\x16\xD7\x91\x90a2bV[`\x0C\x81\x10a\x16\xE8Wa\x16\xE7a*\xA6V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x17\x04Wa\x17\x03a*\xA6V[[` \x02\x01Q_\x01Q`\x01`\x02\x81\x10a\x17\x1FWa\x17\x1Ea*\xA6V[[` \x02\x01Q\x83`\x03\x83a\x172\x91\x90a2bV[`\x0C\x81\x10a\x17CWa\x17Ba*\xA6V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x17_Wa\x17^a*\xA6V[[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x17zWa\x17ya*\xA6V[[` \x02\x01Q\x83`\x04\x83a\x17\x8D\x91\x90a2bV[`\x0C\x81\x10a\x17\x9EWa\x17\x9Da*\xA6V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x17\xBAWa\x17\xB9a*\xA6V[[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x17\xD6Wa\x17\xD5a*\xA6V[[` \x02\x01Q\x83`\x05\x83a\x17\xE9\x91\x90a2bV[`\x0C\x81\x10a\x17\xFAWa\x17\xF9a*\xA6V[[` \x02\x01\x81\x81RPPP\x80\x80`\x01\x01\x91PPa\x15\xF8V[Pa\x18\x1Aa\x1FIV[_` \x82` `\x0C\x02\x85`\x08\x8C\xFA\x90P\x80_\x83_`\x01\x81\x10a\x18?Wa\x18>a*\xA6V[[` \x02\x01Q\x14\x15\x96P\x96PPPPPP\x95P\x95\x93PPPPV[\x80__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x81`@Qa\x18\xA0\x91\x90a*TV[`@Q\x80\x91\x03\x90\xA1PV[__a\x18\xB6\x84a\x1C\x0FV[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x19\x01W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xF8\x90a9\xA4V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[_\x81Q_R\x81` \x01Q` R`@_ \x90P\x91\x90PV[___\x90P[_\x83\x11\x15a\x19UW`\x01\x83a\x19>\x91\x90a0NV[\x83\x16\x92P\x80\x80a\x19M\x90a9\xCFV[\x91PPa\x19)V[\x80\x91PP\x91\x90PV[a\x19fa\x1E\xA4V[a\x02\0\x82a\xFF\xFF\x16\x10a\x19\xAEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xA5\x90a:BV[`@Q\x80\x91\x03\x90\xFD[`\x01\x82a\xFF\xFF\x16\x03a\x19\xC2W\x82\x90Pa\x1AFV[_`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90P_\x84\x90P_`\x01\x90P__\x90P[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a\x1A>W`\x01\x80\x82`\xFF\x16\x88a\xFF\xFF\x16\x90\x1C\x16a\xFF\xFF\x16\x03a\x1A\x1CWa\x1A\x19\x84\x84a\x12\xC7V[\x93P[a\x1A&\x83\x84a\x12\xC7V[\x92P`\x01\x82a\xFF\xFF\x16\x90\x1B\x91P\x80`\x01\x01\x90Pa\x19\xE8V[\x83\x94PPPPP[\x92\x91PPV[a\x1ATa\x1E\xA4V[_\x82_\x01Q\x14\x80\x15a\x1AiWP_\x82` \x01Q\x14[\x15a\x1A\x8AW`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x1A\xFFV[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x1A\xCE\x91\x90a+\xE2V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1A\xF9\x91\x90a0NV[\x81RP\x90P[\x91\x90PV[_`\x01\x82`\xFF\x16\x84\x90\x1C\x16`\x01\x14\x90P\x92\x91PPV[___\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1BLWa\x1BKa+\xB5V[[`\x03\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1B}Wa\x1B|a+\xB5V[[\x86\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1B\xADWa\x1B\xACa+\xB5V[[\x88\x89\t\t\x08\x90P_a\x1C\0\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1D)V[\x90P\x81\x81\x93P\x93PPP\x91P\x91V[_a\x01\0\x82Q\x11\x15a\x1CVW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1CM\x90a:\xF6V[`@Q\x80\x91\x03\x90\xFD[_\x82Q\x03a\x1CfW_\x90Pa\x1D$V[__\x83_\x81Q\x81\x10a\x1C{Wa\x1Cza*\xA6V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16`\x01\x90\x1B\x91P_`\x01\x90P[\x84Q\x81\x10\x15a\x1D\x1DW\x84\x81\x81Q\x81\x10a\x1C\xB4Wa\x1C\xB3a*\xA6V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16`\x01\x90\x1B\x91P\x82\x82\x11a\x1D\rW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\x04\x90a;\xAAV[`@Q\x80\x91\x03\x90\xFD[\x81\x83\x17\x92P\x80`\x01\x01\x90Pa\x1C\x98V[P\x81\x92PPP[\x91\x90PV[__a\x1D3a\x1FIV[a\x1D;a\x1FkV[` \x81_`\x06\x81\x10a\x1DPWa\x1DOa*\xA6V[[` \x02\x01\x81\x81RPP` \x81`\x01`\x06\x81\x10a\x1DoWa\x1Dna*\xA6V[[` \x02\x01\x81\x81RPP` \x81`\x02`\x06\x81\x10a\x1D\x8EWa\x1D\x8Da*\xA6V[[` \x02\x01\x81\x81RPP\x86\x81`\x03`\x06\x81\x10a\x1D\xACWa\x1D\xABa*\xA6V[[` \x02\x01\x81\x81RPP\x85\x81`\x04`\x06\x81\x10a\x1D\xCAWa\x1D\xC9a*\xA6V[[` \x02\x01\x81\x81RPP\x84\x81`\x05`\x06\x81\x10a\x1D\xE8Wa\x1D\xE7a*\xA6V[[` \x02\x01\x81\x81RPP` \x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82_\x81\x03a\x1E\x0BW\xFE[P\x82a\x1ELW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1EC\x90a<\x12V[`@Q\x80\x91\x03\x90\xFD[\x81_`\x01\x81\x10a\x1E_Wa\x1E^a*\xA6V[[` \x02\x01Q\x93PPPP\x93\x92PPPV[`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80a\x1F\x13a\x1F\x8DV[\x81R` \x01a\x1F a\x1F\x8DV[\x81RP\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x1F\xD2\x81a\x1F\xC0V[\x81\x14a\x1F\xDCW__\xFD[PV[_\x815\x90Pa\x1F\xED\x81a\x1F\xC9V[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a =\x82a\x1F\xF7V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a \\Wa [a \x07V[[\x80`@RPPPV[_a na\x1F\xAFV[\x90Pa z\x82\x82a 4V[\x91\x90PV[__\xFD[_\x81\x90P\x91\x90PV[a \x95\x81a \x83V[\x81\x14a \x9FW__\xFD[PV[_\x815\x90Pa \xB0\x81a \x8CV[\x92\x91PPV[_`@\x82\x84\x03\x12\x15a \xCBWa \xCAa\x1F\xF3V[[a \xD5`@a eV[\x90P_a \xE4\x84\x82\x85\x01a \xA2V[_\x83\x01RP` a \xF7\x84\x82\x85\x01a \xA2V[` \x83\x01RP\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a!!Wa! a \x07V[[` \x82\x02\x90P\x91\x90PV[__\xFD[_a!Ba!=\x84a!\x07V[a eV[\x90P\x80` \x84\x02\x83\x01\x85\x81\x11\x15a!\\Wa![a!,V[[\x83[\x81\x81\x10\x15a!\x85W\x80a!q\x88\x82a \xA2V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa!^V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a!\xA3Wa!\xA2a!\x03V[[`\x02a!\xB0\x84\x82\x85a!0V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a!\xCEWa!\xCDa\x1F\xF3V[[a!\xD8`@a eV[\x90P_a!\xE7\x84\x82\x85\x01a!\x8FV[_\x83\x01RP`@a!\xFA\x84\x82\x85\x01a!\x8FV[` \x83\x01RP\x92\x91PPV[____a\x01 \x85\x87\x03\x12\x15a\"\x1FWa\"\x1Ea\x1F\xB8V[[_a\",\x87\x82\x88\x01a\x1F\xDFV[\x94PP` a\"=\x87\x82\x88\x01a \xB6V[\x93PP``a\"N\x87\x82\x88\x01a!\xB9V[\x92PP`\xE0a\"_\x87\x82\x88\x01a \xB6V[\x91PP\x92\x95\x91\x94P\x92PV[_\x81\x15\x15\x90P\x91\x90PV[a\"\x7F\x81a\"kV[\x82RPPV[_`@\x82\x01\x90Pa\"\x98_\x83\x01\x85a\"vV[a\"\xA5` \x83\x01\x84a\"vV[\x93\x92PPPV[a\"\xB5\x81a\"kV[\x81\x14a\"\xBFW__\xFD[PV[_\x815\x90Pa\"\xD0\x81a\"\xACV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\"\xEBWa\"\xEAa\x1F\xB8V[[_a\"\xF8\x84\x82\x85\x01a\"\xC2V[\x91PP\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a#Ca#>a#9\x84a#\x01V[a# V[a#\x01V[\x90P\x91\x90PV[_a#T\x82a#)V[\x90P\x91\x90PV[_a#e\x82a#JV[\x90P\x91\x90PV[a#u\x81a#[V[\x82RPPV[_` \x82\x01\x90Pa#\x8E_\x83\x01\x84a#lV[\x92\x91PPV[_a#\x9E\x82a#JV[\x90P\x91\x90PV[a#\xAE\x81a#\x94V[\x82RPPV[_` \x82\x01\x90Pa#\xC7_\x83\x01\x84a#\xA5V[\x92\x91PPV[_a#\xD7\x82a#JV[\x90P\x91\x90PV[a#\xE7\x81a#\xCDV[\x82RPPV[_` \x82\x01\x90Pa$\0_\x83\x01\x84a#\xDEV[\x92\x91PPV[__\xFD[__\x83`\x1F\x84\x01\x12a$\x1FWa$\x1Ea!\x03V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$<Wa$;a$\x06V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a$XWa$Wa!,V[[\x92P\x92\x90PV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a$w\x81a$_V[\x81\x14a$\x81W__\xFD[PV[_\x815\x90Pa$\x92\x81a$nV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a$\xB2Wa$\xB1a \x07V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a$\xD5a$\xD0\x84a$\x98V[a eV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a$\xF8Wa$\xF7a!,V[[\x83[\x81\x81\x10\x15a%!W\x80a%\r\x88\x82a$\x84V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa$\xFAV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a%?Wa%>a!\x03V[[\x815a%O\x84\x82` \x86\x01a$\xC3V[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%rWa%qa \x07V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a%\x95a%\x90\x84a%XV[a eV[\x90P\x80\x83\x82R` \x82\x01\x90P`@\x84\x02\x83\x01\x85\x81\x11\x15a%\xB8Wa%\xB7a!,V[[\x83[\x81\x81\x10\x15a%\xE1W\x80a%\xCD\x88\x82a \xB6V[\x84R` \x84\x01\x93PP`@\x81\x01\x90Pa%\xBAV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a%\xFFWa%\xFEa!\x03V[[\x815a&\x0F\x84\x82` \x86\x01a%\x83V[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&2Wa&1a \x07V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a&Ua&P\x84a&\x18V[a eV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a&xWa&wa!,V[[\x83[\x81\x81\x10\x15a&\xBFW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\x9DWa&\x9Ca!\x03V[[\x80\x86\x01a&\xAA\x89\x82a%+V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa&zV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a&\xDDWa&\xDCa!\x03V[[\x815a&\xED\x84\x82` \x86\x01a&CV[\x91PP\x92\x91PPV[_a\x01\x80\x82\x84\x03\x12\x15a'\x0CWa'\x0Ba\x1F\xF3V[[a'\x17a\x01\0a eV[\x90P_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'6Wa'5a \x7FV[[a'B\x84\x82\x85\x01a%+V[_\x83\x01RP` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'eWa'da \x7FV[[a'q\x84\x82\x85\x01a%\xEBV[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\x95Wa'\x94a \x7FV[[a'\xA1\x84\x82\x85\x01a%\xEBV[`@\x83\x01RP``a'\xB5\x84\x82\x85\x01a!\xB9V[``\x83\x01RP`\xE0a'\xC9\x84\x82\x85\x01a \xB6V[`\x80\x83\x01RPa\x01 \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xEEWa'\xEDa \x7FV[[a'\xFA\x84\x82\x85\x01a%+V[`\xA0\x83\x01RPa\x01@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x1FWa(\x1Ea \x7FV[[a(+\x84\x82\x85\x01a%+V[`\xC0\x83\x01RPa\x01`\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(PWa(Oa \x7FV[[a(\\\x84\x82\x85\x01a&\xC9V[`\xE0\x83\x01RP\x92\x91PPV[_____`\x80\x86\x88\x03\x12\x15a(\x81Wa(\x80a\x1F\xB8V[[_a(\x8E\x88\x82\x89\x01a\x1F\xDFV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xAFWa(\xAEa\x1F\xBCV[[a(\xBB\x88\x82\x89\x01a$\nV[\x94P\x94PP`@a(\xCE\x88\x82\x89\x01a$\x84V[\x92PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xEFWa(\xEEa\x1F\xBCV[[a(\xFB\x88\x82\x89\x01a&\xF6V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a)Q\x81a)1V[\x82RPPV[_a)b\x83\x83a)HV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a)\x84\x82a)\x08V[a)\x8E\x81\x85a)\x12V[\x93Pa)\x99\x83a)\"V[\x80_[\x83\x81\x10\x15a)\xC9W\x81Qa)\xB0\x88\x82a)WV[\x97Pa)\xBB\x83a)nV[\x92PP`\x01\x81\x01\x90Pa)\x9CV[P\x85\x93PPPP\x92\x91PPV[_`@\x83\x01_\x83\x01Q\x84\x82\x03_\x86\x01Ra)\xF0\x82\x82a)zV[\x91PP` \x83\x01Q\x84\x82\x03` \x86\x01Ra*\n\x82\x82a)zV[\x91PP\x80\x91PP\x92\x91PPV[a* \x81a\x1F\xC0V[\x82RPPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra*>\x81\x85a)\xD6V[\x90Pa*M` \x83\x01\x84a*\x17V[\x93\x92PPPV[_` \x82\x01\x90Pa*g_\x83\x01\x84a\"vV[\x92\x91PPV[_a*w\x82a#JV[\x90P\x91\x90PV[a*\x87\x81a*mV[\x82RPPV[_` \x82\x01\x90Pa*\xA0_\x83\x01\x84a*~V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a*\xEDa*\xE8\x82a\x1F\xC0V[a*\xD3V[\x82RPPV[_\x81\x90P\x91\x90PV[a+\ra+\x08\x82a \x83V[a*\xF3V[\x82RPPV[_a+\x1E\x82\x8Ca*\xDCV[` \x82\x01\x91Pa+.\x82\x8Ba*\xFCV[` \x82\x01\x91Pa+>\x82\x8Aa*\xFCV[` \x82\x01\x91Pa+N\x82\x89a*\xFCV[` \x82\x01\x91Pa+^\x82\x88a*\xFCV[` \x82\x01\x91Pa+n\x82\x87a*\xFCV[` \x82\x01\x91Pa+~\x82\x86a*\xFCV[` \x82\x01\x91Pa+\x8E\x82\x85a*\xFCV[` \x82\x01\x91Pa+\x9E\x82\x84a*\xFCV[` \x82\x01\x91P\x81\x90P\x9A\x99PPPPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a+\xEC\x82a \x83V[\x91Pa+\xF7\x83a \x83V[\x92P\x82a,\x07Wa,\x06a+\xB5V[[\x82\x82\x06\x90P\x92\x91PPV[_a,\x1C\x82a#\x01V[\x90P\x91\x90PV[a,,\x81a,\x12V[\x81\x14a,6W__\xFD[PV[_\x81Q\x90Pa,G\x81a,#V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a,bWa,aa\x1F\xB8V[[_a,o\x84\x82\x85\x01a,9V[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBLSSignatureChecker.onlyCoordina_\x82\x01R\x7FtorOwner: caller is not the owne` \x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`@\x82\x01RPV[_a-\x08`\\\x83a,xV[\x91Pa-\x13\x82a,\x88V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra-5\x81a,\xFCV[\x90P\x91\x90PV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a-\x96`7\x83a,xV[\x91Pa-\xA1\x82a-<V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra-\xC3\x81a-\x8AV[\x90P\x91\x90PV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: input quorum length mismatc` \x82\x01R\x7Fh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a.J`A\x83a,xV[\x91Pa.U\x82a-\xCAV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra.w\x81a.>V[\x90P\x91\x90PV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: input nonsigner length mism` \x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a.\xFE`D\x83a,xV[\x91Pa/\t\x82a.~V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra/+\x81a.\xF2V[\x90P\x91\x90PV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: invalid reference block\0\0\0\0` \x82\x01RPV[_a/\x8C`<\x83a,xV[\x91Pa/\x97\x82a/2V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra/\xB9\x81a/\x80V[\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[a/\xD5\x81a/\xC0V[\x81\x14a/\xDFW__\xFD[PV[_\x81Q\x90Pa/\xF0\x81a/\xCCV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a0\x0BWa0\na\x1F\xB8V[[_a0\x18\x84\x82\x85\x01a/\xE2V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a0X\x82a \x83V[\x91Pa0c\x83a \x83V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a0{Wa0za0!V[[\x92\x91PPV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: nonSignerPubkeys not sorted` \x82\x01RPV[_a0\xDB`@\x83a,xV[\x91Pa0\xE6\x82a0\x81V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra1\x08\x81a0\xCFV[\x90P\x91\x90PV[a1\x18\x81a$_V[\x82RPPV[_a18a13a1.\x84a$_V[a# V[a \x83V[\x90P\x91\x90PV[a1H\x81a1\x1EV[\x82RPPV[_``\x82\x01\x90Pa1a_\x83\x01\x86a*\x17V[a1n` \x83\x01\x85a1\x0FV[a1{`@\x83\x01\x84a1?V[\x94\x93PPPPV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a1\xAF\x81a1\x83V[\x81\x14a1\xB9W__\xFD[PV[_\x81Q\x90Pa1\xCA\x81a1\xA6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a1\xE5Wa1\xE4a\x1F\xB8V[[_a1\xF2\x84\x82\x85\x01a1\xBCV[\x91PP\x92\x91PPV[a2\x04\x81a/\xC0V[\x82RPPV[_` \x82\x01\x90Pa2\x1D_\x83\x01\x84a1\xFBV[\x92\x91PPV[_\x81Q\x90Pa21\x81a \x8CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a2LWa2Ka\x1F\xB8V[[_a2Y\x84\x82\x85\x01a2#V[\x91PP\x92\x91PPV[_a2l\x82a \x83V[\x91Pa2w\x83a \x83V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a2\x8FWa2\x8Ea0!V[[\x92\x91PPV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: StakeRegistry updates must ` \x82\x01R\x7Fbe within withdrawalDelayBlocks `@\x82\x01R\x7Fwindow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_a3;`f\x83a,xV[\x91Pa3F\x82a2\x95V[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3h\x81a3/V[\x90P\x91\x90PV[_``\x82\x01\x90Pa3\x82_\x83\x01\x86a1\xFBV[a3\x8F` \x83\x01\x85a1\x0FV[a3\x9C`@\x83\x01\x84a1?V[\x94\x93PPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a3\xD8\x81a3\xA4V[\x81\x14a3\xE2W__\xFD[PV[_\x81Q\x90Pa3\xF3\x81a3\xCFV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a4\x0EWa4\ra\x1F\xB8V[[_a4\x1B\x84\x82\x85\x01a3\xE5V[\x91PP\x92\x91PPV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: quorumApk hash in storage d` \x82\x01R\x7Foes not match provided quorum ap`@\x82\x01R\x7Fk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_a4\xCA`a\x83a,xV[\x91Pa4\xD5\x82a4$V[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra4\xF7\x81a4\xBEV[\x90P\x91\x90PV[a5\x07\x81a)1V[\x81\x14a5\x11W__\xFD[PV[_\x81Q\x90Pa5\"\x81a4\xFEV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a5=Wa5<a\x1F\xB8V[[_a5J\x84\x82\x85\x01a5\x14V[\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pa5f_\x83\x01\x87a1\xFBV[a5s` \x83\x01\x86a1\x0FV[a5\x80`@\x83\x01\x85a*\x17V[a5\x8D``\x83\x01\x84a1?V[\x95\x94PPPPPV[_a5\xA0\x82a)1V[\x91Pa5\xAB\x83a)1V[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\xCFWa5\xCEa0!V[[\x92\x91PPV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: pairing precompile call fai` \x82\x01R\x7Fled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a6U`C\x83a,xV[\x91Pa6`\x82a5\xD5V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra6\x82\x81a6IV[\x90P\x91\x90PV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0` \x82\x01RPV[_a6\xE3`9\x83a,xV[\x91Pa6\xEE\x82a6\x89V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra7\x10\x81a6\xD7V[\x90P\x91\x90PV[_\x81`\xE0\x1B\x90P\x91\x90PV[_a7-\x82a7\x17V[\x90P\x91\x90PV[a7Ea7@\x82a$_V[a7#V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a7w\x81a\x1F\xC0V[\x82RPPV[_a7\x88\x83\x83a7nV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a7\xAA\x82a7KV[a7\xB4\x81\x85a7UV[\x93Pa7\xBF\x83a7_V[\x80_[\x83\x81\x10\x15a7\xEFW\x81Qa7\xD6\x88\x82a7}V[\x97Pa7\xE1\x83a7\x94V[\x92PP`\x01\x81\x01\x90Pa7\xC2V[P\x85\x93PPPP\x92\x91PPV[_a8\x07\x82\x85a74V[`\x04\x82\x01\x91Pa8\x17\x82\x84a7\xA0V[\x91P\x81\x90P\x93\x92PPPV[\x7Fec-mul-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a8W`\r\x83a,xV[\x91Pa8b\x82a8#V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8\x84\x81a8KV[\x90P\x91\x90PV[\x7Fec-add-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a8\xBF`\r\x83a,xV[\x91Pa8\xCA\x82a8\x8BV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8\xEC\x81a8\xB3V[\x90P\x91\x90PV[_a8\xFD\x82a \x83V[\x91Pa9\x08\x83a \x83V[\x92P\x82\x82\x02a9\x16\x81a \x83V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a9-Wa9,a0!V[[P\x92\x91PPV[\x7FBitmapUtils.orderedBytesArrayToB_\x82\x01R\x7Fitmap: bitmap exceeds max value\0` \x82\x01RPV[_a9\x8E`?\x83a,xV[\x91Pa9\x99\x82a94V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\xBB\x81a9\x82V[\x90P\x91\x90PV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[_a9\xD9\x82a9\xC2V[\x91Pa\xFF\xFF\x82\x03a9\xEDWa9\xECa0!V[[`\x01\x82\x01\x90P\x91\x90PV[\x7Fscalar-too-large\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a:,`\x10\x83a,xV[\x91Pa:7\x82a9\xF8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra:Y\x81a: V[\x90P\x91\x90PV[\x7FBitmapUtils.orderedBytesArrayToB_\x82\x01R\x7Fitmap: orderedBytesArray is too ` \x82\x01R\x7Flong\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a:\xE0`D\x83a,xV[\x91Pa:\xEB\x82a:`V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\r\x81a:\xD4V[\x90P\x91\x90PV[\x7FBitmapUtils.orderedBytesArrayToB_\x82\x01R\x7Fitmap: orderedBytesArray is not ` \x82\x01R\x7Fordered\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a;\x94`G\x83a,xV[\x91Pa;\x9F\x82a;\x14V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\xC1\x81a;\x88V[\x90P\x91\x90PV[\x7FBN254.expMod: call failure\0\0\0\0\0\0_\x82\x01RPV[_a;\xFC`\x1A\x83a,xV[\x91Pa<\x07\x82a;\xC8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<)\x81a;\xF0V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xDAWu\x03\x88A\x9C\x15\xA9lCe\xAE\x9A\xA5\xDA\xA1\xF2_[\xCD\xE1\xCA\xD9\x9F\xB3>\x93#\x9E\x02hdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610086575f3560e01c80636d14a987116100595780636d14a987146101135780636efb463614610131578063b98d090814610162578063df5cf7231461018057610086565b8063171f1d5b1461008a578063416c7e5e146100bb5780635df45946146100d757806368304835146100f5575b5f5ffd5b6100a4600480360381019061009f9190612206565b61019e565b6040516100b2929190612285565b60405180910390f35b6100d560048036038101906100d091906122d6565b610301565b005b6100df610407565b6040516100ec919061237b565b60405180910390f35b6100fd61042b565b60405161010a91906123b4565b60405180910390f35b61011b61044f565b60405161012891906123ed565b60405180910390f35b61014b60048036038101906101469190612868565b610473565b604051610159929190612a26565b60405180910390f35b61016a6111be565b6040516101779190612a54565b60405180910390f35b6101886111cf565b6040516101959190612a8d565b60405180910390f35b5f5f5f7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187875f01518860200151885f01515f600281106101e2576101e1612aa6565b5b6020020151895f01516001600281106101fe576101fd612aa6565b5b60200201518a602001515f6002811061021a57610219612aa6565b5b60200201518b6020015160016002811061023757610236612aa6565b5b60200201518b5f01518c6020015160405160200161025d99989796959493929190612b13565b604051602081830303815290604052805190602001205f1c61027f9190612be2565b90506102ef6102a961029a83896111f390919063ffffffff16565b866112c790919063ffffffff16565b6102b16113c0565b6102e56102ce856102c061148a565b6111f390919063ffffffff16565b6102d78c6114ae565b6112c790919063ffffffff16565b886201d4c06115b9565b80935081945050505094509492505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561036a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061038e9190612c4d565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146103fb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016103f290612d1e565b60405180910390fd5b61040481611859565b50565b7f000000000000000000000000000000000000000000000000000000000000000081565b7f000000000000000000000000000000000000000000000000000000000000000081565b7f000000000000000000000000000000000000000000000000000000000000000081565b61047b611e70565b5f5f86869050036104c1576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104b890612dac565b60405180910390fd5b826040015151868690501480156104df57508260a001515186869050145b80156104f257508260c001515186869050145b801561050557508260e001515186869050145b610544576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161053b90612e60565b60405180910390fd5b825f0151518360200151511461058f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161058690612f14565b60405180910390fd5b4363ffffffff168463ffffffff16106105dd576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016105d490612fa2565b60405180910390fd5b5f60405180604001604052805f81526020015f81525090506105fd611e70565b8787905067ffffffffffffffff81111561061a57610619612007565b5b6040519080825280602002602001820160405280156106485781602001602082028036833780820191505090505b5081602001819052508787905067ffffffffffffffff81111561066e5761066d612007565b5b60405190808252806020026020018201604052801561069c5781602001602082028036833780820191505090505b50815f01819052506106ac611e8a565b85602001515167ffffffffffffffff8111156106cb576106ca612007565b5b6040519080825280602002602001820160405280156106f95781602001602082028036833780820191505090505b50815f018190525085602001515167ffffffffffffffff8111156107205761071f612007565b5b60405190808252806020026020018201604052801561074e5781602001602082028036833780820191505090505b5081602001819052505f6108318a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610808573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061082c9190612ff6565b6118ab565b90505f5f90505b876020015151811015610ab15761086c8860200151828151811061085f5761085e612aa6565b5b602002602001015161190b565b8360200151828151811061088357610882612aa6565b5b6020026020010181815250505f81146109255782602001516001826108a8919061304e565b815181106108b9576108b8612aa6565b5b60200260200101515f1c836020015182815181106108da576108d9612aa6565b5b60200260200101515f1c11610924576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161091b906130f1565b60405180910390fd5b5b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166304ec63518460200151838151811061097857610977612aa6565b5b60200260200101518b8b5f0151858151811061099757610996612aa6565b5b60200260200101516040518463ffffffff1660e01b81526004016109bd9392919061314e565b602060405180830381865afa1580156109d8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109fc91906131d0565b77ffffffffffffffffffffffffffffffffffffffffffffffff16835f01518281518110610a2c57610a2b612aa6565b5b602002602001018181525050610aa2610a93610a6684865f01518581518110610a5857610a57612aa6565b5b602002602001015116611923565b8a602001518481518110610a7d57610a7c612aa6565b5b602002602001015161195e90919063ffffffff16565b866112c790919063ffffffff16565b94508080600101915050610838565b5050610abc83611a4c565b92505f5f5f9054906101000a900460ff1690505f5f90505f5f90505b8b8b90508110156110db578215610bf3578963ffffffff16827f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663249a0c428f8f86818110610b4057610b3f612aa6565b5b9050013560f81c60f81b60f81c6040518263ffffffff1660e01b8152600401610b69919061320a565b602060405180830381865afa158015610b84573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ba89190613237565b610bb29190613262565b11610bf2576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610be990613351565b60405180910390fd5b5b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166368bccaac8d8d84818110610c4257610c41612aa6565b5b9050013560f81c60f81b60f81c8c8c60a001518581518110610c6757610c66612aa6565b5b60200260200101516040518463ffffffff1660e01b8152600401610c8d9392919061336f565b602060405180830381865afa158015610ca8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ccc91906133f9565b67ffffffffffffffff1916610cfe8a604001518381518110610cf157610cf0612aa6565b5b602002602001015161190b565b67ffffffffffffffff191614610d49576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d40906134e0565b60405180910390fd5b610d7a89604001518281518110610d6357610d62612aa6565b5b6020026020010151876112c790919063ffffffff16565b95507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663c8294c568d8d84818110610dcb57610dca612aa6565b5b9050013560f81c60f81b60f81c8c8c60c001518581518110610df057610def612aa6565b5b60200260200101516040518463ffffffff1660e01b8152600401610e169392919061336f565b602060405180830381865afa158015610e31573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e559190613528565b85602001518281518110610e6c57610e6b612aa6565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff168152505084602001518181518110610ead57610eac612aa6565b5b6020026020010151855f01518281518110610ecb57610eca612aa6565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff16815250505f5f90505f5f90505b8a60200151518110156110cc57610f51865f01518281518110610f2457610f23612aa6565b5b60200260200101518f8f86818110610f3f57610f3e612aa6565b5b9050013560f81c60f81b60f81c611b04565b156110bf577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f2be94ae8f8f86818110610fa557610fa4612aa6565b5b9050013560f81c60f81b60f81c8e89602001518581518110610fca57610fc9612aa6565b5b60200260200101518f60e001518881518110610fe957610fe8612aa6565b5b6020026020010151878151811061100357611002612aa6565b5b60200260200101516040518563ffffffff1660e01b815260040161102a9493929190613553565b602060405180830381865afa158015611045573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110699190613528565b875f0151848151811061107f5761107e612aa6565b5b602002602001018181516110939190613596565b9150906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff16815250508160010191505b8080600101915050610efe565b50508080600101915050610ad8565b5050505f5f6110f48c868a606001518b6080015161019e565b9150915081611138576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161112f9061366b565b60405180910390fd5b80611178576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161116f906136f9565b60405180910390fd5b50505f8782602001516040516020016111929291906137fc565b604051602081830303815290604052805190602001209050828195509550505050509550959350505050565b5f5f9054906101000a900460ff1681565b7f000000000000000000000000000000000000000000000000000000000000000081565b6111fb611ea4565b611203611ebc565b835f0151815f6003811061121a57611219612aa6565b5b60200201818152505083602001518160016003811061123c5761123b612aa6565b5b602002018181525050828160026003811061125a57611259612aa6565b5b6020020181815250505f60408360608460076107d05a03fa9050805f810361127e57fe5b50806112bf576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016112b69061386d565b60405180910390fd5b505092915050565b6112cf611ea4565b6112d7611ede565b835f0151815f600481106112ee576112ed612aa6565b5b6020020181815250508360200151816001600481106113105761130f612aa6565b5b602002018181525050825f01518160026004811061133157611330612aa6565b5b60200201818152505082602001518160036004811061135357611352612aa6565b5b6020020181815250505f60408360808460066107d05a03fa9050805f810361137757fe5b50806113b8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016113af906138d5565b60405180910390fd5b505092915050565b6113c8611f00565b604051806040016040528060405180604001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed815250815260200160405180604001604052807f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec81526020017f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d815250815250905090565b611492611ea4565b6040518060400160405280600181526020016002815250905090565b6114b6611ea4565b5f5f90505f5f90505f7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47855f1c6114ed9190612be2565b90505b6001156115995761150081611b1a565b80935081945050507f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478061153757611536612bb5565b5b828309830361155f5760405180604001604052808281526020018381525093505050506115b4565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478061158e5761158d612bb5565b5b6001820890506114f0565b60405180604001604052805f81526020015f81525093505050505b919050565b5f5f5f60405180604001604052808981526020018781525090505f60405180604001604052808981526020018781525090506115f3611f26565b5f5f90505b6002811015611811575f60068261160f91906138f3565b905084826002811061162457611623612aa6565b5b60200201515f0151835f836116399190613262565b600c811061164a57611649612aa6565b5b60200201818152505084826002811061166657611665612aa6565b5b6020020151602001518360018361167d9190613262565b600c811061168e5761168d612aa6565b5b6020020181815250508382600281106116aa576116a9612aa6565b5b60200201515f01515f600281106116c4576116c3612aa6565b5b6020020151836002836116d79190613262565b600c81106116e8576116e7612aa6565b5b60200201818152505083826002811061170457611703612aa6565b5b60200201515f015160016002811061171f5761171e612aa6565b5b6020020151836003836117329190613262565b600c811061174357611742612aa6565b5b60200201818152505083826002811061175f5761175e612aa6565b5b6020020151602001515f6002811061177a57611779612aa6565b5b60200201518360048361178d9190613262565b600c811061179e5761179d612aa6565b5b6020020181815250508382600281106117ba576117b9612aa6565b5b6020020151602001516001600281106117d6576117d5612aa6565b5b6020020151836005836117e99190613262565b600c81106117fa576117f9612aa6565b5b6020020181815250505080806001019150506115f8565b5061181a611f49565b5f6020826020600c028560088cfa9050805f835f6001811061183f5761183e612aa6565b5b602002015114159650965050505050509550959350505050565b805f5f6101000a81548160ff0219169083151502179055507f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc816040516118a09190612a54565b60405180910390a150565b5f5f6118b684611c0f565b9050808360ff166001901b11611901576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016118f8906139a4565b60405180910390fd5b8091505092915050565b5f81515f52816020015160205260405f209050919050565b5f5f5f90505b5f8311156119555760018361193e919061304e565b83169250808061194d906139cf565b915050611929565b80915050919050565b611966611ea4565b6102008261ffff16106119ae576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016119a590613a42565b60405180910390fd5b60018261ffff16036119c257829050611a46565b5f60405180604001604052805f81526020015f81525090505f8490505f600190505f5f90505b8161ffff168661ffff1610611a3e576001808260ff168861ffff16901c1661ffff1603611a1c57611a1984846112c7565b93505b611a2683846112c7565b925060018261ffff16901b91508060010190506119e8565b839450505050505b92915050565b611a54611ea4565b5f825f0151148015611a6957505f8260200151145b15611a8a5760405180604001604052805f81526020015f8152509050611aff565b6040518060400160405280835f015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478460200151611ace9190612be2565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611af9919061304e565b81525090505b919050565b5f60018260ff1684901c16600114905092915050565b5f5f5f7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611b4c57611b4b612bb5565b5b60037f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611b7d57611b7c612bb5565b5b867f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611bad57611bac612bb5565b5b888909090890505f611c00827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f527f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611d29565b90508181935093505050915091565b5f61010082511115611c56576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611c4d90613af6565b60405180910390fd5b5f825103611c66575f9050611d24565b5f5f835f81518110611c7b57611c7a612aa6565b5b602001015160f81c60f81b60f81c60ff166001901b91505f600190505b8451811015611d1d57848181518110611cb457611cb3612aa6565b5b602001015160f81c60f81b60f81c60ff166001901b9150828211611d0d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611d0490613baa565b60405180910390fd5b8183179250806001019050611c98565b5081925050505b919050565b5f5f611d33611f49565b611d3b611f6b565b6020815f60068110611d5057611d4f612aa6565b5b602002018181525050602081600160068110611d6f57611d6e612aa6565b5b602002018181525050602081600260068110611d8e57611d8d612aa6565b5b6020020181815250508681600360068110611dac57611dab612aa6565b5b6020020181815250508581600460068110611dca57611dc9612aa6565b5b6020020181815250508481600560068110611de857611de7612aa6565b5b60200201818152505060208260c08360056107d05a03fa9250825f8103611e0b57fe5b5082611e4c576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611e4390613c12565b60405180910390fd5b815f60018110611e5f57611e5e612aa6565b5b602002015193505050509392505050565b604051806040016040528060608152602001606081525090565b604051806040016040528060608152602001606081525090565b60405180604001604052805f81526020015f81525090565b6040518060600160405280600390602082028036833780820191505090505090565b6040518060800160405280600490602082028036833780820191505090505090565b6040518060400160405280611f13611f8d565b8152602001611f20611f8d565b81525090565b604051806101800160405280600c90602082028036833780820191505090505090565b6040518060200160405280600190602082028036833780820191505090505090565b6040518060c00160405280600690602082028036833780820191505090505090565b6040518060400160405280600290602082028036833780820191505090505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b611fd281611fc0565b8114611fdc575f5ffd5b50565b5f81359050611fed81611fc9565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61203d82611ff7565b810181811067ffffffffffffffff8211171561205c5761205b612007565b5b80604052505050565b5f61206e611faf565b905061207a8282612034565b919050565b5f5ffd5b5f819050919050565b61209581612083565b811461209f575f5ffd5b50565b5f813590506120b08161208c565b92915050565b5f604082840312156120cb576120ca611ff3565b5b6120d56040612065565b90505f6120e4848285016120a2565b5f8301525060206120f7848285016120a2565b60208301525092915050565b5f5ffd5b5f67ffffffffffffffff82111561212157612120612007565b5b602082029050919050565b5f5ffd5b5f61214261213d84612107565b612065565b9050806020840283018581111561215c5761215b61212c565b5b835b81811015612185578061217188826120a2565b84526020840193505060208101905061215e565b5050509392505050565b5f82601f8301126121a3576121a2612103565b5b60026121b0848285612130565b91505092915050565b5f608082840312156121ce576121cd611ff3565b5b6121d86040612065565b90505f6121e78482850161218f565b5f8301525060406121fa8482850161218f565b60208301525092915050565b5f5f5f5f610120858703121561221f5761221e611fb8565b5b5f61222c87828801611fdf565b945050602061223d878288016120b6565b935050606061224e878288016121b9565b92505060e061225f878288016120b6565b91505092959194509250565b5f8115159050919050565b61227f8161226b565b82525050565b5f6040820190506122985f830185612276565b6122a56020830184612276565b9392505050565b6122b58161226b565b81146122bf575f5ffd5b50565b5f813590506122d0816122ac565b92915050565b5f602082840312156122eb576122ea611fb8565b5b5f6122f8848285016122c2565b91505092915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f819050919050565b5f61234361233e61233984612301565b612320565b612301565b9050919050565b5f61235482612329565b9050919050565b5f6123658261234a565b9050919050565b6123758161235b565b82525050565b5f60208201905061238e5f83018461236c565b92915050565b5f61239e8261234a565b9050919050565b6123ae81612394565b82525050565b5f6020820190506123c75f8301846123a5565b92915050565b5f6123d78261234a565b9050919050565b6123e7816123cd565b82525050565b5f6020820190506124005f8301846123de565b92915050565b5f5ffd5b5f5f83601f84011261241f5761241e612103565b5b8235905067ffffffffffffffff81111561243c5761243b612406565b5b6020830191508360018202830111156124585761245761212c565b5b9250929050565b5f63ffffffff82169050919050565b6124778161245f565b8114612481575f5ffd5b50565b5f813590506124928161246e565b92915050565b5f67ffffffffffffffff8211156124b2576124b1612007565b5b602082029050602081019050919050565b5f6124d56124d084612498565b612065565b905080838252602082019050602084028301858111156124f8576124f761212c565b5b835b81811015612521578061250d8882612484565b8452602084019350506020810190506124fa565b5050509392505050565b5f82601f83011261253f5761253e612103565b5b813561254f8482602086016124c3565b91505092915050565b5f67ffffffffffffffff82111561257257612571612007565b5b602082029050602081019050919050565b5f61259561259084612558565b612065565b905080838252602082019050604084028301858111156125b8576125b761212c565b5b835b818110156125e157806125cd88826120b6565b8452602084019350506040810190506125ba565b5050509392505050565b5f82601f8301126125ff576125fe612103565b5b813561260f848260208601612583565b91505092915050565b5f67ffffffffffffffff82111561263257612631612007565b5b602082029050602081019050919050565b5f61265561265084612618565b612065565b905080838252602082019050602084028301858111156126785761267761212c565b5b835b818110156126bf57803567ffffffffffffffff81111561269d5761269c612103565b5b8086016126aa898261252b565b8552602085019450505060208101905061267a565b5050509392505050565b5f82601f8301126126dd576126dc612103565b5b81356126ed848260208601612643565b91505092915050565b5f610180828403121561270c5761270b611ff3565b5b612717610100612065565b90505f82013567ffffffffffffffff8111156127365761273561207f565b5b6127428482850161252b565b5f83015250602082013567ffffffffffffffff8111156127655761276461207f565b5b612771848285016125eb565b602083015250604082013567ffffffffffffffff8111156127955761279461207f565b5b6127a1848285016125eb565b60408301525060606127b5848285016121b9565b60608301525060e06127c9848285016120b6565b60808301525061012082013567ffffffffffffffff8111156127ee576127ed61207f565b5b6127fa8482850161252b565b60a08301525061014082013567ffffffffffffffff81111561281f5761281e61207f565b5b61282b8482850161252b565b60c08301525061016082013567ffffffffffffffff8111156128505761284f61207f565b5b61285c848285016126c9565b60e08301525092915050565b5f5f5f5f5f6080868803121561288157612880611fb8565b5b5f61288e88828901611fdf565b955050602086013567ffffffffffffffff8111156128af576128ae611fbc565b5b6128bb8882890161240a565b945094505060406128ce88828901612484565b925050606086013567ffffffffffffffff8111156128ef576128ee611fbc565b5b6128fb888289016126f6565b9150509295509295909350565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6bffffffffffffffffffffffff82169050919050565b61295181612931565b82525050565b5f6129628383612948565b60208301905092915050565b5f602082019050919050565b5f61298482612908565b61298e8185612912565b935061299983612922565b805f5b838110156129c95781516129b08882612957565b97506129bb8361296e565b92505060018101905061299c565b5085935050505092915050565b5f604083015f8301518482035f8601526129f0828261297a565b91505060208301518482036020860152612a0a828261297a565b9150508091505092915050565b612a2081611fc0565b82525050565b5f6040820190508181035f830152612a3e81856129d6565b9050612a4d6020830184612a17565b9392505050565b5f602082019050612a675f830184612276565b92915050565b5f612a778261234a565b9050919050565b612a8781612a6d565b82525050565b5f602082019050612aa05f830184612a7e565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b612aed612ae882611fc0565b612ad3565b82525050565b5f819050919050565b612b0d612b0882612083565b612af3565b82525050565b5f612b1e828c612adc565b602082019150612b2e828b612afc565b602082019150612b3e828a612afc565b602082019150612b4e8289612afc565b602082019150612b5e8288612afc565b602082019150612b6e8287612afc565b602082019150612b7e8286612afc565b602082019150612b8e8285612afc565b602082019150612b9e8284612afc565b6020820191508190509a9950505050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f612bec82612083565b9150612bf783612083565b925082612c0757612c06612bb5565b5b828206905092915050565b5f612c1c82612301565b9050919050565b612c2c81612c12565b8114612c36575f5ffd5b50565b5f81519050612c4781612c23565b92915050565b5f60208284031215612c6257612c61611fb8565b5b5f612c6f84828501612c39565b91505092915050565b5f82825260208201905092915050565b7f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e615f8201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560208201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000604082015250565b5f612d08605c83612c78565b9150612d1382612c88565b606082019050919050565b5f6020820190508181035f830152612d3581612cfc565b9050919050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a20656d7074792071756f72756d20696e707574000000000000000000602082015250565b5f612d96603783612c78565b9150612da182612d3c565b604082019050919050565b5f6020820190508181035f830152612dc381612d8a565b9050919050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d61746360208201527f6800000000000000000000000000000000000000000000000000000000000000604082015250565b5f612e4a604183612c78565b9150612e5582612dca565b606082019050919050565b5f6020820190508181035f830152612e7781612e3e565b9050919050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d60208201527f6174636800000000000000000000000000000000000000000000000000000000604082015250565b5f612efe604483612c78565b9150612f0982612e7e565b606082019050919050565b5f6020820190508181035f830152612f2b81612ef2565b9050919050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a20696e76616c6964207265666572656e636520626c6f636b00000000602082015250565b5f612f8c603c83612c78565b9150612f9782612f32565b604082019050919050565b5f6020820190508181035f830152612fb981612f80565b9050919050565b5f60ff82169050919050565b612fd581612fc0565b8114612fdf575f5ffd5b50565b5f81519050612ff081612fcc565b92915050565b5f6020828403121561300b5761300a611fb8565b5b5f61301884828501612fe2565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61305882612083565b915061306383612083565b925082820390508181111561307b5761307a613021565b5b92915050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f72746564602082015250565b5f6130db604083612c78565b91506130e682613081565b604082019050919050565b5f6020820190508181035f830152613108816130cf565b9050919050565b6131188161245f565b82525050565b5f61313861313361312e8461245f565b612320565b612083565b9050919050565b6131488161311e565b82525050565b5f6060820190506131615f830186612a17565b61316e602083018561310f565b61317b604083018461313f565b949350505050565b5f77ffffffffffffffffffffffffffffffffffffffffffffffff82169050919050565b6131af81613183565b81146131b9575f5ffd5b50565b5f815190506131ca816131a6565b92915050565b5f602082840312156131e5576131e4611fb8565b5b5f6131f2848285016131bc565b91505092915050565b61320481612fc0565b82525050565b5f60208201905061321d5f8301846131fb565b92915050565b5f815190506132318161208c565b92915050565b5f6020828403121561324c5761324b611fb8565b5b5f61325984828501613223565b91505092915050565b5f61326c82612083565b915061327783612083565b925082820190508082111561328f5761328e613021565b5b92915050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a205374616b6552656769737472792075706461746573206d7573742060208201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060408201527f77696e646f770000000000000000000000000000000000000000000000000000606082015250565b5f61333b606683612c78565b915061334682613295565b608082019050919050565b5f6020820190508181035f8301526133688161332f565b9050919050565b5f6060820190506133825f8301866131fb565b61338f602083018561310f565b61339c604083018461313f565b949350505050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000082169050919050565b6133d8816133a4565b81146133e2575f5ffd5b50565b5f815190506133f3816133cf565b92915050565b5f6020828403121561340e5761340d611fb8565b5b5f61341b848285016133e5565b91505092915050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460208201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d20617060408201527f6b00000000000000000000000000000000000000000000000000000000000000606082015250565b5f6134ca606183612c78565b91506134d582613424565b608082019050919050565b5f6020820190508181035f8301526134f7816134be565b9050919050565b61350781612931565b8114613511575f5ffd5b50565b5f81519050613522816134fe565b92915050565b5f6020828403121561353d5761353c611fb8565b5b5f61354a84828501613514565b91505092915050565b5f6080820190506135665f8301876131fb565b613573602083018661310f565b6135806040830185612a17565b61358d606083018461313f565b95945050505050565b5f6135a082612931565b91506135ab83612931565b925082820390506bffffffffffffffffffffffff8111156135cf576135ce613021565b5b92915050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c2066616960208201527f6c65640000000000000000000000000000000000000000000000000000000000604082015250565b5f613655604383612c78565b9150613660826135d5565b606082019050919050565b5f6020820190508181035f83015261368281613649565b9050919050565b7f424c535369676e6174757265436865636b65722e636865636b5369676e6174755f8201527f7265733a207369676e617475726520697320696e76616c696400000000000000602082015250565b5f6136e3603983612c78565b91506136ee82613689565b604082019050919050565b5f6020820190508181035f830152613710816136d7565b9050919050565b5f8160e01b9050919050565b5f61372d82613717565b9050919050565b6137456137408261245f565b613723565b82525050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b61377781611fc0565b82525050565b5f613788838361376e565b60208301905092915050565b5f602082019050919050565b5f6137aa8261374b565b6137b48185613755565b93506137bf8361375f565b805f5b838110156137ef5781516137d6888261377d565b97506137e183613794565b9250506001810190506137c2565b5085935050505092915050565b5f6138078285613734565b60048201915061381782846137a0565b91508190509392505050565b7f65632d6d756c2d6661696c6564000000000000000000000000000000000000005f82015250565b5f613857600d83612c78565b915061386282613823565b602082019050919050565b5f6020820190508181035f8301526138848161384b565b9050919050565b7f65632d6164642d6661696c6564000000000000000000000000000000000000005f82015250565b5f6138bf600d83612c78565b91506138ca8261388b565b602082019050919050565b5f6020820190508181035f8301526138ec816138b3565b9050919050565b5f6138fd82612083565b915061390883612083565b925082820261391681612083565b9150828204841483151761392d5761392c613021565b5b5092915050565b7f4269746d61705574696c732e6f72646572656442797465734172726179546f425f8201527f69746d61703a206269746d61702065786365656473206d61782076616c756500602082015250565b5f61398e603f83612c78565b915061399982613934565b604082019050919050565b5f6020820190508181035f8301526139bb81613982565b9050919050565b5f61ffff82169050919050565b5f6139d9826139c2565b915061ffff82036139ed576139ec613021565b5b600182019050919050565b7f7363616c61722d746f6f2d6c61726765000000000000000000000000000000005f82015250565b5f613a2c601083612c78565b9150613a37826139f8565b602082019050919050565b5f6020820190508181035f830152613a5981613a20565b9050919050565b7f4269746d61705574696c732e6f72646572656442797465734172726179546f425f8201527f69746d61703a206f7264657265644279746573417272617920697320746f6f2060208201527f6c6f6e6700000000000000000000000000000000000000000000000000000000604082015250565b5f613ae0604483612c78565b9150613aeb82613a60565b606082019050919050565b5f6020820190508181035f830152613b0d81613ad4565b9050919050565b7f4269746d61705574696c732e6f72646572656442797465734172726179546f425f8201527f69746d61703a206f72646572656442797465734172726179206973206e6f742060208201527f6f72646572656400000000000000000000000000000000000000000000000000604082015250565b5f613b94604783612c78565b9150613b9f82613b14565b606082019050919050565b5f6020820190508181035f830152613bc181613b88565b9050919050565b7f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000005f82015250565b5f613bfc601a83612c78565b9150613c0782613bc8565b602082019050919050565b5f6020820190508181035f830152613c2981613bf0565b905091905056fea2646970667358221220da57750388419c15a96c4365ae9aa5daa1f25f5bcde1cad99fb33e93239e026864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x86W_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0YW\x80cm\x14\xA9\x87\x14a\x01\x13W\x80cn\xFBF6\x14a\x011W\x80c\xB9\x8D\t\x08\x14a\x01bW\x80c\xDF\\\xF7#\x14a\x01\x80Wa\0\x86V[\x80c\x17\x1F\x1D[\x14a\0\x8AW\x80cAl~^\x14a\0\xBBW\x80c]\xF4YF\x14a\0\xD7W\x80ch0H5\x14a\0\xF5W[__\xFD[a\0\xA4`\x04\x806\x03\x81\x01\x90a\0\x9F\x91\x90a\"\x06V[a\x01\x9EV[`@Qa\0\xB2\x92\x91\x90a\"\x85V[`@Q\x80\x91\x03\x90\xF3[a\0\xD5`\x04\x806\x03\x81\x01\x90a\0\xD0\x91\x90a\"\xD6V[a\x03\x01V[\0[a\0\xDFa\x04\x07V[`@Qa\0\xEC\x91\x90a#{V[`@Q\x80\x91\x03\x90\xF3[a\0\xFDa\x04+V[`@Qa\x01\n\x91\x90a#\xB4V[`@Q\x80\x91\x03\x90\xF3[a\x01\x1Ba\x04OV[`@Qa\x01(\x91\x90a#\xEDV[`@Q\x80\x91\x03\x90\xF3[a\x01K`\x04\x806\x03\x81\x01\x90a\x01F\x91\x90a(hV[a\x04sV[`@Qa\x01Y\x92\x91\x90a*&V[`@Q\x80\x91\x03\x90\xF3[a\x01ja\x11\xBEV[`@Qa\x01w\x91\x90a*TV[`@Q\x80\x91\x03\x90\xF3[a\x01\x88a\x11\xCFV[`@Qa\x01\x95\x91\x90a*\x8DV[`@Q\x80\x91\x03\x90\xF3[___\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87_\x01Q\x88` \x01Q\x88_\x01Q_`\x02\x81\x10a\x01\xE2Wa\x01\xE1a*\xA6V[[` \x02\x01Q\x89_\x01Q`\x01`\x02\x81\x10a\x01\xFEWa\x01\xFDa*\xA6V[[` \x02\x01Q\x8A` \x01Q_`\x02\x81\x10a\x02\x1AWa\x02\x19a*\xA6V[[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x027Wa\x026a*\xA6V[[` \x02\x01Q\x8B_\x01Q\x8C` \x01Q`@Q` \x01a\x02]\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a+\x13V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x02\x7F\x91\x90a+\xE2V[\x90Pa\x02\xEFa\x02\xA9a\x02\x9A\x83\x89a\x11\xF3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x12\xC7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02\xB1a\x13\xC0V[a\x02\xE5a\x02\xCE\x85a\x02\xC0a\x14\x8AV[a\x11\xF3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x02\xD7\x8Ca\x14\xAEV[a\x12\xC7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88b\x01\xD4\xC0a\x15\xB9V[\x80\x93P\x81\x94PPPP\x94P\x94\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03jW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x8E\x91\x90a,MV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x03\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\xF2\x90a-\x1EV[`@Q\x80\x91\x03\x90\xFD[a\x04\x04\x81a\x18YV[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04{a\x1EpV[__\x86\x86\x90P\x03a\x04\xC1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xB8\x90a-\xACV[`@Q\x80\x91\x03\x90\xFD[\x82`@\x01QQ\x86\x86\x90P\x14\x80\x15a\x04\xDFWP\x82`\xA0\x01QQ\x86\x86\x90P\x14[\x80\x15a\x04\xF2WP\x82`\xC0\x01QQ\x86\x86\x90P\x14[\x80\x15a\x05\x05WP\x82`\xE0\x01QQ\x86\x86\x90P\x14[a\x05DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05;\x90a.`V[`@Q\x80\x91\x03\x90\xFD[\x82_\x01QQ\x83` \x01QQ\x14a\x05\x8FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\x86\x90a/\x14V[`@Q\x80\x91\x03\x90\xFD[Cc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10a\x05\xDDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xD4\x90a/\xA2V[`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x05\xFDa\x1EpV[\x87\x87\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x1AWa\x06\x19a \x07V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06HW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81` \x01\x81\x90RP\x87\x87\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06nWa\x06ma \x07V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\x9CW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81_\x01\x81\x90RPa\x06\xACa\x1E\x8AV[\x85` \x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xCBWa\x06\xCAa \x07V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xF9W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81_\x01\x81\x90RP\x85` \x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07 Wa\x07\x1Fa \x07V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07NW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81` \x01\x81\x90RP_a\x081\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08,\x91\x90a/\xF6V[a\x18\xABV[\x90P__\x90P[\x87` \x01QQ\x81\x10\x15a\n\xB1Wa\x08l\x88` \x01Q\x82\x81Q\x81\x10a\x08_Wa\x08^a*\xA6V[[` \x02` \x01\x01Qa\x19\x0BV[\x83` \x01Q\x82\x81Q\x81\x10a\x08\x83Wa\x08\x82a*\xA6V[[` \x02` \x01\x01\x81\x81RPP_\x81\x14a\t%W\x82` \x01Q`\x01\x82a\x08\xA8\x91\x90a0NV[\x81Q\x81\x10a\x08\xB9Wa\x08\xB8a*\xA6V[[` \x02` \x01\x01Q_\x1C\x83` \x01Q\x82\x81Q\x81\x10a\x08\xDAWa\x08\xD9a*\xA6V[[` \x02` \x01\x01Q_\x1C\x11a\t$W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x1B\x90a0\xF1V[`@Q\x80\x91\x03\x90\xFD[[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x04\xECcQ\x84` \x01Q\x83\x81Q\x81\x10a\txWa\twa*\xA6V[[` \x02` \x01\x01Q\x8B\x8B_\x01Q\x85\x81Q\x81\x10a\t\x97Wa\t\x96a*\xA6V[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xBD\x93\x92\x91\x90a1NV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xFC\x91\x90a1\xD0V[w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83_\x01Q\x82\x81Q\x81\x10a\n,Wa\n+a*\xA6V[[` \x02` \x01\x01\x81\x81RPPa\n\xA2a\n\x93a\nf\x84\x86_\x01Q\x85\x81Q\x81\x10a\nXWa\nWa*\xA6V[[` \x02` \x01\x01Q\x16a\x19#V[\x8A` \x01Q\x84\x81Q\x81\x10a\n}Wa\n|a*\xA6V[[` \x02` \x01\x01Qa\x19^\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x12\xC7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P\x80\x80`\x01\x01\x91PPa\x088V[PPa\n\xBC\x83a\x1ALV[\x92P___\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P__\x90P__\x90P[\x8B\x8B\x90P\x81\x10\x15a\x10\xDBW\x82\x15a\x0B\xF3W\x89c\xFF\xFF\xFF\xFF\x16\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c$\x9A\x0CB\x8F\x8F\x86\x81\x81\x10a\x0B@Wa\x0B?a*\xA6V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Bi\x91\x90a2\nV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xA8\x91\x90a27V[a\x0B\xB2\x91\x90a2bV[\x11a\x0B\xF2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xE9\x90a3QV[`@Q\x80\x91\x03\x90\xFD[[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xBC\xCA\xAC\x8D\x8D\x84\x81\x81\x10a\x0CBWa\x0CAa*\xA6V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xA0\x01Q\x85\x81Q\x81\x10a\x0CgWa\x0Cfa*\xA6V[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x8D\x93\x92\x91\x90a3oV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xCC\x91\x90a3\xF9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16a\x0C\xFE\x8A`@\x01Q\x83\x81Q\x81\x10a\x0C\xF1Wa\x0C\xF0a*\xA6V[[` \x02` \x01\x01Qa\x19\x0BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\rIW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r@\x90a4\xE0V[`@Q\x80\x91\x03\x90\xFD[a\rz\x89`@\x01Q\x82\x81Q\x81\x10a\rcWa\rba*\xA6V[[` \x02` \x01\x01Q\x87a\x12\xC7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC8)LV\x8D\x8D\x84\x81\x81\x10a\r\xCBWa\r\xCAa*\xA6V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xC0\x01Q\x85\x81Q\x81\x10a\r\xF0Wa\r\xEFa*\xA6V[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x16\x93\x92\x91\x90a3oV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EU\x91\x90a5(V[\x85` \x01Q\x82\x81Q\x81\x10a\x0ElWa\x0Eka*\xA6V[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x84` \x01Q\x81\x81Q\x81\x10a\x0E\xADWa\x0E\xACa*\xA6V[[` \x02` \x01\x01Q\x85_\x01Q\x82\x81Q\x81\x10a\x0E\xCBWa\x0E\xCAa*\xA6V[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP__\x90P__\x90P[\x8A` \x01QQ\x81\x10\x15a\x10\xCCWa\x0FQ\x86_\x01Q\x82\x81Q\x81\x10a\x0F$Wa\x0F#a*\xA6V[[` \x02` \x01\x01Q\x8F\x8F\x86\x81\x81\x10a\x0F?Wa\x0F>a*\xA6V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1Ca\x1B\x04V[\x15a\x10\xBFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF2\xBE\x94\xAE\x8F\x8F\x86\x81\x81\x10a\x0F\xA5Wa\x0F\xA4a*\xA6V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8E\x89` \x01Q\x85\x81Q\x81\x10a\x0F\xCAWa\x0F\xC9a*\xA6V[[` \x02` \x01\x01Q\x8F`\xE0\x01Q\x88\x81Q\x81\x10a\x0F\xE9Wa\x0F\xE8a*\xA6V[[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x10\x03Wa\x10\x02a*\xA6V[[` \x02` \x01\x01Q`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10*\x94\x93\x92\x91\x90a5SV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10i\x91\x90a5(V[\x87_\x01Q\x84\x81Q\x81\x10a\x10\x7FWa\x10~a*\xA6V[[` \x02` \x01\x01\x81\x81Qa\x10\x93\x91\x90a5\x96V[\x91P\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81`\x01\x01\x91P[\x80\x80`\x01\x01\x91PPa\x0E\xFEV[PP\x80\x80`\x01\x01\x91PPa\n\xD8V[PPP__a\x10\xF4\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x01\x9EV[\x91P\x91P\x81a\x118W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11/\x90a6kV[`@Q\x80\x91\x03\x90\xFD[\x80a\x11xW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11o\x90a6\xF9V[`@Q\x80\x91\x03\x90\xFD[PP_\x87\x82` \x01Q`@Q` \x01a\x11\x92\x92\x91\x90a7\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x82\x81\x95P\x95PPPPP\x95P\x95\x93PPPPV[__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x11\xFBa\x1E\xA4V[a\x12\x03a\x1E\xBCV[\x83_\x01Q\x81_`\x03\x81\x10a\x12\x1AWa\x12\x19a*\xA6V[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x03\x81\x10a\x12<Wa\x12;a*\xA6V[[` \x02\x01\x81\x81RPP\x82\x81`\x02`\x03\x81\x10a\x12ZWa\x12Ya*\xA6V[[` \x02\x01\x81\x81RPP_`@\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x12~W\xFE[P\x80a\x12\xBFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xB6\x90a8mV[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x12\xCFa\x1E\xA4V[a\x12\xD7a\x1E\xDEV[\x83_\x01Q\x81_`\x04\x81\x10a\x12\xEEWa\x12\xEDa*\xA6V[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x04\x81\x10a\x13\x10Wa\x13\x0Fa*\xA6V[[` \x02\x01\x81\x81RPP\x82_\x01Q\x81`\x02`\x04\x81\x10a\x131Wa\x130a*\xA6V[[` \x02\x01\x81\x81RPP\x82` \x01Q\x81`\x03`\x04\x81\x10a\x13SWa\x13Ra*\xA6V[[` \x02\x01\x81\x81RPP_`@\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x13wW\xFE[P\x80a\x13\xB8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\xAF\x90a8\xD5V[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x13\xC8a\x1F\0V[`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x81R` \x01\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D\x81RP\x81RP\x90P\x90V[a\x14\x92a\x1E\xA4V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x02\x81RP\x90P\x90V[a\x14\xB6a\x1E\xA4V[__\x90P__\x90P_\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x85_\x1Ca\x14\xED\x91\x90a+\xE2V[\x90P[`\x01\x15a\x15\x99Wa\x15\0\x81a\x1B\x1AV[\x80\x93P\x81\x94PPP\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x157Wa\x156a+\xB5V[[\x82\x83\t\x83\x03a\x15_W`@Q\x80`@\x01`@R\x80\x82\x81R` \x01\x83\x81RP\x93PPPPa\x15\xB4V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x15\x8EWa\x15\x8Da+\xB5V[[`\x01\x82\x08\x90Pa\x14\xF0V[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x93PPPP[\x91\x90PV[___`@Q\x80`@\x01`@R\x80\x89\x81R` \x01\x87\x81RP\x90P_`@Q\x80`@\x01`@R\x80\x89\x81R` \x01\x87\x81RP\x90Pa\x15\xF3a\x1F&V[__\x90P[`\x02\x81\x10\x15a\x18\x11W_`\x06\x82a\x16\x0F\x91\x90a8\xF3V[\x90P\x84\x82`\x02\x81\x10a\x16$Wa\x16#a*\xA6V[[` \x02\x01Q_\x01Q\x83_\x83a\x169\x91\x90a2bV[`\x0C\x81\x10a\x16JWa\x16Ia*\xA6V[[` \x02\x01\x81\x81RPP\x84\x82`\x02\x81\x10a\x16fWa\x16ea*\xA6V[[` \x02\x01Q` \x01Q\x83`\x01\x83a\x16}\x91\x90a2bV[`\x0C\x81\x10a\x16\x8EWa\x16\x8Da*\xA6V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x16\xAAWa\x16\xA9a*\xA6V[[` \x02\x01Q_\x01Q_`\x02\x81\x10a\x16\xC4Wa\x16\xC3a*\xA6V[[` \x02\x01Q\x83`\x02\x83a\x16\xD7\x91\x90a2bV[`\x0C\x81\x10a\x16\xE8Wa\x16\xE7a*\xA6V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x17\x04Wa\x17\x03a*\xA6V[[` \x02\x01Q_\x01Q`\x01`\x02\x81\x10a\x17\x1FWa\x17\x1Ea*\xA6V[[` \x02\x01Q\x83`\x03\x83a\x172\x91\x90a2bV[`\x0C\x81\x10a\x17CWa\x17Ba*\xA6V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x17_Wa\x17^a*\xA6V[[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x17zWa\x17ya*\xA6V[[` \x02\x01Q\x83`\x04\x83a\x17\x8D\x91\x90a2bV[`\x0C\x81\x10a\x17\x9EWa\x17\x9Da*\xA6V[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x17\xBAWa\x17\xB9a*\xA6V[[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x17\xD6Wa\x17\xD5a*\xA6V[[` \x02\x01Q\x83`\x05\x83a\x17\xE9\x91\x90a2bV[`\x0C\x81\x10a\x17\xFAWa\x17\xF9a*\xA6V[[` \x02\x01\x81\x81RPPP\x80\x80`\x01\x01\x91PPa\x15\xF8V[Pa\x18\x1Aa\x1FIV[_` \x82` `\x0C\x02\x85`\x08\x8C\xFA\x90P\x80_\x83_`\x01\x81\x10a\x18?Wa\x18>a*\xA6V[[` \x02\x01Q\x14\x15\x96P\x96PPPPPP\x95P\x95\x93PPPPV[\x80__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x81`@Qa\x18\xA0\x91\x90a*TV[`@Q\x80\x91\x03\x90\xA1PV[__a\x18\xB6\x84a\x1C\x0FV[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x19\x01W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xF8\x90a9\xA4V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[_\x81Q_R\x81` \x01Q` R`@_ \x90P\x91\x90PV[___\x90P[_\x83\x11\x15a\x19UW`\x01\x83a\x19>\x91\x90a0NV[\x83\x16\x92P\x80\x80a\x19M\x90a9\xCFV[\x91PPa\x19)V[\x80\x91PP\x91\x90PV[a\x19fa\x1E\xA4V[a\x02\0\x82a\xFF\xFF\x16\x10a\x19\xAEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xA5\x90a:BV[`@Q\x80\x91\x03\x90\xFD[`\x01\x82a\xFF\xFF\x16\x03a\x19\xC2W\x82\x90Pa\x1AFV[_`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90P_\x84\x90P_`\x01\x90P__\x90P[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a\x1A>W`\x01\x80\x82`\xFF\x16\x88a\xFF\xFF\x16\x90\x1C\x16a\xFF\xFF\x16\x03a\x1A\x1CWa\x1A\x19\x84\x84a\x12\xC7V[\x93P[a\x1A&\x83\x84a\x12\xC7V[\x92P`\x01\x82a\xFF\xFF\x16\x90\x1B\x91P\x80`\x01\x01\x90Pa\x19\xE8V[\x83\x94PPPPP[\x92\x91PPV[a\x1ATa\x1E\xA4V[_\x82_\x01Q\x14\x80\x15a\x1AiWP_\x82` \x01Q\x14[\x15a\x1A\x8AW`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x1A\xFFV[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x1A\xCE\x91\x90a+\xE2V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1A\xF9\x91\x90a0NV[\x81RP\x90P[\x91\x90PV[_`\x01\x82`\xFF\x16\x84\x90\x1C\x16`\x01\x14\x90P\x92\x91PPV[___\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1BLWa\x1BKa+\xB5V[[`\x03\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1B}Wa\x1B|a+\xB5V[[\x86\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1B\xADWa\x1B\xACa+\xB5V[[\x88\x89\t\t\x08\x90P_a\x1C\0\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1D)V[\x90P\x81\x81\x93P\x93PPP\x91P\x91V[_a\x01\0\x82Q\x11\x15a\x1CVW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1CM\x90a:\xF6V[`@Q\x80\x91\x03\x90\xFD[_\x82Q\x03a\x1CfW_\x90Pa\x1D$V[__\x83_\x81Q\x81\x10a\x1C{Wa\x1Cza*\xA6V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16`\x01\x90\x1B\x91P_`\x01\x90P[\x84Q\x81\x10\x15a\x1D\x1DW\x84\x81\x81Q\x81\x10a\x1C\xB4Wa\x1C\xB3a*\xA6V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16`\x01\x90\x1B\x91P\x82\x82\x11a\x1D\rW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\x04\x90a;\xAAV[`@Q\x80\x91\x03\x90\xFD[\x81\x83\x17\x92P\x80`\x01\x01\x90Pa\x1C\x98V[P\x81\x92PPP[\x91\x90PV[__a\x1D3a\x1FIV[a\x1D;a\x1FkV[` \x81_`\x06\x81\x10a\x1DPWa\x1DOa*\xA6V[[` \x02\x01\x81\x81RPP` \x81`\x01`\x06\x81\x10a\x1DoWa\x1Dna*\xA6V[[` \x02\x01\x81\x81RPP` \x81`\x02`\x06\x81\x10a\x1D\x8EWa\x1D\x8Da*\xA6V[[` \x02\x01\x81\x81RPP\x86\x81`\x03`\x06\x81\x10a\x1D\xACWa\x1D\xABa*\xA6V[[` \x02\x01\x81\x81RPP\x85\x81`\x04`\x06\x81\x10a\x1D\xCAWa\x1D\xC9a*\xA6V[[` \x02\x01\x81\x81RPP\x84\x81`\x05`\x06\x81\x10a\x1D\xE8Wa\x1D\xE7a*\xA6V[[` \x02\x01\x81\x81RPP` \x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82_\x81\x03a\x1E\x0BW\xFE[P\x82a\x1ELW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1EC\x90a<\x12V[`@Q\x80\x91\x03\x90\xFD[\x81_`\x01\x81\x10a\x1E_Wa\x1E^a*\xA6V[[` \x02\x01Q\x93PPPP\x93\x92PPPV[`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80a\x1F\x13a\x1F\x8DV[\x81R` \x01a\x1F a\x1F\x8DV[\x81RP\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x1F\xD2\x81a\x1F\xC0V[\x81\x14a\x1F\xDCW__\xFD[PV[_\x815\x90Pa\x1F\xED\x81a\x1F\xC9V[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a =\x82a\x1F\xF7V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a \\Wa [a \x07V[[\x80`@RPPPV[_a na\x1F\xAFV[\x90Pa z\x82\x82a 4V[\x91\x90PV[__\xFD[_\x81\x90P\x91\x90PV[a \x95\x81a \x83V[\x81\x14a \x9FW__\xFD[PV[_\x815\x90Pa \xB0\x81a \x8CV[\x92\x91PPV[_`@\x82\x84\x03\x12\x15a \xCBWa \xCAa\x1F\xF3V[[a \xD5`@a eV[\x90P_a \xE4\x84\x82\x85\x01a \xA2V[_\x83\x01RP` a \xF7\x84\x82\x85\x01a \xA2V[` \x83\x01RP\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a!!Wa! a \x07V[[` \x82\x02\x90P\x91\x90PV[__\xFD[_a!Ba!=\x84a!\x07V[a eV[\x90P\x80` \x84\x02\x83\x01\x85\x81\x11\x15a!\\Wa![a!,V[[\x83[\x81\x81\x10\x15a!\x85W\x80a!q\x88\x82a \xA2V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa!^V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a!\xA3Wa!\xA2a!\x03V[[`\x02a!\xB0\x84\x82\x85a!0V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a!\xCEWa!\xCDa\x1F\xF3V[[a!\xD8`@a eV[\x90P_a!\xE7\x84\x82\x85\x01a!\x8FV[_\x83\x01RP`@a!\xFA\x84\x82\x85\x01a!\x8FV[` \x83\x01RP\x92\x91PPV[____a\x01 \x85\x87\x03\x12\x15a\"\x1FWa\"\x1Ea\x1F\xB8V[[_a\",\x87\x82\x88\x01a\x1F\xDFV[\x94PP` a\"=\x87\x82\x88\x01a \xB6V[\x93PP``a\"N\x87\x82\x88\x01a!\xB9V[\x92PP`\xE0a\"_\x87\x82\x88\x01a \xB6V[\x91PP\x92\x95\x91\x94P\x92PV[_\x81\x15\x15\x90P\x91\x90PV[a\"\x7F\x81a\"kV[\x82RPPV[_`@\x82\x01\x90Pa\"\x98_\x83\x01\x85a\"vV[a\"\xA5` \x83\x01\x84a\"vV[\x93\x92PPPV[a\"\xB5\x81a\"kV[\x81\x14a\"\xBFW__\xFD[PV[_\x815\x90Pa\"\xD0\x81a\"\xACV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\"\xEBWa\"\xEAa\x1F\xB8V[[_a\"\xF8\x84\x82\x85\x01a\"\xC2V[\x91PP\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a#Ca#>a#9\x84a#\x01V[a# V[a#\x01V[\x90P\x91\x90PV[_a#T\x82a#)V[\x90P\x91\x90PV[_a#e\x82a#JV[\x90P\x91\x90PV[a#u\x81a#[V[\x82RPPV[_` \x82\x01\x90Pa#\x8E_\x83\x01\x84a#lV[\x92\x91PPV[_a#\x9E\x82a#JV[\x90P\x91\x90PV[a#\xAE\x81a#\x94V[\x82RPPV[_` \x82\x01\x90Pa#\xC7_\x83\x01\x84a#\xA5V[\x92\x91PPV[_a#\xD7\x82a#JV[\x90P\x91\x90PV[a#\xE7\x81a#\xCDV[\x82RPPV[_` \x82\x01\x90Pa$\0_\x83\x01\x84a#\xDEV[\x92\x91PPV[__\xFD[__\x83`\x1F\x84\x01\x12a$\x1FWa$\x1Ea!\x03V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$<Wa$;a$\x06V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a$XWa$Wa!,V[[\x92P\x92\x90PV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a$w\x81a$_V[\x81\x14a$\x81W__\xFD[PV[_\x815\x90Pa$\x92\x81a$nV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a$\xB2Wa$\xB1a \x07V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a$\xD5a$\xD0\x84a$\x98V[a eV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a$\xF8Wa$\xF7a!,V[[\x83[\x81\x81\x10\x15a%!W\x80a%\r\x88\x82a$\x84V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa$\xFAV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a%?Wa%>a!\x03V[[\x815a%O\x84\x82` \x86\x01a$\xC3V[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%rWa%qa \x07V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a%\x95a%\x90\x84a%XV[a eV[\x90P\x80\x83\x82R` \x82\x01\x90P`@\x84\x02\x83\x01\x85\x81\x11\x15a%\xB8Wa%\xB7a!,V[[\x83[\x81\x81\x10\x15a%\xE1W\x80a%\xCD\x88\x82a \xB6V[\x84R` \x84\x01\x93PP`@\x81\x01\x90Pa%\xBAV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a%\xFFWa%\xFEa!\x03V[[\x815a&\x0F\x84\x82` \x86\x01a%\x83V[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&2Wa&1a \x07V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a&Ua&P\x84a&\x18V[a eV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a&xWa&wa!,V[[\x83[\x81\x81\x10\x15a&\xBFW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\x9DWa&\x9Ca!\x03V[[\x80\x86\x01a&\xAA\x89\x82a%+V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa&zV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a&\xDDWa&\xDCa!\x03V[[\x815a&\xED\x84\x82` \x86\x01a&CV[\x91PP\x92\x91PPV[_a\x01\x80\x82\x84\x03\x12\x15a'\x0CWa'\x0Ba\x1F\xF3V[[a'\x17a\x01\0a eV[\x90P_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'6Wa'5a \x7FV[[a'B\x84\x82\x85\x01a%+V[_\x83\x01RP` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'eWa'da \x7FV[[a'q\x84\x82\x85\x01a%\xEBV[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\x95Wa'\x94a \x7FV[[a'\xA1\x84\x82\x85\x01a%\xEBV[`@\x83\x01RP``a'\xB5\x84\x82\x85\x01a!\xB9V[``\x83\x01RP`\xE0a'\xC9\x84\x82\x85\x01a \xB6V[`\x80\x83\x01RPa\x01 \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xEEWa'\xEDa \x7FV[[a'\xFA\x84\x82\x85\x01a%+V[`\xA0\x83\x01RPa\x01@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x1FWa(\x1Ea \x7FV[[a(+\x84\x82\x85\x01a%+V[`\xC0\x83\x01RPa\x01`\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(PWa(Oa \x7FV[[a(\\\x84\x82\x85\x01a&\xC9V[`\xE0\x83\x01RP\x92\x91PPV[_____`\x80\x86\x88\x03\x12\x15a(\x81Wa(\x80a\x1F\xB8V[[_a(\x8E\x88\x82\x89\x01a\x1F\xDFV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xAFWa(\xAEa\x1F\xBCV[[a(\xBB\x88\x82\x89\x01a$\nV[\x94P\x94PP`@a(\xCE\x88\x82\x89\x01a$\x84V[\x92PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xEFWa(\xEEa\x1F\xBCV[[a(\xFB\x88\x82\x89\x01a&\xF6V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a)Q\x81a)1V[\x82RPPV[_a)b\x83\x83a)HV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a)\x84\x82a)\x08V[a)\x8E\x81\x85a)\x12V[\x93Pa)\x99\x83a)\"V[\x80_[\x83\x81\x10\x15a)\xC9W\x81Qa)\xB0\x88\x82a)WV[\x97Pa)\xBB\x83a)nV[\x92PP`\x01\x81\x01\x90Pa)\x9CV[P\x85\x93PPPP\x92\x91PPV[_`@\x83\x01_\x83\x01Q\x84\x82\x03_\x86\x01Ra)\xF0\x82\x82a)zV[\x91PP` \x83\x01Q\x84\x82\x03` \x86\x01Ra*\n\x82\x82a)zV[\x91PP\x80\x91PP\x92\x91PPV[a* \x81a\x1F\xC0V[\x82RPPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra*>\x81\x85a)\xD6V[\x90Pa*M` \x83\x01\x84a*\x17V[\x93\x92PPPV[_` \x82\x01\x90Pa*g_\x83\x01\x84a\"vV[\x92\x91PPV[_a*w\x82a#JV[\x90P\x91\x90PV[a*\x87\x81a*mV[\x82RPPV[_` \x82\x01\x90Pa*\xA0_\x83\x01\x84a*~V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a*\xEDa*\xE8\x82a\x1F\xC0V[a*\xD3V[\x82RPPV[_\x81\x90P\x91\x90PV[a+\ra+\x08\x82a \x83V[a*\xF3V[\x82RPPV[_a+\x1E\x82\x8Ca*\xDCV[` \x82\x01\x91Pa+.\x82\x8Ba*\xFCV[` \x82\x01\x91Pa+>\x82\x8Aa*\xFCV[` \x82\x01\x91Pa+N\x82\x89a*\xFCV[` \x82\x01\x91Pa+^\x82\x88a*\xFCV[` \x82\x01\x91Pa+n\x82\x87a*\xFCV[` \x82\x01\x91Pa+~\x82\x86a*\xFCV[` \x82\x01\x91Pa+\x8E\x82\x85a*\xFCV[` \x82\x01\x91Pa+\x9E\x82\x84a*\xFCV[` \x82\x01\x91P\x81\x90P\x9A\x99PPPPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a+\xEC\x82a \x83V[\x91Pa+\xF7\x83a \x83V[\x92P\x82a,\x07Wa,\x06a+\xB5V[[\x82\x82\x06\x90P\x92\x91PPV[_a,\x1C\x82a#\x01V[\x90P\x91\x90PV[a,,\x81a,\x12V[\x81\x14a,6W__\xFD[PV[_\x81Q\x90Pa,G\x81a,#V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a,bWa,aa\x1F\xB8V[[_a,o\x84\x82\x85\x01a,9V[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBLSSignatureChecker.onlyCoordina_\x82\x01R\x7FtorOwner: caller is not the owne` \x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`@\x82\x01RPV[_a-\x08`\\\x83a,xV[\x91Pa-\x13\x82a,\x88V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra-5\x81a,\xFCV[\x90P\x91\x90PV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a-\x96`7\x83a,xV[\x91Pa-\xA1\x82a-<V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra-\xC3\x81a-\x8AV[\x90P\x91\x90PV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: input quorum length mismatc` \x82\x01R\x7Fh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a.J`A\x83a,xV[\x91Pa.U\x82a-\xCAV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra.w\x81a.>V[\x90P\x91\x90PV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: input nonsigner length mism` \x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a.\xFE`D\x83a,xV[\x91Pa/\t\x82a.~V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra/+\x81a.\xF2V[\x90P\x91\x90PV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: invalid reference block\0\0\0\0` \x82\x01RPV[_a/\x8C`<\x83a,xV[\x91Pa/\x97\x82a/2V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra/\xB9\x81a/\x80V[\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[a/\xD5\x81a/\xC0V[\x81\x14a/\xDFW__\xFD[PV[_\x81Q\x90Pa/\xF0\x81a/\xCCV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a0\x0BWa0\na\x1F\xB8V[[_a0\x18\x84\x82\x85\x01a/\xE2V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a0X\x82a \x83V[\x91Pa0c\x83a \x83V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a0{Wa0za0!V[[\x92\x91PPV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: nonSignerPubkeys not sorted` \x82\x01RPV[_a0\xDB`@\x83a,xV[\x91Pa0\xE6\x82a0\x81V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra1\x08\x81a0\xCFV[\x90P\x91\x90PV[a1\x18\x81a$_V[\x82RPPV[_a18a13a1.\x84a$_V[a# V[a \x83V[\x90P\x91\x90PV[a1H\x81a1\x1EV[\x82RPPV[_``\x82\x01\x90Pa1a_\x83\x01\x86a*\x17V[a1n` \x83\x01\x85a1\x0FV[a1{`@\x83\x01\x84a1?V[\x94\x93PPPPV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a1\xAF\x81a1\x83V[\x81\x14a1\xB9W__\xFD[PV[_\x81Q\x90Pa1\xCA\x81a1\xA6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a1\xE5Wa1\xE4a\x1F\xB8V[[_a1\xF2\x84\x82\x85\x01a1\xBCV[\x91PP\x92\x91PPV[a2\x04\x81a/\xC0V[\x82RPPV[_` \x82\x01\x90Pa2\x1D_\x83\x01\x84a1\xFBV[\x92\x91PPV[_\x81Q\x90Pa21\x81a \x8CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a2LWa2Ka\x1F\xB8V[[_a2Y\x84\x82\x85\x01a2#V[\x91PP\x92\x91PPV[_a2l\x82a \x83V[\x91Pa2w\x83a \x83V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a2\x8FWa2\x8Ea0!V[[\x92\x91PPV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: StakeRegistry updates must ` \x82\x01R\x7Fbe within withdrawalDelayBlocks `@\x82\x01R\x7Fwindow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_a3;`f\x83a,xV[\x91Pa3F\x82a2\x95V[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3h\x81a3/V[\x90P\x91\x90PV[_``\x82\x01\x90Pa3\x82_\x83\x01\x86a1\xFBV[a3\x8F` \x83\x01\x85a1\x0FV[a3\x9C`@\x83\x01\x84a1?V[\x94\x93PPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a3\xD8\x81a3\xA4V[\x81\x14a3\xE2W__\xFD[PV[_\x81Q\x90Pa3\xF3\x81a3\xCFV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a4\x0EWa4\ra\x1F\xB8V[[_a4\x1B\x84\x82\x85\x01a3\xE5V[\x91PP\x92\x91PPV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: quorumApk hash in storage d` \x82\x01R\x7Foes not match provided quorum ap`@\x82\x01R\x7Fk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_a4\xCA`a\x83a,xV[\x91Pa4\xD5\x82a4$V[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra4\xF7\x81a4\xBEV[\x90P\x91\x90PV[a5\x07\x81a)1V[\x81\x14a5\x11W__\xFD[PV[_\x81Q\x90Pa5\"\x81a4\xFEV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a5=Wa5<a\x1F\xB8V[[_a5J\x84\x82\x85\x01a5\x14V[\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pa5f_\x83\x01\x87a1\xFBV[a5s` \x83\x01\x86a1\x0FV[a5\x80`@\x83\x01\x85a*\x17V[a5\x8D``\x83\x01\x84a1?V[\x95\x94PPPPPV[_a5\xA0\x82a)1V[\x91Pa5\xAB\x83a)1V[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\xCFWa5\xCEa0!V[[\x92\x91PPV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: pairing precompile call fai` \x82\x01R\x7Fled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a6U`C\x83a,xV[\x91Pa6`\x82a5\xD5V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra6\x82\x81a6IV[\x90P\x91\x90PV[\x7FBLSSignatureChecker.checkSignatu_\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0` \x82\x01RPV[_a6\xE3`9\x83a,xV[\x91Pa6\xEE\x82a6\x89V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra7\x10\x81a6\xD7V[\x90P\x91\x90PV[_\x81`\xE0\x1B\x90P\x91\x90PV[_a7-\x82a7\x17V[\x90P\x91\x90PV[a7Ea7@\x82a$_V[a7#V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a7w\x81a\x1F\xC0V[\x82RPPV[_a7\x88\x83\x83a7nV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a7\xAA\x82a7KV[a7\xB4\x81\x85a7UV[\x93Pa7\xBF\x83a7_V[\x80_[\x83\x81\x10\x15a7\xEFW\x81Qa7\xD6\x88\x82a7}V[\x97Pa7\xE1\x83a7\x94V[\x92PP`\x01\x81\x01\x90Pa7\xC2V[P\x85\x93PPPP\x92\x91PPV[_a8\x07\x82\x85a74V[`\x04\x82\x01\x91Pa8\x17\x82\x84a7\xA0V[\x91P\x81\x90P\x93\x92PPPV[\x7Fec-mul-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a8W`\r\x83a,xV[\x91Pa8b\x82a8#V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8\x84\x81a8KV[\x90P\x91\x90PV[\x7Fec-add-failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a8\xBF`\r\x83a,xV[\x91Pa8\xCA\x82a8\x8BV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8\xEC\x81a8\xB3V[\x90P\x91\x90PV[_a8\xFD\x82a \x83V[\x91Pa9\x08\x83a \x83V[\x92P\x82\x82\x02a9\x16\x81a \x83V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a9-Wa9,a0!V[[P\x92\x91PPV[\x7FBitmapUtils.orderedBytesArrayToB_\x82\x01R\x7Fitmap: bitmap exceeds max value\0` \x82\x01RPV[_a9\x8E`?\x83a,xV[\x91Pa9\x99\x82a94V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\xBB\x81a9\x82V[\x90P\x91\x90PV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[_a9\xD9\x82a9\xC2V[\x91Pa\xFF\xFF\x82\x03a9\xEDWa9\xECa0!V[[`\x01\x82\x01\x90P\x91\x90PV[\x7Fscalar-too-large\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a:,`\x10\x83a,xV[\x91Pa:7\x82a9\xF8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra:Y\x81a: V[\x90P\x91\x90PV[\x7FBitmapUtils.orderedBytesArrayToB_\x82\x01R\x7Fitmap: orderedBytesArray is too ` \x82\x01R\x7Flong\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a:\xE0`D\x83a,xV[\x91Pa:\xEB\x82a:`V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\r\x81a:\xD4V[\x90P\x91\x90PV[\x7FBitmapUtils.orderedBytesArrayToB_\x82\x01R\x7Fitmap: orderedBytesArray is not ` \x82\x01R\x7Fordered\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a;\x94`G\x83a,xV[\x91Pa;\x9F\x82a;\x14V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\xC1\x81a;\x88V[\x90P\x91\x90PV[\x7FBN254.expMod: call failure\0\0\0\0\0\0_\x82\x01RPV[_a;\xFC`\x1A\x83a,xV[\x91Pa<\x07\x82a;\xC8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<)\x81a;\xF0V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xDAWu\x03\x88A\x9C\x15\xA9lCe\xAE\x9A\xA5\xDA\xA1\xF2_[\xCD\xE1\xCA\xD9\x9F\xB3>\x93#\x9E\x02hdsolcC\0\x08\x1B\x003",
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
