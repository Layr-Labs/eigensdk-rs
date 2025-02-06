///Module containing a contract's types and functions.
/**

```solidity
library BN254 {
    struct G1Point { uint256 X; uint256 Y; }
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
library ISlashingRegistryCoordinatorTypes {
    type OperatorStatus is uint8;
    struct OperatorInfo { bytes32 operatorId; OperatorStatus status; }
    struct OperatorKickParam { uint8 quorumNumber; address operator; }
    struct OperatorSetParam { uint32 maxOperatorCount; uint16 kickBIPsOfOperatorStake; uint16 kickBIPsOfTotalStake; }
    struct QuorumBitmapUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint192 quorumBitmap; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISlashingRegistryCoordinatorTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<OperatorStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl OperatorStatus {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OperatorStatus {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorStatus {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    /**```solidity
struct OperatorInfo { bytes32 operatorId; OperatorStatus status; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorInfo {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub status: <OperatorStatus as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            OperatorStatus,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            <OperatorStatus as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<OperatorInfo> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorInfo) -> Self {
                (value.operatorId, value.status)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorId: tuple.0,
                    status: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <OperatorStatus as alloy_sol_types::SolType>::tokenize(&self.status),
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
        impl alloy_sol_types::SolType for OperatorInfo {
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
        impl alloy_sol_types::SolStruct for OperatorInfo {
            const NAME: &'static str = "OperatorInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorInfo(bytes32 operatorId,uint8 status)",
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
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorId)
                        .0,
                    <OperatorStatus as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorId,
                    )
                    + <OperatorStatus as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
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
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorId,
                    out,
                );
                <OperatorStatus as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
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
struct OperatorKickParam { uint8 quorumNumber; address operator; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorKickParam {
        pub quorumNumber: u8,
        pub operator: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u8, alloy::sol_types::private::Address);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorKickParam> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorKickParam) -> Self {
                (value.quorumNumber, value.operator)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorKickParam {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    quorumNumber: tuple.0,
                    operator: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorKickParam {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorKickParam {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
        impl alloy_sol_types::SolType for OperatorKickParam {
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
        impl alloy_sol_types::SolStruct for OperatorKickParam {
            const NAME: &'static str = "OperatorKickParam";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorKickParam(uint8 quorumNumber,address operator)",
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
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumNumber)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operator,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorKickParam {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumNumber,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
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
struct OperatorSetParam { uint32 maxOperatorCount; uint16 kickBIPsOfOperatorStake; uint16 kickBIPsOfTotalStake; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSetParam {
        pub maxOperatorCount: u32,
        pub kickBIPsOfOperatorStake: u16,
        pub kickBIPsOfTotalStake: u16,
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
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Uint<16>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, u16, u16);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorSetParam> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSetParam) -> Self {
                (
                    value.maxOperatorCount,
                    value.kickBIPsOfOperatorStake,
                    value.kickBIPsOfTotalStake,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSetParam {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxOperatorCount: tuple.0,
                    kickBIPsOfOperatorStake: tuple.1,
                    kickBIPsOfTotalStake: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorSetParam {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorSetParam {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxOperatorCount),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.kickBIPsOfOperatorStake,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.kickBIPsOfTotalStake),
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
        impl alloy_sol_types::SolType for OperatorSetParam {
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
        impl alloy_sol_types::SolStruct for OperatorSetParam {
            const NAME: &'static str = "OperatorSetParam";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorSetParam(uint32 maxOperatorCount,uint16 kickBIPsOfOperatorStake,uint16 kickBIPsOfTotalStake)",
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
                            &self.maxOperatorCount,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.kickBIPsOfOperatorStake,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.kickBIPsOfTotalStake,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorSetParam {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxOperatorCount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kickBIPsOfOperatorStake,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kickBIPsOfTotalStake,
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
                    &rust.maxOperatorCount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kickBIPsOfOperatorStake,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kickBIPsOfTotalStake,
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
struct QuorumBitmapUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint192 quorumBitmap; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumBitmapUpdate {
        pub updateBlockNumber: u32,
        pub nextUpdateBlockNumber: u32,
        pub quorumBitmap: alloy::sol_types::private::primitives::aliases::U192,
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
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<192>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            u32,
            alloy::sol_types::private::primitives::aliases::U192,
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
        impl ::core::convert::From<QuorumBitmapUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumBitmapUpdate) -> Self {
                (
                    value.updateBlockNumber,
                    value.nextUpdateBlockNumber,
                    value.quorumBitmap,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumBitmapUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    updateBlockNumber: tuple.0,
                    nextUpdateBlockNumber: tuple.1,
                    quorumBitmap: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumBitmapUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumBitmapUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.updateBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nextUpdateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        192,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumBitmap),
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
        impl alloy_sol_types::SolType for QuorumBitmapUpdate {
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
        impl alloy_sol_types::SolStruct for QuorumBitmapUpdate {
            const NAME: &'static str = "QuorumBitmapUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumBitmapUpdate(uint32 updateBlockNumber,uint32 nextUpdateBlockNumber,uint192 quorumBitmap)",
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
                            &self.updateBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nextUpdateBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        192,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumBitmap)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumBitmapUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
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
                    + <alloy::sol_types::sol_data::Uint<
                        192,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumBitmap,
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
                    &rust.updateBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nextUpdateBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    192,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumBitmap,
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
    /**Creates a new wrapper around an on-chain [`ISlashingRegistryCoordinatorTypes`](self) contract instance.

See the [wrapper's documentation](`ISlashingRegistryCoordinatorTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
        ISlashingRegistryCoordinatorTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISlashingRegistryCoordinatorTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISlashingRegistryCoordinatorTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISlashingRegistryCoordinatorTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug
    for ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISlashingRegistryCoordinatorTypesInstance")
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
    > ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISlashingRegistryCoordinatorTypes`](self) contract instance.

See the [wrapper's documentation](`ISlashingRegistryCoordinatorTypesInstance`) for more details.*/
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
    impl<
        T,
        P: ::core::clone::Clone,
        N,
    > ISlashingRegistryCoordinatorTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
            ISlashingRegistryCoordinatorTypesInstance {
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
    > ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
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
    > ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
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
library IStakeRegistryTypes {
    struct StrategyParams { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IStakeRegistryTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct StrategyParams { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyParams {
        pub strategy: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<StrategyParams> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyParams) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyParams {
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
        impl alloy_sol_types::SolType for StrategyParams {
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
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
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
        impl alloy_sol_types::EventTopic for StrategyParams {
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
    /**Creates a new wrapper around an on-chain [`IStakeRegistryTypes`](self) contract instance.

See the [wrapper's documentation](`IStakeRegistryTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IStakeRegistryTypesInstance<T, P, N> {
        IStakeRegistryTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IStakeRegistryTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IStakeRegistryTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IStakeRegistryTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IStakeRegistryTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IStakeRegistryTypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IStakeRegistryTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IStakeRegistryTypes`](self) contract instance.

See the [wrapper's documentation](`IStakeRegistryTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IStakeRegistryTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IStakeRegistryTypesInstance<T, P, N> {
            IStakeRegistryTypesInstance {
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
    > IStakeRegistryTypesInstance<T, P, N> {
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
    > IStakeRegistryTypesInstance<T, P, N> {
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
}

library ISlashingRegistryCoordinatorTypes {
    type OperatorStatus is uint8;
    struct OperatorInfo {
        bytes32 operatorId;
        OperatorStatus status;
    }
    struct OperatorKickParam {
        uint8 quorumNumber;
        address operator;
    }
    struct OperatorSetParam {
        uint32 maxOperatorCount;
        uint16 kickBIPsOfOperatorStake;
        uint16 kickBIPsOfTotalStake;
    }
    struct QuorumBitmapUpdate {
        uint32 updateBlockNumber;
        uint32 nextUpdateBlockNumber;
        uint192 quorumBitmap;
    }
}

library IStakeRegistryTypes {
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }
}

interface SlashingRegistryCoordinator {
    error AlreadyRegisteredForQuorums();
    error BitmapCannotBeZero();
    error BitmapEmpty();
    error BitmapUpdateIsAfterBlockNumber();
    error BitmapValueTooLarge();
    error BytesArrayLengthTooLong();
    error BytesArrayNotOrdered();
    error CannotChurnSelf();
    error CannotKickOperatorAboveThreshold();
    error CannotReregisterYet();
    error ChurnApproverSaltUsed();
    error CurrentlyPaused();
    error ExpModFailed();
    error InputAddressZero();
    error InputLengthMismatch();
    error InsufficientStakeForChurn();
    error InvalidNewPausedStatus();
    error InvalidRegistrationType();
    error InvalidSignature();
    error MaxQuorumsReached();
    error NextBitmapUpdateIsBeforeBlockNumber();
    error NotRegistered();
    error NotRegisteredForQuorum();
    error NotSorted();
    error OnlyAllocationManager();
    error OnlyEjector();
    error OnlyPauser();
    error OnlyUnpauser();
    error OperatorSetsNotEnabled();
    error QuorumDoesNotExist();
    error QuorumOperatorCountMismatch();
    error SignatureExpired();

    event ChurnApproverUpdated(address prevChurnApprover, address newChurnApprover);
    event EjectorUpdated(address prevEjector, address newEjector);
    event Initialized(uint8 version);
    event OperatorDeregistered(address indexed operator, bytes32 indexed operatorId);
    event OperatorRegistered(address indexed operator, bytes32 indexed operatorId);
    event OperatorSetParamsUpdated(uint8 indexed quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam operatorSetParams);
    event OperatorSocketUpdate(bytes32 indexed operatorId, string socket);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event QuorumBlockNumberUpdated(uint8 indexed quorumNumber, uint256 blocknumber);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _stakeRegistry, address _blsApkRegistry, address _indexRegistry, address _socketRegistry, address _allocationManager, address _pauserRegistry);

    function OPERATOR_CHURN_APPROVAL_TYPEHASH() external view returns (bytes32);
    function PUBKEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
    function accountIdentifier() external view returns (address);
    function allocationManager() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function calculateOperatorChurnApprovalDigestHash(address registeringOperator, bytes32 registeringOperatorId, ISlashingRegistryCoordinatorTypes.OperatorKickParam[] memory operatorKickParams, bytes32 salt, uint256 expiry) external view returns (bytes32);
    function churnApprover() external view returns (address);
    function createSlashableStakeQuorum(ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory strategyParams, uint32 lookAheadPeriod) external;
    function createTotalDelegatedStakeQuorum(ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory strategyParams) external;
    function deregisterOperator(address operator, uint32[] memory operatorSetIds) external;
    function ejectOperator(address operator, bytes memory quorumNumbers) external;
    function ejectionCooldown() external view returns (uint256);
    function ejector() external view returns (address);
    function getCurrentQuorumBitmap(bytes32 operatorId) external view returns (uint192);
    function getOperator(address operator) external view returns (ISlashingRegistryCoordinatorTypes.OperatorInfo memory);
    function getOperatorFromId(bytes32 operatorId) external view returns (address);
    function getOperatorId(address operator) external view returns (bytes32);
    function getOperatorSetParams(uint8 quorumNumber) external view returns (ISlashingRegistryCoordinatorTypes.OperatorSetParam memory);
    function getOperatorStatus(address operator) external view returns (ISlashingRegistryCoordinatorTypes.OperatorStatus);
    function getQuorumBitmapAtBlockNumberByIndex(bytes32 operatorId, uint32 blockNumber, uint256 index) external view returns (uint192);
    function getQuorumBitmapHistoryLength(bytes32 operatorId) external view returns (uint256);
    function getQuorumBitmapIndicesAtBlockNumber(uint32 blockNumber, bytes32[] memory operatorIds) external view returns (uint32[] memory);
    function getQuorumBitmapUpdateByIndex(bytes32 operatorId, uint256 index) external view returns (ISlashingRegistryCoordinatorTypes.QuorumBitmapUpdate memory);
    function indexRegistry() external view returns (address);
    function initialize(address _initialOwner, address _churnApprover, address _ejector, uint256 _initialPausedStatus, address _accountIdentifier) external;
    function isChurnApproverSaltUsed(bytes32) external view returns (bool);
    function isM2Quorum(uint8 quorumNumber) external view returns (bool);
    function lastEjectionTimestamp(address) external view returns (uint256);
    function m2QuorumsDisabled() external view returns (bool);
    function numRegistries() external view returns (uint256);
    function operatorSetsEnabled() external view returns (bool);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function pubkeyRegistrationMessageHash(address operator) external view returns (BN254.G1Point memory);
    function quorumCount() external view returns (uint8);
    function quorumUpdateBlockNumber(uint8) external view returns (uint256);
    function registerOperator(address operator, uint32[] memory operatorSetIds, bytes memory data) external;
    function registries(uint256) external view returns (address);
    function renounceOwnership() external;
    function setAccountIdentifier(address _accountIdentifier) external;
    function setChurnApprover(address _churnApprover) external;
    function setEjectionCooldown(uint256 _ejectionCooldown) external;
    function setEjector(address _ejector) external;
    function setOperatorSetParams(uint8 quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams) external;
    function socketRegistry() external view returns (address);
    function stakeRegistry() external view returns (address);
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
    function updateOperators(address[] memory operators) external;
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory quorumNumbers) external;
    function updateSocket(string memory socket) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      },
      {
        "name": "_blsApkRegistry",
        "type": "address",
        "internalType": "contract IBLSApkRegistry"
      },
      {
        "name": "_indexRegistry",
        "type": "address",
        "internalType": "contract IIndexRegistry"
      },
      {
        "name": "_socketRegistry",
        "type": "address",
        "internalType": "contract ISocketRegistry"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      },
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "OPERATOR_CHURN_APPROVAL_TYPEHASH",
    "inputs": [],
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
    "name": "PUBKEY_REGISTRATION_TYPEHASH",
    "inputs": [],
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
    "name": "accountIdentifier",
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
    "name": "allocationManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAllocationManager"
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
    "name": "calculateOperatorChurnApprovalDigestHash",
    "inputs": [
      {
        "name": "registeringOperator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "registeringOperatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "operatorKickParams",
        "type": "tuple[]",
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorKickParam[]",
        "components": [
          {
            "name": "quorumNumber",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          }
        ]
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
    "name": "churnApprover",
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
    "name": "createSlashableStakeQuorum",
    "inputs": [
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
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
        "name": "lookAheadPeriod",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createTotalDelegatedStakeQuorum",
    "inputs": [
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "ejectOperator",
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
    "name": "ejectionCooldown",
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
    "name": "ejector",
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
    "name": "getCurrentQuorumBitmap",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperator",
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
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorInfo",
        "components": [
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum ISlashingRegistryCoordinatorTypes.OperatorStatus"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorFromId",
    "inputs": [
      {
        "name": "operatorId",
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
    "name": "getOperatorSetParams",
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
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorStatus",
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
        "type": "uint8",
        "internalType": "enum ISlashingRegistryCoordinatorTypes.OperatorStatus"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapAtBlockNumberByIndex",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
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
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapHistoryLength",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
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
    "name": "getQuorumBitmapIndicesAtBlockNumber",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
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
    "name": "getQuorumBitmapUpdateByIndex",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
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
        "internalType": "struct ISlashingRegistryCoordinatorTypes.QuorumBitmapUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumBitmap",
            "type": "uint192",
            "internalType": "uint192"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "indexRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IIndexRegistry"
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
      },
      {
        "name": "_churnApprover",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_ejector",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_initialPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_accountIdentifier",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isChurnApproverSaltUsed",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isM2Quorum",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lastEjectionTimestamp",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "m2QuorumsDisabled",
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
    "name": "numRegistries",
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
    "name": "operatorSetsEnabled",
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
    "name": "pubkeyRegistrationMessageHash",
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
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorumCount",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorumUpdateBlockNumber",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
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
    "name": "registerOperator",
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
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registries",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setAccountIdentifier",
    "inputs": [
      {
        "name": "_accountIdentifier",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setChurnApprover",
    "inputs": [
      {
        "name": "_churnApprover",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setEjectionCooldown",
    "inputs": [
      {
        "name": "_ejectionCooldown",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setEjector",
    "inputs": [
      {
        "name": "_ejector",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setOperatorSetParams",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "socketRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISocketRegistry"
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
    "type": "function",
    "name": "updateOperators",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorsForQuorum",
    "inputs": [
      {
        "name": "operatorsPerQuorum",
        "type": "address[][]",
        "internalType": "address[][]"
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
    "name": "updateSocket",
    "inputs": [
      {
        "name": "socket",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "ChurnApproverUpdated",
    "inputs": [
      {
        "name": "prevChurnApprover",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newChurnApprover",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EjectorUpdated",
    "inputs": [
      {
        "name": "prevEjector",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newEjector",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
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
    "name": "OperatorDeregistered",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSetParamsUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSocketUpdate",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "socket",
        "type": "string",
        "indexed": false,
        "internalType": "string"
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
    "name": "QuorumBlockNumberUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "blocknumber",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "AlreadyRegisteredForQuorums",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapCannotBeZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapEmpty",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapUpdateIsAfterBlockNumber",
    "inputs": []
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
    "name": "CannotChurnSelf",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotKickOperatorAboveThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotReregisterYet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ChurnApproverSaltUsed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
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
    "name": "InputLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientStakeForChurn",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidRegistrationType",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MaxQuorumsReached",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NextBitmapUpdateIsBeforeBlockNumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotRegisteredForQuorum",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyAllocationManager",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEjector",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyPauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyUnpauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorSetsNotEnabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumDoesNotExist",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumOperatorCountMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SignatureExpired",
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
pub mod SlashingRegistryCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61020080604052346103805760c081614f2080380380916100208285610384565b8339810103126103805780516001600160a01b03811681036103805760208201516001600160a01b0381168103610380576040830151906001600160a01b0382168203610380576060840151926001600160a01b0384168403610380576080850151946001600160a01b03861686036103805760a001516001600160a01b038116808203610380576040516100b6604082610384565b6016815260208101907f4156535265676973747279436f6f7264696e61746f72000000000000000000008252604051916100f1604084610384565b6006835260208301916576302e302e3160d01b8352519020915190208160e05280610100524660a0526040519060208201925f516020614f005f395f51905f528452604083015260608201524660808201523060a082015260a0815261015860c082610384565b5190206080523060c0525f516020614f005f395f51905f52610120521561037157610140526101a052610180526101c052610160526101e0525f5460ff8160081c1661031c5760ff808216106102e2575b604051614b4490816103bc8239608051816141e4015260a0518161429b015260c051816141ae015260e05181614233015261010051816142590152610120518161421001526101405181818161094a015281816113a301528181611aa801526122350152610160518181816107ba015261214c015261018051818181610b6b01528181610f09015281816114e10152818161291a0152818161362f01528181613adb0152613f7d01526101a051818181610e42015281816115ad0152818161180e015281816128e801528181612e7c015281816136940152818161400601526145d601526101c051818181610ea801528181611c100152818161253c0152818161294c015281816136f6015261407001526101e051818181611042015281816118df01528181612057015281816138f501526139800152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6101a9565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b6339b190bb60e11b5f5260045ffd5b5f80fd5b601f909101601f19168101906001600160401b038211908210176103a757604052565b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8062cf2ab51461038357806303fd34921461037e57806304ec635114610379578063054310e6146103745780630764cb931461036f5780630cf4b7671461036a5780630d3f213414610365578063125e05841461036057806313542a4e1461035b578063136439dd14610356578063143e5915146103515780631478851f1461034c5780631eb812da14610347578063249a0c421461034257806328f61b311461033d578063296bb0641461033857806329d1e0c3146103335780632cdd1e861461032e5780633c2a7f4c146103295780633eef3a51146103245780635140a5481461031f578063530b97a41461031a5780635865c60c14610315578063595c6a67146103105780635ac86ab71461030b5780635b0b829f146103065780635c975abb146103015780635df45946146102fc5780636347c900146102f757806368304835146102f25780636e3b17db146102ed578063715018a6146102e857806381f936d2146102e35780638281ab75146102de57806384ca5213146102d9578063871ef049146102d4578063886f1195146102cf5780638da5cb5b146102ca5780639aa1653d146102c55780639d8e0c23146102c05780639e9923c2146102bb5780639feab859146102b6578063a4d7871f146102b1578063a96f783e146102ac578063adcf73f7146102a7578063b2d8678d146102a2578063c391425e1461029d578063ca0de88214610298578063ca8aa7c714610293578063d72d8dd61461028e578063e65797ad14610289578063ea32afae14610284578063f2fde38b1461027f578063fabc1cbc1461027a5763fd39105a14610275575f80fd5b6122ee565b61220c565b61217b565b612137565b6120a3565b612086565b612042565b612008565b611f64565b611f00565b611ccf565b611cb2565b611c79565b611c3f565b611bfb565b611b86565b611aff565b611ad7565b611a93565b611a63565b611a07565b611776565b611754565b6116f9565b6115dc565b611598565b61153e565b6114cc565b6114af565b61141a565b6113eb565b611378565b61130c565b6111d1565b61110b565b610d74565b610c42565b610c15565b610be8565b610b35565b610b0d565b610adb565b610a53565b610a24565b6109d3565b61091a565b6108df565b6108a4565b610883565b610750565b6106b5565b61068d565b6105ad565b610575565b6104ab565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176103b757604052565b610388565b606081019081106001600160401b038211176103b757604052565b90601f801991011681019081106001600160401b038211176103b757604052565b604051906104076040836103d7565b565b604051906104076060836103d7565b6001600160401b0381116103b75760051b60200190565b6001600160a01b0381160361044057565b5f80fd5b9080601f8301121561044057813561045b81610418565b9261046960405194856103d7565b81845260208085019260051b82010192831161044057602001905b8282106104915750505090565b6020809183356104a08161042f565b815201910190610484565b34610440576020366003190112610440576004356001600160401b038111610440576104db903690600401610444565b6104f26104ec600480600154161490565b15612334565b5f5b81518110156105735760019061056d6001600160a01b036105158386612357565b5116805f52609960205260405f2061054660ff86604051936105368561039c565b805485520154166020830161236b565b6105676105626105568351614167565b6001600160c01b031690565b612d2c565b91612e32565b016104f4565b005b34610440576020366003190112610440576004355f526098602052602060405f2054604051908152f35b63ffffffff81160361044057565b34610440576060366003190112610440576024356105ed6105e76004356105d38461059f565b604435905f52609860205260405f20611524565b50612420565b63ffffffff808251169216918210610674576040816106296106519460206106379501519063ffffffff821615918215610655575b5050612f08565b01516001600160c01b031690565b6040516001600160c01b0390911681529081906020820190565b0390f35b90915061066c9063ffffffff165b63ffffffff1690565b115f80610622565b636cb19aff60e01b5f5260045ffd5b5f91031261044057565b34610440575f36600319011261044057609d546040516001600160a01b039091168152602090f35b34610440575f3660031901126104405760a15460405160109190911c6001600160a01b03168152602090f35b6001600160401b0381116103b757601f01601f191660200190565b929192610708826106e1565b9161071660405193846103d7565b829481845281830111610440578281602093845f960137010152565b9080601f830112156104405781602061074d933591016106fc565b90565b34610440576020366003190112610440576004356001600160401b03811161044057610780903690600401610732565b335f52609960205260ff600160405f20015416600381101561087e5760016107a8911461239d565b335f90815260996020526040902054907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610440575f60405180936378219b3f60e11b8252856004830152604060248301528183816108176044820188612dea565b03925af1908115610879577fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa9261085a9261085f575b5060405191829182612f1e565b0390a2005b8061086d5f610873936103d7565b80610683565b5f61084d565b6123cb565b6112e1565b346104405760203660031901126104405760043561089f612f2f565b60a055005b34610440576020366003190112610440576004356108c18161042f565b60018060a01b03165f52609f602052602060405f2054604051908152f35b34610440576020366003190112610440576004356108fc8161042f565b60018060a01b03165f526099602052602060405f2054604051908152f35b346104405760203660031901126104405760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156108795761057392610990915f916109a4575b506123d6565b61099f600154828116146123ec565b612fbb565b6109c6915060203d6020116109cc575b6109be81836103d7565b8101906123b3565b5f61098a565b503d6109b4565b34610440576020366003190112610440576105736004356109f38161042f565b6109fb612f2f565b60a1805462010000600160b01b03191660109290921b62010000600160b01b0316919091179055565b34610440576020366003190112610440576004355f52609a602052602060ff60405f2054166040519015158152f35b34610440576040366003190112610440576060610a8b6105e7602435600435610a7a612402565b505f52609860205260405f20611524565b6040519063ffffffff815116825263ffffffff6020820151166020830152604060018060c01b03910151166040820152f35b6004359060ff8216820361044057565b359060ff8216820361044057565b346104405760203660031901126104405760ff610af6610abd565b165f52609b602052602060405f2054604051908152f35b34610440575f36600319011261044057609e546040516001600160a01b039091168152602090f35b34610440576020366003190112610440576040516308f6629d60e31b815260048035908201526020816024816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa801561087957610651915f91610bb9575b506040516001600160a01b0390911681529081906020820190565b610bdb915060203d602011610be1575b610bd381836103d7565b810190612451565b5f610b9e565b503d610bc9565b3461044057602036600319011261044057610573600435610c088161042f565b610c10612f2f565b612fed565b3461044057602036600319011261044057610573600435610c358161042f565b610c3d612f2f565b61304b565b34610440576020366003190112610440576040610c69600435610c648161042f565b61247e565b610c7f8251809260208091805184520151910152565bf35b60609060031901126104405760405190610c9a826103bc565b81600435610ca78161059f565b815260243561ffff811681036104405760208201526044359061ffff821682036104405760400152565b6001600160601b0381160361044057565b81601f8201121561044057803590610cf982610418565b92610d0760405194856103d7565b82845260208085019360061b8301019181831161044057602001925b828410610d31575050505090565b6040848303126104405760206040918251610d4b8161039c565b8635610d568161042f565b815282870135610d6581610cd1565b83820152815201930192610d23565b346104405760c036600319011261044057610d8e36610c81565b606435610d9a81610cd1565b6084356001600160401b03811161044057610db9903690600401610ce2565b9060a43591610dc78361059f565b610dcf612f2f565b610ddd60ff60a154166124e4565b60965460ff16938490610e1890610df660c08410612c5e565b610e12610e0288613214565b60ff1660ff196096541617609655565b86613477565b60a15460ff8116806110bc575b610f9e575b5050610e366001612b7b565b610e406001612b7b565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561044057610e96935f809460405196879586948593630662d3e160e51b85528b600486016133a7565b03925af1801561087957610f8a575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156104405760405163136ca0f960e11b815260ff83166004820152905f908290602490829084905af1801561087957610f76575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156104405760405163136ca0f960e11b815260ff83166004820152905f908290602490829084905af1801561087957610f6857005b8061086d5f610573936103d7565b8061086d5f610f84936103d7565b5f610f06565b8061086d5f610f98936103d7565b5f610ea5565b92610faa949194613226565b93610fb58451613273565b945f5b85518110156110015780610ffb610fe2610fd46001948a612357565b51516001600160a01b031690565b610fec838b612357565b6001600160a01b039091169052565b01610fb8565b5091949590929561101f6110136103f8565b63ffffffff9093168352565b602082015261102d8261234a565b526110378161234a565b506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169260101c1690823b1561044057611093925f9283604051809681958294630130fc2760e51b84526004840161329b565b03925af18015610879576110a8575b80610e2a565b8061086d5f6110b6936103d7565b5f6110a2565b506110d96110d58760a25460ff600192161c1660011490565b1590565b610e25565b9181601f84011215610440578235916001600160401b038311610440576020838186019501011161044057565b34610440576040366003190112610440576004356001600160401b038111610440573660238201121561044057806004013561114681610418565b9161115460405193846103d7565b8183526024602084019260051b820101903682116104405760248101925b8284106111a257602435856001600160401b0382116104405761119c6105739236906004016110de565b916124fa565b83356001600160401b038111610440576020916111c6839260243691870101610444565b815201930192611172565b346104405760a0366003190112610440576004356111ee8161042f565b6112596024356111fd8161042f565b6044356112098161042f565b606435906084359261121a8461042f565b5f549561123f60ff600889901c1615809881996112d3575b81156112b3575b50612802565b86611250600160ff195f5416175f55565b61129c576128ca565b61125f57005b61126d61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b6112ae61010061ff00195f5416175f55565b6128ca565b303b159150816112c5575b505f611239565b60ff1660011490505f6112be565b600160ff8216109150611232565b634e487b7160e01b5f52602160045260245ffd5b6003111561087e57565b90600382101561087e5752565b34610440576020366003190112610440576004356113298161042f565b611331612466565b5060018060a01b03165f52609960205260405f2061135960ff6001604051936105368561039c565b60405180916106516020604084019280518552015160208401906112ff565b34610440575f3660031901126104405760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610879576113e3915f916109a457506123d6565b610573612f87565b34610440576020366003190112610440576020600160ff61140a610abd565b161b806001541614604051908152f35b3461044057608036600319011261044057611433610abd565b60603660231901126104405760405161144b816103bc565b6024356114578161059f565b815260443561ffff8116810361044057602082015260643561ffff81168103610440576040820152611487612f2f565b60ff6096541660ff831610156114a05761057391613477565b637310cff560e11b5f5260045ffd5b34610440575f366003190112610440576020600154604051908152f35b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b634e487b7160e01b5f52603260045260245ffd5b8054821015611539575f5260205f2001905f90565b611510565b3461044057602036600319011261044057600435609c5481101561044057609c5f527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c01546040516001600160a01b039091168152602090f35b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610440576040366003190112610440576004356115f98161042f565b6024356001600160401b03811161044057611618903690600401610732565b609e546001600160a01b031633036116ea576001600160a01b0382165f908152609f6020908152604080832042905560999091529020805460016116808161167761167161055661166b60965460ff1690565b89613404565b94614167565b94015460ff1690565b611689816112f5565b1491826116d7575b826116be575b505061169f57005b6116a98183613589565b60a15460ff166116b557005b61057391613859565b81166001600160c01b0390811691161490505f80611697565b6001600160c01b03821615159250611691565b6376d8ab1760e11b5f5260045ffd5b34610440575f36600319011261044057611711612f2f565b606480546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610440575f36600319011261044057602060ff60a154166040519015158152f35b346104405760a03660031901126104405761179036610c81565b60643561179c81610cd1565b6084356001600160401b038111610440576117bb903690600401610ce2565b906117c4612f2f565b60965460ff169283906117ef906117dd60c08410612c5e565b6117e9610e0287613214565b85613477565b60a15460ff811680611959575b611862575b505061180c5f612b7b565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561044057610e96925f9283604051809681958294633aea0b9d60e11b84528a600485016133dc565b9161186f94919394613226565b9261187a8651613273565b935f5b87518110156118a957806118a3611899610fd46001948c612357565b610fec838a612357565b0161187d565b50919395949092956118bc6110136103f8565b60208201526118ca8261234a565b526118d48161234a565b506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169260101c1690823b1561044057611930925f9283604051809681958294630130fc2760e51b84526004840161329b565b03925af1801561087957611945575b80611801565b8061086d5f611953936103d7565b5f61193f565b506119726110d58660a25460ff600192161c1660011490565b6117fc565b81601f820112156104405780359061198e82610418565b9261199c60405194856103d7565b82845260208085019360061b8301019181831161044057602001925b8284106119c6575050505090565b60408483031261044057602060409182516119e08161039c565b6119e987610acd565b8152828701356119f88161042f565b838201528152019301926119b8565b346104405760a036600319011261044057600435611a248161042f565b60243590604435906001600160401b03821161044057602092611a4e611a5b933690600401611977565b606435916084359361298a565b604051908152f35b34610440576020366003190112610440576020611a81600435614167565b6040516001600160c01b039091168152f35b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610440575f366003190112610440576064546040516001600160a01b039091168152602090f35b34610440575f36600319011261044057602060ff60965416604051908152f35b9080601f83011215610440578135611b3681610418565b92611b4460405194856103d7565b81845260208085019260051b82010192831161044057602001905b828210611b6c5750505090565b602080918335611b7b8161059f565b815201910190611b5f565b3461044057604036600319011261044057600435611ba38161042f565b602435906001600160401b03821161044057611bf5611bc9610573933690600401611b1f565b611bd161397e565b611be2600280600154161415612334565b611bf060ff60a154166124e4565b6139bf565b90613589565b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610440575f3660031901126104405760206040517f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de68152f35b34610440576020366003190112610440576020611ca8611c97610abd565b60a25460ff600192161c1660011490565b6040519015158152f35b34610440575f36600319011261044057602060a054604051908152f35b3461044057606036600319011261044057600435611cec8161042f565b6024356001600160401b03811161044057611d0b903690600401611b1f565b906044356001600160401b03811161044057611d79611d31611d609236906004016110de565b929094611d3c61397e565b611d4c6104ec6001808054161490565b611bf0611d5b60a15460ff1690565b6124e4565b918401611d6d8186612b38565b86979297949194613ab9565b95611d8381612b7b565b80611ebd57505050611d9790828585613eb2565b51925f5b8251811015611e205780611e1a8663ffffffff611e11610663611e07611df5611deb88611de5611ddf611dd18f9d60019e612c4d565b516001600160f81b03191690565b60f81c90565b97612357565b5163ffffffff1690565b9460ff165f52609760205260405f2090565b5463ffffffff1690565b91161115612c5e565b01611d9b565b509250505b6001611e4d81611e458460018060a01b03165f52609960205260405f2090565b015460ff1690565b611e56816112f5565b03611e5d57005b611e8e611e686103f8565b838152600160208201526001600160a01b0383165f908152609960205260409020612c74565b6001600160a01b03167fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe5f80a3005b80611ecc600192969496612b7b565b03611ef157611eec93611ede91612b85565b959350939150508686613bbb565b611e25565b63354bb8ab60e01b5f5260045ffd5b34610440575f36600319011261044057602060ff60a15460081c166040519015158152f35b60206040818301928281528451809452019201905f5b818110611f485750505090565b825163ffffffff16845260209384019390920191600101611f3b565b3461044057604036600319011261044057600435611f818161059f565b602435906001600160401b038211610440573660238301121561044057816004013591611fad83610418565b92611fbb60405194856103d7565b8084526024602085019160051b8301019136831161044057602401905b828210611ff857610651611fec8686614108565b60405191829182611f25565b8135815260209182019101611fd8565b34610440575f3660031901126104405760206040517f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a8152f35b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610440575f366003190112610440576020609c54604051908152f35b346104405760203660031901126104405760ff6120be610abd565b6120c6612402565b50165f52609760205261065160405f2061ffff604051916120e6836103bc565b5463ffffffff81168352818160201c16602084015260301c16604082015260405191829182919091604061ffff81606084019563ffffffff8151168552826020820151166020860152015116910152565b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610440576020366003190112610440576004356121988161042f565b6121a0612f2f565b6001600160a01b038116156121b8576105739061342f565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346104405760203660031901126104405760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610879575f916122cf575b506001600160a01b031633036122c05761228e6001541982198116146123ec565b806001556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b6122e8915060203d602011610be157610bd381836103d7565b5f61226d565b346104405760203660031901126104405760043561230b8161042f565b60018060a01b03165f526099602052602060ff600160405f20015416610c7f60405180926112ff565b1561233b57565b63840a48d560e01b5f5260045ffd5b8051156115395760200190565b80518210156115395760209160051b010190565b600382101561087e5752565b906104076040516123878161039c565b602060ff6001839680548552015416910161236b565b156123a457565b63aba4733960e01b5f5260045ffd5b90816020910312610440575180151581036104405790565b6040513d5f823e3d90fd5b156123dd57565b631d77d47760e21b5f5260045ffd5b156123f357565b63c61dca5d60e01b5f5260045ffd5b6040519061240f826103bc565b5f6040838281528260208201520152565b9060405161242d816103bc565b604081935463ffffffff8116835263ffffffff8160201c166020840152811c910152565b90816020910312610440575161074d8161042f565b604051906124738261039c565b5f6020838281520152565b6124df61074d9161248d612466565b50604080517f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de6602082019081526001600160a01b03909316818301529081526124d76060826103d7565b5190206130a9565b6130f6565b156124eb57565b635b77901960e01b5f5260045ffd5b90929161250e6104ec600480600154161490565b61252d61251d60965460ff1690565b6125283684886106fc565b613404565b5061253a8183511461273d565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316935f5b82811061257657505050509050565b612595611ddf612587838686612753565b356001600160f81b03191690565b926125a08286612357565b5180516040516379a0849160e11b815260ff87166004820152919791906020826024818d5afa918215610879576125e69263ffffffff915f9161270f575b501614612774565b5f97885b88518a10156126a35760019061269b6126136126068d8d612357565b516001600160a01b031690565b916126766126396126348560018060a01b03165f52609960205260405f2090565b612377565b9161266161265c8d61264e6105568751614167565b60ff600192161c1660011490565b61278a565b858060a01b0316858060a01b038516116127a0565b61269461268d6126858a6127ca565b8a8a8d6127ea565b36916106fc565b9083612e32565b9901986125ea565b50965096509290600191949294436126c68260ff165f52609b60205260405f2090565b557f46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db460ff6040519216918061270043829190602083019252565b0390a201949394929092612567565b612730915060203d8111612736575b61272881836103d7565b81019061275f565b5f6125de565b503d61271e565b1561274457565b63aaad13f760e01b5f5260045ffd5b90821015611539570190565b90816020910312610440575161074d8161059f565b1561277b57565b638e5aeee760e01b5f5260045ffd5b1561279157565b63d053aa2160e01b5f5260045ffd5b156127a757565b63ba50f91160e01b5f5260045ffd5b634e487b7160e01b5f52601160045260245ffd5b90600182018092116127d857565b6127b6565b919082018092116127d857565b90939293848311610440578411610440578101920390565b1561280957565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b609c54600160401b8110156103b75760018101609c55609c5481101561153957609c5f527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c0180546001600160a01b0319166001600160a01b03909216919091179055565b6128e3949361099f6109fb9493610c10610c3d9461342f565b6129157f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612865565b6129477f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612865565b6129797f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612865565b61010161ffff1960a154161760a155565b919493909260405192602084019460e08501917f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a875260018060a01b03166040860152606085015260c060808501528651809152602061010085019701905f5b818110612a175750505061074d94956124d792849260a084015260c083015203601f1981018352826103d7565b8251805160ff168a526020908101516001600160a01b0316818b0152604090990198909201916001016129ea565b3590600282101561044057565b919082604091031261044057604051612a6a8161039c565b6020808294803584520135910152565b9080601f830112156104405760405191612a956040846103d7565b82906040810192831161044057905b828210612ab15750505090565b8135815260209182019101612aa4565b8082039291610100841261044057604051612adb816103bc565b60808195612ae98486612a52565b8352612af88460408701612a52565b6020840152607f19011261044057612b2f60409260c0845195612b1a8761039c565b612b278360808301612a7a565b875201612a7a565b60208401520152565b90916101408284031261044057612b4e82612a45565b926020830135906001600160401b038211610440576040612b748261074d948701610732565b9401612ac1565b6002111561087e57565b91906101808382031261044057612b9b83612a45565b9260208101356001600160401b0381116104405782612bbb918301610732565b92612bc98360408401612ac1565b926101408301356001600160401b0381116104405781612bea918501611977565b92610160810135906001600160401b0382116104405701906060828203126104405760405191612c19836103bc565b80356001600160401b03811161044057604092612c37918301610732565b8352602081013560208401520135604082015290565b908151811015611539570160200190565b15612c6557565b633cb89c9760e01b5f5260045ffd5b60016020918351815501910151600381101561087e5760ff80198354169116179055565b90604051612ca5816103bc565b604061ffff82945463ffffffff81168452818160201c16602085015260301c16910152565b60408051909190612cdb83826103d7565b6001815291601f1901366020840137565b90612cf6826106e1565b612d0360405191826103d7565b8281528092612d14601f19916106e1565b0190602036910137565b5f1981146127d85760010190565b5f81805b612da65750612d429061ffff16612cec565b5f5f5b8251821080612d9b575b15612d94576001811b8416612d6d575b612d6890612d1e565b612d45565b906001612d689160ff60f81b8460f81b165f1a612d8a8287612c4d565b5301919050612d5f565b5050905090565b506101008110612d4f565b5f1981018181116127d85761ffff9116911661ffff81146127d8576001019080612d30565b9081602091031261044057516001600160c01b03811681036104405790565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b61074d939260609260018060a01b0316825260208201528160408201520190612dea565b919060016020820151612e44816112f5565b612e4d816112f5565b03612f0357516040516333567f7f60e11b81529160209183918291612e7791908760048501612e0e565b03815f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af1908115610879575f91612ed4575b506001600160c01b03169081612ec8575050565b611bf561040792612d2c565b612ef6915060203d602011612efc575b612eee81836103d7565b810190612dcb565b5f612eb4565b503d612ee4565b505050565b15612f0f57565b63bbba60cb60e01b5f5260045ffd5b90602061074d928181520190612dea565b6064546001600160a01b03163303612f4357565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b5f196001556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806001556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b609d54604080516001600160a01b038084168252841660208201529192917f315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c9190a16001600160a01b03166001600160a01b03199190911617609d55565b609e54604080516001600160a01b038084168252841660208201529192917f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc99190a16001600160a01b03166001600160a01b03199190911617609e55565b6130b16141ab565b9060405190602082019261190160f01b845260228301526042820152604281526130dc6062826103d7565b51902090565b634e487b7160e01b5f52601260045260245ffd5b5f516020614aef5f395f51905f529061310d612466565b505f919006602060c0835b61320d575f935f516020614aef5f395f51905f526003818681818009090860405161314385826103d7565b8436823784818560405161315782826103d7565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614aef5f395f51905f5260a082015260056107cf195a01fa8015613212576131c1906147aa565b519161320d575f516020614aef5f395f51905f52828009146131f857505f516020614aef5f395f51905f5260015f94089293613118565b929350506132046103f8565b92835282015290565b6130e2565bfe5b60ff60019116019060ff82116127d857565b6040805190919061323783826103d7565b6001815291601f1901825f5b82811061324f57505050565b60209060405161325e8161039c565b5f815260608382015282828501015201613243565b9061327d82610418565b61328a60405191826103d7565b8281528092612d14601f1991610418565b90604082019060018060a01b031682526040602083015282518091526060820191602060608360051b8301019401925f915b8383106132dc57505050505090565b9091929394605f1982820301835285516020606081604085019363ffffffff81511686520151936040838201528451809452019201905f905b80821061333457505050602080600192970193019301919392906132cd565b82516001600160a01b0316845260209384019390920191600190910190613315565b90602080835192838152019201905f5b8181106133735750505090565b825180516001600160a01b031685526020908101516001600160601b03168186015260409094019390920191600101613366565b9061074d94936001600160601b0360809460ff63ffffffff941685521660208401521660408201528160608201520190613356565b6001600160601b0361074d949360ff6060941683521660208201528160408201520190613356565b90600161341260ff936142d7565b928392161b11156134205790565b63ca95733360e01b5f5260045ffd5b606480546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61353560ff7f3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac921692835f52609760205260405f206134cd63ffffffff835116829063ffffffff1663ffffffff19825416179055565b6020820151815465ffff0000000067ffff000000000000604086015160301b169260201b169067ffffffff0000000019161717905560405191829182919091604061ffff81606084019563ffffffff8151168552826020820151166020860152015116910152565b0390a2565b1561354157565b6368b6a87560e11b5f5260045ffd5b6001600160a01b03909116815260406020820181905261074d92910190612dea565b60409061074d939281528160208201520190612dea565b6001600160a01b0381165f908152609960205260409020906136148254926135c860016135b98161167788614167565b6135c2816112f5565b1461239d565b6135e36105566105566135dd60965460ff1690565b88613404565b906135ef82151561353a565b6136058282166001600160c01b0316831461278a565b9019166001600160c01b031690565b61361e818461434f565b6001600160c01b031615613790575b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561044057835f91613682938360405180968195829463f4e24fe560e01b845260048401613550565b03925af180156108795761377c575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b15610440575f604051809263bd29b8cd60e01b82528183816136e4898960048401613572565b03925af1801561087957613768575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561044057613749925f928360405180968195829463bd29b8cd60e01b845260048401613572565b03925af180156108795761375a5750565b8061086d5f610407936103d7565b8061086d5f613776936103d7565b5f6136f3565b8061086d5f61378a936103d7565b5f613691565b6001600160a01b0381165f9081526099602052604090206137bb90600101805460ff19166002179055565b816001600160a01b0382167f396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e45f80a361362d565b602080825282516001600160a01b039081168284015281840151166040808401919091529092015160608083015280516080830181905260a09092019201905f5b81811061383d5750505090565b825163ffffffff16845260209384019390920191600101613830565b90916138658351613273565b60a2545f92835b86518110156138d757613885611ddf611dd1838a612c4d565b613898818560ff600192161c1660011490565b6138a6575b5060010161386c565b94906138d1826138c660ff6138bd60019693612d1e565b99169188612357565b9063ffffffff169052565b9061389d565b5094505090806138e657505050565b815260a1546001600160a01b037f00000000000000000000000000000000000000000000000000000000000000008116929161394d9160109190911c1661393d61392e610409565b6001600160a01b039096168652565b6001600160a01b03166020850152565b6040830152803b1561044057604051636e3492b560e01b8152915f91839182908490829061374990600483016137ef565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036139b057565b6323d871a560e01b5f5260045ffd5b906139ca8251612cec565b5f5b8351811015613a03576001906001600160f81b03196139eb8287612357565b5160f81b165f1a6139fc8285612c4d565b53016139cc565b509150565b90816020910312610440575190565b905f905b60028210613a2857505050565b6020806001928551815201930191019091613a1b565b61012090613aa860206040610407969897959861016085019960018060a01b03168552613a78838601825160208091805184520151910152565b80830151805160608701526020015160808601520151613a9c60a085018251613a17565b015160e0830190613a17565b019060208091805184520151910152565b6040516309aa152760e11b81526001600160a01b0382811660048301529091907f000000000000000000000000000000000000000000000000000000000000000016602083602481845afa928315610879575f93613b9a575b508215613b20575050905090565b60209250613b50935f613b328461247e565b6040516317ef39cb60e31b8152968795869485939160048501613a3e565b03925af1908115610879575f91613b6b575b50805f80612d94565b613b8d915060203d602011613b93575b613b8581836103d7565b810190613a08565b5f613b62565b503d613b7b565b613bb491935060203d602011613b9357613b8581836103d7565b915f613b12565b929091613be392613bdd829897613bd5885185511461273d565b878388614469565b84613eb2565b905f5b8551811015613d185780613c1e613c19613c08611ddf611dd16001968c612c4d565b60ff165f52609760205260405f2090565b612c98565b613c2c611deb838751612357565b63ffffffff613c42610663845163ffffffff1690565b911611613c51575b5001613be6565b613ca790613c65611ddf611dd1858c612c4d565b613c83613c768560408a0151612357565b516001600160601b031690565b86613c95613c768760208c0151612357565b91613ca0878c612357565b519361454a565b613caf612cca565b613cbc611dd1838a612c4d565b5f1a613cc78261234a565b53613cef81613cea6020613cdb868b612357565b5101516001600160a01b031690565b613589565b60a15460ff1615613c4a57613d1290613d0d6020613cdb858a612357565b613859565b5f613c4a565b505050509050565b60405190613d2d826103bc565b60606040838281528260208201520152565b15613d4657565b6313ca465760e01b5f5260045ffd5b15613d5c57565b630c6816cd60e01b5f5260045ffd5b15613d7257565b631968677d60e11b5f5260045ffd5b9080601f83011215610440578151613d9881610418565b92613da660405194856103d7565b81845260208085019260051b82010192831161044057602001905b828210613dce5750505090565b602080918351613ddd81610cd1565b815201910190613dc1565b9190916040818403126104405780516001600160401b0381116104405783613e11918301613d81565b9260208201516001600160401b0381116104405761074d9201613d81565b602081830312610440578051906001600160401b03821161044057019080601f83011215610440578151613e6281610418565b92613e7060405194856103d7565b81845260208085019260051b82010192831161044057602001905b828210613e985750505090565b602080918351613ea78161059f565b815201910190613e8b565b817fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa613f78613edf613d20565b96613f6c613efb610556613ef560965460ff1690565b8a613404565b613f0486614167565b6001600160c01b0390911690613f1b821515613d3f565b60018060c01b0316613f35613f308284161590565b613d55565b6001600160a01b0388165f908152609f60205260409020613f6590613f5e905460a054906127dd565b4211613d6b565b178561434f565b60405191829182612f1e565b0390a27f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b15610440575f6040518092631fd93ca960e11b8252818381613fcd8a8960048401613550565b03925af180156108795784925f9285926140f4575b506140016040519687938493632550477760e01b855260048501612e0e565b0381837f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af19182156108795761406b935f93849185916140d0575b5060408701526020860152604051938492839262bff04d60e01b845260048401613572565b0381837f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af1908115610879575f916140ae575b50815290565b6140ca91503d805f833e6140c281836103d7565b810190613e2f565b5f6140a8565b90506140ee91503d8086833e6140e681836103d7565b810190613de8565b5f614046565b8061086d85614102936103d7565b5f613fe2565b91906141148151613273565b905f5b8151811015614152578061413961413060019385612357565b518760986146a0565b63ffffffff6141488387612357565b9116905201614117565b5090925050565b5f198101919082116127d857565b805f52609860205260405f20549081155f146141835750505f90565b5f52609860205260405f20905f1981019081116127d8576141a391611524565b505460401c90565b307f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03161480614298575b15614206577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f000000000000000000000000000000000000000000000000000000000000000082527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a081526130dc60c0826103d7565b507f000000000000000000000000000000000000000000000000000000000000000046146141dd565b156142c857565b631019106960e31b5f5260045ffd5b906101008251116143405781511561433b57602082015160019060f81c81901b5b835182101561433657600190614321614317611ddf611dd18689612c4d565b60ff600191161b90565b9061432d8183116142c1565b179101906142f8565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b90614362825f52609860205260405f2090565b54806143af575061437e610407925f52609860205260405f2090565b6143aa614389610409565b4363ffffffff168152925b5f60208501526001600160c01b03166040840152565b6147c0565b916143da63ffffffff936143d46143ce845f52609860205260405f2090565b91614159565b90611524565b50906143ea825463ffffffff1690565b4385169416840361441557506104079250906001600160401b0382549181199060401b169116179055565b815467ffffffff000000001916602085901b67ffffffff0000000016179091556104079291906143aa90614451905f52609860205260405f2090565b9161439461445d610409565b63ffffffff9095168552565b919290602082019283515f52609a60205260ff60405f2054166144e45760408301805142116144d557610407956144cd9386515f52609a6020526144b760405f20600160ff19825416179055565b609d546001600160a01b0316965192519361298a565b90519161482e565b630819bdcd60e01b5f5260045ffd5b636fbefec360e11b5f5260045ffd5b156144fa57565b6356168b4160e11b5f5260045ffd5b90816020910312610440575161074d81610cd1565b1561452557565b634c44995d60e01b5f5260045ffd5b1561453b57565b63b187e86960e01b5f5260045ffd5b602091926145a761459a6145d298969761459361456f8783015160018060a01b031690565b6001600160a01b039081165f818152609960205260409020549690911614156144f3565b5160ff1690565b60ff808516911614612774565b604051635401ed2760e01b8152600481019190915260ff909116602482015294859081906044820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa92831561087957610407945f94614656575b508261464e92614649614635936001600160601b0361464161463582998b614872565b6001600160601b031690565b91161161451e565b614895565b911610614534565b6146359194509261464e9261464961468d6001600160601b039660203d602011614699575b61468581836103d7565b810190614509565b96935050925092614612565b503d61467b565b9190815f528260205260405f2054925f5b8481106147495760405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a490fd5b8085038581116127d85761066361475f91614159565b61478861477d826147788887905f5260205260405f2090565b611524565b505463ffffffff1690565b63ffffffff808616911611156147a157506001016146b1565b94505050505090565b156147b157565b63d51edae360e01b5f5260045ffd5b8054600160401b8110156103b7576147dd91600182018155611524565b61481b57815160208084015160409485015163ffffffff909316911b67ffffffff00000000161767ffffffffffffffff199190931b16919091179055565b634e487b7160e01b5f525f60045260245ffd5b90614839929161490c565b1561484057565b638baa579f60e01b5f5260045ffd5b906001600160601b03809116911602906001600160601b0382169182036127d857565b6148906001600160601b039161ffff6020612710950151169061484f565b160490565b6148906001600160601b039161ffff6040612710950151169061484f565b6005111561087e57565b3d156148e7573d906148ce826106e1565b916148dc60405193846103d7565b82523d5f602084013e565b606090565b9081602091031261044057516001600160e01b0319811681036104405790565b91909161491982846149d7565b614922816148b3565b1590816149c1575b506149b9575f9261495761496585946040519283916020830195630b135d3f60e11b875260248401613572565b03601f1981018352826103d7565b51915afa6149716148bd565b816149ad575b81614980575090565b8051630b135d3f60e11b92506001600160e01b0319916149a8918101602090810191016148ec565b161490565b80516020149150614977565b505050600190565b6001600160a01b0383811691161490505f61492a565b815160418103614a035750906149ff91602082015190606060408401519301515f1a90614a45565b9091565b604003614a3c5760406020830151920151918260ff1c91601b83018093116127d8576149ff936001600160ff1b03169260ff1690614a45565b50505f90600290565b9291907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311614ae35760ff16601b81141580614ad8575b614acd576020935f93604051938493608085019385528785015260408401526060830152838052039060015afa15610879575f516001600160a01b03811615614ac557905f90565b505f90600190565b505050505f90600490565b50601c811415614a7d565b505050505f9060039056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220a0f52feb6762df4a106a0afcb3c8bdddc282cea4885da5717dd983812850244964736f6c634300081b00338b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x02\0\x80`@R4a\x03\x80W`\xC0\x81aO \x808\x03\x80\x91a\0 \x82\x85a\x03\x84V[\x839\x81\x01\x03\x12a\x03\x80W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x80W` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x80W`@\x83\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\x80W``\x84\x01Q\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x84\x03a\x03\x80W`\x80\x85\x01Q\x94`\x01`\x01`\xA0\x1B\x03\x86\x16\x86\x03a\x03\x80W`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x82\x03a\x03\x80W`@Qa\0\xB6`@\x82a\x03\x84V[`\x16\x81R` \x81\x01\x90\x7FAVSRegistryCoordinator\0\0\0\0\0\0\0\0\0\0\x82R`@Q\x91a\0\xF1`@\x84a\x03\x84V[`\x06\x83R` \x83\x01\x91ev0.0.1`\xD0\x1B\x83RQ\x90 \x91Q\x90 \x81`\xE0R\x80a\x01\0RF`\xA0R`@Q\x90` \x82\x01\x92_Q` aO\0_9_Q\x90_R\x84R`@\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x01X`\xC0\x82a\x03\x84V[Q\x90 `\x80R0`\xC0R_Q` aO\0_9_Q\x90_Ra\x01 R\x15a\x03qWa\x01@Ra\x01\xA0Ra\x01\x80Ra\x01\xC0Ra\x01`Ra\x01\xE0R_T`\xFF\x81`\x08\x1C\x16a\x03\x1CW`\xFF\x80\x82\x16\x10a\x02\xE2W[`@QaKD\x90\x81a\x03\xBC\x829`\x80Q\x81aA\xE4\x01R`\xA0Q\x81aB\x9B\x01R`\xC0Q\x81aA\xAE\x01R`\xE0Q\x81aB3\x01Ra\x01\0Q\x81aBY\x01Ra\x01 Q\x81aB\x10\x01Ra\x01@Q\x81\x81\x81a\tJ\x01R\x81\x81a\x13\xA3\x01R\x81\x81a\x1A\xA8\x01Ra\"5\x01Ra\x01`Q\x81\x81\x81a\x07\xBA\x01Ra!L\x01Ra\x01\x80Q\x81\x81\x81a\x0Bk\x01R\x81\x81a\x0F\t\x01R\x81\x81a\x14\xE1\x01R\x81\x81a)\x1A\x01R\x81\x81a6/\x01R\x81\x81a:\xDB\x01Ra?}\x01Ra\x01\xA0Q\x81\x81\x81a\x0EB\x01R\x81\x81a\x15\xAD\x01R\x81\x81a\x18\x0E\x01R\x81\x81a(\xE8\x01R\x81\x81a.|\x01R\x81\x81a6\x94\x01R\x81\x81a@\x06\x01RaE\xD6\x01Ra\x01\xC0Q\x81\x81\x81a\x0E\xA8\x01R\x81\x81a\x1C\x10\x01R\x81\x81a%<\x01R\x81\x81a)L\x01R\x81\x81a6\xF6\x01Ra@p\x01Ra\x01\xE0Q\x81\x81\x81a\x10B\x01R\x81\x81a\x18\xDF\x01R\x81\x81a W\x01R\x81\x81a8\xF5\x01Ra9\x80\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\x01\xA9V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[_\x80\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x03\xA7W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\xCF*\xB5\x14a\x03\x83W\x80c\x03\xFD4\x92\x14a\x03~W\x80c\x04\xECcQ\x14a\x03yW\x80c\x05C\x10\xE6\x14a\x03tW\x80c\x07d\xCB\x93\x14a\x03oW\x80c\x0C\xF4\xB7g\x14a\x03jW\x80c\r?!4\x14a\x03eW\x80c\x12^\x05\x84\x14a\x03`W\x80c\x13T*N\x14a\x03[W\x80c\x13d9\xDD\x14a\x03VW\x80c\x14>Y\x15\x14a\x03QW\x80c\x14x\x85\x1F\x14a\x03LW\x80c\x1E\xB8\x12\xDA\x14a\x03GW\x80c$\x9A\x0CB\x14a\x03BW\x80c(\xF6\x1B1\x14a\x03=W\x80c)k\xB0d\x14a\x038W\x80c)\xD1\xE0\xC3\x14a\x033W\x80c,\xDD\x1E\x86\x14a\x03.W\x80c<*\x7FL\x14a\x03)W\x80c>\xEF:Q\x14a\x03$W\x80cQ@\xA5H\x14a\x03\x1FW\x80cS\x0B\x97\xA4\x14a\x03\x1AW\x80cXe\xC6\x0C\x14a\x03\x15W\x80cY\\jg\x14a\x03\x10W\x80cZ\xC8j\xB7\x14a\x03\x0BW\x80c[\x0B\x82\x9F\x14a\x03\x06W\x80c\\\x97Z\xBB\x14a\x03\x01W\x80c]\xF4YF\x14a\x02\xFCW\x80ccG\xC9\0\x14a\x02\xF7W\x80ch0H5\x14a\x02\xF2W\x80cn;\x17\xDB\x14a\x02\xEDW\x80cqP\x18\xA6\x14a\x02\xE8W\x80c\x81\xF96\xD2\x14a\x02\xE3W\x80c\x82\x81\xABu\x14a\x02\xDEW\x80c\x84\xCAR\x13\x14a\x02\xD9W\x80c\x87\x1E\xF0I\x14a\x02\xD4W\x80c\x88o\x11\x95\x14a\x02\xCFW\x80c\x8D\xA5\xCB[\x14a\x02\xCAW\x80c\x9A\xA1e=\x14a\x02\xC5W\x80c\x9D\x8E\x0C#\x14a\x02\xC0W\x80c\x9E\x99#\xC2\x14a\x02\xBBW\x80c\x9F\xEA\xB8Y\x14a\x02\xB6W\x80c\xA4\xD7\x87\x1F\x14a\x02\xB1W\x80c\xA9ox>\x14a\x02\xACW\x80c\xAD\xCFs\xF7\x14a\x02\xA7W\x80c\xB2\xD8g\x8D\x14a\x02\xA2W\x80c\xC3\x91B^\x14a\x02\x9DW\x80c\xCA\r\xE8\x82\x14a\x02\x98W\x80c\xCA\x8A\xA7\xC7\x14a\x02\x93W\x80c\xD7-\x8D\xD6\x14a\x02\x8EW\x80c\xE6W\x97\xAD\x14a\x02\x89W\x80c\xEA2\xAF\xAE\x14a\x02\x84W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x7FW\x80c\xFA\xBC\x1C\xBC\x14a\x02zWc\xFD9\x10Z\x14a\x02uW_\x80\xFD[a\"\xEEV[a\"\x0CV[a!{V[a!7V[a \xA3V[a \x86V[a BV[a \x08V[a\x1FdV[a\x1F\0V[a\x1C\xCFV[a\x1C\xB2V[a\x1CyV[a\x1C?V[a\x1B\xFBV[a\x1B\x86V[a\x1A\xFFV[a\x1A\xD7V[a\x1A\x93V[a\x1AcV[a\x1A\x07V[a\x17vV[a\x17TV[a\x16\xF9V[a\x15\xDCV[a\x15\x98V[a\x15>V[a\x14\xCCV[a\x14\xAFV[a\x14\x1AV[a\x13\xEBV[a\x13xV[a\x13\x0CV[a\x11\xD1V[a\x11\x0BV[a\rtV[a\x0CBV[a\x0C\x15V[a\x0B\xE8V[a\x0B5V[a\x0B\rV[a\n\xDBV[a\nSV[a\n$V[a\t\xD3V[a\t\x1AV[a\x08\xDFV[a\x08\xA4V[a\x08\x83V[a\x07PV[a\x06\xB5V[a\x06\x8DV[a\x05\xADV[a\x05uV[a\x04\xABV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xB7W`@RV[a\x03\x88V[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xB7W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xB7W`@RV[`@Q\x90a\x04\x07`@\x83a\x03\xD7V[V[`@Q\x90a\x04\x07``\x83a\x03\xD7V[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xB7W`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04@WV[_\x80\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x04@W\x815a\x04[\x81a\x04\x18V[\x92a\x04i`@Q\x94\x85a\x03\xD7V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04@W` \x01\x90[\x82\x82\x10a\x04\x91WPPP\x90V[` \x80\x91\x835a\x04\xA0\x81a\x04/V[\x81R\x01\x91\x01\x90a\x04\x84V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x04\xDB\x906\x90`\x04\x01a\x04DV[a\x04\xF2a\x04\xEC`\x04\x80`\x01T\x16\x14\x90V[\x15a#4V[_[\x81Q\x81\x10\x15a\x05sW`\x01\x90a\x05m`\x01`\x01`\xA0\x1B\x03a\x05\x15\x83\x86a#WV[Q\x16\x80_R`\x99` R`@_ a\x05F`\xFF\x86`@Q\x93a\x056\x85a\x03\x9CV[\x80T\x85R\x01T\x16` \x83\x01a#kV[a\x05ga\x05ba\x05V\x83QaAgV[`\x01`\x01`\xC0\x1B\x03\x16\x90V[a-,V[\x91a.2V[\x01a\x04\xF4V[\0[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045_R`\x98` R` `@_ T`@Q\x90\x81R\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x04@WV[4a\x04@W``6`\x03\x19\x01\x12a\x04@W`$5a\x05\xEDa\x05\xE7`\x045a\x05\xD3\x84a\x05\x9FV[`D5\x90_R`\x98` R`@_ a\x15$V[Pa$ V[c\xFF\xFF\xFF\xFF\x80\x82Q\x16\x92\x16\x91\x82\x10a\x06tW`@\x81a\x06)a\x06Q\x94` a\x067\x95\x01Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x15\x91\x82\x15a\x06UW[PPa/\x08V[\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x90\x91Pa\x06l\x90c\xFF\xFF\xFF\xFF\x16[c\xFF\xFF\xFF\xFF\x16\x90V[\x11_\x80a\x06\"V[cl\xB1\x9A\xFF`\xE0\x1B_R`\x04_\xFD[_\x91\x03\x12a\x04@WV[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`\x9DT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`\xA1T`@Q`\x10\x91\x90\x91\x1C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xB7W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\x08\x82a\x06\xE1V[\x91a\x07\x16`@Q\x93\x84a\x03\xD7V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x04@W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x04@W\x81` a\x07M\x935\x91\x01a\x06\xFCV[\x90V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x07\x80\x906\x90`\x04\x01a\x072V[3_R`\x99` R`\xFF`\x01`@_ \x01T\x16`\x03\x81\x10\x15a\x08~W`\x01a\x07\xA8\x91\x14a#\x9DV[3_\x90\x81R`\x99` R`@\x90 T\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04@W_`@Q\x80\x93cx!\x9B?`\xE1\x1B\x82R\x85`\x04\x83\x01R`@`$\x83\x01R\x81\x83\x81a\x08\x17`D\x82\x01\x88a-\xEAV[\x03\x92Z\xF1\x90\x81\x15a\x08yW\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x92a\x08Z\x92a\x08_W[P`@Q\x91\x82\x91\x82a/\x1EV[\x03\x90\xA2\0[\x80a\x08m_a\x08s\x93a\x03\xD7V[\x80a\x06\x83V[_a\x08MV[a#\xCBV[a\x12\xE1V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a\x08\x9Fa//V[`\xA0U\0[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a\x08\xC1\x81a\x04/V[`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R` `@_ T`@Q\x90\x81R\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a\x08\xFC\x81a\x04/V[`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R` `@_ T`@Q\x90\x81R\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x08yWa\x05s\x92a\t\x90\x91_\x91a\t\xA4W[Pa#\xD6V[a\t\x9F`\x01T\x82\x81\x16\x14a#\xECV[a/\xBBV[a\t\xC6\x91P` =` \x11a\t\xCCW[a\t\xBE\x81\x83a\x03\xD7V[\x81\x01\x90a#\xB3V[_a\t\x8AV[P=a\t\xB4V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@Wa\x05s`\x045a\t\xF3\x81a\x04/V[a\t\xFBa//V[`\xA1\x80Tb\x01\0\0`\x01`\xB0\x1B\x03\x19\x16`\x10\x92\x90\x92\x1Bb\x01\0\0`\x01`\xB0\x1B\x03\x16\x91\x90\x91\x17\x90UV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045_R`\x9A` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04@W`@6`\x03\x19\x01\x12a\x04@W``a\n\x8Ba\x05\xE7`$5`\x045a\nza$\x02V[P_R`\x98` R`@_ a\x15$V[`@Q\x90c\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@`\x01\x80`\xC0\x1B\x03\x91\x01Q\x16`@\x82\x01R\xF3[`\x045\x90`\xFF\x82\x16\x82\x03a\x04@WV[5\x90`\xFF\x82\x16\x82\x03a\x04@WV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\xFFa\n\xF6a\n\xBDV[\x16_R`\x9B` R` `@_ T`@Q\x90\x81R\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`\x9ET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x805\x90\x82\x01R` \x81`$\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x08yWa\x06Q\x91_\x91a\x0B\xB9W[P`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[a\x0B\xDB\x91P` =` \x11a\x0B\xE1W[a\x0B\xD3\x81\x83a\x03\xD7V[\x81\x01\x90a$QV[_a\x0B\x9EV[P=a\x0B\xC9V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@Wa\x05s`\x045a\x0C\x08\x81a\x04/V[a\x0C\x10a//V[a/\xEDV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@Wa\x05s`\x045a\x0C5\x81a\x04/V[a\x0C=a//V[a0KV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`@a\x0Ci`\x045a\x0Cd\x81a\x04/V[a$~V[a\x0C\x7F\x82Q\x80\x92` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\xF3[``\x90`\x03\x19\x01\x12a\x04@W`@Q\x90a\x0C\x9A\x82a\x03\xBCV[\x81`\x045a\x0C\xA7\x81a\x05\x9FV[\x81R`$5a\xFF\xFF\x81\x16\x81\x03a\x04@W` \x82\x01R`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x04@W`@\x01RV[`\x01`\x01``\x1B\x03\x81\x16\x03a\x04@WV[\x81`\x1F\x82\x01\x12\x15a\x04@W\x805\x90a\x0C\xF9\x82a\x04\x18V[\x92a\r\x07`@Q\x94\x85a\x03\xD7V[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x04@W` \x01\x92[\x82\x84\x10a\r1WPPPP\x90V[`@\x84\x83\x03\x12a\x04@W` `@\x91\x82Qa\rK\x81a\x03\x9CV[\x865a\rV\x81a\x04/V[\x81R\x82\x87\x015a\re\x81a\x0C\xD1V[\x83\x82\x01R\x81R\x01\x93\x01\x92a\r#V[4a\x04@W`\xC06`\x03\x19\x01\x12a\x04@Wa\r\x8E6a\x0C\x81V[`d5a\r\x9A\x81a\x0C\xD1V[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\r\xB9\x906\x90`\x04\x01a\x0C\xE2V[\x90`\xA45\x91a\r\xC7\x83a\x05\x9FV[a\r\xCFa//V[a\r\xDD`\xFF`\xA1T\x16a$\xE4V[`\x96T`\xFF\x16\x93\x84\x90a\x0E\x18\x90a\r\xF6`\xC0\x84\x10a,^V[a\x0E\x12a\x0E\x02\x88a2\x14V[`\xFF\x16`\xFF\x19`\x96T\x16\x17`\x96UV[\x86a4wV[`\xA1T`\xFF\x81\x16\x80a\x10\xBCW[a\x0F\x9EW[PPa\x0E6`\x01a+{V[a\x0E@`\x01a+{V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@Wa\x0E\x96\x93_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x06b\xD3\xE1`\xE5\x1B\x85R\x8B`\x04\x86\x01a3\xA7V[\x03\x92Z\xF1\x80\x15a\x08yWa\x0F\x8AW[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x08yWa\x0FvW[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x08yWa\x0FhW\0[\x80a\x08m_a\x05s\x93a\x03\xD7V[\x80a\x08m_a\x0F\x84\x93a\x03\xD7V[_a\x0F\x06V[\x80a\x08m_a\x0F\x98\x93a\x03\xD7V[_a\x0E\xA5V[\x92a\x0F\xAA\x94\x91\x94a2&V[\x93a\x0F\xB5\x84Qa2sV[\x94_[\x85Q\x81\x10\x15a\x10\x01W\x80a\x0F\xFBa\x0F\xE2a\x0F\xD4`\x01\x94\x8Aa#WV[QQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x0F\xEC\x83\x8Ba#WV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\x0F\xB8V[P\x91\x94\x95\x90\x92\x95a\x10\x1Fa\x10\x13a\x03\xF8V[c\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[` \x82\x01Ra\x10-\x82a#JV[Ra\x107\x81a#JV[P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92`\x10\x1C\x16\x90\x82;\x15a\x04@Wa\x10\x93\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\x010\xFC'`\xE5\x1B\x84R`\x04\x84\x01a2\x9BV[\x03\x92Z\xF1\x80\x15a\x08yWa\x10\xA8W[\x80a\x0E*V[\x80a\x08m_a\x10\xB6\x93a\x03\xD7V[_a\x10\xA2V[Pa\x10\xD9a\x10\xD5\x87`\xA2T`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[\x15\x90V[a\x0E%V[\x91\x81`\x1F\x84\x01\x12\x15a\x04@W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x04@W` \x83\x81\x86\x01\x95\x01\x01\x11a\x04@WV[4a\x04@W`@6`\x03\x19\x01\x12a\x04@W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04@W6`#\x82\x01\x12\x15a\x04@W\x80`\x04\x015a\x11F\x81a\x04\x18V[\x91a\x11T`@Q\x93\x84a\x03\xD7V[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x04@W`$\x81\x01\x92[\x82\x84\x10a\x11\xA2W`$5\x85`\x01`\x01`@\x1B\x03\x82\x11a\x04@Wa\x11\x9Ca\x05s\x926\x90`\x04\x01a\x10\xDEV[\x91a$\xFAV[\x835`\x01`\x01`@\x1B\x03\x81\x11a\x04@W` \x91a\x11\xC6\x83\x92`$6\x91\x87\x01\x01a\x04DV[\x81R\x01\x93\x01\x92a\x11rV[4a\x04@W`\xA06`\x03\x19\x01\x12a\x04@W`\x045a\x11\xEE\x81a\x04/V[a\x12Y`$5a\x11\xFD\x81a\x04/V[`D5a\x12\t\x81a\x04/V[`d5\x90`\x845\x92a\x12\x1A\x84a\x04/V[_T\x95a\x12?`\xFF`\x08\x89\x90\x1C\x16\x15\x80\x98\x81\x99a\x12\xD3W[\x81\x15a\x12\xB3W[Pa(\x02V[\x86a\x12P`\x01`\xFF\x19_T\x16\x17_UV[a\x12\x9CWa(\xCAV[a\x12_W\0[a\x12ma\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x12\xAEa\x01\0a\xFF\0\x19_T\x16\x17_UV[a(\xCAV[0;\x15\x91P\x81a\x12\xC5W[P_a\x129V[`\xFF\x16`\x01\x14\x90P_a\x12\xBEV[`\x01`\xFF\x82\x16\x10\x91Pa\x122V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x11\x15a\x08~WV[\x90`\x03\x82\x10\x15a\x08~WRV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a\x13)\x81a\x04/V[a\x131a$fV[P`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ a\x13Y`\xFF`\x01`@Q\x93a\x056\x85a\x03\x9CV[`@Q\x80\x91a\x06Q` `@\x84\x01\x92\x80Q\x85R\x01Q` \x84\x01\x90a\x12\xFFV[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x08yWa\x13\xE3\x91_\x91a\t\xA4WPa#\xD6V[a\x05sa/\x87V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W` `\x01`\xFFa\x14\na\n\xBDV[\x16\x1B\x80`\x01T\x16\x14`@Q\x90\x81R\xF3[4a\x04@W`\x806`\x03\x19\x01\x12a\x04@Wa\x143a\n\xBDV[``6`#\x19\x01\x12a\x04@W`@Qa\x14K\x81a\x03\xBCV[`$5a\x14W\x81a\x05\x9FV[\x81R`D5a\xFF\xFF\x81\x16\x81\x03a\x04@W` \x82\x01R`d5a\xFF\xFF\x81\x16\x81\x03a\x04@W`@\x82\x01Ra\x14\x87a//V[`\xFF`\x96T\x16`\xFF\x83\x16\x10\x15a\x14\xA0Wa\x05s\x91a4wV[cs\x10\xCF\xF5`\xE1\x1B_R`\x04_\xFD[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\x01T`@Q\x90\x81R\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x82\x10\x15a\x159W_R` _ \x01\x90_\x90V[a\x15\x10V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045`\x9CT\x81\x10\x15a\x04@W`\x9C_R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04@W`@6`\x03\x19\x01\x12a\x04@W`\x045a\x15\xF9\x81a\x04/V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x16\x18\x906\x90`\x04\x01a\x072V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x03a\x16\xEAW`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 B\x90U`\x99\x90\x91R\x90 \x80T`\x01a\x16\x80\x81a\x16wa\x16qa\x05Va\x16k`\x96T`\xFF\x16\x90V[\x89a4\x04V[\x94aAgV[\x94\x01T`\xFF\x16\x90V[a\x16\x89\x81a\x12\xF5V[\x14\x91\x82a\x16\xD7W[\x82a\x16\xBEW[PPa\x16\x9FW\0[a\x16\xA9\x81\x83a5\x89V[`\xA1T`\xFF\x16a\x16\xB5W\0[a\x05s\x91a8YV[\x81\x16`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x91\x16\x14\x90P_\x80a\x16\x97V[`\x01`\x01`\xC0\x1B\x03\x82\x16\x15\x15\x92Pa\x16\x91V[cv\xD8\xAB\x17`\xE1\x1B_R`\x04_\xFD[4a\x04@W_6`\x03\x19\x01\x12a\x04@Wa\x17\x11a//V[`d\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\xFF`\xA1T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04@W`\xA06`\x03\x19\x01\x12a\x04@Wa\x17\x906a\x0C\x81V[`d5a\x17\x9C\x81a\x0C\xD1V[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x17\xBB\x906\x90`\x04\x01a\x0C\xE2V[\x90a\x17\xC4a//V[`\x96T`\xFF\x16\x92\x83\x90a\x17\xEF\x90a\x17\xDD`\xC0\x84\x10a,^V[a\x17\xE9a\x0E\x02\x87a2\x14V[\x85a4wV[`\xA1T`\xFF\x81\x16\x80a\x19YW[a\x18bW[PPa\x18\x0C_a+{V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x04@Wa\x0E\x96\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c:\xEA\x0B\x9D`\xE1\x1B\x84R\x8A`\x04\x85\x01a3\xDCV[\x91a\x18o\x94\x91\x93\x94a2&V[\x92a\x18z\x86Qa2sV[\x93_[\x87Q\x81\x10\x15a\x18\xA9W\x80a\x18\xA3a\x18\x99a\x0F\xD4`\x01\x94\x8Ca#WV[a\x0F\xEC\x83\x8Aa#WV[\x01a\x18}V[P\x91\x93\x95\x94\x90\x92\x95a\x18\xBCa\x10\x13a\x03\xF8V[` \x82\x01Ra\x18\xCA\x82a#JV[Ra\x18\xD4\x81a#JV[P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92`\x10\x1C\x16\x90\x82;\x15a\x04@Wa\x190\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\x010\xFC'`\xE5\x1B\x84R`\x04\x84\x01a2\x9BV[\x03\x92Z\xF1\x80\x15a\x08yWa\x19EW[\x80a\x18\x01V[\x80a\x08m_a\x19S\x93a\x03\xD7V[_a\x19?V[Pa\x19ra\x10\xD5\x86`\xA2T`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a\x17\xFCV[\x81`\x1F\x82\x01\x12\x15a\x04@W\x805\x90a\x19\x8E\x82a\x04\x18V[\x92a\x19\x9C`@Q\x94\x85a\x03\xD7V[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x04@W` \x01\x92[\x82\x84\x10a\x19\xC6WPPPP\x90V[`@\x84\x83\x03\x12a\x04@W` `@\x91\x82Qa\x19\xE0\x81a\x03\x9CV[a\x19\xE9\x87a\n\xCDV[\x81R\x82\x87\x015a\x19\xF8\x81a\x04/V[\x83\x82\x01R\x81R\x01\x93\x01\x92a\x19\xB8V[4a\x04@W`\xA06`\x03\x19\x01\x12a\x04@W`\x045a\x1A$\x81a\x04/V[`$5\x90`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@W` \x92a\x1ANa\x1A[\x936\x90`\x04\x01a\x19wV[`d5\x91`\x845\x93a)\x8AV[`@Q\x90\x81R\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W` a\x1A\x81`\x045aAgV[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`dT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\xFF`\x96T\x16`@Q\x90\x81R\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x04@W\x815a\x1B6\x81a\x04\x18V[\x92a\x1BD`@Q\x94\x85a\x03\xD7V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04@W` \x01\x90[\x82\x82\x10a\x1BlWPPP\x90V[` \x80\x91\x835a\x1B{\x81a\x05\x9FV[\x81R\x01\x91\x01\x90a\x1B_V[4a\x04@W`@6`\x03\x19\x01\x12a\x04@W`\x045a\x1B\xA3\x81a\x04/V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@Wa\x1B\xF5a\x1B\xC9a\x05s\x936\x90`\x04\x01a\x1B\x1FV[a\x1B\xD1a9~V[a\x1B\xE2`\x02\x80`\x01T\x16\x14\x15a#4V[a\x1B\xF0`\xFF`\xA1T\x16a$\xE4V[a9\xBFV[\x90a5\x89V[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `@Q\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81R\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W` a\x1C\xA8a\x1C\x97a\n\xBDV[`\xA2T`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\xA0T`@Q\x90\x81R\xF3[4a\x04@W``6`\x03\x19\x01\x12a\x04@W`\x045a\x1C\xEC\x81a\x04/V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x1D\x0B\x906\x90`\x04\x01a\x1B\x1FV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x1Dya\x1D1a\x1D`\x926\x90`\x04\x01a\x10\xDEV[\x92\x90\x94a\x1D<a9~V[a\x1DLa\x04\xEC`\x01\x80\x80T\x16\x14\x90V[a\x1B\xF0a\x1D[`\xA1T`\xFF\x16\x90V[a$\xE4V[\x91\x84\x01a\x1Dm\x81\x86a+8V[\x86\x97\x92\x97\x94\x91\x94a:\xB9V[\x95a\x1D\x83\x81a+{V[\x80a\x1E\xBDWPPPa\x1D\x97\x90\x82\x85\x85a>\xB2V[Q\x92_[\x82Q\x81\x10\x15a\x1E W\x80a\x1E\x1A\x86c\xFF\xFF\xFF\xFFa\x1E\x11a\x06ca\x1E\x07a\x1D\xF5a\x1D\xEB\x88a\x1D\xE5a\x1D\xDFa\x1D\xD1\x8F\x9D`\x01\x9Ea,MV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[\x97a#WV[Qc\xFF\xFF\xFF\xFF\x16\x90V[\x94`\xFF\x16_R`\x97` R`@_ \x90V[Tc\xFF\xFF\xFF\xFF\x16\x90V[\x91\x16\x11\x15a,^V[\x01a\x1D\x9BV[P\x92PP[`\x01a\x1EM\x81a\x1EE\x84`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[\x01T`\xFF\x16\x90V[a\x1EV\x81a\x12\xF5V[\x03a\x1E]W\0[a\x1E\x8Ea\x1Eha\x03\xF8V[\x83\x81R`\x01` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x99` R`@\x90 a,tV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE_\x80\xA3\0[\x80a\x1E\xCC`\x01\x92\x96\x94\x96a+{V[\x03a\x1E\xF1Wa\x1E\xEC\x93a\x1E\xDE\x91a+\x85V[\x95\x93P\x93\x91PP\x86\x86a;\xBBV[a\x1E%V[c5K\xB8\xAB`\xE0\x1B_R`\x04_\xFD[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\xFF`\xA1T`\x08\x1C\x16`@Q\x90\x15\x15\x81R\xF3[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x1FHWPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1F;V[4a\x04@W`@6`\x03\x19\x01\x12a\x04@W`\x045a\x1F\x81\x81a\x05\x9FV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@W6`#\x83\x01\x12\x15a\x04@W\x81`\x04\x015\x91a\x1F\xAD\x83a\x04\x18V[\x92a\x1F\xBB`@Q\x94\x85a\x03\xD7V[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x04@W`$\x01\x90[\x82\x82\x10a\x1F\xF8Wa\x06Qa\x1F\xEC\x86\x86aA\x08V[`@Q\x91\x82\x91\x82a\x1F%V[\x815\x81R` \x91\x82\x01\x91\x01a\x1F\xD8V[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `@Q\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81R\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\x9CT`@Q\x90\x81R\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\xFFa \xBEa\n\xBDV[a \xC6a$\x02V[P\x16_R`\x97` Ra\x06Q`@_ a\xFF\xFF`@Q\x91a \xE6\x83a\x03\xBCV[Tc\xFF\xFF\xFF\xFF\x81\x16\x83R\x81\x81` \x1C\x16` \x84\x01R`0\x1C\x16`@\x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91`@a\xFF\xFF\x81``\x84\x01\x95c\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a!\x98\x81a\x04/V[a!\xA0a//V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a!\xB8Wa\x05s\x90a4/V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x08yW_\x91a\"\xCFW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\"\xC0Wa\"\x8E`\x01T\x19\x82\x19\x81\x16\x14a#\xECV[\x80`\x01U`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a\"\xE8\x91P` =` \x11a\x0B\xE1Wa\x0B\xD3\x81\x83a\x03\xD7V[_a\"mV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a#\x0B\x81a\x04/V[`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R` `\xFF`\x01`@_ \x01T\x16a\x0C\x7F`@Q\x80\x92a\x12\xFFV[\x15a#;WV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x80Q\x15a\x159W` \x01\x90V[\x80Q\x82\x10\x15a\x159W` \x91`\x05\x1B\x01\x01\x90V[`\x03\x82\x10\x15a\x08~WRV[\x90a\x04\x07`@Qa#\x87\x81a\x03\x9CV[` `\xFF`\x01\x83\x96\x80T\x85R\x01T\x16\x91\x01a#kV[\x15a#\xA4WV[c\xAB\xA4s9`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x04@WQ\x80\x15\x15\x81\x03a\x04@W\x90V[`@Q=_\x82>=\x90\xFD[\x15a#\xDDWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a#\xF3WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[`@Q\x90a$\x0F\x82a\x03\xBCV[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x90`@Qa$-\x81a\x03\xBCV[`@\x81\x93Tc\xFF\xFF\xFF\xFF\x81\x16\x83Rc\xFF\xFF\xFF\xFF\x81` \x1C\x16` \x84\x01R\x81\x1C\x91\x01RV[\x90\x81` \x91\x03\x12a\x04@WQa\x07M\x81a\x04/V[`@Q\x90a$s\x82a\x03\x9CV[_` \x83\x82\x81R\x01RV[a$\xDFa\x07M\x91a$\x8Da$fV[P`@\x80Q\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6` \x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x81\x83\x01R\x90\x81Ra$\xD7``\x82a\x03\xD7V[Q\x90 a0\xA9V[a0\xF6V[\x15a$\xEBWV[c[w\x90\x19`\xE0\x1B_R`\x04_\xFD[\x90\x92\x91a%\x0Ea\x04\xEC`\x04\x80`\x01T\x16\x14\x90V[a%-a%\x1D`\x96T`\xFF\x16\x90V[a%(6\x84\x88a\x06\xFCV[a4\x04V[Pa%:\x81\x83Q\x14a'=V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x93_[\x82\x81\x10a%vWPPPP\x90PV[a%\x95a\x1D\xDFa%\x87\x83\x86\x86a'SV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[\x92a%\xA0\x82\x86a#WV[Q\x80Q`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x87\x16`\x04\x82\x01R\x91\x97\x91\x90` \x82`$\x81\x8DZ\xFA\x91\x82\x15a\x08yWa%\xE6\x92c\xFF\xFF\xFF\xFF\x91_\x91a'\x0FW[P\x16\x14a'tV[_\x97\x88[\x88Q\x8A\x10\x15a&\xA3W`\x01\x90a&\x9Ba&\x13a&\x06\x8D\x8Da#WV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91a&va&9a&4\x85`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[a#wV[\x91a&aa&\\\x8Da&Na\x05V\x87QaAgV[`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a'\x8AV[\x85\x80`\xA0\x1B\x03\x16\x85\x80`\xA0\x1B\x03\x85\x16\x11a'\xA0V[a&\x94a&\x8Da&\x85\x8Aa'\xCAV[\x8A\x8A\x8Da'\xEAV[6\x91a\x06\xFCV[\x90\x83a.2V[\x99\x01\x98a%\xEAV[P\x96P\x96P\x92\x90`\x01\x91\x94\x92\x94Ca&\xC6\x82`\xFF\x16_R`\x9B` R`@_ \x90V[U\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4`\xFF`@Q\x92\x16\x91\x80a'\0C\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA2\x01\x94\x93\x94\x92\x90\x92a%gV[a'0\x91P` =\x81\x11a'6W[a'(\x81\x83a\x03\xD7V[\x81\x01\x90a'_V[_a%\xDEV[P=a'\x1EV[\x15a'DWV[c\xAA\xAD\x13\xF7`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a\x159W\x01\x90V[\x90\x81` \x91\x03\x12a\x04@WQa\x07M\x81a\x05\x9FV[\x15a'{WV[c\x8EZ\xEE\xE7`\xE0\x1B_R`\x04_\xFD[\x15a'\x91WV[c\xD0S\xAA!`\xE0\x1B_R`\x04_\xFD[\x15a'\xA7WV[c\xBAP\xF9\x11`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a'\xD8WV[a'\xB6V[\x91\x90\x82\x01\x80\x92\x11a'\xD8WV[\x90\x93\x92\x93\x84\x83\x11a\x04@W\x84\x11a\x04@W\x81\x01\x92\x03\x90V[\x15a(\tWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`\x9CT`\x01`@\x1B\x81\x10\x15a\x03\xB7W`\x01\x81\x01`\x9CU`\x9CT\x81\x10\x15a\x159W`\x9C_R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a(\xE3\x94\x93a\t\x9Fa\t\xFB\x94\x93a\x0C\x10a\x0C=\x94a4/V[a)\x15\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a(eV[a)G\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a(eV[a)y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a(eV[a\x01\x01a\xFF\xFF\x19`\xA1T\x16\x17`\xA1UV[\x91\x94\x93\x90\x92`@Q\x92` \x84\x01\x94`\xE0\x85\x01\x91\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87R`\x01\x80`\xA0\x1B\x03\x16`@\x86\x01R``\x85\x01R`\xC0`\x80\x85\x01R\x86Q\x80\x91R` a\x01\0\x85\x01\x97\x01\x90_[\x81\x81\x10a*\x17WPPPa\x07M\x94\x95a$\xD7\x92\x84\x92`\xA0\x84\x01R`\xC0\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xD7V[\x82Q\x80Q`\xFF\x16\x8AR` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x8B\x01R`@\x90\x99\x01\x98\x90\x92\x01\x91`\x01\x01a)\xEAV[5\x90`\x02\x82\x10\x15a\x04@WV[\x91\x90\x82`@\x91\x03\x12a\x04@W`@Qa*j\x81a\x03\x9CV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x04@W`@Q\x91a*\x95`@\x84a\x03\xD7V[\x82\x90`@\x81\x01\x92\x83\x11a\x04@W\x90[\x82\x82\x10a*\xB1WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a*\xA4V[\x80\x82\x03\x92\x91a\x01\0\x84\x12a\x04@W`@Qa*\xDB\x81a\x03\xBCV[`\x80\x81\x95a*\xE9\x84\x86a*RV[\x83Ra*\xF8\x84`@\x87\x01a*RV[` \x84\x01R`\x7F\x19\x01\x12a\x04@Wa+/`@\x92`\xC0\x84Q\x95a+\x1A\x87a\x03\x9CV[a+'\x83`\x80\x83\x01a*zV[\x87R\x01a*zV[` \x84\x01R\x01RV[\x90\x91a\x01@\x82\x84\x03\x12a\x04@Wa+N\x82a*EV[\x92` \x83\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@W`@a+t\x82a\x07M\x94\x87\x01a\x072V[\x94\x01a*\xC1V[`\x02\x11\x15a\x08~WV[\x91\x90a\x01\x80\x83\x82\x03\x12a\x04@Wa+\x9B\x83a*EV[\x92` \x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x04@W\x82a+\xBB\x91\x83\x01a\x072V[\x92a+\xC9\x83`@\x84\x01a*\xC1V[\x92a\x01@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11a\x04@W\x81a+\xEA\x91\x85\x01a\x19wV[\x92a\x01`\x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@W\x01\x90``\x82\x82\x03\x12a\x04@W`@Q\x91a,\x19\x83a\x03\xBCV[\x805`\x01`\x01`@\x1B\x03\x81\x11a\x04@W`@\x92a,7\x91\x83\x01a\x072V[\x83R` \x81\x015` \x84\x01R\x015`@\x82\x01R\x90V[\x90\x81Q\x81\x10\x15a\x159W\x01` \x01\x90V[\x15a,eWV[c<\xB8\x9C\x97`\xE0\x1B_R`\x04_\xFD[`\x01` \x91\x83Q\x81U\x01\x91\x01Q`\x03\x81\x10\x15a\x08~W`\xFF\x80\x19\x83T\x16\x91\x16\x17\x90UV[\x90`@Qa,\xA5\x81a\x03\xBCV[`@a\xFF\xFF\x82\x94Tc\xFF\xFF\xFF\xFF\x81\x16\x84R\x81\x81` \x1C\x16` \x85\x01R`0\x1C\x16\x91\x01RV[`@\x80Q\x90\x91\x90a,\xDB\x83\x82a\x03\xD7V[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a,\xF6\x82a\x06\xE1V[a-\x03`@Q\x91\x82a\x03\xD7V[\x82\x81R\x80\x92a-\x14`\x1F\x19\x91a\x06\xE1V[\x01\x90` 6\x91\x017V[_\x19\x81\x14a'\xD8W`\x01\x01\x90V[_\x81\x80[a-\xA6WPa-B\x90a\xFF\xFF\x16a,\xECV[__[\x82Q\x82\x10\x80a-\x9BW[\x15a-\x94W`\x01\x81\x1B\x84\x16a-mW[a-h\x90a-\x1EV[a-EV[\x90`\x01a-h\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa-\x8A\x82\x87a,MV[S\x01\x91\x90Pa-_V[PP\x90P\x90V[Pa\x01\0\x81\x10a-OV[_\x19\x81\x01\x81\x81\x11a'\xD8Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a'\xD8W`\x01\x01\x90\x80a-0V[\x90\x81` \x91\x03\x12a\x04@WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x04@W\x90V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[a\x07M\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a-\xEAV[\x91\x90`\x01` \x82\x01Qa.D\x81a\x12\xF5V[a.M\x81a\x12\xF5V[\x03a/\x03WQ`@Qc3V\x7F\x7F`\xE1\x1B\x81R\x91` \x91\x83\x91\x82\x91a.w\x91\x90\x87`\x04\x85\x01a.\x0EV[\x03\x81_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x90\x81\x15a\x08yW_\x91a.\xD4W[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81a.\xC8WPPV[a\x1B\xF5a\x04\x07\x92a-,V[a.\xF6\x91P` =` \x11a.\xFCW[a.\xEE\x81\x83a\x03\xD7V[\x81\x01\x90a-\xCBV[_a.\xB4V[P=a.\xE4V[PPPV[\x15a/\x0FWV[c\xBB\xBA`\xCB`\xE0\x1B_R`\x04_\xFD[\x90` a\x07M\x92\x81\x81R\x01\x90a-\xEAV[`dT`\x01`\x01`\xA0\x1B\x03\x163\x03a/CWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[_\x19`\x01U`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`\x01U`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`\x9DUV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`\x9EUV[a0\xB1aA\xABV[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra0\xDC`b\x82a\x03\xD7V[Q\x90 \x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_Q` aJ\xEF_9_Q\x90_R\x90a1\ra$fV[P_\x91\x90\x06` `\xC0\x83[a2\rW_\x93_Q` aJ\xEF_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa1C\x85\x82a\x03\xD7V[\x846\x827\x84\x81\x85`@Qa1W\x82\x82a\x03\xD7V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aJ\xEF_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a2\x12Wa1\xC1\x90aG\xAAV[Q\x91a2\rW_Q` aJ\xEF_9_Q\x90_R\x82\x80\t\x14a1\xF8WP_Q` aJ\xEF_9_Q\x90_R`\x01_\x94\x08\x92\x93a1\x18V[\x92\x93PPa2\x04a\x03\xF8V[\x92\x83R\x82\x01R\x90V[a0\xE2V[\xFE[`\xFF`\x01\x91\x16\x01\x90`\xFF\x82\x11a'\xD8WV[`@\x80Q\x90\x91\x90a27\x83\x82a\x03\xD7V[`\x01\x81R\x91`\x1F\x19\x01\x82_[\x82\x81\x10a2OWPPPV[` \x90`@Qa2^\x81a\x03\x9CV[_\x81R``\x83\x82\x01R\x82\x82\x85\x01\x01R\x01a2CV[\x90a2}\x82a\x04\x18V[a2\x8A`@Q\x91\x82a\x03\xD7V[\x82\x81R\x80\x92a-\x14`\x1F\x19\x91a\x04\x18V[\x90`@\x82\x01\x90`\x01\x80`\xA0\x1B\x03\x16\x82R`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x91` ``\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a2\xDCWPPPPP\x90V[\x90\x91\x92\x93\x94`_\x19\x82\x82\x03\x01\x83R\x85Q` ``\x81`@\x85\x01\x93c\xFF\xFF\xFF\xFF\x81Q\x16\x86R\x01Q\x93`@\x83\x82\x01R\x84Q\x80\x94R\x01\x92\x01\x90_\x90[\x80\x82\x10a34WPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a2\xCDV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x90\x91\x01\x90a3\x15V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a3sWPPP\x90V[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x86\x01R`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a3fV[\x90a\x07M\x94\x93`\x01`\x01``\x1B\x03`\x80\x94`\xFFc\xFF\xFF\xFF\xFF\x94\x16\x85R\x16` \x84\x01R\x16`@\x82\x01R\x81``\x82\x01R\x01\x90a3VV[`\x01`\x01``\x1B\x03a\x07M\x94\x93`\xFF``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a3VV[\x90`\x01a4\x12`\xFF\x93aB\xD7V[\x92\x83\x92\x16\x1B\x11\x15a4 W\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[`d\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a55`\xFF\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x92\x16\x92\x83_R`\x97` R`@_ a4\xCDc\xFF\xFF\xFF\xFF\x83Q\x16\x82\x90c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[` \x82\x01Q\x81Te\xFF\xFF\0\0\0\0g\xFF\xFF\0\0\0\0\0\0`@\x86\x01Q`0\x1B\x16\x92` \x1B\x16\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x17\x17\x90U`@Q\x91\x82\x91\x82\x91\x90\x91`@a\xFF\xFF\x81``\x84\x01\x95c\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[\x03\x90\xA2V[\x15a5AWV[ch\xB6\xA8u`\xE1\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra\x07M\x92\x91\x01\x90a-\xEAV[`@\x90a\x07M\x93\x92\x81R\x81` \x82\x01R\x01\x90a-\xEAV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x99` R`@\x90 \x90a6\x14\x82T\x92a5\xC8`\x01a5\xB9\x81a\x16w\x88aAgV[a5\xC2\x81a\x12\xF5V[\x14a#\x9DV[a5\xE3a\x05Va\x05Va5\xDD`\x96T`\xFF\x16\x90V[\x88a4\x04V[\x90a5\xEF\x82\x15\x15a5:V[a6\x05\x82\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x83\x14a'\x8AV[\x90\x19\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[a6\x1E\x81\x84aCOV[`\x01`\x01`\xC0\x1B\x03\x16\x15a7\x90W[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@W\x83_\x91a6\x82\x93\x83`@Q\x80\x96\x81\x95\x82\x94c\xF4\xE2O\xE5`\xE0\x1B\x84R`\x04\x84\x01a5PV[\x03\x92Z\xF1\x80\x15a\x08yWa7|W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@W_`@Q\x80\x92c\xBD)\xB8\xCD`\xE0\x1B\x82R\x81\x83\x81a6\xE4\x89\x89`\x04\x84\x01a5rV[\x03\x92Z\xF1\x80\x15a\x08yWa7hW[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x04@Wa7I\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xBD)\xB8\xCD`\xE0\x1B\x84R`\x04\x84\x01a5rV[\x03\x92Z\xF1\x80\x15a\x08yWa7ZWPV[\x80a\x08m_a\x04\x07\x93a\x03\xD7V[\x80a\x08m_a7v\x93a\x03\xD7V[_a6\xF3V[\x80a\x08m_a7\x8A\x93a\x03\xD7V[_a6\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x99` R`@\x90 a7\xBB\x90`\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UV[\x81`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4_\x80\xA3a6-V[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x84\x01R\x81\x84\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x90\x92\x01Q``\x80\x83\x01R\x80Q`\x80\x83\x01\x81\x90R`\xA0\x90\x92\x01\x92\x01\x90_[\x81\x81\x10a8=WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a80V[\x90\x91a8e\x83Qa2sV[`\xA2T_\x92\x83[\x86Q\x81\x10\x15a8\xD7Wa8\x85a\x1D\xDFa\x1D\xD1\x83\x8Aa,MV[a8\x98\x81\x85`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a8\xA6W[P`\x01\x01a8lV[\x94\x90a8\xD1\x82a8\xC6`\xFFa8\xBD`\x01\x96\x93a-\x1EV[\x99\x16\x91\x88a#WV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x90a8\x9DV[P\x94PP\x90\x80a8\xE6WPPPV[\x81R`\xA1T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92\x91a9M\x91`\x10\x91\x90\x91\x1C\x16a9=a9.a\x04\tV[`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x86RV[`\x01`\x01`\xA0\x1B\x03\x16` \x85\x01RV[`@\x83\x01R\x80;\x15a\x04@W`@Qcn4\x92\xB5`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a7I\x90`\x04\x83\x01a7\xEFV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a9\xB0WV[c#\xD8q\xA5`\xE0\x1B_R`\x04_\xFD[\x90a9\xCA\x82Qa,\xECV[_[\x83Q\x81\x10\x15a:\x03W`\x01\x90`\x01`\x01`\xF8\x1B\x03\x19a9\xEB\x82\x87a#WV[Q`\xF8\x1B\x16_\x1Aa9\xFC\x82\x85a,MV[S\x01a9\xCCV[P\x91PV[\x90\x81` \x91\x03\x12a\x04@WQ\x90V[\x90_\x90[`\x02\x82\x10a:(WPPPV[` \x80`\x01\x92\x85Q\x81R\x01\x93\x01\x91\x01\x90\x91a:\x1BV[a\x01 \x90a:\xA8` `@a\x04\x07\x96\x98\x97\x95\x98a\x01`\x85\x01\x99`\x01\x80`\xA0\x1B\x03\x16\x85Ra:x\x83\x86\x01\x82Q` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\x80\x83\x01Q\x80Q``\x87\x01R` \x01Q`\x80\x86\x01R\x01Qa:\x9C`\xA0\x85\x01\x82Qa:\x17V[\x01Q`\xE0\x83\x01\x90a:\x17V[\x01\x90` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x90\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x83`$\x81\x84Z\xFA\x92\x83\x15a\x08yW_\x93a;\x9AW[P\x82\x15a; WPP\x90P\x90V[` \x92Pa;P\x93_a;2\x84a$~V[`@Qc\x17\xEF9\xCB`\xE3\x1B\x81R\x96\x87\x95\x86\x94\x85\x93\x91`\x04\x85\x01a:>V[\x03\x92Z\xF1\x90\x81\x15a\x08yW_\x91a;kW[P\x80_\x80a-\x94V[a;\x8D\x91P` =` \x11a;\x93W[a;\x85\x81\x83a\x03\xD7V[\x81\x01\x90a:\x08V[_a;bV[P=a;{V[a;\xB4\x91\x93P` =` \x11a;\x93Wa;\x85\x81\x83a\x03\xD7V[\x91_a;\x12V[\x92\x90\x91a;\xE3\x92a;\xDD\x82\x98\x97a;\xD5\x88Q\x85Q\x14a'=V[\x87\x83\x88aDiV[\x84a>\xB2V[\x90_[\x85Q\x81\x10\x15a=\x18W\x80a<\x1Ea<\x19a<\x08a\x1D\xDFa\x1D\xD1`\x01\x96\x8Ca,MV[`\xFF\x16_R`\x97` R`@_ \x90V[a,\x98V[a<,a\x1D\xEB\x83\x87Qa#WV[c\xFF\xFF\xFF\xFFa<Ba\x06c\x84Qc\xFF\xFF\xFF\xFF\x16\x90V[\x91\x16\x11a<QW[P\x01a;\xE6V[a<\xA7\x90a<ea\x1D\xDFa\x1D\xD1\x85\x8Ca,MV[a<\x83a<v\x85`@\x8A\x01Qa#WV[Q`\x01`\x01``\x1B\x03\x16\x90V[\x86a<\x95a<v\x87` \x8C\x01Qa#WV[\x91a<\xA0\x87\x8Ca#WV[Q\x93aEJV[a<\xAFa,\xCAV[a<\xBCa\x1D\xD1\x83\x8Aa,MV[_\x1Aa<\xC7\x82a#JV[Sa<\xEF\x81a<\xEA` a<\xDB\x86\x8Ba#WV[Q\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a5\x89V[`\xA1T`\xFF\x16\x15a<JWa=\x12\x90a=\r` a<\xDB\x85\x8Aa#WV[a8YV[_a<JV[PPPP\x90PV[`@Q\x90a=-\x82a\x03\xBCV[```@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x15a=FWV[c\x13\xCAFW`\xE0\x1B_R`\x04_\xFD[\x15a=\\WV[c\x0Ch\x16\xCD`\xE0\x1B_R`\x04_\xFD[\x15a=rWV[c\x19hg}`\xE1\x1B_R`\x04_\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x04@W\x81Qa=\x98\x81a\x04\x18V[\x92a=\xA6`@Q\x94\x85a\x03\xD7V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04@W` \x01\x90[\x82\x82\x10a=\xCEWPPP\x90V[` \x80\x91\x83Qa=\xDD\x81a\x0C\xD1V[\x81R\x01\x91\x01\x90a=\xC1V[\x91\x90\x91`@\x81\x84\x03\x12a\x04@W\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x04@W\x83a>\x11\x91\x83\x01a=\x81V[\x92` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x07M\x92\x01a=\x81V[` \x81\x83\x03\x12a\x04@W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x04@W\x81Qa>b\x81a\x04\x18V[\x92a>p`@Q\x94\x85a\x03\xD7V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04@W` \x01\x90[\x82\x82\x10a>\x98WPPP\x90V[` \x80\x91\x83Qa>\xA7\x81a\x05\x9FV[\x81R\x01\x91\x01\x90a>\x8BV[\x81\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAAa?xa>\xDFa= V[\x96a?la>\xFBa\x05Va>\xF5`\x96T`\xFF\x16\x90V[\x8Aa4\x04V[a?\x04\x86aAgV[`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x90a?\x1B\x82\x15\x15a=?V[`\x01\x80`\xC0\x1B\x03\x16a?5a?0\x82\x84\x16\x15\x90V[a=UV[`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9F` R`@\x90 a?e\x90a?^\x90T`\xA0T\x90a'\xDDV[B\x11a=kV[\x17\x85aCOV[`@Q\x91\x82\x91\x82a/\x1EV[\x03\x90\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@W_`@Q\x80\x92c\x1F\xD9<\xA9`\xE1\x1B\x82R\x81\x83\x81a?\xCD\x8A\x89`\x04\x84\x01a5PV[\x03\x92Z\xF1\x80\x15a\x08yW\x84\x92_\x92\x85\x92a@\xF4W[Pa@\x01`@Q\x96\x87\x93\x84\x93c%PGw`\xE0\x1B\x85R`\x04\x85\x01a.\x0EV[\x03\x81\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x91\x82\x15a\x08yWa@k\x93_\x93\x84\x91\x85\x91a@\xD0W[P`@\x87\x01R` \x86\x01R`@Q\x93\x84\x92\x83\x92b\xBF\xF0M`\xE0\x1B\x84R`\x04\x84\x01a5rV[\x03\x81\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x90\x81\x15a\x08yW_\x91a@\xAEW[P\x81R\x90V[a@\xCA\x91P=\x80_\x83>a@\xC2\x81\x83a\x03\xD7V[\x81\x01\x90a>/V[_a@\xA8V[\x90Pa@\xEE\x91P=\x80\x86\x83>a@\xE6\x81\x83a\x03\xD7V[\x81\x01\x90a=\xE8V[_a@FV[\x80a\x08m\x85aA\x02\x93a\x03\xD7V[_a?\xE2V[\x91\x90aA\x14\x81Qa2sV[\x90_[\x81Q\x81\x10\x15aARW\x80aA9aA0`\x01\x93\x85a#WV[Q\x87`\x98aF\xA0V[c\xFF\xFF\xFF\xFFaAH\x83\x87a#WV[\x91\x16\x90R\x01aA\x17V[P\x90\x92PPV[_\x19\x81\x01\x91\x90\x82\x11a'\xD8WV[\x80_R`\x98` R`@_ T\x90\x81\x15_\x14aA\x83WPP_\x90V[_R`\x98` R`@_ \x90_\x19\x81\x01\x90\x81\x11a'\xD8WaA\xA3\x91a\x15$V[PT`@\x1C\x90V[0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80aB\x98W[\x15aB\x06W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra0\xDC`\xC0\x82a\x03\xD7V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14aA\xDDV[\x15aB\xC8WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aC@W\x81Q\x15aC;W` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aC6W`\x01\x90aC!aC\x17a\x1D\xDFa\x1D\xD1\x86\x89a,MV[`\xFF`\x01\x91\x16\x1B\x90V[\x90aC-\x81\x83\x11aB\xC1V[\x17\x91\x01\x90aB\xF8V[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x90aCb\x82_R`\x98` R`@_ \x90V[T\x80aC\xAFWPaC~a\x04\x07\x92_R`\x98` R`@_ \x90V[aC\xAAaC\x89a\x04\tV[Cc\xFF\xFF\xFF\xFF\x16\x81R\x92[_` \x85\x01R`\x01`\x01`\xC0\x1B\x03\x16`@\x84\x01RV[aG\xC0V[\x91aC\xDAc\xFF\xFF\xFF\xFF\x93aC\xD4aC\xCE\x84_R`\x98` R`@_ \x90V[\x91aAYV[\x90a\x15$V[P\x90aC\xEA\x82Tc\xFF\xFF\xFF\xFF\x16\x90V[C\x85\x16\x94\x16\x84\x03aD\x15WPa\x04\x07\x92P\x90`\x01`\x01`@\x1B\x03\x82T\x91\x81\x19\x90`@\x1B\x16\x91\x16\x17\x90UV[\x81Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x85\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x91Ua\x04\x07\x92\x91\x90aC\xAA\x90aDQ\x90_R`\x98` R`@_ \x90V[\x91aC\x94aD]a\x04\tV[c\xFF\xFF\xFF\xFF\x90\x95\x16\x85RV[\x91\x92\x90` \x82\x01\x92\x83Q_R`\x9A` R`\xFF`@_ T\x16aD\xE4W`@\x83\x01\x80QB\x11aD\xD5Wa\x04\x07\x95aD\xCD\x93\x86Q_R`\x9A` RaD\xB7`@_ `\x01`\xFF\x19\x82T\x16\x17\x90UV[`\x9DT`\x01`\x01`\xA0\x1B\x03\x16\x96Q\x92Q\x93a)\x8AV[\x90Q\x91aH.V[c\x08\x19\xBD\xCD`\xE0\x1B_R`\x04_\xFD[co\xBE\xFE\xC3`\xE1\x1B_R`\x04_\xFD[\x15aD\xFAWV[cV\x16\x8BA`\xE1\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x04@WQa\x07M\x81a\x0C\xD1V[\x15aE%WV[cLD\x99]`\xE0\x1B_R`\x04_\xFD[\x15aE;WV[c\xB1\x87\xE8i`\xE0\x1B_R`\x04_\xFD[` \x91\x92aE\xA7aE\x9AaE\xD2\x98\x96\x97aE\x93aEo\x87\x83\x01Q`\x01\x80`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x99` R`@\x90 T\x96\x90\x91\x16\x14\x15aD\xF3V[Q`\xFF\x16\x90V[`\xFF\x80\x85\x16\x91\x16\x14a'tV[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x90\x91\x16`$\x82\x01R\x94\x85\x90\x81\x90`D\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\x08yWa\x04\x07\x94_\x94aFVW[P\x82aFN\x92aFIaF5\x93`\x01`\x01``\x1B\x03aFAaF5\x82\x99\x8BaHrV[`\x01`\x01``\x1B\x03\x16\x90V[\x91\x16\x11aE\x1EV[aH\x95V[\x91\x16\x10aE4V[aF5\x91\x94P\x92aFN\x92aFIaF\x8D`\x01`\x01``\x1B\x03\x96` =` \x11aF\x99W[aF\x85\x81\x83a\x03\xD7V[\x81\x01\x90aE\tV[\x96\x93PP\x92P\x92aF\x12V[P=aF{V[\x91\x90\x81_R\x82` R`@_ T\x92_[\x84\x81\x10aGIW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[\x80\x85\x03\x85\x81\x11a'\xD8Wa\x06caG_\x91aAYV[aG\x88aG}\x82aGx\x88\x87\x90_R` R`@_ \x90V[a\x15$V[PTc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x11\x15aG\xA1WP`\x01\x01aF\xB1V[\x94PPPPP\x90V[\x15aG\xB1WV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x03\xB7WaG\xDD\x91`\x01\x82\x01\x81Ua\x15$V[aH\x1BW\x81Q` \x80\x84\x01Q`@\x94\x85\x01Qc\xFF\xFF\xFF\xFF\x90\x93\x16\x91\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x90\x93\x1B\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x90aH9\x92\x91aI\x0CV[\x15aH@WV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a'\xD8WV[aH\x90`\x01`\x01``\x1B\x03\x91a\xFF\xFF` a'\x10\x95\x01Q\x16\x90aHOV[\x16\x04\x90V[aH\x90`\x01`\x01``\x1B\x03\x91a\xFF\xFF`@a'\x10\x95\x01Q\x16\x90aHOV[`\x05\x11\x15a\x08~WV[=\x15aH\xE7W=\x90aH\xCE\x82a\x06\xE1V[\x91aH\xDC`@Q\x93\x84a\x03\xD7V[\x82R=_` \x84\x01>V[``\x90V[\x90\x81` \x91\x03\x12a\x04@WQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x04@W\x90V[\x91\x90\x91aI\x19\x82\x84aI\xD7V[aI\"\x81aH\xB3V[\x15\x90\x81aI\xC1W[PaI\xB9W_\x92aIWaIe\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01a5rV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xD7V[Q\x91Z\xFAaIqaH\xBDV[\x81aI\xADW[\x81aI\x80WP\x90V[\x80Qc\x0B\x13]?`\xE1\x1B\x92P`\x01`\x01`\xE0\x1B\x03\x19\x91aI\xA8\x91\x81\x01` \x90\x81\x01\x91\x01aH\xECV[\x16\x14\x90V[\x80Q` \x14\x91PaIwV[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_aI*V[\x81Q`A\x81\x03aJ\x03WP\x90aI\xFF\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aJEV[\x90\x91V[`@\x03aJ<W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a'\xD8WaI\xFF\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90aJEV[PP_\x90`\x02\x90V[\x92\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11aJ\xE3W`\xFF\x16`\x1B\x81\x14\x15\x80aJ\xD8W[aJ\xCDW` \x93_\x93`@Q\x93\x84\x93`\x80\x85\x01\x93\x85R\x87\x85\x01R`@\x84\x01R``\x83\x01R\x83\x80R\x03\x90`\x01Z\xFA\x15a\x08yW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aJ\xC5W\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x81\x14\x15aJ}V[PPPP_\x90`\x03\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xA0\xF5/\xEBgb\xDFJ\x10j\n\xFC\xB3\xC8\xBD\xDD\xC2\x82\xCE\xA4\x88]\xA5q}\xD9\x83\x81(P$IdsolcC\0\x08\x1B\x003\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8062cf2ab51461038357806303fd34921461037e57806304ec635114610379578063054310e6146103745780630764cb931461036f5780630cf4b7671461036a5780630d3f213414610365578063125e05841461036057806313542a4e1461035b578063136439dd14610356578063143e5915146103515780631478851f1461034c5780631eb812da14610347578063249a0c421461034257806328f61b311461033d578063296bb0641461033857806329d1e0c3146103335780632cdd1e861461032e5780633c2a7f4c146103295780633eef3a51146103245780635140a5481461031f578063530b97a41461031a5780635865c60c14610315578063595c6a67146103105780635ac86ab71461030b5780635b0b829f146103065780635c975abb146103015780635df45946146102fc5780636347c900146102f757806368304835146102f25780636e3b17db146102ed578063715018a6146102e857806381f936d2146102e35780638281ab75146102de57806384ca5213146102d9578063871ef049146102d4578063886f1195146102cf5780638da5cb5b146102ca5780639aa1653d146102c55780639d8e0c23146102c05780639e9923c2146102bb5780639feab859146102b6578063a4d7871f146102b1578063a96f783e146102ac578063adcf73f7146102a7578063b2d8678d146102a2578063c391425e1461029d578063ca0de88214610298578063ca8aa7c714610293578063d72d8dd61461028e578063e65797ad14610289578063ea32afae14610284578063f2fde38b1461027f578063fabc1cbc1461027a5763fd39105a14610275575f80fd5b6122ee565b61220c565b61217b565b612137565b6120a3565b612086565b612042565b612008565b611f64565b611f00565b611ccf565b611cb2565b611c79565b611c3f565b611bfb565b611b86565b611aff565b611ad7565b611a93565b611a63565b611a07565b611776565b611754565b6116f9565b6115dc565b611598565b61153e565b6114cc565b6114af565b61141a565b6113eb565b611378565b61130c565b6111d1565b61110b565b610d74565b610c42565b610c15565b610be8565b610b35565b610b0d565b610adb565b610a53565b610a24565b6109d3565b61091a565b6108df565b6108a4565b610883565b610750565b6106b5565b61068d565b6105ad565b610575565b6104ab565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176103b757604052565b610388565b606081019081106001600160401b038211176103b757604052565b90601f801991011681019081106001600160401b038211176103b757604052565b604051906104076040836103d7565b565b604051906104076060836103d7565b6001600160401b0381116103b75760051b60200190565b6001600160a01b0381160361044057565b5f80fd5b9080601f8301121561044057813561045b81610418565b9261046960405194856103d7565b81845260208085019260051b82010192831161044057602001905b8282106104915750505090565b6020809183356104a08161042f565b815201910190610484565b34610440576020366003190112610440576004356001600160401b038111610440576104db903690600401610444565b6104f26104ec600480600154161490565b15612334565b5f5b81518110156105735760019061056d6001600160a01b036105158386612357565b5116805f52609960205260405f2061054660ff86604051936105368561039c565b805485520154166020830161236b565b6105676105626105568351614167565b6001600160c01b031690565b612d2c565b91612e32565b016104f4565b005b34610440576020366003190112610440576004355f526098602052602060405f2054604051908152f35b63ffffffff81160361044057565b34610440576060366003190112610440576024356105ed6105e76004356105d38461059f565b604435905f52609860205260405f20611524565b50612420565b63ffffffff808251169216918210610674576040816106296106519460206106379501519063ffffffff821615918215610655575b5050612f08565b01516001600160c01b031690565b6040516001600160c01b0390911681529081906020820190565b0390f35b90915061066c9063ffffffff165b63ffffffff1690565b115f80610622565b636cb19aff60e01b5f5260045ffd5b5f91031261044057565b34610440575f36600319011261044057609d546040516001600160a01b039091168152602090f35b34610440575f3660031901126104405760a15460405160109190911c6001600160a01b03168152602090f35b6001600160401b0381116103b757601f01601f191660200190565b929192610708826106e1565b9161071660405193846103d7565b829481845281830111610440578281602093845f960137010152565b9080601f830112156104405781602061074d933591016106fc565b90565b34610440576020366003190112610440576004356001600160401b03811161044057610780903690600401610732565b335f52609960205260ff600160405f20015416600381101561087e5760016107a8911461239d565b335f90815260996020526040902054907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610440575f60405180936378219b3f60e11b8252856004830152604060248301528183816108176044820188612dea565b03925af1908115610879577fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa9261085a9261085f575b5060405191829182612f1e565b0390a2005b8061086d5f610873936103d7565b80610683565b5f61084d565b6123cb565b6112e1565b346104405760203660031901126104405760043561089f612f2f565b60a055005b34610440576020366003190112610440576004356108c18161042f565b60018060a01b03165f52609f602052602060405f2054604051908152f35b34610440576020366003190112610440576004356108fc8161042f565b60018060a01b03165f526099602052602060405f2054604051908152f35b346104405760203660031901126104405760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156108795761057392610990915f916109a4575b506123d6565b61099f600154828116146123ec565b612fbb565b6109c6915060203d6020116109cc575b6109be81836103d7565b8101906123b3565b5f61098a565b503d6109b4565b34610440576020366003190112610440576105736004356109f38161042f565b6109fb612f2f565b60a1805462010000600160b01b03191660109290921b62010000600160b01b0316919091179055565b34610440576020366003190112610440576004355f52609a602052602060ff60405f2054166040519015158152f35b34610440576040366003190112610440576060610a8b6105e7602435600435610a7a612402565b505f52609860205260405f20611524565b6040519063ffffffff815116825263ffffffff6020820151166020830152604060018060c01b03910151166040820152f35b6004359060ff8216820361044057565b359060ff8216820361044057565b346104405760203660031901126104405760ff610af6610abd565b165f52609b602052602060405f2054604051908152f35b34610440575f36600319011261044057609e546040516001600160a01b039091168152602090f35b34610440576020366003190112610440576040516308f6629d60e31b815260048035908201526020816024816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa801561087957610651915f91610bb9575b506040516001600160a01b0390911681529081906020820190565b610bdb915060203d602011610be1575b610bd381836103d7565b810190612451565b5f610b9e565b503d610bc9565b3461044057602036600319011261044057610573600435610c088161042f565b610c10612f2f565b612fed565b3461044057602036600319011261044057610573600435610c358161042f565b610c3d612f2f565b61304b565b34610440576020366003190112610440576040610c69600435610c648161042f565b61247e565b610c7f8251809260208091805184520151910152565bf35b60609060031901126104405760405190610c9a826103bc565b81600435610ca78161059f565b815260243561ffff811681036104405760208201526044359061ffff821682036104405760400152565b6001600160601b0381160361044057565b81601f8201121561044057803590610cf982610418565b92610d0760405194856103d7565b82845260208085019360061b8301019181831161044057602001925b828410610d31575050505090565b6040848303126104405760206040918251610d4b8161039c565b8635610d568161042f565b815282870135610d6581610cd1565b83820152815201930192610d23565b346104405760c036600319011261044057610d8e36610c81565b606435610d9a81610cd1565b6084356001600160401b03811161044057610db9903690600401610ce2565b9060a43591610dc78361059f565b610dcf612f2f565b610ddd60ff60a154166124e4565b60965460ff16938490610e1890610df660c08410612c5e565b610e12610e0288613214565b60ff1660ff196096541617609655565b86613477565b60a15460ff8116806110bc575b610f9e575b5050610e366001612b7b565b610e406001612b7b565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561044057610e96935f809460405196879586948593630662d3e160e51b85528b600486016133a7565b03925af1801561087957610f8a575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156104405760405163136ca0f960e11b815260ff83166004820152905f908290602490829084905af1801561087957610f76575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156104405760405163136ca0f960e11b815260ff83166004820152905f908290602490829084905af1801561087957610f6857005b8061086d5f610573936103d7565b8061086d5f610f84936103d7565b5f610f06565b8061086d5f610f98936103d7565b5f610ea5565b92610faa949194613226565b93610fb58451613273565b945f5b85518110156110015780610ffb610fe2610fd46001948a612357565b51516001600160a01b031690565b610fec838b612357565b6001600160a01b039091169052565b01610fb8565b5091949590929561101f6110136103f8565b63ffffffff9093168352565b602082015261102d8261234a565b526110378161234a565b506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169260101c1690823b1561044057611093925f9283604051809681958294630130fc2760e51b84526004840161329b565b03925af18015610879576110a8575b80610e2a565b8061086d5f6110b6936103d7565b5f6110a2565b506110d96110d58760a25460ff600192161c1660011490565b1590565b610e25565b9181601f84011215610440578235916001600160401b038311610440576020838186019501011161044057565b34610440576040366003190112610440576004356001600160401b038111610440573660238201121561044057806004013561114681610418565b9161115460405193846103d7565b8183526024602084019260051b820101903682116104405760248101925b8284106111a257602435856001600160401b0382116104405761119c6105739236906004016110de565b916124fa565b83356001600160401b038111610440576020916111c6839260243691870101610444565b815201930192611172565b346104405760a0366003190112610440576004356111ee8161042f565b6112596024356111fd8161042f565b6044356112098161042f565b606435906084359261121a8461042f565b5f549561123f60ff600889901c1615809881996112d3575b81156112b3575b50612802565b86611250600160ff195f5416175f55565b61129c576128ca565b61125f57005b61126d61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b6112ae61010061ff00195f5416175f55565b6128ca565b303b159150816112c5575b505f611239565b60ff1660011490505f6112be565b600160ff8216109150611232565b634e487b7160e01b5f52602160045260245ffd5b6003111561087e57565b90600382101561087e5752565b34610440576020366003190112610440576004356113298161042f565b611331612466565b5060018060a01b03165f52609960205260405f2061135960ff6001604051936105368561039c565b60405180916106516020604084019280518552015160208401906112ff565b34610440575f3660031901126104405760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610879576113e3915f916109a457506123d6565b610573612f87565b34610440576020366003190112610440576020600160ff61140a610abd565b161b806001541614604051908152f35b3461044057608036600319011261044057611433610abd565b60603660231901126104405760405161144b816103bc565b6024356114578161059f565b815260443561ffff8116810361044057602082015260643561ffff81168103610440576040820152611487612f2f565b60ff6096541660ff831610156114a05761057391613477565b637310cff560e11b5f5260045ffd5b34610440575f366003190112610440576020600154604051908152f35b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b634e487b7160e01b5f52603260045260245ffd5b8054821015611539575f5260205f2001905f90565b611510565b3461044057602036600319011261044057600435609c5481101561044057609c5f527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c01546040516001600160a01b039091168152602090f35b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610440576040366003190112610440576004356115f98161042f565b6024356001600160401b03811161044057611618903690600401610732565b609e546001600160a01b031633036116ea576001600160a01b0382165f908152609f6020908152604080832042905560999091529020805460016116808161167761167161055661166b60965460ff1690565b89613404565b94614167565b94015460ff1690565b611689816112f5565b1491826116d7575b826116be575b505061169f57005b6116a98183613589565b60a15460ff166116b557005b61057391613859565b81166001600160c01b0390811691161490505f80611697565b6001600160c01b03821615159250611691565b6376d8ab1760e11b5f5260045ffd5b34610440575f36600319011261044057611711612f2f565b606480546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610440575f36600319011261044057602060ff60a154166040519015158152f35b346104405760a03660031901126104405761179036610c81565b60643561179c81610cd1565b6084356001600160401b038111610440576117bb903690600401610ce2565b906117c4612f2f565b60965460ff169283906117ef906117dd60c08410612c5e565b6117e9610e0287613214565b85613477565b60a15460ff811680611959575b611862575b505061180c5f612b7b565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561044057610e96925f9283604051809681958294633aea0b9d60e11b84528a600485016133dc565b9161186f94919394613226565b9261187a8651613273565b935f5b87518110156118a957806118a3611899610fd46001948c612357565b610fec838a612357565b0161187d565b50919395949092956118bc6110136103f8565b60208201526118ca8261234a565b526118d48161234a565b506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169260101c1690823b1561044057611930925f9283604051809681958294630130fc2760e51b84526004840161329b565b03925af1801561087957611945575b80611801565b8061086d5f611953936103d7565b5f61193f565b506119726110d58660a25460ff600192161c1660011490565b6117fc565b81601f820112156104405780359061198e82610418565b9261199c60405194856103d7565b82845260208085019360061b8301019181831161044057602001925b8284106119c6575050505090565b60408483031261044057602060409182516119e08161039c565b6119e987610acd565b8152828701356119f88161042f565b838201528152019301926119b8565b346104405760a036600319011261044057600435611a248161042f565b60243590604435906001600160401b03821161044057602092611a4e611a5b933690600401611977565b606435916084359361298a565b604051908152f35b34610440576020366003190112610440576020611a81600435614167565b6040516001600160c01b039091168152f35b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610440575f366003190112610440576064546040516001600160a01b039091168152602090f35b34610440575f36600319011261044057602060ff60965416604051908152f35b9080601f83011215610440578135611b3681610418565b92611b4460405194856103d7565b81845260208085019260051b82010192831161044057602001905b828210611b6c5750505090565b602080918335611b7b8161059f565b815201910190611b5f565b3461044057604036600319011261044057600435611ba38161042f565b602435906001600160401b03821161044057611bf5611bc9610573933690600401611b1f565b611bd161397e565b611be2600280600154161415612334565b611bf060ff60a154166124e4565b6139bf565b90613589565b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610440575f3660031901126104405760206040517f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de68152f35b34610440576020366003190112610440576020611ca8611c97610abd565b60a25460ff600192161c1660011490565b6040519015158152f35b34610440575f36600319011261044057602060a054604051908152f35b3461044057606036600319011261044057600435611cec8161042f565b6024356001600160401b03811161044057611d0b903690600401611b1f565b906044356001600160401b03811161044057611d79611d31611d609236906004016110de565b929094611d3c61397e565b611d4c6104ec6001808054161490565b611bf0611d5b60a15460ff1690565b6124e4565b918401611d6d8186612b38565b86979297949194613ab9565b95611d8381612b7b565b80611ebd57505050611d9790828585613eb2565b51925f5b8251811015611e205780611e1a8663ffffffff611e11610663611e07611df5611deb88611de5611ddf611dd18f9d60019e612c4d565b516001600160f81b03191690565b60f81c90565b97612357565b5163ffffffff1690565b9460ff165f52609760205260405f2090565b5463ffffffff1690565b91161115612c5e565b01611d9b565b509250505b6001611e4d81611e458460018060a01b03165f52609960205260405f2090565b015460ff1690565b611e56816112f5565b03611e5d57005b611e8e611e686103f8565b838152600160208201526001600160a01b0383165f908152609960205260409020612c74565b6001600160a01b03167fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe5f80a3005b80611ecc600192969496612b7b565b03611ef157611eec93611ede91612b85565b959350939150508686613bbb565b611e25565b63354bb8ab60e01b5f5260045ffd5b34610440575f36600319011261044057602060ff60a15460081c166040519015158152f35b60206040818301928281528451809452019201905f5b818110611f485750505090565b825163ffffffff16845260209384019390920191600101611f3b565b3461044057604036600319011261044057600435611f818161059f565b602435906001600160401b038211610440573660238301121561044057816004013591611fad83610418565b92611fbb60405194856103d7565b8084526024602085019160051b8301019136831161044057602401905b828210611ff857610651611fec8686614108565b60405191829182611f25565b8135815260209182019101611fd8565b34610440575f3660031901126104405760206040517f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a8152f35b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610440575f366003190112610440576020609c54604051908152f35b346104405760203660031901126104405760ff6120be610abd565b6120c6612402565b50165f52609760205261065160405f2061ffff604051916120e6836103bc565b5463ffffffff81168352818160201c16602084015260301c16604082015260405191829182919091604061ffff81606084019563ffffffff8151168552826020820151166020860152015116910152565b34610440575f366003190112610440576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610440576020366003190112610440576004356121988161042f565b6121a0612f2f565b6001600160a01b038116156121b8576105739061342f565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346104405760203660031901126104405760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610879575f916122cf575b506001600160a01b031633036122c05761228e6001541982198116146123ec565b806001556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b6122e8915060203d602011610be157610bd381836103d7565b5f61226d565b346104405760203660031901126104405760043561230b8161042f565b60018060a01b03165f526099602052602060ff600160405f20015416610c7f60405180926112ff565b1561233b57565b63840a48d560e01b5f5260045ffd5b8051156115395760200190565b80518210156115395760209160051b010190565b600382101561087e5752565b906104076040516123878161039c565b602060ff6001839680548552015416910161236b565b156123a457565b63aba4733960e01b5f5260045ffd5b90816020910312610440575180151581036104405790565b6040513d5f823e3d90fd5b156123dd57565b631d77d47760e21b5f5260045ffd5b156123f357565b63c61dca5d60e01b5f5260045ffd5b6040519061240f826103bc565b5f6040838281528260208201520152565b9060405161242d816103bc565b604081935463ffffffff8116835263ffffffff8160201c166020840152811c910152565b90816020910312610440575161074d8161042f565b604051906124738261039c565b5f6020838281520152565b6124df61074d9161248d612466565b50604080517f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de6602082019081526001600160a01b03909316818301529081526124d76060826103d7565b5190206130a9565b6130f6565b156124eb57565b635b77901960e01b5f5260045ffd5b90929161250e6104ec600480600154161490565b61252d61251d60965460ff1690565b6125283684886106fc565b613404565b5061253a8183511461273d565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316935f5b82811061257657505050509050565b612595611ddf612587838686612753565b356001600160f81b03191690565b926125a08286612357565b5180516040516379a0849160e11b815260ff87166004820152919791906020826024818d5afa918215610879576125e69263ffffffff915f9161270f575b501614612774565b5f97885b88518a10156126a35760019061269b6126136126068d8d612357565b516001600160a01b031690565b916126766126396126348560018060a01b03165f52609960205260405f2090565b612377565b9161266161265c8d61264e6105568751614167565b60ff600192161c1660011490565b61278a565b858060a01b0316858060a01b038516116127a0565b61269461268d6126858a6127ca565b8a8a8d6127ea565b36916106fc565b9083612e32565b9901986125ea565b50965096509290600191949294436126c68260ff165f52609b60205260405f2090565b557f46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db460ff6040519216918061270043829190602083019252565b0390a201949394929092612567565b612730915060203d8111612736575b61272881836103d7565b81019061275f565b5f6125de565b503d61271e565b1561274457565b63aaad13f760e01b5f5260045ffd5b90821015611539570190565b90816020910312610440575161074d8161059f565b1561277b57565b638e5aeee760e01b5f5260045ffd5b1561279157565b63d053aa2160e01b5f5260045ffd5b156127a757565b63ba50f91160e01b5f5260045ffd5b634e487b7160e01b5f52601160045260245ffd5b90600182018092116127d857565b6127b6565b919082018092116127d857565b90939293848311610440578411610440578101920390565b1561280957565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b609c54600160401b8110156103b75760018101609c55609c5481101561153957609c5f527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c0180546001600160a01b0319166001600160a01b03909216919091179055565b6128e3949361099f6109fb9493610c10610c3d9461342f565b6129157f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612865565b6129477f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612865565b6129797f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612865565b61010161ffff1960a154161760a155565b919493909260405192602084019460e08501917f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a875260018060a01b03166040860152606085015260c060808501528651809152602061010085019701905f5b818110612a175750505061074d94956124d792849260a084015260c083015203601f1981018352826103d7565b8251805160ff168a526020908101516001600160a01b0316818b0152604090990198909201916001016129ea565b3590600282101561044057565b919082604091031261044057604051612a6a8161039c565b6020808294803584520135910152565b9080601f830112156104405760405191612a956040846103d7565b82906040810192831161044057905b828210612ab15750505090565b8135815260209182019101612aa4565b8082039291610100841261044057604051612adb816103bc565b60808195612ae98486612a52565b8352612af88460408701612a52565b6020840152607f19011261044057612b2f60409260c0845195612b1a8761039c565b612b278360808301612a7a565b875201612a7a565b60208401520152565b90916101408284031261044057612b4e82612a45565b926020830135906001600160401b038211610440576040612b748261074d948701610732565b9401612ac1565b6002111561087e57565b91906101808382031261044057612b9b83612a45565b9260208101356001600160401b0381116104405782612bbb918301610732565b92612bc98360408401612ac1565b926101408301356001600160401b0381116104405781612bea918501611977565b92610160810135906001600160401b0382116104405701906060828203126104405760405191612c19836103bc565b80356001600160401b03811161044057604092612c37918301610732565b8352602081013560208401520135604082015290565b908151811015611539570160200190565b15612c6557565b633cb89c9760e01b5f5260045ffd5b60016020918351815501910151600381101561087e5760ff80198354169116179055565b90604051612ca5816103bc565b604061ffff82945463ffffffff81168452818160201c16602085015260301c16910152565b60408051909190612cdb83826103d7565b6001815291601f1901366020840137565b90612cf6826106e1565b612d0360405191826103d7565b8281528092612d14601f19916106e1565b0190602036910137565b5f1981146127d85760010190565b5f81805b612da65750612d429061ffff16612cec565b5f5f5b8251821080612d9b575b15612d94576001811b8416612d6d575b612d6890612d1e565b612d45565b906001612d689160ff60f81b8460f81b165f1a612d8a8287612c4d565b5301919050612d5f565b5050905090565b506101008110612d4f565b5f1981018181116127d85761ffff9116911661ffff81146127d8576001019080612d30565b9081602091031261044057516001600160c01b03811681036104405790565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b61074d939260609260018060a01b0316825260208201528160408201520190612dea565b919060016020820151612e44816112f5565b612e4d816112f5565b03612f0357516040516333567f7f60e11b81529160209183918291612e7791908760048501612e0e565b03815f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af1908115610879575f91612ed4575b506001600160c01b03169081612ec8575050565b611bf561040792612d2c565b612ef6915060203d602011612efc575b612eee81836103d7565b810190612dcb565b5f612eb4565b503d612ee4565b505050565b15612f0f57565b63bbba60cb60e01b5f5260045ffd5b90602061074d928181520190612dea565b6064546001600160a01b03163303612f4357565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b5f196001556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806001556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b609d54604080516001600160a01b038084168252841660208201529192917f315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c9190a16001600160a01b03166001600160a01b03199190911617609d55565b609e54604080516001600160a01b038084168252841660208201529192917f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc99190a16001600160a01b03166001600160a01b03199190911617609e55565b6130b16141ab565b9060405190602082019261190160f01b845260228301526042820152604281526130dc6062826103d7565b51902090565b634e487b7160e01b5f52601260045260245ffd5b5f516020614aef5f395f51905f529061310d612466565b505f919006602060c0835b61320d575f935f516020614aef5f395f51905f526003818681818009090860405161314385826103d7565b8436823784818560405161315782826103d7565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614aef5f395f51905f5260a082015260056107cf195a01fa8015613212576131c1906147aa565b519161320d575f516020614aef5f395f51905f52828009146131f857505f516020614aef5f395f51905f5260015f94089293613118565b929350506132046103f8565b92835282015290565b6130e2565bfe5b60ff60019116019060ff82116127d857565b6040805190919061323783826103d7565b6001815291601f1901825f5b82811061324f57505050565b60209060405161325e8161039c565b5f815260608382015282828501015201613243565b9061327d82610418565b61328a60405191826103d7565b8281528092612d14601f1991610418565b90604082019060018060a01b031682526040602083015282518091526060820191602060608360051b8301019401925f915b8383106132dc57505050505090565b9091929394605f1982820301835285516020606081604085019363ffffffff81511686520151936040838201528451809452019201905f905b80821061333457505050602080600192970193019301919392906132cd565b82516001600160a01b0316845260209384019390920191600190910190613315565b90602080835192838152019201905f5b8181106133735750505090565b825180516001600160a01b031685526020908101516001600160601b03168186015260409094019390920191600101613366565b9061074d94936001600160601b0360809460ff63ffffffff941685521660208401521660408201528160608201520190613356565b6001600160601b0361074d949360ff6060941683521660208201528160408201520190613356565b90600161341260ff936142d7565b928392161b11156134205790565b63ca95733360e01b5f5260045ffd5b606480546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61353560ff7f3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac921692835f52609760205260405f206134cd63ffffffff835116829063ffffffff1663ffffffff19825416179055565b6020820151815465ffff0000000067ffff000000000000604086015160301b169260201b169067ffffffff0000000019161717905560405191829182919091604061ffff81606084019563ffffffff8151168552826020820151166020860152015116910152565b0390a2565b1561354157565b6368b6a87560e11b5f5260045ffd5b6001600160a01b03909116815260406020820181905261074d92910190612dea565b60409061074d939281528160208201520190612dea565b6001600160a01b0381165f908152609960205260409020906136148254926135c860016135b98161167788614167565b6135c2816112f5565b1461239d565b6135e36105566105566135dd60965460ff1690565b88613404565b906135ef82151561353a565b6136058282166001600160c01b0316831461278a565b9019166001600160c01b031690565b61361e818461434f565b6001600160c01b031615613790575b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561044057835f91613682938360405180968195829463f4e24fe560e01b845260048401613550565b03925af180156108795761377c575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b15610440575f604051809263bd29b8cd60e01b82528183816136e4898960048401613572565b03925af1801561087957613768575b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561044057613749925f928360405180968195829463bd29b8cd60e01b845260048401613572565b03925af180156108795761375a5750565b8061086d5f610407936103d7565b8061086d5f613776936103d7565b5f6136f3565b8061086d5f61378a936103d7565b5f613691565b6001600160a01b0381165f9081526099602052604090206137bb90600101805460ff19166002179055565b816001600160a01b0382167f396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e45f80a361362d565b602080825282516001600160a01b039081168284015281840151166040808401919091529092015160608083015280516080830181905260a09092019201905f5b81811061383d5750505090565b825163ffffffff16845260209384019390920191600101613830565b90916138658351613273565b60a2545f92835b86518110156138d757613885611ddf611dd1838a612c4d565b613898818560ff600192161c1660011490565b6138a6575b5060010161386c565b94906138d1826138c660ff6138bd60019693612d1e565b99169188612357565b9063ffffffff169052565b9061389d565b5094505090806138e657505050565b815260a1546001600160a01b037f00000000000000000000000000000000000000000000000000000000000000008116929161394d9160109190911c1661393d61392e610409565b6001600160a01b039096168652565b6001600160a01b03166020850152565b6040830152803b1561044057604051636e3492b560e01b8152915f91839182908490829061374990600483016137ef565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036139b057565b6323d871a560e01b5f5260045ffd5b906139ca8251612cec565b5f5b8351811015613a03576001906001600160f81b03196139eb8287612357565b5160f81b165f1a6139fc8285612c4d565b53016139cc565b509150565b90816020910312610440575190565b905f905b60028210613a2857505050565b6020806001928551815201930191019091613a1b565b61012090613aa860206040610407969897959861016085019960018060a01b03168552613a78838601825160208091805184520151910152565b80830151805160608701526020015160808601520151613a9c60a085018251613a17565b015160e0830190613a17565b019060208091805184520151910152565b6040516309aa152760e11b81526001600160a01b0382811660048301529091907f000000000000000000000000000000000000000000000000000000000000000016602083602481845afa928315610879575f93613b9a575b508215613b20575050905090565b60209250613b50935f613b328461247e565b6040516317ef39cb60e31b8152968795869485939160048501613a3e565b03925af1908115610879575f91613b6b575b50805f80612d94565b613b8d915060203d602011613b93575b613b8581836103d7565b810190613a08565b5f613b62565b503d613b7b565b613bb491935060203d602011613b9357613b8581836103d7565b915f613b12565b929091613be392613bdd829897613bd5885185511461273d565b878388614469565b84613eb2565b905f5b8551811015613d185780613c1e613c19613c08611ddf611dd16001968c612c4d565b60ff165f52609760205260405f2090565b612c98565b613c2c611deb838751612357565b63ffffffff613c42610663845163ffffffff1690565b911611613c51575b5001613be6565b613ca790613c65611ddf611dd1858c612c4d565b613c83613c768560408a0151612357565b516001600160601b031690565b86613c95613c768760208c0151612357565b91613ca0878c612357565b519361454a565b613caf612cca565b613cbc611dd1838a612c4d565b5f1a613cc78261234a565b53613cef81613cea6020613cdb868b612357565b5101516001600160a01b031690565b613589565b60a15460ff1615613c4a57613d1290613d0d6020613cdb858a612357565b613859565b5f613c4a565b505050509050565b60405190613d2d826103bc565b60606040838281528260208201520152565b15613d4657565b6313ca465760e01b5f5260045ffd5b15613d5c57565b630c6816cd60e01b5f5260045ffd5b15613d7257565b631968677d60e11b5f5260045ffd5b9080601f83011215610440578151613d9881610418565b92613da660405194856103d7565b81845260208085019260051b82010192831161044057602001905b828210613dce5750505090565b602080918351613ddd81610cd1565b815201910190613dc1565b9190916040818403126104405780516001600160401b0381116104405783613e11918301613d81565b9260208201516001600160401b0381116104405761074d9201613d81565b602081830312610440578051906001600160401b03821161044057019080601f83011215610440578151613e6281610418565b92613e7060405194856103d7565b81845260208085019260051b82010192831161044057602001905b828210613e985750505090565b602080918351613ea78161059f565b815201910190613e8b565b817fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa613f78613edf613d20565b96613f6c613efb610556613ef560965460ff1690565b8a613404565b613f0486614167565b6001600160c01b0390911690613f1b821515613d3f565b60018060c01b0316613f35613f308284161590565b613d55565b6001600160a01b0388165f908152609f60205260409020613f6590613f5e905460a054906127dd565b4211613d6b565b178561434f565b60405191829182612f1e565b0390a27f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b15610440575f6040518092631fd93ca960e11b8252818381613fcd8a8960048401613550565b03925af180156108795784925f9285926140f4575b506140016040519687938493632550477760e01b855260048501612e0e565b0381837f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af19182156108795761406b935f93849185916140d0575b5060408701526020860152604051938492839262bff04d60e01b845260048401613572565b0381837f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af1908115610879575f916140ae575b50815290565b6140ca91503d805f833e6140c281836103d7565b810190613e2f565b5f6140a8565b90506140ee91503d8086833e6140e681836103d7565b810190613de8565b5f614046565b8061086d85614102936103d7565b5f613fe2565b91906141148151613273565b905f5b8151811015614152578061413961413060019385612357565b518760986146a0565b63ffffffff6141488387612357565b9116905201614117565b5090925050565b5f198101919082116127d857565b805f52609860205260405f20549081155f146141835750505f90565b5f52609860205260405f20905f1981019081116127d8576141a391611524565b505460401c90565b307f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03161480614298575b15614206577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f000000000000000000000000000000000000000000000000000000000000000082527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a081526130dc60c0826103d7565b507f000000000000000000000000000000000000000000000000000000000000000046146141dd565b156142c857565b631019106960e31b5f5260045ffd5b906101008251116143405781511561433b57602082015160019060f81c81901b5b835182101561433657600190614321614317611ddf611dd18689612c4d565b60ff600191161b90565b9061432d8183116142c1565b179101906142f8565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b90614362825f52609860205260405f2090565b54806143af575061437e610407925f52609860205260405f2090565b6143aa614389610409565b4363ffffffff168152925b5f60208501526001600160c01b03166040840152565b6147c0565b916143da63ffffffff936143d46143ce845f52609860205260405f2090565b91614159565b90611524565b50906143ea825463ffffffff1690565b4385169416840361441557506104079250906001600160401b0382549181199060401b169116179055565b815467ffffffff000000001916602085901b67ffffffff0000000016179091556104079291906143aa90614451905f52609860205260405f2090565b9161439461445d610409565b63ffffffff9095168552565b919290602082019283515f52609a60205260ff60405f2054166144e45760408301805142116144d557610407956144cd9386515f52609a6020526144b760405f20600160ff19825416179055565b609d546001600160a01b0316965192519361298a565b90519161482e565b630819bdcd60e01b5f5260045ffd5b636fbefec360e11b5f5260045ffd5b156144fa57565b6356168b4160e11b5f5260045ffd5b90816020910312610440575161074d81610cd1565b1561452557565b634c44995d60e01b5f5260045ffd5b1561453b57565b63b187e86960e01b5f5260045ffd5b602091926145a761459a6145d298969761459361456f8783015160018060a01b031690565b6001600160a01b039081165f818152609960205260409020549690911614156144f3565b5160ff1690565b60ff808516911614612774565b604051635401ed2760e01b8152600481019190915260ff909116602482015294859081906044820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa92831561087957610407945f94614656575b508261464e92614649614635936001600160601b0361464161463582998b614872565b6001600160601b031690565b91161161451e565b614895565b911610614534565b6146359194509261464e9261464961468d6001600160601b039660203d602011614699575b61468581836103d7565b810190614509565b96935050925092614612565b503d61467b565b9190815f528260205260405f2054925f5b8481106147495760405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a490fd5b8085038581116127d85761066361475f91614159565b61478861477d826147788887905f5260205260405f2090565b611524565b505463ffffffff1690565b63ffffffff808616911611156147a157506001016146b1565b94505050505090565b156147b157565b63d51edae360e01b5f5260045ffd5b8054600160401b8110156103b7576147dd91600182018155611524565b61481b57815160208084015160409485015163ffffffff909316911b67ffffffff00000000161767ffffffffffffffff199190931b16919091179055565b634e487b7160e01b5f525f60045260245ffd5b90614839929161490c565b1561484057565b638baa579f60e01b5f5260045ffd5b906001600160601b03809116911602906001600160601b0382169182036127d857565b6148906001600160601b039161ffff6020612710950151169061484f565b160490565b6148906001600160601b039161ffff6040612710950151169061484f565b6005111561087e57565b3d156148e7573d906148ce826106e1565b916148dc60405193846103d7565b82523d5f602084013e565b606090565b9081602091031261044057516001600160e01b0319811681036104405790565b91909161491982846149d7565b614922816148b3565b1590816149c1575b506149b9575f9261495761496585946040519283916020830195630b135d3f60e11b875260248401613572565b03601f1981018352826103d7565b51915afa6149716148bd565b816149ad575b81614980575090565b8051630b135d3f60e11b92506001600160e01b0319916149a8918101602090810191016148ec565b161490565b80516020149150614977565b505050600190565b6001600160a01b0383811691161490505f61492a565b815160418103614a035750906149ff91602082015190606060408401519301515f1a90614a45565b9091565b604003614a3c5760406020830151920151918260ff1c91601b83018093116127d8576149ff936001600160ff1b03169260ff1690614a45565b50505f90600290565b9291907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311614ae35760ff16601b81141580614ad8575b614acd576020935f93604051938493608085019385528785015260408401526060830152838052039060015afa15610879575f516001600160a01b03811615614ac557905f90565b505f90600190565b505050505f90600490565b50601c811415614a7d565b505050505f9060039056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220a0f52feb6762df4a106a0afcb3c8bdddc282cea4885da5717dd983812850244964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\xCF*\xB5\x14a\x03\x83W\x80c\x03\xFD4\x92\x14a\x03~W\x80c\x04\xECcQ\x14a\x03yW\x80c\x05C\x10\xE6\x14a\x03tW\x80c\x07d\xCB\x93\x14a\x03oW\x80c\x0C\xF4\xB7g\x14a\x03jW\x80c\r?!4\x14a\x03eW\x80c\x12^\x05\x84\x14a\x03`W\x80c\x13T*N\x14a\x03[W\x80c\x13d9\xDD\x14a\x03VW\x80c\x14>Y\x15\x14a\x03QW\x80c\x14x\x85\x1F\x14a\x03LW\x80c\x1E\xB8\x12\xDA\x14a\x03GW\x80c$\x9A\x0CB\x14a\x03BW\x80c(\xF6\x1B1\x14a\x03=W\x80c)k\xB0d\x14a\x038W\x80c)\xD1\xE0\xC3\x14a\x033W\x80c,\xDD\x1E\x86\x14a\x03.W\x80c<*\x7FL\x14a\x03)W\x80c>\xEF:Q\x14a\x03$W\x80cQ@\xA5H\x14a\x03\x1FW\x80cS\x0B\x97\xA4\x14a\x03\x1AW\x80cXe\xC6\x0C\x14a\x03\x15W\x80cY\\jg\x14a\x03\x10W\x80cZ\xC8j\xB7\x14a\x03\x0BW\x80c[\x0B\x82\x9F\x14a\x03\x06W\x80c\\\x97Z\xBB\x14a\x03\x01W\x80c]\xF4YF\x14a\x02\xFCW\x80ccG\xC9\0\x14a\x02\xF7W\x80ch0H5\x14a\x02\xF2W\x80cn;\x17\xDB\x14a\x02\xEDW\x80cqP\x18\xA6\x14a\x02\xE8W\x80c\x81\xF96\xD2\x14a\x02\xE3W\x80c\x82\x81\xABu\x14a\x02\xDEW\x80c\x84\xCAR\x13\x14a\x02\xD9W\x80c\x87\x1E\xF0I\x14a\x02\xD4W\x80c\x88o\x11\x95\x14a\x02\xCFW\x80c\x8D\xA5\xCB[\x14a\x02\xCAW\x80c\x9A\xA1e=\x14a\x02\xC5W\x80c\x9D\x8E\x0C#\x14a\x02\xC0W\x80c\x9E\x99#\xC2\x14a\x02\xBBW\x80c\x9F\xEA\xB8Y\x14a\x02\xB6W\x80c\xA4\xD7\x87\x1F\x14a\x02\xB1W\x80c\xA9ox>\x14a\x02\xACW\x80c\xAD\xCFs\xF7\x14a\x02\xA7W\x80c\xB2\xD8g\x8D\x14a\x02\xA2W\x80c\xC3\x91B^\x14a\x02\x9DW\x80c\xCA\r\xE8\x82\x14a\x02\x98W\x80c\xCA\x8A\xA7\xC7\x14a\x02\x93W\x80c\xD7-\x8D\xD6\x14a\x02\x8EW\x80c\xE6W\x97\xAD\x14a\x02\x89W\x80c\xEA2\xAF\xAE\x14a\x02\x84W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x7FW\x80c\xFA\xBC\x1C\xBC\x14a\x02zWc\xFD9\x10Z\x14a\x02uW_\x80\xFD[a\"\xEEV[a\"\x0CV[a!{V[a!7V[a \xA3V[a \x86V[a BV[a \x08V[a\x1FdV[a\x1F\0V[a\x1C\xCFV[a\x1C\xB2V[a\x1CyV[a\x1C?V[a\x1B\xFBV[a\x1B\x86V[a\x1A\xFFV[a\x1A\xD7V[a\x1A\x93V[a\x1AcV[a\x1A\x07V[a\x17vV[a\x17TV[a\x16\xF9V[a\x15\xDCV[a\x15\x98V[a\x15>V[a\x14\xCCV[a\x14\xAFV[a\x14\x1AV[a\x13\xEBV[a\x13xV[a\x13\x0CV[a\x11\xD1V[a\x11\x0BV[a\rtV[a\x0CBV[a\x0C\x15V[a\x0B\xE8V[a\x0B5V[a\x0B\rV[a\n\xDBV[a\nSV[a\n$V[a\t\xD3V[a\t\x1AV[a\x08\xDFV[a\x08\xA4V[a\x08\x83V[a\x07PV[a\x06\xB5V[a\x06\x8DV[a\x05\xADV[a\x05uV[a\x04\xABV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xB7W`@RV[a\x03\x88V[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xB7W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xB7W`@RV[`@Q\x90a\x04\x07`@\x83a\x03\xD7V[V[`@Q\x90a\x04\x07``\x83a\x03\xD7V[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xB7W`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x04@WV[_\x80\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x04@W\x815a\x04[\x81a\x04\x18V[\x92a\x04i`@Q\x94\x85a\x03\xD7V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04@W` \x01\x90[\x82\x82\x10a\x04\x91WPPP\x90V[` \x80\x91\x835a\x04\xA0\x81a\x04/V[\x81R\x01\x91\x01\x90a\x04\x84V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x04\xDB\x906\x90`\x04\x01a\x04DV[a\x04\xF2a\x04\xEC`\x04\x80`\x01T\x16\x14\x90V[\x15a#4V[_[\x81Q\x81\x10\x15a\x05sW`\x01\x90a\x05m`\x01`\x01`\xA0\x1B\x03a\x05\x15\x83\x86a#WV[Q\x16\x80_R`\x99` R`@_ a\x05F`\xFF\x86`@Q\x93a\x056\x85a\x03\x9CV[\x80T\x85R\x01T\x16` \x83\x01a#kV[a\x05ga\x05ba\x05V\x83QaAgV[`\x01`\x01`\xC0\x1B\x03\x16\x90V[a-,V[\x91a.2V[\x01a\x04\xF4V[\0[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045_R`\x98` R` `@_ T`@Q\x90\x81R\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x04@WV[4a\x04@W``6`\x03\x19\x01\x12a\x04@W`$5a\x05\xEDa\x05\xE7`\x045a\x05\xD3\x84a\x05\x9FV[`D5\x90_R`\x98` R`@_ a\x15$V[Pa$ V[c\xFF\xFF\xFF\xFF\x80\x82Q\x16\x92\x16\x91\x82\x10a\x06tW`@\x81a\x06)a\x06Q\x94` a\x067\x95\x01Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x15\x91\x82\x15a\x06UW[PPa/\x08V[\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x90\x91Pa\x06l\x90c\xFF\xFF\xFF\xFF\x16[c\xFF\xFF\xFF\xFF\x16\x90V[\x11_\x80a\x06\"V[cl\xB1\x9A\xFF`\xE0\x1B_R`\x04_\xFD[_\x91\x03\x12a\x04@WV[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`\x9DT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`\xA1T`@Q`\x10\x91\x90\x91\x1C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xB7W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\x08\x82a\x06\xE1V[\x91a\x07\x16`@Q\x93\x84a\x03\xD7V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x04@W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x04@W\x81` a\x07M\x935\x91\x01a\x06\xFCV[\x90V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x07\x80\x906\x90`\x04\x01a\x072V[3_R`\x99` R`\xFF`\x01`@_ \x01T\x16`\x03\x81\x10\x15a\x08~W`\x01a\x07\xA8\x91\x14a#\x9DV[3_\x90\x81R`\x99` R`@\x90 T\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04@W_`@Q\x80\x93cx!\x9B?`\xE1\x1B\x82R\x85`\x04\x83\x01R`@`$\x83\x01R\x81\x83\x81a\x08\x17`D\x82\x01\x88a-\xEAV[\x03\x92Z\xF1\x90\x81\x15a\x08yW\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x92a\x08Z\x92a\x08_W[P`@Q\x91\x82\x91\x82a/\x1EV[\x03\x90\xA2\0[\x80a\x08m_a\x08s\x93a\x03\xD7V[\x80a\x06\x83V[_a\x08MV[a#\xCBV[a\x12\xE1V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a\x08\x9Fa//V[`\xA0U\0[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a\x08\xC1\x81a\x04/V[`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R` `@_ T`@Q\x90\x81R\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a\x08\xFC\x81a\x04/V[`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R` `@_ T`@Q\x90\x81R\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x08yWa\x05s\x92a\t\x90\x91_\x91a\t\xA4W[Pa#\xD6V[a\t\x9F`\x01T\x82\x81\x16\x14a#\xECV[a/\xBBV[a\t\xC6\x91P` =` \x11a\t\xCCW[a\t\xBE\x81\x83a\x03\xD7V[\x81\x01\x90a#\xB3V[_a\t\x8AV[P=a\t\xB4V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@Wa\x05s`\x045a\t\xF3\x81a\x04/V[a\t\xFBa//V[`\xA1\x80Tb\x01\0\0`\x01`\xB0\x1B\x03\x19\x16`\x10\x92\x90\x92\x1Bb\x01\0\0`\x01`\xB0\x1B\x03\x16\x91\x90\x91\x17\x90UV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045_R`\x9A` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04@W`@6`\x03\x19\x01\x12a\x04@W``a\n\x8Ba\x05\xE7`$5`\x045a\nza$\x02V[P_R`\x98` R`@_ a\x15$V[`@Q\x90c\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@`\x01\x80`\xC0\x1B\x03\x91\x01Q\x16`@\x82\x01R\xF3[`\x045\x90`\xFF\x82\x16\x82\x03a\x04@WV[5\x90`\xFF\x82\x16\x82\x03a\x04@WV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\xFFa\n\xF6a\n\xBDV[\x16_R`\x9B` R` `@_ T`@Q\x90\x81R\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`\x9ET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x805\x90\x82\x01R` \x81`$\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x08yWa\x06Q\x91_\x91a\x0B\xB9W[P`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[a\x0B\xDB\x91P` =` \x11a\x0B\xE1W[a\x0B\xD3\x81\x83a\x03\xD7V[\x81\x01\x90a$QV[_a\x0B\x9EV[P=a\x0B\xC9V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@Wa\x05s`\x045a\x0C\x08\x81a\x04/V[a\x0C\x10a//V[a/\xEDV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@Wa\x05s`\x045a\x0C5\x81a\x04/V[a\x0C=a//V[a0KV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`@a\x0Ci`\x045a\x0Cd\x81a\x04/V[a$~V[a\x0C\x7F\x82Q\x80\x92` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\xF3[``\x90`\x03\x19\x01\x12a\x04@W`@Q\x90a\x0C\x9A\x82a\x03\xBCV[\x81`\x045a\x0C\xA7\x81a\x05\x9FV[\x81R`$5a\xFF\xFF\x81\x16\x81\x03a\x04@W` \x82\x01R`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x04@W`@\x01RV[`\x01`\x01``\x1B\x03\x81\x16\x03a\x04@WV[\x81`\x1F\x82\x01\x12\x15a\x04@W\x805\x90a\x0C\xF9\x82a\x04\x18V[\x92a\r\x07`@Q\x94\x85a\x03\xD7V[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x04@W` \x01\x92[\x82\x84\x10a\r1WPPPP\x90V[`@\x84\x83\x03\x12a\x04@W` `@\x91\x82Qa\rK\x81a\x03\x9CV[\x865a\rV\x81a\x04/V[\x81R\x82\x87\x015a\re\x81a\x0C\xD1V[\x83\x82\x01R\x81R\x01\x93\x01\x92a\r#V[4a\x04@W`\xC06`\x03\x19\x01\x12a\x04@Wa\r\x8E6a\x0C\x81V[`d5a\r\x9A\x81a\x0C\xD1V[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\r\xB9\x906\x90`\x04\x01a\x0C\xE2V[\x90`\xA45\x91a\r\xC7\x83a\x05\x9FV[a\r\xCFa//V[a\r\xDD`\xFF`\xA1T\x16a$\xE4V[`\x96T`\xFF\x16\x93\x84\x90a\x0E\x18\x90a\r\xF6`\xC0\x84\x10a,^V[a\x0E\x12a\x0E\x02\x88a2\x14V[`\xFF\x16`\xFF\x19`\x96T\x16\x17`\x96UV[\x86a4wV[`\xA1T`\xFF\x81\x16\x80a\x10\xBCW[a\x0F\x9EW[PPa\x0E6`\x01a+{V[a\x0E@`\x01a+{V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@Wa\x0E\x96\x93_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x06b\xD3\xE1`\xE5\x1B\x85R\x8B`\x04\x86\x01a3\xA7V[\x03\x92Z\xF1\x80\x15a\x08yWa\x0F\x8AW[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x08yWa\x0FvW[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@W`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R\x90_\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x08yWa\x0FhW\0[\x80a\x08m_a\x05s\x93a\x03\xD7V[\x80a\x08m_a\x0F\x84\x93a\x03\xD7V[_a\x0F\x06V[\x80a\x08m_a\x0F\x98\x93a\x03\xD7V[_a\x0E\xA5V[\x92a\x0F\xAA\x94\x91\x94a2&V[\x93a\x0F\xB5\x84Qa2sV[\x94_[\x85Q\x81\x10\x15a\x10\x01W\x80a\x0F\xFBa\x0F\xE2a\x0F\xD4`\x01\x94\x8Aa#WV[QQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x0F\xEC\x83\x8Ba#WV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\x0F\xB8V[P\x91\x94\x95\x90\x92\x95a\x10\x1Fa\x10\x13a\x03\xF8V[c\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[` \x82\x01Ra\x10-\x82a#JV[Ra\x107\x81a#JV[P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92`\x10\x1C\x16\x90\x82;\x15a\x04@Wa\x10\x93\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\x010\xFC'`\xE5\x1B\x84R`\x04\x84\x01a2\x9BV[\x03\x92Z\xF1\x80\x15a\x08yWa\x10\xA8W[\x80a\x0E*V[\x80a\x08m_a\x10\xB6\x93a\x03\xD7V[_a\x10\xA2V[Pa\x10\xD9a\x10\xD5\x87`\xA2T`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[\x15\x90V[a\x0E%V[\x91\x81`\x1F\x84\x01\x12\x15a\x04@W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x04@W` \x83\x81\x86\x01\x95\x01\x01\x11a\x04@WV[4a\x04@W`@6`\x03\x19\x01\x12a\x04@W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04@W6`#\x82\x01\x12\x15a\x04@W\x80`\x04\x015a\x11F\x81a\x04\x18V[\x91a\x11T`@Q\x93\x84a\x03\xD7V[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x04@W`$\x81\x01\x92[\x82\x84\x10a\x11\xA2W`$5\x85`\x01`\x01`@\x1B\x03\x82\x11a\x04@Wa\x11\x9Ca\x05s\x926\x90`\x04\x01a\x10\xDEV[\x91a$\xFAV[\x835`\x01`\x01`@\x1B\x03\x81\x11a\x04@W` \x91a\x11\xC6\x83\x92`$6\x91\x87\x01\x01a\x04DV[\x81R\x01\x93\x01\x92a\x11rV[4a\x04@W`\xA06`\x03\x19\x01\x12a\x04@W`\x045a\x11\xEE\x81a\x04/V[a\x12Y`$5a\x11\xFD\x81a\x04/V[`D5a\x12\t\x81a\x04/V[`d5\x90`\x845\x92a\x12\x1A\x84a\x04/V[_T\x95a\x12?`\xFF`\x08\x89\x90\x1C\x16\x15\x80\x98\x81\x99a\x12\xD3W[\x81\x15a\x12\xB3W[Pa(\x02V[\x86a\x12P`\x01`\xFF\x19_T\x16\x17_UV[a\x12\x9CWa(\xCAV[a\x12_W\0[a\x12ma\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x12\xAEa\x01\0a\xFF\0\x19_T\x16\x17_UV[a(\xCAV[0;\x15\x91P\x81a\x12\xC5W[P_a\x129V[`\xFF\x16`\x01\x14\x90P_a\x12\xBEV[`\x01`\xFF\x82\x16\x10\x91Pa\x122V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x11\x15a\x08~WV[\x90`\x03\x82\x10\x15a\x08~WRV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a\x13)\x81a\x04/V[a\x131a$fV[P`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ a\x13Y`\xFF`\x01`@Q\x93a\x056\x85a\x03\x9CV[`@Q\x80\x91a\x06Q` `@\x84\x01\x92\x80Q\x85R\x01Q` \x84\x01\x90a\x12\xFFV[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x08yWa\x13\xE3\x91_\x91a\t\xA4WPa#\xD6V[a\x05sa/\x87V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W` `\x01`\xFFa\x14\na\n\xBDV[\x16\x1B\x80`\x01T\x16\x14`@Q\x90\x81R\xF3[4a\x04@W`\x806`\x03\x19\x01\x12a\x04@Wa\x143a\n\xBDV[``6`#\x19\x01\x12a\x04@W`@Qa\x14K\x81a\x03\xBCV[`$5a\x14W\x81a\x05\x9FV[\x81R`D5a\xFF\xFF\x81\x16\x81\x03a\x04@W` \x82\x01R`d5a\xFF\xFF\x81\x16\x81\x03a\x04@W`@\x82\x01Ra\x14\x87a//V[`\xFF`\x96T\x16`\xFF\x83\x16\x10\x15a\x14\xA0Wa\x05s\x91a4wV[cs\x10\xCF\xF5`\xE1\x1B_R`\x04_\xFD[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\x01T`@Q\x90\x81R\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x82\x10\x15a\x159W_R` _ \x01\x90_\x90V[a\x15\x10V[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045`\x9CT\x81\x10\x15a\x04@W`\x9C_R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04@W`@6`\x03\x19\x01\x12a\x04@W`\x045a\x15\xF9\x81a\x04/V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x16\x18\x906\x90`\x04\x01a\x072V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x03a\x16\xEAW`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 B\x90U`\x99\x90\x91R\x90 \x80T`\x01a\x16\x80\x81a\x16wa\x16qa\x05Va\x16k`\x96T`\xFF\x16\x90V[\x89a4\x04V[\x94aAgV[\x94\x01T`\xFF\x16\x90V[a\x16\x89\x81a\x12\xF5V[\x14\x91\x82a\x16\xD7W[\x82a\x16\xBEW[PPa\x16\x9FW\0[a\x16\xA9\x81\x83a5\x89V[`\xA1T`\xFF\x16a\x16\xB5W\0[a\x05s\x91a8YV[\x81\x16`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x91\x16\x14\x90P_\x80a\x16\x97V[`\x01`\x01`\xC0\x1B\x03\x82\x16\x15\x15\x92Pa\x16\x91V[cv\xD8\xAB\x17`\xE1\x1B_R`\x04_\xFD[4a\x04@W_6`\x03\x19\x01\x12a\x04@Wa\x17\x11a//V[`d\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\xFF`\xA1T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04@W`\xA06`\x03\x19\x01\x12a\x04@Wa\x17\x906a\x0C\x81V[`d5a\x17\x9C\x81a\x0C\xD1V[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x17\xBB\x906\x90`\x04\x01a\x0C\xE2V[\x90a\x17\xC4a//V[`\x96T`\xFF\x16\x92\x83\x90a\x17\xEF\x90a\x17\xDD`\xC0\x84\x10a,^V[a\x17\xE9a\x0E\x02\x87a2\x14V[\x85a4wV[`\xA1T`\xFF\x81\x16\x80a\x19YW[a\x18bW[PPa\x18\x0C_a+{V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x04@Wa\x0E\x96\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c:\xEA\x0B\x9D`\xE1\x1B\x84R\x8A`\x04\x85\x01a3\xDCV[\x91a\x18o\x94\x91\x93\x94a2&V[\x92a\x18z\x86Qa2sV[\x93_[\x87Q\x81\x10\x15a\x18\xA9W\x80a\x18\xA3a\x18\x99a\x0F\xD4`\x01\x94\x8Ca#WV[a\x0F\xEC\x83\x8Aa#WV[\x01a\x18}V[P\x91\x93\x95\x94\x90\x92\x95a\x18\xBCa\x10\x13a\x03\xF8V[` \x82\x01Ra\x18\xCA\x82a#JV[Ra\x18\xD4\x81a#JV[P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92`\x10\x1C\x16\x90\x82;\x15a\x04@Wa\x190\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\x010\xFC'`\xE5\x1B\x84R`\x04\x84\x01a2\x9BV[\x03\x92Z\xF1\x80\x15a\x08yWa\x19EW[\x80a\x18\x01V[\x80a\x08m_a\x19S\x93a\x03\xD7V[_a\x19?V[Pa\x19ra\x10\xD5\x86`\xA2T`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a\x17\xFCV[\x81`\x1F\x82\x01\x12\x15a\x04@W\x805\x90a\x19\x8E\x82a\x04\x18V[\x92a\x19\x9C`@Q\x94\x85a\x03\xD7V[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x04@W` \x01\x92[\x82\x84\x10a\x19\xC6WPPPP\x90V[`@\x84\x83\x03\x12a\x04@W` `@\x91\x82Qa\x19\xE0\x81a\x03\x9CV[a\x19\xE9\x87a\n\xCDV[\x81R\x82\x87\x015a\x19\xF8\x81a\x04/V[\x83\x82\x01R\x81R\x01\x93\x01\x92a\x19\xB8V[4a\x04@W`\xA06`\x03\x19\x01\x12a\x04@W`\x045a\x1A$\x81a\x04/V[`$5\x90`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@W` \x92a\x1ANa\x1A[\x936\x90`\x04\x01a\x19wV[`d5\x91`\x845\x93a)\x8AV[`@Q\x90\x81R\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W` a\x1A\x81`\x045aAgV[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`dT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\xFF`\x96T\x16`@Q\x90\x81R\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x04@W\x815a\x1B6\x81a\x04\x18V[\x92a\x1BD`@Q\x94\x85a\x03\xD7V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04@W` \x01\x90[\x82\x82\x10a\x1BlWPPP\x90V[` \x80\x91\x835a\x1B{\x81a\x05\x9FV[\x81R\x01\x91\x01\x90a\x1B_V[4a\x04@W`@6`\x03\x19\x01\x12a\x04@W`\x045a\x1B\xA3\x81a\x04/V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@Wa\x1B\xF5a\x1B\xC9a\x05s\x936\x90`\x04\x01a\x1B\x1FV[a\x1B\xD1a9~V[a\x1B\xE2`\x02\x80`\x01T\x16\x14\x15a#4V[a\x1B\xF0`\xFF`\xA1T\x16a$\xE4V[a9\xBFV[\x90a5\x89V[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `@Q\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81R\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W` a\x1C\xA8a\x1C\x97a\n\xBDV[`\xA2T`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\xA0T`@Q\x90\x81R\xF3[4a\x04@W``6`\x03\x19\x01\x12a\x04@W`\x045a\x1C\xEC\x81a\x04/V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x1D\x0B\x906\x90`\x04\x01a\x1B\x1FV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x1Dya\x1D1a\x1D`\x926\x90`\x04\x01a\x10\xDEV[\x92\x90\x94a\x1D<a9~V[a\x1DLa\x04\xEC`\x01\x80\x80T\x16\x14\x90V[a\x1B\xF0a\x1D[`\xA1T`\xFF\x16\x90V[a$\xE4V[\x91\x84\x01a\x1Dm\x81\x86a+8V[\x86\x97\x92\x97\x94\x91\x94a:\xB9V[\x95a\x1D\x83\x81a+{V[\x80a\x1E\xBDWPPPa\x1D\x97\x90\x82\x85\x85a>\xB2V[Q\x92_[\x82Q\x81\x10\x15a\x1E W\x80a\x1E\x1A\x86c\xFF\xFF\xFF\xFFa\x1E\x11a\x06ca\x1E\x07a\x1D\xF5a\x1D\xEB\x88a\x1D\xE5a\x1D\xDFa\x1D\xD1\x8F\x9D`\x01\x9Ea,MV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[\x97a#WV[Qc\xFF\xFF\xFF\xFF\x16\x90V[\x94`\xFF\x16_R`\x97` R`@_ \x90V[Tc\xFF\xFF\xFF\xFF\x16\x90V[\x91\x16\x11\x15a,^V[\x01a\x1D\x9BV[P\x92PP[`\x01a\x1EM\x81a\x1EE\x84`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[\x01T`\xFF\x16\x90V[a\x1EV\x81a\x12\xF5V[\x03a\x1E]W\0[a\x1E\x8Ea\x1Eha\x03\xF8V[\x83\x81R`\x01` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x99` R`@\x90 a,tV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE_\x80\xA3\0[\x80a\x1E\xCC`\x01\x92\x96\x94\x96a+{V[\x03a\x1E\xF1Wa\x1E\xEC\x93a\x1E\xDE\x91a+\x85V[\x95\x93P\x93\x91PP\x86\x86a;\xBBV[a\x1E%V[c5K\xB8\xAB`\xE0\x1B_R`\x04_\xFD[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\xFF`\xA1T`\x08\x1C\x16`@Q\x90\x15\x15\x81R\xF3[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x1FHWPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1F;V[4a\x04@W`@6`\x03\x19\x01\x12a\x04@W`\x045a\x1F\x81\x81a\x05\x9FV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@W6`#\x83\x01\x12\x15a\x04@W\x81`\x04\x015\x91a\x1F\xAD\x83a\x04\x18V[\x92a\x1F\xBB`@Q\x94\x85a\x03\xD7V[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x04@W`$\x01\x90[\x82\x82\x10a\x1F\xF8Wa\x06Qa\x1F\xEC\x86\x86aA\x08V[`@Q\x91\x82\x91\x82a\x1F%V[\x815\x81R` \x91\x82\x01\x91\x01a\x1F\xD8V[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `@Q\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81R\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04@W_6`\x03\x19\x01\x12a\x04@W` `\x9CT`@Q\x90\x81R\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\xFFa \xBEa\n\xBDV[a \xC6a$\x02V[P\x16_R`\x97` Ra\x06Q`@_ a\xFF\xFF`@Q\x91a \xE6\x83a\x03\xBCV[Tc\xFF\xFF\xFF\xFF\x81\x16\x83R\x81\x81` \x1C\x16` \x84\x01R`0\x1C\x16`@\x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91`@a\xFF\xFF\x81``\x84\x01\x95c\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[4a\x04@W_6`\x03\x19\x01\x12a\x04@W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a!\x98\x81a\x04/V[a!\xA0a//V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a!\xB8Wa\x05s\x90a4/V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x08yW_\x91a\"\xCFW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\"\xC0Wa\"\x8E`\x01T\x19\x82\x19\x81\x16\x14a#\xECV[\x80`\x01U`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a\"\xE8\x91P` =` \x11a\x0B\xE1Wa\x0B\xD3\x81\x83a\x03\xD7V[_a\"mV[4a\x04@W` 6`\x03\x19\x01\x12a\x04@W`\x045a#\x0B\x81a\x04/V[`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R` `\xFF`\x01`@_ \x01T\x16a\x0C\x7F`@Q\x80\x92a\x12\xFFV[\x15a#;WV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x80Q\x15a\x159W` \x01\x90V[\x80Q\x82\x10\x15a\x159W` \x91`\x05\x1B\x01\x01\x90V[`\x03\x82\x10\x15a\x08~WRV[\x90a\x04\x07`@Qa#\x87\x81a\x03\x9CV[` `\xFF`\x01\x83\x96\x80T\x85R\x01T\x16\x91\x01a#kV[\x15a#\xA4WV[c\xAB\xA4s9`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x04@WQ\x80\x15\x15\x81\x03a\x04@W\x90V[`@Q=_\x82>=\x90\xFD[\x15a#\xDDWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a#\xF3WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[`@Q\x90a$\x0F\x82a\x03\xBCV[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x90`@Qa$-\x81a\x03\xBCV[`@\x81\x93Tc\xFF\xFF\xFF\xFF\x81\x16\x83Rc\xFF\xFF\xFF\xFF\x81` \x1C\x16` \x84\x01R\x81\x1C\x91\x01RV[\x90\x81` \x91\x03\x12a\x04@WQa\x07M\x81a\x04/V[`@Q\x90a$s\x82a\x03\x9CV[_` \x83\x82\x81R\x01RV[a$\xDFa\x07M\x91a$\x8Da$fV[P`@\x80Q\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6` \x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x81\x83\x01R\x90\x81Ra$\xD7``\x82a\x03\xD7V[Q\x90 a0\xA9V[a0\xF6V[\x15a$\xEBWV[c[w\x90\x19`\xE0\x1B_R`\x04_\xFD[\x90\x92\x91a%\x0Ea\x04\xEC`\x04\x80`\x01T\x16\x14\x90V[a%-a%\x1D`\x96T`\xFF\x16\x90V[a%(6\x84\x88a\x06\xFCV[a4\x04V[Pa%:\x81\x83Q\x14a'=V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x93_[\x82\x81\x10a%vWPPPP\x90PV[a%\x95a\x1D\xDFa%\x87\x83\x86\x86a'SV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[\x92a%\xA0\x82\x86a#WV[Q\x80Q`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x87\x16`\x04\x82\x01R\x91\x97\x91\x90` \x82`$\x81\x8DZ\xFA\x91\x82\x15a\x08yWa%\xE6\x92c\xFF\xFF\xFF\xFF\x91_\x91a'\x0FW[P\x16\x14a'tV[_\x97\x88[\x88Q\x8A\x10\x15a&\xA3W`\x01\x90a&\x9Ba&\x13a&\x06\x8D\x8Da#WV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91a&va&9a&4\x85`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[a#wV[\x91a&aa&\\\x8Da&Na\x05V\x87QaAgV[`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a'\x8AV[\x85\x80`\xA0\x1B\x03\x16\x85\x80`\xA0\x1B\x03\x85\x16\x11a'\xA0V[a&\x94a&\x8Da&\x85\x8Aa'\xCAV[\x8A\x8A\x8Da'\xEAV[6\x91a\x06\xFCV[\x90\x83a.2V[\x99\x01\x98a%\xEAV[P\x96P\x96P\x92\x90`\x01\x91\x94\x92\x94Ca&\xC6\x82`\xFF\x16_R`\x9B` R`@_ \x90V[U\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4`\xFF`@Q\x92\x16\x91\x80a'\0C\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA2\x01\x94\x93\x94\x92\x90\x92a%gV[a'0\x91P` =\x81\x11a'6W[a'(\x81\x83a\x03\xD7V[\x81\x01\x90a'_V[_a%\xDEV[P=a'\x1EV[\x15a'DWV[c\xAA\xAD\x13\xF7`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a\x159W\x01\x90V[\x90\x81` \x91\x03\x12a\x04@WQa\x07M\x81a\x05\x9FV[\x15a'{WV[c\x8EZ\xEE\xE7`\xE0\x1B_R`\x04_\xFD[\x15a'\x91WV[c\xD0S\xAA!`\xE0\x1B_R`\x04_\xFD[\x15a'\xA7WV[c\xBAP\xF9\x11`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a'\xD8WV[a'\xB6V[\x91\x90\x82\x01\x80\x92\x11a'\xD8WV[\x90\x93\x92\x93\x84\x83\x11a\x04@W\x84\x11a\x04@W\x81\x01\x92\x03\x90V[\x15a(\tWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`\x9CT`\x01`@\x1B\x81\x10\x15a\x03\xB7W`\x01\x81\x01`\x9CU`\x9CT\x81\x10\x15a\x159W`\x9C_R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a(\xE3\x94\x93a\t\x9Fa\t\xFB\x94\x93a\x0C\x10a\x0C=\x94a4/V[a)\x15\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a(eV[a)G\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a(eV[a)y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a(eV[a\x01\x01a\xFF\xFF\x19`\xA1T\x16\x17`\xA1UV[\x91\x94\x93\x90\x92`@Q\x92` \x84\x01\x94`\xE0\x85\x01\x91\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87R`\x01\x80`\xA0\x1B\x03\x16`@\x86\x01R``\x85\x01R`\xC0`\x80\x85\x01R\x86Q\x80\x91R` a\x01\0\x85\x01\x97\x01\x90_[\x81\x81\x10a*\x17WPPPa\x07M\x94\x95a$\xD7\x92\x84\x92`\xA0\x84\x01R`\xC0\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xD7V[\x82Q\x80Q`\xFF\x16\x8AR` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x8B\x01R`@\x90\x99\x01\x98\x90\x92\x01\x91`\x01\x01a)\xEAV[5\x90`\x02\x82\x10\x15a\x04@WV[\x91\x90\x82`@\x91\x03\x12a\x04@W`@Qa*j\x81a\x03\x9CV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x04@W`@Q\x91a*\x95`@\x84a\x03\xD7V[\x82\x90`@\x81\x01\x92\x83\x11a\x04@W\x90[\x82\x82\x10a*\xB1WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a*\xA4V[\x80\x82\x03\x92\x91a\x01\0\x84\x12a\x04@W`@Qa*\xDB\x81a\x03\xBCV[`\x80\x81\x95a*\xE9\x84\x86a*RV[\x83Ra*\xF8\x84`@\x87\x01a*RV[` \x84\x01R`\x7F\x19\x01\x12a\x04@Wa+/`@\x92`\xC0\x84Q\x95a+\x1A\x87a\x03\x9CV[a+'\x83`\x80\x83\x01a*zV[\x87R\x01a*zV[` \x84\x01R\x01RV[\x90\x91a\x01@\x82\x84\x03\x12a\x04@Wa+N\x82a*EV[\x92` \x83\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@W`@a+t\x82a\x07M\x94\x87\x01a\x072V[\x94\x01a*\xC1V[`\x02\x11\x15a\x08~WV[\x91\x90a\x01\x80\x83\x82\x03\x12a\x04@Wa+\x9B\x83a*EV[\x92` \x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x04@W\x82a+\xBB\x91\x83\x01a\x072V[\x92a+\xC9\x83`@\x84\x01a*\xC1V[\x92a\x01@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11a\x04@W\x81a+\xEA\x91\x85\x01a\x19wV[\x92a\x01`\x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@W\x01\x90``\x82\x82\x03\x12a\x04@W`@Q\x91a,\x19\x83a\x03\xBCV[\x805`\x01`\x01`@\x1B\x03\x81\x11a\x04@W`@\x92a,7\x91\x83\x01a\x072V[\x83R` \x81\x015` \x84\x01R\x015`@\x82\x01R\x90V[\x90\x81Q\x81\x10\x15a\x159W\x01` \x01\x90V[\x15a,eWV[c<\xB8\x9C\x97`\xE0\x1B_R`\x04_\xFD[`\x01` \x91\x83Q\x81U\x01\x91\x01Q`\x03\x81\x10\x15a\x08~W`\xFF\x80\x19\x83T\x16\x91\x16\x17\x90UV[\x90`@Qa,\xA5\x81a\x03\xBCV[`@a\xFF\xFF\x82\x94Tc\xFF\xFF\xFF\xFF\x81\x16\x84R\x81\x81` \x1C\x16` \x85\x01R`0\x1C\x16\x91\x01RV[`@\x80Q\x90\x91\x90a,\xDB\x83\x82a\x03\xD7V[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a,\xF6\x82a\x06\xE1V[a-\x03`@Q\x91\x82a\x03\xD7V[\x82\x81R\x80\x92a-\x14`\x1F\x19\x91a\x06\xE1V[\x01\x90` 6\x91\x017V[_\x19\x81\x14a'\xD8W`\x01\x01\x90V[_\x81\x80[a-\xA6WPa-B\x90a\xFF\xFF\x16a,\xECV[__[\x82Q\x82\x10\x80a-\x9BW[\x15a-\x94W`\x01\x81\x1B\x84\x16a-mW[a-h\x90a-\x1EV[a-EV[\x90`\x01a-h\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa-\x8A\x82\x87a,MV[S\x01\x91\x90Pa-_V[PP\x90P\x90V[Pa\x01\0\x81\x10a-OV[_\x19\x81\x01\x81\x81\x11a'\xD8Wa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a'\xD8W`\x01\x01\x90\x80a-0V[\x90\x81` \x91\x03\x12a\x04@WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x04@W\x90V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[a\x07M\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a-\xEAV[\x91\x90`\x01` \x82\x01Qa.D\x81a\x12\xF5V[a.M\x81a\x12\xF5V[\x03a/\x03WQ`@Qc3V\x7F\x7F`\xE1\x1B\x81R\x91` \x91\x83\x91\x82\x91a.w\x91\x90\x87`\x04\x85\x01a.\x0EV[\x03\x81_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x90\x81\x15a\x08yW_\x91a.\xD4W[P`\x01`\x01`\xC0\x1B\x03\x16\x90\x81a.\xC8WPPV[a\x1B\xF5a\x04\x07\x92a-,V[a.\xF6\x91P` =` \x11a.\xFCW[a.\xEE\x81\x83a\x03\xD7V[\x81\x01\x90a-\xCBV[_a.\xB4V[P=a.\xE4V[PPPV[\x15a/\x0FWV[c\xBB\xBA`\xCB`\xE0\x1B_R`\x04_\xFD[\x90` a\x07M\x92\x81\x81R\x01\x90a-\xEAV[`dT`\x01`\x01`\xA0\x1B\x03\x163\x03a/CWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[_\x19`\x01U`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`\x01U`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`\x9DUV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`\x9EUV[a0\xB1aA\xABV[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra0\xDC`b\x82a\x03\xD7V[Q\x90 \x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_Q` aJ\xEF_9_Q\x90_R\x90a1\ra$fV[P_\x91\x90\x06` `\xC0\x83[a2\rW_\x93_Q` aJ\xEF_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@Qa1C\x85\x82a\x03\xD7V[\x846\x827\x84\x81\x85`@Qa1W\x82\x82a\x03\xD7V[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aJ\xEF_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15a2\x12Wa1\xC1\x90aG\xAAV[Q\x91a2\rW_Q` aJ\xEF_9_Q\x90_R\x82\x80\t\x14a1\xF8WP_Q` aJ\xEF_9_Q\x90_R`\x01_\x94\x08\x92\x93a1\x18V[\x92\x93PPa2\x04a\x03\xF8V[\x92\x83R\x82\x01R\x90V[a0\xE2V[\xFE[`\xFF`\x01\x91\x16\x01\x90`\xFF\x82\x11a'\xD8WV[`@\x80Q\x90\x91\x90a27\x83\x82a\x03\xD7V[`\x01\x81R\x91`\x1F\x19\x01\x82_[\x82\x81\x10a2OWPPPV[` \x90`@Qa2^\x81a\x03\x9CV[_\x81R``\x83\x82\x01R\x82\x82\x85\x01\x01R\x01a2CV[\x90a2}\x82a\x04\x18V[a2\x8A`@Q\x91\x82a\x03\xD7V[\x82\x81R\x80\x92a-\x14`\x1F\x19\x91a\x04\x18V[\x90`@\x82\x01\x90`\x01\x80`\xA0\x1B\x03\x16\x82R`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x91` ``\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a2\xDCWPPPPP\x90V[\x90\x91\x92\x93\x94`_\x19\x82\x82\x03\x01\x83R\x85Q` ``\x81`@\x85\x01\x93c\xFF\xFF\xFF\xFF\x81Q\x16\x86R\x01Q\x93`@\x83\x82\x01R\x84Q\x80\x94R\x01\x92\x01\x90_\x90[\x80\x82\x10a34WPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a2\xCDV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x90\x91\x01\x90a3\x15V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a3sWPPP\x90V[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x86\x01R`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a3fV[\x90a\x07M\x94\x93`\x01`\x01``\x1B\x03`\x80\x94`\xFFc\xFF\xFF\xFF\xFF\x94\x16\x85R\x16` \x84\x01R\x16`@\x82\x01R\x81``\x82\x01R\x01\x90a3VV[`\x01`\x01``\x1B\x03a\x07M\x94\x93`\xFF``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a3VV[\x90`\x01a4\x12`\xFF\x93aB\xD7V[\x92\x83\x92\x16\x1B\x11\x15a4 W\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[`d\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a55`\xFF\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x92\x16\x92\x83_R`\x97` R`@_ a4\xCDc\xFF\xFF\xFF\xFF\x83Q\x16\x82\x90c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[` \x82\x01Q\x81Te\xFF\xFF\0\0\0\0g\xFF\xFF\0\0\0\0\0\0`@\x86\x01Q`0\x1B\x16\x92` \x1B\x16\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x17\x17\x90U`@Q\x91\x82\x91\x82\x91\x90\x91`@a\xFF\xFF\x81``\x84\x01\x95c\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x82` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[\x03\x90\xA2V[\x15a5AWV[ch\xB6\xA8u`\xE1\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra\x07M\x92\x91\x01\x90a-\xEAV[`@\x90a\x07M\x93\x92\x81R\x81` \x82\x01R\x01\x90a-\xEAV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x99` R`@\x90 \x90a6\x14\x82T\x92a5\xC8`\x01a5\xB9\x81a\x16w\x88aAgV[a5\xC2\x81a\x12\xF5V[\x14a#\x9DV[a5\xE3a\x05Va\x05Va5\xDD`\x96T`\xFF\x16\x90V[\x88a4\x04V[\x90a5\xEF\x82\x15\x15a5:V[a6\x05\x82\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x83\x14a'\x8AV[\x90\x19\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[a6\x1E\x81\x84aCOV[`\x01`\x01`\xC0\x1B\x03\x16\x15a7\x90W[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@W\x83_\x91a6\x82\x93\x83`@Q\x80\x96\x81\x95\x82\x94c\xF4\xE2O\xE5`\xE0\x1B\x84R`\x04\x84\x01a5PV[\x03\x92Z\xF1\x80\x15a\x08yWa7|W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@W_`@Q\x80\x92c\xBD)\xB8\xCD`\xE0\x1B\x82R\x81\x83\x81a6\xE4\x89\x89`\x04\x84\x01a5rV[\x03\x92Z\xF1\x80\x15a\x08yWa7hW[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x04@Wa7I\x92_\x92\x83`@Q\x80\x96\x81\x95\x82\x94c\xBD)\xB8\xCD`\xE0\x1B\x84R`\x04\x84\x01a5rV[\x03\x92Z\xF1\x80\x15a\x08yWa7ZWPV[\x80a\x08m_a\x04\x07\x93a\x03\xD7V[\x80a\x08m_a7v\x93a\x03\xD7V[_a6\xF3V[\x80a\x08m_a7\x8A\x93a\x03\xD7V[_a6\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x99` R`@\x90 a7\xBB\x90`\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UV[\x81`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4_\x80\xA3a6-V[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x84\x01R\x81\x84\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x90\x92\x01Q``\x80\x83\x01R\x80Q`\x80\x83\x01\x81\x90R`\xA0\x90\x92\x01\x92\x01\x90_[\x81\x81\x10a8=WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a80V[\x90\x91a8e\x83Qa2sV[`\xA2T_\x92\x83[\x86Q\x81\x10\x15a8\xD7Wa8\x85a\x1D\xDFa\x1D\xD1\x83\x8Aa,MV[a8\x98\x81\x85`\xFF`\x01\x92\x16\x1C\x16`\x01\x14\x90V[a8\xA6W[P`\x01\x01a8lV[\x94\x90a8\xD1\x82a8\xC6`\xFFa8\xBD`\x01\x96\x93a-\x1EV[\x99\x16\x91\x88a#WV[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x90a8\x9DV[P\x94PP\x90\x80a8\xE6WPPPV[\x81R`\xA1T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92\x91a9M\x91`\x10\x91\x90\x91\x1C\x16a9=a9.a\x04\tV[`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x86RV[`\x01`\x01`\xA0\x1B\x03\x16` \x85\x01RV[`@\x83\x01R\x80;\x15a\x04@W`@Qcn4\x92\xB5`\xE0\x1B\x81R\x91_\x91\x83\x91\x82\x90\x84\x90\x82\x90a7I\x90`\x04\x83\x01a7\xEFV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a9\xB0WV[c#\xD8q\xA5`\xE0\x1B_R`\x04_\xFD[\x90a9\xCA\x82Qa,\xECV[_[\x83Q\x81\x10\x15a:\x03W`\x01\x90`\x01`\x01`\xF8\x1B\x03\x19a9\xEB\x82\x87a#WV[Q`\xF8\x1B\x16_\x1Aa9\xFC\x82\x85a,MV[S\x01a9\xCCV[P\x91PV[\x90\x81` \x91\x03\x12a\x04@WQ\x90V[\x90_\x90[`\x02\x82\x10a:(WPPPV[` \x80`\x01\x92\x85Q\x81R\x01\x93\x01\x91\x01\x90\x91a:\x1BV[a\x01 \x90a:\xA8` `@a\x04\x07\x96\x98\x97\x95\x98a\x01`\x85\x01\x99`\x01\x80`\xA0\x1B\x03\x16\x85Ra:x\x83\x86\x01\x82Q` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\x80\x83\x01Q\x80Q``\x87\x01R` \x01Q`\x80\x86\x01R\x01Qa:\x9C`\xA0\x85\x01\x82Qa:\x17V[\x01Q`\xE0\x83\x01\x90a:\x17V[\x01\x90` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x90\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x83`$\x81\x84Z\xFA\x92\x83\x15a\x08yW_\x93a;\x9AW[P\x82\x15a; WPP\x90P\x90V[` \x92Pa;P\x93_a;2\x84a$~V[`@Qc\x17\xEF9\xCB`\xE3\x1B\x81R\x96\x87\x95\x86\x94\x85\x93\x91`\x04\x85\x01a:>V[\x03\x92Z\xF1\x90\x81\x15a\x08yW_\x91a;kW[P\x80_\x80a-\x94V[a;\x8D\x91P` =` \x11a;\x93W[a;\x85\x81\x83a\x03\xD7V[\x81\x01\x90a:\x08V[_a;bV[P=a;{V[a;\xB4\x91\x93P` =` \x11a;\x93Wa;\x85\x81\x83a\x03\xD7V[\x91_a;\x12V[\x92\x90\x91a;\xE3\x92a;\xDD\x82\x98\x97a;\xD5\x88Q\x85Q\x14a'=V[\x87\x83\x88aDiV[\x84a>\xB2V[\x90_[\x85Q\x81\x10\x15a=\x18W\x80a<\x1Ea<\x19a<\x08a\x1D\xDFa\x1D\xD1`\x01\x96\x8Ca,MV[`\xFF\x16_R`\x97` R`@_ \x90V[a,\x98V[a<,a\x1D\xEB\x83\x87Qa#WV[c\xFF\xFF\xFF\xFFa<Ba\x06c\x84Qc\xFF\xFF\xFF\xFF\x16\x90V[\x91\x16\x11a<QW[P\x01a;\xE6V[a<\xA7\x90a<ea\x1D\xDFa\x1D\xD1\x85\x8Ca,MV[a<\x83a<v\x85`@\x8A\x01Qa#WV[Q`\x01`\x01``\x1B\x03\x16\x90V[\x86a<\x95a<v\x87` \x8C\x01Qa#WV[\x91a<\xA0\x87\x8Ca#WV[Q\x93aEJV[a<\xAFa,\xCAV[a<\xBCa\x1D\xD1\x83\x8Aa,MV[_\x1Aa<\xC7\x82a#JV[Sa<\xEF\x81a<\xEA` a<\xDB\x86\x8Ba#WV[Q\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a5\x89V[`\xA1T`\xFF\x16\x15a<JWa=\x12\x90a=\r` a<\xDB\x85\x8Aa#WV[a8YV[_a<JV[PPPP\x90PV[`@Q\x90a=-\x82a\x03\xBCV[```@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x15a=FWV[c\x13\xCAFW`\xE0\x1B_R`\x04_\xFD[\x15a=\\WV[c\x0Ch\x16\xCD`\xE0\x1B_R`\x04_\xFD[\x15a=rWV[c\x19hg}`\xE1\x1B_R`\x04_\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x04@W\x81Qa=\x98\x81a\x04\x18V[\x92a=\xA6`@Q\x94\x85a\x03\xD7V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04@W` \x01\x90[\x82\x82\x10a=\xCEWPPP\x90V[` \x80\x91\x83Qa=\xDD\x81a\x0C\xD1V[\x81R\x01\x91\x01\x90a=\xC1V[\x91\x90\x91`@\x81\x84\x03\x12a\x04@W\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x04@W\x83a>\x11\x91\x83\x01a=\x81V[\x92` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x04@Wa\x07M\x92\x01a=\x81V[` \x81\x83\x03\x12a\x04@W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04@W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x04@W\x81Qa>b\x81a\x04\x18V[\x92a>p`@Q\x94\x85a\x03\xD7V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04@W` \x01\x90[\x82\x82\x10a>\x98WPPP\x90V[` \x80\x91\x83Qa>\xA7\x81a\x05\x9FV[\x81R\x01\x91\x01\x90a>\x8BV[\x81\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAAa?xa>\xDFa= V[\x96a?la>\xFBa\x05Va>\xF5`\x96T`\xFF\x16\x90V[\x8Aa4\x04V[a?\x04\x86aAgV[`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x90a?\x1B\x82\x15\x15a=?V[`\x01\x80`\xC0\x1B\x03\x16a?5a?0\x82\x84\x16\x15\x90V[a=UV[`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9F` R`@\x90 a?e\x90a?^\x90T`\xA0T\x90a'\xDDV[B\x11a=kV[\x17\x85aCOV[`@Q\x91\x82\x91\x82a/\x1EV[\x03\x90\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04@W_`@Q\x80\x92c\x1F\xD9<\xA9`\xE1\x1B\x82R\x81\x83\x81a?\xCD\x8A\x89`\x04\x84\x01a5PV[\x03\x92Z\xF1\x80\x15a\x08yW\x84\x92_\x92\x85\x92a@\xF4W[Pa@\x01`@Q\x96\x87\x93\x84\x93c%PGw`\xE0\x1B\x85R`\x04\x85\x01a.\x0EV[\x03\x81\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x91\x82\x15a\x08yWa@k\x93_\x93\x84\x91\x85\x91a@\xD0W[P`@\x87\x01R` \x86\x01R`@Q\x93\x84\x92\x83\x92b\xBF\xF0M`\xE0\x1B\x84R`\x04\x84\x01a5rV[\x03\x81\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x90\x81\x15a\x08yW_\x91a@\xAEW[P\x81R\x90V[a@\xCA\x91P=\x80_\x83>a@\xC2\x81\x83a\x03\xD7V[\x81\x01\x90a>/V[_a@\xA8V[\x90Pa@\xEE\x91P=\x80\x86\x83>a@\xE6\x81\x83a\x03\xD7V[\x81\x01\x90a=\xE8V[_a@FV[\x80a\x08m\x85aA\x02\x93a\x03\xD7V[_a?\xE2V[\x91\x90aA\x14\x81Qa2sV[\x90_[\x81Q\x81\x10\x15aARW\x80aA9aA0`\x01\x93\x85a#WV[Q\x87`\x98aF\xA0V[c\xFF\xFF\xFF\xFFaAH\x83\x87a#WV[\x91\x16\x90R\x01aA\x17V[P\x90\x92PPV[_\x19\x81\x01\x91\x90\x82\x11a'\xD8WV[\x80_R`\x98` R`@_ T\x90\x81\x15_\x14aA\x83WPP_\x90V[_R`\x98` R`@_ \x90_\x19\x81\x01\x90\x81\x11a'\xD8WaA\xA3\x91a\x15$V[PT`@\x1C\x90V[0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80aB\x98W[\x15aB\x06W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra0\xDC`\xC0\x82a\x03\xD7V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14aA\xDDV[\x15aB\xC8WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aC@W\x81Q\x15aC;W` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aC6W`\x01\x90aC!aC\x17a\x1D\xDFa\x1D\xD1\x86\x89a,MV[`\xFF`\x01\x91\x16\x1B\x90V[\x90aC-\x81\x83\x11aB\xC1V[\x17\x91\x01\x90aB\xF8V[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x90aCb\x82_R`\x98` R`@_ \x90V[T\x80aC\xAFWPaC~a\x04\x07\x92_R`\x98` R`@_ \x90V[aC\xAAaC\x89a\x04\tV[Cc\xFF\xFF\xFF\xFF\x16\x81R\x92[_` \x85\x01R`\x01`\x01`\xC0\x1B\x03\x16`@\x84\x01RV[aG\xC0V[\x91aC\xDAc\xFF\xFF\xFF\xFF\x93aC\xD4aC\xCE\x84_R`\x98` R`@_ \x90V[\x91aAYV[\x90a\x15$V[P\x90aC\xEA\x82Tc\xFF\xFF\xFF\xFF\x16\x90V[C\x85\x16\x94\x16\x84\x03aD\x15WPa\x04\x07\x92P\x90`\x01`\x01`@\x1B\x03\x82T\x91\x81\x19\x90`@\x1B\x16\x91\x16\x17\x90UV[\x81Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x85\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x91Ua\x04\x07\x92\x91\x90aC\xAA\x90aDQ\x90_R`\x98` R`@_ \x90V[\x91aC\x94aD]a\x04\tV[c\xFF\xFF\xFF\xFF\x90\x95\x16\x85RV[\x91\x92\x90` \x82\x01\x92\x83Q_R`\x9A` R`\xFF`@_ T\x16aD\xE4W`@\x83\x01\x80QB\x11aD\xD5Wa\x04\x07\x95aD\xCD\x93\x86Q_R`\x9A` RaD\xB7`@_ `\x01`\xFF\x19\x82T\x16\x17\x90UV[`\x9DT`\x01`\x01`\xA0\x1B\x03\x16\x96Q\x92Q\x93a)\x8AV[\x90Q\x91aH.V[c\x08\x19\xBD\xCD`\xE0\x1B_R`\x04_\xFD[co\xBE\xFE\xC3`\xE1\x1B_R`\x04_\xFD[\x15aD\xFAWV[cV\x16\x8BA`\xE1\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x04@WQa\x07M\x81a\x0C\xD1V[\x15aE%WV[cLD\x99]`\xE0\x1B_R`\x04_\xFD[\x15aE;WV[c\xB1\x87\xE8i`\xE0\x1B_R`\x04_\xFD[` \x91\x92aE\xA7aE\x9AaE\xD2\x98\x96\x97aE\x93aEo\x87\x83\x01Q`\x01\x80`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x99` R`@\x90 T\x96\x90\x91\x16\x14\x15aD\xF3V[Q`\xFF\x16\x90V[`\xFF\x80\x85\x16\x91\x16\x14a'tV[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x90\x91\x16`$\x82\x01R\x94\x85\x90\x81\x90`D\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\x08yWa\x04\x07\x94_\x94aFVW[P\x82aFN\x92aFIaF5\x93`\x01`\x01``\x1B\x03aFAaF5\x82\x99\x8BaHrV[`\x01`\x01``\x1B\x03\x16\x90V[\x91\x16\x11aE\x1EV[aH\x95V[\x91\x16\x10aE4V[aF5\x91\x94P\x92aFN\x92aFIaF\x8D`\x01`\x01``\x1B\x03\x96` =` \x11aF\x99W[aF\x85\x81\x83a\x03\xD7V[\x81\x01\x90aE\tV[\x96\x93PP\x92P\x92aF\x12V[P=aF{V[\x91\x90\x81_R\x82` R`@_ T\x92_[\x84\x81\x10aGIW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[\x80\x85\x03\x85\x81\x11a'\xD8Wa\x06caG_\x91aAYV[aG\x88aG}\x82aGx\x88\x87\x90_R` R`@_ \x90V[a\x15$V[PTc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x11\x15aG\xA1WP`\x01\x01aF\xB1V[\x94PPPPP\x90V[\x15aG\xB1WV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x03\xB7WaG\xDD\x91`\x01\x82\x01\x81Ua\x15$V[aH\x1BW\x81Q` \x80\x84\x01Q`@\x94\x85\x01Qc\xFF\xFF\xFF\xFF\x90\x93\x16\x91\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x90\x93\x1B\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x90aH9\x92\x91aI\x0CV[\x15aH@WV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a'\xD8WV[aH\x90`\x01`\x01``\x1B\x03\x91a\xFF\xFF` a'\x10\x95\x01Q\x16\x90aHOV[\x16\x04\x90V[aH\x90`\x01`\x01``\x1B\x03\x91a\xFF\xFF`@a'\x10\x95\x01Q\x16\x90aHOV[`\x05\x11\x15a\x08~WV[=\x15aH\xE7W=\x90aH\xCE\x82a\x06\xE1V[\x91aH\xDC`@Q\x93\x84a\x03\xD7V[\x82R=_` \x84\x01>V[``\x90V[\x90\x81` \x91\x03\x12a\x04@WQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x04@W\x90V[\x91\x90\x91aI\x19\x82\x84aI\xD7V[aI\"\x81aH\xB3V[\x15\x90\x81aI\xC1W[PaI\xB9W_\x92aIWaIe\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01a5rV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xD7V[Q\x91Z\xFAaIqaH\xBDV[\x81aI\xADW[\x81aI\x80WP\x90V[\x80Qc\x0B\x13]?`\xE1\x1B\x92P`\x01`\x01`\xE0\x1B\x03\x19\x91aI\xA8\x91\x81\x01` \x90\x81\x01\x91\x01aH\xECV[\x16\x14\x90V[\x80Q` \x14\x91PaIwV[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_aI*V[\x81Q`A\x81\x03aJ\x03WP\x90aI\xFF\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aJEV[\x90\x91V[`@\x03aJ<W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a'\xD8WaI\xFF\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90aJEV[PP_\x90`\x02\x90V[\x92\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11aJ\xE3W`\xFF\x16`\x1B\x81\x14\x15\x80aJ\xD8W[aJ\xCDW` \x93_\x93`@Q\x93\x84\x93`\x80\x85\x01\x93\x85R\x87\x85\x01R`@\x84\x01R``\x83\x01R\x83\x80R\x03\x90`\x01Z\xFA\x15a\x08yW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aJ\xC5W\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x81\x14\x15aJ}V[PPPP_\x90`\x03\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xA0\xF5/\xEBgb\xDFJ\x10j\n\xFC\xB3\xC8\xBD\xDD\xC2\x82\xCE\xA4\x88]\xA5q}\xD9\x83\x81(P$IdsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `AlreadyRegisteredForQuorums()` and selector `0x0c6816cd`.
```solidity
error AlreadyRegisteredForQuorums();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyRegisteredForQuorums {}
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
        impl ::core::convert::From<AlreadyRegisteredForQuorums>
        for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyRegisteredForQuorums) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AlreadyRegisteredForQuorums {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyRegisteredForQuorums {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyRegisteredForQuorums()";
            const SELECTOR: [u8; 4] = [12u8, 104u8, 22u8, 205u8];
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
    /**Custom error with signature `BitmapCannotBeZero()` and selector `0xd16d50ea`.
```solidity
error BitmapCannotBeZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapCannotBeZero {}
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
        impl ::core::convert::From<BitmapCannotBeZero> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapCannotBeZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapCannotBeZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapCannotBeZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapCannotBeZero()";
            const SELECTOR: [u8; 4] = [209u8, 109u8, 80u8, 234u8];
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
    /**Custom error with signature `BitmapEmpty()` and selector `0x13ca4657`.
```solidity
error BitmapEmpty();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapEmpty {}
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
        impl ::core::convert::From<BitmapEmpty> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapEmpty) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapEmpty {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapEmpty {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapEmpty()";
            const SELECTOR: [u8; 4] = [19u8, 202u8, 70u8, 87u8];
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
    /**Custom error with signature `BitmapUpdateIsAfterBlockNumber()` and selector `0x6cb19aff`.
```solidity
error BitmapUpdateIsAfterBlockNumber();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapUpdateIsAfterBlockNumber {}
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
        impl ::core::convert::From<BitmapUpdateIsAfterBlockNumber>
        for UnderlyingRustTuple<'_> {
            fn from(value: BitmapUpdateIsAfterBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for BitmapUpdateIsAfterBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapUpdateIsAfterBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapUpdateIsAfterBlockNumber()";
            const SELECTOR: [u8; 4] = [108u8, 177u8, 154u8, 255u8];
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
    /**Custom error with signature `CannotChurnSelf()` and selector `0xac2d1682`.
```solidity
error CannotChurnSelf();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotChurnSelf {}
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
        impl ::core::convert::From<CannotChurnSelf> for UnderlyingRustTuple<'_> {
            fn from(value: CannotChurnSelf) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotChurnSelf {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotChurnSelf {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotChurnSelf()";
            const SELECTOR: [u8; 4] = [172u8, 45u8, 22u8, 130u8];
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
    /**Custom error with signature `CannotKickOperatorAboveThreshold()` and selector `0xb187e869`.
```solidity
error CannotKickOperatorAboveThreshold();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotKickOperatorAboveThreshold {}
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
        impl ::core::convert::From<CannotKickOperatorAboveThreshold>
        for UnderlyingRustTuple<'_> {
            fn from(value: CannotKickOperatorAboveThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CannotKickOperatorAboveThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotKickOperatorAboveThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotKickOperatorAboveThreshold()";
            const SELECTOR: [u8; 4] = [177u8, 135u8, 232u8, 105u8];
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
    /**Custom error with signature `CannotReregisterYet()` and selector `0x32d0cefa`.
```solidity
error CannotReregisterYet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotReregisterYet {}
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
        impl ::core::convert::From<CannotReregisterYet> for UnderlyingRustTuple<'_> {
            fn from(value: CannotReregisterYet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotReregisterYet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotReregisterYet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotReregisterYet()";
            const SELECTOR: [u8; 4] = [50u8, 208u8, 206u8, 250u8];
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
    /**Custom error with signature `ChurnApproverSaltUsed()` and selector `0xdf7dfd86`.
```solidity
error ChurnApproverSaltUsed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChurnApproverSaltUsed {}
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
        impl ::core::convert::From<ChurnApproverSaltUsed> for UnderlyingRustTuple<'_> {
            fn from(value: ChurnApproverSaltUsed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChurnApproverSaltUsed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ChurnApproverSaltUsed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ChurnApproverSaltUsed()";
            const SELECTOR: [u8; 4] = [223u8, 125u8, 253u8, 134u8];
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
    /**Custom error with signature `InputLengthMismatch()` and selector `0xaaad13f7`.
```solidity
error InputLengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputLengthMismatch {}
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
        impl ::core::convert::From<InputLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputLengthMismatch()";
            const SELECTOR: [u8; 4] = [170u8, 173u8, 19u8, 247u8];
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
    /**Custom error with signature `InsufficientStakeForChurn()` and selector `0x4c44995d`.
```solidity
error InsufficientStakeForChurn();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientStakeForChurn {}
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
        impl ::core::convert::From<InsufficientStakeForChurn>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientStakeForChurn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientStakeForChurn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientStakeForChurn {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientStakeForChurn()";
            const SELECTOR: [u8; 4] = [76u8, 68u8, 153u8, 93u8];
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
    /**Custom error with signature `InvalidRegistrationType()` and selector `0x354bb8ab`.
```solidity
error InvalidRegistrationType();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRegistrationType {}
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
        impl ::core::convert::From<InvalidRegistrationType> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRegistrationType) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRegistrationType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRegistrationType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRegistrationType()";
            const SELECTOR: [u8; 4] = [53u8, 75u8, 184u8, 171u8];
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
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
```solidity
error InvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
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
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
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
    /**Custom error with signature `MaxQuorumsReached()` and selector `0x3cb89c97`.
```solidity
error MaxQuorumsReached();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxQuorumsReached {}
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
        impl ::core::convert::From<MaxQuorumsReached> for UnderlyingRustTuple<'_> {
            fn from(value: MaxQuorumsReached) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MaxQuorumsReached {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxQuorumsReached {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxQuorumsReached()";
            const SELECTOR: [u8; 4] = [60u8, 184u8, 156u8, 151u8];
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
    /**Custom error with signature `NextBitmapUpdateIsBeforeBlockNumber()` and selector `0xbbba60cb`.
```solidity
error NextBitmapUpdateIsBeforeBlockNumber();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NextBitmapUpdateIsBeforeBlockNumber {}
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
        impl ::core::convert::From<NextBitmapUpdateIsBeforeBlockNumber>
        for UnderlyingRustTuple<'_> {
            fn from(value: NextBitmapUpdateIsBeforeBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NextBitmapUpdateIsBeforeBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NextBitmapUpdateIsBeforeBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NextBitmapUpdateIsBeforeBlockNumber()";
            const SELECTOR: [u8; 4] = [187u8, 186u8, 96u8, 203u8];
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
    /**Custom error with signature `NotRegistered()` and selector `0xaba47339`.
```solidity
error NotRegistered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotRegistered {}
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
        impl ::core::convert::From<NotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: NotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotRegistered()";
            const SELECTOR: [u8; 4] = [171u8, 164u8, 115u8, 57u8];
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
    /**Custom error with signature `NotRegisteredForQuorum()` and selector `0xd053aa21`.
```solidity
error NotRegisteredForQuorum();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotRegisteredForQuorum {}
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
        impl ::core::convert::From<NotRegisteredForQuorum> for UnderlyingRustTuple<'_> {
            fn from(value: NotRegisteredForQuorum) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotRegisteredForQuorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotRegisteredForQuorum {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotRegisteredForQuorum()";
            const SELECTOR: [u8; 4] = [208u8, 83u8, 170u8, 33u8];
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
    /**Custom error with signature `NotSorted()` and selector `0xba50f911`.
```solidity
error NotSorted();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotSorted {}
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
        impl ::core::convert::From<NotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotSorted()";
            const SELECTOR: [u8; 4] = [186u8, 80u8, 249u8, 17u8];
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
    /**Custom error with signature `OnlyAllocationManager()` and selector `0x23d871a5`.
```solidity
error OnlyAllocationManager();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyAllocationManager {}
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
        impl ::core::convert::From<OnlyAllocationManager> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyAllocationManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyAllocationManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyAllocationManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyAllocationManager()";
            const SELECTOR: [u8; 4] = [35u8, 216u8, 113u8, 165u8];
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
    /**Custom error with signature `OnlyEjector()` and selector `0xedb1562e`.
```solidity
error OnlyEjector();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEjector {}
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
        impl ::core::convert::From<OnlyEjector> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEjector) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyEjector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEjector {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEjector()";
            const SELECTOR: [u8; 4] = [237u8, 177u8, 86u8, 46u8];
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
    /**Custom error with signature `OperatorSetsNotEnabled()` and selector `0x5b779019`.
```solidity
error OperatorSetsNotEnabled();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSetsNotEnabled {}
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
        impl ::core::convert::From<OperatorSetsNotEnabled> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSetsNotEnabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSetsNotEnabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorSetsNotEnabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorSetsNotEnabled()";
            const SELECTOR: [u8; 4] = [91u8, 119u8, 144u8, 25u8];
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
    /**Custom error with signature `QuorumDoesNotExist()` and selector `0xe6219fea`.
```solidity
error QuorumDoesNotExist();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumDoesNotExist {}
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
        impl ::core::convert::From<QuorumDoesNotExist> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QuorumDoesNotExist()";
            const SELECTOR: [u8; 4] = [230u8, 33u8, 159u8, 234u8];
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
    /**Custom error with signature `QuorumOperatorCountMismatch()` and selector `0x8e5aeee7`.
```solidity
error QuorumOperatorCountMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumOperatorCountMismatch {}
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
        impl ::core::convert::From<QuorumOperatorCountMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: QuorumOperatorCountMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for QuorumOperatorCountMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumOperatorCountMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QuorumOperatorCountMismatch()";
            const SELECTOR: [u8; 4] = [142u8, 90u8, 238u8, 231u8];
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
    /**Custom error with signature `SignatureExpired()` and selector `0x0819bdcd`.
```solidity
error SignatureExpired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureExpired {}
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
        impl ::core::convert::From<SignatureExpired> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureExpired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignatureExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignatureExpired()";
            const SELECTOR: [u8; 4] = [8u8, 25u8, 189u8, 205u8];
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
    /**Event with signature `ChurnApproverUpdated(address,address)` and selector `0x315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c`.
```solidity
event ChurnApproverUpdated(address prevChurnApprover, address newChurnApprover);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChurnApproverUpdated {
        #[allow(missing_docs)]
        pub prevChurnApprover: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newChurnApprover: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ChurnApproverUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ChurnApproverUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                84u8,
                87u8,
                216u8,
                168u8,
                254u8,
                96u8,
                240u8,
                74u8,
                241u8,
                124u8,
                22u8,
                226u8,
                245u8,
                165u8,
                225u8,
                219u8,
                97u8,
                43u8,
                49u8,
                100u8,
                142u8,
                88u8,
                3u8,
                3u8,
                96u8,
                117u8,
                158u8,
                248u8,
                243u8,
                82u8,
                140u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevChurnApprover: data.0,
                    newChurnApprover: data.1,
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
                        &self.prevChurnApprover,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newChurnApprover,
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
        impl alloy_sol_types::private::IntoLogData for ChurnApproverUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChurnApproverUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChurnApproverUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EjectorUpdated(address,address)` and selector `0x8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9`.
```solidity
event EjectorUpdated(address prevEjector, address newEjector);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EjectorUpdated {
        #[allow(missing_docs)]
        pub prevEjector: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newEjector: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for EjectorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EjectorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                143u8,
                48u8,
                171u8,
                9u8,
                244u8,
                58u8,
                108u8,
                21u8,
                125u8,
                127u8,
                206u8,
                126u8,
                10u8,
                19u8,
                192u8,
                3u8,
                4u8,
                44u8,
                28u8,
                149u8,
                232u8,
                167u8,
                46u8,
                122u8,
                20u8,
                106u8,
                33u8,
                192u8,
                202u8,
                162u8,
                77u8,
                201u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevEjector: data.0,
                    newEjector: data.1,
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
                        &self.prevEjector,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newEjector,
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
        impl alloy_sol_types::private::IntoLogData for EjectorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EjectorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EjectorUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
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
    /**Event with signature `OperatorDeregistered(address,bytes32)` and selector `0x396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e4`.
```solidity
event OperatorDeregistered(address indexed operator, bytes32 indexed operatorId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorDeregistered {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorDeregistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                57u8,
                111u8,
                220u8,
                177u8,
                128u8,
                203u8,
                15u8,
                234u8,
                38u8,
                146u8,
                129u8,
                19u8,
                251u8,
                15u8,
                209u8,
                195u8,
                84u8,
                152u8,
                99u8,
                249u8,
                205u8,
                86u8,
                62u8,
                106u8,
                24u8,
                79u8,
                29u8,
                87u8,
                129u8,
                22u8,
                200u8,
                228u8,
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
                    operatorId: topics.2,
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
                    self.operator.clone(),
                    self.operatorId.clone(),
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
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorDeregistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDeregistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorDeregistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,bytes32)` and selector `0xe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe`.
```solidity
event OperatorRegistered(address indexed operator, bytes32 indexed operatorId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorRegistered {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorRegistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8,
                230u8,
                140u8,
                239u8,
                28u8,
                58u8,
                118u8,
                30u8,
                215u8,
                190u8,
                126u8,
                132u8,
                99u8,
                163u8,
                117u8,
                242u8,
                127u8,
                123u8,
                195u8,
                53u8,
                229u8,
                24u8,
                36u8,
                34u8,
                60u8,
                172u8,
                206u8,
                99u8,
                110u8,
                197u8,
                195u8,
                254u8,
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
                    operatorId: topics.2,
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
                    self.operator.clone(),
                    self.operatorId.clone(),
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
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSetParamsUpdated(uint8,(uint32,uint16,uint16))` and selector `0x3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac`.
```solidity
event OperatorSetParamsUpdated(uint8 indexed quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam operatorSetParams);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetParamsUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorSetParamsUpdated {
            type DataTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorSetParam,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "OperatorSetParamsUpdated(uint8,(uint32,uint16,uint16))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                62u8,
                230u8,
                254u8,
                141u8,
                84u8,
                97u8,
                2u8,
                68u8,
                195u8,
                233u8,
                211u8,
                192u8,
                102u8,
                174u8,
                74u8,
                238u8,
                153u8,
                120u8,
                132u8,
                170u8,
                40u8,
                241u8,
                6u8,
                22u8,
                174u8,
                130u8,
                25u8,
                37u8,
                64u8,
                19u8,
                24u8,
                172u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    operatorSetParams: data.0,
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
                    <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSetParamsUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetParamsUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorSetParamsUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSocketUpdate(bytes32,string)` and selector `0xec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa`.
```solidity
event OperatorSocketUpdate(bytes32 indexed operatorId, string socket);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSocketUpdate {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub socket: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for OperatorSocketUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorSocketUpdate(bytes32,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                236u8,
                41u8,
                99u8,
                171u8,
                33u8,
                193u8,
                229u8,
                14u8,
                30u8,
                88u8,
                42u8,
                165u8,
                66u8,
                175u8,
                46u8,
                75u8,
                247u8,
                191u8,
                56u8,
                230u8,
                225u8,
                64u8,
                60u8,
                39u8,
                180u8,
                46u8,
                28u8,
                93u8,
                110u8,
                98u8,
                30u8,
                170u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorId: topics.1,
                    socket: data.0,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.socket,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operatorId.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSocketUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSocketUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSocketUpdate) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `QuorumBlockNumberUpdated(uint8,uint256)` and selector `0x46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db4`.
```solidity
event QuorumBlockNumberUpdated(uint8 indexed quorumNumber, uint256 blocknumber);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumBlockNumberUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub blocknumber: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for QuorumBlockNumberUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "QuorumBlockNumberUpdated(uint8,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                70u8,
                7u8,
                125u8,
                85u8,
                51u8,
                7u8,
                99u8,
                241u8,
                98u8,
                105u8,
                253u8,
                117u8,
                229u8,
                118u8,
                22u8,
                99u8,
                244u8,
                25u8,
                45u8,
                39u8,
                145u8,
                116u8,
                124u8,
                1u8,
                137u8,
                177u8,
                106u8,
                211u8,
                29u8,
                176u8,
                125u8,
                180u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    blocknumber: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.blocknumber),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumBlockNumberUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumBlockNumberUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &QuorumBlockNumberUpdated,
            ) -> alloy_sol_types::private::LogData {
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
constructor(address _stakeRegistry, address _blsApkRegistry, address _indexRegistry, address _socketRegistry, address _allocationManager, address _pauserRegistry);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _stakeRegistry: alloy::sol_types::private::Address,
        pub _blsApkRegistry: alloy::sol_types::private::Address,
        pub _indexRegistry: alloy::sol_types::private::Address,
        pub _socketRegistry: alloy::sol_types::private::Address,
        pub _allocationManager: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                        value._stakeRegistry,
                        value._blsApkRegistry,
                        value._indexRegistry,
                        value._socketRegistry,
                        value._allocationManager,
                        value._pauserRegistry,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _stakeRegistry: tuple.0,
                        _blsApkRegistry: tuple.1,
                        _indexRegistry: tuple.2,
                        _socketRegistry: tuple.3,
                        _allocationManager: tuple.4,
                        _pauserRegistry: tuple.5,
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
                        &self._stakeRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._blsApkRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._indexRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._socketRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                )
            }
        }
    };
    /**Function with signature `OPERATOR_CHURN_APPROVAL_TYPEHASH()` and selector `0xca0de882`.
```solidity
function OPERATOR_CHURN_APPROVAL_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_CHURN_APPROVAL_TYPEHASHCall {}
    ///Container type for the return parameters of the [`OPERATOR_CHURN_APPROVAL_TYPEHASH()`](OPERATOR_CHURN_APPROVAL_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_CHURN_APPROVAL_TYPEHASHReturn {
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
            impl ::core::convert::From<OPERATOR_CHURN_APPROVAL_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_CHURN_APPROVAL_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_CHURN_APPROVAL_TYPEHASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<OPERATOR_CHURN_APPROVAL_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_CHURN_APPROVAL_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_CHURN_APPROVAL_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_CHURN_APPROVAL_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = OPERATOR_CHURN_APPROVAL_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPERATOR_CHURN_APPROVAL_TYPEHASH()";
            const SELECTOR: [u8; 4] = [202u8, 13u8, 232u8, 130u8];
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
    /**Function with signature `PUBKEY_REGISTRATION_TYPEHASH()` and selector `0x9feab859`.
```solidity
function PUBKEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PUBKEY_REGISTRATION_TYPEHASHCall {}
    ///Container type for the return parameters of the [`PUBKEY_REGISTRATION_TYPEHASH()`](PUBKEY_REGISTRATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PUBKEY_REGISTRATION_TYPEHASHReturn {
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
            impl ::core::convert::From<PUBKEY_REGISTRATION_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: PUBKEY_REGISTRATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PUBKEY_REGISTRATION_TYPEHASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<PUBKEY_REGISTRATION_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: PUBKEY_REGISTRATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PUBKEY_REGISTRATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PUBKEY_REGISTRATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = PUBKEY_REGISTRATION_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PUBKEY_REGISTRATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [159u8, 234u8, 184u8, 89u8];
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
    /**Function with signature `accountIdentifier()` and selector `0x0764cb93`.
```solidity
function accountIdentifier() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct accountIdentifierCall {}
    ///Container type for the return parameters of the [`accountIdentifier()`](accountIdentifierCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct accountIdentifierReturn {
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
            impl ::core::convert::From<accountIdentifierCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: accountIdentifierCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for accountIdentifierCall {
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
            impl ::core::convert::From<accountIdentifierReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: accountIdentifierReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for accountIdentifierReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for accountIdentifierCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = accountIdentifierReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "accountIdentifier()";
            const SELECTOR: [u8; 4] = [7u8, 100u8, 203u8, 147u8];
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
    /**Function with signature `calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)` and selector `0x84ca5213`.
```solidity
function calculateOperatorChurnApprovalDigestHash(address registeringOperator, bytes32 registeringOperatorId, ISlashingRegistryCoordinatorTypes.OperatorKickParam[] memory operatorKickParams, bytes32 salt, uint256 expiry) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorChurnApprovalDigestHashCall {
        pub registeringOperator: alloy::sol_types::private::Address,
        pub registeringOperatorId: alloy::sol_types::private::FixedBytes<32>,
        pub operatorKickParams: alloy::sol_types::private::Vec<
            <ISlashingRegistryCoordinatorTypes::OperatorKickParam as alloy::sol_types::SolType>::RustType,
        >,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)`](calculateOperatorChurnApprovalDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorChurnApprovalDigestHashReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    ISlashingRegistryCoordinatorTypes::OperatorKickParam,
                >,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    <ISlashingRegistryCoordinatorTypes::OperatorKickParam as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<calculateOperatorChurnApprovalDigestHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateOperatorChurnApprovalDigestHashCall) -> Self {
                    (
                        value.registeringOperator,
                        value.registeringOperatorId,
                        value.operatorKickParams,
                        value.salt,
                        value.expiry,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateOperatorChurnApprovalDigestHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registeringOperator: tuple.0,
                        registeringOperatorId: tuple.1,
                        operatorKickParams: tuple.2,
                        salt: tuple.3,
                        expiry: tuple.4,
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
            impl ::core::convert::From<calculateOperatorChurnApprovalDigestHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateOperatorChurnApprovalDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateOperatorChurnApprovalDigestHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateOperatorChurnApprovalDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    ISlashingRegistryCoordinatorTypes::OperatorKickParam,
                >,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateOperatorChurnApprovalDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)";
            const SELECTOR: [u8; 4] = [132u8, 202u8, 82u8, 19u8];
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
                        &self.registeringOperator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.registeringOperatorId,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        ISlashingRegistryCoordinatorTypes::OperatorKickParam,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorKickParams),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
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
    /**Function with signature `churnApprover()` and selector `0x054310e6`.
```solidity
function churnApprover() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct churnApproverCall {}
    ///Container type for the return parameters of the [`churnApprover()`](churnApproverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct churnApproverReturn {
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
            impl ::core::convert::From<churnApproverCall> for UnderlyingRustTuple<'_> {
                fn from(value: churnApproverCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for churnApproverCall {
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
            impl ::core::convert::From<churnApproverReturn> for UnderlyingRustTuple<'_> {
                fn from(value: churnApproverReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for churnApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for churnApproverCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = churnApproverReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "churnApprover()";
            const SELECTOR: [u8; 4] = [5u8, 67u8, 16u8, 230u8];
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
    /**Function with signature `createSlashableStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[],uint32)` and selector `0x3eef3a51`.
```solidity
function createSlashableStakeQuorum(ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory strategyParams, uint32 lookAheadPeriod) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSlashableStakeQuorumCall {
        pub operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        pub strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
        pub lookAheadPeriod: u32,
    }
    ///Container type for the return parameters of the [`createSlashableStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[],uint32)`](createSlashableStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSlashableStakeQuorumReturn {}
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
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createSlashableStakeQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createSlashableStakeQuorumCall) -> Self {
                    (
                        value.operatorSetParams,
                        value.minimumStake,
                        value.strategyParams,
                        value.lookAheadPeriod,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createSlashableStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetParams: tuple.0,
                        minimumStake: tuple.1,
                        strategyParams: tuple.2,
                        lookAheadPeriod: tuple.3,
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
            impl ::core::convert::From<createSlashableStakeQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createSlashableStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createSlashableStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createSlashableStakeQuorumCall {
            type Parameters<'a> = (
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createSlashableStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createSlashableStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[],uint32)";
            const SELECTOR: [u8; 4] = [62u8, 239u8, 58u8, 81u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategyParams),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.lookAheadPeriod),
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
    /**Function with signature `createTotalDelegatedStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])` and selector `0x8281ab75`.
```solidity
function createTotalDelegatedStakeQuorum(ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory strategyParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTotalDelegatedStakeQuorumCall {
        pub operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        pub strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createTotalDelegatedStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])`](createTotalDelegatedStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTotalDelegatedStakeQuorumReturn {}
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
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createTotalDelegatedStakeQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createTotalDelegatedStakeQuorumCall) -> Self {
                    (value.operatorSetParams, value.minimumStake, value.strategyParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createTotalDelegatedStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetParams: tuple.0,
                        minimumStake: tuple.1,
                        strategyParams: tuple.2,
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
            impl ::core::convert::From<createTotalDelegatedStakeQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createTotalDelegatedStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createTotalDelegatedStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createTotalDelegatedStakeQuorumCall {
            type Parameters<'a> = (
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createTotalDelegatedStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createTotalDelegatedStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [130u8, 129u8, 171u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategyParams),
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
    /**Function with signature `deregisterOperator(address,uint32[])` and selector `0x9d8e0c23`.
```solidity
function deregisterOperator(address operator, uint32[] memory operatorSetIds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`deregisterOperator(address,uint32[])`](deregisterOperatorCall) function.
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
            impl ::core::convert::From<deregisterOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.operator, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorCall {
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator(address,uint32[])";
            const SELECTOR: [u8; 4] = [157u8, 142u8, 12u8, 35u8];
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
    /**Function with signature `ejectOperator(address,bytes)` and selector `0x6e3b17db`.
```solidity
function ejectOperator(address operator, bytes memory quorumNumbers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`ejectOperator(address,bytes)`](ejectOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectOperatorReturn {}
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
            impl ::core::convert::From<ejectOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: ejectOperatorCall) -> Self {
                    (value.operator, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectOperatorCall {
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
            impl ::core::convert::From<ejectOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ejectOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ejectOperator(address,bytes)";
            const SELECTOR: [u8; 4] = [110u8, 59u8, 23u8, 219u8];
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
    /**Function with signature `ejectionCooldown()` and selector `0xa96f783e`.
```solidity
function ejectionCooldown() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectionCooldownCall {}
    ///Container type for the return parameters of the [`ejectionCooldown()`](ejectionCooldownCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectionCooldownReturn {
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
            impl ::core::convert::From<ejectionCooldownCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ejectionCooldownCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ejectionCooldownCall {
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
            impl ::core::convert::From<ejectionCooldownReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ejectionCooldownReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ejectionCooldownReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectionCooldownCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectionCooldownReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ejectionCooldown()";
            const SELECTOR: [u8; 4] = [169u8, 111u8, 120u8, 62u8];
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
    /**Function with signature `ejector()` and selector `0x28f61b31`.
```solidity
function ejector() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectorCall {}
    ///Container type for the return parameters of the [`ejector()`](ejectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectorReturn {
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
            impl ::core::convert::From<ejectorCall> for UnderlyingRustTuple<'_> {
                fn from(value: ejectorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectorCall {
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
            impl ::core::convert::From<ejectorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ejectorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ejector()";
            const SELECTOR: [u8; 4] = [40u8, 246u8, 27u8, 49u8];
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
    /**Function with signature `getCurrentQuorumBitmap(bytes32)` and selector `0x871ef049`.
```solidity
function getCurrentQuorumBitmap(bytes32 operatorId) external view returns (uint192);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentQuorumBitmapCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getCurrentQuorumBitmap(bytes32)`](getCurrentQuorumBitmapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentQuorumBitmapReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U192,
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
            impl ::core::convert::From<getCurrentQuorumBitmapCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentQuorumBitmapCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentQuorumBitmapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U192,
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
            impl ::core::convert::From<getCurrentQuorumBitmapReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentQuorumBitmapReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentQuorumBitmapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentQuorumBitmapCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentQuorumBitmapReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentQuorumBitmap(bytes32)";
            const SELECTOR: [u8; 4] = [135u8, 30u8, 240u8, 73u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
    /**Function with signature `getOperator(address)` and selector `0x5865c60c`.
```solidity
function getOperator(address operator) external view returns (ISlashingRegistryCoordinatorTypes.OperatorInfo memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperator(address)`](getOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorReturn {
        pub _0: <ISlashingRegistryCoordinatorTypes::OperatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ISlashingRegistryCoordinatorTypes::OperatorInfo,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperator(address)";
            const SELECTOR: [u8; 4] = [88u8, 101u8, 198u8, 12u8];
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
    /**Function with signature `getOperatorFromId(bytes32)` and selector `0x296bb064`.
```solidity
function getOperatorFromId(bytes32 operatorId) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromIdCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getOperatorFromId(bytes32)`](getOperatorFromIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromIdReturn {
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
            impl ::core::convert::From<getOperatorFromIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromIdCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorFromIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorId: tuple.0 }
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
            impl ::core::convert::From<getOperatorFromIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorFromIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorFromIdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorFromIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorFromId(bytes32)";
            const SELECTOR: [u8; 4] = [41u8, 107u8, 176u8, 100u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
    /**Function with signature `getOperatorSetParams(uint8)` and selector `0xe65797ad`.
```solidity
function getOperatorSetParams(uint8 quorumNumber) external view returns (ISlashingRegistryCoordinatorTypes.OperatorSetParam memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetParamsCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getOperatorSetParams(uint8)`](getOperatorSetParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetParamsReturn {
        pub _0: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorSetParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetParamsCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSetParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorSetParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetParamsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSetParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetParamsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSetParamsReturn;
            type ReturnTuple<'a> = (
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSetParams(uint8)";
            const SELECTOR: [u8; 4] = [230u8, 87u8, 151u8, 173u8];
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
    /**Function with signature `getOperatorStatus(address)` and selector `0xfd39105a`.
```solidity
function getOperatorStatus(address operator) external view returns (ISlashingRegistryCoordinatorTypes.OperatorStatus);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStatusCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorStatus(address)`](getOperatorStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStatusReturn {
        pub _0: <ISlashingRegistryCoordinatorTypes::OperatorStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorStatusCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStatusCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ISlashingRegistryCoordinatorTypes::OperatorStatus,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorStatusReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStatusReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorStatusCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorStatusReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorStatus,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorStatus(address)";
            const SELECTOR: [u8; 4] = [253u8, 57u8, 16u8, 90u8];
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
    /**Function with signature `getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)` and selector `0x04ec6351`.
```solidity
function getQuorumBitmapAtBlockNumberByIndex(bytes32 operatorId, uint32 blockNumber, uint256 index) external view returns (uint192);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapAtBlockNumberByIndexCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub blockNumber: u32,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)`](getQuorumBitmapAtBlockNumberByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapAtBlockNumberByIndexReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U192,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getQuorumBitmapAtBlockNumberByIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapAtBlockNumberByIndexCall) -> Self {
                    (value.operatorId, value.blockNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapAtBlockNumberByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        blockNumber: tuple.1,
                        index: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U192,
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
            impl ::core::convert::From<getQuorumBitmapAtBlockNumberByIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapAtBlockNumberByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapAtBlockNumberByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapAtBlockNumberByIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapAtBlockNumberByIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)";
            const SELECTOR: [u8; 4] = [4u8, 236u8, 99u8, 81u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
    /**Function with signature `getQuorumBitmapHistoryLength(bytes32)` and selector `0x03fd3492`.
```solidity
function getQuorumBitmapHistoryLength(bytes32 operatorId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapHistoryLengthCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapHistoryLength(bytes32)`](getQuorumBitmapHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapHistoryLengthReturn {
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
            impl ::core::convert::From<getQuorumBitmapHistoryLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapHistoryLengthCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorId: tuple.0 }
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
            impl ::core::convert::From<getQuorumBitmapHistoryLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapHistoryLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapHistoryLength(bytes32)";
            const SELECTOR: [u8; 4] = [3u8, 253u8, 52u8, 146u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
    /**Function with signature `getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])` and selector `0xc391425e`.
```solidity
function getQuorumBitmapIndicesAtBlockNumber(uint32 blockNumber, bytes32[] memory operatorIds) external view returns (uint32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapIndicesAtBlockNumberCall {
        pub blockNumber: u32,
        pub operatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])`](getQuorumBitmapIndicesAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapIndicesAtBlockNumberReturn {
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
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
            impl ::core::convert::From<getQuorumBitmapIndicesAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapIndicesAtBlockNumberCall) -> Self {
                    (value.blockNumber, value.operatorIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapIndicesAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumber: tuple.0,
                        operatorIds: tuple.1,
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
            impl ::core::convert::From<getQuorumBitmapIndicesAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapIndicesAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapIndicesAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapIndicesAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapIndicesAtBlockNumberReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])";
            const SELECTOR: [u8; 4] = [195u8, 145u8, 66u8, 94u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
    /**Function with signature `getQuorumBitmapUpdateByIndex(bytes32,uint256)` and selector `0x1eb812da`.
```solidity
function getQuorumBitmapUpdateByIndex(bytes32 operatorId, uint256 index) external view returns (ISlashingRegistryCoordinatorTypes.QuorumBitmapUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapUpdateByIndexCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapUpdateByIndex(bytes32,uint256)`](getQuorumBitmapUpdateByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapUpdateByIndexReturn {
        pub _0: <ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getQuorumBitmapUpdateByIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapUpdateByIndexCall) -> Self {
                    (value.operatorId, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapUpdateByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getQuorumBitmapUpdateByIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapUpdateByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapUpdateByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapUpdateByIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapUpdateByIndexReturn;
            type ReturnTuple<'a> = (
                ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapUpdateByIndex(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [30u8, 184u8, 18u8, 218u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
    /**Function with signature `indexRegistry()` and selector `0x9e9923c2`.
```solidity
function indexRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct indexRegistryCall {}
    ///Container type for the return parameters of the [`indexRegistry()`](indexRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct indexRegistryReturn {
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
            impl ::core::convert::From<indexRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: indexRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for indexRegistryCall {
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
            impl ::core::convert::From<indexRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: indexRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for indexRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for indexRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = indexRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "indexRegistry()";
            const SELECTOR: [u8; 4] = [158u8, 153u8, 35u8, 194u8];
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
    /**Function with signature `initialize(address,address,address,uint256,address)` and selector `0x530b97a4`.
```solidity
function initialize(address _initialOwner, address _churnApprover, address _ejector, uint256 _initialPausedStatus, address _accountIdentifier) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _initialOwner: alloy::sol_types::private::Address,
        pub _churnApprover: alloy::sol_types::private::Address,
        pub _ejector: alloy::sol_types::private::Address,
        pub _initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        pub _accountIdentifier: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address,address,uint256,address)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
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
                        value._initialOwner,
                        value._churnApprover,
                        value._ejector,
                        value._initialPausedStatus,
                        value._accountIdentifier,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _initialOwner: tuple.0,
                        _churnApprover: tuple.1,
                        _ejector: tuple.2,
                        _initialPausedStatus: tuple.3,
                        _accountIdentifier: tuple.4,
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
                alloy::sol_types::sol_data::Uint<256>,
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
            const SIGNATURE: &'static str = "initialize(address,address,address,uint256,address)";
            const SELECTOR: [u8; 4] = [83u8, 11u8, 151u8, 164u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._churnApprover,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._ejector,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._initialPausedStatus),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._accountIdentifier,
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
    /**Function with signature `isChurnApproverSaltUsed(bytes32)` and selector `0x1478851f`.
```solidity
function isChurnApproverSaltUsed(bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChurnApproverSaltUsedCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isChurnApproverSaltUsed(bytes32)`](isChurnApproverSaltUsedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChurnApproverSaltUsedReturn {
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
            impl ::core::convert::From<isChurnApproverSaltUsedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isChurnApproverSaltUsedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isChurnApproverSaltUsedCall {
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
            impl ::core::convert::From<isChurnApproverSaltUsedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isChurnApproverSaltUsedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isChurnApproverSaltUsedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isChurnApproverSaltUsedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isChurnApproverSaltUsedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isChurnApproverSaltUsed(bytes32)";
            const SELECTOR: [u8; 4] = [20u8, 120u8, 133u8, 31u8];
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
    /**Function with signature `isM2Quorum(uint8)` and selector `0xa4d7871f`.
```solidity
function isM2Quorum(uint8 quorumNumber) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isM2QuorumCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`isM2Quorum(uint8)`](isM2QuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isM2QuorumReturn {
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
            impl ::core::convert::From<isM2QuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: isM2QuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isM2QuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
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
            impl ::core::convert::From<isM2QuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isM2QuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isM2QuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isM2QuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isM2QuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isM2Quorum(uint8)";
            const SELECTOR: [u8; 4] = [164u8, 215u8, 135u8, 31u8];
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
    /**Function with signature `lastEjectionTimestamp(address)` and selector `0x125e0584`.
```solidity
function lastEjectionTimestamp(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastEjectionTimestampCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`lastEjectionTimestamp(address)`](lastEjectionTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastEjectionTimestampReturn {
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
            impl ::core::convert::From<lastEjectionTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastEjectionTimestampCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastEjectionTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<lastEjectionTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastEjectionTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastEjectionTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastEjectionTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastEjectionTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastEjectionTimestamp(address)";
            const SELECTOR: [u8; 4] = [18u8, 94u8, 5u8, 132u8];
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
    /**Function with signature `m2QuorumsDisabled()` and selector `0xb2d8678d`.
```solidity
function m2QuorumsDisabled() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct m2QuorumsDisabledCall {}
    ///Container type for the return parameters of the [`m2QuorumsDisabled()`](m2QuorumsDisabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct m2QuorumsDisabledReturn {
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
            impl ::core::convert::From<m2QuorumsDisabledCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: m2QuorumsDisabledCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for m2QuorumsDisabledCall {
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
            impl ::core::convert::From<m2QuorumsDisabledReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: m2QuorumsDisabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for m2QuorumsDisabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for m2QuorumsDisabledCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = m2QuorumsDisabledReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "m2QuorumsDisabled()";
            const SELECTOR: [u8; 4] = [178u8, 216u8, 103u8, 141u8];
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
    /**Function with signature `numRegistries()` and selector `0xd72d8dd6`.
```solidity
function numRegistries() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numRegistriesCall {}
    ///Container type for the return parameters of the [`numRegistries()`](numRegistriesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numRegistriesReturn {
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
            impl ::core::convert::From<numRegistriesCall> for UnderlyingRustTuple<'_> {
                fn from(value: numRegistriesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numRegistriesCall {
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
            impl ::core::convert::From<numRegistriesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: numRegistriesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numRegistriesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for numRegistriesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = numRegistriesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "numRegistries()";
            const SELECTOR: [u8; 4] = [215u8, 45u8, 141u8, 214u8];
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
    /**Function with signature `operatorSetsEnabled()` and selector `0x81f936d2`.
```solidity
function operatorSetsEnabled() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetsEnabledCall {}
    ///Container type for the return parameters of the [`operatorSetsEnabled()`](operatorSetsEnabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetsEnabledReturn {
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
            impl ::core::convert::From<operatorSetsEnabledCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetsEnabledCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSetsEnabledCall {
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
            impl ::core::convert::From<operatorSetsEnabledReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetsEnabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSetsEnabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorSetsEnabledCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSetsEnabledReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorSetsEnabled()";
            const SELECTOR: [u8; 4] = [129u8, 249u8, 54u8, 210u8];
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
        pub index: u8,
    }
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
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
    /**Function with signature `pubkeyRegistrationMessageHash(address)` and selector `0x3c2a7f4c`.
```solidity
function pubkeyRegistrationMessageHash(address operator) external view returns (BN254.G1Point memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyRegistrationMessageHashCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`pubkeyRegistrationMessageHash(address)`](pubkeyRegistrationMessageHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyRegistrationMessageHashReturn {
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
            impl ::core::convert::From<pubkeyRegistrationMessageHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyRegistrationMessageHashCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pubkeyRegistrationMessageHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<pubkeyRegistrationMessageHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyRegistrationMessageHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pubkeyRegistrationMessageHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pubkeyRegistrationMessageHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyRegistrationMessageHashReturn;
            type ReturnTuple<'a> = (BN254::G1Point,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pubkeyRegistrationMessageHash(address)";
            const SELECTOR: [u8; 4] = [60u8, 42u8, 127u8, 76u8];
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
    /**Function with signature `quorumCount()` and selector `0x9aa1653d`.
```solidity
function quorumCount() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCountCall {}
    ///Container type for the return parameters of the [`quorumCount()`](quorumCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCountReturn {
        pub _0: u8,
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
            impl ::core::convert::From<quorumCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<quorumCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorumCount()";
            const SELECTOR: [u8; 4] = [154u8, 161u8, 101u8, 61u8];
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
    /**Function with signature `quorumUpdateBlockNumber(uint8)` and selector `0x249a0c42`.
```solidity
function quorumUpdateBlockNumber(uint8) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumUpdateBlockNumberCall {
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`quorumUpdateBlockNumber(uint8)`](quorumUpdateBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumUpdateBlockNumberReturn {
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
            impl ::core::convert::From<quorumUpdateBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: quorumUpdateBlockNumberCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quorumUpdateBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<quorumUpdateBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: quorumUpdateBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quorumUpdateBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumUpdateBlockNumberCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumUpdateBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorumUpdateBlockNumber(uint8)";
            const SELECTOR: [u8; 4] = [36u8, 154u8, 12u8, 66u8];
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
    /**Function with signature `registerOperator(address,uint32[],bytes)` and selector `0xadcf73f7`.
```solidity
function registerOperator(address operator, uint32[] memory operatorSetIds, bytes memory data) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(address,uint32[],bytes)`](registerOperatorCall) function.
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
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
                    (value.operator, value.operatorSetIds, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSetIds: tuple.1,
                        data: tuple.2,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
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
            const SIGNATURE: &'static str = "registerOperator(address,uint32[],bytes)";
            const SELECTOR: [u8; 4] = [173u8, 207u8, 115u8, 247u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
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
    /**Function with signature `registries(uint256)` and selector `0x6347c900`.
```solidity
function registries(uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registriesCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`registries(uint256)`](registriesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registriesReturn {
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
            impl ::core::convert::From<registriesCall> for UnderlyingRustTuple<'_> {
                fn from(value: registriesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registriesCall {
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
            impl ::core::convert::From<registriesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registriesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registriesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registriesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registriesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registries(uint256)";
            const SELECTOR: [u8; 4] = [99u8, 71u8, 201u8, 0u8];
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
    /**Function with signature `setAccountIdentifier(address)` and selector `0x143e5915`.
```solidity
function setAccountIdentifier(address _accountIdentifier) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAccountIdentifierCall {
        pub _accountIdentifier: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setAccountIdentifier(address)`](setAccountIdentifierCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAccountIdentifierReturn {}
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
            impl ::core::convert::From<setAccountIdentifierCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAccountIdentifierCall) -> Self {
                    (value._accountIdentifier,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAccountIdentifierCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _accountIdentifier: tuple.0,
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
            impl ::core::convert::From<setAccountIdentifierReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAccountIdentifierReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAccountIdentifierReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAccountIdentifierCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAccountIdentifierReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAccountIdentifier(address)";
            const SELECTOR: [u8; 4] = [20u8, 62u8, 89u8, 21u8];
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
                        &self._accountIdentifier,
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
    /**Function with signature `setChurnApprover(address)` and selector `0x29d1e0c3`.
```solidity
function setChurnApprover(address _churnApprover) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setChurnApproverCall {
        pub _churnApprover: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setChurnApprover(address)`](setChurnApproverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setChurnApproverReturn {}
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
            impl ::core::convert::From<setChurnApproverCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setChurnApproverCall) -> Self {
                    (value._churnApprover,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setChurnApproverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _churnApprover: tuple.0 }
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
            impl ::core::convert::From<setChurnApproverReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setChurnApproverReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setChurnApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setChurnApproverCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setChurnApproverReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setChurnApprover(address)";
            const SELECTOR: [u8; 4] = [41u8, 209u8, 224u8, 195u8];
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
                        &self._churnApprover,
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
    /**Function with signature `setEjectionCooldown(uint256)` and selector `0x0d3f2134`.
```solidity
function setEjectionCooldown(uint256 _ejectionCooldown) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectionCooldownCall {
        pub _ejectionCooldown: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setEjectionCooldown(uint256)`](setEjectionCooldownCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectionCooldownReturn {}
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
            impl ::core::convert::From<setEjectionCooldownCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setEjectionCooldownCall) -> Self {
                    (value._ejectionCooldown,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setEjectionCooldownCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _ejectionCooldown: tuple.0 }
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
            impl ::core::convert::From<setEjectionCooldownReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setEjectionCooldownReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setEjectionCooldownReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEjectionCooldownCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEjectionCooldownReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setEjectionCooldown(uint256)";
            const SELECTOR: [u8; 4] = [13u8, 63u8, 33u8, 52u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._ejectionCooldown),
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
    /**Function with signature `setEjector(address)` and selector `0x2cdd1e86`.
```solidity
function setEjector(address _ejector) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectorCall {
        pub _ejector: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setEjector(address)`](setEjectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectorReturn {}
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
            impl ::core::convert::From<setEjectorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectorCall) -> Self {
                    (value._ejector,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _ejector: tuple.0 }
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
            impl ::core::convert::From<setEjectorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEjectorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEjectorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setEjector(address)";
            const SELECTOR: [u8; 4] = [44u8, 221u8, 30u8, 134u8];
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
                        &self._ejector,
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
    /**Function with signature `setOperatorSetParams(uint8,(uint32,uint16,uint16))` and selector `0x5b0b829f`.
```solidity
function setOperatorSetParams(uint8 quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorSetParamsCall {
        pub quorumNumber: u8,
        pub operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setOperatorSetParams(uint8,(uint32,uint16,uint16))`](setOperatorSetParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorSetParamsReturn {}
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
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<setOperatorSetParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorSetParamsCall) -> Self {
                    (value.quorumNumber, value.operatorSetParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOperatorSetParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operatorSetParams: tuple.1,
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
            impl ::core::convert::From<setOperatorSetParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorSetParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOperatorSetParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorSetParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorSetParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOperatorSetParams(uint8,(uint32,uint16,uint16))";
            const SELECTOR: [u8; 4] = [91u8, 11u8, 130u8, 159u8];
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
                    <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
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
    /**Function with signature `socketRegistry()` and selector `0xea32afae`.
```solidity
function socketRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct socketRegistryCall {}
    ///Container type for the return parameters of the [`socketRegistry()`](socketRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct socketRegistryReturn {
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
            impl ::core::convert::From<socketRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: socketRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for socketRegistryCall {
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
            impl ::core::convert::From<socketRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: socketRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for socketRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for socketRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = socketRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "socketRegistry()";
            const SELECTOR: [u8; 4] = [234u8, 50u8, 175u8, 174u8];
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
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
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
    /**Function with signature `unpause(uint256)` and selector `0xfabc1cbc`.
```solidity
function unpause(uint256 newPausedStatus) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {
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
    /**Function with signature `updateOperators(address[])` and selector `0x00cf2ab5`.
```solidity
function updateOperators(address[] memory operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsCall {
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateOperators(address[])`](updateOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsReturn {}
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
            impl ::core::convert::From<updateOperatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsCall) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
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
            impl ::core::convert::From<updateOperatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperators(address[])";
            const SELECTOR: [u8; 4] = [0u8, 207u8, 42u8, 181u8];
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
    /**Function with signature `updateOperatorsForQuorum(address[][],bytes)` and selector `0x5140a548`.
```solidity
function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory quorumNumbers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumCall {
        pub operatorsPerQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorsForQuorum(address[][],bytes)`](updateOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumReturn {}
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
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
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
            impl ::core::convert::From<updateOperatorsForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumCall) -> Self {
                    (value.operatorsPerQuorum, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorsPerQuorum: tuple.0,
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
            impl ::core::convert::From<updateOperatorsForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorsForQuorum(address[][],bytes)";
            const SELECTOR: [u8; 4] = [81u8, 64u8, 165u8, 72u8];
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
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Address,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorsPerQuorum),
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
    /**Function with signature `updateSocket(string)` and selector `0x0cf4b767`.
```solidity
function updateSocket(string memory socket) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSocketCall {
        pub socket: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateSocket(string)`](updateSocketCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSocketReturn {}
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
            impl ::core::convert::From<updateSocketCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateSocketCall) -> Self {
                    (value.socket,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateSocketCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { socket: tuple.0 }
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
            impl ::core::convert::From<updateSocketReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateSocketReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateSocketReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateSocketCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateSocketReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateSocket(string)";
            const SELECTOR: [u8; 4] = [12u8, 244u8, 183u8, 103u8];
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
                        &self.socket,
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
    ///Container for all the [`SlashingRegistryCoordinator`](self) function calls.
    pub enum SlashingRegistryCoordinatorCalls {
        OPERATOR_CHURN_APPROVAL_TYPEHASH(OPERATOR_CHURN_APPROVAL_TYPEHASHCall),
        PUBKEY_REGISTRATION_TYPEHASH(PUBKEY_REGISTRATION_TYPEHASHCall),
        accountIdentifier(accountIdentifierCall),
        allocationManager(allocationManagerCall),
        blsApkRegistry(blsApkRegistryCall),
        calculateOperatorChurnApprovalDigestHash(
            calculateOperatorChurnApprovalDigestHashCall,
        ),
        churnApprover(churnApproverCall),
        createSlashableStakeQuorum(createSlashableStakeQuorumCall),
        createTotalDelegatedStakeQuorum(createTotalDelegatedStakeQuorumCall),
        deregisterOperator(deregisterOperatorCall),
        ejectOperator(ejectOperatorCall),
        ejectionCooldown(ejectionCooldownCall),
        ejector(ejectorCall),
        getCurrentQuorumBitmap(getCurrentQuorumBitmapCall),
        getOperator(getOperatorCall),
        getOperatorFromId(getOperatorFromIdCall),
        getOperatorId(getOperatorIdCall),
        getOperatorSetParams(getOperatorSetParamsCall),
        getOperatorStatus(getOperatorStatusCall),
        getQuorumBitmapAtBlockNumberByIndex(getQuorumBitmapAtBlockNumberByIndexCall),
        getQuorumBitmapHistoryLength(getQuorumBitmapHistoryLengthCall),
        getQuorumBitmapIndicesAtBlockNumber(getQuorumBitmapIndicesAtBlockNumberCall),
        getQuorumBitmapUpdateByIndex(getQuorumBitmapUpdateByIndexCall),
        indexRegistry(indexRegistryCall),
        initialize(initializeCall),
        isChurnApproverSaltUsed(isChurnApproverSaltUsedCall),
        isM2Quorum(isM2QuorumCall),
        lastEjectionTimestamp(lastEjectionTimestampCall),
        m2QuorumsDisabled(m2QuorumsDisabledCall),
        numRegistries(numRegistriesCall),
        operatorSetsEnabled(operatorSetsEnabledCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        pubkeyRegistrationMessageHash(pubkeyRegistrationMessageHashCall),
        quorumCount(quorumCountCall),
        quorumUpdateBlockNumber(quorumUpdateBlockNumberCall),
        registerOperator(registerOperatorCall),
        registries(registriesCall),
        renounceOwnership(renounceOwnershipCall),
        setAccountIdentifier(setAccountIdentifierCall),
        setChurnApprover(setChurnApproverCall),
        setEjectionCooldown(setEjectionCooldownCall),
        setEjector(setEjectorCall),
        setOperatorSetParams(setOperatorSetParamsCall),
        socketRegistry(socketRegistryCall),
        stakeRegistry(stakeRegistryCall),
        transferOwnership(transferOwnershipCall),
        unpause(unpauseCall),
        updateOperators(updateOperatorsCall),
        updateOperatorsForQuorum(updateOperatorsForQuorumCall),
        updateSocket(updateSocketCall),
    }
    #[automatically_derived]
    impl SlashingRegistryCoordinatorCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 207u8, 42u8, 181u8],
            [3u8, 253u8, 52u8, 146u8],
            [4u8, 236u8, 99u8, 81u8],
            [5u8, 67u8, 16u8, 230u8],
            [7u8, 100u8, 203u8, 147u8],
            [12u8, 244u8, 183u8, 103u8],
            [13u8, 63u8, 33u8, 52u8],
            [18u8, 94u8, 5u8, 132u8],
            [19u8, 84u8, 42u8, 78u8],
            [19u8, 100u8, 57u8, 221u8],
            [20u8, 62u8, 89u8, 21u8],
            [20u8, 120u8, 133u8, 31u8],
            [30u8, 184u8, 18u8, 218u8],
            [36u8, 154u8, 12u8, 66u8],
            [40u8, 246u8, 27u8, 49u8],
            [41u8, 107u8, 176u8, 100u8],
            [41u8, 209u8, 224u8, 195u8],
            [44u8, 221u8, 30u8, 134u8],
            [60u8, 42u8, 127u8, 76u8],
            [62u8, 239u8, 58u8, 81u8],
            [81u8, 64u8, 165u8, 72u8],
            [83u8, 11u8, 151u8, 164u8],
            [88u8, 101u8, 198u8, 12u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [91u8, 11u8, 130u8, 159u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 244u8, 89u8, 70u8],
            [99u8, 71u8, 201u8, 0u8],
            [104u8, 48u8, 72u8, 53u8],
            [110u8, 59u8, 23u8, 219u8],
            [113u8, 80u8, 24u8, 166u8],
            [129u8, 249u8, 54u8, 210u8],
            [130u8, 129u8, 171u8, 117u8],
            [132u8, 202u8, 82u8, 19u8],
            [135u8, 30u8, 240u8, 73u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [154u8, 161u8, 101u8, 61u8],
            [157u8, 142u8, 12u8, 35u8],
            [158u8, 153u8, 35u8, 194u8],
            [159u8, 234u8, 184u8, 89u8],
            [164u8, 215u8, 135u8, 31u8],
            [169u8, 111u8, 120u8, 62u8],
            [173u8, 207u8, 115u8, 247u8],
            [178u8, 216u8, 103u8, 141u8],
            [195u8, 145u8, 66u8, 94u8],
            [202u8, 13u8, 232u8, 130u8],
            [202u8, 138u8, 167u8, 199u8],
            [215u8, 45u8, 141u8, 214u8],
            [230u8, 87u8, 151u8, 173u8],
            [234u8, 50u8, 175u8, 174u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 188u8, 28u8, 188u8],
            [253u8, 57u8, 16u8, 90u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SlashingRegistryCoordinatorCalls {
        const NAME: &'static str = "SlashingRegistryCoordinatorCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 55usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(_) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(_) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::accountIdentifier(_) => {
                    <accountIdentifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorChurnApprovalDigestHash(_) => {
                    <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::churnApprover(_) => {
                    <churnApproverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createSlashableStakeQuorum(_) => {
                    <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createTotalDelegatedStakeQuorum(_) => {
                    <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ejectOperator(_) => {
                    <ejectOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ejectionCooldown(_) => {
                    <ejectionCooldownCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ejector(_) => <ejectorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getCurrentQuorumBitmap(_) => {
                    <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperator(_) => {
                    <getOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorFromId(_) => {
                    <getOperatorFromIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorId(_) => {
                    <getOperatorIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSetParams(_) => {
                    <getOperatorSetParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorStatus(_) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapAtBlockNumberByIndex(_) => {
                    <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapHistoryLength(_) => {
                    <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapIndicesAtBlockNumber(_) => {
                    <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapUpdateByIndex(_) => {
                    <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::indexRegistry(_) => {
                    <indexRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isChurnApproverSaltUsed(_) => {
                    <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isM2Quorum(_) => {
                    <isM2QuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastEjectionTimestamp(_) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::m2QuorumsDisabled(_) => {
                    <m2QuorumsDisabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::numRegistries(_) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorSetsEnabled(_) => {
                    <operatorSetsEnabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pubkeyRegistrationMessageHash(_) => {
                    <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::quorumCount(_) => {
                    <quorumCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::quorumUpdateBlockNumber(_) => {
                    <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator(_) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registries(_) => {
                    <registriesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAccountIdentifier(_) => {
                    <setAccountIdentifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setChurnApprover(_) => {
                    <setChurnApproverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setEjectionCooldown(_) => {
                    <setEjectionCooldownCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setEjector(_) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorSetParams(_) => {
                    <setOperatorSetParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::socketRegistry(_) => {
                    <socketRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateOperators(_) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorsForQuorum(_) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateSocket(_) => {
                    <updateSocketCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>] = &[
                {
                    fn updateOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <updateOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::updateOperators)
                    }
                    updateOperators
                },
                {
                    fn getQuorumBitmapHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::getQuorumBitmapHistoryLength,
                            )
                    }
                    getQuorumBitmapHistoryLength
                },
                {
                    fn getQuorumBitmapAtBlockNumberByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::getQuorumBitmapAtBlockNumberByIndex,
                            )
                    }
                    getQuorumBitmapAtBlockNumberByIndex
                },
                {
                    fn churnApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <churnApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::churnApprover)
                    }
                    churnApprover
                },
                {
                    fn accountIdentifier(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <accountIdentifierCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::accountIdentifier)
                    }
                    accountIdentifier
                },
                {
                    fn updateSocket(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <updateSocketCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::updateSocket)
                    }
                    updateSocket
                },
                {
                    fn setEjectionCooldown(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <setEjectionCooldownCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::setEjectionCooldown)
                    }
                    setEjectionCooldown
                },
                {
                    fn lastEjectionTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::lastEjectionTimestamp)
                    }
                    lastEjectionTimestamp
                },
                {
                    fn getOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <getOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::getOperatorId)
                    }
                    getOperatorId
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::pause)
                    }
                    pause
                },
                {
                    fn setAccountIdentifier(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <setAccountIdentifierCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::setAccountIdentifier)
                    }
                    setAccountIdentifier
                },
                {
                    fn isChurnApproverSaltUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::isChurnApproverSaltUsed,
                            )
                    }
                    isChurnApproverSaltUsed
                },
                {
                    fn getQuorumBitmapUpdateByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::getQuorumBitmapUpdateByIndex,
                            )
                    }
                    getQuorumBitmapUpdateByIndex
                },
                {
                    fn quorumUpdateBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::quorumUpdateBlockNumber,
                            )
                    }
                    quorumUpdateBlockNumber
                },
                {
                    fn ejector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <ejectorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::ejector)
                    }
                    ejector
                },
                {
                    fn getOperatorFromId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <getOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::getOperatorFromId)
                    }
                    getOperatorFromId
                },
                {
                    fn setChurnApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <setChurnApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::setChurnApprover)
                    }
                    setChurnApprover
                },
                {
                    fn setEjector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <setEjectorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::setEjector)
                    }
                    setEjector
                },
                {
                    fn pubkeyRegistrationMessageHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::pubkeyRegistrationMessageHash,
                            )
                    }
                    pubkeyRegistrationMessageHash
                },
                {
                    fn createSlashableStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::createSlashableStakeQuorum,
                            )
                    }
                    createSlashableStakeQuorum
                },
                {
                    fn updateOperatorsForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::updateOperatorsForQuorum,
                            )
                    }
                    updateOperatorsForQuorum
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <getOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::getOperator)
                    }
                    getOperator
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn setOperatorSetParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <setOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::setOperatorSetParams)
                    }
                    setOperatorSetParams
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn registries(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <registriesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::registries)
                    }
                    registries
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn ejectOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <ejectOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::ejectOperator)
                    }
                    ejectOperator
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn operatorSetsEnabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <operatorSetsEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::operatorSetsEnabled)
                    }
                    operatorSetsEnabled
                },
                {
                    fn createTotalDelegatedStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::createTotalDelegatedStakeQuorum,
                            )
                    }
                    createTotalDelegatedStakeQuorum
                },
                {
                    fn calculateOperatorChurnApprovalDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::calculateOperatorChurnApprovalDigestHash,
                            )
                    }
                    calculateOperatorChurnApprovalDigestHash
                },
                {
                    fn getCurrentQuorumBitmap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::getCurrentQuorumBitmap,
                            )
                    }
                    getCurrentQuorumBitmap
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::owner)
                    }
                    owner
                },
                {
                    fn quorumCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <quorumCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::quorumCount)
                    }
                    quorumCount
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn indexRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <indexRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::indexRegistry)
                    }
                    indexRegistry
                },
                {
                    fn PUBKEY_REGISTRATION_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::PUBKEY_REGISTRATION_TYPEHASH,
                            )
                    }
                    PUBKEY_REGISTRATION_TYPEHASH
                },
                {
                    fn isM2Quorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <isM2QuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::isM2Quorum)
                    }
                    isM2Quorum
                },
                {
                    fn ejectionCooldown(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <ejectionCooldownCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::ejectionCooldown)
                    }
                    ejectionCooldown
                },
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn m2QuorumsDisabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <m2QuorumsDisabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::m2QuorumsDisabled)
                    }
                    m2QuorumsDisabled
                },
                {
                    fn getQuorumBitmapIndicesAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::getQuorumBitmapIndicesAtBlockNumber,
                            )
                    }
                    getQuorumBitmapIndicesAtBlockNumber
                },
                {
                    fn OPERATOR_CHURN_APPROVAL_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::OPERATOR_CHURN_APPROVAL_TYPEHASH,
                            )
                    }
                    OPERATOR_CHURN_APPROVAL_TYPEHASH
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn numRegistries(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <numRegistriesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::numRegistries)
                    }
                    numRegistries
                },
                {
                    fn getOperatorSetParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <getOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::getOperatorSetParams)
                    }
                    getOperatorSetParams
                },
                {
                    fn socketRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <socketRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::socketRegistry)
                    }
                    socketRegistry
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::unpause)
                    }
                    unpause
                },
                {
                    fn getOperatorStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls> {
                        <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorCalls::getOperatorStatus)
                    }
                    getOperatorStatus
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
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(inner) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(inner) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::accountIdentifier(inner) => {
                    <accountIdentifierCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::calculateOperatorChurnApprovalDigestHash(inner) => {
                    <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::churnApprover(inner) => {
                    <churnApproverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createSlashableStakeQuorum(inner) => {
                    <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createTotalDelegatedStakeQuorum(inner) => {
                    <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ejectOperator(inner) => {
                    <ejectOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ejectionCooldown(inner) => {
                    <ejectionCooldownCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ejector(inner) => {
                    <ejectorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getCurrentQuorumBitmap(inner) => {
                    <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperator(inner) => {
                    <getOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorFromId(inner) => {
                    <getOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorId(inner) => {
                    <getOperatorIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSetParams(inner) => {
                    <getOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorStatus(inner) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapAtBlockNumberByIndex(inner) => {
                    <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapHistoryLength(inner) => {
                    <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapIndicesAtBlockNumber(inner) => {
                    <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapUpdateByIndex(inner) => {
                    <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::indexRegistry(inner) => {
                    <indexRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isChurnApproverSaltUsed(inner) => {
                    <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isM2Quorum(inner) => {
                    <isM2QuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lastEjectionTimestamp(inner) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::m2QuorumsDisabled(inner) => {
                    <m2QuorumsDisabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::numRegistries(inner) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorSetsEnabled(inner) => {
                    <operatorSetsEnabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::pubkeyRegistrationMessageHash(inner) => {
                    <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::quorumCount(inner) => {
                    <quorumCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::quorumUpdateBlockNumber(inner) => {
                    <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registries(inner) => {
                    <registriesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAccountIdentifier(inner) => {
                    <setAccountIdentifierCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setChurnApprover(inner) => {
                    <setChurnApproverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setEjectionCooldown(inner) => {
                    <setEjectionCooldownCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setEjector(inner) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setOperatorSetParams(inner) => {
                    <setOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::socketRegistry(inner) => {
                    <socketRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateSocket(inner) => {
                    <updateSocketCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(inner) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(inner) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::accountIdentifier(inner) => {
                    <accountIdentifierCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::calculateOperatorChurnApprovalDigestHash(inner) => {
                    <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::churnApprover(inner) => {
                    <churnApproverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createSlashableStakeQuorum(inner) => {
                    <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createTotalDelegatedStakeQuorum(inner) => {
                    <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::ejectOperator(inner) => {
                    <ejectOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ejectionCooldown(inner) => {
                    <ejectionCooldownCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ejector(inner) => {
                    <ejectorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getCurrentQuorumBitmap(inner) => {
                    <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperator(inner) => {
                    <getOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorFromId(inner) => {
                    <getOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getOperatorSetParams(inner) => {
                    <getOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorStatus(inner) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapAtBlockNumberByIndex(inner) => {
                    <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapHistoryLength(inner) => {
                    <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapIndicesAtBlockNumber(inner) => {
                    <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapUpdateByIndex(inner) => {
                    <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::indexRegistry(inner) => {
                    <indexRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isChurnApproverSaltUsed(inner) => {
                    <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isM2Quorum(inner) => {
                    <isM2QuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastEjectionTimestamp(inner) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::m2QuorumsDisabled(inner) => {
                    <m2QuorumsDisabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::numRegistries(inner) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorSetsEnabled(inner) => {
                    <operatorSetsEnabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::pubkeyRegistrationMessageHash(inner) => {
                    <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::quorumCount(inner) => {
                    <quorumCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::quorumUpdateBlockNumber(inner) => {
                    <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::registries(inner) => {
                    <registriesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setAccountIdentifier(inner) => {
                    <setAccountIdentifierCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setChurnApprover(inner) => {
                    <setChurnApproverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setEjectionCooldown(inner) => {
                    <setEjectionCooldownCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setEjector(inner) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperatorSetParams(inner) => {
                    <setOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::socketRegistry(inner) => {
                    <socketRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateSocket(inner) => {
                    <updateSocketCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SlashingRegistryCoordinator`](self) custom errors.
    pub enum SlashingRegistryCoordinatorErrors {
        AlreadyRegisteredForQuorums(AlreadyRegisteredForQuorums),
        BitmapCannotBeZero(BitmapCannotBeZero),
        BitmapEmpty(BitmapEmpty),
        BitmapUpdateIsAfterBlockNumber(BitmapUpdateIsAfterBlockNumber),
        BitmapValueTooLarge(BitmapValueTooLarge),
        BytesArrayLengthTooLong(BytesArrayLengthTooLong),
        BytesArrayNotOrdered(BytesArrayNotOrdered),
        CannotChurnSelf(CannotChurnSelf),
        CannotKickOperatorAboveThreshold(CannotKickOperatorAboveThreshold),
        CannotReregisterYet(CannotReregisterYet),
        ChurnApproverSaltUsed(ChurnApproverSaltUsed),
        CurrentlyPaused(CurrentlyPaused),
        ExpModFailed(ExpModFailed),
        InputAddressZero(InputAddressZero),
        InputLengthMismatch(InputLengthMismatch),
        InsufficientStakeForChurn(InsufficientStakeForChurn),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        InvalidRegistrationType(InvalidRegistrationType),
        InvalidSignature(InvalidSignature),
        MaxQuorumsReached(MaxQuorumsReached),
        NextBitmapUpdateIsBeforeBlockNumber(NextBitmapUpdateIsBeforeBlockNumber),
        NotRegistered(NotRegistered),
        NotRegisteredForQuorum(NotRegisteredForQuorum),
        NotSorted(NotSorted),
        OnlyAllocationManager(OnlyAllocationManager),
        OnlyEjector(OnlyEjector),
        OnlyPauser(OnlyPauser),
        OnlyUnpauser(OnlyUnpauser),
        OperatorSetsNotEnabled(OperatorSetsNotEnabled),
        QuorumDoesNotExist(QuorumDoesNotExist),
        QuorumOperatorCountMismatch(QuorumOperatorCountMismatch),
        SignatureExpired(SignatureExpired),
    }
    #[automatically_derived]
    impl SlashingRegistryCoordinatorErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 25u8, 189u8, 205u8],
            [12u8, 104u8, 22u8, 205u8],
            [19u8, 202u8, 70u8, 87u8],
            [35u8, 216u8, 113u8, 165u8],
            [50u8, 208u8, 206u8, 250u8],
            [53u8, 75u8, 184u8, 171u8],
            [60u8, 184u8, 156u8, 151u8],
            [76u8, 68u8, 153u8, 93u8],
            [91u8, 119u8, 144u8, 25u8],
            [108u8, 177u8, 154u8, 255u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [128u8, 200u8, 131u8, 72u8],
            [132u8, 10u8, 72u8, 213u8],
            [139u8, 170u8, 87u8, 159u8],
            [142u8, 90u8, 238u8, 231u8],
            [170u8, 173u8, 19u8, 247u8],
            [171u8, 164u8, 115u8, 57u8],
            [172u8, 45u8, 22u8, 130u8],
            [177u8, 135u8, 232u8, 105u8],
            [186u8, 80u8, 249u8, 17u8],
            [187u8, 186u8, 96u8, 203u8],
            [198u8, 29u8, 202u8, 93u8],
            [202u8, 149u8, 115u8, 51u8],
            [208u8, 83u8, 170u8, 33u8],
            [209u8, 109u8, 80u8, 234u8],
            [213u8, 30u8, 218u8, 227u8],
            [223u8, 125u8, 253u8, 134u8],
            [230u8, 33u8, 159u8, 234u8],
            [237u8, 177u8, 86u8, 46u8],
            [251u8, 74u8, 156u8, 142u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SlashingRegistryCoordinatorErrors {
        const NAME: &'static str = "SlashingRegistryCoordinatorErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 32usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyRegisteredForQuorums(_) => {
                    <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BitmapCannotBeZero(_) => {
                    <BitmapCannotBeZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BitmapEmpty(_) => {
                    <BitmapEmpty as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BitmapUpdateIsAfterBlockNumber(_) => {
                    <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BitmapValueTooLarge(_) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayLengthTooLong(_) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayNotOrdered(_) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotChurnSelf(_) => {
                    <CannotChurnSelf as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotKickOperatorAboveThreshold(_) => {
                    <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotReregisterYet(_) => {
                    <CannotReregisterYet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ChurnApproverSaltUsed(_) => {
                    <ChurnApproverSaltUsed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpModFailed(_) => {
                    <ExpModFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputLengthMismatch(_) => {
                    <InputLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientStakeForChurn(_) => {
                    <InsufficientStakeForChurn as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRegistrationType(_) => {
                    <InvalidRegistrationType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxQuorumsReached(_) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NextBitmapUpdateIsBeforeBlockNumber(_) => {
                    <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotRegistered(_) => {
                    <NotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotRegisteredForQuorum(_) => {
                    <NotRegisteredForQuorum as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSorted(_) => <NotSorted as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyAllocationManager(_) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEjector(_) => {
                    <OnlyEjector as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => {
                    <OnlyPauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorSetsNotEnabled(_) => {
                    <OperatorSetsNotEnabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumDoesNotExist(_) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumOperatorCountMismatch(_) => {
                    <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn AlreadyRegisteredForQuorums(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::AlreadyRegisteredForQuorums,
                            )
                    }
                    AlreadyRegisteredForQuorums
                },
                {
                    fn BitmapEmpty(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <BitmapEmpty as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::BitmapEmpty)
                    }
                    BitmapEmpty
                },
                {
                    fn OnlyAllocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <OnlyAllocationManager as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::OnlyAllocationManager,
                            )
                    }
                    OnlyAllocationManager
                },
                {
                    fn CannotReregisterYet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <CannotReregisterYet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::CannotReregisterYet)
                    }
                    CannotReregisterYet
                },
                {
                    fn InvalidRegistrationType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <InvalidRegistrationType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::InvalidRegistrationType,
                            )
                    }
                    InvalidRegistrationType
                },
                {
                    fn MaxQuorumsReached(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <MaxQuorumsReached as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::MaxQuorumsReached)
                    }
                    MaxQuorumsReached
                },
                {
                    fn InsufficientStakeForChurn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <InsufficientStakeForChurn as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::InsufficientStakeForChurn,
                            )
                    }
                    InsufficientStakeForChurn
                },
                {
                    fn OperatorSetsNotEnabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <OperatorSetsNotEnabled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::OperatorSetsNotEnabled,
                            )
                    }
                    OperatorSetsNotEnabled
                },
                {
                    fn BitmapUpdateIsAfterBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::BitmapUpdateIsAfterBlockNumber,
                            )
                    }
                    BitmapUpdateIsAfterBlockNumber
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::BytesArrayNotOrdered)
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn QuorumOperatorCountMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::QuorumOperatorCountMismatch,
                            )
                    }
                    QuorumOperatorCountMismatch
                },
                {
                    fn InputLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <InputLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::InputLengthMismatch)
                    }
                    InputLengthMismatch
                },
                {
                    fn NotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <NotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::NotRegistered)
                    }
                    NotRegistered
                },
                {
                    fn CannotChurnSelf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <CannotChurnSelf as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::CannotChurnSelf)
                    }
                    CannotChurnSelf
                },
                {
                    fn CannotKickOperatorAboveThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::CannotKickOperatorAboveThreshold,
                            )
                    }
                    CannotKickOperatorAboveThreshold
                },
                {
                    fn NotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn NextBitmapUpdateIsBeforeBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::NextBitmapUpdateIsBeforeBlockNumber,
                            )
                    }
                    NextBitmapUpdateIsBeforeBlockNumber
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::InvalidNewPausedStatus,
                            )
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn NotRegisteredForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <NotRegisteredForQuorum as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::NotRegisteredForQuorum,
                            )
                    }
                    NotRegisteredForQuorum
                },
                {
                    fn BitmapCannotBeZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <BitmapCannotBeZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::BitmapCannotBeZero)
                    }
                    BitmapCannotBeZero
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn ChurnApproverSaltUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <ChurnApproverSaltUsed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::ChurnApproverSaltUsed,
                            )
                    }
                    ChurnApproverSaltUsed
                },
                {
                    fn QuorumDoesNotExist(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::QuorumDoesNotExist)
                    }
                    QuorumDoesNotExist
                },
                {
                    fn OnlyEjector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <OnlyEjector as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlashingRegistryCoordinatorErrors::OnlyEjector)
                    }
                    OnlyEjector
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors> {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::BytesArrayLengthTooLong,
                            )
                    }
                    BytesArrayLengthTooLong
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
                Self::AlreadyRegisteredForQuorums(inner) => {
                    <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BitmapCannotBeZero(inner) => {
                    <BitmapCannotBeZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BitmapEmpty(inner) => {
                    <BitmapEmpty as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BitmapUpdateIsAfterBlockNumber(inner) => {
                    <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
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
                Self::CannotChurnSelf(inner) => {
                    <CannotChurnSelf as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotKickOperatorAboveThreshold(inner) => {
                    <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotReregisterYet(inner) => {
                    <CannotReregisterYet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ChurnApproverSaltUsed(inner) => {
                    <ChurnApproverSaltUsed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputLengthMismatch(inner) => {
                    <InputLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientStakeForChurn(inner) => {
                    <InsufficientStakeForChurn as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidRegistrationType(inner) => {
                    <InvalidRegistrationType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxQuorumsReached(inner) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NextBitmapUpdateIsBeforeBlockNumber(inner) => {
                    <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotRegistered(inner) => {
                    <NotRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotRegisteredForQuorum(inner) => {
                    <NotRegisteredForQuorum as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyAllocationManager(inner) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEjector(inner) => {
                    <OnlyEjector as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorSetsNotEnabled(inner) => {
                    <OperatorSetsNotEnabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumDoesNotExist(inner) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumOperatorCountMismatch(inner) => {
                    <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyRegisteredForQuorums(inner) => {
                    <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapCannotBeZero(inner) => {
                    <BitmapCannotBeZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapEmpty(inner) => {
                    <BitmapEmpty as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapUpdateIsAfterBlockNumber(inner) => {
                    <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
                Self::CannotChurnSelf(inner) => {
                    <CannotChurnSelf as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotKickOperatorAboveThreshold(inner) => {
                    <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotReregisterYet(inner) => {
                    <CannotReregisterYet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ChurnApproverSaltUsed(inner) => {
                    <ChurnApproverSaltUsed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InputLengthMismatch(inner) => {
                    <InputLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientStakeForChurn(inner) => {
                    <InsufficientStakeForChurn as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidRegistrationType(inner) => {
                    <InvalidRegistrationType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxQuorumsReached(inner) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NextBitmapUpdateIsBeforeBlockNumber(inner) => {
                    <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotRegistered(inner) => {
                    <NotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotRegisteredForQuorum(inner) => {
                    <NotRegisteredForQuorum as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyAllocationManager(inner) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEjector(inner) => {
                    <OnlyEjector as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorSetsNotEnabled(inner) => {
                    <OperatorSetsNotEnabled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::QuorumDoesNotExist(inner) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::QuorumOperatorCountMismatch(inner) => {
                    <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SlashingRegistryCoordinator`](self) events.
    pub enum SlashingRegistryCoordinatorEvents {
        ChurnApproverUpdated(ChurnApproverUpdated),
        EjectorUpdated(EjectorUpdated),
        Initialized(Initialized),
        OperatorDeregistered(OperatorDeregistered),
        OperatorRegistered(OperatorRegistered),
        OperatorSetParamsUpdated(OperatorSetParamsUpdated),
        OperatorSocketUpdate(OperatorSocketUpdate),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        QuorumBlockNumberUpdated(QuorumBlockNumberUpdated),
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl SlashingRegistryCoordinatorEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                49u8,
                84u8,
                87u8,
                216u8,
                168u8,
                254u8,
                96u8,
                240u8,
                74u8,
                241u8,
                124u8,
                22u8,
                226u8,
                245u8,
                165u8,
                225u8,
                219u8,
                97u8,
                43u8,
                49u8,
                100u8,
                142u8,
                88u8,
                3u8,
                3u8,
                96u8,
                117u8,
                158u8,
                248u8,
                243u8,
                82u8,
                140u8,
            ],
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
                57u8,
                111u8,
                220u8,
                177u8,
                128u8,
                203u8,
                15u8,
                234u8,
                38u8,
                146u8,
                129u8,
                19u8,
                251u8,
                15u8,
                209u8,
                195u8,
                84u8,
                152u8,
                99u8,
                249u8,
                205u8,
                86u8,
                62u8,
                106u8,
                24u8,
                79u8,
                29u8,
                87u8,
                129u8,
                22u8,
                200u8,
                228u8,
            ],
            [
                62u8,
                230u8,
                254u8,
                141u8,
                84u8,
                97u8,
                2u8,
                68u8,
                195u8,
                233u8,
                211u8,
                192u8,
                102u8,
                174u8,
                74u8,
                238u8,
                153u8,
                120u8,
                132u8,
                170u8,
                40u8,
                241u8,
                6u8,
                22u8,
                174u8,
                130u8,
                25u8,
                37u8,
                64u8,
                19u8,
                24u8,
                172u8,
            ],
            [
                70u8,
                7u8,
                125u8,
                85u8,
                51u8,
                7u8,
                99u8,
                241u8,
                98u8,
                105u8,
                253u8,
                117u8,
                229u8,
                118u8,
                22u8,
                99u8,
                244u8,
                25u8,
                45u8,
                39u8,
                145u8,
                116u8,
                124u8,
                1u8,
                137u8,
                177u8,
                106u8,
                211u8,
                29u8,
                176u8,
                125u8,
                180u8,
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
                143u8,
                48u8,
                171u8,
                9u8,
                244u8,
                58u8,
                108u8,
                21u8,
                125u8,
                127u8,
                206u8,
                126u8,
                10u8,
                19u8,
                192u8,
                3u8,
                4u8,
                44u8,
                28u8,
                149u8,
                232u8,
                167u8,
                46u8,
                122u8,
                20u8,
                106u8,
                33u8,
                192u8,
                202u8,
                162u8,
                77u8,
                201u8,
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
                232u8,
                230u8,
                140u8,
                239u8,
                28u8,
                58u8,
                118u8,
                30u8,
                215u8,
                190u8,
                126u8,
                132u8,
                99u8,
                163u8,
                117u8,
                242u8,
                127u8,
                123u8,
                195u8,
                53u8,
                229u8,
                24u8,
                36u8,
                34u8,
                60u8,
                172u8,
                206u8,
                99u8,
                110u8,
                197u8,
                195u8,
                254u8,
            ],
            [
                236u8,
                41u8,
                99u8,
                171u8,
                33u8,
                193u8,
                229u8,
                14u8,
                30u8,
                88u8,
                42u8,
                165u8,
                66u8,
                175u8,
                46u8,
                75u8,
                247u8,
                191u8,
                56u8,
                230u8,
                225u8,
                64u8,
                60u8,
                39u8,
                180u8,
                46u8,
                28u8,
                93u8,
                110u8,
                98u8,
                30u8,
                170u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SlashingRegistryCoordinatorEvents {
        const NAME: &'static str = "SlashingRegistryCoordinatorEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <ChurnApproverUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ChurnApproverUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ChurnApproverUpdated)
                }
                Some(<EjectorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EjectorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EjectorUpdated)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorDeregistered)
                }
                Some(
                    <OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorRegistered)
                }
                Some(
                    <OperatorSetParamsUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSetParamsUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSetParamsUpdated)
                }
                Some(
                    <OperatorSocketUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSocketUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSocketUpdate)
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
                    <QuorumBlockNumberUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <QuorumBlockNumberUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::QuorumBlockNumberUpdated)
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
    impl alloy_sol_types::private::IntoLogData for SlashingRegistryCoordinatorEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChurnApproverUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EjectorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetParamsUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSocketUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumBlockNumberUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChurnApproverUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EjectorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetParamsUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSocketUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumBlockNumberUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SlashingRegistryCoordinator`](self) contract instance.

See the [wrapper's documentation](`SlashingRegistryCoordinatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SlashingRegistryCoordinatorInstance<T, P, N> {
        SlashingRegistryCoordinatorInstance::<T, P, N>::new(address, provider)
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
        _stakeRegistry: alloy::sol_types::private::Address,
        _blsApkRegistry: alloy::sol_types::private::Address,
        _indexRegistry: alloy::sol_types::private::Address,
        _socketRegistry: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SlashingRegistryCoordinatorInstance<T, P, N>>,
    > {
        SlashingRegistryCoordinatorInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _stakeRegistry,
            _blsApkRegistry,
            _indexRegistry,
            _socketRegistry,
            _allocationManager,
            _pauserRegistry,
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
        _stakeRegistry: alloy::sol_types::private::Address,
        _blsApkRegistry: alloy::sol_types::private::Address,
        _indexRegistry: alloy::sol_types::private::Address,
        _socketRegistry: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SlashingRegistryCoordinatorInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _stakeRegistry,
            _blsApkRegistry,
            _indexRegistry,
            _socketRegistry,
            _allocationManager,
            _pauserRegistry,
        )
    }
    /**A [`SlashingRegistryCoordinator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SlashingRegistryCoordinator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SlashingRegistryCoordinatorInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SlashingRegistryCoordinatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SlashingRegistryCoordinatorInstance")
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
    > SlashingRegistryCoordinatorInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SlashingRegistryCoordinator`](self) contract instance.

See the [wrapper's documentation](`SlashingRegistryCoordinatorInstance`) for more details.*/
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
            _stakeRegistry: alloy::sol_types::private::Address,
            _blsApkRegistry: alloy::sol_types::private::Address,
            _indexRegistry: alloy::sol_types::private::Address,
            _socketRegistry: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<SlashingRegistryCoordinatorInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _stakeRegistry,
                _blsApkRegistry,
                _indexRegistry,
                _socketRegistry,
                _allocationManager,
                _pauserRegistry,
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
            _stakeRegistry: alloy::sol_types::private::Address,
            _blsApkRegistry: alloy::sol_types::private::Address,
            _indexRegistry: alloy::sol_types::private::Address,
            _socketRegistry: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _stakeRegistry,
                            _blsApkRegistry,
                            _indexRegistry,
                            _socketRegistry,
                            _allocationManager,
                            _pauserRegistry,
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
    impl<T, P: ::core::clone::Clone, N> SlashingRegistryCoordinatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> SlashingRegistryCoordinatorInstance<T, P, N> {
            SlashingRegistryCoordinatorInstance {
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
    > SlashingRegistryCoordinatorInstance<T, P, N> {
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
        ///Creates a new call builder for the [`OPERATOR_CHURN_APPROVAL_TYPEHASH`] function.
        pub fn OPERATOR_CHURN_APPROVAL_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            OPERATOR_CHURN_APPROVAL_TYPEHASHCall,
            N,
        > {
            self.call_builder(
                &OPERATOR_CHURN_APPROVAL_TYPEHASHCall {
                },
            )
        }
        ///Creates a new call builder for the [`PUBKEY_REGISTRATION_TYPEHASH`] function.
        pub fn PUBKEY_REGISTRATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, PUBKEY_REGISTRATION_TYPEHASHCall, N> {
            self.call_builder(
                &PUBKEY_REGISTRATION_TYPEHASHCall {
                },
            )
        }
        ///Creates a new call builder for the [`accountIdentifier`] function.
        pub fn accountIdentifier(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, accountIdentifierCall, N> {
            self.call_builder(&accountIdentifierCall {})
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
        ///Creates a new call builder for the [`calculateOperatorChurnApprovalDigestHash`] function.
        pub fn calculateOperatorChurnApprovalDigestHash(
            &self,
            registeringOperator: alloy::sol_types::private::Address,
            registeringOperatorId: alloy::sol_types::private::FixedBytes<32>,
            operatorKickParams: alloy::sol_types::private::Vec<
                <ISlashingRegistryCoordinatorTypes::OperatorKickParam as alloy::sol_types::SolType>::RustType,
            >,
            salt: alloy::sol_types::private::FixedBytes<32>,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            calculateOperatorChurnApprovalDigestHashCall,
            N,
        > {
            self.call_builder(
                &calculateOperatorChurnApprovalDigestHashCall {
                    registeringOperator,
                    registeringOperatorId,
                    operatorKickParams,
                    salt,
                    expiry,
                },
            )
        }
        ///Creates a new call builder for the [`churnApprover`] function.
        pub fn churnApprover(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, churnApproverCall, N> {
            self.call_builder(&churnApproverCall {})
        }
        ///Creates a new call builder for the [`createSlashableStakeQuorum`] function.
        pub fn createSlashableStakeQuorum(
            &self,
            operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
            lookAheadPeriod: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, createSlashableStakeQuorumCall, N> {
            self.call_builder(
                &createSlashableStakeQuorumCall {
                    operatorSetParams,
                    minimumStake,
                    strategyParams,
                    lookAheadPeriod,
                },
            )
        }
        ///Creates a new call builder for the [`createTotalDelegatedStakeQuorum`] function.
        pub fn createTotalDelegatedStakeQuorum(
            &self,
            operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            createTotalDelegatedStakeQuorumCall,
            N,
        > {
            self.call_builder(
                &createTotalDelegatedStakeQuorumCall {
                    operatorSetParams,
                    minimumStake,
                    strategyParams,
                },
            )
        }
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(
                &deregisterOperatorCall {
                    operator,
                    operatorSetIds,
                },
            )
        }
        ///Creates a new call builder for the [`ejectOperator`] function.
        pub fn ejectOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, ejectOperatorCall, N> {
            self.call_builder(
                &ejectOperatorCall {
                    operator,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`ejectionCooldown`] function.
        pub fn ejectionCooldown(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, ejectionCooldownCall, N> {
            self.call_builder(&ejectionCooldownCall {})
        }
        ///Creates a new call builder for the [`ejector`] function.
        pub fn ejector(&self) -> alloy_contract::SolCallBuilder<T, &P, ejectorCall, N> {
            self.call_builder(&ejectorCall {})
        }
        ///Creates a new call builder for the [`getCurrentQuorumBitmap`] function.
        pub fn getCurrentQuorumBitmap(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentQuorumBitmapCall, N> {
            self.call_builder(
                &getCurrentQuorumBitmapCall {
                    operatorId,
                },
            )
        }
        ///Creates a new call builder for the [`getOperator`] function.
        pub fn getOperator(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorCall, N> {
            self.call_builder(&getOperatorCall { operator })
        }
        ///Creates a new call builder for the [`getOperatorFromId`] function.
        pub fn getOperatorFromId(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorFromIdCall, N> {
            self.call_builder(
                &getOperatorFromIdCall {
                    operatorId,
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
        ///Creates a new call builder for the [`getOperatorSetParams`] function.
        pub fn getOperatorSetParams(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSetParamsCall, N> {
            self.call_builder(
                &getOperatorSetParamsCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorStatus`] function.
        pub fn getOperatorStatus(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorStatusCall, N> {
            self.call_builder(&getOperatorStatusCall { operator })
        }
        ///Creates a new call builder for the [`getQuorumBitmapAtBlockNumberByIndex`] function.
        pub fn getQuorumBitmapAtBlockNumberByIndex(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            blockNumber: u32,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getQuorumBitmapAtBlockNumberByIndexCall,
            N,
        > {
            self.call_builder(
                &getQuorumBitmapAtBlockNumberByIndexCall {
                    operatorId,
                    blockNumber,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`getQuorumBitmapHistoryLength`] function.
        pub fn getQuorumBitmapHistoryLength(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapHistoryLengthCall, N> {
            self.call_builder(
                &getQuorumBitmapHistoryLengthCall {
                    operatorId,
                },
            )
        }
        ///Creates a new call builder for the [`getQuorumBitmapIndicesAtBlockNumber`] function.
        pub fn getQuorumBitmapIndicesAtBlockNumber(
            &self,
            blockNumber: u32,
            operatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getQuorumBitmapIndicesAtBlockNumberCall,
            N,
        > {
            self.call_builder(
                &getQuorumBitmapIndicesAtBlockNumberCall {
                    blockNumber,
                    operatorIds,
                },
            )
        }
        ///Creates a new call builder for the [`getQuorumBitmapUpdateByIndex`] function.
        pub fn getQuorumBitmapUpdateByIndex(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapUpdateByIndexCall, N> {
            self.call_builder(
                &getQuorumBitmapUpdateByIndexCall {
                    operatorId,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`indexRegistry`] function.
        pub fn indexRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, indexRegistryCall, N> {
            self.call_builder(&indexRegistryCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _initialOwner: alloy::sol_types::private::Address,
            _churnApprover: alloy::sol_types::private::Address,
            _ejector: alloy::sol_types::private::Address,
            _initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
            _accountIdentifier: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _initialOwner,
                    _churnApprover,
                    _ejector,
                    _initialPausedStatus,
                    _accountIdentifier,
                },
            )
        }
        ///Creates a new call builder for the [`isChurnApproverSaltUsed`] function.
        pub fn isChurnApproverSaltUsed(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isChurnApproverSaltUsedCall, N> {
            self.call_builder(&isChurnApproverSaltUsedCall { _0 })
        }
        ///Creates a new call builder for the [`isM2Quorum`] function.
        pub fn isM2Quorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, isM2QuorumCall, N> {
            self.call_builder(&isM2QuorumCall { quorumNumber })
        }
        ///Creates a new call builder for the [`lastEjectionTimestamp`] function.
        pub fn lastEjectionTimestamp(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastEjectionTimestampCall, N> {
            self.call_builder(&lastEjectionTimestampCall { _0 })
        }
        ///Creates a new call builder for the [`m2QuorumsDisabled`] function.
        pub fn m2QuorumsDisabled(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, m2QuorumsDisabledCall, N> {
            self.call_builder(&m2QuorumsDisabledCall {})
        }
        ///Creates a new call builder for the [`numRegistries`] function.
        pub fn numRegistries(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, numRegistriesCall, N> {
            self.call_builder(&numRegistriesCall {})
        }
        ///Creates a new call builder for the [`operatorSetsEnabled`] function.
        pub fn operatorSetsEnabled(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSetsEnabledCall, N> {
            self.call_builder(&operatorSetsEnabledCall {})
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
        ///Creates a new call builder for the [`pubkeyRegistrationMessageHash`] function.
        pub fn pubkeyRegistrationMessageHash(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            pubkeyRegistrationMessageHashCall,
            N,
        > {
            self.call_builder(
                &pubkeyRegistrationMessageHashCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`quorumCount`] function.
        pub fn quorumCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, quorumCountCall, N> {
            self.call_builder(&quorumCountCall {})
        }
        ///Creates a new call builder for the [`quorumUpdateBlockNumber`] function.
        pub fn quorumUpdateBlockNumber(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, quorumUpdateBlockNumberCall, N> {
            self.call_builder(&quorumUpdateBlockNumberCall { _0 })
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(
                &registerOperatorCall {
                    operator,
                    operatorSetIds,
                    data,
                },
            )
        }
        ///Creates a new call builder for the [`registries`] function.
        pub fn registries(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, registriesCall, N> {
            self.call_builder(&registriesCall { _0 })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`setAccountIdentifier`] function.
        pub fn setAccountIdentifier(
            &self,
            _accountIdentifier: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAccountIdentifierCall, N> {
            self.call_builder(
                &setAccountIdentifierCall {
                    _accountIdentifier,
                },
            )
        }
        ///Creates a new call builder for the [`setChurnApprover`] function.
        pub fn setChurnApprover(
            &self,
            _churnApprover: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setChurnApproverCall, N> {
            self.call_builder(
                &setChurnApproverCall {
                    _churnApprover,
                },
            )
        }
        ///Creates a new call builder for the [`setEjectionCooldown`] function.
        pub fn setEjectionCooldown(
            &self,
            _ejectionCooldown: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setEjectionCooldownCall, N> {
            self.call_builder(
                &setEjectionCooldownCall {
                    _ejectionCooldown,
                },
            )
        }
        ///Creates a new call builder for the [`setEjector`] function.
        pub fn setEjector(
            &self,
            _ejector: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setEjectorCall, N> {
            self.call_builder(&setEjectorCall { _ejector })
        }
        ///Creates a new call builder for the [`setOperatorSetParams`] function.
        pub fn setOperatorSetParams(
            &self,
            quorumNumber: u8,
            operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setOperatorSetParamsCall, N> {
            self.call_builder(
                &setOperatorSetParamsCall {
                    quorumNumber,
                    operatorSetParams,
                },
            )
        }
        ///Creates a new call builder for the [`socketRegistry`] function.
        pub fn socketRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, socketRegistryCall, N> {
            self.call_builder(&socketRegistryCall {})
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`updateOperators`] function.
        pub fn updateOperators(
            &self,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsCall, N> {
            self.call_builder(&updateOperatorsCall { operators })
        }
        ///Creates a new call builder for the [`updateOperatorsForQuorum`] function.
        pub fn updateOperatorsForQuorum(
            &self,
            operatorsPerQuorum: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsForQuorumCall, N> {
            self.call_builder(
                &updateOperatorsForQuorumCall {
                    operatorsPerQuorum,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`updateSocket`] function.
        pub fn updateSocket(
            &self,
            socket: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateSocketCall, N> {
            self.call_builder(&updateSocketCall { socket })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SlashingRegistryCoordinatorInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ChurnApproverUpdated`] event.
        pub fn ChurnApproverUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ChurnApproverUpdated, N> {
            self.event_filter::<ChurnApproverUpdated>()
        }
        ///Creates a new event filter for the [`EjectorUpdated`] event.
        pub fn EjectorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EjectorUpdated, N> {
            self.event_filter::<EjectorUpdated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OperatorDeregistered`] event.
        pub fn OperatorDeregistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDeregistered, N> {
            self.event_filter::<OperatorDeregistered>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorSetParamsUpdated`] event.
        pub fn OperatorSetParamsUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSetParamsUpdated, N> {
            self.event_filter::<OperatorSetParamsUpdated>()
        }
        ///Creates a new event filter for the [`OperatorSocketUpdate`] event.
        pub fn OperatorSocketUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSocketUpdate, N> {
            self.event_filter::<OperatorSocketUpdate>()
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
        ///Creates a new event filter for the [`QuorumBlockNumberUpdated`] event.
        pub fn QuorumBlockNumberUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumBlockNumberUpdated, N> {
            self.event_filter::<QuorumBlockNumberUpdated>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
