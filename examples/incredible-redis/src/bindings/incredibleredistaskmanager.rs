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
        #[allow(missing_docs)]
        pub X: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub X: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        #[allow(missing_docs)]
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
library IBLSSignatureCheckerTypes {
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IBLSSignatureCheckerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerStakesAndSignature {
        #[allow(missing_docs)]
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerPubkeys: alloy::sol_types::private::Vec<
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub quorumApks: alloy::sol_types::private::Vec<
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub signedStakeForQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
        >,
        #[allow(missing_docs)]
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
    /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBLSSignatureCheckerTypesInstance<T, P, N> {
        IBLSSignatureCheckerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IBLSSignatureCheckerTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IBLSSignatureCheckerTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBLSSignatureCheckerTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBLSSignatureCheckerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBLSSignatureCheckerTypesInstance")
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
    > IBLSSignatureCheckerTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IBLSSignatureCheckerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBLSSignatureCheckerTypesInstance<T, P, N> {
            IBLSSignatureCheckerTypesInstance {
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
    > IBLSSignatureCheckerTypesInstance<T, P, N> {
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
    > IBLSSignatureCheckerTypesInstance<T, P, N> {
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
library IIncredibleRedisTaskManager {
    struct SetInput { string key; string value; }
    struct Task { SetInput input; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
    struct TaskResponse { uint32 referenceTaskIndex; bytes32 result; }
    struct TaskResponseMetadata { uint32 taskRespondedBlock; bytes32 hashOfNonSigners; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IIncredibleRedisTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SetInput { string key; string value; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SetInput {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<SetInput> for UnderlyingRustTuple<'_> {
            fn from(value: SetInput) -> Self {
                (value.key, value.value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SetInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    key: tuple.0,
                    value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SetInput {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SetInput {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.value,
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
        impl alloy_sol_types::SolType for SetInput {
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
        impl alloy_sol_types::SolStruct for SetInput {
            const NAME: &'static str = "SetInput";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SetInput(string key,string value)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.key,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.value,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SetInput {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.key,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.value,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.key,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.value,
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
struct Task { SetInput input; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Task {
        #[allow(missing_docs)]
        pub input: <SetInput as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskCreatedBlock: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub quorumThresholdPercentage: u32,
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
            SetInput,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <SetInput as alloy::sol_types::SolType>::RustType,
            u32,
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<Task> for UnderlyingRustTuple<'_> {
            fn from(value: Task) -> Self {
                (
                    value.input,
                    value.taskCreatedBlock,
                    value.quorumNumbers,
                    value.quorumThresholdPercentage,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Task {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    input: tuple.0,
                    taskCreatedBlock: tuple.1,
                    quorumNumbers: tuple.2,
                    quorumThresholdPercentage: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Task {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Task {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <SetInput as alloy_sol_types::SolType>::tokenize(&self.input),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.taskCreatedBlock),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.quorumThresholdPercentage,
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
        impl alloy_sol_types::SolType for Task {
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
        impl alloy_sol_types::SolStruct for Task {
            const NAME: &'static str = "Task";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Task(SetInput input,uint32 taskCreatedBlock,bytes quorumNumbers,uint32 quorumThresholdPercentage)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<SetInput as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <SetInput as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <SetInput as alloy_sol_types::SolType>::eip712_data_word(&self.input)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.taskCreatedBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumNumbers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumThresholdPercentage,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Task {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <SetInput as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.input,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.taskCreatedBlock,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumNumbers,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumThresholdPercentage,
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
                <SetInput as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.input,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.taskCreatedBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumNumbers,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumThresholdPercentage,
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
struct TaskResponse { uint32 referenceTaskIndex; bytes32 result; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TaskResponse {
        #[allow(missing_docs)]
        pub referenceTaskIndex: u32,
        #[allow(missing_docs)]
        pub result: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::FixedBytes<32>);
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
        impl ::core::convert::From<TaskResponse> for UnderlyingRustTuple<'_> {
            fn from(value: TaskResponse) -> Self {
                (value.referenceTaskIndex, value.result)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskResponse {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    referenceTaskIndex: tuple.0,
                    result: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TaskResponse {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TaskResponse {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTaskIndex),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.result),
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
        impl alloy_sol_types::SolType for TaskResponse {
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
        impl alloy_sol_types::SolStruct for TaskResponse {
            const NAME: &'static str = "TaskResponse";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TaskResponse(uint32 referenceTaskIndex,bytes32 result)",
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.referenceTaskIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.result)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TaskResponse {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.referenceTaskIndex,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.result,
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
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.referenceTaskIndex,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.result,
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
struct TaskResponseMetadata { uint32 taskRespondedBlock; bytes32 hashOfNonSigners; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TaskResponseMetadata {
        #[allow(missing_docs)]
        pub taskRespondedBlock: u32,
        #[allow(missing_docs)]
        pub hashOfNonSigners: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::FixedBytes<32>);
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
        impl ::core::convert::From<TaskResponseMetadata> for UnderlyingRustTuple<'_> {
            fn from(value: TaskResponseMetadata) -> Self {
                (value.taskRespondedBlock, value.hashOfNonSigners)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskResponseMetadata {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    taskRespondedBlock: tuple.0,
                    hashOfNonSigners: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TaskResponseMetadata {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TaskResponseMetadata {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.taskRespondedBlock),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hashOfNonSigners),
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
        impl alloy_sol_types::SolType for TaskResponseMetadata {
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
        impl alloy_sol_types::SolStruct for TaskResponseMetadata {
            const NAME: &'static str = "TaskResponseMetadata";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TaskResponseMetadata(uint32 taskRespondedBlock,bytes32 hashOfNonSigners)",
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.taskRespondedBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hashOfNonSigners,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TaskResponseMetadata {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.taskRespondedBlock,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hashOfNonSigners,
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
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.taskRespondedBlock,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hashOfNonSigners,
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
    /**Creates a new wrapper around an on-chain [`IIncredibleRedisTaskManager`](self) contract instance.

See the [wrapper's documentation](`IIncredibleRedisTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IIncredibleRedisTaskManagerInstance<T, P, N> {
        IIncredibleRedisTaskManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IIncredibleRedisTaskManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IIncredibleRedisTaskManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IIncredibleRedisTaskManagerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IIncredibleRedisTaskManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IIncredibleRedisTaskManagerInstance")
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
    > IIncredibleRedisTaskManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IIncredibleRedisTaskManager`](self) contract instance.

See the [wrapper's documentation](`IIncredibleRedisTaskManagerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IIncredibleRedisTaskManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> IIncredibleRedisTaskManagerInstance<T, P, N> {
            IIncredibleRedisTaskManagerInstance {
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
    > IIncredibleRedisTaskManagerInstance<T, P, N> {
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
    > IIncredibleRedisTaskManagerInstance<T, P, N> {
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
library OperatorStateRetriever {
    struct CheckSignaturesIndices { uint32[] nonSignerQuorumBitmapIndices; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct Operator { address operator; bytes32 operatorId; uint96 stake; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod OperatorStateRetriever {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct CheckSignaturesIndices { uint32[] nonSignerQuorumBitmapIndices; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckSignaturesIndices {
        #[allow(missing_docs)]
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<u32>,
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
        impl ::core::convert::From<CheckSignaturesIndices> for UnderlyingRustTuple<'_> {
            fn from(value: CheckSignaturesIndices) -> Self {
                (
                    value.nonSignerQuorumBitmapIndices,
                    value.quorumApkIndices,
                    value.totalStakeIndices,
                    value.nonSignerStakeIndices,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CheckSignaturesIndices {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonSignerQuorumBitmapIndices: tuple.0,
                    quorumApkIndices: tuple.1,
                    totalStakeIndices: tuple.2,
                    nonSignerStakeIndices: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CheckSignaturesIndices {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CheckSignaturesIndices {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerQuorumBitmapIndices,
                    ),
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
        impl alloy_sol_types::SolType for CheckSignaturesIndices {
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
        impl alloy_sol_types::SolStruct for CheckSignaturesIndices {
            const NAME: &'static str = "CheckSignaturesIndices";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CheckSignaturesIndices(uint32[] nonSignerQuorumBitmapIndices,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
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
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerQuorumBitmapIndices,
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
        impl alloy_sol_types::EventTopic for CheckSignaturesIndices {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerQuorumBitmapIndices,
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
struct Operator { address operator; bytes32 operatorId; uint96 stake; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Operator {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub stake: alloy::sol_types::private::primitives::aliases::U96,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U96,
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
        impl ::core::convert::From<Operator> for UnderlyingRustTuple<'_> {
            fn from(value: Operator) -> Self {
                (value.operator, value.operatorId, value.stake)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Operator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    operatorId: tuple.1,
                    stake: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Operator {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Operator {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.stake),
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
        impl alloy_sol_types::SolType for Operator {
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
        impl alloy_sol_types::SolStruct for Operator {
            const NAME: &'static str = "Operator";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Operator(address operator,bytes32 operatorId,uint96 stake)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.stake)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Operator {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.stake)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stake,
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
    /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OperatorStateRetrieverInstance<T, P, N> {
        OperatorStateRetrieverInstance::<T, P, N>::new(address, provider)
    }
    /**A [`OperatorStateRetriever`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OperatorStateRetriever`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OperatorStateRetrieverInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OperatorStateRetrieverInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OperatorStateRetrieverInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OperatorStateRetrieverInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> OperatorStateRetrieverInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OperatorStateRetrieverInstance<T, P, N> {
            OperatorStateRetrieverInstance {
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
    > OperatorStateRetrieverInstance<T, P, N> {
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
    > OperatorStateRetrieverInstance<T, P, N> {
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

library IBLSSignatureCheckerTypes {
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

library IIncredibleRedisTaskManager {
    struct SetInput {
        string key;
        string value;
    }
    struct Task {
        SetInput input;
        uint32 taskCreatedBlock;
        bytes quorumNumbers;
        uint32 quorumThresholdPercentage;
    }
    struct TaskResponse {
        uint32 referenceTaskIndex;
        bytes32 result;
    }
    struct TaskResponseMetadata {
        uint32 taskRespondedBlock;
        bytes32 hashOfNonSigners;
    }
}

library OperatorStateRetriever {
    struct CheckSignaturesIndices {
        uint32[] nonSignerQuorumBitmapIndices;
        uint32[] quorumApkIndices;
        uint32[] totalStakeIndices;
        uint32[][] nonSignerStakeIndices;
    }
    struct Operator {
        address operator;
        bytes32 operatorId;
        uint96 stake;
    }
}

interface IncredibleRedisTaskManager {
    error BitmapValueTooLarge();
    error BytesArrayLengthTooLong();
    error BytesArrayNotOrdered();
    error CurrentlyPaused();
    error ECAddFailed();
    error ECMulFailed();
    error ExpModFailed();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InputEmptyQuorumNumbers();
    error InputNonSignerLengthMismatch();
    error InvalidBLSPairingKey();
    error InvalidBLSSignature();
    error InvalidNewPausedStatus();
    error InvalidQuorumApkHash();
    error InvalidReferenceBlocknumber();
    error NonSignerPubkeysNotSorted();
    error OnlyPauser();
    error OnlyRegistryCoordinatorOwner();
    error OnlyUnpauser();
    error OperatorNotRegistered();
    error ScalarTooLarge();
    error StaleStakesForbidden();

    event Initialized(uint8 version);
    event NewTaskCreated(uint32 indexed taskIndex, IIncredibleRedisTaskManager.Task task);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event StaleStakesForbiddenUpdate(bool value);
    event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger);
    event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger);
    event TaskCompleted(uint32 indexed taskIndex);
    event TaskResponded(IIncredibleRedisTaskManager.TaskResponse taskResponse, IIncredibleRedisTaskManager.TaskResponseMetadata taskResponseMetadata);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _registryCoordinator, address _pauserRegistry, uint32 _taskResponseWindowBlock);

    function TASK_CHALLENGE_WINDOW_BLOCK() external view returns (uint32);
    function TASK_RESPONSE_WINDOW_BLOCK() external view returns (uint32);
    function WADS_TO_SLASH() external view returns (uint256);
    function aggregator() external view returns (address);
    function allTaskHashes(uint32) external view returns (bytes32);
    function allTaskResponses(uint32) external view returns (bytes32);
    function allocationManager() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureCheckerTypes.QuorumStakeTotals memory, bytes32);
    function createNewTask(IIncredibleRedisTaskManager.SetInput memory input, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
    function delegation() external view returns (address);
    function generator() external view returns (address);
    function getBatchOperatorFromId(address registryCoordinator, bytes32[] memory operatorIds) external view returns (address[] memory operators);
    function getBatchOperatorId(address registryCoordinator, address[] memory operators) external view returns (bytes32[] memory operatorIds);
    function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes memory quorumNumbers, bytes32[] memory nonSignerOperatorIds) external view returns (OperatorStateRetriever.CheckSignaturesIndices memory);
    function getOperatorState(address registryCoordinator, bytes memory quorumNumbers, uint32 blockNumber) external view returns (OperatorStateRetriever.Operator[][] memory);
    function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) external view returns (uint256, OperatorStateRetriever.Operator[][] memory);
    function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] memory operatorIds, uint32 blockNumber) external view returns (uint256[] memory);
    function getTaskResponseWindowBlock() external view returns (uint32);
    function initialize(address initialOwner, address _aggregator, address _generator, address _allocationManager, address _slasher, address _serviceManager) external;
    function instantSlasher() external view returns (address);
    function latestTaskNum() external view returns (uint32);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function raiseAndResolveChallenge(IIncredibleRedisTaskManager.Task memory task, IIncredibleRedisTaskManager.TaskResponse memory taskResponse, IIncredibleRedisTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
    function redisState(string memory) external view returns (string memory);
    function registryCoordinator() external view returns (address);
    function renounceOwnership() external;
    function respondToTask(IIncredibleRedisTaskManager.Task memory task, IIncredibleRedisTaskManager.TaskResponse memory taskResponse, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
    function serviceManager() external view returns (address);
    function setStaleStakesForbidden(bool value) external;
    function stakeRegistry() external view returns (address);
    function staleStakesForbidden() external view returns (bool);
    function taskNumber() external view returns (uint32);
    function taskSuccesfullyChallenged(uint32) external view returns (bool);
    function transferOwnership(address newOwner) external;
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
    function unpause(uint256 newPausedStatus) external;
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
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "_taskResponseWindowBlock",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "TASK_CHALLENGE_WINDOW_BLOCK",
    "inputs": [],
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
    "name": "TASK_RESPONSE_WINDOW_BLOCK",
    "inputs": [],
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
    "name": "WADS_TO_SLASH",
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
    "name": "aggregator",
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
    "name": "allTaskHashes",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "allTaskResponses",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "allocationManager",
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
        "internalType": "struct IBLSSignatureCheckerTypes.NonSignerStakesAndSignature",
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
        "internalType": "struct IBLSSignatureCheckerTypes.QuorumStakeTotals",
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
    "name": "createNewTask",
    "inputs": [
      {
        "name": "input",
        "type": "tuple",
        "internalType": "struct IIncredibleRedisTaskManager.SetInput",
        "components": [
          {
            "name": "key",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "value",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "quorumThresholdPercentage",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "generator",
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
    "name": "getBatchOperatorFromId",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getBatchOperatorId",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCheckSignaturesIndices",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "nonSignerOperatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OperatorStateRetriever.CheckSignaturesIndices",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorState",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[][]",
        "internalType": "struct OperatorStateRetriever.Operator[][]",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorState",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "tuple[][]",
        "internalType": "struct OperatorStateRetriever.Operator[][]",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapsAtBlockNumber",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTaskResponseWindowBlock",
    "inputs": [],
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
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_aggregator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_generator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_slasher",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "instantSlasher",
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
    "name": "latestTaskNum",
    "inputs": [],
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
    "name": "raiseAndResolveChallenge",
    "inputs": [
      {
        "name": "task",
        "type": "tuple",
        "internalType": "struct IIncredibleRedisTaskManager.Task",
        "components": [
          {
            "name": "input",
            "type": "tuple",
            "internalType": "struct IIncredibleRedisTaskManager.SetInput",
            "components": [
              {
                "name": "key",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "value",
                "type": "string",
                "internalType": "string"
              }
            ]
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "taskResponse",
        "type": "tuple",
        "internalType": "struct IIncredibleRedisTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "result",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "taskResponseMetadata",
        "type": "tuple",
        "internalType": "struct IIncredibleRedisTaskManager.TaskResponseMetadata",
        "components": [
          {
            "name": "taskRespondedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "hashOfNonSigners",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "pubkeysOfNonSigningOperators",
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "redisState",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
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
    "name": "registryCoordinator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
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
    "name": "respondToTask",
    "inputs": [
      {
        "name": "task",
        "type": "tuple",
        "internalType": "struct IIncredibleRedisTaskManager.Task",
        "components": [
          {
            "name": "input",
            "type": "tuple",
            "internalType": "struct IIncredibleRedisTaskManager.SetInput",
            "components": [
              {
                "name": "key",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "value",
                "type": "string",
                "internalType": "string"
              }
            ]
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "taskResponse",
        "type": "tuple",
        "internalType": "struct IIncredibleRedisTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "result",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "nonSignerStakesAndSignature",
        "type": "tuple",
        "internalType": "struct IBLSSignatureCheckerTypes.NonSignerStakesAndSignature",
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
        "internalType": "address"
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
    "name": "taskNumber",
    "inputs": [],
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
    "name": "taskSuccesfullyChallenged",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "NewTaskCreated",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "task",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IIncredibleRedisTaskManager.Task",
        "components": [
          {
            "name": "input",
            "type": "tuple",
            "internalType": "struct IIncredibleRedisTaskManager.SetInput",
            "components": [
              {
                "name": "key",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "value",
                "type": "string",
                "internalType": "string"
              }
            ]
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
  },
  {
    "type": "event",
    "name": "TaskChallengedSuccessfully",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "challenger",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskChallengedUnsuccessfully",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "challenger",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskCompleted",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskResponded",
    "inputs": [
      {
        "name": "taskResponse",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IIncredibleRedisTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "result",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "taskResponseMetadata",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IIncredibleRedisTaskManager.TaskResponseMetadata",
        "components": [
          {
            "name": "taskRespondedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "hashOfNonSigners",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
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
    "name": "BitmapValueTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayLengthTooLong",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayNotOrdered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECAddFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECMulFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpModFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputAddressZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputArrayLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputEmptyQuorumNumbers",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputNonSignerLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBLSPairingKey",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBLSSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidQuorumApkHash",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidReferenceBlocknumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NonSignerPubkeysNotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyPauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyRegistryCoordinatorOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyUnpauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ScalarTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StaleStakesForbidden",
    "inputs": []
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
pub mod IncredibleRedisTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61014080604052346101e65760608161558b80380380916100208285610291565b8339810103126101e65780516001600160a01b038116908181036101e65760208301516001600160a01b038116938482036101e657604001519363ffffffff851685036101e657156102825760805260a052604051636830483560e01b8152602081600481855afa9081156101f2575f9161023f575b5060c052604051632efa2ca360e11b815290602090829060049082905afa9081156101f2575f916101fd575b5060e05260c05160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa9081156101f2575f916101ac575b5061010052610120526040516152c290816102c982396080518181816102da01528181610f0b0152818161168d0152612012015260a0518181816109800152818161125d0152818161315801528181613241015281816137d3015261408a015260c05181818161121901528181613543015261368e015260e0518181816111d50152818161347f0152613fab015261010051818181611d7a01526133540152610120518181816105b201526118d30152f35b90506020813d6020116101ea575b816101c760209383610291565b810103126101e657516001600160a01b03811681036101e6575f6100fa565b5f80fd5b3d91506101ba565b6040513d5f823e3d90fd5b90506020813d602011610237575b8161021860209383610291565b810103126101e657516001600160a01b03811681036101e6575f6100c2565b3d915061020b565b90506020813d60201161027a575b8161025a60209383610291565b810103126101e657516001600160a01b03811681036101e6576004610096565b3d915061024d565b6339b190bb60e11b5f5260045ffd5b601f909101601f19168101906001600160401b038211908210176102b457604052565b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8063136439dd146102aa578063171f1d5b146102a55780631ad43189146101f1578063245a7bfc146102a05780632cb223d51461029b5780632d89f6fc1461029657806331b36bd9146102915780633563b0d11461028c5780633998fdd314610287578063416c7e5e146102825780634d2b57fe1461027d5780634f41b818146102785780634f739f7414610273578063595c6a671461026e5780635a2d7f02146102695780635ac86ab7146102645780635c1556621461025f5780635c975abb1461025a5780635decc3f5146102555780635df4594614610250578063683048351461024b5780636d14a987146102465780636efb463614610241578063715018a61461023c57806372d18e8d1461022d5780637afa1eed14610237578063886f1195146102325780638b00ce7c1461022d5780638da5cb5b1461022857806392a51824146102235780639b290e981461021e578063b702394914610219578063b98d090814610214578063ca8aa7c71461020f578063cc2a9a5b1461020a578063cefdc1d414610205578063df5cf72314610200578063ef029dbc146101fb578063f2fde38b146101f6578063f5c9899d146101f1578063f63c5bab146101ec5763fabc1cbc146101e7575f80fd5b611fe9565b611fce565b610596565b611f3d565b611da9565b611d65565b611c21565b611ae1565b611ab9565b611a97565b6117aa565b611782565b611711565b6116bc565b61162d565b611678565b611650565b6115d2565b611525565b611248565b611204565b6111c0565b611182565b611165565b610fec565b610f80565b610f53565b610ee0565b610e39565b610c27565b610aae565b61094e565b61091c565b6108a2565b6106f8565b610650565b610617565b6105d6565b610524565b3461036a57602036600319011261036a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156103655761033492610320915f91610336575b506120eb565b61032f60665482811614612101565b6149df565b005b610358915060203d60201161035e575b61035081836103a2565b8101906120cb565b5f61031a565b503d610346565b6120e0565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761039d57604052565b61036e565b90601f801991011681019081106001600160401b0382111761039d57604052565b604051906103d3610100836103a2565b565b604051906103d36040836103a2565b604051906103d36080836103a2565b604051906103d36060836103a2565b604051906103d360a0836103a2565b906103d360405192836103a2565b60409060e319011261036a576040519061043882610382565b60e4358252610104356020830152565b919082604091031261036a5760405161046081610382565b6020808294803584520135910152565b9080601f8301121561036a576040519161048b6040846103a2565b82906040810192831161036a57905b8282106104a75750505090565b813581526020918201910161049a565b90608060631983011261036a576040516104d081610382565b60206104eb82946104e2816064610470565b845260a4610470565b910152565b919060808382031261036a5760206104eb6040519261050e84610382565b6040849661051c8382610470565b865201610470565b3461036a5761012036600319011261036a57600435604036602319011261036a5761057c604091825161055681610382565b6024358152604435602082015261056c366104b7565b906105763661041f565b92612155565b8251911515825215156020820152f35b5f91031261036a57565b3461036a575f36600319011261036a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461036a575f36600319011261036a5760ce546040516001600160a01b039091168152602090f35b63ffffffff81160361036a57565b35906103d3826105fe565b3461036a57602036600319011261036a5763ffffffff600435610639816105fe565b165f5260cb602052602060405f2054604051908152f35b3461036a57602036600319011261036a5763ffffffff600435610672816105fe565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b0381160361036a57565b6001600160401b03811161039d5760051b60200190565b90602080835192838152019201905f5b8181106106ce5750505090565b82518452602093840193909201916001016106c1565b9060206106f59281815201906106b1565b90565b3461036a57604036600319011261036a5760043561071581610689565b602435906001600160401b03821161036a573660238301121561036a578160040135916107418361069a565b9261074f60405194856103a2565b8084526024602085019160051b8301019136831161036a57602401905b8282106107905761078c61078086866122bf565b604051918291826106e4565b0390f35b60208091833561079f81610689565b81520191019061076c565b6001600160401b03811161039d57601f01601f191660200190565b9291926107d1826107aa565b916107df60405193846103a2565b82948184528183011161036a578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b83831061082657505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b8082106108665750505060208060019297019301930191939290610817565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610847565b3461036a57606036600319011261036a576004356108bf81610689565b6024356001600160401b03811161036a573660238201121561036a5761078c916108f66109089236906024816004013591016107c5565b60443591610903836105fe565b6124fd565b6040519182916020835260208301906107fb565b3461036a575f36600319011261036a5760d2546040516001600160a01b039091168152602090f35b8015150361036a57565b3461036a57602036600319011261036a5760043561096b81610944565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610365575f916109e0575b506001600160a01b031633036109d15761033490614e72565b637070f3b160e11b5f5260045ffd5b610a02915060203d602011610a08575b6109fa81836103a2565b810190612384565b5f6109b8565b503d6109f0565b9080601f8301121561036a578135610a268161069a565b92610a3460405194856103a2565b81845260208085019260051b82010192831161036a57602001905b828210610a5c5750505090565b8135815260209182019101610a4f565b60206040818301928281528451809452019201905f5b818110610a8f5750505090565b82516001600160a01b0316845260209384019390920191600101610a82565b3461036a57604036600319011261036a57600435610acb81610689565b6024356001600160401b03811161036a57610aea903690600401610a0f565b610af4815161225d565b916001600160a01b03165f5b8251811015610b9157806020610b19610b39938661229c565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561036557600192610b6d915f91610b73575b50610b5e838861229c565b6001600160a01b039091169052565b01610b00565b610b8b915060203d8111610a08576109fa81836103a2565b5f610b53565b6040518061078c8682610a6c565b9080601f8301121561036a578160206106f5933591016107c5565b90600182811c92168015610be8575b6020831014610bd457565b634e487b7160e01b5f52602260045260245ffd5b91607f1691610bc9565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060206106f5928181520190610bf2565b3461036a57602036600319011261036a576004356001600160401b03811161036a576020610c5a81923690600401610b9f565b604051928184925191829101835e810160cd815203019020604051905f90805490610c8482610bba565b8085529160018116908115610cf85750600114610cb8575b61078c84610cac818603826103a2565b60405191829182610c16565b5f90815260208120939250905b808210610cde57509091508101602001610cac82610c9c565b919260018160209254838588010152019101909291610cc5565b60ff191660208087019190915292151560051b85019092019250610cac9150839050610c9c565b9181601f8401121561036a578235916001600160401b03831161036a576020838186019501011161036a57565b90602080835192838152019201905f5b818110610d695750505090565b825163ffffffff16845260209384019390920191600101610d5c565b90602082526060610dd3610dbe610da884516080602088015260a0870190610d4c565b6020850151868203601f19016040880152610d4c565b6040840151858203601f190184870152610d4c565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610e0c57505050505090565b9091929394602080610e2a600193601f198682030187528951610d4c565b97019301930191939290610dfd565b3461036a57608036600319011261036a57600435610e5681610689565b60243590610e63826105fe565b6044356001600160401b03811161036a57610e82903690600401610d1f565b91606435926001600160401b03841161036a573660238501121561036a578360040135926001600160401b03841161036a573660248560051b8701011161036a5761078c956024610ed4960193612a21565b60405191829182610d85565b3461036a575f36600319011261036a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561036557610f4b915f9161033657506120eb565b6103346149ab565b3461036a575f36600319011261036a57602060405167016345785d8a00008152f35b60ff81160361036a57565b3461036a57602036600319011261036a576020600160ff600435610fa381610f75565b161b806066541614604051908152f35b60206040818301928281528451809452019201905f5b818110610fd65750505090565b8251845260209384019390920191600101610fc9565b3461036a57606036600319011261036a5760043561100981610689565b6024356001600160401b03811161036a57611028903690600401610a0f565b60443591611035836105fe565b6040516361c8a12f60e11b8152906001600160a01b03165f828061105d868860048401612e89565b0381845afa918215610365575f92611141575b5061107b835161225d565b935f5b845181101561113357611091818661229c565b51906020836110ad6110a3848961229c565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa8015610365576001925f91611105575b50828060c01b03166110fe828961229c565b520161107e565b611126915060203d811161112c575b61111e81836103a2565b810190612998565b5f6110ec565b503d611114565b6040518061078c8882610fb3565b61115e9192503d805f833e61115681836103a2565b810190612867565b905f611070565b3461036a575f36600319011261036a576020606654604051908152f35b3461036a57602036600319011261036a5763ffffffff6004356111a4816105fe565b165f5260cc602052602060ff60405f2054166040519015158152f35b3461036a575f36600319011261036a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461036a575f36600319011261036a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461036a575f36600319011261036a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080601f8301121561036a5781356112a38161069a565b926112b160405194856103a2565b81845260208085019260051b82010192831161036a57602001905b8282106112d95750505090565b6020809183356112e8816105fe565b8152019101906112cc565b81601f8201121561036a5780356113098161069a565b9261131760405194856103a2565b81845260208085019260061b8401019281841161036a57602001915b838310611341575050505090565b60206040916113508486610448565b815201920191611333565b9080601f8301121561036a5781356113728161069a565b9261138060405194856103a2565b81845260208085019260051b8201019183831161036a5760208201905b8382106113ac57505050505090565b81356001600160401b03811161036a576020916113ce8784809488010161128c565b81520191019061139d565b9190916101808184031261036a576113ef6103c3565b9281356001600160401b03811161036a578161140c91840161128c565b845260208201356001600160401b03811161036a578161142d9184016112f3565b602085015260408201356001600160401b03811161036a57816114519184016112f3565b604085015261146381606084016104f0565b60608501526114758160e08401610448565b60808501526101208201356001600160401b03811161036a578161149a91840161128c565b60a08501526101408201356001600160401b03811161036a57816114bf91840161128c565b60c08501526101608201356001600160401b03811161036a576114e2920161135b565b60e0830152565b90602080835192838152019201905f5b8181106115065750505090565b82516001600160601b03168452602093840193909201916001016114f9565b3461036a57608036600319011261036a576004356024356001600160401b03811161036a57611558903690600401610d1f565b9091604435611566816105fe565b606435926001600160401b03841161036a576115c89461158d6115939536906004016113d9565b9361307b565b6040519283926040845260206115b4825160408088015260808701906114e9565b910151848203603f190160608601526114e9565b9060208301520390f35b3461036a575f36600319011261036a576115ea61504e565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461036a575f36600319011261036a57602063ffffffff60c95416604051908152f35b3461036a575f36600319011261036a5760cf546040516001600160a01b039091168152602090f35b3461036a575f36600319011261036a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461036a575f36600319011261036a576033546040516001600160a01b039091168152602090f35b9081608091031261036a5790565b604090602319011261036a57602490565b9081604091031261036a5790565b3461036a5760c036600319011261036a576004356001600160401b03811161036a576117419036906004016116e4565b61174a366116f2565b90604036606319011261036a5760a4356001600160401b03811161036a576103349261177c60649236906004016112f3565b92613e29565b3461036a575f36600319011261036a5760d0546040516001600160a01b039091168152602090f35b3461036a57608036600319011261036a576004356001600160401b03811161036a576117da9036906004016116e4565b6117e3366116f2565b6064356001600160401b03811161036a576118029036906004016113d9565b60ce549091906001600160a01b03163303611a525761192761182660208501613918565b936118346040820182613ca0565b92909461184360608401613918565b96611895604051602081019061186b8161185d8985614478565b03601f1981018352826103a2565b51902061188e61187a89613918565b63ffffffff165f5260ca60205260405f2090565b5414614530565b6118bf6118b86118a488613918565b63ffffffff165f5260cb60205260405f2090565b54156145a2565b8463ffffffff4316976119096119016118f87f000000000000000000000000000000000000000000000000000000000000000086613b4e565b63ffffffff1690565b8a1115614603565b604051602081019061191f8161185d8c85614665565b51902061307b565b929060ff5f9716965b8281106119f0577ff2af11fad73d4349c99cf62f298d337641ea0bb7c0f5a8db92a98a275f734f588787876119996119888961199461198e61197f6119758480613922565b6020810190613ca0565b94909380613922565b80613ca0565b90614735565b61479d565b6119b06119a46103d5565b63ffffffff9094168452565b602083015260405160208101906119cc8161185d868686614865565b5190206119db6118a483613918565b556119eb60405192839283614865565b0390a1005b80611a4c611a28611a23611a17611a0a600196885161229c565b516001600160601b031690565b6001600160601b031690565b614675565b611a45611a178c611a40611a0a8760208b015161229c565b6146a1565b11156146c4565b01611930565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b3461036a575f36600319011261036a57602060ff609754166040519015158152f35b3461036a575f36600319011261036a5760d1546040516001600160a01b039091168152602090f35b3461036a5760c036600319011261036a57600435611afe81610689565b611b7e602435611b0d81610689565b604435611b1981610689565b606435611b2581610689565b60843591611b3283610689565b60a43593611b3f85610689565b5f5496611b6460ff60088a901c16158099819a611bfc575b8115611bdc575b5061488f565b87611b75600160ff195f5416175f55565b611bc5576148f2565b611b8457005b611b9261ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016119eb565b611bd761010061ff00195f5416175f55565b6148f2565b303b15915081611bee575b505f611b5e565b60ff1660011490505f611be7565b600160ff8216109150611b57565b6040906106f59392815281602082015201906107fb565b3461036a57606036600319011261036a57600435611c3e81610689565b602435604435611c4d816105fe565b611c8e611c5861223b565b9280611c638561228f565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401612e89565b0381875afa9384156103655783611cb86118f86110a3611ced986020975f91611d4b575b5061228f565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561036557611d1c925f91611d2c575b506001600160c01b031692611d168461512a565b906124fd565b9061078c60405192839283611c0a565b611d45915060203d60201161112c5761111e81836103a2565b5f611d02565b611d5f91503d805f833e61115681836103a2565b5f611cb2565b3461036a575f36600319011261036a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461036a57606036600319011261036a576004356001600160401b03811161036a57611dd9903690600401611703565b60243590611de6826105fe565b6044356001600160401b03811161036a57611e05903690600401610d1f565b60cf54909291906001600160a01b03163303611eee5761033493611e52611e6492611ed895611e3d611e356103e4565b963690613937565b86524363ffffffff16602087015236916107c5565b604084015263ffffffff166060830152565b6040516020810190611e7a8161185d858561495b565b519020611e8f61187a60c95463ffffffff1690565b5560c95463ffffffff16907fa6b1912d066fd3a5413a6222a5785ae43f242458f3eef45c50dd808987b878fd60405180611ed063ffffffff8616948261495b565b0390a2613b1e565b63ffffffff1663ffffffff1960c954161760c955565b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b3461036a57602036600319011261036a57600435611f5a81610689565b611f6261504e565b6001600160a01b03811615611f7a57610334906150a6565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461036a575f36600319011261036a57602060405160648152f35b3461036a57602036600319011261036a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610365575f916120ac575b506001600160a01b0316330361209d5761206b606654198219811614612101565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b6120c5915060203d602011610a08576109fa81836103a2565b5f61204a565b9081602091031261036a57516106f581610944565b6040513d5f823e3d90fd5b156120f257565b631d77d47760e21b5f5260045ffd5b1561210857565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b90600281101561213c5760051b0190565b612117565b634e487b7160e01b5f52601260045260245ffd5b61223161220e6122379561220861220185875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e08401526101008301526121d881610120840103601f1981018352826103a2565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b8096614a55565b90614a9b565b9261220861222361221d614afd565b94614bf4565b9161222c614d10565b614a55565b91614d44565b9091565b6040805190919061224c83826103a2565b6001815291601f1901366020840137565b906122678261069a565b61227460405191826103a2565b8281528092612285601f199161069a565b0190602036910137565b80511561213c5760200190565b805182101561213c5760209160051b010190565b9081602091031261036a575190565b9190916122cc835161225d565b925f5b815181101561237f578060206122f86122eb612321948661229c565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa8015610365576001925f91612351575b5061234a828861229c565b52016122cf565b612372915060203d8111612378575b61236a81836103a2565b8101906122b0565b5f61233f565b503d612360565b505050565b9081602091031261036a57516106f581610689565b906123a38261069a565b6123b060405191826103a2565b828152602081936123c3601f199161069a565b0191015f5b8281106123d457505050565b6060828201526020016123c8565b90815181101561213c570160200190565b60208183031261036a578051906001600160401b03821161036a57019080601f8301121561036a5781516124268161069a565b9261243460405194856103a2565b81845260208085019260051b82010192831161036a57602001905b82821061245c5750505090565b815181526020918201910161244f565b906124768261069a565b61248360405191826103a2565b8281528092612494601f199161069a565b015f5b8181106124a357505050565b6040519060608201918083106001600160401b0384111761039d576020926040525f81525f838201525f604082015282828601015201612497565b9081602091031261036a57516001600160601b038116810361036a5790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa948515610365575f95612812575b50604051634f4c91e160e11b815294602086600481855afa918215610365576004965f936127f0575b5060209060405197888092632efa2ca360e11b82525afa958615610365575f966127cf575b5061258b8593929551612399565b945f935b80518510156127c5576125bc6125b66125a887846123e2565b516001600160f81b03191690565b60f81c90565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa938415610365575f946127a1575b5061260c845161246c565b612616888b61229c565b52612621878a61229c565b505f5b84518110156127905780602061263d61265f938861229c565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610365575f92612770575b50612685818761229c565b518a60208a612694858b61229c565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa938415610365576127278c8f6127226001986127399789975f92612740575b5061270d6126fe6103f3565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b61229c565b5190612733838361229c565b5261229c565b5001612624565b61276291925060203d8111612769575b61275a81836103a2565b8101906124de565b905f6126f2565b503d612750565b61278991925060203d8111610a08576109fa81836103a2565b905f61267a565b50600190960195909450915061258f565b6127be9194503d805f833e6127b681836103a2565b8101906123f3565b925f612601565b5050509350505090565b6127e991965060203d602011610a08576109fa81836103a2565b945f61257d565b602091935061280b90823d8411610a08576109fa81836103a2565b9290612558565b61282c91955060203d602011610a08576109fa81836103a2565b935f61252f565b60405190608082018281106001600160401b0382111761039d57604052606080838181528160208201528160408201520152565b60208183031261036a578051906001600160401b03821161036a57019080601f8301121561036a57815161289a8161069a565b926128a860405194856103a2565b81845260208085019260051b82010192831161036a57602001905b8282106128d05750505090565b6020809183516128df816105fe565b8152019101906128c3565b63ffffffff909116815260406020820181905281018390526001600160fb1b03831161036a5760609260051b809284830137010190565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6106f595931681528160208201520191612921565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff81146129835760010190565b61295e565b919081101561213c5760051b0190565b9081602091031261036a57516001600160c01b038116810361036a5790565b156129be57565b6325ec6c1f60e01b5f5260045ffd5b9082101561213c570190565b9081602091031261036a57516106f5816105fe565b5f1981146129835760010190565b91612a1a60209263ffffffff92969596604086526040860191612921565b9416910152565b9593949592909192612a31612833565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa948515610365575f95612e68575b50612a6e612833565b946040516361c8a12f60e11b81525f8180612a8e8d8d8b600485016128ea565b0381885afa908115610365575f91612e4e575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f8180612ad185878b60048501612941565b0381875afa908115610365575f91612e34575b506040870152612af381612399565b9860608701998a525f5b60ff811683811015612d7f57885f612b26838f612b198861225d565b905190612733838361229c565b505f8a868f5b818410612ba9575050505090508c612b438261225d565b915f5b818110612b7057505091612b6591612b6b94935190612733838361229c565b50612972565b612afd565b80612ba3612b8e6110a3600194612b888a895161229c565b5161229c565b612b98838861229c565b9063ffffffff169052565b01612b46565b6110a384612bbe8160209695612bc695612988565b35975161229c565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561036557888f888a918f94612c6b6001612c5e81938d809d5f92612d53575b506125b6612c3a612c4892612c33878060c01b03861615156129b7565b8b8d6129cd565b356001600160f81b03191690565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14612c87575b5050505050600191925001908a918a868f612b2c565b8597612ca993612ca260209799986125b695612c3a95612988565b35956129cd565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa908115610365578f612d0790612d0c9383886001975f93612d1b575b50612b8890612b9893945161229c565b6129ee565b905082918a888f888a91612c71565b612b98935090612d44612b889260203d8111612d4c575b612d3c81836103a2565b8101906129d9565b935090612cf7565b503d612d32565b612c48919250612c3a612d766125b69260203d811161112c5761111e81836103a2565b93925050612c16565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa90811561036557612dd5945f948593612e13575b5060405163354952a360e21b815295869485938493600485016129fc565b03916001600160a01b03165afa908115610365575f91612df9575b50602082015290565b612e0d91503d805f833e61115681836103a2565b5f612df0565b612e2d91935060203d602011610a08576109fa81836103a2565b915f612db7565b612e4891503d805f833e61115681836103a2565b5f612ae4565b612e6291503d805f833e61115681836103a2565b5f612aa1565b612e8291955060203d602011610a08576109fa81836103a2565b935f612a65565b60409063ffffffff6106f5949316815281602082015201906106b1565b60405190612eb382610382565b60606020838281520152565b15612ec657565b62f8202d60e51b5f5260045ffd5b15612edb57565b6343714afd60e01b5f5260045ffd5b15612ef157565b635f832f4160e01b5f5260045ffd5b15612f0757565b634b874f4560e01b5f5260045ffd5b9081602091031261036a57516106f581610f75565b5f1981019190821161298357565b15612f4057565b633fdc650560e21b5f5260045ffd5b906001820180921161298357565b906002820180921161298357565b906003820180921161298357565b906004820180921161298357565b906005820180921161298357565b9190820180921161298357565b15612fa957565b63affc5edb60e01b5f5260045ffd5b9081602091031261036a575167ffffffffffffffff198116810361036a5790565b15612fe057565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b03821161298357565b1561301657565b6367988d3360e01b5f5260045ffd5b1561302c57565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b8181106130655750505090565b8251845260209384019390920191600101613058565b949392909193613089612ea6565b50613095851515612ebf565b60408401515185148061390a575b806138fc575b806138ee575b6130b890612ed4565b6130ca60208501515185515114612eea565b6130e163ffffffff431663ffffffff841610612f00565b6130e96103d5565b5f81525f6020820152926130fb612ea6565b6131048761225d565b60208201526131128761225d565b815261311c612ea6565b9261312b60208801515161225d565b845261313b60208801515161225d565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610365576131a4915f916138bf575b5061319f368b876107c5565b614eb0565b985f965b602089015180518910156133165760208861320b6110a38c6132038f96868e6131e86131d586809561229c565b5180515f526020015160205260405f2090565b6131f5848484015161229c565b52826132e3575b015161229c565b51955161229c565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa918215610365576122088a6132b88f6132b18f8460208f926132a8936132a08460019e6132be9e5f916132c6575b508f8060c01b0316925161229c565b52015161229c565b51938d5161229c565b5116614edb565b90614f0c565b9701966131a8565b6132dd9150863d811161112c5761111e81836103a2565b5f613291565b6133116132f3848484015161229c565b5161330a8484015161330487612f2b565b9061229c565b5110612f39565b6131fc565b5090959794965061332b919893929950614fc9565b9161333860975460ff1690565b9081156138b7576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610365575f91613898575b5091905b5f925b8184106133e9575050505050926133d06133cb6133c46133e3958561185d9860806060602099015192015192612155565b919061300f565b613025565b015160405192839160208301958661303b565b51902090565b92989596909399919794878b888c888d613792575b6110a38260a061343e6125b6612c3a846134469761343861342a6131d58f9c604060209f9e015161229c565b67ffffffffffffffff191690565b9b6129cd565b97015161229c565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156103655761350a6110a38f958f906135028f978f96848f6134fc60c0966134f5848f60209f906131fc612c3a996040936125b69c5f91613764575b5067ffffffffffffffff19918216911614612fd9565b5190614a9b565b9c6129cd565b96015161229c565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561036557613597918c8f925f92613740575b5060206135899293015161229c565b906001600160601b03169052565b6135b78c6135898c6135b0611a0a82602086015161229c565b925161229c565b5f985f5b60208a015151811015613727578b8d6135f9896135ec6125b6612c3a868f896135e4915161229c565b5194876129cd565b60ff161c60019081161490565b613608575b50506001016135bb565b8a8a61368a859f948f9686612b888f9360e06136416110a39560206136396125b6612c3a839f61364a9c89916129cd565b9a015161229c565b519b015161229c565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610365578f6136f6908f936001959486955f92613701575b506136f0613589929351936136eb611a0a848761229c565b612fef565b9261229c565b019a90508b8d6135fe565b61358992506137206136f09160203d81116127695761275a81836103a2565b92506136d3565b5093919796996001919699509a94929a01929190613393565b613589925061375d602091823d81116127695761275a81836103a2565b925061357a565b602061378592503d811161378b575b61377d81836103a2565b810190612fb8565b5f6134df565b503d613773565b6137cf94506137ac92506125b691612c3a916020956129cd565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610365576020896134468f938f60a08f976125b6612c3a8f8f9061343861342a6131d58f60408b96918f88936110a39f613853906138599361343e9f5f9261386f575b5063ffffffff809116931690612f95565b11612fa2565b50505050505097505050505050929350506133fe565b602063ffffffff9293508291613890913d81116123785761236a81836103a2565b929150613842565b6138b1915060203d602011612d4c57612d3c81836103a2565b5f61338c565b5f9190613390565b6138e1915060203d6020116138e7575b6138d981836103a2565b810190612f16565b5f613193565b503d6138cf565b5060e08401515185146130af565b5060c08401515185146130a9565b5060a08401515185146130a3565b356106f5816105fe565b903590603e198136030182121561036a570190565b919060408382031261036a576040519061395082610382565b819380356001600160401b03811161036a578261396e918301610b9f565b83526020810135916001600160401b03831161036a576020926104eb9201610b9f565b6106f5903690613937565b156139a357565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b6020809163ffffffff8135613a06816105fe565b1684520135910152565b9160406103d3929493613a278160808101976139f2565b01906139f2565b15613a3557565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15613aa757565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b63ffffffff60019116019063ffffffff821161298357565b63ffffffff60649116019063ffffffff821161298357565b9063ffffffff8091169116019063ffffffff821161298357565b15613b6f57565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b6106f5916020613bf38351604084526040840190610bf2565b920151906020818403910152610bf2565b9060206106f5928181520190613bda565b15613c1c57565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b903590601e198136030182121561036a57018035906001600160401b03821161036a5760200191813603831361036a57565b60208183031261036a578051906001600160401b03821161036a57019080601f8301121561036a578151613d058161069a565b92613d1360405194856103a2565b81845260208085019260051b82010192831161036a57602001905b828210613d3b5750505090565b602080918351613d4a81610689565b815201910190613d2e565b60405190613d646040836103a2565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b818110613e0a575050506080613df56106f594956060850151601f1985830301848601526106b1565b9201519060a0601f1982850301910152610bf2565b82516001600160a01b0316875260209687019690920191600101613dcc565b9092916001613e3785613918565b946020613e4c613e478680613922565b613991565b613e6f613e678963ffffffff165f5260cb60205260405f2090565b54151561399c565b613eaa613e8a8963ffffffff165f5260cb60205260405f2090565b5460405184810190613ea18161185d8b8a86613a10565b51902014613a2e565b613ed5613ecf613ec88a63ffffffff165f5260cc60205260405f2090565b5460ff1690565b15613aa0565b613efa613eec6118f8613ee788613918565b613b36565b63ffffffff43161115613b68565b604051613f0e8161185d8582019485613c04565b51902091613f1b836150ee565b013590613f27826150ee565b141461441657613f37835161225d565b935f5b8451811015613f645780613f536131d56001938861229c565b613f5d828961229c565b5201613f3a565b509092939194613f9e60208701946020613f7d87613918565b604051613f928161185d8a868301958661303b565b51902091013514613c15565b613fa8855161225d565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b875181101561405857806020613fef61400f938961229c565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa91821561036557600192614034915f9161403a575b50610b5e838d61229c565b01613fd6565b614052915060203d8111610a08576109fa81836103a2565b5f614029565b50929695509250926140b761408061408860408601946140788688613ca0565b939091613918565b9236916107c5565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166124fd565b945f915b86518310156143b8575f979697955b6140d4848a61229c565b51518710156143a85797614115986020806140f38a612b88898761229c565b510151604051809c81926308f6629d60e31b8352600483019190602083019252565b0381865afa998a15610365575f9a614388575b506001985f5b855181101561437a576141536141476122eb838961229c565b6001600160a01b031690565b6001600160a01b038d161461416a5760010161412e565b5099909791985060015f5b15151461418a575b50600101959796976140ca565b98969786985f879593986141ef60ff6141c76125b6612c3a8c6141c16142449f6141bb60d25460018060a01b031690565b98613ca0565b906129cd565b6141e16141d26103d5565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d15461420690614147906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529098899190829081906044820190565b03915afa968715610365575f97614356575b50614261875161225d565b985f5b8a5181101561428a578067016345785d8a00006142836001938e61229c565b5201614264565b509a9294966142c860ff6142af6125b6612c3a8f9d979f969e966141c18e8e92613ca0565b6142ba6126fe610402565b1663ffffffff166020860152565b604084015260608301526142da613d55565b608083015260d0546142f690614147906001600160a01b031681565b803b1561036a57604051636a669b4160e01b8152925f9184918290849082906143229060048301613d85565b03925af19182156103655760019261433c575b509061417d565b8061434a5f614350936103a2565b8061058c565b5f614335565b6143739197503d805f833e61436b81836103a2565b810190613cd2565b955f614256565b509990979198600190614175565b6143a1919a5060203d8111610a08576109fa81836103a2565b985f614128565b96979695506001909201916140bb565b50505050509190506143e86143db8263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b50505063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b9035601e198236030181121561036a5701602081359101916001600160401b03821161036a57813603831361036a57565b60208152813590603e198336030182121561036a576080614525606061451e6144e4876106f597018560208801526144d26144c76144b68380614447565b604060a08c015260e08b0191612921565b916020810190614447565b888303609f190160c08a015290612921565b6145006144f360208a0161060c565b63ffffffff166040880152565b61450d6040890189614447565b878303601f19018589015290612921565b950161060c565b63ffffffff16910152565b1561453757565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156145a957565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b1561460a57565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6040810192916103d391906139f2565b9060648202918083046064149015171561298357565b9060068202918083046006149015171561298357565b906001600160601b03809116911602906001600160601b03821691820361298357565b156146cb57565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b602090826040519384928337810160cd81520301902090565b601f821161475b57505050565b5f5260205f20906020601f840160051c83019310614793575b601f0160051c01905b818110614788575050565b5f815560010161477d565b9091508190614774565b9092916001600160401b03811161039d576147c2816147bc8454610bba565b8461474e565b5f601f82116001146148005781906147f19394955f926147f5575b50508160011b915f199060031b1c19161790565b9055565b013590505f806147dd565b601f19821694614813845f5260205f2090565b915f5b87811061484d575083600195969710614834575b505050811b019055565b01355f19600384901b60f8161c191690555f808061482a565b90926020600181928686013581550194019101614816565b909291602060609161487b8460808101976139f2565b63ffffffff81511660408501520151910152565b1561489657565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6148fb906150a6565b60ce80546001600160a01b03199081166001600160a01b039384161790915560cf805482169383169390931790925560d1805483169382169390931790925560d0805482169383169390931790925560d280549092169216919091179055565b60208152608063ffffffff60606149a1614980865185602088015260a0870190613bda565b8360208801511660408701526040870151601f198783030184880152610bf2565b9401511691015290565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b60405190614a1e82610382565b5f6020838281520152565b60405190610180614a3a81846103a2565b368337565b60405190614a4e6020836103a2565b6020368337565b91906040906060614a64614a11565b9485926020855192614a7685856103a2565b8436853780518452015160208301528482015260076107cf195a01fa15614a9957565bfe5b602092916080604092614aac614a11565b95869381865193614abd86866103a2565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa8015614a995715614aee57565b63d4b68fd760e01b5f5260045ffd5b604051614b0981610382565b6040908151614b1883826103a2565b8236823781526020825191614b2d84846103a2565b8336843701528051614b3f82826103a2565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6020820152815190614b9583836103a2565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152614bea835193846103a2565b8252602082015290565b5f51602061526d5f395f51905f5290614c0b614a11565b505f919006602060c0835b614d0b575f935f51602061526d5f395f51905f5260038186818180090908604051614c4185826103a2565b84368237848185604051614c5582826103a2565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f51602061526d5f395f51905f5260a082015260056107cf195a01fa8015614a9957614cbf90615256565b5191614d0b575f51602061526d5f395f51905f5282800914614cf657505f51602061526d5f395f51905f5260015f94089293614c16565b92935050614d026103d5565b92835282015290565b612141565b614d18614a11565b50604051614d2581610382565b600181526002602082015290565b90600c81101561213c5760051b0190565b93929091614d526040610411565b9485526020850152614d646040610411565b9182526020820152614d74614a29565b925f5b60028110614da157505050602061018092614d90614a3f565b93849160086201d4c0fa9151151590565b80614dad60019261468b565b614db7828561212b565b5151614dc38289614d33565b526020614dd0838661212b565b510151614de5614ddf83612f4f565b89614d33565b52614df0828661212b565b515151614dff614ddf83612f5d565b52614e15614e0d838761212b565b515160200190565b51614e22614ddf83612f6b565b526020614e2f838761212b565b51015151614e3f614ddf83612f79565b52614e6b614e65614e5e6020614e55868a61212b565b51015160200190565b5192612f87565b88614d33565b5201614d77565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614ebe60ff936151de565b928392161b1115614ecc5790565b63ca95733360e01b5f5260045ffd5b805f915b614ee7575090565b5f1981018181116129835761ffff9116911661ffff8114612983576001019080614edf565b90614f15614a11565b5061ffff811690610200821015614fba5760018214614fb557614f366103d5565b5f81525f602082015292906001905f925b61ffff8316851015614f5b57505050505090565b600161ffff831660ff86161c811614614f95575b6001614f8b614f808360ff94614a9b565b9460011b61fffe1690565b9401169291614f47565b946001614f8b614f80614faa8960ff95614a9b565b989350505050614f6f565b505090565b637fc4ea7d60e11b5f5260045ffd5b614fd1614a11565b50805190811580615042575b15614ffe575050604051614ff26040826103a2565b5f81525f602082015290565b60205f51602061526d5f395f51905f52910151065f51602061526d5f395f51905f52035f51602061526d5f395f51905f5281116129835760405191614bea83610382565b50602081015115614fdd565b6033546001600160a01b0316330361506257565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b5f809160405160208101916327b7cf8560e01b83526024820152602481526151176044826103a2565b51906a636f6e736f6c652e6c6f675afa50565b61ffff61513682614edb565b16615140816107aa565b9061514e60405192836103a2565b80825261515d601f19916107aa565b013660208301375f5f5b82518210806151bd575b156151b6576001811b841661518f575b61518a906129ee565b615167565b90600161518a9160ff60f81b8460f81b165f1a6151ac82876123e2565b5301919050615181565b5050905090565b506101008110615171565b156151cf57565b631019106960e31b5f5260045ffd5b906101008251116152475781511561524257602082015160019060f81c81901b5b835182101561523d5760019061522861521e6125b66125a886896123e2565b60ff600191161b90565b906152348183116151c8565b179101906151ff565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b1561525d57565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220fc928aa05e8197e904cac0bcde47a5835ccb1486165078b6b2f5480bb6d0a25a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01@\x80`@R4a\x01\xE6W``\x81aU\x8B\x808\x03\x80\x91a\0 \x82\x85a\x02\x91V[\x839\x81\x01\x03\x12a\x01\xE6W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\x01\xE6W` \x83\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x84\x82\x03a\x01\xE6W`@\x01Q\x93c\xFF\xFF\xFF\xFF\x85\x16\x85\x03a\x01\xE6W\x15a\x02\x82W`\x80R`\xA0R`@Qch0H5`\xE0\x1B\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x02?W[P`\xC0R`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x01\xFDW[P`\xE0R`\xC0Q`@Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x01\xACW[Pa\x01\0Ra\x01 R`@QaR\xC2\x90\x81a\x02\xC9\x829`\x80Q\x81\x81\x81a\x02\xDA\x01R\x81\x81a\x0F\x0B\x01R\x81\x81a\x16\x8D\x01Ra \x12\x01R`\xA0Q\x81\x81\x81a\t\x80\x01R\x81\x81a\x12]\x01R\x81\x81a1X\x01R\x81\x81a2A\x01R\x81\x81a7\xD3\x01Ra@\x8A\x01R`\xC0Q\x81\x81\x81a\x12\x19\x01R\x81\x81a5C\x01Ra6\x8E\x01R`\xE0Q\x81\x81\x81a\x11\xD5\x01R\x81\x81a4\x7F\x01Ra?\xAB\x01Ra\x01\0Q\x81\x81\x81a\x1Dz\x01Ra3T\x01Ra\x01 Q\x81\x81\x81a\x05\xB2\x01Ra\x18\xD3\x01R\xF3[\x90P` \x81=` \x11a\x01\xEAW[\x81a\x01\xC7` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W_a\0\xFAV[_\x80\xFD[=\x91Pa\x01\xBAV[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x027W[\x81a\x02\x18` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W_a\0\xC2V[=\x91Pa\x02\x0BV[\x90P` \x81=` \x11a\x02zW[\x81a\x02Z` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W`\x04a\0\x96V[=\x91Pa\x02MV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x02\xB4W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x13d9\xDD\x14a\x02\xAAW\x80c\x17\x1F\x1D[\x14a\x02\xA5W\x80c\x1A\xD41\x89\x14a\x01\xF1W\x80c$Z{\xFC\x14a\x02\xA0W\x80c,\xB2#\xD5\x14a\x02\x9BW\x80c-\x89\xF6\xFC\x14a\x02\x96W\x80c1\xB3k\xD9\x14a\x02\x91W\x80c5c\xB0\xD1\x14a\x02\x8CW\x80c9\x98\xFD\xD3\x14a\x02\x87W\x80cAl~^\x14a\x02\x82W\x80cM+W\xFE\x14a\x02}W\x80cOA\xB8\x18\x14a\x02xW\x80cOs\x9Ft\x14a\x02sW\x80cY\\jg\x14a\x02nW\x80cZ-\x7F\x02\x14a\x02iW\x80cZ\xC8j\xB7\x14a\x02dW\x80c\\\x15Vb\x14a\x02_W\x80c\\\x97Z\xBB\x14a\x02ZW\x80c]\xEC\xC3\xF5\x14a\x02UW\x80c]\xF4YF\x14a\x02PW\x80ch0H5\x14a\x02KW\x80cm\x14\xA9\x87\x14a\x02FW\x80cn\xFBF6\x14a\x02AW\x80cqP\x18\xA6\x14a\x02<W\x80cr\xD1\x8E\x8D\x14a\x02-W\x80cz\xFA\x1E\xED\x14a\x027W\x80c\x88o\x11\x95\x14a\x022W\x80c\x8B\0\xCE|\x14a\x02-W\x80c\x8D\xA5\xCB[\x14a\x02(W\x80c\x92\xA5\x18$\x14a\x02#W\x80c\x9B)\x0E\x98\x14a\x02\x1EW\x80c\xB7\x029I\x14a\x02\x19W\x80c\xB9\x8D\t\x08\x14a\x02\x14W\x80c\xCA\x8A\xA7\xC7\x14a\x02\x0FW\x80c\xCC*\x9A[\x14a\x02\nW\x80c\xCE\xFD\xC1\xD4\x14a\x02\x05W\x80c\xDF\\\xF7#\x14a\x02\0W\x80c\xEF\x02\x9D\xBC\x14a\x01\xFBW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xF6W\x80c\xF5\xC9\x89\x9D\x14a\x01\xF1W\x80c\xF6<[\xAB\x14a\x01\xECWc\xFA\xBC\x1C\xBC\x14a\x01\xE7W_\x80\xFD[a\x1F\xE9V[a\x1F\xCEV[a\x05\x96V[a\x1F=V[a\x1D\xA9V[a\x1DeV[a\x1C!V[a\x1A\xE1V[a\x1A\xB9V[a\x1A\x97V[a\x17\xAAV[a\x17\x82V[a\x17\x11V[a\x16\xBCV[a\x16-V[a\x16xV[a\x16PV[a\x15\xD2V[a\x15%V[a\x12HV[a\x12\x04V[a\x11\xC0V[a\x11\x82V[a\x11eV[a\x0F\xECV[a\x0F\x80V[a\x0FSV[a\x0E\xE0V[a\x0E9V[a\x0C'V[a\n\xAEV[a\tNV[a\t\x1CV[a\x08\xA2V[a\x06\xF8V[a\x06PV[a\x06\x17V[a\x05\xD6V[a\x05$V[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03eWa\x034\x92a\x03 \x91_\x91a\x036W[Pa \xEBV[a\x03/`fT\x82\x81\x16\x14a!\x01V[aI\xDFV[\0[a\x03X\x91P` =` \x11a\x03^W[a\x03P\x81\x83a\x03\xA2V[\x81\x01\x90a \xCBV[_a\x03\x1AV[P=a\x03FV[a \xE0V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x9DW`@RV[a\x03nV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x9DW`@RV[`@Q\x90a\x03\xD3a\x01\0\x83a\x03\xA2V[V[`@Q\x90a\x03\xD3`@\x83a\x03\xA2V[`@Q\x90a\x03\xD3`\x80\x83a\x03\xA2V[`@Q\x90a\x03\xD3``\x83a\x03\xA2V[`@Q\x90a\x03\xD3`\xA0\x83a\x03\xA2V[\x90a\x03\xD3`@Q\x92\x83a\x03\xA2V[`@\x90`\xE3\x19\x01\x12a\x03jW`@Q\x90a\x048\x82a\x03\x82V[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x03jW`@Qa\x04`\x81a\x03\x82V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03jW`@Q\x91a\x04\x8B`@\x84a\x03\xA2V[\x82\x90`@\x81\x01\x92\x83\x11a\x03jW\x90[\x82\x82\x10a\x04\xA7WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x04\x9AV[\x90`\x80`c\x19\x83\x01\x12a\x03jW`@Qa\x04\xD0\x81a\x03\x82V[` a\x04\xEB\x82\x94a\x04\xE2\x81`da\x04pV[\x84R`\xA4a\x04pV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03jW` a\x04\xEB`@Q\x92a\x05\x0E\x84a\x03\x82V[`@\x84\x96a\x05\x1C\x83\x82a\x04pV[\x86R\x01a\x04pV[4a\x03jWa\x01 6`\x03\x19\x01\x12a\x03jW`\x045`@6`#\x19\x01\x12a\x03jWa\x05|`@\x91\x82Qa\x05V\x81a\x03\x82V[`$5\x81R`D5` \x82\x01Ra\x05l6a\x04\xB7V[\x90a\x05v6a\x04\x1FV[\x92a!UV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x03jWV[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03jWV[5\x90a\x03\xD3\x82a\x05\xFEV[4a\x03jW` 6`\x03\x19\x01\x12a\x03jWc\xFF\xFF\xFF\xFF`\x045a\x069\x81a\x05\xFEV[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x03jW` 6`\x03\x19\x01\x12a\x03jWc\xFF\xFF\xFF\xFF`\x045a\x06r\x81a\x05\xFEV[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03jWV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x9DW`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xCEWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\xC1V[\x90` a\x06\xF5\x92\x81\x81R\x01\x90a\x06\xB1V[\x90V[4a\x03jW`@6`\x03\x19\x01\x12a\x03jW`\x045a\x07\x15\x81a\x06\x89V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03jW6`#\x83\x01\x12\x15a\x03jW\x81`\x04\x015\x91a\x07A\x83a\x06\x9AV[\x92a\x07O`@Q\x94\x85a\x03\xA2V[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03jW`$\x01\x90[\x82\x82\x10a\x07\x90Wa\x07\x8Ca\x07\x80\x86\x86a\"\xBFV[`@Q\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[` \x80\x91\x835a\x07\x9F\x81a\x06\x89V[\x81R\x01\x91\x01\x90a\x07lV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x9DW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xD1\x82a\x07\xAAV[\x91a\x07\xDF`@Q\x93\x84a\x03\xA2V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03jW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x08&WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x08fWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x08\x17V[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\x08GV[4a\x03jW``6`\x03\x19\x01\x12a\x03jW`\x045a\x08\xBF\x81a\x06\x89V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03jW6`#\x82\x01\x12\x15a\x03jWa\x07\x8C\x91a\x08\xF6a\t\x08\x926\x90`$\x81`\x04\x015\x91\x01a\x07\xC5V[`D5\x91a\t\x03\x83a\x05\xFEV[a$\xFDV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xFBV[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`\xD2T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x03jWV[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW`\x045a\tk\x81a\tDV[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03eW_\x91a\t\xE0W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\t\xD1Wa\x034\x90aNrV[cpp\xF3\xB1`\xE1\x1B_R`\x04_\xFD[a\n\x02\x91P` =` \x11a\n\x08W[a\t\xFA\x81\x83a\x03\xA2V[\x81\x01\x90a#\x84V[_a\t\xB8V[P=a\t\xF0V[\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x815a\n&\x81a\x06\x9AV[\x92a\n4`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03jW` \x01\x90[\x82\x82\x10a\n\\WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\nOV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\n\x8FWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\x82V[4a\x03jW`@6`\x03\x19\x01\x12a\x03jW`\x045a\n\xCB\x81a\x06\x89V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\n\xEA\x906\x90`\x04\x01a\n\x0FV[a\n\xF4\x81Qa\"]V[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\x0B\x91W\x80` a\x0B\x19a\x0B9\x93\x86a\"\x9CV[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x03eW`\x01\x92a\x0Bm\x91_\x91a\x0BsW[Pa\x0B^\x83\x88a\"\x9CV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\x0B\0V[a\x0B\x8B\x91P` =\x81\x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[_a\x0BSV[`@Q\x80a\x07\x8C\x86\x82a\nlV[\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x81` a\x06\xF5\x935\x91\x01a\x07\xC5V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0B\xE8W[` \x83\x10\x14a\x0B\xD4WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x0B\xC9V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x06\xF5\x92\x81\x81R\x01\x90a\x0B\xF2V[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03jW` a\x0CZ\x81\x926\x90`\x04\x01a\x0B\x9FV[`@Q\x92\x81\x84\x92Q\x91\x82\x91\x01\x83^\x81\x01`\xCD\x81R\x03\x01\x90 `@Q\x90_\x90\x80T\x90a\x0C\x84\x82a\x0B\xBAV[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a\x0C\xF8WP`\x01\x14a\x0C\xB8W[a\x07\x8C\x84a\x0C\xAC\x81\x86\x03\x82a\x03\xA2V[`@Q\x91\x82\x91\x82a\x0C\x16V[_\x90\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x0C\xDEWP\x90\x91P\x81\x01` \x01a\x0C\xAC\x82a\x0C\x9CV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x0C\xC5V[`\xFF\x19\x16` \x80\x87\x01\x91\x90\x91R\x92\x15\x15`\x05\x1B\x85\x01\x90\x92\x01\x92Pa\x0C\xAC\x91P\x83\x90Pa\x0C\x9CV[\x91\x81`\x1F\x84\x01\x12\x15a\x03jW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03jW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03jWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\riWPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\r\\V[\x90` \x82R``a\r\xD3a\r\xBEa\r\xA8\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\rLV[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\rLV[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\rLV[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0E\x0CWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0E*`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\rLV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\r\xFDV[4a\x03jW`\x806`\x03\x19\x01\x12a\x03jW`\x045a\x0EV\x81a\x06\x89V[`$5\x90a\x0Ec\x82a\x05\xFEV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x0E\x82\x906\x90`\x04\x01a\r\x1FV[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03jW6`#\x85\x01\x12\x15a\x03jW\x83`\x04\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03jW6`$\x85`\x05\x1B\x87\x01\x01\x11a\x03jWa\x07\x8C\x95`$a\x0E\xD4\x96\x01\x93a*!V[`@Q\x91\x82\x91\x82a\r\x85V[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03eWa\x0FK\x91_\x91a\x036WPa \xEBV[a\x034aI\xABV[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` `@Qg\x01cEx]\x8A\0\0\x81R\xF3[`\xFF\x81\x16\x03a\x03jWV[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW` `\x01`\xFF`\x045a\x0F\xA3\x81a\x0FuV[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x0F\xD6WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0F\xC9V[4a\x03jW``6`\x03\x19\x01\x12a\x03jW`\x045a\x10\t\x81a\x06\x89V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x10(\x906\x90`\x04\x01a\n\x0FV[`D5\x91a\x105\x83a\x05\xFEV[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x10]\x86\x88`\x04\x84\x01a.\x89V[\x03\x81\x84Z\xFA\x91\x82\x15a\x03eW_\x92a\x11AW[Pa\x10{\x83Qa\"]V[\x93_[\x84Q\x81\x10\x15a\x113Wa\x10\x91\x81\x86a\"\x9CV[Q\x90` \x83a\x10\xADa\x10\xA3\x84\x89a\"\x9CV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x03eW`\x01\x92_\x91a\x11\x05W[P\x82\x80`\xC0\x1B\x03\x16a\x10\xFE\x82\x89a\"\x9CV[R\x01a\x10~V[a\x11&\x91P` =\x81\x11a\x11,W[a\x11\x1E\x81\x83a\x03\xA2V[\x81\x01\x90a)\x98V[_a\x10\xECV[P=a\x11\x14V[`@Q\x80a\x07\x8C\x88\x82a\x0F\xB3V[a\x11^\x91\x92P=\x80_\x83>a\x11V\x81\x83a\x03\xA2V[\x81\x01\x90a(gV[\x90_a\x10pV[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` `fT`@Q\x90\x81R\xF3[4a\x03jW` 6`\x03\x19\x01\x12a\x03jWc\xFF\xFF\xFF\xFF`\x045a\x11\xA4\x81a\x05\xFEV[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x815a\x12\xA3\x81a\x06\x9AV[\x92a\x12\xB1`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03jW` \x01\x90[\x82\x82\x10a\x12\xD9WPPP\x90V[` \x80\x91\x835a\x12\xE8\x81a\x05\xFEV[\x81R\x01\x91\x01\x90a\x12\xCCV[\x81`\x1F\x82\x01\x12\x15a\x03jW\x805a\x13\t\x81a\x06\x9AV[\x92a\x13\x17`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03jW` \x01\x91[\x83\x83\x10a\x13AWPPPP\x90V[` `@\x91a\x13P\x84\x86a\x04HV[\x81R\x01\x92\x01\x91a\x133V[\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x815a\x13r\x81a\x06\x9AV[\x92a\x13\x80`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03jW` \x82\x01\x90[\x83\x82\x10a\x13\xACWPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03jW` \x91a\x13\xCE\x87\x84\x80\x94\x88\x01\x01a\x12\x8CV[\x81R\x01\x91\x01\x90a\x13\x9DV[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03jWa\x13\xEFa\x03\xC3V[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x81a\x14\x0C\x91\x84\x01a\x12\x8CV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x81a\x14-\x91\x84\x01a\x12\xF3V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x81a\x14Q\x91\x84\x01a\x12\xF3V[`@\x85\x01Ra\x14c\x81``\x84\x01a\x04\xF0V[``\x85\x01Ra\x14u\x81`\xE0\x84\x01a\x04HV[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x81a\x14\x9A\x91\x84\x01a\x12\x8CV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x81a\x14\xBF\x91\x84\x01a\x12\x8CV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x14\xE2\x92\x01a\x13[V[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x15\x06WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x14\xF9V[4a\x03jW`\x806`\x03\x19\x01\x12a\x03jW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x15X\x906\x90`\x04\x01a\r\x1FV[\x90\x91`D5a\x15f\x81a\x05\xFEV[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03jWa\x15\xC8\x94a\x15\x8Da\x15\x93\x956\x90`\x04\x01a\x13\xD9V[\x93a0{V[`@Q\x92\x83\x92`@\x84R` a\x15\xB4\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x14\xE9V[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x14\xE9V[\x90` \x83\x01R\x03\x90\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jWa\x15\xEAaPNV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90\x81`\x80\x91\x03\x12a\x03jW\x90V[`@\x90`#\x19\x01\x12a\x03jW`$\x90V[\x90\x81`@\x91\x03\x12a\x03jW\x90V[4a\x03jW`\xC06`\x03\x19\x01\x12a\x03jW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x17A\x906\x90`\x04\x01a\x16\xE4V[a\x17J6a\x16\xF2V[\x90`@6`c\x19\x01\x12a\x03jW`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x034\x92a\x17|`d\x926\x90`\x04\x01a\x12\xF3V[\x92a>)V[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03jW`\x806`\x03\x19\x01\x12a\x03jW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x17\xDA\x906\x90`\x04\x01a\x16\xE4V[a\x17\xE36a\x16\xF2V[`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x18\x02\x906\x90`\x04\x01a\x13\xD9V[`\xCET\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1ARWa\x19'a\x18&` \x85\x01a9\x18V[\x93a\x184`@\x82\x01\x82a<\xA0V[\x92\x90\x94a\x18C``\x84\x01a9\x18V[\x96a\x18\x95`@Q` \x81\x01\x90a\x18k\x81a\x18]\x89\x85aDxV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xA2V[Q\x90 a\x18\x8Ea\x18z\x89a9\x18V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[T\x14aE0V[a\x18\xBFa\x18\xB8a\x18\xA4\x88a9\x18V[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15aE\xA2V[\x84c\xFF\xFF\xFF\xFFC\x16\x97a\x19\ta\x19\x01a\x18\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a;NV[c\xFF\xFF\xFF\xFF\x16\x90V[\x8A\x11\x15aF\x03V[`@Q` \x81\x01\x90a\x19\x1F\x81a\x18]\x8C\x85aFeV[Q\x90 a0{V[\x92\x90`\xFF_\x97\x16\x96[\x82\x81\x10a\x19\xF0W\x7F\xF2\xAF\x11\xFA\xD7=CI\xC9\x9C\xF6/)\x8D3vA\xEA\x0B\xB7\xC0\xF5\xA8\xDB\x92\xA9\x8A'_sOX\x87\x87\x87a\x19\x99a\x19\x88\x89a\x19\x94a\x19\x8Ea\x19\x7Fa\x19u\x84\x80a9\"V[` \x81\x01\x90a<\xA0V[\x94\x90\x93\x80a9\"V[\x80a<\xA0V[\x90aG5V[aG\x9DV[a\x19\xB0a\x19\xA4a\x03\xD5V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x19\xCC\x81a\x18]\x86\x86\x86aHeV[Q\x90 a\x19\xDBa\x18\xA4\x83a9\x18V[Ua\x19\xEB`@Q\x92\x83\x92\x83aHeV[\x03\x90\xA1\0[\x80a\x1ALa\x1A(a\x1A#a\x1A\x17a\x1A\n`\x01\x96\x88Qa\"\x9CV[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[aFuV[a\x1AEa\x1A\x17\x8Ca\x1A@a\x1A\n\x87` \x8B\x01Qa\"\x9CV[aF\xA1V[\x11\x15aF\xC4V[\x01a\x190V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03jW`\xC06`\x03\x19\x01\x12a\x03jW`\x045a\x1A\xFE\x81a\x06\x89V[a\x1B~`$5a\x1B\r\x81a\x06\x89V[`D5a\x1B\x19\x81a\x06\x89V[`d5a\x1B%\x81a\x06\x89V[`\x845\x91a\x1B2\x83a\x06\x89V[`\xA45\x93a\x1B?\x85a\x06\x89V[_T\x96a\x1Bd`\xFF`\x08\x8A\x90\x1C\x16\x15\x80\x99\x81\x9Aa\x1B\xFCW[\x81\x15a\x1B\xDCW[PaH\x8FV[\x87a\x1Bu`\x01`\xFF\x19_T\x16\x17_UV[a\x1B\xC5WaH\xF2V[a\x1B\x84W\0[a\x1B\x92a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x19\xEBV[a\x1B\xD7a\x01\0a\xFF\0\x19_T\x16\x17_UV[aH\xF2V[0;\x15\x91P\x81a\x1B\xEEW[P_a\x1B^V[`\xFF\x16`\x01\x14\x90P_a\x1B\xE7V[`\x01`\xFF\x82\x16\x10\x91Pa\x1BWV[`@\x90a\x06\xF5\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x07\xFBV[4a\x03jW``6`\x03\x19\x01\x12a\x03jW`\x045a\x1C>\x81a\x06\x89V[`$5`D5a\x1CM\x81a\x05\xFEV[a\x1C\x8Ea\x1CXa\";V[\x92\x80a\x1Cc\x85a\"\x8FV[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a.\x89V[\x03\x81\x87Z\xFA\x93\x84\x15a\x03eW\x83a\x1C\xB8a\x18\xF8a\x10\xA3a\x1C\xED\x98` \x97_\x91a\x1DKW[Pa\"\x8FV[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x03eWa\x1D\x1C\x92_\x91a\x1D,W[P`\x01`\x01`\xC0\x1B\x03\x16\x92a\x1D\x16\x84aQ*V[\x90a$\xFDV[\x90a\x07\x8C`@Q\x92\x83\x92\x83a\x1C\nV[a\x1DE\x91P` =` \x11a\x11,Wa\x11\x1E\x81\x83a\x03\xA2V[_a\x1D\x02V[a\x1D_\x91P=\x80_\x83>a\x11V\x81\x83a\x03\xA2V[_a\x1C\xB2V[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03jW``6`\x03\x19\x01\x12a\x03jW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x1D\xD9\x906\x90`\x04\x01a\x17\x03V[`$5\x90a\x1D\xE6\x82a\x05\xFEV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x1E\x05\x906\x90`\x04\x01a\r\x1FV[`\xCFT\x90\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1E\xEEWa\x034\x93a\x1ERa\x1Ed\x92a\x1E\xD8\x95a\x1E=a\x1E5a\x03\xE4V[\x966\x90a97V[\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01R6\x91a\x07\xC5V[`@\x84\x01Rc\xFF\xFF\xFF\xFF\x16``\x83\x01RV[`@Q` \x81\x01\x90a\x1Ez\x81a\x18]\x85\x85aI[V[Q\x90 a\x1E\x8Fa\x18z`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\xA6\xB1\x91-\x06o\xD3\xA5A:b\"\xA5xZ\xE4?$$X\xF3\xEE\xF4\\P\xDD\x80\x89\x87\xB8x\xFD`@Q\x80a\x1E\xD0c\xFF\xFF\xFF\xFF\x86\x16\x94\x82aI[V[\x03\x90\xA2a;\x1EV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW`\x045a\x1FZ\x81a\x06\x89V[a\x1FbaPNV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1FzWa\x034\x90aP\xA6V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` `@Q`d\x81R\xF3[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03eW_\x91a \xACW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a \x9DWa k`fT\x19\x82\x19\x81\x16\x14a!\x01V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a \xC5\x91P` =` \x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[_a JV[\x90\x81` \x91\x03\x12a\x03jWQa\x06\xF5\x81a\tDV[`@Q=_\x82>=\x90\xFD[\x15a \xF2WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a!\x08WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a!<W`\x05\x1B\x01\x90V[a!\x17V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\"1a\"\x0Ea\"7\x95a\"\x08a\"\x01\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra!\xD8\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xA2V[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aJUV[\x90aJ\x9BV[\x92a\"\x08a\"#a\"\x1DaJ\xFDV[\x94aK\xF4V[\x91a\",aM\x10V[aJUV[\x91aMDV[\x90\x91V[`@\x80Q\x90\x91\x90a\"L\x83\x82a\x03\xA2V[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a\"g\x82a\x06\x9AV[a\"t`@Q\x91\x82a\x03\xA2V[\x82\x81R\x80\x92a\"\x85`\x1F\x19\x91a\x06\x9AV[\x01\x90` 6\x91\x017V[\x80Q\x15a!<W` \x01\x90V[\x80Q\x82\x10\x15a!<W` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03jWQ\x90V[\x91\x90\x91a\"\xCC\x83Qa\"]V[\x92_[\x81Q\x81\x10\x15a#\x7FW\x80` a\"\xF8a\"\xEBa#!\x94\x86a\"\x9CV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03eW`\x01\x92_\x91a#QW[Pa#J\x82\x88a\"\x9CV[R\x01a\"\xCFV[a#r\x91P` =\x81\x11a#xW[a#j\x81\x83a\x03\xA2V[\x81\x01\x90a\"\xB0V[_a#?V[P=a#`V[PPPV[\x90\x81` \x91\x03\x12a\x03jWQa\x06\xF5\x81a\x06\x89V[\x90a#\xA3\x82a\x06\x9AV[a#\xB0`@Q\x91\x82a\x03\xA2V[\x82\x81R` \x81\x93a#\xC3`\x1F\x19\x91a\x06\x9AV[\x01\x91\x01_[\x82\x81\x10a#\xD4WPPPV[``\x82\x82\x01R` \x01a#\xC8V[\x90\x81Q\x81\x10\x15a!<W\x01` \x01\x90V[` \x81\x83\x03\x12a\x03jW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03jW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x81Qa$&\x81a\x06\x9AV[\x92a$4`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03jW` \x01\x90[\x82\x82\x10a$\\WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a$OV[\x90a$v\x82a\x06\x9AV[a$\x83`@Q\x91\x82a\x03\xA2V[\x82\x81R\x80\x92a$\x94`\x1F\x19\x91a\x06\x9AV[\x01_[\x81\x81\x10a$\xA3WPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x03\x9DW` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a$\x97V[\x90\x81` \x91\x03\x12a\x03jWQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x03jW\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x03eW_\x95a(\x12W[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x03eW`\x04\x96_\x93a'\xF0W[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x03eW_\x96a'\xCFW[Pa%\x8B\x85\x93\x92\x95Qa#\x99V[\x94_\x93[\x80Q\x85\x10\x15a'\xC5Wa%\xBCa%\xB6a%\xA8\x87\x84a#\xE2V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x03eW_\x94a'\xA1W[Pa&\x0C\x84Qa$lV[a&\x16\x88\x8Ba\"\x9CV[Ra&!\x87\x8Aa\"\x9CV[P_[\x84Q\x81\x10\x15a'\x90W\x80` a&=a&_\x93\x88a\"\x9CV[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03eW_\x92a'pW[Pa&\x85\x81\x87a\"\x9CV[Q\x8A` \x8Aa&\x94\x85\x8Ba\"\x9CV[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x03eWa''\x8C\x8Fa'\"`\x01\x98a'9\x97\x89\x97_\x92a'@W[Pa'\ra&\xFEa\x03\xF3V[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a\"\x9CV[Q\x90a'3\x83\x83a\"\x9CV[Ra\"\x9CV[P\x01a&$V[a'b\x91\x92P` =\x81\x11a'iW[a'Z\x81\x83a\x03\xA2V[\x81\x01\x90a$\xDEV[\x90_a&\xF2V[P=a'PV[a'\x89\x91\x92P` =\x81\x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x90_a&zV[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa%\x8FV[a'\xBE\x91\x94P=\x80_\x83>a'\xB6\x81\x83a\x03\xA2V[\x81\x01\x90a#\xF3V[\x92_a&\x01V[PPP\x93PPP\x90V[a'\xE9\x91\x96P` =` \x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x94_a%}V[` \x91\x93Pa(\x0B\x90\x82=\x84\x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x92\x90a%XV[a(,\x91\x95P` =` \x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x93_a%/V[`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x9DW`@R``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x03jW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03jW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x81Qa(\x9A\x81a\x06\x9AV[\x92a(\xA8`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03jW` \x01\x90[\x82\x82\x10a(\xD0WPPP\x90V[` \x80\x91\x83Qa(\xDF\x81a\x05\xFEV[\x81R\x01\x91\x01\x90a(\xC3V[c\xFF\xFF\xFF\xFF\x90\x91\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x83\x90R`\x01`\x01`\xFB\x1B\x03\x83\x11a\x03jW``\x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF5\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a)!V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a)\x83W`\x01\x01\x90V[a)^V[\x91\x90\x81\x10\x15a!<W`\x05\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x03jWQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03jW\x90V[\x15a)\xBEWV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a!<W\x01\x90V[\x90\x81` \x91\x03\x12a\x03jWQa\x06\xF5\x81a\x05\xFEV[_\x19\x81\x14a)\x83W`\x01\x01\x90V[\x91a*\x1A` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a)!V[\x94\x16\x91\x01RV[\x95\x93\x94\x95\x92\x90\x91\x92a*1a(3V[P`@Qch0H5`\xE0\x1B\x81R\x93`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90` \x85`\x04\x81\x86Z\xFA\x94\x85\x15a\x03eW_\x95a.hW[Pa*na(3V[\x94`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a*\x8E\x8D\x8D\x8B`\x04\x85\x01a(\xEAV[\x03\x81\x88Z\xFA\x90\x81\x15a\x03eW_\x91a.NW[P\x86R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x92\x90_\x81\x80a*\xD1\x85\x87\x8B`\x04\x85\x01a)AV[\x03\x81\x87Z\xFA\x90\x81\x15a\x03eW_\x91a.4W[P`@\x87\x01Ra*\xF3\x81a#\x99V[\x98``\x87\x01\x99\x8AR_[`\xFF\x81\x16\x83\x81\x10\x15a-\x7FW\x88_a+&\x83\x8Fa+\x19\x88a\"]V[\x90Q\x90a'3\x83\x83a\"\x9CV[P_\x8A\x86\x8F[\x81\x84\x10a+\xA9WPPPP\x90P\x8Ca+C\x82a\"]V[\x91_[\x81\x81\x10a+pWPP\x91a+e\x91a+k\x94\x93Q\x90a'3\x83\x83a\"\x9CV[Pa)rV[a*\xFDV[\x80a+\xA3a+\x8Ea\x10\xA3`\x01\x94a+\x88\x8A\x89Qa\"\x9CV[Qa\"\x9CV[a+\x98\x83\x88a\"\x9CV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x01a+FV[a\x10\xA3\x84a+\xBE\x81` \x96\x95a+\xC6\x95a)\x88V[5\x97Qa\"\x9CV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x87\x01R\x16`D\x85\x01R\x83`d\x81\x8DZ\xFA\x80\x15a\x03eW\x88\x8F\x88\x8A\x91\x8F\x94a,k`\x01a,^\x81\x93\x8D\x80\x9D_\x92a-SW[Pa%\xB6a,:a,H\x92a,3\x87\x80`\xC0\x1B\x03\x86\x16\x15\x15a)\xB7V[\x8B\x8Da)\xCDV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a,\x87W[PPPPP`\x01\x91\x92P\x01\x90\x8A\x91\x8A\x86\x8Fa+,V[\x85\x97a,\xA9\x93a,\xA2` \x97\x99\x98a%\xB6\x95a,:\x95a)\x88V[5\x95a)\xCDV[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`D\x84\x01R\x82`d\x81\x8CZ\xFA\x90\x81\x15a\x03eW\x8Fa-\x07\x90a-\x0C\x93\x83\x88`\x01\x97_\x93a-\x1BW[Pa+\x88\x90a+\x98\x93\x94Qa\"\x9CV[a)\xEEV[\x90P\x82\x91\x8A\x88\x8F\x88\x8A\x91a,qV[a+\x98\x93P\x90a-Da+\x88\x92` =\x81\x11a-LW[a-<\x81\x83a\x03\xA2V[\x81\x01\x90a)\xD9V[\x93P\x90a,\xF7V[P=a-2V[a,H\x91\x92Pa,:a-va%\xB6\x92` =\x81\x11a\x11,Wa\x11\x1E\x81\x83a\x03\xA2V[\x93\x92PPa,\x16V[PPP\x92\x90\x95\x97P`\x04\x94\x96P` \x91P`@Q\x94\x85\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x90\x81\x15a\x03eWa-\xD5\x94_\x94\x85\x93a.\x13W[P`@Qc5IR\xA3`\xE2\x1B\x81R\x95\x86\x94\x85\x93\x84\x93`\x04\x85\x01a)\xFCV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03eW_\x91a-\xF9W[P` \x82\x01R\x90V[a.\r\x91P=\x80_\x83>a\x11V\x81\x83a\x03\xA2V[_a-\xF0V[a.-\x91\x93P` =` \x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x91_a-\xB7V[a.H\x91P=\x80_\x83>a\x11V\x81\x83a\x03\xA2V[_a*\xE4V[a.b\x91P=\x80_\x83>a\x11V\x81\x83a\x03\xA2V[_a*\xA1V[a.\x82\x91\x95P` =` \x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x93_a*eV[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF5\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x06\xB1V[`@Q\x90a.\xB3\x82a\x03\x82V[``` \x83\x82\x81R\x01RV[\x15a.\xC6WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a.\xDBWV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a.\xF1WV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a/\x07WV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03jWQa\x06\xF5\x81a\x0FuV[_\x19\x81\x01\x91\x90\x82\x11a)\x83WV[\x15a/@WV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90`\x01\x82\x01\x80\x92\x11a)\x83WV[\x90`\x02\x82\x01\x80\x92\x11a)\x83WV[\x90`\x03\x82\x01\x80\x92\x11a)\x83WV[\x90`\x04\x82\x01\x80\x92\x11a)\x83WV[\x90`\x05\x82\x01\x80\x92\x11a)\x83WV[\x91\x90\x82\x01\x80\x92\x11a)\x83WV[\x15a/\xA9WV[c\xAF\xFC^\xDB`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03jWQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03jW\x90V[\x15a/\xE0WV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a)\x83WV[\x15a0\x16WV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a0,WV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a0eWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a0XV[\x94\x93\x92\x90\x91\x93a0\x89a.\xA6V[Pa0\x95\x85\x15\x15a.\xBFV[`@\x84\x01QQ\x85\x14\x80a9\nW[\x80a8\xFCW[\x80a8\xEEW[a0\xB8\x90a.\xD4V[a0\xCA` \x85\x01QQ\x85QQ\x14a.\xEAV[a0\xE1c\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a/\0V[a0\xE9a\x03\xD5V[_\x81R_` \x82\x01R\x92a0\xFBa.\xA6V[a1\x04\x87a\"]V[` \x82\x01Ra1\x12\x87a\"]V[\x81Ra1\x1Ca.\xA6V[\x92a1+` \x88\x01QQa\"]V[\x84Ra1;` \x88\x01QQa\"]V[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03eWa1\xA4\x91_\x91a8\xBFW[Pa1\x9F6\x8B\x87a\x07\xC5V[aN\xB0V[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a3\x16W` \x88a2\x0Ba\x10\xA3\x8Ca2\x03\x8F\x96\x86\x8Ea1\xE8a1\xD5\x86\x80\x95a\"\x9CV[Q\x80Q_R` \x01Q` R`@_ \x90V[a1\xF5\x84\x84\x84\x01Qa\"\x9CV[R\x82a2\xE3W[\x01Qa\"\x9CV[Q\x95Qa\"\x9CV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x03eWa\"\x08\x8Aa2\xB8\x8Fa2\xB1\x8F\x84` \x8F\x92a2\xA8\x93a2\xA0\x84`\x01\x9Ea2\xBE\x9E_\x91a2\xC6W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\"\x9CV[R\x01Qa\"\x9CV[Q\x93\x8DQa\"\x9CV[Q\x16aN\xDBV[\x90aO\x0CV[\x97\x01\x96a1\xA8V[a2\xDD\x91P\x86=\x81\x11a\x11,Wa\x11\x1E\x81\x83a\x03\xA2V[_a2\x91V[a3\x11a2\xF3\x84\x84\x84\x01Qa\"\x9CV[Qa3\n\x84\x84\x01Qa3\x04\x87a/+V[\x90a\"\x9CV[Q\x10a/9V[a1\xFCV[P\x90\x95\x97\x94\x96Pa3+\x91\x98\x93\x92\x99PaO\xC9V[\x91a38`\x97T`\xFF\x16\x90V[\x90\x81\x15a8\xB7W`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03eW_\x91a8\x98W[P\x91\x90[_\x92[\x81\x84\x10a3\xE9WPPPPP\x92a3\xD0a3\xCBa3\xC4a3\xE3\x95\x85a\x18]\x98`\x80``` \x99\x01Q\x92\x01Q\x92a!UV[\x91\x90a0\x0FV[a0%V[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a0;V[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da7\x92W[a\x10\xA3\x82`\xA0a4>a%\xB6a,:\x84a4F\x97a48a4*a1\xD5\x8F\x9C`@` \x9F\x9E\x01Qa\"\x9CV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba)\xCDV[\x97\x01Qa\"\x9CV[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03eWa5\na\x10\xA3\x8F\x95\x8F\x90a5\x02\x8F\x97\x8F\x96\x84\x8Fa4\xFC`\xC0\x96a4\xF5\x84\x8F` \x9F\x90a1\xFCa,:\x99`@\x93a%\xB6\x9C_\x91a7dW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a/\xD9V[Q\x90aJ\x9BV[\x9Ca)\xCDV[\x96\x01Qa\"\x9CV[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03eWa5\x97\x91\x8C\x8F\x92_\x92a7@W[P` a5\x89\x92\x93\x01Qa\"\x9CV[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a5\xB7\x8Ca5\x89\x8Ca5\xB0a\x1A\n\x82` \x86\x01Qa\"\x9CV[\x92Qa\"\x9CV[_\x98_[` \x8A\x01QQ\x81\x10\x15a7'W\x8B\x8Da5\xF9\x89a5\xECa%\xB6a,:\x86\x8F\x89a5\xE4\x91Qa\"\x9CV[Q\x94\x87a)\xCDV[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a6\x08W[PP`\x01\x01a5\xBBV[\x8A\x8Aa6\x8A\x85\x9F\x94\x8F\x96\x86a+\x88\x8F\x93`\xE0a6Aa\x10\xA3\x95` a69a%\xB6a,:\x83\x9Fa6J\x9C\x89\x91a)\xCDV[\x9A\x01Qa\"\x9CV[Q\x9B\x01Qa\"\x9CV[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03eW\x8Fa6\xF6\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a7\x01W[Pa6\xF0a5\x89\x92\x93Q\x93a6\xEBa\x1A\n\x84\x87a\"\x9CV[a/\xEFV[\x92a\"\x9CV[\x01\x9A\x90P\x8B\x8Da5\xFEV[a5\x89\x92Pa7 a6\xF0\x91` =\x81\x11a'iWa'Z\x81\x83a\x03\xA2V[\x92Pa6\xD3V[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a3\x93V[a5\x89\x92Pa7]` \x91\x82=\x81\x11a'iWa'Z\x81\x83a\x03\xA2V[\x92Pa5zV[` a7\x85\x92P=\x81\x11a7\x8BW[a7}\x81\x83a\x03\xA2V[\x81\x01\x90a/\xB8V[_a4\xDFV[P=a7sV[a7\xCF\x94Pa7\xAC\x92Pa%\xB6\x91a,:\x91` \x95a)\xCDV[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03eW` \x89a4F\x8F\x93\x8F`\xA0\x8F\x97a%\xB6a,:\x8F\x8F\x90a48a4*a1\xD5\x8F`@\x8B\x96\x91\x8F\x88\x93a\x10\xA3\x9Fa8S\x90a8Y\x93a4>\x9F_\x92a8oW[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a/\x95V[\x11a/\xA2V[PPPPPP\x97PPPPPP\x92\x93PPa3\xFEV[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a8\x90\x91=\x81\x11a#xWa#j\x81\x83a\x03\xA2V[\x92\x91Pa8BV[a8\xB1\x91P` =` \x11a-LWa-<\x81\x83a\x03\xA2V[_a3\x8CV[_\x91\x90a3\x90V[a8\xE1\x91P` =` \x11a8\xE7W[a8\xD9\x81\x83a\x03\xA2V[\x81\x01\x90a/\x16V[_a1\x93V[P=a8\xCFV[P`\xE0\x84\x01QQ\x85\x14a0\xAFV[P`\xC0\x84\x01QQ\x85\x14a0\xA9V[P`\xA0\x84\x01QQ\x85\x14a0\xA3V[5a\x06\xF5\x81a\x05\xFEV[\x905\x90`>\x19\x816\x03\x01\x82\x12\x15a\x03jW\x01\x90V[\x91\x90`@\x83\x82\x03\x12a\x03jW`@Q\x90a9P\x82a\x03\x82V[\x81\x93\x805`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x82a9n\x91\x83\x01a\x0B\x9FV[\x83R` \x81\x015\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03jW` \x92a\x04\xEB\x92\x01a\x0B\x9FV[a\x06\xF5\x906\x90a97V[\x15a9\xA3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a:\x06\x81a\x05\xFEV[\x16\x84R\x015\x91\x01RV[\x91`@a\x03\xD3\x92\x94\x93a:'\x81`\x80\x81\x01\x97a9\xF2V[\x01\x90a9\xF2V[\x15a:5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a:\xA7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a)\x83WV[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a)\x83WV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a)\x83WV[\x15a;oWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[a\x06\xF5\x91` a;\xF3\x83Q`@\x84R`@\x84\x01\x90a\x0B\xF2V[\x92\x01Q\x90` \x81\x84\x03\x91\x01Ra\x0B\xF2V[\x90` a\x06\xF5\x92\x81\x81R\x01\x90a;\xDAV[\x15a<\x1CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03jW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03jW` \x01\x91\x816\x03\x83\x13a\x03jWV[` \x81\x83\x03\x12a\x03jW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03jW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x81Qa=\x05\x81a\x06\x9AV[\x92a=\x13`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03jW` \x01\x90[\x82\x82\x10a=;WPPP\x90V[` \x80\x91\x83Qa=J\x81a\x06\x89V[\x81R\x01\x91\x01\x90a=.V[`@Q\x90a=d`@\x83a\x03\xA2V[`\x12\x82Rq9\xB60\xB9\xB4/\xBA42\xAF\xB7\xB82\xB90\xBA7\xB9`q\x1B` \x83\x01RV[\x91\x90` \x83R`\xC0\x83\x01\x92`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x93`\xA0``\x83\x01R\x84Q\x80\x91R` `\xE0\x83\x01\x95\x01\x90_[\x81\x81\x10a>\nWPPP`\x80a=\xF5a\x06\xF5\x94\x95``\x85\x01Q`\x1F\x19\x85\x83\x03\x01\x84\x86\x01Ra\x06\xB1V[\x92\x01Q\x90`\xA0`\x1F\x19\x82\x85\x03\x01\x91\x01Ra\x0B\xF2V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x96\x87\x01\x96\x90\x92\x01\x91`\x01\x01a=\xCCV[\x90\x92\x91`\x01a>7\x85a9\x18V[\x94` a>La>G\x86\x80a9\"V[a9\x91V[a>oa>g\x89c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a9\x9CV[a>\xAAa>\x8A\x89c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T`@Q\x84\x81\x01\x90a>\xA1\x81a\x18]\x8B\x8A\x86a:\x10V[Q\x90 \x14a:.V[a>\xD5a>\xCFa>\xC8\x8Ac\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a:\xA0V[a>\xFAa>\xECa\x18\xF8a>\xE7\x88a9\x18V[a;6V[c\xFF\xFF\xFF\xFFC\x16\x11\x15a;hV[`@Qa?\x0E\x81a\x18]\x85\x82\x01\x94\x85a<\x04V[Q\x90 \x91a?\x1B\x83aP\xEEV[\x015\x90a?'\x82aP\xEEV[\x14\x14aD\x16Wa?7\x83Qa\"]V[\x93_[\x84Q\x81\x10\x15a?dW\x80a?Sa1\xD5`\x01\x93\x88a\"\x9CV[a?]\x82\x89a\"\x9CV[R\x01a?:V[P\x90\x92\x93\x91\x94a?\x9E` \x87\x01\x94` a?}\x87a9\x18V[`@Qa?\x92\x81a\x18]\x8A\x86\x83\x01\x95\x86a0;V[Q\x90 \x91\x015\x14a<\x15V[a?\xA8\x85Qa\"]V[\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94_[\x87Q\x81\x10\x15a@XW\x80` a?\xEFa@\x0F\x93\x89a\"\x9CV[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8BZ\xFA\x91\x82\x15a\x03eW`\x01\x92a@4\x91_\x91a@:W[Pa\x0B^\x83\x8Da\"\x9CV[\x01a?\xD6V[a@R\x91P` =\x81\x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[_a@)V[P\x92\x96\x95P\x92P\x92a@\xB7a@\x80a@\x88`@\x86\x01\x94a@x\x86\x88a<\xA0V[\x93\x90\x91a9\x18V[\x926\x91a\x07\xC5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a$\xFDV[\x94_\x91[\x86Q\x83\x10\x15aC\xB8W_\x97\x96\x97\x95[a@\xD4\x84\x8Aa\"\x9CV[QQ\x87\x10\x15aC\xA8W\x97aA\x15\x98` \x80a@\xF3\x8Aa+\x88\x89\x87a\"\x9CV[Q\x01Q`@Q\x80\x9C\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x99\x8A\x15a\x03eW_\x9AaC\x88W[P`\x01\x98_[\x85Q\x81\x10\x15aCzWaASaAGa\"\xEB\x83\x89a\"\x9CV[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x14aAjW`\x01\x01aA.V[P\x99\x90\x97\x91\x98P`\x01_[\x15\x15\x14aA\x8AW[P`\x01\x01\x95\x97\x96\x97a@\xCAV[\x98\x96\x97\x86\x98_\x87\x95\x93\x98aA\xEF`\xFFaA\xC7a%\xB6a,:\x8CaA\xC1aBD\x9FaA\xBB`\xD2T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x98a<\xA0V[\x90a)\xCDV[aA\xE1aA\xD2a\x03\xD5V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[\x16c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`\xD1TaB\x06\x90aAG\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\x10]\xEA\x1F`\xE2\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\x04\x82\x01R` \x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16`$\x83\x01R\x90\x98\x89\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x96\x87\x15a\x03eW_\x97aCVW[PaBa\x87Qa\"]V[\x98_[\x8AQ\x81\x10\x15aB\x8AW\x80g\x01cEx]\x8A\0\0aB\x83`\x01\x93\x8Ea\"\x9CV[R\x01aBdV[P\x9A\x92\x94\x96aB\xC8`\xFFaB\xAFa%\xB6a,:\x8F\x9D\x97\x9F\x96\x9E\x96aA\xC1\x8E\x8E\x92a<\xA0V[aB\xBAa&\xFEa\x04\x02V[\x16c\xFF\xFF\xFF\xFF\x16` \x86\x01RV[`@\x84\x01R``\x83\x01RaB\xDAa=UV[`\x80\x83\x01R`\xD0TaB\xF6\x90aAG\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03jW`@Qcjf\x9BA`\xE0\x1B\x81R\x92_\x91\x84\x91\x82\x90\x84\x90\x82\x90aC\"\x90`\x04\x83\x01a=\x85V[\x03\x92Z\xF1\x91\x82\x15a\x03eW`\x01\x92aC<W[P\x90aA}V[\x80aCJ_aCP\x93a\x03\xA2V[\x80a\x05\x8CV[_aC5V[aCs\x91\x97P=\x80_\x83>aCk\x81\x83a\x03\xA2V[\x81\x01\x90a<\xD2V[\x95_aBVV[P\x99\x90\x97\x91\x98`\x01\x90aAuV[aC\xA1\x91\x9AP` =\x81\x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x98_aA(V[\x96\x97\x96\x95P`\x01\x90\x92\x01\x91a@\xBBV[PPPPP\x91\x90PaC\xE8aC\xDB\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3V[PPPc\xFF\xFF\xFF\xFF3\x91\x16\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05_\x80\xA3V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x03jW\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x03jW\x816\x03\x83\x13a\x03jWV[` \x81R\x815\x90`>\x19\x836\x03\x01\x82\x12\x15a\x03jW`\x80aE%``aE\x1EaD\xE4\x87a\x06\xF5\x97\x01\x85` \x88\x01RaD\xD2aD\xC7aD\xB6\x83\x80aDGV[`@`\xA0\x8C\x01R`\xE0\x8B\x01\x91a)!V[\x91` \x81\x01\x90aDGV[\x88\x83\x03`\x9F\x19\x01`\xC0\x8A\x01R\x90a)!V[aE\0aD\xF3` \x8A\x01a\x06\x0CV[c\xFF\xFF\xFF\xFF\x16`@\x88\x01RV[aE\r`@\x89\x01\x89aDGV[\x87\x83\x03`\x1F\x19\x01\x85\x89\x01R\x90a)!V[\x95\x01a\x06\x0CV[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15aE7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15aE\xA9WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[\x15aF\nWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[`@\x81\x01\x92\x91a\x03\xD3\x91\x90a9\xF2V[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a)\x83WV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a)\x83WV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a)\x83WV[\x15aF\xCBWV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[` \x90\x82`@Q\x93\x84\x92\x837\x81\x01`\xCD\x81R\x03\x01\x90 \x90V[`\x1F\x82\x11aG[WPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10aG\x93W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10aG\x88WPPV[_\x81U`\x01\x01aG}V[\x90\x91P\x81\x90aGtV[\x90\x92\x91`\x01`\x01`@\x1B\x03\x81\x11a\x03\x9DWaG\xC2\x81aG\xBC\x84Ta\x0B\xBAV[\x84aGNV[_`\x1F\x82\x11`\x01\x14aH\0W\x81\x90aG\xF1\x93\x94\x95_\x92aG\xF5W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90UV[\x015\x90P_\x80aG\xDDV[`\x1F\x19\x82\x16\x94aH\x13\x84_R` _ \x90V[\x91_[\x87\x81\x10aHMWP\x83`\x01\x95\x96\x97\x10aH4W[PPP\x81\x1B\x01\x90UV[\x015_\x19`\x03\x84\x90\x1B`\xF8\x16\x1C\x19\x16\x90U_\x80\x80aH*V[\x90\x92` `\x01\x81\x92\x86\x86\x015\x81U\x01\x94\x01\x91\x01aH\x16V[\x90\x92\x91` ``\x91aH{\x84`\x80\x81\x01\x97a9\xF2V[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[\x15aH\x96WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aH\xFB\x90aP\xA6V[`\xCE\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xCF\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x83\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\xD0\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD2\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[` \x81R`\x80c\xFF\xFF\xFF\xFF``aI\xA1aI\x80\x86Q\x85` \x88\x01R`\xA0\x87\x01\x90a;\xDAV[\x83` \x88\x01Q\x16`@\x87\x01R`@\x87\x01Q`\x1F\x19\x87\x83\x03\x01\x84\x88\x01Ra\x0B\xF2V[\x94\x01Q\x16\x91\x01R\x90V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aJ\x1E\x82a\x03\x82V[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aJ:\x81\x84a\x03\xA2V[6\x837V[`@Q\x90aJN` \x83a\x03\xA2V[` 6\x837V[\x91\x90`@\x90``aJdaJ\x11V[\x94\x85\x92` \x85Q\x92aJv\x85\x85a\x03\xA2V[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aJ\x99WV[\xFE[` \x92\x91`\x80`@\x92aJ\xACaJ\x11V[\x95\x86\x93\x81\x86Q\x93aJ\xBD\x86\x86a\x03\xA2V[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aJ\x99W\x15aJ\xEEWV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@QaK\t\x81a\x03\x82V[`@\x90\x81QaK\x18\x83\x82a\x03\xA2V[\x826\x827\x81R` \x82Q\x91aK-\x84\x84a\x03\xA2V[\x836\x847\x01R\x80QaK?\x82\x82a\x03\xA2V[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aK\x95\x83\x83a\x03\xA2V[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaK\xEA\x83Q\x93\x84a\x03\xA2V[\x82R` \x82\x01R\x90V[_Q` aRm_9_Q\x90_R\x90aL\x0BaJ\x11V[P_\x91\x90\x06` `\xC0\x83[aM\x0BW_\x93_Q` aRm_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaLA\x85\x82a\x03\xA2V[\x846\x827\x84\x81\x85`@QaLU\x82\x82a\x03\xA2V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aRm_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aJ\x99WaL\xBF\x90aRVV[Q\x91aM\x0BW_Q` aRm_9_Q\x90_R\x82\x80\t\x14aL\xF6WP_Q` aRm_9_Q\x90_R`\x01_\x94\x08\x92\x93aL\x16V[\x92\x93PPaM\x02a\x03\xD5V[\x92\x83R\x82\x01R\x90V[a!AV[aM\x18aJ\x11V[P`@QaM%\x81a\x03\x82V[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x0C\x81\x10\x15a!<W`\x05\x1B\x01\x90V[\x93\x92\x90\x91aMR`@a\x04\x11V[\x94\x85R` \x85\x01RaMd`@a\x04\x11V[\x91\x82R` \x82\x01RaMtaJ)V[\x92_[`\x02\x81\x10aM\xA1WPPP` a\x01\x80\x92aM\x90aJ?V[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aM\xAD`\x01\x92aF\x8BV[aM\xB7\x82\x85a!+V[QQaM\xC3\x82\x89aM3V[R` aM\xD0\x83\x86a!+V[Q\x01QaM\xE5aM\xDF\x83a/OV[\x89aM3V[RaM\xF0\x82\x86a!+V[QQQaM\xFFaM\xDF\x83a/]V[RaN\x15aN\r\x83\x87a!+V[QQ` \x01\x90V[QaN\"aM\xDF\x83a/kV[R` aN/\x83\x87a!+V[Q\x01QQaN?aM\xDF\x83a/yV[RaNkaNeaN^` aNU\x86\x8Aa!+V[Q\x01Q` \x01\x90V[Q\x92a/\x87V[\x88aM3V[R\x01aMwV[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01aN\xBE`\xFF\x93aQ\xDEV[\x92\x83\x92\x16\x1B\x11\x15aN\xCCW\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[aN\xE7WP\x90V[_\x19\x81\x01\x81\x81\x11a)\x83Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a)\x83W`\x01\x01\x90\x80aN\xDFV[\x90aO\x15aJ\x11V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aO\xBAW`\x01\x82\x14aO\xB5WaO6a\x03\xD5V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aO[WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aO\x95W[`\x01aO\x8BaO\x80\x83`\xFF\x94aJ\x9BV[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aOGV[\x94`\x01aO\x8BaO\x80aO\xAA\x89`\xFF\x95aJ\x9BV[\x98\x93PPPPaOoV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[aO\xD1aJ\x11V[P\x80Q\x90\x81\x15\x80aPBW[\x15aO\xFEWPP`@QaO\xF2`@\x82a\x03\xA2V[_\x81R_` \x82\x01R\x90V[` _Q` aRm_9_Q\x90_R\x91\x01Q\x06_Q` aRm_9_Q\x90_R\x03_Q` aRm_9_Q\x90_R\x81\x11a)\x83W`@Q\x91aK\xEA\x83a\x03\x82V[P` \x81\x01Q\x15aO\xDDV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aPbWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[_\x80\x91`@Q` \x81\x01\x91c'\xB7\xCF\x85`\xE0\x1B\x83R`$\x82\x01R`$\x81RaQ\x17`D\x82a\x03\xA2V[Q\x90jconsole.logZ\xFAPV[a\xFF\xFFaQ6\x82aN\xDBV[\x16aQ@\x81a\x07\xAAV[\x90aQN`@Q\x92\x83a\x03\xA2V[\x80\x82RaQ]`\x1F\x19\x91a\x07\xAAV[\x016` \x83\x017__[\x82Q\x82\x10\x80aQ\xBDW[\x15aQ\xB6W`\x01\x81\x1B\x84\x16aQ\x8FW[aQ\x8A\x90a)\xEEV[aQgV[\x90`\x01aQ\x8A\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaQ\xAC\x82\x87a#\xE2V[S\x01\x91\x90PaQ\x81V[PP\x90P\x90V[Pa\x01\0\x81\x10aQqV[\x15aQ\xCFWV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aRGW\x81Q\x15aRBW` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aR=W`\x01\x90aR(aR\x1Ea%\xB6a%\xA8\x86\x89a#\xE2V[`\xFF`\x01\x91\x16\x1B\x90V[\x90aR4\x81\x83\x11aQ\xC8V[\x17\x91\x01\x90aQ\xFFV[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15aR]WV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xFC\x92\x8A\xA0^\x81\x97\xE9\x04\xCA\xC0\xBC\xDEG\xA5\x83\\\xCB\x14\x86\x16Px\xB6\xB2\xF5H\x0B\xB6\xD0\xA2ZdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8063136439dd146102aa578063171f1d5b146102a55780631ad43189146101f1578063245a7bfc146102a05780632cb223d51461029b5780632d89f6fc1461029657806331b36bd9146102915780633563b0d11461028c5780633998fdd314610287578063416c7e5e146102825780634d2b57fe1461027d5780634f41b818146102785780634f739f7414610273578063595c6a671461026e5780635a2d7f02146102695780635ac86ab7146102645780635c1556621461025f5780635c975abb1461025a5780635decc3f5146102555780635df4594614610250578063683048351461024b5780636d14a987146102465780636efb463614610241578063715018a61461023c57806372d18e8d1461022d5780637afa1eed14610237578063886f1195146102325780638b00ce7c1461022d5780638da5cb5b1461022857806392a51824146102235780639b290e981461021e578063b702394914610219578063b98d090814610214578063ca8aa7c71461020f578063cc2a9a5b1461020a578063cefdc1d414610205578063df5cf72314610200578063ef029dbc146101fb578063f2fde38b146101f6578063f5c9899d146101f1578063f63c5bab146101ec5763fabc1cbc146101e7575f80fd5b611fe9565b611fce565b610596565b611f3d565b611da9565b611d65565b611c21565b611ae1565b611ab9565b611a97565b6117aa565b611782565b611711565b6116bc565b61162d565b611678565b611650565b6115d2565b611525565b611248565b611204565b6111c0565b611182565b611165565b610fec565b610f80565b610f53565b610ee0565b610e39565b610c27565b610aae565b61094e565b61091c565b6108a2565b6106f8565b610650565b610617565b6105d6565b610524565b3461036a57602036600319011261036a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156103655761033492610320915f91610336575b506120eb565b61032f60665482811614612101565b6149df565b005b610358915060203d60201161035e575b61035081836103a2565b8101906120cb565b5f61031a565b503d610346565b6120e0565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761039d57604052565b61036e565b90601f801991011681019081106001600160401b0382111761039d57604052565b604051906103d3610100836103a2565b565b604051906103d36040836103a2565b604051906103d36080836103a2565b604051906103d36060836103a2565b604051906103d360a0836103a2565b906103d360405192836103a2565b60409060e319011261036a576040519061043882610382565b60e4358252610104356020830152565b919082604091031261036a5760405161046081610382565b6020808294803584520135910152565b9080601f8301121561036a576040519161048b6040846103a2565b82906040810192831161036a57905b8282106104a75750505090565b813581526020918201910161049a565b90608060631983011261036a576040516104d081610382565b60206104eb82946104e2816064610470565b845260a4610470565b910152565b919060808382031261036a5760206104eb6040519261050e84610382565b6040849661051c8382610470565b865201610470565b3461036a5761012036600319011261036a57600435604036602319011261036a5761057c604091825161055681610382565b6024358152604435602082015261056c366104b7565b906105763661041f565b92612155565b8251911515825215156020820152f35b5f91031261036a57565b3461036a575f36600319011261036a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461036a575f36600319011261036a5760ce546040516001600160a01b039091168152602090f35b63ffffffff81160361036a57565b35906103d3826105fe565b3461036a57602036600319011261036a5763ffffffff600435610639816105fe565b165f5260cb602052602060405f2054604051908152f35b3461036a57602036600319011261036a5763ffffffff600435610672816105fe565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b0381160361036a57565b6001600160401b03811161039d5760051b60200190565b90602080835192838152019201905f5b8181106106ce5750505090565b82518452602093840193909201916001016106c1565b9060206106f59281815201906106b1565b90565b3461036a57604036600319011261036a5760043561071581610689565b602435906001600160401b03821161036a573660238301121561036a578160040135916107418361069a565b9261074f60405194856103a2565b8084526024602085019160051b8301019136831161036a57602401905b8282106107905761078c61078086866122bf565b604051918291826106e4565b0390f35b60208091833561079f81610689565b81520191019061076c565b6001600160401b03811161039d57601f01601f191660200190565b9291926107d1826107aa565b916107df60405193846103a2565b82948184528183011161036a578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b83831061082657505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b8082106108665750505060208060019297019301930191939290610817565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610847565b3461036a57606036600319011261036a576004356108bf81610689565b6024356001600160401b03811161036a573660238201121561036a5761078c916108f66109089236906024816004013591016107c5565b60443591610903836105fe565b6124fd565b6040519182916020835260208301906107fb565b3461036a575f36600319011261036a5760d2546040516001600160a01b039091168152602090f35b8015150361036a57565b3461036a57602036600319011261036a5760043561096b81610944565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610365575f916109e0575b506001600160a01b031633036109d15761033490614e72565b637070f3b160e11b5f5260045ffd5b610a02915060203d602011610a08575b6109fa81836103a2565b810190612384565b5f6109b8565b503d6109f0565b9080601f8301121561036a578135610a268161069a565b92610a3460405194856103a2565b81845260208085019260051b82010192831161036a57602001905b828210610a5c5750505090565b8135815260209182019101610a4f565b60206040818301928281528451809452019201905f5b818110610a8f5750505090565b82516001600160a01b0316845260209384019390920191600101610a82565b3461036a57604036600319011261036a57600435610acb81610689565b6024356001600160401b03811161036a57610aea903690600401610a0f565b610af4815161225d565b916001600160a01b03165f5b8251811015610b9157806020610b19610b39938661229c565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561036557600192610b6d915f91610b73575b50610b5e838861229c565b6001600160a01b039091169052565b01610b00565b610b8b915060203d8111610a08576109fa81836103a2565b5f610b53565b6040518061078c8682610a6c565b9080601f8301121561036a578160206106f5933591016107c5565b90600182811c92168015610be8575b6020831014610bd457565b634e487b7160e01b5f52602260045260245ffd5b91607f1691610bc9565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060206106f5928181520190610bf2565b3461036a57602036600319011261036a576004356001600160401b03811161036a576020610c5a81923690600401610b9f565b604051928184925191829101835e810160cd815203019020604051905f90805490610c8482610bba565b8085529160018116908115610cf85750600114610cb8575b61078c84610cac818603826103a2565b60405191829182610c16565b5f90815260208120939250905b808210610cde57509091508101602001610cac82610c9c565b919260018160209254838588010152019101909291610cc5565b60ff191660208087019190915292151560051b85019092019250610cac9150839050610c9c565b9181601f8401121561036a578235916001600160401b03831161036a576020838186019501011161036a57565b90602080835192838152019201905f5b818110610d695750505090565b825163ffffffff16845260209384019390920191600101610d5c565b90602082526060610dd3610dbe610da884516080602088015260a0870190610d4c565b6020850151868203601f19016040880152610d4c565b6040840151858203601f190184870152610d4c565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610e0c57505050505090565b9091929394602080610e2a600193601f198682030187528951610d4c565b97019301930191939290610dfd565b3461036a57608036600319011261036a57600435610e5681610689565b60243590610e63826105fe565b6044356001600160401b03811161036a57610e82903690600401610d1f565b91606435926001600160401b03841161036a573660238501121561036a578360040135926001600160401b03841161036a573660248560051b8701011161036a5761078c956024610ed4960193612a21565b60405191829182610d85565b3461036a575f36600319011261036a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561036557610f4b915f9161033657506120eb565b6103346149ab565b3461036a575f36600319011261036a57602060405167016345785d8a00008152f35b60ff81160361036a57565b3461036a57602036600319011261036a576020600160ff600435610fa381610f75565b161b806066541614604051908152f35b60206040818301928281528451809452019201905f5b818110610fd65750505090565b8251845260209384019390920191600101610fc9565b3461036a57606036600319011261036a5760043561100981610689565b6024356001600160401b03811161036a57611028903690600401610a0f565b60443591611035836105fe565b6040516361c8a12f60e11b8152906001600160a01b03165f828061105d868860048401612e89565b0381845afa918215610365575f92611141575b5061107b835161225d565b935f5b845181101561113357611091818661229c565b51906020836110ad6110a3848961229c565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa8015610365576001925f91611105575b50828060c01b03166110fe828961229c565b520161107e565b611126915060203d811161112c575b61111e81836103a2565b810190612998565b5f6110ec565b503d611114565b6040518061078c8882610fb3565b61115e9192503d805f833e61115681836103a2565b810190612867565b905f611070565b3461036a575f36600319011261036a576020606654604051908152f35b3461036a57602036600319011261036a5763ffffffff6004356111a4816105fe565b165f5260cc602052602060ff60405f2054166040519015158152f35b3461036a575f36600319011261036a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461036a575f36600319011261036a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461036a575f36600319011261036a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080601f8301121561036a5781356112a38161069a565b926112b160405194856103a2565b81845260208085019260051b82010192831161036a57602001905b8282106112d95750505090565b6020809183356112e8816105fe565b8152019101906112cc565b81601f8201121561036a5780356113098161069a565b9261131760405194856103a2565b81845260208085019260061b8401019281841161036a57602001915b838310611341575050505090565b60206040916113508486610448565b815201920191611333565b9080601f8301121561036a5781356113728161069a565b9261138060405194856103a2565b81845260208085019260051b8201019183831161036a5760208201905b8382106113ac57505050505090565b81356001600160401b03811161036a576020916113ce8784809488010161128c565b81520191019061139d565b9190916101808184031261036a576113ef6103c3565b9281356001600160401b03811161036a578161140c91840161128c565b845260208201356001600160401b03811161036a578161142d9184016112f3565b602085015260408201356001600160401b03811161036a57816114519184016112f3565b604085015261146381606084016104f0565b60608501526114758160e08401610448565b60808501526101208201356001600160401b03811161036a578161149a91840161128c565b60a08501526101408201356001600160401b03811161036a57816114bf91840161128c565b60c08501526101608201356001600160401b03811161036a576114e2920161135b565b60e0830152565b90602080835192838152019201905f5b8181106115065750505090565b82516001600160601b03168452602093840193909201916001016114f9565b3461036a57608036600319011261036a576004356024356001600160401b03811161036a57611558903690600401610d1f565b9091604435611566816105fe565b606435926001600160401b03841161036a576115c89461158d6115939536906004016113d9565b9361307b565b6040519283926040845260206115b4825160408088015260808701906114e9565b910151848203603f190160608601526114e9565b9060208301520390f35b3461036a575f36600319011261036a576115ea61504e565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461036a575f36600319011261036a57602063ffffffff60c95416604051908152f35b3461036a575f36600319011261036a5760cf546040516001600160a01b039091168152602090f35b3461036a575f36600319011261036a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461036a575f36600319011261036a576033546040516001600160a01b039091168152602090f35b9081608091031261036a5790565b604090602319011261036a57602490565b9081604091031261036a5790565b3461036a5760c036600319011261036a576004356001600160401b03811161036a576117419036906004016116e4565b61174a366116f2565b90604036606319011261036a5760a4356001600160401b03811161036a576103349261177c60649236906004016112f3565b92613e29565b3461036a575f36600319011261036a5760d0546040516001600160a01b039091168152602090f35b3461036a57608036600319011261036a576004356001600160401b03811161036a576117da9036906004016116e4565b6117e3366116f2565b6064356001600160401b03811161036a576118029036906004016113d9565b60ce549091906001600160a01b03163303611a525761192761182660208501613918565b936118346040820182613ca0565b92909461184360608401613918565b96611895604051602081019061186b8161185d8985614478565b03601f1981018352826103a2565b51902061188e61187a89613918565b63ffffffff165f5260ca60205260405f2090565b5414614530565b6118bf6118b86118a488613918565b63ffffffff165f5260cb60205260405f2090565b54156145a2565b8463ffffffff4316976119096119016118f87f000000000000000000000000000000000000000000000000000000000000000086613b4e565b63ffffffff1690565b8a1115614603565b604051602081019061191f8161185d8c85614665565b51902061307b565b929060ff5f9716965b8281106119f0577ff2af11fad73d4349c99cf62f298d337641ea0bb7c0f5a8db92a98a275f734f588787876119996119888961199461198e61197f6119758480613922565b6020810190613ca0565b94909380613922565b80613ca0565b90614735565b61479d565b6119b06119a46103d5565b63ffffffff9094168452565b602083015260405160208101906119cc8161185d868686614865565b5190206119db6118a483613918565b556119eb60405192839283614865565b0390a1005b80611a4c611a28611a23611a17611a0a600196885161229c565b516001600160601b031690565b6001600160601b031690565b614675565b611a45611a178c611a40611a0a8760208b015161229c565b6146a1565b11156146c4565b01611930565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b3461036a575f36600319011261036a57602060ff609754166040519015158152f35b3461036a575f36600319011261036a5760d1546040516001600160a01b039091168152602090f35b3461036a5760c036600319011261036a57600435611afe81610689565b611b7e602435611b0d81610689565b604435611b1981610689565b606435611b2581610689565b60843591611b3283610689565b60a43593611b3f85610689565b5f5496611b6460ff60088a901c16158099819a611bfc575b8115611bdc575b5061488f565b87611b75600160ff195f5416175f55565b611bc5576148f2565b611b8457005b611b9261ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016119eb565b611bd761010061ff00195f5416175f55565b6148f2565b303b15915081611bee575b505f611b5e565b60ff1660011490505f611be7565b600160ff8216109150611b57565b6040906106f59392815281602082015201906107fb565b3461036a57606036600319011261036a57600435611c3e81610689565b602435604435611c4d816105fe565b611c8e611c5861223b565b9280611c638561228f565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401612e89565b0381875afa9384156103655783611cb86118f86110a3611ced986020975f91611d4b575b5061228f565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561036557611d1c925f91611d2c575b506001600160c01b031692611d168461512a565b906124fd565b9061078c60405192839283611c0a565b611d45915060203d60201161112c5761111e81836103a2565b5f611d02565b611d5f91503d805f833e61115681836103a2565b5f611cb2565b3461036a575f36600319011261036a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461036a57606036600319011261036a576004356001600160401b03811161036a57611dd9903690600401611703565b60243590611de6826105fe565b6044356001600160401b03811161036a57611e05903690600401610d1f565b60cf54909291906001600160a01b03163303611eee5761033493611e52611e6492611ed895611e3d611e356103e4565b963690613937565b86524363ffffffff16602087015236916107c5565b604084015263ffffffff166060830152565b6040516020810190611e7a8161185d858561495b565b519020611e8f61187a60c95463ffffffff1690565b5560c95463ffffffff16907fa6b1912d066fd3a5413a6222a5785ae43f242458f3eef45c50dd808987b878fd60405180611ed063ffffffff8616948261495b565b0390a2613b1e565b63ffffffff1663ffffffff1960c954161760c955565b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b3461036a57602036600319011261036a57600435611f5a81610689565b611f6261504e565b6001600160a01b03811615611f7a57610334906150a6565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461036a575f36600319011261036a57602060405160648152f35b3461036a57602036600319011261036a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610365575f916120ac575b506001600160a01b0316330361209d5761206b606654198219811614612101565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b6120c5915060203d602011610a08576109fa81836103a2565b5f61204a565b9081602091031261036a57516106f581610944565b6040513d5f823e3d90fd5b156120f257565b631d77d47760e21b5f5260045ffd5b1561210857565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b90600281101561213c5760051b0190565b612117565b634e487b7160e01b5f52601260045260245ffd5b61223161220e6122379561220861220185875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e08401526101008301526121d881610120840103601f1981018352826103a2565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b8096614a55565b90614a9b565b9261220861222361221d614afd565b94614bf4565b9161222c614d10565b614a55565b91614d44565b9091565b6040805190919061224c83826103a2565b6001815291601f1901366020840137565b906122678261069a565b61227460405191826103a2565b8281528092612285601f199161069a565b0190602036910137565b80511561213c5760200190565b805182101561213c5760209160051b010190565b9081602091031261036a575190565b9190916122cc835161225d565b925f5b815181101561237f578060206122f86122eb612321948661229c565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa8015610365576001925f91612351575b5061234a828861229c565b52016122cf565b612372915060203d8111612378575b61236a81836103a2565b8101906122b0565b5f61233f565b503d612360565b505050565b9081602091031261036a57516106f581610689565b906123a38261069a565b6123b060405191826103a2565b828152602081936123c3601f199161069a565b0191015f5b8281106123d457505050565b6060828201526020016123c8565b90815181101561213c570160200190565b60208183031261036a578051906001600160401b03821161036a57019080601f8301121561036a5781516124268161069a565b9261243460405194856103a2565b81845260208085019260051b82010192831161036a57602001905b82821061245c5750505090565b815181526020918201910161244f565b906124768261069a565b61248360405191826103a2565b8281528092612494601f199161069a565b015f5b8181106124a357505050565b6040519060608201918083106001600160401b0384111761039d576020926040525f81525f838201525f604082015282828601015201612497565b9081602091031261036a57516001600160601b038116810361036a5790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa948515610365575f95612812575b50604051634f4c91e160e11b815294602086600481855afa918215610365576004965f936127f0575b5060209060405197888092632efa2ca360e11b82525afa958615610365575f966127cf575b5061258b8593929551612399565b945f935b80518510156127c5576125bc6125b66125a887846123e2565b516001600160f81b03191690565b60f81c90565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa938415610365575f946127a1575b5061260c845161246c565b612616888b61229c565b52612621878a61229c565b505f5b84518110156127905780602061263d61265f938861229c565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610365575f92612770575b50612685818761229c565b518a60208a612694858b61229c565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa938415610365576127278c8f6127226001986127399789975f92612740575b5061270d6126fe6103f3565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b61229c565b5190612733838361229c565b5261229c565b5001612624565b61276291925060203d8111612769575b61275a81836103a2565b8101906124de565b905f6126f2565b503d612750565b61278991925060203d8111610a08576109fa81836103a2565b905f61267a565b50600190960195909450915061258f565b6127be9194503d805f833e6127b681836103a2565b8101906123f3565b925f612601565b5050509350505090565b6127e991965060203d602011610a08576109fa81836103a2565b945f61257d565b602091935061280b90823d8411610a08576109fa81836103a2565b9290612558565b61282c91955060203d602011610a08576109fa81836103a2565b935f61252f565b60405190608082018281106001600160401b0382111761039d57604052606080838181528160208201528160408201520152565b60208183031261036a578051906001600160401b03821161036a57019080601f8301121561036a57815161289a8161069a565b926128a860405194856103a2565b81845260208085019260051b82010192831161036a57602001905b8282106128d05750505090565b6020809183516128df816105fe565b8152019101906128c3565b63ffffffff909116815260406020820181905281018390526001600160fb1b03831161036a5760609260051b809284830137010190565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6106f595931681528160208201520191612921565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff81146129835760010190565b61295e565b919081101561213c5760051b0190565b9081602091031261036a57516001600160c01b038116810361036a5790565b156129be57565b6325ec6c1f60e01b5f5260045ffd5b9082101561213c570190565b9081602091031261036a57516106f5816105fe565b5f1981146129835760010190565b91612a1a60209263ffffffff92969596604086526040860191612921565b9416910152565b9593949592909192612a31612833565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa948515610365575f95612e68575b50612a6e612833565b946040516361c8a12f60e11b81525f8180612a8e8d8d8b600485016128ea565b0381885afa908115610365575f91612e4e575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f8180612ad185878b60048501612941565b0381875afa908115610365575f91612e34575b506040870152612af381612399565b9860608701998a525f5b60ff811683811015612d7f57885f612b26838f612b198861225d565b905190612733838361229c565b505f8a868f5b818410612ba9575050505090508c612b438261225d565b915f5b818110612b7057505091612b6591612b6b94935190612733838361229c565b50612972565b612afd565b80612ba3612b8e6110a3600194612b888a895161229c565b5161229c565b612b98838861229c565b9063ffffffff169052565b01612b46565b6110a384612bbe8160209695612bc695612988565b35975161229c565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561036557888f888a918f94612c6b6001612c5e81938d809d5f92612d53575b506125b6612c3a612c4892612c33878060c01b03861615156129b7565b8b8d6129cd565b356001600160f81b03191690565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14612c87575b5050505050600191925001908a918a868f612b2c565b8597612ca993612ca260209799986125b695612c3a95612988565b35956129cd565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa908115610365578f612d0790612d0c9383886001975f93612d1b575b50612b8890612b9893945161229c565b6129ee565b905082918a888f888a91612c71565b612b98935090612d44612b889260203d8111612d4c575b612d3c81836103a2565b8101906129d9565b935090612cf7565b503d612d32565b612c48919250612c3a612d766125b69260203d811161112c5761111e81836103a2565b93925050612c16565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa90811561036557612dd5945f948593612e13575b5060405163354952a360e21b815295869485938493600485016129fc565b03916001600160a01b03165afa908115610365575f91612df9575b50602082015290565b612e0d91503d805f833e61115681836103a2565b5f612df0565b612e2d91935060203d602011610a08576109fa81836103a2565b915f612db7565b612e4891503d805f833e61115681836103a2565b5f612ae4565b612e6291503d805f833e61115681836103a2565b5f612aa1565b612e8291955060203d602011610a08576109fa81836103a2565b935f612a65565b60409063ffffffff6106f5949316815281602082015201906106b1565b60405190612eb382610382565b60606020838281520152565b15612ec657565b62f8202d60e51b5f5260045ffd5b15612edb57565b6343714afd60e01b5f5260045ffd5b15612ef157565b635f832f4160e01b5f5260045ffd5b15612f0757565b634b874f4560e01b5f5260045ffd5b9081602091031261036a57516106f581610f75565b5f1981019190821161298357565b15612f4057565b633fdc650560e21b5f5260045ffd5b906001820180921161298357565b906002820180921161298357565b906003820180921161298357565b906004820180921161298357565b906005820180921161298357565b9190820180921161298357565b15612fa957565b63affc5edb60e01b5f5260045ffd5b9081602091031261036a575167ffffffffffffffff198116810361036a5790565b15612fe057565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b03821161298357565b1561301657565b6367988d3360e01b5f5260045ffd5b1561302c57565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b8181106130655750505090565b8251845260209384019390920191600101613058565b949392909193613089612ea6565b50613095851515612ebf565b60408401515185148061390a575b806138fc575b806138ee575b6130b890612ed4565b6130ca60208501515185515114612eea565b6130e163ffffffff431663ffffffff841610612f00565b6130e96103d5565b5f81525f6020820152926130fb612ea6565b6131048761225d565b60208201526131128761225d565b815261311c612ea6565b9261312b60208801515161225d565b845261313b60208801515161225d565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610365576131a4915f916138bf575b5061319f368b876107c5565b614eb0565b985f965b602089015180518910156133165760208861320b6110a38c6132038f96868e6131e86131d586809561229c565b5180515f526020015160205260405f2090565b6131f5848484015161229c565b52826132e3575b015161229c565b51955161229c565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa918215610365576122088a6132b88f6132b18f8460208f926132a8936132a08460019e6132be9e5f916132c6575b508f8060c01b0316925161229c565b52015161229c565b51938d5161229c565b5116614edb565b90614f0c565b9701966131a8565b6132dd9150863d811161112c5761111e81836103a2565b5f613291565b6133116132f3848484015161229c565b5161330a8484015161330487612f2b565b9061229c565b5110612f39565b6131fc565b5090959794965061332b919893929950614fc9565b9161333860975460ff1690565b9081156138b7576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610365575f91613898575b5091905b5f925b8184106133e9575050505050926133d06133cb6133c46133e3958561185d9860806060602099015192015192612155565b919061300f565b613025565b015160405192839160208301958661303b565b51902090565b92989596909399919794878b888c888d613792575b6110a38260a061343e6125b6612c3a846134469761343861342a6131d58f9c604060209f9e015161229c565b67ffffffffffffffff191690565b9b6129cd565b97015161229c565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156103655761350a6110a38f958f906135028f978f96848f6134fc60c0966134f5848f60209f906131fc612c3a996040936125b69c5f91613764575b5067ffffffffffffffff19918216911614612fd9565b5190614a9b565b9c6129cd565b96015161229c565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561036557613597918c8f925f92613740575b5060206135899293015161229c565b906001600160601b03169052565b6135b78c6135898c6135b0611a0a82602086015161229c565b925161229c565b5f985f5b60208a015151811015613727578b8d6135f9896135ec6125b6612c3a868f896135e4915161229c565b5194876129cd565b60ff161c60019081161490565b613608575b50506001016135bb565b8a8a61368a859f948f9686612b888f9360e06136416110a39560206136396125b6612c3a839f61364a9c89916129cd565b9a015161229c565b519b015161229c565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610365578f6136f6908f936001959486955f92613701575b506136f0613589929351936136eb611a0a848761229c565b612fef565b9261229c565b019a90508b8d6135fe565b61358992506137206136f09160203d81116127695761275a81836103a2565b92506136d3565b5093919796996001919699509a94929a01929190613393565b613589925061375d602091823d81116127695761275a81836103a2565b925061357a565b602061378592503d811161378b575b61377d81836103a2565b810190612fb8565b5f6134df565b503d613773565b6137cf94506137ac92506125b691612c3a916020956129cd565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610365576020896134468f938f60a08f976125b6612c3a8f8f9061343861342a6131d58f60408b96918f88936110a39f613853906138599361343e9f5f9261386f575b5063ffffffff809116931690612f95565b11612fa2565b50505050505097505050505050929350506133fe565b602063ffffffff9293508291613890913d81116123785761236a81836103a2565b929150613842565b6138b1915060203d602011612d4c57612d3c81836103a2565b5f61338c565b5f9190613390565b6138e1915060203d6020116138e7575b6138d981836103a2565b810190612f16565b5f613193565b503d6138cf565b5060e08401515185146130af565b5060c08401515185146130a9565b5060a08401515185146130a3565b356106f5816105fe565b903590603e198136030182121561036a570190565b919060408382031261036a576040519061395082610382565b819380356001600160401b03811161036a578261396e918301610b9f565b83526020810135916001600160401b03831161036a576020926104eb9201610b9f565b6106f5903690613937565b156139a357565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b6020809163ffffffff8135613a06816105fe565b1684520135910152565b9160406103d3929493613a278160808101976139f2565b01906139f2565b15613a3557565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15613aa757565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b63ffffffff60019116019063ffffffff821161298357565b63ffffffff60649116019063ffffffff821161298357565b9063ffffffff8091169116019063ffffffff821161298357565b15613b6f57565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b6106f5916020613bf38351604084526040840190610bf2565b920151906020818403910152610bf2565b9060206106f5928181520190613bda565b15613c1c57565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b903590601e198136030182121561036a57018035906001600160401b03821161036a5760200191813603831361036a57565b60208183031261036a578051906001600160401b03821161036a57019080601f8301121561036a578151613d058161069a565b92613d1360405194856103a2565b81845260208085019260051b82010192831161036a57602001905b828210613d3b5750505090565b602080918351613d4a81610689565b815201910190613d2e565b60405190613d646040836103a2565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b818110613e0a575050506080613df56106f594956060850151601f1985830301848601526106b1565b9201519060a0601f1982850301910152610bf2565b82516001600160a01b0316875260209687019690920191600101613dcc565b9092916001613e3785613918565b946020613e4c613e478680613922565b613991565b613e6f613e678963ffffffff165f5260cb60205260405f2090565b54151561399c565b613eaa613e8a8963ffffffff165f5260cb60205260405f2090565b5460405184810190613ea18161185d8b8a86613a10565b51902014613a2e565b613ed5613ecf613ec88a63ffffffff165f5260cc60205260405f2090565b5460ff1690565b15613aa0565b613efa613eec6118f8613ee788613918565b613b36565b63ffffffff43161115613b68565b604051613f0e8161185d8582019485613c04565b51902091613f1b836150ee565b013590613f27826150ee565b141461441657613f37835161225d565b935f5b8451811015613f645780613f536131d56001938861229c565b613f5d828961229c565b5201613f3a565b509092939194613f9e60208701946020613f7d87613918565b604051613f928161185d8a868301958661303b565b51902091013514613c15565b613fa8855161225d565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b875181101561405857806020613fef61400f938961229c565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa91821561036557600192614034915f9161403a575b50610b5e838d61229c565b01613fd6565b614052915060203d8111610a08576109fa81836103a2565b5f614029565b50929695509250926140b761408061408860408601946140788688613ca0565b939091613918565b9236916107c5565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166124fd565b945f915b86518310156143b8575f979697955b6140d4848a61229c565b51518710156143a85797614115986020806140f38a612b88898761229c565b510151604051809c81926308f6629d60e31b8352600483019190602083019252565b0381865afa998a15610365575f9a614388575b506001985f5b855181101561437a576141536141476122eb838961229c565b6001600160a01b031690565b6001600160a01b038d161461416a5760010161412e565b5099909791985060015f5b15151461418a575b50600101959796976140ca565b98969786985f879593986141ef60ff6141c76125b6612c3a8c6141c16142449f6141bb60d25460018060a01b031690565b98613ca0565b906129cd565b6141e16141d26103d5565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d15461420690614147906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529098899190829081906044820190565b03915afa968715610365575f97614356575b50614261875161225d565b985f5b8a5181101561428a578067016345785d8a00006142836001938e61229c565b5201614264565b509a9294966142c860ff6142af6125b6612c3a8f9d979f969e966141c18e8e92613ca0565b6142ba6126fe610402565b1663ffffffff166020860152565b604084015260608301526142da613d55565b608083015260d0546142f690614147906001600160a01b031681565b803b1561036a57604051636a669b4160e01b8152925f9184918290849082906143229060048301613d85565b03925af19182156103655760019261433c575b509061417d565b8061434a5f614350936103a2565b8061058c565b5f614335565b6143739197503d805f833e61436b81836103a2565b810190613cd2565b955f614256565b509990979198600190614175565b6143a1919a5060203d8111610a08576109fa81836103a2565b985f614128565b96979695506001909201916140bb565b50505050509190506143e86143db8263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b50505063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b9035601e198236030181121561036a5701602081359101916001600160401b03821161036a57813603831361036a57565b60208152813590603e198336030182121561036a576080614525606061451e6144e4876106f597018560208801526144d26144c76144b68380614447565b604060a08c015260e08b0191612921565b916020810190614447565b888303609f190160c08a015290612921565b6145006144f360208a0161060c565b63ffffffff166040880152565b61450d6040890189614447565b878303601f19018589015290612921565b950161060c565b63ffffffff16910152565b1561453757565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156145a957565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b1561460a57565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6040810192916103d391906139f2565b9060648202918083046064149015171561298357565b9060068202918083046006149015171561298357565b906001600160601b03809116911602906001600160601b03821691820361298357565b156146cb57565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b602090826040519384928337810160cd81520301902090565b601f821161475b57505050565b5f5260205f20906020601f840160051c83019310614793575b601f0160051c01905b818110614788575050565b5f815560010161477d565b9091508190614774565b9092916001600160401b03811161039d576147c2816147bc8454610bba565b8461474e565b5f601f82116001146148005781906147f19394955f926147f5575b50508160011b915f199060031b1c19161790565b9055565b013590505f806147dd565b601f19821694614813845f5260205f2090565b915f5b87811061484d575083600195969710614834575b505050811b019055565b01355f19600384901b60f8161c191690555f808061482a565b90926020600181928686013581550194019101614816565b909291602060609161487b8460808101976139f2565b63ffffffff81511660408501520151910152565b1561489657565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6148fb906150a6565b60ce80546001600160a01b03199081166001600160a01b039384161790915560cf805482169383169390931790925560d1805483169382169390931790925560d0805482169383169390931790925560d280549092169216919091179055565b60208152608063ffffffff60606149a1614980865185602088015260a0870190613bda565b8360208801511660408701526040870151601f198783030184880152610bf2565b9401511691015290565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b60405190614a1e82610382565b5f6020838281520152565b60405190610180614a3a81846103a2565b368337565b60405190614a4e6020836103a2565b6020368337565b91906040906060614a64614a11565b9485926020855192614a7685856103a2565b8436853780518452015160208301528482015260076107cf195a01fa15614a9957565bfe5b602092916080604092614aac614a11565b95869381865193614abd86866103a2565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa8015614a995715614aee57565b63d4b68fd760e01b5f5260045ffd5b604051614b0981610382565b6040908151614b1883826103a2565b8236823781526020825191614b2d84846103a2565b8336843701528051614b3f82826103a2565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6020820152815190614b9583836103a2565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152614bea835193846103a2565b8252602082015290565b5f51602061526d5f395f51905f5290614c0b614a11565b505f919006602060c0835b614d0b575f935f51602061526d5f395f51905f5260038186818180090908604051614c4185826103a2565b84368237848185604051614c5582826103a2565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f51602061526d5f395f51905f5260a082015260056107cf195a01fa8015614a9957614cbf90615256565b5191614d0b575f51602061526d5f395f51905f5282800914614cf657505f51602061526d5f395f51905f5260015f94089293614c16565b92935050614d026103d5565b92835282015290565b612141565b614d18614a11565b50604051614d2581610382565b600181526002602082015290565b90600c81101561213c5760051b0190565b93929091614d526040610411565b9485526020850152614d646040610411565b9182526020820152614d74614a29565b925f5b60028110614da157505050602061018092614d90614a3f565b93849160086201d4c0fa9151151590565b80614dad60019261468b565b614db7828561212b565b5151614dc38289614d33565b526020614dd0838661212b565b510151614de5614ddf83612f4f565b89614d33565b52614df0828661212b565b515151614dff614ddf83612f5d565b52614e15614e0d838761212b565b515160200190565b51614e22614ddf83612f6b565b526020614e2f838761212b565b51015151614e3f614ddf83612f79565b52614e6b614e65614e5e6020614e55868a61212b565b51015160200190565b5192612f87565b88614d33565b5201614d77565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614ebe60ff936151de565b928392161b1115614ecc5790565b63ca95733360e01b5f5260045ffd5b805f915b614ee7575090565b5f1981018181116129835761ffff9116911661ffff8114612983576001019080614edf565b90614f15614a11565b5061ffff811690610200821015614fba5760018214614fb557614f366103d5565b5f81525f602082015292906001905f925b61ffff8316851015614f5b57505050505090565b600161ffff831660ff86161c811614614f95575b6001614f8b614f808360ff94614a9b565b9460011b61fffe1690565b9401169291614f47565b946001614f8b614f80614faa8960ff95614a9b565b989350505050614f6f565b505090565b637fc4ea7d60e11b5f5260045ffd5b614fd1614a11565b50805190811580615042575b15614ffe575050604051614ff26040826103a2565b5f81525f602082015290565b60205f51602061526d5f395f51905f52910151065f51602061526d5f395f51905f52035f51602061526d5f395f51905f5281116129835760405191614bea83610382565b50602081015115614fdd565b6033546001600160a01b0316330361506257565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b5f809160405160208101916327b7cf8560e01b83526024820152602481526151176044826103a2565b51906a636f6e736f6c652e6c6f675afa50565b61ffff61513682614edb565b16615140816107aa565b9061514e60405192836103a2565b80825261515d601f19916107aa565b013660208301375f5f5b82518210806151bd575b156151b6576001811b841661518f575b61518a906129ee565b615167565b90600161518a9160ff60f81b8460f81b165f1a6151ac82876123e2565b5301919050615181565b5050905090565b506101008110615171565b156151cf57565b631019106960e31b5f5260045ffd5b906101008251116152475781511561524257602082015160019060f81c81901b5b835182101561523d5760019061522861521e6125b66125a886896123e2565b60ff600191161b90565b906152348183116151c8565b179101906151ff565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b1561525d57565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220fc928aa05e8197e904cac0bcde47a5835ccb1486165078b6b2f5480bb6d0a25a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x13d9\xDD\x14a\x02\xAAW\x80c\x17\x1F\x1D[\x14a\x02\xA5W\x80c\x1A\xD41\x89\x14a\x01\xF1W\x80c$Z{\xFC\x14a\x02\xA0W\x80c,\xB2#\xD5\x14a\x02\x9BW\x80c-\x89\xF6\xFC\x14a\x02\x96W\x80c1\xB3k\xD9\x14a\x02\x91W\x80c5c\xB0\xD1\x14a\x02\x8CW\x80c9\x98\xFD\xD3\x14a\x02\x87W\x80cAl~^\x14a\x02\x82W\x80cM+W\xFE\x14a\x02}W\x80cOA\xB8\x18\x14a\x02xW\x80cOs\x9Ft\x14a\x02sW\x80cY\\jg\x14a\x02nW\x80cZ-\x7F\x02\x14a\x02iW\x80cZ\xC8j\xB7\x14a\x02dW\x80c\\\x15Vb\x14a\x02_W\x80c\\\x97Z\xBB\x14a\x02ZW\x80c]\xEC\xC3\xF5\x14a\x02UW\x80c]\xF4YF\x14a\x02PW\x80ch0H5\x14a\x02KW\x80cm\x14\xA9\x87\x14a\x02FW\x80cn\xFBF6\x14a\x02AW\x80cqP\x18\xA6\x14a\x02<W\x80cr\xD1\x8E\x8D\x14a\x02-W\x80cz\xFA\x1E\xED\x14a\x027W\x80c\x88o\x11\x95\x14a\x022W\x80c\x8B\0\xCE|\x14a\x02-W\x80c\x8D\xA5\xCB[\x14a\x02(W\x80c\x92\xA5\x18$\x14a\x02#W\x80c\x9B)\x0E\x98\x14a\x02\x1EW\x80c\xB7\x029I\x14a\x02\x19W\x80c\xB9\x8D\t\x08\x14a\x02\x14W\x80c\xCA\x8A\xA7\xC7\x14a\x02\x0FW\x80c\xCC*\x9A[\x14a\x02\nW\x80c\xCE\xFD\xC1\xD4\x14a\x02\x05W\x80c\xDF\\\xF7#\x14a\x02\0W\x80c\xEF\x02\x9D\xBC\x14a\x01\xFBW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xF6W\x80c\xF5\xC9\x89\x9D\x14a\x01\xF1W\x80c\xF6<[\xAB\x14a\x01\xECWc\xFA\xBC\x1C\xBC\x14a\x01\xE7W_\x80\xFD[a\x1F\xE9V[a\x1F\xCEV[a\x05\x96V[a\x1F=V[a\x1D\xA9V[a\x1DeV[a\x1C!V[a\x1A\xE1V[a\x1A\xB9V[a\x1A\x97V[a\x17\xAAV[a\x17\x82V[a\x17\x11V[a\x16\xBCV[a\x16-V[a\x16xV[a\x16PV[a\x15\xD2V[a\x15%V[a\x12HV[a\x12\x04V[a\x11\xC0V[a\x11\x82V[a\x11eV[a\x0F\xECV[a\x0F\x80V[a\x0FSV[a\x0E\xE0V[a\x0E9V[a\x0C'V[a\n\xAEV[a\tNV[a\t\x1CV[a\x08\xA2V[a\x06\xF8V[a\x06PV[a\x06\x17V[a\x05\xD6V[a\x05$V[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03eWa\x034\x92a\x03 \x91_\x91a\x036W[Pa \xEBV[a\x03/`fT\x82\x81\x16\x14a!\x01V[aI\xDFV[\0[a\x03X\x91P` =` \x11a\x03^W[a\x03P\x81\x83a\x03\xA2V[\x81\x01\x90a \xCBV[_a\x03\x1AV[P=a\x03FV[a \xE0V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x9DW`@RV[a\x03nV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x9DW`@RV[`@Q\x90a\x03\xD3a\x01\0\x83a\x03\xA2V[V[`@Q\x90a\x03\xD3`@\x83a\x03\xA2V[`@Q\x90a\x03\xD3`\x80\x83a\x03\xA2V[`@Q\x90a\x03\xD3``\x83a\x03\xA2V[`@Q\x90a\x03\xD3`\xA0\x83a\x03\xA2V[\x90a\x03\xD3`@Q\x92\x83a\x03\xA2V[`@\x90`\xE3\x19\x01\x12a\x03jW`@Q\x90a\x048\x82a\x03\x82V[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x03jW`@Qa\x04`\x81a\x03\x82V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03jW`@Q\x91a\x04\x8B`@\x84a\x03\xA2V[\x82\x90`@\x81\x01\x92\x83\x11a\x03jW\x90[\x82\x82\x10a\x04\xA7WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x04\x9AV[\x90`\x80`c\x19\x83\x01\x12a\x03jW`@Qa\x04\xD0\x81a\x03\x82V[` a\x04\xEB\x82\x94a\x04\xE2\x81`da\x04pV[\x84R`\xA4a\x04pV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03jW` a\x04\xEB`@Q\x92a\x05\x0E\x84a\x03\x82V[`@\x84\x96a\x05\x1C\x83\x82a\x04pV[\x86R\x01a\x04pV[4a\x03jWa\x01 6`\x03\x19\x01\x12a\x03jW`\x045`@6`#\x19\x01\x12a\x03jWa\x05|`@\x91\x82Qa\x05V\x81a\x03\x82V[`$5\x81R`D5` \x82\x01Ra\x05l6a\x04\xB7V[\x90a\x05v6a\x04\x1FV[\x92a!UV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x03jWV[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03jWV[5\x90a\x03\xD3\x82a\x05\xFEV[4a\x03jW` 6`\x03\x19\x01\x12a\x03jWc\xFF\xFF\xFF\xFF`\x045a\x069\x81a\x05\xFEV[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x03jW` 6`\x03\x19\x01\x12a\x03jWc\xFF\xFF\xFF\xFF`\x045a\x06r\x81a\x05\xFEV[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03jWV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x9DW`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xCEWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\xC1V[\x90` a\x06\xF5\x92\x81\x81R\x01\x90a\x06\xB1V[\x90V[4a\x03jW`@6`\x03\x19\x01\x12a\x03jW`\x045a\x07\x15\x81a\x06\x89V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03jW6`#\x83\x01\x12\x15a\x03jW\x81`\x04\x015\x91a\x07A\x83a\x06\x9AV[\x92a\x07O`@Q\x94\x85a\x03\xA2V[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03jW`$\x01\x90[\x82\x82\x10a\x07\x90Wa\x07\x8Ca\x07\x80\x86\x86a\"\xBFV[`@Q\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[` \x80\x91\x835a\x07\x9F\x81a\x06\x89V[\x81R\x01\x91\x01\x90a\x07lV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x9DW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xD1\x82a\x07\xAAV[\x91a\x07\xDF`@Q\x93\x84a\x03\xA2V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03jW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x08&WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x08fWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x08\x17V[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\x08GV[4a\x03jW``6`\x03\x19\x01\x12a\x03jW`\x045a\x08\xBF\x81a\x06\x89V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03jW6`#\x82\x01\x12\x15a\x03jWa\x07\x8C\x91a\x08\xF6a\t\x08\x926\x90`$\x81`\x04\x015\x91\x01a\x07\xC5V[`D5\x91a\t\x03\x83a\x05\xFEV[a$\xFDV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xFBV[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`\xD2T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x03jWV[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW`\x045a\tk\x81a\tDV[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03eW_\x91a\t\xE0W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\t\xD1Wa\x034\x90aNrV[cpp\xF3\xB1`\xE1\x1B_R`\x04_\xFD[a\n\x02\x91P` =` \x11a\n\x08W[a\t\xFA\x81\x83a\x03\xA2V[\x81\x01\x90a#\x84V[_a\t\xB8V[P=a\t\xF0V[\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x815a\n&\x81a\x06\x9AV[\x92a\n4`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03jW` \x01\x90[\x82\x82\x10a\n\\WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\nOV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\n\x8FWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\x82V[4a\x03jW`@6`\x03\x19\x01\x12a\x03jW`\x045a\n\xCB\x81a\x06\x89V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\n\xEA\x906\x90`\x04\x01a\n\x0FV[a\n\xF4\x81Qa\"]V[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\x0B\x91W\x80` a\x0B\x19a\x0B9\x93\x86a\"\x9CV[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x03eW`\x01\x92a\x0Bm\x91_\x91a\x0BsW[Pa\x0B^\x83\x88a\"\x9CV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\x0B\0V[a\x0B\x8B\x91P` =\x81\x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[_a\x0BSV[`@Q\x80a\x07\x8C\x86\x82a\nlV[\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x81` a\x06\xF5\x935\x91\x01a\x07\xC5V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0B\xE8W[` \x83\x10\x14a\x0B\xD4WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x0B\xC9V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x06\xF5\x92\x81\x81R\x01\x90a\x0B\xF2V[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03jW` a\x0CZ\x81\x926\x90`\x04\x01a\x0B\x9FV[`@Q\x92\x81\x84\x92Q\x91\x82\x91\x01\x83^\x81\x01`\xCD\x81R\x03\x01\x90 `@Q\x90_\x90\x80T\x90a\x0C\x84\x82a\x0B\xBAV[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a\x0C\xF8WP`\x01\x14a\x0C\xB8W[a\x07\x8C\x84a\x0C\xAC\x81\x86\x03\x82a\x03\xA2V[`@Q\x91\x82\x91\x82a\x0C\x16V[_\x90\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x0C\xDEWP\x90\x91P\x81\x01` \x01a\x0C\xAC\x82a\x0C\x9CV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x0C\xC5V[`\xFF\x19\x16` \x80\x87\x01\x91\x90\x91R\x92\x15\x15`\x05\x1B\x85\x01\x90\x92\x01\x92Pa\x0C\xAC\x91P\x83\x90Pa\x0C\x9CV[\x91\x81`\x1F\x84\x01\x12\x15a\x03jW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03jW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03jWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\riWPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\r\\V[\x90` \x82R``a\r\xD3a\r\xBEa\r\xA8\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\rLV[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\rLV[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\rLV[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0E\x0CWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0E*`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\rLV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\r\xFDV[4a\x03jW`\x806`\x03\x19\x01\x12a\x03jW`\x045a\x0EV\x81a\x06\x89V[`$5\x90a\x0Ec\x82a\x05\xFEV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x0E\x82\x906\x90`\x04\x01a\r\x1FV[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03jW6`#\x85\x01\x12\x15a\x03jW\x83`\x04\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03jW6`$\x85`\x05\x1B\x87\x01\x01\x11a\x03jWa\x07\x8C\x95`$a\x0E\xD4\x96\x01\x93a*!V[`@Q\x91\x82\x91\x82a\r\x85V[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03eWa\x0FK\x91_\x91a\x036WPa \xEBV[a\x034aI\xABV[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` `@Qg\x01cEx]\x8A\0\0\x81R\xF3[`\xFF\x81\x16\x03a\x03jWV[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW` `\x01`\xFF`\x045a\x0F\xA3\x81a\x0FuV[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x0F\xD6WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0F\xC9V[4a\x03jW``6`\x03\x19\x01\x12a\x03jW`\x045a\x10\t\x81a\x06\x89V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x10(\x906\x90`\x04\x01a\n\x0FV[`D5\x91a\x105\x83a\x05\xFEV[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x10]\x86\x88`\x04\x84\x01a.\x89V[\x03\x81\x84Z\xFA\x91\x82\x15a\x03eW_\x92a\x11AW[Pa\x10{\x83Qa\"]V[\x93_[\x84Q\x81\x10\x15a\x113Wa\x10\x91\x81\x86a\"\x9CV[Q\x90` \x83a\x10\xADa\x10\xA3\x84\x89a\"\x9CV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x03eW`\x01\x92_\x91a\x11\x05W[P\x82\x80`\xC0\x1B\x03\x16a\x10\xFE\x82\x89a\"\x9CV[R\x01a\x10~V[a\x11&\x91P` =\x81\x11a\x11,W[a\x11\x1E\x81\x83a\x03\xA2V[\x81\x01\x90a)\x98V[_a\x10\xECV[P=a\x11\x14V[`@Q\x80a\x07\x8C\x88\x82a\x0F\xB3V[a\x11^\x91\x92P=\x80_\x83>a\x11V\x81\x83a\x03\xA2V[\x81\x01\x90a(gV[\x90_a\x10pV[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` `fT`@Q\x90\x81R\xF3[4a\x03jW` 6`\x03\x19\x01\x12a\x03jWc\xFF\xFF\xFF\xFF`\x045a\x11\xA4\x81a\x05\xFEV[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x815a\x12\xA3\x81a\x06\x9AV[\x92a\x12\xB1`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03jW` \x01\x90[\x82\x82\x10a\x12\xD9WPPP\x90V[` \x80\x91\x835a\x12\xE8\x81a\x05\xFEV[\x81R\x01\x91\x01\x90a\x12\xCCV[\x81`\x1F\x82\x01\x12\x15a\x03jW\x805a\x13\t\x81a\x06\x9AV[\x92a\x13\x17`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03jW` \x01\x91[\x83\x83\x10a\x13AWPPPP\x90V[` `@\x91a\x13P\x84\x86a\x04HV[\x81R\x01\x92\x01\x91a\x133V[\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x815a\x13r\x81a\x06\x9AV[\x92a\x13\x80`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03jW` \x82\x01\x90[\x83\x82\x10a\x13\xACWPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03jW` \x91a\x13\xCE\x87\x84\x80\x94\x88\x01\x01a\x12\x8CV[\x81R\x01\x91\x01\x90a\x13\x9DV[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03jWa\x13\xEFa\x03\xC3V[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x81a\x14\x0C\x91\x84\x01a\x12\x8CV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x81a\x14-\x91\x84\x01a\x12\xF3V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x81a\x14Q\x91\x84\x01a\x12\xF3V[`@\x85\x01Ra\x14c\x81``\x84\x01a\x04\xF0V[``\x85\x01Ra\x14u\x81`\xE0\x84\x01a\x04HV[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x81a\x14\x9A\x91\x84\x01a\x12\x8CV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x81a\x14\xBF\x91\x84\x01a\x12\x8CV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x14\xE2\x92\x01a\x13[V[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x15\x06WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x14\xF9V[4a\x03jW`\x806`\x03\x19\x01\x12a\x03jW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x15X\x906\x90`\x04\x01a\r\x1FV[\x90\x91`D5a\x15f\x81a\x05\xFEV[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03jWa\x15\xC8\x94a\x15\x8Da\x15\x93\x956\x90`\x04\x01a\x13\xD9V[\x93a0{V[`@Q\x92\x83\x92`@\x84R` a\x15\xB4\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x14\xE9V[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x14\xE9V[\x90` \x83\x01R\x03\x90\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jWa\x15\xEAaPNV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90\x81`\x80\x91\x03\x12a\x03jW\x90V[`@\x90`#\x19\x01\x12a\x03jW`$\x90V[\x90\x81`@\x91\x03\x12a\x03jW\x90V[4a\x03jW`\xC06`\x03\x19\x01\x12a\x03jW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x17A\x906\x90`\x04\x01a\x16\xE4V[a\x17J6a\x16\xF2V[\x90`@6`c\x19\x01\x12a\x03jW`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x034\x92a\x17|`d\x926\x90`\x04\x01a\x12\xF3V[\x92a>)V[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03jW`\x806`\x03\x19\x01\x12a\x03jW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x17\xDA\x906\x90`\x04\x01a\x16\xE4V[a\x17\xE36a\x16\xF2V[`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x18\x02\x906\x90`\x04\x01a\x13\xD9V[`\xCET\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1ARWa\x19'a\x18&` \x85\x01a9\x18V[\x93a\x184`@\x82\x01\x82a<\xA0V[\x92\x90\x94a\x18C``\x84\x01a9\x18V[\x96a\x18\x95`@Q` \x81\x01\x90a\x18k\x81a\x18]\x89\x85aDxV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xA2V[Q\x90 a\x18\x8Ea\x18z\x89a9\x18V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[T\x14aE0V[a\x18\xBFa\x18\xB8a\x18\xA4\x88a9\x18V[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15aE\xA2V[\x84c\xFF\xFF\xFF\xFFC\x16\x97a\x19\ta\x19\x01a\x18\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a;NV[c\xFF\xFF\xFF\xFF\x16\x90V[\x8A\x11\x15aF\x03V[`@Q` \x81\x01\x90a\x19\x1F\x81a\x18]\x8C\x85aFeV[Q\x90 a0{V[\x92\x90`\xFF_\x97\x16\x96[\x82\x81\x10a\x19\xF0W\x7F\xF2\xAF\x11\xFA\xD7=CI\xC9\x9C\xF6/)\x8D3vA\xEA\x0B\xB7\xC0\xF5\xA8\xDB\x92\xA9\x8A'_sOX\x87\x87\x87a\x19\x99a\x19\x88\x89a\x19\x94a\x19\x8Ea\x19\x7Fa\x19u\x84\x80a9\"V[` \x81\x01\x90a<\xA0V[\x94\x90\x93\x80a9\"V[\x80a<\xA0V[\x90aG5V[aG\x9DV[a\x19\xB0a\x19\xA4a\x03\xD5V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x19\xCC\x81a\x18]\x86\x86\x86aHeV[Q\x90 a\x19\xDBa\x18\xA4\x83a9\x18V[Ua\x19\xEB`@Q\x92\x83\x92\x83aHeV[\x03\x90\xA1\0[\x80a\x1ALa\x1A(a\x1A#a\x1A\x17a\x1A\n`\x01\x96\x88Qa\"\x9CV[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[aFuV[a\x1AEa\x1A\x17\x8Ca\x1A@a\x1A\n\x87` \x8B\x01Qa\"\x9CV[aF\xA1V[\x11\x15aF\xC4V[\x01a\x190V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03jW`\xC06`\x03\x19\x01\x12a\x03jW`\x045a\x1A\xFE\x81a\x06\x89V[a\x1B~`$5a\x1B\r\x81a\x06\x89V[`D5a\x1B\x19\x81a\x06\x89V[`d5a\x1B%\x81a\x06\x89V[`\x845\x91a\x1B2\x83a\x06\x89V[`\xA45\x93a\x1B?\x85a\x06\x89V[_T\x96a\x1Bd`\xFF`\x08\x8A\x90\x1C\x16\x15\x80\x99\x81\x9Aa\x1B\xFCW[\x81\x15a\x1B\xDCW[PaH\x8FV[\x87a\x1Bu`\x01`\xFF\x19_T\x16\x17_UV[a\x1B\xC5WaH\xF2V[a\x1B\x84W\0[a\x1B\x92a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x19\xEBV[a\x1B\xD7a\x01\0a\xFF\0\x19_T\x16\x17_UV[aH\xF2V[0;\x15\x91P\x81a\x1B\xEEW[P_a\x1B^V[`\xFF\x16`\x01\x14\x90P_a\x1B\xE7V[`\x01`\xFF\x82\x16\x10\x91Pa\x1BWV[`@\x90a\x06\xF5\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x07\xFBV[4a\x03jW``6`\x03\x19\x01\x12a\x03jW`\x045a\x1C>\x81a\x06\x89V[`$5`D5a\x1CM\x81a\x05\xFEV[a\x1C\x8Ea\x1CXa\";V[\x92\x80a\x1Cc\x85a\"\x8FV[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a.\x89V[\x03\x81\x87Z\xFA\x93\x84\x15a\x03eW\x83a\x1C\xB8a\x18\xF8a\x10\xA3a\x1C\xED\x98` \x97_\x91a\x1DKW[Pa\"\x8FV[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x03eWa\x1D\x1C\x92_\x91a\x1D,W[P`\x01`\x01`\xC0\x1B\x03\x16\x92a\x1D\x16\x84aQ*V[\x90a$\xFDV[\x90a\x07\x8C`@Q\x92\x83\x92\x83a\x1C\nV[a\x1DE\x91P` =` \x11a\x11,Wa\x11\x1E\x81\x83a\x03\xA2V[_a\x1D\x02V[a\x1D_\x91P=\x80_\x83>a\x11V\x81\x83a\x03\xA2V[_a\x1C\xB2V[4a\x03jW_6`\x03\x19\x01\x12a\x03jW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03jW``6`\x03\x19\x01\x12a\x03jW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x1D\xD9\x906\x90`\x04\x01a\x17\x03V[`$5\x90a\x1D\xE6\x82a\x05\xFEV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03jWa\x1E\x05\x906\x90`\x04\x01a\r\x1FV[`\xCFT\x90\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1E\xEEWa\x034\x93a\x1ERa\x1Ed\x92a\x1E\xD8\x95a\x1E=a\x1E5a\x03\xE4V[\x966\x90a97V[\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01R6\x91a\x07\xC5V[`@\x84\x01Rc\xFF\xFF\xFF\xFF\x16``\x83\x01RV[`@Q` \x81\x01\x90a\x1Ez\x81a\x18]\x85\x85aI[V[Q\x90 a\x1E\x8Fa\x18z`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\xA6\xB1\x91-\x06o\xD3\xA5A:b\"\xA5xZ\xE4?$$X\xF3\xEE\xF4\\P\xDD\x80\x89\x87\xB8x\xFD`@Q\x80a\x1E\xD0c\xFF\xFF\xFF\xFF\x86\x16\x94\x82aI[V[\x03\x90\xA2a;\x1EV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW`\x045a\x1FZ\x81a\x06\x89V[a\x1FbaPNV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1FzWa\x034\x90aP\xA6V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03jW_6`\x03\x19\x01\x12a\x03jW` `@Q`d\x81R\xF3[4a\x03jW` 6`\x03\x19\x01\x12a\x03jW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03eW_\x91a \xACW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a \x9DWa k`fT\x19\x82\x19\x81\x16\x14a!\x01V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a \xC5\x91P` =` \x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[_a JV[\x90\x81` \x91\x03\x12a\x03jWQa\x06\xF5\x81a\tDV[`@Q=_\x82>=\x90\xFD[\x15a \xF2WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a!\x08WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a!<W`\x05\x1B\x01\x90V[a!\x17V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\"1a\"\x0Ea\"7\x95a\"\x08a\"\x01\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra!\xD8\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xA2V[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aJUV[\x90aJ\x9BV[\x92a\"\x08a\"#a\"\x1DaJ\xFDV[\x94aK\xF4V[\x91a\",aM\x10V[aJUV[\x91aMDV[\x90\x91V[`@\x80Q\x90\x91\x90a\"L\x83\x82a\x03\xA2V[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a\"g\x82a\x06\x9AV[a\"t`@Q\x91\x82a\x03\xA2V[\x82\x81R\x80\x92a\"\x85`\x1F\x19\x91a\x06\x9AV[\x01\x90` 6\x91\x017V[\x80Q\x15a!<W` \x01\x90V[\x80Q\x82\x10\x15a!<W` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03jWQ\x90V[\x91\x90\x91a\"\xCC\x83Qa\"]V[\x92_[\x81Q\x81\x10\x15a#\x7FW\x80` a\"\xF8a\"\xEBa#!\x94\x86a\"\x9CV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03eW`\x01\x92_\x91a#QW[Pa#J\x82\x88a\"\x9CV[R\x01a\"\xCFV[a#r\x91P` =\x81\x11a#xW[a#j\x81\x83a\x03\xA2V[\x81\x01\x90a\"\xB0V[_a#?V[P=a#`V[PPPV[\x90\x81` \x91\x03\x12a\x03jWQa\x06\xF5\x81a\x06\x89V[\x90a#\xA3\x82a\x06\x9AV[a#\xB0`@Q\x91\x82a\x03\xA2V[\x82\x81R` \x81\x93a#\xC3`\x1F\x19\x91a\x06\x9AV[\x01\x91\x01_[\x82\x81\x10a#\xD4WPPPV[``\x82\x82\x01R` \x01a#\xC8V[\x90\x81Q\x81\x10\x15a!<W\x01` \x01\x90V[` \x81\x83\x03\x12a\x03jW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03jW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x81Qa$&\x81a\x06\x9AV[\x92a$4`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03jW` \x01\x90[\x82\x82\x10a$\\WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a$OV[\x90a$v\x82a\x06\x9AV[a$\x83`@Q\x91\x82a\x03\xA2V[\x82\x81R\x80\x92a$\x94`\x1F\x19\x91a\x06\x9AV[\x01_[\x81\x81\x10a$\xA3WPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x03\x9DW` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a$\x97V[\x90\x81` \x91\x03\x12a\x03jWQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x03jW\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x03eW_\x95a(\x12W[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x03eW`\x04\x96_\x93a'\xF0W[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x03eW_\x96a'\xCFW[Pa%\x8B\x85\x93\x92\x95Qa#\x99V[\x94_\x93[\x80Q\x85\x10\x15a'\xC5Wa%\xBCa%\xB6a%\xA8\x87\x84a#\xE2V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x03eW_\x94a'\xA1W[Pa&\x0C\x84Qa$lV[a&\x16\x88\x8Ba\"\x9CV[Ra&!\x87\x8Aa\"\x9CV[P_[\x84Q\x81\x10\x15a'\x90W\x80` a&=a&_\x93\x88a\"\x9CV[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03eW_\x92a'pW[Pa&\x85\x81\x87a\"\x9CV[Q\x8A` \x8Aa&\x94\x85\x8Ba\"\x9CV[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x03eWa''\x8C\x8Fa'\"`\x01\x98a'9\x97\x89\x97_\x92a'@W[Pa'\ra&\xFEa\x03\xF3V[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a\"\x9CV[Q\x90a'3\x83\x83a\"\x9CV[Ra\"\x9CV[P\x01a&$V[a'b\x91\x92P` =\x81\x11a'iW[a'Z\x81\x83a\x03\xA2V[\x81\x01\x90a$\xDEV[\x90_a&\xF2V[P=a'PV[a'\x89\x91\x92P` =\x81\x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x90_a&zV[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa%\x8FV[a'\xBE\x91\x94P=\x80_\x83>a'\xB6\x81\x83a\x03\xA2V[\x81\x01\x90a#\xF3V[\x92_a&\x01V[PPP\x93PPP\x90V[a'\xE9\x91\x96P` =` \x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x94_a%}V[` \x91\x93Pa(\x0B\x90\x82=\x84\x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x92\x90a%XV[a(,\x91\x95P` =` \x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x93_a%/V[`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x9DW`@R``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x03jW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03jW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x81Qa(\x9A\x81a\x06\x9AV[\x92a(\xA8`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03jW` \x01\x90[\x82\x82\x10a(\xD0WPPP\x90V[` \x80\x91\x83Qa(\xDF\x81a\x05\xFEV[\x81R\x01\x91\x01\x90a(\xC3V[c\xFF\xFF\xFF\xFF\x90\x91\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x83\x90R`\x01`\x01`\xFB\x1B\x03\x83\x11a\x03jW``\x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF5\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a)!V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a)\x83W`\x01\x01\x90V[a)^V[\x91\x90\x81\x10\x15a!<W`\x05\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x03jWQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03jW\x90V[\x15a)\xBEWV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a!<W\x01\x90V[\x90\x81` \x91\x03\x12a\x03jWQa\x06\xF5\x81a\x05\xFEV[_\x19\x81\x14a)\x83W`\x01\x01\x90V[\x91a*\x1A` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a)!V[\x94\x16\x91\x01RV[\x95\x93\x94\x95\x92\x90\x91\x92a*1a(3V[P`@Qch0H5`\xE0\x1B\x81R\x93`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90` \x85`\x04\x81\x86Z\xFA\x94\x85\x15a\x03eW_\x95a.hW[Pa*na(3V[\x94`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a*\x8E\x8D\x8D\x8B`\x04\x85\x01a(\xEAV[\x03\x81\x88Z\xFA\x90\x81\x15a\x03eW_\x91a.NW[P\x86R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x92\x90_\x81\x80a*\xD1\x85\x87\x8B`\x04\x85\x01a)AV[\x03\x81\x87Z\xFA\x90\x81\x15a\x03eW_\x91a.4W[P`@\x87\x01Ra*\xF3\x81a#\x99V[\x98``\x87\x01\x99\x8AR_[`\xFF\x81\x16\x83\x81\x10\x15a-\x7FW\x88_a+&\x83\x8Fa+\x19\x88a\"]V[\x90Q\x90a'3\x83\x83a\"\x9CV[P_\x8A\x86\x8F[\x81\x84\x10a+\xA9WPPPP\x90P\x8Ca+C\x82a\"]V[\x91_[\x81\x81\x10a+pWPP\x91a+e\x91a+k\x94\x93Q\x90a'3\x83\x83a\"\x9CV[Pa)rV[a*\xFDV[\x80a+\xA3a+\x8Ea\x10\xA3`\x01\x94a+\x88\x8A\x89Qa\"\x9CV[Qa\"\x9CV[a+\x98\x83\x88a\"\x9CV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x01a+FV[a\x10\xA3\x84a+\xBE\x81` \x96\x95a+\xC6\x95a)\x88V[5\x97Qa\"\x9CV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x87\x01R\x16`D\x85\x01R\x83`d\x81\x8DZ\xFA\x80\x15a\x03eW\x88\x8F\x88\x8A\x91\x8F\x94a,k`\x01a,^\x81\x93\x8D\x80\x9D_\x92a-SW[Pa%\xB6a,:a,H\x92a,3\x87\x80`\xC0\x1B\x03\x86\x16\x15\x15a)\xB7V[\x8B\x8Da)\xCDV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a,\x87W[PPPPP`\x01\x91\x92P\x01\x90\x8A\x91\x8A\x86\x8Fa+,V[\x85\x97a,\xA9\x93a,\xA2` \x97\x99\x98a%\xB6\x95a,:\x95a)\x88V[5\x95a)\xCDV[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`D\x84\x01R\x82`d\x81\x8CZ\xFA\x90\x81\x15a\x03eW\x8Fa-\x07\x90a-\x0C\x93\x83\x88`\x01\x97_\x93a-\x1BW[Pa+\x88\x90a+\x98\x93\x94Qa\"\x9CV[a)\xEEV[\x90P\x82\x91\x8A\x88\x8F\x88\x8A\x91a,qV[a+\x98\x93P\x90a-Da+\x88\x92` =\x81\x11a-LW[a-<\x81\x83a\x03\xA2V[\x81\x01\x90a)\xD9V[\x93P\x90a,\xF7V[P=a-2V[a,H\x91\x92Pa,:a-va%\xB6\x92` =\x81\x11a\x11,Wa\x11\x1E\x81\x83a\x03\xA2V[\x93\x92PPa,\x16V[PPP\x92\x90\x95\x97P`\x04\x94\x96P` \x91P`@Q\x94\x85\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x90\x81\x15a\x03eWa-\xD5\x94_\x94\x85\x93a.\x13W[P`@Qc5IR\xA3`\xE2\x1B\x81R\x95\x86\x94\x85\x93\x84\x93`\x04\x85\x01a)\xFCV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03eW_\x91a-\xF9W[P` \x82\x01R\x90V[a.\r\x91P=\x80_\x83>a\x11V\x81\x83a\x03\xA2V[_a-\xF0V[a.-\x91\x93P` =` \x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x91_a-\xB7V[a.H\x91P=\x80_\x83>a\x11V\x81\x83a\x03\xA2V[_a*\xE4V[a.b\x91P=\x80_\x83>a\x11V\x81\x83a\x03\xA2V[_a*\xA1V[a.\x82\x91\x95P` =` \x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x93_a*eV[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF5\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x06\xB1V[`@Q\x90a.\xB3\x82a\x03\x82V[``` \x83\x82\x81R\x01RV[\x15a.\xC6WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a.\xDBWV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a.\xF1WV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a/\x07WV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03jWQa\x06\xF5\x81a\x0FuV[_\x19\x81\x01\x91\x90\x82\x11a)\x83WV[\x15a/@WV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90`\x01\x82\x01\x80\x92\x11a)\x83WV[\x90`\x02\x82\x01\x80\x92\x11a)\x83WV[\x90`\x03\x82\x01\x80\x92\x11a)\x83WV[\x90`\x04\x82\x01\x80\x92\x11a)\x83WV[\x90`\x05\x82\x01\x80\x92\x11a)\x83WV[\x91\x90\x82\x01\x80\x92\x11a)\x83WV[\x15a/\xA9WV[c\xAF\xFC^\xDB`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03jWQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03jW\x90V[\x15a/\xE0WV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a)\x83WV[\x15a0\x16WV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a0,WV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a0eWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a0XV[\x94\x93\x92\x90\x91\x93a0\x89a.\xA6V[Pa0\x95\x85\x15\x15a.\xBFV[`@\x84\x01QQ\x85\x14\x80a9\nW[\x80a8\xFCW[\x80a8\xEEW[a0\xB8\x90a.\xD4V[a0\xCA` \x85\x01QQ\x85QQ\x14a.\xEAV[a0\xE1c\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a/\0V[a0\xE9a\x03\xD5V[_\x81R_` \x82\x01R\x92a0\xFBa.\xA6V[a1\x04\x87a\"]V[` \x82\x01Ra1\x12\x87a\"]V[\x81Ra1\x1Ca.\xA6V[\x92a1+` \x88\x01QQa\"]V[\x84Ra1;` \x88\x01QQa\"]V[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03eWa1\xA4\x91_\x91a8\xBFW[Pa1\x9F6\x8B\x87a\x07\xC5V[aN\xB0V[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a3\x16W` \x88a2\x0Ba\x10\xA3\x8Ca2\x03\x8F\x96\x86\x8Ea1\xE8a1\xD5\x86\x80\x95a\"\x9CV[Q\x80Q_R` \x01Q` R`@_ \x90V[a1\xF5\x84\x84\x84\x01Qa\"\x9CV[R\x82a2\xE3W[\x01Qa\"\x9CV[Q\x95Qa\"\x9CV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x03eWa\"\x08\x8Aa2\xB8\x8Fa2\xB1\x8F\x84` \x8F\x92a2\xA8\x93a2\xA0\x84`\x01\x9Ea2\xBE\x9E_\x91a2\xC6W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\"\x9CV[R\x01Qa\"\x9CV[Q\x93\x8DQa\"\x9CV[Q\x16aN\xDBV[\x90aO\x0CV[\x97\x01\x96a1\xA8V[a2\xDD\x91P\x86=\x81\x11a\x11,Wa\x11\x1E\x81\x83a\x03\xA2V[_a2\x91V[a3\x11a2\xF3\x84\x84\x84\x01Qa\"\x9CV[Qa3\n\x84\x84\x01Qa3\x04\x87a/+V[\x90a\"\x9CV[Q\x10a/9V[a1\xFCV[P\x90\x95\x97\x94\x96Pa3+\x91\x98\x93\x92\x99PaO\xC9V[\x91a38`\x97T`\xFF\x16\x90V[\x90\x81\x15a8\xB7W`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03eW_\x91a8\x98W[P\x91\x90[_\x92[\x81\x84\x10a3\xE9WPPPPP\x92a3\xD0a3\xCBa3\xC4a3\xE3\x95\x85a\x18]\x98`\x80``` \x99\x01Q\x92\x01Q\x92a!UV[\x91\x90a0\x0FV[a0%V[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a0;V[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da7\x92W[a\x10\xA3\x82`\xA0a4>a%\xB6a,:\x84a4F\x97a48a4*a1\xD5\x8F\x9C`@` \x9F\x9E\x01Qa\"\x9CV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba)\xCDV[\x97\x01Qa\"\x9CV[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03eWa5\na\x10\xA3\x8F\x95\x8F\x90a5\x02\x8F\x97\x8F\x96\x84\x8Fa4\xFC`\xC0\x96a4\xF5\x84\x8F` \x9F\x90a1\xFCa,:\x99`@\x93a%\xB6\x9C_\x91a7dW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a/\xD9V[Q\x90aJ\x9BV[\x9Ca)\xCDV[\x96\x01Qa\"\x9CV[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03eWa5\x97\x91\x8C\x8F\x92_\x92a7@W[P` a5\x89\x92\x93\x01Qa\"\x9CV[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a5\xB7\x8Ca5\x89\x8Ca5\xB0a\x1A\n\x82` \x86\x01Qa\"\x9CV[\x92Qa\"\x9CV[_\x98_[` \x8A\x01QQ\x81\x10\x15a7'W\x8B\x8Da5\xF9\x89a5\xECa%\xB6a,:\x86\x8F\x89a5\xE4\x91Qa\"\x9CV[Q\x94\x87a)\xCDV[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a6\x08W[PP`\x01\x01a5\xBBV[\x8A\x8Aa6\x8A\x85\x9F\x94\x8F\x96\x86a+\x88\x8F\x93`\xE0a6Aa\x10\xA3\x95` a69a%\xB6a,:\x83\x9Fa6J\x9C\x89\x91a)\xCDV[\x9A\x01Qa\"\x9CV[Q\x9B\x01Qa\"\x9CV[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03eW\x8Fa6\xF6\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a7\x01W[Pa6\xF0a5\x89\x92\x93Q\x93a6\xEBa\x1A\n\x84\x87a\"\x9CV[a/\xEFV[\x92a\"\x9CV[\x01\x9A\x90P\x8B\x8Da5\xFEV[a5\x89\x92Pa7 a6\xF0\x91` =\x81\x11a'iWa'Z\x81\x83a\x03\xA2V[\x92Pa6\xD3V[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a3\x93V[a5\x89\x92Pa7]` \x91\x82=\x81\x11a'iWa'Z\x81\x83a\x03\xA2V[\x92Pa5zV[` a7\x85\x92P=\x81\x11a7\x8BW[a7}\x81\x83a\x03\xA2V[\x81\x01\x90a/\xB8V[_a4\xDFV[P=a7sV[a7\xCF\x94Pa7\xAC\x92Pa%\xB6\x91a,:\x91` \x95a)\xCDV[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03eW` \x89a4F\x8F\x93\x8F`\xA0\x8F\x97a%\xB6a,:\x8F\x8F\x90a48a4*a1\xD5\x8F`@\x8B\x96\x91\x8F\x88\x93a\x10\xA3\x9Fa8S\x90a8Y\x93a4>\x9F_\x92a8oW[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a/\x95V[\x11a/\xA2V[PPPPPP\x97PPPPPP\x92\x93PPa3\xFEV[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a8\x90\x91=\x81\x11a#xWa#j\x81\x83a\x03\xA2V[\x92\x91Pa8BV[a8\xB1\x91P` =` \x11a-LWa-<\x81\x83a\x03\xA2V[_a3\x8CV[_\x91\x90a3\x90V[a8\xE1\x91P` =` \x11a8\xE7W[a8\xD9\x81\x83a\x03\xA2V[\x81\x01\x90a/\x16V[_a1\x93V[P=a8\xCFV[P`\xE0\x84\x01QQ\x85\x14a0\xAFV[P`\xC0\x84\x01QQ\x85\x14a0\xA9V[P`\xA0\x84\x01QQ\x85\x14a0\xA3V[5a\x06\xF5\x81a\x05\xFEV[\x905\x90`>\x19\x816\x03\x01\x82\x12\x15a\x03jW\x01\x90V[\x91\x90`@\x83\x82\x03\x12a\x03jW`@Q\x90a9P\x82a\x03\x82V[\x81\x93\x805`\x01`\x01`@\x1B\x03\x81\x11a\x03jW\x82a9n\x91\x83\x01a\x0B\x9FV[\x83R` \x81\x015\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03jW` \x92a\x04\xEB\x92\x01a\x0B\x9FV[a\x06\xF5\x906\x90a97V[\x15a9\xA3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a:\x06\x81a\x05\xFEV[\x16\x84R\x015\x91\x01RV[\x91`@a\x03\xD3\x92\x94\x93a:'\x81`\x80\x81\x01\x97a9\xF2V[\x01\x90a9\xF2V[\x15a:5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a:\xA7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a)\x83WV[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a)\x83WV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a)\x83WV[\x15a;oWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[a\x06\xF5\x91` a;\xF3\x83Q`@\x84R`@\x84\x01\x90a\x0B\xF2V[\x92\x01Q\x90` \x81\x84\x03\x91\x01Ra\x0B\xF2V[\x90` a\x06\xF5\x92\x81\x81R\x01\x90a;\xDAV[\x15a<\x1CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03jW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03jW` \x01\x91\x816\x03\x83\x13a\x03jWV[` \x81\x83\x03\x12a\x03jW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03jW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03jW\x81Qa=\x05\x81a\x06\x9AV[\x92a=\x13`@Q\x94\x85a\x03\xA2V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03jW` \x01\x90[\x82\x82\x10a=;WPPP\x90V[` \x80\x91\x83Qa=J\x81a\x06\x89V[\x81R\x01\x91\x01\x90a=.V[`@Q\x90a=d`@\x83a\x03\xA2V[`\x12\x82Rq9\xB60\xB9\xB4/\xBA42\xAF\xB7\xB82\xB90\xBA7\xB9`q\x1B` \x83\x01RV[\x91\x90` \x83R`\xC0\x83\x01\x92`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x93`\xA0``\x83\x01R\x84Q\x80\x91R` `\xE0\x83\x01\x95\x01\x90_[\x81\x81\x10a>\nWPPP`\x80a=\xF5a\x06\xF5\x94\x95``\x85\x01Q`\x1F\x19\x85\x83\x03\x01\x84\x86\x01Ra\x06\xB1V[\x92\x01Q\x90`\xA0`\x1F\x19\x82\x85\x03\x01\x91\x01Ra\x0B\xF2V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x96\x87\x01\x96\x90\x92\x01\x91`\x01\x01a=\xCCV[\x90\x92\x91`\x01a>7\x85a9\x18V[\x94` a>La>G\x86\x80a9\"V[a9\x91V[a>oa>g\x89c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a9\x9CV[a>\xAAa>\x8A\x89c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T`@Q\x84\x81\x01\x90a>\xA1\x81a\x18]\x8B\x8A\x86a:\x10V[Q\x90 \x14a:.V[a>\xD5a>\xCFa>\xC8\x8Ac\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a:\xA0V[a>\xFAa>\xECa\x18\xF8a>\xE7\x88a9\x18V[a;6V[c\xFF\xFF\xFF\xFFC\x16\x11\x15a;hV[`@Qa?\x0E\x81a\x18]\x85\x82\x01\x94\x85a<\x04V[Q\x90 \x91a?\x1B\x83aP\xEEV[\x015\x90a?'\x82aP\xEEV[\x14\x14aD\x16Wa?7\x83Qa\"]V[\x93_[\x84Q\x81\x10\x15a?dW\x80a?Sa1\xD5`\x01\x93\x88a\"\x9CV[a?]\x82\x89a\"\x9CV[R\x01a?:V[P\x90\x92\x93\x91\x94a?\x9E` \x87\x01\x94` a?}\x87a9\x18V[`@Qa?\x92\x81a\x18]\x8A\x86\x83\x01\x95\x86a0;V[Q\x90 \x91\x015\x14a<\x15V[a?\xA8\x85Qa\"]V[\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94_[\x87Q\x81\x10\x15a@XW\x80` a?\xEFa@\x0F\x93\x89a\"\x9CV[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8BZ\xFA\x91\x82\x15a\x03eW`\x01\x92a@4\x91_\x91a@:W[Pa\x0B^\x83\x8Da\"\x9CV[\x01a?\xD6V[a@R\x91P` =\x81\x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[_a@)V[P\x92\x96\x95P\x92P\x92a@\xB7a@\x80a@\x88`@\x86\x01\x94a@x\x86\x88a<\xA0V[\x93\x90\x91a9\x18V[\x926\x91a\x07\xC5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a$\xFDV[\x94_\x91[\x86Q\x83\x10\x15aC\xB8W_\x97\x96\x97\x95[a@\xD4\x84\x8Aa\"\x9CV[QQ\x87\x10\x15aC\xA8W\x97aA\x15\x98` \x80a@\xF3\x8Aa+\x88\x89\x87a\"\x9CV[Q\x01Q`@Q\x80\x9C\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x99\x8A\x15a\x03eW_\x9AaC\x88W[P`\x01\x98_[\x85Q\x81\x10\x15aCzWaASaAGa\"\xEB\x83\x89a\"\x9CV[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x14aAjW`\x01\x01aA.V[P\x99\x90\x97\x91\x98P`\x01_[\x15\x15\x14aA\x8AW[P`\x01\x01\x95\x97\x96\x97a@\xCAV[\x98\x96\x97\x86\x98_\x87\x95\x93\x98aA\xEF`\xFFaA\xC7a%\xB6a,:\x8CaA\xC1aBD\x9FaA\xBB`\xD2T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x98a<\xA0V[\x90a)\xCDV[aA\xE1aA\xD2a\x03\xD5V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[\x16c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`\xD1TaB\x06\x90aAG\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\x10]\xEA\x1F`\xE2\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\x04\x82\x01R` \x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16`$\x83\x01R\x90\x98\x89\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x96\x87\x15a\x03eW_\x97aCVW[PaBa\x87Qa\"]V[\x98_[\x8AQ\x81\x10\x15aB\x8AW\x80g\x01cEx]\x8A\0\0aB\x83`\x01\x93\x8Ea\"\x9CV[R\x01aBdV[P\x9A\x92\x94\x96aB\xC8`\xFFaB\xAFa%\xB6a,:\x8F\x9D\x97\x9F\x96\x9E\x96aA\xC1\x8E\x8E\x92a<\xA0V[aB\xBAa&\xFEa\x04\x02V[\x16c\xFF\xFF\xFF\xFF\x16` \x86\x01RV[`@\x84\x01R``\x83\x01RaB\xDAa=UV[`\x80\x83\x01R`\xD0TaB\xF6\x90aAG\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03jW`@Qcjf\x9BA`\xE0\x1B\x81R\x92_\x91\x84\x91\x82\x90\x84\x90\x82\x90aC\"\x90`\x04\x83\x01a=\x85V[\x03\x92Z\xF1\x91\x82\x15a\x03eW`\x01\x92aC<W[P\x90aA}V[\x80aCJ_aCP\x93a\x03\xA2V[\x80a\x05\x8CV[_aC5V[aCs\x91\x97P=\x80_\x83>aCk\x81\x83a\x03\xA2V[\x81\x01\x90a<\xD2V[\x95_aBVV[P\x99\x90\x97\x91\x98`\x01\x90aAuV[aC\xA1\x91\x9AP` =\x81\x11a\n\x08Wa\t\xFA\x81\x83a\x03\xA2V[\x98_aA(V[\x96\x97\x96\x95P`\x01\x90\x92\x01\x91a@\xBBV[PPPPP\x91\x90PaC\xE8aC\xDB\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3V[PPPc\xFF\xFF\xFF\xFF3\x91\x16\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05_\x80\xA3V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x03jW\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x03jW\x816\x03\x83\x13a\x03jWV[` \x81R\x815\x90`>\x19\x836\x03\x01\x82\x12\x15a\x03jW`\x80aE%``aE\x1EaD\xE4\x87a\x06\xF5\x97\x01\x85` \x88\x01RaD\xD2aD\xC7aD\xB6\x83\x80aDGV[`@`\xA0\x8C\x01R`\xE0\x8B\x01\x91a)!V[\x91` \x81\x01\x90aDGV[\x88\x83\x03`\x9F\x19\x01`\xC0\x8A\x01R\x90a)!V[aE\0aD\xF3` \x8A\x01a\x06\x0CV[c\xFF\xFF\xFF\xFF\x16`@\x88\x01RV[aE\r`@\x89\x01\x89aDGV[\x87\x83\x03`\x1F\x19\x01\x85\x89\x01R\x90a)!V[\x95\x01a\x06\x0CV[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15aE7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15aE\xA9WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[\x15aF\nWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[`@\x81\x01\x92\x91a\x03\xD3\x91\x90a9\xF2V[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a)\x83WV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a)\x83WV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a)\x83WV[\x15aF\xCBWV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[` \x90\x82`@Q\x93\x84\x92\x837\x81\x01`\xCD\x81R\x03\x01\x90 \x90V[`\x1F\x82\x11aG[WPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10aG\x93W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10aG\x88WPPV[_\x81U`\x01\x01aG}V[\x90\x91P\x81\x90aGtV[\x90\x92\x91`\x01`\x01`@\x1B\x03\x81\x11a\x03\x9DWaG\xC2\x81aG\xBC\x84Ta\x0B\xBAV[\x84aGNV[_`\x1F\x82\x11`\x01\x14aH\0W\x81\x90aG\xF1\x93\x94\x95_\x92aG\xF5W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90UV[\x015\x90P_\x80aG\xDDV[`\x1F\x19\x82\x16\x94aH\x13\x84_R` _ \x90V[\x91_[\x87\x81\x10aHMWP\x83`\x01\x95\x96\x97\x10aH4W[PPP\x81\x1B\x01\x90UV[\x015_\x19`\x03\x84\x90\x1B`\xF8\x16\x1C\x19\x16\x90U_\x80\x80aH*V[\x90\x92` `\x01\x81\x92\x86\x86\x015\x81U\x01\x94\x01\x91\x01aH\x16V[\x90\x92\x91` ``\x91aH{\x84`\x80\x81\x01\x97a9\xF2V[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[\x15aH\x96WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aH\xFB\x90aP\xA6V[`\xCE\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xCF\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x83\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\xD0\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD2\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[` \x81R`\x80c\xFF\xFF\xFF\xFF``aI\xA1aI\x80\x86Q\x85` \x88\x01R`\xA0\x87\x01\x90a;\xDAV[\x83` \x88\x01Q\x16`@\x87\x01R`@\x87\x01Q`\x1F\x19\x87\x83\x03\x01\x84\x88\x01Ra\x0B\xF2V[\x94\x01Q\x16\x91\x01R\x90V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aJ\x1E\x82a\x03\x82V[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aJ:\x81\x84a\x03\xA2V[6\x837V[`@Q\x90aJN` \x83a\x03\xA2V[` 6\x837V[\x91\x90`@\x90``aJdaJ\x11V[\x94\x85\x92` \x85Q\x92aJv\x85\x85a\x03\xA2V[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aJ\x99WV[\xFE[` \x92\x91`\x80`@\x92aJ\xACaJ\x11V[\x95\x86\x93\x81\x86Q\x93aJ\xBD\x86\x86a\x03\xA2V[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aJ\x99W\x15aJ\xEEWV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@QaK\t\x81a\x03\x82V[`@\x90\x81QaK\x18\x83\x82a\x03\xA2V[\x826\x827\x81R` \x82Q\x91aK-\x84\x84a\x03\xA2V[\x836\x847\x01R\x80QaK?\x82\x82a\x03\xA2V[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aK\x95\x83\x83a\x03\xA2V[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaK\xEA\x83Q\x93\x84a\x03\xA2V[\x82R` \x82\x01R\x90V[_Q` aRm_9_Q\x90_R\x90aL\x0BaJ\x11V[P_\x91\x90\x06` `\xC0\x83[aM\x0BW_\x93_Q` aRm_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaLA\x85\x82a\x03\xA2V[\x846\x827\x84\x81\x85`@QaLU\x82\x82a\x03\xA2V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aRm_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aJ\x99WaL\xBF\x90aRVV[Q\x91aM\x0BW_Q` aRm_9_Q\x90_R\x82\x80\t\x14aL\xF6WP_Q` aRm_9_Q\x90_R`\x01_\x94\x08\x92\x93aL\x16V[\x92\x93PPaM\x02a\x03\xD5V[\x92\x83R\x82\x01R\x90V[a!AV[aM\x18aJ\x11V[P`@QaM%\x81a\x03\x82V[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x0C\x81\x10\x15a!<W`\x05\x1B\x01\x90V[\x93\x92\x90\x91aMR`@a\x04\x11V[\x94\x85R` \x85\x01RaMd`@a\x04\x11V[\x91\x82R` \x82\x01RaMtaJ)V[\x92_[`\x02\x81\x10aM\xA1WPPP` a\x01\x80\x92aM\x90aJ?V[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aM\xAD`\x01\x92aF\x8BV[aM\xB7\x82\x85a!+V[QQaM\xC3\x82\x89aM3V[R` aM\xD0\x83\x86a!+V[Q\x01QaM\xE5aM\xDF\x83a/OV[\x89aM3V[RaM\xF0\x82\x86a!+V[QQQaM\xFFaM\xDF\x83a/]V[RaN\x15aN\r\x83\x87a!+V[QQ` \x01\x90V[QaN\"aM\xDF\x83a/kV[R` aN/\x83\x87a!+V[Q\x01QQaN?aM\xDF\x83a/yV[RaNkaNeaN^` aNU\x86\x8Aa!+V[Q\x01Q` \x01\x90V[Q\x92a/\x87V[\x88aM3V[R\x01aMwV[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01aN\xBE`\xFF\x93aQ\xDEV[\x92\x83\x92\x16\x1B\x11\x15aN\xCCW\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[aN\xE7WP\x90V[_\x19\x81\x01\x81\x81\x11a)\x83Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a)\x83W`\x01\x01\x90\x80aN\xDFV[\x90aO\x15aJ\x11V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aO\xBAW`\x01\x82\x14aO\xB5WaO6a\x03\xD5V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aO[WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aO\x95W[`\x01aO\x8BaO\x80\x83`\xFF\x94aJ\x9BV[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aOGV[\x94`\x01aO\x8BaO\x80aO\xAA\x89`\xFF\x95aJ\x9BV[\x98\x93PPPPaOoV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[aO\xD1aJ\x11V[P\x80Q\x90\x81\x15\x80aPBW[\x15aO\xFEWPP`@QaO\xF2`@\x82a\x03\xA2V[_\x81R_` \x82\x01R\x90V[` _Q` aRm_9_Q\x90_R\x91\x01Q\x06_Q` aRm_9_Q\x90_R\x03_Q` aRm_9_Q\x90_R\x81\x11a)\x83W`@Q\x91aK\xEA\x83a\x03\x82V[P` \x81\x01Q\x15aO\xDDV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aPbWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[_\x80\x91`@Q` \x81\x01\x91c'\xB7\xCF\x85`\xE0\x1B\x83R`$\x82\x01R`$\x81RaQ\x17`D\x82a\x03\xA2V[Q\x90jconsole.logZ\xFAPV[a\xFF\xFFaQ6\x82aN\xDBV[\x16aQ@\x81a\x07\xAAV[\x90aQN`@Q\x92\x83a\x03\xA2V[\x80\x82RaQ]`\x1F\x19\x91a\x07\xAAV[\x016` \x83\x017__[\x82Q\x82\x10\x80aQ\xBDW[\x15aQ\xB6W`\x01\x81\x1B\x84\x16aQ\x8FW[aQ\x8A\x90a)\xEEV[aQgV[\x90`\x01aQ\x8A\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaQ\xAC\x82\x87a#\xE2V[S\x01\x91\x90PaQ\x81V[PP\x90P\x90V[Pa\x01\0\x81\x10aQqV[\x15aQ\xCFWV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aRGW\x81Q\x15aRBW` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aR=W`\x01\x90aR(aR\x1Ea%\xB6a%\xA8\x86\x89a#\xE2V[`\xFF`\x01\x91\x16\x1B\x90V[\x90aR4\x81\x83\x11aQ\xC8V[\x17\x91\x01\x90aQ\xFFV[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15aR]WV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xFC\x92\x8A\xA0^\x81\x97\xE9\x04\xCA\xC0\xBC\xDEG\xA5\x83\\\xCB\x14\x86\x16Px\xB6\xB2\xF5H\x0B\xB6\xD0\xA2ZdsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `BitmapValueTooLarge()` and selector `0xca957333`.
```solidity
error BitmapValueTooLarge();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapValueTooLarge {}
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
        impl ::core::convert::From<BitmapValueTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapValueTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapValueTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapValueTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapValueTooLarge()";
            const SELECTOR: [u8; 4] = [202u8, 149u8, 115u8, 51u8];
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
    /**Custom error with signature `BytesArrayLengthTooLong()` and selector `0xfb4a9c8e`.
```solidity
error BytesArrayLengthTooLong();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayLengthTooLong {}
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
        impl ::core::convert::From<BytesArrayLengthTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayLengthTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayLengthTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayLengthTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayLengthTooLong()";
            const SELECTOR: [u8; 4] = [251u8, 74u8, 156u8, 142u8];
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
    /**Custom error with signature `BytesArrayNotOrdered()` and selector `0x80c88348`.
```solidity
error BytesArrayNotOrdered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayNotOrdered {}
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
        impl ::core::convert::From<BytesArrayNotOrdered> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayNotOrdered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayNotOrdered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayNotOrdered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayNotOrdered()";
            const SELECTOR: [u8; 4] = [128u8, 200u8, 131u8, 72u8];
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `ECAddFailed()` and selector `0xd4b68fd7`.
```solidity
error ECAddFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECAddFailed {}
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
        impl ::core::convert::From<ECAddFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECAddFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECAddFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECAddFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECAddFailed()";
            const SELECTOR: [u8; 4] = [212u8, 182u8, 143u8, 215u8];
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
    /**Custom error with signature `ECMulFailed()` and selector `0x4633be32`.
```solidity
error ECMulFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECMulFailed {}
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
        impl ::core::convert::From<ECMulFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECMulFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECMulFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECMulFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECMulFailed()";
            const SELECTOR: [u8; 4] = [70u8, 51u8, 190u8, 50u8];
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
    /**Custom error with signature `ExpModFailed()` and selector `0xd51edae3`.
```solidity
error ExpModFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpModFailed {}
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
        impl ::core::convert::From<ExpModFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ExpModFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpModFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpModFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpModFailed()";
            const SELECTOR: [u8; 4] = [213u8, 30u8, 218u8, 227u8];
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `InputArrayLengthMismatch()` and selector `0x43714afd`.
```solidity
error InputArrayLengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthMismatch {}
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
        impl ::core::convert::From<InputArrayLengthMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InputArrayLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthMismatch()";
            const SELECTOR: [u8; 4] = [67u8, 113u8, 74u8, 253u8];
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
    /**Custom error with signature `InputEmptyQuorumNumbers()` and selector `0x1f0405a0`.
```solidity
error InputEmptyQuorumNumbers();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputEmptyQuorumNumbers {}
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
        impl ::core::convert::From<InputEmptyQuorumNumbers> for UnderlyingRustTuple<'_> {
            fn from(value: InputEmptyQuorumNumbers) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputEmptyQuorumNumbers {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputEmptyQuorumNumbers {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputEmptyQuorumNumbers()";
            const SELECTOR: [u8; 4] = [31u8, 4u8, 5u8, 160u8];
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
    /**Custom error with signature `InputNonSignerLengthMismatch()` and selector `0x5f832f41`.
```solidity
error InputNonSignerLengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputNonSignerLengthMismatch {}
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
        impl ::core::convert::From<InputNonSignerLengthMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: InputNonSignerLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InputNonSignerLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputNonSignerLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputNonSignerLengthMismatch()";
            const SELECTOR: [u8; 4] = [95u8, 131u8, 47u8, 65u8];
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
    /**Custom error with signature `InvalidBLSPairingKey()` and selector `0x67988d33`.
```solidity
error InvalidBLSPairingKey();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBLSPairingKey {}
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
        impl ::core::convert::From<InvalidBLSPairingKey> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBLSPairingKey) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBLSPairingKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBLSPairingKey {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBLSPairingKey()";
            const SELECTOR: [u8; 4] = [103u8, 152u8, 141u8, 51u8];
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
    /**Custom error with signature `InvalidBLSSignature()` and selector `0xab1b236b`.
```solidity
error InvalidBLSSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBLSSignature {}
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
        impl ::core::convert::From<InvalidBLSSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBLSSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBLSSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBLSSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBLSSignature()";
            const SELECTOR: [u8; 4] = [171u8, 27u8, 35u8, 107u8];
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `InvalidQuorumApkHash()` and selector `0xe1310aed`.
```solidity
error InvalidQuorumApkHash();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidQuorumApkHash {}
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
        impl ::core::convert::From<InvalidQuorumApkHash> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorumApkHash) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorumApkHash {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorumApkHash {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidQuorumApkHash()";
            const SELECTOR: [u8; 4] = [225u8, 49u8, 10u8, 237u8];
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
    /**Custom error with signature `InvalidReferenceBlocknumber()` and selector `0x4b874f45`.
```solidity
error InvalidReferenceBlocknumber();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidReferenceBlocknumber {}
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
        impl ::core::convert::From<InvalidReferenceBlocknumber>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReferenceBlocknumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidReferenceBlocknumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReferenceBlocknumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidReferenceBlocknumber()";
            const SELECTOR: [u8; 4] = [75u8, 135u8, 79u8, 69u8];
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
    /**Custom error with signature `NonSignerPubkeysNotSorted()` and selector `0xff719414`.
```solidity
error NonSignerPubkeysNotSorted();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerPubkeysNotSorted {}
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
        impl ::core::convert::From<NonSignerPubkeysNotSorted>
        for UnderlyingRustTuple<'_> {
            fn from(value: NonSignerPubkeysNotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NonSignerPubkeysNotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonSignerPubkeysNotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonSignerPubkeysNotSorted()";
            const SELECTOR: [u8; 4] = [255u8, 113u8, 148u8, 20u8];
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `OnlyRegistryCoordinatorOwner()` and selector `0xe0e1e762`.
```solidity
error OnlyRegistryCoordinatorOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRegistryCoordinatorOwner {}
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
        impl ::core::convert::From<OnlyRegistryCoordinatorOwner>
        for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRegistryCoordinatorOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OnlyRegistryCoordinatorOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRegistryCoordinatorOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRegistryCoordinatorOwner()";
            const SELECTOR: [u8; 4] = [224u8, 225u8, 231u8, 98u8];
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
```solidity
error OperatorNotRegistered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered {}
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
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegistered()";
            const SELECTOR: [u8; 4] = [37u8, 236u8, 108u8, 31u8];
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
    /**Custom error with signature `ScalarTooLarge()` and selector `0xff89d4fa`.
```solidity
error ScalarTooLarge();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ScalarTooLarge {}
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
        impl ::core::convert::From<ScalarTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: ScalarTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ScalarTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ScalarTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ScalarTooLarge()";
            const SELECTOR: [u8; 4] = [255u8, 137u8, 212u8, 250u8];
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
    /**Custom error with signature `StaleStakesForbidden()` and selector `0xaffc5edb`.
```solidity
error StaleStakesForbidden();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StaleStakesForbidden {}
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
        impl ::core::convert::From<StaleStakesForbidden> for UnderlyingRustTuple<'_> {
            fn from(value: StaleStakesForbidden) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StaleStakesForbidden {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StaleStakesForbidden {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StaleStakesForbidden()";
            const SELECTOR: [u8; 4] = [175u8, 252u8, 94u8, 219u8];
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
    /**Event with signature `NewTaskCreated(uint32,((string,string),uint32,bytes,uint32))` and selector `0xa6b1912d066fd3a5413a6222a5785ae43f242458f3eef45c50dd808987b878fd`.
```solidity
event NewTaskCreated(uint32 indexed taskIndex, IIncredibleRedisTaskManager.Task task);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewTaskCreated {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub task: <IIncredibleRedisTaskManager::Task as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for NewTaskCreated {
            type DataTuple<'a> = (IIncredibleRedisTaskManager::Task,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "NewTaskCreated(uint32,((string,string),uint32,bytes,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                166u8,
                177u8,
                145u8,
                45u8,
                6u8,
                111u8,
                211u8,
                165u8,
                65u8,
                58u8,
                98u8,
                34u8,
                165u8,
                120u8,
                90u8,
                228u8,
                63u8,
                36u8,
                36u8,
                88u8,
                243u8,
                238u8,
                244u8,
                92u8,
                80u8,
                221u8,
                128u8,
                137u8,
                135u8,
                184u8,
                120u8,
                253u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    task: data.0,
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
                    <IIncredibleRedisTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.taskIndex.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewTaskCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewTaskCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewTaskCreated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Paused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                171u8,
                64u8,
                163u8,
                116u8,
                188u8,
                81u8,
                222u8,
                55u8,
                34u8,
                0u8,
                168u8,
                188u8,
                152u8,
                26u8,
                248u8,
                201u8,
                236u8,
                220u8,
                8u8,
                223u8,
                218u8,
                239u8,
                11u8,
                182u8,
                224u8,
                159u8,
                136u8,
                243u8,
                198u8,
                22u8,
                239u8,
                61u8,
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
    /**Event with signature `TaskChallengedSuccessfully(uint32,address)` and selector `0xc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec`.
```solidity
event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskChallengedSuccessfully {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for TaskChallengedSuccessfully {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TaskChallengedSuccessfully(uint32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                194u8,
                13u8,
                27u8,
                176u8,
                241u8,
                98u8,
                54u8,
                128u8,
                48u8,
                107u8,
                131u8,
                212u8,
                255u8,
                75u8,
                185u8,
                154u8,
                43u8,
                235u8,
                157u8,
                134u8,
                217u8,
                120u8,
                50u8,
                243u8,
                202u8,
                64u8,
                253u8,
                19u8,
                162u8,
                157u8,
                241u8,
                236u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    challenger: topics.2,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.taskIndex.clone(),
                    self.challenger.clone(),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.challenger,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskChallengedSuccessfully {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskChallengedSuccessfully> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &TaskChallengedSuccessfully,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskChallengedUnsuccessfully(uint32,address)` and selector `0xfd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb05`.
```solidity
event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskChallengedUnsuccessfully {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for TaskChallengedUnsuccessfully {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TaskChallengedUnsuccessfully(uint32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                253u8,
                62u8,
                38u8,
                190u8,
                235u8,
                89u8,
                103u8,
                252u8,
                90u8,
                87u8,
                160u8,
                68u8,
                105u8,
                20u8,
                234u8,
                188u8,
                69u8,
                180u8,
                170u8,
                71u8,
                76u8,
                103u8,
                165u8,
                27u8,
                75u8,
                81u8,
                96u8,
                202u8,
                198u8,
                13u8,
                219u8,
                5u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    challenger: topics.2,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.taskIndex.clone(),
                    self.challenger.clone(),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.challenger,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskChallengedUnsuccessfully {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskChallengedUnsuccessfully> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &TaskChallengedUnsuccessfully,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskCompleted(uint32)` and selector `0x9a144f228a931b9d0d1696fbcdaf310b24b5d2d21e799db623fc986a0f547430`.
```solidity
event TaskCompleted(uint32 indexed taskIndex);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskCompleted {
        #[allow(missing_docs)]
        pub taskIndex: u32,
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
        impl alloy_sol_types::SolEvent for TaskCompleted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "TaskCompleted(uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                154u8,
                20u8,
                79u8,
                34u8,
                138u8,
                147u8,
                27u8,
                157u8,
                13u8,
                22u8,
                150u8,
                251u8,
                205u8,
                175u8,
                49u8,
                11u8,
                36u8,
                181u8,
                210u8,
                210u8,
                30u8,
                121u8,
                157u8,
                182u8,
                35u8,
                252u8,
                152u8,
                106u8,
                15u8,
                84u8,
                116u8,
                48u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { taskIndex: topics.1 }
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.taskIndex.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskCompleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskCompleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskCompleted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskResponded((uint32,bytes32),(uint32,bytes32))` and selector `0xf2af11fad73d4349c99cf62f298d337641ea0bb7c0f5a8db92a98a275f734f58`.
```solidity
event TaskResponded(IIncredibleRedisTaskManager.TaskResponse taskResponse, IIncredibleRedisTaskManager.TaskResponseMetadata taskResponseMetadata);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskResponded {
        #[allow(missing_docs)]
        pub taskResponse: <IIncredibleRedisTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponseMetadata: <IIncredibleRedisTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for TaskResponded {
            type DataTuple<'a> = (
                IIncredibleRedisTaskManager::TaskResponse,
                IIncredibleRedisTaskManager::TaskResponseMetadata,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TaskResponded((uint32,bytes32),(uint32,bytes32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                242u8,
                175u8,
                17u8,
                250u8,
                215u8,
                61u8,
                67u8,
                73u8,
                201u8,
                156u8,
                246u8,
                47u8,
                41u8,
                141u8,
                51u8,
                118u8,
                65u8,
                234u8,
                11u8,
                183u8,
                192u8,
                245u8,
                168u8,
                219u8,
                146u8,
                169u8,
                138u8,
                39u8,
                95u8,
                115u8,
                79u8,
                88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskResponse: data.0,
                    taskResponseMetadata: data.1,
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
                    <IIncredibleRedisTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IIncredibleRedisTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponseMetadata,
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
        impl alloy_sol_types::private::IntoLogData for TaskResponded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskResponded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskResponded) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Unpaused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                53u8,
                130u8,
                209u8,
                130u8,
                142u8,
                38u8,
                191u8,
                86u8,
                189u8,
                128u8,
                21u8,
                2u8,
                188u8,
                2u8,
                26u8,
                192u8,
                188u8,
                138u8,
                251u8,
                87u8,
                200u8,
                38u8,
                228u8,
                152u8,
                107u8,
                69u8,
                89u8,
                60u8,
                143u8,
                173u8,
                56u8,
                156u8,
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
constructor(address _registryCoordinator, address _pauserRegistry, uint32 _taskResponseWindowBlock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _taskResponseWindowBlock: u32,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._registryCoordinator,
                        value._pauserRegistry,
                        value._taskResponseWindowBlock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                        _pauserRegistry: tuple.1,
                        _taskResponseWindowBlock: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._taskResponseWindowBlock,
                    ),
                )
            }
        }
    };
    /**Function with signature `TASK_CHALLENGE_WINDOW_BLOCK()` and selector `0xf63c5bab`.
```solidity
function TASK_CHALLENGE_WINDOW_BLOCK() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_CHALLENGE_WINDOW_BLOCKCall {}
    ///Container type for the return parameters of the [`TASK_CHALLENGE_WINDOW_BLOCK()`](TASK_CHALLENGE_WINDOW_BLOCKCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_CHALLENGE_WINDOW_BLOCKReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<TASK_CHALLENGE_WINDOW_BLOCKCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: TASK_CHALLENGE_WINDOW_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TASK_CHALLENGE_WINDOW_BLOCKCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<TASK_CHALLENGE_WINDOW_BLOCKReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: TASK_CHALLENGE_WINDOW_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TASK_CHALLENGE_WINDOW_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TASK_CHALLENGE_WINDOW_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = TASK_CHALLENGE_WINDOW_BLOCKReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TASK_CHALLENGE_WINDOW_BLOCK()";
            const SELECTOR: [u8; 4] = [246u8, 60u8, 91u8, 171u8];
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
    /**Function with signature `TASK_RESPONSE_WINDOW_BLOCK()` and selector `0x1ad43189`.
```solidity
function TASK_RESPONSE_WINDOW_BLOCK() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_RESPONSE_WINDOW_BLOCKCall {}
    ///Container type for the return parameters of the [`TASK_RESPONSE_WINDOW_BLOCK()`](TASK_RESPONSE_WINDOW_BLOCKCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_RESPONSE_WINDOW_BLOCKReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<TASK_RESPONSE_WINDOW_BLOCKCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: TASK_RESPONSE_WINDOW_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TASK_RESPONSE_WINDOW_BLOCKCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<TASK_RESPONSE_WINDOW_BLOCKReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: TASK_RESPONSE_WINDOW_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TASK_RESPONSE_WINDOW_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TASK_RESPONSE_WINDOW_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = TASK_RESPONSE_WINDOW_BLOCKReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TASK_RESPONSE_WINDOW_BLOCK()";
            const SELECTOR: [u8; 4] = [26u8, 212u8, 49u8, 137u8];
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
    /**Function with signature `WADS_TO_SLASH()` and selector `0x5a2d7f02`.
```solidity
function WADS_TO_SLASH() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WADS_TO_SLASHCall {}
    ///Container type for the return parameters of the [`WADS_TO_SLASH()`](WADS_TO_SLASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WADS_TO_SLASHReturn {
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
            impl ::core::convert::From<WADS_TO_SLASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: WADS_TO_SLASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for WADS_TO_SLASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<WADS_TO_SLASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: WADS_TO_SLASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for WADS_TO_SLASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for WADS_TO_SLASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = WADS_TO_SLASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WADS_TO_SLASH()";
            const SELECTOR: [u8; 4] = [90u8, 45u8, 127u8, 2u8];
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
    /**Function with signature `aggregator()` and selector `0x245a7bfc`.
```solidity
function aggregator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatorCall {}
    ///Container type for the return parameters of the [`aggregator()`](aggregatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatorReturn {
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
            impl ::core::convert::From<aggregatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: aggregatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aggregatorCall {
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
            impl ::core::convert::From<aggregatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: aggregatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aggregatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for aggregatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = aggregatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "aggregator()";
            const SELECTOR: [u8; 4] = [36u8, 90u8, 123u8, 252u8];
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
    /**Function with signature `allTaskHashes(uint32)` and selector `0x2d89f6fc`.
```solidity
function allTaskHashes(uint32) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`allTaskHashes(uint32)`](allTaskHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesReturn {
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
            impl ::core::convert::From<allTaskHashesCall> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskHashesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesCall {
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
            impl ::core::convert::From<allTaskHashesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskHashesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allTaskHashesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allTaskHashes(uint32)";
            const SELECTOR: [u8; 4] = [45u8, 137u8, 246u8, 252u8];
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
    /**Function with signature `allTaskResponses(uint32)` and selector `0x2cb223d5`.
```solidity
function allTaskResponses(uint32) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`allTaskResponses(uint32)`](allTaskResponsesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesReturn {
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
            impl ::core::convert::From<allTaskResponsesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allTaskResponsesCall {
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
            impl ::core::convert::From<allTaskResponsesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allTaskResponsesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskResponsesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allTaskResponsesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allTaskResponses(uint32)";
            const SELECTOR: [u8; 4] = [44u8, 178u8, 35u8, 213u8];
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
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
```solidity
function allocationManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall {}
    ///Container type for the return parameters of the [`allocationManager()`](allocationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerReturn {
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
            impl ::core::convert::From<allocationManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerCall {
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
            impl ::core::convert::From<allocationManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManager()";
            const SELECTOR: [u8; 4] = [202u8, 138u8, 167u8, 199u8];
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
function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureCheckerTypes.QuorumStakeTotals memory, bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
        #[allow(missing_docs)]
        pub params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](checkSignaturesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesReturn {
        #[allow(missing_docs)]
        pub _0: <IBLSSignatureCheckerTypes::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
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
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                u32,
                <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
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
                IBLSSignatureCheckerTypes::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IBLSSignatureCheckerTypes::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
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
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkSignaturesReturn;
            type ReturnTuple<'a> = (
                IBLSSignatureCheckerTypes::QuorumStakeTotals,
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
                    <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `createNewTask((string,string),uint32,bytes)` and selector `0xef029dbc`.
```solidity
function createNewTask(IIncredibleRedisTaskManager.SetInput memory input, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskCall {
        #[allow(missing_docs)]
        pub input: <IIncredibleRedisTaskManager::SetInput as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub quorumThresholdPercentage: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`createNewTask((string,string),uint32,bytes)`](createNewTaskCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskReturn {}
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
                IIncredibleRedisTaskManager::SetInput,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIncredibleRedisTaskManager::SetInput as alloy::sol_types::SolType>::RustType,
                u32,
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
            impl ::core::convert::From<createNewTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskCall) -> Self {
                    (value.input, value.quorumThresholdPercentage, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        input: tuple.0,
                        quorumThresholdPercentage: tuple.1,
                        quorumNumbers: tuple.2,
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
            impl ::core::convert::From<createNewTaskReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createNewTaskCall {
            type Parameters<'a> = (
                IIncredibleRedisTaskManager::SetInput,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createNewTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createNewTask((string,string),uint32,bytes)";
            const SELECTOR: [u8; 4] = [239u8, 2u8, 157u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IIncredibleRedisTaskManager::SetInput as alloy_sol_types::SolType>::tokenize(
                        &self.input,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.quorumThresholdPercentage,
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
    /**Function with signature `generator()` and selector `0x7afa1eed`.
```solidity
function generator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generatorCall {}
    ///Container type for the return parameters of the [`generator()`](generatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generatorReturn {
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
            impl ::core::convert::From<generatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: generatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for generatorCall {
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
            impl ::core::convert::From<generatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: generatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for generatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for generatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = generatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "generator()";
            const SELECTOR: [u8; 4] = [122u8, 250u8, 30u8, 237u8];
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
    /**Function with signature `getBatchOperatorFromId(address,bytes32[])` and selector `0x4d2b57fe`.
```solidity
function getBatchOperatorFromId(address registryCoordinator, bytes32[] memory operatorIds) external view returns (address[] memory operators);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorFromIdCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`getBatchOperatorFromId(address,bytes32[])`](getBatchOperatorFromIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorFromIdReturn {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getBatchOperatorFromIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorFromIdCall) -> Self {
                    (value.registryCoordinator, value.operatorIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBatchOperatorFromIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorIds: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getBatchOperatorFromIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorFromIdReturn) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBatchOperatorFromIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchOperatorFromIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBatchOperatorFromIdReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBatchOperatorFromId(address,bytes32[])";
            const SELECTOR: [u8; 4] = [77u8, 43u8, 87u8, 254u8];
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
                        &self.registryCoordinator,
                    ),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getBatchOperatorId(address,address[])` and selector `0x31b36bd9`.
```solidity
function getBatchOperatorId(address registryCoordinator, address[] memory operators) external view returns (bytes32[] memory operatorIds);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorIdCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getBatchOperatorId(address,address[])`](getBatchOperatorIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorIdReturn {
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getBatchOperatorIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorIdCall) -> Self {
                    (value.registryCoordinator, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBatchOperatorIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operators: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getBatchOperatorIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorIdReturn) -> Self {
                    (value.operatorIds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBatchOperatorIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorIds: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchOperatorIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBatchOperatorIdReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBatchOperatorId(address,address[])";
            const SELECTOR: [u8; 4] = [49u8, 179u8, 107u8, 217u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
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
    /**Function with signature `getCheckSignaturesIndices(address,uint32,bytes,bytes32[])` and selector `0x4f739f74`.
```solidity
function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes memory quorumNumbers, bytes32[] memory nonSignerOperatorIds) external view returns (OperatorStateRetriever.CheckSignaturesIndices memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckSignaturesIndicesCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub nonSignerOperatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`getCheckSignaturesIndices(address,uint32,bytes,bytes32[])`](getCheckSignaturesIndicesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckSignaturesIndicesReturn {
        #[allow(missing_docs)]
        pub _0: <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u32,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getCheckSignaturesIndicesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCheckSignaturesIndicesCall) -> Self {
                    (
                        value.registryCoordinator,
                        value.referenceBlockNumber,
                        value.quorumNumbers,
                        value.nonSignerOperatorIds,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCheckSignaturesIndicesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        referenceBlockNumber: tuple.1,
                        quorumNumbers: tuple.2,
                        nonSignerOperatorIds: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                OperatorStateRetriever::CheckSignaturesIndices,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCheckSignaturesIndicesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCheckSignaturesIndicesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCheckSignaturesIndicesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCheckSignaturesIndicesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCheckSignaturesIndicesReturn;
            type ReturnTuple<'a> = (OperatorStateRetriever::CheckSignaturesIndices,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCheckSignaturesIndices(address,uint32,bytes,bytes32[])";
            const SELECTOR: [u8; 4] = [79u8, 115u8, 159u8, 116u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerOperatorIds),
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
    /**Function with signature `getOperatorState(address,bytes,uint32)` and selector `0x3563b0d1`.
```solidity
function getOperatorState(address registryCoordinator, bytes memory quorumNumbers, uint32 blockNumber) external view returns (OperatorStateRetriever.Operator[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_0Call {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorState(address,bytes,uint32)`](getOperatorState_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_0Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
            >,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<getOperatorState_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_0Call) -> Self {
                    (value.registryCoordinator, value.quorumNumbers, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorState_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        quorumNumbers: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
                    >,
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
            impl ::core::convert::From<getOperatorState_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorState_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorState_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorState_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorState(address,bytes,uint32)";
            const SELECTOR: [u8; 4] = [53u8, 99u8, 176u8, 209u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
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
    /**Function with signature `getOperatorState(address,bytes32,uint32)` and selector `0xcefdc1d4`.
```solidity
function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) external view returns (uint256, OperatorStateRetriever.Operator[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_1Call {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorState(address,bytes32,uint32)`](getOperatorState_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_1Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
            >,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getOperatorState_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_1Call) -> Self {
                    (value.registryCoordinator, value.operatorId, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorState_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorId: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
                    >,
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
            impl ::core::convert::From<getOperatorState_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_1Return) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorState_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorState_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorState_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorState(address,bytes32,uint32)";
            const SELECTOR: [u8; 4] = [206u8, 253u8, 193u8, 212u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        32,
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
    /**Function with signature `getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)` and selector `0x5c155662`.
```solidity
function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] memory operatorIds, uint32 blockNumber) external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapsAtBlockNumberCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)`](getQuorumBitmapsAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapsAtBlockNumberReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
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
            impl ::core::convert::From<getQuorumBitmapsAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapsAtBlockNumberCall) -> Self {
                    (value.registryCoordinator, value.operatorIds, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapsAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorIds: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getQuorumBitmapsAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapsAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapsAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapsAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapsAtBlockNumberReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)";
            const SELECTOR: [u8; 4] = [92u8, 21u8, 86u8, 98u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIds),
                    <alloy::sol_types::sol_data::Uint<
                        32,
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
    /**Function with signature `getTaskResponseWindowBlock()` and selector `0xf5c9899d`.
```solidity
function getTaskResponseWindowBlock() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTaskResponseWindowBlockCall {}
    ///Container type for the return parameters of the [`getTaskResponseWindowBlock()`](getTaskResponseWindowBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTaskResponseWindowBlockReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<getTaskResponseWindowBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTaskResponseWindowBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTaskResponseWindowBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getTaskResponseWindowBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTaskResponseWindowBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTaskResponseWindowBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTaskResponseWindowBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTaskResponseWindowBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTaskResponseWindowBlock()";
            const SELECTOR: [u8; 4] = [245u8, 201u8, 137u8, 157u8];
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
    /**Function with signature `initialize(address,address,address,address,address,address)` and selector `0xcc2a9a5b`.
```solidity
function initialize(address initialOwner, address _aggregator, address _generator, address _allocationManager, address _slasher, address _serviceManager) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _aggregator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _generator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _slasher: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _serviceManager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address,address,address,address,address)`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value.initialOwner,
                        value._aggregator,
                        value._generator,
                        value._allocationManager,
                        value._slasher,
                        value._serviceManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        _aggregator: tuple.1,
                        _generator: tuple.2,
                        _allocationManager: tuple.3,
                        _slasher: tuple.4,
                        _serviceManager: tuple.5,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address,address,address,address,address)";
            const SELECTOR: [u8; 4] = [204u8, 42u8, 154u8, 91u8];
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
                        &self.initialOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._aggregator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._generator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._slasher,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._serviceManager,
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
    /**Function with signature `instantSlasher()` and selector `0x9b290e98`.
```solidity
function instantSlasher() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct instantSlasherCall {}
    ///Container type for the return parameters of the [`instantSlasher()`](instantSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct instantSlasherReturn {
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
            impl ::core::convert::From<instantSlasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: instantSlasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for instantSlasherCall {
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
            impl ::core::convert::From<instantSlasherReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: instantSlasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for instantSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for instantSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = instantSlasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "instantSlasher()";
            const SELECTOR: [u8; 4] = [155u8, 41u8, 14u8, 152u8];
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
    /**Function with signature `latestTaskNum()` and selector `0x8b00ce7c`.
```solidity
function latestTaskNum() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumCall {}
    ///Container type for the return parameters of the [`latestTaskNum()`](latestTaskNumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<latestTaskNumCall> for UnderlyingRustTuple<'_> {
                fn from(value: latestTaskNumCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestTaskNumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<latestTaskNumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: latestTaskNumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestTaskNumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestTaskNumCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = latestTaskNumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestTaskNum()";
            const SELECTOR: [u8; 4] = [139u8, 0u8, 206u8, 124u8];
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        #[allow(missing_docs)]
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
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPausedStatus: tuple.0 }
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        #[allow(missing_docs)]
        pub index: u8,
    }
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
        #[allow(missing_docs)]
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_0Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        8,
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
            type UnderlyingRustTuple<'a> = (
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<pauserRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauserRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauserRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `raiseAndResolveChallenge(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32,bytes32),(uint256,uint256)[])` and selector `0x92a51824`.
```solidity
function raiseAndResolveChallenge(IIncredibleRedisTaskManager.Task memory task, IIncredibleRedisTaskManager.TaskResponse memory taskResponse, IIncredibleRedisTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct raiseAndResolveChallengeCall {
        #[allow(missing_docs)]
        pub task: <IIncredibleRedisTaskManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponse: <IIncredibleRedisTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponseMetadata: <IIncredibleRedisTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`raiseAndResolveChallenge(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32,bytes32),(uint256,uint256)[])`](raiseAndResolveChallengeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct raiseAndResolveChallengeReturn {}
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
                IIncredibleRedisTaskManager::Task,
                IIncredibleRedisTaskManager::TaskResponse,
                IIncredibleRedisTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIncredibleRedisTaskManager::Task as alloy::sol_types::SolType>::RustType,
                <IIncredibleRedisTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
                <IIncredibleRedisTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<raiseAndResolveChallengeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: raiseAndResolveChallengeCall) -> Self {
                    (
                        value.task,
                        value.taskResponse,
                        value.taskResponseMetadata,
                        value.pubkeysOfNonSigningOperators,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for raiseAndResolveChallengeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        task: tuple.0,
                        taskResponse: tuple.1,
                        taskResponseMetadata: tuple.2,
                        pubkeysOfNonSigningOperators: tuple.3,
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
            impl ::core::convert::From<raiseAndResolveChallengeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: raiseAndResolveChallengeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for raiseAndResolveChallengeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for raiseAndResolveChallengeCall {
            type Parameters<'a> = (
                IIncredibleRedisTaskManager::Task,
                IIncredibleRedisTaskManager::TaskResponse,
                IIncredibleRedisTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = raiseAndResolveChallengeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "raiseAndResolveChallenge(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32,bytes32),(uint256,uint256)[])";
            const SELECTOR: [u8; 4] = [146u8, 165u8, 24u8, 36u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IIncredibleRedisTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <IIncredibleRedisTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IIncredibleRedisTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponseMetadata,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.pubkeysOfNonSigningOperators,
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
    /**Function with signature `redisState(string)` and selector `0x4f41b818`.
```solidity
function redisState(string memory) external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redisStateCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`redisState(string)`](redisStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redisStateReturn {
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
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<redisStateCall> for UnderlyingRustTuple<'_> {
                fn from(value: redisStateCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for redisStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<redisStateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: redisStateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for redisStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for redisStateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = redisStateReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "redisState(string)";
            const SELECTOR: [u8; 4] = [79u8, 65u8, 184u8, 24u8];
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
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
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
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `respondToTask(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0xb7023949`.
```solidity
function respondToTask(IIncredibleRedisTaskManager.Task memory task, IIncredibleRedisTaskManager.TaskResponse memory taskResponse, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskCall {
        #[allow(missing_docs)]
        pub task: <IIncredibleRedisTaskManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponse: <IIncredibleRedisTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub nonSignerStakesAndSignature: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`respondToTask(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](respondToTaskCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskReturn {}
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
                IIncredibleRedisTaskManager::Task,
                IIncredibleRedisTaskManager::TaskResponse,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIncredibleRedisTaskManager::Task as alloy::sol_types::SolType>::RustType,
                <IIncredibleRedisTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
                <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<respondToTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskCall) -> Self {
                    (value.task, value.taskResponse, value.nonSignerStakesAndSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondToTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        task: tuple.0,
                        taskResponse: tuple.1,
                        nonSignerStakesAndSignature: tuple.2,
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
            impl ::core::convert::From<respondToTaskReturn> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondToTaskReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for respondToTaskCall {
            type Parameters<'a> = (
                IIncredibleRedisTaskManager::Task,
                IIncredibleRedisTaskManager::TaskResponse,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = respondToTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "respondToTask(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [183u8, 2u8, 57u8, 73u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IIncredibleRedisTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <IIncredibleRedisTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerStakesAndSignature,
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
            impl ::core::convert::From<serviceManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for serviceManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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
    /**Function with signature `taskNumber()` and selector `0x72d18e8d`.
```solidity
function taskNumber() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskNumberCall {}
    ///Container type for the return parameters of the [`taskNumber()`](taskNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskNumberReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<taskNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: taskNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<taskNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: taskNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for taskNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = taskNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "taskNumber()";
            const SELECTOR: [u8; 4] = [114u8, 209u8, 142u8, 141u8];
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
    /**Function with signature `taskSuccesfullyChallenged(uint32)` and selector `0x5decc3f5`.
```solidity
function taskSuccesfullyChallenged(uint32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskSuccesfullyChallengedCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`taskSuccesfullyChallenged(uint32)`](taskSuccesfullyChallengedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskSuccesfullyChallengedReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<taskSuccesfullyChallengedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: taskSuccesfullyChallengedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for taskSuccesfullyChallengedCall {
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
            impl ::core::convert::From<taskSuccesfullyChallengedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: taskSuccesfullyChallengedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for taskSuccesfullyChallengedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for taskSuccesfullyChallengedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = taskSuccesfullyChallengedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "taskSuccesfullyChallenged(uint32)";
            const SELECTOR: [u8; 4] = [93u8, 236u8, 195u8, 245u8];
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
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
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
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`](trySignatureAndApkVerificationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationReturn {
        #[allow(missing_docs)]
        pub pairingSuccessful: bool,
        #[allow(missing_docs)]
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
    /**Function with signature `unpause(uint256)` and selector `0xfabc1cbc`.
```solidity
function unpause(uint256 newPausedStatus) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {
        #[allow(missing_docs)]
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
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPausedStatus: tuple.0 }
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
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
    ///Container for all the [`IncredibleRedisTaskManager`](self) function calls.
    pub enum IncredibleRedisTaskManagerCalls {
        #[allow(missing_docs)]
        TASK_CHALLENGE_WINDOW_BLOCK(TASK_CHALLENGE_WINDOW_BLOCKCall),
        #[allow(missing_docs)]
        TASK_RESPONSE_WINDOW_BLOCK(TASK_RESPONSE_WINDOW_BLOCKCall),
        #[allow(missing_docs)]
        WADS_TO_SLASH(WADS_TO_SLASHCall),
        #[allow(missing_docs)]
        aggregator(aggregatorCall),
        #[allow(missing_docs)]
        allTaskHashes(allTaskHashesCall),
        #[allow(missing_docs)]
        allTaskResponses(allTaskResponsesCall),
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        blsApkRegistry(blsApkRegistryCall),
        #[allow(missing_docs)]
        checkSignatures(checkSignaturesCall),
        #[allow(missing_docs)]
        createNewTask(createNewTaskCall),
        #[allow(missing_docs)]
        delegation(delegationCall),
        #[allow(missing_docs)]
        generator(generatorCall),
        #[allow(missing_docs)]
        getBatchOperatorFromId(getBatchOperatorFromIdCall),
        #[allow(missing_docs)]
        getBatchOperatorId(getBatchOperatorIdCall),
        #[allow(missing_docs)]
        getCheckSignaturesIndices(getCheckSignaturesIndicesCall),
        #[allow(missing_docs)]
        getOperatorState_0(getOperatorState_0Call),
        #[allow(missing_docs)]
        getOperatorState_1(getOperatorState_1Call),
        #[allow(missing_docs)]
        getQuorumBitmapsAtBlockNumber(getQuorumBitmapsAtBlockNumberCall),
        #[allow(missing_docs)]
        getTaskResponseWindowBlock(getTaskResponseWindowBlockCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        instantSlasher(instantSlasherCall),
        #[allow(missing_docs)]
        latestTaskNum(latestTaskNumCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        pauseAll(pauseAllCall),
        #[allow(missing_docs)]
        paused_0(paused_0Call),
        #[allow(missing_docs)]
        paused_1(paused_1Call),
        #[allow(missing_docs)]
        pauserRegistry(pauserRegistryCall),
        #[allow(missing_docs)]
        raiseAndResolveChallenge(raiseAndResolveChallengeCall),
        #[allow(missing_docs)]
        redisState(redisStateCall),
        #[allow(missing_docs)]
        registryCoordinator(registryCoordinatorCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        respondToTask(respondToTaskCall),
        #[allow(missing_docs)]
        serviceManager(serviceManagerCall),
        #[allow(missing_docs)]
        setStaleStakesForbidden(setStaleStakesForbiddenCall),
        #[allow(missing_docs)]
        stakeRegistry(stakeRegistryCall),
        #[allow(missing_docs)]
        staleStakesForbidden(staleStakesForbiddenCall),
        #[allow(missing_docs)]
        taskNumber(taskNumberCall),
        #[allow(missing_docs)]
        taskSuccesfullyChallenged(taskSuccesfullyChallengedCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        trySignatureAndApkVerification(trySignatureAndApkVerificationCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
    }
    #[automatically_derived]
    impl IncredibleRedisTaskManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [19u8, 100u8, 57u8, 221u8],
            [23u8, 31u8, 29u8, 91u8],
            [26u8, 212u8, 49u8, 137u8],
            [36u8, 90u8, 123u8, 252u8],
            [44u8, 178u8, 35u8, 213u8],
            [45u8, 137u8, 246u8, 252u8],
            [49u8, 179u8, 107u8, 217u8],
            [53u8, 99u8, 176u8, 209u8],
            [57u8, 152u8, 253u8, 211u8],
            [65u8, 108u8, 126u8, 94u8],
            [77u8, 43u8, 87u8, 254u8],
            [79u8, 65u8, 184u8, 24u8],
            [79u8, 115u8, 159u8, 116u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 45u8, 127u8, 2u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 21u8, 86u8, 98u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 236u8, 195u8, 245u8],
            [93u8, 244u8, 89u8, 70u8],
            [104u8, 48u8, 72u8, 53u8],
            [109u8, 20u8, 169u8, 135u8],
            [110u8, 251u8, 70u8, 54u8],
            [113u8, 80u8, 24u8, 166u8],
            [114u8, 209u8, 142u8, 141u8],
            [122u8, 250u8, 30u8, 237u8],
            [136u8, 111u8, 17u8, 149u8],
            [139u8, 0u8, 206u8, 124u8],
            [141u8, 165u8, 203u8, 91u8],
            [146u8, 165u8, 24u8, 36u8],
            [155u8, 41u8, 14u8, 152u8],
            [183u8, 2u8, 57u8, 73u8],
            [185u8, 141u8, 9u8, 8u8],
            [202u8, 138u8, 167u8, 199u8],
            [204u8, 42u8, 154u8, 91u8],
            [206u8, 253u8, 193u8, 212u8],
            [223u8, 92u8, 247u8, 35u8],
            [239u8, 2u8, 157u8, 188u8],
            [242u8, 253u8, 227u8, 139u8],
            [245u8, 201u8, 137u8, 157u8],
            [246u8, 60u8, 91u8, 171u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IncredibleRedisTaskManagerCalls {
        const NAME: &'static str = "IncredibleRedisTaskManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 42usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(_) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(_) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::WADS_TO_SLASH(_) => {
                    <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::aggregator(_) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allTaskHashes(_) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allTaskResponses(_) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkSignatures(_) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createNewTask(_) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::generator(_) => {
                    <generatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBatchOperatorFromId(_) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBatchOperatorId(_) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCheckSignaturesIndices(_) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorState_0(_) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorState_1(_) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapsAtBlockNumber(_) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTaskResponseWindowBlock(_) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::instantSlasher(_) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestTaskNum(_) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::raiseAndResolveChallenge(_) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::redisState(_) => {
                    <redisStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::respondToTask(_) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::serviceManager(_) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::taskNumber(_) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::taskSuccesfullyChallenged(_) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::trySignatureAndApkVerification(_) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls>] = &[
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerCalls::trySignatureAndApkVerification,
                            )
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn TASK_RESPONSE_WINDOW_BLOCK(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerCalls::TASK_RESPONSE_WINDOW_BLOCK,
                            )
                    }
                    TASK_RESPONSE_WINDOW_BLOCK
                },
                {
                    fn aggregator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <aggregatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::aggregator)
                    }
                    aggregator
                },
                {
                    fn allTaskResponses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::allTaskResponses)
                    }
                    allTaskResponses
                },
                {
                    fn allTaskHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <allTaskHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::allTaskHashes)
                    }
                    allTaskHashes
                },
                {
                    fn getBatchOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::getBatchOperatorId)
                    }
                    getBatchOperatorId
                },
                {
                    fn getOperatorState_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::getOperatorState_0)
                    }
                    getOperatorState_0
                },
                {
                    fn serviceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn setStaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerCalls::setStaleStakesForbidden,
                            )
                    }
                    setStaleStakesForbidden
                },
                {
                    fn getBatchOperatorFromId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::getBatchOperatorFromId)
                    }
                    getBatchOperatorFromId
                },
                {
                    fn redisState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <redisStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::redisState)
                    }
                    redisState
                },
                {
                    fn getCheckSignaturesIndices(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerCalls::getCheckSignaturesIndices,
                            )
                    }
                    getCheckSignaturesIndices
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn WADS_TO_SLASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::WADS_TO_SLASH)
                    }
                    WADS_TO_SLASH
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn getQuorumBitmapsAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerCalls::getQuorumBitmapsAtBlockNumber,
                            )
                    }
                    getQuorumBitmapsAtBlockNumber
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn taskSuccesfullyChallenged(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerCalls::taskSuccesfullyChallenged,
                            )
                    }
                    taskSuccesfullyChallenged
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn taskNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <taskNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::taskNumber)
                    }
                    taskNumber
                },
                {
                    fn generator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <generatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::generator)
                    }
                    generator
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn latestTaskNum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <latestTaskNumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::latestTaskNum)
                    }
                    latestTaskNum
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn raiseAndResolveChallenge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerCalls::raiseAndResolveChallenge,
                            )
                    }
                    raiseAndResolveChallenge
                },
                {
                    fn instantSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <instantSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::instantSlasher)
                    }
                    instantSlasher
                },
                {
                    fn respondToTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <respondToTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::respondToTask)
                    }
                    respondToTask
                },
                {
                    fn staleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::staleStakesForbidden)
                    }
                    staleStakesForbidden
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOperatorState_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::getOperatorState_1)
                    }
                    getOperatorState_1
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn createNewTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <createNewTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::createNewTask)
                    }
                    createNewTask
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getTaskResponseWindowBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerCalls::getTaskResponseWindowBlock,
                            )
                    }
                    getTaskResponseWindowBlock
                },
                {
                    fn TASK_CHALLENGE_WINDOW_BLOCK(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerCalls::TASK_CHALLENGE_WINDOW_BLOCK,
                            )
                    }
                    TASK_CHALLENGE_WINDOW_BLOCK
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerCalls::unpause)
                    }
                    unpause
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
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(inner) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(inner) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WADS_TO_SLASH(inner) => {
                    <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::aggregator(inner) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
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
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::generator(inner) => {
                    <generatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getBatchOperatorFromId(inner) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBatchOperatorId(inner) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCheckSignaturesIndices(inner) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorState_0(inner) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorState_1(inner) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapsAtBlockNumber(inner) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTaskResponseWindowBlock(inner) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::instantSlasher(inner) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::raiseAndResolveChallenge(inner) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::redisState(inner) => {
                    <redisStateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::taskNumber(inner) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::taskSuccesfullyChallenged(inner) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(inner) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(inner) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WADS_TO_SLASH(inner) => {
                    <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::aggregator(inner) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::generator(inner) => {
                    <generatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBatchOperatorFromId(inner) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBatchOperatorId(inner) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCheckSignaturesIndices(inner) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorState_0(inner) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorState_1(inner) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapsAtBlockNumber(inner) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTaskResponseWindowBlock(inner) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::instantSlasher(inner) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::raiseAndResolveChallenge(inner) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::redisState(inner) => {
                    <redisStateCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::taskNumber(inner) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::taskSuccesfullyChallenged(inner) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`IncredibleRedisTaskManager`](self) custom errors.
    pub enum IncredibleRedisTaskManagerErrors {
        #[allow(missing_docs)]
        BitmapValueTooLarge(BitmapValueTooLarge),
        #[allow(missing_docs)]
        BytesArrayLengthTooLong(BytesArrayLengthTooLong),
        #[allow(missing_docs)]
        BytesArrayNotOrdered(BytesArrayNotOrdered),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        ECAddFailed(ECAddFailed),
        #[allow(missing_docs)]
        ECMulFailed(ECMulFailed),
        #[allow(missing_docs)]
        ExpModFailed(ExpModFailed),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        #[allow(missing_docs)]
        InputEmptyQuorumNumbers(InputEmptyQuorumNumbers),
        #[allow(missing_docs)]
        InputNonSignerLengthMismatch(InputNonSignerLengthMismatch),
        #[allow(missing_docs)]
        InvalidBLSPairingKey(InvalidBLSPairingKey),
        #[allow(missing_docs)]
        InvalidBLSSignature(InvalidBLSSignature),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidQuorumApkHash(InvalidQuorumApkHash),
        #[allow(missing_docs)]
        InvalidReferenceBlocknumber(InvalidReferenceBlocknumber),
        #[allow(missing_docs)]
        NonSignerPubkeysNotSorted(NonSignerPubkeysNotSorted),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyRegistryCoordinatorOwner(OnlyRegistryCoordinatorOwner),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        OperatorNotRegistered(OperatorNotRegistered),
        #[allow(missing_docs)]
        ScalarTooLarge(ScalarTooLarge),
        #[allow(missing_docs)]
        StaleStakesForbidden(StaleStakesForbidden),
    }
    #[automatically_derived]
    impl IncredibleRedisTaskManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [31u8, 4u8, 5u8, 160u8],
            [37u8, 236u8, 108u8, 31u8],
            [67u8, 113u8, 74u8, 253u8],
            [70u8, 51u8, 190u8, 50u8],
            [75u8, 135u8, 79u8, 69u8],
            [95u8, 131u8, 47u8, 65u8],
            [103u8, 152u8, 141u8, 51u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [128u8, 200u8, 131u8, 72u8],
            [132u8, 10u8, 72u8, 213u8],
            [171u8, 27u8, 35u8, 107u8],
            [175u8, 252u8, 94u8, 219u8],
            [198u8, 29u8, 202u8, 93u8],
            [202u8, 149u8, 115u8, 51u8],
            [212u8, 182u8, 143u8, 215u8],
            [213u8, 30u8, 218u8, 227u8],
            [224u8, 225u8, 231u8, 98u8],
            [225u8, 49u8, 10u8, 237u8],
            [251u8, 74u8, 156u8, 142u8],
            [255u8, 113u8, 148u8, 20u8],
            [255u8, 137u8, 212u8, 250u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IncredibleRedisTaskManagerErrors {
        const NAME: &'static str = "IncredibleRedisTaskManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 23usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BitmapValueTooLarge(_) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayLengthTooLong(_) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayNotOrdered(_) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECAddFailed(_) => {
                    <ECAddFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECMulFailed(_) => {
                    <ECMulFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpModFailed(_) => {
                    <ExpModFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputEmptyQuorumNumbers(_) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputNonSignerLengthMismatch(_) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBLSPairingKey(_) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBLSSignature(_) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidQuorumApkHash(_) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidReferenceBlocknumber(_) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonSignerPubkeysNotSorted(_) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => {
                    <OnlyPauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyRegistryCoordinatorOwner(_) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ScalarTooLarge(_) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StaleStakesForbidden(_) => {
                    <StaleStakesForbidden as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors>] = &[
                {
                    fn InputEmptyQuorumNumbers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerErrors::InputEmptyQuorumNumbers,
                            )
                    }
                    InputEmptyQuorumNumbers
                },
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerErrors::InputArrayLengthMismatch,
                            )
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn ECMulFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn InvalidReferenceBlocknumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerErrors::InvalidReferenceBlocknumber,
                            )
                    }
                    InvalidReferenceBlocknumber
                },
                {
                    fn InputNonSignerLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerErrors::InputNonSignerLengthMismatch,
                            )
                    }
                    InputNonSignerLengthMismatch
                },
                {
                    fn InvalidBLSPairingKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::InvalidBLSPairingKey)
                    }
                    InvalidBLSPairingKey
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::BytesArrayNotOrdered)
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidBLSSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <InvalidBLSSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::InvalidBLSSignature)
                    }
                    InvalidBLSSignature
                },
                {
                    fn StaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <StaleStakesForbidden as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::StaleStakesForbidden)
                    }
                    StaleStakesForbidden
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerErrors::InvalidNewPausedStatus,
                            )
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn ECAddFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn OnlyRegistryCoordinatorOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerErrors::OnlyRegistryCoordinatorOwner,
                            )
                    }
                    OnlyRegistryCoordinatorOwner
                },
                {
                    fn InvalidQuorumApkHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::InvalidQuorumApkHash)
                    }
                    InvalidQuorumApkHash
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerErrors::BytesArrayLengthTooLong,
                            )
                    }
                    BytesArrayLengthTooLong
                },
                {
                    fn NonSignerPubkeysNotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleRedisTaskManagerErrors::NonSignerPubkeysNotSorted,
                            )
                    }
                    NonSignerPubkeysNotSorted
                },
                {
                    fn ScalarTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleRedisTaskManagerErrors> {
                        <ScalarTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleRedisTaskManagerErrors::ScalarTooLarge)
                    }
                    ScalarTooLarge
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
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputEmptyQuorumNumbers(inner) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputNonSignerLengthMismatch(inner) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBLSPairingKey(inner) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBLSSignature(inner) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidQuorumApkHash(inner) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidReferenceBlocknumber(inner) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonSignerPubkeysNotSorted(inner) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StaleStakesForbidden(inner) => {
                    <StaleStakesForbidden as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputEmptyQuorumNumbers(inner) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputNonSignerLengthMismatch(inner) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBLSPairingKey(inner) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBLSSignature(inner) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidQuorumApkHash(inner) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidReferenceBlocknumber(inner) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NonSignerPubkeysNotSorted(inner) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StaleStakesForbidden(inner) => {
                    <StaleStakesForbidden as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IncredibleRedisTaskManager`](self) events.
    pub enum IncredibleRedisTaskManagerEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        NewTaskCreated(NewTaskCreated),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        StaleStakesForbiddenUpdate(StaleStakesForbiddenUpdate),
        #[allow(missing_docs)]
        TaskChallengedSuccessfully(TaskChallengedSuccessfully),
        #[allow(missing_docs)]
        TaskChallengedUnsuccessfully(TaskChallengedUnsuccessfully),
        #[allow(missing_docs)]
        TaskCompleted(TaskCompleted),
        #[allow(missing_docs)]
        TaskResponded(TaskResponded),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl IncredibleRedisTaskManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                53u8,
                130u8,
                209u8,
                130u8,
                142u8,
                38u8,
                191u8,
                86u8,
                189u8,
                128u8,
                21u8,
                2u8,
                188u8,
                2u8,
                26u8,
                192u8,
                188u8,
                138u8,
                251u8,
                87u8,
                200u8,
                38u8,
                228u8,
                152u8,
                107u8,
                69u8,
                89u8,
                60u8,
                143u8,
                173u8,
                56u8,
                156u8,
            ],
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
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                154u8,
                20u8,
                79u8,
                34u8,
                138u8,
                147u8,
                27u8,
                157u8,
                13u8,
                22u8,
                150u8,
                251u8,
                205u8,
                175u8,
                49u8,
                11u8,
                36u8,
                181u8,
                210u8,
                210u8,
                30u8,
                121u8,
                157u8,
                182u8,
                35u8,
                252u8,
                152u8,
                106u8,
                15u8,
                84u8,
                116u8,
                48u8,
            ],
            [
                166u8,
                177u8,
                145u8,
                45u8,
                6u8,
                111u8,
                211u8,
                165u8,
                65u8,
                58u8,
                98u8,
                34u8,
                165u8,
                120u8,
                90u8,
                228u8,
                63u8,
                36u8,
                36u8,
                88u8,
                243u8,
                238u8,
                244u8,
                92u8,
                80u8,
                221u8,
                128u8,
                137u8,
                135u8,
                184u8,
                120u8,
                253u8,
            ],
            [
                171u8,
                64u8,
                163u8,
                116u8,
                188u8,
                81u8,
                222u8,
                55u8,
                34u8,
                0u8,
                168u8,
                188u8,
                152u8,
                26u8,
                248u8,
                201u8,
                236u8,
                220u8,
                8u8,
                223u8,
                218u8,
                239u8,
                11u8,
                182u8,
                224u8,
                159u8,
                136u8,
                243u8,
                198u8,
                22u8,
                239u8,
                61u8,
            ],
            [
                194u8,
                13u8,
                27u8,
                176u8,
                241u8,
                98u8,
                54u8,
                128u8,
                48u8,
                107u8,
                131u8,
                212u8,
                255u8,
                75u8,
                185u8,
                154u8,
                43u8,
                235u8,
                157u8,
                134u8,
                217u8,
                120u8,
                50u8,
                243u8,
                202u8,
                64u8,
                253u8,
                19u8,
                162u8,
                157u8,
                241u8,
                236u8,
            ],
            [
                242u8,
                175u8,
                17u8,
                250u8,
                215u8,
                61u8,
                67u8,
                73u8,
                201u8,
                156u8,
                246u8,
                47u8,
                41u8,
                141u8,
                51u8,
                118u8,
                65u8,
                234u8,
                11u8,
                183u8,
                192u8,
                245u8,
                168u8,
                219u8,
                146u8,
                169u8,
                138u8,
                39u8,
                95u8,
                115u8,
                79u8,
                88u8,
            ],
            [
                253u8,
                62u8,
                38u8,
                190u8,
                235u8,
                89u8,
                103u8,
                252u8,
                90u8,
                87u8,
                160u8,
                68u8,
                105u8,
                20u8,
                234u8,
                188u8,
                69u8,
                180u8,
                170u8,
                71u8,
                76u8,
                103u8,
                165u8,
                27u8,
                75u8,
                81u8,
                96u8,
                202u8,
                198u8,
                13u8,
                219u8,
                5u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IncredibleRedisTaskManagerEvents {
        const NAME: &'static str = "IncredibleRedisTaskManagerEvents";
        const COUNT: usize = 10usize;
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
                Some(<NewTaskCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTaskCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NewTaskCreated)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Paused)
                }
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
                Some(
                    <TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TaskChallengedSuccessfully)
                }
                Some(
                    <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TaskChallengedUnsuccessfully)
                }
                Some(<TaskCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TaskCompleted)
                }
                Some(<TaskResponded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskResponded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TaskResponded)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Unpaused)
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
    impl alloy_sol_types::private::IntoLogData for IncredibleRedisTaskManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewTaskCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskChallengedSuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskChallengedUnsuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskResponded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewTaskCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskChallengedSuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskChallengedUnsuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskResponded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IncredibleRedisTaskManager`](self) contract instance.

See the [wrapper's documentation](`IncredibleRedisTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IncredibleRedisTaskManagerInstance<T, P, N> {
        IncredibleRedisTaskManagerInstance::<T, P, N>::new(address, provider)
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
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IncredibleRedisTaskManagerInstance<T, P, N>>,
    > {
        IncredibleRedisTaskManagerInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _registryCoordinator,
            _pauserRegistry,
            _taskResponseWindowBlock,
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
        _registryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        IncredibleRedisTaskManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _registryCoordinator,
            _pauserRegistry,
            _taskResponseWindowBlock,
        )
    }
    /**A [`IncredibleRedisTaskManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IncredibleRedisTaskManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IncredibleRedisTaskManagerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IncredibleRedisTaskManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IncredibleRedisTaskManagerInstance")
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
    > IncredibleRedisTaskManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IncredibleRedisTaskManager`](self) contract instance.

See the [wrapper's documentation](`IncredibleRedisTaskManagerInstance`) for more details.*/
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
            _pauserRegistry: alloy::sol_types::private::Address,
            _taskResponseWindowBlock: u32,
        ) -> alloy_contract::Result<IncredibleRedisTaskManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _registryCoordinator,
                _pauserRegistry,
                _taskResponseWindowBlock,
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
            _registryCoordinator: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _taskResponseWindowBlock: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _registryCoordinator,
                            _pauserRegistry,
                            _taskResponseWindowBlock,
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
    impl<T, P: ::core::clone::Clone, N> IncredibleRedisTaskManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> IncredibleRedisTaskManagerInstance<T, P, N> {
            IncredibleRedisTaskManagerInstance {
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
    > IncredibleRedisTaskManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`TASK_CHALLENGE_WINDOW_BLOCK`] function.
        pub fn TASK_CHALLENGE_WINDOW_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TASK_CHALLENGE_WINDOW_BLOCKCall, N> {
            self.call_builder(&TASK_CHALLENGE_WINDOW_BLOCKCall {})
        }
        ///Creates a new call builder for the [`TASK_RESPONSE_WINDOW_BLOCK`] function.
        pub fn TASK_RESPONSE_WINDOW_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TASK_RESPONSE_WINDOW_BLOCKCall, N> {
            self.call_builder(&TASK_RESPONSE_WINDOW_BLOCKCall {})
        }
        ///Creates a new call builder for the [`WADS_TO_SLASH`] function.
        pub fn WADS_TO_SLASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, WADS_TO_SLASHCall, N> {
            self.call_builder(&WADS_TO_SLASHCall {})
        }
        ///Creates a new call builder for the [`aggregator`] function.
        pub fn aggregator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, aggregatorCall, N> {
            self.call_builder(&aggregatorCall {})
        }
        ///Creates a new call builder for the [`allTaskHashes`] function.
        pub fn allTaskHashes(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, allTaskHashesCall, N> {
            self.call_builder(&allTaskHashesCall { _0 })
        }
        ///Creates a new call builder for the [`allTaskResponses`] function.
        pub fn allTaskResponses(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, allTaskResponsesCall, N> {
            self.call_builder(&allTaskResponsesCall { _0 })
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
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
            params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
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
        ///Creates a new call builder for the [`createNewTask`] function.
        pub fn createNewTask(
            &self,
            input: <IIncredibleRedisTaskManager::SetInput as alloy::sol_types::SolType>::RustType,
            quorumThresholdPercentage: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, createNewTaskCall, N> {
            self.call_builder(
                &createNewTaskCall {
                    input,
                    quorumThresholdPercentage,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`generator`] function.
        pub fn generator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, generatorCall, N> {
            self.call_builder(&generatorCall {})
        }
        ///Creates a new call builder for the [`getBatchOperatorFromId`] function.
        pub fn getBatchOperatorFromId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchOperatorFromIdCall, N> {
            self.call_builder(
                &getBatchOperatorFromIdCall {
                    registryCoordinator,
                    operatorIds,
                },
            )
        }
        ///Creates a new call builder for the [`getBatchOperatorId`] function.
        pub fn getBatchOperatorId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchOperatorIdCall, N> {
            self.call_builder(
                &getBatchOperatorIdCall {
                    registryCoordinator,
                    operators,
                },
            )
        }
        ///Creates a new call builder for the [`getCheckSignaturesIndices`] function.
        pub fn getCheckSignaturesIndices(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            referenceBlockNumber: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
            nonSignerOperatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCheckSignaturesIndicesCall, N> {
            self.call_builder(
                &getCheckSignaturesIndicesCall {
                    registryCoordinator,
                    referenceBlockNumber,
                    quorumNumbers,
                    nonSignerOperatorIds,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorState_0`] function.
        pub fn getOperatorState_0(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorState_0Call, N> {
            self.call_builder(
                &getOperatorState_0Call {
                    registryCoordinator,
                    quorumNumbers,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorState_1`] function.
        pub fn getOperatorState_1(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorState_1Call, N> {
            self.call_builder(
                &getOperatorState_1Call {
                    registryCoordinator,
                    operatorId,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getQuorumBitmapsAtBlockNumber`] function.
        pub fn getQuorumBitmapsAtBlockNumber(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getQuorumBitmapsAtBlockNumberCall,
            N,
        > {
            self.call_builder(
                &getQuorumBitmapsAtBlockNumberCall {
                    registryCoordinator,
                    operatorIds,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getTaskResponseWindowBlock`] function.
        pub fn getTaskResponseWindowBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTaskResponseWindowBlockCall, N> {
            self.call_builder(&getTaskResponseWindowBlockCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _aggregator: alloy::sol_types::private::Address,
            _generator: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _slasher: alloy::sol_types::private::Address,
            _serviceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initialOwner,
                    _aggregator,
                    _generator,
                    _allocationManager,
                    _slasher,
                    _serviceManager,
                },
            )
        }
        ///Creates a new call builder for the [`instantSlasher`] function.
        pub fn instantSlasher(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, instantSlasherCall, N> {
            self.call_builder(&instantSlasherCall {})
        }
        ///Creates a new call builder for the [`latestTaskNum`] function.
        pub fn latestTaskNum(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, latestTaskNumCall, N> {
            self.call_builder(&latestTaskNumCall {})
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
        pub fn pauseAll(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauseAllCall, N> {
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
        pub fn paused_1(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, paused_1Call, N> {
            self.call_builder(&paused_1Call {})
        }
        ///Creates a new call builder for the [`pauserRegistry`] function.
        pub fn pauserRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauserRegistryCall, N> {
            self.call_builder(&pauserRegistryCall {})
        }
        ///Creates a new call builder for the [`raiseAndResolveChallenge`] function.
        pub fn raiseAndResolveChallenge(
            &self,
            task: <IIncredibleRedisTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IIncredibleRedisTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            taskResponseMetadata: <IIncredibleRedisTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
            pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, raiseAndResolveChallengeCall, N> {
            self.call_builder(
                &raiseAndResolveChallengeCall {
                    task,
                    taskResponse,
                    taskResponseMetadata,
                    pubkeysOfNonSigningOperators,
                },
            )
        }
        ///Creates a new call builder for the [`redisState`] function.
        pub fn redisState(
            &self,
            _0: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, redisStateCall, N> {
            self.call_builder(&redisStateCall { _0 })
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`respondToTask`] function.
        pub fn respondToTask(
            &self,
            task: <IIncredibleRedisTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IIncredibleRedisTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            nonSignerStakesAndSignature: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, respondToTaskCall, N> {
            self.call_builder(
                &respondToTaskCall {
                    task,
                    taskResponse,
                    nonSignerStakesAndSignature,
                },
            )
        }
        ///Creates a new call builder for the [`serviceManager`] function.
        pub fn serviceManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceManagerCall, N> {
            self.call_builder(&serviceManagerCall {})
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
        ///Creates a new call builder for the [`taskNumber`] function.
        pub fn taskNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, taskNumberCall, N> {
            self.call_builder(&taskNumberCall {})
        }
        ///Creates a new call builder for the [`taskSuccesfullyChallenged`] function.
        pub fn taskSuccesfullyChallenged(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, taskSuccesfullyChallengedCall, N> {
            self.call_builder(
                &taskSuccesfullyChallengedCall {
                    _0,
                },
            )
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
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
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IncredibleRedisTaskManagerInstance<T, P, N> {
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
        ///Creates a new event filter for the [`NewTaskCreated`] event.
        pub fn NewTaskCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, NewTaskCreated, N> {
            self.event_filter::<NewTaskCreated>()
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
        ///Creates a new event filter for the [`StaleStakesForbiddenUpdate`] event.
        pub fn StaleStakesForbiddenUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StaleStakesForbiddenUpdate, N> {
            self.event_filter::<StaleStakesForbiddenUpdate>()
        }
        ///Creates a new event filter for the [`TaskChallengedSuccessfully`] event.
        pub fn TaskChallengedSuccessfully_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskChallengedSuccessfully, N> {
            self.event_filter::<TaskChallengedSuccessfully>()
        }
        ///Creates a new event filter for the [`TaskChallengedUnsuccessfully`] event.
        pub fn TaskChallengedUnsuccessfully_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskChallengedUnsuccessfully, N> {
            self.event_filter::<TaskChallengedUnsuccessfully>()
        }
        ///Creates a new event filter for the [`TaskCompleted`] event.
        pub fn TaskCompleted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskCompleted, N> {
            self.event_filter::<TaskCompleted>()
        }
        ///Creates a new event filter for the [`TaskResponded`] event.
        pub fn TaskResponded_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskResponded, N> {
            self.event_filter::<TaskResponded>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
