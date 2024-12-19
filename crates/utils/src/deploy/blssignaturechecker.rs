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
        pub nonSignerPubkeys:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
        pub quorumApks:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
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
    /**```solidity
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumStakeTotals {
        pub signedStakeForQuorum:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
    pub struct IBLSSignatureCheckerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBLSSignatureCheckerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBLSSignatureCheckerInstance")
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
        > IBLSSignatureCheckerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IBLSSignatureChecker`](self) contract instance.

        See the [wrapper's documentation](`IBLSSignatureCheckerInstance`) for more details.*/
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
        > IBLSSignatureCheckerInstance<T, P, N>
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
        > IBLSSignatureCheckerInstance<T, P, N>
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
    ///0x610100604052348015610010575f5ffd5b5060405161280c38038061280c83398101604081905261002f916101c4565b6001600160a01b038116608081905260408051636830483560e01b815290516368304835916004808201926020929091908290030181865afa158015610077573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061009b91906101c4565b6001600160a01b031660a0816001600160a01b031681525050806001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100f0573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061011491906101c4565b6001600160a01b031660c0816001600160a01b03168152505060a0516001600160a01b031663df5cf7236040518163ffffffff1660e01b8152600401602060405180830381865afa15801561016b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061018f91906101c4565b6001600160a01b031660e052505f805460ff191660011790556101e6565b6001600160a01b03811681146101c1575f5ffd5b50565b5f602082840312156101d4575f5ffd5b81516101df816101ad565b9392505050565b60805160a05160c05160e0516125bc6102505f395f818161019c0152610b8301525f818160d20152610d5f01525f818161011101528181610f3201526110e701525f81816101380152818161034201528181610864015281816109f50152610c1d01526125bc5ff3fe608060405234801561000f575f5ffd5b5060043610610085575f3560e01c80636d14a987116100585780636d14a987146101335780636efb46361461015a578063b98d09081461017b578063df5cf72314610197575f5ffd5b8063171f1d5b14610089578063416c7e5e146100b85780635df45946146100cd578063683048351461010c575b5f5ffd5b61009c610097366004611efe565b6101be565b6040805192151583529015156020830152015b60405180910390f35b6100cb6100c6366004611f4c565b610340565b005b6100f47f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100af565b6100f47f000000000000000000000000000000000000000000000000000000000000000081565b6100f47f000000000000000000000000000000000000000000000000000000000000000081565b61016d610168366004612232565b6104b7565b6040516100af929190612324565b5f546101879060ff1681565b60405190151581526020016100af565b6100f47f000000000000000000000000000000000000000000000000000000000000000081565b5f5f5f7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187875f01518860200151885f01515f600281106102015761020161236c565b60200201518951600160200201518a602001515f600281106102255761022561236c565b60200201518b602001516001600281106102415761024161236c565b602090810291909101518c518d83015160405161029e9a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b604051602081830303815290604052805190602001205f1c6102c09190612380565b90506103326102d96102d28884611381565b8690611410565b6102e16114a3565b610328610319856103136040805180820182525f80825260209182015281518083019092526001825260029082015290565b90611381565b6103228c611563565b90611410565b886201d4c06115ed565b909890975095505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561039c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103c0919061239f565b6001600160a01b0316336001600160a01b0316146104715760405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a4015b60405180910390fd5b5f805460ff19168215159081179091556040519081527f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc9060200160405180910390a150565b60408051808201909152606080825260208201525f84810361052e5760405162461bcd60e51b815260206004820152603760248201525f5160206125675f395f51905f5260448201527f7265733a20656d7074792071756f72756d20696e7075740000000000000000006064820152608401610468565b60408301515185148015610546575060a08301515185145b8015610556575060c08301515185145b8015610566575060e08301515185145b6105cf5760405162461bcd60e51b815260206004820152604160248201525f5160206125675f395f51905f5260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a401610468565b825151602084015151146106465760405162461bcd60e51b8152602060048201526044602482018190525f5160206125675f395f51905f52908201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b608482015260a401610468565b4363ffffffff168463ffffffff16106106b45760405162461bcd60e51b815260206004820152603c60248201525f5160206125675f395f51905f5260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608401610468565b6040805180820182525f808252602080830191909152825180840190935260608084529083015290866001600160401b038111156106f4576106f4611db5565b60405190808252806020026020018201604052801561071d578160200160208202803683370190505b506020820152866001600160401b0381111561073b5761073b611db5565b604051908082528060200260200182016040528015610764578160200160208202803683370190505b50815260408051808201909152606080825260208201528560200151516001600160401b0381111561079857610798611db5565b6040519080825280602002602001820160405280156107c1578160200160208202803683370190505b5081526020860151516001600160401b038111156107e1576107e1611db5565b60405190808252806020026020018201604052801561080a578160200160208202803683370190505b5081602001819052505f6108d88a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505060408051639aa1653d60e01b815290516001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169350639aa1653d925060048083019260209291908290030181865afa1580156108af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108d391906123c5565b611801565b90505f5b876020015151811015610b5f57610920886020015182815181106109025761090261236c565b602002602001015180515f9081526020918201519091526040902090565b836020015182815181106109365761093661236c565b602090810291909101015280156109f35760208301516109576001836123f9565b815181106109675761096761236c565b60200260200101515f1c836020015182815181106109875761098761236c565b60200260200101515f1c116109f3576040805162461bcd60e51b81526020600482015260248101919091525f5160206125675f395f51905f5260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152608401610468565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166304ec635184602001518381518110610a3857610a3861236c565b60200260200101518b8b5f01518581518110610a5657610a5661236c565b60200260200101516040518463ffffffff1660e01b8152600401610a939392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa158015610aae573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ad2919061240c565b6001600160c01b0316835f01518281518110610af057610af061236c565b602002602001018181525050610b556102d2610b2984865f01518581518110610b1b57610b1b61236c565b602002602001015116611893565b8a602001518481518110610b3f57610b3f61236c565b60200260200101516118bd90919063ffffffff16565b94506001016108dc565b5050610b6a8361199e565b5f805491945060ff9091169081610b81575f610c01565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c448feb86040518163ffffffff1660e01b8152600401602060405180830381865afa158015610bdd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c019190612432565b90505f5b8a811015611254578215610d5d578963ffffffff16827f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663249a0c428f8f86818110610c5c57610c5c61236c565b60405160e085901b6001600160e01b031916815292013560f81c600483015250602401602060405180830381865afa158015610c9a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cbe9190612432565b610cc89190612449565b11610d5d5760405162461bcd60e51b815260206004820152606660248201525f5160206125675f395f51905f5260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c401610468565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166368bccaac8d8d84818110610d9e57610d9e61236c565b9050013560f81c60f81b60f81c8c8c60a001518581518110610dc257610dc261236c565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015610e1c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e40919061245c565b6001600160401b031916610e638a6040015183815181106109025761090261236c565b67ffffffffffffffff191614610efe5760405162461bcd60e51b815260206004820152606160248201525f5160206125675f395f51905f5260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c401610468565b610f2e89604001518281518110610f1757610f1761236c565b60200260200101518761141090919063ffffffff16565b95507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c8294c568d8d84818110610f7157610f7161236c565b9050013560f81c60f81b60f81c8c8c60c001518581518110610f9557610f9561236c565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015610fef573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110139190612484565b856020015182815181106110295761102961236c565b6001600160601b039092166020928302919091018201528501518051829081106110555761105561236c565b6020026020010151855f015182815181106110725761107261236c565b6001600160601b03909216602092830291909101909101525f805b8a602001515181101561124a576110e0865f015182815181106110b2576110b261236c565b60200260200101518f8f868181106110cc576110cc61236c565b600192013560f81c9290921c811614919050565b15611242577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f2be94ae8f8f868181106111265761112661236c565b9050013560f81c60f81b60f81c8e8960200151858151811061114a5761114a61236c565b60200260200101518f60e0015188815181106111685761116861236c565b602002602001015187815181106111815761118161236c565b60209081029190910101516040516001600160e01b031960e087901b16815260ff909416600485015263ffffffff92831660248501526044840191909152166064820152608401602060405180830381865afa1580156111e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112079190612484565b875180518590811061121b5761121b61236c565b6020026020010181815161122f91906124aa565b6001600160601b03169052506001909101905b60010161108d565b5050600101610c05565b5050505f5f61126d8c868a606001518b608001516101be565b91509150816112dd5760405162461bcd60e51b815260206004820152604360248201525f5160206125675f395f51905f5260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a401610468565b8061133d5760405162461bcd60e51b815260206004820152603960248201525f5160206125675f395f51905f5260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608401610468565b50505f8782602001516040516020016113579291906124c9565b60408051808303601f190181529190528051602090910120929b929a509198505050505050505050565b604080518082019091525f808252602082015261139c611cdb565b835181526020808501519082015260408082018490525f908360608460076107d05a03fa905080806113ca57fe5b50806114085760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610468565b505092915050565b604080518082019091525f808252602082015261142b611cf9565b835181526020808501518183015283516040808401919091529084015160608301525f908360808460066107d05a03fa9050808061146557fe5b50806114085760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610468565b6114ab611d17565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b604080518082019091525f80825260208201525f80806115905f5160206125475f395f51905f5286612380565b90505b61159c81611a34565b90935091505f5160206125475f395f51905f5282830983036115d4576040805180820190915290815260208101919091529392505050565b5f5160206125475f395f51905f52600182089050611593565b6040805180820182528681526020808201869052825180840190935286835282018490525f9182919061161e611d3c565b5f5b60028110156117d5575f61163582600661250f565b90508482600281106116495761164961236c565b6020020151518361165a835f612449565b600c811061166a5761166a61236c565b60200201528482600281106116815761168161236c565b602002015160200151838260016116989190612449565b600c81106116a8576116a861236c565b60200201528382600281106116bf576116bf61236c565b60200201515151836116d2836002612449565b600c81106116e2576116e261236c565b60200201528382600281106116f9576116f961236c565b6020020151516001602002015183611712836003612449565b600c81106117225761172261236c565b60200201528382600281106117395761173961236c565b6020020151602001515f600281106117535761175361236c565b602002015183611764836004612449565b600c81106117745761177461236c565b602002015283826002811061178b5761178b61236c565b6020020151602001516001600281106117a6576117a661236c565b6020020151836117b7836005612449565b600c81106117c7576117c761236c565b602002015250600101611620565b506117de611d5b565b5f6020826101808560088cfa9151919c9115159b50909950505050505050505050565b5f5f61180c84611ab0565b9050808360ff166001901b1161188a5760405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608401610468565b90505b92915050565b5f805b821561188d576118a76001846123f9565b90921691806118b581612526565b915050611896565b604080518082019091525f80825260208201526102008261ffff16106119185760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606401610468565b8161ffff1660010361192b57508161188d565b604080518082019091525f8082526020820181905284906001905b8161ffff168661ffff161061199357600161ffff871660ff83161c81169003611976576119738484611410565b93505b6119808384611410565b92506201fffe600192831b169101611946565b509195945050505050565b604080518082019091525f808252602082015281511580156119c257506020820151155b156119df575050604080518082019091525f808252602082015290565b6040518060400160405280835f015181526020015f5160206125475f395f51905f528460200151611a109190612380565b611a27905f5160206125475f395f51905f526123f9565b905292915050565b919050565b5f80805f5160206125475f395f51905f5260035f5160206125475f395f51905f52865f5160206125475f395f51905f52888909090890505f611aa4827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f5160206125475f395f51905f52611c33565b91959194509092505050565b5f61010082511115611b385760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a401610468565b81515f03611b4757505f919050565b5f5f835f81518110611b5b57611b5b61236c565b0160200151600160f89190911c81901b92505b8451811015611c2a57848181518110611b8957611b8961236c565b0160200151600160f89190911c1b9150828211611c1e5760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a401610468565b91811791600101611b6e565b50909392505050565b5f5f611c3d611d5b565b611c45611d79565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280611c8257fe5b5082611cd05760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606401610468565b505195945050505050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280611d2a611d97565b8152602001611d37611d97565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b0381118282101715611deb57611deb611db5565b60405290565b60405161010081016001600160401b0381118282101715611deb57611deb611db5565b604051601f8201601f191681016001600160401b0381118282101715611e3c57611e3c611db5565b604052919050565b5f60408284031215611e54575f5ffd5b611e5c611dc9565b823581526020928301359281019290925250919050565b5f82601f830112611e82575f5ffd5b611e8a611dc9565b806040840185811115611e9b575f5ffd5b845b81811015611eb5578035845260209384019301611e9d565b509095945050505050565b5f60808284031215611ed0575f5ffd5b611ed8611dc9565b9050611ee48383611e73565b8152611ef38360408401611e73565b602082015292915050565b5f5f5f5f6101208587031215611f12575f5ffd5b84359350611f238660208701611e44565b9250611f328660608701611ec0565b9150611f418660e08701611e44565b905092959194509250565b5f60208284031215611f5c575f5ffd5b8135801515811461188a575f5ffd5b803563ffffffff81168114611a2f575f5ffd5b5f6001600160401b03821115611f9657611f96611db5565b5060051b60200190565b5f82601f830112611faf575f5ffd5b8135611fc2611fbd82611f7e565b611e14565b8082825260208201915060208360051b860101925085831115611fe3575f5ffd5b602085015b8381101561200757611ff981611f6b565b835260209283019201611fe8565b5095945050505050565b5f82601f830112612020575f5ffd5b813561202e611fbd82611f7e565b8082825260208201915060208360061b86010192508583111561204f575f5ffd5b602085015b83811015612007576120668782611e44565b8352602090920191604001612054565b5f82601f830112612085575f5ffd5b8135612093611fbd82611f7e565b8082825260208201915060208360051b8601019250858311156120b4575f5ffd5b602085015b838110156120075780356001600160401b038111156120d6575f5ffd5b6120e5886020838a0101611fa0565b845250602092830192016120b9565b5f6101808284031215612105575f5ffd5b61210d611df1565b905081356001600160401b03811115612124575f5ffd5b61213084828501611fa0565b82525060208201356001600160401b0381111561214b575f5ffd5b61215784828501612011565b60208301525060408201356001600160401b03811115612175575f5ffd5b61218184828501612011565b6040830152506121948360608401611ec0565b60608201526121a68360e08401611e44565b60808201526101208201356001600160401b038111156121c4575f5ffd5b6121d084828501611fa0565b60a0830152506101408201356001600160401b038111156121ef575f5ffd5b6121fb84828501611fa0565b60c0830152506101608201356001600160401b0381111561221a575f5ffd5b61222684828501612076565b60e08301525092915050565b5f5f5f5f5f60808688031215612246575f5ffd5b8535945060208601356001600160401b03811115612262575f5ffd5b8601601f81018813612272575f5ffd5b80356001600160401b03811115612287575f5ffd5b886020828401011115612298575f5ffd5b602091909101945092506122ae60408701611f6b565b915060608601356001600160401b038111156122c8575f5ffd5b6122d4888289016120f4565b9150509295509295909350565b5f8151808452602084019350602083015f5b8281101561231a5781516001600160601b03168652602095860195909101906001016122f3565b5093949350505050565b604081525f835160408084015261233e60808401826122e1565b90506020850151603f1984830301606085015261235b82826122e1565b925050508260208301529392505050565b634e487b7160e01b5f52603260045260245ffd5b5f8261239a57634e487b7160e01b5f52601260045260245ffd5b500690565b5f602082840312156123af575f5ffd5b81516001600160a01b038116811461188a575f5ffd5b5f602082840312156123d5575f5ffd5b815160ff8116811461188a575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561188d5761188d6123e5565b5f6020828403121561241c575f5ffd5b81516001600160c01b038116811461188a575f5ffd5b5f60208284031215612442575f5ffd5b5051919050565b8082018082111561188d5761188d6123e5565b5f6020828403121561246c575f5ffd5b815167ffffffffffffffff198116811461188a575f5ffd5b5f60208284031215612494575f5ffd5b81516001600160601b038116811461188a575f5ffd5b6001600160601b03828116828216039081111561188d5761188d6123e5565b63ffffffff60e01b8360e01b1681525f600482018351602085015f5b828110156125035781518452602093840193909101906001016124e5565b50919695505050505050565b808202811582820484141761188d5761188d6123e5565b5f61ffff821661ffff810361253d5761253d6123e5565b6001019291505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a264697066735822122082025ed7651bb995a6a4e70722cd87a3383217335e0dc1e57ed8944f9dfed39d64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@Qa(\x0C8\x03\x80a(\x0C\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xC4V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80\x81\x90R`@\x80Qch0H5`\xE0\x1B\x81R\x90Qch0H5\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\0wW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\x9B\x91\x90a\x01\xC4V[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x14\x91\x90a\x01\xC4V[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP`\xA0Q`\x01`\x01`\xA0\x1B\x03\x16c\xDF\\\xF7#`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01kW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x8F\x91\x90a\x01\xC4V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0RP_\x80T`\xFF\x19\x16`\x01\x17\x90Ua\x01\xE6V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xC1W__\xFD[PV[_` \x82\x84\x03\x12\x15a\x01\xD4W__\xFD[\x81Qa\x01\xDF\x81a\x01\xADV[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa%\xBCa\x02P_9_\x81\x81a\x01\x9C\x01Ra\x0B\x83\x01R_\x81\x81`\xD2\x01Ra\r_\x01R_\x81\x81a\x01\x11\x01R\x81\x81a\x0F2\x01Ra\x10\xE7\x01R_\x81\x81a\x018\x01R\x81\x81a\x03B\x01R\x81\x81a\x08d\x01R\x81\x81a\t\xF5\x01Ra\x0C\x1D\x01Ra%\xBC_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x85W_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0XW\x80cm\x14\xA9\x87\x14a\x013W\x80cn\xFBF6\x14a\x01ZW\x80c\xB9\x8D\t\x08\x14a\x01{W\x80c\xDF\\\xF7#\x14a\x01\x97W__\xFD[\x80c\x17\x1F\x1D[\x14a\0\x89W\x80cAl~^\x14a\0\xB8W\x80c]\xF4YF\x14a\0\xCDW\x80ch0H5\x14a\x01\x0CW[__\xFD[a\0\x9Ca\0\x976`\x04a\x1E\xFEV[a\x01\xBEV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCBa\0\xC66`\x04a\x1FLV[a\x03@V[\0[a\0\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xAFV[a\0\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01ma\x01h6`\x04a\"2V[a\x04\xB7V[`@Qa\0\xAF\x92\x91\x90a#$V[_Ta\x01\x87\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xAFV[a\0\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[___\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87_\x01Q\x88` \x01Q\x88_\x01Q_`\x02\x81\x10a\x02\x01Wa\x02\x01a#lV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q_`\x02\x81\x10a\x02%Wa\x02%a#lV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x02AWa\x02Aa#lV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x02\x9E\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x02\xC0\x91\x90a#\x80V[\x90Pa\x032a\x02\xD9a\x02\xD2\x88\x84a\x13\x81V[\x86\x90a\x14\x10V[a\x02\xE1a\x14\xA3V[a\x03(a\x03\x19\x85a\x03\x13`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\x13\x81V[a\x03\"\x8Ca\x15cV[\x90a\x14\x10V[\x88b\x01\xD4\xC0a\x15\xEDV[\x90\x98\x90\x97P\x95PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x9CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xC0\x91\x90a#\x9FV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R_\x84\x81\x03a\x05.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04hV[`@\x83\x01QQ\x85\x14\x80\x15a\x05FWP`\xA0\x83\x01QQ\x85\x14[\x80\x15a\x05VWP`\xC0\x83\x01QQ\x85\x14[\x80\x15a\x05fWP`\xE0\x83\x01QQ\x85\x14[a\x05\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x01a\x04hV[\x82QQ` \x84\x01QQ\x14a\x06FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R_Q` a%g_9_Q\x90_R\x90\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x04hV[Cc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10a\x06\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x01a\x04hV[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R``\x80\x84R\x90\x83\x01R\x90\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xF4Wa\x06\xF4a\x1D\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x1DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07;Wa\x07;a\x1D\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07dW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x85` \x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x98Wa\x07\x98a\x1D\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xC1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x86\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xE1Wa\x07\xE1a\x1D\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\nW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81` \x01\x81\x90RP_a\x08\xD8\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Qc\x9A\xA1e=`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93Pc\x9A\xA1e=\x92P`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD3\x91\x90a#\xC5V[a\x18\x01V[\x90P_[\x87` \x01QQ\x81\x10\x15a\x0B_Wa\t \x88` \x01Q\x82\x81Q\x81\x10a\t\x02Wa\t\x02a#lV[` \x02` \x01\x01Q\x80Q_\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83` \x01Q\x82\x81Q\x81\x10a\t6Wa\t6a#lV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\t\xF3W` \x83\x01Qa\tW`\x01\x83a#\xF9V[\x81Q\x81\x10a\tgWa\tga#lV[` \x02` \x01\x01Q_\x1C\x83` \x01Q\x82\x81Q\x81\x10a\t\x87Wa\t\x87a#lV[` \x02` \x01\x01Q_\x1C\x11a\t\xF3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x04hV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x84` \x01Q\x83\x81Q\x81\x10a\n8Wa\n8a#lV[` \x02` \x01\x01Q\x8B\x8B_\x01Q\x85\x81Q\x81\x10a\nVWa\nVa#lV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x93\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xAEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xD2\x91\x90a$\x0CV[`\x01`\x01`\xC0\x1B\x03\x16\x83_\x01Q\x82\x81Q\x81\x10a\n\xF0Wa\n\xF0a#lV[` \x02` \x01\x01\x81\x81RPPa\x0BUa\x02\xD2a\x0B)\x84\x86_\x01Q\x85\x81Q\x81\x10a\x0B\x1BWa\x0B\x1Ba#lV[` \x02` \x01\x01Q\x16a\x18\x93V[\x8A` \x01Q\x84\x81Q\x81\x10a\x0B?Wa\x0B?a#lV[` \x02` \x01\x01Qa\x18\xBD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P`\x01\x01a\x08\xDCV[PPa\x0Bj\x83a\x19\x9EV[_\x80T\x91\x94P`\xFF\x90\x91\x16\x90\x81a\x0B\x81W_a\x0C\x01V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4H\xFE\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x01\x91\x90a$2V[\x90P_[\x8A\x81\x10\x15a\x12TW\x82\x15a\r]W\x89c\xFF\xFF\xFF\xFF\x16\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c$\x9A\x0CB\x8F\x8F\x86\x81\x81\x10a\x0C\\Wa\x0C\\a#lV[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R\x92\x015`\xF8\x1C`\x04\x83\x01RP`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xBE\x91\x90a$2V[a\x0C\xC8\x91\x90a$IV[\x11a\r]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x01a\x04hV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ch\xBC\xCA\xAC\x8D\x8D\x84\x81\x81\x10a\r\x9EWa\r\x9Ea#lV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xA0\x01Q\x85\x81Q\x81\x10a\r\xC2Wa\r\xC2a#lV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E@\x91\x90a$\\V[`\x01`\x01`@\x1B\x03\x19\x16a\x0Ec\x8A`@\x01Q\x83\x81Q\x81\x10a\t\x02Wa\t\x02a#lV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\x0E\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x01a\x04hV[a\x0F.\x89`@\x01Q\x82\x81Q\x81\x10a\x0F\x17Wa\x0F\x17a#lV[` \x02` \x01\x01Q\x87a\x14\x10\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC8)LV\x8D\x8D\x84\x81\x81\x10a\x0FqWa\x0Fqa#lV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xC0\x01Q\x85\x81Q\x81\x10a\x0F\x95Wa\x0F\x95a#lV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x13\x91\x90a$\x84V[\x85` \x01Q\x82\x81Q\x81\x10a\x10)Wa\x10)a#lV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x85\x01Q\x80Q\x82\x90\x81\x10a\x10UWa\x10Ua#lV[` \x02` \x01\x01Q\x85_\x01Q\x82\x81Q\x81\x10a\x10rWa\x10ra#lV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R_\x80[\x8A` \x01QQ\x81\x10\x15a\x12JWa\x10\xE0\x86_\x01Q\x82\x81Q\x81\x10a\x10\xB2Wa\x10\xB2a#lV[` \x02` \x01\x01Q\x8F\x8F\x86\x81\x81\x10a\x10\xCCWa\x10\xCCa#lV[`\x01\x92\x015`\xF8\x1C\x92\x90\x92\x1C\x81\x16\x14\x91\x90PV[\x15a\x12BW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF2\xBE\x94\xAE\x8F\x8F\x86\x81\x81\x10a\x11&Wa\x11&a#lV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8E\x89` \x01Q\x85\x81Q\x81\x10a\x11JWa\x11Ja#lV[` \x02` \x01\x01Q\x8F`\xE0\x01Q\x88\x81Q\x81\x10a\x11hWa\x11ha#lV[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x11\x81Wa\x11\x81a#lV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\xFF\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x92\x83\x16`$\x85\x01R`D\x84\x01\x91\x90\x91R\x16`d\x82\x01R`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x07\x91\x90a$\x84V[\x87Q\x80Q\x85\x90\x81\x10a\x12\x1BWa\x12\x1Ba#lV[` \x02` \x01\x01\x81\x81Qa\x12/\x91\x90a$\xAAV[`\x01`\x01``\x1B\x03\x16\x90RP`\x01\x90\x91\x01\x90[`\x01\x01a\x10\x8DV[PP`\x01\x01a\x0C\x05V[PPP__a\x12m\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x01\xBEV[\x91P\x91P\x81a\x12\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x04hV[\x80a\x13=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04hV[PP_\x87\x82` \x01Q`@Q` \x01a\x13W\x92\x91\x90a$\xC9V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x9B\x92\x9AP\x91\x98PPPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x13\x9Ca\x1C\xDBV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R_\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x13\xCAW\xFE[P\x80a\x14\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04hV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x14+a\x1C\xF9V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R_\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x14eW\xFE[P\x80a\x14\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04hV[a\x14\xABa\x1D\x17V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a\x15\x90_Q` a%G_9_Q\x90_R\x86a#\x80V[\x90P[a\x15\x9C\x81a\x1A4V[\x90\x93P\x91P_Q` a%G_9_Q\x90_R\x82\x83\t\x83\x03a\x15\xD4W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` a%G_9_Q\x90_R`\x01\x82\x08\x90Pa\x15\x93V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R_\x91\x82\x91\x90a\x16\x1Ea\x1D<V[_[`\x02\x81\x10\x15a\x17\xD5W_a\x165\x82`\x06a%\x0FV[\x90P\x84\x82`\x02\x81\x10a\x16IWa\x16Ia#lV[` \x02\x01QQ\x83a\x16Z\x83_a$IV[`\x0C\x81\x10a\x16jWa\x16ja#lV[` \x02\x01R\x84\x82`\x02\x81\x10a\x16\x81Wa\x16\x81a#lV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x16\x98\x91\x90a$IV[`\x0C\x81\x10a\x16\xA8Wa\x16\xA8a#lV[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\xBFWa\x16\xBFa#lV[` \x02\x01QQQ\x83a\x16\xD2\x83`\x02a$IV[`\x0C\x81\x10a\x16\xE2Wa\x16\xE2a#lV[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\xF9Wa\x16\xF9a#lV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x17\x12\x83`\x03a$IV[`\x0C\x81\x10a\x17\"Wa\x17\"a#lV[` \x02\x01R\x83\x82`\x02\x81\x10a\x179Wa\x179a#lV[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x17SWa\x17Sa#lV[` \x02\x01Q\x83a\x17d\x83`\x04a$IV[`\x0C\x81\x10a\x17tWa\x17ta#lV[` \x02\x01R\x83\x82`\x02\x81\x10a\x17\x8BWa\x17\x8Ba#lV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x17\xA6Wa\x17\xA6a#lV[` \x02\x01Q\x83a\x17\xB7\x83`\x05a$IV[`\x0C\x81\x10a\x17\xC7Wa\x17\xC7a#lV[` \x02\x01RP`\x01\x01a\x16 V[Pa\x17\xDEa\x1D[V[_` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[__a\x18\x0C\x84a\x1A\xB0V[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x18\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\x04hV[\x90P[\x92\x91PPV[_\x80[\x82\x15a\x18\x8DWa\x18\xA7`\x01\x84a#\xF9V[\x90\x92\x16\x91\x80a\x18\xB5\x81a%&V[\x91PPa\x18\x96V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a\x19\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x04hV[\x81a\xFF\xFF\x16`\x01\x03a\x19+WP\x81a\x18\x8DV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a\x19\x93W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x90\x03a\x19vWa\x19s\x84\x84a\x14\x10V[\x93P[a\x19\x80\x83\x84a\x14\x10V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a\x19FV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x19\xC2WP` \x82\x01Q\x15[\x15a\x19\xDFWPP`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01_Q` a%G_9_Q\x90_R\x84` \x01Qa\x1A\x10\x91\x90a#\x80V[a\x1A'\x90_Q` a%G_9_Q\x90_Ra#\xF9V[\x90R\x92\x91PPV[\x91\x90PV[_\x80\x80_Q` a%G_9_Q\x90_R`\x03_Q` a%G_9_Q\x90_R\x86_Q` a%G_9_Q\x90_R\x88\x89\t\t\x08\x90P_a\x1A\xA4\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` a%G_9_Q\x90_Ra\x1C3V[\x91\x95\x91\x94P\x90\x92PPPV[_a\x01\0\x82Q\x11\x15a\x1B8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x04hV[\x81Q_\x03a\x1BGWP_\x91\x90PV[__\x83_\x81Q\x81\x10a\x1B[Wa\x1B[a#lV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a\x1C*W\x84\x81\x81Q\x81\x10a\x1B\x89Wa\x1B\x89a#lV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a\x1C\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x04hV[\x91\x81\x17\x91`\x01\x01a\x1BnV[P\x90\x93\x92PPPV[__a\x1C=a\x1D[V[a\x1CEa\x1DyV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a\x1C\x82W\xFE[P\x82a\x1C\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04hV[PQ\x95\x94PPPPPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x1D*a\x1D\x97V[\x81R` \x01a\x1D7a\x1D\x97V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\xEBWa\x1D\xEBa\x1D\xB5V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\xEBWa\x1D\xEBa\x1D\xB5V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E<Wa\x1E<a\x1D\xB5V[`@R\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x1ETW__\xFD[a\x1E\\a\x1D\xC9V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x1E\x82W__\xFD[a\x1E\x8Aa\x1D\xC9V[\x80`@\x84\x01\x85\x81\x11\x15a\x1E\x9BW__\xFD[\x84[\x81\x81\x10\x15a\x1E\xB5W\x805\x84R` \x93\x84\x01\x93\x01a\x1E\x9DV[P\x90\x95\x94PPPPPV[_`\x80\x82\x84\x03\x12\x15a\x1E\xD0W__\xFD[a\x1E\xD8a\x1D\xC9V[\x90Pa\x1E\xE4\x83\x83a\x1EsV[\x81Ra\x1E\xF3\x83`@\x84\x01a\x1EsV[` \x82\x01R\x92\x91PPV[____a\x01 \x85\x87\x03\x12\x15a\x1F\x12W__\xFD[\x845\x93Pa\x1F#\x86` \x87\x01a\x1EDV[\x92Pa\x1F2\x86``\x87\x01a\x1E\xC0V[\x91Pa\x1FA\x86`\xE0\x87\x01a\x1EDV[\x90P\x92\x95\x91\x94P\x92PV[_` \x82\x84\x03\x12\x15a\x1F\\W__\xFD[\x815\x80\x15\x15\x81\x14a\x18\x8AW__\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A/W__\xFD[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1F\x96Wa\x1F\x96a\x1D\xB5V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1F\xAFW__\xFD[\x815a\x1F\xC2a\x1F\xBD\x82a\x1F~V[a\x1E\x14V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x1F\xE3W__\xFD[` \x85\x01[\x83\x81\x10\x15a \x07Wa\x1F\xF9\x81a\x1FkV[\x83R` \x92\x83\x01\x92\x01a\x1F\xE8V[P\x95\x94PPPPPV[_\x82`\x1F\x83\x01\x12a  W__\xFD[\x815a .a\x1F\xBD\x82a\x1F~V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a OW__\xFD[` \x85\x01[\x83\x81\x10\x15a \x07Wa f\x87\x82a\x1EDV[\x83R` \x90\x92\x01\x91`@\x01a TV[_\x82`\x1F\x83\x01\x12a \x85W__\xFD[\x815a \x93a\x1F\xBD\x82a\x1F~V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a \xB4W__\xFD[` \x85\x01[\x83\x81\x10\x15a \x07W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a \xD6W__\xFD[a \xE5\x88` \x83\x8A\x01\x01a\x1F\xA0V[\x84RP` \x92\x83\x01\x92\x01a \xB9V[_a\x01\x80\x82\x84\x03\x12\x15a!\x05W__\xFD[a!\ra\x1D\xF1V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a!$W__\xFD[a!0\x84\x82\x85\x01a\x1F\xA0V[\x82RP` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!KW__\xFD[a!W\x84\x82\x85\x01a \x11V[` \x83\x01RP`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!uW__\xFD[a!\x81\x84\x82\x85\x01a \x11V[`@\x83\x01RPa!\x94\x83``\x84\x01a\x1E\xC0V[``\x82\x01Ra!\xA6\x83`\xE0\x84\x01a\x1EDV[`\x80\x82\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xC4W__\xFD[a!\xD0\x84\x82\x85\x01a\x1F\xA0V[`\xA0\x83\x01RPa\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xEFW__\xFD[a!\xFB\x84\x82\x85\x01a\x1F\xA0V[`\xC0\x83\x01RPa\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x1AW__\xFD[a\"&\x84\x82\x85\x01a vV[`\xE0\x83\x01RP\x92\x91PPV[_____`\x80\x86\x88\x03\x12\x15a\"FW__\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"bW__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\"rW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x87W__\xFD[\x88` \x82\x84\x01\x01\x11\x15a\"\x98W__\xFD[` \x91\x90\x91\x01\x94P\x92Pa\"\xAE`@\x87\x01a\x1FkV[\x91P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xC8W__\xFD[a\"\xD4\x88\x82\x89\x01a \xF4V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a#\x1AW\x81Q`\x01`\x01``\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\"\xF3V[P\x93\x94\x93PPPPV[`@\x81R_\x83Q`@\x80\x84\x01Ra#>`\x80\x84\x01\x82a\"\xE1V[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01Ra#[\x82\x82a\"\xE1V[\x92PPP\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x82a#\x9AWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[_` \x82\x84\x03\x12\x15a#\xAFW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\x8AW__\xFD[_` \x82\x84\x03\x12\x15a#\xD5W__\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x18\x8AW__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x18\x8DWa\x18\x8Da#\xE5V[_` \x82\x84\x03\x12\x15a$\x1CW__\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x18\x8AW__\xFD[_` \x82\x84\x03\x12\x15a$BW__\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x18\x8DWa\x18\x8Da#\xE5V[_` \x82\x84\x03\x12\x15a$lW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a\x18\x8AW__\xFD[_` \x82\x84\x03\x12\x15a$\x94W__\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x18\x8AW__\xFD[`\x01`\x01``\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x18\x8DWa\x18\x8Da#\xE5V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x83`\xE0\x1B\x16\x81R_`\x04\x82\x01\x83Q` \x85\x01_[\x82\x81\x10\x15a%\x03W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a$\xE5V[P\x91\x96\x95PPPPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x18\x8DWa\x18\x8Da#\xE5V[_a\xFF\xFF\x82\x16a\xFF\xFF\x81\x03a%=Wa%=a#\xE5V[`\x01\x01\x92\x91PPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \x82\x02^\xD7e\x1B\xB9\x95\xA6\xA4\xE7\x07\"\xCD\x87\xA382\x173^\r\xC1\xE5~\xD8\x94O\x9D\xFE\xD3\x9DdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610085575f3560e01c80636d14a987116100585780636d14a987146101335780636efb46361461015a578063b98d09081461017b578063df5cf72314610197575f5ffd5b8063171f1d5b14610089578063416c7e5e146100b85780635df45946146100cd578063683048351461010c575b5f5ffd5b61009c610097366004611efe565b6101be565b6040805192151583529015156020830152015b60405180910390f35b6100cb6100c6366004611f4c565b610340565b005b6100f47f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100af565b6100f47f000000000000000000000000000000000000000000000000000000000000000081565b6100f47f000000000000000000000000000000000000000000000000000000000000000081565b61016d610168366004612232565b6104b7565b6040516100af929190612324565b5f546101879060ff1681565b60405190151581526020016100af565b6100f47f000000000000000000000000000000000000000000000000000000000000000081565b5f5f5f7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187875f01518860200151885f01515f600281106102015761020161236c565b60200201518951600160200201518a602001515f600281106102255761022561236c565b60200201518b602001516001600281106102415761024161236c565b602090810291909101518c518d83015160405161029e9a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b604051602081830303815290604052805190602001205f1c6102c09190612380565b90506103326102d96102d28884611381565b8690611410565b6102e16114a3565b610328610319856103136040805180820182525f80825260209182015281518083019092526001825260029082015290565b90611381565b6103228c611563565b90611410565b886201d4c06115ed565b909890975095505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561039c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103c0919061239f565b6001600160a01b0316336001600160a01b0316146104715760405162461bcd60e51b815260206004820152605c60248201527f424c535369676e6174757265436865636b65722e6f6e6c79436f6f7264696e6160448201527f746f724f776e65723a2063616c6c6572206973206e6f7420746865206f776e6560648201527f72206f6620746865207265676973747279436f6f7264696e61746f7200000000608482015260a4015b60405180910390fd5b5f805460ff19168215159081179091556040519081527f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc9060200160405180910390a150565b60408051808201909152606080825260208201525f84810361052e5760405162461bcd60e51b815260206004820152603760248201525f5160206125675f395f51905f5260448201527f7265733a20656d7074792071756f72756d20696e7075740000000000000000006064820152608401610468565b60408301515185148015610546575060a08301515185145b8015610556575060c08301515185145b8015610566575060e08301515185145b6105cf5760405162461bcd60e51b815260206004820152604160248201525f5160206125675f395f51905f5260448201527f7265733a20696e7075742071756f72756d206c656e677468206d69736d6174636064820152600d60fb1b608482015260a401610468565b825151602084015151146106465760405162461bcd60e51b8152602060048201526044602482018190525f5160206125675f395f51905f52908201527f7265733a20696e707574206e6f6e7369676e6572206c656e677468206d69736d6064820152630c2e8c6d60e31b608482015260a401610468565b4363ffffffff168463ffffffff16106106b45760405162461bcd60e51b815260206004820152603c60248201525f5160206125675f395f51905f5260448201527f7265733a20696e76616c6964207265666572656e636520626c6f636b000000006064820152608401610468565b6040805180820182525f808252602080830191909152825180840190935260608084529083015290866001600160401b038111156106f4576106f4611db5565b60405190808252806020026020018201604052801561071d578160200160208202803683370190505b506020820152866001600160401b0381111561073b5761073b611db5565b604051908082528060200260200182016040528015610764578160200160208202803683370190505b50815260408051808201909152606080825260208201528560200151516001600160401b0381111561079857610798611db5565b6040519080825280602002602001820160405280156107c1578160200160208202803683370190505b5081526020860151516001600160401b038111156107e1576107e1611db5565b60405190808252806020026020018201604052801561080a578160200160208202803683370190505b5081602001819052505f6108d88a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505060408051639aa1653d60e01b815290516001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169350639aa1653d925060048083019260209291908290030181865afa1580156108af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108d391906123c5565b611801565b90505f5b876020015151811015610b5f57610920886020015182815181106109025761090261236c565b602002602001015180515f9081526020918201519091526040902090565b836020015182815181106109365761093661236c565b602090810291909101015280156109f35760208301516109576001836123f9565b815181106109675761096761236c565b60200260200101515f1c836020015182815181106109875761098761236c565b60200260200101515f1c116109f3576040805162461bcd60e51b81526020600482015260248101919091525f5160206125675f395f51905f5260448201527f7265733a206e6f6e5369676e65725075626b657973206e6f7420736f727465646064820152608401610468565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166304ec635184602001518381518110610a3857610a3861236c565b60200260200101518b8b5f01518581518110610a5657610a5661236c565b60200260200101516040518463ffffffff1660e01b8152600401610a939392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa158015610aae573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ad2919061240c565b6001600160c01b0316835f01518281518110610af057610af061236c565b602002602001018181525050610b556102d2610b2984865f01518581518110610b1b57610b1b61236c565b602002602001015116611893565b8a602001518481518110610b3f57610b3f61236c565b60200260200101516118bd90919063ffffffff16565b94506001016108dc565b5050610b6a8361199e565b5f805491945060ff9091169081610b81575f610c01565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c448feb86040518163ffffffff1660e01b8152600401602060405180830381865afa158015610bdd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c019190612432565b90505f5b8a811015611254578215610d5d578963ffffffff16827f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663249a0c428f8f86818110610c5c57610c5c61236c565b60405160e085901b6001600160e01b031916815292013560f81c600483015250602401602060405180830381865afa158015610c9a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cbe9190612432565b610cc89190612449565b11610d5d5760405162461bcd60e51b815260206004820152606660248201525f5160206125675f395f51905f5260448201527f7265733a205374616b6552656769737472792075706461746573206d7573742060648201527f62652077697468696e207769746864726177616c44656c6179426c6f636b732060848201526577696e646f7760d01b60a482015260c401610468565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166368bccaac8d8d84818110610d9e57610d9e61236c565b9050013560f81c60f81b60f81c8c8c60a001518581518110610dc257610dc261236c565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015610e1c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e40919061245c565b6001600160401b031916610e638a6040015183815181106109025761090261236c565b67ffffffffffffffff191614610efe5760405162461bcd60e51b815260206004820152606160248201525f5160206125675f395f51905f5260448201527f7265733a2071756f72756d41706b206861736820696e2073746f72616765206460648201527f6f6573206e6f74206d617463682070726f76696465642071756f72756d2061706084820152606b60f81b60a482015260c401610468565b610f2e89604001518281518110610f1757610f1761236c565b60200260200101518761141090919063ffffffff16565b95507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c8294c568d8d84818110610f7157610f7161236c565b9050013560f81c60f81b60f81c8c8c60c001518581518110610f9557610f9561236c565b60209081029190910101516040516001600160e01b031960e086901b16815260ff909316600484015263ffffffff9182166024840152166044820152606401602060405180830381865afa158015610fef573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110139190612484565b856020015182815181106110295761102961236c565b6001600160601b039092166020928302919091018201528501518051829081106110555761105561236c565b6020026020010151855f015182815181106110725761107261236c565b6001600160601b03909216602092830291909101909101525f805b8a602001515181101561124a576110e0865f015182815181106110b2576110b261236c565b60200260200101518f8f868181106110cc576110cc61236c565b600192013560f81c9290921c811614919050565b15611242577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f2be94ae8f8f868181106111265761112661236c565b9050013560f81c60f81b60f81c8e8960200151858151811061114a5761114a61236c565b60200260200101518f60e0015188815181106111685761116861236c565b602002602001015187815181106111815761118161236c565b60209081029190910101516040516001600160e01b031960e087901b16815260ff909416600485015263ffffffff92831660248501526044840191909152166064820152608401602060405180830381865afa1580156111e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112079190612484565b875180518590811061121b5761121b61236c565b6020026020010181815161122f91906124aa565b6001600160601b03169052506001909101905b60010161108d565b5050600101610c05565b5050505f5f61126d8c868a606001518b608001516101be565b91509150816112dd5760405162461bcd60e51b815260206004820152604360248201525f5160206125675f395f51905f5260448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a401610468565b8061133d5760405162461bcd60e51b815260206004820152603960248201525f5160206125675f395f51905f5260448201527f7265733a207369676e617475726520697320696e76616c6964000000000000006064820152608401610468565b50505f8782602001516040516020016113579291906124c9565b60408051808303601f190181529190528051602090910120929b929a509198505050505050505050565b604080518082019091525f808252602082015261139c611cdb565b835181526020808501519082015260408082018490525f908360608460076107d05a03fa905080806113ca57fe5b50806114085760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b6044820152606401610468565b505092915050565b604080518082019091525f808252602082015261142b611cf9565b835181526020808501518183015283516040808401919091529084015160608301525f908360808460066107d05a03fa9050808061146557fe5b50806114085760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b6044820152606401610468565b6114ab611d17565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b604080518082019091525f80825260208201525f80806115905f5160206125475f395f51905f5286612380565b90505b61159c81611a34565b90935091505f5160206125475f395f51905f5282830983036115d4576040805180820190915290815260208101919091529392505050565b5f5160206125475f395f51905f52600182089050611593565b6040805180820182528681526020808201869052825180840190935286835282018490525f9182919061161e611d3c565b5f5b60028110156117d5575f61163582600661250f565b90508482600281106116495761164961236c565b6020020151518361165a835f612449565b600c811061166a5761166a61236c565b60200201528482600281106116815761168161236c565b602002015160200151838260016116989190612449565b600c81106116a8576116a861236c565b60200201528382600281106116bf576116bf61236c565b60200201515151836116d2836002612449565b600c81106116e2576116e261236c565b60200201528382600281106116f9576116f961236c565b6020020151516001602002015183611712836003612449565b600c81106117225761172261236c565b60200201528382600281106117395761173961236c565b6020020151602001515f600281106117535761175361236c565b602002015183611764836004612449565b600c81106117745761177461236c565b602002015283826002811061178b5761178b61236c565b6020020151602001516001600281106117a6576117a661236c565b6020020151836117b7836005612449565b600c81106117c7576117c761236c565b602002015250600101611620565b506117de611d5b565b5f6020826101808560088cfa9151919c9115159b50909950505050505050505050565b5f5f61180c84611ab0565b9050808360ff166001901b1161188a5760405162461bcd60e51b815260206004820152603f60248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206269746d61702065786365656473206d61782076616c7565006064820152608401610468565b90505b92915050565b5f805b821561188d576118a76001846123f9565b90921691806118b581612526565b915050611896565b604080518082019091525f80825260208201526102008261ffff16106119185760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b6044820152606401610468565b8161ffff1660010361192b57508161188d565b604080518082019091525f8082526020820181905284906001905b8161ffff168661ffff161061199357600161ffff871660ff83161c81169003611976576119738484611410565b93505b6119808384611410565b92506201fffe600192831b169101611946565b509195945050505050565b604080518082019091525f808252602082015281511580156119c257506020820151155b156119df575050604080518082019091525f808252602082015290565b6040518060400160405280835f015181526020015f5160206125475f395f51905f528460200151611a109190612380565b611a27905f5160206125475f395f51905f526123f9565b905292915050565b919050565b5f80805f5160206125475f395f51905f5260035f5160206125475f395f51905f52865f5160206125475f395f51905f52888909090890505f611aa4827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f5160206125475f395f51905f52611c33565b91959194509092505050565b5f61010082511115611b385760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a401610468565b81515f03611b4757505f919050565b5f5f835f81518110611b5b57611b5b61236c565b0160200151600160f89190911c81901b92505b8451811015611c2a57848181518110611b8957611b8961236c565b0160200151600160f89190911c1b9150828211611c1e5760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a401610468565b91811791600101611b6e565b50909392505050565b5f5f611c3d611d5b565b611c45611d79565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280611c8257fe5b5082611cd05760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c7572650000000000006044820152606401610468565b505195945050505050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280611d2a611d97565b8152602001611d37611d97565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b0381118282101715611deb57611deb611db5565b60405290565b60405161010081016001600160401b0381118282101715611deb57611deb611db5565b604051601f8201601f191681016001600160401b0381118282101715611e3c57611e3c611db5565b604052919050565b5f60408284031215611e54575f5ffd5b611e5c611dc9565b823581526020928301359281019290925250919050565b5f82601f830112611e82575f5ffd5b611e8a611dc9565b806040840185811115611e9b575f5ffd5b845b81811015611eb5578035845260209384019301611e9d565b509095945050505050565b5f60808284031215611ed0575f5ffd5b611ed8611dc9565b9050611ee48383611e73565b8152611ef38360408401611e73565b602082015292915050565b5f5f5f5f6101208587031215611f12575f5ffd5b84359350611f238660208701611e44565b9250611f328660608701611ec0565b9150611f418660e08701611e44565b905092959194509250565b5f60208284031215611f5c575f5ffd5b8135801515811461188a575f5ffd5b803563ffffffff81168114611a2f575f5ffd5b5f6001600160401b03821115611f9657611f96611db5565b5060051b60200190565b5f82601f830112611faf575f5ffd5b8135611fc2611fbd82611f7e565b611e14565b8082825260208201915060208360051b860101925085831115611fe3575f5ffd5b602085015b8381101561200757611ff981611f6b565b835260209283019201611fe8565b5095945050505050565b5f82601f830112612020575f5ffd5b813561202e611fbd82611f7e565b8082825260208201915060208360061b86010192508583111561204f575f5ffd5b602085015b83811015612007576120668782611e44565b8352602090920191604001612054565b5f82601f830112612085575f5ffd5b8135612093611fbd82611f7e565b8082825260208201915060208360051b8601019250858311156120b4575f5ffd5b602085015b838110156120075780356001600160401b038111156120d6575f5ffd5b6120e5886020838a0101611fa0565b845250602092830192016120b9565b5f6101808284031215612105575f5ffd5b61210d611df1565b905081356001600160401b03811115612124575f5ffd5b61213084828501611fa0565b82525060208201356001600160401b0381111561214b575f5ffd5b61215784828501612011565b60208301525060408201356001600160401b03811115612175575f5ffd5b61218184828501612011565b6040830152506121948360608401611ec0565b60608201526121a68360e08401611e44565b60808201526101208201356001600160401b038111156121c4575f5ffd5b6121d084828501611fa0565b60a0830152506101408201356001600160401b038111156121ef575f5ffd5b6121fb84828501611fa0565b60c0830152506101608201356001600160401b0381111561221a575f5ffd5b61222684828501612076565b60e08301525092915050565b5f5f5f5f5f60808688031215612246575f5ffd5b8535945060208601356001600160401b03811115612262575f5ffd5b8601601f81018813612272575f5ffd5b80356001600160401b03811115612287575f5ffd5b886020828401011115612298575f5ffd5b602091909101945092506122ae60408701611f6b565b915060608601356001600160401b038111156122c8575f5ffd5b6122d4888289016120f4565b9150509295509295909350565b5f8151808452602084019350602083015f5b8281101561231a5781516001600160601b03168652602095860195909101906001016122f3565b5093949350505050565b604081525f835160408084015261233e60808401826122e1565b90506020850151603f1984830301606085015261235b82826122e1565b925050508260208301529392505050565b634e487b7160e01b5f52603260045260245ffd5b5f8261239a57634e487b7160e01b5f52601260045260245ffd5b500690565b5f602082840312156123af575f5ffd5b81516001600160a01b038116811461188a575f5ffd5b5f602082840312156123d5575f5ffd5b815160ff8116811461188a575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561188d5761188d6123e5565b5f6020828403121561241c575f5ffd5b81516001600160c01b038116811461188a575f5ffd5b5f60208284031215612442575f5ffd5b5051919050565b8082018082111561188d5761188d6123e5565b5f6020828403121561246c575f5ffd5b815167ffffffffffffffff198116811461188a575f5ffd5b5f60208284031215612494575f5ffd5b81516001600160601b038116811461188a575f5ffd5b6001600160601b03828116828216039081111561188d5761188d6123e5565b63ffffffff60e01b8360e01b1681525f600482018351602085015f5b828110156125035781518452602093840193909101906001016124e5565b50919695505050505050565b808202811582820484141761188d5761188d6123e5565b5f61ffff821661ffff810361253d5761253d6123e5565b6001019291505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47424c535369676e6174757265436865636b65722e636865636b5369676e617475a264697066735822122082025ed7651bb995a6a4e70722cd87a3383217335e0dc1e57ed8944f9dfed39d64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x85W_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0XW\x80cm\x14\xA9\x87\x14a\x013W\x80cn\xFBF6\x14a\x01ZW\x80c\xB9\x8D\t\x08\x14a\x01{W\x80c\xDF\\\xF7#\x14a\x01\x97W__\xFD[\x80c\x17\x1F\x1D[\x14a\0\x89W\x80cAl~^\x14a\0\xB8W\x80c]\xF4YF\x14a\0\xCDW\x80ch0H5\x14a\x01\x0CW[__\xFD[a\0\x9Ca\0\x976`\x04a\x1E\xFEV[a\x01\xBEV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCBa\0\xC66`\x04a\x1FLV[a\x03@V[\0[a\0\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xAFV[a\0\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01ma\x01h6`\x04a\"2V[a\x04\xB7V[`@Qa\0\xAF\x92\x91\x90a#$V[_Ta\x01\x87\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xAFV[a\0\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[___\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87_\x01Q\x88` \x01Q\x88_\x01Q_`\x02\x81\x10a\x02\x01Wa\x02\x01a#lV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q_`\x02\x81\x10a\x02%Wa\x02%a#lV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x02AWa\x02Aa#lV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x02\x9E\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x02\xC0\x91\x90a#\x80V[\x90Pa\x032a\x02\xD9a\x02\xD2\x88\x84a\x13\x81V[\x86\x90a\x14\x10V[a\x02\xE1a\x14\xA3V[a\x03(a\x03\x19\x85a\x03\x13`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\x13\x81V[a\x03\"\x8Ca\x15cV[\x90a\x14\x10V[\x88b\x01\xD4\xC0a\x15\xEDV[\x90\x98\x90\x97P\x95PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x9CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xC0\x91\x90a#\x9FV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R_\x84\x81\x03a\x05.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04hV[`@\x83\x01QQ\x85\x14\x80\x15a\x05FWP`\xA0\x83\x01QQ\x85\x14[\x80\x15a\x05VWP`\xC0\x83\x01QQ\x85\x14[\x80\x15a\x05fWP`\xE0\x83\x01QQ\x85\x14[a\x05\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x01a\x04hV[\x82QQ` \x84\x01QQ\x14a\x06FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R_Q` a%g_9_Q\x90_R\x90\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x04hV[Cc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10a\x06\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x01a\x04hV[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R``\x80\x84R\x90\x83\x01R\x90\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xF4Wa\x06\xF4a\x1D\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x1DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07;Wa\x07;a\x1D\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07dW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x85` \x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x98Wa\x07\x98a\x1D\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xC1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x86\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xE1Wa\x07\xE1a\x1D\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\nW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81` \x01\x81\x90RP_a\x08\xD8\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Qc\x9A\xA1e=`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93Pc\x9A\xA1e=\x92P`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD3\x91\x90a#\xC5V[a\x18\x01V[\x90P_[\x87` \x01QQ\x81\x10\x15a\x0B_Wa\t \x88` \x01Q\x82\x81Q\x81\x10a\t\x02Wa\t\x02a#lV[` \x02` \x01\x01Q\x80Q_\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83` \x01Q\x82\x81Q\x81\x10a\t6Wa\t6a#lV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\t\xF3W` \x83\x01Qa\tW`\x01\x83a#\xF9V[\x81Q\x81\x10a\tgWa\tga#lV[` \x02` \x01\x01Q_\x1C\x83` \x01Q\x82\x81Q\x81\x10a\t\x87Wa\t\x87a#lV[` \x02` \x01\x01Q_\x1C\x11a\t\xF3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x04hV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x84` \x01Q\x83\x81Q\x81\x10a\n8Wa\n8a#lV[` \x02` \x01\x01Q\x8B\x8B_\x01Q\x85\x81Q\x81\x10a\nVWa\nVa#lV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x93\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xAEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xD2\x91\x90a$\x0CV[`\x01`\x01`\xC0\x1B\x03\x16\x83_\x01Q\x82\x81Q\x81\x10a\n\xF0Wa\n\xF0a#lV[` \x02` \x01\x01\x81\x81RPPa\x0BUa\x02\xD2a\x0B)\x84\x86_\x01Q\x85\x81Q\x81\x10a\x0B\x1BWa\x0B\x1Ba#lV[` \x02` \x01\x01Q\x16a\x18\x93V[\x8A` \x01Q\x84\x81Q\x81\x10a\x0B?Wa\x0B?a#lV[` \x02` \x01\x01Qa\x18\xBD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P`\x01\x01a\x08\xDCV[PPa\x0Bj\x83a\x19\x9EV[_\x80T\x91\x94P`\xFF\x90\x91\x16\x90\x81a\x0B\x81W_a\x0C\x01V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4H\xFE\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x01\x91\x90a$2V[\x90P_[\x8A\x81\x10\x15a\x12TW\x82\x15a\r]W\x89c\xFF\xFF\xFF\xFF\x16\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c$\x9A\x0CB\x8F\x8F\x86\x81\x81\x10a\x0C\\Wa\x0C\\a#lV[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R\x92\x015`\xF8\x1C`\x04\x83\x01RP`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xBE\x91\x90a$2V[a\x0C\xC8\x91\x90a$IV[\x11a\r]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x01a\x04hV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ch\xBC\xCA\xAC\x8D\x8D\x84\x81\x81\x10a\r\x9EWa\r\x9Ea#lV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xA0\x01Q\x85\x81Q\x81\x10a\r\xC2Wa\r\xC2a#lV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E@\x91\x90a$\\V[`\x01`\x01`@\x1B\x03\x19\x16a\x0Ec\x8A`@\x01Q\x83\x81Q\x81\x10a\t\x02Wa\t\x02a#lV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\x0E\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x01a\x04hV[a\x0F.\x89`@\x01Q\x82\x81Q\x81\x10a\x0F\x17Wa\x0F\x17a#lV[` \x02` \x01\x01Q\x87a\x14\x10\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC8)LV\x8D\x8D\x84\x81\x81\x10a\x0FqWa\x0Fqa#lV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xC0\x01Q\x85\x81Q\x81\x10a\x0F\x95Wa\x0F\x95a#lV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x13\x91\x90a$\x84V[\x85` \x01Q\x82\x81Q\x81\x10a\x10)Wa\x10)a#lV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x85\x01Q\x80Q\x82\x90\x81\x10a\x10UWa\x10Ua#lV[` \x02` \x01\x01Q\x85_\x01Q\x82\x81Q\x81\x10a\x10rWa\x10ra#lV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R_\x80[\x8A` \x01QQ\x81\x10\x15a\x12JWa\x10\xE0\x86_\x01Q\x82\x81Q\x81\x10a\x10\xB2Wa\x10\xB2a#lV[` \x02` \x01\x01Q\x8F\x8F\x86\x81\x81\x10a\x10\xCCWa\x10\xCCa#lV[`\x01\x92\x015`\xF8\x1C\x92\x90\x92\x1C\x81\x16\x14\x91\x90PV[\x15a\x12BW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF2\xBE\x94\xAE\x8F\x8F\x86\x81\x81\x10a\x11&Wa\x11&a#lV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8E\x89` \x01Q\x85\x81Q\x81\x10a\x11JWa\x11Ja#lV[` \x02` \x01\x01Q\x8F`\xE0\x01Q\x88\x81Q\x81\x10a\x11hWa\x11ha#lV[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x11\x81Wa\x11\x81a#lV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\xFF\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x92\x83\x16`$\x85\x01R`D\x84\x01\x91\x90\x91R\x16`d\x82\x01R`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x07\x91\x90a$\x84V[\x87Q\x80Q\x85\x90\x81\x10a\x12\x1BWa\x12\x1Ba#lV[` \x02` \x01\x01\x81\x81Qa\x12/\x91\x90a$\xAAV[`\x01`\x01``\x1B\x03\x16\x90RP`\x01\x90\x91\x01\x90[`\x01\x01a\x10\x8DV[PP`\x01\x01a\x0C\x05V[PPP__a\x12m\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x01\xBEV[\x91P\x91P\x81a\x12\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x04hV[\x80a\x13=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R_Q` a%g_9_Q\x90_R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x04hV[PP_\x87\x82` \x01Q`@Q` \x01a\x13W\x92\x91\x90a$\xC9V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x9B\x92\x9AP\x91\x98PPPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x13\x9Ca\x1C\xDBV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R_\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x13\xCAW\xFE[P\x80a\x14\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04hV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x14+a\x1C\xF9V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R_\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x14eW\xFE[P\x80a\x14\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x04hV[a\x14\xABa\x1D\x17V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a\x15\x90_Q` a%G_9_Q\x90_R\x86a#\x80V[\x90P[a\x15\x9C\x81a\x1A4V[\x90\x93P\x91P_Q` a%G_9_Q\x90_R\x82\x83\t\x83\x03a\x15\xD4W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` a%G_9_Q\x90_R`\x01\x82\x08\x90Pa\x15\x93V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R_\x91\x82\x91\x90a\x16\x1Ea\x1D<V[_[`\x02\x81\x10\x15a\x17\xD5W_a\x165\x82`\x06a%\x0FV[\x90P\x84\x82`\x02\x81\x10a\x16IWa\x16Ia#lV[` \x02\x01QQ\x83a\x16Z\x83_a$IV[`\x0C\x81\x10a\x16jWa\x16ja#lV[` \x02\x01R\x84\x82`\x02\x81\x10a\x16\x81Wa\x16\x81a#lV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x16\x98\x91\x90a$IV[`\x0C\x81\x10a\x16\xA8Wa\x16\xA8a#lV[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\xBFWa\x16\xBFa#lV[` \x02\x01QQQ\x83a\x16\xD2\x83`\x02a$IV[`\x0C\x81\x10a\x16\xE2Wa\x16\xE2a#lV[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\xF9Wa\x16\xF9a#lV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x17\x12\x83`\x03a$IV[`\x0C\x81\x10a\x17\"Wa\x17\"a#lV[` \x02\x01R\x83\x82`\x02\x81\x10a\x179Wa\x179a#lV[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x17SWa\x17Sa#lV[` \x02\x01Q\x83a\x17d\x83`\x04a$IV[`\x0C\x81\x10a\x17tWa\x17ta#lV[` \x02\x01R\x83\x82`\x02\x81\x10a\x17\x8BWa\x17\x8Ba#lV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x17\xA6Wa\x17\xA6a#lV[` \x02\x01Q\x83a\x17\xB7\x83`\x05a$IV[`\x0C\x81\x10a\x17\xC7Wa\x17\xC7a#lV[` \x02\x01RP`\x01\x01a\x16 V[Pa\x17\xDEa\x1D[V[_` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[__a\x18\x0C\x84a\x1A\xB0V[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x18\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\x04hV[\x90P[\x92\x91PPV[_\x80[\x82\x15a\x18\x8DWa\x18\xA7`\x01\x84a#\xF9V[\x90\x92\x16\x91\x80a\x18\xB5\x81a%&V[\x91PPa\x18\x96V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a\x19\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x04hV[\x81a\xFF\xFF\x16`\x01\x03a\x19+WP\x81a\x18\x8DV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a\x19\x93W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x90\x03a\x19vWa\x19s\x84\x84a\x14\x10V[\x93P[a\x19\x80\x83\x84a\x14\x10V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a\x19FV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x19\xC2WP` \x82\x01Q\x15[\x15a\x19\xDFWPP`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01_Q` a%G_9_Q\x90_R\x84` \x01Qa\x1A\x10\x91\x90a#\x80V[a\x1A'\x90_Q` a%G_9_Q\x90_Ra#\xF9V[\x90R\x92\x91PPV[\x91\x90PV[_\x80\x80_Q` a%G_9_Q\x90_R`\x03_Q` a%G_9_Q\x90_R\x86_Q` a%G_9_Q\x90_R\x88\x89\t\t\x08\x90P_a\x1A\xA4\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` a%G_9_Q\x90_Ra\x1C3V[\x91\x95\x91\x94P\x90\x92PPPV[_a\x01\0\x82Q\x11\x15a\x1B8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x04hV[\x81Q_\x03a\x1BGWP_\x91\x90PV[__\x83_\x81Q\x81\x10a\x1B[Wa\x1B[a#lV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a\x1C*W\x84\x81\x81Q\x81\x10a\x1B\x89Wa\x1B\x89a#lV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a\x1C\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x04hV[\x91\x81\x17\x91`\x01\x01a\x1BnV[P\x90\x93\x92PPPV[__a\x1C=a\x1D[V[a\x1CEa\x1DyV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a\x1C\x82W\xFE[P\x82a\x1C\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04hV[PQ\x95\x94PPPPPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x1D*a\x1D\x97V[\x81R` \x01a\x1D7a\x1D\x97V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\xEBWa\x1D\xEBa\x1D\xB5V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\xEBWa\x1D\xEBa\x1D\xB5V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E<Wa\x1E<a\x1D\xB5V[`@R\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x1ETW__\xFD[a\x1E\\a\x1D\xC9V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x1E\x82W__\xFD[a\x1E\x8Aa\x1D\xC9V[\x80`@\x84\x01\x85\x81\x11\x15a\x1E\x9BW__\xFD[\x84[\x81\x81\x10\x15a\x1E\xB5W\x805\x84R` \x93\x84\x01\x93\x01a\x1E\x9DV[P\x90\x95\x94PPPPPV[_`\x80\x82\x84\x03\x12\x15a\x1E\xD0W__\xFD[a\x1E\xD8a\x1D\xC9V[\x90Pa\x1E\xE4\x83\x83a\x1EsV[\x81Ra\x1E\xF3\x83`@\x84\x01a\x1EsV[` \x82\x01R\x92\x91PPV[____a\x01 \x85\x87\x03\x12\x15a\x1F\x12W__\xFD[\x845\x93Pa\x1F#\x86` \x87\x01a\x1EDV[\x92Pa\x1F2\x86``\x87\x01a\x1E\xC0V[\x91Pa\x1FA\x86`\xE0\x87\x01a\x1EDV[\x90P\x92\x95\x91\x94P\x92PV[_` \x82\x84\x03\x12\x15a\x1F\\W__\xFD[\x815\x80\x15\x15\x81\x14a\x18\x8AW__\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A/W__\xFD[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1F\x96Wa\x1F\x96a\x1D\xB5V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1F\xAFW__\xFD[\x815a\x1F\xC2a\x1F\xBD\x82a\x1F~V[a\x1E\x14V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x1F\xE3W__\xFD[` \x85\x01[\x83\x81\x10\x15a \x07Wa\x1F\xF9\x81a\x1FkV[\x83R` \x92\x83\x01\x92\x01a\x1F\xE8V[P\x95\x94PPPPPV[_\x82`\x1F\x83\x01\x12a  W__\xFD[\x815a .a\x1F\xBD\x82a\x1F~V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a OW__\xFD[` \x85\x01[\x83\x81\x10\x15a \x07Wa f\x87\x82a\x1EDV[\x83R` \x90\x92\x01\x91`@\x01a TV[_\x82`\x1F\x83\x01\x12a \x85W__\xFD[\x815a \x93a\x1F\xBD\x82a\x1F~V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a \xB4W__\xFD[` \x85\x01[\x83\x81\x10\x15a \x07W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a \xD6W__\xFD[a \xE5\x88` \x83\x8A\x01\x01a\x1F\xA0V[\x84RP` \x92\x83\x01\x92\x01a \xB9V[_a\x01\x80\x82\x84\x03\x12\x15a!\x05W__\xFD[a!\ra\x1D\xF1V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a!$W__\xFD[a!0\x84\x82\x85\x01a\x1F\xA0V[\x82RP` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!KW__\xFD[a!W\x84\x82\x85\x01a \x11V[` \x83\x01RP`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!uW__\xFD[a!\x81\x84\x82\x85\x01a \x11V[`@\x83\x01RPa!\x94\x83``\x84\x01a\x1E\xC0V[``\x82\x01Ra!\xA6\x83`\xE0\x84\x01a\x1EDV[`\x80\x82\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xC4W__\xFD[a!\xD0\x84\x82\x85\x01a\x1F\xA0V[`\xA0\x83\x01RPa\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xEFW__\xFD[a!\xFB\x84\x82\x85\x01a\x1F\xA0V[`\xC0\x83\x01RPa\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x1AW__\xFD[a\"&\x84\x82\x85\x01a vV[`\xE0\x83\x01RP\x92\x91PPV[_____`\x80\x86\x88\x03\x12\x15a\"FW__\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"bW__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\"rW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x87W__\xFD[\x88` \x82\x84\x01\x01\x11\x15a\"\x98W__\xFD[` \x91\x90\x91\x01\x94P\x92Pa\"\xAE`@\x87\x01a\x1FkV[\x91P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xC8W__\xFD[a\"\xD4\x88\x82\x89\x01a \xF4V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a#\x1AW\x81Q`\x01`\x01``\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\"\xF3V[P\x93\x94\x93PPPPV[`@\x81R_\x83Q`@\x80\x84\x01Ra#>`\x80\x84\x01\x82a\"\xE1V[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01Ra#[\x82\x82a\"\xE1V[\x92PPP\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x82a#\x9AWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[_` \x82\x84\x03\x12\x15a#\xAFW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\x8AW__\xFD[_` \x82\x84\x03\x12\x15a#\xD5W__\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x18\x8AW__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x18\x8DWa\x18\x8Da#\xE5V[_` \x82\x84\x03\x12\x15a$\x1CW__\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x18\x8AW__\xFD[_` \x82\x84\x03\x12\x15a$BW__\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x18\x8DWa\x18\x8Da#\xE5V[_` \x82\x84\x03\x12\x15a$lW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a\x18\x8AW__\xFD[_` \x82\x84\x03\x12\x15a$\x94W__\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x18\x8AW__\xFD[`\x01`\x01``\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x18\x8DWa\x18\x8Da#\xE5V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x83`\xE0\x1B\x16\x81R_`\x04\x82\x01\x83Q` \x85\x01_[\x82\x81\x10\x15a%\x03W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a$\xE5V[P\x91\x96\x95PPPPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x18\x8DWa\x18\x8Da#\xE5V[_a\xFF\xFF\x82\x16a\xFF\xFF\x81\x03a%=Wa%=a#\xE5V[`\x01\x01\x92\x91PPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \x82\x02^\xD7e\x1B\xB9\x95\xA6\xA4\xE7\x07\"\xCD\x87\xA382\x173^\r\xC1\xE5~\xD8\x94O\x9D\xFE\xD3\x9DdsolcC\0\x08\x1B\x003",
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
                IBLSSignatureChecker::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkSignaturesReturn;
            type ReturnTuple<'a> = (
                IBLSSignatureChecker::QuorumStakeTotals,
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type Return = staleStakesForbiddenReturn;
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
                Self::delegation(_) => <delegationCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setStaleStakesForbidden(_) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR,
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
            )
                -> alloy_sol_types::Result<BLSSignatureCheckerCalls>] = &[
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(BLSSignatureCheckerCalls::delegation)
                    }
                    delegation
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
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
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
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
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
        pub const SELECTORS: &'static [[u8; 32usize]] = &[[
            64u8, 228u8, 237u8, 136u8, 10u8, 41u8, 224u8, 246u8, 221u8, 206u8, 48u8, 116u8, 87u8,
            251u8, 117u8, 205u8, 223u8, 79u8, 238u8, 247u8, 211u8, 236u8, 176u8, 48u8, 27u8, 253u8,
            244u8, 151u8, 106u8, 14u8, 45u8, 252u8,
        ]];
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
                Some(<StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StaleStakesForbiddenUpdate)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<BLSSignatureCheckerInstance<T, P, N>>>
    {
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
        BLSSignatureCheckerInstance::<T, P, N>::deploy_builder(provider, _registryCoordinator)
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
            f.debug_tuple("BLSSignatureCheckerInstance")
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
        > BLSSignatureCheckerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BLSSignatureChecker`](self) contract instance.

        See the [wrapper's documentation](`BLSSignatureCheckerInstance`) for more details.*/
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
        > BLSSignatureCheckerInstance<T, P, N>
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
            self.call_builder(&checkSignaturesCall {
                msgHash,
                quorumNumbers,
                referenceBlockNumber,
                params,
            })
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
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
            self.call_builder(&setStaleStakesForbiddenCall { value })
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, trySignatureAndApkVerificationCall, N> {
            self.call_builder(&trySignatureAndApkVerificationCall {
                msgHash,
                apk,
                apkG2,
                sigma,
            })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > BLSSignatureCheckerInstance<T, P, N>
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
        ///Creates a new event filter for the [`StaleStakesForbiddenUpdate`] event.
        pub fn StaleStakesForbiddenUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StaleStakesForbiddenUpdate, N> {
            self.event_filter::<StaleStakesForbiddenUpdate>()
        }
    }
}
