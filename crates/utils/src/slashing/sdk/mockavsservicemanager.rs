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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    "NonSignerStakesAndSignature(uint32[] nonSignerQuorumBitmapIndices,BN254.G1Point[] nonSignerPubkeys,BN254.G1Point[] quorumApks,BN254.G2Point apkG2,BN254.G1Point sigma,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    pub struct IBLSSignatureCheckerTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
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
        > IBLSSignatureCheckerTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

        See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
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
        > IBLSSignatureCheckerTypesInstance<T, P, N>
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
        > IBLSSignatureCheckerTypesInstance<T, P, N>
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
library IRewardsCoordinatorTypes {
    struct OperatorDirectedRewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; OperatorReward[] operatorRewards; uint32 startTimestamp; uint32 duration; string description; }
    struct OperatorReward { address operator; uint256 amount; }
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct OperatorDirectedRewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; OperatorReward[] operatorRewards; uint32 startTimestamp; uint32 duration; string description; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorDirectedRewardsSubmission {
        #[allow(missing_docs)]
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorRewards:
            alloy::sol_types::private::Vec<<OperatorReward as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
        pub startTimestamp: u32,
        #[allow(missing_docs)]
        pub duration: u32,
        #[allow(missing_docs)]
        pub description: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::Array<OperatorReward>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<<OperatorReward as alloy::sol_types::SolType>::RustType>,
            u32,
            u32,
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<OperatorDirectedRewardsSubmission> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorDirectedRewardsSubmission) -> Self {
                (
                    value.strategiesAndMultipliers,
                    value.token,
                    value.operatorRewards,
                    value.startTimestamp,
                    value.duration,
                    value.description,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorDirectedRewardsSubmission {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategiesAndMultipliers: tuple.0,
                    token: tuple.1,
                    operatorRewards: tuple.2,
                    startTimestamp: tuple.3,
                    duration: tuple.4,
                    description: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorDirectedRewardsSubmission {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorDirectedRewardsSubmission {
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
                    <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorRewards),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
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
        impl alloy_sol_types::SolType for OperatorDirectedRewardsSubmission {
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
        impl alloy_sol_types::SolStruct for OperatorDirectedRewardsSubmission {
            const NAME: &'static str = "OperatorDirectedRewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorDirectedRewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,OperatorReward[] operatorRewards,uint32 startTimestamp,uint32 duration,string description)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components.push(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                );
                components.extend(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                );
                components.push(<OperatorReward as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<OperatorReward as alloy_sol_types::SolStruct>::eip712_components());
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
                    <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorRewards,
                        )
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.description,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorDirectedRewardsSubmission {
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
                    + <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorRewards,
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
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.description,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
                <alloy::sol_types::sol_data::Array<
                    OperatorReward,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorRewards,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.description,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct OperatorReward { address operator; uint256 amount; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorReward {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<OperatorReward> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorReward) -> Self {
                (value.operator, value.amount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorReward {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    amount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorReward {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorReward {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
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
        impl alloy_sol_types::SolType for OperatorReward {
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
        impl alloy_sol_types::SolStruct for OperatorReward {
            const NAME: &'static str = "OperatorReward";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorReward(address operator,uint256 amount)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorReward {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for RewardsSubmission {
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
        impl alloy_sol_types::SolStruct for RewardsSubmission {
            const NAME: &'static str = "RewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,uint256 amount,uint32 startTimestamp,uint32 duration)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                );
                components.extend(
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.multiplier,
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
        impl alloy_sol_types::SolType for StrategyAndMultiplier {
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
        impl alloy_sol_types::SolStruct for StrategyAndMultiplier {
            const NAME: &'static str = "StrategyAndMultiplier";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyAndMultiplier(address strategy,uint96 multiplier)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
    pub struct IRewardsCoordinatorTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

        See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
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
library ISignatureUtilsMixinTypes {
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
pub mod ISignatureUtilsMixinTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::private::SolTypeValue<Self> for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISignatureUtilsMixinTypes`](self) contract instance.

    See the [wrapper's documentation](`ISignatureUtilsMixinTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsMixinTypesInstance<T, P, N> {
        ISignatureUtilsMixinTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtilsMixinTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISignatureUtilsMixinTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsMixinTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsMixinTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsMixinTypesInstance")
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
        > ISignatureUtilsMixinTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISignatureUtilsMixinTypes`](self) contract instance.

        See the [wrapper's documentation](`ISignatureUtilsMixinTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsMixinTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsMixinTypesInstance<T, P, N> {
            ISignatureUtilsMixinTypesInstance {
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
        > ISignatureUtilsMixinTypesInstance<T, P, N>
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
        > ISignatureUtilsMixinTypesInstance<T, P, N>
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

library IRewardsCoordinatorTypes {
    struct OperatorDirectedRewardsSubmission {
        StrategyAndMultiplier[] strategiesAndMultipliers;
        address token;
        OperatorReward[] operatorRewards;
        uint32 startTimestamp;
        uint32 duration;
        string description;
    }
    struct OperatorReward {
        address operator;
        uint256 amount;
    }
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

library ISignatureUtilsMixinTypes {
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
    error OnlyStakeRegistry();
    error ScalarTooLarge();

    event Initialized(uint8 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);

    constructor(address _avsDirectory, address _slashingRegCoordinator, address _stakeRegistry, address rewards_coordinator, address _permissionController, address _allocationManager);

    function addPendingAdmin(address admin) external;
    function avsDirectory() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureCheckerTypes.QuorumStakeTotals memory, bytes32);
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function createOperatorDirectedAVSRewardsSubmission(IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
    function delegation() external view returns (address);
    function deregisterOperatorFromAVS(address operator) external;
    function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
    function getOperatorRestakedStrategies(address operator) external view returns (address[] memory);
    function getRestakeableStrategies() external view returns (address[] memory);
    function initialize(address _initialOwner) external;
    function owner() external view returns (address);
    function registerOperatorToAVS(address operator, ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function registryCoordinator() external view returns (address);
    function removeAdmin(address admin) external;
    function removeAppointee(address appointee, address target, bytes4 selector) external;
    function removePendingAdmin(address pendingAdmin) external;
    function renounceOwnership() external;
    function rewardsInitiator() external view returns (address);
    function setAppointee(address appointee, address target, bytes4 selector) external;
    function setClaimerFor(address claimer) external;
    function setRewardsInitiator(address newRewardsInitiator) external;
    function stakeRegistry() external view returns (address);
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
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      },
      {
        "name": "_slashingRegCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      },
      {
        "name": "rewards_coordinator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_permissionController",
        "type": "address",
        "internalType": "contract IPermissionController"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "createOperatorDirectedAVSRewardsSubmission",
    "inputs": [
      {
        "name": "operatorDirectedRewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[]",
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
            "name": "operatorRewards",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinatorTypes.OperatorReward[]",
            "components": [
              {
                "name": "operator",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
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
          },
          {
            "name": "description",
            "type": "string",
            "internalType": "string"
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
        "internalType": "struct ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry",
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
    "name": "setClaimerFor",
    "inputs": [
      {
        "name": "claimer",
        "type": "address",
        "internalType": "address"
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
    "name": "OnlyStakeRegistry",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ScalarTooLarge",
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
    ///0x6101c080604052346102945760c081613bde803803809161002082856103bd565b8339810103126102945780516001600160a01b03811681036102945760208201516001600160a01b0381169290919083830361029457610062604082016103f4565b60608201516001600160a01b03811690819003610294576080830151926001600160a01b03841684036102945760a00151936001600160a01b03851685036102945760805260c0528360e052610100526101205260a0525f5460ff8160081c166103685760ff8082160361032e575b5061014052604051636830483560e01b8152602081600481855afa9081156102a0575f916102ed575b5061016052604051632efa2ca360e11b815290602090829060049082905afa9081156102a0575f916102ab575b50610180526101605160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa9081156102a0575f9161025a575b506101a0526040516137d59081610409823960805181818161074a01528181610fd1015281816110860152612450015260a05181611200015260c051818181610d8201528181610e560152611477015260e051818181610fa1015281816111a8015281816117fe0152818161242001526128c801526101005181818161189a0152612914015261012051818181610412015281816104ea0152818161059a01528181610cf801526110f201526101405181818161078e01528181611dd00152611ec30152610160518181816107060152818161216d01526122bc0152610180518181816106c201526120ab01526101a0518161135b0152f35b90506020813d602011610298575b81610275602093836103bd565b8101031261029457516001600160a01b0381168103610294575f610161565b5f80fd5b3d9150610268565b6040513d5f823e3d90fd5b90506020813d6020116102e5575b816102c6602093836103bd565b8101031261029457516001600160a01b0381168103610294575f610127565b3d91506102b9565b90506020813d602011610326575b81610308602093836103bd565b810103126102945760049161031e6020926103f4565b9150916100fa565b3d91506102fb565b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100d1565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b601f909101601f19168101906001600160401b038211908210176103e057604052565b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036102945756fe60806040526004361015610011575f80fd5b5f3560e01c8063171f1d5b146101c45780631785f53c146101bf5780631fdb0cfd146101ba578063279432eb146101b557806333cfb7b7146101b05780633bc28c8c146101ab5780635df45946146101a657806368304835146101a15780636b3aa72e1461019c5780636d14a987146101975780636efb463614610192578063715018a61461018d5780638da5cb5b146101885780639926ee7d146101835780639da16d8e1461017e578063a0169ddd14610179578063a20b99bf14610174578063a364f4da1461016f578063a98fb3551461016a578063ba55088014610165578063c1a8e2c514610160578063c4d66de81461015b578063df5cf72314610156578063e481af9d14610151578063f2fde38b1461014c578063fc299dee146101475763fce36c7d14610142575f80fd5b61145e565b611436565b6113a5565b61138a565b611346565b611251565b611166565b6110da565b61103a565b610f7d565b610e3d565b610d5a565b610cd0565b610c34565b610bbb565b610b60565b610acb565b610779565b610735565b6106f1565b6106ad565b610676565b61063e565b610572565b6104d2565b6103ea565b610371565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176101f857604052565b6101c9565b606081019081106001600160401b038211176101f857604052565b90601f801991011681019081106001600160401b038211176101f857604052565b6040519061024961010083610218565b565b60405190610249604083610218565b906102496040519283610218565b60409060e31901126102915760405190610281826101dd565b60e4358252610104356020830152565b5f80fd5b9190826040910312610291576040516102ad816101dd565b6020808294803584520135910152565b9080601f8301121561029157604051916102d8604084610218565b82906040810192831161029157905b8282106102f45750505090565b81358152602091820191016102e7565b9060806063198301126102915760405161031d816101dd565b6020610338829461032f8160646102bd565b845260a46102bd565b910152565b91906080838203126102915760206103386040519261035b846101dd565b6040849661036983826102bd565b8652016102bd565b3461029157610120366003190112610291576004356040366023190112610291576103c960409182516103a3816101dd565b602435815260443560208201526103b936610304565b906103c336610268565b92611570565b8251911515825215156020820152f35b6001600160a01b0381160361029157565b34610291575f602036600319011261029157600435610408816103d9565b610410613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102915760405163268959e560e01b81523060048201526001600160a01b0390921660248301525f908290818381604481015b03925af1801561049057610482575080f35b61048e91505f90610218565b005b611656565b6060906003190112610291576004356104ad816103d9565b906024356104ba816103d9565b906044356001600160e01b0319811681036102915790565b34610291576104e036610495565b6104e8613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692833b1561029157604051634a86c03760e11b81523060048201526001600160a01b039182166024820152921660448301526001600160e01b03191660648201525f81608481015b93818381819703925af1801561049057610482575080f35b34610291575f602036600319011261029157600435610590816103d9565b610598613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102915760405163eb5a4e8760e01b81523060048201526001600160a01b0390921660248301525f90829081838160448101610470565b60206040818301928281528451809452019201905f5b81811061061f5750505090565b82516001600160a01b0316845260209384019390920191600101610612565b3461029157602036600319011261029157610672610666600435610661816103d9565b6117df565b604051918291826105fc565b0390f35b346102915760203660031901126102915761048e600435610696816103d9565b61069e613054565b61314a565b5f91031261029157565b34610291575f366003190112610291576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610291575f366003190112610291576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610291575f366003190112610291576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610291575f366003190112610291576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b6044359063ffffffff8216820361029157565b359063ffffffff8216820361029157565b6001600160401b0381116101f85760051b60200190565b9080601f8301121561029157813561080f816107e1565b9261081d6040519485610218565b81845260208085019260051b82010192831161029157602001905b8282106108455750505090565b60208091610852846107d0565b815201910190610838565b81601f82011215610291578035610873816107e1565b926108816040519485610218565b81845260208085019260061b8401019281841161029157602001915b8383106108ab575050505090565b60206040916108ba8486610295565b81520192019161089d565b9080601f830112156102915781356108dc816107e1565b926108ea6040519485610218565b81845260208085019260051b820101918383116102915760208201905b83821061091657505050505090565b81356001600160401b03811161029157602091610938878480948801016107f8565b815201910190610907565b9190916101808184031261029157610959610239565b9281356001600160401b03811161029157816109769184016107f8565b845260208201356001600160401b038111610291578161099791840161085d565b602085015260408201356001600160401b03811161029157816109bb91840161085d565b60408501526109cd816060840161033d565b60608501526109df8160e08401610295565b60808501526101208201356001600160401b0381116102915781610a049184016107f8565b60a08501526101408201356001600160401b0381116102915781610a299184016107f8565b60c08501526101608201356001600160401b03811161029157610a4c92016108c5565b60e0830152565b90602080835192838152019201905f5b818110610a705750505090565b82516001600160601b0316845260209384019390920191600101610a63565b929190610ac66020916040865282610ab282516040808a01526080890190610a53565b910151868203603f19016060880152610a53565b930152565b34610291576080366003190112610291576004356024356001600160401b03811161029157366023820112156102915780600401356001600160401b03811161029157366024828401011161029157610b226107bd565b90606435936001600160401b038511610291576024610b48610b50963690600401610943565b940190611cf3565b9061067260405192839283610a8f565b34610291575f36600319011261029157610b78613054565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610291575f366003190112610291576033546040516001600160a01b039091168152602090f35b6001600160401b0381116101f857601f01601f191660200190565b929192610c0a82610be3565b91610c186040519384610218565b829481845281830111610291578281602093845f960137010152565b3461029157604036600319011261029157600435610c51816103d9565b602435906001600160401b03821161029157606060031983360301126102915760405190610c7e826101fd565b82600401356001600160401b038111610291578301366023820112156102915761048e93610cb86044923690602460048201359101610bfe565b8452602481013560208501520135604083015261241a565b34610291575f602036600319011261029157600435610cee816103d9565b610cf6613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561029157604051634f906cf960e01b81523060048201526001600160a01b0390921660248301525f90829081838160448101610470565b34610291575f602036600319011261029157600435610d78816103d9565b610d80613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156102915760405163a0169ddd60e01b81526001600160a01b039091166004820152905f908290602490829084905af1801561049057610482575080f35b906020600319830112610291576004356001600160401b0381116102915760040182601f82011215610291578035926001600160401b038411610291576020808301928560051b010111610291579190565b3461029157610e4b36610deb565b90610e5461338e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b818110610ed85750823b1561029157610eb4925f9283604051809681958294634e5cd2fd60e11b8452306004850161266b565b03925af1801561049057610ec457005b80610ed25f61048e93610218565b806106a3565b915f93915f915b610ef7610eed868484612518565b604081019061253a565b9050831015610f33576001610f2981976020610f2187610f1b610eed8c8a8a612518565b9061256f565b013590611765565b9301929550610edf565b9390929460019250610f7790610f61813088610f5c6020610f56898c3395612518565b0161257f565b6133b1565b86610f726020610f5686898b612518565b6133fa565b01610e81565b34610291575f602036600319011261029157600435610f9b816103d9565b610fcf337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612404565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610291576040516351b27a6d60e11b81526001600160a01b039091166004820152905f908290602490829084905af1801561049057610482575080f35b34610291575f6020366003190112610291576004356001600160401b03811161029157366023820112156102915761107c903690602481600401359101610bfe565b611084613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102915760405163a98fb35560e01b8152915f91839182908490829061047090600483016127db565b34610291576110e836610495565b6110f0613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692833b1561029157604051630664120160e01b81523060048201526001600160a01b039182166024820152921660448301526001600160e01b03191660648201525f816084810161055a565b3461029157604036600319011261029157600435611183816103d9565b6024356001600160401b038111610291576111a29036906004016107f8565b6111d6337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612404565b6111f3604051926111e6846101fd565b6001600160a01b03168352565b30602083015260408201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561029157604051636e3492b560e01b8152905f908290818381610eb488600483016127ec565b346102915760203660031901126102915760043561126e816103d9565b6112be5f54916112a261128c6112888560ff9060081c1690565b1590565b80948195611338575b8115611318575b50612856565b826112b3600160ff195f5416175f55565b611301575b806134a4565b6112c457005b6112d261ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b61131361010061ff00195f5416175f55565b6112b8565b303b1591508161132a575b505f61129c565b60ff1660011490505f611323565b600160ff8216109150611295565b34610291575f366003190112610291576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610291575f366003190112610291576106726106666128b9565b34610291576020366003190112610291576004356113c2816103d9565b6113ca613054565b6001600160a01b038116156113e25761048e90613346565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b34610291575f366003190112610291576065546040516001600160a01b039091168152602090f35b346102915761146c36610deb565b9061147561338e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b8181106114d45750823b1561029157610eb4925f928360405180968195829463fce36c7d60e01b845260048401612b08565b806115036114ea6020610f566001958789612ae6565b60406114f7848789612ae6565b013590309033906133b1565b61152c6115166020610f56848789612ae6565b86604061152485888a612ae6565b0135916133fa565b016114a2565b634e487b7160e01b5f52603260045260245ffd5b9060028110156115575760051b0190565b611532565b634e487b7160e01b5f52601260045260245ffd5b61164c6116296116529561162361161c85875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e08401526101008301526115f381610120840103601f198101835282610218565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b8096612c21565b90612c67565b9261162361163e611638612cc9565b94612dc0565b91611647612edc565b612c21565b91612f26565b9091565b6040513d5f823e3d90fd5b90816020910312610291575190565b9081602091031261029157516001600160c01b03811681036102915790565b90816020910312610291575160ff811681036102915790565b604051906116b7602083610218565b5f808352366020840137565b906116cd826107e1565b6116da6040519182610218565b82815280926116eb601f19916107e1565b0190602036910137565b908151811015611557570160200190565b634e487b7160e01b5f52601160045260245ffd5b906001820180921161172857565b611706565b906002820180921161172857565b906003820180921161172857565b906004820180921161172857565b906005820180921161172857565b9190820180921161172857565b6001600160601b0381160361029157565b908160409103126102915760206040519161179d836101dd565b80516117a8816103d9565b835201516117b581611772565b602082015290565b80518210156115575760209160051b010190565b5f1981146117285760010190565b6040516309aa152760e11b81526001600160a01b0391821660048201527f000000000000000000000000000000000000000000000000000000000000000090911690602081602481855afa9081156104905761185f916020915f91611b5e575b506040518093819263871ef04960e01b8352600483019190602083019252565b0381855afa908115610490575f91611b2f575b506001600160c01b0316908115908115611acc575b50611ac057611895906130ac565b5f91907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b81518510156119715761191a60206118f76118f16118e389876116f5565b516001600160f81b03191690565b60f81c90565b604051633ca5a5f560e01b815260ff909116600482015291829081906024820190565b0381875afa801561049057600192611939925f92611941575b50611765565b9401936118c5565b61196391925060203d811161196a575b61195b8183610218565b810190611661565b905f611933565b503d611951565b61197c9194506116c3565b925f905f5b8151811015611aba5761199a6118f16118e383856116f5565b604051633ca5a5f560e01b815260ff8216600482015290602082602481895afa918215610490575f92611a9a575b50905f915b8183106119df57505050600101611981565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa91821561049057611a5e8b611a4f83611a49611a3d600198611a63985f91611a6c575b50516001600160a01b031690565b6001600160a01b031690565b926117bd565b6001600160a01b039091169052565b6117d1565b950191906119cd565b611a8d915060403d8111611a93575b611a858183610218565b810190611783565b5f611a2f565b503d611a7b565b611ab391925060203d811161196a5761195b8183610218565b905f6119c8565b50505050565b50611ac96116a8565b90565b604051639aa1653d60e01b81529150602090829060049082905afa80156104905760ff915f91611b00575b5016155f611887565b611b22915060203d602011611b28575b611b1a8183610218565b81019061168f565b5f611af7565b503d611b10565b611b51915060203d602011611b57575b611b498183610218565b810190611670565b5f611872565b503d611b3f565b611b759150823d841161196a5761195b8183610218565b5f61183f565b60405190611b88826101dd565b60606020838281520152565b15611b9b57565b62f8202d60e51b5f5260045ffd5b15611bb057565b6343714afd60e01b5f5260045ffd5b15611bc657565b635f832f4160e01b5f5260045ffd5b15611bdc57565b634b874f4560e01b5f5260045ffd5b5f1981019190821161172857565b15611c0057565b633fdc650560e21b5f5260045ffd5b90821015611557570190565b90816020910312610291575167ffffffffffffffff19811681036102915790565b15611c4357565b63e1310aed60e01b5f5260045ffd5b908160209103126102915751611ac981611772565b906001600160601b03809116911603906001600160601b03821161172857565b15611c8e57565b6367988d3360e01b5f5260045ffd5b15611ca457565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b818110611cdd5750505090565b8251845260209384019390920191600101611cd0565b949392909193611d01611b7b565b50611d0d851515611b94565b6040840151518514806123f6575b806123e8575b806123da575b611d3090611ba9565b611d4260208501515185515114611bbf565b611d5963ffffffff431663ffffffff841610611bd5565b611d6161024b565b5f81525f602082015292611d73611b7b565b611d7c876116c3565b6020820152611d8a876116c3565b8152611d94611b7b565b92611da36020880151516116c3565b8452611db36020880151516116c3565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561049057611e1c915f916123bb575b50611e17368b87610bfe565b6131a8565b985f965b60208901518051891015611f9857602088611e8d611e838c611e7b8f96868e611e60611e4d8680956117bd565b5180515f526020015160205260405f2090565b611e6d84848401516117bd565b5282611f65575b01516117bd565b5195516117bd565b5163ffffffff1690565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa918215610490576116238a611f3a8f611f338f8460208f92611f2a93611f228460019e611f409e5f91611f48575b508f8060c01b031692516117bd565b5201516117bd565b51938d516117bd565b51166131d3565b90613204565b970196611e20565b611f5f9150863d8111611b5757611b498183610218565b5f611f13565b611f93611f7584848401516117bd565b51611f8c84840151611f8687611beb565b906117bd565b5110611bf9565b611e74565b50909597949650611fad9198939299506132c1565b915f905b8082106120135750505092611fec611fe7611fe061200d9585611fff9860806060602099015192015192611570565b9190611c87565b611c9d565b0151604051928391602083019586611cb3565b03601f198101835282610218565b51902090565b918495969361205e6118f1612050858761204a879f989e9b611e4d84604061203c9301516117bd565b67ffffffffffffffff191690565b96611c0f565b356001600160f81b03191690565b602087612072611e838d60a08d01516117bd565b604051631a2f32ab60e21b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9283156104905761210e936120f9925f9161238d575b5067ffffffffffffffff19918216911614611c3c565b6121078960408901516117bd565b5190612c67565b906121206118f16120508a868d611c0f565b602086612134611e838c60c08c01516117bd565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa8015610490576121ca8a60208e611e746121d7956121e2975f91612370575b506121bc85858501516117bd565b906001600160601b03169052565b516001600160601b031690565b6121bc8a8d516117bd565b5f965f5b60208801515181101561235c57898b612222612203848a516117bd565b516122156118f1612050868c87611c0f565b60ff161c60019081161490565b612231575b50506001016121e6565b88886122b8612278611e83879f8f978060e08f849c60206122616118f1612050839f98612269966122729a611c0f565b9a01516117bd565b519a01516117bd565b516117bd565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830195909552919093166064840152829081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156104905761231b8e6001948e86955f92612326575b50611a496121bc929351936123166121ca84876117bd565b611c67565b01989050898b612227565b6121bc925061234e611a499160203d8111612355575b6123468183610218565b810190611c52565b92506122fe565b503d61233c565b509594976001919993929497500190611fb1565b6123879150843d8111612355576123468183610218565b5f6121ae565b6123ae915060203d81116123b4575b6123a68183610218565b810190611c1b565b5f6120e3565b503d61239c565b6123d4915060203d602011611b2857611b1a8183610218565b5f611e0b565b5060e0840151518514611d27565b5060c0840151518514611d21565b5060a0840151518514611d1b565b1561240b57565b634394dbdf60e11b5f5260045ffd5b61244e337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612404565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b15610291575f928392604051948580948193639926ee7d60e01b835260018060a01b031660048301526040602483015260406124c382516060604486015260a48501906124f4565b91602081015160648501520151608483015203925af18015610490576124e65750565b80610ed25f61024993610218565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91908110156115575760051b8101359060be1981360301821215610291570190565b903590601e198136030182121561029157018035906001600160401b03821161029157602001918160061b3603831361029157565b91908110156115575760061b0190565b35611ac9816103d9565b9035601e19823603018112156102915701602081359101916001600160401b038211610291578160061b3603831361029157565b916020908281520191905f5b8181106125d65750505090565b90919260408060019286356125ea816103d9565b848060a01b031681526001600160601b03602088013561260981611772565b1660208201520194019291016125c9565b9035601e19823603018112156102915701602081359101916001600160401b03821161029157813603831361029157565b908060209392818452848401375f828201840152601f01601f1916010190565b6001600160a01b0390911681526040602082018190528101839052600583901b810160609081019383923684900360be1901925f91908101905b8383106126b6575050505050505090565b90919293949596605f19828203018352873586811215610291578701906126ee6126e08380612589565b60c0845260c08401916125bd565b9160208101356126fd816103d9565b6001600160a01b031660208381019190915261271c6040830183612589565b848603604086015280865294909101935f5b8181106127a7575050506127966001936020936127888461276261275560608998016107d0565b63ffffffff166060850152565b61277e612771608083016107d0565b63ffffffff166080850152565b60a081019061261a565b9160a081850391015261264b565b9901930193019195949392906126a5565b90919460408060019288356127bb816103d9565b848060a01b0316815260208901356020820152019601910191909161272e565b906020611ac99281815201906124f4565b602080825282516001600160a01b039081168284015281840151166040808401919091529092015160608083015280516080830181905260a09092019201905f5b81811061283a5750505090565b825163ffffffff1684526020938401939092019160010161282d565b1561285d57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156104905760ff915f91612ac7575b50168015612abd577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b808310612a79575061295491506116c3565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156104905760ff915f91612a5b575b5016811015612a5457604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa918215610490575f92612a34575b50905f915b8183106129ce57505050600101612959565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa91821561049057611a5e8b611a4f83611a49611a3d600198612a2b985f91611a6c5750516001600160a01b031690565b950191906129bc565b612a4d91925060203d811161196a5761195b8183610218565b905f6129b7565b5092505050565b612a73915060203d8111611b2857611b1a8183610218565b5f61297f565b604051633ca5a5f560e01b815260ff84166004820152909190602081602481885afa801561049057600192612ab4925f926119415750611765565b92019190612942565b5050611ac96116a8565b612ae0915060203d602011611b2857611b1a8183610218565b5f61290a565b91908110156115575760051b81013590609e1981360301821215610291570190565b909180602083016020845252604082019260408260051b8401019381935f91609e1984360301915b858410612b41575050505050505090565b90919293949596603f19828203018352873590848212156102915760208091886001940190608063ffffffff612bc882612b8c612b7e8780612589565b60a0885260a08801916125bd565b9587810135612b9a816103d9565b8a8060a01b0316888701526040810135604087015283612bbc606083016107d0565b166060870152016107d0565b16910152990193019401929195949390612b30565b60405190612bea826101dd565b5f6020838281520152565b60405190610180612c068184610218565b368337565b60405190612c1a602083610218565b6020368337565b91906040906060612c30612bdd565b9485926020855192612c428585610218565b8436853780518452015160208301528482015260076107cf195a01fa15612c6557565bfe5b602092916080604092612c78612bdd565b95869381865193612c898686610218565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa8015612c655715612cba57565b63d4b68fd760e01b5f5260045ffd5b604051612cd5816101dd565b6040908151612ce48382610218565b8236823781526020825191612cf98484610218565b8336843701528051612d0b8282610218565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6020820152815190612d618383610218565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152612db683519384610218565b8252602082015290565b5f5160206137805f395f51905f5290612dd7612bdd565b505f919006602060c0835b612ed7575f935f5160206137805f395f51905f5260038186818180090908604051612e0d8582610218565b84368237848185604051612e218282610218565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f5160206137805f395f51905f5260a082015260056107cf195a01fa8015612c6557612e8b906136d4565b5191612ed7575f5160206137805f395f51905f5282800914612ec257505f5160206137805f395f51905f5260015f94089293612de2565b92935050612ece61024b565b92835282015290565b61155c565b612ee4612bdd565b50604051612ef1816101dd565b600181526002602082015290565b9060068202918083046006149015171561172857565b90600c8110156115575760051b0190565b93929091612f34604061025a565b9485526020850152612f46604061025a565b9182526020820152612f56612bf5565b925f5b60028110612f8357505050602061018092612f72612c0b565b93849160086201d4c0fa9151151590565b80612f8f600192612eff565b612f998285611546565b5151612fa58289612f15565b526020612fb28386611546565b510151612fc7612fc18361171a565b89612f15565b52612fd28286611546565b515151612fe1612fc18361172d565b52612ff7612fef8387611546565b515160200190565b51613004612fc18361173b565b5260206130118387611546565b51015151613021612fc183611749565b5261304d6130476130406020613037868a611546565b51015160200190565b5192611757565b88612f15565b5201612f59565b6033546001600160a01b0316330361306857565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b61ffff6130b8826131d3565b166130c281610be3565b906130d06040519283610218565b8082526130df601f1991610be3565b013660208301375f5f5b825182108061313f575b15613138576001811b8416613111575b61310c906117d1565b6130e9565b90600161310c9160ff60f81b8460f81b165f1a61312e82876116f5565b5301919050613103565b5050905090565b5061010081106130f3565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b9060016131b660ff9361352d565b928392161b11156131c45790565b63ca95733360e01b5f5260045ffd5b805f915b6131df575090565b5f1981018181116117285761ffff9116911661ffff81146117285760010190806131d7565b9061320d612bdd565b5061ffff8116906102008210156132b257600182146132ad5761322e61024b565b5f81525f602082015292906001905f925b61ffff831685101561325357505050505090565b600161ffff831660ff86161c81161461328d575b60016132836132788360ff94612c67565b9460011b61fffe1690565b940116929161323f565b9460016132836132786132a28960ff95612c67565b989350505050613267565b505090565b637fc4ea7d60e11b5f5260045ffd5b6132c9612bdd565b5080519081158061333a575b156132f65750506040516132ea604082610218565b5f81525f602082015290565b60205f5160206137805f395f51905f52910151065f5160206137805f395f51905f52035f5160206137805f395f51905f5281116117285760405191612db6836101dd565b506020810151156132d5565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6065546001600160a01b031633036133a257565b638e79fdb560e01b5f5260045ffd5b6040516323b872dd60e01b60208201526001600160a01b039283166024820152929091166044830152606480830193909352918152610249916133f5608483610218565b61361c565b604051636eb1769f60e11b81523060048201526001600160a01b0383166024820152602081806044810103816001600160a01b0386165afa90811561049057610249946133f592613451925f916134855750611765565b60405163095ea7b360e01b60208201526001600160a01b039490941660248501526044808501919091528352606483610218565b61349e915060203d60201161196a5761195b8183610218565b5f611933565b9060ff5f5460081c16156134be5761069e61024992613346565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b1561351e57565b631019106960e31b5f5260045ffd5b906101008251116135965781511561359157602082015160019060f81c81901b5b835182101561358c5760019061357761356d6118f16118e386896116f5565b60ff600191161b90565b90613583818311613517565b1791019061354e565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b90816020910312610291575180151581036102915790565b156135c457565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b6102499161369e9160018060a01b03165f806040519361363d604086610218565b602085527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564602086015260208151910182855af13d156136cc573d9161368283610be3565b926136906040519485610218565b83523d5f602085013e6136ea565b80519081159182156136b2575b50506135bd565b6136c592506020809183010191016135a5565b5f806136ab565b6060916136ea565b156136db57565b63d51edae360e01b5f5260045ffd5b9192901561374c57508151156136fe575090565b3b156137075790565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b82519091501561375f5750805190602001fd5b60405162461bcd60e51b815290819061377b90600483016127db565b0390fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a264697066735822122036d93be62503ce2e5a6aab237e870984bcff3e60933878bff1cf0cc8d2f686e764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\xC0\x80`@R4a\x02\x94W`\xC0\x81a;\xDE\x808\x03\x80\x91a\0 \x82\x85a\x03\xBDV[\x839\x81\x01\x03\x12a\x02\x94W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x94W` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x90\x91\x90\x83\x83\x03a\x02\x94Wa\0b`@\x82\x01a\x03\xF4V[``\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x02\x94W`\x80\x83\x01Q\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x84\x03a\x02\x94W`\xA0\x01Q\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x85\x03a\x02\x94W`\x80R`\xC0R\x83`\xE0Ra\x01\0Ra\x01 R`\xA0R_T`\xFF\x81`\x08\x1C\x16a\x03hW`\xFF\x80\x82\x16\x03a\x03.W[Pa\x01@R`@Qch0H5`\xE0\x1B\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x02\xA0W_\x91a\x02\xEDW[Pa\x01`R`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\x02\xA0W_\x91a\x02\xABW[Pa\x01\x80Ra\x01`Q`@Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x02\xA0W_\x91a\x02ZW[Pa\x01\xA0R`@Qa7\xD5\x90\x81a\x04\t\x829`\x80Q\x81\x81\x81a\x07J\x01R\x81\x81a\x0F\xD1\x01R\x81\x81a\x10\x86\x01Ra$P\x01R`\xA0Q\x81a\x12\0\x01R`\xC0Q\x81\x81\x81a\r\x82\x01R\x81\x81a\x0EV\x01Ra\x14w\x01R`\xE0Q\x81\x81\x81a\x0F\xA1\x01R\x81\x81a\x11\xA8\x01R\x81\x81a\x17\xFE\x01R\x81\x81a$ \x01Ra(\xC8\x01Ra\x01\0Q\x81\x81\x81a\x18\x9A\x01Ra)\x14\x01Ra\x01 Q\x81\x81\x81a\x04\x12\x01R\x81\x81a\x04\xEA\x01R\x81\x81a\x05\x9A\x01R\x81\x81a\x0C\xF8\x01Ra\x10\xF2\x01Ra\x01@Q\x81\x81\x81a\x07\x8E\x01R\x81\x81a\x1D\xD0\x01Ra\x1E\xC3\x01Ra\x01`Q\x81\x81\x81a\x07\x06\x01R\x81\x81a!m\x01Ra\"\xBC\x01Ra\x01\x80Q\x81\x81\x81a\x06\xC2\x01Ra \xAB\x01Ra\x01\xA0Q\x81a\x13[\x01R\xF3[\x90P` \x81=` \x11a\x02\x98W[\x81a\x02u` \x93\x83a\x03\xBDV[\x81\x01\x03\x12a\x02\x94WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x94W_a\x01aV[_\x80\xFD[=\x91Pa\x02hV[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x02\xE5W[\x81a\x02\xC6` \x93\x83a\x03\xBDV[\x81\x01\x03\x12a\x02\x94WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x94W_a\x01'V[=\x91Pa\x02\xB9V[\x90P` \x81=` \x11a\x03&W[\x81a\x03\x08` \x93\x83a\x03\xBDV[\x81\x01\x03\x12a\x02\x94W`\x04\x91a\x03\x1E` \x92a\x03\xF4V[\x91P\x91a\0\xFAV[=\x91Pa\x02\xFBV[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xD1V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x03\xE0W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\x94WV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x17\x1F\x1D[\x14a\x01\xC4W\x80c\x17\x85\xF5<\x14a\x01\xBFW\x80c\x1F\xDB\x0C\xFD\x14a\x01\xBAW\x80c'\x942\xEB\x14a\x01\xB5W\x80c3\xCF\xB7\xB7\x14a\x01\xB0W\x80c;\xC2\x8C\x8C\x14a\x01\xABW\x80c]\xF4YF\x14a\x01\xA6W\x80ch0H5\x14a\x01\xA1W\x80ck:\xA7.\x14a\x01\x9CW\x80cm\x14\xA9\x87\x14a\x01\x97W\x80cn\xFBF6\x14a\x01\x92W\x80cqP\x18\xA6\x14a\x01\x8DW\x80c\x8D\xA5\xCB[\x14a\x01\x88W\x80c\x99&\xEE}\x14a\x01\x83W\x80c\x9D\xA1m\x8E\x14a\x01~W\x80c\xA0\x16\x9D\xDD\x14a\x01yW\x80c\xA2\x0B\x99\xBF\x14a\x01tW\x80c\xA3d\xF4\xDA\x14a\x01oW\x80c\xA9\x8F\xB3U\x14a\x01jW\x80c\xBAU\x08\x80\x14a\x01eW\x80c\xC1\xA8\xE2\xC5\x14a\x01`W\x80c\xC4\xD6m\xE8\x14a\x01[W\x80c\xDF\\\xF7#\x14a\x01VW\x80c\xE4\x81\xAF\x9D\x14a\x01QW\x80c\xF2\xFD\xE3\x8B\x14a\x01LW\x80c\xFC)\x9D\xEE\x14a\x01GWc\xFC\xE3l}\x14a\x01BW_\x80\xFD[a\x14^V[a\x146V[a\x13\xA5V[a\x13\x8AV[a\x13FV[a\x12QV[a\x11fV[a\x10\xDAV[a\x10:V[a\x0F}V[a\x0E=V[a\rZV[a\x0C\xD0V[a\x0C4V[a\x0B\xBBV[a\x0B`V[a\n\xCBV[a\x07yV[a\x075V[a\x06\xF1V[a\x06\xADV[a\x06vV[a\x06>V[a\x05rV[a\x04\xD2V[a\x03\xEAV[a\x03qV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xF8W`@RV[a\x01\xC9V[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xF8W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xF8W`@RV[`@Q\x90a\x02Ia\x01\0\x83a\x02\x18V[V[`@Q\x90a\x02I`@\x83a\x02\x18V[\x90a\x02I`@Q\x92\x83a\x02\x18V[`@\x90`\xE3\x19\x01\x12a\x02\x91W`@Q\x90a\x02\x81\x82a\x01\xDDV[`\xE45\x82Ra\x01\x045` \x83\x01RV[_\x80\xFD[\x91\x90\x82`@\x91\x03\x12a\x02\x91W`@Qa\x02\xAD\x81a\x01\xDDV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x91W`@Q\x91a\x02\xD8`@\x84a\x02\x18V[\x82\x90`@\x81\x01\x92\x83\x11a\x02\x91W\x90[\x82\x82\x10a\x02\xF4WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x02\xE7V[\x90`\x80`c\x19\x83\x01\x12a\x02\x91W`@Qa\x03\x1D\x81a\x01\xDDV[` a\x038\x82\x94a\x03/\x81`da\x02\xBDV[\x84R`\xA4a\x02\xBDV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x02\x91W` a\x038`@Q\x92a\x03[\x84a\x01\xDDV[`@\x84\x96a\x03i\x83\x82a\x02\xBDV[\x86R\x01a\x02\xBDV[4a\x02\x91Wa\x01 6`\x03\x19\x01\x12a\x02\x91W`\x045`@6`#\x19\x01\x12a\x02\x91Wa\x03\xC9`@\x91\x82Qa\x03\xA3\x81a\x01\xDDV[`$5\x81R`D5` \x82\x01Ra\x03\xB96a\x03\x04V[\x90a\x03\xC36a\x02hV[\x92a\x15pV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\x91WV[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x04\x08\x81a\x03\xD9V[a\x04\x10a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x91W`@Qc&\x89Y\xE5`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x04\x90Wa\x04\x82WP\x80\xF3[a\x04\x8E\x91P_\x90a\x02\x18V[\0[a\x16VV[``\x90`\x03\x19\x01\x12a\x02\x91W`\x045a\x04\xAD\x81a\x03\xD9V[\x90`$5a\x04\xBA\x81a\x03\xD9V[\x90`D5`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x02\x91W\x90V[4a\x02\x91Wa\x04\xE06a\x04\x95V[a\x04\xE8a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x83;\x15a\x02\x91W`@QcJ\x86\xC07`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`$\x82\x01R\x92\x16`D\x83\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x82\x01R_\x81`\x84\x81\x01[\x93\x81\x83\x81\x81\x97\x03\x92Z\xF1\x80\x15a\x04\x90Wa\x04\x82WP\x80\xF3[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x05\x90\x81a\x03\xD9V[a\x05\x98a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x91W`@Qc\xEBZN\x87`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01a\x04pV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x06\x1FWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\x12V[4a\x02\x91W` 6`\x03\x19\x01\x12a\x02\x91Wa\x06ra\x06f`\x045a\x06a\x81a\x03\xD9V[a\x17\xDFV[`@Q\x91\x82\x91\x82a\x05\xFCV[\x03\x90\xF3[4a\x02\x91W` 6`\x03\x19\x01\x12a\x02\x91Wa\x04\x8E`\x045a\x06\x96\x81a\x03\xD9V[a\x06\x9Ea0TV[a1JV[_\x91\x03\x12a\x02\x91WV[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`D5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\x91WV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\x91WV[`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF8W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x91W\x815a\x08\x0F\x81a\x07\xE1V[\x92a\x08\x1D`@Q\x94\x85a\x02\x18V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\x91W` \x01\x90[\x82\x82\x10a\x08EWPPP\x90V[` \x80\x91a\x08R\x84a\x07\xD0V[\x81R\x01\x91\x01\x90a\x088V[\x81`\x1F\x82\x01\x12\x15a\x02\x91W\x805a\x08s\x81a\x07\xE1V[\x92a\x08\x81`@Q\x94\x85a\x02\x18V[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x02\x91W` \x01\x91[\x83\x83\x10a\x08\xABWPPPP\x90V[` `@\x91a\x08\xBA\x84\x86a\x02\x95V[\x81R\x01\x92\x01\x91a\x08\x9DV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x91W\x815a\x08\xDC\x81a\x07\xE1V[\x92a\x08\xEA`@Q\x94\x85a\x02\x18V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x02\x91W` \x82\x01\x90[\x83\x82\x10a\t\x16WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W` \x91a\t8\x87\x84\x80\x94\x88\x01\x01a\x07\xF8V[\x81R\x01\x91\x01\x90a\t\x07V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x02\x91Wa\tYa\x029V[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x81a\tv\x91\x84\x01a\x07\xF8V[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x81a\t\x97\x91\x84\x01a\x08]V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x81a\t\xBB\x91\x84\x01a\x08]V[`@\x85\x01Ra\t\xCD\x81``\x84\x01a\x03=V[``\x85\x01Ra\t\xDF\x81`\xE0\x84\x01a\x02\x95V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x81a\n\x04\x91\x84\x01a\x07\xF8V[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x81a\n)\x91\x84\x01a\x07\xF8V[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91Wa\nL\x92\x01a\x08\xC5V[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\npWPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\ncV[\x92\x91\x90a\n\xC6` \x91`@\x86R\x82a\n\xB2\x82Q`@\x80\x8A\x01R`\x80\x89\x01\x90a\nSV[\x91\x01Q\x86\x82\x03`?\x19\x01``\x88\x01Ra\nSV[\x93\x01RV[4a\x02\x91W`\x806`\x03\x19\x01\x12a\x02\x91W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W6`#\x82\x01\x12\x15a\x02\x91W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W6`$\x82\x84\x01\x01\x11a\x02\x91Wa\x0B\"a\x07\xBDV[\x90`d5\x93`\x01`\x01`@\x1B\x03\x85\x11a\x02\x91W`$a\x0BHa\x0BP\x966\x90`\x04\x01a\tCV[\x94\x01\x90a\x1C\xF3V[\x90a\x06r`@Q\x92\x83\x92\x83a\n\x8FV[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91Wa\x0Bxa0TV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF8W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x0C\n\x82a\x0B\xE3V[\x91a\x0C\x18`@Q\x93\x84a\x02\x18V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02\x91W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[4a\x02\x91W`@6`\x03\x19\x01\x12a\x02\x91W`\x045a\x0CQ\x81a\x03\xD9V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x91W```\x03\x19\x836\x03\x01\x12a\x02\x91W`@Q\x90a\x0C~\x82a\x01\xFDV[\x82`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x83\x016`#\x82\x01\x12\x15a\x02\x91Wa\x04\x8E\x93a\x0C\xB8`D\x926\x90`$`\x04\x82\x015\x91\x01a\x0B\xFEV[\x84R`$\x81\x015` \x85\x01R\x015`@\x83\x01Ra$\x1AV[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x0C\xEE\x81a\x03\xD9V[a\x0C\xF6a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x91W`@QcO\x90l\xF9`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01a\x04pV[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\rx\x81a\x03\xD9V[a\r\x80a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\x91W`@Qc\xA0\x16\x9D\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04\x90Wa\x04\x82WP\x80\xF3[\x90` `\x03\x19\x83\x01\x12a\x02\x91W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W`\x04\x01\x82`\x1F\x82\x01\x12\x15a\x02\x91W\x805\x92`\x01`\x01`@\x1B\x03\x84\x11a\x02\x91W` \x80\x83\x01\x92\x85`\x05\x1B\x01\x01\x11a\x02\x91W\x91\x90V[4a\x02\x91Wa\x0EK6a\r\xEBV[\x90a\x0ETa3\x8EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a\x0E\xD8WP\x82;\x15a\x02\x91Wa\x0E\xB4\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94cN\\\xD2\xFD`\xE1\x1B\x84R0`\x04\x85\x01a&kV[\x03\x92Z\xF1\x80\x15a\x04\x90Wa\x0E\xC4W\0[\x80a\x0E\xD2_a\x04\x8E\x93a\x02\x18V[\x80a\x06\xA3V[\x91_\x93\x91_\x91[a\x0E\xF7a\x0E\xED\x86\x84\x84a%\x18V[`@\x81\x01\x90a%:V[\x90P\x83\x10\x15a\x0F3W`\x01a\x0F)\x81\x97` a\x0F!\x87a\x0F\x1Ba\x0E\xED\x8C\x8A\x8Aa%\x18V[\x90a%oV[\x015\x90a\x17eV[\x93\x01\x92\x95Pa\x0E\xDFV[\x93\x90\x92\x94`\x01\x92Pa\x0Fw\x90a\x0Fa\x810\x88a\x0F\\` a\x0FV\x89\x8C3\x95a%\x18V[\x01a%\x7FV[a3\xB1V[\x86a\x0Fr` a\x0FV\x86\x89\x8Ba%\x18V[a3\xFAV[\x01a\x0E\x81V[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x0F\x9B\x81a\x03\xD9V[a\x0F\xCF3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a$\x04V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\x91W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04\x90Wa\x04\x82WP\x80\xF3[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W6`#\x82\x01\x12\x15a\x02\x91Wa\x10|\x906\x90`$\x81`\x04\x015\x91\x01a\x0B\xFEV[a\x10\x84a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x91W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x04p\x90`\x04\x83\x01a'\xDBV[4a\x02\x91Wa\x10\xE86a\x04\x95V[a\x10\xF0a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x83;\x15a\x02\x91W`@Qc\x06d\x12\x01`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`$\x82\x01R\x92\x16`D\x83\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x82\x01R_\x81`\x84\x81\x01a\x05ZV[4a\x02\x91W`@6`\x03\x19\x01\x12a\x02\x91W`\x045a\x11\x83\x81a\x03\xD9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91Wa\x11\xA2\x906\x90`\x04\x01a\x07\xF8V[a\x11\xD63\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a$\x04V[a\x11\xF3`@Q\x92a\x11\xE6\x84a\x01\xFDV[`\x01`\x01`\xA0\x1B\x03\x16\x83RV[0` \x83\x01R`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x91W`@Qcn4\x92\xB5`\xE0\x1B\x81R\x90_\x90\x82\x90\x81\x83\x81a\x0E\xB4\x88`\x04\x83\x01a'\xECV[4a\x02\x91W` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x12n\x81a\x03\xD9V[a\x12\xBE_T\x91a\x12\xA2a\x12\x8Ca\x12\x88\x85`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x94\x81\x95a\x138W[\x81\x15a\x13\x18W[Pa(VV[\x82a\x12\xB3`\x01`\xFF\x19_T\x16\x17_UV[a\x13\x01W[\x80a4\xA4V[a\x12\xC4W\0[a\x12\xD2a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x13\x13a\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x12\xB8V[0;\x15\x91P\x81a\x13*W[P_a\x12\x9CV[`\xFF\x16`\x01\x14\x90P_a\x13#V[`\x01`\xFF\x82\x16\x10\x91Pa\x12\x95V[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91Wa\x06ra\x06fa(\xB9V[4a\x02\x91W` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x13\xC2\x81a\x03\xD9V[a\x13\xCAa0TV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x13\xE2Wa\x04\x8E\x90a3FV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x91Wa\x14l6a\r\xEBV[\x90a\x14ua3\x8EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a\x14\xD4WP\x82;\x15a\x02\x91Wa\x0E\xB4\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xFC\xE3l}`\xE0\x1B\x84R`\x04\x84\x01a+\x08V[\x80a\x15\x03a\x14\xEA` a\x0FV`\x01\x95\x87\x89a*\xE6V[`@a\x14\xF7\x84\x87\x89a*\xE6V[\x015\x900\x903\x90a3\xB1V[a\x15,a\x15\x16` a\x0FV\x84\x87\x89a*\xE6V[\x86`@a\x15$\x85\x88\x8Aa*\xE6V[\x015\x91a3\xFAV[\x01a\x14\xA2V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x15WW`\x05\x1B\x01\x90V[a\x152V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x16La\x16)a\x16R\x95a\x16#a\x16\x1C\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x15\xF3\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x18V[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96a,!V[\x90a,gV[\x92a\x16#a\x16>a\x168a,\xC9V[\x94a-\xC0V[\x91a\x16Ga.\xDCV[a,!V[\x91a/&V[\x90\x91V[`@Q=_\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x02\x91WQ\x90V[\x90\x81` \x91\x03\x12a\x02\x91WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x02\x91W\x90V[\x90\x81` \x91\x03\x12a\x02\x91WQ`\xFF\x81\x16\x81\x03a\x02\x91W\x90V[`@Q\x90a\x16\xB7` \x83a\x02\x18V[_\x80\x83R6` \x84\x017V[\x90a\x16\xCD\x82a\x07\xE1V[a\x16\xDA`@Q\x91\x82a\x02\x18V[\x82\x81R\x80\x92a\x16\xEB`\x1F\x19\x91a\x07\xE1V[\x01\x90` 6\x91\x017V[\x90\x81Q\x81\x10\x15a\x15WW\x01` \x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x17(WV[a\x17\x06V[\x90`\x02\x82\x01\x80\x92\x11a\x17(WV[\x90`\x03\x82\x01\x80\x92\x11a\x17(WV[\x90`\x04\x82\x01\x80\x92\x11a\x17(WV[\x90`\x05\x82\x01\x80\x92\x11a\x17(WV[\x91\x90\x82\x01\x80\x92\x11a\x17(WV[`\x01`\x01``\x1B\x03\x81\x16\x03a\x02\x91WV[\x90\x81`@\x91\x03\x12a\x02\x91W` `@Q\x91a\x17\x9D\x83a\x01\xDDV[\x80Qa\x17\xA8\x81a\x03\xD9V[\x83R\x01Qa\x17\xB5\x81a\x17rV[` \x82\x01R\x90V[\x80Q\x82\x10\x15a\x15WW` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x17(W`\x01\x01\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x04\x90Wa\x18_\x91` \x91_\x91a\x1B^W[P`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x90\x81\x15a\x04\x90W_\x91a\x1B/W[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81\x15\x90\x81\x15a\x1A\xCCW[Pa\x1A\xC0Wa\x18\x95\x90a0\xACV[_\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x19qWa\x19\x1A` a\x18\xF7a\x18\xF1a\x18\xE3\x89\x87a\x16\xF5V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x87Z\xFA\x80\x15a\x04\x90W`\x01\x92a\x199\x92_\x92a\x19AW[Pa\x17eV[\x94\x01\x93a\x18\xC5V[a\x19c\x91\x92P` =\x81\x11a\x19jW[a\x19[\x81\x83a\x02\x18V[\x81\x01\x90a\x16aV[\x90_a\x193V[P=a\x19QV[a\x19|\x91\x94Pa\x16\xC3V[\x92_\x90_[\x81Q\x81\x10\x15a\x1A\xBAWa\x19\x9Aa\x18\xF1a\x18\xE3\x83\x85a\x16\xF5V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x90` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x04\x90W_\x92a\x1A\x9AW[P\x90_\x91[\x81\x83\x10a\x19\xDFWPPP`\x01\x01a\x19\x81V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x04\x90Wa\x1A^\x8Ba\x1AO\x83a\x1AIa\x1A=`\x01\x98a\x1Ac\x98_\x91a\x1AlW[PQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x17\xBDV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x17\xD1V[\x95\x01\x91\x90a\x19\xCDV[a\x1A\x8D\x91P`@=\x81\x11a\x1A\x93W[a\x1A\x85\x81\x83a\x02\x18V[\x81\x01\x90a\x17\x83V[_a\x1A/V[P=a\x1A{V[a\x1A\xB3\x91\x92P` =\x81\x11a\x19jWa\x19[\x81\x83a\x02\x18V[\x90_a\x19\xC8V[PPPPV[Pa\x1A\xC9a\x16\xA8V[\x90V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x04\x90W`\xFF\x91_\x91a\x1B\0W[P\x16\x15_a\x18\x87V[a\x1B\"\x91P` =` \x11a\x1B(W[a\x1B\x1A\x81\x83a\x02\x18V[\x81\x01\x90a\x16\x8FV[_a\x1A\xF7V[P=a\x1B\x10V[a\x1BQ\x91P` =` \x11a\x1BWW[a\x1BI\x81\x83a\x02\x18V[\x81\x01\x90a\x16pV[_a\x18rV[P=a\x1B?V[a\x1Bu\x91P\x82=\x84\x11a\x19jWa\x19[\x81\x83a\x02\x18V[_a\x18?V[`@Q\x90a\x1B\x88\x82a\x01\xDDV[``` \x83\x82\x81R\x01RV[\x15a\x1B\x9BWV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a\x1B\xB0WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a\x1B\xC6WV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a\x1B\xDCWV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[_\x19\x81\x01\x91\x90\x82\x11a\x17(WV[\x15a\x1C\0WV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90\x82\x10\x15a\x15WW\x01\x90V[\x90\x81` \x91\x03\x12a\x02\x91WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x02\x91W\x90V[\x15a\x1CCWV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\x91WQa\x1A\xC9\x81a\x17rV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a\x17(WV[\x15a\x1C\x8EWV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a\x1C\xA4WV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a\x1C\xDDWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1C\xD0V[\x94\x93\x92\x90\x91\x93a\x1D\x01a\x1B{V[Pa\x1D\r\x85\x15\x15a\x1B\x94V[`@\x84\x01QQ\x85\x14\x80a#\xF6W[\x80a#\xE8W[\x80a#\xDAW[a\x1D0\x90a\x1B\xA9V[a\x1DB` \x85\x01QQ\x85QQ\x14a\x1B\xBFV[a\x1DYc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a\x1B\xD5V[a\x1Daa\x02KV[_\x81R_` \x82\x01R\x92a\x1Dsa\x1B{V[a\x1D|\x87a\x16\xC3V[` \x82\x01Ra\x1D\x8A\x87a\x16\xC3V[\x81Ra\x1D\x94a\x1B{V[\x92a\x1D\xA3` \x88\x01QQa\x16\xC3V[\x84Ra\x1D\xB3` \x88\x01QQa\x16\xC3V[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x04\x90Wa\x1E\x1C\x91_\x91a#\xBBW[Pa\x1E\x176\x8B\x87a\x0B\xFEV[a1\xA8V[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a\x1F\x98W` \x88a\x1E\x8Da\x1E\x83\x8Ca\x1E{\x8F\x96\x86\x8Ea\x1E`a\x1EM\x86\x80\x95a\x17\xBDV[Q\x80Q_R` \x01Q` R`@_ \x90V[a\x1Em\x84\x84\x84\x01Qa\x17\xBDV[R\x82a\x1FeW[\x01Qa\x17\xBDV[Q\x95Qa\x17\xBDV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x04\x90Wa\x16#\x8Aa\x1F:\x8Fa\x1F3\x8F\x84` \x8F\x92a\x1F*\x93a\x1F\"\x84`\x01\x9Ea\x1F@\x9E_\x91a\x1FHW[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\x17\xBDV[R\x01Qa\x17\xBDV[Q\x93\x8DQa\x17\xBDV[Q\x16a1\xD3V[\x90a2\x04V[\x97\x01\x96a\x1E V[a\x1F_\x91P\x86=\x81\x11a\x1BWWa\x1BI\x81\x83a\x02\x18V[_a\x1F\x13V[a\x1F\x93a\x1Fu\x84\x84\x84\x01Qa\x17\xBDV[Qa\x1F\x8C\x84\x84\x01Qa\x1F\x86\x87a\x1B\xEBV[\x90a\x17\xBDV[Q\x10a\x1B\xF9V[a\x1EtV[P\x90\x95\x97\x94\x96Pa\x1F\xAD\x91\x98\x93\x92\x99Pa2\xC1V[\x91_\x90[\x80\x82\x10a \x13WPPP\x92a\x1F\xECa\x1F\xE7a\x1F\xE0a \r\x95\x85a\x1F\xFF\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x15pV[\x91\x90a\x1C\x87V[a\x1C\x9DV[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a\x1C\xB3V[\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x18V[Q\x90 \x90V[\x91\x84\x95\x96\x93a ^a\x18\xF1a P\x85\x87a J\x87\x9F\x98\x9E\x9Ba\x1EM\x84`@a <\x93\x01Qa\x17\xBDV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x96a\x1C\x0FV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[` \x87a ra\x1E\x83\x8D`\xA0\x8D\x01Qa\x17\xBDV[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x92\x83\x15a\x04\x90Wa!\x0E\x93a \xF9\x92_\x91a#\x8DW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a\x1C<V[a!\x07\x89`@\x89\x01Qa\x17\xBDV[Q\x90a,gV[\x90a! a\x18\xF1a P\x8A\x86\x8Da\x1C\x0FV[` \x86a!4a\x1E\x83\x8C`\xC0\x8C\x01Qa\x17\xBDV[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x04\x90Wa!\xCA\x8A` \x8Ea\x1Eta!\xD7\x95a!\xE2\x97_\x91a#pW[Pa!\xBC\x85\x85\x85\x01Qa\x17\xBDV[\x90`\x01`\x01``\x1B\x03\x16\x90RV[Q`\x01`\x01``\x1B\x03\x16\x90V[a!\xBC\x8A\x8DQa\x17\xBDV[_\x96_[` \x88\x01QQ\x81\x10\x15a#\\W\x89\x8Ba\"\"a\"\x03\x84\x8AQa\x17\xBDV[Qa\"\x15a\x18\xF1a P\x86\x8C\x87a\x1C\x0FV[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a\"1W[PP`\x01\x01a!\xE6V[\x88\x88a\"\xB8a\"xa\x1E\x83\x87\x9F\x8F\x97\x80`\xE0\x8F\x84\x9C` a\"aa\x18\xF1a P\x83\x9F\x98a\"i\x96a\"r\x9Aa\x1C\x0FV[\x9A\x01Qa\x17\xBDV[Q\x9A\x01Qa\x17\xBDV[Qa\x17\xBDV[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x95\x90\x95R\x91\x90\x93\x16`d\x84\x01R\x82\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x04\x90Wa#\x1B\x8E`\x01\x94\x8E\x86\x95_\x92a#&W[Pa\x1AIa!\xBC\x92\x93Q\x93a#\x16a!\xCA\x84\x87a\x17\xBDV[a\x1CgV[\x01\x98\x90P\x89\x8Ba\"'V[a!\xBC\x92Pa#Na\x1AI\x91` =\x81\x11a#UW[a#F\x81\x83a\x02\x18V[\x81\x01\x90a\x1CRV[\x92Pa\"\xFEV[P=a#<V[P\x95\x94\x97`\x01\x91\x99\x93\x92\x94\x97P\x01\x90a\x1F\xB1V[a#\x87\x91P\x84=\x81\x11a#UWa#F\x81\x83a\x02\x18V[_a!\xAEV[a#\xAE\x91P` =\x81\x11a#\xB4W[a#\xA6\x81\x83a\x02\x18V[\x81\x01\x90a\x1C\x1BV[_a \xE3V[P=a#\x9CV[a#\xD4\x91P` =` \x11a\x1B(Wa\x1B\x1A\x81\x83a\x02\x18V[_a\x1E\x0BV[P`\xE0\x84\x01QQ\x85\x14a\x1D'V[P`\xC0\x84\x01QQ\x85\x14a\x1D!V[P`\xA0\x84\x01QQ\x85\x14a\x1D\x1BV[\x15a$\x0BWV[cC\x94\xDB\xDF`\xE1\x1B_R`\x04_\xFD[a$N3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a$\x04V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x02\x91W_\x92\x83\x92`@Q\x94\x85\x80\x94\x81\x93c\x99&\xEE}`\xE0\x1B\x83R`\x01\x80`\xA0\x1B\x03\x16`\x04\x83\x01R`@`$\x83\x01R`@a$\xC3\x82Q```D\x86\x01R`\xA4\x85\x01\x90a$\xF4V[\x91` \x81\x01Q`d\x85\x01R\x01Q`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\x04\x90Wa$\xE6WPV[\x80a\x0E\xD2_a\x02I\x93a\x02\x18V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90\x81\x10\x15a\x15WW`\x05\x1B\x81\x015\x90`\xBE\x19\x816\x03\x01\x82\x12\x15a\x02\x91W\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x02\x91W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x91W` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\x02\x91WV[\x91\x90\x81\x10\x15a\x15WW`\x06\x1B\x01\x90V[5a\x1A\xC9\x81a\x03\xD9V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x02\x91W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x02\x91W\x81`\x06\x1B6\x03\x83\x13a\x02\x91WV[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a%\xD6WPPP\x90V[\x90\x91\x92`@\x80`\x01\x92\x865a%\xEA\x81a\x03\xD9V[\x84\x80`\xA0\x1B\x03\x16\x81R`\x01`\x01``\x1B\x03` \x88\x015a&\t\x81a\x17rV[\x16` \x82\x01R\x01\x94\x01\x92\x91\x01a%\xC9V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x02\x91W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x02\x91W\x816\x03\x83\x13a\x02\x91WV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x83\x90R`\x05\x83\x90\x1B\x81\x01``\x90\x81\x01\x93\x83\x926\x84\x90\x03`\xBE\x19\x01\x92_\x91\x90\x81\x01\x90[\x83\x83\x10a&\xB6WPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`_\x19\x82\x82\x03\x01\x83R\x875\x86\x81\x12\x15a\x02\x91W\x87\x01\x90a&\xEEa&\xE0\x83\x80a%\x89V[`\xC0\x84R`\xC0\x84\x01\x91a%\xBDV[\x91` \x81\x015a&\xFD\x81a\x03\xD9V[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x81\x01\x91\x90\x91Ra'\x1C`@\x83\x01\x83a%\x89V[\x84\x86\x03`@\x86\x01R\x80\x86R\x94\x90\x91\x01\x93_[\x81\x81\x10a'\xA7WPPPa'\x96`\x01\x93` \x93a'\x88\x84a'ba'U``\x89\x98\x01a\x07\xD0V[c\xFF\xFF\xFF\xFF\x16``\x85\x01RV[a'~a'q`\x80\x83\x01a\x07\xD0V[c\xFF\xFF\xFF\xFF\x16`\x80\x85\x01RV[`\xA0\x81\x01\x90a&\x1AV[\x91`\xA0\x81\x85\x03\x91\x01Ra&KV[\x99\x01\x93\x01\x93\x01\x91\x95\x94\x93\x92\x90a&\xA5V[\x90\x91\x94`@\x80`\x01\x92\x885a'\xBB\x81a\x03\xD9V[\x84\x80`\xA0\x1B\x03\x16\x81R` \x89\x015` \x82\x01R\x01\x96\x01\x91\x01\x91\x90\x91a'.V[\x90` a\x1A\xC9\x92\x81\x81R\x01\x90a$\xF4V[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x84\x01R\x81\x84\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x90\x92\x01Q``\x80\x83\x01R\x80Q`\x80\x83\x01\x81\x90R`\xA0\x90\x92\x01\x92\x01\x90_[\x81\x81\x10a(:WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a(-V[\x15a(]WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x04\x90W`\xFF\x91_\x91a*\xC7W[P\x16\x80\x15a*\xBDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a*yWPa)T\x91Pa\x16\xC3V[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x04\x90W`\xFF\x91_\x91a*[W[P\x16\x81\x10\x15a*TW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x04\x90W_\x92a*4W[P\x90_\x91[\x81\x83\x10a)\xCEWPPP`\x01\x01a)YV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x04\x90Wa\x1A^\x8Ba\x1AO\x83a\x1AIa\x1A=`\x01\x98a*+\x98_\x91a\x1AlWPQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01\x91\x90a)\xBCV[a*M\x91\x92P` =\x81\x11a\x19jWa\x19[\x81\x83a\x02\x18V[\x90_a)\xB7V[P\x92PPPV[a*s\x91P` =\x81\x11a\x1B(Wa\x1B\x1A\x81\x83a\x02\x18V[_a)\x7FV[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x04\x90W`\x01\x92a*\xB4\x92_\x92a\x19AWPa\x17eV[\x92\x01\x91\x90a)BV[PPa\x1A\xC9a\x16\xA8V[a*\xE0\x91P` =` \x11a\x1B(Wa\x1B\x1A\x81\x83a\x02\x18V[_a)\nV[\x91\x90\x81\x10\x15a\x15WW`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x02\x91W\x01\x90V[\x90\x91\x80` \x83\x01` \x84RR`@\x82\x01\x92`@\x82`\x05\x1B\x84\x01\x01\x93\x81\x93_\x91`\x9E\x19\x846\x03\x01\x91[\x85\x84\x10a+AWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`?\x19\x82\x82\x03\x01\x83R\x875\x90\x84\x82\x12\x15a\x02\x91W` \x80\x91\x88`\x01\x94\x01\x90`\x80c\xFF\xFF\xFF\xFFa+\xC8\x82a+\x8Ca+~\x87\x80a%\x89V[`\xA0\x88R`\xA0\x88\x01\x91a%\xBDV[\x95\x87\x81\x015a+\x9A\x81a\x03\xD9V[\x8A\x80`\xA0\x1B\x03\x16\x88\x87\x01R`@\x81\x015`@\x87\x01R\x83a+\xBC``\x83\x01a\x07\xD0V[\x16``\x87\x01R\x01a\x07\xD0V[\x16\x91\x01R\x99\x01\x93\x01\x94\x01\x92\x91\x95\x94\x93\x90a+0V[`@Q\x90a+\xEA\x82a\x01\xDDV[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80a,\x06\x81\x84a\x02\x18V[6\x837V[`@Q\x90a,\x1A` \x83a\x02\x18V[` 6\x837V[\x91\x90`@\x90``a,0a+\xDDV[\x94\x85\x92` \x85Q\x92a,B\x85\x85a\x02\x18V[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15a,eWV[\xFE[` \x92\x91`\x80`@\x92a,xa+\xDDV[\x95\x86\x93\x81\x86Q\x93a,\x89\x86\x86a\x02\x18V[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15a,eW\x15a,\xBAWV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@Qa,\xD5\x81a\x01\xDDV[`@\x90\x81Qa,\xE4\x83\x82a\x02\x18V[\x826\x827\x81R` \x82Q\x91a,\xF9\x84\x84a\x02\x18V[\x836\x847\x01R\x80Qa-\x0B\x82\x82a\x02\x18V[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90a-a\x83\x83a\x02\x18V[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01Ra-\xB6\x83Q\x93\x84a\x02\x18V[\x82R` \x82\x01R\x90V[_Q` a7\x80_9_Q\x90_R\x90a-\xD7a+\xDDV[P_\x91\x90\x06` `\xC0\x83[a.\xD7W_\x93_Q` a7\x80_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa.\r\x85\x82a\x02\x18V[\x846\x827\x84\x81\x85`@Qa.!\x82\x82a\x02\x18V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` a7\x80_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a,eWa.\x8B\x90a6\xD4V[Q\x91a.\xD7W_Q` a7\x80_9_Q\x90_R\x82\x80\t\x14a.\xC2WP_Q` a7\x80_9_Q\x90_R`\x01_\x94\x08\x92\x93a-\xE2V[\x92\x93PPa.\xCEa\x02KV[\x92\x83R\x82\x01R\x90V[a\x15\\V[a.\xE4a+\xDDV[P`@Qa.\xF1\x81a\x01\xDDV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a\x17(WV[\x90`\x0C\x81\x10\x15a\x15WW`\x05\x1B\x01\x90V[\x93\x92\x90\x91a/4`@a\x02ZV[\x94\x85R` \x85\x01Ra/F`@a\x02ZV[\x91\x82R` \x82\x01Ra/Va+\xF5V[\x92_[`\x02\x81\x10a/\x83WPPP` a\x01\x80\x92a/ra,\x0BV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80a/\x8F`\x01\x92a.\xFFV[a/\x99\x82\x85a\x15FV[QQa/\xA5\x82\x89a/\x15V[R` a/\xB2\x83\x86a\x15FV[Q\x01Qa/\xC7a/\xC1\x83a\x17\x1AV[\x89a/\x15V[Ra/\xD2\x82\x86a\x15FV[QQQa/\xE1a/\xC1\x83a\x17-V[Ra/\xF7a/\xEF\x83\x87a\x15FV[QQ` \x01\x90V[Qa0\x04a/\xC1\x83a\x17;V[R` a0\x11\x83\x87a\x15FV[Q\x01QQa0!a/\xC1\x83a\x17IV[Ra0Ma0Ga0@` a07\x86\x8Aa\x15FV[Q\x01Q` \x01\x90V[Q\x92a\x17WV[\x88a/\x15V[R\x01a/YV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a0hWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[a\xFF\xFFa0\xB8\x82a1\xD3V[\x16a0\xC2\x81a\x0B\xE3V[\x90a0\xD0`@Q\x92\x83a\x02\x18V[\x80\x82Ra0\xDF`\x1F\x19\x91a\x0B\xE3V[\x016` \x83\x017__[\x82Q\x82\x10\x80a1?W[\x15a18W`\x01\x81\x1B\x84\x16a1\x11W[a1\x0C\x90a\x17\xD1V[a0\xE9V[\x90`\x01a1\x0C\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa1.\x82\x87a\x16\xF5V[S\x01\x91\x90Pa1\x03V[PP\x90P\x90V[Pa\x01\0\x81\x10a0\xF3V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eUV[\x90`\x01a1\xB6`\xFF\x93a5-V[\x92\x83\x92\x16\x1B\x11\x15a1\xC4W\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[a1\xDFWP\x90V[_\x19\x81\x01\x81\x81\x11a\x17(Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x17(W`\x01\x01\x90\x80a1\xD7V[\x90a2\ra+\xDDV[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15a2\xB2W`\x01\x82\x14a2\xADWa2.a\x02KV[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15a2SWPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14a2\x8DW[`\x01a2\x83a2x\x83`\xFF\x94a,gV[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91a2?V[\x94`\x01a2\x83a2xa2\xA2\x89`\xFF\x95a,gV[\x98\x93PPPPa2gV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[a2\xC9a+\xDDV[P\x80Q\x90\x81\x15\x80a3:W[\x15a2\xF6WPP`@Qa2\xEA`@\x82a\x02\x18V[_\x81R_` \x82\x01R\x90V[` _Q` a7\x80_9_Q\x90_R\x91\x01Q\x06_Q` a7\x80_9_Q\x90_R\x03_Q` a7\x80_9_Q\x90_R\x81\x11a\x17(W`@Q\x91a-\xB6\x83a\x01\xDDV[P` \x81\x01Q\x15a2\xD5V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x03a3\xA2WV[c\x8Ey\xFD\xB5`\xE0\x1B_R`\x04_\xFD[`@Qc#\xB8r\xDD`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x82\x01R\x92\x90\x91\x16`D\x83\x01R`d\x80\x83\x01\x93\x90\x93R\x91\x81Ra\x02I\x91a3\xF5`\x84\x83a\x02\x18V[a6\x1CV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R` \x81\x80`D\x81\x01\x03\x81`\x01`\x01`\xA0\x1B\x03\x86\x16Z\xFA\x90\x81\x15a\x04\x90Wa\x02I\x94a3\xF5\x92a4Q\x92_\x91a4\x85WPa\x17eV[`@Qc\t^\xA7\xB3`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`$\x85\x01R`D\x80\x85\x01\x91\x90\x91R\x83R`d\x83a\x02\x18V[a4\x9E\x91P` =` \x11a\x19jWa\x19[\x81\x83a\x02\x18V[_a\x193V[\x90`\xFF_T`\x08\x1C\x16\x15a4\xBEWa\x06\x9Ea\x02I\x92a3FV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[\x15a5\x1EWV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11a5\x96W\x81Q\x15a5\x91W` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15a5\x8CW`\x01\x90a5wa5ma\x18\xF1a\x18\xE3\x86\x89a\x16\xF5V[`\xFF`\x01\x91\x16\x1B\x90V[\x90a5\x83\x81\x83\x11a5\x17V[\x17\x91\x01\x90a5NV[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\x91WQ\x80\x15\x15\x81\x03a\x02\x91W\x90V[\x15a5\xC4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[a\x02I\x91a6\x9E\x91`\x01\x80`\xA0\x1B\x03\x16_\x80`@Q\x93a6=`@\x86a\x02\x18V[` \x85R\x7FSafeERC20: low-level call failed` \x86\x01R` \x81Q\x91\x01\x82\x85Z\xF1=\x15a6\xCCW=\x91a6\x82\x83a\x0B\xE3V[\x92a6\x90`@Q\x94\x85a\x02\x18V[\x83R=_` \x85\x01>a6\xEAV[\x80Q\x90\x81\x15\x91\x82\x15a6\xB2W[PPa5\xBDV[a6\xC5\x92P` \x80\x91\x83\x01\x01\x91\x01a5\xA5V[_\x80a6\xABV[``\x91a6\xEAV[\x15a6\xDBWV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD[\x91\x92\x90\x15a7LWP\x81Q\x15a6\xFEWP\x90V[;\x15a7\x07W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[\x82Q\x90\x91P\x15a7_WP\x80Q\x90` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x90a7{\x90`\x04\x83\x01a'\xDBV[\x03\x90\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 6\xD9;\xE6%\x03\xCE.Zj\xAB#~\x87\t\x84\xBC\xFF>`\x938x\xBF\xF1\xCF\x0C\xC8\xD2\xF6\x86\xE7dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8063171f1d5b146101c45780631785f53c146101bf5780631fdb0cfd146101ba578063279432eb146101b557806333cfb7b7146101b05780633bc28c8c146101ab5780635df45946146101a657806368304835146101a15780636b3aa72e1461019c5780636d14a987146101975780636efb463614610192578063715018a61461018d5780638da5cb5b146101885780639926ee7d146101835780639da16d8e1461017e578063a0169ddd14610179578063a20b99bf14610174578063a364f4da1461016f578063a98fb3551461016a578063ba55088014610165578063c1a8e2c514610160578063c4d66de81461015b578063df5cf72314610156578063e481af9d14610151578063f2fde38b1461014c578063fc299dee146101475763fce36c7d14610142575f80fd5b61145e565b611436565b6113a5565b61138a565b611346565b611251565b611166565b6110da565b61103a565b610f7d565b610e3d565b610d5a565b610cd0565b610c34565b610bbb565b610b60565b610acb565b610779565b610735565b6106f1565b6106ad565b610676565b61063e565b610572565b6104d2565b6103ea565b610371565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176101f857604052565b6101c9565b606081019081106001600160401b038211176101f857604052565b90601f801991011681019081106001600160401b038211176101f857604052565b6040519061024961010083610218565b565b60405190610249604083610218565b906102496040519283610218565b60409060e31901126102915760405190610281826101dd565b60e4358252610104356020830152565b5f80fd5b9190826040910312610291576040516102ad816101dd565b6020808294803584520135910152565b9080601f8301121561029157604051916102d8604084610218565b82906040810192831161029157905b8282106102f45750505090565b81358152602091820191016102e7565b9060806063198301126102915760405161031d816101dd565b6020610338829461032f8160646102bd565b845260a46102bd565b910152565b91906080838203126102915760206103386040519261035b846101dd565b6040849661036983826102bd565b8652016102bd565b3461029157610120366003190112610291576004356040366023190112610291576103c960409182516103a3816101dd565b602435815260443560208201526103b936610304565b906103c336610268565b92611570565b8251911515825215156020820152f35b6001600160a01b0381160361029157565b34610291575f602036600319011261029157600435610408816103d9565b610410613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102915760405163268959e560e01b81523060048201526001600160a01b0390921660248301525f908290818381604481015b03925af1801561049057610482575080f35b61048e91505f90610218565b005b611656565b6060906003190112610291576004356104ad816103d9565b906024356104ba816103d9565b906044356001600160e01b0319811681036102915790565b34610291576104e036610495565b6104e8613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692833b1561029157604051634a86c03760e11b81523060048201526001600160a01b039182166024820152921660448301526001600160e01b03191660648201525f81608481015b93818381819703925af1801561049057610482575080f35b34610291575f602036600319011261029157600435610590816103d9565b610598613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102915760405163eb5a4e8760e01b81523060048201526001600160a01b0390921660248301525f90829081838160448101610470565b60206040818301928281528451809452019201905f5b81811061061f5750505090565b82516001600160a01b0316845260209384019390920191600101610612565b3461029157602036600319011261029157610672610666600435610661816103d9565b6117df565b604051918291826105fc565b0390f35b346102915760203660031901126102915761048e600435610696816103d9565b61069e613054565b61314a565b5f91031261029157565b34610291575f366003190112610291576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610291575f366003190112610291576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610291575f366003190112610291576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610291575f366003190112610291576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b6044359063ffffffff8216820361029157565b359063ffffffff8216820361029157565b6001600160401b0381116101f85760051b60200190565b9080601f8301121561029157813561080f816107e1565b9261081d6040519485610218565b81845260208085019260051b82010192831161029157602001905b8282106108455750505090565b60208091610852846107d0565b815201910190610838565b81601f82011215610291578035610873816107e1565b926108816040519485610218565b81845260208085019260061b8401019281841161029157602001915b8383106108ab575050505090565b60206040916108ba8486610295565b81520192019161089d565b9080601f830112156102915781356108dc816107e1565b926108ea6040519485610218565b81845260208085019260051b820101918383116102915760208201905b83821061091657505050505090565b81356001600160401b03811161029157602091610938878480948801016107f8565b815201910190610907565b9190916101808184031261029157610959610239565b9281356001600160401b03811161029157816109769184016107f8565b845260208201356001600160401b038111610291578161099791840161085d565b602085015260408201356001600160401b03811161029157816109bb91840161085d565b60408501526109cd816060840161033d565b60608501526109df8160e08401610295565b60808501526101208201356001600160401b0381116102915781610a049184016107f8565b60a08501526101408201356001600160401b0381116102915781610a299184016107f8565b60c08501526101608201356001600160401b03811161029157610a4c92016108c5565b60e0830152565b90602080835192838152019201905f5b818110610a705750505090565b82516001600160601b0316845260209384019390920191600101610a63565b929190610ac66020916040865282610ab282516040808a01526080890190610a53565b910151868203603f19016060880152610a53565b930152565b34610291576080366003190112610291576004356024356001600160401b03811161029157366023820112156102915780600401356001600160401b03811161029157366024828401011161029157610b226107bd565b90606435936001600160401b038511610291576024610b48610b50963690600401610943565b940190611cf3565b9061067260405192839283610a8f565b34610291575f36600319011261029157610b78613054565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610291575f366003190112610291576033546040516001600160a01b039091168152602090f35b6001600160401b0381116101f857601f01601f191660200190565b929192610c0a82610be3565b91610c186040519384610218565b829481845281830111610291578281602093845f960137010152565b3461029157604036600319011261029157600435610c51816103d9565b602435906001600160401b03821161029157606060031983360301126102915760405190610c7e826101fd565b82600401356001600160401b038111610291578301366023820112156102915761048e93610cb86044923690602460048201359101610bfe565b8452602481013560208501520135604083015261241a565b34610291575f602036600319011261029157600435610cee816103d9565b610cf6613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561029157604051634f906cf960e01b81523060048201526001600160a01b0390921660248301525f90829081838160448101610470565b34610291575f602036600319011261029157600435610d78816103d9565b610d80613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156102915760405163a0169ddd60e01b81526001600160a01b039091166004820152905f908290602490829084905af1801561049057610482575080f35b906020600319830112610291576004356001600160401b0381116102915760040182601f82011215610291578035926001600160401b038411610291576020808301928560051b010111610291579190565b3461029157610e4b36610deb565b90610e5461338e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b818110610ed85750823b1561029157610eb4925f9283604051809681958294634e5cd2fd60e11b8452306004850161266b565b03925af1801561049057610ec457005b80610ed25f61048e93610218565b806106a3565b915f93915f915b610ef7610eed868484612518565b604081019061253a565b9050831015610f33576001610f2981976020610f2187610f1b610eed8c8a8a612518565b9061256f565b013590611765565b9301929550610edf565b9390929460019250610f7790610f61813088610f5c6020610f56898c3395612518565b0161257f565b6133b1565b86610f726020610f5686898b612518565b6133fa565b01610e81565b34610291575f602036600319011261029157600435610f9b816103d9565b610fcf337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612404565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610291576040516351b27a6d60e11b81526001600160a01b039091166004820152905f908290602490829084905af1801561049057610482575080f35b34610291575f6020366003190112610291576004356001600160401b03811161029157366023820112156102915761107c903690602481600401359101610bfe565b611084613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156102915760405163a98fb35560e01b8152915f91839182908490829061047090600483016127db565b34610291576110e836610495565b6110f0613054565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692833b1561029157604051630664120160e01b81523060048201526001600160a01b039182166024820152921660448301526001600160e01b03191660648201525f816084810161055a565b3461029157604036600319011261029157600435611183816103d9565b6024356001600160401b038111610291576111a29036906004016107f8565b6111d6337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612404565b6111f3604051926111e6846101fd565b6001600160a01b03168352565b30602083015260408201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561029157604051636e3492b560e01b8152905f908290818381610eb488600483016127ec565b346102915760203660031901126102915760043561126e816103d9565b6112be5f54916112a261128c6112888560ff9060081c1690565b1590565b80948195611338575b8115611318575b50612856565b826112b3600160ff195f5416175f55565b611301575b806134a4565b6112c457005b6112d261ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b61131361010061ff00195f5416175f55565b6112b8565b303b1591508161132a575b505f61129c565b60ff1660011490505f611323565b600160ff8216109150611295565b34610291575f366003190112610291576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610291575f366003190112610291576106726106666128b9565b34610291576020366003190112610291576004356113c2816103d9565b6113ca613054565b6001600160a01b038116156113e25761048e90613346565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b34610291575f366003190112610291576065546040516001600160a01b039091168152602090f35b346102915761146c36610deb565b9061147561338e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f5b8181106114d45750823b1561029157610eb4925f928360405180968195829463fce36c7d60e01b845260048401612b08565b806115036114ea6020610f566001958789612ae6565b60406114f7848789612ae6565b013590309033906133b1565b61152c6115166020610f56848789612ae6565b86604061152485888a612ae6565b0135916133fa565b016114a2565b634e487b7160e01b5f52603260045260245ffd5b9060028110156115575760051b0190565b611532565b634e487b7160e01b5f52601260045260245ffd5b61164c6116296116529561162361161c85875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e08401526101008301526115f381610120840103601f198101835282610218565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b8096612c21565b90612c67565b9261162361163e611638612cc9565b94612dc0565b91611647612edc565b612c21565b91612f26565b9091565b6040513d5f823e3d90fd5b90816020910312610291575190565b9081602091031261029157516001600160c01b03811681036102915790565b90816020910312610291575160ff811681036102915790565b604051906116b7602083610218565b5f808352366020840137565b906116cd826107e1565b6116da6040519182610218565b82815280926116eb601f19916107e1565b0190602036910137565b908151811015611557570160200190565b634e487b7160e01b5f52601160045260245ffd5b906001820180921161172857565b611706565b906002820180921161172857565b906003820180921161172857565b906004820180921161172857565b906005820180921161172857565b9190820180921161172857565b6001600160601b0381160361029157565b908160409103126102915760206040519161179d836101dd565b80516117a8816103d9565b835201516117b581611772565b602082015290565b80518210156115575760209160051b010190565b5f1981146117285760010190565b6040516309aa152760e11b81526001600160a01b0391821660048201527f000000000000000000000000000000000000000000000000000000000000000090911690602081602481855afa9081156104905761185f916020915f91611b5e575b506040518093819263871ef04960e01b8352600483019190602083019252565b0381855afa908115610490575f91611b2f575b506001600160c01b0316908115908115611acc575b50611ac057611895906130ac565b5f91907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b81518510156119715761191a60206118f76118f16118e389876116f5565b516001600160f81b03191690565b60f81c90565b604051633ca5a5f560e01b815260ff909116600482015291829081906024820190565b0381875afa801561049057600192611939925f92611941575b50611765565b9401936118c5565b61196391925060203d811161196a575b61195b8183610218565b810190611661565b905f611933565b503d611951565b61197c9194506116c3565b925f905f5b8151811015611aba5761199a6118f16118e383856116f5565b604051633ca5a5f560e01b815260ff8216600482015290602082602481895afa918215610490575f92611a9a575b50905f915b8183106119df57505050600101611981565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa91821561049057611a5e8b611a4f83611a49611a3d600198611a63985f91611a6c575b50516001600160a01b031690565b6001600160a01b031690565b926117bd565b6001600160a01b039091169052565b6117d1565b950191906119cd565b611a8d915060403d8111611a93575b611a858183610218565b810190611783565b5f611a2f565b503d611a7b565b611ab391925060203d811161196a5761195b8183610218565b905f6119c8565b50505050565b50611ac96116a8565b90565b604051639aa1653d60e01b81529150602090829060049082905afa80156104905760ff915f91611b00575b5016155f611887565b611b22915060203d602011611b28575b611b1a8183610218565b81019061168f565b5f611af7565b503d611b10565b611b51915060203d602011611b57575b611b498183610218565b810190611670565b5f611872565b503d611b3f565b611b759150823d841161196a5761195b8183610218565b5f61183f565b60405190611b88826101dd565b60606020838281520152565b15611b9b57565b62f8202d60e51b5f5260045ffd5b15611bb057565b6343714afd60e01b5f5260045ffd5b15611bc657565b635f832f4160e01b5f5260045ffd5b15611bdc57565b634b874f4560e01b5f5260045ffd5b5f1981019190821161172857565b15611c0057565b633fdc650560e21b5f5260045ffd5b90821015611557570190565b90816020910312610291575167ffffffffffffffff19811681036102915790565b15611c4357565b63e1310aed60e01b5f5260045ffd5b908160209103126102915751611ac981611772565b906001600160601b03809116911603906001600160601b03821161172857565b15611c8e57565b6367988d3360e01b5f5260045ffd5b15611ca457565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b818110611cdd5750505090565b8251845260209384019390920191600101611cd0565b949392909193611d01611b7b565b50611d0d851515611b94565b6040840151518514806123f6575b806123e8575b806123da575b611d3090611ba9565b611d4260208501515185515114611bbf565b611d5963ffffffff431663ffffffff841610611bd5565b611d6161024b565b5f81525f602082015292611d73611b7b565b611d7c876116c3565b6020820152611d8a876116c3565b8152611d94611b7b565b92611da36020880151516116c3565b8452611db36020880151516116c3565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561049057611e1c915f916123bb575b50611e17368b87610bfe565b6131a8565b985f965b60208901518051891015611f9857602088611e8d611e838c611e7b8f96868e611e60611e4d8680956117bd565b5180515f526020015160205260405f2090565b611e6d84848401516117bd565b5282611f65575b01516117bd565b5195516117bd565b5163ffffffff1690565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa918215610490576116238a611f3a8f611f338f8460208f92611f2a93611f228460019e611f409e5f91611f48575b508f8060c01b031692516117bd565b5201516117bd565b51938d516117bd565b51166131d3565b90613204565b970196611e20565b611f5f9150863d8111611b5757611b498183610218565b5f611f13565b611f93611f7584848401516117bd565b51611f8c84840151611f8687611beb565b906117bd565b5110611bf9565b611e74565b50909597949650611fad9198939299506132c1565b915f905b8082106120135750505092611fec611fe7611fe061200d9585611fff9860806060602099015192015192611570565b9190611c87565b611c9d565b0151604051928391602083019586611cb3565b03601f198101835282610218565b51902090565b918495969361205e6118f1612050858761204a879f989e9b611e4d84604061203c9301516117bd565b67ffffffffffffffff191690565b96611c0f565b356001600160f81b03191690565b602087612072611e838d60a08d01516117bd565b604051631a2f32ab60e21b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9283156104905761210e936120f9925f9161238d575b5067ffffffffffffffff19918216911614611c3c565b6121078960408901516117bd565b5190612c67565b906121206118f16120508a868d611c0f565b602086612134611e838c60c08c01516117bd565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa8015610490576121ca8a60208e611e746121d7956121e2975f91612370575b506121bc85858501516117bd565b906001600160601b03169052565b516001600160601b031690565b6121bc8a8d516117bd565b5f965f5b60208801515181101561235c57898b612222612203848a516117bd565b516122156118f1612050868c87611c0f565b60ff161c60019081161490565b612231575b50506001016121e6565b88886122b8612278611e83879f8f978060e08f849c60206122616118f1612050839f98612269966122729a611c0f565b9a01516117bd565b519a01516117bd565b516117bd565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830195909552919093166064840152829081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156104905761231b8e6001948e86955f92612326575b50611a496121bc929351936123166121ca84876117bd565b611c67565b01989050898b612227565b6121bc925061234e611a499160203d8111612355575b6123468183610218565b810190611c52565b92506122fe565b503d61233c565b509594976001919993929497500190611fb1565b6123879150843d8111612355576123468183610218565b5f6121ae565b6123ae915060203d81116123b4575b6123a68183610218565b810190611c1b565b5f6120e3565b503d61239c565b6123d4915060203d602011611b2857611b1a8183610218565b5f611e0b565b5060e0840151518514611d27565b5060c0840151518514611d21565b5060a0840151518514611d1b565b1561240b57565b634394dbdf60e11b5f5260045ffd5b61244e337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614612404565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b15610291575f928392604051948580948193639926ee7d60e01b835260018060a01b031660048301526040602483015260406124c382516060604486015260a48501906124f4565b91602081015160648501520151608483015203925af18015610490576124e65750565b80610ed25f61024993610218565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91908110156115575760051b8101359060be1981360301821215610291570190565b903590601e198136030182121561029157018035906001600160401b03821161029157602001918160061b3603831361029157565b91908110156115575760061b0190565b35611ac9816103d9565b9035601e19823603018112156102915701602081359101916001600160401b038211610291578160061b3603831361029157565b916020908281520191905f5b8181106125d65750505090565b90919260408060019286356125ea816103d9565b848060a01b031681526001600160601b03602088013561260981611772565b1660208201520194019291016125c9565b9035601e19823603018112156102915701602081359101916001600160401b03821161029157813603831361029157565b908060209392818452848401375f828201840152601f01601f1916010190565b6001600160a01b0390911681526040602082018190528101839052600583901b810160609081019383923684900360be1901925f91908101905b8383106126b6575050505050505090565b90919293949596605f19828203018352873586811215610291578701906126ee6126e08380612589565b60c0845260c08401916125bd565b9160208101356126fd816103d9565b6001600160a01b031660208381019190915261271c6040830183612589565b848603604086015280865294909101935f5b8181106127a7575050506127966001936020936127888461276261275560608998016107d0565b63ffffffff166060850152565b61277e612771608083016107d0565b63ffffffff166080850152565b60a081019061261a565b9160a081850391015261264b565b9901930193019195949392906126a5565b90919460408060019288356127bb816103d9565b848060a01b0316815260208901356020820152019601910191909161272e565b906020611ac99281815201906124f4565b602080825282516001600160a01b039081168284015281840151166040808401919091529092015160608083015280516080830181905260a09092019201905f5b81811061283a5750505090565b825163ffffffff1684526020938401939092019160010161282d565b1561285d57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156104905760ff915f91612ac7575b50168015612abd577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b808310612a79575061295491506116c3565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156104905760ff915f91612a5b575b5016811015612a5457604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa918215610490575f92612a34575b50905f915b8183106129ce57505050600101612959565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa91821561049057611a5e8b611a4f83611a49611a3d600198612a2b985f91611a6c5750516001600160a01b031690565b950191906129bc565b612a4d91925060203d811161196a5761195b8183610218565b905f6129b7565b5092505050565b612a73915060203d8111611b2857611b1a8183610218565b5f61297f565b604051633ca5a5f560e01b815260ff84166004820152909190602081602481885afa801561049057600192612ab4925f926119415750611765565b92019190612942565b5050611ac96116a8565b612ae0915060203d602011611b2857611b1a8183610218565b5f61290a565b91908110156115575760051b81013590609e1981360301821215610291570190565b909180602083016020845252604082019260408260051b8401019381935f91609e1984360301915b858410612b41575050505050505090565b90919293949596603f19828203018352873590848212156102915760208091886001940190608063ffffffff612bc882612b8c612b7e8780612589565b60a0885260a08801916125bd565b9587810135612b9a816103d9565b8a8060a01b0316888701526040810135604087015283612bbc606083016107d0565b166060870152016107d0565b16910152990193019401929195949390612b30565b60405190612bea826101dd565b5f6020838281520152565b60405190610180612c068184610218565b368337565b60405190612c1a602083610218565b6020368337565b91906040906060612c30612bdd565b9485926020855192612c428585610218565b8436853780518452015160208301528482015260076107cf195a01fa15612c6557565bfe5b602092916080604092612c78612bdd565b95869381865193612c898686610218565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa8015612c655715612cba57565b63d4b68fd760e01b5f5260045ffd5b604051612cd5816101dd565b6040908151612ce48382610218565b8236823781526020825191612cf98484610218565b8336843701528051612d0b8282610218565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6020820152815190612d618383610218565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152612db683519384610218565b8252602082015290565b5f5160206137805f395f51905f5290612dd7612bdd565b505f919006602060c0835b612ed7575f935f5160206137805f395f51905f5260038186818180090908604051612e0d8582610218565b84368237848185604051612e218282610218565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f5160206137805f395f51905f5260a082015260056107cf195a01fa8015612c6557612e8b906136d4565b5191612ed7575f5160206137805f395f51905f5282800914612ec257505f5160206137805f395f51905f5260015f94089293612de2565b92935050612ece61024b565b92835282015290565b61155c565b612ee4612bdd565b50604051612ef1816101dd565b600181526002602082015290565b9060068202918083046006149015171561172857565b90600c8110156115575760051b0190565b93929091612f34604061025a565b9485526020850152612f46604061025a565b9182526020820152612f56612bf5565b925f5b60028110612f8357505050602061018092612f72612c0b565b93849160086201d4c0fa9151151590565b80612f8f600192612eff565b612f998285611546565b5151612fa58289612f15565b526020612fb28386611546565b510151612fc7612fc18361171a565b89612f15565b52612fd28286611546565b515151612fe1612fc18361172d565b52612ff7612fef8387611546565b515160200190565b51613004612fc18361173b565b5260206130118387611546565b51015151613021612fc183611749565b5261304d6130476130406020613037868a611546565b51015160200190565b5192611757565b88612f15565b5201612f59565b6033546001600160a01b0316330361306857565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b61ffff6130b8826131d3565b166130c281610be3565b906130d06040519283610218565b8082526130df601f1991610be3565b013660208301375f5f5b825182108061313f575b15613138576001811b8416613111575b61310c906117d1565b6130e9565b90600161310c9160ff60f81b8460f81b165f1a61312e82876116f5565b5301919050613103565b5050905090565b5061010081106130f3565b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b9060016131b660ff9361352d565b928392161b11156131c45790565b63ca95733360e01b5f5260045ffd5b805f915b6131df575090565b5f1981018181116117285761ffff9116911661ffff81146117285760010190806131d7565b9061320d612bdd565b5061ffff8116906102008210156132b257600182146132ad5761322e61024b565b5f81525f602082015292906001905f925b61ffff831685101561325357505050505090565b600161ffff831660ff86161c81161461328d575b60016132836132788360ff94612c67565b9460011b61fffe1690565b940116929161323f565b9460016132836132786132a28960ff95612c67565b989350505050613267565b505090565b637fc4ea7d60e11b5f5260045ffd5b6132c9612bdd565b5080519081158061333a575b156132f65750506040516132ea604082610218565b5f81525f602082015290565b60205f5160206137805f395f51905f52910151065f5160206137805f395f51905f52035f5160206137805f395f51905f5281116117285760405191612db6836101dd565b506020810151156132d5565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6065546001600160a01b031633036133a257565b638e79fdb560e01b5f5260045ffd5b6040516323b872dd60e01b60208201526001600160a01b039283166024820152929091166044830152606480830193909352918152610249916133f5608483610218565b61361c565b604051636eb1769f60e11b81523060048201526001600160a01b0383166024820152602081806044810103816001600160a01b0386165afa90811561049057610249946133f592613451925f916134855750611765565b60405163095ea7b360e01b60208201526001600160a01b039490941660248501526044808501919091528352606483610218565b61349e915060203d60201161196a5761195b8183610218565b5f611933565b9060ff5f5460081c16156134be5761069e61024992613346565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b1561351e57565b631019106960e31b5f5260045ffd5b906101008251116135965781511561359157602082015160019060f81c81901b5b835182101561358c5760019061357761356d6118f16118e386896116f5565b60ff600191161b90565b90613583818311613517565b1791019061354e565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b90816020910312610291575180151581036102915790565b156135c457565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b6102499161369e9160018060a01b03165f806040519361363d604086610218565b602085527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564602086015260208151910182855af13d156136cc573d9161368283610be3565b926136906040519485610218565b83523d5f602085013e6136ea565b80519081159182156136b2575b50506135bd565b6136c592506020809183010191016135a5565b5f806136ab565b6060916136ea565b156136db57565b63d51edae360e01b5f5260045ffd5b9192901561374c57508151156136fe575090565b3b156137075790565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b82519091501561375f5750805190602001fd5b60405162461bcd60e51b815290819061377b90600483016127db565b0390fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a264697066735822122036d93be62503ce2e5a6aab237e870984bcff3e60933878bff1cf0cc8d2f686e764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x17\x1F\x1D[\x14a\x01\xC4W\x80c\x17\x85\xF5<\x14a\x01\xBFW\x80c\x1F\xDB\x0C\xFD\x14a\x01\xBAW\x80c'\x942\xEB\x14a\x01\xB5W\x80c3\xCF\xB7\xB7\x14a\x01\xB0W\x80c;\xC2\x8C\x8C\x14a\x01\xABW\x80c]\xF4YF\x14a\x01\xA6W\x80ch0H5\x14a\x01\xA1W\x80ck:\xA7.\x14a\x01\x9CW\x80cm\x14\xA9\x87\x14a\x01\x97W\x80cn\xFBF6\x14a\x01\x92W\x80cqP\x18\xA6\x14a\x01\x8DW\x80c\x8D\xA5\xCB[\x14a\x01\x88W\x80c\x99&\xEE}\x14a\x01\x83W\x80c\x9D\xA1m\x8E\x14a\x01~W\x80c\xA0\x16\x9D\xDD\x14a\x01yW\x80c\xA2\x0B\x99\xBF\x14a\x01tW\x80c\xA3d\xF4\xDA\x14a\x01oW\x80c\xA9\x8F\xB3U\x14a\x01jW\x80c\xBAU\x08\x80\x14a\x01eW\x80c\xC1\xA8\xE2\xC5\x14a\x01`W\x80c\xC4\xD6m\xE8\x14a\x01[W\x80c\xDF\\\xF7#\x14a\x01VW\x80c\xE4\x81\xAF\x9D\x14a\x01QW\x80c\xF2\xFD\xE3\x8B\x14a\x01LW\x80c\xFC)\x9D\xEE\x14a\x01GWc\xFC\xE3l}\x14a\x01BW_\x80\xFD[a\x14^V[a\x146V[a\x13\xA5V[a\x13\x8AV[a\x13FV[a\x12QV[a\x11fV[a\x10\xDAV[a\x10:V[a\x0F}V[a\x0E=V[a\rZV[a\x0C\xD0V[a\x0C4V[a\x0B\xBBV[a\x0B`V[a\n\xCBV[a\x07yV[a\x075V[a\x06\xF1V[a\x06\xADV[a\x06vV[a\x06>V[a\x05rV[a\x04\xD2V[a\x03\xEAV[a\x03qV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xF8W`@RV[a\x01\xC9V[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xF8W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\xF8W`@RV[`@Q\x90a\x02Ia\x01\0\x83a\x02\x18V[V[`@Q\x90a\x02I`@\x83a\x02\x18V[\x90a\x02I`@Q\x92\x83a\x02\x18V[`@\x90`\xE3\x19\x01\x12a\x02\x91W`@Q\x90a\x02\x81\x82a\x01\xDDV[`\xE45\x82Ra\x01\x045` \x83\x01RV[_\x80\xFD[\x91\x90\x82`@\x91\x03\x12a\x02\x91W`@Qa\x02\xAD\x81a\x01\xDDV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x91W`@Q\x91a\x02\xD8`@\x84a\x02\x18V[\x82\x90`@\x81\x01\x92\x83\x11a\x02\x91W\x90[\x82\x82\x10a\x02\xF4WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x02\xE7V[\x90`\x80`c\x19\x83\x01\x12a\x02\x91W`@Qa\x03\x1D\x81a\x01\xDDV[` a\x038\x82\x94a\x03/\x81`da\x02\xBDV[\x84R`\xA4a\x02\xBDV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x02\x91W` a\x038`@Q\x92a\x03[\x84a\x01\xDDV[`@\x84\x96a\x03i\x83\x82a\x02\xBDV[\x86R\x01a\x02\xBDV[4a\x02\x91Wa\x01 6`\x03\x19\x01\x12a\x02\x91W`\x045`@6`#\x19\x01\x12a\x02\x91Wa\x03\xC9`@\x91\x82Qa\x03\xA3\x81a\x01\xDDV[`$5\x81R`D5` \x82\x01Ra\x03\xB96a\x03\x04V[\x90a\x03\xC36a\x02hV[\x92a\x15pV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\x91WV[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x04\x08\x81a\x03\xD9V[a\x04\x10a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x91W`@Qc&\x89Y\xE5`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x04\x90Wa\x04\x82WP\x80\xF3[a\x04\x8E\x91P_\x90a\x02\x18V[\0[a\x16VV[``\x90`\x03\x19\x01\x12a\x02\x91W`\x045a\x04\xAD\x81a\x03\xD9V[\x90`$5a\x04\xBA\x81a\x03\xD9V[\x90`D5`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x02\x91W\x90V[4a\x02\x91Wa\x04\xE06a\x04\x95V[a\x04\xE8a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x83;\x15a\x02\x91W`@QcJ\x86\xC07`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`$\x82\x01R\x92\x16`D\x83\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x82\x01R_\x81`\x84\x81\x01[\x93\x81\x83\x81\x81\x97\x03\x92Z\xF1\x80\x15a\x04\x90Wa\x04\x82WP\x80\xF3[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x05\x90\x81a\x03\xD9V[a\x05\x98a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x91W`@Qc\xEBZN\x87`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01a\x04pV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x06\x1FWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\x12V[4a\x02\x91W` 6`\x03\x19\x01\x12a\x02\x91Wa\x06ra\x06f`\x045a\x06a\x81a\x03\xD9V[a\x17\xDFV[`@Q\x91\x82\x91\x82a\x05\xFCV[\x03\x90\xF3[4a\x02\x91W` 6`\x03\x19\x01\x12a\x02\x91Wa\x04\x8E`\x045a\x06\x96\x81a\x03\xD9V[a\x06\x9Ea0TV[a1JV[_\x91\x03\x12a\x02\x91WV[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`D5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\x91WV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\x91WV[`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF8W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x91W\x815a\x08\x0F\x81a\x07\xE1V[\x92a\x08\x1D`@Q\x94\x85a\x02\x18V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\x91W` \x01\x90[\x82\x82\x10a\x08EWPPP\x90V[` \x80\x91a\x08R\x84a\x07\xD0V[\x81R\x01\x91\x01\x90a\x088V[\x81`\x1F\x82\x01\x12\x15a\x02\x91W\x805a\x08s\x81a\x07\xE1V[\x92a\x08\x81`@Q\x94\x85a\x02\x18V[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x02\x91W` \x01\x91[\x83\x83\x10a\x08\xABWPPPP\x90V[` `@\x91a\x08\xBA\x84\x86a\x02\x95V[\x81R\x01\x92\x01\x91a\x08\x9DV[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x91W\x815a\x08\xDC\x81a\x07\xE1V[\x92a\x08\xEA`@Q\x94\x85a\x02\x18V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x02\x91W` \x82\x01\x90[\x83\x82\x10a\t\x16WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W` \x91a\t8\x87\x84\x80\x94\x88\x01\x01a\x07\xF8V[\x81R\x01\x91\x01\x90a\t\x07V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x02\x91Wa\tYa\x029V[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x81a\tv\x91\x84\x01a\x07\xF8V[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x81a\t\x97\x91\x84\x01a\x08]V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x81a\t\xBB\x91\x84\x01a\x08]V[`@\x85\x01Ra\t\xCD\x81``\x84\x01a\x03=V[``\x85\x01Ra\t\xDF\x81`\xE0\x84\x01a\x02\x95V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x81a\n\x04\x91\x84\x01a\x07\xF8V[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x81a\n)\x91\x84\x01a\x07\xF8V[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91Wa\nL\x92\x01a\x08\xC5V[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\npWPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\ncV[\x92\x91\x90a\n\xC6` \x91`@\x86R\x82a\n\xB2\x82Q`@\x80\x8A\x01R`\x80\x89\x01\x90a\nSV[\x91\x01Q\x86\x82\x03`?\x19\x01``\x88\x01Ra\nSV[\x93\x01RV[4a\x02\x91W`\x806`\x03\x19\x01\x12a\x02\x91W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W6`#\x82\x01\x12\x15a\x02\x91W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W6`$\x82\x84\x01\x01\x11a\x02\x91Wa\x0B\"a\x07\xBDV[\x90`d5\x93`\x01`\x01`@\x1B\x03\x85\x11a\x02\x91W`$a\x0BHa\x0BP\x966\x90`\x04\x01a\tCV[\x94\x01\x90a\x1C\xF3V[\x90a\x06r`@Q\x92\x83\x92\x83a\n\x8FV[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91Wa\x0Bxa0TV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x01\xF8W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x0C\n\x82a\x0B\xE3V[\x91a\x0C\x18`@Q\x93\x84a\x02\x18V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02\x91W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[4a\x02\x91W`@6`\x03\x19\x01\x12a\x02\x91W`\x045a\x0CQ\x81a\x03\xD9V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x91W```\x03\x19\x836\x03\x01\x12a\x02\x91W`@Q\x90a\x0C~\x82a\x01\xFDV[\x82`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W\x83\x016`#\x82\x01\x12\x15a\x02\x91Wa\x04\x8E\x93a\x0C\xB8`D\x926\x90`$`\x04\x82\x015\x91\x01a\x0B\xFEV[\x84R`$\x81\x015` \x85\x01R\x015`@\x83\x01Ra$\x1AV[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x0C\xEE\x81a\x03\xD9V[a\x0C\xF6a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x91W`@QcO\x90l\xF9`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R_\x90\x82\x90\x81\x83\x81`D\x81\x01a\x04pV[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\rx\x81a\x03\xD9V[a\r\x80a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\x91W`@Qc\xA0\x16\x9D\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04\x90Wa\x04\x82WP\x80\xF3[\x90` `\x03\x19\x83\x01\x12a\x02\x91W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W`\x04\x01\x82`\x1F\x82\x01\x12\x15a\x02\x91W\x805\x92`\x01`\x01`@\x1B\x03\x84\x11a\x02\x91W` \x80\x83\x01\x92\x85`\x05\x1B\x01\x01\x11a\x02\x91W\x91\x90V[4a\x02\x91Wa\x0EK6a\r\xEBV[\x90a\x0ETa3\x8EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a\x0E\xD8WP\x82;\x15a\x02\x91Wa\x0E\xB4\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94cN\\\xD2\xFD`\xE1\x1B\x84R0`\x04\x85\x01a&kV[\x03\x92Z\xF1\x80\x15a\x04\x90Wa\x0E\xC4W\0[\x80a\x0E\xD2_a\x04\x8E\x93a\x02\x18V[\x80a\x06\xA3V[\x91_\x93\x91_\x91[a\x0E\xF7a\x0E\xED\x86\x84\x84a%\x18V[`@\x81\x01\x90a%:V[\x90P\x83\x10\x15a\x0F3W`\x01a\x0F)\x81\x97` a\x0F!\x87a\x0F\x1Ba\x0E\xED\x8C\x8A\x8Aa%\x18V[\x90a%oV[\x015\x90a\x17eV[\x93\x01\x92\x95Pa\x0E\xDFV[\x93\x90\x92\x94`\x01\x92Pa\x0Fw\x90a\x0Fa\x810\x88a\x0F\\` a\x0FV\x89\x8C3\x95a%\x18V[\x01a%\x7FV[a3\xB1V[\x86a\x0Fr` a\x0FV\x86\x89\x8Ba%\x18V[a3\xFAV[\x01a\x0E\x81V[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x0F\x9B\x81a\x03\xD9V[a\x0F\xCF3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a$\x04V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\x91W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04\x90Wa\x04\x82WP\x80\xF3[4a\x02\x91W_` 6`\x03\x19\x01\x12a\x02\x91W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91W6`#\x82\x01\x12\x15a\x02\x91Wa\x10|\x906\x90`$\x81`\x04\x015\x91\x01a\x0B\xFEV[a\x10\x84a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x91W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x04p\x90`\x04\x83\x01a'\xDBV[4a\x02\x91Wa\x10\xE86a\x04\x95V[a\x10\xF0a0TV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x83;\x15a\x02\x91W`@Qc\x06d\x12\x01`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`$\x82\x01R\x92\x16`D\x83\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x82\x01R_\x81`\x84\x81\x01a\x05ZV[4a\x02\x91W`@6`\x03\x19\x01\x12a\x02\x91W`\x045a\x11\x83\x81a\x03\xD9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x91Wa\x11\xA2\x906\x90`\x04\x01a\x07\xF8V[a\x11\xD63\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a$\x04V[a\x11\xF3`@Q\x92a\x11\xE6\x84a\x01\xFDV[`\x01`\x01`\xA0\x1B\x03\x16\x83RV[0` \x83\x01R`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\x91W`@Qcn4\x92\xB5`\xE0\x1B\x81R\x90_\x90\x82\x90\x81\x83\x81a\x0E\xB4\x88`\x04\x83\x01a'\xECV[4a\x02\x91W` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x12n\x81a\x03\xD9V[a\x12\xBE_T\x91a\x12\xA2a\x12\x8Ca\x12\x88\x85`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x94\x81\x95a\x138W[\x81\x15a\x13\x18W[Pa(VV[\x82a\x12\xB3`\x01`\xFF\x19_T\x16\x17_UV[a\x13\x01W[\x80a4\xA4V[a\x12\xC4W\0[a\x12\xD2a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x13\x13a\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x12\xB8V[0;\x15\x91P\x81a\x13*W[P_a\x12\x9CV[`\xFF\x16`\x01\x14\x90P_a\x13#V[`\x01`\xFF\x82\x16\x10\x91Pa\x12\x95V[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91Wa\x06ra\x06fa(\xB9V[4a\x02\x91W` 6`\x03\x19\x01\x12a\x02\x91W`\x045a\x13\xC2\x81a\x03\xD9V[a\x13\xCAa0TV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x13\xE2Wa\x04\x8E\x90a3FV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02\x91W_6`\x03\x19\x01\x12a\x02\x91W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02\x91Wa\x14l6a\r\xEBV[\x90a\x14ua3\x8EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_[\x81\x81\x10a\x14\xD4WP\x82;\x15a\x02\x91Wa\x0E\xB4\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xFC\xE3l}`\xE0\x1B\x84R`\x04\x84\x01a+\x08V[\x80a\x15\x03a\x14\xEA` a\x0FV`\x01\x95\x87\x89a*\xE6V[`@a\x14\xF7\x84\x87\x89a*\xE6V[\x015\x900\x903\x90a3\xB1V[a\x15,a\x15\x16` a\x0FV\x84\x87\x89a*\xE6V[\x86`@a\x15$\x85\x88\x8Aa*\xE6V[\x015\x91a3\xFAV[\x01a\x14\xA2V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x15WW`\x05\x1B\x01\x90V[a\x152V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x16La\x16)a\x16R\x95a\x16#a\x16\x1C\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x15\xF3\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x18V[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96a,!V[\x90a,gV[\x92a\x16#a\x16>a\x168a,\xC9V[\x94a-\xC0V[\x91a\x16Ga.\xDCV[a,!V[\x91a/&V[\x90\x91V[`@Q=_\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x02\x91WQ\x90V[\x90\x81` \x91\x03\x12a\x02\x91WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x02\x91W\x90V[\x90\x81` \x91\x03\x12a\x02\x91WQ`\xFF\x81\x16\x81\x03a\x02\x91W\x90V[`@Q\x90a\x16\xB7` \x83a\x02\x18V[_\x80\x83R6` \x84\x017V[\x90a\x16\xCD\x82a\x07\xE1V[a\x16\xDA`@Q\x91\x82a\x02\x18V[\x82\x81R\x80\x92a\x16\xEB`\x1F\x19\x91a\x07\xE1V[\x01\x90` 6\x91\x017V[\x90\x81Q\x81\x10\x15a\x15WW\x01` \x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x17(WV[a\x17\x06V[\x90`\x02\x82\x01\x80\x92\x11a\x17(WV[\x90`\x03\x82\x01\x80\x92\x11a\x17(WV[\x90`\x04\x82\x01\x80\x92\x11a\x17(WV[\x90`\x05\x82\x01\x80\x92\x11a\x17(WV[\x91\x90\x82\x01\x80\x92\x11a\x17(WV[`\x01`\x01``\x1B\x03\x81\x16\x03a\x02\x91WV[\x90\x81`@\x91\x03\x12a\x02\x91W` `@Q\x91a\x17\x9D\x83a\x01\xDDV[\x80Qa\x17\xA8\x81a\x03\xD9V[\x83R\x01Qa\x17\xB5\x81a\x17rV[` \x82\x01R\x90V[\x80Q\x82\x10\x15a\x15WW` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x17(W`\x01\x01\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x04\x90Wa\x18_\x91` \x91_\x91a\x1B^W[P`@Q\x80\x93\x81\x92c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x90\x81\x15a\x04\x90W_\x91a\x1B/W[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81\x15\x90\x81\x15a\x1A\xCCW[Pa\x1A\xC0Wa\x18\x95\x90a0\xACV[_\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x19qWa\x19\x1A` a\x18\xF7a\x18\xF1a\x18\xE3\x89\x87a\x16\xF5V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x87Z\xFA\x80\x15a\x04\x90W`\x01\x92a\x199\x92_\x92a\x19AW[Pa\x17eV[\x94\x01\x93a\x18\xC5V[a\x19c\x91\x92P` =\x81\x11a\x19jW[a\x19[\x81\x83a\x02\x18V[\x81\x01\x90a\x16aV[\x90_a\x193V[P=a\x19QV[a\x19|\x91\x94Pa\x16\xC3V[\x92_\x90_[\x81Q\x81\x10\x15a\x1A\xBAWa\x19\x9Aa\x18\xF1a\x18\xE3\x83\x85a\x16\xF5V[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x90` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x04\x90W_\x92a\x1A\x9AW[P\x90_\x91[\x81\x83\x10a\x19\xDFWPPP`\x01\x01a\x19\x81V[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x04\x90Wa\x1A^\x8Ba\x1AO\x83a\x1AIa\x1A=`\x01\x98a\x1Ac\x98_\x91a\x1AlW[PQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x17\xBDV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a\x17\xD1V[\x95\x01\x91\x90a\x19\xCDV[a\x1A\x8D\x91P`@=\x81\x11a\x1A\x93W[a\x1A\x85\x81\x83a\x02\x18V[\x81\x01\x90a\x17\x83V[_a\x1A/V[P=a\x1A{V[a\x1A\xB3\x91\x92P` =\x81\x11a\x19jWa\x19[\x81\x83a\x02\x18V[\x90_a\x19\xC8V[PPPPV[Pa\x1A\xC9a\x16\xA8V[\x90V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x04\x90W`\xFF\x91_\x91a\x1B\0W[P\x16\x15_a\x18\x87V[a\x1B\"\x91P` =` \x11a\x1B(W[a\x1B\x1A\x81\x83a\x02\x18V[\x81\x01\x90a\x16\x8FV[_a\x1A\xF7V[P=a\x1B\x10V[a\x1BQ\x91P` =` \x11a\x1BWW[a\x1BI\x81\x83a\x02\x18V[\x81\x01\x90a\x16pV[_a\x18rV[P=a\x1B?V[a\x1Bu\x91P\x82=\x84\x11a\x19jWa\x19[\x81\x83a\x02\x18V[_a\x18?V[`@Q\x90a\x1B\x88\x82a\x01\xDDV[``` \x83\x82\x81R\x01RV[\x15a\x1B\x9BWV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a\x1B\xB0WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a\x1B\xC6WV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a\x1B\xDCWV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[_\x19\x81\x01\x91\x90\x82\x11a\x17(WV[\x15a\x1C\0WV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90\x82\x10\x15a\x15WW\x01\x90V[\x90\x81` \x91\x03\x12a\x02\x91WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x02\x91W\x90V[\x15a\x1CCWV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\x91WQa\x1A\xC9\x81a\x17rV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a\x17(WV[\x15a\x1C\x8EWV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a\x1C\xA4WV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a\x1C\xDDWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1C\xD0V[\x94\x93\x92\x90\x91\x93a\x1D\x01a\x1B{V[Pa\x1D\r\x85\x15\x15a\x1B\x94V[`@\x84\x01QQ\x85\x14\x80a#\xF6W[\x80a#\xE8W[\x80a#\xDAW[a\x1D0\x90a\x1B\xA9V[a\x1DB` \x85\x01QQ\x85QQ\x14a\x1B\xBFV[a\x1DYc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a\x1B\xD5V[a\x1Daa\x02KV[_\x81R_` \x82\x01R\x92a\x1Dsa\x1B{V[a\x1D|\x87a\x16\xC3V[` \x82\x01Ra\x1D\x8A\x87a\x16\xC3V[\x81Ra\x1D\x94a\x1B{V[\x92a\x1D\xA3` \x88\x01QQa\x16\xC3V[\x84Ra\x1D\xB3` \x88\x01QQa\x16\xC3V[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x04\x90Wa\x1E\x1C\x91_\x91a#\xBBW[Pa\x1E\x176\x8B\x87a\x0B\xFEV[a1\xA8V[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a\x1F\x98W` \x88a\x1E\x8Da\x1E\x83\x8Ca\x1E{\x8F\x96\x86\x8Ea\x1E`a\x1EM\x86\x80\x95a\x17\xBDV[Q\x80Q_R` \x01Q` R`@_ \x90V[a\x1Em\x84\x84\x84\x01Qa\x17\xBDV[R\x82a\x1FeW[\x01Qa\x17\xBDV[Q\x95Qa\x17\xBDV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x04\x90Wa\x16#\x8Aa\x1F:\x8Fa\x1F3\x8F\x84` \x8F\x92a\x1F*\x93a\x1F\"\x84`\x01\x9Ea\x1F@\x9E_\x91a\x1FHW[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa\x17\xBDV[R\x01Qa\x17\xBDV[Q\x93\x8DQa\x17\xBDV[Q\x16a1\xD3V[\x90a2\x04V[\x97\x01\x96a\x1E V[a\x1F_\x91P\x86=\x81\x11a\x1BWWa\x1BI\x81\x83a\x02\x18V[_a\x1F\x13V[a\x1F\x93a\x1Fu\x84\x84\x84\x01Qa\x17\xBDV[Qa\x1F\x8C\x84\x84\x01Qa\x1F\x86\x87a\x1B\xEBV[\x90a\x17\xBDV[Q\x10a\x1B\xF9V[a\x1EtV[P\x90\x95\x97\x94\x96Pa\x1F\xAD\x91\x98\x93\x92\x99Pa2\xC1V[\x91_\x90[\x80\x82\x10a \x13WPPP\x92a\x1F\xECa\x1F\xE7a\x1F\xE0a \r\x95\x85a\x1F\xFF\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x15pV[\x91\x90a\x1C\x87V[a\x1C\x9DV[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a\x1C\xB3V[\x03`\x1F\x19\x81\x01\x83R\x82a\x02\x18V[Q\x90 \x90V[\x91\x84\x95\x96\x93a ^a\x18\xF1a P\x85\x87a J\x87\x9F\x98\x9E\x9Ba\x1EM\x84`@a <\x93\x01Qa\x17\xBDV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x96a\x1C\x0FV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[` \x87a ra\x1E\x83\x8D`\xA0\x8D\x01Qa\x17\xBDV[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x92\x83\x15a\x04\x90Wa!\x0E\x93a \xF9\x92_\x91a#\x8DW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a\x1C<V[a!\x07\x89`@\x89\x01Qa\x17\xBDV[Q\x90a,gV[\x90a! a\x18\xF1a P\x8A\x86\x8Da\x1C\x0FV[` \x86a!4a\x1E\x83\x8C`\xC0\x8C\x01Qa\x17\xBDV[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x04\x90Wa!\xCA\x8A` \x8Ea\x1Eta!\xD7\x95a!\xE2\x97_\x91a#pW[Pa!\xBC\x85\x85\x85\x01Qa\x17\xBDV[\x90`\x01`\x01``\x1B\x03\x16\x90RV[Q`\x01`\x01``\x1B\x03\x16\x90V[a!\xBC\x8A\x8DQa\x17\xBDV[_\x96_[` \x88\x01QQ\x81\x10\x15a#\\W\x89\x8Ba\"\"a\"\x03\x84\x8AQa\x17\xBDV[Qa\"\x15a\x18\xF1a P\x86\x8C\x87a\x1C\x0FV[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a\"1W[PP`\x01\x01a!\xE6V[\x88\x88a\"\xB8a\"xa\x1E\x83\x87\x9F\x8F\x97\x80`\xE0\x8F\x84\x9C` a\"aa\x18\xF1a P\x83\x9F\x98a\"i\x96a\"r\x9Aa\x1C\x0FV[\x9A\x01Qa\x17\xBDV[Q\x9A\x01Qa\x17\xBDV[Qa\x17\xBDV[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x95\x90\x95R\x91\x90\x93\x16`d\x84\x01R\x82\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x04\x90Wa#\x1B\x8E`\x01\x94\x8E\x86\x95_\x92a#&W[Pa\x1AIa!\xBC\x92\x93Q\x93a#\x16a!\xCA\x84\x87a\x17\xBDV[a\x1CgV[\x01\x98\x90P\x89\x8Ba\"'V[a!\xBC\x92Pa#Na\x1AI\x91` =\x81\x11a#UW[a#F\x81\x83a\x02\x18V[\x81\x01\x90a\x1CRV[\x92Pa\"\xFEV[P=a#<V[P\x95\x94\x97`\x01\x91\x99\x93\x92\x94\x97P\x01\x90a\x1F\xB1V[a#\x87\x91P\x84=\x81\x11a#UWa#F\x81\x83a\x02\x18V[_a!\xAEV[a#\xAE\x91P` =\x81\x11a#\xB4W[a#\xA6\x81\x83a\x02\x18V[\x81\x01\x90a\x1C\x1BV[_a \xE3V[P=a#\x9CV[a#\xD4\x91P` =` \x11a\x1B(Wa\x1B\x1A\x81\x83a\x02\x18V[_a\x1E\x0BV[P`\xE0\x84\x01QQ\x85\x14a\x1D'V[P`\xC0\x84\x01QQ\x85\x14a\x1D!V[P`\xA0\x84\x01QQ\x85\x14a\x1D\x1BV[\x15a$\x0BWV[cC\x94\xDB\xDF`\xE1\x1B_R`\x04_\xFD[a$N3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a$\x04V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x02\x91W_\x92\x83\x92`@Q\x94\x85\x80\x94\x81\x93c\x99&\xEE}`\xE0\x1B\x83R`\x01\x80`\xA0\x1B\x03\x16`\x04\x83\x01R`@`$\x83\x01R`@a$\xC3\x82Q```D\x86\x01R`\xA4\x85\x01\x90a$\xF4V[\x91` \x81\x01Q`d\x85\x01R\x01Q`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\x04\x90Wa$\xE6WPV[\x80a\x0E\xD2_a\x02I\x93a\x02\x18V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90\x81\x10\x15a\x15WW`\x05\x1B\x81\x015\x90`\xBE\x19\x816\x03\x01\x82\x12\x15a\x02\x91W\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x02\x91W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x91W` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\x02\x91WV[\x91\x90\x81\x10\x15a\x15WW`\x06\x1B\x01\x90V[5a\x1A\xC9\x81a\x03\xD9V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x02\x91W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x02\x91W\x81`\x06\x1B6\x03\x83\x13a\x02\x91WV[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a%\xD6WPPP\x90V[\x90\x91\x92`@\x80`\x01\x92\x865a%\xEA\x81a\x03\xD9V[\x84\x80`\xA0\x1B\x03\x16\x81R`\x01`\x01``\x1B\x03` \x88\x015a&\t\x81a\x17rV[\x16` \x82\x01R\x01\x94\x01\x92\x91\x01a%\xC9V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x02\x91W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x02\x91W\x816\x03\x83\x13a\x02\x91WV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x83\x90R`\x05\x83\x90\x1B\x81\x01``\x90\x81\x01\x93\x83\x926\x84\x90\x03`\xBE\x19\x01\x92_\x91\x90\x81\x01\x90[\x83\x83\x10a&\xB6WPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`_\x19\x82\x82\x03\x01\x83R\x875\x86\x81\x12\x15a\x02\x91W\x87\x01\x90a&\xEEa&\xE0\x83\x80a%\x89V[`\xC0\x84R`\xC0\x84\x01\x91a%\xBDV[\x91` \x81\x015a&\xFD\x81a\x03\xD9V[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x81\x01\x91\x90\x91Ra'\x1C`@\x83\x01\x83a%\x89V[\x84\x86\x03`@\x86\x01R\x80\x86R\x94\x90\x91\x01\x93_[\x81\x81\x10a'\xA7WPPPa'\x96`\x01\x93` \x93a'\x88\x84a'ba'U``\x89\x98\x01a\x07\xD0V[c\xFF\xFF\xFF\xFF\x16``\x85\x01RV[a'~a'q`\x80\x83\x01a\x07\xD0V[c\xFF\xFF\xFF\xFF\x16`\x80\x85\x01RV[`\xA0\x81\x01\x90a&\x1AV[\x91`\xA0\x81\x85\x03\x91\x01Ra&KV[\x99\x01\x93\x01\x93\x01\x91\x95\x94\x93\x92\x90a&\xA5V[\x90\x91\x94`@\x80`\x01\x92\x885a'\xBB\x81a\x03\xD9V[\x84\x80`\xA0\x1B\x03\x16\x81R` \x89\x015` \x82\x01R\x01\x96\x01\x91\x01\x91\x90\x91a'.V[\x90` a\x1A\xC9\x92\x81\x81R\x01\x90a$\xF4V[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x84\x01R\x81\x84\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x90\x92\x01Q``\x80\x83\x01R\x80Q`\x80\x83\x01\x81\x90R`\xA0\x90\x92\x01\x92\x01\x90_[\x81\x81\x10a(:WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a(-V[\x15a(]WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x04\x90W`\xFF\x91_\x91a*\xC7W[P\x16\x80\x15a*\xBDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a*yWPa)T\x91Pa\x16\xC3V[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x04\x90W`\xFF\x91_\x91a*[W[P\x16\x81\x10\x15a*TW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x04\x90W_\x92a*4W[P\x90_\x91[\x81\x83\x10a)\xCEWPPP`\x01\x01a)YV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x04\x90Wa\x1A^\x8Ba\x1AO\x83a\x1AIa\x1A=`\x01\x98a*+\x98_\x91a\x1AlWPQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95\x01\x91\x90a)\xBCV[a*M\x91\x92P` =\x81\x11a\x19jWa\x19[\x81\x83a\x02\x18V[\x90_a)\xB7V[P\x92PPPV[a*s\x91P` =\x81\x11a\x1B(Wa\x1B\x1A\x81\x83a\x02\x18V[_a)\x7FV[`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x88Z\xFA\x80\x15a\x04\x90W`\x01\x92a*\xB4\x92_\x92a\x19AWPa\x17eV[\x92\x01\x91\x90a)BV[PPa\x1A\xC9a\x16\xA8V[a*\xE0\x91P` =` \x11a\x1B(Wa\x1B\x1A\x81\x83a\x02\x18V[_a)\nV[\x91\x90\x81\x10\x15a\x15WW`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x02\x91W\x01\x90V[\x90\x91\x80` \x83\x01` \x84RR`@\x82\x01\x92`@\x82`\x05\x1B\x84\x01\x01\x93\x81\x93_\x91`\x9E\x19\x846\x03\x01\x91[\x85\x84\x10a+AWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`?\x19\x82\x82\x03\x01\x83R\x875\x90\x84\x82\x12\x15a\x02\x91W` \x80\x91\x88`\x01\x94\x01\x90`\x80c\xFF\xFF\xFF\xFFa+\xC8\x82a+\x8Ca+~\x87\x80a%\x89V[`\xA0\x88R`\xA0\x88\x01\x91a%\xBDV[\x95\x87\x81\x015a+\x9A\x81a\x03\xD9V[\x8A\x80`\xA0\x1B\x03\x16\x88\x87\x01R`@\x81\x015`@\x87\x01R\x83a+\xBC``\x83\x01a\x07\xD0V[\x16``\x87\x01R\x01a\x07\xD0V[\x16\x91\x01R\x99\x01\x93\x01\x94\x01\x92\x91\x95\x94\x93\x90a+0V[`@Q\x90a+\xEA\x82a\x01\xDDV[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80a,\x06\x81\x84a\x02\x18V[6\x837V[`@Q\x90a,\x1A` \x83a\x02\x18V[` 6\x837V[\x91\x90`@\x90``a,0a+\xDDV[\x94\x85\x92` \x85Q\x92a,B\x85\x85a\x02\x18V[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15a,eWV[\xFE[` \x92\x91`\x80`@\x92a,xa+\xDDV[\x95\x86\x93\x81\x86Q\x93a,\x89\x86\x86a\x02\x18V[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15a,eW\x15a,\xBAWV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@Qa,\xD5\x81a\x01\xDDV[`@\x90\x81Qa,\xE4\x83\x82a\x02\x18V[\x826\x827\x81R` \x82Q\x91a,\xF9\x84\x84a\x02\x18V[\x836\x847\x01R\x80Qa-\x0B\x82\x82a\x02\x18V[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90a-a\x83\x83a\x02\x18V[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01Ra-\xB6\x83Q\x93\x84a\x02\x18V[\x82R` \x82\x01R\x90V[_Q` a7\x80_9_Q\x90_R\x90a-\xD7a+\xDDV[P_\x91\x90\x06` `\xC0\x83[a.\xD7W_\x93_Q` a7\x80_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa.\r\x85\x82a\x02\x18V[\x846\x827\x84\x81\x85`@Qa.!\x82\x82a\x02\x18V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` a7\x80_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a,eWa.\x8B\x90a6\xD4V[Q\x91a.\xD7W_Q` a7\x80_9_Q\x90_R\x82\x80\t\x14a.\xC2WP_Q` a7\x80_9_Q\x90_R`\x01_\x94\x08\x92\x93a-\xE2V[\x92\x93PPa.\xCEa\x02KV[\x92\x83R\x82\x01R\x90V[a\x15\\V[a.\xE4a+\xDDV[P`@Qa.\xF1\x81a\x01\xDDV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a\x17(WV[\x90`\x0C\x81\x10\x15a\x15WW`\x05\x1B\x01\x90V[\x93\x92\x90\x91a/4`@a\x02ZV[\x94\x85R` \x85\x01Ra/F`@a\x02ZV[\x91\x82R` \x82\x01Ra/Va+\xF5V[\x92_[`\x02\x81\x10a/\x83WPPP` a\x01\x80\x92a/ra,\x0BV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80a/\x8F`\x01\x92a.\xFFV[a/\x99\x82\x85a\x15FV[QQa/\xA5\x82\x89a/\x15V[R` a/\xB2\x83\x86a\x15FV[Q\x01Qa/\xC7a/\xC1\x83a\x17\x1AV[\x89a/\x15V[Ra/\xD2\x82\x86a\x15FV[QQQa/\xE1a/\xC1\x83a\x17-V[Ra/\xF7a/\xEF\x83\x87a\x15FV[QQ` \x01\x90V[Qa0\x04a/\xC1\x83a\x17;V[R` a0\x11\x83\x87a\x15FV[Q\x01QQa0!a/\xC1\x83a\x17IV[Ra0Ma0Ga0@` a07\x86\x8Aa\x15FV[Q\x01Q` \x01\x90V[Q\x92a\x17WV[\x88a/\x15V[R\x01a/YV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a0hWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[a\xFF\xFFa0\xB8\x82a1\xD3V[\x16a0\xC2\x81a\x0B\xE3V[\x90a0\xD0`@Q\x92\x83a\x02\x18V[\x80\x82Ra0\xDF`\x1F\x19\x91a\x0B\xE3V[\x016` \x83\x017__[\x82Q\x82\x10\x80a1?W[\x15a18W`\x01\x81\x1B\x84\x16a1\x11W[a1\x0C\x90a\x17\xD1V[a0\xE9V[\x90`\x01a1\x0C\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa1.\x82\x87a\x16\xF5V[S\x01\x91\x90Pa1\x03V[PP\x90P\x90V[Pa\x01\0\x81\x10a0\xF3V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eUV[\x90`\x01a1\xB6`\xFF\x93a5-V[\x92\x83\x92\x16\x1B\x11\x15a1\xC4W\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[a1\xDFWP\x90V[_\x19\x81\x01\x81\x81\x11a\x17(Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x17(W`\x01\x01\x90\x80a1\xD7V[\x90a2\ra+\xDDV[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15a2\xB2W`\x01\x82\x14a2\xADWa2.a\x02KV[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15a2SWPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14a2\x8DW[`\x01a2\x83a2x\x83`\xFF\x94a,gV[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91a2?V[\x94`\x01a2\x83a2xa2\xA2\x89`\xFF\x95a,gV[\x98\x93PPPPa2gV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[a2\xC9a+\xDDV[P\x80Q\x90\x81\x15\x80a3:W[\x15a2\xF6WPP`@Qa2\xEA`@\x82a\x02\x18V[_\x81R_` \x82\x01R\x90V[` _Q` a7\x80_9_Q\x90_R\x91\x01Q\x06_Q` a7\x80_9_Q\x90_R\x03_Q` a7\x80_9_Q\x90_R\x81\x11a\x17(W`@Q\x91a-\xB6\x83a\x01\xDDV[P` \x81\x01Q\x15a2\xD5V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x03a3\xA2WV[c\x8Ey\xFD\xB5`\xE0\x1B_R`\x04_\xFD[`@Qc#\xB8r\xDD`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x82\x01R\x92\x90\x91\x16`D\x83\x01R`d\x80\x83\x01\x93\x90\x93R\x91\x81Ra\x02I\x91a3\xF5`\x84\x83a\x02\x18V[a6\x1CV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R` \x81\x80`D\x81\x01\x03\x81`\x01`\x01`\xA0\x1B\x03\x86\x16Z\xFA\x90\x81\x15a\x04\x90Wa\x02I\x94a3\xF5\x92a4Q\x92_\x91a4\x85WPa\x17eV[`@Qc\t^\xA7\xB3`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`$\x85\x01R`D\x80\x85\x01\x91\x90\x91R\x83R`d\x83a\x02\x18V[a4\x9E\x91P` =` \x11a\x19jWa\x19[\x81\x83a\x02\x18V[_a\x193V[\x90`\xFF_T`\x08\x1C\x16\x15a4\xBEWa\x06\x9Ea\x02I\x92a3FV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[\x15a5\x1EWV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11a5\x96W\x81Q\x15a5\x91W` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15a5\x8CW`\x01\x90a5wa5ma\x18\xF1a\x18\xE3\x86\x89a\x16\xF5V[`\xFF`\x01\x91\x16\x1B\x90V[\x90a5\x83\x81\x83\x11a5\x17V[\x17\x91\x01\x90a5NV[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x02\x91WQ\x80\x15\x15\x81\x03a\x02\x91W\x90V[\x15a5\xC4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[a\x02I\x91a6\x9E\x91`\x01\x80`\xA0\x1B\x03\x16_\x80`@Q\x93a6=`@\x86a\x02\x18V[` \x85R\x7FSafeERC20: low-level call failed` \x86\x01R` \x81Q\x91\x01\x82\x85Z\xF1=\x15a6\xCCW=\x91a6\x82\x83a\x0B\xE3V[\x92a6\x90`@Q\x94\x85a\x02\x18V[\x83R=_` \x85\x01>a6\xEAV[\x80Q\x90\x81\x15\x91\x82\x15a6\xB2W[PPa5\xBDV[a6\xC5\x92P` \x80\x91\x83\x01\x01\x91\x01a5\xA5V[_\x80a6\xABV[``\x91a6\xEAV[\x15a6\xDBWV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD[\x91\x92\x90\x15a7LWP\x81Q\x15a6\xFEWP\x90V[;\x15a7\x07W\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[\x82Q\x90\x91P\x15a7_WP\x80Q\x90` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x90a7{\x90`\x04\x83\x01a'\xDBV[\x03\x90\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 6\xD9;\xE6%\x03\xCE.Zj\xAB#~\x87\t\x84\xBC\xFF>`\x938x\xBF\xF1\xCF\x0C\xC8\xD2\xF6\x86\xE7dsolcC\0\x08\x1B\x003",
    );
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RewardsInitiatorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    225u8, 28u8, 221u8, 241u8, 129u8, 106u8, 67u8, 49u8, 140u8, 161u8, 117u8,
                    187u8, 197u8, 44u8, 208u8, 24u8, 84u8, 54u8, 233u8, 203u8, 234u8, 215u8, 200u8,
                    58u8, 204u8, 84u8, 167u8, 62u8, 70u8, 23u8, 23u8, 227u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &RewardsInitiatorUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _avsDirectory, address _slashingRegCoordinator, address _stakeRegistry, address rewards_coordinator, address _permissionController, address _allocationManager);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _avsDirectory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _slashingRegCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _stakeRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rewards_coordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _permissionController: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._avsDirectory,
                        value._slashingRegCoordinator,
                        value._stakeRegistry,
                        value.rewards_coordinator,
                        value._permissionController,
                        value._allocationManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _avsDirectory: tuple.0,
                        _slashingRegCoordinator: tuple.1,
                        _stakeRegistry: tuple.2,
                        rewards_coordinator: tuple.3,
                        _permissionController: tuple.4,
                        _allocationManager: tuple.5,
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
                alloy::sol_types::sol_data::Address,
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
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._slashingRegCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._stakeRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.rewards_coordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionController,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addPendingAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addPendingAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addPendingAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addPendingAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addPendingAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `avsDirectory()` and selector `0x6b3aa72e`.
    ```solidity
    function avsDirectory() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `blsApkRegistry()` and selector `0x5df45946`.
    ```solidity
    function blsApkRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = blsApkRegistryReturn;
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive()]
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IRewardsCoordinatorTypes::RewardsSubmission>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createAVSRewardsSubmissionCall> for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createAVSRewardsSubmissionCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createAVSRewardsSubmissionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createAVSRewardsSubmissionCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IRewardsCoordinatorTypes::RewardsSubmission>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [252u8, 227u8, 108u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.rewardsSubmissions,
                ),)
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
    #[derive()]
    /**Function with signature `createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])` and selector `0xa20b99bf`.
    ```solidity
    function createOperatorDirectedAVSRewardsSubmission(IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorDirectedAVSRewardsSubmissionCall {
        #[allow(missing_docs)]
        pub operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])`](createOperatorDirectedAVSRewardsSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorDirectedAVSRewardsSubmissionReturn {}
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
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createOperatorDirectedAVSRewardsSubmissionCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: createOperatorDirectedAVSRewardsSubmissionCall) -> Self {
                    (value.operatorDirectedRewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for createOperatorDirectedAVSRewardsSubmissionCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorDirectedRewardsSubmissions: tuple.0,
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
            impl ::core::convert::From<createOperatorDirectedAVSRewardsSubmissionReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: createOperatorDirectedAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for createOperatorDirectedAVSRewardsSubmissionReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOperatorDirectedAVSRewardsSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorDirectedAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])";
            const SELECTOR: [u8; 4] = [162u8, 11u8, 153u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.operatorDirectedRewardsSubmissions,
                ),)
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegation()` and selector `0xdf5cf723`.
    ```solidity
    function delegation() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = delegationReturn;
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromAVSCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsCall) -> Self {
                    (value.operator, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromOperatorSetsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromOperatorSetsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorRestakedStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorRestakedStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<getOperatorRestakedStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorRestakedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorRestakedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorRestakedStrategiesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRestakeableStrategies()` and selector `0xe481af9d`.
    ```solidity
    function getRestakeableStrategies() external view returns (address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRestakeableStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRestakeableStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getRestakeableStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRestakeableStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRestakeableStrategiesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRestakeableStrategiesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    (value._initialOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _initialOwner: tuple.0,
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
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
    ```solidity
    function owner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = ownerReturn;
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registerOperatorToAVS(address,(bytes,bytes32,uint256))` and selector `0x9926ee7d`.
    ```solidity
    function registerOperatorToAVS(address operator, ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry memory operatorSignature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSignature: <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
                ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerOperatorToAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSCall) -> Self {
                    (value.operator, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorToAVSCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorToAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorToAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorToAVSCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "registerOperatorToAVS(address,(bytes,bytes32,uint256))";
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
                    <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registryCoordinator()` and selector `0x6d14a987`.
    ```solidity
    function registryCoordinator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeAppointeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeAppointeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAppointeeReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeAppointeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removePendingAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: removePendingAdminCall) -> Self {
                    (value.pendingAdmin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removePendingAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pendingAdmin: tuple.0,
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
            impl ::core::convert::From<removePendingAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removePendingAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removePendingAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removePendingAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removePendingAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `rewardsInitiator()` and selector `0xfc299dee`.
    ```solidity
    function rewardsInitiator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rewardsInitiatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardsInitiatorCall {
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
            impl ::core::convert::From<rewardsInitiatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsInitiatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsInitiatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAppointeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setClaimerFor(address)` and selector `0xa0169ddd`.
    ```solidity
    function setClaimerFor(address claimer) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerForCall {
        #[allow(missing_docs)]
        pub claimer: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setClaimerFor(address)`](setClaimerForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerForReturn {}
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
            impl ::core::convert::From<setClaimerForCall> for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerForCall) -> Self {
                    (value.claimer,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setClaimerForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { claimer: tuple.0 }
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
            impl ::core::convert::From<setClaimerForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerForReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setClaimerForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setClaimerForCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setClaimerForReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setClaimerFor(address)";
            const SELECTOR: [u8; 4] = [160u8, 22u8, 157u8, 221u8];
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
                        &self.claimer,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsInitiatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorCall) -> Self {
                    (value.newRewardsInitiator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsInitiatorCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsInitiatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsInitiatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsInitiatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
    ```solidity
    function stakeRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = stakeRegistryReturn;
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateAVSMetadataURICall> for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value._metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _metadataURI: tuple.0,
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
            impl ::core::convert::From<updateAVSMetadataURIReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateAVSMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateAVSMetadataURICall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`MockAvsServiceManager`](self) function calls.
    #[derive()]
    pub enum MockAvsServiceManagerCalls {
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
        createOperatorDirectedAVSRewardsSubmission(createOperatorDirectedAVSRewardsSubmissionCall),
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
        owner(ownerCall),
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
        setClaimerFor(setClaimerForCall),
        #[allow(missing_docs)]
        setRewardsInitiator(setRewardsInitiatorCall),
        #[allow(missing_docs)]
        stakeRegistry(stakeRegistryCall),
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
            [93u8, 244u8, 89u8, 70u8],
            [104u8, 48u8, 72u8, 53u8],
            [107u8, 58u8, 167u8, 46u8],
            [109u8, 20u8, 169u8, 135u8],
            [110u8, 251u8, 70u8, 54u8],
            [113u8, 80u8, 24u8, 166u8],
            [141u8, 165u8, 203u8, 91u8],
            [153u8, 38u8, 238u8, 125u8],
            [157u8, 161u8, 109u8, 142u8],
            [160u8, 22u8, 157u8, 221u8],
            [162u8, 11u8, 153u8, 191u8],
            [163u8, 100u8, 244u8, 218u8],
            [169u8, 143u8, 179u8, 85u8],
            [186u8, 85u8, 8u8, 128u8],
            [193u8, 168u8, 226u8, 197u8],
            [196u8, 214u8, 109u8, 232u8],
            [223u8, 92u8, 247u8, 35u8],
            [228u8, 129u8, 175u8, 157u8],
            [242u8, 253u8, 227u8, 139u8],
            [252u8, 41u8, 157u8, 238u8],
            [252u8, 227u8, 108u8, 125u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockAvsServiceManagerCalls {
        const NAME: &'static str = "MockAvsServiceManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 27usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
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
                Self::createOperatorDirectedAVSRewardsSubmission(_) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::setClaimerFor(_) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRewardsInitiator(_) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
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
            )
                -> alloy_sol_types::Result<MockAvsServiceManagerCalls>] = &[
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(MockAvsServiceManagerCalls::setRewardsInitiator)
                    }
                    setRewardsInitiator
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(MockAvsServiceManagerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(MockAvsServiceManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(MockAvsServiceManagerCalls::removePendingAdmin)
                    }
                    removePendingAdmin
                },
                {
                    fn setClaimerFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <setClaimerForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(MockAvsServiceManagerCalls::setClaimerFor)
                    }
                    setClaimerFor
                },
                {
                    fn createOperatorDirectedAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                MockAvsServiceManagerCalls::createOperatorDirectedAVSRewardsSubmission,
                            )
                    }
                    createOperatorDirectedAVSRewardsSubmission
                },
                {
                    fn deregisterOperatorFromAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
                        )
                        .map(MockAvsServiceManagerCalls::updateAVSMetadataURI)
                    }
                    updateAVSMetadataURI
                },
                {
                    fn removeAppointee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <removeAppointeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(MockAvsServiceManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(MockAvsServiceManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn getRestakeableStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerCalls> {
                        <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(MockAvsServiceManagerCalls::rewardsInitiator)
                    }
                    rewardsInitiator
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
                Self::createOperatorDirectedAVSRewardsSubmission(inner) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::setClaimerFor(inner) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::createOperatorDirectedAVSRewardsSubmission(inner) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::setClaimerFor(inner) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    #[derive(Debug, PartialEq, Eq, Hash)]
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
        OnlyStakeRegistry(OnlyStakeRegistry),
        #[allow(missing_docs)]
        ScalarTooLarge(ScalarTooLarge),
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
            [128u8, 200u8, 131u8, 72u8],
            [135u8, 41u8, 183u8, 190u8],
            [142u8, 121u8, 253u8, 181u8],
            [171u8, 27u8, 35u8, 107u8],
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
        const COUNT: usize = 20usize;
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
                Self::ECAddFailed(_) => <ECAddFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ECMulFailed(_) => <ECMulFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ExpModFailed(_) => <ExpModFailed as alloy_sol_types::SolError>::SELECTOR,
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
                Self::OnlyStakeRegistry(_) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ScalarTooLarge(_) => <ScalarTooLarge as alloy_sol_types::SolError>::SELECTOR,
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
                -> alloy_sol_types::Result<MockAvsServiceManagerErrors>] = &[
                {
                    fn InputEmptyQuorumNumbers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(MockAvsServiceManagerErrors::InvalidReferenceBlocknumber)
                    }
                    InvalidReferenceBlocknumber
                },
                {
                    fn InputNonSignerLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(MockAvsServiceManagerErrors::InputNonSignerLengthMismatch)
                    }
                    InputNonSignerLengthMismatch
                },
                {
                    fn InvalidBLSPairingKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(MockAvsServiceManagerErrors::InvalidBLSPairingKey)
                    }
                    InvalidBLSPairingKey
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(MockAvsServiceManagerErrors::InvalidBLSSignature)
                    }
                    InvalidBLSSignature
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
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
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(MockAvsServiceManagerErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
                        )
                        .map(MockAvsServiceManagerErrors::OnlyRegistryCoordinatorOwner)
                    }
                    OnlyRegistryCoordinatorOwner
                },
                {
                    fn InvalidQuorumApkHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockAvsServiceManagerErrors> {
                        <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(MockAvsServiceManagerErrors::ScalarTooLarge)
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
            DECODE_SHIMS[idx](data, validate)
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
                Self::DelayPeriodNotPassed(inner) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::OnlyRegistryCoordinator(inner) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyRewardsInitiator(inner) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyStakeRegistry(inner) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::DelayPeriodNotPassed(inner) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::OnlyRegistryCoordinator(inner) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyRewardsInitiator(inner) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyStakeRegistry(inner) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`MockAvsServiceManager`](self) events.
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum MockAvsServiceManagerEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        RewardsInitiatorUpdated(RewardsInitiatorUpdated),
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
                225u8, 28u8, 221u8, 241u8, 129u8, 106u8, 67u8, 49u8, 140u8, 161u8, 117u8, 187u8,
                197u8, 44u8, 208u8, 24u8, 84u8, 54u8, 233u8, 203u8, 234u8, 215u8, 200u8, 58u8,
                204u8, 84u8, 167u8, 62u8, 70u8, 23u8, 23u8, 227u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for MockAvsServiceManagerEvents {
        const NAME: &'static str = "MockAvsServiceManagerEvents";
        const COUNT: usize = 3usize;
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
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::RewardsInitiatorUpdated)
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
        _avsDirectory: alloy::sol_types::private::Address,
        _slashingRegCoordinator: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        rewards_coordinator: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<MockAvsServiceManagerInstance<T, P, N>>,
    > {
        MockAvsServiceManagerInstance::<T, P, N>::deploy(
            provider,
            _avsDirectory,
            _slashingRegCoordinator,
            _stakeRegistry,
            rewards_coordinator,
            _permissionController,
            _allocationManager,
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
        _avsDirectory: alloy::sol_types::private::Address,
        _slashingRegCoordinator: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        rewards_coordinator: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        MockAvsServiceManagerInstance::<T, P, N>::deploy_builder(
            provider,
            _avsDirectory,
            _slashingRegCoordinator,
            _stakeRegistry,
            rewards_coordinator,
            _permissionController,
            _allocationManager,
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
    pub struct MockAvsServiceManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for MockAvsServiceManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MockAvsServiceManagerInstance")
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
        > MockAvsServiceManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`MockAvsServiceManager`](self) contract instance.

        See the [wrapper's documentation](`MockAvsServiceManagerInstance`) for more details.*/
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
            _avsDirectory: alloy::sol_types::private::Address,
            _slashingRegCoordinator: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            rewards_coordinator: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<MockAvsServiceManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _avsDirectory,
                _slashingRegCoordinator,
                _stakeRegistry,
                rewards_coordinator,
                _permissionController,
                _allocationManager,
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
            _avsDirectory: alloy::sol_types::private::Address,
            _slashingRegCoordinator: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            rewards_coordinator: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _avsDirectory,
                        _slashingRegCoordinator,
                        _stakeRegistry,
                        rewards_coordinator,
                        _permissionController,
                        _allocationManager,
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
        > MockAvsServiceManagerInstance<T, P, N>
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
        ///Creates a new call builder for the [`addPendingAdmin`] function.
        pub fn addPendingAdmin(
            &self,
            admin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, addPendingAdminCall, N> {
            self.call_builder(&addPendingAdminCall { admin })
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(&self) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
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
            params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkSignaturesCall, N> {
            self.call_builder(&checkSignaturesCall {
                msgHash,
                quorumNumbers,
                referenceBlockNumber,
                params,
            })
        }
        ///Creates a new call builder for the [`createAVSRewardsSubmission`] function.
        pub fn createAVSRewardsSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createAVSRewardsSubmissionCall, N> {
            self.call_builder(&createAVSRewardsSubmissionCall { rewardsSubmissions })
        }
        ///Creates a new call builder for the [`createOperatorDirectedAVSRewardsSubmission`] function.
        pub fn createOperatorDirectedAVSRewardsSubmission(
            &self,
            operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorDirectedAVSRewardsSubmissionCall, N>
        {
            self.call_builder(&createOperatorDirectedAVSRewardsSubmissionCall {
                operatorDirectedRewardsSubmissions,
            })
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`deregisterOperatorFromAVS`] function.
        pub fn deregisterOperatorFromAVS(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromAVSCall, N> {
            self.call_builder(&deregisterOperatorFromAVSCall { operator })
        }
        ///Creates a new call builder for the [`deregisterOperatorFromOperatorSets`] function.
        pub fn deregisterOperatorFromOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromOperatorSetsCall, N>
        {
            self.call_builder(&deregisterOperatorFromOperatorSetsCall {
                operator,
                operatorSetIds,
            })
        }
        ///Creates a new call builder for the [`getOperatorRestakedStrategies`] function.
        pub fn getOperatorRestakedStrategies(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorRestakedStrategiesCall, N> {
            self.call_builder(&getOperatorRestakedStrategiesCall { operator })
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
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`registerOperatorToAVS`] function.
        pub fn registerOperatorToAVS(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSignature: <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorToAVSCall, N> {
            self.call_builder(&registerOperatorToAVSCall {
                operator,
                operatorSignature,
            })
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
            self.call_builder(&removeAppointeeCall {
                appointee,
                target,
                selector,
            })
        }
        ///Creates a new call builder for the [`removePendingAdmin`] function.
        pub fn removePendingAdmin(
            &self,
            pendingAdmin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, removePendingAdminCall, N> {
            self.call_builder(&removePendingAdminCall { pendingAdmin })
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
            self.call_builder(&setAppointeeCall {
                appointee,
                target,
                selector,
            })
        }
        ///Creates a new call builder for the [`setClaimerFor`] function.
        pub fn setClaimerFor(
            &self,
            claimer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setClaimerForCall, N> {
            self.call_builder(&setClaimerForCall { claimer })
        }
        ///Creates a new call builder for the [`setRewardsInitiator`] function.
        pub fn setRewardsInitiator(
            &self,
            newRewardsInitiator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsInitiatorCall, N> {
            self.call_builder(&setRewardsInitiatorCall {
                newRewardsInitiator,
            })
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
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
        ) -> alloy_contract::SolCallBuilder<T, &P, trySignatureAndApkVerificationCall, N> {
            self.call_builder(&trySignatureAndApkVerificationCall {
                msgHash,
                apk,
                apkG2,
                sigma,
            })
        }
        ///Creates a new call builder for the [`updateAVSMetadataURI`] function.
        pub fn updateAVSMetadataURI(
            &self,
            _metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(&updateAVSMetadataURICall { _metadataURI })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > MockAvsServiceManagerInstance<T, P, N>
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
    }
}
