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
library IBLSSignatureChecker {
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
pub mod IBLSSignatureChecker {
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
///Module containing a contract's types and functions.
/**

```solidity
library IRewardsCoordinatorTypes {
    struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
    struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IRewardsCoordinatorTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RewardsSubmission {
        #[allow(missing_docs)]
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub startTimestamp: u32,
        #[allow(missing_docs)]
        pub duration: u32,
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
            alloy::sol_types::sol_data::Array<StrategyAndMultiplier>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<RewardsSubmission> for UnderlyingRustTuple<'_> {
            fn from(value: RewardsSubmission) -> Self {
                (
                    value.strategiesAndMultipliers,
                    value.token,
                    value.amount,
                    value.startTimestamp,
                    value.duration,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RewardsSubmission {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategiesAndMultipliers: tuple.0,
                    token: tuple.1,
                    amount: tuple.2,
                    startTimestamp: tuple.3,
                    duration: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RewardsSubmission {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RewardsSubmission {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.strategiesAndMultipliers,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
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
        impl alloy_sol_types::SolType for RewardsSubmission {
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
        impl alloy_sol_types::SolStruct for RewardsSubmission {
            const NAME: &'static str = "RewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,uint256 amount,uint32 startTimestamp,uint32 duration)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategiesAndMultipliers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.duration)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RewardsSubmission {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategiesAndMultipliers,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.duration,
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
                    StrategyAndMultiplier,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategiesAndMultipliers,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.duration,
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
struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyAndMultiplier {
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyAndMultiplier> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyAndMultiplier) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyAndMultiplier {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyAndMultiplier {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyAndMultiplier {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
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
        impl alloy_sol_types::SolType for StrategyAndMultiplier {
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
        impl alloy_sol_types::SolStruct for StrategyAndMultiplier {
            const NAME: &'static str = "StrategyAndMultiplier";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyAndMultiplier(address strategy,uint96 multiplier)",
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
        impl alloy_sol_types::EventTopic for StrategyAndMultiplier {
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
    /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRewardsCoordinatorTypesInstance<T, P, N> {
        IRewardsCoordinatorTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRewardsCoordinatorTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IRewardsCoordinatorTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRewardsCoordinatorTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRewardsCoordinatorTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRewardsCoordinatorTypesInstance")
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
    > IRewardsCoordinatorTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IRewardsCoordinatorTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRewardsCoordinatorTypesInstance<T, P, N> {
            IRewardsCoordinatorTypesInstance {
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
    > IRewardsCoordinatorTypesInstance<T, P, N> {
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
    > IRewardsCoordinatorTypesInstance<T, P, N> {
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
    clippy::style,
    clippy::empty_structs_with_brackets
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
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry>
        for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::private::SolTypeValue<Self>
        for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
            f.debug_tuple("ISignatureUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
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
    > ISignatureUtilsInstance<T, P, N> {
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
    > ISignatureUtilsInstance<T, P, N> {
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

library IRewardsCoordinatorTypes {
    struct RewardsSubmission {
        StrategyAndMultiplier[] strategiesAndMultipliers;
        address token;
        uint256 amount;
        uint32 startTimestamp;
        uint32 duration;
    }
    struct StrategyAndMultiplier {
        address strategy;
        uint96 multiplier;
    }
}

library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface MockAvsServiceManager {
    error BitmapValueTooLarge();
    error BytesArrayLengthTooLong();
    error BytesArrayNotOrdered();
    error DelayPeriodNotPassed();
    error ECAddFailed();
    error ECMulFailed();
    error ExpModFailed();
    error InputArrayLengthMismatch();
    error InputEmptyQuorumNumbers();
    error InputNonSignerLengthMismatch();
    error InvalidBLSPairingKey();
    error InvalidBLSSignature();
    error InvalidQuorumApkHash();
    error InvalidReferenceBlocknumber();
    error NonSignerPubkeysNotSorted();
    error OnlyRegistryCoordinator();
    error OnlyRegistryCoordinatorOwner();
    error OnlyRewardsInitiator();
    error OnlySlasher();
    error OnlyStakeRegistry();
    error ScalarTooLarge();
    error StaleStakesForbidden();

    event Initialized(uint8 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);
    event StaleStakesForbiddenUpdate(bool value);

    constructor(address _registryCoordinator, address _avsDirectory, address _rewardsCoordinator, address _allocationManager, address _permissionController);

    function SLASHER_PROPOSAL_DELAY() external view returns (uint256);
    function addPendingAdmin(address admin) external;
    function avsDirectory() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureChecker.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureChecker.QuorumStakeTotals memory, bytes32);
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function delegation() external view returns (address);
    function deregisterOperatorFromAVS(address operator) external;
    function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
    function getOperatorRestakedStrategies(address operator) external view returns (address[] memory);
    function getRestakeableStrategies() external view returns (address[] memory);
    function initialize(address _initialOwner) external;
    function migrationFinalized() external view returns (bool);
    function owner() external view returns (address);
    function proposedSlasher() external view returns (address);
    function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function registryCoordinator() external view returns (address);
    function removeAdmin(address admin) external;
    function removeAppointee(address appointee, address target, bytes4 selector) external;
    function removePendingAdmin(address pendingAdmin) external;
    function renounceOwnership() external;
    function rewardsInitiator() external view returns (address);
    function setAppointee(address appointee, address target, bytes4 selector) external;
    function setRewardsInitiator(address newRewardsInitiator) external;
    function setStaleStakesForbidden(bool value) external;
    function slasher() external view returns (address);
    function slasherProposalTimestamp() external view returns (uint256);
    function stakeRegistry() external view returns (address);
    function staleStakesForbidden() external view returns (bool);
    function transferOwnership(address newOwner) external;
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
    function updateAVSMetadataURI(string memory _metadataURI) external;
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
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      },
      {
        "name": "_rewardsCoordinator",
        "type": "address",
        "internalType": "contract IRewardsCoordinator"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      },
      {
        "name": "_permissionController",
        "type": "address",
        "internalType": "contract IPermissionController"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "SLASHER_PROPOSAL_DELAY",
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
    "name": "addPendingAdmin",
    "inputs": [
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "avsDirectory",
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
    "name": "createAVSRewardsSubmission",
    "inputs": [
      {
        "name": "rewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinatorTypes.RewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinatorTypes.StrategyAndMultiplier[]",
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
            "name": "token",
            "type": "address",
            "internalType": "contract IERC20"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "duration",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "deregisterOperatorFromAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperatorFromOperatorSets",
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
    "name": "getOperatorRestakedStrategies",
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
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRestakeableStrategies",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "migrationFinalized",
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
    "name": "proposedSlasher",
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
    "name": "registerOperatorToAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
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
    "name": "removeAdmin",
    "inputs": [
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeAppointee",
    "inputs": [
      {
        "name": "appointee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "selector",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removePendingAdmin",
    "inputs": [
      {
        "name": "pendingAdmin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "rewardsInitiator",
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
    "name": "setAppointee",
    "inputs": [
      {
        "name": "appointee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "selector",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setRewardsInitiator",
    "inputs": [
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "slasher",
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
    "name": "slasherProposalTimestamp",
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
    "name": "updateAVSMetadataURI",
    "inputs": [
      {
        "name": "_metadataURI",
        "type": "string",
        "internalType": "string"
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
    "name": "RewardsInitiatorUpdated",
    "inputs": [
      {
        "name": "prevRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
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
    "name": "DelayPeriodNotPassed",
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
    "name": "OnlyRegistryCoordinator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyRegistryCoordinatorOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyRewardsInitiator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlySlasher",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyStakeRegistry",
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
pub mod MockAvsServiceManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101c080604052346102b15760a081613b1a803803809161002082856103e7565b8339810103126102b15780516001600160a01b038116918282036102b15760208101516001600160a01b03811681036102b1576040820151906001600160a01b03821682036102b1576060830151926001600160a01b03841684036102b15760800151916001600160a01b03831683036102b157604051636830483560e01b8152916020836004818a5afa9283156102bd575f936103c6575b5060805260c0528360e052610100526101205260a0525f5460ff8160081c166103715760ff80821610610337575b5061014052604051636830483560e01b8152602081600481855afa9182156102bd576004926020925f9161030a575b5061016052604051632efa2ca360e11b815292839182905afa9081156102bd575f916102c8575b50610180526101605160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa9081156102bd575f91610277575b506101a0526040516136dc908161043e823960805181818161088e01528181610f1801528181610fcd01526125d3015260a05181611199015260c051816129a9015260e051818181610ee80152818161114101528181611787015281816125a3015261277701526101005181818161182301526127c30152610120518181816104620152818161053a015281816105ea01528181610e62015261108b01526101405181818161072f015281816108d201528181611d8401528181611e77015261244201526101605181818161084a0152818161219501526122f30152610180518181816107e901526120d101526101a0518181816113180152611f8a0152f35b90506020813d6020116102b5575b81610292602093836103e7565b810103126102b157516001600160a01b03811681036102b1575f610177565b5f80fd5b3d9150610285565b6040513d5f823e3d90fd5b90506020813d602011610302575b816102e3602093836103e7565b810103126102b157516001600160a01b03811681036102b1575f61013d565b3d91506102d6565b61032a9150833d8511610330575b61032281836103e7565b81019061041e565b5f610116565b503d610318565b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100e7565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b6103e091935060203d6020116103305761032281836103e7565b915f6100b9565b601f909101601f19168101906001600160401b0382119082101761040a57604052565b634e487b7160e01b5f52604160045260245ffd5b908160209103126102b157516001600160a01b03811681036102b1579056fe60806040526004361015610011575f80fd5b5f3560e01c8063171f1d5b146102145780631785f53c1461020f5780631fdb0cfd1461020a578063279432eb1461020557806333cfb7b7146102005780633bc28c8c146101fb578063416c7e5e146101f65780635df45946146101f157806367940c89146101ec57806368304835146101e75780636b3aa72e146101e25780636d14a987146101dd5780636efb4636146101d8578063715018a6146101d35780638d68349a146101ce5780638da5cb5b146101c95780639926ee7d146101c45780639da16d8e146101bf578063a364f4da146101ba578063a98fb355146101b5578063b1344271146101b0578063b98d0908146101ab578063ba550880146101a6578063c1a8e2c5146101a1578063c4d66de81461019c578063df5cf72314610197578063e46f181614610192578063e481af9d1461018d578063f2fde38b14610188578063fc299dee14610183578063fcd1c3751461017e5763fce36c7d14610179575f80fd5b611460565b611443565b61141b565b61138a565b61136f565b611347565b611303565b61120e565b6110ff565b611073565b611051565b611029565b610f81565b610ec4565b610e3a565b610d9e565b610d25565b610d03565b610ca8565b610c13565b6108bd565b610879565b610835565b610818565b6107d4565b6106fd565b6106c6565b61068e565b6105c2565b610522565b61043a565b6103c1565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761024857604052565b610219565b606081019081106001600160401b0382111761024857604052565b90601f801991011681019081106001600160401b0382111761024857604052565b6040519061029961010083610268565b565b60405190610299604083610268565b906102996040519283610268565b60409060e31901126102e157604051906102d18261022d565b60e4358252610104356020830152565b5f80fd5b91908260409103126102e1576040516102fd8161022d565b6020808294803584520135910152565b9080601f830112156102e15760405191610328604084610268565b8290604081019283116102e157905b8282106103445750505090565b8135815260209182019101610337565b9060806063198301126102e15760405161036d8161022d565b6020610388829461037f81606461030d565b845260a461030d565b910152565b91906080838203126102e1576020610388604051926103ab8461022d565b604084966103b9838261030d565b86520161030d565b346102e1576101203660031901126102e15760043560403660231901126102e15761041960409182516103f38161022d565b6024358152604435602082015261040936610354565b90610413366102b8565b926114f9565b8251911515825215156020820152f35b6001600160a01b038116036102e157565b346102e1575f60203660031901126102e15760043561045881610429565b6104606131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102e15760405163268959e560e01b81523060048201526001600160a01b0390921660248301525f908290818381604481015b03925af180156104e0576104d2575080f35b6104de91505f90610268565b005b6115df565b60609060031901126102e1576004356104fd81610429565b9060243561050a81610429565b906044356001600160e01b0319811681036102e15790565b346102e157610530366104e5565b6105386131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692833b156102e157604051634a86c03760e11b81523060048201526001600160a01b039182166024820152921660448301526001600160e01b03191660648201525f81608481015b93818381819703925af180156104e0576104d2575080f35b346102e1575f60203660031901126102e1576004356105e081610429565b6105e86131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102e15760405163eb5a4e8760e01b81523060048201526001600160a01b0390921660248301525f908290818381604481016104c0565b60206040818301928281528451809452019201905f5b81811061066f5750505090565b82516001600160a01b0316845260209384019390920191600101610662565b346102e15760203660031901126102e1576106c26106b66004356106b181610429565b611768565b6040519182918261064c565b0390f35b346102e15760203660031901126102e1576104de6004356106e681610429565b6106ee6131f7565b6132ed565b801515036102e157565b346102e15760203660031901126102e15760043561071a816106f3565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156104e0575f9161078f575b506001600160a01b03163303610780576104de9061334b565b637070f3b160e11b5f5260045ffd5b90506020813d6020116107c2575b816107aa60209383610268565b810103126102e157516107bc81610429565b5f610767565b3d915061079d565b5f9103126102e157565b346102e1575f3660031901126102e1576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102e1575f3660031901126102e157602060405162093a808152f35b346102e1575f3660031901126102e1576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102e1575f3660031901126102e1576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102e1575f3660031901126102e1576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b63ffffffff8116036102e157565b6044359061029982610901565b359061029982610901565b6001600160401b0381116102485760051b60200190565b9080601f830112156102e157813561095581610927565b926109636040519485610268565b81845260208085019260051b8201019283116102e157602001905b82821061098b5750505090565b60208091833561099a81610901565b81520191019061097e565b81601f820112156102e15780356109bb81610927565b926109c96040519485610268565b81845260208085019260061b840101928184116102e157602001915b8383106109f3575050505090565b6020604091610a0284866102e5565b8152019201916109e5565b9080601f830112156102e1578135610a2481610927565b92610a326040519485610268565b81845260208085019260051b820101918383116102e15760208201905b838210610a5e57505050505090565b81356001600160401b0381116102e157602091610a808784809488010161093e565b815201910190610a4f565b919091610180818403126102e157610aa1610289565b9281356001600160401b0381116102e15781610abe91840161093e565b845260208201356001600160401b0381116102e15781610adf9184016109a5565b602085015260408201356001600160401b0381116102e15781610b039184016109a5565b6040850152610b15816060840161038d565b6060850152610b278160e084016102e5565b60808501526101208201356001600160401b0381116102e15781610b4c91840161093e565b60a08501526101408201356001600160401b0381116102e15781610b7191840161093e565b60c08501526101608201356001600160401b0381116102e157610b949201610a0d565b60e0830152565b90602080835192838152019201905f5b818110610bb85750505090565b82516001600160601b0316845260209384019390920191600101610bab565b929190610c0e6020916040865282610bfa82516040808a01526080890190610b9b565b910151868203603f19016060880152610b9b565b930152565b346102e15760803660031901126102e1576004356024356001600160401b0381116102e157366023820112156102e15780600401356001600160401b0381116102e15736602482840101116102e157610c6a61090f565b90606435936001600160401b0385116102e1576024610c90610c98963690600401610a8b565b940190611ca7565b906106c260405192839283610bd7565b346102e1575f3660031901126102e157610cc06131f7565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346102e1575f3660031901126102e157602060ff606954166040519015158152f35b346102e1575f3660031901126102e1576033546040516001600160a01b039091168152602090f35b6001600160401b03811161024857601f01601f191660200190565b929192610d7482610d4d565b91610d826040519384610268565b8294818452818301116102e1578281602093845f960137010152565b346102e15760403660031901126102e157600435610dbb81610429565b602435906001600160401b0382116102e157606060031983360301126102e15760405190610de88261024d565b82600401356001600160401b0381116102e1578301366023820112156102e1576104de93610e226044923690602460048201359101610d68565b8452602481013560208501520135604083015261259d565b346102e1575f60203660031901126102e157600435610e5881610429565b610e606131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102e157604051634f906cf960e01b81523060048201526001600160a01b0390921660248301525f908290818381604481016104c0565b346102e1575f60203660031901126102e157600435610ee281610429565b610f16337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612587565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156102e1576040516351b27a6d60e11b81526001600160a01b039091166004820152905f908290602490829084905af180156104e0576104d2575080f35b346102e1575f60203660031901126102e1576004356001600160401b0381116102e157366023820112156102e157610fc3903690602481600401359101610d68565b610fcb6131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102e15760405163a98fb35560e01b815260206004820152915f9183918290849082906104c0906024830190612677565b346102e1575f3660031901126102e1576066546040516001600160a01b039091168152602090f35b346102e1575f3660031901126102e157602060ff609754166040519015158152f35b346102e157611081366104e5565b6110896131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692833b156102e157604051630664120160e01b81523060048201526001600160a01b039182166024820152921660448301526001600160e01b03191660648201525f81608481016105aa565b346102e15760403660031901126102e15760043561111c81610429565b6024356001600160401b0381116102e15761113b90369060040161093e565b61116f337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612587565b61118c6040519261117f8461024d565b6001600160a01b03168352565b30602083015260408201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102e157604051636e3492b560e01b8152905f9082908183816111ea886004830161269b565b03925af180156104e0576111fa57005b806112085f6104de93610268565b806107ca565b346102e15760203660031901126102e15760043561122b81610429565b61127b5f549161125f6112496112458560ff9060081c1690565b1590565b809481956112f5575b81156112d5575b50612705565b82611270600160ff195f5416175f55565b6112be575b8061356f565b61128157005b61128f61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b6112d061010061ff00195f5416175f55565b611275565b303b159150816112e7575b505f611259565b60ff1660011490505f6112e0565b600160ff8216109150611252565b346102e1575f3660031901126102e1576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102e1575f3660031901126102e1576067546040516001600160a01b039091168152602090f35b346102e1575f3660031901126102e1576106c26106b6612768565b346102e15760203660031901126102e1576004356113a781610429565b6113af6131f7565b6001600160a01b038116156113c7576104de90613527565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346102e1575f3660031901126102e1576065546040516001600160a01b039091168152602090f35b346102e1575f3660031901126102e1576020606854604051908152f35b346102e15760203660031901126102e1576004356001600160401b0381116102e157366023820112156102e15780600401356001600160401b0381116102e1573660248260051b840101116102e15760246104de9201612995565b634e487b7160e01b5f52603260045260245ffd5b9060028110156114e05760051b0190565b6114bb565b634e487b7160e01b5f52601260045260245ffd5b6115d56115b26115db956115ac6115a585875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e084015261010083015261157c81610120840103601f198101835282610268565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b8096612dc4565b90612e0a565b926115ac6115c76115c1612e6c565b94612f63565b916115d061307f565b612dc4565b916130c9565b9091565b6040513d5f823e3d90fd5b908160209103126102e1575190565b908160209103126102e157516001600160c01b03811681036102e15790565b908160209103126102e1575160ff811681036102e15790565b60405190611640602083610268565b5f808352366020840137565b9061165682610927565b6116636040519182610268565b8281528092611674601f1991610927565b0190602036910137565b9081518110156114e0570160200190565b634e487b7160e01b5f52601160045260245ffd5b90600182018092116116b157565b61168f565b90600282018092116116b157565b90600382018092116116b157565b90600482018092116116b157565b90600582018092116116b157565b919082018092116116b157565b6001600160601b038116036102e157565b908160409103126102e1576020604051916117268361022d565b805161173181610429565b8352015161173e816116fb565b602082015290565b80518210156114e05760209160051b010190565b5f1981146116b15760010190565b6040516309aa152760e11b81526001600160a01b0391821660048201527f000000000000000000000000000000000000000000000000000000000000000090911690602081602481855afa9081156104e0576117e8916020915f91611ae7575b506040518093819263871ef04960e01b8352600483019190602083019252565b0381855afa9081156104e0575f91611ab8575b506001600160c01b0316908115908115611a55575b50611a495761181e9061324f565b5f91907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b81518510156118fa576118a3602061188061187a61186c898761167e565b516001600160f81b03191690565b60f81c90565b604051633ca5a5f560e01b815260ff909116600482015291829081906024820190565b0381875afa80156104e0576001926118c2925f926118ca575b506116ee565b94019361184e565b6118ec91925060203d81116118f3575b6118e48183610268565b8101906115ea565b905f6118bc565b503d6118da565b61190591945061164c565b925f905f5b8151811015611a435761192361187a61186c838561167e565b604051633ca5a5f560e01b815260ff8216600482015290602082602481895afa9182156104e0575f92611a23575b50905f915b8183106119685750505060010161190a565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156104e0576119e78b6119d8836119d26119c66001986119ec985f916119f5575b50516001600160a01b031690565b6001600160a01b031690565b92611746565b6001600160a01b039091169052565b61175a565b95019190611956565b611a16915060403d8111611a1c575b611a0e8183610268565b81019061170c565b5f6119b8565b503d611a04565b611a3c91925060203d81116118f3576118e48183610268565b905f611951565b50505050565b50611a52611631565b90565b604051639aa1653d60e01b81529150602090829060049082905afa80156104e05760ff915f91611a89575b5016155f611810565b611aab915060203d602011611ab1575b611aa38183610268565b810190611618565b5f611a80565b503d611a99565b611ada915060203d602011611ae0575b611ad28183610268565b8101906115f9565b5f6117fb565b503d611ac8565b611afe9150823d84116118f3576118e48183610268565b5f6117c8565b60405190611b118261022d565b60606020838281520152565b15611b2457565b62f8202d60e51b5f5260045ffd5b15611b3957565b6343714afd60e01b5f5260045ffd5b15611b4f57565b635f832f4160e01b5f5260045ffd5b15611b6557565b634b874f4560e01b5f5260045ffd5b5f198101919082116116b157565b15611b8957565b633fdc650560e21b5f5260045ffd5b908160209103126102e15751611a5281610901565b908210156114e0570190565b15611bc057565b63affc5edb60e01b5f5260045ffd5b908160209103126102e1575167ffffffffffffffff19811681036102e15790565b15611bf757565b63e1310aed60e01b5f5260045ffd5b908160209103126102e15751611a52816116fb565b906001600160601b03809116911603906001600160601b0382116116b157565b15611c4257565b6367988d3360e01b5f5260045ffd5b15611c5857565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b818110611c915750505090565b8251845260209384019390920191600101611c84565b949392909193611cb5611b04565b50611cc1851515611b1d565b604084015151851480612579575b8061256b575b8061255d575b611ce490611b32565b611cf660208501515185515114611b48565b611d0d63ffffffff431663ffffffff841610611b5e565b611d1561029b565b5f81525f602082015292611d27611b04565b611d308761164c565b6020820152611d3e8761164c565b8152611d48611b04565b92611d5760208801515161164c565b8452611d6760208801515161164c565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156104e057611dd0915f9161253e575b50611dcb368b87610d68565b613389565b985f965b60208901518051891015611f4c57602088611e41611e378c611e2f8f96868e611e14611e01868095611746565b5180515f526020015160205260405f2090565b611e218484840151611746565b5282611f19575b0151611746565b519551611746565b5163ffffffff1690565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9182156104e0576115ac8a611eee8f611ee78f8460208f92611ede93611ed68460019e611ef49e5f91611efc575b508f8060c01b03169251611746565b520151611746565b51938d51611746565b51166133b4565b906133e5565b970196611dd4565b611f139150863d8111611ae057611ad28183610268565b5f611ec7565b611f47611f298484840151611746565b51611f4084840151611f3a87611b74565b90611746565b5110611b82565b611e28565b50909597949650611f619198939299506134a2565b91611f6e60975460ff1690565b908115612536576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156104e0575f91612507575b5091905b5f925b81841061202d57505050505092612006612001611ffa612027958561201998608060606020990151920151926114f9565b9190611c3b565b611c51565b0151604051928391602083019586611c67565b03601f198101835282610268565b51902090565b92989596909399919794878b888c888d612401575b611e378260a061209061187a612082846120989761207c61206e611e018f9c604060209f9e0151611746565b67ffffffffffffffff191690565b9b611bad565b356001600160f81b03191690565b970151611746565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156104e05761215c611e378f958f906121548f978f96848f61214e60c096612147848f60209f90611e286120829960409361187a9c5f916123d3575b5067ffffffffffffffff19918216911614611bf0565b5190612e0a565b9c611bad565b960151611746565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156104e0576121e9918c8f925f926123af575b5060206121db92930151611746565b906001600160601b03169052565b6122168c6121db8c61220f612202826020860151611746565b516001600160601b031690565b9251611746565b5f985f5b60208a015151811015612396578b8d6122588961224b61187a612082868f896122439151611746565b519487611bad565b60ff161c60019081161490565b612267575b505060010161221a565b8a8a6122ef859f948f96866122a98f9360e06122a0611e3795602061229861187a612082839f6122af9c8991611bad565b9a0151611746565b519b0151611746565b51611746565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156104e0578f612355908f936001959486955f92612360575b506119d26121db929351936123506122028487611746565b611c1b565b019a90508b8d61225d565b6121db92506123886119d29160203d811161238f575b6123808183610268565b810190611c06565b9250612338565b503d612376565b5093919796996001919699509a94929a01929190611fc9565b6121db92506123cc602091823d811161238f576123808183610268565b92506121cc565b60206123f492503d81116123fa575b6123ec8183610268565b810190611bcf565b5f612131565b503d6123e2565b61243e945061241b925061187a9161208291602095611bad565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156104e0576020896120988f938f60a08f9761187a6120828f8f9061207c61206e611e018f60408b96918f8893611e379f6124c2906124c8936120909f5f926124de575b5063ffffffff8091169316906116ee565b11611bb9565b5050505050509750505050505092935050612042565b602063ffffffff92935082916124ff913d81116118f3576118e48183610268565b9291506124b1565b612529915060203d60201161252f575b6125218183610268565b810190611b98565b5f611fc2565b503d612517565b5f9190611fc6565b612557915060203d602011611ab157611aa38183610268565b5f611dbf565b5060e0840151518514611cdb565b5060c0840151518514611cd5565b5060a0840151518514611ccf565b1561258e57565b634394dbdf60e11b5f5260045ffd5b6125d1337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612587565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b156102e1575f928392604051948580948193639926ee7d60e01b835260018060a01b0316600483015260406024830152604061264682516060604486015260a4850190612677565b91602081015160648501520151608483015203925af180156104e0576126695750565b806112085f61029993610268565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b602080825282516001600160a01b039081168284015281840151166040808401919091529092015160608083015280516080830181905260a09092019201905f5b8181106126e95750505090565b825163ffffffff168452602093840193909201916001016126dc565b1561270c57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156104e05760ff915f91612976575b5016801561296c577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b8083106129285750612803915061164c565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156104e05760ff915f9161290a575b501681101561290357604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa9182156104e0575f926128e3575b50905f915b81831061287d57505050600101612808565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156104e0576119e78b6119d8836119d26119c66001986128da985f916119f55750516001600160a01b031690565b9501919061286b565b6128fc91925060203d81116118f3576118e48183610268565b905f612866565b5092505050565b612922915060203d8111611ab157611aa38183610268565b5f61282e565b604051633ca5a5f560e01b815260ff84166004820152909190602081602481885afa80156104e057600192612963925f926118ca57506116ee565b920191906127f1565b5050611a52611631565b61298f915060203d602011611ab157611aa38183610268565b5f6127b9565b6065546001600160a01b03163303612bb9577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b818110612a175750823b156102e157612a06925f928360405180968195829463fce36c7d60e01b845260048401612c71565b03925af180156104e0576126695750565b5f6020612a72612a356119c683612a2f87898b612bc8565b01612bea565b6040612a4286888a612bc8565b6040516323b872dd60e01b8152336004820152306024820152910135604482015293849283919082906064820190565b03925af180156104e057612b9d575b50612a956119c66020612a2f848688612bc8565b604051636eb1769f60e11b81523060048201526001600160a01b03861660248201529190602090839060449082905afa80156104e057612b386020915f948591612b80575b50612b05612af06119c685612a2f888b8d612bc8565b916040612afe878a8c612bc8565b01356116ee565b60405163095ea7b360e01b81526001600160a01b038a166004820152602481019190915294859283919082906044820190565b03925af19182156104e057600192612b52575b50016129d4565b612b729060203d8111612b79575b612b6a8183610268565b810190612bf4565b505f612b4b565b503d612b60565b612b979150833d81116118f3576118e48183610268565b5f612ada565b612bb49060203d8111612b7957612b6a8183610268565b612a81565b638e79fdb560e01b5f5260045ffd5b91908110156114e05760051b81013590609e19813603018212156102e1570190565b35611a5281610429565b908160209103126102e15751611a52816106f3565b916020908281520191905f5b818110612c225750505090565b9091926040806001928635612c3681610429565b848060a01b031681526001600160601b036020880135612c55816116fb565b166020820152019401929101612c15565b359061029982610429565b9180602084016020855252604083019060408160051b85010193835f91609e1982360301905b848410612ca8575050505050505090565b90919293949596603f198282030187528735838112156102e1578401908135601e19833603018112156102e157820191602083359301906001600160401b0384116102e1578360061b360382136102e157612d71836080612d6681612d1c6020989760019a60a08b9a5260a0870191612c09565b95612d3a612d2b898301612c66565b6001600160a01b0316868a0152565b60408101356040860152612d60612d536060830161091c565b63ffffffff166060870152565b0161091c565b63ffffffff16910152565b99019701959401929190612c97565b60405190612d8d8261022d565b5f6020838281520152565b60405190610180612da98184610268565b368337565b60405190612dbd602083610268565b6020368337565b91906040906060612dd3612d80565b9485926020855192612de58585610268565b8436853780518452015160208301528482015260076107cf195a01fa15612e0857565bfe5b602092916080604092612e1b612d80565b95869381865193612e2c8686610268565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa8015612e085715612e5d57565b63d4b68fd760e01b5f5260045ffd5b604051612e788161022d565b6040908151612e878382610268565b8236823781526020825191612e9c8484610268565b8336843701528051612eae8282610268565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6020820152815190612f048383610268565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152612f5983519384610268565b8252602082015290565b5f5160206136875f395f51905f5290612f7a612d80565b505f919006602060c0835b61307a575f935f5160206136875f395f51905f5260038186818180090908604051612fb08582610268565b84368237848185604051612fc48282610268565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f5160206136875f395f51905f5260a082015260056107cf195a01fa8015612e085761302e90613670565b519161307a575f5160206136875f395f51905f528280091461306557505f5160206136875f395f51905f5260015f94089293612f85565b9293505061307161029b565b92835282015290565b6114e5565b613087612d80565b506040516130948161022d565b600181526002602082015290565b906006820291808304600614901517156116b157565b90600c8110156114e05760051b0190565b939290916130d760406102aa565b94855260208501526130e960406102aa565b91825260208201526130f9612d98565b925f5b6002811061312657505050602061018092613115612dae565b93849160086201d4c0fa9151151590565b806131326001926130a2565b61313c82856114cf565b515161314882896130b8565b52602061315583866114cf565b51015161316a613164836116a3565b896130b8565b5261317582866114cf565b515151613184613164836116b6565b5261319a61319283876114cf565b515160200190565b516131a7613164836116c4565b5260206131b483876114cf565b510151516131c4613164836116d2565b526131f06131ea6131e360206131da868a6114cf565b51015160200190565b51926116e0565b886130b8565b52016130fc565b6033546001600160a01b0316330361320b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b61ffff61325b826133b4565b1661326581610d4d565b906132736040519283610268565b808252613282601f1991610d4d565b013660208301375f5f5b82518210806132e2575b156132db576001811b84166132b4575b6132af9061175a565b61328c565b9060016132af9160ff60f81b8460f81b165f1a6132d1828761167e565b53019190506132a6565b5050905090565b506101008110613296565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b90600161339760ff936135f8565b928392161b11156133a55790565b63ca95733360e01b5f5260045ffd5b805f915b6133c0575090565b5f1981018181116116b15761ffff9116911661ffff81146116b15760010190806133b8565b906133ee612d80565b5061ffff811690610200821015613493576001821461348e5761340f61029b565b5f81525f602082015292906001905f925b61ffff831685101561343457505050505090565b600161ffff831660ff86161c81161461346e575b60016134646134598360ff94612e0a565b9460011b61fffe1690565b9401169291613420565b9460016134646134596134838960ff95612e0a565b989350505050613448565b505090565b637fc4ea7d60e11b5f5260045ffd5b6134aa612d80565b5080519081158061351b575b156134d75750506040516134cb604082610268565b5f81525f602082015290565b60205f5160206136875f395f51905f52910151065f5160206136875f395f51905f52035f5160206136875f395f51905f5281116116b15760405191612f598361022d565b506020810151156134b6565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b9060ff5f5460081c1615613589576106ee61029992613527565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b156135e957565b631019106960e31b5f5260045ffd5b906101008251116136615781511561365c57602082015160019060f81c81901b5b83518210156136575760019061364261363861187a61186c868961167e565b60ff600191161b90565b9061364e8183116135e2565b17910190613619565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b1561367757565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220fa8f1a81f652d4ef05c87d3b19c99ab8d78c3bbc87fe01c2cd66386c6ae2b3b764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\xC0\x80`@R4a\x02\xB1W`\xA0\x81a;\x1A\x808\x03\x80\x91a\0 \x82\x85a\x03\xE7V[\x839\x81\x01\x03\x12a\x02\xB1W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x82\x82\x03a\x02\xB1W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xB1W`@\x82\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xB1W``\x83\x01Q\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x84\x03a\x02\xB1W`\x80\x01Q\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x83\x03a\x02\xB1W`@Qch0H5`\xE0\x1B\x81R\x91` \x83`\x04\x81\x8AZ\xFA\x92\x83\x15a\x02\xBDW_\x93a\x03\xC6W[P`\x80R`\xC0R\x83`\xE0Ra\x01\0Ra\x01 R`\xA0R_T`\xFF\x81`\x08\x1C\x16a\x03qW`\xFF\x80\x82\x16\x10a\x037W[Pa\x01@R`@Qch0H5`\xE0\x1B\x81R` \x81`\x04\x81\x85Z\xFA\x91\x82\x15a\x02\xBDW`\x04\x92` \x92_\x91a\x03\nW[Pa\x01`R`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x92\x83\x91\x82\x90Z\xFA\x90\x81\x15a\x02\xBDW_\x91a\x02\xC8W[Pa\x01\x80Ra\x01`Q`@Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x02\xBDW_\x91a\x02wW[Pa\x01\xA0R`@Qa6\xDC\x90\x81a\x04>\x829`\x80Q\x81\x81\x81a\x08\x8E\x01R\x81\x81a\x0F\x18\x01R\x81\x81a\x0F\xCD\x01Ra%\xD3\x01R`\xA0Q\x81a\x11\x99\x01R`\xC0Q\x81a)\xA9\x01R`\xE0Q\x81\x81\x81a\x0E\xE8\x01R\x81\x81a\x11A\x01R\x81\x81a\x17\x87\x01R\x81\x81a%\xA3\x01Ra'w\x01Ra\x01\0Q\x81\x81\x81a\x18#\x01Ra'\xC3\x01Ra\x01 Q\x81\x81\x81a\x04b\x01R\x81\x81a\x05:\x01R\x81\x81a\x05\xEA\x01R\x81\x81a\x0Eb\x01Ra\x10\x8B\x01Ra\x01@Q\x81\x81\x81a\x07/\x01R\x81\x81a\x08\xD2\x01R\x81\x81a\x1D\x84\x01R\x81\x81a\x1Ew\x01Ra$B\x01Ra\x01`Q\x81\x81\x81a\x08J\x01R\x81\x81a!\x95\x01Ra\"\xF3\x01Ra\x01\x80Q\x81\x81\x81a\x07\xE9\x01Ra \xD1\x01Ra\x01\xA0Q\x81\x81\x81a\x13\x18\x01Ra\x1F\x8A\x01R\xF3[\x90P` \x81=` \x11a\x02\xB5W[\x81a\x02\x92` \x93\x83a\x03\xE7V[\x81\x01\x03\x12a\x02\xB1WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xB1W_a\x01wV[_\x80\xFD[=\x91Pa\x02\x85V[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x03\x02W[\x81a\x02\xE3` \x93\x83a\x03\xE7V[\x81\x01\x03\x12a\x02\xB1WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xB1W_a\x01=V[=\x91Pa\x02\xD6V[a\x03*\x91P\x83=\x85\x11a\x030W[a\x03\"\x81\x83a\x03\xE7V[\x81\x01\x90a\x04\x1EV[_a\x01\x16V[P=a\x03\x18V[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xE7V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xE0\x91\x93P` =` \x11a\x030Wa\x03\"\x81\x83a\x03\xE7V[\x91_a\0\xB9V[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x04\nW`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90\x81` \x91\x03\x12a\x02\xB1WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xB1W\x90V\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x17\x1F\x1D[\x14a\x02\x14W\x80c\x17\x85\xF5<\x14a\x02\x0FW\x80c\x1F\xDB\x0C\xFD\x14a\x02\nW\x80c'\x942\xEB\x14a\x02\x05W\x80c3\xCF\xB7\xB7\x14a\x02\0W\x80c;\xC2\x8C\x8C\x14a\x01\xFBW\x80cAl~^\x14a\x01\xF6W\x80c]\xF4YF\x14a\x01\xF1W\x80cg\x94\x0C\x89\x14a\x01\xECW\x80ch0H5\x14a\x01\xE7W\x80ck:\xA7.\x14a\x01\xE2W\x80cm\x14\xA9\x87\x14a\x01\xDDW\x80cn\xFBF6\x14a\x01\xD8W\x80cqP\x18\xA6\x14a\x01\xD3W\x80c\x8Dh4\x9A\x14a\x01\xCEW\x80c\x8D\xA5\xCB[\x14a\x01\xC9W\x80c\x99&\xEE}\x14a\x01\xC4W\x80c\x9D\xA1m\x8E\x14a\x01\xBFW\x80c\xA3d\xF4\xDA\x14a\x01\xBAW\x80c\xA9\x8F\xB3U\x14a\x01\xB5W\x80c\xB14Bq\x14a\x01\xB0W\x80c\xB9\x8D\t\x08\x14a\x01\xABW\x80c\xBAU\x08\x80\x14a\x01\xA6W\x80c\xC1\xA8\xE2\xC5\x14a\x01\xA1W\x80c\xC4\xD6m\xE8\x14a\x01\x9CW\x80c\xDF\\\xF7#\x14a\x01\x97W\x80c\xE4o\x18\x16\x14a\x01\x92W\x80c\xE4\x81\xAF\x9D\x14a\x01\x8DW\x80c\xF2\xFD\xE3\x8B\x14a\x01\x88W\x80c\xFC)\x9D\xEE\x14a\x01\x83W\x80c\xFC\xD1\xC3u\x14a\x01~Wc\xFC\xE3l}\x14a\x01yW_\x80\xFD[a\x14`V[a\x14CV[a\x14\x1BV[a\x13\x8AV[a\x13oV[a\x13GV[a\x13\x03V[a\x12\x0EV[a\x10\xFFV[a\x10sV[a\x10QV[a\x10)V[a\x0F\x81V[a\x0E\xC4V[a\x0E:V[a\r\x9EV[a\r%V[a\r\x03V[a\x0C\xA8V[a\x0C\x13V[a\x08\xBDV[a\x08yV[a\x085V[a\x08\x18V[a\x07\xD4V[a\x06\xFDV[a\x06\xC6V[a\x06\x8EV[a\x05\xC2V[a\x05\"V[a\x04:V[a\x03\xC1V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02HW`@RV[a\x02\x19V[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02HW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02HW`@RV[`@Q\x90a\x02\x99a\x01\0\x83a\x02hV[V[`@Q\x90a\x02\x99`@\x83a\x02hV[\x90a\x02\x99`@Q\x92\x83a\x02hV[`@\x90`\xE3\x19\x01\x12a\x02\xE1W`@Q\x90a\x02\xD1\x82a\x02-V[`\xE45\x82Ra\x01\x045` \x83\x01RV[_\x80\xFD[\x91\x90\x82`@\x91\x03\x12a\x02\xE1W`@Qa\x02\xFD\x81a\x02-V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xE1W`@Q\x91a\x03(`@\x84a\x02hV[\x82\x90`@\x81\x01\x92\x83\x11a\x02\xE1W\x90[\x82\x82\x10a\x03DWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x037V[\x90`\x80`c\x19\x83\x01\x12a\x02\xE1W`@Qa\x03m\x81a\x02-V[` a\x03\x88\x82\x94a\x03\x7F\x81`da\x03\rV[\x84R`\xA4a\x03\rV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x02\xE1W` a\x03\x88`@Q\x92a\x03\xAB\x84a\x02-V[`@\x84\x96a\x03\xB9\x83\x82a\x03\rV[\x86R\x01a\x03\rV[4a\x02\xE1Wa\x01 6`\x03\x19\x01\x12a\x02\xE1W`\x045`@6`#\x19\x01\x12a\x02\xE1Wa\x04\x19`@\x91\x82Qa\x03\xF3\x81a\x02-V[`$5\x81R`D5` \x82\x01Ra\x04\t6a\x03TV[\x90a\x04\x136a\x02\xB8V[\x92a\x14\xF9V[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\xE1WV[4a\x02\xE1W_` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x04X\x81a\x04)V[a\x04`a1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xE1W`@Qc&\x89Y\xE5`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x04\xE0Wa\x04\xD2WP\x80\xF3[a\x04\xDE\x91P_\x90a\x02hV[\0[a\x15\xDFV[``\x90`\x03\x19\x01\x12a\x02\xE1W`\x045a\x04\xFD\x81a\x04)V[\x90`$5a\x05\n\x81a\x04)V[\x90`D5`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x02\xE1W\x90V[4a\x02\xE1Wa\x0506a\x04\xE5V[a\x058a1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x83;\x15a\x02\xE1W`@QcJ\x86\xC07`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`$\x82\x01R\x92\x16`D\x83\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x82\x01R_\x81`\x84\x81\x01[\x93\x81\x83\x81\x81\x97\x03\x92Z\xF1\x80\x15a\x04\xE0Wa\x04\xD2WP\x80\xF3[4a\x02\xE1W_` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x05\xE0\x81a\x04)V[a\x05\xE8a1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xE1W`@Qc\xEBZN\x87`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01a\x04\xC0V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x06oWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06bV[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1Wa\x06\xC2a\x06\xB6`\x045a\x06\xB1\x81a\x04)V[a\x17hV[`@Q\x91\x82\x91\x82a\x06LV[\x03\x90\xF3[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1Wa\x04\xDE`\x045a\x06\xE6\x81a\x04)V[a\x06\xEEa1\xF7V[a2\xEDV[\x80\x15\x15\x03a\x02\xE1WV[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x07\x1A\x81a\x06\xF3V[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x04\xE0W_\x91a\x07\x8FW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x07\x80Wa\x04\xDE\x90a3KV[cpp\xF3\xB1`\xE1\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a\x07\xC2W[\x81a\x07\xAA` \x93\x83a\x02hV[\x81\x01\x03\x12a\x02\xE1WQa\x07\xBC\x81a\x04)V[_a\x07gV[=\x91Pa\x07\x9DV[_\x91\x03\x12a\x02\xE1WV[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W` `@Qb\t:\x80\x81R\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x02\xE1WV[`D5\x90a\x02\x99\x82a\t\x01V[5\x90a\x02\x99\x82a\t\x01V[`\x01`\x01`@\x1B\x03\x81\x11a\x02HW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xE1W\x815a\tU\x81a\t'V[\x92a\tc`@Q\x94\x85a\x02hV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xE1W` \x01\x90[\x82\x82\x10a\t\x8BWPPP\x90V[` \x80\x91\x835a\t\x9A\x81a\t\x01V[\x81R\x01\x91\x01\x90a\t~V[\x81`\x1F\x82\x01\x12\x15a\x02\xE1W\x805a\t\xBB\x81a\t'V[\x92a\t\xC9`@Q\x94\x85a\x02hV[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x02\xE1W` \x01\x91[\x83\x83\x10a\t\xF3WPPPP\x90V[` `@\x91a\n\x02\x84\x86a\x02\xE5V[\x81R\x01\x92\x01\x91a\t\xE5V[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xE1W\x815a\n$\x81a\t'V[\x92a\n2`@Q\x94\x85a\x02hV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x02\xE1W` \x82\x01\x90[\x83\x82\x10a\n^WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W` \x91a\n\x80\x87\x84\x80\x94\x88\x01\x01a\t>V[\x81R\x01\x91\x01\x90a\nOV[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x02\xE1Wa\n\xA1a\x02\x89V[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x81a\n\xBE\x91\x84\x01a\t>V[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x81a\n\xDF\x91\x84\x01a\t\xA5V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x81a\x0B\x03\x91\x84\x01a\t\xA5V[`@\x85\x01Ra\x0B\x15\x81``\x84\x01a\x03\x8DV[``\x85\x01Ra\x0B'\x81`\xE0\x84\x01a\x02\xE5V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x81a\x0BL\x91\x84\x01a\t>V[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x81a\x0Bq\x91\x84\x01a\t>V[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1Wa\x0B\x94\x92\x01a\n\rV[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0B\xB8WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\xABV[\x92\x91\x90a\x0C\x0E` \x91`@\x86R\x82a\x0B\xFA\x82Q`@\x80\x8A\x01R`\x80\x89\x01\x90a\x0B\x9BV[\x91\x01Q\x86\x82\x03`?\x19\x01``\x88\x01Ra\x0B\x9BV[\x93\x01RV[4a\x02\xE1W`\x806`\x03\x19\x01\x12a\x02\xE1W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W6`#\x82\x01\x12\x15a\x02\xE1W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W6`$\x82\x84\x01\x01\x11a\x02\xE1Wa\x0Cja\t\x0FV[\x90`d5\x93`\x01`\x01`@\x1B\x03\x85\x11a\x02\xE1W`$a\x0C\x90a\x0C\x98\x966\x90`\x04\x01a\n\x8BV[\x94\x01\x90a\x1C\xA7V[\x90a\x06\xC2`@Q\x92\x83\x92\x83a\x0B\xD7V[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1Wa\x0C\xC0a1\xF7V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W` `\xFF`iT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x02HW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\rt\x82a\rMV[\x91a\r\x82`@Q\x93\x84a\x02hV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02\xE1W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[4a\x02\xE1W`@6`\x03\x19\x01\x12a\x02\xE1W`\x045a\r\xBB\x81a\x04)V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xE1W```\x03\x19\x836\x03\x01\x12a\x02\xE1W`@Q\x90a\r\xE8\x82a\x02MV[\x82`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x83\x016`#\x82\x01\x12\x15a\x02\xE1Wa\x04\xDE\x93a\x0E\"`D\x926\x90`$`\x04\x82\x015\x91\x01a\rhV[\x84R`$\x81\x015` \x85\x01R\x015`@\x83\x01Ra%\x9DV[4a\x02\xE1W_` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x0EX\x81a\x04)V[a\x0E`a1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xE1W`@QcO\x90l\xF9`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01a\x04\xC0V[4a\x02\xE1W_` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x0E\xE2\x81a\x04)V[a\x0F\x163\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a%\x87V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\xE1W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04\xE0Wa\x04\xD2WP\x80\xF3[4a\x02\xE1W_` 6`\x03\x19\x01\x12a\x02\xE1W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W6`#\x82\x01\x12\x15a\x02\xE1Wa\x0F\xC3\x906\x90`$\x81`\x04\x015\x91\x01a\rhV[a\x0F\xCBa1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xE1W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x04\xC0\x90`$\x83\x01\x90a&wV[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`fT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xE1Wa\x10\x816a\x04\xE5V[a\x10\x89a1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x83;\x15a\x02\xE1W`@Qc\x06d\x12\x01`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`$\x82\x01R\x92\x16`D\x83\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x82\x01R_\x81`\x84\x81\x01a\x05\xAAV[4a\x02\xE1W`@6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x11\x1C\x81a\x04)V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1Wa\x11;\x906\x90`\x04\x01a\t>V[a\x11o3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a%\x87V[a\x11\x8C`@Q\x92a\x11\x7F\x84a\x02MV[`\x01`\x01`\xA0\x1B\x03\x16\x83RV[0` \x83\x01R`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xE1W`@Qcn4\x92\xB5`\xE0\x1B\x81R\x90_\x90\x82\x90\x81\x83\x81a\x11\xEA\x88`\x04\x83\x01a&\x9BV[\x03\x92Z\xF1\x80\x15a\x04\xE0Wa\x11\xFAW\0[\x80a\x12\x08_a\x04\xDE\x93a\x02hV[\x80a\x07\xCAV[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x12+\x81a\x04)V[a\x12{_T\x91a\x12_a\x12Ia\x12E\x85`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x94\x81\x95a\x12\xF5W[\x81\x15a\x12\xD5W[Pa'\x05V[\x82a\x12p`\x01`\xFF\x19_T\x16\x17_UV[a\x12\xBEW[\x80a5oV[a\x12\x81W\0[a\x12\x8Fa\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x12\xD0a\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x12uV[0;\x15\x91P\x81a\x12\xE7W[P_a\x12YV[`\xFF\x16`\x01\x14\x90P_a\x12\xE0V[`\x01`\xFF\x82\x16\x10\x91Pa\x12RV[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1Wa\x06\xC2a\x06\xB6a'hV[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x13\xA7\x81a\x04)V[a\x13\xAFa1\xF7V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x13\xC7Wa\x04\xDE\x90a5'V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W` `hT`@Q\x90\x81R\xF3[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W6`#\x82\x01\x12\x15a\x02\xE1W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W6`$\x82`\x05\x1B\x84\x01\x01\x11a\x02\xE1W`$a\x04\xDE\x92\x01a)\x95V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x14\xE0W`\x05\x1B\x01\x90V[a\x14\xBBV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x15\xD5a\x15\xB2a\x15\xDB\x95a\x15\xACa\x15\xA5\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x15|\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x02hV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96a-\xC4V[\x90a.\nV[\x92a\x15\xACa\x15\xC7a\x15\xC1a.lV[\x94a/cV[\x91a\x15\xD0a0\x7FV[a-\xC4V[\x91a0\xC9V[\x90\x91V[`@Q=_\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x02\xE1WQ\x90V[\x90\x81` \x91\x03\x12a\x02\xE1WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x02\xE1W\x90V[\x90\x81` \x91\x03\x12a\x02\xE1WQ`\xFF\x81\x16\x81\x03a\x02\xE1W\x90V[`@Q\x90a\x16@` \x83a\x02hV[_\x80\x83R6` \x84\x017V[\x90a\x16V\x82a\t'V[a\x16c`@Q\x91\x82a\x02hV[\x82\x81R\x80\x92a\x16t`\x1F\x19\x91a\t'V[\x01\x90` 6\x91\x017V[\x90\x81Q\x81\x10\x15a\x14\xE0W\x01` \x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x16\xB1WV[a\x16\x8FV[\x90`\x02\x82\x01\x80\x92\x11a\x16\xB1WV[\x90`\x03\x82\x01\x80\x92\x11a\x16\xB1WV[\x90`\x04\x82\x01\x80\x92\x11a\x16\xB1WV[\x90`\x05\x82\x01\x80\x92\x11a\x16\xB1WV[\x91\x90\x82\x01\x80\x92\x11a\x16\xB1WV[`\x01`\x01``\x1B\x03\x81\x16\x03a\x02\xE1WV[\x90\x81`@\x91\x03\x12a\x02\xE1W` `@Q\x91a\x17&\x83a\x02-V[\x80Qa\x171\x81a\x04)V[\x83R\x01Qa\x17>\x81a\x16\xFBV[` \x82\x01R\x90V[\x80Q\x82\x10\x15a\x14\xE0W` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x16\xB1W`\x01\x01\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x04\xE0Wa\x17\xE8\x91` \x91_\x91a\x1A\xE7W[P`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x90\x81\x15a\x04\xE0W_\x91a\x1A\xB8W[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81\x15\x90\x81\x15a\x1AUW[Pa\x1AIWa\x18\x1E\x90a2OV[_\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x18\xFAWa\x18\xA3` a\x18\x80a\x18za\x18l\x89\x87a\x16~V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x87Z\xFA\x80\x15a\x04\xE0W`\x01\x92a\x18\xC2\x92_\x92a\x18\xCAW[Pa\x16\xEEV[\x94\x01\x93a\x18NV[a\x18\xEC\x91\x92P` =\x81\x11a\x18\xF3W[a\x18\xE4\x81\x83a\x02hV[\x81\x01\x90a\x15\xEAV[\x90_a\x18\xBCV[P=a\x18\xDAV[a\x19\x05\x91\x94Pa\x16LV[\x92_\x90_[\x81Q\x81\x10\x15a\x1ACWa\x19#a\x18za\x18l\x83\x85a\x16~V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x90` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x04\xE0W_\x92a\x1A#W[P\x90_\x91[\x81\x83\x10a\x19hWPPP`\x01\x01a\x19\nV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x04\xE0Wa\x19\xE7\x8Ba\x19\xD8\x83a\x19\xD2a\x19\xC6`\x01\x98a\x19\xEC\x98_\x91a\x19\xF5W[PQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x17FV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x17ZV[\x95\x01\x91\x90a\x19VV[a\x1A\x16\x91P`@=\x81\x11a\x1A\x1CW[a\x1A\x0E\x81\x83a\x02hV[\x81\x01\x90a\x17\x0CV[_a\x19\xB8V[P=a\x1A\x04V[a\x1A<\x91\x92P` =\x81\x11a\x18\xF3Wa\x18\xE4\x81\x83a\x02hV[\x90_a\x19QV[PPPPV[Pa\x1ARa\x161V[\x90V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x04\xE0W`\xFF\x91_\x91a\x1A\x89W[P\x16\x15_a\x18\x10V[a\x1A\xAB\x91P` =` \x11a\x1A\xB1W[a\x1A\xA3\x81\x83a\x02hV[\x81\x01\x90a\x16\x18V[_a\x1A\x80V[P=a\x1A\x99V[a\x1A\xDA\x91P` =` \x11a\x1A\xE0W[a\x1A\xD2\x81\x83a\x02hV[\x81\x01\x90a\x15\xF9V[_a\x17\xFBV[P=a\x1A\xC8V[a\x1A\xFE\x91P\x82=\x84\x11a\x18\xF3Wa\x18\xE4\x81\x83a\x02hV[_a\x17\xC8V[`@Q\x90a\x1B\x11\x82a\x02-V[``` \x83\x82\x81R\x01RV[\x15a\x1B$WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a\x1B9WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a\x1BOWV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a\x1BeWV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[_\x19\x81\x01\x91\x90\x82\x11a\x16\xB1WV[\x15a\x1B\x89WV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\xE1WQa\x1AR\x81a\t\x01V[\x90\x82\x10\x15a\x14\xE0W\x01\x90V[\x15a\x1B\xC0WV[c\xAF\xFC^\xDB`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\xE1WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x02\xE1W\x90V[\x15a\x1B\xF7WV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\xE1WQa\x1AR\x81a\x16\xFBV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a\x16\xB1WV[\x15a\x1CBWV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a\x1CXWV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a\x1C\x91WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1C\x84V[\x94\x93\x92\x90\x91\x93a\x1C\xB5a\x1B\x04V[Pa\x1C\xC1\x85\x15\x15a\x1B\x1DV[`@\x84\x01QQ\x85\x14\x80a%yW[\x80a%kW[\x80a%]W[a\x1C\xE4\x90a\x1B2V[a\x1C\xF6` \x85\x01QQ\x85QQ\x14a\x1BHV[a\x1D\rc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a\x1B^V[a\x1D\x15a\x02\x9BV[_\x81R_` \x82\x01R\x92a\x1D'a\x1B\x04V[a\x1D0\x87a\x16LV[` \x82\x01Ra\x1D>\x87a\x16LV[\x81Ra\x1DHa\x1B\x04V[\x92a\x1DW` \x88\x01QQa\x16LV[\x84Ra\x1Dg` \x88\x01QQa\x16LV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x04\xE0Wa\x1D\xD0\x91_\x91a%>W[Pa\x1D\xCB6\x8B\x87a\rhV[a3\x89V[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a\x1FLW` \x88a\x1EAa\x1E7\x8Ca\x1E/\x8F\x96\x86\x8Ea\x1E\x14a\x1E\x01\x86\x80\x95a\x17FV[Q\x80Q_R` \x01Q` R`@_ \x90V[a\x1E!\x84\x84\x84\x01Qa\x17FV[R\x82a\x1F\x19W[\x01Qa\x17FV[Q\x95Qa\x17FV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x04\xE0Wa\x15\xAC\x8Aa\x1E\xEE\x8Fa\x1E\xE7\x8F\x84` \x8F\x92a\x1E\xDE\x93a\x1E\xD6\x84`\x01\x9Ea\x1E\xF4\x9E_\x91a\x1E\xFCW[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\x17FV[R\x01Qa\x17FV[Q\x93\x8DQa\x17FV[Q\x16a3\xB4V[\x90a3\xE5V[\x97\x01\x96a\x1D\xD4V[a\x1F\x13\x91P\x86=\x81\x11a\x1A\xE0Wa\x1A\xD2\x81\x83a\x02hV[_a\x1E\xC7V[a\x1FGa\x1F)\x84\x84\x84\x01Qa\x17FV[Qa\x1F@\x84\x84\x01Qa\x1F:\x87a\x1BtV[\x90a\x17FV[Q\x10a\x1B\x82V[a\x1E(V[P\x90\x95\x97\x94\x96Pa\x1Fa\x91\x98\x93\x92\x99Pa4\xA2V[\x91a\x1Fn`\x97T`\xFF\x16\x90V[\x90\x81\x15a%6W`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x04\xE0W_\x91a%\x07W[P\x91\x90[_\x92[\x81\x84\x10a -WPPPPP\x92a \x06a \x01a\x1F\xFAa '\x95\x85a \x19\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x14\xF9V[\x91\x90a\x1C;V[a\x1CQV[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a\x1CgV[\x03`\x1F\x19\x81\x01\x83R\x82a\x02hV[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da$\x01W[a\x1E7\x82`\xA0a \x90a\x18za \x82\x84a \x98\x97a |a na\x1E\x01\x8F\x9C`@` \x9F\x9E\x01Qa\x17FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba\x1B\xADV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[\x97\x01Qa\x17FV[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x04\xE0Wa!\\a\x1E7\x8F\x95\x8F\x90a!T\x8F\x97\x8F\x96\x84\x8Fa!N`\xC0\x96a!G\x84\x8F` \x9F\x90a\x1E(a \x82\x99`@\x93a\x18z\x9C_\x91a#\xD3W[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a\x1B\xF0V[Q\x90a.\nV[\x9Ca\x1B\xADV[\x96\x01Qa\x17FV[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x04\xE0Wa!\xE9\x91\x8C\x8F\x92_\x92a#\xAFW[P` a!\xDB\x92\x93\x01Qa\x17FV[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a\"\x16\x8Ca!\xDB\x8Ca\"\x0Fa\"\x02\x82` \x86\x01Qa\x17FV[Q`\x01`\x01``\x1B\x03\x16\x90V[\x92Qa\x17FV[_\x98_[` \x8A\x01QQ\x81\x10\x15a#\x96W\x8B\x8Da\"X\x89a\"Ka\x18za \x82\x86\x8F\x89a\"C\x91Qa\x17FV[Q\x94\x87a\x1B\xADV[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a\"gW[PP`\x01\x01a\"\x1AV[\x8A\x8Aa\"\xEF\x85\x9F\x94\x8F\x96\x86a\"\xA9\x8F\x93`\xE0a\"\xA0a\x1E7\x95` a\"\x98a\x18za \x82\x83\x9Fa\"\xAF\x9C\x89\x91a\x1B\xADV[\x9A\x01Qa\x17FV[Q\x9B\x01Qa\x17FV[Qa\x17FV[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x04\xE0W\x8Fa#U\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a#`W[Pa\x19\xD2a!\xDB\x92\x93Q\x93a#Pa\"\x02\x84\x87a\x17FV[a\x1C\x1BV[\x01\x9A\x90P\x8B\x8Da\"]V[a!\xDB\x92Pa#\x88a\x19\xD2\x91` =\x81\x11a#\x8FW[a#\x80\x81\x83a\x02hV[\x81\x01\x90a\x1C\x06V[\x92Pa#8V[P=a#vV[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a\x1F\xC9V[a!\xDB\x92Pa#\xCC` \x91\x82=\x81\x11a#\x8FWa#\x80\x81\x83a\x02hV[\x92Pa!\xCCV[` a#\xF4\x92P=\x81\x11a#\xFAW[a#\xEC\x81\x83a\x02hV[\x81\x01\x90a\x1B\xCFV[_a!1V[P=a#\xE2V[a$>\x94Pa$\x1B\x92Pa\x18z\x91a \x82\x91` \x95a\x1B\xADV[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x04\xE0W` \x89a \x98\x8F\x93\x8F`\xA0\x8F\x97a\x18za \x82\x8F\x8F\x90a |a na\x1E\x01\x8F`@\x8B\x96\x91\x8F\x88\x93a\x1E7\x9Fa$\xC2\x90a$\xC8\x93a \x90\x9F_\x92a$\xDEW[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a\x16\xEEV[\x11a\x1B\xB9V[PPPPPP\x97PPPPPP\x92\x93PPa BV[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a$\xFF\x91=\x81\x11a\x18\xF3Wa\x18\xE4\x81\x83a\x02hV[\x92\x91Pa$\xB1V[a%)\x91P` =` \x11a%/W[a%!\x81\x83a\x02hV[\x81\x01\x90a\x1B\x98V[_a\x1F\xC2V[P=a%\x17V[_\x91\x90a\x1F\xC6V[a%W\x91P` =` \x11a\x1A\xB1Wa\x1A\xA3\x81\x83a\x02hV[_a\x1D\xBFV[P`\xE0\x84\x01QQ\x85\x14a\x1C\xDBV[P`\xC0\x84\x01QQ\x85\x14a\x1C\xD5V[P`\xA0\x84\x01QQ\x85\x14a\x1C\xCFV[\x15a%\x8EWV[cC\x94\xDB\xDF`\xE1\x1B_R`\x04_\xFD[a%\xD13\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a%\x87V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x02\xE1W_\x92\x83\x92`@Q\x94\x85\x80\x94\x81\x93c\x99&\xEE}`\xE0\x1B\x83R`\x01\x80`\xA0\x1B\x03\x16`\x04\x83\x01R`@`$\x83\x01R`@a&F\x82Q```D\x86\x01R`\xA4\x85\x01\x90a&wV[\x91` \x81\x01Q`d\x85\x01R\x01Q`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\x04\xE0Wa&iWPV[\x80a\x12\x08_a\x02\x99\x93a\x02hV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x84\x01R\x81\x84\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x90\x92\x01Q``\x80\x83\x01R\x80Q`\x80\x83\x01\x81\x90R`\xA0\x90\x92\x01\x92\x01\x90_[\x81\x81\x10a&\xE9WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a&\xDCV[\x15a'\x0CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x04\xE0W`\xFF\x91_\x91a)vW[P\x16\x80\x15a)lW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a)(WPa(\x03\x91Pa\x16LV[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x04\xE0W`\xFF\x91_\x91a)\nW[P\x16\x81\x10\x15a)\x03W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x04\xE0W_\x92a(\xE3W[P\x90_\x91[\x81\x83\x10a(}WPPP`\x01\x01a(\x08V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x04\xE0Wa\x19\xE7\x8Ba\x19\xD8\x83a\x19\xD2a\x19\xC6`\x01\x98a(\xDA\x98_\x91a\x19\xF5WPQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01\x91\x90a(kV[a(\xFC\x91\x92P` =\x81\x11a\x18\xF3Wa\x18\xE4\x81\x83a\x02hV[\x90_a(fV[P\x92PPPV[a)\"\x91P` =\x81\x11a\x1A\xB1Wa\x1A\xA3\x81\x83a\x02hV[_a(.V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x04\xE0W`\x01\x92a)c\x92_\x92a\x18\xCAWPa\x16\xEEV[\x92\x01\x91\x90a'\xF1V[PPa\x1ARa\x161V[a)\x8F\x91P` =` \x11a\x1A\xB1Wa\x1A\xA3\x81\x83a\x02hV[_a'\xB9V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x03a+\xB9W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a*\x17WP\x82;\x15a\x02\xE1Wa*\x06\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xFC\xE3l}`\xE0\x1B\x84R`\x04\x84\x01a,qV[\x03\x92Z\xF1\x80\x15a\x04\xE0Wa&iWPV[_` a*ra*5a\x19\xC6\x83a*/\x87\x89\x8Ba+\xC8V[\x01a+\xEAV[`@a*B\x86\x88\x8Aa+\xC8V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x91\x015`D\x82\x01R\x93\x84\x92\x83\x91\x90\x82\x90`d\x82\x01\x90V[\x03\x92Z\xF1\x80\x15a\x04\xE0Wa+\x9DW[Pa*\x95a\x19\xC6` a*/\x84\x86\x88a+\xC8V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x16`$\x82\x01R\x91\x90` \x90\x83\x90`D\x90\x82\x90Z\xFA\x80\x15a\x04\xE0Wa+8` \x91_\x94\x85\x91a+\x80W[Pa+\x05a*\xF0a\x19\xC6\x85a*/\x88\x8B\x8Da+\xC8V[\x91`@a*\xFE\x87\x8A\x8Ca+\xC8V[\x015a\x16\xEEV[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x94\x85\x92\x83\x91\x90\x82\x90`D\x82\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\x04\xE0W`\x01\x92a+RW[P\x01a)\xD4V[a+r\x90` =\x81\x11a+yW[a+j\x81\x83a\x02hV[\x81\x01\x90a+\xF4V[P_a+KV[P=a+`V[a+\x97\x91P\x83=\x81\x11a\x18\xF3Wa\x18\xE4\x81\x83a\x02hV[_a*\xDAV[a+\xB4\x90` =\x81\x11a+yWa+j\x81\x83a\x02hV[a*\x81V[c\x8Ey\xFD\xB5`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a\x14\xE0W`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x02\xE1W\x01\x90V[5a\x1AR\x81a\x04)V[\x90\x81` \x91\x03\x12a\x02\xE1WQa\x1AR\x81a\x06\xF3V[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a,\"WPPP\x90V[\x90\x91\x92`@\x80`\x01\x92\x865a,6\x81a\x04)V[\x84\x80`\xA0\x1B\x03\x16\x81R`\x01`\x01``\x1B\x03` \x88\x015a,U\x81a\x16\xFBV[\x16` \x82\x01R\x01\x94\x01\x92\x91\x01a,\x15V[5\x90a\x02\x99\x82a\x04)V[\x91\x80` \x84\x01` \x85RR`@\x83\x01\x90`@\x81`\x05\x1B\x85\x01\x01\x93\x83_\x91`\x9E\x19\x826\x03\x01\x90[\x84\x84\x10a,\xA8WPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`?\x19\x82\x82\x03\x01\x87R\x875\x83\x81\x12\x15a\x02\xE1W\x84\x01\x90\x815`\x1E\x19\x836\x03\x01\x81\x12\x15a\x02\xE1W\x82\x01\x91` \x835\x93\x01\x90`\x01`\x01`@\x1B\x03\x84\x11a\x02\xE1W\x83`\x06\x1B6\x03\x82\x13a\x02\xE1Wa-q\x83`\x80a-f\x81a-\x1C` \x98\x97`\x01\x9A`\xA0\x8B\x9AR`\xA0\x87\x01\x91a,\tV[\x95a-:a-+\x89\x83\x01a,fV[`\x01`\x01`\xA0\x1B\x03\x16\x86\x8A\x01RV[`@\x81\x015`@\x86\x01Ra-`a-S``\x83\x01a\t\x1CV[c\xFF\xFF\xFF\xFF\x16``\x87\x01RV[\x01a\t\x1CV[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x99\x01\x97\x01\x95\x94\x01\x92\x91\x90a,\x97V[`@Q\x90a-\x8D\x82a\x02-V[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80a-\xA9\x81\x84a\x02hV[6\x837V[`@Q\x90a-\xBD` \x83a\x02hV[` 6\x837V[\x91\x90`@\x90``a-\xD3a-\x80V[\x94\x85\x92` \x85Q\x92a-\xE5\x85\x85a\x02hV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15a.\x08WV[\xFE[` \x92\x91`\x80`@\x92a.\x1Ba-\x80V[\x95\x86\x93\x81\x86Q\x93a.,\x86\x86a\x02hV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15a.\x08W\x15a.]WV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@Qa.x\x81a\x02-V[`@\x90\x81Qa.\x87\x83\x82a\x02hV[\x826\x827\x81R` \x82Q\x91a.\x9C\x84\x84a\x02hV[\x836\x847\x01R\x80Qa.\xAE\x82\x82a\x02hV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90a/\x04\x83\x83a\x02hV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01Ra/Y\x83Q\x93\x84a\x02hV[\x82R` \x82\x01R\x90V[_Q` a6\x87_9_Q\x90_R\x90a/za-\x80V[P_\x91\x90\x06` `\xC0\x83[a0zW_\x93_Q` a6\x87_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa/\xB0\x85\x82a\x02hV[\x846\x827\x84\x81\x85`@Qa/\xC4\x82\x82a\x02hV[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` a6\x87_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a.\x08Wa0.\x90a6pV[Q\x91a0zW_Q` a6\x87_9_Q\x90_R\x82\x80\t\x14a0eWP_Q` a6\x87_9_Q\x90_R`\x01_\x94\x08\x92\x93a/\x85V[\x92\x93PPa0qa\x02\x9BV[\x92\x83R\x82\x01R\x90V[a\x14\xE5V[a0\x87a-\x80V[P`@Qa0\x94\x81a\x02-V[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a\x16\xB1WV[\x90`\x0C\x81\x10\x15a\x14\xE0W`\x05\x1B\x01\x90V[\x93\x92\x90\x91a0\xD7`@a\x02\xAAV[\x94\x85R` \x85\x01Ra0\xE9`@a\x02\xAAV[\x91\x82R` \x82\x01Ra0\xF9a-\x98V[\x92_[`\x02\x81\x10a1&WPPP` a\x01\x80\x92a1\x15a-\xAEV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80a12`\x01\x92a0\xA2V[a1<\x82\x85a\x14\xCFV[QQa1H\x82\x89a0\xB8V[R` a1U\x83\x86a\x14\xCFV[Q\x01Qa1ja1d\x83a\x16\xA3V[\x89a0\xB8V[Ra1u\x82\x86a\x14\xCFV[QQQa1\x84a1d\x83a\x16\xB6V[Ra1\x9Aa1\x92\x83\x87a\x14\xCFV[QQ` \x01\x90V[Qa1\xA7a1d\x83a\x16\xC4V[R` a1\xB4\x83\x87a\x14\xCFV[Q\x01QQa1\xC4a1d\x83a\x16\xD2V[Ra1\xF0a1\xEAa1\xE3` a1\xDA\x86\x8Aa\x14\xCFV[Q\x01Q` \x01\x90V[Q\x92a\x16\xE0V[\x88a0\xB8V[R\x01a0\xFCV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a2\x0BWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[a\xFF\xFFa2[\x82a3\xB4V[\x16a2e\x81a\rMV[\x90a2s`@Q\x92\x83a\x02hV[\x80\x82Ra2\x82`\x1F\x19\x91a\rMV[\x016` \x83\x017__[\x82Q\x82\x10\x80a2\xE2W[\x15a2\xDBW`\x01\x81\x1B\x84\x16a2\xB4W[a2\xAF\x90a\x17ZV[a2\x8CV[\x90`\x01a2\xAF\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa2\xD1\x82\x87a\x16~V[S\x01\x91\x90Pa2\xA6V[PP\x90P\x90V[Pa\x01\0\x81\x10a2\x96V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eUV[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01a3\x97`\xFF\x93a5\xF8V[\x92\x83\x92\x16\x1B\x11\x15a3\xA5W\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[a3\xC0WP\x90V[_\x19\x81\x01\x81\x81\x11a\x16\xB1Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x16\xB1W`\x01\x01\x90\x80a3\xB8V[\x90a3\xEEa-\x80V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15a4\x93W`\x01\x82\x14a4\x8EWa4\x0Fa\x02\x9BV[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15a44WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14a4nW[`\x01a4da4Y\x83`\xFF\x94a.\nV[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91a4 V[\x94`\x01a4da4Ya4\x83\x89`\xFF\x95a.\nV[\x98\x93PPPPa4HV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[a4\xAAa-\x80V[P\x80Q\x90\x81\x15\x80a5\x1BW[\x15a4\xD7WPP`@Qa4\xCB`@\x82a\x02hV[_\x81R_` \x82\x01R\x90V[` _Q` a6\x87_9_Q\x90_R\x91\x01Q\x06_Q` a6\x87_9_Q\x90_R\x03_Q` a6\x87_9_Q\x90_R\x81\x11a\x16\xB1W`@Q\x91a/Y\x83a\x02-V[P` \x81\x01Q\x15a4\xB6V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x90`\xFF_T`\x08\x1C\x16\x15a5\x89Wa\x06\xEEa\x02\x99\x92a5'V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[\x15a5\xE9WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11a6aW\x81Q\x15a6\\W` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15a6WW`\x01\x90a6Ba68a\x18za\x18l\x86\x89a\x16~V[`\xFF`\x01\x91\x16\x1B\x90V[\x90a6N\x81\x83\x11a5\xE2V[\x17\x91\x01\x90a6\x19V[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15a6wWV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xFA\x8F\x1A\x81\xF6R\xD4\xEF\x05\xC8};\x19\xC9\x9A\xB8\xD7\x8C;\xBC\x87\xFE\x01\xC2\xCDf8lj\xE2\xB3\xB7dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8063171f1d5b146102145780631785f53c1461020f5780631fdb0cfd1461020a578063279432eb1461020557806333cfb7b7146102005780633bc28c8c146101fb578063416c7e5e146101f65780635df45946146101f157806367940c89146101ec57806368304835146101e75780636b3aa72e146101e25780636d14a987146101dd5780636efb4636146101d8578063715018a6146101d35780638d68349a146101ce5780638da5cb5b146101c95780639926ee7d146101c45780639da16d8e146101bf578063a364f4da146101ba578063a98fb355146101b5578063b1344271146101b0578063b98d0908146101ab578063ba550880146101a6578063c1a8e2c5146101a1578063c4d66de81461019c578063df5cf72314610197578063e46f181614610192578063e481af9d1461018d578063f2fde38b14610188578063fc299dee14610183578063fcd1c3751461017e5763fce36c7d14610179575f80fd5b611460565b611443565b61141b565b61138a565b61136f565b611347565b611303565b61120e565b6110ff565b611073565b611051565b611029565b610f81565b610ec4565b610e3a565b610d9e565b610d25565b610d03565b610ca8565b610c13565b6108bd565b610879565b610835565b610818565b6107d4565b6106fd565b6106c6565b61068e565b6105c2565b610522565b61043a565b6103c1565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761024857604052565b610219565b606081019081106001600160401b0382111761024857604052565b90601f801991011681019081106001600160401b0382111761024857604052565b6040519061029961010083610268565b565b60405190610299604083610268565b906102996040519283610268565b60409060e31901126102e157604051906102d18261022d565b60e4358252610104356020830152565b5f80fd5b91908260409103126102e1576040516102fd8161022d565b6020808294803584520135910152565b9080601f830112156102e15760405191610328604084610268565b8290604081019283116102e157905b8282106103445750505090565b8135815260209182019101610337565b9060806063198301126102e15760405161036d8161022d565b6020610388829461037f81606461030d565b845260a461030d565b910152565b91906080838203126102e1576020610388604051926103ab8461022d565b604084966103b9838261030d565b86520161030d565b346102e1576101203660031901126102e15760043560403660231901126102e15761041960409182516103f38161022d565b6024358152604435602082015261040936610354565b90610413366102b8565b926114f9565b8251911515825215156020820152f35b6001600160a01b038116036102e157565b346102e1575f60203660031901126102e15760043561045881610429565b6104606131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102e15760405163268959e560e01b81523060048201526001600160a01b0390921660248301525f908290818381604481015b03925af180156104e0576104d2575080f35b6104de91505f90610268565b005b6115df565b60609060031901126102e1576004356104fd81610429565b9060243561050a81610429565b906044356001600160e01b0319811681036102e15790565b346102e157610530366104e5565b6105386131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692833b156102e157604051634a86c03760e11b81523060048201526001600160a01b039182166024820152921660448301526001600160e01b03191660648201525f81608481015b93818381819703925af180156104e0576104d2575080f35b346102e1575f60203660031901126102e1576004356105e081610429565b6105e86131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102e15760405163eb5a4e8760e01b81523060048201526001600160a01b0390921660248301525f908290818381604481016104c0565b60206040818301928281528451809452019201905f5b81811061066f5750505090565b82516001600160a01b0316845260209384019390920191600101610662565b346102e15760203660031901126102e1576106c26106b66004356106b181610429565b611768565b6040519182918261064c565b0390f35b346102e15760203660031901126102e1576104de6004356106e681610429565b6106ee6131f7565b6132ed565b801515036102e157565b346102e15760203660031901126102e15760043561071a816106f3565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156104e0575f9161078f575b506001600160a01b03163303610780576104de9061334b565b637070f3b160e11b5f5260045ffd5b90506020813d6020116107c2575b816107aa60209383610268565b810103126102e157516107bc81610429565b5f610767565b3d915061079d565b5f9103126102e157565b346102e1575f3660031901126102e1576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102e1575f3660031901126102e157602060405162093a808152f35b346102e1575f3660031901126102e1576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102e1575f3660031901126102e1576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102e1575f3660031901126102e1576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b63ffffffff8116036102e157565b6044359061029982610901565b359061029982610901565b6001600160401b0381116102485760051b60200190565b9080601f830112156102e157813561095581610927565b926109636040519485610268565b81845260208085019260051b8201019283116102e157602001905b82821061098b5750505090565b60208091833561099a81610901565b81520191019061097e565b81601f820112156102e15780356109bb81610927565b926109c96040519485610268565b81845260208085019260061b840101928184116102e157602001915b8383106109f3575050505090565b6020604091610a0284866102e5565b8152019201916109e5565b9080601f830112156102e1578135610a2481610927565b92610a326040519485610268565b81845260208085019260051b820101918383116102e15760208201905b838210610a5e57505050505090565b81356001600160401b0381116102e157602091610a808784809488010161093e565b815201910190610a4f565b919091610180818403126102e157610aa1610289565b9281356001600160401b0381116102e15781610abe91840161093e565b845260208201356001600160401b0381116102e15781610adf9184016109a5565b602085015260408201356001600160401b0381116102e15781610b039184016109a5565b6040850152610b15816060840161038d565b6060850152610b278160e084016102e5565b60808501526101208201356001600160401b0381116102e15781610b4c91840161093e565b60a08501526101408201356001600160401b0381116102e15781610b7191840161093e565b60c08501526101608201356001600160401b0381116102e157610b949201610a0d565b60e0830152565b90602080835192838152019201905f5b818110610bb85750505090565b82516001600160601b0316845260209384019390920191600101610bab565b929190610c0e6020916040865282610bfa82516040808a01526080890190610b9b565b910151868203603f19016060880152610b9b565b930152565b346102e15760803660031901126102e1576004356024356001600160401b0381116102e157366023820112156102e15780600401356001600160401b0381116102e15736602482840101116102e157610c6a61090f565b90606435936001600160401b0385116102e1576024610c90610c98963690600401610a8b565b940190611ca7565b906106c260405192839283610bd7565b346102e1575f3660031901126102e157610cc06131f7565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346102e1575f3660031901126102e157602060ff606954166040519015158152f35b346102e1575f3660031901126102e1576033546040516001600160a01b039091168152602090f35b6001600160401b03811161024857601f01601f191660200190565b929192610d7482610d4d565b91610d826040519384610268565b8294818452818301116102e1578281602093845f960137010152565b346102e15760403660031901126102e157600435610dbb81610429565b602435906001600160401b0382116102e157606060031983360301126102e15760405190610de88261024d565b82600401356001600160401b0381116102e1578301366023820112156102e1576104de93610e226044923690602460048201359101610d68565b8452602481013560208501520135604083015261259d565b346102e1575f60203660031901126102e157600435610e5881610429565b610e606131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102e157604051634f906cf960e01b81523060048201526001600160a01b0390921660248301525f908290818381604481016104c0565b346102e1575f60203660031901126102e157600435610ee281610429565b610f16337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612587565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156102e1576040516351b27a6d60e11b81526001600160a01b039091166004820152905f908290602490829084905af180156104e0576104d2575080f35b346102e1575f60203660031901126102e1576004356001600160401b0381116102e157366023820112156102e157610fc3903690602481600401359101610d68565b610fcb6131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102e15760405163a98fb35560e01b815260206004820152915f9183918290849082906104c0906024830190612677565b346102e1575f3660031901126102e1576066546040516001600160a01b039091168152602090f35b346102e1575f3660031901126102e157602060ff609754166040519015158152f35b346102e157611081366104e5565b6110896131f7565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692833b156102e157604051630664120160e01b81523060048201526001600160a01b039182166024820152921660448301526001600160e01b03191660648201525f81608481016105aa565b346102e15760403660031901126102e15760043561111c81610429565b6024356001600160401b0381116102e15761113b90369060040161093e565b61116f337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612587565b61118c6040519261117f8461024d565b6001600160a01b03168352565b30602083015260408201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102e157604051636e3492b560e01b8152905f9082908183816111ea886004830161269b565b03925af180156104e0576111fa57005b806112085f6104de93610268565b806107ca565b346102e15760203660031901126102e15760043561122b81610429565b61127b5f549161125f6112496112458560ff9060081c1690565b1590565b809481956112f5575b81156112d5575b50612705565b82611270600160ff195f5416175f55565b6112be575b8061356f565b61128157005b61128f61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b6112d061010061ff00195f5416175f55565b611275565b303b159150816112e7575b505f611259565b60ff1660011490505f6112e0565b600160ff8216109150611252565b346102e1575f3660031901126102e1576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102e1575f3660031901126102e1576067546040516001600160a01b039091168152602090f35b346102e1575f3660031901126102e1576106c26106b6612768565b346102e15760203660031901126102e1576004356113a781610429565b6113af6131f7565b6001600160a01b038116156113c7576104de90613527565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346102e1575f3660031901126102e1576065546040516001600160a01b039091168152602090f35b346102e1575f3660031901126102e1576020606854604051908152f35b346102e15760203660031901126102e1576004356001600160401b0381116102e157366023820112156102e15780600401356001600160401b0381116102e1573660248260051b840101116102e15760246104de9201612995565b634e487b7160e01b5f52603260045260245ffd5b9060028110156114e05760051b0190565b6114bb565b634e487b7160e01b5f52601260045260245ffd5b6115d56115b26115db956115ac6115a585875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e084015261010083015261157c81610120840103601f198101835282610268565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b8096612dc4565b90612e0a565b926115ac6115c76115c1612e6c565b94612f63565b916115d061307f565b612dc4565b916130c9565b9091565b6040513d5f823e3d90fd5b908160209103126102e1575190565b908160209103126102e157516001600160c01b03811681036102e15790565b908160209103126102e1575160ff811681036102e15790565b60405190611640602083610268565b5f808352366020840137565b9061165682610927565b6116636040519182610268565b8281528092611674601f1991610927565b0190602036910137565b9081518110156114e0570160200190565b634e487b7160e01b5f52601160045260245ffd5b90600182018092116116b157565b61168f565b90600282018092116116b157565b90600382018092116116b157565b90600482018092116116b157565b90600582018092116116b157565b919082018092116116b157565b6001600160601b038116036102e157565b908160409103126102e1576020604051916117268361022d565b805161173181610429565b8352015161173e816116fb565b602082015290565b80518210156114e05760209160051b010190565b5f1981146116b15760010190565b6040516309aa152760e11b81526001600160a01b0391821660048201527f000000000000000000000000000000000000000000000000000000000000000090911690602081602481855afa9081156104e0576117e8916020915f91611ae7575b506040518093819263871ef04960e01b8352600483019190602083019252565b0381855afa9081156104e0575f91611ab8575b506001600160c01b0316908115908115611a55575b50611a495761181e9061324f565b5f91907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b81518510156118fa576118a3602061188061187a61186c898761167e565b516001600160f81b03191690565b60f81c90565b604051633ca5a5f560e01b815260ff909116600482015291829081906024820190565b0381875afa80156104e0576001926118c2925f926118ca575b506116ee565b94019361184e565b6118ec91925060203d81116118f3575b6118e48183610268565b8101906115ea565b905f6118bc565b503d6118da565b61190591945061164c565b925f905f5b8151811015611a435761192361187a61186c838561167e565b604051633ca5a5f560e01b815260ff8216600482015290602082602481895afa9182156104e0575f92611a23575b50905f915b8183106119685750505060010161190a565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156104e0576119e78b6119d8836119d26119c66001986119ec985f916119f5575b50516001600160a01b031690565b6001600160a01b031690565b92611746565b6001600160a01b039091169052565b61175a565b95019190611956565b611a16915060403d8111611a1c575b611a0e8183610268565b81019061170c565b5f6119b8565b503d611a04565b611a3c91925060203d81116118f3576118e48183610268565b905f611951565b50505050565b50611a52611631565b90565b604051639aa1653d60e01b81529150602090829060049082905afa80156104e05760ff915f91611a89575b5016155f611810565b611aab915060203d602011611ab1575b611aa38183610268565b810190611618565b5f611a80565b503d611a99565b611ada915060203d602011611ae0575b611ad28183610268565b8101906115f9565b5f6117fb565b503d611ac8565b611afe9150823d84116118f3576118e48183610268565b5f6117c8565b60405190611b118261022d565b60606020838281520152565b15611b2457565b62f8202d60e51b5f5260045ffd5b15611b3957565b6343714afd60e01b5f5260045ffd5b15611b4f57565b635f832f4160e01b5f5260045ffd5b15611b6557565b634b874f4560e01b5f5260045ffd5b5f198101919082116116b157565b15611b8957565b633fdc650560e21b5f5260045ffd5b908160209103126102e15751611a5281610901565b908210156114e0570190565b15611bc057565b63affc5edb60e01b5f5260045ffd5b908160209103126102e1575167ffffffffffffffff19811681036102e15790565b15611bf757565b63e1310aed60e01b5f5260045ffd5b908160209103126102e15751611a52816116fb565b906001600160601b03809116911603906001600160601b0382116116b157565b15611c4257565b6367988d3360e01b5f5260045ffd5b15611c5857565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b818110611c915750505090565b8251845260209384019390920191600101611c84565b949392909193611cb5611b04565b50611cc1851515611b1d565b604084015151851480612579575b8061256b575b8061255d575b611ce490611b32565b611cf660208501515185515114611b48565b611d0d63ffffffff431663ffffffff841610611b5e565b611d1561029b565b5f81525f602082015292611d27611b04565b611d308761164c565b6020820152611d3e8761164c565b8152611d48611b04565b92611d5760208801515161164c565b8452611d6760208801515161164c565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156104e057611dd0915f9161253e575b50611dcb368b87610d68565b613389565b985f965b60208901518051891015611f4c57602088611e41611e378c611e2f8f96868e611e14611e01868095611746565b5180515f526020015160205260405f2090565b611e218484840151611746565b5282611f19575b0151611746565b519551611746565b5163ffffffff1690565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9182156104e0576115ac8a611eee8f611ee78f8460208f92611ede93611ed68460019e611ef49e5f91611efc575b508f8060c01b03169251611746565b520151611746565b51938d51611746565b51166133b4565b906133e5565b970196611dd4565b611f139150863d8111611ae057611ad28183610268565b5f611ec7565b611f47611f298484840151611746565b51611f4084840151611f3a87611b74565b90611746565b5110611b82565b611e28565b50909597949650611f619198939299506134a2565b91611f6e60975460ff1690565b908115612536576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156104e0575f91612507575b5091905b5f925b81841061202d57505050505092612006612001611ffa612027958561201998608060606020990151920151926114f9565b9190611c3b565b611c51565b0151604051928391602083019586611c67565b03601f198101835282610268565b51902090565b92989596909399919794878b888c888d612401575b611e378260a061209061187a612082846120989761207c61206e611e018f9c604060209f9e0151611746565b67ffffffffffffffff191690565b9b611bad565b356001600160f81b03191690565b970151611746565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156104e05761215c611e378f958f906121548f978f96848f61214e60c096612147848f60209f90611e286120829960409361187a9c5f916123d3575b5067ffffffffffffffff19918216911614611bf0565b5190612e0a565b9c611bad565b960151611746565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9081156104e0576121e9918c8f925f926123af575b5060206121db92930151611746565b906001600160601b03169052565b6122168c6121db8c61220f612202826020860151611746565b516001600160601b031690565b9251611746565b5f985f5b60208a015151811015612396578b8d6122588961224b61187a612082868f896122439151611746565b519487611bad565b60ff161c60019081161490565b612267575b505060010161221a565b8a8a6122ef859f948f96866122a98f9360e06122a0611e3795602061229861187a612082839f6122af9c8991611bad565b9a0151611746565b519b0151611746565b51611746565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156104e0578f612355908f936001959486955f92612360575b506119d26121db929351936123506122028487611746565b611c1b565b019a90508b8d61225d565b6121db92506123886119d29160203d811161238f575b6123808183610268565b810190611c06565b9250612338565b503d612376565b5093919796996001919699509a94929a01929190611fc9565b6121db92506123cc602091823d811161238f576123808183610268565b92506121cc565b60206123f492503d81116123fa575b6123ec8183610268565b810190611bcf565b5f612131565b503d6123e2565b61243e945061241b925061187a9161208291602095611bad565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156104e0576020896120988f938f60a08f9761187a6120828f8f9061207c61206e611e018f60408b96918f8893611e379f6124c2906124c8936120909f5f926124de575b5063ffffffff8091169316906116ee565b11611bb9565b5050505050509750505050505092935050612042565b602063ffffffff92935082916124ff913d81116118f3576118e48183610268565b9291506124b1565b612529915060203d60201161252f575b6125218183610268565b810190611b98565b5f611fc2565b503d612517565b5f9190611fc6565b612557915060203d602011611ab157611aa38183610268565b5f611dbf565b5060e0840151518514611cdb565b5060c0840151518514611cd5565b5060a0840151518514611ccf565b1561258e57565b634394dbdf60e11b5f5260045ffd5b6125d1337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612587565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b156102e1575f928392604051948580948193639926ee7d60e01b835260018060a01b0316600483015260406024830152604061264682516060604486015260a4850190612677565b91602081015160648501520151608483015203925af180156104e0576126695750565b806112085f61029993610268565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b602080825282516001600160a01b039081168284015281840151166040808401919091529092015160608083015280516080830181905260a09092019201905f5b8181106126e95750505090565b825163ffffffff168452602093840193909201916001016126dc565b1561270c57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156104e05760ff915f91612976575b5016801561296c577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b8083106129285750612803915061164c565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156104e05760ff915f9161290a575b501681101561290357604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa9182156104e0575f926128e3575b50905f915b81831061287d57505050600101612808565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156104e0576119e78b6119d8836119d26119c66001986128da985f916119f55750516001600160a01b031690565b9501919061286b565b6128fc91925060203d81116118f3576118e48183610268565b905f612866565b5092505050565b612922915060203d8111611ab157611aa38183610268565b5f61282e565b604051633ca5a5f560e01b815260ff84166004820152909190602081602481885afa80156104e057600192612963925f926118ca57506116ee565b920191906127f1565b5050611a52611631565b61298f915060203d602011611ab157611aa38183610268565b5f6127b9565b6065546001600160a01b03163303612bb9577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b818110612a175750823b156102e157612a06925f928360405180968195829463fce36c7d60e01b845260048401612c71565b03925af180156104e0576126695750565b5f6020612a72612a356119c683612a2f87898b612bc8565b01612bea565b6040612a4286888a612bc8565b6040516323b872dd60e01b8152336004820152306024820152910135604482015293849283919082906064820190565b03925af180156104e057612b9d575b50612a956119c66020612a2f848688612bc8565b604051636eb1769f60e11b81523060048201526001600160a01b03861660248201529190602090839060449082905afa80156104e057612b386020915f948591612b80575b50612b05612af06119c685612a2f888b8d612bc8565b916040612afe878a8c612bc8565b01356116ee565b60405163095ea7b360e01b81526001600160a01b038a166004820152602481019190915294859283919082906044820190565b03925af19182156104e057600192612b52575b50016129d4565b612b729060203d8111612b79575b612b6a8183610268565b810190612bf4565b505f612b4b565b503d612b60565b612b979150833d81116118f3576118e48183610268565b5f612ada565b612bb49060203d8111612b7957612b6a8183610268565b612a81565b638e79fdb560e01b5f5260045ffd5b91908110156114e05760051b81013590609e19813603018212156102e1570190565b35611a5281610429565b908160209103126102e15751611a52816106f3565b916020908281520191905f5b818110612c225750505090565b9091926040806001928635612c3681610429565b848060a01b031681526001600160601b036020880135612c55816116fb565b166020820152019401929101612c15565b359061029982610429565b9180602084016020855252604083019060408160051b85010193835f91609e1982360301905b848410612ca8575050505050505090565b90919293949596603f198282030187528735838112156102e1578401908135601e19833603018112156102e157820191602083359301906001600160401b0384116102e1578360061b360382136102e157612d71836080612d6681612d1c6020989760019a60a08b9a5260a0870191612c09565b95612d3a612d2b898301612c66565b6001600160a01b0316868a0152565b60408101356040860152612d60612d536060830161091c565b63ffffffff166060870152565b0161091c565b63ffffffff16910152565b99019701959401929190612c97565b60405190612d8d8261022d565b5f6020838281520152565b60405190610180612da98184610268565b368337565b60405190612dbd602083610268565b6020368337565b91906040906060612dd3612d80565b9485926020855192612de58585610268565b8436853780518452015160208301528482015260076107cf195a01fa15612e0857565bfe5b602092916080604092612e1b612d80565b95869381865193612e2c8686610268565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa8015612e085715612e5d57565b63d4b68fd760e01b5f5260045ffd5b604051612e788161022d565b6040908151612e878382610268565b8236823781526020825191612e9c8484610268565b8336843701528051612eae8282610268565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6020820152815190612f048383610268565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152612f5983519384610268565b8252602082015290565b5f5160206136875f395f51905f5290612f7a612d80565b505f919006602060c0835b61307a575f935f5160206136875f395f51905f5260038186818180090908604051612fb08582610268565b84368237848185604051612fc48282610268565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f5160206136875f395f51905f5260a082015260056107cf195a01fa8015612e085761302e90613670565b519161307a575f5160206136875f395f51905f528280091461306557505f5160206136875f395f51905f5260015f94089293612f85565b9293505061307161029b565b92835282015290565b6114e5565b613087612d80565b506040516130948161022d565b600181526002602082015290565b906006820291808304600614901517156116b157565b90600c8110156114e05760051b0190565b939290916130d760406102aa565b94855260208501526130e960406102aa565b91825260208201526130f9612d98565b925f5b6002811061312657505050602061018092613115612dae565b93849160086201d4c0fa9151151590565b806131326001926130a2565b61313c82856114cf565b515161314882896130b8565b52602061315583866114cf565b51015161316a613164836116a3565b896130b8565b5261317582866114cf565b515151613184613164836116b6565b5261319a61319283876114cf565b515160200190565b516131a7613164836116c4565b5260206131b483876114cf565b510151516131c4613164836116d2565b526131f06131ea6131e360206131da868a6114cf565b51015160200190565b51926116e0565b886130b8565b52016130fc565b6033546001600160a01b0316330361320b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b61ffff61325b826133b4565b1661326581610d4d565b906132736040519283610268565b808252613282601f1991610d4d565b013660208301375f5f5b82518210806132e2575b156132db576001811b84166132b4575b6132af9061175a565b61328c565b9060016132af9160ff60f81b8460f81b165f1a6132d1828761167e565b53019190506132a6565b5050905090565b506101008110613296565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b90600161339760ff936135f8565b928392161b11156133a55790565b63ca95733360e01b5f5260045ffd5b805f915b6133c0575090565b5f1981018181116116b15761ffff9116911661ffff81146116b15760010190806133b8565b906133ee612d80565b5061ffff811690610200821015613493576001821461348e5761340f61029b565b5f81525f602082015292906001905f925b61ffff831685101561343457505050505090565b600161ffff831660ff86161c81161461346e575b60016134646134598360ff94612e0a565b9460011b61fffe1690565b9401169291613420565b9460016134646134596134838960ff95612e0a565b989350505050613448565b505090565b637fc4ea7d60e11b5f5260045ffd5b6134aa612d80565b5080519081158061351b575b156134d75750506040516134cb604082610268565b5f81525f602082015290565b60205f5160206136875f395f51905f52910151065f5160206136875f395f51905f52035f5160206136875f395f51905f5281116116b15760405191612f598361022d565b506020810151156134b6565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b9060ff5f5460081c1615613589576106ee61029992613527565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b156135e957565b631019106960e31b5f5260045ffd5b906101008251116136615781511561365c57602082015160019060f81c81901b5b83518210156136575760019061364261363861187a61186c868961167e565b60ff600191161b90565b9061364e8183116135e2565b17910190613619565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b1561367757565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220fa8f1a81f652d4ef05c87d3b19c99ab8d78c3bbc87fe01c2cd66386c6ae2b3b764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x17\x1F\x1D[\x14a\x02\x14W\x80c\x17\x85\xF5<\x14a\x02\x0FW\x80c\x1F\xDB\x0C\xFD\x14a\x02\nW\x80c'\x942\xEB\x14a\x02\x05W\x80c3\xCF\xB7\xB7\x14a\x02\0W\x80c;\xC2\x8C\x8C\x14a\x01\xFBW\x80cAl~^\x14a\x01\xF6W\x80c]\xF4YF\x14a\x01\xF1W\x80cg\x94\x0C\x89\x14a\x01\xECW\x80ch0H5\x14a\x01\xE7W\x80ck:\xA7.\x14a\x01\xE2W\x80cm\x14\xA9\x87\x14a\x01\xDDW\x80cn\xFBF6\x14a\x01\xD8W\x80cqP\x18\xA6\x14a\x01\xD3W\x80c\x8Dh4\x9A\x14a\x01\xCEW\x80c\x8D\xA5\xCB[\x14a\x01\xC9W\x80c\x99&\xEE}\x14a\x01\xC4W\x80c\x9D\xA1m\x8E\x14a\x01\xBFW\x80c\xA3d\xF4\xDA\x14a\x01\xBAW\x80c\xA9\x8F\xB3U\x14a\x01\xB5W\x80c\xB14Bq\x14a\x01\xB0W\x80c\xB9\x8D\t\x08\x14a\x01\xABW\x80c\xBAU\x08\x80\x14a\x01\xA6W\x80c\xC1\xA8\xE2\xC5\x14a\x01\xA1W\x80c\xC4\xD6m\xE8\x14a\x01\x9CW\x80c\xDF\\\xF7#\x14a\x01\x97W\x80c\xE4o\x18\x16\x14a\x01\x92W\x80c\xE4\x81\xAF\x9D\x14a\x01\x8DW\x80c\xF2\xFD\xE3\x8B\x14a\x01\x88W\x80c\xFC)\x9D\xEE\x14a\x01\x83W\x80c\xFC\xD1\xC3u\x14a\x01~Wc\xFC\xE3l}\x14a\x01yW_\x80\xFD[a\x14`V[a\x14CV[a\x14\x1BV[a\x13\x8AV[a\x13oV[a\x13GV[a\x13\x03V[a\x12\x0EV[a\x10\xFFV[a\x10sV[a\x10QV[a\x10)V[a\x0F\x81V[a\x0E\xC4V[a\x0E:V[a\r\x9EV[a\r%V[a\r\x03V[a\x0C\xA8V[a\x0C\x13V[a\x08\xBDV[a\x08yV[a\x085V[a\x08\x18V[a\x07\xD4V[a\x06\xFDV[a\x06\xC6V[a\x06\x8EV[a\x05\xC2V[a\x05\"V[a\x04:V[a\x03\xC1V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02HW`@RV[a\x02\x19V[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02HW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02HW`@RV[`@Q\x90a\x02\x99a\x01\0\x83a\x02hV[V[`@Q\x90a\x02\x99`@\x83a\x02hV[\x90a\x02\x99`@Q\x92\x83a\x02hV[`@\x90`\xE3\x19\x01\x12a\x02\xE1W`@Q\x90a\x02\xD1\x82a\x02-V[`\xE45\x82Ra\x01\x045` \x83\x01RV[_\x80\xFD[\x91\x90\x82`@\x91\x03\x12a\x02\xE1W`@Qa\x02\xFD\x81a\x02-V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xE1W`@Q\x91a\x03(`@\x84a\x02hV[\x82\x90`@\x81\x01\x92\x83\x11a\x02\xE1W\x90[\x82\x82\x10a\x03DWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x037V[\x90`\x80`c\x19\x83\x01\x12a\x02\xE1W`@Qa\x03m\x81a\x02-V[` a\x03\x88\x82\x94a\x03\x7F\x81`da\x03\rV[\x84R`\xA4a\x03\rV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x02\xE1W` a\x03\x88`@Q\x92a\x03\xAB\x84a\x02-V[`@\x84\x96a\x03\xB9\x83\x82a\x03\rV[\x86R\x01a\x03\rV[4a\x02\xE1Wa\x01 6`\x03\x19\x01\x12a\x02\xE1W`\x045`@6`#\x19\x01\x12a\x02\xE1Wa\x04\x19`@\x91\x82Qa\x03\xF3\x81a\x02-V[`$5\x81R`D5` \x82\x01Ra\x04\t6a\x03TV[\x90a\x04\x136a\x02\xB8V[\x92a\x14\xF9V[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\xE1WV[4a\x02\xE1W_` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x04X\x81a\x04)V[a\x04`a1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xE1W`@Qc&\x89Y\xE5`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x04\xE0Wa\x04\xD2WP\x80\xF3[a\x04\xDE\x91P_\x90a\x02hV[\0[a\x15\xDFV[``\x90`\x03\x19\x01\x12a\x02\xE1W`\x045a\x04\xFD\x81a\x04)V[\x90`$5a\x05\n\x81a\x04)V[\x90`D5`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x02\xE1W\x90V[4a\x02\xE1Wa\x0506a\x04\xE5V[a\x058a1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x83;\x15a\x02\xE1W`@QcJ\x86\xC07`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`$\x82\x01R\x92\x16`D\x83\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x82\x01R_\x81`\x84\x81\x01[\x93\x81\x83\x81\x81\x97\x03\x92Z\xF1\x80\x15a\x04\xE0Wa\x04\xD2WP\x80\xF3[4a\x02\xE1W_` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x05\xE0\x81a\x04)V[a\x05\xE8a1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xE1W`@Qc\xEBZN\x87`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01a\x04\xC0V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x06oWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06bV[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1Wa\x06\xC2a\x06\xB6`\x045a\x06\xB1\x81a\x04)V[a\x17hV[`@Q\x91\x82\x91\x82a\x06LV[\x03\x90\xF3[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1Wa\x04\xDE`\x045a\x06\xE6\x81a\x04)V[a\x06\xEEa1\xF7V[a2\xEDV[\x80\x15\x15\x03a\x02\xE1WV[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x07\x1A\x81a\x06\xF3V[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x04\xE0W_\x91a\x07\x8FW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x07\x80Wa\x04\xDE\x90a3KV[cpp\xF3\xB1`\xE1\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a\x07\xC2W[\x81a\x07\xAA` \x93\x83a\x02hV[\x81\x01\x03\x12a\x02\xE1WQa\x07\xBC\x81a\x04)V[_a\x07gV[=\x91Pa\x07\x9DV[_\x91\x03\x12a\x02\xE1WV[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W` `@Qb\t:\x80\x81R\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x02\xE1WV[`D5\x90a\x02\x99\x82a\t\x01V[5\x90a\x02\x99\x82a\t\x01V[`\x01`\x01`@\x1B\x03\x81\x11a\x02HW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xE1W\x815a\tU\x81a\t'V[\x92a\tc`@Q\x94\x85a\x02hV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xE1W` \x01\x90[\x82\x82\x10a\t\x8BWPPP\x90V[` \x80\x91\x835a\t\x9A\x81a\t\x01V[\x81R\x01\x91\x01\x90a\t~V[\x81`\x1F\x82\x01\x12\x15a\x02\xE1W\x805a\t\xBB\x81a\t'V[\x92a\t\xC9`@Q\x94\x85a\x02hV[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x02\xE1W` \x01\x91[\x83\x83\x10a\t\xF3WPPPP\x90V[` `@\x91a\n\x02\x84\x86a\x02\xE5V[\x81R\x01\x92\x01\x91a\t\xE5V[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xE1W\x815a\n$\x81a\t'V[\x92a\n2`@Q\x94\x85a\x02hV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x02\xE1W` \x82\x01\x90[\x83\x82\x10a\n^WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W` \x91a\n\x80\x87\x84\x80\x94\x88\x01\x01a\t>V[\x81R\x01\x91\x01\x90a\nOV[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x02\xE1Wa\n\xA1a\x02\x89V[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x81a\n\xBE\x91\x84\x01a\t>V[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x81a\n\xDF\x91\x84\x01a\t\xA5V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x81a\x0B\x03\x91\x84\x01a\t\xA5V[`@\x85\x01Ra\x0B\x15\x81``\x84\x01a\x03\x8DV[``\x85\x01Ra\x0B'\x81`\xE0\x84\x01a\x02\xE5V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x81a\x0BL\x91\x84\x01a\t>V[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x81a\x0Bq\x91\x84\x01a\t>V[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1Wa\x0B\x94\x92\x01a\n\rV[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0B\xB8WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\xABV[\x92\x91\x90a\x0C\x0E` \x91`@\x86R\x82a\x0B\xFA\x82Q`@\x80\x8A\x01R`\x80\x89\x01\x90a\x0B\x9BV[\x91\x01Q\x86\x82\x03`?\x19\x01``\x88\x01Ra\x0B\x9BV[\x93\x01RV[4a\x02\xE1W`\x806`\x03\x19\x01\x12a\x02\xE1W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W6`#\x82\x01\x12\x15a\x02\xE1W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W6`$\x82\x84\x01\x01\x11a\x02\xE1Wa\x0Cja\t\x0FV[\x90`d5\x93`\x01`\x01`@\x1B\x03\x85\x11a\x02\xE1W`$a\x0C\x90a\x0C\x98\x966\x90`\x04\x01a\n\x8BV[\x94\x01\x90a\x1C\xA7V[\x90a\x06\xC2`@Q\x92\x83\x92\x83a\x0B\xD7V[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1Wa\x0C\xC0a1\xF7V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W` `\xFF`iT\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x02HW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\rt\x82a\rMV[\x91a\r\x82`@Q\x93\x84a\x02hV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02\xE1W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[4a\x02\xE1W`@6`\x03\x19\x01\x12a\x02\xE1W`\x045a\r\xBB\x81a\x04)V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xE1W```\x03\x19\x836\x03\x01\x12a\x02\xE1W`@Q\x90a\r\xE8\x82a\x02MV[\x82`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W\x83\x016`#\x82\x01\x12\x15a\x02\xE1Wa\x04\xDE\x93a\x0E\"`D\x926\x90`$`\x04\x82\x015\x91\x01a\rhV[\x84R`$\x81\x015` \x85\x01R\x015`@\x83\x01Ra%\x9DV[4a\x02\xE1W_` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x0EX\x81a\x04)V[a\x0E`a1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xE1W`@QcO\x90l\xF9`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01a\x04\xC0V[4a\x02\xE1W_` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x0E\xE2\x81a\x04)V[a\x0F\x163\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a%\x87V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\xE1W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04\xE0Wa\x04\xD2WP\x80\xF3[4a\x02\xE1W_` 6`\x03\x19\x01\x12a\x02\xE1W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W6`#\x82\x01\x12\x15a\x02\xE1Wa\x0F\xC3\x906\x90`$\x81`\x04\x015\x91\x01a\rhV[a\x0F\xCBa1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xE1W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x04\xC0\x90`$\x83\x01\x90a&wV[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`fT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xE1Wa\x10\x816a\x04\xE5V[a\x10\x89a1\xF7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x83;\x15a\x02\xE1W`@Qc\x06d\x12\x01`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`$\x82\x01R\x92\x16`D\x83\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x82\x01R_\x81`\x84\x81\x01a\x05\xAAV[4a\x02\xE1W`@6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x11\x1C\x81a\x04)V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1Wa\x11;\x906\x90`\x04\x01a\t>V[a\x11o3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a%\x87V[a\x11\x8C`@Q\x92a\x11\x7F\x84a\x02MV[`\x01`\x01`\xA0\x1B\x03\x16\x83RV[0` \x83\x01R`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xE1W`@Qcn4\x92\xB5`\xE0\x1B\x81R\x90_\x90\x82\x90\x81\x83\x81a\x11\xEA\x88`\x04\x83\x01a&\x9BV[\x03\x92Z\xF1\x80\x15a\x04\xE0Wa\x11\xFAW\0[\x80a\x12\x08_a\x04\xDE\x93a\x02hV[\x80a\x07\xCAV[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x12+\x81a\x04)V[a\x12{_T\x91a\x12_a\x12Ia\x12E\x85`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x94\x81\x95a\x12\xF5W[\x81\x15a\x12\xD5W[Pa'\x05V[\x82a\x12p`\x01`\xFF\x19_T\x16\x17_UV[a\x12\xBEW[\x80a5oV[a\x12\x81W\0[a\x12\x8Fa\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x12\xD0a\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x12uV[0;\x15\x91P\x81a\x12\xE7W[P_a\x12YV[`\xFF\x16`\x01\x14\x90P_a\x12\xE0V[`\x01`\xFF\x82\x16\x10\x91Pa\x12RV[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1Wa\x06\xC2a\x06\xB6a'hV[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1W`\x045a\x13\xA7\x81a\x04)V[a\x13\xAFa1\xF7V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x13\xC7Wa\x04\xDE\x90a5'V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xE1W_6`\x03\x19\x01\x12a\x02\xE1W` `hT`@Q\x90\x81R\xF3[4a\x02\xE1W` 6`\x03\x19\x01\x12a\x02\xE1W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W6`#\x82\x01\x12\x15a\x02\xE1W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xE1W6`$\x82`\x05\x1B\x84\x01\x01\x11a\x02\xE1W`$a\x04\xDE\x92\x01a)\x95V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x14\xE0W`\x05\x1B\x01\x90V[a\x14\xBBV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x15\xD5a\x15\xB2a\x15\xDB\x95a\x15\xACa\x15\xA5\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x15|\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x02hV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96a-\xC4V[\x90a.\nV[\x92a\x15\xACa\x15\xC7a\x15\xC1a.lV[\x94a/cV[\x91a\x15\xD0a0\x7FV[a-\xC4V[\x91a0\xC9V[\x90\x91V[`@Q=_\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x02\xE1WQ\x90V[\x90\x81` \x91\x03\x12a\x02\xE1WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x02\xE1W\x90V[\x90\x81` \x91\x03\x12a\x02\xE1WQ`\xFF\x81\x16\x81\x03a\x02\xE1W\x90V[`@Q\x90a\x16@` \x83a\x02hV[_\x80\x83R6` \x84\x017V[\x90a\x16V\x82a\t'V[a\x16c`@Q\x91\x82a\x02hV[\x82\x81R\x80\x92a\x16t`\x1F\x19\x91a\t'V[\x01\x90` 6\x91\x017V[\x90\x81Q\x81\x10\x15a\x14\xE0W\x01` \x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x16\xB1WV[a\x16\x8FV[\x90`\x02\x82\x01\x80\x92\x11a\x16\xB1WV[\x90`\x03\x82\x01\x80\x92\x11a\x16\xB1WV[\x90`\x04\x82\x01\x80\x92\x11a\x16\xB1WV[\x90`\x05\x82\x01\x80\x92\x11a\x16\xB1WV[\x91\x90\x82\x01\x80\x92\x11a\x16\xB1WV[`\x01`\x01``\x1B\x03\x81\x16\x03a\x02\xE1WV[\x90\x81`@\x91\x03\x12a\x02\xE1W` `@Q\x91a\x17&\x83a\x02-V[\x80Qa\x171\x81a\x04)V[\x83R\x01Qa\x17>\x81a\x16\xFBV[` \x82\x01R\x90V[\x80Q\x82\x10\x15a\x14\xE0W` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x16\xB1W`\x01\x01\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x04\xE0Wa\x17\xE8\x91` \x91_\x91a\x1A\xE7W[P`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x90\x81\x15a\x04\xE0W_\x91a\x1A\xB8W[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81\x15\x90\x81\x15a\x1AUW[Pa\x1AIWa\x18\x1E\x90a2OV[_\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x18\xFAWa\x18\xA3` a\x18\x80a\x18za\x18l\x89\x87a\x16~V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x87Z\xFA\x80\x15a\x04\xE0W`\x01\x92a\x18\xC2\x92_\x92a\x18\xCAW[Pa\x16\xEEV[\x94\x01\x93a\x18NV[a\x18\xEC\x91\x92P` =\x81\x11a\x18\xF3W[a\x18\xE4\x81\x83a\x02hV[\x81\x01\x90a\x15\xEAV[\x90_a\x18\xBCV[P=a\x18\xDAV[a\x19\x05\x91\x94Pa\x16LV[\x92_\x90_[\x81Q\x81\x10\x15a\x1ACWa\x19#a\x18za\x18l\x83\x85a\x16~V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x90` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x04\xE0W_\x92a\x1A#W[P\x90_\x91[\x81\x83\x10a\x19hWPPP`\x01\x01a\x19\nV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x04\xE0Wa\x19\xE7\x8Ba\x19\xD8\x83a\x19\xD2a\x19\xC6`\x01\x98a\x19\xEC\x98_\x91a\x19\xF5W[PQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x17FV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x17ZV[\x95\x01\x91\x90a\x19VV[a\x1A\x16\x91P`@=\x81\x11a\x1A\x1CW[a\x1A\x0E\x81\x83a\x02hV[\x81\x01\x90a\x17\x0CV[_a\x19\xB8V[P=a\x1A\x04V[a\x1A<\x91\x92P` =\x81\x11a\x18\xF3Wa\x18\xE4\x81\x83a\x02hV[\x90_a\x19QV[PPPPV[Pa\x1ARa\x161V[\x90V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x04\xE0W`\xFF\x91_\x91a\x1A\x89W[P\x16\x15_a\x18\x10V[a\x1A\xAB\x91P` =` \x11a\x1A\xB1W[a\x1A\xA3\x81\x83a\x02hV[\x81\x01\x90a\x16\x18V[_a\x1A\x80V[P=a\x1A\x99V[a\x1A\xDA\x91P` =` \x11a\x1A\xE0W[a\x1A\xD2\x81\x83a\x02hV[\x81\x01\x90a\x15\xF9V[_a\x17\xFBV[P=a\x1A\xC8V[a\x1A\xFE\x91P\x82=\x84\x11a\x18\xF3Wa\x18\xE4\x81\x83a\x02hV[_a\x17\xC8V[`@Q\x90a\x1B\x11\x82a\x02-V[``` \x83\x82\x81R\x01RV[\x15a\x1B$WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a\x1B9WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a\x1BOWV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a\x1BeWV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[_\x19\x81\x01\x91\x90\x82\x11a\x16\xB1WV[\x15a\x1B\x89WV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\xE1WQa\x1AR\x81a\t\x01V[\x90\x82\x10\x15a\x14\xE0W\x01\x90V[\x15a\x1B\xC0WV[c\xAF\xFC^\xDB`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\xE1WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x02\xE1W\x90V[\x15a\x1B\xF7WV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\xE1WQa\x1AR\x81a\x16\xFBV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a\x16\xB1WV[\x15a\x1CBWV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a\x1CXWV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a\x1C\x91WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1C\x84V[\x94\x93\x92\x90\x91\x93a\x1C\xB5a\x1B\x04V[Pa\x1C\xC1\x85\x15\x15a\x1B\x1DV[`@\x84\x01QQ\x85\x14\x80a%yW[\x80a%kW[\x80a%]W[a\x1C\xE4\x90a\x1B2V[a\x1C\xF6` \x85\x01QQ\x85QQ\x14a\x1BHV[a\x1D\rc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a\x1B^V[a\x1D\x15a\x02\x9BV[_\x81R_` \x82\x01R\x92a\x1D'a\x1B\x04V[a\x1D0\x87a\x16LV[` \x82\x01Ra\x1D>\x87a\x16LV[\x81Ra\x1DHa\x1B\x04V[\x92a\x1DW` \x88\x01QQa\x16LV[\x84Ra\x1Dg` \x88\x01QQa\x16LV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x04\xE0Wa\x1D\xD0\x91_\x91a%>W[Pa\x1D\xCB6\x8B\x87a\rhV[a3\x89V[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a\x1FLW` \x88a\x1EAa\x1E7\x8Ca\x1E/\x8F\x96\x86\x8Ea\x1E\x14a\x1E\x01\x86\x80\x95a\x17FV[Q\x80Q_R` \x01Q` R`@_ \x90V[a\x1E!\x84\x84\x84\x01Qa\x17FV[R\x82a\x1F\x19W[\x01Qa\x17FV[Q\x95Qa\x17FV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x04\xE0Wa\x15\xAC\x8Aa\x1E\xEE\x8Fa\x1E\xE7\x8F\x84` \x8F\x92a\x1E\xDE\x93a\x1E\xD6\x84`\x01\x9Ea\x1E\xF4\x9E_\x91a\x1E\xFCW[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\x17FV[R\x01Qa\x17FV[Q\x93\x8DQa\x17FV[Q\x16a3\xB4V[\x90a3\xE5V[\x97\x01\x96a\x1D\xD4V[a\x1F\x13\x91P\x86=\x81\x11a\x1A\xE0Wa\x1A\xD2\x81\x83a\x02hV[_a\x1E\xC7V[a\x1FGa\x1F)\x84\x84\x84\x01Qa\x17FV[Qa\x1F@\x84\x84\x01Qa\x1F:\x87a\x1BtV[\x90a\x17FV[Q\x10a\x1B\x82V[a\x1E(V[P\x90\x95\x97\x94\x96Pa\x1Fa\x91\x98\x93\x92\x99Pa4\xA2V[\x91a\x1Fn`\x97T`\xFF\x16\x90V[\x90\x81\x15a%6W`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x04\xE0W_\x91a%\x07W[P\x91\x90[_\x92[\x81\x84\x10a -WPPPPP\x92a \x06a \x01a\x1F\xFAa '\x95\x85a \x19\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x14\xF9V[\x91\x90a\x1C;V[a\x1CQV[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a\x1CgV[\x03`\x1F\x19\x81\x01\x83R\x82a\x02hV[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da$\x01W[a\x1E7\x82`\xA0a \x90a\x18za \x82\x84a \x98\x97a |a na\x1E\x01\x8F\x9C`@` \x9F\x9E\x01Qa\x17FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba\x1B\xADV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[\x97\x01Qa\x17FV[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x04\xE0Wa!\\a\x1E7\x8F\x95\x8F\x90a!T\x8F\x97\x8F\x96\x84\x8Fa!N`\xC0\x96a!G\x84\x8F` \x9F\x90a\x1E(a \x82\x99`@\x93a\x18z\x9C_\x91a#\xD3W[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a\x1B\xF0V[Q\x90a.\nV[\x9Ca\x1B\xADV[\x96\x01Qa\x17FV[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x04\xE0Wa!\xE9\x91\x8C\x8F\x92_\x92a#\xAFW[P` a!\xDB\x92\x93\x01Qa\x17FV[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a\"\x16\x8Ca!\xDB\x8Ca\"\x0Fa\"\x02\x82` \x86\x01Qa\x17FV[Q`\x01`\x01``\x1B\x03\x16\x90V[\x92Qa\x17FV[_\x98_[` \x8A\x01QQ\x81\x10\x15a#\x96W\x8B\x8Da\"X\x89a\"Ka\x18za \x82\x86\x8F\x89a\"C\x91Qa\x17FV[Q\x94\x87a\x1B\xADV[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a\"gW[PP`\x01\x01a\"\x1AV[\x8A\x8Aa\"\xEF\x85\x9F\x94\x8F\x96\x86a\"\xA9\x8F\x93`\xE0a\"\xA0a\x1E7\x95` a\"\x98a\x18za \x82\x83\x9Fa\"\xAF\x9C\x89\x91a\x1B\xADV[\x9A\x01Qa\x17FV[Q\x9B\x01Qa\x17FV[Qa\x17FV[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x04\xE0W\x8Fa#U\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a#`W[Pa\x19\xD2a!\xDB\x92\x93Q\x93a#Pa\"\x02\x84\x87a\x17FV[a\x1C\x1BV[\x01\x9A\x90P\x8B\x8Da\"]V[a!\xDB\x92Pa#\x88a\x19\xD2\x91` =\x81\x11a#\x8FW[a#\x80\x81\x83a\x02hV[\x81\x01\x90a\x1C\x06V[\x92Pa#8V[P=a#vV[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a\x1F\xC9V[a!\xDB\x92Pa#\xCC` \x91\x82=\x81\x11a#\x8FWa#\x80\x81\x83a\x02hV[\x92Pa!\xCCV[` a#\xF4\x92P=\x81\x11a#\xFAW[a#\xEC\x81\x83a\x02hV[\x81\x01\x90a\x1B\xCFV[_a!1V[P=a#\xE2V[a$>\x94Pa$\x1B\x92Pa\x18z\x91a \x82\x91` \x95a\x1B\xADV[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x04\xE0W` \x89a \x98\x8F\x93\x8F`\xA0\x8F\x97a\x18za \x82\x8F\x8F\x90a |a na\x1E\x01\x8F`@\x8B\x96\x91\x8F\x88\x93a\x1E7\x9Fa$\xC2\x90a$\xC8\x93a \x90\x9F_\x92a$\xDEW[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a\x16\xEEV[\x11a\x1B\xB9V[PPPPPP\x97PPPPPP\x92\x93PPa BV[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a$\xFF\x91=\x81\x11a\x18\xF3Wa\x18\xE4\x81\x83a\x02hV[\x92\x91Pa$\xB1V[a%)\x91P` =` \x11a%/W[a%!\x81\x83a\x02hV[\x81\x01\x90a\x1B\x98V[_a\x1F\xC2V[P=a%\x17V[_\x91\x90a\x1F\xC6V[a%W\x91P` =` \x11a\x1A\xB1Wa\x1A\xA3\x81\x83a\x02hV[_a\x1D\xBFV[P`\xE0\x84\x01QQ\x85\x14a\x1C\xDBV[P`\xC0\x84\x01QQ\x85\x14a\x1C\xD5V[P`\xA0\x84\x01QQ\x85\x14a\x1C\xCFV[\x15a%\x8EWV[cC\x94\xDB\xDF`\xE1\x1B_R`\x04_\xFD[a%\xD13\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a%\x87V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x02\xE1W_\x92\x83\x92`@Q\x94\x85\x80\x94\x81\x93c\x99&\xEE}`\xE0\x1B\x83R`\x01\x80`\xA0\x1B\x03\x16`\x04\x83\x01R`@`$\x83\x01R`@a&F\x82Q```D\x86\x01R`\xA4\x85\x01\x90a&wV[\x91` \x81\x01Q`d\x85\x01R\x01Q`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\x04\xE0Wa&iWPV[\x80a\x12\x08_a\x02\x99\x93a\x02hV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x84\x01R\x81\x84\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x90\x92\x01Q``\x80\x83\x01R\x80Q`\x80\x83\x01\x81\x90R`\xA0\x90\x92\x01\x92\x01\x90_[\x81\x81\x10a&\xE9WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a&\xDCV[\x15a'\x0CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x04\xE0W`\xFF\x91_\x91a)vW[P\x16\x80\x15a)lW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a)(WPa(\x03\x91Pa\x16LV[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x04\xE0W`\xFF\x91_\x91a)\nW[P\x16\x81\x10\x15a)\x03W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x04\xE0W_\x92a(\xE3W[P\x90_\x91[\x81\x83\x10a(}WPPP`\x01\x01a(\x08V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x04\xE0Wa\x19\xE7\x8Ba\x19\xD8\x83a\x19\xD2a\x19\xC6`\x01\x98a(\xDA\x98_\x91a\x19\xF5WPQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01\x91\x90a(kV[a(\xFC\x91\x92P` =\x81\x11a\x18\xF3Wa\x18\xE4\x81\x83a\x02hV[\x90_a(fV[P\x92PPPV[a)\"\x91P` =\x81\x11a\x1A\xB1Wa\x1A\xA3\x81\x83a\x02hV[_a(.V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x04\xE0W`\x01\x92a)c\x92_\x92a\x18\xCAWPa\x16\xEEV[\x92\x01\x91\x90a'\xF1V[PPa\x1ARa\x161V[a)\x8F\x91P` =` \x11a\x1A\xB1Wa\x1A\xA3\x81\x83a\x02hV[_a'\xB9V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x03a+\xB9W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a*\x17WP\x82;\x15a\x02\xE1Wa*\x06\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xFC\xE3l}`\xE0\x1B\x84R`\x04\x84\x01a,qV[\x03\x92Z\xF1\x80\x15a\x04\xE0Wa&iWPV[_` a*ra*5a\x19\xC6\x83a*/\x87\x89\x8Ba+\xC8V[\x01a+\xEAV[`@a*B\x86\x88\x8Aa+\xC8V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x91\x015`D\x82\x01R\x93\x84\x92\x83\x91\x90\x82\x90`d\x82\x01\x90V[\x03\x92Z\xF1\x80\x15a\x04\xE0Wa+\x9DW[Pa*\x95a\x19\xC6` a*/\x84\x86\x88a+\xC8V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x16`$\x82\x01R\x91\x90` \x90\x83\x90`D\x90\x82\x90Z\xFA\x80\x15a\x04\xE0Wa+8` \x91_\x94\x85\x91a+\x80W[Pa+\x05a*\xF0a\x19\xC6\x85a*/\x88\x8B\x8Da+\xC8V[\x91`@a*\xFE\x87\x8A\x8Ca+\xC8V[\x015a\x16\xEEV[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x94\x85\x92\x83\x91\x90\x82\x90`D\x82\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\x04\xE0W`\x01\x92a+RW[P\x01a)\xD4V[a+r\x90` =\x81\x11a+yW[a+j\x81\x83a\x02hV[\x81\x01\x90a+\xF4V[P_a+KV[P=a+`V[a+\x97\x91P\x83=\x81\x11a\x18\xF3Wa\x18\xE4\x81\x83a\x02hV[_a*\xDAV[a+\xB4\x90` =\x81\x11a+yWa+j\x81\x83a\x02hV[a*\x81V[c\x8Ey\xFD\xB5`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a\x14\xE0W`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x02\xE1W\x01\x90V[5a\x1AR\x81a\x04)V[\x90\x81` \x91\x03\x12a\x02\xE1WQa\x1AR\x81a\x06\xF3V[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a,\"WPPP\x90V[\x90\x91\x92`@\x80`\x01\x92\x865a,6\x81a\x04)V[\x84\x80`\xA0\x1B\x03\x16\x81R`\x01`\x01``\x1B\x03` \x88\x015a,U\x81a\x16\xFBV[\x16` \x82\x01R\x01\x94\x01\x92\x91\x01a,\x15V[5\x90a\x02\x99\x82a\x04)V[\x91\x80` \x84\x01` \x85RR`@\x83\x01\x90`@\x81`\x05\x1B\x85\x01\x01\x93\x83_\x91`\x9E\x19\x826\x03\x01\x90[\x84\x84\x10a,\xA8WPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`?\x19\x82\x82\x03\x01\x87R\x875\x83\x81\x12\x15a\x02\xE1W\x84\x01\x90\x815`\x1E\x19\x836\x03\x01\x81\x12\x15a\x02\xE1W\x82\x01\x91` \x835\x93\x01\x90`\x01`\x01`@\x1B\x03\x84\x11a\x02\xE1W\x83`\x06\x1B6\x03\x82\x13a\x02\xE1Wa-q\x83`\x80a-f\x81a-\x1C` \x98\x97`\x01\x9A`\xA0\x8B\x9AR`\xA0\x87\x01\x91a,\tV[\x95a-:a-+\x89\x83\x01a,fV[`\x01`\x01`\xA0\x1B\x03\x16\x86\x8A\x01RV[`@\x81\x015`@\x86\x01Ra-`a-S``\x83\x01a\t\x1CV[c\xFF\xFF\xFF\xFF\x16``\x87\x01RV[\x01a\t\x1CV[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x99\x01\x97\x01\x95\x94\x01\x92\x91\x90a,\x97V[`@Q\x90a-\x8D\x82a\x02-V[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80a-\xA9\x81\x84a\x02hV[6\x837V[`@Q\x90a-\xBD` \x83a\x02hV[` 6\x837V[\x91\x90`@\x90``a-\xD3a-\x80V[\x94\x85\x92` \x85Q\x92a-\xE5\x85\x85a\x02hV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15a.\x08WV[\xFE[` \x92\x91`\x80`@\x92a.\x1Ba-\x80V[\x95\x86\x93\x81\x86Q\x93a.,\x86\x86a\x02hV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15a.\x08W\x15a.]WV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@Qa.x\x81a\x02-V[`@\x90\x81Qa.\x87\x83\x82a\x02hV[\x826\x827\x81R` \x82Q\x91a.\x9C\x84\x84a\x02hV[\x836\x847\x01R\x80Qa.\xAE\x82\x82a\x02hV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90a/\x04\x83\x83a\x02hV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01Ra/Y\x83Q\x93\x84a\x02hV[\x82R` \x82\x01R\x90V[_Q` a6\x87_9_Q\x90_R\x90a/za-\x80V[P_\x91\x90\x06` `\xC0\x83[a0zW_\x93_Q` a6\x87_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa/\xB0\x85\x82a\x02hV[\x846\x827\x84\x81\x85`@Qa/\xC4\x82\x82a\x02hV[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` a6\x87_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a.\x08Wa0.\x90a6pV[Q\x91a0zW_Q` a6\x87_9_Q\x90_R\x82\x80\t\x14a0eWP_Q` a6\x87_9_Q\x90_R`\x01_\x94\x08\x92\x93a/\x85V[\x92\x93PPa0qa\x02\x9BV[\x92\x83R\x82\x01R\x90V[a\x14\xE5V[a0\x87a-\x80V[P`@Qa0\x94\x81a\x02-V[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a\x16\xB1WV[\x90`\x0C\x81\x10\x15a\x14\xE0W`\x05\x1B\x01\x90V[\x93\x92\x90\x91a0\xD7`@a\x02\xAAV[\x94\x85R` \x85\x01Ra0\xE9`@a\x02\xAAV[\x91\x82R` \x82\x01Ra0\xF9a-\x98V[\x92_[`\x02\x81\x10a1&WPPP` a\x01\x80\x92a1\x15a-\xAEV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80a12`\x01\x92a0\xA2V[a1<\x82\x85a\x14\xCFV[QQa1H\x82\x89a0\xB8V[R` a1U\x83\x86a\x14\xCFV[Q\x01Qa1ja1d\x83a\x16\xA3V[\x89a0\xB8V[Ra1u\x82\x86a\x14\xCFV[QQQa1\x84a1d\x83a\x16\xB6V[Ra1\x9Aa1\x92\x83\x87a\x14\xCFV[QQ` \x01\x90V[Qa1\xA7a1d\x83a\x16\xC4V[R` a1\xB4\x83\x87a\x14\xCFV[Q\x01QQa1\xC4a1d\x83a\x16\xD2V[Ra1\xF0a1\xEAa1\xE3` a1\xDA\x86\x8Aa\x14\xCFV[Q\x01Q` \x01\x90V[Q\x92a\x16\xE0V[\x88a0\xB8V[R\x01a0\xFCV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a2\x0BWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[a\xFF\xFFa2[\x82a3\xB4V[\x16a2e\x81a\rMV[\x90a2s`@Q\x92\x83a\x02hV[\x80\x82Ra2\x82`\x1F\x19\x91a\rMV[\x016` \x83\x017__[\x82Q\x82\x10\x80a2\xE2W[\x15a2\xDBW`\x01\x81\x1B\x84\x16a2\xB4W[a2\xAF\x90a\x17ZV[a2\x8CV[\x90`\x01a2\xAF\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa2\xD1\x82\x87a\x16~V[S\x01\x91\x90Pa2\xA6V[PP\x90P\x90V[Pa\x01\0\x81\x10a2\x96V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eUV[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01a3\x97`\xFF\x93a5\xF8V[\x92\x83\x92\x16\x1B\x11\x15a3\xA5W\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[a3\xC0WP\x90V[_\x19\x81\x01\x81\x81\x11a\x16\xB1Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x16\xB1W`\x01\x01\x90\x80a3\xB8V[\x90a3\xEEa-\x80V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15a4\x93W`\x01\x82\x14a4\x8EWa4\x0Fa\x02\x9BV[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15a44WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14a4nW[`\x01a4da4Y\x83`\xFF\x94a.\nV[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91a4 V[\x94`\x01a4da4Ya4\x83\x89`\xFF\x95a.\nV[\x98\x93PPPPa4HV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[a4\xAAa-\x80V[P\x80Q\x90\x81\x15\x80a5\x1BW[\x15a4\xD7WPP`@Qa4\xCB`@\x82a\x02hV[_\x81R_` \x82\x01R\x90V[` _Q` a6\x87_9_Q\x90_R\x91\x01Q\x06_Q` a6\x87_9_Q\x90_R\x03_Q` a6\x87_9_Q\x90_R\x81\x11a\x16\xB1W`@Q\x91a/Y\x83a\x02-V[P` \x81\x01Q\x15a4\xB6V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x90`\xFF_T`\x08\x1C\x16\x15a5\x89Wa\x06\xEEa\x02\x99\x92a5'V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[\x15a5\xE9WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11a6aW\x81Q\x15a6\\W` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15a6WW`\x01\x90a6Ba68a\x18za\x18l\x86\x89a\x16~V[`\xFF`\x01\x91\x16\x1B\x90V[\x90a6N\x81\x83\x11a5\xE2V[\x17\x91\x01\x90a6\x19V[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15a6wWV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xFA\x8F\x1A\x81\xF6R\xD4\xEF\x05\xC8};\x19\xC9\x9A\xB8\xD7\x8C;\xBC\x87\xFE\x01\xC2\xCDf8lj\xE2\xB3\xB7dsolcC\0\x08\x1B\x003",
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
    /**Custom error with signature `DelayPeriodNotPassed()` and selector `0xfb623b04`.
```solidity
error DelayPeriodNotPassed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelayPeriodNotPassed {}
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
        impl ::core::convert::From<DelayPeriodNotPassed> for UnderlyingRustTuple<'_> {
            fn from(value: DelayPeriodNotPassed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelayPeriodNotPassed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelayPeriodNotPassed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelayPeriodNotPassed()";
            const SELECTOR: [u8; 4] = [251u8, 98u8, 59u8, 4u8];
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
    /**Custom error with signature `OnlyRegistryCoordinator()` and selector `0x8729b7be`.
```solidity
error OnlyRegistryCoordinator();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRegistryCoordinator {}
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
        impl ::core::convert::From<OnlyRegistryCoordinator> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRegistryCoordinator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyRegistryCoordinator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRegistryCoordinator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRegistryCoordinator()";
            const SELECTOR: [u8; 4] = [135u8, 41u8, 183u8, 190u8];
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
    /**Custom error with signature `OnlyRewardsInitiator()` and selector `0x8e79fdb5`.
```solidity
error OnlyRewardsInitiator();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRewardsInitiator {}
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
        impl ::core::convert::From<OnlyRewardsInitiator> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRewardsInitiator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyRewardsInitiator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRewardsInitiator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRewardsInitiator()";
            const SELECTOR: [u8; 4] = [142u8, 121u8, 253u8, 181u8];
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
    /**Custom error with signature `OnlySlasher()` and selector `0x7e57b1e1`.
```solidity
error OnlySlasher();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlySlasher {}
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
        impl ::core::convert::From<OnlySlasher> for UnderlyingRustTuple<'_> {
            fn from(value: OnlySlasher) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlySlasher {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlySlasher {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlySlasher()";
            const SELECTOR: [u8; 4] = [126u8, 87u8, 177u8, 225u8];
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
    /**Custom error with signature `OnlyStakeRegistry()` and selector `0x46bf2281`.
```solidity
error OnlyStakeRegistry();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyStakeRegistry {}
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
        impl ::core::convert::From<OnlyStakeRegistry> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyStakeRegistry) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyStakeRegistry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyStakeRegistry {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyStakeRegistry()";
            const SELECTOR: [u8; 4] = [70u8, 191u8, 34u8, 129u8];
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
    /**Event with signature `RewardsInitiatorUpdated(address,address)` and selector `0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3`.
```solidity
event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsInitiatorUpdated {
        #[allow(missing_docs)]
        pub prevRewardsInitiator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newRewardsInitiator: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RewardsInitiatorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RewardsInitiatorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                225u8,
                28u8,
                221u8,
                241u8,
                129u8,
                106u8,
                67u8,
                49u8,
                140u8,
                161u8,
                117u8,
                187u8,
                197u8,
                44u8,
                208u8,
                24u8,
                84u8,
                54u8,
                233u8,
                203u8,
                234u8,
                215u8,
                200u8,
                58u8,
                204u8,
                84u8,
                167u8,
                62u8,
                70u8,
                23u8,
                23u8,
                227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevRewardsInitiator: data.0,
                    newRewardsInitiator: data.1,
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
                        &self.prevRewardsInitiator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newRewardsInitiator,
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
        impl alloy_sol_types::private::IntoLogData for RewardsInitiatorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsInitiatorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RewardsInitiatorUpdated,
            ) -> alloy_sol_types::private::LogData {
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
    /**Constructor`.
```solidity
constructor(address _registryCoordinator, address _avsDirectory, address _rewardsCoordinator, address _allocationManager, address _permissionController);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avsDirectory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _rewardsCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _permissionController: alloy::sol_types::private::Address,
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._registryCoordinator,
                        value._avsDirectory,
                        value._rewardsCoordinator,
                        value._allocationManager,
                        value._permissionController,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                        _avsDirectory: tuple.1,
                        _rewardsCoordinator: tuple.2,
                        _allocationManager: tuple.3,
                        _permissionController: tuple.4,
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
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._rewardsCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionController,
                    ),
                )
            }
        }
    };
    /**Function with signature `SLASHER_PROPOSAL_DELAY()` and selector `0x67940c89`.
```solidity
function SLASHER_PROPOSAL_DELAY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLASHER_PROPOSAL_DELAYCall {}
    ///Container type for the return parameters of the [`SLASHER_PROPOSAL_DELAY()`](SLASHER_PROPOSAL_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLASHER_PROPOSAL_DELAYReturn {
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
            impl ::core::convert::From<SLASHER_PROPOSAL_DELAYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SLASHER_PROPOSAL_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SLASHER_PROPOSAL_DELAYCall {
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
            impl ::core::convert::From<SLASHER_PROPOSAL_DELAYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SLASHER_PROPOSAL_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SLASHER_PROPOSAL_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SLASHER_PROPOSAL_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = SLASHER_PROPOSAL_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SLASHER_PROPOSAL_DELAY()";
            const SELECTOR: [u8; 4] = [103u8, 148u8, 12u8, 137u8];
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
    /**Function with signature `addPendingAdmin(address)` and selector `0x279432eb`.
```solidity
function addPendingAdmin(address admin) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPendingAdminCall {
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addPendingAdmin(address)`](addPendingAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPendingAdminReturn {}
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
            impl ::core::convert::From<addPendingAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: addPendingAdminCall) -> Self {
                    (value.admin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addPendingAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { admin: tuple.0 }
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
            impl ::core::convert::From<addPendingAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addPendingAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addPendingAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addPendingAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addPendingAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addPendingAdmin(address)";
            const SELECTOR: [u8; 4] = [39u8, 148u8, 50u8, 235u8];
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
                        &self.admin,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureChecker.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureChecker.QuorumStakeTotals memory, bytes32);
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
        pub params: <IBLSSignatureChecker::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](checkSignaturesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesReturn {
        #[allow(missing_docs)]
        pub _0: <IBLSSignatureChecker::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
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
    /**Function with signature `createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xfce36c7d`.
```solidity
function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionCall {
        #[allow(missing_docs)]
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])`](createAVSRewardsSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionReturn {}
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
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createAVSRewardsSubmissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createAVSRewardsSubmissionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        rewardsSubmissions: tuple.0,
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
            impl ::core::convert::From<createAVSRewardsSubmissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createAVSRewardsSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [252u8, 227u8, 108u8, 125u8];
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
                        IRewardsCoordinatorTypes::RewardsSubmission,
                    > as alloy_sol_types::SolType>::tokenize(&self.rewardsSubmissions),
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
    /**Function with signature `deregisterOperatorFromAVS(address)` and selector `0xa364f4da`.
```solidity
function deregisterOperatorFromAVS(address operator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromAVS(address)`](deregisterOperatorFromAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSReturn {}
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
            impl ::core::convert::From<deregisterOperatorFromAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<deregisterOperatorFromAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromAVS(address)";
            const SELECTOR: [u8; 4] = [163u8, 100u8, 244u8, 218u8];
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
    /**Function with signature `deregisterOperatorFromOperatorSets(address,uint32[])` and selector `0xc1a8e2c5`.
```solidity
function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromOperatorSets(address,uint32[])`](deregisterOperatorFromOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsReturn {}
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsCall) -> Self {
                    (value.operator, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromOperatorSetsCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromOperatorSets(address,uint32[])";
            const SELECTOR: [u8; 4] = [193u8, 168u8, 226u8, 197u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorRestakedStrategies(address)` and selector `0x33cfb7b7`.
```solidity
function getOperatorRestakedStrategies(address operator) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorRestakedStrategies(address)`](getOperatorRestakedStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getOperatorRestakedStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorRestakedStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<getOperatorRestakedStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorRestakedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorRestakedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorRestakedStrategiesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorRestakedStrategies(address)";
            const SELECTOR: [u8; 4] = [51u8, 207u8, 183u8, 183u8];
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
    /**Function with signature `getRestakeableStrategies()` and selector `0xe481af9d`.
```solidity
function getRestakeableStrategies() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesCall {}
    ///Container type for the return parameters of the [`getRestakeableStrategies()`](getRestakeableStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getRestakeableStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRestakeableStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getRestakeableStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRestakeableStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRestakeableStrategiesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRestakeableStrategiesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRestakeableStrategies()";
            const SELECTOR: [u8; 4] = [228u8, 129u8, 175u8, 157u8];
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
    /**Function with signature `initialize(address)` and selector `0xc4d66de8`.
```solidity
function initialize(address _initialOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _initialOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address)`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._initialOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _initialOwner: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address)";
            const SELECTOR: [u8; 4] = [196u8, 214u8, 109u8, 232u8];
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
    /**Function with signature `migrationFinalized()` and selector `0x8d68349a`.
```solidity
function migrationFinalized() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrationFinalizedCall {}
    ///Container type for the return parameters of the [`migrationFinalized()`](migrationFinalizedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrationFinalizedReturn {
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
            impl ::core::convert::From<migrationFinalizedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrationFinalizedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrationFinalizedCall {
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
            impl ::core::convert::From<migrationFinalizedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrationFinalizedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrationFinalizedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrationFinalizedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrationFinalizedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrationFinalized()";
            const SELECTOR: [u8; 4] = [141u8, 104u8, 52u8, 154u8];
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
    /**Function with signature `proposedSlasher()` and selector `0xe46f1816`.
```solidity
function proposedSlasher() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposedSlasherCall {}
    ///Container type for the return parameters of the [`proposedSlasher()`](proposedSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposedSlasherReturn {
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
            impl ::core::convert::From<proposedSlasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposedSlasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposedSlasherCall {
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
            impl ::core::convert::From<proposedSlasherReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposedSlasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposedSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposedSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposedSlasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposedSlasher()";
            const SELECTOR: [u8; 4] = [228u8, 111u8, 24u8, 22u8];
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
    /**Function with signature `registerOperatorToAVS(address,(bytes,bytes32,uint256))` and selector `0x9926ee7d`.
```solidity
function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperatorToAVS(address,(bytes,bytes32,uint256))`](registerOperatorToAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSReturn {}
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
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerOperatorToAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSCall) -> Self {
                    (value.operator, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSignature: tuple.1,
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
            impl ::core::convert::From<registerOperatorToAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorToAVSCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorToAVS(address,(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [153u8, 38u8, 238u8, 125u8];
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
    /**Function with signature `removeAdmin(address)` and selector `0x1785f53c`.
```solidity
function removeAdmin(address admin) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAdminCall {
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removeAdmin(address)`](removeAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAdminReturn {}
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
            impl ::core::convert::From<removeAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeAdminCall) -> Self {
                    (value.admin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { admin: tuple.0 }
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
            impl ::core::convert::From<removeAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeAdmin(address)";
            const SELECTOR: [u8; 4] = [23u8, 133u8, 245u8, 60u8];
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
                        &self.admin,
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
    /**Function with signature `removeAppointee(address,address,bytes4)` and selector `0xba550880`.
```solidity
function removeAppointee(address appointee, address target, bytes4 selector) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAppointeeCall {
        #[allow(missing_docs)]
        pub appointee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selector: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`removeAppointee(address,address,bytes4)`](removeAppointeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAppointeeReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<removeAppointeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeAppointeeCall) -> Self {
                    (value.appointee, value.target, value.selector)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAppointeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        appointee: tuple.0,
                        target: tuple.1,
                        selector: tuple.2,
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
            impl ::core::convert::From<removeAppointeeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeAppointeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeAppointeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeAppointeeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeAppointeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeAppointee(address,address,bytes4)";
            const SELECTOR: [u8; 4] = [186u8, 85u8, 8u8, 128u8];
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
                        &self.appointee,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.selector),
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
    /**Function with signature `removePendingAdmin(address)` and selector `0x9da16d8e`.
```solidity
function removePendingAdmin(address pendingAdmin) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removePendingAdminCall {
        #[allow(missing_docs)]
        pub pendingAdmin: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removePendingAdmin(address)`](removePendingAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removePendingAdminReturn {}
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
            impl ::core::convert::From<removePendingAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removePendingAdminCall) -> Self {
                    (value.pendingAdmin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removePendingAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pendingAdmin: tuple.0 }
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
            impl ::core::convert::From<removePendingAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removePendingAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removePendingAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removePendingAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removePendingAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removePendingAdmin(address)";
            const SELECTOR: [u8; 4] = [157u8, 161u8, 109u8, 142u8];
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
                        &self.pendingAdmin,
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
    /**Function with signature `rewardsInitiator()` and selector `0xfc299dee`.
```solidity
function rewardsInitiator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorCall {}
    ///Container type for the return parameters of the [`rewardsInitiator()`](rewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorReturn {
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
            impl ::core::convert::From<rewardsInitiatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsInitiatorCall {
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
            impl ::core::convert::From<rewardsInitiatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsInitiatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsInitiatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rewardsInitiator()";
            const SELECTOR: [u8; 4] = [252u8, 41u8, 157u8, 238u8];
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
    /**Function with signature `setAppointee(address,address,bytes4)` and selector `0x1fdb0cfd`.
```solidity
function setAppointee(address appointee, address target, bytes4 selector) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAppointeeCall {
        #[allow(missing_docs)]
        pub appointee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selector: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`setAppointee(address,address,bytes4)`](setAppointeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAppointeeReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<setAppointeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAppointeeCall) -> Self {
                    (value.appointee, value.target, value.selector)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAppointeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        appointee: tuple.0,
                        target: tuple.1,
                        selector: tuple.2,
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
            impl ::core::convert::From<setAppointeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAppointeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAppointeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAppointeeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAppointeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAppointee(address,address,bytes4)";
            const SELECTOR: [u8; 4] = [31u8, 219u8, 12u8, 253u8];
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
                        &self.appointee,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.selector),
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
    /**Function with signature `setRewardsInitiator(address)` and selector `0x3bc28c8c`.
```solidity
function setRewardsInitiator(address newRewardsInitiator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorCall {
        #[allow(missing_docs)]
        pub newRewardsInitiator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setRewardsInitiator(address)`](setRewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorReturn {}
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
            impl ::core::convert::From<setRewardsInitiatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorCall) -> Self {
                    (value.newRewardsInitiator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsInitiatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newRewardsInitiator: tuple.0,
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
            impl ::core::convert::From<setRewardsInitiatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsInitiatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsInitiatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRewardsInitiator(address)";
            const SELECTOR: [u8; 4] = [59u8, 194u8, 140u8, 140u8];
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
                        &self.newRewardsInitiator,
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
    /**Function with signature `slasher()` and selector `0xb1344271`.
```solidity
function slasher() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherCall {}
    ///Container type for the return parameters of the [`slasher()`](slasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherReturn {
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
            impl ::core::convert::From<slasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: slasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherCall {
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
            impl ::core::convert::From<slasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slasher()";
            const SELECTOR: [u8; 4] = [177u8, 52u8, 66u8, 113u8];
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
    /**Function with signature `slasherProposalTimestamp()` and selector `0xfcd1c375`.
```solidity
function slasherProposalTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherProposalTimestampCall {}
    ///Container type for the return parameters of the [`slasherProposalTimestamp()`](slasherProposalTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherProposalTimestampReturn {
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
            impl ::core::convert::From<slasherProposalTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slasherProposalTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slasherProposalTimestampCall {
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
            impl ::core::convert::From<slasherProposalTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slasherProposalTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slasherProposalTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slasherProposalTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slasherProposalTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slasherProposalTimestamp()";
            const SELECTOR: [u8; 4] = [252u8, 209u8, 195u8, 117u8];
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
    /**Function with signature `updateAVSMetadataURI(string)` and selector `0xa98fb355`.
```solidity
function updateAVSMetadataURI(string memory _metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        #[allow(missing_docs)]
        pub _metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateAVSMetadataURI(string)`](updateAVSMetadataURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURIReturn {}
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
            impl ::core::convert::From<updateAVSMetadataURICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value._metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _metadataURI: tuple.0 }
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
            impl ::core::convert::From<updateAVSMetadataURIReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateAVSMetadataURICall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAVSMetadataURI(string)";
            const SELECTOR: [u8; 4] = [169u8, 143u8, 179u8, 85u8];
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
                        &self._metadataURI,
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
    ///Container for all the [`MockAvsServiceManager`](self) function calls.
    pub enum MockAvsServiceManagerCalls {
        #[allow(missing_docs)]
        SLASHER_PROPOSAL_DELAY(SLASHER_PROPOSAL_DELAYCall),
        #[allow(missing_docs)]
        addPendingAdmin(addPendingAdminCall),
        #[allow(missing_docs)]
        avsDirectory(avsDirectoryCall),
        #[allow(missing_docs)]
        blsApkRegistry(blsApkRegistryCall),
        #[allow(missing_docs)]
        checkSignatures(checkSignaturesCall),
        #[allow(missing_docs)]
        createAVSRewardsSubmission(createAVSRewardsSubmissionCall),
        #[allow(missing_docs)]
        delegation(delegationCall),
        #[allow(missing_docs)]
        deregisterOperatorFromAVS(deregisterOperatorFromAVSCall),
        #[allow(missing_docs)]
        deregisterOperatorFromOperatorSets(deregisterOperatorFromOperatorSetsCall),
        #[allow(missing_docs)]
        getOperatorRestakedStrategies(getOperatorRestakedStrategiesCall),
        #[allow(missing_docs)]
        getRestakeableStrategies(getRestakeableStrategiesCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        migrationFinalized(migrationFinalizedCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        proposedSlasher(proposedSlasherCall),
        #[allow(missing_docs)]
        registerOperatorToAVS(registerOperatorToAVSCall),
        #[allow(missing_docs)]
        registryCoordinator(registryCoordinatorCall),
        #[allow(missing_docs)]
        removeAdmin(removeAdminCall),
        #[allow(missing_docs)]
        removeAppointee(removeAppointeeCall),
        #[allow(missing_docs)]
        removePendingAdmin(removePendingAdminCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        rewardsInitiator(rewardsInitiatorCall),
        #[allow(missing_docs)]
        setAppointee(setAppointeeCall),
        #[allow(missing_docs)]
        setRewardsInitiator(setRewardsInitiatorCall),
        #[allow(missing_docs)]
        setStaleStakesForbidden(setStaleStakesForbiddenCall),
        #[allow(missing_docs)]
        slasher(slasherCall),
        #[allow(missing_docs)]
        slasherProposalTimestamp(slasherProposalTimestampCall),
        #[allow(missing_docs)]
        stakeRegistry(stakeRegistryCall),
        #[allow(missing_docs)]
        staleStakesForbidden(staleStakesForbiddenCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        trySignatureAndApkVerification(trySignatureAndApkVerificationCall),
        #[allow(missing_docs)]
        updateAVSMetadataURI(updateAVSMetadataURICall),
    }
    #[automatically_derived]
    impl MockAvsServiceManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [23u8, 31u8, 29u8, 91u8],
            [23u8, 133u8, 245u8, 60u8],
            [31u8, 219u8, 12u8, 253u8],
            [39u8, 148u8, 50u8, 235u8],
            [51u8, 207u8, 183u8, 183u8],
            [59u8, 194u8, 140u8, 140u8],
            [65u8, 108u8, 126u8, 94u8],
            [93u8, 244u8, 89u8, 70u8],
            [103u8, 148u8, 12u8, 137u8],
            [104u8, 48u8, 72u8, 53u8],
            [107u8, 58u8, 167u8, 46u8],
            [109u8, 20u8, 169u8, 135u8],
            [110u8, 251u8, 70u8, 54u8],
            [113u8, 80u8, 24u8, 166u8],
            [141u8, 104u8, 52u8, 154u8],
            [141u8, 165u8, 203u8, 91u8],
            [153u8, 38u8, 238u8, 125u8],
            [157u8, 161u8, 109u8, 142u8],
            [163u8, 100u8, 244u8, 218u8],
            [169u8, 143u8, 179u8, 85u8],
            [177u8, 52u8, 66u8, 113u8],
            [185u8, 141u8, 9u8, 8u8],
            [186u8, 85u8, 8u8, 128u8],
            [193u8, 168u8, 226u8, 197u8],
            [196u8, 214u8, 109u8, 232u8],
            [223u8, 92u8, 247u8, 35u8],
            [228u8, 111u8, 24u8, 22u8],
            [228u8, 129u8, 175u8, 157u8],
            [242u8, 253u8, 227u8, 139u8],
            [252u8, 41u8, 157u8, 238u8],
            [252u8, 209u8, 195u8, 117u8],
            [252u8, 227u8, 108u8, 125u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockAvsServiceManagerCalls {
        const NAME: &'static str = "MockAvsServiceManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 32usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::SLASHER_PROPOSAL_DELAY(_) => {
                    <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addPendingAdmin(_) => {
                    <addPendingAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkSignatures(_) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createAVSRewardsSubmission(_) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromAVS(_) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromOperatorSets(_) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorRestakedStrategies(_) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRestakeableStrategies(_) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::migrationFinalized(_) => {
                    <migrationFinalizedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proposedSlasher(_) => {
                    <proposedSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperatorToAVS(_) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeAdmin(_) => {
                    <removeAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeAppointee(_) => {
                    <removeAppointeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removePendingAdmin(_) => {
                    <removePendingAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardsInitiator(_) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAppointee(_) => {
                    <setAppointeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRewardsInitiator(_) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setStaleStakesForbidden(_) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slasher(_) => <slasherCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::slasherProposalTimestamp(_) => {
                    <slasherProposalTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::staleStakesForbidden(_) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::trySignatureAndApkVerification(_) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateAVSMetadataURI(_) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls>] = &[
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerCalls::trySignatureAndApkVerification,
                            )
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn removeAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <removeAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::removeAdmin)
                    }
                    removeAdmin
                },
                {
                    fn setAppointee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <setAppointeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::setAppointee)
                    }
                    setAppointee
                },
                {
                    fn addPendingAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <addPendingAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::addPendingAdmin)
                    }
                    addPendingAdmin
                },
                {
                    fn getOperatorRestakedStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerCalls::getOperatorRestakedStrategies,
                            )
                    }
                    getOperatorRestakedStrategies
                },
                {
                    fn setRewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::setRewardsInitiator)
                    }
                    setRewardsInitiator
                },
                {
                    fn setStaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::setStaleStakesForbidden)
                    }
                    setStaleStakesForbidden
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn SLASHER_PROPOSAL_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::SLASHER_PROPOSAL_DELAY)
                    }
                    SLASHER_PROPOSAL_DELAY
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn migrationFinalized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <migrationFinalizedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::migrationFinalized)
                    }
                    migrationFinalized
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn registerOperatorToAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::registerOperatorToAVS)
                    }
                    registerOperatorToAVS
                },
                {
                    fn removePendingAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <removePendingAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::removePendingAdmin)
                    }
                    removePendingAdmin
                },
                {
                    fn deregisterOperatorFromAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::deregisterOperatorFromAVS)
                    }
                    deregisterOperatorFromAVS
                },
                {
                    fn updateAVSMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::updateAVSMetadataURI)
                    }
                    updateAVSMetadataURI
                },
                {
                    fn slasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <slasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::slasher)
                    }
                    slasher
                },
                {
                    fn staleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::staleStakesForbidden)
                    }
                    staleStakesForbidden
                },
                {
                    fn removeAppointee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <removeAppointeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::removeAppointee)
                    }
                    removeAppointee
                },
                {
                    fn deregisterOperatorFromOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerCalls::deregisterOperatorFromOperatorSets,
                            )
                    }
                    deregisterOperatorFromOperatorSets
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn proposedSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <proposedSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::proposedSlasher)
                    }
                    proposedSlasher
                },
                {
                    fn getRestakeableStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::getRestakeableStrategies)
                    }
                    getRestakeableStrategies
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn rewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::rewardsInitiator)
                    }
                    rewardsInitiator
                },
                {
                    fn slasherProposalTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <slasherProposalTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::slasherProposalTimestamp)
                    }
                    slasherProposalTimestamp
                },
                {
                    fn createAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerCalls::createAVSRewardsSubmission)
                    }
                    createAVSRewardsSubmission
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
                Self::SLASHER_PROPOSAL_DELAY(inner) => {
                    <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addPendingAdmin(inner) => {
                    <addPendingAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::migrationFinalized(inner) => {
                    <migrationFinalizedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proposedSlasher(inner) => {
                    <proposedSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeAdmin(inner) => {
                    <removeAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeAppointee(inner) => {
                    <removeAppointeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removePendingAdmin(inner) => {
                    <removePendingAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAppointee(inner) => {
                    <setAppointeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setStaleStakesForbidden(inner) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::slasherProposalTimestamp(inner) => {
                    <slasherProposalTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::SLASHER_PROPOSAL_DELAY(inner) => {
                    <SLASHER_PROPOSAL_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addPendingAdmin(inner) => {
                    <addPendingAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::migrationFinalized(inner) => {
                    <migrationFinalizedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proposedSlasher(inner) => {
                    <proposedSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::removeAdmin(inner) => {
                    <removeAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeAppointee(inner) => {
                    <removeAppointeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removePendingAdmin(inner) => {
                    <removePendingAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAppointee(inner) => {
                    <setAppointeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::slasherProposalTimestamp(inner) => {
                    <slasherProposalTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`MockAvsServiceManager`](self) custom errors.
    pub enum MockAvsServiceManagerErrors {
        #[allow(missing_docs)]
        BitmapValueTooLarge(BitmapValueTooLarge),
        #[allow(missing_docs)]
        BytesArrayLengthTooLong(BytesArrayLengthTooLong),
        #[allow(missing_docs)]
        BytesArrayNotOrdered(BytesArrayNotOrdered),
        #[allow(missing_docs)]
        DelayPeriodNotPassed(DelayPeriodNotPassed),
        #[allow(missing_docs)]
        ECAddFailed(ECAddFailed),
        #[allow(missing_docs)]
        ECMulFailed(ECMulFailed),
        #[allow(missing_docs)]
        ExpModFailed(ExpModFailed),
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
        InvalidQuorumApkHash(InvalidQuorumApkHash),
        #[allow(missing_docs)]
        InvalidReferenceBlocknumber(InvalidReferenceBlocknumber),
        #[allow(missing_docs)]
        NonSignerPubkeysNotSorted(NonSignerPubkeysNotSorted),
        #[allow(missing_docs)]
        OnlyRegistryCoordinator(OnlyRegistryCoordinator),
        #[allow(missing_docs)]
        OnlyRegistryCoordinatorOwner(OnlyRegistryCoordinatorOwner),
        #[allow(missing_docs)]
        OnlyRewardsInitiator(OnlyRewardsInitiator),
        #[allow(missing_docs)]
        OnlySlasher(OnlySlasher),
        #[allow(missing_docs)]
        OnlyStakeRegistry(OnlyStakeRegistry),
        #[allow(missing_docs)]
        ScalarTooLarge(ScalarTooLarge),
        #[allow(missing_docs)]
        StaleStakesForbidden(StaleStakesForbidden),
    }
    #[automatically_derived]
    impl MockAvsServiceManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [31u8, 4u8, 5u8, 160u8],
            [67u8, 113u8, 74u8, 253u8],
            [70u8, 51u8, 190u8, 50u8],
            [70u8, 191u8, 34u8, 129u8],
            [75u8, 135u8, 79u8, 69u8],
            [95u8, 131u8, 47u8, 65u8],
            [103u8, 152u8, 141u8, 51u8],
            [126u8, 87u8, 177u8, 225u8],
            [128u8, 200u8, 131u8, 72u8],
            [135u8, 41u8, 183u8, 190u8],
            [142u8, 121u8, 253u8, 181u8],
            [171u8, 27u8, 35u8, 107u8],
            [175u8, 252u8, 94u8, 219u8],
            [202u8, 149u8, 115u8, 51u8],
            [212u8, 182u8, 143u8, 215u8],
            [213u8, 30u8, 218u8, 227u8],
            [224u8, 225u8, 231u8, 98u8],
            [225u8, 49u8, 10u8, 237u8],
            [251u8, 74u8, 156u8, 142u8],
            [251u8, 98u8, 59u8, 4u8],
            [255u8, 113u8, 148u8, 20u8],
            [255u8, 137u8, 212u8, 250u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockAvsServiceManagerErrors {
        const NAME: &'static str = "MockAvsServiceManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 22usize;
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
                Self::DelayPeriodNotPassed(_) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidQuorumApkHash(_) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidReferenceBlocknumber(_) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonSignerPubkeysNotSorted(_) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyRegistryCoordinator(_) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyRegistryCoordinatorOwner(_) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyRewardsInitiator(_) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlySlasher(_) => {
                    <OnlySlasher as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyStakeRegistry(_) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors>] = &[
                {
                    fn InputEmptyQuorumNumbers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::InputEmptyQuorumNumbers)
                    }
                    InputEmptyQuorumNumbers
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn ECMulFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn OnlyStakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::OnlyStakeRegistry)
                    }
                    OnlyStakeRegistry
                },
                {
                    fn InvalidReferenceBlocknumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerErrors::InvalidReferenceBlocknumber,
                            )
                    }
                    InvalidReferenceBlocknumber
                },
                {
                    fn InputNonSignerLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerErrors::InputNonSignerLengthMismatch,
                            )
                    }
                    InputNonSignerLengthMismatch
                },
                {
                    fn InvalidBLSPairingKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::InvalidBLSPairingKey)
                    }
                    InvalidBLSPairingKey
                },
                {
                    fn OnlySlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <OnlySlasher as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::OnlySlasher)
                    }
                    OnlySlasher
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::BytesArrayNotOrdered)
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn OnlyRegistryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::OnlyRegistryCoordinator)
                    }
                    OnlyRegistryCoordinator
                },
                {
                    fn OnlyRewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::OnlyRewardsInitiator)
                    }
                    OnlyRewardsInitiator
                },
                {
                    fn InvalidBLSSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InvalidBLSSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::InvalidBLSSignature)
                    }
                    InvalidBLSSignature
                },
                {
                    fn StaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <StaleStakesForbidden as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::StaleStakesForbidden)
                    }
                    StaleStakesForbidden
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn ECAddFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn OnlyRegistryCoordinatorOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerErrors::OnlyRegistryCoordinatorOwner,
                            )
                    }
                    OnlyRegistryCoordinatorOwner
                },
                {
                    fn InvalidQuorumApkHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::InvalidQuorumApkHash)
                    }
                    InvalidQuorumApkHash
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::BytesArrayLengthTooLong)
                    }
                    BytesArrayLengthTooLong
                },
                {
                    fn DelayPeriodNotPassed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::DelayPeriodNotPassed)
                    }
                    DelayPeriodNotPassed
                },
                {
                    fn NonSignerPubkeysNotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::NonSignerPubkeysNotSorted)
                    }
                    NonSignerPubkeysNotSorted
                },
                {
                    fn ScalarTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <ScalarTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockAvsServiceManagerErrors::ScalarTooLarge)
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
                Self::DelayPeriodNotPassed(inner) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::OnlyRegistryCoordinator(inner) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyRewardsInitiator(inner) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlySlasher(inner) => {
                    <OnlySlasher as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyStakeRegistry(inner) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::DelayPeriodNotPassed(inner) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OnlyRegistryCoordinator(inner) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyRewardsInitiator(inner) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlySlasher(inner) => {
                    <OnlySlasher as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyStakeRegistry(inner) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`MockAvsServiceManager`](self) events.
    pub enum MockAvsServiceManagerEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        RewardsInitiatorUpdated(RewardsInitiatorUpdated),
        #[allow(missing_docs)]
        StaleStakesForbiddenUpdate(StaleStakesForbiddenUpdate),
    }
    #[automatically_derived]
    impl MockAvsServiceManagerEvents {
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
                225u8,
                28u8,
                221u8,
                241u8,
                129u8,
                106u8,
                67u8,
                49u8,
                140u8,
                161u8,
                117u8,
                187u8,
                197u8,
                44u8,
                208u8,
                24u8,
                84u8,
                54u8,
                233u8,
                203u8,
                234u8,
                215u8,
                200u8,
                58u8,
                204u8,
                84u8,
                167u8,
                62u8,
                70u8,
                23u8,
                23u8,
                227u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for MockAvsServiceManagerEvents {
        const NAME: &'static str = "MockAvsServiceManagerEvents";
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
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsInitiatorUpdated)
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
    impl alloy_sol_types::private::IntoLogData for MockAvsServiceManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`MockAvsServiceManager`](self) contract instance.

See the [wrapper's documentation](`MockAvsServiceManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> MockAvsServiceManagerInstance<T, P, N> {
        MockAvsServiceManagerInstance::<T, P, N>::new(address, provider)
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
        _avsDirectory: alloy::sol_types::private::Address,
        _rewardsCoordinator: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<MockAvsServiceManagerInstance<T, P, N>>,
    > {
        MockAvsServiceManagerInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _registryCoordinator,
            _avsDirectory,
            _rewardsCoordinator,
            _allocationManager,
            _permissionController,
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
        _avsDirectory: alloy::sol_types::private::Address,
        _rewardsCoordinator: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        MockAvsServiceManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _registryCoordinator,
            _avsDirectory,
            _rewardsCoordinator,
            _allocationManager,
            _permissionController,
        )
    }
    /**A [`MockAvsServiceManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`MockAvsServiceManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct MockAvsServiceManagerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for MockAvsServiceManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MockAvsServiceManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > MockAvsServiceManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`MockAvsServiceManager`](self) contract instance.

See the [wrapper's documentation](`MockAvsServiceManagerInstance`) for more details.*/
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
            _avsDirectory: alloy::sol_types::private::Address,
            _rewardsCoordinator: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<MockAvsServiceManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _registryCoordinator,
                _avsDirectory,
                _rewardsCoordinator,
                _allocationManager,
                _permissionController,
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
            _avsDirectory: alloy::sol_types::private::Address,
            _rewardsCoordinator: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _registryCoordinator,
                            _avsDirectory,
                            _rewardsCoordinator,
                            _allocationManager,
                            _permissionController,
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
    impl<T, P: ::core::clone::Clone, N> MockAvsServiceManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> MockAvsServiceManagerInstance<T, P, N> {
            MockAvsServiceManagerInstance {
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
    > MockAvsServiceManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`SLASHER_PROPOSAL_DELAY`] function.
        pub fn SLASHER_PROPOSAL_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, SLASHER_PROPOSAL_DELAYCall, N> {
            self.call_builder(&SLASHER_PROPOSAL_DELAYCall {})
        }
        ///Creates a new call builder for the [`addPendingAdmin`] function.
        pub fn addPendingAdmin(
            &self,
            admin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, addPendingAdminCall, N> {
            self.call_builder(&addPendingAdminCall { admin })
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
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
        ///Creates a new call builder for the [`createAVSRewardsSubmission`] function.
        pub fn createAVSRewardsSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createAVSRewardsSubmissionCall, N> {
            self.call_builder(
                &createAVSRewardsSubmissionCall {
                    rewardsSubmissions,
                },
            )
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`deregisterOperatorFromAVS`] function.
        pub fn deregisterOperatorFromAVS(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromAVSCall, N> {
            self.call_builder(
                &deregisterOperatorFromAVSCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`deregisterOperatorFromOperatorSets`] function.
        pub fn deregisterOperatorFromOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            deregisterOperatorFromOperatorSetsCall,
            N,
        > {
            self.call_builder(
                &deregisterOperatorFromOperatorSetsCall {
                    operator,
                    operatorSetIds,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorRestakedStrategies`] function.
        pub fn getOperatorRestakedStrategies(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getOperatorRestakedStrategiesCall,
            N,
        > {
            self.call_builder(
                &getOperatorRestakedStrategiesCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getRestakeableStrategies`] function.
        pub fn getRestakeableStrategies(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRestakeableStrategiesCall, N> {
            self.call_builder(&getRestakeableStrategiesCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _initialOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall { _initialOwner })
        }
        ///Creates a new call builder for the [`migrationFinalized`] function.
        pub fn migrationFinalized(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, migrationFinalizedCall, N> {
            self.call_builder(&migrationFinalizedCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`proposedSlasher`] function.
        pub fn proposedSlasher(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposedSlasherCall, N> {
            self.call_builder(&proposedSlasherCall {})
        }
        ///Creates a new call builder for the [`registerOperatorToAVS`] function.
        pub fn registerOperatorToAVS(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorToAVSCall, N> {
            self.call_builder(
                &registerOperatorToAVSCall {
                    operator,
                    operatorSignature,
                },
            )
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`removeAdmin`] function.
        pub fn removeAdmin(
            &self,
            admin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeAdminCall, N> {
            self.call_builder(&removeAdminCall { admin })
        }
        ///Creates a new call builder for the [`removeAppointee`] function.
        pub fn removeAppointee(
            &self,
            appointee: alloy::sol_types::private::Address,
            target: alloy::sol_types::private::Address,
            selector: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeAppointeeCall, N> {
            self.call_builder(
                &removeAppointeeCall {
                    appointee,
                    target,
                    selector,
                },
            )
        }
        ///Creates a new call builder for the [`removePendingAdmin`] function.
        pub fn removePendingAdmin(
            &self,
            pendingAdmin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, removePendingAdminCall, N> {
            self.call_builder(
                &removePendingAdminCall {
                    pendingAdmin,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`rewardsInitiator`] function.
        pub fn rewardsInitiator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, rewardsInitiatorCall, N> {
            self.call_builder(&rewardsInitiatorCall {})
        }
        ///Creates a new call builder for the [`setAppointee`] function.
        pub fn setAppointee(
            &self,
            appointee: alloy::sol_types::private::Address,
            target: alloy::sol_types::private::Address,
            selector: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAppointeeCall, N> {
            self.call_builder(
                &setAppointeeCall {
                    appointee,
                    target,
                    selector,
                },
            )
        }
        ///Creates a new call builder for the [`setRewardsInitiator`] function.
        pub fn setRewardsInitiator(
            &self,
            newRewardsInitiator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsInitiatorCall, N> {
            self.call_builder(
                &setRewardsInitiatorCall {
                    newRewardsInitiator,
                },
            )
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
        ///Creates a new call builder for the [`slasher`] function.
        pub fn slasher(&self) -> alloy_contract::SolCallBuilder<T, &P, slasherCall, N> {
            self.call_builder(&slasherCall {})
        }
        ///Creates a new call builder for the [`slasherProposalTimestamp`] function.
        pub fn slasherProposalTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, slasherProposalTimestampCall, N> {
            self.call_builder(&slasherProposalTimestampCall {})
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
        ///Creates a new call builder for the [`updateAVSMetadataURI`] function.
        pub fn updateAVSMetadataURI(
            &self,
            _metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(
                &updateAVSMetadataURICall {
                    _metadataURI,
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
    > MockAvsServiceManagerInstance<T, P, N> {
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
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`RewardsInitiatorUpdated`] event.
        pub fn RewardsInitiatorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsInitiatorUpdated, N> {
            self.event_filter::<RewardsInitiatorUpdated>()
        }
        ///Creates a new event filter for the [`StaleStakesForbiddenUpdate`] event.
        pub fn StaleStakesForbiddenUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StaleStakesForbiddenUpdate, N> {
            self.event_filter::<StaleStakesForbiddenUpdate>()
        }
    }
}
