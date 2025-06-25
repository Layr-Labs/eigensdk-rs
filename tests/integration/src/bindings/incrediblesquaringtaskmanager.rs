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
library IIncredibleSquaringTaskManager {
  struct Task { uint256 numberToBeSquared; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
  struct TaskResponse { uint32 referenceTaskIndex; uint256 numberSquared; }
  struct TaskResponseMetadata { uint32 taskResponsedBlock; bytes32 hashOfNonSigners; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IIncredibleSquaringTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct Task { uint256 numberToBeSquared; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Debug, Clone)]
    pub struct Task {
        #[allow(missing_docs)]
        pub numberToBeSquared: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
                    value.numberToBeSquared,
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
                    numberToBeSquared: tuple.0,
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.numberToBeSquared,
                    ),
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
                  "Task(uint256 numberToBeSquared,uint32 taskCreatedBlock,bytes quorumNumbers,uint32 quorumThresholdPercentage)",
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
                      256,
                  > as alloy_sol_types::SolType>::eip712_data_word(
                          &self.numberToBeSquared,
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
                  + <alloy::sol_types::sol_data::Uint<
                      256,
                  > as alloy_sol_types::EventTopic>::topic_preimage_length(
                      &rust.numberToBeSquared,
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
                <alloy::sol_types::sol_data::Uint<
                  256,
              > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                  &rust.numberToBeSquared,
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
    /**```solidity
    struct TaskResponse { uint32 referenceTaskIndex; uint256 numberSquared; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TaskResponse {
        #[allow(missing_docs)]
        pub referenceTaskIndex: u32,
        #[allow(missing_docs)]
        pub numberSquared: alloy::sol_types::private::primitives::aliases::U256,
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
                (value.referenceTaskIndex, value.numberSquared)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskResponse {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    referenceTaskIndex: tuple.0,
                    numberSquared: tuple.1,
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
                        &self.numberSquared,
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
                    "TaskResponse(uint32 referenceTaskIndex,uint256 numberSquared)",
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
                  > as alloy_sol_types::SolType>::eip712_data_word(&self.numberSquared)
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
                      &rust.numberSquared,
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
                  &rust.numberSquared,
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
    struct TaskResponseMetadata { uint32 taskResponsedBlock; bytes32 hashOfNonSigners; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TaskResponseMetadata {
        #[allow(missing_docs)]
        pub taskResponsedBlock: u32,
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
                (value.taskResponsedBlock, value.hashOfNonSigners)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskResponseMetadata {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    taskResponsedBlock: tuple.0,
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
                  > as alloy_sol_types::SolType>::tokenize(&self.taskResponsedBlock),
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
                    "TaskResponseMetadata(uint32 taskResponsedBlock,bytes32 hashOfNonSigners)",
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
                          &self.taskResponsedBlock,
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
                      &rust.taskResponsedBlock,
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
                  &rust.taskResponsedBlock,
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
    /**Creates a new wrapper around an on-chain [`IIncredibleSquaringTaskManager`](self) contract instance.

    See the [wrapper's documentation](`IIncredibleSquaringTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IIncredibleSquaringTaskManagerInstance<T, P, N> {
        IIncredibleSquaringTaskManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IIncredibleSquaringTaskManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IIncredibleSquaringTaskManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IIncredibleSquaringTaskManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IIncredibleSquaringTaskManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IIncredibleSquaringTaskManagerInstance")
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
        > IIncredibleSquaringTaskManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IIncredibleSquaringTaskManager`](self) contract instance.

        See the [wrapper's documentation](`IIncredibleSquaringTaskManagerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IIncredibleSquaringTaskManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IIncredibleSquaringTaskManagerInstance<T, P, N> {
            IIncredibleSquaringTaskManagerInstance {
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
        > IIncredibleSquaringTaskManagerInstance<T, P, N>
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
        > IIncredibleSquaringTaskManagerInstance<T, P, N>
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
    pub struct OperatorStateRetrieverInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OperatorStateRetrieverInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OperatorStateRetrieverInstance")
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
        > OperatorStateRetrieverInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

        See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
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
        > OperatorStateRetrieverInstance<T, P, N>
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
        > OperatorStateRetrieverInstance<T, P, N>
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

library IIncredibleSquaringTaskManager {
  struct Task {
      uint256 numberToBeSquared;
      uint32 taskCreatedBlock;
      bytes quorumNumbers;
      uint32 quorumThresholdPercentage;
  }
  struct TaskResponse {
      uint32 referenceTaskIndex;
      uint256 numberSquared;
  }
  struct TaskResponseMetadata {
      uint32 taskResponsedBlock;
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

interface IncredibleSquaringTaskManager {
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
  event NewTaskCreated(uint32 indexed taskIndex, IIncredibleSquaringTaskManager.Task task);
  event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
  event Paused(address indexed account, uint256 newPausedStatus);
  event StaleStakesForbiddenUpdate(bool value);
  event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger);
  event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger);
  event TaskCompleted(uint32 indexed taskIndex);
  event TaskResponded(IIncredibleSquaringTaskManager.TaskResponse taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata taskResponseMetadata);
  event Unpaused(address indexed account, uint256 newPausedStatus);

  constructor(address _slashingRegistryCoordinator, address _pauserRegistry, uint32 _taskResponseWindowBlock);

  function TASK_CHALLENGE_WINDOW_BLOCK() external view returns (uint32);
  function TASK_RESPONSE_WINDOW_BLOCK() external view returns (uint32);
  function WADS_TO_SLASH() external view returns (uint256);
  function aggregator() external view returns (address);
  function allTaskHashes(uint32) external view returns (bytes32);
  function allTaskResponses(uint32) external view returns (bytes32);
  function allocationManager() external view returns (address);
  function blsApkRegistry() external view returns (address);
  function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureCheckerTypes.QuorumStakeTotals memory, bytes32);
  function createNewTask(uint256 numberToBeSquared, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
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
  function raiseAndResolveChallenge(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
  function registryCoordinator() external view returns (address);
  function renounceOwnership() external;
  function respondToTask(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
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
      "name": "_slashingRegistryCoordinator",
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
      "name": "numberToBeSquared",
      "type": "uint256",
      "internalType": "uint256"
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
      "internalType": "struct IIncredibleSquaringTaskManager.Task",
      "components": [
        {
          "name": "numberToBeSquared",
          "type": "uint256",
          "internalType": "uint256"
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
      "internalType": "struct IIncredibleSquaringTaskManager.TaskResponse",
      "components": [
        {
          "name": "referenceTaskIndex",
          "type": "uint32",
          "internalType": "uint32"
        },
        {
          "name": "numberSquared",
          "type": "uint256",
          "internalType": "uint256"
        }
      ]
    },
    {
      "name": "taskResponseMetadata",
      "type": "tuple",
      "internalType": "struct IIncredibleSquaringTaskManager.TaskResponseMetadata",
      "components": [
        {
          "name": "taskResponsedBlock",
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
      "internalType": "struct IIncredibleSquaringTaskManager.Task",
      "components": [
        {
          "name": "numberToBeSquared",
          "type": "uint256",
          "internalType": "uint256"
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
      "internalType": "struct IIncredibleSquaringTaskManager.TaskResponse",
      "components": [
        {
          "name": "referenceTaskIndex",
          "type": "uint32",
          "internalType": "uint32"
        },
        {
          "name": "numberSquared",
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
      "internalType": "struct IIncredibleSquaringTaskManager.Task",
      "components": [
        {
          "name": "numberToBeSquared",
          "type": "uint256",
          "internalType": "uint256"
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
      "internalType": "struct IIncredibleSquaringTaskManager.TaskResponse",
      "components": [
        {
          "name": "referenceTaskIndex",
          "type": "uint32",
          "internalType": "uint32"
        },
        {
          "name": "numberSquared",
          "type": "uint256",
          "internalType": "uint256"
        }
      ]
    },
    {
      "name": "taskResponseMetadata",
      "type": "tuple",
      "indexed": false,
      "internalType": "struct IIncredibleSquaringTaskManager.TaskResponseMetadata",
      "components": [
        {
          "name": "taskResponsedBlock",
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
pub mod IncredibleSquaringTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
  ///
  /// ```text
  ///0x61014080604052346101e65760608161513880380380916100208285610291565b8339810103126101e65780516001600160a01b038116908181036101e65760208301516001600160a01b038116938482036101e657604001519363ffffffff851685036101e657156102825760805260a052604051636830483560e01b8152602081600481855afa9081156101f2575f9161023f575b5060c052604051632efa2ca360e11b815290602090829060049082905afa9081156101f2575f916101fd575b5060e05260c05160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa9081156101f2575f916101ac575b506101005261012052604051614e6f90816102c982396080518181816102ca01528181610d87015281816119b80152611e1e015260a05181818161097c015281816117e50152818161368301528181613d1b01528181613df10152614383015260c0518181816115c2015281816140f3015261423e015260e05181818161157e015281816135a4015261402f015261010051818181611d1a0152613f040152610120518181816105ae01526111d60152f35b90506020813d6020116101ea575b816101c760209383610291565b810103126101e657516001600160a01b03811681036101e6575f6100fa565b5f80fd5b3d91506101ba565b6040513d5f823e3d90fd5b90506020813d602011610237575b8161021860209383610291565b810103126101e657516001600160a01b03811681036101e6575f6100c2565b3d915061020b565b90506020813d60201161027a575b8161025a60209383610291565b810103126101e657516001600160a01b03811681036101e6576004610096565b3d915061024d565b6339b190bb60e11b5f5260045ffd5b601f909101601f19168101906001600160401b038211908210176102b457604052565b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8063136439dd1461029a578063171f1d5b146102955780631ad43189146101e6578063245a7bfc146102905780632cb223d51461028b5780632d89f6fc1461028657806331b36bd9146102815780633563b0d11461027c5780633998fdd314610277578063416c7e5e146102725780634d2b57fe1461026d5780634f739f7414610268578063595c6a67146102635780635a2d7f021461025e5780635ac86ab7146102595780635baec9a0146102545780635c1556621461024f5780635c975abb1461024a5780635decc3f5146102455780635df4594614610240578063683048351461023b5780636b532e9e146102365780636b92787e146102315780636d14a9871461022c5780636efb463614610227578063715018a61461022257806372d18e8d146102135780637afa1eed1461021d578063886f1195146102185780638b00ce7c146102135780638da5cb5b1461020e5780639b290e9814610209578063b98d090814610204578063ca8aa7c7146101ff578063cc2a9a5b146101fa578063cefdc1d4146101f5578063df5cf723146101f0578063f2fde38b146101eb578063f5c9899d146101e6578063f63c5bab146101e15763fabc1cbc146101dc575f80fd5b611df5565b611dda565b610592565b611d49565b611d05565b611bc1565b611a81565b611a59565b611a37565b611a0f565b6119e7565b611958565b6119a3565b61197b565b6118fd565b611850565b6117d0565b611662565b6115f1565b6115ad565b611569565b61152b565b61150e565b611395565b6110ab565b610dfc565b610dcf565b610d5c565b610cb5565b610aaa565b61094a565b610918565b61089e565b6106f4565b61064c565b610613565b6105d2565b610520565b3461035a57602036600319011261035a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156103555761032492610310915f91610326575b50611ef7565b61031f60665482811614611f0d565b6145c8565b005b610348915060203d60201161034e575b61034081836103ad565b810190611ed7565b5f61030a565b503d610336565b611eec565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761038d57604052565b61035e565b608081019081106001600160401b0382111761038d57604052565b90601f801991011681019081106001600160401b0382111761038d57604052565b604051906103de610100836103ad565b565b604051906103de6040836103ad565b604051906103de6060836103ad565b604051906103de60a0836103ad565b906103de60405192836103ad565b60409060e319011261035a576040519061043482610372565b60e4358252610104356020830152565b919082604091031261035a5760405161045c81610372565b6020808294803584520135910152565b9080601f8301121561035a57604051916104876040846103ad565b82906040810192831161035a57905b8282106104a35750505090565b8135815260209182019101610496565b90608060631983011261035a576040516104cc81610372565b60206104e782946104de81606461046c565b845260a461046c565b910152565b919060808382031261035a5760206104e76040519261050a84610372565b60408496610518838261046c565b86520161046c565b3461035a5761012036600319011261035a57600435604036602319011261035a57610578604091825161055281610372565b60243581526044356020820152610568366104b3565b906105723661041b565b92611f61565b8251911515825215156020820152f35b5f91031261035a57565b3461035a575f36600319011261035a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461035a575f36600319011261035a5760ce546040516001600160a01b039091168152602090f35b63ffffffff81160361035a57565b35906103de826105fa565b3461035a57602036600319011261035a5763ffffffff600435610635816105fa565b165f5260cb602052602060405f2054604051908152f35b3461035a57602036600319011261035a5763ffffffff60043561066e816105fa565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b0381160361035a57565b6001600160401b03811161038d5760051b60200190565b90602080835192838152019201905f5b8181106106ca5750505090565b82518452602093840193909201916001016106bd565b9060206106f19281815201906106ad565b90565b3461035a57604036600319011261035a5760043561071181610685565b602435906001600160401b03821161035a573660238301121561035a5781600401359161073d83610696565b9261074b60405194856103ad565b8084526024602085019160051b8301019136831161035a57602401905b82821061078c5761078861077c86866120cb565b604051918291826106e0565b0390f35b60208091833561079b81610685565b815201910190610768565b6001600160401b03811161038d57601f01601f191660200190565b9291926107cd826107a6565b916107db60405193846103ad565b82948184528183011161035a578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b83831061082257505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b8082106108625750505060208060019297019301930191939290610813565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610843565b3461035a57606036600319011261035a576004356108bb81610685565b6024356001600160401b03811161035a573660238201121561035a57610788916108f26109049236906024816004013591016107c1565b604435916108ff836105fa565b612309565b6040519182916020835260208301906107f7565b3461035a575f36600319011261035a5760cd546040516001600160a01b039091168152602090f35b8015150361035a57565b3461035a57602036600319011261035a5760043561096781610940565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f916109dc575b506001600160a01b031633036109cd5761032490614a5b565b637070f3b160e11b5f5260045ffd5b6109fe915060203d602011610a04575b6109f681836103ad565b810190612190565b5f6109b4565b503d6109ec565b9080601f8301121561035a578135610a2281610696565b92610a3060405194856103ad565b81845260208085019260051b82010192831161035a57602001905b828210610a585750505090565b8135815260209182019101610a4b565b60206040818301928281528451809452019201905f5b818110610a8b5750505090565b82516001600160a01b0316845260209384019390920191600101610a7e565b3461035a57604036600319011261035a57600435610ac781610685565b6024356001600160401b03811161035a57610ae6903690600401610a0b565b610af08151612069565b916001600160a01b03165f5b8251811015610b8d57806020610b15610b3593866120a8565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561035557600192610b69915f91610b6f575b50610b5a83886120a8565b6001600160a01b039091169052565b01610afc565b610b87915060203d8111610a04576109f681836103ad565b5f610b4f565b604051806107888682610a68565b9181601f8401121561035a578235916001600160401b03831161035a576020838186019501011161035a57565b90602080835192838152019201905f5b818110610be55750505090565b825163ffffffff16845260209384019390920191600101610bd8565b90602082526060610c4f610c3a610c2484516080602088015260a0870190610bc8565b6020850151868203601f19016040880152610bc8565b6040840151858203601f190184870152610bc8565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610c8857505050505090565b9091929394602080610ca6600193601f198682030187528951610bc8565b97019301930191939290610c79565b3461035a57608036600319011261035a57600435610cd281610685565b60243590610cdf826105fa565b6044356001600160401b03811161035a57610cfe903690600401610b9b565b91606435926001600160401b03841161035a573660238501121561035a578360040135926001600160401b03841161035a573660248560051b8701011161035a57610788956024610d5096019361281d565b60405191829182610c01565b3461035a575f36600319011261035a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561035557610dc7915f916103265750611ef7565b610324614594565b3461035a575f36600319011261035a57602060405167016345785d8a00008152f35b60ff81160361035a57565b3461035a57602036600319011261035a576020600160ff600435610e1f81610df1565b161b806066541614604051908152f35b9081608091031261035a5790565b604090602319011261035a57602490565b9080601f8301121561035a578135610e6581610696565b92610e7360405194856103ad565b81845260208085019260051b82010192831161035a57602001905b828210610e9b5750505090565b602080918335610eaa816105fa565b815201910190610e8e565b81601f8201121561035a578035610ecb81610696565b92610ed960405194856103ad565b81845260208085019260061b8401019281841161035a57602001915b838310610f03575050505090565b6020604091610f128486610444565b815201920191610ef5565b9080601f8301121561035a578135610f3481610696565b92610f4260405194856103ad565b81845260208085019260051b8201019183831161035a5760208201905b838210610f6e57505050505090565b81356001600160401b03811161035a57602091610f9087848094880101610e4e565b815201910190610f5f565b9190916101808184031261035a57610fb16103ce565b9281356001600160401b03811161035a5781610fce918401610e4e565b845260208201356001600160401b03811161035a5781610fef918401610eb5565b602085015260408201356001600160401b03811161035a5781611013918401610eb5565b604085015261102581606084016104ec565b60608501526110378160e08401610444565b60808501526101208201356001600160401b03811161035a578161105c918401610e4e565b60a08501526101408201356001600160401b03811161035a5781611081918401610e4e565b60c08501526101608201356001600160401b03811161035a576110a49201610f1d565b60e0830152565b3461035a57608036600319011261035a576004356001600160401b03811161035a576110db903690600401610e2f565b6110e436610e3d565b906064356001600160401b03811161035a57611104903690600401610f9b565b60ce549092906001600160a01b0316330361131757611127602083949301612c85565b9161122a6111386040860186612c8f565b92909461119861114a60608901612c85565b9760405161116e81611160602082019485612cc1565b03601f1981018352826103ad565b51902061119161117d88612c85565b63ffffffff165f5260ca60205260405f2090565b5414612d48565b6111c26111bb6111a787612c85565b63ffffffff165f5260cb60205260405f2090565b5415612dba565b8363ffffffff43169661120c6112046111fb7f000000000000000000000000000000000000000000000000000000000000000086612e4b565b63ffffffff1690565b891115612e65565b6040516020810190611222816111608b85612ee5565b519020613c3e565b919060ff5f9616955b8281106112b5577f349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a8686866112756112696103e0565b63ffffffff9094168452565b6020830152604051602081019061129181611160868686612fc8565b5190206112a06111a783612c85565b556112b060405192839283612fc8565b0390a1005b806113116112ed6112e86112dc6112cf60019688516120a8565b516001600160601b031690565b6001600160601b031690565b612ef5565b61130a6112dc8b6113056112cf8760208b01516120a8565b612f34565b1115612f57565b01611233565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b60206040818301928281528451809452019201905f5b81811061137f5750505090565b8251845260209384019390920191600101611372565b3461035a57606036600319011261035a576004356113b281610685565b6024356001600160401b03811161035a576113d1903690600401610a0b565b604435916113de836105fa565b6040516361c8a12f60e11b8152906001600160a01b03165f8280611406868860048401612ff2565b0381845afa918215610355575f926114ea575b506114248351612069565b935f5b84518110156114dc5761143a81866120a8565b519060208361145661144c84896120a8565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa8015610355576001925f916114ae575b50828060c01b03166114a782896120a8565b5201611427565b6114cf915060203d81116114d5575b6114c781836103ad565b810190612794565b5f611495565b503d6114bd565b60405180610788888261135c565b6115079192503d805f833e6114ff81836103ad565b810190612663565b905f611419565b3461035a575f36600319011261035a576020606654604051908152f35b3461035a57602036600319011261035a5763ffffffff60043561154d816105fa565b165f5260cc602052602060ff60405f2054166040519015158152f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a5760c036600319011261035a576004356001600160401b03811161035a57611621903690600401610e2f565b61162a36610e3d565b90604036606319011261035a5760a4356001600160401b03811161035a576103249261165c6064923690600401610eb5565b92613440565b3461035a57606036600319011261035a57602435600435611682826105fa565b6044356001600160401b03811161035a576116a1903690600401610b9b565b60cf5491939092916001600160a01b03163303611781576103249361176b936116eb6116f2936116cf613a40565b9586524363ffffffff16602087015263ffffffff166060860152565b36916107c1565b6040820152604051602081019061170d816111608585613a64565b51902061172261117d60c95463ffffffff1690565b5560c95463ffffffff16907f1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c486040518061176363ffffffff86169482613a64565b0390a2612e1b565b63ffffffff1663ffffffff1960c954161760c955565b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b90602080835192838152019201905f5b8181106118315750505090565b82516001600160601b0316845260209384019390920191600101611824565b3461035a57608036600319011261035a576004356024356001600160401b03811161035a57611883903690600401610b9b565b9091604435611891816105fa565b606435926001600160401b03841161035a576118f3946118b86118be953690600401610f9b565b93613c3e565b6040519283926040845260206118df82516040808801526080870190611814565b910151848203603f19016060860152611814565b9060208301520390f35b3461035a575f36600319011261035a57611915614c37565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461035a575f36600319011261035a57602063ffffffff60c95416604051908152f35b3461035a575f36600319011261035a5760cf546040516001600160a01b039091168152602090f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a575f36600319011261035a576033546040516001600160a01b039091168152602090f35b3461035a575f36600319011261035a5760d0546040516001600160a01b039091168152602090f35b3461035a575f36600319011261035a57602060ff609754166040519015158152f35b3461035a575f36600319011261035a5760d1546040516001600160a01b039091168152602090f35b3461035a5760c036600319011261035a57600435611a9e81610685565b611b1e602435611aad81610685565b604435611ab981610685565b606435611ac581610685565b60843591611ad283610685565b60a43593611adf85610685565b5f5496611b0460ff60088a901c16158099819a611b9c575b8115611b7c575b506144c8565b87611b15600160ff195f5416175f55565b611b655761452b565b611b2457005b611b3261ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016112b0565b611b7761010061ff00195f5416175f55565b61452b565b303b15915081611b8e575b505f611afe565b60ff1660011490505f611b87565b600160ff8216109150611af7565b6040906106f19392815281602082015201906107f7565b3461035a57606036600319011261035a57600435611bde81610685565b602435604435611bed816105fa565b611c2e611bf8612047565b9280611c038561209b565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401612ff2565b0381875afa9384156103555783611c586111fb61144c611c8d986020975f91611ceb575b5061209b565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561035557611cbc925f91611ccc575b506001600160c01b031692611cb684614cd7565b90612309565b9061078860405192839283611baa565b611ce5915060203d6020116114d5576114c781836103ad565b5f611ca2565b611cff91503d805f833e6114ff81836103ad565b5f611c52565b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a57602036600319011261035a57600435611d6681610685565b611d6e614c37565b6001600160a01b03811615611d865761032490614c8f565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461035a575f36600319011261035a57602060405160648152f35b3461035a57602036600319011261035a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f91611eb8575b506001600160a01b03163303611ea957611e77606654198219811614611f0d565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b611ed1915060203d602011610a04576109f681836103ad565b5f611e56565b9081602091031261035a57516106f181610940565b6040513d5f823e3d90fd5b15611efe57565b631d77d47760e21b5f5260045ffd5b15611f1457565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b906002811015611f485760051b0190565b611f23565b634e487b7160e01b5f52601260045260245ffd5b61203d61201a6120439561201461200d85875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e0840152610100830152611fe481610120840103601f1981018352826103ad565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b809661463e565b90614684565b9261201461202f6120296146e6565b946147dd565b916120386148f9565b61463e565b9161492d565b9091565b6040805190919061205883826103ad565b6001815291601f1901366020840137565b9061207382610696565b61208060405191826103ad565b8281528092612091601f1991610696565b0190602036910137565b805115611f485760200190565b8051821015611f485760209160051b010190565b9081602091031261035a575190565b9190916120d88351612069565b925f5b815181101561218b578060206121046120f761212d94866120a8565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa8015610355576001925f9161215d575b5061215682886120a8565b52016120db565b61217e915060203d8111612184575b61217681836103ad565b8101906120bc565b5f61214b565b503d61216c565b505050565b9081602091031261035a57516106f181610685565b906121af82610696565b6121bc60405191826103ad565b828152602081936121cf601f1991610696565b0191015f5b8281106121e057505050565b6060828201526020016121d4565b908151811015611f48570160200190565b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a57815161223281610696565b9261224060405194856103ad565b81845260208085019260051b82010192831161035a57602001905b8282106122685750505090565b815181526020918201910161225b565b9061228282610696565b61228f60405191826103ad565b82815280926122a0601f1991610696565b015f5b8181106122af57505050565b6040519060608201918083106001600160401b0384111761038d576020926040525f81525f838201525f6040820152828286010152016122a3565b9081602091031261035a57516001600160601b038116810361035a5790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa948515610355575f9561261e575b50604051634f4c91e160e11b815294602086600481855afa918215610355576004965f936125fc575b5060209060405197888092632efa2ca360e11b82525afa958615610355575f966125db575b5061239785939295516121a5565b945f935b80518510156125d1576123c86123c26123b487846121ee565b516001600160f81b03191690565b60f81c90565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa938415610355575f946125ad575b506124188451612278565b612422888b6120a8565b5261242d878a6120a8565b505f5b845181101561259c5780602061244961246b93886120a8565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610355575f9261257c575b5061249181876120a8565b518a60208a6124a0858b6120a8565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa938415610355576125338c8f61252e6001986125459789975f9261254c575b5061251961250a6103ef565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b6120a8565b519061253f83836120a8565b526120a8565b5001612430565b61256e91925060203d8111612575575b61256681836103ad565b8101906122ea565b905f6124fe565b503d61255c565b61259591925060203d8111610a04576109f681836103ad565b905f612486565b50600190960195909450915061239b565b6125ca9194503d805f833e6125c281836103ad565b8101906121ff565b925f61240d565b5050509350505090565b6125f591965060203d602011610a04576109f681836103ad565b945f612389565b602091935061261790823d8411610a04576109f681836103ad565b9290612364565b61263891955060203d602011610a04576109f681836103ad565b935f61233b565b6040519061264c82610392565b606080838181528160208201528160408201520152565b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a57815161269681610696565b926126a460405194856103ad565b81845260208085019260051b82010192831161035a57602001905b8282106126cc5750505090565b6020809183516126db816105fa565b8152019101906126bf565b63ffffffff909116815260406020820181905281018390526001600160fb1b03831161035a5760609260051b809284830137010190565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6106f19593168152816020820152019161271d565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff811461277f5760010190565b61275a565b9190811015611f485760051b0190565b9081602091031261035a57516001600160c01b038116810361035a5790565b156127ba57565b6325ec6c1f60e01b5f5260045ffd5b90821015611f48570190565b9081602091031261035a57516106f1816105fa565b5f19811461277f5760010190565b9161281660209263ffffffff9296959660408652604086019161271d565b9416910152565b959394959290919261282d61263f565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa948515610355575f95612c64575b5061286a61263f565b946040516361c8a12f60e11b81525f818061288a8d8d8b600485016126e6565b0381885afa908115610355575f91612c4a575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f81806128cd85878b6004850161273d565b0381875afa908115610355575f91612c30575b5060408701526128ef816121a5565b9860608701998a525f5b60ff811683811015612b7b57885f612922838f61291588612069565b90519061253f83836120a8565b505f8a868f5b8184106129a5575050505090508c61293f82612069565b915f5b81811061296c57505091612961916129679493519061253f83836120a8565b5061276e565b6128f9565b8061299f61298a61144c6001946129848a89516120a8565b516120a8565b61299483886120a8565b9063ffffffff169052565b01612942565b61144c846129ba81602096956129c295612784565b3597516120a8565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561035557888f888a918f94612a676001612a5a81938d809d5f92612b4f575b506123c2612a36612a4492612a2f878060c01b03861615156127b3565b8b8d6127c9565b356001600160f81b03191690565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14612a83575b5050505050600191925001908a918a868f612928565b8597612aa593612a9e60209799986123c295612a3695612784565b35956127c9565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa908115610355578f612b0390612b089383886001975f93612b17575b50612984906129949394516120a8565b6127ea565b905082918a888f888a91612a6d565b612994935090612b406129849260203d8111612b48575b612b3881836103ad565b8101906127d5565b935090612af3565b503d612b2e565b612a44919250612a36612b726123c29260203d81116114d5576114c781836103ad565b93925050612a12565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa90811561035557612bd1945f948593612c0f575b5060405163354952a360e21b815295869485938493600485016127f8565b03916001600160a01b03165afa908115610355575f91612bf5575b50602082015290565b612c0991503d805f833e6114ff81836103ad565b5f612bec565b612c2991935060203d602011610a04576109f681836103ad565b915f612bb3565b612c4491503d805f833e6114ff81836103ad565b5f6128e0565b612c5e91503d805f833e6114ff81836103ad565b5f61289d565b612c7e91955060203d602011610a04576109f681836103ad565b935f612861565b356106f1816105fa565b903590601e198136030182121561035a57018035906001600160401b03821161035a5760200191813603831361035a57565b602081528135602082015263ffffffff6020830135612cdf816105fa565b1660408201526040820135601e198336030181121561035a578201906020823592016001600160401b03831161035a57823603811361035a57612d3d6060612d366080936106f196858488015260a087019161271d565b9501610608565b63ffffffff16910152565b15612d4f57565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15612dc157565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b63ffffffff60019116019063ffffffff821161277f57565b63ffffffff60649116019063ffffffff821161277f57565b9063ffffffff8091169116019063ffffffff821161277f57565b15612e6c57565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6020809163ffffffff8135612edb816105fa565b1684520135910152565b6040810192916103de9190612ec7565b9060648202918083046064149015171561277f57565b9060068202918083046006149015171561277f57565b8181029291811591840414171561277f57565b906001600160601b03809116911602906001600160601b03821691820361277f57565b15612f5e57565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091612fde846080810197612ec7565b63ffffffff81511660408501520151910152565b60409063ffffffff6106f1949316815281602082015201906106ad565b1561301657565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b909291602060609161307b846080810197612ec7565b63ffffffff813561308b816105fa565b1660408501520135910152565b1561309f57565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b1561311157565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b1561318f57565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b8181106132245750505090565b8251845260209384019390920191600101613217565b1561324157565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a5781516132f881610696565b9261330660405194856103ad565b81845260208085019260051b82010192831161035a57602001905b82821061332e5750505090565b60208091835161333d81610685565b815201910190613321565b604051906133576040836103ad565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b81811061342157505050608061340c6106f194956060850151601f1985830301848601526106ad565b9201519060a0601f1982850301910152613378565b82516001600160a01b03168752602096870196909201916001016133e3565b909291600161344e85612c85565b94602061350a85356134796134718a63ffffffff165f5260cb60205260405f2090565b54151561300f565b6134b46134948a63ffffffff165f5260cb60205260405f2090565b54604051858101906134ab816111608c8b86613065565b51902014613098565b6134df6134d96134d28b63ffffffff165f5260cc60205260405f2090565b5460ff1690565b1561310a565b6135046134f66111fb6134f189612c85565b612e33565b63ffffffff43161115613188565b80612f21565b9101351414613a0f5761351d8351612069565b935f5b845181101561355d578061354c613539600193886120a8565b5180515f526020015160205260405f2090565b61355682896120a8565b5201613520565b5090929391946135976020870194602061357687612c85565b60405161358b816111608a86830195866131fa565b5190209101351461323a565b6135a18551612069565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b8751811015613651578060206135e861360893896120a8565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa9182156103555760019261362d915f91613633575b50610b5a838d6120a8565b016135cf565b61364b915060203d8111610a04576109f681836103ad565b5f613622565b50929695509250926136b061367961368160408601946136718688612c8f565b939091612c85565b9236916107c1565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612309565b945f915b86518310156139b1575f979697955b6136cd848a6120a8565b51518710156139a1579761370e986020806136ec8a61298489876120a8565b510151604051809c81926308f6629d60e31b8352600483019190602083019252565b0381865afa998a15610355575f9a613981575b506001985f5b85518110156139735761374c6137406120f783896120a8565b6001600160a01b031690565b6001600160a01b038d161461376357600101613727565b5099909791985060015f5b151514613783575b50600101959796976136c3565b98969786985f879593986137e860ff6137c06123c2612a368c6137ba61383d9f6137b460cd5460018060a01b031690565b98612c8f565b906127c9565b6137da6137cb6103e0565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d1546137ff90613740906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529098899190829081906044820190565b03915afa968715610355575f9761394f575b5061385a8751612069565b985f5b8a51811015613883578067016345785d8a000061387c6001938e6120a8565b520161385d565b509a9294966138c160ff6138a86123c2612a368f9d979f969e966137ba8e8e92612c8f565b6138b361250a6103fe565b1663ffffffff166020860152565b604084015260608301526138d3613348565b608083015260d0546138ef90613740906001600160a01b031681565b803b1561035a57604051636a669b4160e01b8152925f91849182908490829061391b906004830161339c565b03925af191821561035557600192613935575b5090613776565b806139435f613949936103ad565b80610588565b5f61392e565b61396c9197503d805f833e61396481836103ad565b8101906132c5565b955f61384f565b50999097919860019061376e565b61399a919a5060203d8111610a04576109f681836103ad565b985f613721565b96979695506001909201916136b4565b50505050509190506139e16139d48263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b50505063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b60405190613a4d82610392565b5f6060838281528260208201528160408201520152565b602081528151602082015263ffffffff6020830151166040820152608063ffffffff6060613a9f6040860151848387015260a0860190613378565b9401511691015290565b60405190613ab682610372565b60606020838281520152565b15613ac957565b62f8202d60e51b5f5260045ffd5b15613ade57565b6343714afd60e01b5f5260045ffd5b15613af457565b635f832f4160e01b5f5260045ffd5b15613b0a57565b634b874f4560e01b5f5260045ffd5b9081602091031261035a57516106f181610df1565b5f1981019190821161277f57565b15613b4357565b633fdc650560e21b5f5260045ffd5b906001820180921161277f57565b906002820180921161277f57565b906003820180921161277f57565b906004820180921161277f57565b906005820180921161277f57565b9190820180921161277f57565b15613bac57565b63affc5edb60e01b5f5260045ffd5b9081602091031261035a575167ffffffffffffffff198116810361035a5790565b15613be357565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b03821161277f57565b15613c1957565b6367988d3360e01b5f5260045ffd5b15613c2f57565b63ab1b236b60e01b5f5260045ffd5b949392909193613c4c613aa9565b50613c58851515613ac2565b6040840151518514806144ba575b806144ac575b8061449e575b613c7b90613ad7565b613c8d60208501515185515114613aed565b613ca463ffffffff431663ffffffff841610613b03565b613cac6103e0565b5f81525f602082015292613cbe613aa9565b613cc787612069565b6020820152613cd587612069565b8152613cdf613aa9565b92613cee602088015151612069565b8452613cfe602088015151612069565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561035557613d67915f9161446f575b50613d62368b876107c1565b614a99565b985f965b60208901518051891015613ec657602088613dbb61144c8c613db38f96868e613d986135398680956120a8565b613da584848401516120a8565b5282613e93575b01516120a8565b5195516120a8565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa918215610355576120148a613e688f613e618f8460208f92613e5893613e508460019e613e6e9e5f91613e76575b508f8060c01b031692516120a8565b5201516120a8565b51938d516120a8565b5116614ac4565b90614af5565b970196613d6b565b613e8d9150863d81116114d5576114c781836103ad565b5f613e41565b613ec1613ea384848401516120a8565b51613eba84840151613eb487613b2e565b906120a8565b5110613b3c565b613dac565b50909597949650613edb919893929950614bb2565b91613ee860975460ff1690565b908115614467576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f91614448575b5091905b5f925b818410613f9957505050505092613f80613f7b613f74613f9395856111609860806060602099015192015192611f61565b9190613c12565b613c28565b01516040519283916020830195866131fa565b51902090565b92989596909399919794878b888c888d614342575b61144c8260a0613fee6123c2612a3684613ff697613fe8613fda6135398f9c604060209f9e01516120a8565b67ffffffffffffffff191690565b9b6127c9565b9701516120a8565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa908115610355576140ba61144c8f958f906140b28f978f96848f6140ac60c0966140a5848f60209f90613dac612a36996040936123c29c5f91614314575b5067ffffffffffffffff19918216911614613bdc565b5190614684565b9c6127c9565b9601516120a8565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561035557614147918c8f925f926142f0575b506020614139929301516120a8565b906001600160601b03169052565b6141678c6141398c6141606112cf8260208601516120a8565b92516120a8565b5f985f5b60208a0151518110156142d7578b8d6141a98961419c6123c2612a36868f8961419491516120a8565b5194876127c9565b60ff161c60019081161490565b6141b8575b505060010161416b565b8a8a61423a859f948f96866129848f9360e06141f161144c9560206141e96123c2612a36839f6141fa9c89916127c9565b9a01516120a8565b519b01516120a8565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355578f6142a6908f936001959486955f926142b1575b506142a06141399293519361429b6112cf84876120a8565b613bf2565b926120a8565b019a90508b8d6141ae565b61413992506142d06142a09160203d81116125755761256681836103ad565b9250614283565b5093919796996001919699509a94929a01929190613f43565b614139925061430d602091823d81116125755761256681836103ad565b925061412a565b602061433592503d811161433b575b61432d81836103ad565b810190613bbb565b5f61408f565b503d614323565b61437f945061435c92506123c291612a36916020956127c9565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561035557602089613ff68f938f60a08f976123c2612a368f8f90613fe8613fda6135398f60408b96918f889361144c9f6144039061440993613fee9f5f9261441f575b5063ffffffff809116931690613b98565b11613ba5565b5050505050509750505050505092935050613fae565b602063ffffffff9293508291614440913d81116121845761217681836103ad565b9291506143f2565b614461915060203d602011612b4857612b3881836103ad565b5f613f3c565b5f9190613f40565b614491915060203d602011614497575b61448981836103ad565b810190613b19565b5f613d56565b503d61447f565b5060e0840151518514613c72565b5060c0840151518514613c6c565b5060a0840151518514613c66565b156144cf57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b61453490614c8f565b60ce80546001600160a01b03199081166001600160a01b039384161790915560cf805482169383169390931790925560d1805483169382169390931790925560d0805482169383169390931790925560cd80549092169216919091179055565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b6040519061460782610372565b5f6020838281520152565b6040519061018061462381846103ad565b368337565b604051906146376020836103ad565b6020368337565b9190604090606061464d6145fa565b948592602085519261465f85856103ad565b8436853780518452015160208301528482015260076107cf195a01fa1561468257565bfe5b6020929160806040926146956145fa565b958693818651936146a686866103ad565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa801561468257156146d757565b63d4b68fd760e01b5f5260045ffd5b6040516146f281610372565b604090815161470183826103ad565b823682378152602082519161471684846103ad565b833684370152805161472882826103ad565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061477e83836103ad565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208301526147d3835193846103ad565b8252602082015290565b5f516020614e1a5f395f51905f52906147f46145fa565b505f919006602060c0835b6148f4575f935f516020614e1a5f395f51905f526003818681818009090860405161482a85826103ad565b8436823784818560405161483e82826103ad565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614e1a5f395f51905f5260a082015260056107cf195a01fa8015614682576148a890614e03565b51916148f4575f516020614e1a5f395f51905f52828009146148df57505f516020614e1a5f395f51905f5260015f940892936147ff565b929350506148eb6103e0565b92835282015290565b611f4d565b6149016145fa565b5060405161490e81610372565b600181526002602082015290565b90600c811015611f485760051b0190565b9392909161493b604061040d565b948552602085015261494d604061040d565b918252602082015261495d614612565b925f5b6002811061498a57505050602061018092614979614628565b93849160086201d4c0fa9151151590565b80614996600192612f0b565b6149a08285611f37565b51516149ac828961491c565b5260206149b98386611f37565b5101516149ce6149c883613b52565b8961491c565b526149d98286611f37565b5151516149e86149c883613b60565b526149fe6149f68387611f37565b515160200190565b51614a0b6149c883613b6e565b526020614a188387611f37565b51015151614a286149c883613b7c565b52614a54614a4e614a476020614a3e868a611f37565b51015160200190565b5192613b8a565b8861491c565b5201614960565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614aa760ff93614d8b565b928392161b1115614ab55790565b63ca95733360e01b5f5260045ffd5b805f915b614ad0575090565b5f19810181811161277f5761ffff9116911661ffff811461277f576001019080614ac8565b90614afe6145fa565b5061ffff811690610200821015614ba35760018214614b9e57614b1f6103e0565b5f81525f602082015292906001905f925b61ffff8316851015614b4457505050505090565b600161ffff831660ff86161c811614614b7e575b6001614b74614b698360ff94614684565b9460011b61fffe1690565b9401169291614b30565b946001614b74614b69614b938960ff95614684565b989350505050614b58565b505090565b637fc4ea7d60e11b5f5260045ffd5b614bba6145fa565b50805190811580614c2b575b15614be7575050604051614bdb6040826103ad565b5f81525f602082015290565b60205f516020614e1a5f395f51905f52910151065f516020614e1a5f395f51905f52035f516020614e1a5f395f51905f52811161277f57604051916147d383610372565b50602081015115614bc6565b6033546001600160a01b03163303614c4b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff614ce382614ac4565b16614ced816107a6565b90614cfb60405192836103ad565b808252614d0a601f19916107a6565b013660208301375f5f5b8251821080614d6a575b15614d63576001811b8416614d3c575b614d37906127ea565b614d14565b906001614d379160ff60f81b8460f81b165f1a614d5982876121ee565b5301919050614d2e565b5050905090565b506101008110614d1e565b15614d7c57565b631019106960e31b5f5260045ffd5b90610100825111614df457815115614def57602082015160019060f81c81901b5b8351821015614dea57600190614dd5614dcb6123c26123b486896121ee565b60ff600191161b90565b90614de1818311614d75565b17910190614dac565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b15614e0a57565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212208c5445538e6e18b060f7608076c29ebcbd1cf2a0a3507b01096529e6d0000b6764736f6c634300081b0033
  /// ```
  #[rustfmt::skip]
  #[allow(clippy::all)]
  pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
      b"a\x01@\x80`@R4a\x01\xE6W``\x81aQ8\x808\x03\x80\x91a\0 \x82\x85a\x02\x91V[\x839\x81\x01\x03\x12a\x01\xE6W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\x01\xE6W` \x83\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x84\x82\x03a\x01\xE6W`@\x01Q\x93c\xFF\xFF\xFF\xFF\x85\x16\x85\x03a\x01\xE6W\x15a\x02\x82W`\x80R`\xA0R`@Qch0H5`\xE0\x1B\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x02?W[P`\xC0R`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x01\xFDW[P`\xE0R`\xC0Q`@Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x01\xACW[Pa\x01\0Ra\x01 R`@QaNo\x90\x81a\x02\xC9\x829`\x80Q\x81\x81\x81a\x02\xCA\x01R\x81\x81a\r\x87\x01R\x81\x81a\x19\xB8\x01Ra\x1E\x1E\x01R`\xA0Q\x81\x81\x81a\t|\x01R\x81\x81a\x17\xE5\x01R\x81\x81a6\x83\x01R\x81\x81a=\x1B\x01R\x81\x81a=\xF1\x01RaC\x83\x01R`\xC0Q\x81\x81\x81a\x15\xC2\x01R\x81\x81a@\xF3\x01RaB>\x01R`\xE0Q\x81\x81\x81a\x15~\x01R\x81\x81a5\xA4\x01Ra@/\x01Ra\x01\0Q\x81\x81\x81a\x1D\x1A\x01Ra?\x04\x01Ra\x01 Q\x81\x81\x81a\x05\xAE\x01Ra\x11\xD6\x01R\xF3[\x90P` \x81=` \x11a\x01\xEAW[\x81a\x01\xC7` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W_a\0\xFAV[_\x80\xFD[=\x91Pa\x01\xBAV[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x027W[\x81a\x02\x18` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W_a\0\xC2V[=\x91Pa\x02\x0BV[\x90P` \x81=` \x11a\x02zW[\x81a\x02Z` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W`\x04a\0\x96V[=\x91Pa\x02MV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x02\xB4W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x13d9\xDD\x14a\x02\x9AW\x80c\x17\x1F\x1D[\x14a\x02\x95W\x80c\x1A\xD41\x89\x14a\x01\xE6W\x80c$Z{\xFC\x14a\x02\x90W\x80c,\xB2#\xD5\x14a\x02\x8BW\x80c-\x89\xF6\xFC\x14a\x02\x86W\x80c1\xB3k\xD9\x14a\x02\x81W\x80c5c\xB0\xD1\x14a\x02|W\x80c9\x98\xFD\xD3\x14a\x02wW\x80cAl~^\x14a\x02rW\x80cM+W\xFE\x14a\x02mW\x80cOs\x9Ft\x14a\x02hW\x80cY\\jg\x14a\x02cW\x80cZ-\x7F\x02\x14a\x02^W\x80cZ\xC8j\xB7\x14a\x02YW\x80c[\xAE\xC9\xA0\x14a\x02TW\x80c\\\x15Vb\x14a\x02OW\x80c\\\x97Z\xBB\x14a\x02JW\x80c]\xEC\xC3\xF5\x14a\x02EW\x80c]\xF4YF\x14a\x02@W\x80ch0H5\x14a\x02;W\x80ckS.\x9E\x14a\x026W\x80ck\x92x~\x14a\x021W\x80cm\x14\xA9\x87\x14a\x02,W\x80cn\xFBF6\x14a\x02'W\x80cqP\x18\xA6\x14a\x02\"W\x80cr\xD1\x8E\x8D\x14a\x02\x13W\x80cz\xFA\x1E\xED\x14a\x02\x1DW\x80c\x88o\x11\x95\x14a\x02\x18W\x80c\x8B\0\xCE|\x14a\x02\x13W\x80c\x8D\xA5\xCB[\x14a\x02\x0EW\x80c\x9B)\x0E\x98\x14a\x02\tW\x80c\xB9\x8D\t\x08\x14a\x02\x04W\x80c\xCA\x8A\xA7\xC7\x14a\x01\xFFW\x80c\xCC*\x9A[\x14a\x01\xFAW\x80c\xCE\xFD\xC1\xD4\x14a\x01\xF5W\x80c\xDF\\\xF7#\x14a\x01\xF0W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xEBW\x80c\xF5\xC9\x89\x9D\x14a\x01\xE6W\x80c\xF6<[\xAB\x14a\x01\xE1Wc\xFA\xBC\x1C\xBC\x14a\x01\xDCW_\x80\xFD[a\x1D\xF5V[a\x1D\xDAV[a\x05\x92V[a\x1DIV[a\x1D\x05V[a\x1B\xC1V[a\x1A\x81V[a\x1AYV[a\x1A7V[a\x1A\x0FV[a\x19\xE7V[a\x19XV[a\x19\xA3V[a\x19{V[a\x18\xFDV[a\x18PV[a\x17\xD0V[a\x16bV[a\x15\xF1V[a\x15\xADV[a\x15iV[a\x15+V[a\x15\x0EV[a\x13\x95V[a\x10\xABV[a\r\xFCV[a\r\xCFV[a\r\\V[a\x0C\xB5V[a\n\xAAV[a\tJV[a\t\x18V[a\x08\x9EV[a\x06\xF4V[a\x06LV[a\x06\x13V[a\x05\xD2V[a\x05 V[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03UWa\x03$\x92a\x03\x10\x91_\x91a\x03&W[Pa\x1E\xF7V[a\x03\x1F`fT\x82\x81\x16\x14a\x1F\rV[aE\xC8V[\0[a\x03H\x91P` =` \x11a\x03NW[a\x03@\x81\x83a\x03\xADV[\x81\x01\x90a\x1E\xD7V[_a\x03\nV[P=a\x036V[a\x1E\xECV[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[a\x03^V[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[`@Q\x90a\x03\xDEa\x01\0\x83a\x03\xADV[V[`@Q\x90a\x03\xDE`@\x83a\x03\xADV[`@Q\x90a\x03\xDE``\x83a\x03\xADV[`@Q\x90a\x03\xDE`\xA0\x83a\x03\xADV[\x90a\x03\xDE`@Q\x92\x83a\x03\xADV[`@\x90`\xE3\x19\x01\x12a\x03ZW`@Q\x90a\x044\x82a\x03rV[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x03ZW`@Qa\x04\\\x81a\x03rV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW`@Q\x91a\x04\x87`@\x84a\x03\xADV[\x82\x90`@\x81\x01\x92\x83\x11a\x03ZW\x90[\x82\x82\x10a\x04\xA3WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x04\x96V[\x90`\x80`c\x19\x83\x01\x12a\x03ZW`@Qa\x04\xCC\x81a\x03rV[` a\x04\xE7\x82\x94a\x04\xDE\x81`da\x04lV[\x84R`\xA4a\x04lV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03ZW` a\x04\xE7`@Q\x92a\x05\n\x84a\x03rV[`@\x84\x96a\x05\x18\x83\x82a\x04lV[\x86R\x01a\x04lV[4a\x03ZWa\x01 6`\x03\x19\x01\x12a\x03ZW`\x045`@6`#\x19\x01\x12a\x03ZWa\x05x`@\x91\x82Qa\x05R\x81a\x03rV[`$5\x81R`D5` \x82\x01Ra\x05h6a\x04\xB3V[\x90a\x05r6a\x04\x1BV[\x92a\x1FaV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x03ZWV[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03ZWV[5\x90a\x03\xDE\x82a\x05\xFAV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x065\x81a\x05\xFAV[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x06n\x81a\x05\xFAV[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03ZWV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x8DW`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xCAWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\xBDV[\x90` a\x06\xF1\x92\x81\x81R\x01\x90a\x06\xADV[\x90V[4a\x03ZW`@6`\x03\x19\x01\x12a\x03ZW`\x045a\x07\x11\x81a\x06\x85V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW6`#\x83\x01\x12\x15a\x03ZW\x81`\x04\x015\x91a\x07=\x83a\x06\x96V[\x92a\x07K`@Q\x94\x85a\x03\xADV[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03ZW`$\x01\x90[\x82\x82\x10a\x07\x8CWa\x07\x88a\x07|\x86\x86a \xCBV[`@Q\x91\x82\x91\x82a\x06\xE0V[\x03\x90\xF3[` \x80\x91\x835a\x07\x9B\x81a\x06\x85V[\x81R\x01\x91\x01\x90a\x07hV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x8DW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xCD\x82a\x07\xA6V[\x91a\x07\xDB`@Q\x93\x84a\x03\xADV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03ZW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x08\"WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x08bWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x08\x13V[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\x08CV[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a\x08\xBB\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW6`#\x82\x01\x12\x15a\x03ZWa\x07\x88\x91a\x08\xF2a\t\x04\x926\x90`$\x81`\x04\x015\x91\x01a\x07\xC1V[`D5\x91a\x08\xFF\x83a\x05\xFAV[a#\tV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xF7V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCDT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x03ZWV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045a\tg\x81a\t@V[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a\t\xDCW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\t\xCDWa\x03$\x90aJ[V[cpp\xF3\xB1`\xE1\x1B_R`\x04_\xFD[a\t\xFE\x91P` =` \x11a\n\x04W[a\t\xF6\x81\x83a\x03\xADV[\x81\x01\x90a!\x90V[_a\t\xB4V[P=a\t\xECV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\n\"\x81a\x06\x96V[\x92a\n0`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a\nXWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\nKV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\n\x8BWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n~V[4a\x03ZW`@6`\x03\x19\x01\x12a\x03ZW`\x045a\n\xC7\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\n\xE6\x906\x90`\x04\x01a\n\x0BV[a\n\xF0\x81Qa iV[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\x0B\x8DW\x80` a\x0B\x15a\x0B5\x93\x86a \xA8V[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x03UW`\x01\x92a\x0Bi\x91_\x91a\x0BoW[Pa\x0BZ\x83\x88a \xA8V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\n\xFCV[a\x0B\x87\x91P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a\x0BOV[`@Q\x80a\x07\x88\x86\x82a\nhV[\x91\x81`\x1F\x84\x01\x12\x15a\x03ZW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03ZW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03ZWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0B\xE5WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\xD8V[\x90` \x82R``a\x0COa\x0C:a\x0C$\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\x0B\xC8V[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\x0B\xC8V[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\x0B\xC8V[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0C\x88WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0C\xA6`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x0B\xC8V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0CyV[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045a\x0C\xD2\x81a\x06\x85V[`$5\x90a\x0C\xDF\x82a\x05\xFAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x0C\xFE\x906\x90`\x04\x01a\x0B\x9BV[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZW6`#\x85\x01\x12\x15a\x03ZW\x83`\x04\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZW6`$\x85`\x05\x1B\x87\x01\x01\x11a\x03ZWa\x07\x88\x95`$a\rP\x96\x01\x93a(\x1DV[`@Q\x91\x82\x91\x82a\x0C\x01V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UWa\r\xC7\x91_\x91a\x03&WPa\x1E\xF7V[a\x03$aE\x94V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Qg\x01cEx]\x8A\0\0\x81R\xF3[`\xFF\x81\x16\x03a\x03ZWV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW` `\x01`\xFF`\x045a\x0E\x1F\x81a\r\xF1V[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[\x90\x81`\x80\x91\x03\x12a\x03ZW\x90V[`@\x90`#\x19\x01\x12a\x03ZW`$\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\x0Ee\x81a\x06\x96V[\x92a\x0Es`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a\x0E\x9BWPPP\x90V[` \x80\x91\x835a\x0E\xAA\x81a\x05\xFAV[\x81R\x01\x91\x01\x90a\x0E\x8EV[\x81`\x1F\x82\x01\x12\x15a\x03ZW\x805a\x0E\xCB\x81a\x06\x96V[\x92a\x0E\xD9`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03ZW` \x01\x91[\x83\x83\x10a\x0F\x03WPPPP\x90V[` `@\x91a\x0F\x12\x84\x86a\x04DV[\x81R\x01\x92\x01\x91a\x0E\xF5V[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\x0F4\x81a\x06\x96V[\x92a\x0FB`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03ZW` \x82\x01\x90[\x83\x82\x10a\x0FnWPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW` \x91a\x0F\x90\x87\x84\x80\x94\x88\x01\x01a\x0ENV[\x81R\x01\x91\x01\x90a\x0F_V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03ZWa\x0F\xB1a\x03\xCEV[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x0F\xCE\x91\x84\x01a\x0ENV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x0F\xEF\x91\x84\x01a\x0E\xB5V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x10\x13\x91\x84\x01a\x0E\xB5V[`@\x85\x01Ra\x10%\x81``\x84\x01a\x04\xECV[``\x85\x01Ra\x107\x81`\xE0\x84\x01a\x04DV[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x10\\\x91\x84\x01a\x0ENV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x10\x81\x91\x84\x01a\x0ENV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x10\xA4\x92\x01a\x0F\x1DV[`\xE0\x83\x01RV[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x10\xDB\x906\x90`\x04\x01a\x0E/V[a\x10\xE46a\x0E=V[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x11\x04\x906\x90`\x04\x01a\x0F\x9BV[`\xCET\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x13\x17Wa\x11'` \x83\x94\x93\x01a,\x85V[\x91a\x12*a\x118`@\x86\x01\x86a,\x8FV[\x92\x90\x94a\x11\x98a\x11J``\x89\x01a,\x85V[\x97`@Qa\x11n\x81a\x11`` \x82\x01\x94\x85a,\xC1V[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xADV[Q\x90 a\x11\x91a\x11}\x88a,\x85V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[T\x14a-HV[a\x11\xC2a\x11\xBBa\x11\xA7\x87a,\x85V[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15a-\xBAV[\x83c\xFF\xFF\xFF\xFFC\x16\x96a\x12\x0Ca\x12\x04a\x11\xFB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a.KV[c\xFF\xFF\xFF\xFF\x16\x90V[\x89\x11\x15a.eV[`@Q` \x81\x01\x90a\x12\"\x81a\x11`\x8B\x85a.\xE5V[Q\x90 a<>V[\x91\x90`\xFF_\x96\x16\x95[\x82\x81\x10a\x12\xB5W\x7F4\x9C\x1E\xE6\x0EN\x89r\xEE\x9D\xBAd,\x17tT=\\A6\x87\x9B\x7FL\xAA\xF0K\xF8\x1AHz*\x86\x86\x86a\x12ua\x12ia\x03\xE0V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x12\x91\x81a\x11`\x86\x86\x86a/\xC8V[Q\x90 a\x12\xA0a\x11\xA7\x83a,\x85V[Ua\x12\xB0`@Q\x92\x83\x92\x83a/\xC8V[\x03\x90\xA1\0[\x80a\x13\x11a\x12\xEDa\x12\xE8a\x12\xDCa\x12\xCF`\x01\x96\x88Qa \xA8V[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[a.\xF5V[a\x13\na\x12\xDC\x8Ba\x13\x05a\x12\xCF\x87` \x8B\x01Qa \xA8V[a/4V[\x11\x15a/WV[\x01a\x123V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x13\x7FWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13rV[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a\x13\xB2\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x13\xD1\x906\x90`\x04\x01a\n\x0BV[`D5\x91a\x13\xDE\x83a\x05\xFAV[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x14\x06\x86\x88`\x04\x84\x01a/\xF2V[\x03\x81\x84Z\xFA\x91\x82\x15a\x03UW_\x92a\x14\xEAW[Pa\x14$\x83Qa iV[\x93_[\x84Q\x81\x10\x15a\x14\xDCWa\x14:\x81\x86a \xA8V[Q\x90` \x83a\x14Va\x14L\x84\x89a \xA8V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x03UW`\x01\x92_\x91a\x14\xAEW[P\x82\x80`\xC0\x1B\x03\x16a\x14\xA7\x82\x89a \xA8V[R\x01a\x14'V[a\x14\xCF\x91P` =\x81\x11a\x14\xD5W[a\x14\xC7\x81\x83a\x03\xADV[\x81\x01\x90a'\x94V[_a\x14\x95V[P=a\x14\xBDV[`@Q\x80a\x07\x88\x88\x82a\x13\\V[a\x15\x07\x91\x92P=\x80_\x83>a\x14\xFF\x81\x83a\x03\xADV[\x81\x01\x90a&cV[\x90_a\x14\x19V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `fT`@Q\x90\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x15M\x81a\x05\xFAV[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW`\xC06`\x03\x19\x01\x12a\x03ZW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x16!\x906\x90`\x04\x01a\x0E/V[a\x16*6a\x0E=V[\x90`@6`c\x19\x01\x12a\x03ZW`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x03$\x92a\x16\\`d\x926\x90`\x04\x01a\x0E\xB5V[\x92a4@V[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`$5`\x045a\x16\x82\x82a\x05\xFAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x16\xA1\x906\x90`\x04\x01a\x0B\x9BV[`\xCFT\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17\x81Wa\x03$\x93a\x17k\x93a\x16\xEBa\x16\xF2\x93a\x16\xCFa:@V[\x95\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01Rc\xFF\xFF\xFF\xFF\x16``\x86\x01RV[6\x91a\x07\xC1V[`@\x82\x01R`@Q` \x81\x01\x90a\x17\r\x81a\x11`\x85\x85a:dV[Q\x90 a\x17\"a\x11}`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\x16\x95\xB8\xD0n\xC8\0\xB4a^t\\\xFB[\xD0\x0C\x1F(ua]B\x92\\;Z\xFAT;\xB2LH`@Q\x80a\x17cc\xFF\xFF\xFF\xFF\x86\x16\x94\x82a:dV[\x03\x90\xA2a.\x1BV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x181WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x18$V[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x18\x83\x906\x90`\x04\x01a\x0B\x9BV[\x90\x91`D5a\x18\x91\x81a\x05\xFAV[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZWa\x18\xF3\x94a\x18\xB8a\x18\xBE\x956\x90`\x04\x01a\x0F\x9BV[\x93a<>V[`@Q\x92\x83\x92`@\x84R` a\x18\xDF\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x18\x14V[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x18\x14V[\x90` \x83\x01R\x03\x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZWa\x19\x15aL7V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW`\xC06`\x03\x19\x01\x12a\x03ZW`\x045a\x1A\x9E\x81a\x06\x85V[a\x1B\x1E`$5a\x1A\xAD\x81a\x06\x85V[`D5a\x1A\xB9\x81a\x06\x85V[`d5a\x1A\xC5\x81a\x06\x85V[`\x845\x91a\x1A\xD2\x83a\x06\x85V[`\xA45\x93a\x1A\xDF\x85a\x06\x85V[_T\x96a\x1B\x04`\xFF`\x08\x8A\x90\x1C\x16\x15\x80\x99\x81\x9Aa\x1B\x9CW[\x81\x15a\x1B|W[PaD\xC8V[\x87a\x1B\x15`\x01`\xFF\x19_T\x16\x17_UV[a\x1BeWaE+V[a\x1B$W\0[a\x1B2a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x12\xB0V[a\x1Bwa\x01\0a\xFF\0\x19_T\x16\x17_UV[aE+V[0;\x15\x91P\x81a\x1B\x8EW[P_a\x1A\xFEV[`\xFF\x16`\x01\x14\x90P_a\x1B\x87V[`\x01`\xFF\x82\x16\x10\x91Pa\x1A\xF7V[`@\x90a\x06\xF1\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x07\xF7V[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a\x1B\xDE\x81a\x06\x85V[`$5`D5a\x1B\xED\x81a\x05\xFAV[a\x1C.a\x1B\xF8a GV[\x92\x80a\x1C\x03\x85a \x9BV[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a/\xF2V[\x03\x81\x87Z\xFA\x93\x84\x15a\x03UW\x83a\x1CXa\x11\xFBa\x14La\x1C\x8D\x98` \x97_\x91a\x1C\xEBW[Pa \x9BV[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x03UWa\x1C\xBC\x92_\x91a\x1C\xCCW[P`\x01`\x01`\xC0\x1B\x03\x16\x92a\x1C\xB6\x84aL\xD7V[\x90a#\tV[\x90a\x07\x88`@Q\x92\x83\x92\x83a\x1B\xAAV[a\x1C\xE5\x91P` =` \x11a\x14\xD5Wa\x14\xC7\x81\x83a\x03\xADV[_a\x1C\xA2V[a\x1C\xFF\x91P=\x80_\x83>a\x14\xFF\x81\x83a\x03\xADV[_a\x1CRV[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045a\x1Df\x81a\x06\x85V[a\x1DnaL7V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1D\x86Wa\x03$\x90aL\x8FV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Q`d\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a\x1E\xB8W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1E\xA9Wa\x1Ew`fT\x19\x82\x19\x81\x16\x14a\x1F\rV[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a\x1E\xD1\x91P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a\x1EVV[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\t@V[`@Q=_\x82>=\x90\xFD[\x15a\x1E\xFEWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a\x1F\x14WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x1FHW`\x05\x1B\x01\x90V[a\x1F#V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a =a \x1Aa C\x95a \x14a \r\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x1F\xE4\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xADV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aF>V[\x90aF\x84V[\x92a \x14a /a )aF\xE6V[\x94aG\xDDV[\x91a 8aH\xF9V[aF>V[\x91aI-V[\x90\x91V[`@\x80Q\x90\x91\x90a X\x83\x82a\x03\xADV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a s\x82a\x06\x96V[a \x80`@Q\x91\x82a\x03\xADV[\x82\x81R\x80\x92a \x91`\x1F\x19\x91a\x06\x96V[\x01\x90` 6\x91\x017V[\x80Q\x15a\x1FHW` \x01\x90V[\x80Q\x82\x10\x15a\x1FHW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQ\x90V[\x91\x90\x91a \xD8\x83Qa iV[\x92_[\x81Q\x81\x10\x15a!\x8BW\x80` a!\x04a \xF7a!-\x94\x86a \xA8V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03UW`\x01\x92_\x91a!]W[Pa!V\x82\x88a \xA8V[R\x01a \xDBV[a!~\x91P` =\x81\x11a!\x84W[a!v\x81\x83a\x03\xADV[\x81\x01\x90a \xBCV[_a!KV[P=a!lV[PPPV[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\x06\x85V[\x90a!\xAF\x82a\x06\x96V[a!\xBC`@Q\x91\x82a\x03\xADV[\x82\x81R` \x81\x93a!\xCF`\x1F\x19\x91a\x06\x96V[\x01\x91\x01_[\x82\x81\x10a!\xE0WPPPV[``\x82\x82\x01R` \x01a!\xD4V[\x90\x81Q\x81\x10\x15a\x1FHW\x01` \x01\x90V[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa\"2\x81a\x06\x96V[\x92a\"@`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a\"hWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\"[V[\x90a\"\x82\x82a\x06\x96V[a\"\x8F`@Q\x91\x82a\x03\xADV[\x82\x81R\x80\x92a\"\xA0`\x1F\x19\x91a\x06\x96V[\x01_[\x81\x81\x10a\"\xAFWPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x03\x8DW` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a\"\xA3V[\x90\x81` \x91\x03\x12a\x03ZWQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x03ZW\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x03UW_\x95a&\x1EW[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x03UW`\x04\x96_\x93a%\xFCW[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x03UW_\x96a%\xDBW[Pa#\x97\x85\x93\x92\x95Qa!\xA5V[\x94_\x93[\x80Q\x85\x10\x15a%\xD1Wa#\xC8a#\xC2a#\xB4\x87\x84a!\xEEV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x03UW_\x94a%\xADW[Pa$\x18\x84Qa\"xV[a$\"\x88\x8Ba \xA8V[Ra$-\x87\x8Aa \xA8V[P_[\x84Q\x81\x10\x15a%\x9CW\x80` a$Ia$k\x93\x88a \xA8V[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03UW_\x92a%|W[Pa$\x91\x81\x87a \xA8V[Q\x8A` \x8Aa$\xA0\x85\x8Ba \xA8V[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x03UWa%3\x8C\x8Fa%.`\x01\x98a%E\x97\x89\x97_\x92a%LW[Pa%\x19a%\na\x03\xEFV[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a \xA8V[Q\x90a%?\x83\x83a \xA8V[Ra \xA8V[P\x01a$0V[a%n\x91\x92P` =\x81\x11a%uW[a%f\x81\x83a\x03\xADV[\x81\x01\x90a\"\xEAV[\x90_a$\xFEV[P=a%\\V[a%\x95\x91\x92P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x90_a$\x86V[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa#\x9BV[a%\xCA\x91\x94P=\x80_\x83>a%\xC2\x81\x83a\x03\xADV[\x81\x01\x90a!\xFFV[\x92_a$\rV[PPP\x93PPP\x90V[a%\xF5\x91\x96P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x94_a#\x89V[` \x91\x93Pa&\x17\x90\x82=\x84\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x92\x90a#dV[a&8\x91\x95P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x93_a#;V[`@Q\x90a&L\x82a\x03\x92V[``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa&\x96\x81a\x06\x96V[\x92a&\xA4`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a&\xCCWPPP\x90V[` \x80\x91\x83Qa&\xDB\x81a\x05\xFAV[\x81R\x01\x91\x01\x90a&\xBFV[c\xFF\xFF\xFF\xFF\x90\x91\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x83\x90R`\x01`\x01`\xFB\x1B\x03\x83\x11a\x03ZW``\x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF1\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a'\x1DV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a'\x7FW`\x01\x01\x90V[a'ZV[\x91\x90\x81\x10\x15a\x1FHW`\x05\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03ZW\x90V[\x15a'\xBAWV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a\x1FHW\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\x05\xFAV[_\x19\x81\x14a'\x7FW`\x01\x01\x90V[\x91a(\x16` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a'\x1DV[\x94\x16\x91\x01RV[\x95\x93\x94\x95\x92\x90\x91\x92a(-a&?V[P`@Qch0H5`\xE0\x1B\x81R\x93`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90` \x85`\x04\x81\x86Z\xFA\x94\x85\x15a\x03UW_\x95a,dW[Pa(ja&?V[\x94`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a(\x8A\x8D\x8D\x8B`\x04\x85\x01a&\xE6V[\x03\x81\x88Z\xFA\x90\x81\x15a\x03UW_\x91a,JW[P\x86R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x92\x90_\x81\x80a(\xCD\x85\x87\x8B`\x04\x85\x01a'=V[\x03\x81\x87Z\xFA\x90\x81\x15a\x03UW_\x91a,0W[P`@\x87\x01Ra(\xEF\x81a!\xA5V[\x98``\x87\x01\x99\x8AR_[`\xFF\x81\x16\x83\x81\x10\x15a+{W\x88_a)\"\x83\x8Fa)\x15\x88a iV[\x90Q\x90a%?\x83\x83a \xA8V[P_\x8A\x86\x8F[\x81\x84\x10a)\xA5WPPPP\x90P\x8Ca)?\x82a iV[\x91_[\x81\x81\x10a)lWPP\x91a)a\x91a)g\x94\x93Q\x90a%?\x83\x83a \xA8V[Pa'nV[a(\xF9V[\x80a)\x9Fa)\x8Aa\x14L`\x01\x94a)\x84\x8A\x89Qa \xA8V[Qa \xA8V[a)\x94\x83\x88a \xA8V[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x01a)BV[a\x14L\x84a)\xBA\x81` \x96\x95a)\xC2\x95a'\x84V[5\x97Qa \xA8V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x87\x01R\x16`D\x85\x01R\x83`d\x81\x8DZ\xFA\x80\x15a\x03UW\x88\x8F\x88\x8A\x91\x8F\x94a*g`\x01a*Z\x81\x93\x8D\x80\x9D_\x92a+OW[Pa#\xC2a*6a*D\x92a*/\x87\x80`\xC0\x1B\x03\x86\x16\x15\x15a'\xB3V[\x8B\x8Da'\xC9V[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a*\x83W[PPPPP`\x01\x91\x92P\x01\x90\x8A\x91\x8A\x86\x8Fa)(V[\x85\x97a*\xA5\x93a*\x9E` \x97\x99\x98a#\xC2\x95a*6\x95a'\x84V[5\x95a'\xC9V[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`D\x84\x01R\x82`d\x81\x8CZ\xFA\x90\x81\x15a\x03UW\x8Fa+\x03\x90a+\x08\x93\x83\x88`\x01\x97_\x93a+\x17W[Pa)\x84\x90a)\x94\x93\x94Qa \xA8V[a'\xEAV[\x90P\x82\x91\x8A\x88\x8F\x88\x8A\x91a*mV[a)\x94\x93P\x90a+@a)\x84\x92` =\x81\x11a+HW[a+8\x81\x83a\x03\xADV[\x81\x01\x90a'\xD5V[\x93P\x90a*\xF3V[P=a+.V[a*D\x91\x92Pa*6a+ra#\xC2\x92` =\x81\x11a\x14\xD5Wa\x14\xC7\x81\x83a\x03\xADV[\x93\x92PPa*\x12V[PPP\x92\x90\x95\x97P`\x04\x94\x96P` \x91P`@Q\x94\x85\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x90\x81\x15a\x03UWa+\xD1\x94_\x94\x85\x93a,\x0FW[P`@Qc5IR\xA3`\xE2\x1B\x81R\x95\x86\x94\x85\x93\x84\x93`\x04\x85\x01a'\xF8V[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a+\xF5W[P` \x82\x01R\x90V[a,\t\x91P=\x80_\x83>a\x14\xFF\x81\x83a\x03\xADV[_a+\xECV[a,)\x91\x93P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x91_a+\xB3V[a,D\x91P=\x80_\x83>a\x14\xFF\x81\x83a\x03\xADV[_a(\xE0V[a,^\x91P=\x80_\x83>a\x14\xFF\x81\x83a\x03\xADV[_a(\x9DV[a,~\x91\x95P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x93_a(aV[5a\x06\xF1\x81a\x05\xFAV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03ZW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW` \x01\x91\x816\x03\x83\x13a\x03ZWV[` \x81R\x815` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x015a,\xDF\x81a\x05\xFAV[\x16`@\x82\x01R`@\x82\x015`\x1E\x19\x836\x03\x01\x81\x12\x15a\x03ZW\x82\x01\x90` \x825\x92\x01`\x01`\x01`@\x1B\x03\x83\x11a\x03ZW\x826\x03\x81\x13a\x03ZWa-=``a-6`\x80\x93a\x06\xF1\x96\x85\x84\x88\x01R`\xA0\x87\x01\x91a'\x1DV[\x95\x01a\x06\x08V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15a-OWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a-\xC1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x7FWV[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x7FWV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x7FWV[\x15a.lWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a.\xDB\x81a\x05\xFAV[\x16\x84R\x015\x91\x01RV[`@\x81\x01\x92\x91a\x03\xDE\x91\x90a.\xC7V[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a'\x7FWV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a'\x7FWV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a'\x7FWV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a'\x7FWV[\x15a/^WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[\x90\x92\x91` ``\x91a/\xDE\x84`\x80\x81\x01\x97a.\xC7V[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF1\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x06\xADV[\x15a0\x16WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x92\x91` ``\x91a0{\x84`\x80\x81\x01\x97a.\xC7V[c\xFF\xFF\xFF\xFF\x815a0\x8B\x81a\x05\xFAV[\x16`@\x85\x01R\x015\x91\x01RV[\x15a0\x9FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a1\x11WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a1\x8FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a2$WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a2\x17V[\x15a2AWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa2\xF8\x81a\x06\x96V[\x92a3\x06`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a3.WPPP\x90V[` \x80\x91\x83Qa3=\x81a\x06\x85V[\x81R\x01\x91\x01\x90a3!V[`@Q\x90a3W`@\x83a\x03\xADV[`\x12\x82Rq9\xB60\xB9\xB4/\xBA42\xAF\xB7\xB82\xB90\xBA7\xB9`q\x1B` \x83\x01RV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90` \x83R`\xC0\x83\x01\x92`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x93`\xA0``\x83\x01R\x84Q\x80\x91R` `\xE0\x83\x01\x95\x01\x90_[\x81\x81\x10a4!WPPP`\x80a4\x0Ca\x06\xF1\x94\x95``\x85\x01Q`\x1F\x19\x85\x83\x03\x01\x84\x86\x01Ra\x06\xADV[\x92\x01Q\x90`\xA0`\x1F\x19\x82\x85\x03\x01\x91\x01Ra3xV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x96\x87\x01\x96\x90\x92\x01\x91`\x01\x01a3\xE3V[\x90\x92\x91`\x01a4N\x85a,\x85V[\x94` a5\n\x855a4ya4q\x8Ac\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a0\x0FV[a4\xB4a4\x94\x8Ac\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T`@Q\x85\x81\x01\x90a4\xAB\x81a\x11`\x8C\x8B\x86a0eV[Q\x90 \x14a0\x98V[a4\xDFa4\xD9a4\xD2\x8Bc\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a1\nV[a5\x04a4\xF6a\x11\xFBa4\xF1\x89a,\x85V[a.3V[c\xFF\xFF\xFF\xFFC\x16\x11\x15a1\x88V[\x80a/!V[\x91\x015\x14\x14a:\x0FWa5\x1D\x83Qa iV[\x93_[\x84Q\x81\x10\x15a5]W\x80a5La59`\x01\x93\x88a \xA8V[Q\x80Q_R` \x01Q` R`@_ \x90V[a5V\x82\x89a \xA8V[R\x01a5 V[P\x90\x92\x93\x91\x94a5\x97` \x87\x01\x94` a5v\x87a,\x85V[`@Qa5\x8B\x81a\x11`\x8A\x86\x83\x01\x95\x86a1\xFAV[Q\x90 \x91\x015\x14a2:V[a5\xA1\x85Qa iV[\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94_[\x87Q\x81\x10\x15a6QW\x80` a5\xE8a6\x08\x93\x89a \xA8V[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8BZ\xFA\x91\x82\x15a\x03UW`\x01\x92a6-\x91_\x91a63W[Pa\x0BZ\x83\x8Da \xA8V[\x01a5\xCFV[a6K\x91P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a6\"V[P\x92\x96\x95P\x92P\x92a6\xB0a6ya6\x81`@\x86\x01\x94a6q\x86\x88a,\x8FV[\x93\x90\x91a,\x85V[\x926\x91a\x07\xC1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a#\tV[\x94_\x91[\x86Q\x83\x10\x15a9\xB1W_\x97\x96\x97\x95[a6\xCD\x84\x8Aa \xA8V[QQ\x87\x10\x15a9\xA1W\x97a7\x0E\x98` \x80a6\xEC\x8Aa)\x84\x89\x87a \xA8V[Q\x01Q`@Q\x80\x9C\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x99\x8A\x15a\x03UW_\x9Aa9\x81W[P`\x01\x98_[\x85Q\x81\x10\x15a9sWa7La7@a \xF7\x83\x89a \xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x14a7cW`\x01\x01a7'V[P\x99\x90\x97\x91\x98P`\x01_[\x15\x15\x14a7\x83W[P`\x01\x01\x95\x97\x96\x97a6\xC3V[\x98\x96\x97\x86\x98_\x87\x95\x93\x98a7\xE8`\xFFa7\xC0a#\xC2a*6\x8Ca7\xBAa8=\x9Fa7\xB4`\xCDT`\x01\x80`\xA0\x1B\x03\x16\x90V[\x98a,\x8FV[\x90a'\xC9V[a7\xDAa7\xCBa\x03\xE0V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[\x16c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`\xD1Ta7\xFF\x90a7@\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\x10]\xEA\x1F`\xE2\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\x04\x82\x01R` \x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16`$\x83\x01R\x90\x98\x89\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x96\x87\x15a\x03UW_\x97a9OW[Pa8Z\x87Qa iV[\x98_[\x8AQ\x81\x10\x15a8\x83W\x80g\x01cEx]\x8A\0\0a8|`\x01\x93\x8Ea \xA8V[R\x01a8]V[P\x9A\x92\x94\x96a8\xC1`\xFFa8\xA8a#\xC2a*6\x8F\x9D\x97\x9F\x96\x9E\x96a7\xBA\x8E\x8E\x92a,\x8FV[a8\xB3a%\na\x03\xFEV[\x16c\xFF\xFF\xFF\xFF\x16` \x86\x01RV[`@\x84\x01R``\x83\x01Ra8\xD3a3HV[`\x80\x83\x01R`\xD0Ta8\xEF\x90a7@\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03ZW`@Qcjf\x9BA`\xE0\x1B\x81R\x92_\x91\x84\x91\x82\x90\x84\x90\x82\x90a9\x1B\x90`\x04\x83\x01a3\x9CV[\x03\x92Z\xF1\x91\x82\x15a\x03UW`\x01\x92a95W[P\x90a7vV[\x80a9C_a9I\x93a\x03\xADV[\x80a\x05\x88V[_a9.V[a9l\x91\x97P=\x80_\x83>a9d\x81\x83a\x03\xADV[\x81\x01\x90a2\xC5V[\x95_a8OV[P\x99\x90\x97\x91\x98`\x01\x90a7nV[a9\x9A\x91\x9AP` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x98_a7!V[\x96\x97\x96\x95P`\x01\x90\x92\x01\x91a6\xB4V[PPPPP\x91\x90Pa9\xE1a9\xD4\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3V[PPPc\xFF\xFF\xFF\xFF3\x91\x16\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05_\x80\xA3V[`@Q\x90a:M\x82a\x03\x92V[_``\x83\x82\x81R\x82` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81R\x81Q` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`\x80c\xFF\xFF\xFF\xFF``a:\x9F`@\x86\x01Q\x84\x83\x87\x01R`\xA0\x86\x01\x90a3xV[\x94\x01Q\x16\x91\x01R\x90V[`@Q\x90a:\xB6\x82a\x03rV[``` \x83\x82\x81R\x01RV[\x15a:\xC9WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a:\xDEWV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a:\xF4WV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a;\nWV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\r\xF1V[_\x19\x81\x01\x91\x90\x82\x11a'\x7FWV[\x15a;CWV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90`\x01\x82\x01\x80\x92\x11a'\x7FWV[\x90`\x02\x82\x01\x80\x92\x11a'\x7FWV[\x90`\x03\x82\x01\x80\x92\x11a'\x7FWV[\x90`\x04\x82\x01\x80\x92\x11a'\x7FWV[\x90`\x05\x82\x01\x80\x92\x11a'\x7FWV[\x91\x90\x82\x01\x80\x92\x11a'\x7FWV[\x15a;\xACWV[c\xAF\xFC^\xDB`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03ZWQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03ZW\x90V[\x15a;\xE3WV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a'\x7FWV[\x15a<\x19WV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a</WV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[\x94\x93\x92\x90\x91\x93a<La:\xA9V[Pa<X\x85\x15\x15a:\xC2V[`@\x84\x01QQ\x85\x14\x80aD\xBAW[\x80aD\xACW[\x80aD\x9EW[a<{\x90a:\xD7V[a<\x8D` \x85\x01QQ\x85QQ\x14a:\xEDV[a<\xA4c\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a;\x03V[a<\xACa\x03\xE0V[_\x81R_` \x82\x01R\x92a<\xBEa:\xA9V[a<\xC7\x87a iV[` \x82\x01Ra<\xD5\x87a iV[\x81Ra<\xDFa:\xA9V[\x92a<\xEE` \x88\x01QQa iV[\x84Ra<\xFE` \x88\x01QQa iV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UWa=g\x91_\x91aDoW[Pa=b6\x8B\x87a\x07\xC1V[aJ\x99V[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a>\xC6W` \x88a=\xBBa\x14L\x8Ca=\xB3\x8F\x96\x86\x8Ea=\x98a59\x86\x80\x95a \xA8V[a=\xA5\x84\x84\x84\x01Qa \xA8V[R\x82a>\x93W[\x01Qa \xA8V[Q\x95Qa \xA8V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x03UWa \x14\x8Aa>h\x8Fa>a\x8F\x84` \x8F\x92a>X\x93a>P\x84`\x01\x9Ea>n\x9E_\x91a>vW[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa \xA8V[R\x01Qa \xA8V[Q\x93\x8DQa \xA8V[Q\x16aJ\xC4V[\x90aJ\xF5V[\x97\x01\x96a=kV[a>\x8D\x91P\x86=\x81\x11a\x14\xD5Wa\x14\xC7\x81\x83a\x03\xADV[_a>AV[a>\xC1a>\xA3\x84\x84\x84\x01Qa \xA8V[Qa>\xBA\x84\x84\x01Qa>\xB4\x87a;.V[\x90a \xA8V[Q\x10a;<V[a=\xACV[P\x90\x95\x97\x94\x96Pa>\xDB\x91\x98\x93\x92\x99PaK\xB2V[\x91a>\xE8`\x97T`\xFF\x16\x90V[\x90\x81\x15aDgW`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91aDHW[P\x91\x90[_\x92[\x81\x84\x10a?\x99WPPPPP\x92a?\x80a?{a?ta?\x93\x95\x85a\x11`\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x1FaV[\x91\x90a<\x12V[a<(V[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a1\xFAV[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8DaCBW[a\x14L\x82`\xA0a?\xEEa#\xC2a*6\x84a?\xF6\x97a?\xE8a?\xDAa59\x8F\x9C`@` \x9F\x9E\x01Qa \xA8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba'\xC9V[\x97\x01Qa \xA8V[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03UWa@\xBAa\x14L\x8F\x95\x8F\x90a@\xB2\x8F\x97\x8F\x96\x84\x8Fa@\xAC`\xC0\x96a@\xA5\x84\x8F` \x9F\x90a=\xACa*6\x99`@\x93a#\xC2\x9C_\x91aC\x14W[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a;\xDCV[Q\x90aF\x84V[\x9Ca'\xC9V[\x96\x01Qa \xA8V[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03UWaAG\x91\x8C\x8F\x92_\x92aB\xF0W[P` aA9\x92\x93\x01Qa \xA8V[\x90`\x01`\x01``\x1B\x03\x16\x90RV[aAg\x8CaA9\x8CaA`a\x12\xCF\x82` \x86\x01Qa \xA8V[\x92Qa \xA8V[_\x98_[` \x8A\x01QQ\x81\x10\x15aB\xD7W\x8B\x8DaA\xA9\x89aA\x9Ca#\xC2a*6\x86\x8F\x89aA\x94\x91Qa \xA8V[Q\x94\x87a'\xC9V[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[aA\xB8W[PP`\x01\x01aAkV[\x8A\x8AaB:\x85\x9F\x94\x8F\x96\x86a)\x84\x8F\x93`\xE0aA\xF1a\x14L\x95` aA\xE9a#\xC2a*6\x83\x9FaA\xFA\x9C\x89\x91a'\xC9V[\x9A\x01Qa \xA8V[Q\x9B\x01Qa \xA8V[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW\x8FaB\xA6\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92aB\xB1W[PaB\xA0aA9\x92\x93Q\x93aB\x9Ba\x12\xCF\x84\x87a \xA8V[a;\xF2V[\x92a \xA8V[\x01\x9A\x90P\x8B\x8DaA\xAEV[aA9\x92PaB\xD0aB\xA0\x91` =\x81\x11a%uWa%f\x81\x83a\x03\xADV[\x92PaB\x83V[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a?CV[aA9\x92PaC\r` \x91\x82=\x81\x11a%uWa%f\x81\x83a\x03\xADV[\x92PaA*V[` aC5\x92P=\x81\x11aC;W[aC-\x81\x83a\x03\xADV[\x81\x01\x90a;\xBBV[_a@\x8FV[P=aC#V[aC\x7F\x94PaC\\\x92Pa#\xC2\x91a*6\x91` \x95a'\xC9V[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UW` \x89a?\xF6\x8F\x93\x8F`\xA0\x8F\x97a#\xC2a*6\x8F\x8F\x90a?\xE8a?\xDAa59\x8F`@\x8B\x96\x91\x8F\x88\x93a\x14L\x9FaD\x03\x90aD\t\x93a?\xEE\x9F_\x92aD\x1FW[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a;\x98V[\x11a;\xA5V[PPPPPP\x97PPPPPP\x92\x93PPa?\xAEV[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91aD@\x91=\x81\x11a!\x84Wa!v\x81\x83a\x03\xADV[\x92\x91PaC\xF2V[aDa\x91P` =` \x11a+HWa+8\x81\x83a\x03\xADV[_a?<V[_\x91\x90a?@V[aD\x91\x91P` =` \x11aD\x97W[aD\x89\x81\x83a\x03\xADV[\x81\x01\x90a;\x19V[_a=VV[P=aD\x7FV[P`\xE0\x84\x01QQ\x85\x14a<rV[P`\xC0\x84\x01QQ\x85\x14a<lV[P`\xA0\x84\x01QQ\x85\x14a<fV[\x15aD\xCFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aE4\x90aL\x8FV[`\xCE\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xCF\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x83\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\xD0\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xCD\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aF\x07\x82a\x03rV[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aF#\x81\x84a\x03\xADV[6\x837V[`@Q\x90aF7` \x83a\x03\xADV[` 6\x837V[\x91\x90`@\x90``aFMaE\xFAV[\x94\x85\x92` \x85Q\x92aF_\x85\x85a\x03\xADV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aF\x82WV[\xFE[` \x92\x91`\x80`@\x92aF\x95aE\xFAV[\x95\x86\x93\x81\x86Q\x93aF\xA6\x86\x86a\x03\xADV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aF\x82W\x15aF\xD7WV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@QaF\xF2\x81a\x03rV[`@\x90\x81QaG\x01\x83\x82a\x03\xADV[\x826\x827\x81R` \x82Q\x91aG\x16\x84\x84a\x03\xADV[\x836\x847\x01R\x80QaG(\x82\x82a\x03\xADV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aG~\x83\x83a\x03\xADV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaG\xD3\x83Q\x93\x84a\x03\xADV[\x82R` \x82\x01R\x90V[_Q` aN\x1A_9_Q\x90_R\x90aG\xF4aE\xFAV[P_\x91\x90\x06` `\xC0\x83[aH\xF4W_\x93_Q` aN\x1A_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaH*\x85\x82a\x03\xADV[\x846\x827\x84\x81\x85`@QaH>\x82\x82a\x03\xADV[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aN\x1A_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aF\x82WaH\xA8\x90aN\x03V[Q\x91aH\xF4W_Q` aN\x1A_9_Q\x90_R\x82\x80\t\x14aH\xDFWP_Q` aN\x1A_9_Q\x90_R`\x01_\x94\x08\x92\x93aG\xFFV[\x92\x93PPaH\xEBa\x03\xE0V[\x92\x83R\x82\x01R\x90V[a\x1FMV[aI\x01aE\xFAV[P`@QaI\x0E\x81a\x03rV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x0C\x81\x10\x15a\x1FHW`\x05\x1B\x01\x90V[\x93\x92\x90\x91aI;`@a\x04\rV[\x94\x85R` \x85\x01RaIM`@a\x04\rV[\x91\x82R` \x82\x01RaI]aF\x12V[\x92_[`\x02\x81\x10aI\x8AWPPP` a\x01\x80\x92aIyaF(V[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aI\x96`\x01\x92a/\x0BV[aI\xA0\x82\x85a\x1F7V[QQaI\xAC\x82\x89aI\x1CV[R` aI\xB9\x83\x86a\x1F7V[Q\x01QaI\xCEaI\xC8\x83a;RV[\x89aI\x1CV[RaI\xD9\x82\x86a\x1F7V[QQQaI\xE8aI\xC8\x83a;`V[RaI\xFEaI\xF6\x83\x87a\x1F7V[QQ` \x01\x90V[QaJ\x0BaI\xC8\x83a;nV[R` aJ\x18\x83\x87a\x1F7V[Q\x01QQaJ(aI\xC8\x83a;|V[RaJTaJNaJG` aJ>\x86\x8Aa\x1F7V[Q\x01Q` \x01\x90V[Q\x92a;\x8AV[\x88aI\x1CV[R\x01aI`V[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01aJ\xA7`\xFF\x93aM\x8BV[\x92\x83\x92\x16\x1B\x11\x15aJ\xB5W\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[aJ\xD0WP\x90V[_\x19\x81\x01\x81\x81\x11a'\x7FWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a'\x7FW`\x01\x01\x90\x80aJ\xC8V[\x90aJ\xFEaE\xFAV[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aK\xA3W`\x01\x82\x14aK\x9EWaK\x1Fa\x03\xE0V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aKDWPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aK~W[`\x01aKtaKi\x83`\xFF\x94aF\x84V[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aK0V[\x94`\x01aKtaKiaK\x93\x89`\xFF\x95aF\x84V[\x98\x93PPPPaKXV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[aK\xBAaE\xFAV[P\x80Q\x90\x81\x15\x80aL+W[\x15aK\xE7WPP`@QaK\xDB`@\x82a\x03\xADV[_\x81R_` \x82\x01R\x90V[` _Q` aN\x1A_9_Q\x90_R\x91\x01Q\x06_Q` aN\x1A_9_Q\x90_R\x03_Q` aN\x1A_9_Q\x90_R\x81\x11a'\x7FW`@Q\x91aG\xD3\x83a\x03rV[P` \x81\x01Q\x15aK\xC6V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aLKWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\xFF\xFFaL\xE3\x82aJ\xC4V[\x16aL\xED\x81a\x07\xA6V[\x90aL\xFB`@Q\x92\x83a\x03\xADV[\x80\x82RaM\n`\x1F\x19\x91a\x07\xA6V[\x016` \x83\x017__[\x82Q\x82\x10\x80aMjW[\x15aMcW`\x01\x81\x1B\x84\x16aM<W[aM7\x90a'\xEAV[aM\x14V[\x90`\x01aM7\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaMY\x82\x87a!\xEEV[S\x01\x91\x90PaM.V[PP\x90P\x90V[Pa\x01\0\x81\x10aM\x1EV[\x15aM|WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aM\xF4W\x81Q\x15aM\xEFW` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aM\xEAW`\x01\x90aM\xD5aM\xCBa#\xC2a#\xB4\x86\x89a!\xEEV[`\xFF`\x01\x91\x16\x1B\x90V[\x90aM\xE1\x81\x83\x11aMuV[\x17\x91\x01\x90aM\xACV[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15aN\nWV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x8CTES\x8En\x18\xB0`\xF7`\x80v\xC2\x9E\xBC\xBD\x1C\xF2\xA0\xA3P{\x01\te)\xE6\xD0\0\x0BgdsolcC\0\x08\x1B\x003",
  );
    /// The runtime bytecode of the contract, as deployed on the network.
  ///
  /// ```text
  ///0x60806040526004361015610011575f80fd5b5f3560e01c8063136439dd1461029a578063171f1d5b146102955780631ad43189146101e6578063245a7bfc146102905780632cb223d51461028b5780632d89f6fc1461028657806331b36bd9146102815780633563b0d11461027c5780633998fdd314610277578063416c7e5e146102725780634d2b57fe1461026d5780634f739f7414610268578063595c6a67146102635780635a2d7f021461025e5780635ac86ab7146102595780635baec9a0146102545780635c1556621461024f5780635c975abb1461024a5780635decc3f5146102455780635df4594614610240578063683048351461023b5780636b532e9e146102365780636b92787e146102315780636d14a9871461022c5780636efb463614610227578063715018a61461022257806372d18e8d146102135780637afa1eed1461021d578063886f1195146102185780638b00ce7c146102135780638da5cb5b1461020e5780639b290e9814610209578063b98d090814610204578063ca8aa7c7146101ff578063cc2a9a5b146101fa578063cefdc1d4146101f5578063df5cf723146101f0578063f2fde38b146101eb578063f5c9899d146101e6578063f63c5bab146101e15763fabc1cbc146101dc575f80fd5b611df5565b611dda565b610592565b611d49565b611d05565b611bc1565b611a81565b611a59565b611a37565b611a0f565b6119e7565b611958565b6119a3565b61197b565b6118fd565b611850565b6117d0565b611662565b6115f1565b6115ad565b611569565b61152b565b61150e565b611395565b6110ab565b610dfc565b610dcf565b610d5c565b610cb5565b610aaa565b61094a565b610918565b61089e565b6106f4565b61064c565b610613565b6105d2565b610520565b3461035a57602036600319011261035a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156103555761032492610310915f91610326575b50611ef7565b61031f60665482811614611f0d565b6145c8565b005b610348915060203d60201161034e575b61034081836103ad565b810190611ed7565b5f61030a565b503d610336565b611eec565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761038d57604052565b61035e565b608081019081106001600160401b0382111761038d57604052565b90601f801991011681019081106001600160401b0382111761038d57604052565b604051906103de610100836103ad565b565b604051906103de6040836103ad565b604051906103de6060836103ad565b604051906103de60a0836103ad565b906103de60405192836103ad565b60409060e319011261035a576040519061043482610372565b60e4358252610104356020830152565b919082604091031261035a5760405161045c81610372565b6020808294803584520135910152565b9080601f8301121561035a57604051916104876040846103ad565b82906040810192831161035a57905b8282106104a35750505090565b8135815260209182019101610496565b90608060631983011261035a576040516104cc81610372565b60206104e782946104de81606461046c565b845260a461046c565b910152565b919060808382031261035a5760206104e76040519261050a84610372565b60408496610518838261046c565b86520161046c565b3461035a5761012036600319011261035a57600435604036602319011261035a57610578604091825161055281610372565b60243581526044356020820152610568366104b3565b906105723661041b565b92611f61565b8251911515825215156020820152f35b5f91031261035a57565b3461035a575f36600319011261035a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461035a575f36600319011261035a5760ce546040516001600160a01b039091168152602090f35b63ffffffff81160361035a57565b35906103de826105fa565b3461035a57602036600319011261035a5763ffffffff600435610635816105fa565b165f5260cb602052602060405f2054604051908152f35b3461035a57602036600319011261035a5763ffffffff60043561066e816105fa565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b0381160361035a57565b6001600160401b03811161038d5760051b60200190565b90602080835192838152019201905f5b8181106106ca5750505090565b82518452602093840193909201916001016106bd565b9060206106f19281815201906106ad565b90565b3461035a57604036600319011261035a5760043561071181610685565b602435906001600160401b03821161035a573660238301121561035a5781600401359161073d83610696565b9261074b60405194856103ad565b8084526024602085019160051b8301019136831161035a57602401905b82821061078c5761078861077c86866120cb565b604051918291826106e0565b0390f35b60208091833561079b81610685565b815201910190610768565b6001600160401b03811161038d57601f01601f191660200190565b9291926107cd826107a6565b916107db60405193846103ad565b82948184528183011161035a578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b83831061082257505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b8082106108625750505060208060019297019301930191939290610813565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610843565b3461035a57606036600319011261035a576004356108bb81610685565b6024356001600160401b03811161035a573660238201121561035a57610788916108f26109049236906024816004013591016107c1565b604435916108ff836105fa565b612309565b6040519182916020835260208301906107f7565b3461035a575f36600319011261035a5760cd546040516001600160a01b039091168152602090f35b8015150361035a57565b3461035a57602036600319011261035a5760043561096781610940565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f916109dc575b506001600160a01b031633036109cd5761032490614a5b565b637070f3b160e11b5f5260045ffd5b6109fe915060203d602011610a04575b6109f681836103ad565b810190612190565b5f6109b4565b503d6109ec565b9080601f8301121561035a578135610a2281610696565b92610a3060405194856103ad565b81845260208085019260051b82010192831161035a57602001905b828210610a585750505090565b8135815260209182019101610a4b565b60206040818301928281528451809452019201905f5b818110610a8b5750505090565b82516001600160a01b0316845260209384019390920191600101610a7e565b3461035a57604036600319011261035a57600435610ac781610685565b6024356001600160401b03811161035a57610ae6903690600401610a0b565b610af08151612069565b916001600160a01b03165f5b8251811015610b8d57806020610b15610b3593866120a8565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561035557600192610b69915f91610b6f575b50610b5a83886120a8565b6001600160a01b039091169052565b01610afc565b610b87915060203d8111610a04576109f681836103ad565b5f610b4f565b604051806107888682610a68565b9181601f8401121561035a578235916001600160401b03831161035a576020838186019501011161035a57565b90602080835192838152019201905f5b818110610be55750505090565b825163ffffffff16845260209384019390920191600101610bd8565b90602082526060610c4f610c3a610c2484516080602088015260a0870190610bc8565b6020850151868203601f19016040880152610bc8565b6040840151858203601f190184870152610bc8565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610c8857505050505090565b9091929394602080610ca6600193601f198682030187528951610bc8565b97019301930191939290610c79565b3461035a57608036600319011261035a57600435610cd281610685565b60243590610cdf826105fa565b6044356001600160401b03811161035a57610cfe903690600401610b9b565b91606435926001600160401b03841161035a573660238501121561035a578360040135926001600160401b03841161035a573660248560051b8701011161035a57610788956024610d5096019361281d565b60405191829182610c01565b3461035a575f36600319011261035a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561035557610dc7915f916103265750611ef7565b610324614594565b3461035a575f36600319011261035a57602060405167016345785d8a00008152f35b60ff81160361035a57565b3461035a57602036600319011261035a576020600160ff600435610e1f81610df1565b161b806066541614604051908152f35b9081608091031261035a5790565b604090602319011261035a57602490565b9080601f8301121561035a578135610e6581610696565b92610e7360405194856103ad565b81845260208085019260051b82010192831161035a57602001905b828210610e9b5750505090565b602080918335610eaa816105fa565b815201910190610e8e565b81601f8201121561035a578035610ecb81610696565b92610ed960405194856103ad565b81845260208085019260061b8401019281841161035a57602001915b838310610f03575050505090565b6020604091610f128486610444565b815201920191610ef5565b9080601f8301121561035a578135610f3481610696565b92610f4260405194856103ad565b81845260208085019260051b8201019183831161035a5760208201905b838210610f6e57505050505090565b81356001600160401b03811161035a57602091610f9087848094880101610e4e565b815201910190610f5f565b9190916101808184031261035a57610fb16103ce565b9281356001600160401b03811161035a5781610fce918401610e4e565b845260208201356001600160401b03811161035a5781610fef918401610eb5565b602085015260408201356001600160401b03811161035a5781611013918401610eb5565b604085015261102581606084016104ec565b60608501526110378160e08401610444565b60808501526101208201356001600160401b03811161035a578161105c918401610e4e565b60a08501526101408201356001600160401b03811161035a5781611081918401610e4e565b60c08501526101608201356001600160401b03811161035a576110a49201610f1d565b60e0830152565b3461035a57608036600319011261035a576004356001600160401b03811161035a576110db903690600401610e2f565b6110e436610e3d565b906064356001600160401b03811161035a57611104903690600401610f9b565b60ce549092906001600160a01b0316330361131757611127602083949301612c85565b9161122a6111386040860186612c8f565b92909461119861114a60608901612c85565b9760405161116e81611160602082019485612cc1565b03601f1981018352826103ad565b51902061119161117d88612c85565b63ffffffff165f5260ca60205260405f2090565b5414612d48565b6111c26111bb6111a787612c85565b63ffffffff165f5260cb60205260405f2090565b5415612dba565b8363ffffffff43169661120c6112046111fb7f000000000000000000000000000000000000000000000000000000000000000086612e4b565b63ffffffff1690565b891115612e65565b6040516020810190611222816111608b85612ee5565b519020613c3e565b919060ff5f9616955b8281106112b5577f349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a8686866112756112696103e0565b63ffffffff9094168452565b6020830152604051602081019061129181611160868686612fc8565b5190206112a06111a783612c85565b556112b060405192839283612fc8565b0390a1005b806113116112ed6112e86112dc6112cf60019688516120a8565b516001600160601b031690565b6001600160601b031690565b612ef5565b61130a6112dc8b6113056112cf8760208b01516120a8565b612f34565b1115612f57565b01611233565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b60206040818301928281528451809452019201905f5b81811061137f5750505090565b8251845260209384019390920191600101611372565b3461035a57606036600319011261035a576004356113b281610685565b6024356001600160401b03811161035a576113d1903690600401610a0b565b604435916113de836105fa565b6040516361c8a12f60e11b8152906001600160a01b03165f8280611406868860048401612ff2565b0381845afa918215610355575f926114ea575b506114248351612069565b935f5b84518110156114dc5761143a81866120a8565b519060208361145661144c84896120a8565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa8015610355576001925f916114ae575b50828060c01b03166114a782896120a8565b5201611427565b6114cf915060203d81116114d5575b6114c781836103ad565b810190612794565b5f611495565b503d6114bd565b60405180610788888261135c565b6115079192503d805f833e6114ff81836103ad565b810190612663565b905f611419565b3461035a575f36600319011261035a576020606654604051908152f35b3461035a57602036600319011261035a5763ffffffff60043561154d816105fa565b165f5260cc602052602060ff60405f2054166040519015158152f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a5760c036600319011261035a576004356001600160401b03811161035a57611621903690600401610e2f565b61162a36610e3d565b90604036606319011261035a5760a4356001600160401b03811161035a576103249261165c6064923690600401610eb5565b92613440565b3461035a57606036600319011261035a57602435600435611682826105fa565b6044356001600160401b03811161035a576116a1903690600401610b9b565b60cf5491939092916001600160a01b03163303611781576103249361176b936116eb6116f2936116cf613a40565b9586524363ffffffff16602087015263ffffffff166060860152565b36916107c1565b6040820152604051602081019061170d816111608585613a64565b51902061172261117d60c95463ffffffff1690565b5560c95463ffffffff16907f1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c486040518061176363ffffffff86169482613a64565b0390a2612e1b565b63ffffffff1663ffffffff1960c954161760c955565b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b90602080835192838152019201905f5b8181106118315750505090565b82516001600160601b0316845260209384019390920191600101611824565b3461035a57608036600319011261035a576004356024356001600160401b03811161035a57611883903690600401610b9b565b9091604435611891816105fa565b606435926001600160401b03841161035a576118f3946118b86118be953690600401610f9b565b93613c3e565b6040519283926040845260206118df82516040808801526080870190611814565b910151848203603f19016060860152611814565b9060208301520390f35b3461035a575f36600319011261035a57611915614c37565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461035a575f36600319011261035a57602063ffffffff60c95416604051908152f35b3461035a575f36600319011261035a5760cf546040516001600160a01b039091168152602090f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a575f36600319011261035a576033546040516001600160a01b039091168152602090f35b3461035a575f36600319011261035a5760d0546040516001600160a01b039091168152602090f35b3461035a575f36600319011261035a57602060ff609754166040519015158152f35b3461035a575f36600319011261035a5760d1546040516001600160a01b039091168152602090f35b3461035a5760c036600319011261035a57600435611a9e81610685565b611b1e602435611aad81610685565b604435611ab981610685565b606435611ac581610685565b60843591611ad283610685565b60a43593611adf85610685565b5f5496611b0460ff60088a901c16158099819a611b9c575b8115611b7c575b506144c8565b87611b15600160ff195f5416175f55565b611b655761452b565b611b2457005b611b3261ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016112b0565b611b7761010061ff00195f5416175f55565b61452b565b303b15915081611b8e575b505f611afe565b60ff1660011490505f611b87565b600160ff8216109150611af7565b6040906106f19392815281602082015201906107f7565b3461035a57606036600319011261035a57600435611bde81610685565b602435604435611bed816105fa565b611c2e611bf8612047565b9280611c038561209b565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401612ff2565b0381875afa9384156103555783611c586111fb61144c611c8d986020975f91611ceb575b5061209b565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561035557611cbc925f91611ccc575b506001600160c01b031692611cb684614cd7565b90612309565b9061078860405192839283611baa565b611ce5915060203d6020116114d5576114c781836103ad565b5f611ca2565b611cff91503d805f833e6114ff81836103ad565b5f611c52565b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a57602036600319011261035a57600435611d6681610685565b611d6e614c37565b6001600160a01b03811615611d865761032490614c8f565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461035a575f36600319011261035a57602060405160648152f35b3461035a57602036600319011261035a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f91611eb8575b506001600160a01b03163303611ea957611e77606654198219811614611f0d565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b611ed1915060203d602011610a04576109f681836103ad565b5f611e56565b9081602091031261035a57516106f181610940565b6040513d5f823e3d90fd5b15611efe57565b631d77d47760e21b5f5260045ffd5b15611f1457565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b906002811015611f485760051b0190565b611f23565b634e487b7160e01b5f52601260045260245ffd5b61203d61201a6120439561201461200d85875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e0840152610100830152611fe481610120840103601f1981018352826103ad565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b809661463e565b90614684565b9261201461202f6120296146e6565b946147dd565b916120386148f9565b61463e565b9161492d565b9091565b6040805190919061205883826103ad565b6001815291601f1901366020840137565b9061207382610696565b61208060405191826103ad565b8281528092612091601f1991610696565b0190602036910137565b805115611f485760200190565b8051821015611f485760209160051b010190565b9081602091031261035a575190565b9190916120d88351612069565b925f5b815181101561218b578060206121046120f761212d94866120a8565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa8015610355576001925f9161215d575b5061215682886120a8565b52016120db565b61217e915060203d8111612184575b61217681836103ad565b8101906120bc565b5f61214b565b503d61216c565b505050565b9081602091031261035a57516106f181610685565b906121af82610696565b6121bc60405191826103ad565b828152602081936121cf601f1991610696565b0191015f5b8281106121e057505050565b6060828201526020016121d4565b908151811015611f48570160200190565b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a57815161223281610696565b9261224060405194856103ad565b81845260208085019260051b82010192831161035a57602001905b8282106122685750505090565b815181526020918201910161225b565b9061228282610696565b61228f60405191826103ad565b82815280926122a0601f1991610696565b015f5b8181106122af57505050565b6040519060608201918083106001600160401b0384111761038d576020926040525f81525f838201525f6040820152828286010152016122a3565b9081602091031261035a57516001600160601b038116810361035a5790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa948515610355575f9561261e575b50604051634f4c91e160e11b815294602086600481855afa918215610355576004965f936125fc575b5060209060405197888092632efa2ca360e11b82525afa958615610355575f966125db575b5061239785939295516121a5565b945f935b80518510156125d1576123c86123c26123b487846121ee565b516001600160f81b03191690565b60f81c90565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa938415610355575f946125ad575b506124188451612278565b612422888b6120a8565b5261242d878a6120a8565b505f5b845181101561259c5780602061244961246b93886120a8565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610355575f9261257c575b5061249181876120a8565b518a60208a6124a0858b6120a8565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa938415610355576125338c8f61252e6001986125459789975f9261254c575b5061251961250a6103ef565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b6120a8565b519061253f83836120a8565b526120a8565b5001612430565b61256e91925060203d8111612575575b61256681836103ad565b8101906122ea565b905f6124fe565b503d61255c565b61259591925060203d8111610a04576109f681836103ad565b905f612486565b50600190960195909450915061239b565b6125ca9194503d805f833e6125c281836103ad565b8101906121ff565b925f61240d565b5050509350505090565b6125f591965060203d602011610a04576109f681836103ad565b945f612389565b602091935061261790823d8411610a04576109f681836103ad565b9290612364565b61263891955060203d602011610a04576109f681836103ad565b935f61233b565b6040519061264c82610392565b606080838181528160208201528160408201520152565b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a57815161269681610696565b926126a460405194856103ad565b81845260208085019260051b82010192831161035a57602001905b8282106126cc5750505090565b6020809183516126db816105fa565b8152019101906126bf565b63ffffffff909116815260406020820181905281018390526001600160fb1b03831161035a5760609260051b809284830137010190565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6106f19593168152816020820152019161271d565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff811461277f5760010190565b61275a565b9190811015611f485760051b0190565b9081602091031261035a57516001600160c01b038116810361035a5790565b156127ba57565b6325ec6c1f60e01b5f5260045ffd5b90821015611f48570190565b9081602091031261035a57516106f1816105fa565b5f19811461277f5760010190565b9161281660209263ffffffff9296959660408652604086019161271d565b9416910152565b959394959290919261282d61263f565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa948515610355575f95612c64575b5061286a61263f565b946040516361c8a12f60e11b81525f818061288a8d8d8b600485016126e6565b0381885afa908115610355575f91612c4a575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f81806128cd85878b6004850161273d565b0381875afa908115610355575f91612c30575b5060408701526128ef816121a5565b9860608701998a525f5b60ff811683811015612b7b57885f612922838f61291588612069565b90519061253f83836120a8565b505f8a868f5b8184106129a5575050505090508c61293f82612069565b915f5b81811061296c57505091612961916129679493519061253f83836120a8565b5061276e565b6128f9565b8061299f61298a61144c6001946129848a89516120a8565b516120a8565b61299483886120a8565b9063ffffffff169052565b01612942565b61144c846129ba81602096956129c295612784565b3597516120a8565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561035557888f888a918f94612a676001612a5a81938d809d5f92612b4f575b506123c2612a36612a4492612a2f878060c01b03861615156127b3565b8b8d6127c9565b356001600160f81b03191690565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14612a83575b5050505050600191925001908a918a868f612928565b8597612aa593612a9e60209799986123c295612a3695612784565b35956127c9565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa908115610355578f612b0390612b089383886001975f93612b17575b50612984906129949394516120a8565b6127ea565b905082918a888f888a91612a6d565b612994935090612b406129849260203d8111612b48575b612b3881836103ad565b8101906127d5565b935090612af3565b503d612b2e565b612a44919250612a36612b726123c29260203d81116114d5576114c781836103ad565b93925050612a12565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa90811561035557612bd1945f948593612c0f575b5060405163354952a360e21b815295869485938493600485016127f8565b03916001600160a01b03165afa908115610355575f91612bf5575b50602082015290565b612c0991503d805f833e6114ff81836103ad565b5f612bec565b612c2991935060203d602011610a04576109f681836103ad565b915f612bb3565b612c4491503d805f833e6114ff81836103ad565b5f6128e0565b612c5e91503d805f833e6114ff81836103ad565b5f61289d565b612c7e91955060203d602011610a04576109f681836103ad565b935f612861565b356106f1816105fa565b903590601e198136030182121561035a57018035906001600160401b03821161035a5760200191813603831361035a57565b602081528135602082015263ffffffff6020830135612cdf816105fa565b1660408201526040820135601e198336030181121561035a578201906020823592016001600160401b03831161035a57823603811361035a57612d3d6060612d366080936106f196858488015260a087019161271d565b9501610608565b63ffffffff16910152565b15612d4f57565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15612dc157565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b63ffffffff60019116019063ffffffff821161277f57565b63ffffffff60649116019063ffffffff821161277f57565b9063ffffffff8091169116019063ffffffff821161277f57565b15612e6c57565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6020809163ffffffff8135612edb816105fa565b1684520135910152565b6040810192916103de9190612ec7565b9060648202918083046064149015171561277f57565b9060068202918083046006149015171561277f57565b8181029291811591840414171561277f57565b906001600160601b03809116911602906001600160601b03821691820361277f57565b15612f5e57565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091612fde846080810197612ec7565b63ffffffff81511660408501520151910152565b60409063ffffffff6106f1949316815281602082015201906106ad565b1561301657565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b909291602060609161307b846080810197612ec7565b63ffffffff813561308b816105fa565b1660408501520135910152565b1561309f57565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b1561311157565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b1561318f57565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b8181106132245750505090565b8251845260209384019390920191600101613217565b1561324157565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a5781516132f881610696565b9261330660405194856103ad565b81845260208085019260051b82010192831161035a57602001905b82821061332e5750505090565b60208091835161333d81610685565b815201910190613321565b604051906133576040836103ad565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b81811061342157505050608061340c6106f194956060850151601f1985830301848601526106ad565b9201519060a0601f1982850301910152613378565b82516001600160a01b03168752602096870196909201916001016133e3565b909291600161344e85612c85565b94602061350a85356134796134718a63ffffffff165f5260cb60205260405f2090565b54151561300f565b6134b46134948a63ffffffff165f5260cb60205260405f2090565b54604051858101906134ab816111608c8b86613065565b51902014613098565b6134df6134d96134d28b63ffffffff165f5260cc60205260405f2090565b5460ff1690565b1561310a565b6135046134f66111fb6134f189612c85565b612e33565b63ffffffff43161115613188565b80612f21565b9101351414613a0f5761351d8351612069565b935f5b845181101561355d578061354c613539600193886120a8565b5180515f526020015160205260405f2090565b61355682896120a8565b5201613520565b5090929391946135976020870194602061357687612c85565b60405161358b816111608a86830195866131fa565b5190209101351461323a565b6135a18551612069565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b8751811015613651578060206135e861360893896120a8565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa9182156103555760019261362d915f91613633575b50610b5a838d6120a8565b016135cf565b61364b915060203d8111610a04576109f681836103ad565b5f613622565b50929695509250926136b061367961368160408601946136718688612c8f565b939091612c85565b9236916107c1565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612309565b945f915b86518310156139b1575f979697955b6136cd848a6120a8565b51518710156139a1579761370e986020806136ec8a61298489876120a8565b510151604051809c81926308f6629d60e31b8352600483019190602083019252565b0381865afa998a15610355575f9a613981575b506001985f5b85518110156139735761374c6137406120f783896120a8565b6001600160a01b031690565b6001600160a01b038d161461376357600101613727565b5099909791985060015f5b151514613783575b50600101959796976136c3565b98969786985f879593986137e860ff6137c06123c2612a368c6137ba61383d9f6137b460cd5460018060a01b031690565b98612c8f565b906127c9565b6137da6137cb6103e0565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d1546137ff90613740906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529098899190829081906044820190565b03915afa968715610355575f9761394f575b5061385a8751612069565b985f5b8a51811015613883578067016345785d8a000061387c6001938e6120a8565b520161385d565b509a9294966138c160ff6138a86123c2612a368f9d979f969e966137ba8e8e92612c8f565b6138b361250a6103fe565b1663ffffffff166020860152565b604084015260608301526138d3613348565b608083015260d0546138ef90613740906001600160a01b031681565b803b1561035a57604051636a669b4160e01b8152925f91849182908490829061391b906004830161339c565b03925af191821561035557600192613935575b5090613776565b806139435f613949936103ad565b80610588565b5f61392e565b61396c9197503d805f833e61396481836103ad565b8101906132c5565b955f61384f565b50999097919860019061376e565b61399a919a5060203d8111610a04576109f681836103ad565b985f613721565b96979695506001909201916136b4565b50505050509190506139e16139d48263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b50505063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b60405190613a4d82610392565b5f6060838281528260208201528160408201520152565b602081528151602082015263ffffffff6020830151166040820152608063ffffffff6060613a9f6040860151848387015260a0860190613378565b9401511691015290565b60405190613ab682610372565b60606020838281520152565b15613ac957565b62f8202d60e51b5f5260045ffd5b15613ade57565b6343714afd60e01b5f5260045ffd5b15613af457565b635f832f4160e01b5f5260045ffd5b15613b0a57565b634b874f4560e01b5f5260045ffd5b9081602091031261035a57516106f181610df1565b5f1981019190821161277f57565b15613b4357565b633fdc650560e21b5f5260045ffd5b906001820180921161277f57565b906002820180921161277f57565b906003820180921161277f57565b906004820180921161277f57565b906005820180921161277f57565b9190820180921161277f57565b15613bac57565b63affc5edb60e01b5f5260045ffd5b9081602091031261035a575167ffffffffffffffff198116810361035a5790565b15613be357565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b03821161277f57565b15613c1957565b6367988d3360e01b5f5260045ffd5b15613c2f57565b63ab1b236b60e01b5f5260045ffd5b949392909193613c4c613aa9565b50613c58851515613ac2565b6040840151518514806144ba575b806144ac575b8061449e575b613c7b90613ad7565b613c8d60208501515185515114613aed565b613ca463ffffffff431663ffffffff841610613b03565b613cac6103e0565b5f81525f602082015292613cbe613aa9565b613cc787612069565b6020820152613cd587612069565b8152613cdf613aa9565b92613cee602088015151612069565b8452613cfe602088015151612069565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561035557613d67915f9161446f575b50613d62368b876107c1565b614a99565b985f965b60208901518051891015613ec657602088613dbb61144c8c613db38f96868e613d986135398680956120a8565b613da584848401516120a8565b5282613e93575b01516120a8565b5195516120a8565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa918215610355576120148a613e688f613e618f8460208f92613e5893613e508460019e613e6e9e5f91613e76575b508f8060c01b031692516120a8565b5201516120a8565b51938d516120a8565b5116614ac4565b90614af5565b970196613d6b565b613e8d9150863d81116114d5576114c781836103ad565b5f613e41565b613ec1613ea384848401516120a8565b51613eba84840151613eb487613b2e565b906120a8565b5110613b3c565b613dac565b50909597949650613edb919893929950614bb2565b91613ee860975460ff1690565b908115614467576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f91614448575b5091905b5f925b818410613f9957505050505092613f80613f7b613f74613f9395856111609860806060602099015192015192611f61565b9190613c12565b613c28565b01516040519283916020830195866131fa565b51902090565b92989596909399919794878b888c888d614342575b61144c8260a0613fee6123c2612a3684613ff697613fe8613fda6135398f9c604060209f9e01516120a8565b67ffffffffffffffff191690565b9b6127c9565b9701516120a8565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa908115610355576140ba61144c8f958f906140b28f978f96848f6140ac60c0966140a5848f60209f90613dac612a36996040936123c29c5f91614314575b5067ffffffffffffffff19918216911614613bdc565b5190614684565b9c6127c9565b9601516120a8565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561035557614147918c8f925f926142f0575b506020614139929301516120a8565b906001600160601b03169052565b6141678c6141398c6141606112cf8260208601516120a8565b92516120a8565b5f985f5b60208a0151518110156142d7578b8d6141a98961419c6123c2612a36868f8961419491516120a8565b5194876127c9565b60ff161c60019081161490565b6141b8575b505060010161416b565b8a8a61423a859f948f96866129848f9360e06141f161144c9560206141e96123c2612a36839f6141fa9c89916127c9565b9a01516120a8565b519b01516120a8565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355578f6142a6908f936001959486955f926142b1575b506142a06141399293519361429b6112cf84876120a8565b613bf2565b926120a8565b019a90508b8d6141ae565b61413992506142d06142a09160203d81116125755761256681836103ad565b9250614283565b5093919796996001919699509a94929a01929190613f43565b614139925061430d602091823d81116125755761256681836103ad565b925061412a565b602061433592503d811161433b575b61432d81836103ad565b810190613bbb565b5f61408f565b503d614323565b61437f945061435c92506123c291612a36916020956127c9565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561035557602089613ff68f938f60a08f976123c2612a368f8f90613fe8613fda6135398f60408b96918f889361144c9f6144039061440993613fee9f5f9261441f575b5063ffffffff809116931690613b98565b11613ba5565b5050505050509750505050505092935050613fae565b602063ffffffff9293508291614440913d81116121845761217681836103ad565b9291506143f2565b614461915060203d602011612b4857612b3881836103ad565b5f613f3c565b5f9190613f40565b614491915060203d602011614497575b61448981836103ad565b810190613b19565b5f613d56565b503d61447f565b5060e0840151518514613c72565b5060c0840151518514613c6c565b5060a0840151518514613c66565b156144cf57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b61453490614c8f565b60ce80546001600160a01b03199081166001600160a01b039384161790915560cf805482169383169390931790925560d1805483169382169390931790925560d0805482169383169390931790925560cd80549092169216919091179055565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b6040519061460782610372565b5f6020838281520152565b6040519061018061462381846103ad565b368337565b604051906146376020836103ad565b6020368337565b9190604090606061464d6145fa565b948592602085519261465f85856103ad565b8436853780518452015160208301528482015260076107cf195a01fa1561468257565bfe5b6020929160806040926146956145fa565b958693818651936146a686866103ad565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa801561468257156146d757565b63d4b68fd760e01b5f5260045ffd5b6040516146f281610372565b604090815161470183826103ad565b823682378152602082519161471684846103ad565b833684370152805161472882826103ad565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061477e83836103ad565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208301526147d3835193846103ad565b8252602082015290565b5f516020614e1a5f395f51905f52906147f46145fa565b505f919006602060c0835b6148f4575f935f516020614e1a5f395f51905f526003818681818009090860405161482a85826103ad565b8436823784818560405161483e82826103ad565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614e1a5f395f51905f5260a082015260056107cf195a01fa8015614682576148a890614e03565b51916148f4575f516020614e1a5f395f51905f52828009146148df57505f516020614e1a5f395f51905f5260015f940892936147ff565b929350506148eb6103e0565b92835282015290565b611f4d565b6149016145fa565b5060405161490e81610372565b600181526002602082015290565b90600c811015611f485760051b0190565b9392909161493b604061040d565b948552602085015261494d604061040d565b918252602082015261495d614612565b925f5b6002811061498a57505050602061018092614979614628565b93849160086201d4c0fa9151151590565b80614996600192612f0b565b6149a08285611f37565b51516149ac828961491c565b5260206149b98386611f37565b5101516149ce6149c883613b52565b8961491c565b526149d98286611f37565b5151516149e86149c883613b60565b526149fe6149f68387611f37565b515160200190565b51614a0b6149c883613b6e565b526020614a188387611f37565b51015151614a286149c883613b7c565b52614a54614a4e614a476020614a3e868a611f37565b51015160200190565b5192613b8a565b8861491c565b5201614960565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614aa760ff93614d8b565b928392161b1115614ab55790565b63ca95733360e01b5f5260045ffd5b805f915b614ad0575090565b5f19810181811161277f5761ffff9116911661ffff811461277f576001019080614ac8565b90614afe6145fa565b5061ffff811690610200821015614ba35760018214614b9e57614b1f6103e0565b5f81525f602082015292906001905f925b61ffff8316851015614b4457505050505090565b600161ffff831660ff86161c811614614b7e575b6001614b74614b698360ff94614684565b9460011b61fffe1690565b9401169291614b30565b946001614b74614b69614b938960ff95614684565b989350505050614b58565b505090565b637fc4ea7d60e11b5f5260045ffd5b614bba6145fa565b50805190811580614c2b575b15614be7575050604051614bdb6040826103ad565b5f81525f602082015290565b60205f516020614e1a5f395f51905f52910151065f516020614e1a5f395f51905f52035f516020614e1a5f395f51905f52811161277f57604051916147d383610372565b50602081015115614bc6565b6033546001600160a01b03163303614c4b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff614ce382614ac4565b16614ced816107a6565b90614cfb60405192836103ad565b808252614d0a601f19916107a6565b013660208301375f5f5b8251821080614d6a575b15614d63576001811b8416614d3c575b614d37906127ea565b614d14565b906001614d379160ff60f81b8460f81b165f1a614d5982876121ee565b5301919050614d2e565b5050905090565b506101008110614d1e565b15614d7c57565b631019106960e31b5f5260045ffd5b90610100825111614df457815115614def57602082015160019060f81c81901b5b8351821015614dea57600190614dd5614dcb6123c26123b486896121ee565b60ff600191161b90565b90614de1818311614d75565b17910190614dac565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b15614e0a57565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212208c5445538e6e18b060f7608076c29ebcbd1cf2a0a3507b01096529e6d0000b6764736f6c634300081b0033
  /// ```
  #[rustfmt::skip]
  #[allow(clippy::all)]
  pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
      b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x13d9\xDD\x14a\x02\x9AW\x80c\x17\x1F\x1D[\x14a\x02\x95W\x80c\x1A\xD41\x89\x14a\x01\xE6W\x80c$Z{\xFC\x14a\x02\x90W\x80c,\xB2#\xD5\x14a\x02\x8BW\x80c-\x89\xF6\xFC\x14a\x02\x86W\x80c1\xB3k\xD9\x14a\x02\x81W\x80c5c\xB0\xD1\x14a\x02|W\x80c9\x98\xFD\xD3\x14a\x02wW\x80cAl~^\x14a\x02rW\x80cM+W\xFE\x14a\x02mW\x80cOs\x9Ft\x14a\x02hW\x80cY\\jg\x14a\x02cW\x80cZ-\x7F\x02\x14a\x02^W\x80cZ\xC8j\xB7\x14a\x02YW\x80c[\xAE\xC9\xA0\x14a\x02TW\x80c\\\x15Vb\x14a\x02OW\x80c\\\x97Z\xBB\x14a\x02JW\x80c]\xEC\xC3\xF5\x14a\x02EW\x80c]\xF4YF\x14a\x02@W\x80ch0H5\x14a\x02;W\x80ckS.\x9E\x14a\x026W\x80ck\x92x~\x14a\x021W\x80cm\x14\xA9\x87\x14a\x02,W\x80cn\xFBF6\x14a\x02'W\x80cqP\x18\xA6\x14a\x02\"W\x80cr\xD1\x8E\x8D\x14a\x02\x13W\x80cz\xFA\x1E\xED\x14a\x02\x1DW\x80c\x88o\x11\x95\x14a\x02\x18W\x80c\x8B\0\xCE|\x14a\x02\x13W\x80c\x8D\xA5\xCB[\x14a\x02\x0EW\x80c\x9B)\x0E\x98\x14a\x02\tW\x80c\xB9\x8D\t\x08\x14a\x02\x04W\x80c\xCA\x8A\xA7\xC7\x14a\x01\xFFW\x80c\xCC*\x9A[\x14a\x01\xFAW\x80c\xCE\xFD\xC1\xD4\x14a\x01\xF5W\x80c\xDF\\\xF7#\x14a\x01\xF0W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xEBW\x80c\xF5\xC9\x89\x9D\x14a\x01\xE6W\x80c\xF6<[\xAB\x14a\x01\xE1Wc\xFA\xBC\x1C\xBC\x14a\x01\xDCW_\x80\xFD[a\x1D\xF5V[a\x1D\xDAV[a\x05\x92V[a\x1DIV[a\x1D\x05V[a\x1B\xC1V[a\x1A\x81V[a\x1AYV[a\x1A7V[a\x1A\x0FV[a\x19\xE7V[a\x19XV[a\x19\xA3V[a\x19{V[a\x18\xFDV[a\x18PV[a\x17\xD0V[a\x16bV[a\x15\xF1V[a\x15\xADV[a\x15iV[a\x15+V[a\x15\x0EV[a\x13\x95V[a\x10\xABV[a\r\xFCV[a\r\xCFV[a\r\\V[a\x0C\xB5V[a\n\xAAV[a\tJV[a\t\x18V[a\x08\x9EV[a\x06\xF4V[a\x06LV[a\x06\x13V[a\x05\xD2V[a\x05 V[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03UWa\x03$\x92a\x03\x10\x91_\x91a\x03&W[Pa\x1E\xF7V[a\x03\x1F`fT\x82\x81\x16\x14a\x1F\rV[aE\xC8V[\0[a\x03H\x91P` =` \x11a\x03NW[a\x03@\x81\x83a\x03\xADV[\x81\x01\x90a\x1E\xD7V[_a\x03\nV[P=a\x036V[a\x1E\xECV[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[a\x03^V[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[`@Q\x90a\x03\xDEa\x01\0\x83a\x03\xADV[V[`@Q\x90a\x03\xDE`@\x83a\x03\xADV[`@Q\x90a\x03\xDE``\x83a\x03\xADV[`@Q\x90a\x03\xDE`\xA0\x83a\x03\xADV[\x90a\x03\xDE`@Q\x92\x83a\x03\xADV[`@\x90`\xE3\x19\x01\x12a\x03ZW`@Q\x90a\x044\x82a\x03rV[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x03ZW`@Qa\x04\\\x81a\x03rV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW`@Q\x91a\x04\x87`@\x84a\x03\xADV[\x82\x90`@\x81\x01\x92\x83\x11a\x03ZW\x90[\x82\x82\x10a\x04\xA3WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x04\x96V[\x90`\x80`c\x19\x83\x01\x12a\x03ZW`@Qa\x04\xCC\x81a\x03rV[` a\x04\xE7\x82\x94a\x04\xDE\x81`da\x04lV[\x84R`\xA4a\x04lV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03ZW` a\x04\xE7`@Q\x92a\x05\n\x84a\x03rV[`@\x84\x96a\x05\x18\x83\x82a\x04lV[\x86R\x01a\x04lV[4a\x03ZWa\x01 6`\x03\x19\x01\x12a\x03ZW`\x045`@6`#\x19\x01\x12a\x03ZWa\x05x`@\x91\x82Qa\x05R\x81a\x03rV[`$5\x81R`D5` \x82\x01Ra\x05h6a\x04\xB3V[\x90a\x05r6a\x04\x1BV[\x92a\x1FaV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x03ZWV[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03ZWV[5\x90a\x03\xDE\x82a\x05\xFAV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x065\x81a\x05\xFAV[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x06n\x81a\x05\xFAV[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03ZWV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x8DW`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xCAWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\xBDV[\x90` a\x06\xF1\x92\x81\x81R\x01\x90a\x06\xADV[\x90V[4a\x03ZW`@6`\x03\x19\x01\x12a\x03ZW`\x045a\x07\x11\x81a\x06\x85V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW6`#\x83\x01\x12\x15a\x03ZW\x81`\x04\x015\x91a\x07=\x83a\x06\x96V[\x92a\x07K`@Q\x94\x85a\x03\xADV[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03ZW`$\x01\x90[\x82\x82\x10a\x07\x8CWa\x07\x88a\x07|\x86\x86a \xCBV[`@Q\x91\x82\x91\x82a\x06\xE0V[\x03\x90\xF3[` \x80\x91\x835a\x07\x9B\x81a\x06\x85V[\x81R\x01\x91\x01\x90a\x07hV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x8DW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xCD\x82a\x07\xA6V[\x91a\x07\xDB`@Q\x93\x84a\x03\xADV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03ZW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x08\"WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x08bWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x08\x13V[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\x08CV[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a\x08\xBB\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW6`#\x82\x01\x12\x15a\x03ZWa\x07\x88\x91a\x08\xF2a\t\x04\x926\x90`$\x81`\x04\x015\x91\x01a\x07\xC1V[`D5\x91a\x08\xFF\x83a\x05\xFAV[a#\tV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xF7V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCDT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x03ZWV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045a\tg\x81a\t@V[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a\t\xDCW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\t\xCDWa\x03$\x90aJ[V[cpp\xF3\xB1`\xE1\x1B_R`\x04_\xFD[a\t\xFE\x91P` =` \x11a\n\x04W[a\t\xF6\x81\x83a\x03\xADV[\x81\x01\x90a!\x90V[_a\t\xB4V[P=a\t\xECV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\n\"\x81a\x06\x96V[\x92a\n0`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a\nXWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\nKV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\n\x8BWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n~V[4a\x03ZW`@6`\x03\x19\x01\x12a\x03ZW`\x045a\n\xC7\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\n\xE6\x906\x90`\x04\x01a\n\x0BV[a\n\xF0\x81Qa iV[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\x0B\x8DW\x80` a\x0B\x15a\x0B5\x93\x86a \xA8V[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x03UW`\x01\x92a\x0Bi\x91_\x91a\x0BoW[Pa\x0BZ\x83\x88a \xA8V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\n\xFCV[a\x0B\x87\x91P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a\x0BOV[`@Q\x80a\x07\x88\x86\x82a\nhV[\x91\x81`\x1F\x84\x01\x12\x15a\x03ZW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03ZW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03ZWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0B\xE5WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\xD8V[\x90` \x82R``a\x0COa\x0C:a\x0C$\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\x0B\xC8V[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\x0B\xC8V[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\x0B\xC8V[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0C\x88WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0C\xA6`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x0B\xC8V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0CyV[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045a\x0C\xD2\x81a\x06\x85V[`$5\x90a\x0C\xDF\x82a\x05\xFAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x0C\xFE\x906\x90`\x04\x01a\x0B\x9BV[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZW6`#\x85\x01\x12\x15a\x03ZW\x83`\x04\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZW6`$\x85`\x05\x1B\x87\x01\x01\x11a\x03ZWa\x07\x88\x95`$a\rP\x96\x01\x93a(\x1DV[`@Q\x91\x82\x91\x82a\x0C\x01V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UWa\r\xC7\x91_\x91a\x03&WPa\x1E\xF7V[a\x03$aE\x94V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Qg\x01cEx]\x8A\0\0\x81R\xF3[`\xFF\x81\x16\x03a\x03ZWV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW` `\x01`\xFF`\x045a\x0E\x1F\x81a\r\xF1V[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[\x90\x81`\x80\x91\x03\x12a\x03ZW\x90V[`@\x90`#\x19\x01\x12a\x03ZW`$\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\x0Ee\x81a\x06\x96V[\x92a\x0Es`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a\x0E\x9BWPPP\x90V[` \x80\x91\x835a\x0E\xAA\x81a\x05\xFAV[\x81R\x01\x91\x01\x90a\x0E\x8EV[\x81`\x1F\x82\x01\x12\x15a\x03ZW\x805a\x0E\xCB\x81a\x06\x96V[\x92a\x0E\xD9`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03ZW` \x01\x91[\x83\x83\x10a\x0F\x03WPPPP\x90V[` `@\x91a\x0F\x12\x84\x86a\x04DV[\x81R\x01\x92\x01\x91a\x0E\xF5V[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\x0F4\x81a\x06\x96V[\x92a\x0FB`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03ZW` \x82\x01\x90[\x83\x82\x10a\x0FnWPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW` \x91a\x0F\x90\x87\x84\x80\x94\x88\x01\x01a\x0ENV[\x81R\x01\x91\x01\x90a\x0F_V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03ZWa\x0F\xB1a\x03\xCEV[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x0F\xCE\x91\x84\x01a\x0ENV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x0F\xEF\x91\x84\x01a\x0E\xB5V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x10\x13\x91\x84\x01a\x0E\xB5V[`@\x85\x01Ra\x10%\x81``\x84\x01a\x04\xECV[``\x85\x01Ra\x107\x81`\xE0\x84\x01a\x04DV[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x10\\\x91\x84\x01a\x0ENV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x10\x81\x91\x84\x01a\x0ENV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x10\xA4\x92\x01a\x0F\x1DV[`\xE0\x83\x01RV[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x10\xDB\x906\x90`\x04\x01a\x0E/V[a\x10\xE46a\x0E=V[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x11\x04\x906\x90`\x04\x01a\x0F\x9BV[`\xCET\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x13\x17Wa\x11'` \x83\x94\x93\x01a,\x85V[\x91a\x12*a\x118`@\x86\x01\x86a,\x8FV[\x92\x90\x94a\x11\x98a\x11J``\x89\x01a,\x85V[\x97`@Qa\x11n\x81a\x11`` \x82\x01\x94\x85a,\xC1V[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xADV[Q\x90 a\x11\x91a\x11}\x88a,\x85V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[T\x14a-HV[a\x11\xC2a\x11\xBBa\x11\xA7\x87a,\x85V[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15a-\xBAV[\x83c\xFF\xFF\xFF\xFFC\x16\x96a\x12\x0Ca\x12\x04a\x11\xFB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a.KV[c\xFF\xFF\xFF\xFF\x16\x90V[\x89\x11\x15a.eV[`@Q` \x81\x01\x90a\x12\"\x81a\x11`\x8B\x85a.\xE5V[Q\x90 a<>V[\x91\x90`\xFF_\x96\x16\x95[\x82\x81\x10a\x12\xB5W\x7F4\x9C\x1E\xE6\x0EN\x89r\xEE\x9D\xBAd,\x17tT=\\A6\x87\x9B\x7FL\xAA\xF0K\xF8\x1AHz*\x86\x86\x86a\x12ua\x12ia\x03\xE0V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x12\x91\x81a\x11`\x86\x86\x86a/\xC8V[Q\x90 a\x12\xA0a\x11\xA7\x83a,\x85V[Ua\x12\xB0`@Q\x92\x83\x92\x83a/\xC8V[\x03\x90\xA1\0[\x80a\x13\x11a\x12\xEDa\x12\xE8a\x12\xDCa\x12\xCF`\x01\x96\x88Qa \xA8V[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[a.\xF5V[a\x13\na\x12\xDC\x8Ba\x13\x05a\x12\xCF\x87` \x8B\x01Qa \xA8V[a/4V[\x11\x15a/WV[\x01a\x123V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x13\x7FWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13rV[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a\x13\xB2\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x13\xD1\x906\x90`\x04\x01a\n\x0BV[`D5\x91a\x13\xDE\x83a\x05\xFAV[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x14\x06\x86\x88`\x04\x84\x01a/\xF2V[\x03\x81\x84Z\xFA\x91\x82\x15a\x03UW_\x92a\x14\xEAW[Pa\x14$\x83Qa iV[\x93_[\x84Q\x81\x10\x15a\x14\xDCWa\x14:\x81\x86a \xA8V[Q\x90` \x83a\x14Va\x14L\x84\x89a \xA8V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x03UW`\x01\x92_\x91a\x14\xAEW[P\x82\x80`\xC0\x1B\x03\x16a\x14\xA7\x82\x89a \xA8V[R\x01a\x14'V[a\x14\xCF\x91P` =\x81\x11a\x14\xD5W[a\x14\xC7\x81\x83a\x03\xADV[\x81\x01\x90a'\x94V[_a\x14\x95V[P=a\x14\xBDV[`@Q\x80a\x07\x88\x88\x82a\x13\\V[a\x15\x07\x91\x92P=\x80_\x83>a\x14\xFF\x81\x83a\x03\xADV[\x81\x01\x90a&cV[\x90_a\x14\x19V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `fT`@Q\x90\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x15M\x81a\x05\xFAV[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW`\xC06`\x03\x19\x01\x12a\x03ZW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x16!\x906\x90`\x04\x01a\x0E/V[a\x16*6a\x0E=V[\x90`@6`c\x19\x01\x12a\x03ZW`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x03$\x92a\x16\\`d\x926\x90`\x04\x01a\x0E\xB5V[\x92a4@V[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`$5`\x045a\x16\x82\x82a\x05\xFAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x16\xA1\x906\x90`\x04\x01a\x0B\x9BV[`\xCFT\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17\x81Wa\x03$\x93a\x17k\x93a\x16\xEBa\x16\xF2\x93a\x16\xCFa:@V[\x95\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01Rc\xFF\xFF\xFF\xFF\x16``\x86\x01RV[6\x91a\x07\xC1V[`@\x82\x01R`@Q` \x81\x01\x90a\x17\r\x81a\x11`\x85\x85a:dV[Q\x90 a\x17\"a\x11}`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\x16\x95\xB8\xD0n\xC8\0\xB4a^t\\\xFB[\xD0\x0C\x1F(ua]B\x92\\;Z\xFAT;\xB2LH`@Q\x80a\x17cc\xFF\xFF\xFF\xFF\x86\x16\x94\x82a:dV[\x03\x90\xA2a.\x1BV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x181WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x18$V[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x18\x83\x906\x90`\x04\x01a\x0B\x9BV[\x90\x91`D5a\x18\x91\x81a\x05\xFAV[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZWa\x18\xF3\x94a\x18\xB8a\x18\xBE\x956\x90`\x04\x01a\x0F\x9BV[\x93a<>V[`@Q\x92\x83\x92`@\x84R` a\x18\xDF\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x18\x14V[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x18\x14V[\x90` \x83\x01R\x03\x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZWa\x19\x15aL7V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW`\xC06`\x03\x19\x01\x12a\x03ZW`\x045a\x1A\x9E\x81a\x06\x85V[a\x1B\x1E`$5a\x1A\xAD\x81a\x06\x85V[`D5a\x1A\xB9\x81a\x06\x85V[`d5a\x1A\xC5\x81a\x06\x85V[`\x845\x91a\x1A\xD2\x83a\x06\x85V[`\xA45\x93a\x1A\xDF\x85a\x06\x85V[_T\x96a\x1B\x04`\xFF`\x08\x8A\x90\x1C\x16\x15\x80\x99\x81\x9Aa\x1B\x9CW[\x81\x15a\x1B|W[PaD\xC8V[\x87a\x1B\x15`\x01`\xFF\x19_T\x16\x17_UV[a\x1BeWaE+V[a\x1B$W\0[a\x1B2a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x12\xB0V[a\x1Bwa\x01\0a\xFF\0\x19_T\x16\x17_UV[aE+V[0;\x15\x91P\x81a\x1B\x8EW[P_a\x1A\xFEV[`\xFF\x16`\x01\x14\x90P_a\x1B\x87V[`\x01`\xFF\x82\x16\x10\x91Pa\x1A\xF7V[`@\x90a\x06\xF1\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x07\xF7V[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a\x1B\xDE\x81a\x06\x85V[`$5`D5a\x1B\xED\x81a\x05\xFAV[a\x1C.a\x1B\xF8a GV[\x92\x80a\x1C\x03\x85a \x9BV[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a/\xF2V[\x03\x81\x87Z\xFA\x93\x84\x15a\x03UW\x83a\x1CXa\x11\xFBa\x14La\x1C\x8D\x98` \x97_\x91a\x1C\xEBW[Pa \x9BV[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x03UWa\x1C\xBC\x92_\x91a\x1C\xCCW[P`\x01`\x01`\xC0\x1B\x03\x16\x92a\x1C\xB6\x84aL\xD7V[\x90a#\tV[\x90a\x07\x88`@Q\x92\x83\x92\x83a\x1B\xAAV[a\x1C\xE5\x91P` =` \x11a\x14\xD5Wa\x14\xC7\x81\x83a\x03\xADV[_a\x1C\xA2V[a\x1C\xFF\x91P=\x80_\x83>a\x14\xFF\x81\x83a\x03\xADV[_a\x1CRV[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045a\x1Df\x81a\x06\x85V[a\x1DnaL7V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x1D\x86Wa\x03$\x90aL\x8FV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Q`d\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a\x1E\xB8W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1E\xA9Wa\x1Ew`fT\x19\x82\x19\x81\x16\x14a\x1F\rV[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a\x1E\xD1\x91P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a\x1EVV[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\t@V[`@Q=_\x82>=\x90\xFD[\x15a\x1E\xFEWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a\x1F\x14WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\x1FHW`\x05\x1B\x01\x90V[a\x1F#V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a =a \x1Aa C\x95a \x14a \r\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x1F\xE4\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xADV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aF>V[\x90aF\x84V[\x92a \x14a /a )aF\xE6V[\x94aG\xDDV[\x91a 8aH\xF9V[aF>V[\x91aI-V[\x90\x91V[`@\x80Q\x90\x91\x90a X\x83\x82a\x03\xADV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a s\x82a\x06\x96V[a \x80`@Q\x91\x82a\x03\xADV[\x82\x81R\x80\x92a \x91`\x1F\x19\x91a\x06\x96V[\x01\x90` 6\x91\x017V[\x80Q\x15a\x1FHW` \x01\x90V[\x80Q\x82\x10\x15a\x1FHW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQ\x90V[\x91\x90\x91a \xD8\x83Qa iV[\x92_[\x81Q\x81\x10\x15a!\x8BW\x80` a!\x04a \xF7a!-\x94\x86a \xA8V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03UW`\x01\x92_\x91a!]W[Pa!V\x82\x88a \xA8V[R\x01a \xDBV[a!~\x91P` =\x81\x11a!\x84W[a!v\x81\x83a\x03\xADV[\x81\x01\x90a \xBCV[_a!KV[P=a!lV[PPPV[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\x06\x85V[\x90a!\xAF\x82a\x06\x96V[a!\xBC`@Q\x91\x82a\x03\xADV[\x82\x81R` \x81\x93a!\xCF`\x1F\x19\x91a\x06\x96V[\x01\x91\x01_[\x82\x81\x10a!\xE0WPPPV[``\x82\x82\x01R` \x01a!\xD4V[\x90\x81Q\x81\x10\x15a\x1FHW\x01` \x01\x90V[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa\"2\x81a\x06\x96V[\x92a\"@`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a\"hWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\"[V[\x90a\"\x82\x82a\x06\x96V[a\"\x8F`@Q\x91\x82a\x03\xADV[\x82\x81R\x80\x92a\"\xA0`\x1F\x19\x91a\x06\x96V[\x01_[\x81\x81\x10a\"\xAFWPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x03\x8DW` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a\"\xA3V[\x90\x81` \x91\x03\x12a\x03ZWQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x03ZW\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x03UW_\x95a&\x1EW[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x03UW`\x04\x96_\x93a%\xFCW[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x03UW_\x96a%\xDBW[Pa#\x97\x85\x93\x92\x95Qa!\xA5V[\x94_\x93[\x80Q\x85\x10\x15a%\xD1Wa#\xC8a#\xC2a#\xB4\x87\x84a!\xEEV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x03UW_\x94a%\xADW[Pa$\x18\x84Qa\"xV[a$\"\x88\x8Ba \xA8V[Ra$-\x87\x8Aa \xA8V[P_[\x84Q\x81\x10\x15a%\x9CW\x80` a$Ia$k\x93\x88a \xA8V[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03UW_\x92a%|W[Pa$\x91\x81\x87a \xA8V[Q\x8A` \x8Aa$\xA0\x85\x8Ba \xA8V[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x03UWa%3\x8C\x8Fa%.`\x01\x98a%E\x97\x89\x97_\x92a%LW[Pa%\x19a%\na\x03\xEFV[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a \xA8V[Q\x90a%?\x83\x83a \xA8V[Ra \xA8V[P\x01a$0V[a%n\x91\x92P` =\x81\x11a%uW[a%f\x81\x83a\x03\xADV[\x81\x01\x90a\"\xEAV[\x90_a$\xFEV[P=a%\\V[a%\x95\x91\x92P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x90_a$\x86V[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa#\x9BV[a%\xCA\x91\x94P=\x80_\x83>a%\xC2\x81\x83a\x03\xADV[\x81\x01\x90a!\xFFV[\x92_a$\rV[PPP\x93PPP\x90V[a%\xF5\x91\x96P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x94_a#\x89V[` \x91\x93Pa&\x17\x90\x82=\x84\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x92\x90a#dV[a&8\x91\x95P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x93_a#;V[`@Q\x90a&L\x82a\x03\x92V[``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa&\x96\x81a\x06\x96V[\x92a&\xA4`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a&\xCCWPPP\x90V[` \x80\x91\x83Qa&\xDB\x81a\x05\xFAV[\x81R\x01\x91\x01\x90a&\xBFV[c\xFF\xFF\xFF\xFF\x90\x91\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x83\x90R`\x01`\x01`\xFB\x1B\x03\x83\x11a\x03ZW``\x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF1\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a'\x1DV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a'\x7FW`\x01\x01\x90V[a'ZV[\x91\x90\x81\x10\x15a\x1FHW`\x05\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03ZW\x90V[\x15a'\xBAWV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a\x1FHW\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\x05\xFAV[_\x19\x81\x14a'\x7FW`\x01\x01\x90V[\x91a(\x16` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a'\x1DV[\x94\x16\x91\x01RV[\x95\x93\x94\x95\x92\x90\x91\x92a(-a&?V[P`@Qch0H5`\xE0\x1B\x81R\x93`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90` \x85`\x04\x81\x86Z\xFA\x94\x85\x15a\x03UW_\x95a,dW[Pa(ja&?V[\x94`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a(\x8A\x8D\x8D\x8B`\x04\x85\x01a&\xE6V[\x03\x81\x88Z\xFA\x90\x81\x15a\x03UW_\x91a,JW[P\x86R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x92\x90_\x81\x80a(\xCD\x85\x87\x8B`\x04\x85\x01a'=V[\x03\x81\x87Z\xFA\x90\x81\x15a\x03UW_\x91a,0W[P`@\x87\x01Ra(\xEF\x81a!\xA5V[\x98``\x87\x01\x99\x8AR_[`\xFF\x81\x16\x83\x81\x10\x15a+{W\x88_a)\"\x83\x8Fa)\x15\x88a iV[\x90Q\x90a%?\x83\x83a \xA8V[P_\x8A\x86\x8F[\x81\x84\x10a)\xA5WPPPP\x90P\x8Ca)?\x82a iV[\x91_[\x81\x81\x10a)lWPP\x91a)a\x91a)g\x94\x93Q\x90a%?\x83\x83a \xA8V[Pa'nV[a(\xF9V[\x80a)\x9Fa)\x8Aa\x14L`\x01\x94a)\x84\x8A\x89Qa \xA8V[Qa \xA8V[a)\x94\x83\x88a \xA8V[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x01a)BV[a\x14L\x84a)\xBA\x81` \x96\x95a)\xC2\x95a'\x84V[5\x97Qa \xA8V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x87\x01R\x16`D\x85\x01R\x83`d\x81\x8DZ\xFA\x80\x15a\x03UW\x88\x8F\x88\x8A\x91\x8F\x94a*g`\x01a*Z\x81\x93\x8D\x80\x9D_\x92a+OW[Pa#\xC2a*6a*D\x92a*/\x87\x80`\xC0\x1B\x03\x86\x16\x15\x15a'\xB3V[\x8B\x8Da'\xC9V[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a*\x83W[PPPPP`\x01\x91\x92P\x01\x90\x8A\x91\x8A\x86\x8Fa)(V[\x85\x97a*\xA5\x93a*\x9E` \x97\x99\x98a#\xC2\x95a*6\x95a'\x84V[5\x95a'\xC9V[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`D\x84\x01R\x82`d\x81\x8CZ\xFA\x90\x81\x15a\x03UW\x8Fa+\x03\x90a+\x08\x93\x83\x88`\x01\x97_\x93a+\x17W[Pa)\x84\x90a)\x94\x93\x94Qa \xA8V[a'\xEAV[\x90P\x82\x91\x8A\x88\x8F\x88\x8A\x91a*mV[a)\x94\x93P\x90a+@a)\x84\x92` =\x81\x11a+HW[a+8\x81\x83a\x03\xADV[\x81\x01\x90a'\xD5V[\x93P\x90a*\xF3V[P=a+.V[a*D\x91\x92Pa*6a+ra#\xC2\x92` =\x81\x11a\x14\xD5Wa\x14\xC7\x81\x83a\x03\xADV[\x93\x92PPa*\x12V[PPP\x92\x90\x95\x97P`\x04\x94\x96P` \x91P`@Q\x94\x85\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x90\x81\x15a\x03UWa+\xD1\x94_\x94\x85\x93a,\x0FW[P`@Qc5IR\xA3`\xE2\x1B\x81R\x95\x86\x94\x85\x93\x84\x93`\x04\x85\x01a'\xF8V[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a+\xF5W[P` \x82\x01R\x90V[a,\t\x91P=\x80_\x83>a\x14\xFF\x81\x83a\x03\xADV[_a+\xECV[a,)\x91\x93P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x91_a+\xB3V[a,D\x91P=\x80_\x83>a\x14\xFF\x81\x83a\x03\xADV[_a(\xE0V[a,^\x91P=\x80_\x83>a\x14\xFF\x81\x83a\x03\xADV[_a(\x9DV[a,~\x91\x95P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x93_a(aV[5a\x06\xF1\x81a\x05\xFAV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03ZW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW` \x01\x91\x816\x03\x83\x13a\x03ZWV[` \x81R\x815` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x015a,\xDF\x81a\x05\xFAV[\x16`@\x82\x01R`@\x82\x015`\x1E\x19\x836\x03\x01\x81\x12\x15a\x03ZW\x82\x01\x90` \x825\x92\x01`\x01`\x01`@\x1B\x03\x83\x11a\x03ZW\x826\x03\x81\x13a\x03ZWa-=``a-6`\x80\x93a\x06\xF1\x96\x85\x84\x88\x01R`\xA0\x87\x01\x91a'\x1DV[\x95\x01a\x06\x08V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15a-OWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a-\xC1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x7FWV[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x7FWV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a'\x7FWV[\x15a.lWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a.\xDB\x81a\x05\xFAV[\x16\x84R\x015\x91\x01RV[`@\x81\x01\x92\x91a\x03\xDE\x91\x90a.\xC7V[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a'\x7FWV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a'\x7FWV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a'\x7FWV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a'\x7FWV[\x15a/^WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[\x90\x92\x91` ``\x91a/\xDE\x84`\x80\x81\x01\x97a.\xC7V[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF1\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x06\xADV[\x15a0\x16WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x92\x91` ``\x91a0{\x84`\x80\x81\x01\x97a.\xC7V[c\xFF\xFF\xFF\xFF\x815a0\x8B\x81a\x05\xFAV[\x16`@\x85\x01R\x015\x91\x01RV[\x15a0\x9FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a1\x11WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a1\x8FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a2$WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a2\x17V[\x15a2AWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa2\xF8\x81a\x06\x96V[\x92a3\x06`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a3.WPPP\x90V[` \x80\x91\x83Qa3=\x81a\x06\x85V[\x81R\x01\x91\x01\x90a3!V[`@Q\x90a3W`@\x83a\x03\xADV[`\x12\x82Rq9\xB60\xB9\xB4/\xBA42\xAF\xB7\xB82\xB90\xBA7\xB9`q\x1B` \x83\x01RV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90` \x83R`\xC0\x83\x01\x92`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x93`\xA0``\x83\x01R\x84Q\x80\x91R` `\xE0\x83\x01\x95\x01\x90_[\x81\x81\x10a4!WPPP`\x80a4\x0Ca\x06\xF1\x94\x95``\x85\x01Q`\x1F\x19\x85\x83\x03\x01\x84\x86\x01Ra\x06\xADV[\x92\x01Q\x90`\xA0`\x1F\x19\x82\x85\x03\x01\x91\x01Ra3xV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x96\x87\x01\x96\x90\x92\x01\x91`\x01\x01a3\xE3V[\x90\x92\x91`\x01a4N\x85a,\x85V[\x94` a5\n\x855a4ya4q\x8Ac\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a0\x0FV[a4\xB4a4\x94\x8Ac\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T`@Q\x85\x81\x01\x90a4\xAB\x81a\x11`\x8C\x8B\x86a0eV[Q\x90 \x14a0\x98V[a4\xDFa4\xD9a4\xD2\x8Bc\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a1\nV[a5\x04a4\xF6a\x11\xFBa4\xF1\x89a,\x85V[a.3V[c\xFF\xFF\xFF\xFFC\x16\x11\x15a1\x88V[\x80a/!V[\x91\x015\x14\x14a:\x0FWa5\x1D\x83Qa iV[\x93_[\x84Q\x81\x10\x15a5]W\x80a5La59`\x01\x93\x88a \xA8V[Q\x80Q_R` \x01Q` R`@_ \x90V[a5V\x82\x89a \xA8V[R\x01a5 V[P\x90\x92\x93\x91\x94a5\x97` \x87\x01\x94` a5v\x87a,\x85V[`@Qa5\x8B\x81a\x11`\x8A\x86\x83\x01\x95\x86a1\xFAV[Q\x90 \x91\x015\x14a2:V[a5\xA1\x85Qa iV[\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94_[\x87Q\x81\x10\x15a6QW\x80` a5\xE8a6\x08\x93\x89a \xA8V[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8BZ\xFA\x91\x82\x15a\x03UW`\x01\x92a6-\x91_\x91a63W[Pa\x0BZ\x83\x8Da \xA8V[\x01a5\xCFV[a6K\x91P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a6\"V[P\x92\x96\x95P\x92P\x92a6\xB0a6ya6\x81`@\x86\x01\x94a6q\x86\x88a,\x8FV[\x93\x90\x91a,\x85V[\x926\x91a\x07\xC1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a#\tV[\x94_\x91[\x86Q\x83\x10\x15a9\xB1W_\x97\x96\x97\x95[a6\xCD\x84\x8Aa \xA8V[QQ\x87\x10\x15a9\xA1W\x97a7\x0E\x98` \x80a6\xEC\x8Aa)\x84\x89\x87a \xA8V[Q\x01Q`@Q\x80\x9C\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x99\x8A\x15a\x03UW_\x9Aa9\x81W[P`\x01\x98_[\x85Q\x81\x10\x15a9sWa7La7@a \xF7\x83\x89a \xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x14a7cW`\x01\x01a7'V[P\x99\x90\x97\x91\x98P`\x01_[\x15\x15\x14a7\x83W[P`\x01\x01\x95\x97\x96\x97a6\xC3V[\x98\x96\x97\x86\x98_\x87\x95\x93\x98a7\xE8`\xFFa7\xC0a#\xC2a*6\x8Ca7\xBAa8=\x9Fa7\xB4`\xCDT`\x01\x80`\xA0\x1B\x03\x16\x90V[\x98a,\x8FV[\x90a'\xC9V[a7\xDAa7\xCBa\x03\xE0V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[\x16c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`\xD1Ta7\xFF\x90a7@\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\x10]\xEA\x1F`\xE2\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\x04\x82\x01R` \x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16`$\x83\x01R\x90\x98\x89\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x96\x87\x15a\x03UW_\x97a9OW[Pa8Z\x87Qa iV[\x98_[\x8AQ\x81\x10\x15a8\x83W\x80g\x01cEx]\x8A\0\0a8|`\x01\x93\x8Ea \xA8V[R\x01a8]V[P\x9A\x92\x94\x96a8\xC1`\xFFa8\xA8a#\xC2a*6\x8F\x9D\x97\x9F\x96\x9E\x96a7\xBA\x8E\x8E\x92a,\x8FV[a8\xB3a%\na\x03\xFEV[\x16c\xFF\xFF\xFF\xFF\x16` \x86\x01RV[`@\x84\x01R``\x83\x01Ra8\xD3a3HV[`\x80\x83\x01R`\xD0Ta8\xEF\x90a7@\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03ZW`@Qcjf\x9BA`\xE0\x1B\x81R\x92_\x91\x84\x91\x82\x90\x84\x90\x82\x90a9\x1B\x90`\x04\x83\x01a3\x9CV[\x03\x92Z\xF1\x91\x82\x15a\x03UW`\x01\x92a95W[P\x90a7vV[\x80a9C_a9I\x93a\x03\xADV[\x80a\x05\x88V[_a9.V[a9l\x91\x97P=\x80_\x83>a9d\x81\x83a\x03\xADV[\x81\x01\x90a2\xC5V[\x95_a8OV[P\x99\x90\x97\x91\x98`\x01\x90a7nV[a9\x9A\x91\x9AP` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x98_a7!V[\x96\x97\x96\x95P`\x01\x90\x92\x01\x91a6\xB4V[PPPPP\x91\x90Pa9\xE1a9\xD4\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3V[PPPc\xFF\xFF\xFF\xFF3\x91\x16\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05_\x80\xA3V[`@Q\x90a:M\x82a\x03\x92V[_``\x83\x82\x81R\x82` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81R\x81Q` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`\x80c\xFF\xFF\xFF\xFF``a:\x9F`@\x86\x01Q\x84\x83\x87\x01R`\xA0\x86\x01\x90a3xV[\x94\x01Q\x16\x91\x01R\x90V[`@Q\x90a:\xB6\x82a\x03rV[``` \x83\x82\x81R\x01RV[\x15a:\xC9WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a:\xDEWV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a:\xF4WV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a;\nWV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\r\xF1V[_\x19\x81\x01\x91\x90\x82\x11a'\x7FWV[\x15a;CWV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90`\x01\x82\x01\x80\x92\x11a'\x7FWV[\x90`\x02\x82\x01\x80\x92\x11a'\x7FWV[\x90`\x03\x82\x01\x80\x92\x11a'\x7FWV[\x90`\x04\x82\x01\x80\x92\x11a'\x7FWV[\x90`\x05\x82\x01\x80\x92\x11a'\x7FWV[\x91\x90\x82\x01\x80\x92\x11a'\x7FWV[\x15a;\xACWV[c\xAF\xFC^\xDB`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03ZWQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03ZW\x90V[\x15a;\xE3WV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a'\x7FWV[\x15a<\x19WV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a</WV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[\x94\x93\x92\x90\x91\x93a<La:\xA9V[Pa<X\x85\x15\x15a:\xC2V[`@\x84\x01QQ\x85\x14\x80aD\xBAW[\x80aD\xACW[\x80aD\x9EW[a<{\x90a:\xD7V[a<\x8D` \x85\x01QQ\x85QQ\x14a:\xEDV[a<\xA4c\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a;\x03V[a<\xACa\x03\xE0V[_\x81R_` \x82\x01R\x92a<\xBEa:\xA9V[a<\xC7\x87a iV[` \x82\x01Ra<\xD5\x87a iV[\x81Ra<\xDFa:\xA9V[\x92a<\xEE` \x88\x01QQa iV[\x84Ra<\xFE` \x88\x01QQa iV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UWa=g\x91_\x91aDoW[Pa=b6\x8B\x87a\x07\xC1V[aJ\x99V[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a>\xC6W` \x88a=\xBBa\x14L\x8Ca=\xB3\x8F\x96\x86\x8Ea=\x98a59\x86\x80\x95a \xA8V[a=\xA5\x84\x84\x84\x01Qa \xA8V[R\x82a>\x93W[\x01Qa \xA8V[Q\x95Qa \xA8V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x03UWa \x14\x8Aa>h\x8Fa>a\x8F\x84` \x8F\x92a>X\x93a>P\x84`\x01\x9Ea>n\x9E_\x91a>vW[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa \xA8V[R\x01Qa \xA8V[Q\x93\x8DQa \xA8V[Q\x16aJ\xC4V[\x90aJ\xF5V[\x97\x01\x96a=kV[a>\x8D\x91P\x86=\x81\x11a\x14\xD5Wa\x14\xC7\x81\x83a\x03\xADV[_a>AV[a>\xC1a>\xA3\x84\x84\x84\x01Qa \xA8V[Qa>\xBA\x84\x84\x01Qa>\xB4\x87a;.V[\x90a \xA8V[Q\x10a;<V[a=\xACV[P\x90\x95\x97\x94\x96Pa>\xDB\x91\x98\x93\x92\x99PaK\xB2V[\x91a>\xE8`\x97T`\xFF\x16\x90V[\x90\x81\x15aDgW`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91aDHW[P\x91\x90[_\x92[\x81\x84\x10a?\x99WPPPPP\x92a?\x80a?{a?ta?\x93\x95\x85a\x11`\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\x1FaV[\x91\x90a<\x12V[a<(V[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a1\xFAV[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8DaCBW[a\x14L\x82`\xA0a?\xEEa#\xC2a*6\x84a?\xF6\x97a?\xE8a?\xDAa59\x8F\x9C`@` \x9F\x9E\x01Qa \xA8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba'\xC9V[\x97\x01Qa \xA8V[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03UWa@\xBAa\x14L\x8F\x95\x8F\x90a@\xB2\x8F\x97\x8F\x96\x84\x8Fa@\xAC`\xC0\x96a@\xA5\x84\x8F` \x9F\x90a=\xACa*6\x99`@\x93a#\xC2\x9C_\x91aC\x14W[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a;\xDCV[Q\x90aF\x84V[\x9Ca'\xC9V[\x96\x01Qa \xA8V[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03UWaAG\x91\x8C\x8F\x92_\x92aB\xF0W[P` aA9\x92\x93\x01Qa \xA8V[\x90`\x01`\x01``\x1B\x03\x16\x90RV[aAg\x8CaA9\x8CaA`a\x12\xCF\x82` \x86\x01Qa \xA8V[\x92Qa \xA8V[_\x98_[` \x8A\x01QQ\x81\x10\x15aB\xD7W\x8B\x8DaA\xA9\x89aA\x9Ca#\xC2a*6\x86\x8F\x89aA\x94\x91Qa \xA8V[Q\x94\x87a'\xC9V[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[aA\xB8W[PP`\x01\x01aAkV[\x8A\x8AaB:\x85\x9F\x94\x8F\x96\x86a)\x84\x8F\x93`\xE0aA\xF1a\x14L\x95` aA\xE9a#\xC2a*6\x83\x9FaA\xFA\x9C\x89\x91a'\xC9V[\x9A\x01Qa \xA8V[Q\x9B\x01Qa \xA8V[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW\x8FaB\xA6\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92aB\xB1W[PaB\xA0aA9\x92\x93Q\x93aB\x9Ba\x12\xCF\x84\x87a \xA8V[a;\xF2V[\x92a \xA8V[\x01\x9A\x90P\x8B\x8DaA\xAEV[aA9\x92PaB\xD0aB\xA0\x91` =\x81\x11a%uWa%f\x81\x83a\x03\xADV[\x92PaB\x83V[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a?CV[aA9\x92PaC\r` \x91\x82=\x81\x11a%uWa%f\x81\x83a\x03\xADV[\x92PaA*V[` aC5\x92P=\x81\x11aC;W[aC-\x81\x83a\x03\xADV[\x81\x01\x90a;\xBBV[_a@\x8FV[P=aC#V[aC\x7F\x94PaC\\\x92Pa#\xC2\x91a*6\x91` \x95a'\xC9V[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UW` \x89a?\xF6\x8F\x93\x8F`\xA0\x8F\x97a#\xC2a*6\x8F\x8F\x90a?\xE8a?\xDAa59\x8F`@\x8B\x96\x91\x8F\x88\x93a\x14L\x9FaD\x03\x90aD\t\x93a?\xEE\x9F_\x92aD\x1FW[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a;\x98V[\x11a;\xA5V[PPPPPP\x97PPPPPP\x92\x93PPa?\xAEV[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91aD@\x91=\x81\x11a!\x84Wa!v\x81\x83a\x03\xADV[\x92\x91PaC\xF2V[aDa\x91P` =` \x11a+HWa+8\x81\x83a\x03\xADV[_a?<V[_\x91\x90a?@V[aD\x91\x91P` =` \x11aD\x97W[aD\x89\x81\x83a\x03\xADV[\x81\x01\x90a;\x19V[_a=VV[P=aD\x7FV[P`\xE0\x84\x01QQ\x85\x14a<rV[P`\xC0\x84\x01QQ\x85\x14a<lV[P`\xA0\x84\x01QQ\x85\x14a<fV[\x15aD\xCFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aE4\x90aL\x8FV[`\xCE\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xCF\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x83\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\xD0\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xCD\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aF\x07\x82a\x03rV[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aF#\x81\x84a\x03\xADV[6\x837V[`@Q\x90aF7` \x83a\x03\xADV[` 6\x837V[\x91\x90`@\x90``aFMaE\xFAV[\x94\x85\x92` \x85Q\x92aF_\x85\x85a\x03\xADV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aF\x82WV[\xFE[` \x92\x91`\x80`@\x92aF\x95aE\xFAV[\x95\x86\x93\x81\x86Q\x93aF\xA6\x86\x86a\x03\xADV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aF\x82W\x15aF\xD7WV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@QaF\xF2\x81a\x03rV[`@\x90\x81QaG\x01\x83\x82a\x03\xADV[\x826\x827\x81R` \x82Q\x91aG\x16\x84\x84a\x03\xADV[\x836\x847\x01R\x80QaG(\x82\x82a\x03\xADV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aG~\x83\x83a\x03\xADV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaG\xD3\x83Q\x93\x84a\x03\xADV[\x82R` \x82\x01R\x90V[_Q` aN\x1A_9_Q\x90_R\x90aG\xF4aE\xFAV[P_\x91\x90\x06` `\xC0\x83[aH\xF4W_\x93_Q` aN\x1A_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaH*\x85\x82a\x03\xADV[\x846\x827\x84\x81\x85`@QaH>\x82\x82a\x03\xADV[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aN\x1A_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aF\x82WaH\xA8\x90aN\x03V[Q\x91aH\xF4W_Q` aN\x1A_9_Q\x90_R\x82\x80\t\x14aH\xDFWP_Q` aN\x1A_9_Q\x90_R`\x01_\x94\x08\x92\x93aG\xFFV[\x92\x93PPaH\xEBa\x03\xE0V[\x92\x83R\x82\x01R\x90V[a\x1FMV[aI\x01aE\xFAV[P`@QaI\x0E\x81a\x03rV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x0C\x81\x10\x15a\x1FHW`\x05\x1B\x01\x90V[\x93\x92\x90\x91aI;`@a\x04\rV[\x94\x85R` \x85\x01RaIM`@a\x04\rV[\x91\x82R` \x82\x01RaI]aF\x12V[\x92_[`\x02\x81\x10aI\x8AWPPP` a\x01\x80\x92aIyaF(V[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aI\x96`\x01\x92a/\x0BV[aI\xA0\x82\x85a\x1F7V[QQaI\xAC\x82\x89aI\x1CV[R` aI\xB9\x83\x86a\x1F7V[Q\x01QaI\xCEaI\xC8\x83a;RV[\x89aI\x1CV[RaI\xD9\x82\x86a\x1F7V[QQQaI\xE8aI\xC8\x83a;`V[RaI\xFEaI\xF6\x83\x87a\x1F7V[QQ` \x01\x90V[QaJ\x0BaI\xC8\x83a;nV[R` aJ\x18\x83\x87a\x1F7V[Q\x01QQaJ(aI\xC8\x83a;|V[RaJTaJNaJG` aJ>\x86\x8Aa\x1F7V[Q\x01Q` \x01\x90V[Q\x92a;\x8AV[\x88aI\x1CV[R\x01aI`V[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01aJ\xA7`\xFF\x93aM\x8BV[\x92\x83\x92\x16\x1B\x11\x15aJ\xB5W\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[aJ\xD0WP\x90V[_\x19\x81\x01\x81\x81\x11a'\x7FWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a'\x7FW`\x01\x01\x90\x80aJ\xC8V[\x90aJ\xFEaE\xFAV[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aK\xA3W`\x01\x82\x14aK\x9EWaK\x1Fa\x03\xE0V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aKDWPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aK~W[`\x01aKtaKi\x83`\xFF\x94aF\x84V[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aK0V[\x94`\x01aKtaKiaK\x93\x89`\xFF\x95aF\x84V[\x98\x93PPPPaKXV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[aK\xBAaE\xFAV[P\x80Q\x90\x81\x15\x80aL+W[\x15aK\xE7WPP`@QaK\xDB`@\x82a\x03\xADV[_\x81R_` \x82\x01R\x90V[` _Q` aN\x1A_9_Q\x90_R\x91\x01Q\x06_Q` aN\x1A_9_Q\x90_R\x03_Q` aN\x1A_9_Q\x90_R\x81\x11a'\x7FW`@Q\x91aG\xD3\x83a\x03rV[P` \x81\x01Q\x15aK\xC6V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aLKWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\xFF\xFFaL\xE3\x82aJ\xC4V[\x16aL\xED\x81a\x07\xA6V[\x90aL\xFB`@Q\x92\x83a\x03\xADV[\x80\x82RaM\n`\x1F\x19\x91a\x07\xA6V[\x016` \x83\x017__[\x82Q\x82\x10\x80aMjW[\x15aMcW`\x01\x81\x1B\x84\x16aM<W[aM7\x90a'\xEAV[aM\x14V[\x90`\x01aM7\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaMY\x82\x87a!\xEEV[S\x01\x91\x90PaM.V[PP\x90P\x90V[Pa\x01\0\x81\x10aM\x1EV[\x15aM|WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aM\xF4W\x81Q\x15aM\xEFW` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aM\xEAW`\x01\x90aM\xD5aM\xCBa#\xC2a#\xB4\x86\x89a!\xEEV[`\xFF`\x01\x91\x16\x1B\x90V[\x90aM\xE1\x81\x83\x11aMuV[\x17\x91\x01\x90aM\xACV[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15aN\nWV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x8CTES\x8En\x18\xB0`\xF7`\x80v\xC2\x9E\xBC\xBD\x1C\xF2\xA0\xA3P{\x01\te)\xE6\xD0\0\x0BgdsolcC\0\x08\x1B\x003",
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
                Self {}
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
                Self {}
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
                Self {}
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
                Self {}
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
                Self {}
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
                Self {}
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
                Self {}
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
    /**Event with signature `NewTaskCreated(uint32,(uint256,uint32,bytes,uint32))` and selector `0x1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c48`.
    ```solidity
    event NewTaskCreated(uint32 indexed taskIndex, IIncredibleSquaringTaskManager.Task task);
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
        pub task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
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
            type DataTuple<'a> = (IIncredibleSquaringTaskManager::Task,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "NewTaskCreated(uint32,(uint256,uint32,bytes,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    22u8, 149u8, 184u8, 208u8, 110u8, 200u8, 0u8, 180u8, 97u8, 94u8, 116u8, 92u8,
                    251u8, 91u8, 208u8, 12u8, 31u8, 40u8, 117u8, 97u8, 93u8, 66u8, 146u8, 92u8,
                    59u8, 90u8, 250u8, 84u8, 59u8, 178u8, 76u8, 72u8,
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
                    <IIncredibleSquaringTaskManager::Task as alloy_sol_types::SolType>::tokenize(
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
    /**Event with signature `TaskResponded((uint32,uint256),(uint32,bytes32))` and selector `0x349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a`.
    ```solidity
    event TaskResponded(IIncredibleSquaringTaskManager.TaskResponse taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata taskResponseMetadata);
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
      pub taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
      #[allow(missing_docs)]
      pub taskResponseMetadata: <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
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
                IIncredibleSquaringTaskManager::TaskResponse,
                IIncredibleSquaringTaskManager::TaskResponseMetadata,
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
                  <IIncredibleSquaringTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                      &self.taskResponse,
                  ),
                  <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
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
    constructor(address _slashingRegistryCoordinator, address _pauserRegistry, uint32 _taskResponseWindowBlock);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _slashingRegistryCoordinator: alloy::sol_types::private::Address,
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
                        value._slashingRegistryCoordinator,
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
                        _slashingRegistryCoordinator: tuple.0,
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
                        &self._slashingRegistryCoordinator,
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
            type Return = TASK_CHALLENGE_WINDOW_BLOCKReturn;
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
            type Return = TASK_RESPONSE_WINDOW_BLOCKReturn;
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
                    Self {}
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
            type Return = WADS_TO_SLASHReturn;
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
            type Return = aggregatorReturn;
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
            type Return = allTaskHashesReturn;
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
                        &self._0,
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskResponsesCall {
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
            type Return = allTaskResponsesReturn;
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
                        &self._0,
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
            type Return = allocationManagerReturn;
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
    /**Function with signature `createNewTask(uint256,uint32,bytes)` and selector `0x6b92787e`.
    ```solidity
    function createNewTask(uint256 numberToBeSquared, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskCall {
        #[allow(missing_docs)]
        pub numberToBeSquared: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub quorumThresholdPercentage: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`createNewTask(uint256,uint32,bytes)`](createNewTaskCall) function.
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
                        value.numberToBeSquared,
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
                        numberToBeSquared: tuple.0,
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
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createNewTaskCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createNewTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createNewTask(uint256,uint32,bytes)";
            const SELECTOR: [u8; 4] = [107u8, 146u8, 120u8, 126u8];
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
                        &self.numberToBeSquared,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
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
            type Return = generatorReturn;
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
            type Return = getBatchOperatorFromIdReturn;
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
            type Return = getBatchOperatorIdReturn;
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
            type Return = getCheckSignaturesIndicesReturn;
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
            type Return = getOperatorState_0Return;
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
            type Return = getQuorumBitmapsAtBlockNumberReturn;
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
            type Return = getTaskResponseWindowBlockReturn;
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
            type Return = instantSlasherReturn;
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
            type Return = latestTaskNumReturn;
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
            type Return = paused_0Return;
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
                    Self {}
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
            type Return = paused_1Return;
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
            type Return = pauserRegistryReturn;
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
    /**Function with signature `raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])` and selector `0x6b532e9e`.
    ```solidity
    function raiseAndResolveChallenge(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct raiseAndResolveChallengeCall {
      #[allow(missing_docs)]
      pub task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
      #[allow(missing_docs)]
      pub taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
      #[allow(missing_docs)]
      pub taskResponseMetadata: <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
      #[allow(missing_docs)]
      pub pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
          <BN254::G1Point as alloy::sol_types::SolType>::RustType,
      >,
  }
    ///Container type for the return parameters of the [`raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])`](raiseAndResolveChallengeCall) function.
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
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IIncredibleSquaringTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
              <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
              <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
              <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
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
        #[automatically_derived]
        impl alloy_sol_types::SolCall for raiseAndResolveChallengeCall {
            type Parameters<'a> = (
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IIncredibleSquaringTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = raiseAndResolveChallengeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])";
            const SELECTOR: [u8; 4] = [107u8, 83u8, 46u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                  <IIncredibleSquaringTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                      &self.task,
                  ),
                  <IIncredibleSquaringTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                      &self.taskResponse,
                  ),
                  <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `respondToTask((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x5baec9a0`.
    ```solidity
    function respondToTask(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskCall {
      #[allow(missing_docs)]
      pub task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
      #[allow(missing_docs)]
      pub taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
      #[allow(missing_docs)]
      pub nonSignerStakesAndSignature: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
  }
    ///Container type for the return parameters of the [`respondToTask((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](respondToTaskCall) function.
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
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
              <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
              <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
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
        #[automatically_derived]
        impl alloy_sol_types::SolCall for respondToTaskCall {
            type Parameters<'a> = (
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = respondToTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "respondToTask((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [91u8, 174u8, 201u8, 160u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                  <IIncredibleSquaringTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                      &self.task,
                  ),
                  <IIncredibleSquaringTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type Return = serviceManagerReturn;
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
            type Return = taskNumberReturn;
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskSuccesfullyChallengedCall {
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
            type Return = taskSuccesfullyChallengedReturn;
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
                        &self._0,
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
    ///Container for all the [`IncredibleSquaringTaskManager`](self) function calls.
    pub enum IncredibleSquaringTaskManagerCalls {
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
    impl IncredibleSquaringTaskManagerCalls {
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
            [79u8, 115u8, 159u8, 116u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 45u8, 127u8, 2u8],
            [90u8, 200u8, 106u8, 183u8],
            [91u8, 174u8, 201u8, 160u8],
            [92u8, 21u8, 86u8, 98u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 236u8, 195u8, 245u8],
            [93u8, 244u8, 89u8, 70u8],
            [104u8, 48u8, 72u8, 53u8],
            [107u8, 83u8, 46u8, 158u8],
            [107u8, 146u8, 120u8, 126u8],
            [109u8, 20u8, 169u8, 135u8],
            [110u8, 251u8, 70u8, 54u8],
            [113u8, 80u8, 24u8, 166u8],
            [114u8, 209u8, 142u8, 141u8],
            [122u8, 250u8, 30u8, 237u8],
            [136u8, 111u8, 17u8, 149u8],
            [139u8, 0u8, 206u8, 124u8],
            [141u8, 165u8, 203u8, 91u8],
            [155u8, 41u8, 14u8, 152u8],
            [185u8, 141u8, 9u8, 8u8],
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
    impl alloy_sol_types::SolInterface for IncredibleSquaringTaskManagerCalls {
        const NAME: &'static str = "IncredibleSquaringTaskManagerCalls";
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
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<
                IncredibleSquaringTaskManagerCalls,
            >] = &[
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                              data,
                              validate,
                          )
                          .map(
                              IncredibleSquaringTaskManagerCalls::trySignatureAndApkVerification,
                          )
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn TASK_RESPONSE_WINDOW_BLOCK(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                              data,
                              validate,
                          )
                          .map(
                              IncredibleSquaringTaskManagerCalls::TASK_RESPONSE_WINDOW_BLOCK,
                          )
                    }
                    TASK_RESPONSE_WINDOW_BLOCK
                },
                {
                    fn aggregator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <aggregatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::aggregator)
                    }
                    aggregator
                },
                {
                    fn allTaskResponses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::allTaskResponses)
                    }
                    allTaskResponses
                },
                {
                    fn allTaskHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <allTaskHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::allTaskHashes)
                    }
                    allTaskHashes
                },
                {
                    fn getBatchOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::getBatchOperatorId)
                    }
                    getBatchOperatorId
                },
                {
                    fn getOperatorState_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::getOperatorState_0)
                    }
                    getOperatorState_0
                },
                {
                    fn serviceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn setStaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::setStaleStakesForbidden)
                    }
                    setStaleStakesForbidden
                },
                {
                    fn getBatchOperatorFromId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::getBatchOperatorFromId)
                    }
                    getBatchOperatorFromId
                },
                {
                    fn getCheckSignaturesIndices(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::getCheckSignaturesIndices)
                    }
                    getCheckSignaturesIndices
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn WADS_TO_SLASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::WADS_TO_SLASH)
                    }
                    WADS_TO_SLASH
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn respondToTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <respondToTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::respondToTask)
                    }
                    respondToTask
                },
                {
                    fn getQuorumBitmapsAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                              data,
                              validate,
                          )
                          .map(
                              IncredibleSquaringTaskManagerCalls::getQuorumBitmapsAtBlockNumber,
                          )
                    }
                    getQuorumBitmapsAtBlockNumber
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn taskSuccesfullyChallenged(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::taskSuccesfullyChallenged)
                    }
                    taskSuccesfullyChallenged
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn raiseAndResolveChallenge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::raiseAndResolveChallenge)
                    }
                    raiseAndResolveChallenge
                },
                {
                    fn createNewTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <createNewTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::createNewTask)
                    }
                    createNewTask
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn taskNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <taskNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::taskNumber)
                    }
                    taskNumber
                },
                {
                    fn generator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <generatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::generator)
                    }
                    generator
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn latestTaskNum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <latestTaskNumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::latestTaskNum)
                    }
                    latestTaskNum
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn instantSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <instantSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::instantSlasher)
                    }
                    instantSlasher
                },
                {
                    fn staleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::staleStakesForbidden)
                    }
                    staleStakesForbidden
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOperatorState_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::getOperatorState_1)
                    }
                    getOperatorState_1
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getTaskResponseWindowBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                              data,
                              validate,
                          )
                          .map(
                              IncredibleSquaringTaskManagerCalls::getTaskResponseWindowBlock,
                          )
                    }
                    getTaskResponseWindowBlock
                },
                {
                    fn TASK_CHALLENGE_WINDOW_BLOCK(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                              data,
                              validate,
                          )
                          .map(
                              IncredibleSquaringTaskManagerCalls::TASK_CHALLENGE_WINDOW_BLOCK,
                          )
                    }
                    TASK_CHALLENGE_WINDOW_BLOCK
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>
                    {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerCalls::unpause)
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
    ///Container for all the [`IncredibleSquaringTaskManager`](self) custom errors.
    pub enum IncredibleSquaringTaskManagerErrors {
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
    impl IncredibleSquaringTaskManagerErrors {
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
    impl alloy_sol_types::SolInterface for IncredibleSquaringTaskManagerErrors {
        const NAME: &'static str = "IncredibleSquaringTaskManagerErrors";
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
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<
                IncredibleSquaringTaskManagerErrors,
            >] = &[
                {
                    fn InputEmptyQuorumNumbers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InputEmptyQuorumNumbers)
                    }
                    InputEmptyQuorumNumbers
                },
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn ECMulFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn InvalidReferenceBlocknumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InvalidReferenceBlocknumber)
                    }
                    InvalidReferenceBlocknumber
                },
                {
                    fn InputNonSignerLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InputNonSignerLengthMismatch)
                    }
                    InputNonSignerLengthMismatch
                },
                {
                    fn InvalidBLSPairingKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InvalidBLSPairingKey)
                    }
                    InvalidBLSPairingKey
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::BytesArrayNotOrdered)
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidBLSSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InvalidBLSSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InvalidBLSSignature)
                    }
                    InvalidBLSSignature
                },
                {
                    fn StaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <StaleStakesForbidden as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::StaleStakesForbidden)
                    }
                    StaleStakesForbidden
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn ECAddFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(IncredibleSquaringTaskManagerErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn OnlyRegistryCoordinatorOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::OnlyRegistryCoordinatorOwner)
                    }
                    OnlyRegistryCoordinatorOwner
                },
                {
                    fn InvalidQuorumApkHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::InvalidQuorumApkHash)
                    }
                    InvalidQuorumApkHash
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::BytesArrayLengthTooLong)
                    }
                    BytesArrayLengthTooLong
                },
                {
                    fn NonSignerPubkeysNotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::NonSignerPubkeysNotSorted)
                    }
                    NonSignerPubkeysNotSorted
                },
                {
                    fn ScalarTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>
                    {
                        <ScalarTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(IncredibleSquaringTaskManagerErrors::ScalarTooLarge)
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
    ///Container for all the [`IncredibleSquaringTaskManager`](self) events.
    pub enum IncredibleSquaringTaskManagerEvents {
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
    impl IncredibleSquaringTaskManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                22u8, 149u8, 184u8, 208u8, 110u8, 200u8, 0u8, 180u8, 97u8, 94u8, 116u8, 92u8,
                251u8, 91u8, 208u8, 12u8, 31u8, 40u8, 117u8, 97u8, 93u8, 66u8, 146u8, 92u8, 59u8,
                90u8, 250u8, 84u8, 59u8, 178u8, 76u8, 72u8,
            ],
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
                253u8, 62u8, 38u8, 190u8, 235u8, 89u8, 103u8, 252u8, 90u8, 87u8, 160u8, 68u8,
                105u8, 20u8, 234u8, 188u8, 69u8, 180u8, 170u8, 71u8, 76u8, 103u8, 165u8, 27u8,
                75u8, 81u8, 96u8, 202u8, 198u8, 13u8, 219u8, 5u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IncredibleSquaringTaskManagerEvents {
        const NAME: &'static str = "IncredibleSquaringTaskManagerEvents";
        const COUNT: usize = 10usize;
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
                Some(<NewTaskCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTaskCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NewTaskCreated)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Paused)
                }
                Some(<StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StaleStakesForbiddenUpdate)
                }
                Some(<TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TaskChallengedSuccessfully)
                }
                Some(
                    <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::TaskChallengedUnsuccessfully),
                Some(<TaskCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TaskCompleted)
                }
                Some(<TaskResponded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskResponded as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TaskResponded)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
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
    impl alloy_sol_types::private::IntoLogData for IncredibleSquaringTaskManagerEvents {
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
    /**Creates a new wrapper around an on-chain [`IncredibleSquaringTaskManager`](self) contract instance.

    See the [wrapper's documentation](`IncredibleSquaringTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IncredibleSquaringTaskManagerInstance<T, P, N> {
        IncredibleSquaringTaskManagerInstance::<T, P, N>::new(address, provider)
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
        _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IncredibleSquaringTaskManagerInstance<T, P, N>>,
    > {
        IncredibleSquaringTaskManagerInstance::<T, P, N>::deploy(
            provider,
            _slashingRegistryCoordinator,
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
        _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        IncredibleSquaringTaskManagerInstance::<T, P, N>::deploy_builder(
            provider,
            _slashingRegistryCoordinator,
            _pauserRegistry,
            _taskResponseWindowBlock,
        )
    }
    /**A [`IncredibleSquaringTaskManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IncredibleSquaringTaskManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IncredibleSquaringTaskManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IncredibleSquaringTaskManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IncredibleSquaringTaskManagerInstance")
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
        > IncredibleSquaringTaskManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IncredibleSquaringTaskManager`](self) contract instance.

        See the [wrapper's documentation](`IncredibleSquaringTaskManagerInstance`) for more details.*/
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
            _slashingRegistryCoordinator: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _taskResponseWindowBlock: u32,
        ) -> alloy_contract::Result<IncredibleSquaringTaskManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _slashingRegistryCoordinator,
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
            _slashingRegistryCoordinator: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _taskResponseWindowBlock: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _slashingRegistryCoordinator,
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
    impl<T, P: ::core::clone::Clone, N> IncredibleSquaringTaskManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IncredibleSquaringTaskManagerInstance<T, P, N> {
            IncredibleSquaringTaskManagerInstance {
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
        > IncredibleSquaringTaskManagerInstance<T, P, N>
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
        pub fn WADS_TO_SLASH(&self) -> alloy_contract::SolCallBuilder<T, &P, WADS_TO_SLASHCall, N> {
            self.call_builder(&WADS_TO_SLASHCall {})
        }
        ///Creates a new call builder for the [`aggregator`] function.
        pub fn aggregator(&self) -> alloy_contract::SolCallBuilder<T, &P, aggregatorCall, N> {
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
            numberToBeSquared: alloy::sol_types::private::primitives::aliases::U256,
            quorumThresholdPercentage: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, createNewTaskCall, N> {
            self.call_builder(&createNewTaskCall {
                numberToBeSquared,
                quorumThresholdPercentage,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`generator`] function.
        pub fn generator(&self) -> alloy_contract::SolCallBuilder<T, &P, generatorCall, N> {
            self.call_builder(&generatorCall {})
        }
        ///Creates a new call builder for the [`getBatchOperatorFromId`] function.
        pub fn getBatchOperatorFromId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchOperatorFromIdCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchOperatorIdCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, getCheckSignaturesIndicesCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorState_0Call, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorState_1Call, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapsAtBlockNumberCall, N> {
            self.call_builder(&getQuorumBitmapsAtBlockNumberCall {
                registryCoordinator,
                operatorIds,
                blockNumber,
            })
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
        pub fn instantSlasher(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, instantSlasherCall, N> {
            self.call_builder(&instantSlasherCall {})
        }
        ///Creates a new call builder for the [`latestTaskNum`] function.
        pub fn latestTaskNum(&self) -> alloy_contract::SolCallBuilder<T, &P, latestTaskNumCall, N> {
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
        pub fn pauseAll(&self) -> alloy_contract::SolCallBuilder<T, &P, pauseAllCall, N> {
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
        pub fn paused_1(&self) -> alloy_contract::SolCallBuilder<T, &P, paused_1Call, N> {
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
            task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            taskResponseMetadata: <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
            pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, raiseAndResolveChallengeCall, N> {
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
            task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            nonSignerStakesAndSignature: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, respondToTaskCall, N> {
            self.call_builder(&respondToTaskCall {
                task,
                taskResponse,
                nonSignerStakesAndSignature,
            })
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
        ///Creates a new call builder for the [`taskNumber`] function.
        pub fn taskNumber(&self) -> alloy_contract::SolCallBuilder<T, &P, taskNumberCall, N> {
            self.call_builder(&taskNumberCall {})
        }
        ///Creates a new call builder for the [`taskSuccesfullyChallenged`] function.
        pub fn taskSuccesfullyChallenged(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, taskSuccesfullyChallengedCall, N> {
            self.call_builder(&taskSuccesfullyChallengedCall { _0 })
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
        > IncredibleSquaringTaskManagerInstance<T, P, N>
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
        ///Creates a new event filter for the [`NewTaskCreated`] event.
        pub fn NewTaskCreated_filter(&self) -> alloy_contract::Event<T, &P, NewTaskCreated, N> {
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
        pub fn TaskCompleted_filter(&self) -> alloy_contract::Event<T, &P, TaskCompleted, N> {
            self.event_filter::<TaskCompleted>()
        }
        ///Creates a new event filter for the [`TaskResponded`] event.
        pub fn TaskResponded_filter(&self) -> alloy_contract::Event<T, &P, TaskResponded, N> {
            self.event_filter::<TaskResponded>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
