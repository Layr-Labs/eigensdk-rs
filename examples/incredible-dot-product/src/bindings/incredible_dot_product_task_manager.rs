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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254Instance<P, N> {
        BN254Instance::<P, N>::new(address, provider)
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
    pub struct BN254Instance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for BN254Instance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BN254Instance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

        See the [wrapper's documentation](`BN254Instance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
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
    impl<P: ::core::clone::Clone, N> BN254Instance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254Instance<P, N> {
            BN254Instance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BN254Instance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BN254Instance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerStakesAndSignature {
        #[allow(missing_docs)]
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerPubkeys:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
        pub quorumApks:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerStakeIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
            <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
        impl ::core::convert::From<NonSignerStakesAndSignature> for UnderlyingRustTuple<'_> {
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
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonSignerStakesAndSignature {
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
        impl alloy_sol_types::private::SolTypeValue<Self> for NonSignerStakesAndSignature {
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
        impl alloy_sol_types::SolType for NonSignerStakesAndSignature {
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
        impl alloy_sol_types::SolStruct for NonSignerStakesAndSignature {
            const NAME: &'static str = "NonSignerStakesAndSignature";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "NonSignerStakesAndSignature(uint32[] nonSignerQuorumBitmapIndices,BN254.G1Point[] nonSignerPubkeys,BN254.G1Point[] quorumApks,G2Point apkG2,G1Point sigma,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(4);
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerStakeIndices,
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumStakeTotals {
        #[allow(missing_docs)]
        pub signedStakeForQuorum:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
        #[allow(missing_docs)]
        pub totalStakeForQuorum:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
        impl alloy_sol_types::SolType for QuorumStakeTotals {
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
        impl alloy_sol_types::SolStruct for QuorumStakeTotals {
            const NAME: &'static str = "QuorumStakeTotals";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumStakeTotals(uint96[] signedStakeForQuorum,uint96[] totalStakeForQuorum)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

    See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBLSSignatureCheckerTypesInstance<P, N> {
        IBLSSignatureCheckerTypesInstance::<P, N>::new(address, provider)
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
    pub struct IBLSSignatureCheckerTypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IBLSSignatureCheckerTypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBLSSignatureCheckerTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IBLSSignatureCheckerTypesInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

        See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
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
    impl<P: ::core::clone::Clone, N> IBLSSignatureCheckerTypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBLSSignatureCheckerTypesInstance<P, N> {
            IBLSSignatureCheckerTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IBLSSignatureCheckerTypesInstance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IBLSSignatureCheckerTypesInstance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library IIncredibleDotProductTaskManager {
    struct DotProductInput { uint256[] X; uint256[] Y; }
    struct Task { DotProductInput pointsToMultiply; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
    struct TaskResponse { uint32 referenceTaskIndex; uint256 result; }
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
pub mod IIncredibleDotProductTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct DotProductInput { uint256[] X; uint256[] Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DotProductInput {
        #[allow(missing_docs)]
        pub X: alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        #[allow(missing_docs)]
        pub Y: alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
        impl ::core::convert::From<DotProductInput> for UnderlyingRustTuple<'_> {
            fn from(value: DotProductInput) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DotProductInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DotProductInput {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DotProductInput {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
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
        impl alloy_sol_types::SolType for DotProductInput {
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
        impl alloy_sol_types::SolStruct for DotProductInput {
            const NAME: &'static str = "DotProductInput";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("DotProductInput(uint256[] X,uint256[] Y)")
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DotProductInput {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct Task { DotProductInput pointsToMultiply; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Task {
        #[allow(missing_docs)]
        pub pointsToMultiply: <DotProductInput as alloy::sol_types::SolType>::RustType,
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
            DotProductInput,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <DotProductInput as alloy::sol_types::SolType>::RustType,
            u32,
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<Task> for UnderlyingRustTuple<'_> {
            fn from(value: Task) -> Self {
                (
                    value.pointsToMultiply,
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
                    pointsToMultiply: tuple.0,
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
                    <DotProductInput as alloy_sol_types::SolType>::tokenize(&self.pointsToMultiply),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.taskCreatedBlock,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumThresholdPercentage,
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
        impl alloy_sol_types::SolType for Task {
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
        impl alloy_sol_types::SolStruct for Task {
            const NAME: &'static str = "Task";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Task(DotProductInput pointsToMultiply,uint32 taskCreatedBlock,bytes quorumNumbers,uint32 quorumThresholdPercentage)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<DotProductInput as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<DotProductInput as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <DotProductInput as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pointsToMultiply,
                        )
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
                    + <DotProductInput as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pointsToMultiply,
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <DotProductInput as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pointsToMultiply,
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct TaskResponse { uint32 referenceTaskIndex; uint256 result; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TaskResponse {
        #[allow(missing_docs)]
        pub referenceTaskIndex: u32,
        #[allow(missing_docs)]
        pub result: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::primitives::aliases::U256);
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTaskIndex,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.result,
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
        impl alloy_sol_types::SolType for TaskResponse {
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
        impl alloy_sol_types::SolStruct for TaskResponse {
            const NAME: &'static str = "TaskResponse";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TaskResponse(uint32 referenceTaskIndex,uint256 result)",
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
                            &self.referenceTaskIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
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
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.result,
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
                    &rust.referenceTaskIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.result,
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for TaskResponseMetadata {
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
        impl alloy_sol_types::SolStruct for TaskResponseMetadata {
            const NAME: &'static str = "TaskResponseMetadata";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TaskResponseMetadata(uint32 taskRespondedBlock,bytes32 hashOfNonSigners)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IIncredibleDotProductTaskManager`](self) contract instance.

    See the [wrapper's documentation](`IIncredibleDotProductTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IIncredibleDotProductTaskManagerInstance<P, N> {
        IIncredibleDotProductTaskManagerInstance::<P, N>::new(address, provider)
    }
    /**A [`IIncredibleDotProductTaskManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IIncredibleDotProductTaskManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IIncredibleDotProductTaskManagerInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IIncredibleDotProductTaskManagerInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IIncredibleDotProductTaskManagerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IIncredibleDotProductTaskManagerInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`IIncredibleDotProductTaskManager`](self) contract instance.

        See the [wrapper's documentation](`IIncredibleDotProductTaskManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
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
    impl<P: ::core::clone::Clone, N> IIncredibleDotProductTaskManagerInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IIncredibleDotProductTaskManagerInstance<P, N> {
            IIncredibleDotProductTaskManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IIncredibleDotProductTaskManagerInstance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IIncredibleDotProductTaskManagerInstance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        pub nonSignerStakeIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for CheckSignaturesIndices {
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
        impl alloy_sol_types::SolStruct for CheckSignaturesIndices {
            const NAME: &'static str = "CheckSignaturesIndices";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CheckSignaturesIndices(uint32[] nonSignerQuorumBitmapIndices,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerStakeIndices,
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for Operator {
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
        impl alloy_sol_types::SolStruct for Operator {
            const NAME: &'static str = "Operator";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Operator(address operator,bytes32 operatorId,uint96 stake)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

    See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OperatorStateRetrieverInstance<P, N> {
        OperatorStateRetrieverInstance::<P, N>::new(address, provider)
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
    pub struct OperatorStateRetrieverInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OperatorStateRetrieverInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OperatorStateRetrieverInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        OperatorStateRetrieverInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

        See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
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
    impl<P: ::core::clone::Clone, N> OperatorStateRetrieverInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OperatorStateRetrieverInstance<P, N> {
            OperatorStateRetrieverInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        OperatorStateRetrieverInstance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        OperatorStateRetrieverInstance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
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

library IIncredibleDotProductTaskManager {
    struct DotProductInput {
        uint256[] X;
        uint256[] Y;
    }
    struct Task {
        DotProductInput pointsToMultiply;
        uint32 taskCreatedBlock;
        bytes quorumNumbers;
        uint32 quorumThresholdPercentage;
    }
    struct TaskResponse {
        uint32 referenceTaskIndex;
        uint256 result;
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

interface IncredibleDotProductTaskManager {
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
    event NewTaskCreated(uint32 indexed taskIndex, IIncredibleDotProductTaskManager.Task task);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event StaleStakesForbiddenUpdate(bool value);
    event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger);
    event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger);
    event TaskCompleted(uint32 indexed taskIndex);
    event TaskResponded(IIncredibleDotProductTaskManager.TaskResponse taskResponse, IIncredibleDotProductTaskManager.TaskResponseMetadata taskResponseMetadata);
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
    function createNewTask(IIncredibleDotProductTaskManager.DotProductInput memory points, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
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
    function raiseAndResolveChallenge(IIncredibleDotProductTaskManager.Task memory task, IIncredibleDotProductTaskManager.TaskResponse memory taskResponse, IIncredibleDotProductTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
    function registryCoordinator() external view returns (address);
    function renounceOwnership() external;
    function respondToTask(IIncredibleDotProductTaskManager.Task memory task, IIncredibleDotProductTaskManager.TaskResponse memory taskResponse, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
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
        "name": "points",
        "type": "tuple",
        "internalType": "struct IIncredibleDotProductTaskManager.DotProductInput",
        "components": [
          {
            "name": "X",
            "type": "uint256[]",
            "internalType": "uint256[]"
          },
          {
            "name": "Y",
            "type": "uint256[]",
            "internalType": "uint256[]"
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
        "internalType": "struct IIncredibleDotProductTaskManager.Task",
        "components": [
          {
            "name": "pointsToMultiply",
            "type": "tuple",
            "internalType": "struct IIncredibleDotProductTaskManager.DotProductInput",
            "components": [
              {
                "name": "X",
                "type": "uint256[]",
                "internalType": "uint256[]"
              },
              {
                "name": "Y",
                "type": "uint256[]",
                "internalType": "uint256[]"
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
        "internalType": "struct IIncredibleDotProductTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "result",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "taskResponseMetadata",
        "type": "tuple",
        "internalType": "struct IIncredibleDotProductTaskManager.TaskResponseMetadata",
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
        "internalType": "struct IIncredibleDotProductTaskManager.Task",
        "components": [
          {
            "name": "pointsToMultiply",
            "type": "tuple",
            "internalType": "struct IIncredibleDotProductTaskManager.DotProductInput",
            "components": [
              {
                "name": "X",
                "type": "uint256[]",
                "internalType": "uint256[]"
              },
              {
                "name": "Y",
                "type": "uint256[]",
                "internalType": "uint256[]"
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
        "internalType": "struct IIncredibleDotProductTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "result",
            "type": "uint256",
            "internalType": "uint256"
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
        "internalType": "struct IIncredibleDotProductTaskManager.Task",
        "components": [
          {
            "name": "pointsToMultiply",
            "type": "tuple",
            "internalType": "struct IIncredibleDotProductTaskManager.DotProductInput",
            "components": [
              {
                "name": "X",
                "type": "uint256[]",
                "internalType": "uint256[]"
              },
              {
                "name": "Y",
                "type": "uint256[]",
                "internalType": "uint256[]"
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
        "internalType": "struct IIncredibleDotProductTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "result",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "taskResponseMetadata",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IIncredibleDotProductTaskManager.TaskResponseMetadata",
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
pub mod IncredibleDotProductTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61014080604052346101e6576060816152e480380380916100208285610291565b8339810103126101e65780516001600160a01b038116908181036101e65760208301516001600160a01b038116938482036101e657604001519363ffffffff851685036101e657156102825760805260a052604051636830483560e01b8152602081600481855afa9081156101f2575f9161023f575b5060c052604051632efa2ca360e11b815290602090829060049082905afa9081156101f2575f916101fd575b5060e05260c05160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa9081156101f2575f916101ac575b50610100526101205260405161501b90816102c982396080518181816104f301528181610f62015281816116ab0152611e1f015260a051818181610b840152818161127b015281816130cf015281816131b80152818161374a0152613f97015260c051818181611237015281816134ba0152613605015260e0518181816111f3015281816133f60152613eb8015261010051818181611d1b01526132cb0152610120518181816107cf01526118d40152f35b90506020813d6020116101ea575b816101c760209383610291565b810103126101e657516001600160a01b03811681036101e6575f6100fa565b5f80fd5b3d91506101ba565b6040513d5f823e3d90fd5b90506020813d602011610237575b8161021860209383610291565b810103126101e657516001600160a01b03811681036101e6575f6100c2565b3d915061020b565b90506020813d60201161027a575b8161025a60209383610291565b810103126101e657516001600160a01b03811681036101e6576004610096565b3d915061024d565b6339b190bb60e11b5f5260045ffd5b601f909101601f19168101906001600160401b038211908210176102b457604052565b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8063018630671461029a578063136439dd14610295578063171f1d5b146102905780631ad43189146101e6578063245a7bfc1461028b5780632cb223d5146102865780632d89f6fc1461028157806331b36bd91461027c5780633563b0d1146102775780633998fdd314610272578063416c7e5e1461026d5780634d2b57fe146102685780634f739f7414610263578063595c6a671461025e5780635a2d7f02146102595780635ac86ab7146102545780635c1556621461024f5780635c975abb1461024a5780635decc3f5146102455780635df4594614610240578063683048351461023b5780636d14a987146102365780636efb463614610231578063715018a61461022c57806372d18e8d1461021d5780637afa1eed14610227578063886f1195146102225780638b00ce7c1461021d5780638da5cb5b146102185780639b290e9814610213578063b490bb411461020e578063b98d090814610209578063c9b1479714610204578063ca8aa7c7146101ff578063cc2a9a5b146101fa578063cefdc1d4146101f5578063df5cf723146101f0578063f2fde38b146101eb578063f5c9899d146101e6578063f63c5bab146101e15763fabc1cbc146101dc575f80fd5b611df6565b611ddb565b6107b3565b611d4a565b611d06565b611bc2565b611a82565b611a5a565b6117cb565b6117a9565b611738565b611702565b6116da565b61164b565b611696565b61166e565b6115f0565b611543565b611266565b611222565b6111de565b6111a0565b611183565b61100a565b610fd7565b610faa565b610f37565b610e90565b610cb2565b610b52565b610b20565b610aa6565b6108fc565b610854565b61081b565b6107f3565b610741565b6104c3565b61030a565b60409060231901126102b057602490565b5f80fd5b908160409103126102b05790565b63ffffffff8116036102b057565b35906102db826102c2565b565b9181601f840112156102b0578235916001600160401b0383116102b057602083818601950101116102b057565b346102b05760603660031901126102b0576004356001600160401b0381116102b05761033a9036906004016102b4565b60243590610347826102c2565b6044356001600160401b0381116102b0576103669036906004016102dd565b60ce5491939092916001600160a01b03163303610474576104729361045c936103ba6103c19361039f610397611ef1565b963690611f1c565b86524363ffffffff16602087015263ffffffff166060860152565b36916109c9565b604082015260405160208101906103ea816103dc8585611fa5565b03601f1981018352826105d0565b5190206104136103ff60c95463ffffffff1690565b63ffffffff165f5260ca60205260405f2090565b5560c95463ffffffff16907ff3d0298607cbbde38baba3c3915d277f1ffd80a384095a02a4a4d7c082ae6cd96040518061045463ffffffff86169482611fa5565b0390a261202b565b63ffffffff1663ffffffff1960c954161760c955565b005b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b346102b05760203660031901126102b05760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561057c5761047292610539915f9161054d575b5061209a565b610548606654828116146120b0565b614774565b61056f915060203d602011610575575b61056781836105d0565b81019061207a565b5f610533565b503d61055d565b61208f565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176105b057604052565b610581565b608081019081106001600160401b038211176105b057604052565b90601f801991011681019081106001600160401b038211176105b057604052565b604051906102db610100836105d0565b604051906102db6040836105d0565b604051906102db6060836105d0565b604051906102db60a0836105d0565b906102db60405192836105d0565b60409060e31901126102b0576040519061065582610595565b60e4358252610104356020830152565b91908260409103126102b05760405161067d81610595565b6020808294803584520135910152565b9080601f830112156102b057604051916106a86040846105d0565b8290604081019283116102b057905b8282106106c45750505090565b81358152602091820191016106b7565b9060806063198301126102b0576040516106ed81610595565b602061070882946106ff81606461068d565b845260a461068d565b910152565b91906080838203126102b05760206107086040519261072b84610595565b60408496610739838261068d565b86520161068d565b346102b0576101203660031901126102b05760043560403660231901126102b057610799604091825161077381610595565b60243581526044356020820152610789366106d4565b906107933661063c565b92612104565b8251911515825215156020820152f35b5f9103126102b057565b346102b0575f3660031901126102b057602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346102b0575f3660031901126102b05760cd546040516001600160a01b039091168152602090f35b346102b05760203660031901126102b05763ffffffff60043561083d816102c2565b165f5260cb602052602060405f2054604051908152f35b346102b05760203660031901126102b05763ffffffff600435610876816102c2565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b038116036102b057565b6001600160401b0381116105b05760051b60200190565b90602080835192838152019201905f5b8181106108d25750505090565b82518452602093840193909201916001016108c5565b9060206108f99281815201906108b5565b90565b346102b05760403660031901126102b0576004356109198161088d565b602435906001600160401b0382116102b057366023830112156102b0578160040135916109458361089e565b9261095360405194856105d0565b8084526024602085019160051b830101913683116102b057602401905b82821061099457610990610984868661226e565b604051918291826108e8565b0390f35b6020809183356109a38161088d565b815201910190610970565b6001600160401b0381116105b057601f01601f191660200190565b9291926109d5826109ae565b916109e360405193846105d0565b8294818452818301116102b0578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b838310610a2a57505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b808210610a6a5750505060208060019297019301930191939290610a1b565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610a4b565b346102b05760603660031901126102b057600435610ac38161088d565b6024356001600160401b0381116102b057366023820112156102b05761099091610afa610b0c9236906024816004013591016109c9565b60443591610b07836102c2565b6124ac565b6040519182916020835260208301906109ff565b346102b0575f3660031901126102b05760d1546040516001600160a01b039091168152602090f35b801515036102b057565b346102b05760203660031901126102b057600435610b6f81610b48565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c575f91610be4575b506001600160a01b03163303610bd55761047290614c07565b637070f3b160e11b5f5260045ffd5b610c06915060203d602011610c0c575b610bfe81836105d0565b810190612333565b5f610bbc565b503d610bf4565b9080601f830112156102b0578135610c2a8161089e565b92610c3860405194856105d0565b81845260208085019260051b8201019283116102b057602001905b828210610c605750505090565b8135815260209182019101610c53565b60206040818301928281528451809452019201905f5b818110610c935750505090565b82516001600160a01b0316845260209384019390920191600101610c86565b346102b05760403660031901126102b057600435610ccf8161088d565b6024356001600160401b0381116102b057610cee903690600401610c13565b610cf8815161220c565b916001600160a01b03165f5b8251811015610d9557806020610d1d610d3d938661224b565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561057c57600192610d71915f91610d77575b50610d62838861224b565b6001600160a01b039091169052565b01610d04565b610d8f915060203d8111610c0c57610bfe81836105d0565b5f610d57565b604051806109908682610c70565b90602080835192838152019201905f5b818110610dc05750505090565b825163ffffffff16845260209384019390920191600101610db3565b90602082526060610e2a610e15610dff84516080602088015260a0870190610da3565b6020850151868203601f19016040880152610da3565b6040840151858203601f190184870152610da3565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610e6357505050505090565b9091929394602080610e81600193601f198682030187528951610da3565b97019301930191939290610e54565b346102b05760803660031901126102b057600435610ead8161088d565b60243590610eba826102c2565b6044356001600160401b0381116102b057610ed99036906004016102dd565b91606435926001600160401b0384116102b057366023850112156102b0578360040135926001600160401b0384116102b0573660248560051b870101116102b057610990956024610f2b9601936129b1565b60405191829182610ddc565b346102b0575f3660031901126102b05760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561057c57610fa2915f9161054d575061209a565b610472614740565b346102b0575f3660031901126102b057602060405167016345785d8a00008152f35b60ff8116036102b057565b346102b05760203660031901126102b0576020600160ff600435610ffa81610fcc565b161b806066541614604051908152f35b346102b05760603660031901126102b0576004356110278161088d565b6024356001600160401b0381116102b057611046903690600401610c13565b60443591611053836102c2565b6040516361c8a12f60e11b8152906001600160a01b03165f828061107b868860048401612e19565b0381845afa91821561057c575f9261115f575b50611099835161220c565b935f5b8451811015611151576110af818661224b565b51906020836110cb6110c1848961224b565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa801561057c576001925f91611123575b50828060c01b031661111c828961224b565b520161109c565b611144915060203d811161114a575b61113c81836105d0565b810190612928565b5f61110a565b503d611132565b6040518061099088826108e8565b61117c9192503d805f833e61117481836105d0565b810190612806565b905f61108e565b346102b0575f3660031901126102b0576020606654604051908152f35b346102b05760203660031901126102b05763ffffffff6004356111c2816102c2565b165f5260cc602052602060ff60405f2054166040519015158152f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080601f830112156102b05781356112c18161089e565b926112cf60405194856105d0565b81845260208085019260051b8201019283116102b057602001905b8282106112f75750505090565b602080918335611306816102c2565b8152019101906112ea565b81601f820112156102b05780356113278161089e565b9261133560405194856105d0565b81845260208085019260061b840101928184116102b057602001915b83831061135f575050505090565b602060409161136e8486610665565b815201920191611351565b9080601f830112156102b05781356113908161089e565b9261139e60405194856105d0565b81845260208085019260051b820101918383116102b05760208201905b8382106113ca57505050505090565b81356001600160401b0381116102b0576020916113ec878480948801016112aa565b8152019101906113bb565b919091610180818403126102b05761140d6105f1565b9281356001600160401b0381116102b0578161142a9184016112aa565b845260208201356001600160401b0381116102b0578161144b918401611311565b602085015260408201356001600160401b0381116102b0578161146f918401611311565b6040850152611481816060840161070d565b60608501526114938160e08401610665565b60808501526101208201356001600160401b0381116102b057816114b89184016112aa565b60a08501526101408201356001600160401b0381116102b057816114dd9184016112aa565b60c08501526101608201356001600160401b0381116102b0576115009201611379565b60e0830152565b90602080835192838152019201905f5b8181106115245750505090565b82516001600160601b0316845260209384019390920191600101611517565b346102b05760803660031901126102b0576004356024356001600160401b0381116102b0576115769036906004016102dd565b9091604435611584816102c2565b606435926001600160401b0384116102b0576115e6946115ab6115b19536906004016113f7565b93612ff2565b6040519283926040845260206115d282516040808801526080870190611507565b910151848203603f19016060860152611507565b9060208301520390f35b346102b0575f3660031901126102b057611608614de3565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346102b0575f3660031901126102b057602063ffffffff60c95416604051908152f35b346102b0575f3660031901126102b05760ce546040516001600160a01b039091168152602090f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b0575f3660031901126102b0576033546040516001600160a01b039091168152602090f35b346102b0575f3660031901126102b05760cf546040516001600160a01b039091168152602090f35b908160809103126102b05790565b346102b05760c03660031901126102b0576004356001600160401b0381116102b05761176890369060040161172a565b6117713661029f565b9060403660631901126102b05760a4356001600160401b0381116102b057610472926117a36064923690600401611311565b92613d0a565b346102b0575f3660031901126102b057602060ff609754166040519015158152f35b346102b05760803660031901126102b0576004356001600160401b0381116102b0576117fb90369060040161172a565b6118043661029f565b906064356001600160401b0381116102b0576118249036906004016113f7565b60cd549092906001600160a01b03163303611a155761184760208394930161388f565b916119286118586040860186613b81565b92909461189661186a6060890161388f565b97604051611880816103dc6020820194856143b9565b51902061188f6103ff8861388f565b5414614471565b6118c06118b96118a58761388f565b63ffffffff165f5260cb60205260405f2090565b54156144e3565b8363ffffffff43169661190a6119026118f97f000000000000000000000000000000000000000000000000000000000000000086612060565b63ffffffff1690565b891115614544565b6040516020810190611920816103dc8b856145a6565b519020612ff2565b919060ff5f9616955b8281106119b3577f349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a868686611973611967610601565b63ffffffff9094168452565b6020830152604051602081019061198f816103dc86868661464a565b51902061199e6118a58361388f565b556119ae6040519283928361464a565b0390a1005b80611a0f6119eb6119e66119da6119cd600196885161224b565b516001600160601b031690565b6001600160601b031690565b613ab7565b611a086119da8b611a036119cd8760208b015161224b565b6145b6565b11156145d9565b01611931565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b346102b0575f3660031901126102b05760d0546040516001600160a01b039091168152602090f35b346102b05760c03660031901126102b057600435611a9f8161088d565b611b1f602435611aae8161088d565b604435611aba8161088d565b606435611ac68161088d565b60843591611ad38361088d565b60a43593611ae08561088d565b5f5496611b0560ff60088a901c16158099819a611b9d575b8115611b7d575b50614674565b87611b16600160ff195f5416175f55565b611b66576146d7565b611b2557005b611b3361ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016119ae565b611b7861010061ff00195f5416175f55565b6146d7565b303b15915081611b8f575b505f611aff565b60ff1660011490505f611b88565b600160ff8216109150611af8565b6040906108f99392815281602082015201906109ff565b346102b05760603660031901126102b057600435611bdf8161088d565b602435604435611bee816102c2565b611c2f611bf96121ea565b9280611c048561223e565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401612e19565b0381875afa93841561057c5783611c596118f96110c1611c8e986020975f91611cec575b5061223e565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561057c57611cbd925f91611ccd575b506001600160c01b031692611cb784614e83565b906124ac565b9061099060405192839283611bab565b611ce6915060203d60201161114a5761113c81836105d0565b5f611ca3565b611d0091503d805f833e61117481836105d0565b5f611c53565b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b05760203660031901126102b057600435611d678161088d565b611d6f614de3565b6001600160a01b03811615611d875761047290614e3b565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346102b0575f3660031901126102b057602060405160648152f35b346102b05760203660031901126102b05760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c575f91611eb9575b506001600160a01b03163303611eaa57611e786066541982198116146120b0565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b611ed2915060203d602011610c0c57610bfe81836105d0565b5f611e57565b60405190611ee582610595565b60606020838281520152565b60405190611efe826105b5565b5f606083611f0a611ed8565b81528260208201528160408201520152565b91906040838203126102b05760405190611f3582610595565b819380356001600160401b0381116102b05782611f53918301610c13565b83526020810135916001600160401b0383116102b0576020926107089201610c13565b6108f9903690611f1c565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b60208152608063ffffffff606061200d611fec86518560208801526020611fd88251604060a08b015260e08a01906108b5565b910151878203609f190160c08901526108b5565b8360208801511660408701526040870151601f198783030184880152611f81565b9401511691015290565b634e487b7160e01b5f52601160045260245ffd5b63ffffffff60019116019063ffffffff821161204357565b612017565b63ffffffff60649116019063ffffffff821161204357565b9063ffffffff8091169116019063ffffffff821161204357565b908160209103126102b057516108f981610b48565b6040513d5f823e3d90fd5b156120a157565b631d77d47760e21b5f5260045ffd5b156120b757565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b9060028110156120eb5760051b0190565b6120c6565b634e487b7160e01b5f52601260045260245ffd5b6121e06121bd6121e6956121b76121b085875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e084015261010083015261218781610120840103601f1981018352826105d0565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b80966147ea565b90614830565b926121b76121d26121cc614892565b94614989565b916121db614aa5565b6147ea565b91614ad9565b9091565b604080519091906121fb83826105d0565b6001815291601f1901366020840137565b906122168261089e565b61222360405191826105d0565b8281528092612234601f199161089e565b0190602036910137565b8051156120eb5760200190565b80518210156120eb5760209160051b010190565b908160209103126102b0575190565b91909161227b835161220c565b925f5b815181101561232e578060206122a761229a6122d0948661224b565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa801561057c576001925f91612300575b506122f9828861224b565b520161227e565b612321915060203d8111612327575b61231981836105d0565b81019061225f565b5f6122ee565b503d61230f565b505050565b908160209103126102b057516108f98161088d565b906123528261089e565b61235f60405191826105d0565b82815260208193612372601f199161089e565b0191015f5b82811061238357505050565b606082820152602001612377565b9081518110156120eb570160200190565b6020818303126102b0578051906001600160401b0382116102b057019080601f830112156102b05781516123d58161089e565b926123e360405194856105d0565b81845260208085019260051b8201019283116102b057602001905b82821061240b5750505090565b81518152602091820191016123fe565b906124258261089e565b61243260405191826105d0565b8281528092612443601f199161089e565b015f5b81811061245257505050565b6040519060608201918083106001600160401b038411176105b0576020926040525f81525f838201525f604082015282828601015201612446565b908160209103126102b057516001600160601b03811681036102b05790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa94851561057c575f956127c1575b50604051634f4c91e160e11b815294602086600481855afa91821561057c576004965f9361279f575b5060209060405197888092632efa2ca360e11b82525afa95861561057c575f9661277e575b5061253a8593929551612348565b945f935b80518510156127745761256b6125656125578784612391565b516001600160f81b03191690565b60f81c90565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa93841561057c575f94612750575b506125bb845161241b565b6125c5888b61224b565b526125d0878a61224b565b505f5b845181101561273f578060206125ec61260e938861224b565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa91821561057c575f9261271f575b50612634818761224b565b518a60208a612643858b61224b565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa93841561057c576126d68c8f6126d16001986126e89789975f926126ef575b506126bc6126ad610610565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b61224b565b51906126e2838361224b565b5261224b565b50016125d3565b61271191925060203d8111612718575b61270981836105d0565b81019061248d565b905f6126a1565b503d6126ff565b61273891925060203d8111610c0c57610bfe81836105d0565b905f612629565b50600190960195909450915061253e565b61276d9194503d805f833e61276581836105d0565b8101906123a2565b925f6125b0565b5050509350505090565b61279891965060203d602011610c0c57610bfe81836105d0565b945f61252c565b60209193506127ba90823d8411610c0c57610bfe81836105d0565b9290612507565b6127db91955060203d602011610c0c57610bfe81836105d0565b935f6124de565b604051906127ef826105b5565b606080838181528160208201528160408201520152565b6020818303126102b0578051906001600160401b0382116102b057019080601f830112156102b05781516128398161089e565b9261284760405194856105d0565b81845260208085019260051b8201019283116102b057602001905b82821061286f5750505090565b60208091835161287e816102c2565b815201910190612862565b81835290916001600160fb1b0383116102b05760209260051b809284830137010190565b60409063ffffffff6108f995931681528160208201520191612889565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6108f9959316815281602082015201916128ca565b60ff1660ff81146120435760010190565b91908110156120eb5760051b0190565b908160209103126102b057516001600160c01b03811681036102b05790565b1561294e57565b6325ec6c1f60e01b5f5260045ffd5b908210156120eb570190565b908160209103126102b057516108f9816102c2565b5f1981146120435760010190565b916129aa60209263ffffffff929695966040865260408601916128ca565b9416910152565b95939495929091926129c16127e2565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa94851561057c575f95612df8575b506129fe6127e2565b946040516361c8a12f60e11b81525f8180612a1e8d8d8b600485016128ad565b0381885afa90811561057c575f91612dde575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f8180612a6185878b600485016128ea565b0381875afa90811561057c575f91612dc4575b506040870152612a8381612348565b9860608701998a525f5b60ff811683811015612d0f57885f612ab6838f612aa98861220c565b9051906126e2838361224b565b505f8a868f5b818410612b39575050505090508c612ad38261220c565b915f5b818110612b0057505091612af591612afb949351906126e2838361224b565b50612907565b612a8d565b80612b33612b1e6110c1600194612b188a895161224b565b5161224b565b612b28838861224b565b9063ffffffff169052565b01612ad6565b6110c184612b4e8160209695612b5695612918565b35975161224b565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561057c57888f888a918f94612bfb6001612bee81938d809d5f92612ce3575b50612565612bca612bd892612bc3878060c01b0386161515612947565b8b8d61295d565b356001600160f81b03191690565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14612c17575b5050505050600191925001908a918a868f612abc565b8597612c3993612c32602097999861256595612bca95612918565b359561295d565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa90811561057c578f612c9790612c9c9383886001975f93612cab575b50612b1890612b2893945161224b565b61297e565b905082918a888f888a91612c01565b612b28935090612cd4612b189260203d8111612cdc575b612ccc81836105d0565b810190612969565b935090612c87565b503d612cc2565b612bd8919250612bca612d066125659260203d811161114a5761113c81836105d0565b93925050612ba6565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa90811561057c57612d65945f948593612da3575b5060405163354952a360e21b8152958694859384936004850161298c565b03916001600160a01b03165afa90811561057c575f91612d89575b50602082015290565b612d9d91503d805f833e61117481836105d0565b5f612d80565b612dbd91935060203d602011610c0c57610bfe81836105d0565b915f612d47565b612dd891503d805f833e61117481836105d0565b5f612a74565b612df291503d805f833e61117481836105d0565b5f612a31565b612e1291955060203d602011610c0c57610bfe81836105d0565b935f6129f5565b60409063ffffffff6108f9949316815281602082015201906108b5565b15612e3d57565b62f8202d60e51b5f5260045ffd5b15612e5257565b6343714afd60e01b5f5260045ffd5b15612e6857565b635f832f4160e01b5f5260045ffd5b15612e7e57565b634b874f4560e01b5f5260045ffd5b908160209103126102b057516108f981610fcc565b5f1981019190821161204357565b15612eb757565b633fdc650560e21b5f5260045ffd5b906001820180921161204357565b906002820180921161204357565b906003820180921161204357565b906004820180921161204357565b906005820180921161204357565b9190820180921161204357565b15612f2057565b63affc5edb60e01b5f5260045ffd5b908160209103126102b0575167ffffffffffffffff19811681036102b05790565b15612f5757565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b03821161204357565b15612f8d57565b6367988d3360e01b5f5260045ffd5b15612fa357565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b818110612fdc5750505090565b8251845260209384019390920191600101612fcf565b949392909193613000611ed8565b5061300c851515612e36565b604084015151851480613881575b80613873575b80613865575b61302f90612e4b565b61304160208501515185515114612e61565b61305863ffffffff431663ffffffff841610612e77565b613060610601565b5f81525f602082015292613072611ed8565b61307b8761220c565b60208201526130898761220c565b8152613093611ed8565b926130a260208801515161220c565b84526130b260208801515161220c565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561057c5761311b915f91613836575b50613116368b876109c9565b614c45565b985f965b6020890151805189101561328d576020886131826110c18c61317a8f96868e61315f61314c86809561224b565b5180515f526020015160205260405f2090565b61316c848484015161224b565b528261325a575b015161224b565b51955161224b565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa91821561057c576121b78a61322f8f6132288f8460208f9261321f936132178460019e6132359e5f9161323d575b508f8060c01b0316925161224b565b52015161224b565b51938d5161224b565b5116614c70565b90614ca1565b97019661311f565b6132549150863d811161114a5761113c81836105d0565b5f613208565b61328861326a848484015161224b565b516132818484015161327b87612ea2565b9061224b565b5110612eb0565b613173565b509095979496506132a2919893929950614d5e565b916132af60975460ff1690565b90811561382e576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c575f9161380f575b5091905b5f925b8184106133605750505050509261334761334261333b61335a95856103dc9860806060602099015192015192612104565b9190612f86565b612f9c565b0151604051928391602083019586612fb2565b51902090565b92989596909399919794878b888c888d613709575b6110c18260a06133b5612565612bca846133bd976133af6133a161314c8f9c604060209f9e015161224b565b67ffffffffffffffff191690565b9b61295d565b97015161224b565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561057c576134816110c18f958f906134798f978f96848f61347360c09661346c848f60209f90613173612bca996040936125659c5f916136db575b5067ffffffffffffffff19918216911614612f50565b5190614830565b9c61295d565b96015161224b565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561057c5761350e918c8f925f926136b7575b5060206135009293015161224b565b906001600160601b03169052565b61352e8c6135008c6135276119cd82602086015161224b565b925161224b565b5f985f5b60208a01515181101561369e578b8d61357089613563612565612bca868f8961355b915161224b565b51948761295d565b60ff161c60019081161490565b61357f575b5050600101613532565b8a8a613601859f948f9686612b188f9360e06135b86110c19560206135b0612565612bca839f6135c19c899161295d565b9a015161224b565b519b015161224b565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c578f61366d908f936001959486955f92613678575b50613667613500929351936136626119cd848761224b565b612f66565b9261224b565b019a90508b8d613575565b61350092506136976136679160203d81116127185761270981836105d0565b925061364a565b5093919796996001919699509a94929a0192919061330a565b61350092506136d4602091823d81116127185761270981836105d0565b92506134f1565b60206136fc92503d8111613702575b6136f481836105d0565b810190612f2f565b5f613456565b503d6136ea565b6137469450613723925061256591612bca9160209561295d565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561057c576020896133bd8f938f60a08f97612565612bca8f8f906133af6133a161314c8f60408b96918f88936110c19f6137ca906137d0936133b59f5f926137e6575b5063ffffffff809116931690612f0c565b11612f19565b5050505050509750505050505092935050613375565b602063ffffffff9293508291613807913d81116123275761231981836105d0565b9291506137b9565b613828915060203d602011612cdc57612ccc81836105d0565b5f613303565b5f9190613307565b613858915060203d60201161385e575b61385081836105d0565b810190612e8d565b5f61310a565b503d613846565b5060e0840151518514613026565b5060c0840151518514613020565b5060a084015151851461301a565b356108f9816102c2565b903590603e19813603018212156102b0570190565b156138b557565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b6020809163ffffffff8135613918816102c2565b1684520135910152565b9092916020606091613938846080810197613904565b63ffffffff8135613948816102c2565b1660408501520135910152565b1561395c57565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156139ce57565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b15613a4c57565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b9060648202918083046064149015171561204357565b9060068202918083046006149015171561204357565b8181029291811591840414171561204357565b15613afd57565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b903590601e19813603018212156102b057018035906001600160401b0382116102b0576020019181360383136102b057565b6020818303126102b0578051906001600160401b0382116102b057019080601f830112156102b0578151613be68161089e565b92613bf460405194856105d0565b81845260208085019260051b8201019283116102b057602001905b828210613c1c5750505090565b602080918351613c2b8161088d565b815201910190613c0f565b60405190613c456040836105d0565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b818110613ceb575050506080613cd66108f994956060850151601f1985830301848601526108b5565b9201519060a0601f1982850301910152611f81565b82516001600160a01b0316875260209687019690920191600101613cad565b9193929093613d188561388f565b93613d2b613d268580613899565b611f76565b94613d4f613d478263ffffffff165f5260cb60205260405f2090565b5415156138ae565b613d8b613d6a8263ffffffff165f5260cb60205260405f2090565b5488604051613d82816103dc89602083019586613922565b51902014613955565b613db6613db0613da98363ffffffff165f5260cc60205260405f2090565b5460ff1690565b156139c7565b613ddb613dcd6118f9613dc88661388f565b612048565b63ffffffff43161115613a45565b5f935f9560208801955b88518051891015613e2457600191613e16613e038b613e1c9461224b565b51613e0f8c8c5161224b565b5190613ae3565b90612f0c565b970196613de5565b50929750945094506020600192970135141461432357613e44835161220c565b935f5b8451811015613e715780613e6061314c6001938861224b565b613e6a828961224b565b5201613e47565b509092939194613eab60208701946020613e8a8761388f565b604051613e9f816103dc8a8683019586612fb2565b51902091013514613af6565b613eb5855161220c565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b8751811015613f6557806020613efc613f1c938961224b565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa91821561057c57600192613f41915f91613f47575b50610d62838d61224b565b01613ee3565b613f5f915060203d8111610c0c57610bfe81836105d0565b5f613f36565b5092969550925092613fc4613f8d613f956040860194613f858688613b81565b93909161388f565b9236916109c9565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166124ac565b945f915b86518310156142c5575f979697955b613fe1848a61224b565b51518710156142b55797614022986020806140008a612b18898761224b565b510151604051809c81926308f6629d60e31b8352600483019190602083019252565b0381865afa998a1561057c575f9a614295575b506001985f5b85518110156142875761406061405461229a838961224b565b6001600160a01b031690565b6001600160a01b038d16146140775760010161403b565b5099909791985060015f5b151514614097575b5060010195979697613fd7565b98969786985f879593986140fc60ff6140d4612565612bca8c6140ce6141519f6140c860d15460018060a01b031690565b98613b81565b9061295d565b6140ee6140df610601565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d05461411390614054906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529098899190829081906044820190565b03915afa96871561057c575f97614263575b5061416e875161220c565b985f5b8a51811015614197578067016345785d8a00006141906001938e61224b565b5201614171565b509a9294966141d560ff6141bc612565612bca8f9d979f969e966140ce8e8e92613b81565b6141c76126ad61061f565b1663ffffffff166020860152565b604084015260608301526141e7613c36565b608083015260cf5461420390614054906001600160a01b031681565b803b156102b057604051636a669b4160e01b8152925f91849182908490829061422f9060048301613c66565b03925af191821561057c57600192614249575b509061408a565b806142575f61425d936105d0565b806107a9565b5f614242565b6142809197503d805f833e61427881836105d0565b810190613bb3565b955f614163565b509990979198600190614082565b6142ae919a5060203d8111610c0c57610bfe81836105d0565b985f614035565b9697969550600190920191613fc8565b50505050509190506142f56142e88263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b50505063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b9035601e19823603018112156102b05701602081359101916001600160401b0382116102b0578160051b360383136102b057565b9035601e19823603018112156102b05701602081359101916001600160401b0382116102b05781360383136102b057565b60208152813590603e19833603018212156102b0576080614466606061445f614425876108f997018560208801526144136144086143f78380614354565b604060a08c015260e08b0191612889565b916020810190614354565b888303609f190160c08a015290612889565b61444161443460208a016102d0565b63ffffffff166040880152565b61444e6040890189614388565b878303601f190185890152906128ca565b95016102d0565b63ffffffff16910152565b1561447857565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156144ea57565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b1561454b57565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6040810192916102db9190613904565b906001600160601b03809116911602906001600160601b03821691820361204357565b156145e057565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091614660846080810197613904565b63ffffffff81511660408501520151910152565b1561467b57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6146e090614e3b565b60cd80546001600160a01b03199081166001600160a01b039384161790915560ce805482169383169390931790925560d0805483169382169390931790925560cf805482169383169390931790925560d180549092169216919091179055565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b604051906147b382610595565b5f6020838281520152565b604051906101806147cf81846105d0565b368337565b604051906147e36020836105d0565b6020368337565b919060409060606147f96147a6565b948592602085519261480b85856105d0565b8436853780518452015160208301528482015260076107cf195a01fa1561482e57565bfe5b6020929160806040926148416147a6565b9586938186519361485286866105d0565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa801561482e571561488357565b63d4b68fd760e01b5f5260045ffd5b60405161489e81610595565b60409081516148ad83826105d0565b82368237815260208251916148c284846105d0565b83368437015280516148d482826105d0565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061492a83836105d0565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d602083015261497f835193846105d0565b8252602082015290565b5f516020614fc65f395f51905f52906149a06147a6565b505f919006602060c0835b614aa0575f935f516020614fc65f395f51905f52600381868181800909086040516149d685826105d0565b843682378481856040516149ea82826105d0565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614fc65f395f51905f5260a082015260056107cf195a01fa801561482e57614a5490614faf565b5191614aa0575f516020614fc65f395f51905f5282800914614a8b57505f516020614fc65f395f51905f5260015f940892936149ab565b92935050614a97610601565b92835282015290565b6120f0565b614aad6147a6565b50604051614aba81610595565b600181526002602082015290565b90600c8110156120eb5760051b0190565b93929091614ae7604061062e565b9485526020850152614af9604061062e565b9182526020820152614b096147be565b925f5b60028110614b3657505050602061018092614b256147d4565b93849160086201d4c0fa9151151590565b80614b42600192613acd565b614b4c82856120da565b5151614b588289614ac8565b526020614b6583866120da565b510151614b7a614b7483612ec6565b89614ac8565b52614b8582866120da565b515151614b94614b7483612ed4565b52614baa614ba283876120da565b515160200190565b51614bb7614b7483612ee2565b526020614bc483876120da565b51015151614bd4614b7483612ef0565b52614c00614bfa614bf36020614bea868a6120da565b51015160200190565b5192612efe565b88614ac8565b5201614b0c565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614c5360ff93614f37565b928392161b1115614c615790565b63ca95733360e01b5f5260045ffd5b805f915b614c7c575090565b5f1981018181116120435761ffff9116911661ffff8114612043576001019080614c74565b90614caa6147a6565b5061ffff811690610200821015614d4f5760018214614d4a57614ccb610601565b5f81525f602082015292906001905f925b61ffff8316851015614cf057505050505090565b600161ffff831660ff86161c811614614d2a575b6001614d20614d158360ff94614830565b9460011b61fffe1690565b9401169291614cdc565b946001614d20614d15614d3f8960ff95614830565b989350505050614d04565b505090565b637fc4ea7d60e11b5f5260045ffd5b614d666147a6565b50805190811580614dd7575b15614d93575050604051614d876040826105d0565b5f81525f602082015290565b60205f516020614fc65f395f51905f52910151065f516020614fc65f395f51905f52035f516020614fc65f395f51905f528111612043576040519161497f83610595565b50602081015115614d72565b6033546001600160a01b03163303614df757565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff614e8f82614c70565b16614e99816109ae565b90614ea760405192836105d0565b808252614eb6601f19916109ae565b013660208301375f5f5b8251821080614f16575b15614f0f576001811b8416614ee8575b614ee39061297e565b614ec0565b906001614ee39160ff60f81b8460f81b165f1a614f058287612391565b5301919050614eda565b5050905090565b506101008110614eca565b15614f2857565b631019106960e31b5f5260045ffd5b90610100825111614fa057815115614f9b57602082015160019060f81c81901b5b8351821015614f9657600190614f81614f776125656125578689612391565b60ff600191161b90565b90614f8d818311614f21565b17910190614f58565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b15614fb657565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212200a93af99721bddafed0252f3397a1abba40b907357c210b7c4ee65b3a430e34564736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01@\x80`@R4a\x01\xE6W``\x81aR\xE4\x808\x03\x80\x91a\0 \x82\x85a\x02\x91V[\x839\x81\x01\x03\x12a\x01\xE6W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\x01\xE6W` \x83\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x84\x82\x03a\x01\xE6W`@\x01Q\x93c\xFF\xFF\xFF\xFF\x85\x16\x85\x03a\x01\xE6W\x15a\x02\x82W`\x80R`\xA0R`@Qch0H5`\xE0\x1B\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x02?W[P`\xC0R`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x01\xFDW[P`\xE0R`\xC0Q`@Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x01\xACW[Pa\x01\0Ra\x01 R`@QaP\x1B\x90\x81a\x02\xC9\x829`\x80Q\x81\x81\x81a\x04\xF3\x01R\x81\x81a\x0Fb\x01R\x81\x81a\x16\xAB\x01Ra\x1E\x1F\x01R`\xA0Q\x81\x81\x81a\x0B\x84\x01R\x81\x81a\x12{\x01R\x81\x81a0\xCF\x01R\x81\x81a1\xB8\x01R\x81\x81a7J\x01Ra?\x97\x01R`\xC0Q\x81\x81\x81a\x127\x01R\x81\x81a4\xBA\x01Ra6\x05\x01R`\xE0Q\x81\x81\x81a\x11\xF3\x01R\x81\x81a3\xF6\x01Ra>\xB8\x01Ra\x01\0Q\x81\x81\x81a\x1D\x1B\x01Ra2\xCB\x01Ra\x01 Q\x81\x81\x81a\x07\xCF\x01Ra\x18\xD4\x01R\xF3[\x90P` \x81=` \x11a\x01\xEAW[\x81a\x01\xC7` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W_a\0\xFAV[_\x80\xFD[=\x91Pa\x01\xBAV[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x027W[\x81a\x02\x18` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W_a\0\xC2V[=\x91Pa\x02\x0BV[\x90P` \x81=` \x11a\x02zW[\x81a\x02Z` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W`\x04a\0\x96V[=\x91Pa\x02MV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x02\xB4W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x01\x860g\x14a\x02\x9AW\x80c\x13d9\xDD\x14a\x02\x95W\x80c\x17\x1F\x1D[\x14a\x02\x90W\x80c\x1A\xD41\x89\x14a\x01\xE6W\x80c$Z{\xFC\x14a\x02\x8BW\x80c,\xB2#\xD5\x14a\x02\x86W\x80c-\x89\xF6\xFC\x14a\x02\x81W\x80c1\xB3k\xD9\x14a\x02|W\x80c5c\xB0\xD1\x14a\x02wW\x80c9\x98\xFD\xD3\x14a\x02rW\x80cAl~^\x14a\x02mW\x80cM+W\xFE\x14a\x02hW\x80cOs\x9Ft\x14a\x02cW\x80cY\\jg\x14a\x02^W\x80cZ-\x7F\x02\x14a\x02YW\x80cZ\xC8j\xB7\x14a\x02TW\x80c\\\x15Vb\x14a\x02OW\x80c\\\x97Z\xBB\x14a\x02JW\x80c]\xEC\xC3\xF5\x14a\x02EW\x80c]\xF4YF\x14a\x02@W\x80ch0H5\x14a\x02;W\x80cm\x14\xA9\x87\x14a\x026W\x80cn\xFBF6\x14a\x021W\x80cqP\x18\xA6\x14a\x02,W\x80cr\xD1\x8E\x8D\x14a\x02\x1DW\x80cz\xFA\x1E\xED\x14a\x02'W\x80c\x88o\x11\x95\x14a\x02\"W\x80c\x8B\0\xCE|\x14a\x02\x1DW\x80c\x8D\xA5\xCB[\x14a\x02\x18W\x80c\x9B)\x0E\x98\x14a\x02\x13W\x80c\xB4\x90\xBBA\x14a\x02\x0EW\x80c\xB9\x8D\t\x08\x14a\x02\tW\x80c\xC9\xB1G\x97\x14a\x02\x04W\x80c\xCA\x8A\xA7\xC7\x14a\x01\xFFW\x80c\xCC*\x9A[\x14a\x01\xFAW\x80c\xCE\xFD\xC1\xD4\x14a\x01\xF5W\x80c\xDF\\\xF7#\x14a\x01\xF0W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xEBW\x80c\xF5\xC9\x89\x9D\x14a\x01\xE6W\x80c\xF6<[\xAB\x14a\x01\xE1Wc\xFA\xBC\x1C\xBC\x14a\x01\xDCW_\x80\xFD[a\x1D\xF6V[a\x1D\xDBV[a\x07\xB3V[a\x1DJV[a\x1D\x06V[a\x1B\xC2V[a\x1A\x82V[a\x1AZV[a\x17\xCBV[a\x17\xA9V[a\x178V[a\x17\x02V[a\x16\xDAV[a\x16KV[a\x16\x96V[a\x16nV[a\x15\xF0V[a\x15CV[a\x12fV[a\x12\"V[a\x11\xDEV[a\x11\xA0V[a\x11\x83V[a\x10\nV[a\x0F\xD7V[a\x0F\xAAV[a\x0F7V[a\x0E\x90V[a\x0C\xB2V[a\x0BRV[a\x0B V[a\n\xA6V[a\x08\xFCV[a\x08TV[a\x08\x1BV[a\x07\xF3V[a\x07AV[a\x04\xC3V[a\x03\nV[`@\x90`#\x19\x01\x12a\x02\xB0W`$\x90V[_\x80\xFD[\x90\x81`@\x91\x03\x12a\x02\xB0W\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x02\xB0WV[5\x90a\x02\xDB\x82a\x02\xC2V[V[\x91\x81`\x1F\x84\x01\x12\x15a\x02\xB0W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xB0W` \x83\x81\x86\x01\x95\x01\x01\x11a\x02\xB0WV[4a\x02\xB0W``6`\x03\x19\x01\x12a\x02\xB0W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x03:\x906\x90`\x04\x01a\x02\xB4V[`$5\x90a\x03G\x82a\x02\xC2V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x03f\x906\x90`\x04\x01a\x02\xDDV[`\xCET\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x04tWa\x04r\x93a\x04\\\x93a\x03\xBAa\x03\xC1\x93a\x03\x9Fa\x03\x97a\x1E\xF1V[\x966\x90a\x1F\x1CV[\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01Rc\xFF\xFF\xFF\xFF\x16``\x86\x01RV[6\x91a\t\xC9V[`@\x82\x01R`@Q` \x81\x01\x90a\x03\xEA\x81a\x03\xDC\x85\x85a\x1F\xA5V[\x03`\x1F\x19\x81\x01\x83R\x82a\x05\xD0V[Q\x90 a\x04\x13a\x03\xFF`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\xF3\xD0)\x86\x07\xCB\xBD\xE3\x8B\xAB\xA3\xC3\x91]'\x7F\x1F\xFD\x80\xA3\x84\tZ\x02\xA4\xA4\xD7\xC0\x82\xAEl\xD9`@Q\x80a\x04Tc\xFF\xFF\xFF\xFF\x86\x16\x94\x82a\x1F\xA5V[\x03\x90\xA2a +V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05|Wa\x04r\x92a\x059\x91_\x91a\x05MW[Pa \x9AV[a\x05H`fT\x82\x81\x16\x14a \xB0V[aGtV[a\x05o\x91P` =` \x11a\x05uW[a\x05g\x81\x83a\x05\xD0V[\x81\x01\x90a zV[_a\x053V[P=a\x05]V[a \x8FV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x05\xB0W`@RV[a\x05\x81V[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x05\xB0W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x05\xB0W`@RV[`@Q\x90a\x02\xDBa\x01\0\x83a\x05\xD0V[`@Q\x90a\x02\xDB`@\x83a\x05\xD0V[`@Q\x90a\x02\xDB``\x83a\x05\xD0V[`@Q\x90a\x02\xDB`\xA0\x83a\x05\xD0V[\x90a\x02\xDB`@Q\x92\x83a\x05\xD0V[`@\x90`\xE3\x19\x01\x12a\x02\xB0W`@Q\x90a\x06U\x82a\x05\x95V[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x02\xB0W`@Qa\x06}\x81a\x05\x95V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W`@Q\x91a\x06\xA8`@\x84a\x05\xD0V[\x82\x90`@\x81\x01\x92\x83\x11a\x02\xB0W\x90[\x82\x82\x10a\x06\xC4WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x06\xB7V[\x90`\x80`c\x19\x83\x01\x12a\x02\xB0W`@Qa\x06\xED\x81a\x05\x95V[` a\x07\x08\x82\x94a\x06\xFF\x81`da\x06\x8DV[\x84R`\xA4a\x06\x8DV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x02\xB0W` a\x07\x08`@Q\x92a\x07+\x84a\x05\x95V[`@\x84\x96a\x079\x83\x82a\x06\x8DV[\x86R\x01a\x06\x8DV[4a\x02\xB0Wa\x01 6`\x03\x19\x01\x12a\x02\xB0W`\x045`@6`#\x19\x01\x12a\x02\xB0Wa\x07\x99`@\x91\x82Qa\x07s\x81a\x05\x95V[`$5\x81R`D5` \x82\x01Ra\x07\x896a\x06\xD4V[\x90a\x07\x936a\x06<V[\x92a!\x04V[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x02\xB0WV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`\xCDT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0Wc\xFF\xFF\xFF\xFF`\x045a\x08=\x81a\x02\xC2V[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0Wc\xFF\xFF\xFF\xFF`\x045a\x08v\x81a\x02\xC2V[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\xB0WV[`\x01`\x01`@\x1B\x03\x81\x11a\x05\xB0W`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x08\xD2WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x08\xC5V[\x90` a\x08\xF9\x92\x81\x81R\x01\x90a\x08\xB5V[\x90V[4a\x02\xB0W`@6`\x03\x19\x01\x12a\x02\xB0W`\x045a\t\x19\x81a\x08\x8DV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W6`#\x83\x01\x12\x15a\x02\xB0W\x81`\x04\x015\x91a\tE\x83a\x08\x9EV[\x92a\tS`@Q\x94\x85a\x05\xD0V[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x02\xB0W`$\x01\x90[\x82\x82\x10a\t\x94Wa\t\x90a\t\x84\x86\x86a\"nV[`@Q\x91\x82\x91\x82a\x08\xE8V[\x03\x90\xF3[` \x80\x91\x835a\t\xA3\x81a\x08\x8DV[\x81R\x01\x91\x01\x90a\tpV[`\x01`\x01`@\x1B\x03\x81\x11a\x05\xB0W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\t\xD5\x82a\t\xAEV[\x91a\t\xE3`@Q\x93\x84a\x05\xD0V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02\xB0W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\n*WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\njWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\n\x1BV[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\nKV[4a\x02\xB0W``6`\x03\x19\x01\x12a\x02\xB0W`\x045a\n\xC3\x81a\x08\x8DV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W6`#\x82\x01\x12\x15a\x02\xB0Wa\t\x90\x91a\n\xFAa\x0B\x0C\x926\x90`$\x81`\x04\x015\x91\x01a\t\xC9V[`D5\x91a\x0B\x07\x83a\x02\xC2V[a$\xACV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\t\xFFV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x02\xB0WV[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0W`\x045a\x0Bo\x81a\x0BHV[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05|W_\x91a\x0B\xE4W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0B\xD5Wa\x04r\x90aL\x07V[cpp\xF3\xB1`\xE1\x1B_R`\x04_\xFD[a\x0C\x06\x91P` =` \x11a\x0C\x0CW[a\x0B\xFE\x81\x83a\x05\xD0V[\x81\x01\x90a#3V[_a\x0B\xBCV[P=a\x0B\xF4V[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x815a\x0C*\x81a\x08\x9EV[\x92a\x0C8`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xB0W` \x01\x90[\x82\x82\x10a\x0C`WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x0CSV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x0C\x93WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0C\x86V[4a\x02\xB0W`@6`\x03\x19\x01\x12a\x02\xB0W`\x045a\x0C\xCF\x81a\x08\x8DV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x0C\xEE\x906\x90`\x04\x01a\x0C\x13V[a\x0C\xF8\x81Qa\"\x0CV[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\r\x95W\x80` a\r\x1Da\r=\x93\x86a\"KV[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x05|W`\x01\x92a\rq\x91_\x91a\rwW[Pa\rb\x83\x88a\"KV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\r\x04V[a\r\x8F\x91P` =\x81\x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[_a\rWV[`@Q\x80a\t\x90\x86\x82a\x0CpV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\r\xC0WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\r\xB3V[\x90` \x82R``a\x0E*a\x0E\x15a\r\xFF\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\r\xA3V[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\r\xA3V[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\r\xA3V[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0EcWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0E\x81`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\r\xA3V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0ETV[4a\x02\xB0W`\x806`\x03\x19\x01\x12a\x02\xB0W`\x045a\x0E\xAD\x81a\x08\x8DV[`$5\x90a\x0E\xBA\x82a\x02\xC2V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x0E\xD9\x906\x90`\x04\x01a\x02\xDDV[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x02\xB0W6`#\x85\x01\x12\x15a\x02\xB0W\x83`\x04\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x02\xB0W6`$\x85`\x05\x1B\x87\x01\x01\x11a\x02\xB0Wa\t\x90\x95`$a\x0F+\x96\x01\x93a)\xB1V[`@Q\x91\x82\x91\x82a\r\xDCV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05|Wa\x0F\xA2\x91_\x91a\x05MWPa \x9AV[a\x04raG@V[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` `@Qg\x01cEx]\x8A\0\0\x81R\xF3[`\xFF\x81\x16\x03a\x02\xB0WV[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0W` `\x01`\xFF`\x045a\x0F\xFA\x81a\x0F\xCCV[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x02\xB0W``6`\x03\x19\x01\x12a\x02\xB0W`\x045a\x10'\x81a\x08\x8DV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x10F\x906\x90`\x04\x01a\x0C\x13V[`D5\x91a\x10S\x83a\x02\xC2V[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x10{\x86\x88`\x04\x84\x01a.\x19V[\x03\x81\x84Z\xFA\x91\x82\x15a\x05|W_\x92a\x11_W[Pa\x10\x99\x83Qa\"\x0CV[\x93_[\x84Q\x81\x10\x15a\x11QWa\x10\xAF\x81\x86a\"KV[Q\x90` \x83a\x10\xCBa\x10\xC1\x84\x89a\"KV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x05|W`\x01\x92_\x91a\x11#W[P\x82\x80`\xC0\x1B\x03\x16a\x11\x1C\x82\x89a\"KV[R\x01a\x10\x9CV[a\x11D\x91P` =\x81\x11a\x11JW[a\x11<\x81\x83a\x05\xD0V[\x81\x01\x90a)(V[_a\x11\nV[P=a\x112V[`@Q\x80a\t\x90\x88\x82a\x08\xE8V[a\x11|\x91\x92P=\x80_\x83>a\x11t\x81\x83a\x05\xD0V[\x81\x01\x90a(\x06V[\x90_a\x10\x8EV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` `fT`@Q\x90\x81R\xF3[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0Wc\xFF\xFF\xFF\xFF`\x045a\x11\xC2\x81a\x02\xC2V[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x815a\x12\xC1\x81a\x08\x9EV[\x92a\x12\xCF`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xB0W` \x01\x90[\x82\x82\x10a\x12\xF7WPPP\x90V[` \x80\x91\x835a\x13\x06\x81a\x02\xC2V[\x81R\x01\x91\x01\x90a\x12\xEAV[\x81`\x1F\x82\x01\x12\x15a\x02\xB0W\x805a\x13'\x81a\x08\x9EV[\x92a\x135`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x02\xB0W` \x01\x91[\x83\x83\x10a\x13_WPPPP\x90V[` `@\x91a\x13n\x84\x86a\x06eV[\x81R\x01\x92\x01\x91a\x13QV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x815a\x13\x90\x81a\x08\x9EV[\x92a\x13\x9E`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x02\xB0W` \x82\x01\x90[\x83\x82\x10a\x13\xCAWPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W` \x91a\x13\xEC\x87\x84\x80\x94\x88\x01\x01a\x12\xAAV[\x81R\x01\x91\x01\x90a\x13\xBBV[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x02\xB0Wa\x14\ra\x05\xF1V[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x81a\x14*\x91\x84\x01a\x12\xAAV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x81a\x14K\x91\x84\x01a\x13\x11V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x81a\x14o\x91\x84\x01a\x13\x11V[`@\x85\x01Ra\x14\x81\x81``\x84\x01a\x07\rV[``\x85\x01Ra\x14\x93\x81`\xE0\x84\x01a\x06eV[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x81a\x14\xB8\x91\x84\x01a\x12\xAAV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x81a\x14\xDD\x91\x84\x01a\x12\xAAV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x15\0\x92\x01a\x13yV[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x15$WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x15\x17V[4a\x02\xB0W`\x806`\x03\x19\x01\x12a\x02\xB0W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x15v\x906\x90`\x04\x01a\x02\xDDV[\x90\x91`D5a\x15\x84\x81a\x02\xC2V[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x02\xB0Wa\x15\xE6\x94a\x15\xABa\x15\xB1\x956\x90`\x04\x01a\x13\xF7V[\x93a/\xF2V[`@Q\x92\x83\x92`@\x84R` a\x15\xD2\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x15\x07V[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x15\x07V[\x90` \x83\x01R\x03\x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0Wa\x16\x08aM\xE3V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90\x81`\x80\x91\x03\x12a\x02\xB0W\x90V[4a\x02\xB0W`\xC06`\x03\x19\x01\x12a\x02\xB0W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x17h\x906\x90`\x04\x01a\x17*V[a\x17q6a\x02\x9FV[\x90`@6`c\x19\x01\x12a\x02\xB0W`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x04r\x92a\x17\xA3`d\x926\x90`\x04\x01a\x13\x11V[\x92a=\nV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xB0W`\x806`\x03\x19\x01\x12a\x02\xB0W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x17\xFB\x906\x90`\x04\x01a\x17*V[a\x18\x046a\x02\x9FV[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x18$\x906\x90`\x04\x01a\x13\xF7V[`\xCDT\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1A\x15Wa\x18G` \x83\x94\x93\x01a8\x8FV[\x91a\x19(a\x18X`@\x86\x01\x86a;\x81V[\x92\x90\x94a\x18\x96a\x18j``\x89\x01a8\x8FV[\x97`@Qa\x18\x80\x81a\x03\xDC` \x82\x01\x94\x85aC\xB9V[Q\x90 a\x18\x8Fa\x03\xFF\x88a8\x8FV[T\x14aDqV[a\x18\xC0a\x18\xB9a\x18\xA5\x87a8\x8FV[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15aD\xE3V[\x83c\xFF\xFF\xFF\xFFC\x16\x96a\x19\na\x19\x02a\x18\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a `V[c\xFF\xFF\xFF\xFF\x16\x90V[\x89\x11\x15aEDV[`@Q` \x81\x01\x90a\x19 \x81a\x03\xDC\x8B\x85aE\xA6V[Q\x90 a/\xF2V[\x91\x90`\xFF_\x96\x16\x95[\x82\x81\x10a\x19\xB3W\x7F4\x9C\x1E\xE6\x0EN\x89r\xEE\x9D\xBAd,\x17tT=\\A6\x87\x9B\x7FL\xAA\xF0K\xF8\x1AHz*\x86\x86\x86a\x19sa\x19ga\x06\x01V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x19\x8F\x81a\x03\xDC\x86\x86\x86aFJV[Q\x90 a\x19\x9Ea\x18\xA5\x83a8\x8FV[Ua\x19\xAE`@Q\x92\x83\x92\x83aFJV[\x03\x90\xA1\0[\x80a\x1A\x0Fa\x19\xEBa\x19\xE6a\x19\xDAa\x19\xCD`\x01\x96\x88Qa\"KV[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[a:\xB7V[a\x1A\x08a\x19\xDA\x8Ba\x1A\x03a\x19\xCD\x87` \x8B\x01Qa\"KV[aE\xB6V[\x11\x15aE\xD9V[\x01a\x191V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xB0W`\xC06`\x03\x19\x01\x12a\x02\xB0W`\x045a\x1A\x9F\x81a\x08\x8DV[a\x1B\x1F`$5a\x1A\xAE\x81a\x08\x8DV[`D5a\x1A\xBA\x81a\x08\x8DV[`d5a\x1A\xC6\x81a\x08\x8DV[`\x845\x91a\x1A\xD3\x83a\x08\x8DV[`\xA45\x93a\x1A\xE0\x85a\x08\x8DV[_T\x96a\x1B\x05`\xFF`\x08\x8A\x90\x1C\x16\x15\x80\x99\x81\x9Aa\x1B\x9DW[\x81\x15a\x1B}W[PaFtV[\x87a\x1B\x16`\x01`\xFF\x19_T\x16\x17_UV[a\x1BfWaF\xD7V[a\x1B%W\0[a\x1B3a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x19\xAEV[a\x1Bxa\x01\0a\xFF\0\x19_T\x16\x17_UV[aF\xD7V[0;\x15\x91P\x81a\x1B\x8FW[P_a\x1A\xFFV[`\xFF\x16`\x01\x14\x90P_a\x1B\x88V[`\x01`\xFF\x82\x16\x10\x91Pa\x1A\xF8V[`@\x90a\x08\xF9\x93\x92\x81R\x81` \x82\x01R\x01\x90a\t\xFFV[4a\x02\xB0W``6`\x03\x19\x01\x12a\x02\xB0W`\x045a\x1B\xDF\x81a\x08\x8DV[`$5`D5a\x1B\xEE\x81a\x02\xC2V[a\x1C/a\x1B\xF9a!\xEAV[\x92\x80a\x1C\x04\x85a\">V[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a.\x19V[\x03\x81\x87Z\xFA\x93\x84\x15a\x05|W\x83a\x1CYa\x18\xF9a\x10\xC1a\x1C\x8E\x98` \x97_\x91a\x1C\xECW[Pa\">V[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x05|Wa\x1C\xBD\x92_\x91a\x1C\xCDW[P`\x01`\x01`\xC0\x1B\x03\x16\x92a\x1C\xB7\x84aN\x83V[\x90a$\xACV[\x90a\t\x90`@Q\x92\x83\x92\x83a\x1B\xABV[a\x1C\xE6\x91P` =` \x11a\x11JWa\x11<\x81\x83a\x05\xD0V[_a\x1C\xA3V[a\x1D\0\x91P=\x80_\x83>a\x11t\x81\x83a\x05\xD0V[_a\x1CSV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0W`\x045a\x1Dg\x81a\x08\x8DV[a\x1DoaM\xE3V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1D\x87Wa\x04r\x90aN;V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` `@Q`d\x81R\xF3[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05|W_\x91a\x1E\xB9W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1E\xAAWa\x1Ex`fT\x19\x82\x19\x81\x16\x14a \xB0V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a\x1E\xD2\x91P` =` \x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[_a\x1EWV[`@Q\x90a\x1E\xE5\x82a\x05\x95V[``` \x83\x82\x81R\x01RV[`@Q\x90a\x1E\xFE\x82a\x05\xB5V[_``\x83a\x1F\na\x1E\xD8V[\x81R\x82` \x82\x01R\x81`@\x82\x01R\x01RV[\x91\x90`@\x83\x82\x03\x12a\x02\xB0W`@Q\x90a\x1F5\x82a\x05\x95V[\x81\x93\x805`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x82a\x1FS\x91\x83\x01a\x0C\x13V[\x83R` \x81\x015\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xB0W` \x92a\x07\x08\x92\x01a\x0C\x13V[a\x08\xF9\x906\x90a\x1F\x1CV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[` \x81R`\x80c\xFF\xFF\xFF\xFF``a \ra\x1F\xEC\x86Q\x85` \x88\x01R` a\x1F\xD8\x82Q`@`\xA0\x8B\x01R`\xE0\x8A\x01\x90a\x08\xB5V[\x91\x01Q\x87\x82\x03`\x9F\x19\x01`\xC0\x89\x01Ra\x08\xB5V[\x83` \x88\x01Q\x16`@\x87\x01R`@\x87\x01Q`\x1F\x19\x87\x83\x03\x01\x84\x88\x01Ra\x1F\x81V[\x94\x01Q\x16\x91\x01R\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a CWV[a \x17V[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a CWV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a CWV[\x90\x81` \x91\x03\x12a\x02\xB0WQa\x08\xF9\x81a\x0BHV[`@Q=_\x82>=\x90\xFD[\x15a \xA1WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a \xB7WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a \xEBW`\x05\x1B\x01\x90V[a \xC6V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a!\xE0a!\xBDa!\xE6\x95a!\xB7a!\xB0\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra!\x87\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x05\xD0V[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aG\xEAV[\x90aH0V[\x92a!\xB7a!\xD2a!\xCCaH\x92V[\x94aI\x89V[\x91a!\xDBaJ\xA5V[aG\xEAV[\x91aJ\xD9V[\x90\x91V[`@\x80Q\x90\x91\x90a!\xFB\x83\x82a\x05\xD0V[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a\"\x16\x82a\x08\x9EV[a\"#`@Q\x91\x82a\x05\xD0V[\x82\x81R\x80\x92a\"4`\x1F\x19\x91a\x08\x9EV[\x01\x90` 6\x91\x017V[\x80Q\x15a \xEBW` \x01\x90V[\x80Q\x82\x10\x15a \xEBW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x02\xB0WQ\x90V[\x91\x90\x91a\"{\x83Qa\"\x0CV[\x92_[\x81Q\x81\x10\x15a#.W\x80` a\"\xA7a\"\x9Aa\"\xD0\x94\x86a\"KV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x05|W`\x01\x92_\x91a#\0W[Pa\"\xF9\x82\x88a\"KV[R\x01a\"~V[a#!\x91P` =\x81\x11a#'W[a#\x19\x81\x83a\x05\xD0V[\x81\x01\x90a\"_V[_a\"\xEEV[P=a#\x0FV[PPPV[\x90\x81` \x91\x03\x12a\x02\xB0WQa\x08\xF9\x81a\x08\x8DV[\x90a#R\x82a\x08\x9EV[a#_`@Q\x91\x82a\x05\xD0V[\x82\x81R` \x81\x93a#r`\x1F\x19\x91a\x08\x9EV[\x01\x91\x01_[\x82\x81\x10a#\x83WPPPV[``\x82\x82\x01R` \x01a#wV[\x90\x81Q\x81\x10\x15a \xEBW\x01` \x01\x90V[` \x81\x83\x03\x12a\x02\xB0W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x81Qa#\xD5\x81a\x08\x9EV[\x92a#\xE3`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xB0W` \x01\x90[\x82\x82\x10a$\x0BWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a#\xFEV[\x90a$%\x82a\x08\x9EV[a$2`@Q\x91\x82a\x05\xD0V[\x82\x81R\x80\x92a$C`\x1F\x19\x91a\x08\x9EV[\x01_[\x81\x81\x10a$RWPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x05\xB0W` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a$FV[\x90\x81` \x91\x03\x12a\x02\xB0WQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x02\xB0W\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x05|W_\x95a'\xC1W[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x05|W`\x04\x96_\x93a'\x9FW[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x05|W_\x96a'~W[Pa%:\x85\x93\x92\x95Qa#HV[\x94_\x93[\x80Q\x85\x10\x15a'tWa%ka%ea%W\x87\x84a#\x91V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x05|W_\x94a'PW[Pa%\xBB\x84Qa$\x1BV[a%\xC5\x88\x8Ba\"KV[Ra%\xD0\x87\x8Aa\"KV[P_[\x84Q\x81\x10\x15a'?W\x80` a%\xECa&\x0E\x93\x88a\"KV[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05|W_\x92a'\x1FW[Pa&4\x81\x87a\"KV[Q\x8A` \x8Aa&C\x85\x8Ba\"KV[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x05|Wa&\xD6\x8C\x8Fa&\xD1`\x01\x98a&\xE8\x97\x89\x97_\x92a&\xEFW[Pa&\xBCa&\xADa\x06\x10V[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a\"KV[Q\x90a&\xE2\x83\x83a\"KV[Ra\"KV[P\x01a%\xD3V[a'\x11\x91\x92P` =\x81\x11a'\x18W[a'\t\x81\x83a\x05\xD0V[\x81\x01\x90a$\x8DV[\x90_a&\xA1V[P=a&\xFFV[a'8\x91\x92P` =\x81\x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x90_a&)V[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa%>V[a'm\x91\x94P=\x80_\x83>a'e\x81\x83a\x05\xD0V[\x81\x01\x90a#\xA2V[\x92_a%\xB0V[PPP\x93PPP\x90V[a'\x98\x91\x96P` =` \x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x94_a%,V[` \x91\x93Pa'\xBA\x90\x82=\x84\x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x92\x90a%\x07V[a'\xDB\x91\x95P` =` \x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x93_a$\xDEV[`@Q\x90a'\xEF\x82a\x05\xB5V[``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x02\xB0W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x81Qa(9\x81a\x08\x9EV[\x92a(G`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xB0W` \x01\x90[\x82\x82\x10a(oWPPP\x90V[` \x80\x91\x83Qa(~\x81a\x02\xC2V[\x81R\x01\x91\x01\x90a(bV[\x81\x83R\x90\x91`\x01`\x01`\xFB\x1B\x03\x83\x11a\x02\xB0W` \x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x08\xF9\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a(\x89V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x08\xF9\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a(\xCAV[`\xFF\x16`\xFF\x81\x14a CW`\x01\x01\x90V[\x91\x90\x81\x10\x15a \xEBW`\x05\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x02\xB0WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x02\xB0W\x90V[\x15a)NWV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a \xEBW\x01\x90V[\x90\x81` \x91\x03\x12a\x02\xB0WQa\x08\xF9\x81a\x02\xC2V[_\x19\x81\x14a CW`\x01\x01\x90V[\x91a)\xAA` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a(\xCAV[\x94\x16\x91\x01RV[\x95\x93\x94\x95\x92\x90\x91\x92a)\xC1a'\xE2V[P`@Qch0H5`\xE0\x1B\x81R\x93`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90` \x85`\x04\x81\x86Z\xFA\x94\x85\x15a\x05|W_\x95a-\xF8W[Pa)\xFEa'\xE2V[\x94`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a*\x1E\x8D\x8D\x8B`\x04\x85\x01a(\xADV[\x03\x81\x88Z\xFA\x90\x81\x15a\x05|W_\x91a-\xDEW[P\x86R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x92\x90_\x81\x80a*a\x85\x87\x8B`\x04\x85\x01a(\xEAV[\x03\x81\x87Z\xFA\x90\x81\x15a\x05|W_\x91a-\xC4W[P`@\x87\x01Ra*\x83\x81a#HV[\x98``\x87\x01\x99\x8AR_[`\xFF\x81\x16\x83\x81\x10\x15a-\x0FW\x88_a*\xB6\x83\x8Fa*\xA9\x88a\"\x0CV[\x90Q\x90a&\xE2\x83\x83a\"KV[P_\x8A\x86\x8F[\x81\x84\x10a+9WPPPP\x90P\x8Ca*\xD3\x82a\"\x0CV[\x91_[\x81\x81\x10a+\0WPP\x91a*\xF5\x91a*\xFB\x94\x93Q\x90a&\xE2\x83\x83a\"KV[Pa)\x07V[a*\x8DV[\x80a+3a+\x1Ea\x10\xC1`\x01\x94a+\x18\x8A\x89Qa\"KV[Qa\"KV[a+(\x83\x88a\"KV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x01a*\xD6V[a\x10\xC1\x84a+N\x81` \x96\x95a+V\x95a)\x18V[5\x97Qa\"KV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x87\x01R\x16`D\x85\x01R\x83`d\x81\x8DZ\xFA\x80\x15a\x05|W\x88\x8F\x88\x8A\x91\x8F\x94a+\xFB`\x01a+\xEE\x81\x93\x8D\x80\x9D_\x92a,\xE3W[Pa%ea+\xCAa+\xD8\x92a+\xC3\x87\x80`\xC0\x1B\x03\x86\x16\x15\x15a)GV[\x8B\x8Da)]V[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a,\x17W[PPPPP`\x01\x91\x92P\x01\x90\x8A\x91\x8A\x86\x8Fa*\xBCV[\x85\x97a,9\x93a,2` \x97\x99\x98a%e\x95a+\xCA\x95a)\x18V[5\x95a)]V[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`D\x84\x01R\x82`d\x81\x8CZ\xFA\x90\x81\x15a\x05|W\x8Fa,\x97\x90a,\x9C\x93\x83\x88`\x01\x97_\x93a,\xABW[Pa+\x18\x90a+(\x93\x94Qa\"KV[a)~V[\x90P\x82\x91\x8A\x88\x8F\x88\x8A\x91a,\x01V[a+(\x93P\x90a,\xD4a+\x18\x92` =\x81\x11a,\xDCW[a,\xCC\x81\x83a\x05\xD0V[\x81\x01\x90a)iV[\x93P\x90a,\x87V[P=a,\xC2V[a+\xD8\x91\x92Pa+\xCAa-\x06a%e\x92` =\x81\x11a\x11JWa\x11<\x81\x83a\x05\xD0V[\x93\x92PPa+\xA6V[PPP\x92\x90\x95\x97P`\x04\x94\x96P` \x91P`@Q\x94\x85\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x90\x81\x15a\x05|Wa-e\x94_\x94\x85\x93a-\xA3W[P`@Qc5IR\xA3`\xE2\x1B\x81R\x95\x86\x94\x85\x93\x84\x93`\x04\x85\x01a)\x8CV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05|W_\x91a-\x89W[P` \x82\x01R\x90V[a-\x9D\x91P=\x80_\x83>a\x11t\x81\x83a\x05\xD0V[_a-\x80V[a-\xBD\x91\x93P` =` \x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x91_a-GV[a-\xD8\x91P=\x80_\x83>a\x11t\x81\x83a\x05\xD0V[_a*tV[a-\xF2\x91P=\x80_\x83>a\x11t\x81\x83a\x05\xD0V[_a*1V[a.\x12\x91\x95P` =` \x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x93_a)\xF5V[`@\x90c\xFF\xFF\xFF\xFFa\x08\xF9\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x08\xB5V[\x15a.=WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a.RWV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a.hWV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a.~WV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\xB0WQa\x08\xF9\x81a\x0F\xCCV[_\x19\x81\x01\x91\x90\x82\x11a CWV[\x15a.\xB7WV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90`\x01\x82\x01\x80\x92\x11a CWV[\x90`\x02\x82\x01\x80\x92\x11a CWV[\x90`\x03\x82\x01\x80\x92\x11a CWV[\x90`\x04\x82\x01\x80\x92\x11a CWV[\x90`\x05\x82\x01\x80\x92\x11a CWV[\x91\x90\x82\x01\x80\x92\x11a CWV[\x15a/ WV[c\xAF\xFC^\xDB`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\xB0WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x02\xB0W\x90V[\x15a/WWV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a CWV[\x15a/\x8DWV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a/\xA3WV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a/\xDCWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a/\xCFV[\x94\x93\x92\x90\x91\x93a0\0a\x1E\xD8V[Pa0\x0C\x85\x15\x15a.6V[`@\x84\x01QQ\x85\x14\x80a8\x81W[\x80a8sW[\x80a8eW[a0/\x90a.KV[a0A` \x85\x01QQ\x85QQ\x14a.aV[a0Xc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a.wV[a0`a\x06\x01V[_\x81R_` \x82\x01R\x92a0ra\x1E\xD8V[a0{\x87a\"\x0CV[` \x82\x01Ra0\x89\x87a\"\x0CV[\x81Ra0\x93a\x1E\xD8V[\x92a0\xA2` \x88\x01QQa\"\x0CV[\x84Ra0\xB2` \x88\x01QQa\"\x0CV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05|Wa1\x1B\x91_\x91a86W[Pa1\x166\x8B\x87a\t\xC9V[aLEV[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a2\x8DW` \x88a1\x82a\x10\xC1\x8Ca1z\x8F\x96\x86\x8Ea1_a1L\x86\x80\x95a\"KV[Q\x80Q_R` \x01Q` R`@_ \x90V[a1l\x84\x84\x84\x01Qa\"KV[R\x82a2ZW[\x01Qa\"KV[Q\x95Qa\"KV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x05|Wa!\xB7\x8Aa2/\x8Fa2(\x8F\x84` \x8F\x92a2\x1F\x93a2\x17\x84`\x01\x9Ea25\x9E_\x91a2=W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\"KV[R\x01Qa\"KV[Q\x93\x8DQa\"KV[Q\x16aLpV[\x90aL\xA1V[\x97\x01\x96a1\x1FV[a2T\x91P\x86=\x81\x11a\x11JWa\x11<\x81\x83a\x05\xD0V[_a2\x08V[a2\x88a2j\x84\x84\x84\x01Qa\"KV[Qa2\x81\x84\x84\x01Qa2{\x87a.\xA2V[\x90a\"KV[Q\x10a.\xB0V[a1sV[P\x90\x95\x97\x94\x96Pa2\xA2\x91\x98\x93\x92\x99PaM^V[\x91a2\xAF`\x97T`\xFF\x16\x90V[\x90\x81\x15a8.W`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05|W_\x91a8\x0FW[P\x91\x90[_\x92[\x81\x84\x10a3`WPPPPP\x92a3Ga3Ba3;a3Z\x95\x85a\x03\xDC\x98`\x80``` \x99\x01Q\x92\x01Q\x92a!\x04V[\x91\x90a/\x86V[a/\x9CV[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a/\xB2V[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da7\tW[a\x10\xC1\x82`\xA0a3\xB5a%ea+\xCA\x84a3\xBD\x97a3\xAFa3\xA1a1L\x8F\x9C`@` \x9F\x9E\x01Qa\"KV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba)]V[\x97\x01Qa\"KV[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x05|Wa4\x81a\x10\xC1\x8F\x95\x8F\x90a4y\x8F\x97\x8F\x96\x84\x8Fa4s`\xC0\x96a4l\x84\x8F` \x9F\x90a1sa+\xCA\x99`@\x93a%e\x9C_\x91a6\xDBW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a/PV[Q\x90aH0V[\x9Ca)]V[\x96\x01Qa\"KV[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x05|Wa5\x0E\x91\x8C\x8F\x92_\x92a6\xB7W[P` a5\0\x92\x93\x01Qa\"KV[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a5.\x8Ca5\0\x8Ca5'a\x19\xCD\x82` \x86\x01Qa\"KV[\x92Qa\"KV[_\x98_[` \x8A\x01QQ\x81\x10\x15a6\x9EW\x8B\x8Da5p\x89a5ca%ea+\xCA\x86\x8F\x89a5[\x91Qa\"KV[Q\x94\x87a)]V[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a5\x7FW[PP`\x01\x01a52V[\x8A\x8Aa6\x01\x85\x9F\x94\x8F\x96\x86a+\x18\x8F\x93`\xE0a5\xB8a\x10\xC1\x95` a5\xB0a%ea+\xCA\x83\x9Fa5\xC1\x9C\x89\x91a)]V[\x9A\x01Qa\"KV[Q\x9B\x01Qa\"KV[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05|W\x8Fa6m\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a6xW[Pa6ga5\0\x92\x93Q\x93a6ba\x19\xCD\x84\x87a\"KV[a/fV[\x92a\"KV[\x01\x9A\x90P\x8B\x8Da5uV[a5\0\x92Pa6\x97a6g\x91` =\x81\x11a'\x18Wa'\t\x81\x83a\x05\xD0V[\x92Pa6JV[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a3\nV[a5\0\x92Pa6\xD4` \x91\x82=\x81\x11a'\x18Wa'\t\x81\x83a\x05\xD0V[\x92Pa4\xF1V[` a6\xFC\x92P=\x81\x11a7\x02W[a6\xF4\x81\x83a\x05\xD0V[\x81\x01\x90a//V[_a4VV[P=a6\xEAV[a7F\x94Pa7#\x92Pa%e\x91a+\xCA\x91` \x95a)]V[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05|W` \x89a3\xBD\x8F\x93\x8F`\xA0\x8F\x97a%ea+\xCA\x8F\x8F\x90a3\xAFa3\xA1a1L\x8F`@\x8B\x96\x91\x8F\x88\x93a\x10\xC1\x9Fa7\xCA\x90a7\xD0\x93a3\xB5\x9F_\x92a7\xE6W[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a/\x0CV[\x11a/\x19V[PPPPPP\x97PPPPPP\x92\x93PPa3uV[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a8\x07\x91=\x81\x11a#'Wa#\x19\x81\x83a\x05\xD0V[\x92\x91Pa7\xB9V[a8(\x91P` =` \x11a,\xDCWa,\xCC\x81\x83a\x05\xD0V[_a3\x03V[_\x91\x90a3\x07V[a8X\x91P` =` \x11a8^W[a8P\x81\x83a\x05\xD0V[\x81\x01\x90a.\x8DV[_a1\nV[P=a8FV[P`\xE0\x84\x01QQ\x85\x14a0&V[P`\xC0\x84\x01QQ\x85\x14a0 V[P`\xA0\x84\x01QQ\x85\x14a0\x1AV[5a\x08\xF9\x81a\x02\xC2V[\x905\x90`>\x19\x816\x03\x01\x82\x12\x15a\x02\xB0W\x01\x90V[\x15a8\xB5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a9\x18\x81a\x02\xC2V[\x16\x84R\x015\x91\x01RV[\x90\x92\x91` ``\x91a98\x84`\x80\x81\x01\x97a9\x04V[c\xFF\xFF\xFF\xFF\x815a9H\x81a\x02\xC2V[\x16`@\x85\x01R\x015\x91\x01RV[\x15a9\\WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a9\xCEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a:LWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a CWV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a CWV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a CWV[\x15a:\xFDWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x02\xB0W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W` \x01\x91\x816\x03\x83\x13a\x02\xB0WV[` \x81\x83\x03\x12a\x02\xB0W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x81Qa;\xE6\x81a\x08\x9EV[\x92a;\xF4`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xB0W` \x01\x90[\x82\x82\x10a<\x1CWPPP\x90V[` \x80\x91\x83Qa<+\x81a\x08\x8DV[\x81R\x01\x91\x01\x90a<\x0FV[`@Q\x90a<E`@\x83a\x05\xD0V[`\x12\x82Rq9\xB60\xB9\xB4/\xBA42\xAF\xB7\xB82\xB90\xBA7\xB9`q\x1B` \x83\x01RV[\x91\x90` \x83R`\xC0\x83\x01\x92`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x93`\xA0``\x83\x01R\x84Q\x80\x91R` `\xE0\x83\x01\x95\x01\x90_[\x81\x81\x10a<\xEBWPPP`\x80a<\xD6a\x08\xF9\x94\x95``\x85\x01Q`\x1F\x19\x85\x83\x03\x01\x84\x86\x01Ra\x08\xB5V[\x92\x01Q\x90`\xA0`\x1F\x19\x82\x85\x03\x01\x91\x01Ra\x1F\x81V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x96\x87\x01\x96\x90\x92\x01\x91`\x01\x01a<\xADV[\x91\x93\x92\x90\x93a=\x18\x85a8\x8FV[\x93a=+a=&\x85\x80a8\x99V[a\x1FvV[\x94a=Oa=G\x82c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a8\xAEV[a=\x8Ba=j\x82c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x88`@Qa=\x82\x81a\x03\xDC\x89` \x83\x01\x95\x86a9\"V[Q\x90 \x14a9UV[a=\xB6a=\xB0a=\xA9\x83c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a9\xC7V[a=\xDBa=\xCDa\x18\xF9a=\xC8\x86a8\x8FV[a HV[c\xFF\xFF\xFF\xFFC\x16\x11\x15a:EV[_\x93_\x95` \x88\x01\x95[\x88Q\x80Q\x89\x10\x15a>$W`\x01\x91a>\x16a>\x03\x8Ba>\x1C\x94a\"KV[Qa>\x0F\x8C\x8CQa\"KV[Q\x90a:\xE3V[\x90a/\x0CV[\x97\x01\x96a=\xE5V[P\x92\x97P\x94P\x94P` `\x01\x92\x97\x015\x14\x14aC#Wa>D\x83Qa\"\x0CV[\x93_[\x84Q\x81\x10\x15a>qW\x80a>`a1L`\x01\x93\x88a\"KV[a>j\x82\x89a\"KV[R\x01a>GV[P\x90\x92\x93\x91\x94a>\xAB` \x87\x01\x94` a>\x8A\x87a8\x8FV[`@Qa>\x9F\x81a\x03\xDC\x8A\x86\x83\x01\x95\x86a/\xB2V[Q\x90 \x91\x015\x14a:\xF6V[a>\xB5\x85Qa\"\x0CV[\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94_[\x87Q\x81\x10\x15a?eW\x80` a>\xFCa?\x1C\x93\x89a\"KV[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8BZ\xFA\x91\x82\x15a\x05|W`\x01\x92a?A\x91_\x91a?GW[Pa\rb\x83\x8Da\"KV[\x01a>\xE3V[a?_\x91P` =\x81\x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[_a?6V[P\x92\x96\x95P\x92P\x92a?\xC4a?\x8Da?\x95`@\x86\x01\x94a?\x85\x86\x88a;\x81V[\x93\x90\x91a8\x8FV[\x926\x91a\t\xC9V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a$\xACV[\x94_\x91[\x86Q\x83\x10\x15aB\xC5W_\x97\x96\x97\x95[a?\xE1\x84\x8Aa\"KV[QQ\x87\x10\x15aB\xB5W\x97a@\"\x98` \x80a@\0\x8Aa+\x18\x89\x87a\"KV[Q\x01Q`@Q\x80\x9C\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x99\x8A\x15a\x05|W_\x9AaB\x95W[P`\x01\x98_[\x85Q\x81\x10\x15aB\x87Wa@`a@Ta\"\x9A\x83\x89a\"KV[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x14a@wW`\x01\x01a@;V[P\x99\x90\x97\x91\x98P`\x01_[\x15\x15\x14a@\x97W[P`\x01\x01\x95\x97\x96\x97a?\xD7V[\x98\x96\x97\x86\x98_\x87\x95\x93\x98a@\xFC`\xFFa@\xD4a%ea+\xCA\x8Ca@\xCEaAQ\x9Fa@\xC8`\xD1T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x98a;\x81V[\x90a)]V[a@\xEEa@\xDFa\x06\x01V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[\x16c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`\xD0TaA\x13\x90a@T\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\x10]\xEA\x1F`\xE2\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\x04\x82\x01R` \x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16`$\x83\x01R\x90\x98\x89\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x96\x87\x15a\x05|W_\x97aBcW[PaAn\x87Qa\"\x0CV[\x98_[\x8AQ\x81\x10\x15aA\x97W\x80g\x01cEx]\x8A\0\0aA\x90`\x01\x93\x8Ea\"KV[R\x01aAqV[P\x9A\x92\x94\x96aA\xD5`\xFFaA\xBCa%ea+\xCA\x8F\x9D\x97\x9F\x96\x9E\x96a@\xCE\x8E\x8E\x92a;\x81V[aA\xC7a&\xADa\x06\x1FV[\x16c\xFF\xFF\xFF\xFF\x16` \x86\x01RV[`@\x84\x01R``\x83\x01RaA\xE7a<6V[`\x80\x83\x01R`\xCFTaB\x03\x90a@T\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x02\xB0W`@Qcjf\x9BA`\xE0\x1B\x81R\x92_\x91\x84\x91\x82\x90\x84\x90\x82\x90aB/\x90`\x04\x83\x01a<fV[\x03\x92Z\xF1\x91\x82\x15a\x05|W`\x01\x92aBIW[P\x90a@\x8AV[\x80aBW_aB]\x93a\x05\xD0V[\x80a\x07\xA9V[_aBBV[aB\x80\x91\x97P=\x80_\x83>aBx\x81\x83a\x05\xD0V[\x81\x01\x90a;\xB3V[\x95_aAcV[P\x99\x90\x97\x91\x98`\x01\x90a@\x82V[aB\xAE\x91\x9AP` =\x81\x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x98_a@5V[\x96\x97\x96\x95P`\x01\x90\x92\x01\x91a?\xC8V[PPPPP\x91\x90PaB\xF5aB\xE8\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3V[PPPc\xFF\xFF\xFF\xFF3\x91\x16\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05_\x80\xA3V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x02\xB0W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W\x81`\x05\x1B6\x03\x83\x13a\x02\xB0WV[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x02\xB0W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W\x816\x03\x83\x13a\x02\xB0WV[` \x81R\x815\x90`>\x19\x836\x03\x01\x82\x12\x15a\x02\xB0W`\x80aDf``aD_aD%\x87a\x08\xF9\x97\x01\x85` \x88\x01RaD\x13aD\x08aC\xF7\x83\x80aCTV[`@`\xA0\x8C\x01R`\xE0\x8B\x01\x91a(\x89V[\x91` \x81\x01\x90aCTV[\x88\x83\x03`\x9F\x19\x01`\xC0\x8A\x01R\x90a(\x89V[aDAaD4` \x8A\x01a\x02\xD0V[c\xFF\xFF\xFF\xFF\x16`@\x88\x01RV[aDN`@\x89\x01\x89aC\x88V[\x87\x83\x03`\x1F\x19\x01\x85\x89\x01R\x90a(\xCAV[\x95\x01a\x02\xD0V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15aDxWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15aD\xEAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[\x15aEKWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[`@\x81\x01\x92\x91a\x02\xDB\x91\x90a9\x04V[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a CWV[\x15aE\xE0WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[\x90\x92\x91` ``\x91aF`\x84`\x80\x81\x01\x97a9\x04V[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[\x15aF{WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aF\xE0\x90aN;V[`\xCD\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xCE\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD0\x80T\x83\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\xCF\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aG\xB3\x82a\x05\x95V[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aG\xCF\x81\x84a\x05\xD0V[6\x837V[`@Q\x90aG\xE3` \x83a\x05\xD0V[` 6\x837V[\x91\x90`@\x90``aG\xF9aG\xA6V[\x94\x85\x92` \x85Q\x92aH\x0B\x85\x85a\x05\xD0V[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aH.WV[\xFE[` \x92\x91`\x80`@\x92aHAaG\xA6V[\x95\x86\x93\x81\x86Q\x93aHR\x86\x86a\x05\xD0V[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aH.W\x15aH\x83WV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@QaH\x9E\x81a\x05\x95V[`@\x90\x81QaH\xAD\x83\x82a\x05\xD0V[\x826\x827\x81R` \x82Q\x91aH\xC2\x84\x84a\x05\xD0V[\x836\x847\x01R\x80QaH\xD4\x82\x82a\x05\xD0V[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aI*\x83\x83a\x05\xD0V[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaI\x7F\x83Q\x93\x84a\x05\xD0V[\x82R` \x82\x01R\x90V[_Q` aO\xC6_9_Q\x90_R\x90aI\xA0aG\xA6V[P_\x91\x90\x06` `\xC0\x83[aJ\xA0W_\x93_Q` aO\xC6_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaI\xD6\x85\x82a\x05\xD0V[\x846\x827\x84\x81\x85`@QaI\xEA\x82\x82a\x05\xD0V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aO\xC6_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aH.WaJT\x90aO\xAFV[Q\x91aJ\xA0W_Q` aO\xC6_9_Q\x90_R\x82\x80\t\x14aJ\x8BWP_Q` aO\xC6_9_Q\x90_R`\x01_\x94\x08\x92\x93aI\xABV[\x92\x93PPaJ\x97a\x06\x01V[\x92\x83R\x82\x01R\x90V[a \xF0V[aJ\xADaG\xA6V[P`@QaJ\xBA\x81a\x05\x95V[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x0C\x81\x10\x15a \xEBW`\x05\x1B\x01\x90V[\x93\x92\x90\x91aJ\xE7`@a\x06.V[\x94\x85R` \x85\x01RaJ\xF9`@a\x06.V[\x91\x82R` \x82\x01RaK\taG\xBEV[\x92_[`\x02\x81\x10aK6WPPP` a\x01\x80\x92aK%aG\xD4V[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aKB`\x01\x92a:\xCDV[aKL\x82\x85a \xDAV[QQaKX\x82\x89aJ\xC8V[R` aKe\x83\x86a \xDAV[Q\x01QaKzaKt\x83a.\xC6V[\x89aJ\xC8V[RaK\x85\x82\x86a \xDAV[QQQaK\x94aKt\x83a.\xD4V[RaK\xAAaK\xA2\x83\x87a \xDAV[QQ` \x01\x90V[QaK\xB7aKt\x83a.\xE2V[R` aK\xC4\x83\x87a \xDAV[Q\x01QQaK\xD4aKt\x83a.\xF0V[RaL\0aK\xFAaK\xF3` aK\xEA\x86\x8Aa \xDAV[Q\x01Q` \x01\x90V[Q\x92a.\xFEV[\x88aJ\xC8V[R\x01aK\x0CV[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01aLS`\xFF\x93aO7V[\x92\x83\x92\x16\x1B\x11\x15aLaW\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[aL|WP\x90V[_\x19\x81\x01\x81\x81\x11a CWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a CW`\x01\x01\x90\x80aLtV[\x90aL\xAAaG\xA6V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aMOW`\x01\x82\x14aMJWaL\xCBa\x06\x01V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aL\xF0WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aM*W[`\x01aM aM\x15\x83`\xFF\x94aH0V[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aL\xDCV[\x94`\x01aM aM\x15aM?\x89`\xFF\x95aH0V[\x98\x93PPPPaM\x04V[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[aMfaG\xA6V[P\x80Q\x90\x81\x15\x80aM\xD7W[\x15aM\x93WPP`@QaM\x87`@\x82a\x05\xD0V[_\x81R_` \x82\x01R\x90V[` _Q` aO\xC6_9_Q\x90_R\x91\x01Q\x06_Q` aO\xC6_9_Q\x90_R\x03_Q` aO\xC6_9_Q\x90_R\x81\x11a CW`@Q\x91aI\x7F\x83a\x05\x95V[P` \x81\x01Q\x15aMrV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aM\xF7WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\xFF\xFFaN\x8F\x82aLpV[\x16aN\x99\x81a\t\xAEV[\x90aN\xA7`@Q\x92\x83a\x05\xD0V[\x80\x82RaN\xB6`\x1F\x19\x91a\t\xAEV[\x016` \x83\x017__[\x82Q\x82\x10\x80aO\x16W[\x15aO\x0FW`\x01\x81\x1B\x84\x16aN\xE8W[aN\xE3\x90a)~V[aN\xC0V[\x90`\x01aN\xE3\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaO\x05\x82\x87a#\x91V[S\x01\x91\x90PaN\xDAV[PP\x90P\x90V[Pa\x01\0\x81\x10aN\xCAV[\x15aO(WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aO\xA0W\x81Q\x15aO\x9BW` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aO\x96W`\x01\x90aO\x81aOwa%ea%W\x86\x89a#\x91V[`\xFF`\x01\x91\x16\x1B\x90V[\x90aO\x8D\x81\x83\x11aO!V[\x17\x91\x01\x90aOXV[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15aO\xB6WV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \n\x93\xAF\x99r\x1B\xDD\xAF\xED\x02R\xF39z\x1A\xBB\xA4\x0B\x90sW\xC2\x10\xB7\xC4\xEEe\xB3\xA40\xE3EdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8063018630671461029a578063136439dd14610295578063171f1d5b146102905780631ad43189146101e6578063245a7bfc1461028b5780632cb223d5146102865780632d89f6fc1461028157806331b36bd91461027c5780633563b0d1146102775780633998fdd314610272578063416c7e5e1461026d5780634d2b57fe146102685780634f739f7414610263578063595c6a671461025e5780635a2d7f02146102595780635ac86ab7146102545780635c1556621461024f5780635c975abb1461024a5780635decc3f5146102455780635df4594614610240578063683048351461023b5780636d14a987146102365780636efb463614610231578063715018a61461022c57806372d18e8d1461021d5780637afa1eed14610227578063886f1195146102225780638b00ce7c1461021d5780638da5cb5b146102185780639b290e9814610213578063b490bb411461020e578063b98d090814610209578063c9b1479714610204578063ca8aa7c7146101ff578063cc2a9a5b146101fa578063cefdc1d4146101f5578063df5cf723146101f0578063f2fde38b146101eb578063f5c9899d146101e6578063f63c5bab146101e15763fabc1cbc146101dc575f80fd5b611df6565b611ddb565b6107b3565b611d4a565b611d06565b611bc2565b611a82565b611a5a565b6117cb565b6117a9565b611738565b611702565b6116da565b61164b565b611696565b61166e565b6115f0565b611543565b611266565b611222565b6111de565b6111a0565b611183565b61100a565b610fd7565b610faa565b610f37565b610e90565b610cb2565b610b52565b610b20565b610aa6565b6108fc565b610854565b61081b565b6107f3565b610741565b6104c3565b61030a565b60409060231901126102b057602490565b5f80fd5b908160409103126102b05790565b63ffffffff8116036102b057565b35906102db826102c2565b565b9181601f840112156102b0578235916001600160401b0383116102b057602083818601950101116102b057565b346102b05760603660031901126102b0576004356001600160401b0381116102b05761033a9036906004016102b4565b60243590610347826102c2565b6044356001600160401b0381116102b0576103669036906004016102dd565b60ce5491939092916001600160a01b03163303610474576104729361045c936103ba6103c19361039f610397611ef1565b963690611f1c565b86524363ffffffff16602087015263ffffffff166060860152565b36916109c9565b604082015260405160208101906103ea816103dc8585611fa5565b03601f1981018352826105d0565b5190206104136103ff60c95463ffffffff1690565b63ffffffff165f5260ca60205260405f2090565b5560c95463ffffffff16907ff3d0298607cbbde38baba3c3915d277f1ffd80a384095a02a4a4d7c082ae6cd96040518061045463ffffffff86169482611fa5565b0390a261202b565b63ffffffff1663ffffffff1960c954161760c955565b005b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b346102b05760203660031901126102b05760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561057c5761047292610539915f9161054d575b5061209a565b610548606654828116146120b0565b614774565b61056f915060203d602011610575575b61056781836105d0565b81019061207a565b5f610533565b503d61055d565b61208f565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176105b057604052565b610581565b608081019081106001600160401b038211176105b057604052565b90601f801991011681019081106001600160401b038211176105b057604052565b604051906102db610100836105d0565b604051906102db6040836105d0565b604051906102db6060836105d0565b604051906102db60a0836105d0565b906102db60405192836105d0565b60409060e31901126102b0576040519061065582610595565b60e4358252610104356020830152565b91908260409103126102b05760405161067d81610595565b6020808294803584520135910152565b9080601f830112156102b057604051916106a86040846105d0565b8290604081019283116102b057905b8282106106c45750505090565b81358152602091820191016106b7565b9060806063198301126102b0576040516106ed81610595565b602061070882946106ff81606461068d565b845260a461068d565b910152565b91906080838203126102b05760206107086040519261072b84610595565b60408496610739838261068d565b86520161068d565b346102b0576101203660031901126102b05760043560403660231901126102b057610799604091825161077381610595565b60243581526044356020820152610789366106d4565b906107933661063c565b92612104565b8251911515825215156020820152f35b5f9103126102b057565b346102b0575f3660031901126102b057602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346102b0575f3660031901126102b05760cd546040516001600160a01b039091168152602090f35b346102b05760203660031901126102b05763ffffffff60043561083d816102c2565b165f5260cb602052602060405f2054604051908152f35b346102b05760203660031901126102b05763ffffffff600435610876816102c2565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b038116036102b057565b6001600160401b0381116105b05760051b60200190565b90602080835192838152019201905f5b8181106108d25750505090565b82518452602093840193909201916001016108c5565b9060206108f99281815201906108b5565b90565b346102b05760403660031901126102b0576004356109198161088d565b602435906001600160401b0382116102b057366023830112156102b0578160040135916109458361089e565b9261095360405194856105d0565b8084526024602085019160051b830101913683116102b057602401905b82821061099457610990610984868661226e565b604051918291826108e8565b0390f35b6020809183356109a38161088d565b815201910190610970565b6001600160401b0381116105b057601f01601f191660200190565b9291926109d5826109ae565b916109e360405193846105d0565b8294818452818301116102b0578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b838310610a2a57505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b808210610a6a5750505060208060019297019301930191939290610a1b565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610a4b565b346102b05760603660031901126102b057600435610ac38161088d565b6024356001600160401b0381116102b057366023820112156102b05761099091610afa610b0c9236906024816004013591016109c9565b60443591610b07836102c2565b6124ac565b6040519182916020835260208301906109ff565b346102b0575f3660031901126102b05760d1546040516001600160a01b039091168152602090f35b801515036102b057565b346102b05760203660031901126102b057600435610b6f81610b48565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c575f91610be4575b506001600160a01b03163303610bd55761047290614c07565b637070f3b160e11b5f5260045ffd5b610c06915060203d602011610c0c575b610bfe81836105d0565b810190612333565b5f610bbc565b503d610bf4565b9080601f830112156102b0578135610c2a8161089e565b92610c3860405194856105d0565b81845260208085019260051b8201019283116102b057602001905b828210610c605750505090565b8135815260209182019101610c53565b60206040818301928281528451809452019201905f5b818110610c935750505090565b82516001600160a01b0316845260209384019390920191600101610c86565b346102b05760403660031901126102b057600435610ccf8161088d565b6024356001600160401b0381116102b057610cee903690600401610c13565b610cf8815161220c565b916001600160a01b03165f5b8251811015610d9557806020610d1d610d3d938661224b565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561057c57600192610d71915f91610d77575b50610d62838861224b565b6001600160a01b039091169052565b01610d04565b610d8f915060203d8111610c0c57610bfe81836105d0565b5f610d57565b604051806109908682610c70565b90602080835192838152019201905f5b818110610dc05750505090565b825163ffffffff16845260209384019390920191600101610db3565b90602082526060610e2a610e15610dff84516080602088015260a0870190610da3565b6020850151868203601f19016040880152610da3565b6040840151858203601f190184870152610da3565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610e6357505050505090565b9091929394602080610e81600193601f198682030187528951610da3565b97019301930191939290610e54565b346102b05760803660031901126102b057600435610ead8161088d565b60243590610eba826102c2565b6044356001600160401b0381116102b057610ed99036906004016102dd565b91606435926001600160401b0384116102b057366023850112156102b0578360040135926001600160401b0384116102b0573660248560051b870101116102b057610990956024610f2b9601936129b1565b60405191829182610ddc565b346102b0575f3660031901126102b05760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561057c57610fa2915f9161054d575061209a565b610472614740565b346102b0575f3660031901126102b057602060405167016345785d8a00008152f35b60ff8116036102b057565b346102b05760203660031901126102b0576020600160ff600435610ffa81610fcc565b161b806066541614604051908152f35b346102b05760603660031901126102b0576004356110278161088d565b6024356001600160401b0381116102b057611046903690600401610c13565b60443591611053836102c2565b6040516361c8a12f60e11b8152906001600160a01b03165f828061107b868860048401612e19565b0381845afa91821561057c575f9261115f575b50611099835161220c565b935f5b8451811015611151576110af818661224b565b51906020836110cb6110c1848961224b565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa801561057c576001925f91611123575b50828060c01b031661111c828961224b565b520161109c565b611144915060203d811161114a575b61113c81836105d0565b810190612928565b5f61110a565b503d611132565b6040518061099088826108e8565b61117c9192503d805f833e61117481836105d0565b810190612806565b905f61108e565b346102b0575f3660031901126102b0576020606654604051908152f35b346102b05760203660031901126102b05763ffffffff6004356111c2816102c2565b165f5260cc602052602060ff60405f2054166040519015158152f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080601f830112156102b05781356112c18161089e565b926112cf60405194856105d0565b81845260208085019260051b8201019283116102b057602001905b8282106112f75750505090565b602080918335611306816102c2565b8152019101906112ea565b81601f820112156102b05780356113278161089e565b9261133560405194856105d0565b81845260208085019260061b840101928184116102b057602001915b83831061135f575050505090565b602060409161136e8486610665565b815201920191611351565b9080601f830112156102b05781356113908161089e565b9261139e60405194856105d0565b81845260208085019260051b820101918383116102b05760208201905b8382106113ca57505050505090565b81356001600160401b0381116102b0576020916113ec878480948801016112aa565b8152019101906113bb565b919091610180818403126102b05761140d6105f1565b9281356001600160401b0381116102b0578161142a9184016112aa565b845260208201356001600160401b0381116102b0578161144b918401611311565b602085015260408201356001600160401b0381116102b0578161146f918401611311565b6040850152611481816060840161070d565b60608501526114938160e08401610665565b60808501526101208201356001600160401b0381116102b057816114b89184016112aa565b60a08501526101408201356001600160401b0381116102b057816114dd9184016112aa565b60c08501526101608201356001600160401b0381116102b0576115009201611379565b60e0830152565b90602080835192838152019201905f5b8181106115245750505090565b82516001600160601b0316845260209384019390920191600101611517565b346102b05760803660031901126102b0576004356024356001600160401b0381116102b0576115769036906004016102dd565b9091604435611584816102c2565b606435926001600160401b0384116102b0576115e6946115ab6115b19536906004016113f7565b93612ff2565b6040519283926040845260206115d282516040808801526080870190611507565b910151848203603f19016060860152611507565b9060208301520390f35b346102b0575f3660031901126102b057611608614de3565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346102b0575f3660031901126102b057602063ffffffff60c95416604051908152f35b346102b0575f3660031901126102b05760ce546040516001600160a01b039091168152602090f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b0575f3660031901126102b0576033546040516001600160a01b039091168152602090f35b346102b0575f3660031901126102b05760cf546040516001600160a01b039091168152602090f35b908160809103126102b05790565b346102b05760c03660031901126102b0576004356001600160401b0381116102b05761176890369060040161172a565b6117713661029f565b9060403660631901126102b05760a4356001600160401b0381116102b057610472926117a36064923690600401611311565b92613d0a565b346102b0575f3660031901126102b057602060ff609754166040519015158152f35b346102b05760803660031901126102b0576004356001600160401b0381116102b0576117fb90369060040161172a565b6118043661029f565b906064356001600160401b0381116102b0576118249036906004016113f7565b60cd549092906001600160a01b03163303611a155761184760208394930161388f565b916119286118586040860186613b81565b92909461189661186a6060890161388f565b97604051611880816103dc6020820194856143b9565b51902061188f6103ff8861388f565b5414614471565b6118c06118b96118a58761388f565b63ffffffff165f5260cb60205260405f2090565b54156144e3565b8363ffffffff43169661190a6119026118f97f000000000000000000000000000000000000000000000000000000000000000086612060565b63ffffffff1690565b891115614544565b6040516020810190611920816103dc8b856145a6565b519020612ff2565b919060ff5f9616955b8281106119b3577f349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a868686611973611967610601565b63ffffffff9094168452565b6020830152604051602081019061198f816103dc86868661464a565b51902061199e6118a58361388f565b556119ae6040519283928361464a565b0390a1005b80611a0f6119eb6119e66119da6119cd600196885161224b565b516001600160601b031690565b6001600160601b031690565b613ab7565b611a086119da8b611a036119cd8760208b015161224b565b6145b6565b11156145d9565b01611931565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b346102b0575f3660031901126102b05760d0546040516001600160a01b039091168152602090f35b346102b05760c03660031901126102b057600435611a9f8161088d565b611b1f602435611aae8161088d565b604435611aba8161088d565b606435611ac68161088d565b60843591611ad38361088d565b60a43593611ae08561088d565b5f5496611b0560ff60088a901c16158099819a611b9d575b8115611b7d575b50614674565b87611b16600160ff195f5416175f55565b611b66576146d7565b611b2557005b611b3361ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016119ae565b611b7861010061ff00195f5416175f55565b6146d7565b303b15915081611b8f575b505f611aff565b60ff1660011490505f611b88565b600160ff8216109150611af8565b6040906108f99392815281602082015201906109ff565b346102b05760603660031901126102b057600435611bdf8161088d565b602435604435611bee816102c2565b611c2f611bf96121ea565b9280611c048561223e565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401612e19565b0381875afa93841561057c5783611c596118f96110c1611c8e986020975f91611cec575b5061223e565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561057c57611cbd925f91611ccd575b506001600160c01b031692611cb784614e83565b906124ac565b9061099060405192839283611bab565b611ce6915060203d60201161114a5761113c81836105d0565b5f611ca3565b611d0091503d805f833e61117481836105d0565b5f611c53565b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b05760203660031901126102b057600435611d678161088d565b611d6f614de3565b6001600160a01b03811615611d875761047290614e3b565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346102b0575f3660031901126102b057602060405160648152f35b346102b05760203660031901126102b05760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c575f91611eb9575b506001600160a01b03163303611eaa57611e786066541982198116146120b0565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b611ed2915060203d602011610c0c57610bfe81836105d0565b5f611e57565b60405190611ee582610595565b60606020838281520152565b60405190611efe826105b5565b5f606083611f0a611ed8565b81528260208201528160408201520152565b91906040838203126102b05760405190611f3582610595565b819380356001600160401b0381116102b05782611f53918301610c13565b83526020810135916001600160401b0383116102b0576020926107089201610c13565b6108f9903690611f1c565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b60208152608063ffffffff606061200d611fec86518560208801526020611fd88251604060a08b015260e08a01906108b5565b910151878203609f190160c08901526108b5565b8360208801511660408701526040870151601f198783030184880152611f81565b9401511691015290565b634e487b7160e01b5f52601160045260245ffd5b63ffffffff60019116019063ffffffff821161204357565b612017565b63ffffffff60649116019063ffffffff821161204357565b9063ffffffff8091169116019063ffffffff821161204357565b908160209103126102b057516108f981610b48565b6040513d5f823e3d90fd5b156120a157565b631d77d47760e21b5f5260045ffd5b156120b757565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b9060028110156120eb5760051b0190565b6120c6565b634e487b7160e01b5f52601260045260245ffd5b6121e06121bd6121e6956121b76121b085875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e084015261010083015261218781610120840103601f1981018352826105d0565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b80966147ea565b90614830565b926121b76121d26121cc614892565b94614989565b916121db614aa5565b6147ea565b91614ad9565b9091565b604080519091906121fb83826105d0565b6001815291601f1901366020840137565b906122168261089e565b61222360405191826105d0565b8281528092612234601f199161089e565b0190602036910137565b8051156120eb5760200190565b80518210156120eb5760209160051b010190565b908160209103126102b0575190565b91909161227b835161220c565b925f5b815181101561232e578060206122a761229a6122d0948661224b565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa801561057c576001925f91612300575b506122f9828861224b565b520161227e565b612321915060203d8111612327575b61231981836105d0565b81019061225f565b5f6122ee565b503d61230f565b505050565b908160209103126102b057516108f98161088d565b906123528261089e565b61235f60405191826105d0565b82815260208193612372601f199161089e565b0191015f5b82811061238357505050565b606082820152602001612377565b9081518110156120eb570160200190565b6020818303126102b0578051906001600160401b0382116102b057019080601f830112156102b05781516123d58161089e565b926123e360405194856105d0565b81845260208085019260051b8201019283116102b057602001905b82821061240b5750505090565b81518152602091820191016123fe565b906124258261089e565b61243260405191826105d0565b8281528092612443601f199161089e565b015f5b81811061245257505050565b6040519060608201918083106001600160401b038411176105b0576020926040525f81525f838201525f604082015282828601015201612446565b908160209103126102b057516001600160601b03811681036102b05790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa94851561057c575f956127c1575b50604051634f4c91e160e11b815294602086600481855afa91821561057c576004965f9361279f575b5060209060405197888092632efa2ca360e11b82525afa95861561057c575f9661277e575b5061253a8593929551612348565b945f935b80518510156127745761256b6125656125578784612391565b516001600160f81b03191690565b60f81c90565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa93841561057c575f94612750575b506125bb845161241b565b6125c5888b61224b565b526125d0878a61224b565b505f5b845181101561273f578060206125ec61260e938861224b565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa91821561057c575f9261271f575b50612634818761224b565b518a60208a612643858b61224b565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa93841561057c576126d68c8f6126d16001986126e89789975f926126ef575b506126bc6126ad610610565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b61224b565b51906126e2838361224b565b5261224b565b50016125d3565b61271191925060203d8111612718575b61270981836105d0565b81019061248d565b905f6126a1565b503d6126ff565b61273891925060203d8111610c0c57610bfe81836105d0565b905f612629565b50600190960195909450915061253e565b61276d9194503d805f833e61276581836105d0565b8101906123a2565b925f6125b0565b5050509350505090565b61279891965060203d602011610c0c57610bfe81836105d0565b945f61252c565b60209193506127ba90823d8411610c0c57610bfe81836105d0565b9290612507565b6127db91955060203d602011610c0c57610bfe81836105d0565b935f6124de565b604051906127ef826105b5565b606080838181528160208201528160408201520152565b6020818303126102b0578051906001600160401b0382116102b057019080601f830112156102b05781516128398161089e565b9261284760405194856105d0565b81845260208085019260051b8201019283116102b057602001905b82821061286f5750505090565b60208091835161287e816102c2565b815201910190612862565b81835290916001600160fb1b0383116102b05760209260051b809284830137010190565b60409063ffffffff6108f995931681528160208201520191612889565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6108f9959316815281602082015201916128ca565b60ff1660ff81146120435760010190565b91908110156120eb5760051b0190565b908160209103126102b057516001600160c01b03811681036102b05790565b1561294e57565b6325ec6c1f60e01b5f5260045ffd5b908210156120eb570190565b908160209103126102b057516108f9816102c2565b5f1981146120435760010190565b916129aa60209263ffffffff929695966040865260408601916128ca565b9416910152565b95939495929091926129c16127e2565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa94851561057c575f95612df8575b506129fe6127e2565b946040516361c8a12f60e11b81525f8180612a1e8d8d8b600485016128ad565b0381885afa90811561057c575f91612dde575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f8180612a6185878b600485016128ea565b0381875afa90811561057c575f91612dc4575b506040870152612a8381612348565b9860608701998a525f5b60ff811683811015612d0f57885f612ab6838f612aa98861220c565b9051906126e2838361224b565b505f8a868f5b818410612b39575050505090508c612ad38261220c565b915f5b818110612b0057505091612af591612afb949351906126e2838361224b565b50612907565b612a8d565b80612b33612b1e6110c1600194612b188a895161224b565b5161224b565b612b28838861224b565b9063ffffffff169052565b01612ad6565b6110c184612b4e8160209695612b5695612918565b35975161224b565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561057c57888f888a918f94612bfb6001612bee81938d809d5f92612ce3575b50612565612bca612bd892612bc3878060c01b0386161515612947565b8b8d61295d565b356001600160f81b03191690565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14612c17575b5050505050600191925001908a918a868f612abc565b8597612c3993612c32602097999861256595612bca95612918565b359561295d565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa90811561057c578f612c9790612c9c9383886001975f93612cab575b50612b1890612b2893945161224b565b61297e565b905082918a888f888a91612c01565b612b28935090612cd4612b189260203d8111612cdc575b612ccc81836105d0565b810190612969565b935090612c87565b503d612cc2565b612bd8919250612bca612d066125659260203d811161114a5761113c81836105d0565b93925050612ba6565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa90811561057c57612d65945f948593612da3575b5060405163354952a360e21b8152958694859384936004850161298c565b03916001600160a01b03165afa90811561057c575f91612d89575b50602082015290565b612d9d91503d805f833e61117481836105d0565b5f612d80565b612dbd91935060203d602011610c0c57610bfe81836105d0565b915f612d47565b612dd891503d805f833e61117481836105d0565b5f612a74565b612df291503d805f833e61117481836105d0565b5f612a31565b612e1291955060203d602011610c0c57610bfe81836105d0565b935f6129f5565b60409063ffffffff6108f9949316815281602082015201906108b5565b15612e3d57565b62f8202d60e51b5f5260045ffd5b15612e5257565b6343714afd60e01b5f5260045ffd5b15612e6857565b635f832f4160e01b5f5260045ffd5b15612e7e57565b634b874f4560e01b5f5260045ffd5b908160209103126102b057516108f981610fcc565b5f1981019190821161204357565b15612eb757565b633fdc650560e21b5f5260045ffd5b906001820180921161204357565b906002820180921161204357565b906003820180921161204357565b906004820180921161204357565b906005820180921161204357565b9190820180921161204357565b15612f2057565b63affc5edb60e01b5f5260045ffd5b908160209103126102b0575167ffffffffffffffff19811681036102b05790565b15612f5757565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b03821161204357565b15612f8d57565b6367988d3360e01b5f5260045ffd5b15612fa357565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b818110612fdc5750505090565b8251845260209384019390920191600101612fcf565b949392909193613000611ed8565b5061300c851515612e36565b604084015151851480613881575b80613873575b80613865575b61302f90612e4b565b61304160208501515185515114612e61565b61305863ffffffff431663ffffffff841610612e77565b613060610601565b5f81525f602082015292613072611ed8565b61307b8761220c565b60208201526130898761220c565b8152613093611ed8565b926130a260208801515161220c565b84526130b260208801515161220c565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561057c5761311b915f91613836575b50613116368b876109c9565b614c45565b985f965b6020890151805189101561328d576020886131826110c18c61317a8f96868e61315f61314c86809561224b565b5180515f526020015160205260405f2090565b61316c848484015161224b565b528261325a575b015161224b565b51955161224b565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa91821561057c576121b78a61322f8f6132288f8460208f9261321f936132178460019e6132359e5f9161323d575b508f8060c01b0316925161224b565b52015161224b565b51938d5161224b565b5116614c70565b90614ca1565b97019661311f565b6132549150863d811161114a5761113c81836105d0565b5f613208565b61328861326a848484015161224b565b516132818484015161327b87612ea2565b9061224b565b5110612eb0565b613173565b509095979496506132a2919893929950614d5e565b916132af60975460ff1690565b90811561382e576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c575f9161380f575b5091905b5f925b8184106133605750505050509261334761334261333b61335a95856103dc9860806060602099015192015192612104565b9190612f86565b612f9c565b0151604051928391602083019586612fb2565b51902090565b92989596909399919794878b888c888d613709575b6110c18260a06133b5612565612bca846133bd976133af6133a161314c8f9c604060209f9e015161224b565b67ffffffffffffffff191690565b9b61295d565b97015161224b565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561057c576134816110c18f958f906134798f978f96848f61347360c09661346c848f60209f90613173612bca996040936125659c5f916136db575b5067ffffffffffffffff19918216911614612f50565b5190614830565b9c61295d565b96015161224b565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561057c5761350e918c8f925f926136b7575b5060206135009293015161224b565b906001600160601b03169052565b61352e8c6135008c6135276119cd82602086015161224b565b925161224b565b5f985f5b60208a01515181101561369e578b8d61357089613563612565612bca868f8961355b915161224b565b51948761295d565b60ff161c60019081161490565b61357f575b5050600101613532565b8a8a613601859f948f9686612b188f9360e06135b86110c19560206135b0612565612bca839f6135c19c899161295d565b9a015161224b565b519b015161224b565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c578f61366d908f936001959486955f92613678575b50613667613500929351936136626119cd848761224b565b612f66565b9261224b565b019a90508b8d613575565b61350092506136976136679160203d81116127185761270981836105d0565b925061364a565b5093919796996001919699509a94929a0192919061330a565b61350092506136d4602091823d81116127185761270981836105d0565b92506134f1565b60206136fc92503d8111613702575b6136f481836105d0565b810190612f2f565b5f613456565b503d6136ea565b6137469450613723925061256591612bca9160209561295d565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561057c576020896133bd8f938f60a08f97612565612bca8f8f906133af6133a161314c8f60408b96918f88936110c19f6137ca906137d0936133b59f5f926137e6575b5063ffffffff809116931690612f0c565b11612f19565b5050505050509750505050505092935050613375565b602063ffffffff9293508291613807913d81116123275761231981836105d0565b9291506137b9565b613828915060203d602011612cdc57612ccc81836105d0565b5f613303565b5f9190613307565b613858915060203d60201161385e575b61385081836105d0565b810190612e8d565b5f61310a565b503d613846565b5060e0840151518514613026565b5060c0840151518514613020565b5060a084015151851461301a565b356108f9816102c2565b903590603e19813603018212156102b0570190565b156138b557565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b6020809163ffffffff8135613918816102c2565b1684520135910152565b9092916020606091613938846080810197613904565b63ffffffff8135613948816102c2565b1660408501520135910152565b1561395c57565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156139ce57565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b15613a4c57565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b9060648202918083046064149015171561204357565b9060068202918083046006149015171561204357565b8181029291811591840414171561204357565b15613afd57565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b903590601e19813603018212156102b057018035906001600160401b0382116102b0576020019181360383136102b057565b6020818303126102b0578051906001600160401b0382116102b057019080601f830112156102b0578151613be68161089e565b92613bf460405194856105d0565b81845260208085019260051b8201019283116102b057602001905b828210613c1c5750505090565b602080918351613c2b8161088d565b815201910190613c0f565b60405190613c456040836105d0565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b818110613ceb575050506080613cd66108f994956060850151601f1985830301848601526108b5565b9201519060a0601f1982850301910152611f81565b82516001600160a01b0316875260209687019690920191600101613cad565b9193929093613d188561388f565b93613d2b613d268580613899565b611f76565b94613d4f613d478263ffffffff165f5260cb60205260405f2090565b5415156138ae565b613d8b613d6a8263ffffffff165f5260cb60205260405f2090565b5488604051613d82816103dc89602083019586613922565b51902014613955565b613db6613db0613da98363ffffffff165f5260cc60205260405f2090565b5460ff1690565b156139c7565b613ddb613dcd6118f9613dc88661388f565b612048565b63ffffffff43161115613a45565b5f935f9560208801955b88518051891015613e2457600191613e16613e038b613e1c9461224b565b51613e0f8c8c5161224b565b5190613ae3565b90612f0c565b970196613de5565b50929750945094506020600192970135141461432357613e44835161220c565b935f5b8451811015613e715780613e6061314c6001938861224b565b613e6a828961224b565b5201613e47565b509092939194613eab60208701946020613e8a8761388f565b604051613e9f816103dc8a8683019586612fb2565b51902091013514613af6565b613eb5855161220c565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b8751811015613f6557806020613efc613f1c938961224b565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa91821561057c57600192613f41915f91613f47575b50610d62838d61224b565b01613ee3565b613f5f915060203d8111610c0c57610bfe81836105d0565b5f613f36565b5092969550925092613fc4613f8d613f956040860194613f858688613b81565b93909161388f565b9236916109c9565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166124ac565b945f915b86518310156142c5575f979697955b613fe1848a61224b565b51518710156142b55797614022986020806140008a612b18898761224b565b510151604051809c81926308f6629d60e31b8352600483019190602083019252565b0381865afa998a1561057c575f9a614295575b506001985f5b85518110156142875761406061405461229a838961224b565b6001600160a01b031690565b6001600160a01b038d16146140775760010161403b565b5099909791985060015f5b151514614097575b5060010195979697613fd7565b98969786985f879593986140fc60ff6140d4612565612bca8c6140ce6141519f6140c860d15460018060a01b031690565b98613b81565b9061295d565b6140ee6140df610601565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d05461411390614054906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529098899190829081906044820190565b03915afa96871561057c575f97614263575b5061416e875161220c565b985f5b8a51811015614197578067016345785d8a00006141906001938e61224b565b5201614171565b509a9294966141d560ff6141bc612565612bca8f9d979f969e966140ce8e8e92613b81565b6141c76126ad61061f565b1663ffffffff166020860152565b604084015260608301526141e7613c36565b608083015260cf5461420390614054906001600160a01b031681565b803b156102b057604051636a669b4160e01b8152925f91849182908490829061422f9060048301613c66565b03925af191821561057c57600192614249575b509061408a565b806142575f61425d936105d0565b806107a9565b5f614242565b6142809197503d805f833e61427881836105d0565b810190613bb3565b955f614163565b509990979198600190614082565b6142ae919a5060203d8111610c0c57610bfe81836105d0565b985f614035565b9697969550600190920191613fc8565b50505050509190506142f56142e88263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b50505063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b9035601e19823603018112156102b05701602081359101916001600160401b0382116102b0578160051b360383136102b057565b9035601e19823603018112156102b05701602081359101916001600160401b0382116102b05781360383136102b057565b60208152813590603e19833603018212156102b0576080614466606061445f614425876108f997018560208801526144136144086143f78380614354565b604060a08c015260e08b0191612889565b916020810190614354565b888303609f190160c08a015290612889565b61444161443460208a016102d0565b63ffffffff166040880152565b61444e6040890189614388565b878303601f190185890152906128ca565b95016102d0565b63ffffffff16910152565b1561447857565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156144ea57565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b1561454b57565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6040810192916102db9190613904565b906001600160601b03809116911602906001600160601b03821691820361204357565b156145e057565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091614660846080810197613904565b63ffffffff81511660408501520151910152565b1561467b57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6146e090614e3b565b60cd80546001600160a01b03199081166001600160a01b039384161790915560ce805482169383169390931790925560d0805483169382169390931790925560cf805482169383169390931790925560d180549092169216919091179055565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b604051906147b382610595565b5f6020838281520152565b604051906101806147cf81846105d0565b368337565b604051906147e36020836105d0565b6020368337565b919060409060606147f96147a6565b948592602085519261480b85856105d0565b8436853780518452015160208301528482015260076107cf195a01fa1561482e57565bfe5b6020929160806040926148416147a6565b9586938186519361485286866105d0565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa801561482e571561488357565b63d4b68fd760e01b5f5260045ffd5b60405161489e81610595565b60409081516148ad83826105d0565b82368237815260208251916148c284846105d0565b83368437015280516148d482826105d0565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061492a83836105d0565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d602083015261497f835193846105d0565b8252602082015290565b5f516020614fc65f395f51905f52906149a06147a6565b505f919006602060c0835b614aa0575f935f516020614fc65f395f51905f52600381868181800909086040516149d685826105d0565b843682378481856040516149ea82826105d0565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614fc65f395f51905f5260a082015260056107cf195a01fa801561482e57614a5490614faf565b5191614aa0575f516020614fc65f395f51905f5282800914614a8b57505f516020614fc65f395f51905f5260015f940892936149ab565b92935050614a97610601565b92835282015290565b6120f0565b614aad6147a6565b50604051614aba81610595565b600181526002602082015290565b90600c8110156120eb5760051b0190565b93929091614ae7604061062e565b9485526020850152614af9604061062e565b9182526020820152614b096147be565b925f5b60028110614b3657505050602061018092614b256147d4565b93849160086201d4c0fa9151151590565b80614b42600192613acd565b614b4c82856120da565b5151614b588289614ac8565b526020614b6583866120da565b510151614b7a614b7483612ec6565b89614ac8565b52614b8582866120da565b515151614b94614b7483612ed4565b52614baa614ba283876120da565b515160200190565b51614bb7614b7483612ee2565b526020614bc483876120da565b51015151614bd4614b7483612ef0565b52614c00614bfa614bf36020614bea868a6120da565b51015160200190565b5192612efe565b88614ac8565b5201614b0c565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614c5360ff93614f37565b928392161b1115614c615790565b63ca95733360e01b5f5260045ffd5b805f915b614c7c575090565b5f1981018181116120435761ffff9116911661ffff8114612043576001019080614c74565b90614caa6147a6565b5061ffff811690610200821015614d4f5760018214614d4a57614ccb610601565b5f81525f602082015292906001905f925b61ffff8316851015614cf057505050505090565b600161ffff831660ff86161c811614614d2a575b6001614d20614d158360ff94614830565b9460011b61fffe1690565b9401169291614cdc565b946001614d20614d15614d3f8960ff95614830565b989350505050614d04565b505090565b637fc4ea7d60e11b5f5260045ffd5b614d666147a6565b50805190811580614dd7575b15614d93575050604051614d876040826105d0565b5f81525f602082015290565b60205f516020614fc65f395f51905f52910151065f516020614fc65f395f51905f52035f516020614fc65f395f51905f528111612043576040519161497f83610595565b50602081015115614d72565b6033546001600160a01b03163303614df757565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff614e8f82614c70565b16614e99816109ae565b90614ea760405192836105d0565b808252614eb6601f19916109ae565b013660208301375f5f5b8251821080614f16575b15614f0f576001811b8416614ee8575b614ee39061297e565b614ec0565b906001614ee39160ff60f81b8460f81b165f1a614f058287612391565b5301919050614eda565b5050905090565b506101008110614eca565b15614f2857565b631019106960e31b5f5260045ffd5b90610100825111614fa057815115614f9b57602082015160019060f81c81901b5b8351821015614f9657600190614f81614f776125656125578689612391565b60ff600191161b90565b90614f8d818311614f21565b17910190614f58565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b15614fb657565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212200a93af99721bddafed0252f3397a1abba40b907357c210b7c4ee65b3a430e34564736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x01\x860g\x14a\x02\x9AW\x80c\x13d9\xDD\x14a\x02\x95W\x80c\x17\x1F\x1D[\x14a\x02\x90W\x80c\x1A\xD41\x89\x14a\x01\xE6W\x80c$Z{\xFC\x14a\x02\x8BW\x80c,\xB2#\xD5\x14a\x02\x86W\x80c-\x89\xF6\xFC\x14a\x02\x81W\x80c1\xB3k\xD9\x14a\x02|W\x80c5c\xB0\xD1\x14a\x02wW\x80c9\x98\xFD\xD3\x14a\x02rW\x80cAl~^\x14a\x02mW\x80cM+W\xFE\x14a\x02hW\x80cOs\x9Ft\x14a\x02cW\x80cY\\jg\x14a\x02^W\x80cZ-\x7F\x02\x14a\x02YW\x80cZ\xC8j\xB7\x14a\x02TW\x80c\\\x15Vb\x14a\x02OW\x80c\\\x97Z\xBB\x14a\x02JW\x80c]\xEC\xC3\xF5\x14a\x02EW\x80c]\xF4YF\x14a\x02@W\x80ch0H5\x14a\x02;W\x80cm\x14\xA9\x87\x14a\x026W\x80cn\xFBF6\x14a\x021W\x80cqP\x18\xA6\x14a\x02,W\x80cr\xD1\x8E\x8D\x14a\x02\x1DW\x80cz\xFA\x1E\xED\x14a\x02'W\x80c\x88o\x11\x95\x14a\x02\"W\x80c\x8B\0\xCE|\x14a\x02\x1DW\x80c\x8D\xA5\xCB[\x14a\x02\x18W\x80c\x9B)\x0E\x98\x14a\x02\x13W\x80c\xB4\x90\xBBA\x14a\x02\x0EW\x80c\xB9\x8D\t\x08\x14a\x02\tW\x80c\xC9\xB1G\x97\x14a\x02\x04W\x80c\xCA\x8A\xA7\xC7\x14a\x01\xFFW\x80c\xCC*\x9A[\x14a\x01\xFAW\x80c\xCE\xFD\xC1\xD4\x14a\x01\xF5W\x80c\xDF\\\xF7#\x14a\x01\xF0W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xEBW\x80c\xF5\xC9\x89\x9D\x14a\x01\xE6W\x80c\xF6<[\xAB\x14a\x01\xE1Wc\xFA\xBC\x1C\xBC\x14a\x01\xDCW_\x80\xFD[a\x1D\xF6V[a\x1D\xDBV[a\x07\xB3V[a\x1DJV[a\x1D\x06V[a\x1B\xC2V[a\x1A\x82V[a\x1AZV[a\x17\xCBV[a\x17\xA9V[a\x178V[a\x17\x02V[a\x16\xDAV[a\x16KV[a\x16\x96V[a\x16nV[a\x15\xF0V[a\x15CV[a\x12fV[a\x12\"V[a\x11\xDEV[a\x11\xA0V[a\x11\x83V[a\x10\nV[a\x0F\xD7V[a\x0F\xAAV[a\x0F7V[a\x0E\x90V[a\x0C\xB2V[a\x0BRV[a\x0B V[a\n\xA6V[a\x08\xFCV[a\x08TV[a\x08\x1BV[a\x07\xF3V[a\x07AV[a\x04\xC3V[a\x03\nV[`@\x90`#\x19\x01\x12a\x02\xB0W`$\x90V[_\x80\xFD[\x90\x81`@\x91\x03\x12a\x02\xB0W\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x02\xB0WV[5\x90a\x02\xDB\x82a\x02\xC2V[V[\x91\x81`\x1F\x84\x01\x12\x15a\x02\xB0W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xB0W` \x83\x81\x86\x01\x95\x01\x01\x11a\x02\xB0WV[4a\x02\xB0W``6`\x03\x19\x01\x12a\x02\xB0W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x03:\x906\x90`\x04\x01a\x02\xB4V[`$5\x90a\x03G\x82a\x02\xC2V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x03f\x906\x90`\x04\x01a\x02\xDDV[`\xCET\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x04tWa\x04r\x93a\x04\\\x93a\x03\xBAa\x03\xC1\x93a\x03\x9Fa\x03\x97a\x1E\xF1V[\x966\x90a\x1F\x1CV[\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01Rc\xFF\xFF\xFF\xFF\x16``\x86\x01RV[6\x91a\t\xC9V[`@\x82\x01R`@Q` \x81\x01\x90a\x03\xEA\x81a\x03\xDC\x85\x85a\x1F\xA5V[\x03`\x1F\x19\x81\x01\x83R\x82a\x05\xD0V[Q\x90 a\x04\x13a\x03\xFF`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\xF3\xD0)\x86\x07\xCB\xBD\xE3\x8B\xAB\xA3\xC3\x91]'\x7F\x1F\xFD\x80\xA3\x84\tZ\x02\xA4\xA4\xD7\xC0\x82\xAEl\xD9`@Q\x80a\x04Tc\xFF\xFF\xFF\xFF\x86\x16\x94\x82a\x1F\xA5V[\x03\x90\xA2a +V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05|Wa\x04r\x92a\x059\x91_\x91a\x05MW[Pa \x9AV[a\x05H`fT\x82\x81\x16\x14a \xB0V[aGtV[a\x05o\x91P` =` \x11a\x05uW[a\x05g\x81\x83a\x05\xD0V[\x81\x01\x90a zV[_a\x053V[P=a\x05]V[a \x8FV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x05\xB0W`@RV[a\x05\x81V[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x05\xB0W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x05\xB0W`@RV[`@Q\x90a\x02\xDBa\x01\0\x83a\x05\xD0V[`@Q\x90a\x02\xDB`@\x83a\x05\xD0V[`@Q\x90a\x02\xDB``\x83a\x05\xD0V[`@Q\x90a\x02\xDB`\xA0\x83a\x05\xD0V[\x90a\x02\xDB`@Q\x92\x83a\x05\xD0V[`@\x90`\xE3\x19\x01\x12a\x02\xB0W`@Q\x90a\x06U\x82a\x05\x95V[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x02\xB0W`@Qa\x06}\x81a\x05\x95V[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W`@Q\x91a\x06\xA8`@\x84a\x05\xD0V[\x82\x90`@\x81\x01\x92\x83\x11a\x02\xB0W\x90[\x82\x82\x10a\x06\xC4WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x06\xB7V[\x90`\x80`c\x19\x83\x01\x12a\x02\xB0W`@Qa\x06\xED\x81a\x05\x95V[` a\x07\x08\x82\x94a\x06\xFF\x81`da\x06\x8DV[\x84R`\xA4a\x06\x8DV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x02\xB0W` a\x07\x08`@Q\x92a\x07+\x84a\x05\x95V[`@\x84\x96a\x079\x83\x82a\x06\x8DV[\x86R\x01a\x06\x8DV[4a\x02\xB0Wa\x01 6`\x03\x19\x01\x12a\x02\xB0W`\x045`@6`#\x19\x01\x12a\x02\xB0Wa\x07\x99`@\x91\x82Qa\x07s\x81a\x05\x95V[`$5\x81R`D5` \x82\x01Ra\x07\x896a\x06\xD4V[\x90a\x07\x936a\x06<V[\x92a!\x04V[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x02\xB0WV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`\xCDT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0Wc\xFF\xFF\xFF\xFF`\x045a\x08=\x81a\x02\xC2V[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0Wc\xFF\xFF\xFF\xFF`\x045a\x08v\x81a\x02\xC2V[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\xB0WV[`\x01`\x01`@\x1B\x03\x81\x11a\x05\xB0W`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x08\xD2WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x08\xC5V[\x90` a\x08\xF9\x92\x81\x81R\x01\x90a\x08\xB5V[\x90V[4a\x02\xB0W`@6`\x03\x19\x01\x12a\x02\xB0W`\x045a\t\x19\x81a\x08\x8DV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W6`#\x83\x01\x12\x15a\x02\xB0W\x81`\x04\x015\x91a\tE\x83a\x08\x9EV[\x92a\tS`@Q\x94\x85a\x05\xD0V[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x02\xB0W`$\x01\x90[\x82\x82\x10a\t\x94Wa\t\x90a\t\x84\x86\x86a\"nV[`@Q\x91\x82\x91\x82a\x08\xE8V[\x03\x90\xF3[` \x80\x91\x835a\t\xA3\x81a\x08\x8DV[\x81R\x01\x91\x01\x90a\tpV[`\x01`\x01`@\x1B\x03\x81\x11a\x05\xB0W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\t\xD5\x82a\t\xAEV[\x91a\t\xE3`@Q\x93\x84a\x05\xD0V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02\xB0W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\n*WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\njWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\n\x1BV[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\nKV[4a\x02\xB0W``6`\x03\x19\x01\x12a\x02\xB0W`\x045a\n\xC3\x81a\x08\x8DV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W6`#\x82\x01\x12\x15a\x02\xB0Wa\t\x90\x91a\n\xFAa\x0B\x0C\x926\x90`$\x81`\x04\x015\x91\x01a\t\xC9V[`D5\x91a\x0B\x07\x83a\x02\xC2V[a$\xACV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\t\xFFV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x02\xB0WV[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0W`\x045a\x0Bo\x81a\x0BHV[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05|W_\x91a\x0B\xE4W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0B\xD5Wa\x04r\x90aL\x07V[cpp\xF3\xB1`\xE1\x1B_R`\x04_\xFD[a\x0C\x06\x91P` =` \x11a\x0C\x0CW[a\x0B\xFE\x81\x83a\x05\xD0V[\x81\x01\x90a#3V[_a\x0B\xBCV[P=a\x0B\xF4V[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x815a\x0C*\x81a\x08\x9EV[\x92a\x0C8`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xB0W` \x01\x90[\x82\x82\x10a\x0C`WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x0CSV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x0C\x93WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0C\x86V[4a\x02\xB0W`@6`\x03\x19\x01\x12a\x02\xB0W`\x045a\x0C\xCF\x81a\x08\x8DV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x0C\xEE\x906\x90`\x04\x01a\x0C\x13V[a\x0C\xF8\x81Qa\"\x0CV[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\r\x95W\x80` a\r\x1Da\r=\x93\x86a\"KV[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x05|W`\x01\x92a\rq\x91_\x91a\rwW[Pa\rb\x83\x88a\"KV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\r\x04V[a\r\x8F\x91P` =\x81\x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[_a\rWV[`@Q\x80a\t\x90\x86\x82a\x0CpV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\r\xC0WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\r\xB3V[\x90` \x82R``a\x0E*a\x0E\x15a\r\xFF\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\r\xA3V[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\r\xA3V[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\r\xA3V[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0EcWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0E\x81`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\r\xA3V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0ETV[4a\x02\xB0W`\x806`\x03\x19\x01\x12a\x02\xB0W`\x045a\x0E\xAD\x81a\x08\x8DV[`$5\x90a\x0E\xBA\x82a\x02\xC2V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x0E\xD9\x906\x90`\x04\x01a\x02\xDDV[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x02\xB0W6`#\x85\x01\x12\x15a\x02\xB0W\x83`\x04\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x02\xB0W6`$\x85`\x05\x1B\x87\x01\x01\x11a\x02\xB0Wa\t\x90\x95`$a\x0F+\x96\x01\x93a)\xB1V[`@Q\x91\x82\x91\x82a\r\xDCV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05|Wa\x0F\xA2\x91_\x91a\x05MWPa \x9AV[a\x04raG@V[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` `@Qg\x01cEx]\x8A\0\0\x81R\xF3[`\xFF\x81\x16\x03a\x02\xB0WV[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0W` `\x01`\xFF`\x045a\x0F\xFA\x81a\x0F\xCCV[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x02\xB0W``6`\x03\x19\x01\x12a\x02\xB0W`\x045a\x10'\x81a\x08\x8DV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x10F\x906\x90`\x04\x01a\x0C\x13V[`D5\x91a\x10S\x83a\x02\xC2V[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x10{\x86\x88`\x04\x84\x01a.\x19V[\x03\x81\x84Z\xFA\x91\x82\x15a\x05|W_\x92a\x11_W[Pa\x10\x99\x83Qa\"\x0CV[\x93_[\x84Q\x81\x10\x15a\x11QWa\x10\xAF\x81\x86a\"KV[Q\x90` \x83a\x10\xCBa\x10\xC1\x84\x89a\"KV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x05|W`\x01\x92_\x91a\x11#W[P\x82\x80`\xC0\x1B\x03\x16a\x11\x1C\x82\x89a\"KV[R\x01a\x10\x9CV[a\x11D\x91P` =\x81\x11a\x11JW[a\x11<\x81\x83a\x05\xD0V[\x81\x01\x90a)(V[_a\x11\nV[P=a\x112V[`@Q\x80a\t\x90\x88\x82a\x08\xE8V[a\x11|\x91\x92P=\x80_\x83>a\x11t\x81\x83a\x05\xD0V[\x81\x01\x90a(\x06V[\x90_a\x10\x8EV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` `fT`@Q\x90\x81R\xF3[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0Wc\xFF\xFF\xFF\xFF`\x045a\x11\xC2\x81a\x02\xC2V[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x815a\x12\xC1\x81a\x08\x9EV[\x92a\x12\xCF`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xB0W` \x01\x90[\x82\x82\x10a\x12\xF7WPPP\x90V[` \x80\x91\x835a\x13\x06\x81a\x02\xC2V[\x81R\x01\x91\x01\x90a\x12\xEAV[\x81`\x1F\x82\x01\x12\x15a\x02\xB0W\x805a\x13'\x81a\x08\x9EV[\x92a\x135`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x02\xB0W` \x01\x91[\x83\x83\x10a\x13_WPPPP\x90V[` `@\x91a\x13n\x84\x86a\x06eV[\x81R\x01\x92\x01\x91a\x13QV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x815a\x13\x90\x81a\x08\x9EV[\x92a\x13\x9E`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x02\xB0W` \x82\x01\x90[\x83\x82\x10a\x13\xCAWPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W` \x91a\x13\xEC\x87\x84\x80\x94\x88\x01\x01a\x12\xAAV[\x81R\x01\x91\x01\x90a\x13\xBBV[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x02\xB0Wa\x14\ra\x05\xF1V[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x81a\x14*\x91\x84\x01a\x12\xAAV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x81a\x14K\x91\x84\x01a\x13\x11V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x81a\x14o\x91\x84\x01a\x13\x11V[`@\x85\x01Ra\x14\x81\x81``\x84\x01a\x07\rV[``\x85\x01Ra\x14\x93\x81`\xE0\x84\x01a\x06eV[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x81a\x14\xB8\x91\x84\x01a\x12\xAAV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x81a\x14\xDD\x91\x84\x01a\x12\xAAV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x15\0\x92\x01a\x13yV[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x15$WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x15\x17V[4a\x02\xB0W`\x806`\x03\x19\x01\x12a\x02\xB0W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x15v\x906\x90`\x04\x01a\x02\xDDV[\x90\x91`D5a\x15\x84\x81a\x02\xC2V[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x02\xB0Wa\x15\xE6\x94a\x15\xABa\x15\xB1\x956\x90`\x04\x01a\x13\xF7V[\x93a/\xF2V[`@Q\x92\x83\x92`@\x84R` a\x15\xD2\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x15\x07V[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x15\x07V[\x90` \x83\x01R\x03\x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0Wa\x16\x08aM\xE3V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90\x81`\x80\x91\x03\x12a\x02\xB0W\x90V[4a\x02\xB0W`\xC06`\x03\x19\x01\x12a\x02\xB0W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x17h\x906\x90`\x04\x01a\x17*V[a\x17q6a\x02\x9FV[\x90`@6`c\x19\x01\x12a\x02\xB0W`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x04r\x92a\x17\xA3`d\x926\x90`\x04\x01a\x13\x11V[\x92a=\nV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xB0W`\x806`\x03\x19\x01\x12a\x02\xB0W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x17\xFB\x906\x90`\x04\x01a\x17*V[a\x18\x046a\x02\x9FV[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0Wa\x18$\x906\x90`\x04\x01a\x13\xF7V[`\xCDT\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1A\x15Wa\x18G` \x83\x94\x93\x01a8\x8FV[\x91a\x19(a\x18X`@\x86\x01\x86a;\x81V[\x92\x90\x94a\x18\x96a\x18j``\x89\x01a8\x8FV[\x97`@Qa\x18\x80\x81a\x03\xDC` \x82\x01\x94\x85aC\xB9V[Q\x90 a\x18\x8Fa\x03\xFF\x88a8\x8FV[T\x14aDqV[a\x18\xC0a\x18\xB9a\x18\xA5\x87a8\x8FV[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15aD\xE3V[\x83c\xFF\xFF\xFF\xFFC\x16\x96a\x19\na\x19\x02a\x18\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a `V[c\xFF\xFF\xFF\xFF\x16\x90V[\x89\x11\x15aEDV[`@Q` \x81\x01\x90a\x19 \x81a\x03\xDC\x8B\x85aE\xA6V[Q\x90 a/\xF2V[\x91\x90`\xFF_\x96\x16\x95[\x82\x81\x10a\x19\xB3W\x7F4\x9C\x1E\xE6\x0EN\x89r\xEE\x9D\xBAd,\x17tT=\\A6\x87\x9B\x7FL\xAA\xF0K\xF8\x1AHz*\x86\x86\x86a\x19sa\x19ga\x06\x01V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x19\x8F\x81a\x03\xDC\x86\x86\x86aFJV[Q\x90 a\x19\x9Ea\x18\xA5\x83a8\x8FV[Ua\x19\xAE`@Q\x92\x83\x92\x83aFJV[\x03\x90\xA1\0[\x80a\x1A\x0Fa\x19\xEBa\x19\xE6a\x19\xDAa\x19\xCD`\x01\x96\x88Qa\"KV[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[a:\xB7V[a\x1A\x08a\x19\xDA\x8Ba\x1A\x03a\x19\xCD\x87` \x8B\x01Qa\"KV[aE\xB6V[\x11\x15aE\xD9V[\x01a\x191V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\xB0W`\xC06`\x03\x19\x01\x12a\x02\xB0W`\x045a\x1A\x9F\x81a\x08\x8DV[a\x1B\x1F`$5a\x1A\xAE\x81a\x08\x8DV[`D5a\x1A\xBA\x81a\x08\x8DV[`d5a\x1A\xC6\x81a\x08\x8DV[`\x845\x91a\x1A\xD3\x83a\x08\x8DV[`\xA45\x93a\x1A\xE0\x85a\x08\x8DV[_T\x96a\x1B\x05`\xFF`\x08\x8A\x90\x1C\x16\x15\x80\x99\x81\x9Aa\x1B\x9DW[\x81\x15a\x1B}W[PaFtV[\x87a\x1B\x16`\x01`\xFF\x19_T\x16\x17_UV[a\x1BfWaF\xD7V[a\x1B%W\0[a\x1B3a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x19\xAEV[a\x1Bxa\x01\0a\xFF\0\x19_T\x16\x17_UV[aF\xD7V[0;\x15\x91P\x81a\x1B\x8FW[P_a\x1A\xFFV[`\xFF\x16`\x01\x14\x90P_a\x1B\x88V[`\x01`\xFF\x82\x16\x10\x91Pa\x1A\xF8V[`@\x90a\x08\xF9\x93\x92\x81R\x81` \x82\x01R\x01\x90a\t\xFFV[4a\x02\xB0W``6`\x03\x19\x01\x12a\x02\xB0W`\x045a\x1B\xDF\x81a\x08\x8DV[`$5`D5a\x1B\xEE\x81a\x02\xC2V[a\x1C/a\x1B\xF9a!\xEAV[\x92\x80a\x1C\x04\x85a\">V[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a.\x19V[\x03\x81\x87Z\xFA\x93\x84\x15a\x05|W\x83a\x1CYa\x18\xF9a\x10\xC1a\x1C\x8E\x98` \x97_\x91a\x1C\xECW[Pa\">V[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x05|Wa\x1C\xBD\x92_\x91a\x1C\xCDW[P`\x01`\x01`\xC0\x1B\x03\x16\x92a\x1C\xB7\x84aN\x83V[\x90a$\xACV[\x90a\t\x90`@Q\x92\x83\x92\x83a\x1B\xABV[a\x1C\xE6\x91P` =` \x11a\x11JWa\x11<\x81\x83a\x05\xD0V[_a\x1C\xA3V[a\x1D\0\x91P=\x80_\x83>a\x11t\x81\x83a\x05\xD0V[_a\x1CSV[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0W`\x045a\x1Dg\x81a\x08\x8DV[a\x1DoaM\xE3V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1D\x87Wa\x04r\x90aN;V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02\xB0W_6`\x03\x19\x01\x12a\x02\xB0W` `@Q`d\x81R\xF3[4a\x02\xB0W` 6`\x03\x19\x01\x12a\x02\xB0W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05|W_\x91a\x1E\xB9W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1E\xAAWa\x1Ex`fT\x19\x82\x19\x81\x16\x14a \xB0V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a\x1E\xD2\x91P` =` \x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[_a\x1EWV[`@Q\x90a\x1E\xE5\x82a\x05\x95V[``` \x83\x82\x81R\x01RV[`@Q\x90a\x1E\xFE\x82a\x05\xB5V[_``\x83a\x1F\na\x1E\xD8V[\x81R\x82` \x82\x01R\x81`@\x82\x01R\x01RV[\x91\x90`@\x83\x82\x03\x12a\x02\xB0W`@Q\x90a\x1F5\x82a\x05\x95V[\x81\x93\x805`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB0W\x82a\x1FS\x91\x83\x01a\x0C\x13V[\x83R` \x81\x015\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xB0W` \x92a\x07\x08\x92\x01a\x0C\x13V[a\x08\xF9\x906\x90a\x1F\x1CV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[` \x81R`\x80c\xFF\xFF\xFF\xFF``a \ra\x1F\xEC\x86Q\x85` \x88\x01R` a\x1F\xD8\x82Q`@`\xA0\x8B\x01R`\xE0\x8A\x01\x90a\x08\xB5V[\x91\x01Q\x87\x82\x03`\x9F\x19\x01`\xC0\x89\x01Ra\x08\xB5V[\x83` \x88\x01Q\x16`@\x87\x01R`@\x87\x01Q`\x1F\x19\x87\x83\x03\x01\x84\x88\x01Ra\x1F\x81V[\x94\x01Q\x16\x91\x01R\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a CWV[a \x17V[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a CWV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a CWV[\x90\x81` \x91\x03\x12a\x02\xB0WQa\x08\xF9\x81a\x0BHV[`@Q=_\x82>=\x90\xFD[\x15a \xA1WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a \xB7WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a \xEBW`\x05\x1B\x01\x90V[a \xC6V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a!\xE0a!\xBDa!\xE6\x95a!\xB7a!\xB0\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra!\x87\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x05\xD0V[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aG\xEAV[\x90aH0V[\x92a!\xB7a!\xD2a!\xCCaH\x92V[\x94aI\x89V[\x91a!\xDBaJ\xA5V[aG\xEAV[\x91aJ\xD9V[\x90\x91V[`@\x80Q\x90\x91\x90a!\xFB\x83\x82a\x05\xD0V[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a\"\x16\x82a\x08\x9EV[a\"#`@Q\x91\x82a\x05\xD0V[\x82\x81R\x80\x92a\"4`\x1F\x19\x91a\x08\x9EV[\x01\x90` 6\x91\x017V[\x80Q\x15a \xEBW` \x01\x90V[\x80Q\x82\x10\x15a \xEBW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x02\xB0WQ\x90V[\x91\x90\x91a\"{\x83Qa\"\x0CV[\x92_[\x81Q\x81\x10\x15a#.W\x80` a\"\xA7a\"\x9Aa\"\xD0\x94\x86a\"KV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x05|W`\x01\x92_\x91a#\0W[Pa\"\xF9\x82\x88a\"KV[R\x01a\"~V[a#!\x91P` =\x81\x11a#'W[a#\x19\x81\x83a\x05\xD0V[\x81\x01\x90a\"_V[_a\"\xEEV[P=a#\x0FV[PPPV[\x90\x81` \x91\x03\x12a\x02\xB0WQa\x08\xF9\x81a\x08\x8DV[\x90a#R\x82a\x08\x9EV[a#_`@Q\x91\x82a\x05\xD0V[\x82\x81R` \x81\x93a#r`\x1F\x19\x91a\x08\x9EV[\x01\x91\x01_[\x82\x81\x10a#\x83WPPPV[``\x82\x82\x01R` \x01a#wV[\x90\x81Q\x81\x10\x15a \xEBW\x01` \x01\x90V[` \x81\x83\x03\x12a\x02\xB0W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x81Qa#\xD5\x81a\x08\x9EV[\x92a#\xE3`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xB0W` \x01\x90[\x82\x82\x10a$\x0BWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a#\xFEV[\x90a$%\x82a\x08\x9EV[a$2`@Q\x91\x82a\x05\xD0V[\x82\x81R\x80\x92a$C`\x1F\x19\x91a\x08\x9EV[\x01_[\x81\x81\x10a$RWPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x05\xB0W` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a$FV[\x90\x81` \x91\x03\x12a\x02\xB0WQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x02\xB0W\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x05|W_\x95a'\xC1W[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x05|W`\x04\x96_\x93a'\x9FW[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x05|W_\x96a'~W[Pa%:\x85\x93\x92\x95Qa#HV[\x94_\x93[\x80Q\x85\x10\x15a'tWa%ka%ea%W\x87\x84a#\x91V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x05|W_\x94a'PW[Pa%\xBB\x84Qa$\x1BV[a%\xC5\x88\x8Ba\"KV[Ra%\xD0\x87\x8Aa\"KV[P_[\x84Q\x81\x10\x15a'?W\x80` a%\xECa&\x0E\x93\x88a\"KV[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05|W_\x92a'\x1FW[Pa&4\x81\x87a\"KV[Q\x8A` \x8Aa&C\x85\x8Ba\"KV[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x05|Wa&\xD6\x8C\x8Fa&\xD1`\x01\x98a&\xE8\x97\x89\x97_\x92a&\xEFW[Pa&\xBCa&\xADa\x06\x10V[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a\"KV[Q\x90a&\xE2\x83\x83a\"KV[Ra\"KV[P\x01a%\xD3V[a'\x11\x91\x92P` =\x81\x11a'\x18W[a'\t\x81\x83a\x05\xD0V[\x81\x01\x90a$\x8DV[\x90_a&\xA1V[P=a&\xFFV[a'8\x91\x92P` =\x81\x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x90_a&)V[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa%>V[a'm\x91\x94P=\x80_\x83>a'e\x81\x83a\x05\xD0V[\x81\x01\x90a#\xA2V[\x92_a%\xB0V[PPP\x93PPP\x90V[a'\x98\x91\x96P` =` \x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x94_a%,V[` \x91\x93Pa'\xBA\x90\x82=\x84\x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x92\x90a%\x07V[a'\xDB\x91\x95P` =` \x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x93_a$\xDEV[`@Q\x90a'\xEF\x82a\x05\xB5V[``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x02\xB0W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x81Qa(9\x81a\x08\x9EV[\x92a(G`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xB0W` \x01\x90[\x82\x82\x10a(oWPPP\x90V[` \x80\x91\x83Qa(~\x81a\x02\xC2V[\x81R\x01\x91\x01\x90a(bV[\x81\x83R\x90\x91`\x01`\x01`\xFB\x1B\x03\x83\x11a\x02\xB0W` \x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x08\xF9\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a(\x89V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x08\xF9\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a(\xCAV[`\xFF\x16`\xFF\x81\x14a CW`\x01\x01\x90V[\x91\x90\x81\x10\x15a \xEBW`\x05\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x02\xB0WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x02\xB0W\x90V[\x15a)NWV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a \xEBW\x01\x90V[\x90\x81` \x91\x03\x12a\x02\xB0WQa\x08\xF9\x81a\x02\xC2V[_\x19\x81\x14a CW`\x01\x01\x90V[\x91a)\xAA` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a(\xCAV[\x94\x16\x91\x01RV[\x95\x93\x94\x95\x92\x90\x91\x92a)\xC1a'\xE2V[P`@Qch0H5`\xE0\x1B\x81R\x93`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90` \x85`\x04\x81\x86Z\xFA\x94\x85\x15a\x05|W_\x95a-\xF8W[Pa)\xFEa'\xE2V[\x94`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a*\x1E\x8D\x8D\x8B`\x04\x85\x01a(\xADV[\x03\x81\x88Z\xFA\x90\x81\x15a\x05|W_\x91a-\xDEW[P\x86R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x92\x90_\x81\x80a*a\x85\x87\x8B`\x04\x85\x01a(\xEAV[\x03\x81\x87Z\xFA\x90\x81\x15a\x05|W_\x91a-\xC4W[P`@\x87\x01Ra*\x83\x81a#HV[\x98``\x87\x01\x99\x8AR_[`\xFF\x81\x16\x83\x81\x10\x15a-\x0FW\x88_a*\xB6\x83\x8Fa*\xA9\x88a\"\x0CV[\x90Q\x90a&\xE2\x83\x83a\"KV[P_\x8A\x86\x8F[\x81\x84\x10a+9WPPPP\x90P\x8Ca*\xD3\x82a\"\x0CV[\x91_[\x81\x81\x10a+\0WPP\x91a*\xF5\x91a*\xFB\x94\x93Q\x90a&\xE2\x83\x83a\"KV[Pa)\x07V[a*\x8DV[\x80a+3a+\x1Ea\x10\xC1`\x01\x94a+\x18\x8A\x89Qa\"KV[Qa\"KV[a+(\x83\x88a\"KV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x01a*\xD6V[a\x10\xC1\x84a+N\x81` \x96\x95a+V\x95a)\x18V[5\x97Qa\"KV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x87\x01R\x16`D\x85\x01R\x83`d\x81\x8DZ\xFA\x80\x15a\x05|W\x88\x8F\x88\x8A\x91\x8F\x94a+\xFB`\x01a+\xEE\x81\x93\x8D\x80\x9D_\x92a,\xE3W[Pa%ea+\xCAa+\xD8\x92a+\xC3\x87\x80`\xC0\x1B\x03\x86\x16\x15\x15a)GV[\x8B\x8Da)]V[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a,\x17W[PPPPP`\x01\x91\x92P\x01\x90\x8A\x91\x8A\x86\x8Fa*\xBCV[\x85\x97a,9\x93a,2` \x97\x99\x98a%e\x95a+\xCA\x95a)\x18V[5\x95a)]V[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`D\x84\x01R\x82`d\x81\x8CZ\xFA\x90\x81\x15a\x05|W\x8Fa,\x97\x90a,\x9C\x93\x83\x88`\x01\x97_\x93a,\xABW[Pa+\x18\x90a+(\x93\x94Qa\"KV[a)~V[\x90P\x82\x91\x8A\x88\x8F\x88\x8A\x91a,\x01V[a+(\x93P\x90a,\xD4a+\x18\x92` =\x81\x11a,\xDCW[a,\xCC\x81\x83a\x05\xD0V[\x81\x01\x90a)iV[\x93P\x90a,\x87V[P=a,\xC2V[a+\xD8\x91\x92Pa+\xCAa-\x06a%e\x92` =\x81\x11a\x11JWa\x11<\x81\x83a\x05\xD0V[\x93\x92PPa+\xA6V[PPP\x92\x90\x95\x97P`\x04\x94\x96P` \x91P`@Q\x94\x85\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x90\x81\x15a\x05|Wa-e\x94_\x94\x85\x93a-\xA3W[P`@Qc5IR\xA3`\xE2\x1B\x81R\x95\x86\x94\x85\x93\x84\x93`\x04\x85\x01a)\x8CV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05|W_\x91a-\x89W[P` \x82\x01R\x90V[a-\x9D\x91P=\x80_\x83>a\x11t\x81\x83a\x05\xD0V[_a-\x80V[a-\xBD\x91\x93P` =` \x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x91_a-GV[a-\xD8\x91P=\x80_\x83>a\x11t\x81\x83a\x05\xD0V[_a*tV[a-\xF2\x91P=\x80_\x83>a\x11t\x81\x83a\x05\xD0V[_a*1V[a.\x12\x91\x95P` =` \x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x93_a)\xF5V[`@\x90c\xFF\xFF\xFF\xFFa\x08\xF9\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x08\xB5V[\x15a.=WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a.RWV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a.hWV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a.~WV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\xB0WQa\x08\xF9\x81a\x0F\xCCV[_\x19\x81\x01\x91\x90\x82\x11a CWV[\x15a.\xB7WV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90`\x01\x82\x01\x80\x92\x11a CWV[\x90`\x02\x82\x01\x80\x92\x11a CWV[\x90`\x03\x82\x01\x80\x92\x11a CWV[\x90`\x04\x82\x01\x80\x92\x11a CWV[\x90`\x05\x82\x01\x80\x92\x11a CWV[\x91\x90\x82\x01\x80\x92\x11a CWV[\x15a/ WV[c\xAF\xFC^\xDB`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\xB0WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x02\xB0W\x90V[\x15a/WWV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a CWV[\x15a/\x8DWV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a/\xA3WV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a/\xDCWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a/\xCFV[\x94\x93\x92\x90\x91\x93a0\0a\x1E\xD8V[Pa0\x0C\x85\x15\x15a.6V[`@\x84\x01QQ\x85\x14\x80a8\x81W[\x80a8sW[\x80a8eW[a0/\x90a.KV[a0A` \x85\x01QQ\x85QQ\x14a.aV[a0Xc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a.wV[a0`a\x06\x01V[_\x81R_` \x82\x01R\x92a0ra\x1E\xD8V[a0{\x87a\"\x0CV[` \x82\x01Ra0\x89\x87a\"\x0CV[\x81Ra0\x93a\x1E\xD8V[\x92a0\xA2` \x88\x01QQa\"\x0CV[\x84Ra0\xB2` \x88\x01QQa\"\x0CV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05|Wa1\x1B\x91_\x91a86W[Pa1\x166\x8B\x87a\t\xC9V[aLEV[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a2\x8DW` \x88a1\x82a\x10\xC1\x8Ca1z\x8F\x96\x86\x8Ea1_a1L\x86\x80\x95a\"KV[Q\x80Q_R` \x01Q` R`@_ \x90V[a1l\x84\x84\x84\x01Qa\"KV[R\x82a2ZW[\x01Qa\"KV[Q\x95Qa\"KV[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x05|Wa!\xB7\x8Aa2/\x8Fa2(\x8F\x84` \x8F\x92a2\x1F\x93a2\x17\x84`\x01\x9Ea25\x9E_\x91a2=W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\"KV[R\x01Qa\"KV[Q\x93\x8DQa\"KV[Q\x16aLpV[\x90aL\xA1V[\x97\x01\x96a1\x1FV[a2T\x91P\x86=\x81\x11a\x11JWa\x11<\x81\x83a\x05\xD0V[_a2\x08V[a2\x88a2j\x84\x84\x84\x01Qa\"KV[Qa2\x81\x84\x84\x01Qa2{\x87a.\xA2V[\x90a\"KV[Q\x10a.\xB0V[a1sV[P\x90\x95\x97\x94\x96Pa2\xA2\x91\x98\x93\x92\x99PaM^V[\x91a2\xAF`\x97T`\xFF\x16\x90V[\x90\x81\x15a8.W`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05|W_\x91a8\x0FW[P\x91\x90[_\x92[\x81\x84\x10a3`WPPPPP\x92a3Ga3Ba3;a3Z\x95\x85a\x03\xDC\x98`\x80``` \x99\x01Q\x92\x01Q\x92a!\x04V[\x91\x90a/\x86V[a/\x9CV[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a/\xB2V[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da7\tW[a\x10\xC1\x82`\xA0a3\xB5a%ea+\xCA\x84a3\xBD\x97a3\xAFa3\xA1a1L\x8F\x9C`@` \x9F\x9E\x01Qa\"KV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba)]V[\x97\x01Qa\"KV[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x05|Wa4\x81a\x10\xC1\x8F\x95\x8F\x90a4y\x8F\x97\x8F\x96\x84\x8Fa4s`\xC0\x96a4l\x84\x8F` \x9F\x90a1sa+\xCA\x99`@\x93a%e\x9C_\x91a6\xDBW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a/PV[Q\x90aH0V[\x9Ca)]V[\x96\x01Qa\"KV[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x05|Wa5\x0E\x91\x8C\x8F\x92_\x92a6\xB7W[P` a5\0\x92\x93\x01Qa\"KV[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a5.\x8Ca5\0\x8Ca5'a\x19\xCD\x82` \x86\x01Qa\"KV[\x92Qa\"KV[_\x98_[` \x8A\x01QQ\x81\x10\x15a6\x9EW\x8B\x8Da5p\x89a5ca%ea+\xCA\x86\x8F\x89a5[\x91Qa\"KV[Q\x94\x87a)]V[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a5\x7FW[PP`\x01\x01a52V[\x8A\x8Aa6\x01\x85\x9F\x94\x8F\x96\x86a+\x18\x8F\x93`\xE0a5\xB8a\x10\xC1\x95` a5\xB0a%ea+\xCA\x83\x9Fa5\xC1\x9C\x89\x91a)]V[\x9A\x01Qa\"KV[Q\x9B\x01Qa\"KV[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05|W\x8Fa6m\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a6xW[Pa6ga5\0\x92\x93Q\x93a6ba\x19\xCD\x84\x87a\"KV[a/fV[\x92a\"KV[\x01\x9A\x90P\x8B\x8Da5uV[a5\0\x92Pa6\x97a6g\x91` =\x81\x11a'\x18Wa'\t\x81\x83a\x05\xD0V[\x92Pa6JV[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a3\nV[a5\0\x92Pa6\xD4` \x91\x82=\x81\x11a'\x18Wa'\t\x81\x83a\x05\xD0V[\x92Pa4\xF1V[` a6\xFC\x92P=\x81\x11a7\x02W[a6\xF4\x81\x83a\x05\xD0V[\x81\x01\x90a//V[_a4VV[P=a6\xEAV[a7F\x94Pa7#\x92Pa%e\x91a+\xCA\x91` \x95a)]V[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05|W` \x89a3\xBD\x8F\x93\x8F`\xA0\x8F\x97a%ea+\xCA\x8F\x8F\x90a3\xAFa3\xA1a1L\x8F`@\x8B\x96\x91\x8F\x88\x93a\x10\xC1\x9Fa7\xCA\x90a7\xD0\x93a3\xB5\x9F_\x92a7\xE6W[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a/\x0CV[\x11a/\x19V[PPPPPP\x97PPPPPP\x92\x93PPa3uV[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a8\x07\x91=\x81\x11a#'Wa#\x19\x81\x83a\x05\xD0V[\x92\x91Pa7\xB9V[a8(\x91P` =` \x11a,\xDCWa,\xCC\x81\x83a\x05\xD0V[_a3\x03V[_\x91\x90a3\x07V[a8X\x91P` =` \x11a8^W[a8P\x81\x83a\x05\xD0V[\x81\x01\x90a.\x8DV[_a1\nV[P=a8FV[P`\xE0\x84\x01QQ\x85\x14a0&V[P`\xC0\x84\x01QQ\x85\x14a0 V[P`\xA0\x84\x01QQ\x85\x14a0\x1AV[5a\x08\xF9\x81a\x02\xC2V[\x905\x90`>\x19\x816\x03\x01\x82\x12\x15a\x02\xB0W\x01\x90V[\x15a8\xB5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a9\x18\x81a\x02\xC2V[\x16\x84R\x015\x91\x01RV[\x90\x92\x91` ``\x91a98\x84`\x80\x81\x01\x97a9\x04V[c\xFF\xFF\xFF\xFF\x815a9H\x81a\x02\xC2V[\x16`@\x85\x01R\x015\x91\x01RV[\x15a9\\WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a9\xCEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a:LWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a CWV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a CWV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a CWV[\x15a:\xFDWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x02\xB0W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W` \x01\x91\x816\x03\x83\x13a\x02\xB0WV[` \x81\x83\x03\x12a\x02\xB0W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02\xB0W\x81Qa;\xE6\x81a\x08\x9EV[\x92a;\xF4`@Q\x94\x85a\x05\xD0V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\xB0W` \x01\x90[\x82\x82\x10a<\x1CWPPP\x90V[` \x80\x91\x83Qa<+\x81a\x08\x8DV[\x81R\x01\x91\x01\x90a<\x0FV[`@Q\x90a<E`@\x83a\x05\xD0V[`\x12\x82Rq9\xB60\xB9\xB4/\xBA42\xAF\xB7\xB82\xB90\xBA7\xB9`q\x1B` \x83\x01RV[\x91\x90` \x83R`\xC0\x83\x01\x92`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x93`\xA0``\x83\x01R\x84Q\x80\x91R` `\xE0\x83\x01\x95\x01\x90_[\x81\x81\x10a<\xEBWPPP`\x80a<\xD6a\x08\xF9\x94\x95``\x85\x01Q`\x1F\x19\x85\x83\x03\x01\x84\x86\x01Ra\x08\xB5V[\x92\x01Q\x90`\xA0`\x1F\x19\x82\x85\x03\x01\x91\x01Ra\x1F\x81V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x96\x87\x01\x96\x90\x92\x01\x91`\x01\x01a<\xADV[\x91\x93\x92\x90\x93a=\x18\x85a8\x8FV[\x93a=+a=&\x85\x80a8\x99V[a\x1FvV[\x94a=Oa=G\x82c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a8\xAEV[a=\x8Ba=j\x82c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x88`@Qa=\x82\x81a\x03\xDC\x89` \x83\x01\x95\x86a9\"V[Q\x90 \x14a9UV[a=\xB6a=\xB0a=\xA9\x83c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a9\xC7V[a=\xDBa=\xCDa\x18\xF9a=\xC8\x86a8\x8FV[a HV[c\xFF\xFF\xFF\xFFC\x16\x11\x15a:EV[_\x93_\x95` \x88\x01\x95[\x88Q\x80Q\x89\x10\x15a>$W`\x01\x91a>\x16a>\x03\x8Ba>\x1C\x94a\"KV[Qa>\x0F\x8C\x8CQa\"KV[Q\x90a:\xE3V[\x90a/\x0CV[\x97\x01\x96a=\xE5V[P\x92\x97P\x94P\x94P` `\x01\x92\x97\x015\x14\x14aC#Wa>D\x83Qa\"\x0CV[\x93_[\x84Q\x81\x10\x15a>qW\x80a>`a1L`\x01\x93\x88a\"KV[a>j\x82\x89a\"KV[R\x01a>GV[P\x90\x92\x93\x91\x94a>\xAB` \x87\x01\x94` a>\x8A\x87a8\x8FV[`@Qa>\x9F\x81a\x03\xDC\x8A\x86\x83\x01\x95\x86a/\xB2V[Q\x90 \x91\x015\x14a:\xF6V[a>\xB5\x85Qa\"\x0CV[\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94_[\x87Q\x81\x10\x15a?eW\x80` a>\xFCa?\x1C\x93\x89a\"KV[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8BZ\xFA\x91\x82\x15a\x05|W`\x01\x92a?A\x91_\x91a?GW[Pa\rb\x83\x8Da\"KV[\x01a>\xE3V[a?_\x91P` =\x81\x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[_a?6V[P\x92\x96\x95P\x92P\x92a?\xC4a?\x8Da?\x95`@\x86\x01\x94a?\x85\x86\x88a;\x81V[\x93\x90\x91a8\x8FV[\x926\x91a\t\xC9V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a$\xACV[\x94_\x91[\x86Q\x83\x10\x15aB\xC5W_\x97\x96\x97\x95[a?\xE1\x84\x8Aa\"KV[QQ\x87\x10\x15aB\xB5W\x97a@\"\x98` \x80a@\0\x8Aa+\x18\x89\x87a\"KV[Q\x01Q`@Q\x80\x9C\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x99\x8A\x15a\x05|W_\x9AaB\x95W[P`\x01\x98_[\x85Q\x81\x10\x15aB\x87Wa@`a@Ta\"\x9A\x83\x89a\"KV[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x14a@wW`\x01\x01a@;V[P\x99\x90\x97\x91\x98P`\x01_[\x15\x15\x14a@\x97W[P`\x01\x01\x95\x97\x96\x97a?\xD7V[\x98\x96\x97\x86\x98_\x87\x95\x93\x98a@\xFC`\xFFa@\xD4a%ea+\xCA\x8Ca@\xCEaAQ\x9Fa@\xC8`\xD1T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x98a;\x81V[\x90a)]V[a@\xEEa@\xDFa\x06\x01V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[\x16c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`\xD0TaA\x13\x90a@T\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\x10]\xEA\x1F`\xE2\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\x04\x82\x01R` \x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16`$\x83\x01R\x90\x98\x89\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x96\x87\x15a\x05|W_\x97aBcW[PaAn\x87Qa\"\x0CV[\x98_[\x8AQ\x81\x10\x15aA\x97W\x80g\x01cEx]\x8A\0\0aA\x90`\x01\x93\x8Ea\"KV[R\x01aAqV[P\x9A\x92\x94\x96aA\xD5`\xFFaA\xBCa%ea+\xCA\x8F\x9D\x97\x9F\x96\x9E\x96a@\xCE\x8E\x8E\x92a;\x81V[aA\xC7a&\xADa\x06\x1FV[\x16c\xFF\xFF\xFF\xFF\x16` \x86\x01RV[`@\x84\x01R``\x83\x01RaA\xE7a<6V[`\x80\x83\x01R`\xCFTaB\x03\x90a@T\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x02\xB0W`@Qcjf\x9BA`\xE0\x1B\x81R\x92_\x91\x84\x91\x82\x90\x84\x90\x82\x90aB/\x90`\x04\x83\x01a<fV[\x03\x92Z\xF1\x91\x82\x15a\x05|W`\x01\x92aBIW[P\x90a@\x8AV[\x80aBW_aB]\x93a\x05\xD0V[\x80a\x07\xA9V[_aBBV[aB\x80\x91\x97P=\x80_\x83>aBx\x81\x83a\x05\xD0V[\x81\x01\x90a;\xB3V[\x95_aAcV[P\x99\x90\x97\x91\x98`\x01\x90a@\x82V[aB\xAE\x91\x9AP` =\x81\x11a\x0C\x0CWa\x0B\xFE\x81\x83a\x05\xD0V[\x98_a@5V[\x96\x97\x96\x95P`\x01\x90\x92\x01\x91a?\xC8V[PPPPP\x91\x90PaB\xF5aB\xE8\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3V[PPPc\xFF\xFF\xFF\xFF3\x91\x16\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05_\x80\xA3V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x02\xB0W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W\x81`\x05\x1B6\x03\x83\x13a\x02\xB0WV[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x02\xB0W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x02\xB0W\x816\x03\x83\x13a\x02\xB0WV[` \x81R\x815\x90`>\x19\x836\x03\x01\x82\x12\x15a\x02\xB0W`\x80aDf``aD_aD%\x87a\x08\xF9\x97\x01\x85` \x88\x01RaD\x13aD\x08aC\xF7\x83\x80aCTV[`@`\xA0\x8C\x01R`\xE0\x8B\x01\x91a(\x89V[\x91` \x81\x01\x90aCTV[\x88\x83\x03`\x9F\x19\x01`\xC0\x8A\x01R\x90a(\x89V[aDAaD4` \x8A\x01a\x02\xD0V[c\xFF\xFF\xFF\xFF\x16`@\x88\x01RV[aDN`@\x89\x01\x89aC\x88V[\x87\x83\x03`\x1F\x19\x01\x85\x89\x01R\x90a(\xCAV[\x95\x01a\x02\xD0V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15aDxWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15aD\xEAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[\x15aEKWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[`@\x81\x01\x92\x91a\x02\xDB\x91\x90a9\x04V[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a CWV[\x15aE\xE0WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[\x90\x92\x91` ``\x91aF`\x84`\x80\x81\x01\x97a9\x04V[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[\x15aF{WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aF\xE0\x90aN;V[`\xCD\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xCE\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD0\x80T\x83\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\xCF\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aG\xB3\x82a\x05\x95V[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aG\xCF\x81\x84a\x05\xD0V[6\x837V[`@Q\x90aG\xE3` \x83a\x05\xD0V[` 6\x837V[\x91\x90`@\x90``aG\xF9aG\xA6V[\x94\x85\x92` \x85Q\x92aH\x0B\x85\x85a\x05\xD0V[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aH.WV[\xFE[` \x92\x91`\x80`@\x92aHAaG\xA6V[\x95\x86\x93\x81\x86Q\x93aHR\x86\x86a\x05\xD0V[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aH.W\x15aH\x83WV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@QaH\x9E\x81a\x05\x95V[`@\x90\x81QaH\xAD\x83\x82a\x05\xD0V[\x826\x827\x81R` \x82Q\x91aH\xC2\x84\x84a\x05\xD0V[\x836\x847\x01R\x80QaH\xD4\x82\x82a\x05\xD0V[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aI*\x83\x83a\x05\xD0V[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaI\x7F\x83Q\x93\x84a\x05\xD0V[\x82R` \x82\x01R\x90V[_Q` aO\xC6_9_Q\x90_R\x90aI\xA0aG\xA6V[P_\x91\x90\x06` `\xC0\x83[aJ\xA0W_\x93_Q` aO\xC6_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaI\xD6\x85\x82a\x05\xD0V[\x846\x827\x84\x81\x85`@QaI\xEA\x82\x82a\x05\xD0V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aO\xC6_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aH.WaJT\x90aO\xAFV[Q\x91aJ\xA0W_Q` aO\xC6_9_Q\x90_R\x82\x80\t\x14aJ\x8BWP_Q` aO\xC6_9_Q\x90_R`\x01_\x94\x08\x92\x93aI\xABV[\x92\x93PPaJ\x97a\x06\x01V[\x92\x83R\x82\x01R\x90V[a \xF0V[aJ\xADaG\xA6V[P`@QaJ\xBA\x81a\x05\x95V[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x0C\x81\x10\x15a \xEBW`\x05\x1B\x01\x90V[\x93\x92\x90\x91aJ\xE7`@a\x06.V[\x94\x85R` \x85\x01RaJ\xF9`@a\x06.V[\x91\x82R` \x82\x01RaK\taG\xBEV[\x92_[`\x02\x81\x10aK6WPPP` a\x01\x80\x92aK%aG\xD4V[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aKB`\x01\x92a:\xCDV[aKL\x82\x85a \xDAV[QQaKX\x82\x89aJ\xC8V[R` aKe\x83\x86a \xDAV[Q\x01QaKzaKt\x83a.\xC6V[\x89aJ\xC8V[RaK\x85\x82\x86a \xDAV[QQQaK\x94aKt\x83a.\xD4V[RaK\xAAaK\xA2\x83\x87a \xDAV[QQ` \x01\x90V[QaK\xB7aKt\x83a.\xE2V[R` aK\xC4\x83\x87a \xDAV[Q\x01QQaK\xD4aKt\x83a.\xF0V[RaL\0aK\xFAaK\xF3` aK\xEA\x86\x8Aa \xDAV[Q\x01Q` \x01\x90V[Q\x92a.\xFEV[\x88aJ\xC8V[R\x01aK\x0CV[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01aLS`\xFF\x93aO7V[\x92\x83\x92\x16\x1B\x11\x15aLaW\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[aL|WP\x90V[_\x19\x81\x01\x81\x81\x11a CWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a CW`\x01\x01\x90\x80aLtV[\x90aL\xAAaG\xA6V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aMOW`\x01\x82\x14aMJWaL\xCBa\x06\x01V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aL\xF0WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aM*W[`\x01aM aM\x15\x83`\xFF\x94aH0V[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aL\xDCV[\x94`\x01aM aM\x15aM?\x89`\xFF\x95aH0V[\x98\x93PPPPaM\x04V[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[aMfaG\xA6V[P\x80Q\x90\x81\x15\x80aM\xD7W[\x15aM\x93WPP`@QaM\x87`@\x82a\x05\xD0V[_\x81R_` \x82\x01R\x90V[` _Q` aO\xC6_9_Q\x90_R\x91\x01Q\x06_Q` aO\xC6_9_Q\x90_R\x03_Q` aO\xC6_9_Q\x90_R\x81\x11a CW`@Q\x91aI\x7F\x83a\x05\x95V[P` \x81\x01Q\x15aMrV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aM\xF7WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\xFF\xFFaN\x8F\x82aLpV[\x16aN\x99\x81a\t\xAEV[\x90aN\xA7`@Q\x92\x83a\x05\xD0V[\x80\x82RaN\xB6`\x1F\x19\x91a\t\xAEV[\x016` \x83\x017__[\x82Q\x82\x10\x80aO\x16W[\x15aO\x0FW`\x01\x81\x1B\x84\x16aN\xE8W[aN\xE3\x90a)~V[aN\xC0V[\x90`\x01aN\xE3\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaO\x05\x82\x87a#\x91V[S\x01\x91\x90PaN\xDAV[PP\x90P\x90V[Pa\x01\0\x81\x10aN\xCAV[\x15aO(WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aO\xA0W\x81Q\x15aO\x9BW` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aO\x96W`\x01\x90aO\x81aOwa%ea%W\x86\x89a#\x91V[`\xFF`\x01\x91\x16\x1B\x90V[\x90aO\x8D\x81\x83\x11aO!V[\x17\x91\x01\x90aOXV[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15aO\xB6WV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \n\x93\xAF\x99r\x1B\xDD\xAF\xED\x02R\xF39z\x1A\xBB\xA4\x0B\x90sW\xC2\x10\xB7\xC4\xEEe\xB3\xA40\xE3EdsolcC\0\x08\x1B\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BitmapValueTooLarge()` and selector `0xca957333`.
    ```solidity
    error BitmapValueTooLarge();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapValueTooLarge;
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
        impl ::core::convert::From<BitmapValueTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapValueTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapValueTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapValueTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BytesArrayLengthTooLong()` and selector `0xfb4a9c8e`.
    ```solidity
    error BytesArrayLengthTooLong();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayLengthTooLong;
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
        impl ::core::convert::From<BytesArrayLengthTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayLengthTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayLengthTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayLengthTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BytesArrayNotOrdered()` and selector `0x80c88348`.
    ```solidity
    error BytesArrayNotOrdered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayNotOrdered;
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
        impl ::core::convert::From<BytesArrayNotOrdered> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayNotOrdered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayNotOrdered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayNotOrdered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CurrentlyPaused()` and selector `0x840a48d5`.
    ```solidity
    error CurrentlyPaused();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CurrentlyPaused;
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
                Self
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECAddFailed()` and selector `0xd4b68fd7`.
    ```solidity
    error ECAddFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECAddFailed;
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
        impl ::core::convert::From<ECAddFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECAddFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECAddFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECAddFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECMulFailed()` and selector `0x4633be32`.
    ```solidity
    error ECMulFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECMulFailed;
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
        impl ::core::convert::From<ECMulFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECMulFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECMulFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECMulFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ExpModFailed()` and selector `0xd51edae3`.
    ```solidity
    error ExpModFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpModFailed;
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
        impl ::core::convert::From<ExpModFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ExpModFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpModFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpModFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputAddressZero()` and selector `0x73632176`.
    ```solidity
    error InputAddressZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputAddressZero;
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
                Self
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputArrayLengthMismatch()` and selector `0x43714afd`.
    ```solidity
    error InputArrayLengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthMismatch;
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
        impl ::core::convert::From<InputArrayLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputArrayLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputEmptyQuorumNumbers()` and selector `0x1f0405a0`.
    ```solidity
    error InputEmptyQuorumNumbers();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputEmptyQuorumNumbers;
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
        impl ::core::convert::From<InputEmptyQuorumNumbers> for UnderlyingRustTuple<'_> {
            fn from(value: InputEmptyQuorumNumbers) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputEmptyQuorumNumbers {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputEmptyQuorumNumbers {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputNonSignerLengthMismatch()` and selector `0x5f832f41`.
    ```solidity
    error InputNonSignerLengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputNonSignerLengthMismatch;
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
        impl ::core::convert::From<InputNonSignerLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputNonSignerLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputNonSignerLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputNonSignerLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidBLSPairingKey()` and selector `0x67988d33`.
    ```solidity
    error InvalidBLSPairingKey();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBLSPairingKey;
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
        impl ::core::convert::From<InvalidBLSPairingKey> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBLSPairingKey) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBLSPairingKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBLSPairingKey {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidBLSSignature()` and selector `0xab1b236b`.
    ```solidity
    error InvalidBLSSignature();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBLSSignature;
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
        impl ::core::convert::From<InvalidBLSSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBLSSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBLSSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBLSSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidNewPausedStatus()` and selector `0xc61dca5d`.
    ```solidity
    error InvalidNewPausedStatus();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNewPausedStatus;
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
                Self
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidQuorumApkHash()` and selector `0xe1310aed`.
    ```solidity
    error InvalidQuorumApkHash();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidQuorumApkHash;
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
        impl ::core::convert::From<InvalidQuorumApkHash> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorumApkHash) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorumApkHash {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorumApkHash {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidReferenceBlocknumber()` and selector `0x4b874f45`.
    ```solidity
    error InvalidReferenceBlocknumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidReferenceBlocknumber;
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
        impl ::core::convert::From<InvalidReferenceBlocknumber> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReferenceBlocknumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidReferenceBlocknumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReferenceBlocknumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NonSignerPubkeysNotSorted()` and selector `0xff719414`.
    ```solidity
    error NonSignerPubkeysNotSorted();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerPubkeysNotSorted;
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
        impl ::core::convert::From<NonSignerPubkeysNotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NonSignerPubkeysNotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonSignerPubkeysNotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonSignerPubkeysNotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyPauser()` and selector `0x75df51dc`.
    ```solidity
    error OnlyPauser();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyPauser;
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
                Self
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyRegistryCoordinatorOwner()` and selector `0xe0e1e762`.
    ```solidity
    error OnlyRegistryCoordinatorOwner();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRegistryCoordinatorOwner;
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
        impl ::core::convert::From<OnlyRegistryCoordinatorOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRegistryCoordinatorOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyRegistryCoordinatorOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRegistryCoordinatorOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyUnpauser()` and selector `0x794821ff`.
    ```solidity
    error OnlyUnpauser();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyUnpauser;
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
                Self
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
    ```solidity
    error OperatorNotRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered;
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
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ScalarTooLarge()` and selector `0xff89d4fa`.
    ```solidity
    error ScalarTooLarge();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ScalarTooLarge;
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
        impl ::core::convert::From<ScalarTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: ScalarTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ScalarTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ScalarTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `StaleStakesForbidden()` and selector `0xaffc5edb`.
    ```solidity
    error StaleStakesForbidden();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StaleStakesForbidden;
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
        impl ::core::convert::From<StaleStakesForbidden> for UnderlyingRustTuple<'_> {
            fn from(value: StaleStakesForbidden) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StaleStakesForbidden {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StaleStakesForbidden {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    /**Event with signature `NewTaskCreated(uint32,((uint256[],uint256[]),uint32,bytes,uint32))` and selector `0xf3d0298607cbbde38baba3c3915d277f1ffd80a384095a02a4a4d7c082ae6cd9`.
    ```solidity
    event NewTaskCreated(uint32 indexed taskIndex, IIncredibleDotProductTaskManager.Task task);
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
        pub task: <IIncredibleDotProductTaskManager::Task as alloy::sol_types::SolType>::RustType,
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
            type DataTuple<'a> = (IIncredibleDotProductTaskManager::Task,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str =
                "NewTaskCreated(uint32,((uint256[],uint256[]),uint32,bytes,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    243u8, 208u8, 41u8, 134u8, 7u8, 203u8, 189u8, 227u8, 139u8, 171u8, 163u8,
                    195u8, 145u8, 93u8, 39u8, 127u8, 31u8, 253u8, 128u8, 163u8, 132u8, 9u8, 90u8,
                    2u8, 164u8, 164u8, 215u8, 192u8, 130u8, 174u8, 108u8, 217u8,
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
                    <IIncredibleDotProductTaskManager::Task as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StaleStakesForbiddenUpdate(bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    64u8, 228u8, 237u8, 136u8, 10u8, 41u8, 224u8, 246u8, 221u8, 206u8, 48u8, 116u8,
                    87u8, 251u8, 117u8, 205u8, 223u8, 79u8, 238u8, 247u8, 211u8, 236u8, 176u8,
                    48u8, 27u8, 253u8, 244u8, 151u8, 106u8, 14u8, 45u8, 252u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &StaleStakesForbiddenUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TaskChallengedSuccessfully(uint32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    194u8, 13u8, 27u8, 176u8, 241u8, 98u8, 54u8, 128u8, 48u8, 107u8, 131u8, 212u8,
                    255u8, 75u8, 185u8, 154u8, 43u8, 235u8, 157u8, 134u8, 217u8, 120u8, 50u8,
                    243u8, 202u8, 64u8, 253u8, 19u8, 162u8, 157u8, 241u8, 236u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &TaskChallengedSuccessfully) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TaskChallengedUnsuccessfully(uint32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    253u8, 62u8, 38u8, 190u8, 235u8, 89u8, 103u8, 252u8, 90u8, 87u8, 160u8, 68u8,
                    105u8, 20u8, 234u8, 188u8, 69u8, 180u8, 170u8, 71u8, 76u8, 103u8, 165u8, 27u8,
                    75u8, 81u8, 96u8, 202u8, 198u8, 13u8, 219u8, 5u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &TaskChallengedUnsuccessfully) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "TaskCompleted(uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    154u8, 20u8, 79u8, 34u8, 138u8, 147u8, 27u8, 157u8, 13u8, 22u8, 150u8, 251u8,
                    205u8, 175u8, 49u8, 11u8, 36u8, 181u8, 210u8, 210u8, 30u8, 121u8, 157u8, 182u8,
                    35u8, 252u8, 152u8, 106u8, 15u8, 84u8, 116u8, 48u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TaskResponded((uint32,uint256),(uint32,bytes32))` and selector `0x349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a`.
    ```solidity
    event TaskResponded(IIncredibleDotProductTaskManager.TaskResponse taskResponse, IIncredibleDotProductTaskManager.TaskResponseMetadata taskResponseMetadata);
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
        pub taskResponse: <IIncredibleDotProductTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponseMetadata: <IIncredibleDotProductTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
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
                IIncredibleDotProductTaskManager::TaskResponse,
                IIncredibleDotProductTaskManager::TaskResponseMetadata,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TaskResponded((uint32,uint256),(uint32,bytes32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    52u8, 156u8, 30u8, 230u8, 14u8, 78u8, 137u8, 114u8, 238u8, 157u8, 186u8, 100u8,
                    44u8, 23u8, 116u8, 84u8, 61u8, 92u8, 65u8, 54u8, 135u8, 155u8, 127u8, 76u8,
                    170u8, 240u8, 75u8, 248u8, 26u8, 72u8, 122u8, 42u8,
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
                    <IIncredibleDotProductTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IIncredibleDotProductTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._taskResponseWindowBlock,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `TASK_CHALLENGE_WINDOW_BLOCK()` and selector `0xf63c5bab`.
    ```solidity
    function TASK_CHALLENGE_WINDOW_BLOCK() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_CHALLENGE_WINDOW_BLOCKCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TASK_CHALLENGE_WINDOW_BLOCKCall> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_CHALLENGE_WINDOW_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_CHALLENGE_WINDOW_BLOCKCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<TASK_CHALLENGE_WINDOW_BLOCKReturn> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_CHALLENGE_WINDOW_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_CHALLENGE_WINDOW_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TASK_CHALLENGE_WINDOW_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: TASK_CHALLENGE_WINDOW_BLOCKReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: TASK_CHALLENGE_WINDOW_BLOCKReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `TASK_RESPONSE_WINDOW_BLOCK()` and selector `0x1ad43189`.
    ```solidity
    function TASK_RESPONSE_WINDOW_BLOCK() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_RESPONSE_WINDOW_BLOCKCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TASK_RESPONSE_WINDOW_BLOCKCall> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_RESPONSE_WINDOW_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_RESPONSE_WINDOW_BLOCKCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<TASK_RESPONSE_WINDOW_BLOCKReturn> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_RESPONSE_WINDOW_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_RESPONSE_WINDOW_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TASK_RESPONSE_WINDOW_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: TASK_RESPONSE_WINDOW_BLOCKReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: TASK_RESPONSE_WINDOW_BLOCKReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `WADS_TO_SLASH()` and selector `0x5a2d7f02`.
    ```solidity
    function WADS_TO_SLASH() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WADS_TO_SLASHCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: WADS_TO_SLASHReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: WADS_TO_SLASHReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `aggregator()` and selector `0x245a7bfc`.
    ```solidity
    function aggregator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatorCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: aggregatorReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: aggregatorReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `allTaskHashes(uint32)` and selector `0x2d89f6fc`.
    ```solidity
    function allTaskHashes(uint32) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesCall(pub u32);
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: allTaskHashesReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: allTaskHashesReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `allTaskResponses(uint32)` and selector `0x2cb223d5`.
    ```solidity
    function allTaskResponses(uint32) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesCall(pub u32);
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allTaskResponsesCall> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskResponsesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<allTaskResponsesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskResponsesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskResponsesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: allTaskResponsesReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: allTaskResponsesReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
    ```solidity
    function allocationManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<allocationManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: allocationManagerReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: allocationManagerReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `blsApkRegistry()` and selector `0x5df45946`.
    ```solidity
    function blsApkRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::Address;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: blsApkRegistryReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: blsApkRegistryReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](checkSignaturesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesReturn {
        #[allow(missing_docs)]
        pub _0:
            <IBLSSignatureCheckerTypes::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkSignaturesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        impl checkSignaturesReturn {
            fn _tokenize(
                &self,
            ) -> <checkSignaturesCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <IBLSSignatureCheckerTypes::QuorumStakeTotals as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkSignaturesReturn;
            type ReturnTuple<'a> = (
                IBLSSignatureCheckerTypes::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                checkSignaturesReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `createNewTask((uint256[],uint256[]),uint32,bytes)` and selector `0x01863067`.
    ```solidity
    function createNewTask(IIncredibleDotProductTaskManager.DotProductInput memory points, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskCall {
        #[allow(missing_docs)]
        pub points: <IIncredibleDotProductTaskManager::DotProductInput as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub quorumThresholdPercentage: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`createNewTask((uint256[],uint256[]),uint32,bytes)`](createNewTaskCall) function.
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
                IIncredibleDotProductTaskManager::DotProductInput,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIncredibleDotProductTaskManager::DotProductInput as alloy::sol_types::SolType>::RustType,
                u32,
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
            impl ::core::convert::From<createNewTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskCall) -> Self {
                    (
                        value.points,
                        value.quorumThresholdPercentage,
                        value.quorumNumbers,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        points: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl createNewTaskReturn {
            fn _tokenize(
                &self,
            ) -> <createNewTaskCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createNewTaskCall {
            type Parameters<'a> = (
                IIncredibleDotProductTaskManager::DotProductInput,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createNewTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createNewTask((uint256[],uint256[]),uint32,bytes)";
            const SELECTOR: [u8; 4] = [1u8, 134u8, 48u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IIncredibleDotProductTaskManager::DotProductInput as alloy_sol_types::SolType>::tokenize(
                        &self.points,
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                createNewTaskReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegation()` and selector `0xdf5cf723`.
    ```solidity
    function delegation() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: delegationReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: delegationReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `generator()` and selector `0x7afa1eed`.
    ```solidity
    function generator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generatorCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: generatorReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: generatorReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getBatchOperatorFromId(address,bytes32[])`](getBatchOperatorFromIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorFromIdReturn {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getBatchOperatorFromIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorFromIdCall) -> Self {
                    (value.registryCoordinator, value.operatorIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorFromIdCall {
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
            impl ::core::convert::From<getBatchOperatorFromIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorFromIdReturn) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorFromIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchOperatorFromIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<alloy::sol_types::private::Address>;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getBatchOperatorFromIdReturn = r.into();
                        r.operators
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: getBatchOperatorFromIdReturn = r.into();
                    r.operators
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getBatchOperatorId(address,address[])`](getBatchOperatorIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorIdReturn {
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getBatchOperatorIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorIdCall) -> Self {
                    (value.registryCoordinator, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorIdCall {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,);
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
            impl ::core::convert::From<getBatchOperatorIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorIdReturn) -> Self {
                    (value.operatorIds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorIds: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchOperatorIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                > as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getBatchOperatorIdReturn = r.into();
                        r.operatorIds
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: getBatchOperatorIdReturn = r.into();
                    r.operatorIds
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        pub nonSignerOperatorIds:
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCheckSignaturesIndices(address,uint32,bytes,bytes32[])`](getCheckSignaturesIndicesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckSignaturesIndicesReturn {
        #[allow(missing_docs)]
        pub _0:
            <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCheckSignaturesIndicesCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCheckSignaturesIndicesCall {
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
            type UnderlyingSolTuple<'a> = (OperatorStateRetriever::CheckSignaturesIndices,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCheckSignaturesIndicesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCheckSignaturesIndicesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCheckSignaturesIndicesReturn {
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (OperatorStateRetriever::CheckSignaturesIndices,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getCheckSignaturesIndices(address,uint32,bytes,bytes32[])";
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <OperatorStateRetriever::CheckSignaturesIndices as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getCheckSignaturesIndicesReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: getCheckSignaturesIndicesReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorState_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_0Call) -> Self {
                    (
                        value.registryCoordinator,
                        value.quorumNumbers,
                        value.blockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_0Call {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorState_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_0Return {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<
                    <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
                >,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                > as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getOperatorState_0Return = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: getOperatorState_0Return = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorState_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_1Call) -> Self {
                    (
                        value.registryCoordinator,
                        value.operatorId,
                        value.blockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_1Call {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorState_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_1Return) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        impl getOperatorState_1Return {
            fn _tokenize(
                &self,
            ) -> <getOperatorState_1Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorState_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorState_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getOperatorState_1Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)`](getQuorumBitmapsAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapsAtBlockNumberReturn {
        #[allow(missing_docs)]
        pub _0:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getQuorumBitmapsAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapsAtBlockNumberCall) -> Self {
                    (
                        value.registryCoordinator,
                        value.operatorIds,
                        value.blockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapsAtBlockNumberCall {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getQuorumBitmapsAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapsAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapsAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapsAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)";
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getQuorumBitmapsAtBlockNumberReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: getQuorumBitmapsAtBlockNumberReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTaskResponseWindowBlock()` and selector `0xf5c9899d`.
    ```solidity
    function getTaskResponseWindowBlock() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTaskResponseWindowBlockCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTaskResponseWindowBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTaskResponseWindowBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTaskResponseWindowBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<getTaskResponseWindowBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTaskResponseWindowBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTaskResponseWindowBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTaskResponseWindowBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getTaskResponseWindowBlockReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: getTaskResponseWindowBlockReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        impl initializeReturn {
            fn _tokenize(&self) -> <initializeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "initialize(address,address,address,address,address,address)";
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `instantSlasher()` and selector `0x9b290e98`.
    ```solidity
    function instantSlasher() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct instantSlasherCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self
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
            impl ::core::convert::From<instantSlasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: instantSlasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for instantSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for instantSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: instantSlasherReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: instantSlasherReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `latestTaskNum()` and selector `0x8b00ce7c`.
    ```solidity
    function latestTaskNum() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: latestTaskNumReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: latestTaskNumReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
    ```solidity
    function owner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::Address;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: ownerReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: ownerReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        impl pauseReturn {
            fn _tokenize(&self) -> <pauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                pauseReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pauseAll()` and selector `0x595c6a67`.
    ```solidity
    function pauseAll() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllCall;
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
                    Self
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
        impl pauseAllReturn {
            fn _tokenize(&self) -> <pauseAllCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                pauseAllReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = bool;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: paused_0Return = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: paused_0Return = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused()` and selector `0x5c975abb`.
    ```solidity
    function paused() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Call;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: paused_1Return = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: paused_1Return = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pauserRegistry()` and selector `0x886f1195`.
    ```solidity
    function pauserRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::Address;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: pauserRegistryReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: pauserRegistryReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    /**Function with signature `raiseAndResolveChallenge(((uint256[],uint256[]),uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])` and selector `0xb490bb41`.
    ```solidity
    function raiseAndResolveChallenge(IIncredibleDotProductTaskManager.Task memory task, IIncredibleDotProductTaskManager.TaskResponse memory taskResponse, IIncredibleDotProductTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct raiseAndResolveChallengeCall {
        #[allow(missing_docs)]
        pub task: <IIncredibleDotProductTaskManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponse: <IIncredibleDotProductTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponseMetadata: <IIncredibleDotProductTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`raiseAndResolveChallenge(((uint256[],uint256[]),uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])`](raiseAndResolveChallengeCall) function.
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
                IIncredibleDotProductTaskManager::Task,
                IIncredibleDotProductTaskManager::TaskResponse,
                IIncredibleDotProductTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIncredibleDotProductTaskManager::Task as alloy::sol_types::SolType>::RustType,
                <IIncredibleDotProductTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
                <IIncredibleDotProductTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<raiseAndResolveChallengeCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for raiseAndResolveChallengeCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<raiseAndResolveChallengeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: raiseAndResolveChallengeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for raiseAndResolveChallengeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl raiseAndResolveChallengeReturn {
            fn _tokenize(
                &self,
            ) -> <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for raiseAndResolveChallengeCall {
            type Parameters<'a> = (
                IIncredibleDotProductTaskManager::Task,
                IIncredibleDotProductTaskManager::TaskResponse,
                IIncredibleDotProductTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = raiseAndResolveChallengeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "raiseAndResolveChallenge(((uint256[],uint256[]),uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])";
            const SELECTOR: [u8; 4] = [180u8, 144u8, 187u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IIncredibleDotProductTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <IIncredibleDotProductTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IIncredibleDotProductTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                raiseAndResolveChallengeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registryCoordinator()` and selector `0x6d14a987`.
    ```solidity
    function registryCoordinator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::Address;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: registryCoordinatorReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: registryCoordinatorReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
    ```solidity
    function renounceOwnership() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
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
                    Self
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
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    /**Function with signature `respondToTask(((uint256[],uint256[]),uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0xc9b14797`.
    ```solidity
    function respondToTask(IIncredibleDotProductTaskManager.Task memory task, IIncredibleDotProductTaskManager.TaskResponse memory taskResponse, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskCall {
        #[allow(missing_docs)]
        pub task: <IIncredibleDotProductTaskManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponse: <IIncredibleDotProductTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub nonSignerStakesAndSignature: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`respondToTask(((uint256[],uint256[]),uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](respondToTaskCall) function.
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
                IIncredibleDotProductTaskManager::Task,
                IIncredibleDotProductTaskManager::TaskResponse,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIncredibleDotProductTaskManager::Task as alloy::sol_types::SolType>::RustType,
                <IIncredibleDotProductTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
                <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<respondToTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskCall) -> Self {
                    (
                        value.task,
                        value.taskResponse,
                        value.nonSignerStakesAndSignature,
                    )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl respondToTaskReturn {
            fn _tokenize(
                &self,
            ) -> <respondToTaskCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for respondToTaskCall {
            type Parameters<'a> = (
                IIncredibleDotProductTaskManager::Task,
                IIncredibleDotProductTaskManager::TaskResponse,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = respondToTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "respondToTask(((uint256[],uint256[]),uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [201u8, 177u8, 71u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IIncredibleDotProductTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <IIncredibleDotProductTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerStakesAndSignature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                respondToTaskReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `serviceManager()` and selector `0x3998fdd3`.
    ```solidity
    function serviceManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceManagerCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::Address;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: serviceManagerReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: serviceManagerReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setStaleStakesForbiddenCall> for UnderlyingRustTuple<'_> {
                fn from(value: setStaleStakesForbiddenCall) -> Self {
                    (value.value,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStaleStakesForbiddenCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setStaleStakesForbiddenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setStaleStakesForbiddenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStaleStakesForbiddenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setStaleStakesForbiddenReturn {
            fn _tokenize(
                &self,
            ) -> <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setStaleStakesForbiddenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bool,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setStaleStakesForbiddenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setStaleStakesForbiddenReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
    ```solidity
    function stakeRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::Address;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: stakeRegistryReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: stakeRegistryReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `staleStakesForbidden()` and selector `0xb98d0908`.
    ```solidity
    function staleStakesForbidden() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct staleStakesForbiddenCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<staleStakesForbiddenCall> for UnderlyingRustTuple<'_> {
                fn from(value: staleStakesForbiddenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for staleStakesForbiddenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<staleStakesForbiddenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: staleStakesForbiddenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for staleStakesForbiddenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for staleStakesForbiddenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: staleStakesForbiddenReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: staleStakesForbiddenReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `taskNumber()` and selector `0x72d18e8d`.
    ```solidity
    function taskNumber() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskNumberCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: taskNumberReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: taskNumberReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `taskSuccesfullyChallenged(uint32)` and selector `0x5decc3f5`.
    ```solidity
    function taskSuccesfullyChallenged(uint32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskSuccesfullyChallengedCall(pub u32);
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<taskSuccesfullyChallengedCall> for UnderlyingRustTuple<'_> {
                fn from(value: taskSuccesfullyChallengedCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskSuccesfullyChallengedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<taskSuccesfullyChallengedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: taskSuccesfullyChallengedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskSuccesfullyChallengedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for taskSuccesfullyChallengedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: taskSuccesfullyChallengedReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: taskSuccesfullyChallengedReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<trySignatureAndApkVerificationCall> for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationCall) -> Self {
                    (value.msgHash, value.apk, value.apkG2, value.sigma)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trySignatureAndApkVerificationCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<trySignatureAndApkVerificationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationReturn) -> Self {
                    (value.pairingSuccessful, value.siganatureIsValid)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trySignatureAndApkVerificationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pairingSuccessful: tuple.0,
                        siganatureIsValid: tuple.1,
                    }
                }
            }
        }
        impl trySignatureAndApkVerificationReturn {
            fn _tokenize(
                &self,
            ) -> <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.pairingSuccessful,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.siganatureIsValid,
                    ),
                )
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = trySignatureAndApkVerificationReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                trySignatureAndApkVerificationReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
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
        impl unpauseReturn {
            fn _tokenize(&self) -> <unpauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                unpauseReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`IncredibleDotProductTaskManager`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum IncredibleDotProductTaskManagerCalls {
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
    impl IncredibleDotProductTaskManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 134u8, 48u8, 103u8],
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
            [155u8, 41u8, 14u8, 152u8],
            [180u8, 144u8, 187u8, 65u8],
            [185u8, 141u8, 9u8, 8u8],
            [201u8, 177u8, 71u8, 151u8],
            [202u8, 138u8, 167u8, 199u8],
            [204u8, 42u8, 154u8, 91u8],
            [206u8, 253u8, 193u8, 212u8],
            [223u8, 92u8, 247u8, 35u8],
            [242u8, 253u8, 227u8, 139u8],
            [245u8, 201u8, 137u8, 157u8],
            [246u8, 60u8, 91u8, 171u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IncredibleDotProductTaskManagerCalls {
        const NAME: &'static str = "IncredibleDotProductTaskManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 41usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(_) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(_) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::WADS_TO_SLASH(_) => <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::aggregator(_) => <aggregatorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::allTaskHashes(_) => <allTaskHashesCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::createNewTask(_) => <createNewTaskCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegation(_) => <delegationCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::generator(_) => <generatorCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::instantSlasher(_) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestTaskNum(_) => <latestTaskNumCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::respondToTask(_) => <respondToTaskCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::serviceManager(_) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setStaleStakesForbidden(_) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::staleStakesForbidden(_) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::taskNumber(_) => <taskNumberCall as alloy_sol_types::SolCall>::SELECTOR,
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
        fn abi_decode_raw(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<
                IncredibleDotProductTaskManagerCalls,
            >] = &[
                {
                    fn createNewTask(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <createNewTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::createNewTask)
                    }
                    createNewTask
                },
                {
                    fn pause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::trySignatureAndApkVerification,
                            )
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn TASK_RESPONSE_WINDOW_BLOCK(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::TASK_RESPONSE_WINDOW_BLOCK,
                            )
                    }
                    TASK_RESPONSE_WINDOW_BLOCK
                },
                {
                    fn aggregator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <aggregatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::aggregator)
                    }
                    aggregator
                },
                {
                    fn allTaskResponses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::allTaskResponses)
                    }
                    allTaskResponses
                },
                {
                    fn allTaskHashes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <allTaskHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::allTaskHashes)
                    }
                    allTaskHashes
                },
                {
                    fn getBatchOperatorId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::getBatchOperatorId)
                    }
                    getBatchOperatorId
                },
                {
                    fn getOperatorState_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::getOperatorState_0)
                    }
                    getOperatorState_0
                },
                {
                    fn serviceManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn setStaleStakesForbidden(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::setStaleStakesForbidden)
                    }
                    setStaleStakesForbidden
                },
                {
                    fn getBatchOperatorFromId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::getBatchOperatorFromId)
                    }
                    getBatchOperatorFromId
                },
                {
                    fn getCheckSignaturesIndices(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::getCheckSignaturesIndices)
                    }
                    getCheckSignaturesIndices
                },
                {
                    fn pauseAll(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn WADS_TO_SLASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::WADS_TO_SLASH)
                    }
                    WADS_TO_SLASH
                },
                {
                    fn paused_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn getQuorumBitmapsAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::getQuorumBitmapsAtBlockNumber,
                            )
                    }
                    getQuorumBitmapsAtBlockNumber
                },
                {
                    fn paused_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn taskSuccesfullyChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::taskSuccesfullyChallenged)
                    }
                    taskSuccesfullyChallenged
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn taskNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <taskNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::taskNumber)
                    }
                    taskNumber
                },
                {
                    fn generator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <generatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::generator)
                    }
                    generator
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn latestTaskNum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <latestTaskNumCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::latestTaskNum)
                    }
                    latestTaskNum
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn instantSlasher(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <instantSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::instantSlasher)
                    }
                    instantSlasher
                },
                {
                    fn raiseAndResolveChallenge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::raiseAndResolveChallenge)
                    }
                    raiseAndResolveChallenge
                },
                {
                    fn staleStakesForbidden(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::staleStakesForbidden)
                    }
                    staleStakesForbidden
                },
                {
                    fn respondToTask(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <respondToTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::respondToTask)
                    }
                    respondToTask
                },
                {
                    fn allocationManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOperatorState_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::getOperatorState_1)
                    }
                    getOperatorState_1
                },
                {
                    fn delegation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getTaskResponseWindowBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::getTaskResponseWindowBlock,
                            )
                    }
                    getTaskResponseWindowBlock
                },
                {
                    fn TASK_CHALLENGE_WINDOW_BLOCK(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::TASK_CHALLENGE_WINDOW_BLOCK,
                            )
                    }
                    TASK_CHALLENGE_WINDOW_BLOCK
                },
                {
                    fn unpause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerCalls::unpause)
                    }
                    unpause
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<
                IncredibleDotProductTaskManagerCalls,
            >] = &[
                {
                    fn createNewTask(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <createNewTaskCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::createNewTask)
                    }
                    createNewTask
                },
                {
                    fn pause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::trySignatureAndApkVerification,
                            )
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn TASK_RESPONSE_WINDOW_BLOCK(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::TASK_RESPONSE_WINDOW_BLOCK,
                            )
                    }
                    TASK_RESPONSE_WINDOW_BLOCK
                },
                {
                    fn aggregator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <aggregatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::aggregator)
                    }
                    aggregator
                },
                {
                    fn allTaskResponses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::allTaskResponses)
                    }
                    allTaskResponses
                },
                {
                    fn allTaskHashes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <allTaskHashesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::allTaskHashes)
                    }
                    allTaskHashes
                },
                {
                    fn getBatchOperatorId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::getBatchOperatorId,
                            )
                    }
                    getBatchOperatorId
                },
                {
                    fn getOperatorState_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::getOperatorState_0,
                            )
                    }
                    getOperatorState_0
                },
                {
                    fn serviceManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn setStaleStakesForbidden(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::setStaleStakesForbidden,
                            )
                    }
                    setStaleStakesForbidden
                },
                {
                    fn getBatchOperatorFromId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::getBatchOperatorFromId,
                            )
                    }
                    getBatchOperatorFromId
                },
                {
                    fn getCheckSignaturesIndices(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::getCheckSignaturesIndices,
                            )
                    }
                    getCheckSignaturesIndices
                },
                {
                    fn pauseAll(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn WADS_TO_SLASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::WADS_TO_SLASH)
                    }
                    WADS_TO_SLASH
                },
                {
                    fn paused_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn getQuorumBitmapsAtBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::getQuorumBitmapsAtBlockNumber,
                            )
                    }
                    getQuorumBitmapsAtBlockNumber
                },
                {
                    fn paused_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn taskSuccesfullyChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::taskSuccesfullyChallenged,
                            )
                    }
                    taskSuccesfullyChallenged
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::registryCoordinator,
                            )
                    }
                    registryCoordinator
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IncredibleDotProductTaskManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn taskNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <taskNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::taskNumber)
                    }
                    taskNumber
                },
                {
                    fn generator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <generatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::generator)
                    }
                    generator
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn latestTaskNum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <latestTaskNumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::latestTaskNum)
                    }
                    latestTaskNum
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn instantSlasher(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <instantSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::instantSlasher)
                    }
                    instantSlasher
                },
                {
                    fn raiseAndResolveChallenge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::raiseAndResolveChallenge,
                            )
                    }
                    raiseAndResolveChallenge
                },
                {
                    fn staleStakesForbidden(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::staleStakesForbidden,
                            )
                    }
                    staleStakesForbidden
                },
                {
                    fn respondToTask(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <respondToTaskCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerCalls::respondToTask)
                    }
                    respondToTask
                },
                {
                    fn allocationManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IncredibleDotProductTaskManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOperatorState_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::getOperatorState_1,
                            )
                    }
                    getOperatorState_1
                },
                {
                    fn delegation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IncredibleDotProductTaskManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getTaskResponseWindowBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::getTaskResponseWindowBlock,
                            )
                    }
                    getTaskResponseWindowBlock
                },
                {
                    fn TASK_CHALLENGE_WINDOW_BLOCK(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerCalls::TASK_CHALLENGE_WINDOW_BLOCK,
                            )
                    }
                    TASK_CHALLENGE_WINDOW_BLOCK
                },
                {
                    fn unpause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerCalls>
                    {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerCalls::unpause)
                    }
                    unpause
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_VALIDATE_SHIMS[idx](data)
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
                        inner, out,
                    )
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(inner) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::WADS_TO_SLASH(inner) => {
                    <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::aggregator(inner) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::generator(inner) => {
                    <generatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getBatchOperatorFromId(inner) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getBatchOperatorId(inner) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getCheckSignaturesIndices(inner) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getOperatorState_0(inner) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getOperatorState_1(inner) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getQuorumBitmapsAtBlockNumber(inner) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getTaskResponseWindowBlock(inner) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::instantSlasher(inner) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::raiseAndResolveChallenge(inner) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setStaleStakesForbidden(inner) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::staleStakesForbidden(inner) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::taskNumber(inner) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::taskSuccesfullyChallenged(inner) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`IncredibleDotProductTaskManager`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum IncredibleDotProductTaskManagerErrors {
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
    impl IncredibleDotProductTaskManagerErrors {
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
    impl alloy_sol_types::SolInterface for IncredibleDotProductTaskManagerErrors {
        const NAME: &'static str = "IncredibleDotProductTaskManagerErrors";
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
                Self::ECAddFailed(_) => <ECAddFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ECMulFailed(_) => <ECMulFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ExpModFailed(_) => <ExpModFailed as alloy_sol_types::SolError>::SELECTOR,
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
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyRegistryCoordinatorOwner(_) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ScalarTooLarge(_) => <ScalarTooLarge as alloy_sol_types::SolError>::SELECTOR,
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
        fn abi_decode_raw(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<
                IncredibleDotProductTaskManagerErrors,
            >] = &[
                {
                    fn InputEmptyQuorumNumbers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::InputEmptyQuorumNumbers)
                    }
                    InputEmptyQuorumNumbers
                },
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn ECMulFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn InvalidReferenceBlocknumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerErrors::InvalidReferenceBlocknumber)
                    }
                    InvalidReferenceBlocknumber
                },
                {
                    fn InputNonSignerLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerErrors::InputNonSignerLengthMismatch)
                    }
                    InputNonSignerLengthMismatch
                },
                {
                    fn InvalidBLSPairingKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::InvalidBLSPairingKey)
                    }
                    InvalidBLSPairingKey
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::BytesArrayNotOrdered)
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidBLSSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InvalidBLSSignature as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::InvalidBLSSignature)
                    }
                    InvalidBLSSignature
                },
                {
                    fn StaleStakesForbidden(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <StaleStakesForbidden as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::StaleStakesForbidden)
                    }
                    StaleStakesForbidden
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn ECAddFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn OnlyRegistryCoordinatorOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerErrors::OnlyRegistryCoordinatorOwner)
                    }
                    OnlyRegistryCoordinatorOwner
                },
                {
                    fn InvalidQuorumApkHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::InvalidQuorumApkHash)
                    }
                    InvalidQuorumApkHash
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::BytesArrayLengthTooLong)
                    }
                    BytesArrayLengthTooLong
                },
                {
                    fn NonSignerPubkeysNotSorted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerErrors::NonSignerPubkeysNotSorted)
                    }
                    NonSignerPubkeysNotSorted
                },
                {
                    fn ScalarTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <ScalarTooLarge as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IncredibleDotProductTaskManagerErrors::ScalarTooLarge)
                    }
                    ScalarTooLarge
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<
                IncredibleDotProductTaskManagerErrors,
            >] = &[
                {
                    fn InputEmptyQuorumNumbers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::InputEmptyQuorumNumbers,
                            )
                    }
                    InputEmptyQuorumNumbers
                },
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::OperatorNotRegistered,
                            )
                    }
                    OperatorNotRegistered
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::InputArrayLengthMismatch,
                            )
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn ECMulFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn InvalidReferenceBlocknumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::InvalidReferenceBlocknumber,
                            )
                    }
                    InvalidReferenceBlocknumber
                },
                {
                    fn InputNonSignerLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::InputNonSignerLengthMismatch,
                            )
                    }
                    InputNonSignerLengthMismatch
                },
                {
                    fn InvalidBLSPairingKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::InvalidBLSPairingKey,
                            )
                    }
                    InvalidBLSPairingKey
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::BytesArrayNotOrdered,
                            )
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidBLSSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InvalidBLSSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerErrors::InvalidBLSSignature)
                    }
                    InvalidBLSSignature
                },
                {
                    fn StaleStakesForbidden(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <StaleStakesForbidden as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::StaleStakesForbidden,
                            )
                    }
                    StaleStakesForbidden
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::InvalidNewPausedStatus,
                            )
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IncredibleDotProductTaskManagerErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn ECAddFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn OnlyRegistryCoordinatorOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::OnlyRegistryCoordinatorOwner,
                            )
                    }
                    OnlyRegistryCoordinatorOwner
                },
                {
                    fn InvalidQuorumApkHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::InvalidQuorumApkHash,
                            )
                    }
                    InvalidQuorumApkHash
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::BytesArrayLengthTooLong,
                            )
                    }
                    BytesArrayLengthTooLong
                },
                {
                    fn NonSignerPubkeysNotSorted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IncredibleDotProductTaskManagerErrors::NonSignerPubkeysNotSorted,
                            )
                    }
                    NonSignerPubkeysNotSorted
                },
                {
                    fn ScalarTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IncredibleDotProductTaskManagerErrors>
                    {
                        <ScalarTooLarge as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IncredibleDotProductTaskManagerErrors::ScalarTooLarge)
                    }
                    ScalarTooLarge
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputEmptyQuorumNumbers(inner) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputNonSignerLengthMismatch(inner) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBLSPairingKey(inner) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidBLSSignature(inner) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidQuorumApkHash(inner) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::StaleStakesForbidden(inner) => {
                    <StaleStakesForbidden as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InputEmptyQuorumNumbers(inner) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InputNonSignerLengthMismatch(inner) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidBLSPairingKey(inner) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidBLSSignature(inner) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidQuorumApkHash(inner) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidReferenceBlocknumber(inner) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::NonSignerPubkeysNotSorted(inner) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::StaleStakesForbidden(inner) => {
                    <StaleStakesForbidden as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`IncredibleDotProductTaskManager`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum IncredibleDotProductTaskManagerEvents {
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
    impl IncredibleDotProductTaskManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                52u8, 156u8, 30u8, 230u8, 14u8, 78u8, 137u8, 114u8, 238u8, 157u8, 186u8, 100u8,
                44u8, 23u8, 116u8, 84u8, 61u8, 92u8, 65u8, 54u8, 135u8, 155u8, 127u8, 76u8, 170u8,
                240u8, 75u8, 248u8, 26u8, 72u8, 122u8, 42u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                64u8, 228u8, 237u8, 136u8, 10u8, 41u8, 224u8, 246u8, 221u8, 206u8, 48u8, 116u8,
                87u8, 251u8, 117u8, 205u8, 223u8, 79u8, 238u8, 247u8, 211u8, 236u8, 176u8, 48u8,
                27u8, 253u8, 244u8, 151u8, 106u8, 14u8, 45u8, 252u8,
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
                154u8, 20u8, 79u8, 34u8, 138u8, 147u8, 27u8, 157u8, 13u8, 22u8, 150u8, 251u8,
                205u8, 175u8, 49u8, 11u8, 36u8, 181u8, 210u8, 210u8, 30u8, 121u8, 157u8, 182u8,
                35u8, 252u8, 152u8, 106u8, 15u8, 84u8, 116u8, 48u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                194u8, 13u8, 27u8, 176u8, 241u8, 98u8, 54u8, 128u8, 48u8, 107u8, 131u8, 212u8,
                255u8, 75u8, 185u8, 154u8, 43u8, 235u8, 157u8, 134u8, 217u8, 120u8, 50u8, 243u8,
                202u8, 64u8, 253u8, 19u8, 162u8, 157u8, 241u8, 236u8,
            ],
            [
                243u8, 208u8, 41u8, 134u8, 7u8, 203u8, 189u8, 227u8, 139u8, 171u8, 163u8, 195u8,
                145u8, 93u8, 39u8, 127u8, 31u8, 253u8, 128u8, 163u8, 132u8, 9u8, 90u8, 2u8, 164u8,
                164u8, 215u8, 192u8, 130u8, 174u8, 108u8, 217u8,
            ],
            [
                253u8, 62u8, 38u8, 190u8, 235u8, 89u8, 103u8, 252u8, 90u8, 87u8, 160u8, 68u8,
                105u8, 20u8, 234u8, 188u8, 69u8, 180u8, 170u8, 71u8, 76u8, 103u8, 165u8, 27u8,
                75u8, 81u8, 96u8, 202u8, 198u8, 13u8, 219u8, 5u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IncredibleDotProductTaskManagerEvents {
        const NAME: &'static str = "IncredibleDotProductTaskManagerEvents";
        const COUNT: usize = 10usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Initialized)
                }
                Some(<NewTaskCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTaskCreated as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::NewTaskCreated)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Paused)
                }
                Some(<StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::StaleStakesForbiddenUpdate)
                }
                Some(<TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::TaskChallengedSuccessfully)
                }
                Some(
                    <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data,
                )
                .map(Self::TaskChallengedUnsuccessfully),
                Some(<TaskCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskCompleted as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::TaskCompleted)
                }
                Some(<TaskResponded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskResponded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::TaskResponded)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
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
    impl alloy_sol_types::private::IntoLogData for IncredibleDotProductTaskManagerEvents {
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
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
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
    /**Creates a new wrapper around an on-chain [`IncredibleDotProductTaskManager`](self) contract instance.

    See the [wrapper's documentation](`IncredibleDotProductTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IncredibleDotProductTaskManagerInstance<P, N> {
        IncredibleDotProductTaskManagerInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
        provider: P,
        _registryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IncredibleDotProductTaskManagerInstance<P, N>>,
    > {
        IncredibleDotProductTaskManagerInstance::<P, N>::deploy(
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
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _registryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        IncredibleDotProductTaskManagerInstance::<P, N>::deploy_builder(
            provider,
            _registryCoordinator,
            _pauserRegistry,
            _taskResponseWindowBlock,
        )
    }
    /**A [`IncredibleDotProductTaskManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IncredibleDotProductTaskManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IncredibleDotProductTaskManagerInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IncredibleDotProductTaskManagerInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IncredibleDotProductTaskManagerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IncredibleDotProductTaskManagerInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`IncredibleDotProductTaskManager`](self) contract instance.

        See the [wrapper's documentation](`IncredibleDotProductTaskManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
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
        ) -> alloy_contract::Result<IncredibleDotProductTaskManagerInstance<P, N>> {
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
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _registryCoordinator,
                        _pauserRegistry,
                        _taskResponseWindowBlock,
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
    impl<P: ::core::clone::Clone, N> IncredibleDotProductTaskManagerInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IncredibleDotProductTaskManagerInstance<P, N> {
            IncredibleDotProductTaskManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IncredibleDotProductTaskManagerInstance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`TASK_CHALLENGE_WINDOW_BLOCK`] function.
        pub fn TASK_CHALLENGE_WINDOW_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, TASK_CHALLENGE_WINDOW_BLOCKCall, N> {
            self.call_builder(&TASK_CHALLENGE_WINDOW_BLOCKCall)
        }
        ///Creates a new call builder for the [`TASK_RESPONSE_WINDOW_BLOCK`] function.
        pub fn TASK_RESPONSE_WINDOW_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, TASK_RESPONSE_WINDOW_BLOCKCall, N> {
            self.call_builder(&TASK_RESPONSE_WINDOW_BLOCKCall)
        }
        ///Creates a new call builder for the [`WADS_TO_SLASH`] function.
        pub fn WADS_TO_SLASH(&self) -> alloy_contract::SolCallBuilder<&P, WADS_TO_SLASHCall, N> {
            self.call_builder(&WADS_TO_SLASHCall)
        }
        ///Creates a new call builder for the [`aggregator`] function.
        pub fn aggregator(&self) -> alloy_contract::SolCallBuilder<&P, aggregatorCall, N> {
            self.call_builder(&aggregatorCall)
        }
        ///Creates a new call builder for the [`allTaskHashes`] function.
        pub fn allTaskHashes(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<&P, allTaskHashesCall, N> {
            self.call_builder(&allTaskHashesCall(_0))
        }
        ///Creates a new call builder for the [`allTaskResponses`] function.
        pub fn allTaskResponses(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<&P, allTaskResponsesCall, N> {
            self.call_builder(&allTaskResponsesCall(_0))
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall)
        }
        ///Creates a new call builder for the [`blsApkRegistry`] function.
        pub fn blsApkRegistry(&self) -> alloy_contract::SolCallBuilder<&P, blsApkRegistryCall, N> {
            self.call_builder(&blsApkRegistryCall)
        }
        ///Creates a new call builder for the [`checkSignatures`] function.
        pub fn checkSignatures(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
            referenceBlockNumber: u32,
            params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, checkSignaturesCall, N> {
            self.call_builder(&checkSignaturesCall {
                msgHash,
                quorumNumbers,
                referenceBlockNumber,
                params,
            })
        }
        ///Creates a new call builder for the [`createNewTask`] function.
        pub fn createNewTask(
            &self,
            points: <IIncredibleDotProductTaskManager::DotProductInput as alloy::sol_types::SolType>::RustType,
            quorumThresholdPercentage: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, createNewTaskCall, N> {
            self.call_builder(&createNewTaskCall {
                points,
                quorumThresholdPercentage,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<&P, delegationCall, N> {
            self.call_builder(&delegationCall)
        }
        ///Creates a new call builder for the [`generator`] function.
        pub fn generator(&self) -> alloy_contract::SolCallBuilder<&P, generatorCall, N> {
            self.call_builder(&generatorCall)
        }
        ///Creates a new call builder for the [`getBatchOperatorFromId`] function.
        pub fn getBatchOperatorFromId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<&P, getBatchOperatorFromIdCall, N> {
            self.call_builder(&getBatchOperatorFromIdCall {
                registryCoordinator,
                operatorIds,
            })
        }
        ///Creates a new call builder for the [`getBatchOperatorId`] function.
        pub fn getBatchOperatorId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<&P, getBatchOperatorIdCall, N> {
            self.call_builder(&getBatchOperatorIdCall {
                registryCoordinator,
                operators,
            })
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
        ) -> alloy_contract::SolCallBuilder<&P, getCheckSignaturesIndicesCall, N> {
            self.call_builder(&getCheckSignaturesIndicesCall {
                registryCoordinator,
                referenceBlockNumber,
                quorumNumbers,
                nonSignerOperatorIds,
            })
        }
        ///Creates a new call builder for the [`getOperatorState_0`] function.
        pub fn getOperatorState_0(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorState_0Call, N> {
            self.call_builder(&getOperatorState_0Call {
                registryCoordinator,
                quorumNumbers,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getOperatorState_1`] function.
        pub fn getOperatorState_1(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorState_1Call, N> {
            self.call_builder(&getOperatorState_1Call {
                registryCoordinator,
                operatorId,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getQuorumBitmapsAtBlockNumber`] function.
        pub fn getQuorumBitmapsAtBlockNumber(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getQuorumBitmapsAtBlockNumberCall, N> {
            self.call_builder(&getQuorumBitmapsAtBlockNumberCall {
                registryCoordinator,
                operatorIds,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getTaskResponseWindowBlock`] function.
        pub fn getTaskResponseWindowBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getTaskResponseWindowBlockCall, N> {
            self.call_builder(&getTaskResponseWindowBlockCall)
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
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                _aggregator,
                _generator,
                _allocationManager,
                _slasher,
                _serviceManager,
            })
        }
        ///Creates a new call builder for the [`instantSlasher`] function.
        pub fn instantSlasher(&self) -> alloy_contract::SolCallBuilder<&P, instantSlasherCall, N> {
            self.call_builder(&instantSlasherCall)
        }
        ///Creates a new call builder for the [`latestTaskNum`] function.
        pub fn latestTaskNum(&self) -> alloy_contract::SolCallBuilder<&P, latestTaskNumCall, N> {
            self.call_builder(&latestTaskNumCall)
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, pauseCall, N> {
            self.call_builder(&pauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`pauseAll`] function.
        pub fn pauseAll(&self) -> alloy_contract::SolCallBuilder<&P, pauseAllCall, N> {
            self.call_builder(&pauseAllCall)
        }
        ///Creates a new call builder for the [`paused_0`] function.
        pub fn paused_0(&self, index: u8) -> alloy_contract::SolCallBuilder<&P, paused_0Call, N> {
            self.call_builder(&paused_0Call { index })
        }
        ///Creates a new call builder for the [`paused_1`] function.
        pub fn paused_1(&self) -> alloy_contract::SolCallBuilder<&P, paused_1Call, N> {
            self.call_builder(&paused_1Call)
        }
        ///Creates a new call builder for the [`pauserRegistry`] function.
        pub fn pauserRegistry(&self) -> alloy_contract::SolCallBuilder<&P, pauserRegistryCall, N> {
            self.call_builder(&pauserRegistryCall)
        }
        ///Creates a new call builder for the [`raiseAndResolveChallenge`] function.
        pub fn raiseAndResolveChallenge(
            &self,
            task: <IIncredibleDotProductTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IIncredibleDotProductTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            taskResponseMetadata: <IIncredibleDotProductTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
            pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, raiseAndResolveChallengeCall, N> {
            self.call_builder(&raiseAndResolveChallengeCall {
                task,
                taskResponse,
                taskResponseMetadata,
                pubkeysOfNonSigningOperators,
            })
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall)
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`respondToTask`] function.
        pub fn respondToTask(
            &self,
            task: <IIncredibleDotProductTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IIncredibleDotProductTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            nonSignerStakesAndSignature: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, respondToTaskCall, N> {
            self.call_builder(&respondToTaskCall {
                task,
                taskResponse,
                nonSignerStakesAndSignature,
            })
        }
        ///Creates a new call builder for the [`serviceManager`] function.
        pub fn serviceManager(&self) -> alloy_contract::SolCallBuilder<&P, serviceManagerCall, N> {
            self.call_builder(&serviceManagerCall)
        }
        ///Creates a new call builder for the [`setStaleStakesForbidden`] function.
        pub fn setStaleStakesForbidden(
            &self,
            value: bool,
        ) -> alloy_contract::SolCallBuilder<&P, setStaleStakesForbiddenCall, N> {
            self.call_builder(&setStaleStakesForbiddenCall { value })
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<&P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall)
        }
        ///Creates a new call builder for the [`staleStakesForbidden`] function.
        pub fn staleStakesForbidden(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, staleStakesForbiddenCall, N> {
            self.call_builder(&staleStakesForbiddenCall)
        }
        ///Creates a new call builder for the [`taskNumber`] function.
        pub fn taskNumber(&self) -> alloy_contract::SolCallBuilder<&P, taskNumberCall, N> {
            self.call_builder(&taskNumberCall)
        }
        ///Creates a new call builder for the [`taskSuccesfullyChallenged`] function.
        pub fn taskSuccesfullyChallenged(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<&P, taskSuccesfullyChallengedCall, N> {
            self.call_builder(&taskSuccesfullyChallengedCall(_0))
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`trySignatureAndApkVerification`] function.
        pub fn trySignatureAndApkVerification(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, trySignatureAndApkVerificationCall, N> {
            self.call_builder(&trySignatureAndApkVerificationCall {
                msgHash,
                apk,
                apkG2,
                sigma,
            })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IncredibleDotProductTaskManagerInstance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`NewTaskCreated`] event.
        pub fn NewTaskCreated_filter(&self) -> alloy_contract::Event<&P, NewTaskCreated, N> {
            self.event_filter::<NewTaskCreated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<&P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`StaleStakesForbiddenUpdate`] event.
        pub fn StaleStakesForbiddenUpdate_filter(
            &self,
        ) -> alloy_contract::Event<&P, StaleStakesForbiddenUpdate, N> {
            self.event_filter::<StaleStakesForbiddenUpdate>()
        }
        ///Creates a new event filter for the [`TaskChallengedSuccessfully`] event.
        pub fn TaskChallengedSuccessfully_filter(
            &self,
        ) -> alloy_contract::Event<&P, TaskChallengedSuccessfully, N> {
            self.event_filter::<TaskChallengedSuccessfully>()
        }
        ///Creates a new event filter for the [`TaskChallengedUnsuccessfully`] event.
        pub fn TaskChallengedUnsuccessfully_filter(
            &self,
        ) -> alloy_contract::Event<&P, TaskChallengedUnsuccessfully, N> {
            self.event_filter::<TaskChallengedUnsuccessfully>()
        }
        ///Creates a new event filter for the [`TaskCompleted`] event.
        pub fn TaskCompleted_filter(&self) -> alloy_contract::Event<&P, TaskCompleted, N> {
            self.event_filter::<TaskCompleted>()
        }
        ///Creates a new event filter for the [`TaskResponded`] event.
        pub fn TaskResponded_filter(&self) -> alloy_contract::Event<&P, TaskResponded, N> {
            self.event_filter::<TaskResponded>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
