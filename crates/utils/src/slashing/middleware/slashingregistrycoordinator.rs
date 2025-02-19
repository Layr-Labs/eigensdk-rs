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
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
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
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>, OperatorStatus);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            <OperatorStatus as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolType for OperatorInfo {
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
        impl alloy_sol_types::SolStruct for OperatorInfo {
            const NAME: &'static str = "OperatorInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorInfo(bytes32 operatorId,uint8 status)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
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
        impl alloy_sol_types::SolType for OperatorKickParam {
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
        impl alloy_sol_types::SolStruct for OperatorKickParam {
            const NAME: &'static str = "OperatorKickParam";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorKickParam(uint8 quorumNumber,address operator)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.maxOperatorCount,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.kickBIPsOfOperatorStake,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.kickBIPsOfTotalStake,
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
        impl alloy_sol_types::SolType for OperatorSetParam {
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
        impl alloy_sol_types::SolStruct for OperatorSetParam {
            const NAME: &'static str = "OperatorSetParam";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorSetParam(uint32 maxOperatorCount,uint16 kickBIPsOfOperatorStake,uint16 kickBIPsOfTotalStake)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.updateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.nextUpdateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<192> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumBitmap,
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
        impl alloy_sol_types::SolType for QuorumBitmapUpdate {
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
        impl alloy_sol_types::SolStruct for QuorumBitmapUpdate {
            const NAME: &'static str = "QuorumBitmapUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumBitmapUpdate(uint32 updateBlockNumber,uint32 nextUpdateBlockNumber,uint192 quorumBitmap)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
    impl<T, P, N> ::core::fmt::Debug for ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
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
        > ISlashingRegistryCoordinatorTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISlashingRegistryCoordinatorTypes`](self) contract instance.

        See the [wrapper's documentation](`ISlashingRegistryCoordinatorTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ISlashingRegistryCoordinatorTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
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
        > ISlashingRegistryCoordinatorTypesInstance<T, P, N>
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
        > ISlashingRegistryCoordinatorTypesInstance<T, P, N>
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for StrategyParams {
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
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
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
            f.debug_tuple("IStakeRegistryTypesInstance")
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
        > IStakeRegistryTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IStakeRegistryTypes`](self) contract instance.

        See the [wrapper's documentation](`IStakeRegistryTypesInstance`) for more details.*/
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
        > IStakeRegistryTypesInstance<T, P, N>
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
        > IStakeRegistryTypesInstance<T, P, N>
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
    error InvalidAVS();
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
    function allocationManager() external view returns (address);
    function avs() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function calculateOperatorChurnApprovalDigestHash(address registeringOperator, bytes32 registeringOperatorId, ISlashingRegistryCoordinatorTypes.OperatorKickParam[] memory operatorKickParams, bytes32 salt, uint256 expiry) external view returns (bytes32);
    function calculatePubkeyRegistrationMessageHash(address operator) external view returns (bytes32);
    function churnApprover() external view returns (address);
    function createSlashableStakeQuorum(ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory strategyParams, uint32 lookAheadPeriod) external;
    function createTotalDelegatedStakeQuorum(ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory strategyParams) external;
    function deregisterOperator(address operator, address avs, uint32[] memory operatorSetIds) external;
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
    function initialize(address _initialOwner, address _churnApprover, address _ejector, uint256 _initialPausedStatus, address _avs) external;
    function isChurnApproverSaltUsed(bytes32) external view returns (bool);
    function lastEjectionTimestamp(address) external view returns (uint256);
    function numRegistries() external view returns (uint256);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function pubkeyRegistrationMessageHash(address operator) external view returns (BN254.G1Point memory);
    function quorumCount() external view returns (uint8);
    function quorumUpdateBlockNumber(uint8) external view returns (uint256);
    function registerOperator(address operator, address avs, uint32[] memory operatorSetIds, bytes memory data) external;
    function registries(uint256) external view returns (address);
    function renounceOwnership() external;
    function setAvs(address _avs) external;
    function setChurnApprover(address _churnApprover) external;
    function setEjectionCooldown(uint256 _ejectionCooldown) external;
    function setEjector(address _ejector) external;
    function setOperatorSetParams(uint8 quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams) external;
    function socketRegistry() external view returns (address);
    function stakeRegistry() external view returns (address);
    function supportsAVS(address _avs) external view returns (bool);
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
    "name": "avs",
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
    "name": "calculatePubkeyRegistrationMessageHash",
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
        "name": "avs",
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
        "name": "_avs",
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
        "name": "avs",
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
    "name": "setAvs",
    "inputs": [
      {
        "name": "_avs",
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
    "name": "supportsAVS",
    "inputs": [
      {
        "name": "_avs",
        "type": "address",
        "internalType": "address"
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
    "name": "InvalidAVS",
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
    ///0x610200604052348015610010575f5ffd5b506040516159c73803806159c783398101604081905261002f9161027e565b604080518082018252601681527f4156535265676973747279436f6f7264696e61746f720000000000000000000060208083019182528351808501909452600684526576302e302e3160d01b908401528151902060e08190527f6bda7e3f385e48841048390444cced5cc795af87758af67622e5f4f0882c4a996101008190524660a0528993899389938993899389939290917f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6101318184846040805160208101859052908101839052606081018290524660808201523060a08201525f9060c0016040516020818303038152906040528051906020012090509392505050565b6080523060c052610120525050506001600160a01b0382169050610168576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b03908116610140529485166101a052928416610180529083166101c052821661016052166101e05261019f6101aa565b505050505050610301565b5f54610100900460ff16156102155760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff9081161015610265575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b038116811461027b575f5ffd5b50565b5f5f5f5f5f5f60c08789031215610293575f5ffd5b865161029e81610267565b60208801519096506102af81610267565b60408801519095506102c081610267565b60608801519094506102d181610267565b60808801519093506102e281610267565b60a08801519092506102f381610267565b809150509295509295509295565b60805160a05160c05160e05161010051610120516101405161016051610180516101a0516101c0516101e05161557f6104485f395f818161081101528181611ed3015281816124f301528181613366015261343a01525f818161072b01528181610fbd0152818161138801528181612207015281816126ad0152612ec001525f81816106460152818161131601528181611afd0152818161218c015281816125910152818161262c01528181612e1e015261396201525f818161060c01528181610d1b01528181611354015281816121130152818161272201528181612aba01528181612b310152612da501525f81816108f5015281816113bd0152611ccb01525f81816106d401528181610bb5015281816114ba01526118ae01525f61353e01525f61358d01525f61356801525f6134c101525f6134eb01525f613515015261557f5ff3fe608060405234801561000f575f5ffd5b5060043610610332575f3560e01c80636347c900116101af578063a57f4c6f116100fe578063d72d8dd61161009e578063ea32afae11610079578063ea32afae146108f0578063f2fde38b14610917578063fabc1cbc1461092a578063fd39105a1461093d575f5ffd5b8063d72d8dd614610833578063de1164bb1461083b578063e65797ad1461084e575f5ffd5b8063c391425e116100d9578063c391425e146107b2578063c63fd502146107d2578063ca0de882146107e5578063ca8aa7c71461080c575f5ffd5b8063a57f4c6f14610774578063a96f783e14610787578063b526578714610790575f5ffd5b806384ca5213116101695780638da5cb5b116101445780638da5cb5b146106f65780639aa1653d146107075780639e9923c2146107265780639feab8591461074d575f5ffd5b806384ca5213146106a9578063871ef049146106bc578063886f1195146106cf575f5ffd5b80636347c9001461062e57806368304835146106415780636e3b17db14610668578063715018a61461067b57806373447992146106835780638281ab7514610696575f5ffd5b8063296bb06411610285578063530b97a4116102255780635ac86ab7116102005780635ac86ab7146105cd5780635b0b829f146105ec5780635c975abb146105ff5780635df4594614610607575f5ffd5b8063530b97a4146105925780635865c60c146105a5578063595c6a67146105c5575f5ffd5b8063303ca95611610260578063303ca956146105395780633c2a7f4c1461054c5780633eef3a511461056c5780635140a5481461057f575f5ffd5b8063296bb0641461050057806329d1e0c3146105135780632cdd1e8614610526575f5ffd5b8063125e0584116102f05780631478851f116102cb5780631478851f146104535780631eb812da14610485578063249a0c42146104ce57806328f61b31146104ed575f5ffd5b8063125e0584146103f957806313542a4e14610418578063136439dd14610440575f5ffd5b8062cf2ab51461033657806303fd34921461034b57806304ec63511461037d578063054310e6146103a85780630cf4b767146103d35780630d3f2134146103e6575b5f5ffd5b6103496103443660046141d1565b610978565b005b61036a610359366004614202565b5f9081526098602052604090205490565b6040519081526020015b60405180910390f35b61039061038b36600461422a565b610b19565b6040516001600160c01b039091168152602001610374565b609d546103bb906001600160a01b031681565b6040516001600160a01b039091168152602001610374565b6103496103e13660046142d0565b610b31565b6103496103f4366004614202565b610b93565b61036a610407366004614301565b609f6020525f908152604090205481565b61036a610426366004614301565b6001600160a01b03165f9081526099602052604090205490565b61034961044e366004614202565b610ba0565b610475610461366004614202565b609a6020525f908152604090205460ff1681565b6040519015158152602001610374565b61049861049336600461431c565b610c75565b60408051825163ffffffff908116825260208085015190911690820152918101516001600160c01b031690820152606001610374565b61036a6104dc366004614351565b609b6020525f908152604090205481565b609e546103bb906001600160a01b031681565b6103bb61050e366004614202565b610d03565b610349610521366004614301565b610d8c565b610349610534366004614301565b610d9d565b6103496105473660046143ce565b610dae565b61055f61055a366004614301565b610e29565b604051610374919061442b565b61034961057a36600461454c565b610ea7565b61034961058d3660046145f8565b610ec3565b6103496105a03660046146d5565b6111ea565b6105b86105b3366004614301565b611433565b604051610374919061476d565b6103496114a5565b6104756105db366004614351565b6001805460ff9092161b9081161490565b6103496105fa366004614788565b611554565b60015461036a565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b6103bb61063c366004614202565b611570565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b6103496106763660046147ba565b611598565b6103496115ac565b61036a610691366004614301565b6115bd565b6103496106a4366004614806565b611606565b61036a6106b73660046148ec565b61161b565b6103906106ca366004614202565b611664565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b6064546001600160a01b03166103bb565b6096546107149060ff1681565b60405160ff9091168152602001610374565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b61036a7f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de681565b610349610782366004614301565b61166e565b61036a60a05481565b61047561079e366004614301565b60a1546001600160a01b0391821691161490565b6107c56107c0366004614951565b61167f565b60405161037491906149f6565b6103496107e0366004614a3e565b61168d565b61036a7f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a81565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b609c5461036a565b60a1546103bb906001600160a01b031681565b6108bc61085c366004614351565b60408051606080820183525f808352602080840182905292840181905260ff9490941684526097825292829020825193840183525463ffffffff8116845261ffff600160201b8204811692850192909252600160301b9004169082015290565b60408051825163ffffffff16815260208084015161ffff908116918301919091529282015190921690820152606001610374565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b610349610925366004614301565b611836565b610349610938366004614202565b6118ac565b61096b61094b366004614301565b6001600160a01b03165f9081526099602052604090206001015460ff1690565b6040516103749190614acb565b6001546002906004908116036109a15760405163840a48d560e01b815260040160405180910390fd5b5f5b8251811015610b14576040805160018082528183019092525f91602080830190803683370190505090508382815181106109df576109df614ad9565b6020026020010151815f815181106109f9576109f9614ad9565b6001600160a01b0392909216602092830291909101909101526040805160018082528183019092525f9181602001602082028036833701905050905060995f868581518110610a4a57610a4a614ad9565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f205f0154815f81518110610a8657610a86614ad9565b6020026020010181815250505f610ab5825f81518110610aa857610aa8614ad9565b60200260200101516119c3565b90505f610aca826001600160c01b03166119cf565b90505f5b8151811015610b0357610afb8585848481518110610aee57610aee614ad9565b016020015160f81c611a98565b600101610ace565b5050600190930192506109a3915050565b505050565b5f610b276098858585611bd0565b90505b9392505050565b6001335f9081526099602052604090206001015460ff166002811115610b5957610b59614739565b14610b775760405163aba4733960e01b815260040160405180910390fd5b335f90815260996020526040902054610b909082611cb4565b50565b610b9b611d5f565b60a055565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015610c02573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c269190614afc565b610c4357604051631d77d47760e21b815260040160405180910390fd5b6001548181168114610c685760405163c61dca5d60e01b815260040160405180910390fd5b610c7182611db9565b5050565b604080516060810182525f80825260208201819052918101919091525f838152609860205260409020805483908110610cb057610cb0614ad9565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160c01b03600160401b909304929092169082015290505b92915050565b6040516308f6629d60e31b8152600481018290525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906347b314e890602401602060405180830381865afa158015610d68573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cfd9190614b15565b610d94611d5f565b610b9081611df6565b610da5611d5f565b610b9081611e5f565b610db6611ec8565b60018054600290811603610ddd5760405163840a48d560e01b815260040160405180910390fd5b60a1546001600160a01b03848116911614610e0b576040516366e565df60e01b815260040160405180910390fd5b5f610e1583611f11565b9050610e2285825f611fb9565b5050505050565b604080518082019091525f8082526020820152610cfd610ea27f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de684604051602001610e879291909182526001600160a01b0316602082015260400190565b60405160208183030381529060405280519060200120612287565b6122d3565b610eaf611d5f565b610ebd84848460018561235d565b50505050565b600154600290600490811603610eec5760405163840a48d560e01b815260040160405180910390fd5b610f3183838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505060965460ff1691506127889050565b5083518214610f535760405163aaad13f760e01b815260040160405180910390fd5b5f5b82811015610e22575f848483818110610f7057610f70614ad9565b885192013560f81c92505f9188915084908110610f8f57610f8f614ad9565b60209081029190910101516040516379a0849160e11b815260ff841660048201529091506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f341092290602401602060405180830381865afa158015611002573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110269190614b30565b63ffffffff1681511461104c57604051638e5aeee760e01b815260040160405180910390fd5b5f81516001600160401b038111156110665761106661409a565b60405190808252806020026020018201604052801561108f578160200160208202803683370190505b5090505f805b8351811015611185575f8482815181106110b1576110b1614ad9565b6020026020010151905060995f826001600160a01b03166001600160a01b031681526020019081526020015f205f01548483815181106110f3576110f3614ad9565b6020026020010181815250505f611115858481518110610aa857610aa8614ad9565b905060016001600160c01b03821660ff89161c8116146111485760405163d053aa2160e01b815260040160405180910390fd5b836001600160a01b0316826001600160a01b03161161117a5760405163ba50f91160e01b815260040160405180910390fd5b509150600101611095565b50611191838386611a98565b60ff84165f818152609b6020908152604091829020439081905591519182527f46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db4910160405180910390a250505050806001019050610f55565b5f54610100900460ff161580801561120857505f54600160ff909116105b806112215750303b15801561122157505f5460ff166001145b6112895760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156112aa575f805461ff0019166101001790555b6112b3866127bc565b6112bc85611df6565b6112c583611db9565b6112ce84611e5f565b6112d78261280d565b609c8054600181810183555f8390527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c91820180546001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166001600160a01b03199283161790925584548084018655840180547f0000000000000000000000000000000000000000000000000000000000000000841690831617905584548084018655840180547f000000000000000000000000000000000000000000000000000000000000000084169083161790558454928301909455910180547f000000000000000000000000000000000000000000000000000000000000000090921691909216179055801561142b575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050565b604080518082019091525f80825260208201526001600160a01b0382165f908152609960209081526040918290208251808401909352805483526001810154909183019060ff16600281111561148b5761148b614739565b600281111561149c5761149c614739565b90525092915050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611507573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061152b9190614afc565b61154857604051631d77d47760e21b815260040160405180910390fd5b6115525f19611db9565b565b61155c611d5f565b816115668161282f565b610b148383612858565b609c818154811061157f575f80fd5b5f918252602090912001546001600160a01b0316905081565b6115a06128fd565b610c7182826001612928565b6115b4611d5f565b6115525f6127bc565b5f610cfd7f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de683604051602001610e879291909182526001600160a01b0316602082015260400190565b61160e611d5f565b610b148383835f5f61235d565b5f61165a7f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a8787878787604051602001610e8796959493929190614b4b565b9695505050505050565b5f610cfd826119c3565b611676611d5f565b610b908161280d565b6060610b2a609884846129ea565b611695611ec8565b600180545f91908116036116bc5760405163840a48d560e01b815260040160405180910390fd5b60a1546001600160a01b038681169116146116ea576040516366e565df60e01b815260040160405180910390fd5b5f6116f485611f11565b90505f808061170586880188614ccf565b9250925092505f6117168b83612a99565b90505f84600181111561172b5761172b614739565b036117d4575f61173f8c8388876001612bc7565b5190505f5b86518110156117cd575f87828151811061176057611760614ad9565b0160209081015160f81c5f8181526097909252604090912054845191925063ffffffff169084908490811061179757611797614ad9565b602002602001015163ffffffff1611156117c457604051633cb89c9760e01b815260040160405180910390fd5b50600101611744565b5050611829565b60018460018111156117e8576117e8614739565b03611810575f806117fb898b018b614d2a565b945094505050506117cd8d8489888686613010565b60405163354bb8ab60e01b815260040160405180910390fd5b5050505050505050505050565b61183e611d5f565b6001600160a01b0381166118a35760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401611280565b610b90816127bc565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611908573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061192c9190614b15565b6001600160a01b0316336001600160a01b03161461195d5760405163794821ff60e01b815260040160405180910390fd5b600154801982198116146119845760405163c61dca5d60e01b815260040160405180910390fd5b600182905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020015b60405180910390a25050565b5f610cfd609883613206565b60605f5f6119dc84613270565b61ffff166001600160401b038111156119f7576119f761409a565b6040519080825280601f01601f191660200182016040528015611a21576020820181803683370190505b5090505f805b825182108015611a38575061010081105b15611a8e576001811b935085841615611a7e578060f81b838381518110611a6157611a61614ad9565b60200101906001600160f81b03191690815f1a9053508160010191505b611a8781614e37565b9050611a27565b5090949350505050565b6040805160018082528183019092525f916020820181803683370190505090508160f81b815f81518110611ace57611ace614ad9565b60200101906001600160f81b03191690815f1a905350604051636c3fb4bf60e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636c3fb4bf90611b3690889088908890600401614e4f565b5f604051808303815f875af1158015611b51573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611b789190810190614edf565b90505f5b855181101561142b57818181518110611b9757611b97614ad9565b602002602001015115611bc857611bc8868281518110611bb957611bb9614ad9565b6020026020010151845f612928565b600101611b7c565b5f838152602085905260408120805482919084908110611bf257611bf2614ad9565b5f91825260209182902060408051606081018252929091015463ffffffff808216808552600160201b8304821695850195909552600160401b9091046001600160c01b03169183019190915290925085161015611c6257604051636cb19aff60e01b815260040160405180910390fd5b602081015163ffffffff161580611c885750806020015163ffffffff168463ffffffff16105b611ca55760405163bbba60cb60e01b815260040160405180910390fd5b6040015190505b949350505050565b6040516378219b3f60e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f043367e90611d029085908590600401614f9a565b5f604051808303815f87803b158015611d19575f5ffd5b505af1158015611d2b573d5f5f3e3d5ffd5b50505050817fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa826040516119b79190614fb2565b6064546001600160a01b031633146115525760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401611280565b600181905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b609d54604080516001600160a01b03928316815291831660208301527f315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c910160405180910390a1609d80546001600160a01b0319166001600160a01b0392909216919091179055565b609e54604080516001600160a01b03928316815291831660208301527f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9910160405180910390a1609e80546001600160a01b0319166001600160a01b0392909216919091179055565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611552576040516323d871a560e01b815260040160405180910390fd5b60605f82516001600160401b03811115611f2d57611f2d61409a565b6040519080825280601f01601f191660200182016040528015611f57576020820181803683370190505b5090505f5b8351811015611fb257838181518110611f7757611f77614ad9565b602002602001015160f81b828281518110611f9457611f94614ad9565b60200101906001600160f81b03191690815f1a905350600101611f5c565b5092915050565b6001600160a01b0383165f90815260996020526040812080549091611fdd826119c3565b905060018084015460ff166002811115611ff957611ff9614739565b146120175760405163aba4733960e01b815260040160405180910390fd5b6096545f9061202a90879060ff16612788565b90506001600160c01b038116612053576040516368b6a87560e11b815260040160405180910390fd5b61206a6001600160c01b0382811690841681161490565b6120875760405163d053aa2160e01b815260040160405180910390fd5b6001600160c01b03818116198316166120a0848261329a565b6001600160c01b0381166120fc576001600160a01b0388165f81815260996020526040808220600101805460ff19166002179055518692917f396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e491a35b60405163f4e24fe560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f4e24fe59061214a908b908b90600401614fc4565b5f604051808303815f87803b158015612161575f5ffd5b505af1158015612173573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd91506121c59087908b90600401614f9a565b5f604051808303815f87803b1580156121dc575f5ffd5b505af11580156121ee573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd91506122409087908b90600401614f9a565b5f604051808303815f87803b158015612257575f5ffd5b505af1158015612269573d5f5f3e3d5ffd5b50505050851561227d5761227d88886132a6565b5050505050505050565b5f610cfd6122936134b5565b8360405161190160f01b602082015260228101839052604281018290525f9060620160405160208183030381529060405280519060200120905092915050565b604080518082019091525f80825260208201525f80806123005f51602061552a5f395f51905f5286614ffb565b90505b61230c816135db565b90935091505f51602061552a5f395f51905f528283098303612344576040805180820190915290815260208101919091529392505050565b5f51602061552a5f395f51905f52600182089050612303565b60965460ff1660c0811061238457604051633cb89c9760e01b815260040160405180910390fd5b60968054600191905f9061239c90849060ff1661500e565b92506101000a81548160ff021916908360ff1602179055506123be8187612858565b6040805160018082528183019092525f91816020015b604080518082019091525f8152606060208201528152602001906001900390816123d45790505090505f85516001600160401b038111156124175761241761409a565b604051908082528060200260200182016040528015612440578160200160208202803683370190505b5090505f5b865181101561249d5786818151811061246057612460614ad9565b60200260200101515f015182828151811061247d5761247d614ad9565b6001600160a01b0390921660209283029190910190910152600101612445565b5060405180604001604052808460ff1663ffffffff16815260200182815250825f815181106124ce576124ce614ad9565b602090810291909101015260a154604051630130fc2760e51b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169263261f84e09261252e9291909116908690600401615027565b5f604051808303815f87803b158015612545575f5ffd5b505af1158015612557573d5f5f3e3d5ffd5b505f9250612563915050565b85600181111561257557612575614739565b036125fc57604051633aea0b9d60e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906375d4173a906125ca9086908b908b90600401615141565b5f604051808303815f87803b1580156125e1575f5ffd5b505af11580156125f3573d5f5f3e3d5ffd5b50505050612695565b600185600181111561261057612610614739565b0361269557604051630662d3e160e51b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063cc5a7c20906126679086908b9089908c90600401615174565b5f604051808303815f87803b15801561267e575f5ffd5b505af1158015612690573d5f5f3e3d5ffd5b505050505b60405163136ca0f960e11b815260ff841660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906326d941f2906024015f604051808303815f87803b1580156126f6575f5ffd5b505af1158015612708573d5f5f3e3d5ffd5b505060405163136ca0f960e11b815260ff861660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692506326d941f291506024015f604051808303815f87803b15801561276d575f5ffd5b505af115801561277f573d5f5f3e3d5ffd5b5050505061227d565b5f5f61279384613657565b9050808360ff166001901b11610b2a5760405163ca95733360e01b815260040160405180910390fd5b606480546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b60a180546001600160a01b0319166001600160a01b0392909216919091179055565b60965460ff90811690821610610b9057604051637310cff560e11b815260040160405180910390fd5b60ff82165f81815260976020908152604091829020845181548684018051888701805163ffffffff90951665ffffffffffff199094168417600160201b61ffff938416021767ffff0000000000001916600160301b95831695909502949094179094558551918252518316938101939093525116918101919091527f3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac906060016119b7565b609e546001600160a01b03163314611552576040516376d8ab1760e11b815260040160405180910390fd5b8015612949576001600160a01b0383165f908152609f602052604090204290555b6001600160a01b0383165f90815260996020526040812080546096549192909161297790869060ff16612788565b90505f612983836119c3565b905060018085015460ff16600281111561299f5761299f614739565b1480156129b457506001600160c01b03821615155b80156129d257506129d26001600160c01b0383811690831681161490565b156129e1576129e187876132a6565b50505050505050565b60605f82516001600160401b03811115612a0657612a0661409a565b604051908082528060200260200182016040528015612a2f578160200160208202803683370190505b5090505f5b8351811015612a9057612a618686868481518110612a5457612a54614ad9565b6020026020010151613712565b828281518110612a7357612a73614ad9565b63ffffffff90921660209283029190910190910152600101612a34565b50949350505050565b6040516309aa152760e11b81526001600160a01b0383811660048301525f917f0000000000000000000000000000000000000000000000000000000000000000909116906313542a4e90602401602060405180830381865afa158015612b01573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b2591906151aa565b90505f819003610cfd577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bf79ce588484612b6987610e29565b6040518463ffffffff1660e01b8152600401612b87939291906151e3565b6020604051808303815f875af1158015612ba3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b2a91906151aa565b612beb60405180606001604052806060815260200160608152602001606081525090565b6096545f90612bfe90869060ff16612788565b90505f612c0a876119c3565b90506001600160c01b038216612c33576040516313ca465760e01b815260040160405180910390fd5b8082166001600160c01b031615612c5d57604051630c6816cd60e01b815260040160405180910390fd5b60a0546001600160a01b0389165f908152609f60205260409020546001600160c01b0383811690851617914291612c94919061525c565b10612cb257604051631968677d60e11b815260040160405180910390fd5b612cbc888261329a565b612cc68887611cb4565b60016001600160a01b038a165f9081526099602052604090206001015460ff166002811115612cf757612cf7614739565b14612d8e57604080518082018252898152600160208083018281526001600160a01b038e165f908152609990925293902082518155925183820180549394939192909160ff191690836002811115612d5157612d51614739565b0217905550506040518991506001600160a01b038b16907fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe905f90a35b604051631fd93ca960e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633fb2795290612ddc908c908b90600401614fc4565b5f604051808303815f87803b158015612df3575f5ffd5b505af1158015612e05573d5f5f3e3d5ffd5b5050604051632550477760e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063255047779150612e59908c908c908c9060040161526f565b5f604051808303815f875af1158015612e74573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612e9b91908101906152f9565b60408087019190915260208601919091525162bff04d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062bff04d90612ef6908b908b90600401614f9a565b5f604051808303815f875af1158015612f11573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612f389190810190615352565b84528415613004575f5b8751811015613002575f60975f8a8481518110612f6157612f61614ad9565b0160209081015160f81c82528181019290925260409081015f208151606081018352905463ffffffff811680835261ffff600160201b8304811695840195909552600160301b90910490931691810191909152875180519193509084908110612fcc57612fcc614ad9565b602002602001015163ffffffff161115612ff957604051633cb89c9760e01b815260040160405180910390fd5b50600101612f42565b505b50505095945050505050565b83518251146130325760405163aaad13f760e01b815260040160405180910390fd5b61303e8686848461382a565b5f61304c878787875f612bc7565b90505f5b855181101561227d575f60975f88848151811061306f5761306f614ad9565b0160209081015160f81c82528181019290925260409081015f208151606081018352905463ffffffff811680835261ffff600160201b8304811695840195909552600160301b909104909316918101919091528451805191935090849081106130da576130da614ad9565b602002602001015163ffffffff1611156131fd5761316e87838151811061310357613103614ad9565b602001015160f81c60f81b60f81c8460400151848151811061312757613127614ad9565b60200260200101518b8660200151868151811061314657613146614ad9565b602002602001015189878151811061316057613160614ad9565b6020026020010151866138d5565b6040805160018082528183019092525f916020820181803683370190505090508783815181106131a0576131a0614ad9565b602001015160f81c60f81b815f815181106131bd576131bd614ad9565b60200101906001600160f81b03191690815f1a9053506131fb8684815181106131e8576131e8614ad9565b602002602001015160200151825f612928565b505b50600101613050565b5f81815260208390526040812054808203613224575f915050610cfd565b5f83815260208590526040902061323c6001836153e1565b8154811061324c5761324c614ad9565b5f91825260209091200154600160401b90046001600160c01b03169150610cfd9050565b5f805b8215610cfd576132846001846153e1565b9092169180613292816153f4565b915050613273565b610c7160988383613a56565b5f81516001600160401b038111156132c0576132c061409a565b6040519080825280602002602001820160405280156132e9578160200160208202803683370190505b5090505f805b8351811015613413575f84828151811061330b5761330b614ad9565b6020910181015160408051808201825260a1546001600160a01b03908116825260f89390931c93810184815291516333869dd160e11b81528a84166004820152905183166024820152905163ffffffff1660448201529192507f0000000000000000000000000000000000000000000000000000000000000000169063670d3ba290606401602060405180830381865afa1580156133ab573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133cf9190614afc565b1561340a57808484815181106133e7576133e7614ad9565b63ffffffff909216602092830291909101909101528261340681614e37565b9350505b506001016132ef565b50808252604080516060810182526001600160a01b03868116825260a154811660208301527f00000000000000000000000000000000000000000000000000000000000000001691636e3492b59190810161346d87613c0f565b8152506040518263ffffffff1660e01b815260040161348c9190615414565b5f604051808303815f87803b1580156134a3575f5ffd5b505af115801561227d573d5f5f3e3d5ffd5b5f306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614801561350d57507f000000000000000000000000000000000000000000000000000000000000000046145b1561353757507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b5f80805f51602061552a5f395f51905f5260035f51602061552a5f395f51905f52865f51602061552a5f395f51905f52888909090890505f61364b827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f51602061552a5f395f51905f52613cb4565b91959194509092505050565b5f6101008251111561367c57604051637da54e4760e11b815260040160405180910390fd5b81515f0361368b57505f919050565b5f5f835f8151811061369f5761369f614ad9565b0160200151600160f89190911c81901b92505b8451811015613709578481815181106136cd576136cd614ad9565b0160200151600160f89190911c1b91508282116136fd57604051631019106960e31b815260040160405180910390fd5b918117916001016136b2565b50909392505050565b5f81815260208490526040812054815b8181101561379557600161373682846153e1565b61374091906153e1565b92508463ffffffff16865f8681526020019081526020015f208463ffffffff168154811061377057613770614ad9565b5f9182526020909120015463ffffffff161161378d575050610b2a565b600101613722565b5060405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a401611280565b6020808201515f908152609a909152604090205460ff161561385f57604051636fbefec360e11b815260040160405180910390fd5b428160400151101561388457604051630819bdcd60e01b815260040160405180910390fd5b602080820180515f908152609a909252604091829020805460ff19166001179055609d54905191830151610ebd926001600160a01b03909216916138ce918891889188919061161b565b8351613d2d565b6020808301516001600160a01b038082165f8181526099909452604090932054919290871603613918576040516356168b4160e11b815260040160405180910390fd5b8760ff16845f015160ff161461394157604051638e5aeee760e01b815260040160405180910390fd5b604051635401ed2760e01b81526004810182905260ff891660248201525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635401ed2790604401602060405180830381865afa1580156139af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139d39190615482565b90506139df8185613d55565b6001600160601b0316866001600160601b031611613a1057604051634c44995d60e01b815260040160405180910390fd5b613a1a8885613d78565b6001600160601b0316816001600160601b031610613a4b5760405163b187e86960e01b815260040160405180910390fd5b505050505050505050565b5f8281526020849052604081205490819003613afa575f83815260208581526040808320815160608101835263ffffffff43811682528185018681526001600160c01b03808a16958401958652845460018101865594885295909620915191909201805495519351909416600160401b026001600160401b03938316600160201b0267ffffffffffffffff1990961691909216179390931716919091179055610ebd565b5f838152602085905260408120613b126001846153e1565b81548110613b2257613b22614ad9565b5f918252602090912001805490915063ffffffff438116911603613b635780546001600160401b0316600160401b6001600160c01b03851602178155610e22565b805463ffffffff438116600160201b81810267ffffffff00000000199094169390931784555f8781526020898152604080832081516060810183529485528483018481526001600160c01b03808c1693870193845282546001810184559286529390942094519401805493519151909216600160401b026001600160401b0391861690960267ffffffffffffffff19909316939094169290921717919091169190911790555050505050565b60605f82516001600160401b03811115613c2b57613c2b61409a565b604051908082528060200260200182016040528015613c54578160200160208202803683370190505b5090505f5b8351811015611fb257838181518110613c7457613c74614ad9565b602001015160f81c60f81b60f81c60ff16828281518110613c9757613c97614ad9565b63ffffffff90921660209283029190910190910152600101613c59565b5f5f613cbe61405e565b613cc661407c565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280613d0357fe5b5082613d225760405163d51edae360e01b815260040160405180910390fd5b505195945050505050565b613d38838383613d91565b610b1457604051638baa579f60e01b815260040160405180910390fd5b60208101515f9061271090613d6e9061ffff168561549d565b610b2a91906154bf565b60408101515f9061271090613d6e9061ffff168561549d565b5f5f5f613d9e8585613ed6565b90925090505f816004811115613db657613db6614739565b148015613dd45750856001600160a01b0316826001600160a01b0316145b15613de457600192505050610b2a565b5f5f876001600160a01b0316631626ba7e60e01b8888604051602401613e0b929190614f9a565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051613e4991906154ec565b5f60405180830381855afa9150503d805f8114613e81576040519150601f19603f3d011682016040523d82523d5f602084013e613e86565b606091505b5091509150818015613e99575080516020145b8015613eca57508051630b135d3f60e11b90613ebe9083016020908101908401615502565b6001600160e01b031916145b98975050505050505050565b5f5f8251604103613f0a576020830151604084015160608501515f1a613efe87828585613f41565b94509450505050613f3a565b8251604003613f335760208301516040840151613f28868383614026565b935093505050613f3a565b505f905060025b9250929050565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115613f7657505f9050600361401d565b8460ff16601b14158015613f8e57508460ff16601c14155b15613f9e57505f9050600461401d565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015613fef573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116614017575f6001925092505061401d565b91505f90505b94509492505050565b5f806001600160ff1b0383168161404260ff86901c601b61525c565b905061405087828885613f41565b935093505050935093915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b03811182821017156140d0576140d061409a565b60405290565b604080519081016001600160401b03811182821017156140d0576140d061409a565b604051601f8201601f191681016001600160401b03811182821017156141205761412061409a565b604052919050565b5f6001600160401b038211156141405761414061409a565b5060051b60200190565b6001600160a01b0381168114610b90575f5ffd5b5f82601f83011261416d575f5ffd5b813561418061417b82614128565b6140f8565b8082825260208201915060208360051b8601019250858311156141a1575f5ffd5b602085015b838110156141c75780356141b98161414a565b8352602092830192016141a6565b5095945050505050565b5f602082840312156141e1575f5ffd5b81356001600160401b038111156141f6575f5ffd5b611cac8482850161415e565b5f60208284031215614212575f5ffd5b5035919050565b63ffffffff81168114610b90575f5ffd5b5f5f5f6060848603121561423c575f5ffd5b83359250602084013561424e81614219565b929592945050506040919091013590565b5f82601f83011261426e575f5ffd5b8135602083015f5f6001600160401b0384111561428d5761428d61409a565b50601f8301601f19166020016142a2816140f8565b9150508281528583830111156142b6575f5ffd5b828260208301375f92810160200192909252509392505050565b5f602082840312156142e0575f5ffd5b81356001600160401b038111156142f5575f5ffd5b611cac8482850161425f565b5f60208284031215614311575f5ffd5b8135610b2a8161414a565b5f5f6040838503121561432d575f5ffd5b50508035926020909101359150565b803560ff8116811461434c575f5ffd5b919050565b5f60208284031215614361575f5ffd5b610b2a8261433c565b5f82601f830112614379575f5ffd5b813561438761417b82614128565b8082825260208201915060208360051b8601019250858311156143a8575f5ffd5b602085015b838110156141c75780356143c081614219565b8352602092830192016143ad565b5f5f5f606084860312156143e0575f5ffd5b83356143eb8161414a565b925060208401356143fb8161414a565b915060408401356001600160401b03811115614415575f5ffd5b6144218682870161436a565b9150509250925092565b815181526020808301519082015260408101610cfd565b803561ffff8116811461434c575f5ffd5b5f60608284031215614463575f5ffd5b61446b6140ae565b9050813561447881614219565b815261448660208301614442565b602082015261449760408301614442565b604082015292915050565b6001600160601b0381168114610b90575f5ffd5b5f82601f8301126144c5575f5ffd5b81356144d361417b82614128565b8082825260208201915060208360061b8601019250858311156144f4575f5ffd5b602085015b838110156141c75760408188031215614510575f5ffd5b6145186140d6565b81356145238161414a565b81526020820135614533816144a2565b60208281019190915290845292909201916040016144f9565b5f5f5f5f60c0858703121561455f575f5ffd5b6145698686614453565b93506060850135614579816144a2565b925060808501356001600160401b03811115614593575f5ffd5b61459f878288016144b6565b92505060a08501356145b081614219565b939692955090935050565b5f5f83601f8401126145cb575f5ffd5b5081356001600160401b038111156145e1575f5ffd5b602083019150836020828501011115613f3a575f5ffd5b5f5f5f6040848603121561460a575f5ffd5b83356001600160401b0381111561461f575f5ffd5b8401601f8101861361462f575f5ffd5b803561463d61417b82614128565b8082825260208201915060208360051b85010192508883111561465e575f5ffd5b602084015b8381101561469e5780356001600160401b03811115614680575f5ffd5b61468f8b60208389010161415e565b84525060209283019201614663565b50955050505060208401356001600160401b038111156146bc575f5ffd5b6146c8868287016145bb565b9497909650939450505050565b5f5f5f5f5f60a086880312156146e9575f5ffd5b85356146f48161414a565b945060208601356147048161414a565b935060408601356147148161414a565b925060608601359150608086013561472b8161414a565b809150509295509295909350565b634e487b7160e01b5f52602160045260245ffd5b6003811061476957634e487b7160e01b5f52602160045260245ffd5b9052565b815181526020808301516040830191611fb29084018261474d565b5f5f60808385031215614799575f5ffd5b6147a28361433c565b91506147b18460208501614453565b90509250929050565b5f5f604083850312156147cb575f5ffd5b82356147d68161414a565b915060208301356001600160401b038111156147f0575f5ffd5b6147fc8582860161425f565b9150509250929050565b5f5f5f60a08486031215614818575f5ffd5b6148228585614453565b92506060840135614832816144a2565b915060808401356001600160401b0381111561484c575f5ffd5b614421868287016144b6565b5f82601f830112614867575f5ffd5b813561487561417b82614128565b8082825260208201915060208360061b860101925085831115614896575f5ffd5b602085015b838110156141c757604081880312156148b2575f5ffd5b6148ba6140d6565b6148c38261433c565b815260208201356148d38161414a565b602082810191909152908452929092019160400161489b565b5f5f5f5f5f60a08688031215614900575f5ffd5b853561490b8161414a565b94506020860135935060408601356001600160401b0381111561492c575f5ffd5b61493888828901614858565b9598949750949560608101359550608001359392505050565b5f5f60408385031215614962575f5ffd5b823561496d81614219565b915060208301356001600160401b03811115614987575f5ffd5b8301601f81018513614997575f5ffd5b80356149a561417b82614128565b8082825260208201915060208360051b8501019250878311156149c6575f5ffd5b6020840193505b828410156149e85783358252602093840193909101906149cd565b809450505050509250929050565b602080825282518282018190525f918401906040840190835b81811015614a3357835163ffffffff16835260209384019390920191600101614a0f565b509095945050505050565b5f5f5f5f5f60808688031215614a52575f5ffd5b8535614a5d8161414a565b94506020860135614a6d8161414a565b935060408601356001600160401b03811115614a87575f5ffd5b614a938882890161436a565b93505060608601356001600160401b03811115614aae575f5ffd5b614aba888289016145bb565b969995985093965092949392505050565b60208101610cfd828461474d565b634e487b7160e01b5f52603260045260245ffd5b8051801515811461434c575f5ffd5b5f60208284031215614b0c575f5ffd5b610b2a82614aed565b5f60208284031215614b25575f5ffd5b8151610b2a8161414a565b5f60208284031215614b40575f5ffd5b8151610b2a81614219565b5f60c0820188835260018060a01b038816602084015286604084015260c0606084015280865180835260e0850191506020880192505f5b81811015614bb8578351805160ff1684526020908101516001600160a01b03168185015290930192604090920191600101614b82565b50506080840195909552505060a00152949350505050565b80356002811061434c575f5ffd5b5f60408284031215614bee575f5ffd5b614bf66140d6565b823581526020928301359281019290925250919050565b5f82601f830112614c1c575f5ffd5b614c246140d6565b806040840185811115614c35575f5ffd5b845b81811015614a33578035845260209384019301614c37565b5f818303610100811215614c61575f5ffd5b614c696140ae565b9150614c758484614bde565b8252614c848460408501614bde565b60208301526080607f1982011215614c9a575f5ffd5b50614ca36140d6565b614cb08460808501614c0d565b8152614cbf8460c08501614c0d565b6020820152604082015292915050565b5f5f5f6101408486031215614ce2575f5ffd5b614ceb84614bd0565b925060208401356001600160401b03811115614d05575f5ffd5b614d118682870161425f565b925050614d218560408601614c4f565b90509250925092565b5f5f5f5f5f6101808688031215614d3f575f5ffd5b614d4886614bd0565b945060208601356001600160401b03811115614d62575f5ffd5b614d6e8882890161425f565b945050614d7e8760408801614c4f565b92506101408601356001600160401b03811115614d99575f5ffd5b614da588828901614858565b9250506101608601356001600160401b03811115614dc1575f5ffd5b860160608189031215614dd2575f5ffd5b614dda6140ae565b81356001600160401b03811115614def575f5ffd5b614dfb8a82850161425f565b8252506020828101359082015260409182013591810191909152949793965091945092919050565b634e487b7160e01b5f52601160045260245ffd5b5f60018201614e4857614e48614e23565b5060010190565b606080825284519082018190525f9060208601906080840190835b81811015614e915783516001600160a01b0316835260209384019390920191600101614e6a565b5050838103602080860191909152865180835291810192508601905f5b81811015614ecc578251845260209384019390920191600101614eae565b50505060ff841660408401529050611cac565b5f60208284031215614eef575f5ffd5b81516001600160401b03811115614f04575f5ffd5b8201601f81018413614f14575f5ffd5b8051614f2261417b82614128565b8082825260208201915060208360051b850101925086831115614f43575f5ffd5b6020840193505b8284101561165a57614f5b84614aed565b825260209384019390910190614f4a565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b828152604060208201525f610b276040830184614f6c565b602081525f610b2a6020830184614f6c565b6001600160a01b03831681526040602082018190525f90610b2790830184614f6c565b634e487b7160e01b5f52601260045260245ffd5b5f8261500957615009614fe7565b500690565b60ff8181168382160190811115610cfd57610cfd614e23565b5f6040820160018060a01b03851683526040602084015280845180835260608501915060608160051b8601019250602086015f5b828110156150dc57868503605f190184528151805163ffffffff168652602090810151604082880181905281519088018190529101905f9060608801905b808310156150c45783516001600160a01b031682526020938401936001939093019290910190615099565b5096505050602093840193919091019060010161505b565b5092979650505050505050565b5f8151808452602084019350602083015f5b8281101561513757815180516001600160a01b031687526020908101516001600160601b031681880152604090960195909101906001016150fb565b5093949350505050565b60ff841681526001600160601b0383166020820152606060408201525f61516b60608301846150e9565b95945050505050565b60ff851681526001600160601b038416602082015263ffffffff83166040820152608060608201525f61165a60808301846150e9565b5f602082840312156151ba575f5ffd5b5051919050565b805f5b6002811015610ebd5781518452602093840193909101906001016151c4565b6001600160a01b0384168152825180516020808401919091520151604082015261016081016020848101518051606085015290810151608084015250604084015161523260a0840182516151c1565b6020015161524360e08401826151c1565b5082516101208301526020830151610140830152611cac565b80820180821115610cfd57610cfd614e23565b60018060a01b0384168152826020820152606060408201525f61516b6060830184614f6c565b5f82601f8301126152a4575f5ffd5b81516152b261417b82614128565b8082825260208201915060208360051b8601019250858311156152d3575f5ffd5b602085015b838110156141c75780516152eb816144a2565b8352602092830192016152d8565b5f5f6040838503121561530a575f5ffd5b82516001600160401b0381111561531f575f5ffd5b61532b85828601615295565b92505060208301516001600160401b03811115615346575f5ffd5b6147fc85828601615295565b5f60208284031215615362575f5ffd5b81516001600160401b03811115615377575f5ffd5b8201601f81018413615387575f5ffd5b805161539561417b82614128565b8082825260208201915060208360051b8501019250868311156153b6575f5ffd5b6020840193505b8284101561165a5783516153d081614219565b8252602093840193909101906153bd565b81810381811115610cfd57610cfd614e23565b5f61ffff821661ffff810361540b5761540b614e23565b60010192915050565b602080825282516001600160a01b039081168383015283820151166040808401919091528301516060808401528051608084018190525f929190910190829060a08501905b808310156141c75763ffffffff8451168252602082019150602084019350600183019250615459565b5f60208284031215615492575f5ffd5b8151610b2a816144a2565b6001600160601b038181168382160290811690818114611fb257611fb2614e23565b5f6001600160601b038316806154d7576154d7614fe7565b806001600160601b0384160491505092915050565b5f82518060208501845e5f920191825250919050565b5f60208284031215615512575f5ffd5b81516001600160e01b031981168114610b2a575f5ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220acf44f61bde51b7c74684d096c55f7653fc5a41dee53d94b22aedbd5a5e8d96664736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x02\0`@R4\x80\x15a\0\x10W__\xFD[P`@QaY\xC78\x03\x80aY\xC7\x839\x81\x01`@\x81\x90Ra\0/\x91a\x02~V[`@\x80Q\x80\x82\x01\x82R`\x16\x81R\x7FAVSRegistryCoordinator\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x82R\x83Q\x80\x85\x01\x90\x94R`\x06\x84Rev0.0.1`\xD0\x1B\x90\x84\x01R\x81Q\x90 `\xE0\x81\x90R\x7Fk\xDA~?8^H\x84\x10H9\x04D\xCC\xED\\\xC7\x95\xAF\x87u\x8A\xF6v\"\xE5\xF4\xF0\x88,J\x99a\x01\0\x81\x90RF`\xA0R\x89\x93\x89\x93\x89\x93\x89\x93\x89\x93\x89\x93\x92\x90\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x011\x81\x84\x84`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\x80R0`\xC0Ra\x01 RPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x90Pa\x01hW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01@R\x94\x85\x16a\x01\xA0R\x92\x84\x16a\x01\x80R\x90\x83\x16a\x01\xC0R\x82\x16a\x01`R\x16a\x01\xE0Ra\x01\x9Fa\x01\xAAV[PPPPPPa\x03\x01V[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x02\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x10\x15a\x02eW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02{W__\xFD[PV[______`\xC0\x87\x89\x03\x12\x15a\x02\x93W__\xFD[\x86Qa\x02\x9E\x81a\x02gV[` \x88\x01Q\x90\x96Pa\x02\xAF\x81a\x02gV[`@\x88\x01Q\x90\x95Pa\x02\xC0\x81a\x02gV[``\x88\x01Q\x90\x94Pa\x02\xD1\x81a\x02gV[`\x80\x88\x01Q\x90\x93Pa\x02\xE2\x81a\x02gV[`\xA0\x88\x01Q\x90\x92Pa\x02\xF3\x81a\x02gV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0Qa\x01\xE0QaU\x7Fa\x04H_9_\x81\x81a\x08\x11\x01R\x81\x81a\x1E\xD3\x01R\x81\x81a$\xF3\x01R\x81\x81a3f\x01Ra4:\x01R_\x81\x81a\x07+\x01R\x81\x81a\x0F\xBD\x01R\x81\x81a\x13\x88\x01R\x81\x81a\"\x07\x01R\x81\x81a&\xAD\x01Ra.\xC0\x01R_\x81\x81a\x06F\x01R\x81\x81a\x13\x16\x01R\x81\x81a\x1A\xFD\x01R\x81\x81a!\x8C\x01R\x81\x81a%\x91\x01R\x81\x81a&,\x01R\x81\x81a.\x1E\x01Ra9b\x01R_\x81\x81a\x06\x0C\x01R\x81\x81a\r\x1B\x01R\x81\x81a\x13T\x01R\x81\x81a!\x13\x01R\x81\x81a'\"\x01R\x81\x81a*\xBA\x01R\x81\x81a+1\x01Ra-\xA5\x01R_\x81\x81a\x08\xF5\x01R\x81\x81a\x13\xBD\x01Ra\x1C\xCB\x01R_\x81\x81a\x06\xD4\x01R\x81\x81a\x0B\xB5\x01R\x81\x81a\x14\xBA\x01Ra\x18\xAE\x01R_a5>\x01R_a5\x8D\x01R_a5h\x01R_a4\xC1\x01R_a4\xEB\x01R_a5\x15\x01RaU\x7F_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x032W_5`\xE0\x1C\x80ccG\xC9\0\x11a\x01\xAFW\x80c\xA5\x7FLo\x11a\0\xFEW\x80c\xD7-\x8D\xD6\x11a\0\x9EW\x80c\xEA2\xAF\xAE\x11a\0yW\x80c\xEA2\xAF\xAE\x14a\x08\xF0W\x80c\xF2\xFD\xE3\x8B\x14a\t\x17W\x80c\xFA\xBC\x1C\xBC\x14a\t*W\x80c\xFD9\x10Z\x14a\t=W__\xFD[\x80c\xD7-\x8D\xD6\x14a\x083W\x80c\xDE\x11d\xBB\x14a\x08;W\x80c\xE6W\x97\xAD\x14a\x08NW__\xFD[\x80c\xC3\x91B^\x11a\0\xD9W\x80c\xC3\x91B^\x14a\x07\xB2W\x80c\xC6?\xD5\x02\x14a\x07\xD2W\x80c\xCA\r\xE8\x82\x14a\x07\xE5W\x80c\xCA\x8A\xA7\xC7\x14a\x08\x0CW__\xFD[\x80c\xA5\x7FLo\x14a\x07tW\x80c\xA9ox>\x14a\x07\x87W\x80c\xB5&W\x87\x14a\x07\x90W__\xFD[\x80c\x84\xCAR\x13\x11a\x01iW\x80c\x8D\xA5\xCB[\x11a\x01DW\x80c\x8D\xA5\xCB[\x14a\x06\xF6W\x80c\x9A\xA1e=\x14a\x07\x07W\x80c\x9E\x99#\xC2\x14a\x07&W\x80c\x9F\xEA\xB8Y\x14a\x07MW__\xFD[\x80c\x84\xCAR\x13\x14a\x06\xA9W\x80c\x87\x1E\xF0I\x14a\x06\xBCW\x80c\x88o\x11\x95\x14a\x06\xCFW__\xFD[\x80ccG\xC9\0\x14a\x06.W\x80ch0H5\x14a\x06AW\x80cn;\x17\xDB\x14a\x06hW\x80cqP\x18\xA6\x14a\x06{W\x80csDy\x92\x14a\x06\x83W\x80c\x82\x81\xABu\x14a\x06\x96W__\xFD[\x80c)k\xB0d\x11a\x02\x85W\x80cS\x0B\x97\xA4\x11a\x02%W\x80cZ\xC8j\xB7\x11a\x02\0W\x80cZ\xC8j\xB7\x14a\x05\xCDW\x80c[\x0B\x82\x9F\x14a\x05\xECW\x80c\\\x97Z\xBB\x14a\x05\xFFW\x80c]\xF4YF\x14a\x06\x07W__\xFD[\x80cS\x0B\x97\xA4\x14a\x05\x92W\x80cXe\xC6\x0C\x14a\x05\xA5W\x80cY\\jg\x14a\x05\xC5W__\xFD[\x80c0<\xA9V\x11a\x02`W\x80c0<\xA9V\x14a\x059W\x80c<*\x7FL\x14a\x05LW\x80c>\xEF:Q\x14a\x05lW\x80cQ@\xA5H\x14a\x05\x7FW__\xFD[\x80c)k\xB0d\x14a\x05\0W\x80c)\xD1\xE0\xC3\x14a\x05\x13W\x80c,\xDD\x1E\x86\x14a\x05&W__\xFD[\x80c\x12^\x05\x84\x11a\x02\xF0W\x80c\x14x\x85\x1F\x11a\x02\xCBW\x80c\x14x\x85\x1F\x14a\x04SW\x80c\x1E\xB8\x12\xDA\x14a\x04\x85W\x80c$\x9A\x0CB\x14a\x04\xCEW\x80c(\xF6\x1B1\x14a\x04\xEDW__\xFD[\x80c\x12^\x05\x84\x14a\x03\xF9W\x80c\x13T*N\x14a\x04\x18W\x80c\x13d9\xDD\x14a\x04@W__\xFD[\x80b\xCF*\xB5\x14a\x036W\x80c\x03\xFD4\x92\x14a\x03KW\x80c\x04\xECcQ\x14a\x03}W\x80c\x05C\x10\xE6\x14a\x03\xA8W\x80c\x0C\xF4\xB7g\x14a\x03\xD3W\x80c\r?!4\x14a\x03\xE6W[__\xFD[a\x03Ia\x03D6`\x04aA\xD1V[a\txV[\0[a\x03ja\x03Y6`\x04aB\x02V[_\x90\x81R`\x98` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\x90a\x03\x8B6`\x04aB*V[a\x0B\x19V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x03tV[`\x9DTa\x03\xBB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03tV[a\x03Ia\x03\xE16`\x04aB\xD0V[a\x0B1V[a\x03Ia\x03\xF46`\x04aB\x02V[a\x0B\x93V[a\x03ja\x04\x076`\x04aC\x01V[`\x9F` R_\x90\x81R`@\x90 T\x81V[a\x03ja\x04&6`\x04aC\x01V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x99` R`@\x90 T\x90V[a\x03Ia\x04N6`\x04aB\x02V[a\x0B\xA0V[a\x04ua\x04a6`\x04aB\x02V[`\x9A` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03tV[a\x04\x98a\x04\x936`\x04aC\x1CV[a\x0CuV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x03tV[a\x03ja\x04\xDC6`\x04aCQV[`\x9B` R_\x90\x81R`@\x90 T\x81V[`\x9ETa\x03\xBB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xBBa\x05\x0E6`\x04aB\x02V[a\r\x03V[a\x03Ia\x05!6`\x04aC\x01V[a\r\x8CV[a\x03Ia\x0546`\x04aC\x01V[a\r\x9DV[a\x03Ia\x05G6`\x04aC\xCEV[a\r\xAEV[a\x05_a\x05Z6`\x04aC\x01V[a\x0E)V[`@Qa\x03t\x91\x90aD+V[a\x03Ia\x05z6`\x04aELV[a\x0E\xA7V[a\x03Ia\x05\x8D6`\x04aE\xF8V[a\x0E\xC3V[a\x03Ia\x05\xA06`\x04aF\xD5V[a\x11\xEAV[a\x05\xB8a\x05\xB36`\x04aC\x01V[a\x143V[`@Qa\x03t\x91\x90aGmV[a\x03Ia\x14\xA5V[a\x04ua\x05\xDB6`\x04aCQV[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x03Ia\x05\xFA6`\x04aG\x88V[a\x15TV[`\x01Ta\x03jV[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xBBa\x06<6`\x04aB\x02V[a\x15pV[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ia\x06v6`\x04aG\xBAV[a\x15\x98V[a\x03Ia\x15\xACV[a\x03ja\x06\x916`\x04aC\x01V[a\x15\xBDV[a\x03Ia\x06\xA46`\x04aH\x06V[a\x16\x06V[a\x03ja\x06\xB76`\x04aH\xECV[a\x16\x1BV[a\x03\x90a\x06\xCA6`\x04aB\x02V[a\x16dV[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`dT`\x01`\x01`\xA0\x1B\x03\x16a\x03\xBBV[`\x96Ta\x07\x14\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03tV[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03j\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x03Ia\x07\x826`\x04aC\x01V[a\x16nV[a\x03j`\xA0T\x81V[a\x04ua\x07\x9E6`\x04aC\x01V[`\xA1T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x90V[a\x07\xC5a\x07\xC06`\x04aIQV[a\x16\x7FV[`@Qa\x03t\x91\x90aI\xF6V[a\x03Ia\x07\xE06`\x04aJ>V[a\x16\x8DV[a\x03j\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81V[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x9CTa\x03jV[`\xA1Ta\x03\xBB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x08\xBCa\x08\\6`\x04aCQV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x97\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x03tV[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ia\t%6`\x04aC\x01V[a\x186V[a\x03Ia\t86`\x04aB\x02V[a\x18\xACV[a\tka\tK6`\x04aC\x01V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x03t\x91\x90aJ\xCBV[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\t\xA1W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82Q\x81\x10\x15a\x0B\x14W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x82\x81Q\x81\x10a\t\xDFWa\t\xDFaJ\xD9V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a\t\xF9Wa\t\xF9aJ\xD9V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\x99_\x86\x85\x81Q\x81\x10a\nJWa\nJaJ\xD9V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x01T\x81_\x81Q\x81\x10a\n\x86Wa\n\x86aJ\xD9V[` \x02` \x01\x01\x81\x81RPP_a\n\xB5\x82_\x81Q\x81\x10a\n\xA8Wa\n\xA8aJ\xD9V[` \x02` \x01\x01Qa\x19\xC3V[\x90P_a\n\xCA\x82`\x01`\x01`\xC0\x1B\x03\x16a\x19\xCFV[\x90P_[\x81Q\x81\x10\x15a\x0B\x03Wa\n\xFB\x85\x85\x84\x84\x81Q\x81\x10a\n\xEEWa\n\xEEaJ\xD9V[\x01` \x01Q`\xF8\x1Ca\x1A\x98V[`\x01\x01a\n\xCEV[PP`\x01\x90\x93\x01\x92Pa\t\xA3\x91PPV[PPPV[_a\x0B'`\x98\x85\x85\x85a\x1B\xD0V[\x90P[\x93\x92PPPV[`\x013_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x0BYWa\x0BYaG9V[\x14a\x0BwW`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x99` R`@\x90 Ta\x0B\x90\x90\x82a\x1C\xB4V[PV[a\x0B\x9Ba\x1D_V[`\xA0UV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C&\x91\x90aJ\xFCV[a\x0CCW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x81\x81\x16\x81\x14a\x0ChW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Cq\x82a\x1D\xB9V[PPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R_\x83\x81R`\x98` R`@\x90 \x80T\x83\x90\x81\x10a\x0C\xB0Wa\x0C\xB0aJ\xD9V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rhW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xFD\x91\x90aK\x15V[a\r\x94a\x1D_V[a\x0B\x90\x81a\x1D\xF6V[a\r\xA5a\x1D_V[a\x0B\x90\x81a\x1E_V[a\r\xB6a\x1E\xC8V[`\x01\x80T`\x02\x90\x81\x16\x03a\r\xDDW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x0E\x0BW`@Qcf\xE5e\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0E\x15\x83a\x1F\x11V[\x90Pa\x0E\"\x85\x82_a\x1F\xB9V[PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x0C\xFDa\x0E\xA2\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x84`@Q` \x01a\x0E\x87\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\"\x87V[a\"\xD3V[a\x0E\xAFa\x1D_V[a\x0E\xBD\x84\x84\x84`\x01\x85a#]V[PPPPV[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\x0E\xECW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F1\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa'\x88\x90PV[P\x83Q\x82\x14a\x0FSW`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\x0E\"W_\x84\x84\x83\x81\x81\x10a\x0FpWa\x0FpaJ\xD9V[\x88Q\x92\x015`\xF8\x1C\x92P_\x91\x88\x91P\x84\x90\x81\x10a\x0F\x8FWa\x0F\x8FaJ\xD9V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10&\x91\x90aK0V[c\xFF\xFF\xFF\xFF\x16\x81Q\x14a\x10LW`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10fWa\x10fa@\x9AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\x8FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x83Q\x81\x10\x15a\x11\x85W_\x84\x82\x81Q\x81\x10a\x10\xB1Wa\x10\xB1aJ\xD9V[` \x02` \x01\x01Q\x90P`\x99_\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x01T\x84\x83\x81Q\x81\x10a\x10\xF3Wa\x10\xF3aJ\xD9V[` \x02` \x01\x01\x81\x81RPP_a\x11\x15\x85\x84\x81Q\x81\x10a\n\xA8Wa\n\xA8aJ\xD9V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x89\x16\x1C\x81\x16\x14a\x11HW`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x11a\x11zW`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x91P`\x01\x01a\x10\x95V[Pa\x11\x91\x83\x83\x86a\x1A\x98V[`\xFF\x84\x16_\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPPP\x80`\x01\x01\x90Pa\x0FUV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\x08WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12!WP0;\x15\x80\x15a\x12!WP_T`\xFF\x16`\x01\x14[a\x12\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12\xAAW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12\xB3\x86a'\xBCV[a\x12\xBC\x85a\x1D\xF6V[a\x12\xC5\x83a\x1D\xB9V[a\x12\xCE\x84a\x1E_V[a\x12\xD7\x82a(\rV[`\x9C\x80T`\x01\x81\x81\x01\x83U_\x83\x90R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x91\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U\x84T\x80\x84\x01\x86U\x84\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x84T\x80\x84\x01\x86U\x84\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x84T\x92\x83\x01\x90\x94U\x91\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x92\x16\x17\x90U\x80\x15a\x14+W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x99` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x14\x8BWa\x14\x8BaG9V[`\x02\x81\x11\x15a\x14\x9CWa\x14\x9CaG9V[\x90RP\x92\x91PPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15+\x91\x90aJ\xFCV[a\x15HW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15R_\x19a\x1D\xB9V[V[a\x15\\a\x1D_V[\x81a\x15f\x81a(/V[a\x0B\x14\x83\x83a(XV[`\x9C\x81\x81T\x81\x10a\x15\x7FW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[a\x15\xA0a(\xFDV[a\x0Cq\x82\x82`\x01a)(V[a\x15\xB4a\x1D_V[a\x15R_a'\xBCV[_a\x0C\xFD\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x83`@Q` \x01a\x0E\x87\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[a\x16\x0Ea\x1D_V[a\x0B\x14\x83\x83\x83__a#]V[_a\x16Z\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87\x87\x87\x87\x87`@Q` \x01a\x0E\x87\x96\x95\x94\x93\x92\x91\x90aKKV[\x96\x95PPPPPPV[_a\x0C\xFD\x82a\x19\xC3V[a\x16va\x1D_V[a\x0B\x90\x81a(\rV[``a\x0B*`\x98\x84\x84a)\xEAV[a\x16\x95a\x1E\xC8V[`\x01\x80T_\x91\x90\x81\x16\x03a\x16\xBCW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\x16\xEAW`@Qcf\xE5e\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x16\xF4\x85a\x1F\x11V[\x90P_\x80\x80a\x17\x05\x86\x88\x01\x88aL\xCFV[\x92P\x92P\x92P_a\x17\x16\x8B\x83a*\x99V[\x90P_\x84`\x01\x81\x11\x15a\x17+Wa\x17+aG9V[\x03a\x17\xD4W_a\x17?\x8C\x83\x88\x87`\x01a+\xC7V[Q\x90P_[\x86Q\x81\x10\x15a\x17\xCDW_\x87\x82\x81Q\x81\x10a\x17`Wa\x17`aJ\xD9V[\x01` \x90\x81\x01Q`\xF8\x1C_\x81\x81R`\x97\x90\x92R`@\x90\x91 T\x84Q\x91\x92Pc\xFF\xFF\xFF\xFF\x16\x90\x84\x90\x84\x90\x81\x10a\x17\x97Wa\x17\x97aJ\xD9V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x17\xC4W`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x17DV[PPa\x18)V[`\x01\x84`\x01\x81\x11\x15a\x17\xE8Wa\x17\xE8aG9V[\x03a\x18\x10W_\x80a\x17\xFB\x89\x8B\x01\x8BaM*V[\x94P\x94PPPPa\x17\xCD\x8D\x84\x89\x88\x86\x86a0\x10V[`@Qc5K\xB8\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[a\x18>a\x1D_V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x12\x80V[a\x0B\x90\x81a'\xBCV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19,\x91\x90aK\x15V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19]W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x80\x19\x82\x19\x81\x16\x14a\x19\x84W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPV[_a\x0C\xFD`\x98\x83a2\x06V[``__a\x19\xDC\x84a2pV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xF7Wa\x19\xF7a@\x9AV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1A!W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_\x80[\x82Q\x82\x10\x80\x15a\x1A8WPa\x01\0\x81\x10[\x15a\x1A\x8EW`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x1A~W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1AaWa\x1AaaJ\xD9V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x81`\x01\x01\x91P[a\x1A\x87\x81aN7V[\x90Pa\x1A'V[P\x90\x94\x93PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x81`\xF8\x1B\x81_\x81Q\x81\x10a\x1A\xCEWa\x1A\xCEaJ\xD9V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`@Qcl?\xB4\xBF`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cl?\xB4\xBF\x90a\x1B6\x90\x88\x90\x88\x90\x88\x90`\x04\x01aNOV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1BQW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1Bx\x91\x90\x81\x01\x90aN\xDFV[\x90P_[\x85Q\x81\x10\x15a\x14+W\x81\x81\x81Q\x81\x10a\x1B\x97Wa\x1B\x97aJ\xD9V[` \x02` \x01\x01Q\x15a\x1B\xC8Wa\x1B\xC8\x86\x82\x81Q\x81\x10a\x1B\xB9Wa\x1B\xB9aJ\xD9V[` \x02` \x01\x01Q\x84_a)(V[`\x01\x01a\x1B|V[_\x83\x81R` \x85\x90R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1B\xF2Wa\x1B\xF2aJ\xD9V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\x1CbW`@Qcl\xB1\x9A\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x1C\x88WP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x1C\xA5W`@Qc\xBB\xBA`\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x01Q\x90P[\x94\x93PPPPV[`@Qcx!\x9B?`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF0C6~\x90a\x1D\x02\x90\x85\x90\x85\x90`\x04\x01aO\x9AV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1D\x19W__\xFD[PZ\xF1\x15\x80\x15a\x1D+W=__>=_\xFD[PPPP\x81\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x82`@Qa\x19\xB7\x91\x90aO\xB2V[`dT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x12\x80V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x15RW`@Qc#\xD8q\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F-Wa\x1F-a@\x9AV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1FWW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1F\xB2W\x83\x81\x81Q\x81\x10a\x1FwWa\x1FwaJ\xD9V[` \x02` \x01\x01Q`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1F\x94Wa\x1F\x94aJ\xD9V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x1F\\V[P\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x99` R`@\x81 \x80T\x90\x91a\x1F\xDD\x82a\x19\xC3V[\x90P`\x01\x80\x84\x01T`\xFF\x16`\x02\x81\x11\x15a\x1F\xF9Wa\x1F\xF9aG9V[\x14a \x17W`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x96T_\x90a *\x90\x87\x90`\xFF\x16a'\x88V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a SW`@Qch\xB6\xA8u`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a j`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x90\x84\x16\x81\x16\x14\x90V[a \x87W`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xC0\x1B\x03\x81\x81\x16\x19\x83\x16\x16a \xA0\x84\x82a2\x9AV[`\x01`\x01`\xC0\x1B\x03\x81\x16a \xFCW`\x01`\x01`\xA0\x1B\x03\x88\x16_\x81\x81R`\x99` R`@\x80\x82 `\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UQ\x86\x92\x91\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x91\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a!J\x90\x8B\x90\x8B\x90`\x04\x01aO\xC4V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!aW__\xFD[PZ\xF1\x15\x80\x15a!sW=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa!\xC5\x90\x87\x90\x8B\x90`\x04\x01aO\x9AV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!\xDCW__\xFD[PZ\xF1\x15\x80\x15a!\xEEW=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa\"@\x90\x87\x90\x8B\x90`\x04\x01aO\x9AV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"WW__\xFD[PZ\xF1\x15\x80\x15a\"iW=__>=_\xFD[PPPP\x85\x15a\"}Wa\"}\x88\x88a2\xA6V[PPPPPPPPV[_a\x0C\xFDa\"\x93a4\xB5V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R_\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a#\0_Q` aU*_9_Q\x90_R\x86aO\xFBV[\x90P[a#\x0C\x81a5\xDBV[\x90\x93P\x91P_Q` aU*_9_Q\x90_R\x82\x83\t\x83\x03a#DW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` aU*_9_Q\x90_R`\x01\x82\x08\x90Pa#\x03V[`\x96T`\xFF\x16`\xC0\x81\x10a#\x84W`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x96\x80T`\x01\x91\x90_\x90a#\x9C\x90\x84\x90`\xFF\x16aP\x0EV[\x92Pa\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UPa#\xBE\x81\x87a(XV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a#\xD4W\x90PP\x90P_\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x17Wa$\x17a@\x9AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$@W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x86Q\x81\x10\x15a$\x9DW\x86\x81\x81Q\x81\x10a$`Wa$`aJ\xD9V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a$}Wa$}aJ\xD9V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a$EV[P`@Q\x80`@\x01`@R\x80\x84`\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x82_\x81Q\x81\x10a$\xCEWa$\xCEaJ\xD9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xA1T`@Qc\x010\xFC'`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92c&\x1F\x84\xE0\x92a%.\x92\x91\x90\x91\x16\x90\x86\x90`\x04\x01aP'V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%EW__\xFD[PZ\xF1\x15\x80\x15a%WW=__>=_\xFD[P_\x92Pa%c\x91PPV[\x85`\x01\x81\x11\x15a%uWa%uaG9V[\x03a%\xFCW`@Qc:\xEA\x0B\x9D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cu\xD4\x17:\x90a%\xCA\x90\x86\x90\x8B\x90\x8B\x90`\x04\x01aQAV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%\xE1W__\xFD[PZ\xF1\x15\x80\x15a%\xF3W=__>=_\xFD[PPPPa&\x95V[`\x01\x85`\x01\x81\x11\x15a&\x10Wa&\x10aG9V[\x03a&\x95W`@Qc\x06b\xD3\xE1`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xCCZ| \x90a&g\x90\x86\x90\x8B\x90\x89\x90\x8C\x90`\x04\x01aQtV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&~W__\xFD[PZ\xF1\x15\x80\x15a&\x90W=__>=_\xFD[PPPP[`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c&\xD9A\xF2\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&\xF6W__\xFD[PZ\xF1\x15\x80\x15a'\x08W=__>=_\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'mW__\xFD[PZ\xF1\x15\x80\x15a'\x7FW=__>=_\xFD[PPPPa\"}V[__a'\x93\x84a6WV[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x0B*W`@Qc\xCA\x95s3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\xA1\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x96T`\xFF\x90\x81\x16\x90\x82\x16\x10a\x0B\x90W`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x16_\x81\x81R`\x97` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01a\x19\xB7V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15RW`@Qcv\xD8\xAB\x17`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a)IW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x9F` R`@\x90 B\x90U[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x99` R`@\x81 \x80T`\x96T\x91\x92\x90\x91a)w\x90\x86\x90`\xFF\x16a'\x88V[\x90P_a)\x83\x83a\x19\xC3V[\x90P`\x01\x80\x85\x01T`\xFF\x16`\x02\x81\x11\x15a)\x9FWa)\x9FaG9V[\x14\x80\x15a)\xB4WP`\x01`\x01`\xC0\x1B\x03\x82\x16\x15\x15[\x80\x15a)\xD2WPa)\xD2`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[\x15a)\xE1Wa)\xE1\x87\x87a2\xA6V[PPPPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x06Wa*\x06a@\x9AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a*/W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a*\x90Wa*a\x86\x86\x86\x84\x81Q\x81\x10a*TWa*TaJ\xD9V[` \x02` \x01\x01Qa7\x12V[\x82\x82\x81Q\x81\x10a*sWa*saJ\xD9V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a*4V[P\x94\x93PPPPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x01W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+%\x91\x90aQ\xAAV[\x90P_\x81\x90\x03a\x0C\xFDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBFy\xCEX\x84\x84a+i\x87a\x0E)V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\x87\x93\x92\x91\x90aQ\xE3V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B*\x91\x90aQ\xAAV[a+\xEB`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\x96T_\x90a+\xFE\x90\x86\x90`\xFF\x16a'\x88V[\x90P_a,\n\x87a\x19\xC3V[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a,3W`@Qc\x13\xCAFW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a,]W`@Qc\x0Ch\x16\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA0T`\x01`\x01`\xA0\x1B\x03\x89\x16_\x90\x81R`\x9F` R`@\x90 T`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17\x91B\x91a,\x94\x91\x90aR\\V[\x10a,\xB2W`@Qc\x19hg}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,\xBC\x88\x82a2\x9AV[a,\xC6\x88\x87a\x1C\xB4V[`\x01`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a,\xF7Wa,\xF7aG9V[\x14a-\x8EW`@\x80Q\x80\x82\x01\x82R\x89\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x16_\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a-QWa-QaG9V[\x02\x17\x90UPP`@Q\x89\x91P`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90_\x90\xA3[`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a-\xDC\x90\x8C\x90\x8B\x90`\x04\x01aO\xC4V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\xF3W__\xFD[PZ\xF1\x15\x80\x15a.\x05W=__>=_\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa.Y\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01aRoV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a.tW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra.\x9B\x91\x90\x81\x01\x90aR\xF9V[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a.\xF6\x90\x8B\x90\x8B\x90`\x04\x01aO\x9AV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a/\x11W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra/8\x91\x90\x81\x01\x90aSRV[\x84R\x84\x15a0\x04W_[\x87Q\x81\x10\x15a0\x02W_`\x97_\x8A\x84\x81Q\x81\x10a/aWa/aaJ\xD9V[\x01` \x90\x81\x01Q`\xF8\x1C\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x87Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a/\xCCWa/\xCCaJ\xD9V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a/\xF9W`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a/BV[P[PPP\x95\x94PPPPPV[\x83Q\x82Q\x14a02W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a0>\x86\x86\x84\x84a8*V[_a0L\x87\x87\x87\x87_a+\xC7V[\x90P_[\x85Q\x81\x10\x15a\"}W_`\x97_\x88\x84\x81Q\x81\x10a0oWa0oaJ\xD9V[\x01` \x90\x81\x01Q`\xF8\x1C\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a0\xDAWa0\xDAaJ\xD9V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a1\xFDWa1n\x87\x83\x81Q\x81\x10a1\x03Wa1\x03aJ\xD9V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a1'Wa1'aJ\xD9V[` \x02` \x01\x01Q\x8B\x86` \x01Q\x86\x81Q\x81\x10a1FWa1FaJ\xD9V[` \x02` \x01\x01Q\x89\x87\x81Q\x81\x10a1`Wa1`aJ\xD9V[` \x02` \x01\x01Q\x86a8\xD5V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a1\xA0Wa1\xA0aJ\xD9V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x81_\x81Q\x81\x10a1\xBDWa1\xBDaJ\xD9V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa1\xFB\x86\x84\x81Q\x81\x10a1\xE8Wa1\xE8aJ\xD9V[` \x02` \x01\x01Q` \x01Q\x82_a)(V[P[P`\x01\x01a0PV[_\x81\x81R` \x83\x90R`@\x81 T\x80\x82\x03a2$W_\x91PPa\x0C\xFDV[_\x83\x81R` \x85\x90R`@\x90 a2<`\x01\x83aS\xE1V[\x81T\x81\x10a2LWa2LaJ\xD9V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x91Pa\x0C\xFD\x90PV[_\x80[\x82\x15a\x0C\xFDWa2\x84`\x01\x84aS\xE1V[\x90\x92\x16\x91\x80a2\x92\x81aS\xF4V[\x91PPa2sV[a\x0Cq`\x98\x83\x83a:VV[_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xC0Wa2\xC0a@\x9AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\xE9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x83Q\x81\x10\x15a4\x13W_\x84\x82\x81Q\x81\x10a3\x0BWa3\x0BaJ\xD9V[` \x91\x01\x81\x01Q`@\x80Q\x80\x82\x01\x82R`\xA1T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\xF8\x93\x90\x93\x1C\x93\x81\x01\x84\x81R\x91Qc3\x86\x9D\xD1`\xE1\x1B\x81R\x8A\x84\x16`\x04\x82\x01R\x90Q\x83\x16`$\x82\x01R\x90Qc\xFF\xFF\xFF\xFF\x16`D\x82\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cg\r;\xA2\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xCF\x91\x90aJ\xFCV[\x15a4\nW\x80\x84\x84\x81Q\x81\x10a3\xE7Wa3\xE7aJ\xD9V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a4\x06\x81aN7V[\x93PP[P`\x01\x01a2\xEFV[P\x80\x82R`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R`\xA1T\x81\x16` \x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91cn4\x92\xB5\x91\x90\x81\x01a4m\x87a<\x0FV[\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\x8C\x91\x90aT\x14V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a4\xA3W__\xFD[PZ\xF1\x15\x80\x15a\"}W=__>=_\xFD[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a5\rWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a57WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[_\x80\x80_Q` aU*_9_Q\x90_R`\x03_Q` aU*_9_Q\x90_R\x86_Q` aU*_9_Q\x90_R\x88\x89\t\t\x08\x90P_a6K\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` aU*_9_Q\x90_Ra<\xB4V[\x91\x95\x91\x94P\x90\x92PPPV[_a\x01\0\x82Q\x11\x15a6|W`@Qc}\xA5NG`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q_\x03a6\x8BWP_\x91\x90PV[__\x83_\x81Q\x81\x10a6\x9FWa6\x9FaJ\xD9V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a7\tW\x84\x81\x81Q\x81\x10a6\xCDWa6\xCDaJ\xD9V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a6\xFDW`@Qc\x10\x19\x10i`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x81\x17\x91`\x01\x01a6\xB2V[P\x90\x93\x92PPPV[_\x81\x81R` \x84\x90R`@\x81 T\x81[\x81\x81\x10\x15a7\x95W`\x01a76\x82\x84aS\xE1V[a7@\x91\x90aS\xE1V[\x92P\x84c\xFF\xFF\xFF\xFF\x16\x86_\x86\x81R` \x01\x90\x81R` \x01_ \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a7pWa7paJ\xD9V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a7\x8DWPPa\x0B*V[`\x01\x01a7\"V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x12\x80V[` \x80\x82\x01Q_\x90\x81R`\x9A\x90\x91R`@\x90 T`\xFF\x16\x15a8_W`@Qco\xBE\xFE\xC3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x81`@\x01Q\x10\x15a8\x84W`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x82\x01\x80Q_\x90\x81R`\x9A\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9DT\x90Q\x91\x83\x01Qa\x0E\xBD\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a8\xCE\x91\x88\x91\x88\x91\x88\x91\x90a\x16\x1BV[\x83Qa=-V[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x81\x81R`\x99\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x03a9\x18W`@QcV\x16\x8BA`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87`\xFF\x16\x84_\x01Q`\xFF\x16\x14a9AW`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xD3\x91\x90aT\x82V[\x90Pa9\xDF\x81\x85a=UV[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11a:\x10W`@QcLD\x99]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a:\x1A\x88\x85a=xV[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a:KW`@Qc\xB1\x87\xE8i`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPV[_\x82\x81R` \x84\x90R`@\x81 T\x90\x81\x90\x03a:\xFAW_\x83\x81R` \x85\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\x0E\xBDV[_\x83\x81R` \x85\x90R`@\x81 a;\x12`\x01\x84aS\xE1V[\x81T\x81\x10a;\"Wa;\"aJ\xD9V[_\x91\x82R` \x90\x91 \x01\x80T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a;cW\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\x0E\"V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U_\x87\x81R` \x89\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a<+Wa<+a@\x9AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<TW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1F\xB2W\x83\x81\x81Q\x81\x10a<tWa<taJ\xD9V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x82\x82\x81Q\x81\x10a<\x97Wa<\x97aJ\xD9V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a<YV[__a<\xBEa@^V[a<\xC6a@|V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a=\x03W\xFE[P\x82a=\"W`@Qc\xD5\x1E\xDA\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x95\x94PPPPPV[a=8\x83\x83\x83a=\x91V[a\x0B\x14W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_\x90a'\x10\x90a=n\x90a\xFF\xFF\x16\x85aT\x9DV[a\x0B*\x91\x90aT\xBFV[`@\x81\x01Q_\x90a'\x10\x90a=n\x90a\xFF\xFF\x16\x85aT\x9DV[___a=\x9E\x85\x85a>\xD6V[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a=\xB6Wa=\xB6aG9V[\x14\x80\x15a=\xD4WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a=\xE4W`\x01\x92PPPa\x0B*V[__\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a>\x0B\x92\x91\x90aO\x9AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa>I\x91\x90aT\xECV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a>\x81W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a>\x86V[``\x91P[P\x91P\x91P\x81\x80\x15a>\x99WP\x80Q` \x14[\x80\x15a>\xCAWP\x80Qc\x0B\x13]?`\xE1\x1B\x90a>\xBE\x90\x83\x01` \x90\x81\x01\x90\x84\x01aU\x02V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[__\x82Q`A\x03a?\nW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa>\xFE\x87\x82\x85\x85a?AV[\x94P\x94PPPPa?:V[\x82Q`@\x03a?3W` \x83\x01Q`@\x84\x01Qa?(\x86\x83\x83a@&V[\x93P\x93PPPa?:V[P_\x90P`\x02[\x92P\x92\x90PV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a?vWP_\x90P`\x03a@\x1DV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a?\x8EWP\x84`\xFF\x16`\x1C\x14\x15[\x15a?\x9EWP_\x90P`\x04a@\x1DV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a?\xEFW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a@\x17W_`\x01\x92P\x92PPa@\x1DV[\x91P_\x90P[\x94P\x94\x92PPPV[_\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a@B`\xFF\x86\x90\x1C`\x1BaR\\V[\x90Pa@P\x87\x82\x88\x85a?AV[\x93P\x93PPP\x93P\x93\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\xD0Wa@\xD0a@\x9AV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\xD0Wa@\xD0a@\x9AV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aA WaA a@\x9AV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aA@WaA@a@\x9AV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x90W__\xFD[_\x82`\x1F\x83\x01\x12aAmW__\xFD[\x815aA\x80aA{\x82aA(V[a@\xF8V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aA\xA1W__\xFD[` \x85\x01[\x83\x81\x10\x15aA\xC7W\x805aA\xB9\x81aAJV[\x83R` \x92\x83\x01\x92\x01aA\xA6V[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15aA\xE1W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xF6W__\xFD[a\x1C\xAC\x84\x82\x85\x01aA^V[_` \x82\x84\x03\x12\x15aB\x12W__\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B\x90W__\xFD[___``\x84\x86\x03\x12\x15aB<W__\xFD[\x835\x92P` \x84\x015aBN\x81aB\x19V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_\x82`\x1F\x83\x01\x12aBnW__\xFD[\x815` \x83\x01__`\x01`\x01`@\x1B\x03\x84\x11\x15aB\x8DWaB\x8Da@\x9AV[P`\x1F\x83\x01`\x1F\x19\x16` \x01aB\xA2\x81a@\xF8V[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15aB\xB6W__\xFD[\x82\x82` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15aB\xE0W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xF5W__\xFD[a\x1C\xAC\x84\x82\x85\x01aB_V[_` \x82\x84\x03\x12\x15aC\x11W__\xFD[\x815a\x0B*\x81aAJV[__`@\x83\x85\x03\x12\x15aC-W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14aCLW__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15aCaW__\xFD[a\x0B*\x82aC<V[_\x82`\x1F\x83\x01\x12aCyW__\xFD[\x815aC\x87aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aC\xA8W__\xFD[` \x85\x01[\x83\x81\x10\x15aA\xC7W\x805aC\xC0\x81aB\x19V[\x83R` \x92\x83\x01\x92\x01aC\xADV[___``\x84\x86\x03\x12\x15aC\xE0W__\xFD[\x835aC\xEB\x81aAJV[\x92P` \x84\x015aC\xFB\x81aAJV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD\x15W__\xFD[aD!\x86\x82\x87\x01aCjV[\x91PP\x92P\x92P\x92V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x0C\xFDV[\x805a\xFF\xFF\x81\x16\x81\x14aCLW__\xFD[_``\x82\x84\x03\x12\x15aDcW__\xFD[aDka@\xAEV[\x90P\x815aDx\x81aB\x19V[\x81RaD\x86` \x83\x01aDBV[` \x82\x01RaD\x97`@\x83\x01aDBV[`@\x82\x01R\x92\x91PPV[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x0B\x90W__\xFD[_\x82`\x1F\x83\x01\x12aD\xC5W__\xFD[\x815aD\xD3aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aD\xF4W__\xFD[` \x85\x01[\x83\x81\x10\x15aA\xC7W`@\x81\x88\x03\x12\x15aE\x10W__\xFD[aE\x18a@\xD6V[\x815aE#\x81aAJV[\x81R` \x82\x015aE3\x81aD\xA2V[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aD\xF9V[____`\xC0\x85\x87\x03\x12\x15aE_W__\xFD[aEi\x86\x86aDSV[\x93P``\x85\x015aEy\x81aD\xA2V[\x92P`\x80\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x93W__\xFD[aE\x9F\x87\x82\x88\x01aD\xB6V[\x92PP`\xA0\x85\x015aE\xB0\x81aB\x19V[\x93\x96\x92\x95P\x90\x93PPV[__\x83`\x1F\x84\x01\x12aE\xCBW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xE1W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a?:W__\xFD[___`@\x84\x86\x03\x12\x15aF\nW__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x1FW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aF/W__\xFD[\x805aF=aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15aF^W__\xFD[` \x84\x01[\x83\x81\x10\x15aF\x9EW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x80W__\xFD[aF\x8F\x8B` \x83\x89\x01\x01aA^V[\x84RP` \x92\x83\x01\x92\x01aFcV[P\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xBCW__\xFD[aF\xC8\x86\x82\x87\x01aE\xBBV[\x94\x97\x90\x96P\x93\x94PPPPV[_____`\xA0\x86\x88\x03\x12\x15aF\xE9W__\xFD[\x855aF\xF4\x81aAJV[\x94P` \x86\x015aG\x04\x81aAJV[\x93P`@\x86\x015aG\x14\x81aAJV[\x92P``\x86\x015\x91P`\x80\x86\x015aG+\x81aAJV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10aGiWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91a\x1F\xB2\x90\x84\x01\x82aGMV[__`\x80\x83\x85\x03\x12\x15aG\x99W__\xFD[aG\xA2\x83aC<V[\x91PaG\xB1\x84` \x85\x01aDSV[\x90P\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aG\xCBW__\xFD[\x825aG\xD6\x81aAJV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xF0W__\xFD[aG\xFC\x85\x82\x86\x01aB_V[\x91PP\x92P\x92\x90PV[___`\xA0\x84\x86\x03\x12\x15aH\x18W__\xFD[aH\"\x85\x85aDSV[\x92P``\x84\x015aH2\x81aD\xA2V[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aHLW__\xFD[aD!\x86\x82\x87\x01aD\xB6V[_\x82`\x1F\x83\x01\x12aHgW__\xFD[\x815aHuaA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aH\x96W__\xFD[` \x85\x01[\x83\x81\x10\x15aA\xC7W`@\x81\x88\x03\x12\x15aH\xB2W__\xFD[aH\xBAa@\xD6V[aH\xC3\x82aC<V[\x81R` \x82\x015aH\xD3\x81aAJV[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aH\x9BV[_____`\xA0\x86\x88\x03\x12\x15aI\0W__\xFD[\x855aI\x0B\x81aAJV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI,W__\xFD[aI8\x88\x82\x89\x01aHXV[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x015\x93\x92PPPV[__`@\x83\x85\x03\x12\x15aIbW__\xFD[\x825aIm\x81aB\x19V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x87W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aI\x97W__\xFD[\x805aI\xA5aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aI\xC6W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aI\xE8W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aI\xCDV[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aJ3W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aJ\x0FV[P\x90\x95\x94PPPPPV[_____`\x80\x86\x88\x03\x12\x15aJRW__\xFD[\x855aJ]\x81aAJV[\x94P` \x86\x015aJm\x81aAJV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\x87W__\xFD[aJ\x93\x88\x82\x89\x01aCjV[\x93PP``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xAEW__\xFD[aJ\xBA\x88\x82\x89\x01aE\xBBV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[` \x81\x01a\x0C\xFD\x82\x84aGMV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x80\x15\x15\x81\x14aCLW__\xFD[_` \x82\x84\x03\x12\x15aK\x0CW__\xFD[a\x0B*\x82aJ\xEDV[_` \x82\x84\x03\x12\x15aK%W__\xFD[\x81Qa\x0B*\x81aAJV[_` \x82\x84\x03\x12\x15aK@W__\xFD[\x81Qa\x0B*\x81aB\x19V[_`\xC0\x82\x01\x88\x83R`\x01\x80`\xA0\x1B\x03\x88\x16` \x84\x01R\x86`@\x84\x01R`\xC0``\x84\x01R\x80\x86Q\x80\x83R`\xE0\x85\x01\x91P` \x88\x01\x92P_[\x81\x81\x10\x15aK\xB8W\x83Q\x80Q`\xFF\x16\x84R` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01aK\x82V[PP`\x80\x84\x01\x95\x90\x95RPP`\xA0\x01R\x94\x93PPPPV[\x805`\x02\x81\x10aCLW__\xFD[_`@\x82\x84\x03\x12\x15aK\xEEW__\xFD[aK\xF6a@\xD6V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12aL\x1CW__\xFD[aL$a@\xD6V[\x80`@\x84\x01\x85\x81\x11\x15aL5W__\xFD[\x84[\x81\x81\x10\x15aJ3W\x805\x84R` \x93\x84\x01\x93\x01aL7V[_\x81\x83\x03a\x01\0\x81\x12\x15aLaW__\xFD[aLia@\xAEV[\x91PaLu\x84\x84aK\xDEV[\x82RaL\x84\x84`@\x85\x01aK\xDEV[` \x83\x01R`\x80`\x7F\x19\x82\x01\x12\x15aL\x9AW__\xFD[PaL\xA3a@\xD6V[aL\xB0\x84`\x80\x85\x01aL\rV[\x81RaL\xBF\x84`\xC0\x85\x01aL\rV[` \x82\x01R`@\x82\x01R\x92\x91PPV[___a\x01@\x84\x86\x03\x12\x15aL\xE2W__\xFD[aL\xEB\x84aK\xD0V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x05W__\xFD[aM\x11\x86\x82\x87\x01aB_V[\x92PPaM!\x85`@\x86\x01aLOV[\x90P\x92P\x92P\x92V[_____a\x01\x80\x86\x88\x03\x12\x15aM?W__\xFD[aMH\x86aK\xD0V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aMbW__\xFD[aMn\x88\x82\x89\x01aB_V[\x94PPaM~\x87`@\x88\x01aLOV[\x92Pa\x01@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x99W__\xFD[aM\xA5\x88\x82\x89\x01aHXV[\x92PPa\x01`\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xC1W__\xFD[\x86\x01``\x81\x89\x03\x12\x15aM\xD2W__\xFD[aM\xDAa@\xAEV[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xEFW__\xFD[aM\xFB\x8A\x82\x85\x01aB_V[\x82RP` \x82\x81\x015\x90\x82\x01R`@\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x94\x97\x93\x96P\x91\x94P\x92\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01aNHWaNHaN#V[P`\x01\x01\x90V[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R_\x90` \x86\x01\x90`\x80\x84\x01\x90\x83[\x81\x81\x10\x15aN\x91W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aNjV[PP\x83\x81\x03` \x80\x86\x01\x91\x90\x91R\x86Q\x80\x83R\x91\x81\x01\x92P\x86\x01\x90_[\x81\x81\x10\x15aN\xCCW\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aN\xAEV[PPP`\xFF\x84\x16`@\x84\x01R\x90Pa\x1C\xACV[_` \x82\x84\x03\x12\x15aN\xEFW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\x04W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aO\x14W__\xFD[\x80QaO\"aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aOCW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x16ZWaO[\x84aJ\xEDV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aOJV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x82\x81R`@` \x82\x01R_a\x0B'`@\x83\x01\x84aOlV[` \x81R_a\x0B*` \x83\x01\x84aOlV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x0B'\x90\x83\x01\x84aOlV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aP\tWaP\taO\xE7V[P\x06\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xFDWa\x0C\xFDaN#V[_`@\x82\x01`\x01\x80`\xA0\x1B\x03\x85\x16\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aP\xDCW\x86\x85\x03`_\x19\x01\x84R\x81Q\x80Qc\xFF\xFF\xFF\xFF\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15aP\xC4W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90aP\x99V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aP[V[P\x92\x97\x96PPPPPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aQ7W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x88\x01R`@\x90\x96\x01\x95\x90\x91\x01\x90`\x01\x01aP\xFBV[P\x93\x94\x93PPPPV[`\xFF\x84\x16\x81R`\x01`\x01``\x1B\x03\x83\x16` \x82\x01R```@\x82\x01R_aQk``\x83\x01\x84aP\xE9V[\x95\x94PPPPPV[`\xFF\x85\x16\x81R`\x01`\x01``\x1B\x03\x84\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R_a\x16Z`\x80\x83\x01\x84aP\xE9V[_` \x82\x84\x03\x12\x15aQ\xBAW__\xFD[PQ\x91\x90PV[\x80_[`\x02\x81\x10\x15a\x0E\xBDW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aQ\xC4V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R\x82Q\x80Q` \x80\x84\x01\x91\x90\x91R\x01Q`@\x82\x01Ra\x01`\x81\x01` \x84\x81\x01Q\x80Q``\x85\x01R\x90\x81\x01Q`\x80\x84\x01RP`@\x84\x01QaR2`\xA0\x84\x01\x82QaQ\xC1V[` \x01QaRC`\xE0\x84\x01\x82aQ\xC1V[P\x82Qa\x01 \x83\x01R` \x83\x01Qa\x01@\x83\x01Ra\x1C\xACV[\x80\x82\x01\x80\x82\x11\x15a\x0C\xFDWa\x0C\xFDaN#V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R_aQk``\x83\x01\x84aOlV[_\x82`\x1F\x83\x01\x12aR\xA4W__\xFD[\x81QaR\xB2aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aR\xD3W__\xFD[` \x85\x01[\x83\x81\x10\x15aA\xC7W\x80QaR\xEB\x81aD\xA2V[\x83R` \x92\x83\x01\x92\x01aR\xD8V[__`@\x83\x85\x03\x12\x15aS\nW__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x1FW__\xFD[aS+\x85\x82\x86\x01aR\x95V[\x92PP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aSFW__\xFD[aG\xFC\x85\x82\x86\x01aR\x95V[_` \x82\x84\x03\x12\x15aSbW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aSwW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aS\x87W__\xFD[\x80QaS\x95aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aS\xB6W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x16ZW\x83QaS\xD0\x81aB\x19V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aS\xBDV[\x81\x81\x03\x81\x81\x11\x15a\x0C\xFDWa\x0C\xFDaN#V[_a\xFF\xFF\x82\x16a\xFF\xFF\x81\x03aT\x0BWaT\x0BaN#V[`\x01\x01\x92\x91PPV[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x83\x01R\x83\x82\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x83\x01Q``\x80\x84\x01R\x80Q`\x80\x84\x01\x81\x90R_\x92\x91\x90\x91\x01\x90\x82\x90`\xA0\x85\x01\x90[\x80\x83\x10\x15aA\xC7Wc\xFF\xFF\xFF\xFF\x84Q\x16\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92PaTYV[_` \x82\x84\x03\x12\x15aT\x92W__\xFD[\x81Qa\x0B*\x81aD\xA2V[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x1F\xB2Wa\x1F\xB2aN#V[_`\x01`\x01``\x1B\x03\x83\x16\x80aT\xD7WaT\xD7aO\xE7V[\x80`\x01`\x01``\x1B\x03\x84\x16\x04\x91PP\x92\x91PPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15aU\x12W__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B*W__\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xAC\xF4Oa\xBD\xE5\x1B|thM\tlU\xF7e?\xC5\xA4\x1D\xEES\xD9K\"\xAE\xDB\xD5\xA5\xE8\xD9fdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610332575f3560e01c80636347c900116101af578063a57f4c6f116100fe578063d72d8dd61161009e578063ea32afae11610079578063ea32afae146108f0578063f2fde38b14610917578063fabc1cbc1461092a578063fd39105a1461093d575f5ffd5b8063d72d8dd614610833578063de1164bb1461083b578063e65797ad1461084e575f5ffd5b8063c391425e116100d9578063c391425e146107b2578063c63fd502146107d2578063ca0de882146107e5578063ca8aa7c71461080c575f5ffd5b8063a57f4c6f14610774578063a96f783e14610787578063b526578714610790575f5ffd5b806384ca5213116101695780638da5cb5b116101445780638da5cb5b146106f65780639aa1653d146107075780639e9923c2146107265780639feab8591461074d575f5ffd5b806384ca5213146106a9578063871ef049146106bc578063886f1195146106cf575f5ffd5b80636347c9001461062e57806368304835146106415780636e3b17db14610668578063715018a61461067b57806373447992146106835780638281ab7514610696575f5ffd5b8063296bb06411610285578063530b97a4116102255780635ac86ab7116102005780635ac86ab7146105cd5780635b0b829f146105ec5780635c975abb146105ff5780635df4594614610607575f5ffd5b8063530b97a4146105925780635865c60c146105a5578063595c6a67146105c5575f5ffd5b8063303ca95611610260578063303ca956146105395780633c2a7f4c1461054c5780633eef3a511461056c5780635140a5481461057f575f5ffd5b8063296bb0641461050057806329d1e0c3146105135780632cdd1e8614610526575f5ffd5b8063125e0584116102f05780631478851f116102cb5780631478851f146104535780631eb812da14610485578063249a0c42146104ce57806328f61b31146104ed575f5ffd5b8063125e0584146103f957806313542a4e14610418578063136439dd14610440575f5ffd5b8062cf2ab51461033657806303fd34921461034b57806304ec63511461037d578063054310e6146103a85780630cf4b767146103d35780630d3f2134146103e6575b5f5ffd5b6103496103443660046141d1565b610978565b005b61036a610359366004614202565b5f9081526098602052604090205490565b6040519081526020015b60405180910390f35b61039061038b36600461422a565b610b19565b6040516001600160c01b039091168152602001610374565b609d546103bb906001600160a01b031681565b6040516001600160a01b039091168152602001610374565b6103496103e13660046142d0565b610b31565b6103496103f4366004614202565b610b93565b61036a610407366004614301565b609f6020525f908152604090205481565b61036a610426366004614301565b6001600160a01b03165f9081526099602052604090205490565b61034961044e366004614202565b610ba0565b610475610461366004614202565b609a6020525f908152604090205460ff1681565b6040519015158152602001610374565b61049861049336600461431c565b610c75565b60408051825163ffffffff908116825260208085015190911690820152918101516001600160c01b031690820152606001610374565b61036a6104dc366004614351565b609b6020525f908152604090205481565b609e546103bb906001600160a01b031681565b6103bb61050e366004614202565b610d03565b610349610521366004614301565b610d8c565b610349610534366004614301565b610d9d565b6103496105473660046143ce565b610dae565b61055f61055a366004614301565b610e29565b604051610374919061442b565b61034961057a36600461454c565b610ea7565b61034961058d3660046145f8565b610ec3565b6103496105a03660046146d5565b6111ea565b6105b86105b3366004614301565b611433565b604051610374919061476d565b6103496114a5565b6104756105db366004614351565b6001805460ff9092161b9081161490565b6103496105fa366004614788565b611554565b60015461036a565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b6103bb61063c366004614202565b611570565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b6103496106763660046147ba565b611598565b6103496115ac565b61036a610691366004614301565b6115bd565b6103496106a4366004614806565b611606565b61036a6106b73660046148ec565b61161b565b6103906106ca366004614202565b611664565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b6064546001600160a01b03166103bb565b6096546107149060ff1681565b60405160ff9091168152602001610374565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b61036a7f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de681565b610349610782366004614301565b61166e565b61036a60a05481565b61047561079e366004614301565b60a1546001600160a01b0391821691161490565b6107c56107c0366004614951565b61167f565b60405161037491906149f6565b6103496107e0366004614a3e565b61168d565b61036a7f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a81565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b609c5461036a565b60a1546103bb906001600160a01b031681565b6108bc61085c366004614351565b60408051606080820183525f808352602080840182905292840181905260ff9490941684526097825292829020825193840183525463ffffffff8116845261ffff600160201b8204811692850192909252600160301b9004169082015290565b60408051825163ffffffff16815260208084015161ffff908116918301919091529282015190921690820152606001610374565b6103bb7f000000000000000000000000000000000000000000000000000000000000000081565b610349610925366004614301565b611836565b610349610938366004614202565b6118ac565b61096b61094b366004614301565b6001600160a01b03165f9081526099602052604090206001015460ff1690565b6040516103749190614acb565b6001546002906004908116036109a15760405163840a48d560e01b815260040160405180910390fd5b5f5b8251811015610b14576040805160018082528183019092525f91602080830190803683370190505090508382815181106109df576109df614ad9565b6020026020010151815f815181106109f9576109f9614ad9565b6001600160a01b0392909216602092830291909101909101526040805160018082528183019092525f9181602001602082028036833701905050905060995f868581518110610a4a57610a4a614ad9565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f205f0154815f81518110610a8657610a86614ad9565b6020026020010181815250505f610ab5825f81518110610aa857610aa8614ad9565b60200260200101516119c3565b90505f610aca826001600160c01b03166119cf565b90505f5b8151811015610b0357610afb8585848481518110610aee57610aee614ad9565b016020015160f81c611a98565b600101610ace565b5050600190930192506109a3915050565b505050565b5f610b276098858585611bd0565b90505b9392505050565b6001335f9081526099602052604090206001015460ff166002811115610b5957610b59614739565b14610b775760405163aba4733960e01b815260040160405180910390fd5b335f90815260996020526040902054610b909082611cb4565b50565b610b9b611d5f565b60a055565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015610c02573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c269190614afc565b610c4357604051631d77d47760e21b815260040160405180910390fd5b6001548181168114610c685760405163c61dca5d60e01b815260040160405180910390fd5b610c7182611db9565b5050565b604080516060810182525f80825260208201819052918101919091525f838152609860205260409020805483908110610cb057610cb0614ad9565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160c01b03600160401b909304929092169082015290505b92915050565b6040516308f6629d60e31b8152600481018290525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906347b314e890602401602060405180830381865afa158015610d68573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cfd9190614b15565b610d94611d5f565b610b9081611df6565b610da5611d5f565b610b9081611e5f565b610db6611ec8565b60018054600290811603610ddd5760405163840a48d560e01b815260040160405180910390fd5b60a1546001600160a01b03848116911614610e0b576040516366e565df60e01b815260040160405180910390fd5b5f610e1583611f11565b9050610e2285825f611fb9565b5050505050565b604080518082019091525f8082526020820152610cfd610ea27f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de684604051602001610e879291909182526001600160a01b0316602082015260400190565b60405160208183030381529060405280519060200120612287565b6122d3565b610eaf611d5f565b610ebd84848460018561235d565b50505050565b600154600290600490811603610eec5760405163840a48d560e01b815260040160405180910390fd5b610f3183838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505060965460ff1691506127889050565b5083518214610f535760405163aaad13f760e01b815260040160405180910390fd5b5f5b82811015610e22575f848483818110610f7057610f70614ad9565b885192013560f81c92505f9188915084908110610f8f57610f8f614ad9565b60209081029190910101516040516379a0849160e11b815260ff841660048201529091506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f341092290602401602060405180830381865afa158015611002573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110269190614b30565b63ffffffff1681511461104c57604051638e5aeee760e01b815260040160405180910390fd5b5f81516001600160401b038111156110665761106661409a565b60405190808252806020026020018201604052801561108f578160200160208202803683370190505b5090505f805b8351811015611185575f8482815181106110b1576110b1614ad9565b6020026020010151905060995f826001600160a01b03166001600160a01b031681526020019081526020015f205f01548483815181106110f3576110f3614ad9565b6020026020010181815250505f611115858481518110610aa857610aa8614ad9565b905060016001600160c01b03821660ff89161c8116146111485760405163d053aa2160e01b815260040160405180910390fd5b836001600160a01b0316826001600160a01b03161161117a5760405163ba50f91160e01b815260040160405180910390fd5b509150600101611095565b50611191838386611a98565b60ff84165f818152609b6020908152604091829020439081905591519182527f46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db4910160405180910390a250505050806001019050610f55565b5f54610100900460ff161580801561120857505f54600160ff909116105b806112215750303b15801561122157505f5460ff166001145b6112895760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156112aa575f805461ff0019166101001790555b6112b3866127bc565b6112bc85611df6565b6112c583611db9565b6112ce84611e5f565b6112d78261280d565b609c8054600181810183555f8390527faf85b9071dfafeac1409d3f1d19bafc9bc7c37974cde8df0ee6168f0086e539c91820180546001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166001600160a01b03199283161790925584548084018655840180547f0000000000000000000000000000000000000000000000000000000000000000841690831617905584548084018655840180547f000000000000000000000000000000000000000000000000000000000000000084169083161790558454928301909455910180547f000000000000000000000000000000000000000000000000000000000000000090921691909216179055801561142b575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050565b604080518082019091525f80825260208201526001600160a01b0382165f908152609960209081526040918290208251808401909352805483526001810154909183019060ff16600281111561148b5761148b614739565b600281111561149c5761149c614739565b90525092915050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611507573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061152b9190614afc565b61154857604051631d77d47760e21b815260040160405180910390fd5b6115525f19611db9565b565b61155c611d5f565b816115668161282f565b610b148383612858565b609c818154811061157f575f80fd5b5f918252602090912001546001600160a01b0316905081565b6115a06128fd565b610c7182826001612928565b6115b4611d5f565b6115525f6127bc565b5f610cfd7f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de683604051602001610e879291909182526001600160a01b0316602082015260400190565b61160e611d5f565b610b148383835f5f61235d565b5f61165a7f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a8787878787604051602001610e8796959493929190614b4b565b9695505050505050565b5f610cfd826119c3565b611676611d5f565b610b908161280d565b6060610b2a609884846129ea565b611695611ec8565b600180545f91908116036116bc5760405163840a48d560e01b815260040160405180910390fd5b60a1546001600160a01b038681169116146116ea576040516366e565df60e01b815260040160405180910390fd5b5f6116f485611f11565b90505f808061170586880188614ccf565b9250925092505f6117168b83612a99565b90505f84600181111561172b5761172b614739565b036117d4575f61173f8c8388876001612bc7565b5190505f5b86518110156117cd575f87828151811061176057611760614ad9565b0160209081015160f81c5f8181526097909252604090912054845191925063ffffffff169084908490811061179757611797614ad9565b602002602001015163ffffffff1611156117c457604051633cb89c9760e01b815260040160405180910390fd5b50600101611744565b5050611829565b60018460018111156117e8576117e8614739565b03611810575f806117fb898b018b614d2a565b945094505050506117cd8d8489888686613010565b60405163354bb8ab60e01b815260040160405180910390fd5b5050505050505050505050565b61183e611d5f565b6001600160a01b0381166118a35760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401611280565b610b90816127bc565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611908573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061192c9190614b15565b6001600160a01b0316336001600160a01b03161461195d5760405163794821ff60e01b815260040160405180910390fd5b600154801982198116146119845760405163c61dca5d60e01b815260040160405180910390fd5b600182905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020015b60405180910390a25050565b5f610cfd609883613206565b60605f5f6119dc84613270565b61ffff166001600160401b038111156119f7576119f761409a565b6040519080825280601f01601f191660200182016040528015611a21576020820181803683370190505b5090505f805b825182108015611a38575061010081105b15611a8e576001811b935085841615611a7e578060f81b838381518110611a6157611a61614ad9565b60200101906001600160f81b03191690815f1a9053508160010191505b611a8781614e37565b9050611a27565b5090949350505050565b6040805160018082528183019092525f916020820181803683370190505090508160f81b815f81518110611ace57611ace614ad9565b60200101906001600160f81b03191690815f1a905350604051636c3fb4bf60e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636c3fb4bf90611b3690889088908890600401614e4f565b5f604051808303815f875af1158015611b51573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611b789190810190614edf565b90505f5b855181101561142b57818181518110611b9757611b97614ad9565b602002602001015115611bc857611bc8868281518110611bb957611bb9614ad9565b6020026020010151845f612928565b600101611b7c565b5f838152602085905260408120805482919084908110611bf257611bf2614ad9565b5f91825260209182902060408051606081018252929091015463ffffffff808216808552600160201b8304821695850195909552600160401b9091046001600160c01b03169183019190915290925085161015611c6257604051636cb19aff60e01b815260040160405180910390fd5b602081015163ffffffff161580611c885750806020015163ffffffff168463ffffffff16105b611ca55760405163bbba60cb60e01b815260040160405180910390fd5b6040015190505b949350505050565b6040516378219b3f60e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f043367e90611d029085908590600401614f9a565b5f604051808303815f87803b158015611d19575f5ffd5b505af1158015611d2b573d5f5f3e3d5ffd5b50505050817fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa826040516119b79190614fb2565b6064546001600160a01b031633146115525760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401611280565b600181905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b609d54604080516001600160a01b03928316815291831660208301527f315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c910160405180910390a1609d80546001600160a01b0319166001600160a01b0392909216919091179055565b609e54604080516001600160a01b03928316815291831660208301527f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9910160405180910390a1609e80546001600160a01b0319166001600160a01b0392909216919091179055565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611552576040516323d871a560e01b815260040160405180910390fd5b60605f82516001600160401b03811115611f2d57611f2d61409a565b6040519080825280601f01601f191660200182016040528015611f57576020820181803683370190505b5090505f5b8351811015611fb257838181518110611f7757611f77614ad9565b602002602001015160f81b828281518110611f9457611f94614ad9565b60200101906001600160f81b03191690815f1a905350600101611f5c565b5092915050565b6001600160a01b0383165f90815260996020526040812080549091611fdd826119c3565b905060018084015460ff166002811115611ff957611ff9614739565b146120175760405163aba4733960e01b815260040160405180910390fd5b6096545f9061202a90879060ff16612788565b90506001600160c01b038116612053576040516368b6a87560e11b815260040160405180910390fd5b61206a6001600160c01b0382811690841681161490565b6120875760405163d053aa2160e01b815260040160405180910390fd5b6001600160c01b03818116198316166120a0848261329a565b6001600160c01b0381166120fc576001600160a01b0388165f81815260996020526040808220600101805460ff19166002179055518692917f396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e491a35b60405163f4e24fe560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f4e24fe59061214a908b908b90600401614fc4565b5f604051808303815f87803b158015612161575f5ffd5b505af1158015612173573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd91506121c59087908b90600401614f9a565b5f604051808303815f87803b1580156121dc575f5ffd5b505af11580156121ee573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd91506122409087908b90600401614f9a565b5f604051808303815f87803b158015612257575f5ffd5b505af1158015612269573d5f5f3e3d5ffd5b50505050851561227d5761227d88886132a6565b5050505050505050565b5f610cfd6122936134b5565b8360405161190160f01b602082015260228101839052604281018290525f9060620160405160208183030381529060405280519060200120905092915050565b604080518082019091525f80825260208201525f80806123005f51602061552a5f395f51905f5286614ffb565b90505b61230c816135db565b90935091505f51602061552a5f395f51905f528283098303612344576040805180820190915290815260208101919091529392505050565b5f51602061552a5f395f51905f52600182089050612303565b60965460ff1660c0811061238457604051633cb89c9760e01b815260040160405180910390fd5b60968054600191905f9061239c90849060ff1661500e565b92506101000a81548160ff021916908360ff1602179055506123be8187612858565b6040805160018082528183019092525f91816020015b604080518082019091525f8152606060208201528152602001906001900390816123d45790505090505f85516001600160401b038111156124175761241761409a565b604051908082528060200260200182016040528015612440578160200160208202803683370190505b5090505f5b865181101561249d5786818151811061246057612460614ad9565b60200260200101515f015182828151811061247d5761247d614ad9565b6001600160a01b0390921660209283029190910190910152600101612445565b5060405180604001604052808460ff1663ffffffff16815260200182815250825f815181106124ce576124ce614ad9565b602090810291909101015260a154604051630130fc2760e51b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169263261f84e09261252e9291909116908690600401615027565b5f604051808303815f87803b158015612545575f5ffd5b505af1158015612557573d5f5f3e3d5ffd5b505f9250612563915050565b85600181111561257557612575614739565b036125fc57604051633aea0b9d60e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906375d4173a906125ca9086908b908b90600401615141565b5f604051808303815f87803b1580156125e1575f5ffd5b505af11580156125f3573d5f5f3e3d5ffd5b50505050612695565b600185600181111561261057612610614739565b0361269557604051630662d3e160e51b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063cc5a7c20906126679086908b9089908c90600401615174565b5f604051808303815f87803b15801561267e575f5ffd5b505af1158015612690573d5f5f3e3d5ffd5b505050505b60405163136ca0f960e11b815260ff841660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906326d941f2906024015f604051808303815f87803b1580156126f6575f5ffd5b505af1158015612708573d5f5f3e3d5ffd5b505060405163136ca0f960e11b815260ff861660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692506326d941f291506024015f604051808303815f87803b15801561276d575f5ffd5b505af115801561277f573d5f5f3e3d5ffd5b5050505061227d565b5f5f61279384613657565b9050808360ff166001901b11610b2a5760405163ca95733360e01b815260040160405180910390fd5b606480546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b60a180546001600160a01b0319166001600160a01b0392909216919091179055565b60965460ff90811690821610610b9057604051637310cff560e11b815260040160405180910390fd5b60ff82165f81815260976020908152604091829020845181548684018051888701805163ffffffff90951665ffffffffffff199094168417600160201b61ffff938416021767ffff0000000000001916600160301b95831695909502949094179094558551918252518316938101939093525116918101919091527f3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac906060016119b7565b609e546001600160a01b03163314611552576040516376d8ab1760e11b815260040160405180910390fd5b8015612949576001600160a01b0383165f908152609f602052604090204290555b6001600160a01b0383165f90815260996020526040812080546096549192909161297790869060ff16612788565b90505f612983836119c3565b905060018085015460ff16600281111561299f5761299f614739565b1480156129b457506001600160c01b03821615155b80156129d257506129d26001600160c01b0383811690831681161490565b156129e1576129e187876132a6565b50505050505050565b60605f82516001600160401b03811115612a0657612a0661409a565b604051908082528060200260200182016040528015612a2f578160200160208202803683370190505b5090505f5b8351811015612a9057612a618686868481518110612a5457612a54614ad9565b6020026020010151613712565b828281518110612a7357612a73614ad9565b63ffffffff90921660209283029190910190910152600101612a34565b50949350505050565b6040516309aa152760e11b81526001600160a01b0383811660048301525f917f0000000000000000000000000000000000000000000000000000000000000000909116906313542a4e90602401602060405180830381865afa158015612b01573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b2591906151aa565b90505f819003610cfd577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bf79ce588484612b6987610e29565b6040518463ffffffff1660e01b8152600401612b87939291906151e3565b6020604051808303815f875af1158015612ba3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b2a91906151aa565b612beb60405180606001604052806060815260200160608152602001606081525090565b6096545f90612bfe90869060ff16612788565b90505f612c0a876119c3565b90506001600160c01b038216612c33576040516313ca465760e01b815260040160405180910390fd5b8082166001600160c01b031615612c5d57604051630c6816cd60e01b815260040160405180910390fd5b60a0546001600160a01b0389165f908152609f60205260409020546001600160c01b0383811690851617914291612c94919061525c565b10612cb257604051631968677d60e11b815260040160405180910390fd5b612cbc888261329a565b612cc68887611cb4565b60016001600160a01b038a165f9081526099602052604090206001015460ff166002811115612cf757612cf7614739565b14612d8e57604080518082018252898152600160208083018281526001600160a01b038e165f908152609990925293902082518155925183820180549394939192909160ff191690836002811115612d5157612d51614739565b0217905550506040518991506001600160a01b038b16907fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe905f90a35b604051631fd93ca960e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633fb2795290612ddc908c908b90600401614fc4565b5f604051808303815f87803b158015612df3575f5ffd5b505af1158015612e05573d5f5f3e3d5ffd5b5050604051632550477760e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063255047779150612e59908c908c908c9060040161526f565b5f604051808303815f875af1158015612e74573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612e9b91908101906152f9565b60408087019190915260208601919091525162bff04d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062bff04d90612ef6908b908b90600401614f9a565b5f604051808303815f875af1158015612f11573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612f389190810190615352565b84528415613004575f5b8751811015613002575f60975f8a8481518110612f6157612f61614ad9565b0160209081015160f81c82528181019290925260409081015f208151606081018352905463ffffffff811680835261ffff600160201b8304811695840195909552600160301b90910490931691810191909152875180519193509084908110612fcc57612fcc614ad9565b602002602001015163ffffffff161115612ff957604051633cb89c9760e01b815260040160405180910390fd5b50600101612f42565b505b50505095945050505050565b83518251146130325760405163aaad13f760e01b815260040160405180910390fd5b61303e8686848461382a565b5f61304c878787875f612bc7565b90505f5b855181101561227d575f60975f88848151811061306f5761306f614ad9565b0160209081015160f81c82528181019290925260409081015f208151606081018352905463ffffffff811680835261ffff600160201b8304811695840195909552600160301b909104909316918101919091528451805191935090849081106130da576130da614ad9565b602002602001015163ffffffff1611156131fd5761316e87838151811061310357613103614ad9565b602001015160f81c60f81b60f81c8460400151848151811061312757613127614ad9565b60200260200101518b8660200151868151811061314657613146614ad9565b602002602001015189878151811061316057613160614ad9565b6020026020010151866138d5565b6040805160018082528183019092525f916020820181803683370190505090508783815181106131a0576131a0614ad9565b602001015160f81c60f81b815f815181106131bd576131bd614ad9565b60200101906001600160f81b03191690815f1a9053506131fb8684815181106131e8576131e8614ad9565b602002602001015160200151825f612928565b505b50600101613050565b5f81815260208390526040812054808203613224575f915050610cfd565b5f83815260208590526040902061323c6001836153e1565b8154811061324c5761324c614ad9565b5f91825260209091200154600160401b90046001600160c01b03169150610cfd9050565b5f805b8215610cfd576132846001846153e1565b9092169180613292816153f4565b915050613273565b610c7160988383613a56565b5f81516001600160401b038111156132c0576132c061409a565b6040519080825280602002602001820160405280156132e9578160200160208202803683370190505b5090505f805b8351811015613413575f84828151811061330b5761330b614ad9565b6020910181015160408051808201825260a1546001600160a01b03908116825260f89390931c93810184815291516333869dd160e11b81528a84166004820152905183166024820152905163ffffffff1660448201529192507f0000000000000000000000000000000000000000000000000000000000000000169063670d3ba290606401602060405180830381865afa1580156133ab573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133cf9190614afc565b1561340a57808484815181106133e7576133e7614ad9565b63ffffffff909216602092830291909101909101528261340681614e37565b9350505b506001016132ef565b50808252604080516060810182526001600160a01b03868116825260a154811660208301527f00000000000000000000000000000000000000000000000000000000000000001691636e3492b59190810161346d87613c0f565b8152506040518263ffffffff1660e01b815260040161348c9190615414565b5f604051808303815f87803b1580156134a3575f5ffd5b505af115801561227d573d5f5f3e3d5ffd5b5f306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614801561350d57507f000000000000000000000000000000000000000000000000000000000000000046145b1561353757507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b5f80805f51602061552a5f395f51905f5260035f51602061552a5f395f51905f52865f51602061552a5f395f51905f52888909090890505f61364b827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f51602061552a5f395f51905f52613cb4565b91959194509092505050565b5f6101008251111561367c57604051637da54e4760e11b815260040160405180910390fd5b81515f0361368b57505f919050565b5f5f835f8151811061369f5761369f614ad9565b0160200151600160f89190911c81901b92505b8451811015613709578481815181106136cd576136cd614ad9565b0160200151600160f89190911c1b91508282116136fd57604051631019106960e31b815260040160405180910390fd5b918117916001016136b2565b50909392505050565b5f81815260208490526040812054815b8181101561379557600161373682846153e1565b61374091906153e1565b92508463ffffffff16865f8681526020019081526020015f208463ffffffff168154811061377057613770614ad9565b5f9182526020909120015463ffffffff161161378d575050610b2a565b600101613722565b5060405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a401611280565b6020808201515f908152609a909152604090205460ff161561385f57604051636fbefec360e11b815260040160405180910390fd5b428160400151101561388457604051630819bdcd60e01b815260040160405180910390fd5b602080820180515f908152609a909252604091829020805460ff19166001179055609d54905191830151610ebd926001600160a01b03909216916138ce918891889188919061161b565b8351613d2d565b6020808301516001600160a01b038082165f8181526099909452604090932054919290871603613918576040516356168b4160e11b815260040160405180910390fd5b8760ff16845f015160ff161461394157604051638e5aeee760e01b815260040160405180910390fd5b604051635401ed2760e01b81526004810182905260ff891660248201525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635401ed2790604401602060405180830381865afa1580156139af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139d39190615482565b90506139df8185613d55565b6001600160601b0316866001600160601b031611613a1057604051634c44995d60e01b815260040160405180910390fd5b613a1a8885613d78565b6001600160601b0316816001600160601b031610613a4b5760405163b187e86960e01b815260040160405180910390fd5b505050505050505050565b5f8281526020849052604081205490819003613afa575f83815260208581526040808320815160608101835263ffffffff43811682528185018681526001600160c01b03808a16958401958652845460018101865594885295909620915191909201805495519351909416600160401b026001600160401b03938316600160201b0267ffffffffffffffff1990961691909216179390931716919091179055610ebd565b5f838152602085905260408120613b126001846153e1565b81548110613b2257613b22614ad9565b5f918252602090912001805490915063ffffffff438116911603613b635780546001600160401b0316600160401b6001600160c01b03851602178155610e22565b805463ffffffff438116600160201b81810267ffffffff00000000199094169390931784555f8781526020898152604080832081516060810183529485528483018481526001600160c01b03808c1693870193845282546001810184559286529390942094519401805493519151909216600160401b026001600160401b0391861690960267ffffffffffffffff19909316939094169290921717919091169190911790555050505050565b60605f82516001600160401b03811115613c2b57613c2b61409a565b604051908082528060200260200182016040528015613c54578160200160208202803683370190505b5090505f5b8351811015611fb257838181518110613c7457613c74614ad9565b602001015160f81c60f81b60f81c60ff16828281518110613c9757613c97614ad9565b63ffffffff90921660209283029190910190910152600101613c59565b5f5f613cbe61405e565b613cc661407c565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280613d0357fe5b5082613d225760405163d51edae360e01b815260040160405180910390fd5b505195945050505050565b613d38838383613d91565b610b1457604051638baa579f60e01b815260040160405180910390fd5b60208101515f9061271090613d6e9061ffff168561549d565b610b2a91906154bf565b60408101515f9061271090613d6e9061ffff168561549d565b5f5f5f613d9e8585613ed6565b90925090505f816004811115613db657613db6614739565b148015613dd45750856001600160a01b0316826001600160a01b0316145b15613de457600192505050610b2a565b5f5f876001600160a01b0316631626ba7e60e01b8888604051602401613e0b929190614f9a565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051613e4991906154ec565b5f60405180830381855afa9150503d805f8114613e81576040519150601f19603f3d011682016040523d82523d5f602084013e613e86565b606091505b5091509150818015613e99575080516020145b8015613eca57508051630b135d3f60e11b90613ebe9083016020908101908401615502565b6001600160e01b031916145b98975050505050505050565b5f5f8251604103613f0a576020830151604084015160608501515f1a613efe87828585613f41565b94509450505050613f3a565b8251604003613f335760208301516040840151613f28868383614026565b935093505050613f3a565b505f905060025b9250929050565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115613f7657505f9050600361401d565b8460ff16601b14158015613f8e57508460ff16601c14155b15613f9e57505f9050600461401d565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015613fef573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116614017575f6001925092505061401d565b91505f90505b94509492505050565b5f806001600160ff1b0383168161404260ff86901c601b61525c565b905061405087828885613f41565b935093505050935093915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b03811182821017156140d0576140d061409a565b60405290565b604080519081016001600160401b03811182821017156140d0576140d061409a565b604051601f8201601f191681016001600160401b03811182821017156141205761412061409a565b604052919050565b5f6001600160401b038211156141405761414061409a565b5060051b60200190565b6001600160a01b0381168114610b90575f5ffd5b5f82601f83011261416d575f5ffd5b813561418061417b82614128565b6140f8565b8082825260208201915060208360051b8601019250858311156141a1575f5ffd5b602085015b838110156141c75780356141b98161414a565b8352602092830192016141a6565b5095945050505050565b5f602082840312156141e1575f5ffd5b81356001600160401b038111156141f6575f5ffd5b611cac8482850161415e565b5f60208284031215614212575f5ffd5b5035919050565b63ffffffff81168114610b90575f5ffd5b5f5f5f6060848603121561423c575f5ffd5b83359250602084013561424e81614219565b929592945050506040919091013590565b5f82601f83011261426e575f5ffd5b8135602083015f5f6001600160401b0384111561428d5761428d61409a565b50601f8301601f19166020016142a2816140f8565b9150508281528583830111156142b6575f5ffd5b828260208301375f92810160200192909252509392505050565b5f602082840312156142e0575f5ffd5b81356001600160401b038111156142f5575f5ffd5b611cac8482850161425f565b5f60208284031215614311575f5ffd5b8135610b2a8161414a565b5f5f6040838503121561432d575f5ffd5b50508035926020909101359150565b803560ff8116811461434c575f5ffd5b919050565b5f60208284031215614361575f5ffd5b610b2a8261433c565b5f82601f830112614379575f5ffd5b813561438761417b82614128565b8082825260208201915060208360051b8601019250858311156143a8575f5ffd5b602085015b838110156141c75780356143c081614219565b8352602092830192016143ad565b5f5f5f606084860312156143e0575f5ffd5b83356143eb8161414a565b925060208401356143fb8161414a565b915060408401356001600160401b03811115614415575f5ffd5b6144218682870161436a565b9150509250925092565b815181526020808301519082015260408101610cfd565b803561ffff8116811461434c575f5ffd5b5f60608284031215614463575f5ffd5b61446b6140ae565b9050813561447881614219565b815261448660208301614442565b602082015261449760408301614442565b604082015292915050565b6001600160601b0381168114610b90575f5ffd5b5f82601f8301126144c5575f5ffd5b81356144d361417b82614128565b8082825260208201915060208360061b8601019250858311156144f4575f5ffd5b602085015b838110156141c75760408188031215614510575f5ffd5b6145186140d6565b81356145238161414a565b81526020820135614533816144a2565b60208281019190915290845292909201916040016144f9565b5f5f5f5f60c0858703121561455f575f5ffd5b6145698686614453565b93506060850135614579816144a2565b925060808501356001600160401b03811115614593575f5ffd5b61459f878288016144b6565b92505060a08501356145b081614219565b939692955090935050565b5f5f83601f8401126145cb575f5ffd5b5081356001600160401b038111156145e1575f5ffd5b602083019150836020828501011115613f3a575f5ffd5b5f5f5f6040848603121561460a575f5ffd5b83356001600160401b0381111561461f575f5ffd5b8401601f8101861361462f575f5ffd5b803561463d61417b82614128565b8082825260208201915060208360051b85010192508883111561465e575f5ffd5b602084015b8381101561469e5780356001600160401b03811115614680575f5ffd5b61468f8b60208389010161415e565b84525060209283019201614663565b50955050505060208401356001600160401b038111156146bc575f5ffd5b6146c8868287016145bb565b9497909650939450505050565b5f5f5f5f5f60a086880312156146e9575f5ffd5b85356146f48161414a565b945060208601356147048161414a565b935060408601356147148161414a565b925060608601359150608086013561472b8161414a565b809150509295509295909350565b634e487b7160e01b5f52602160045260245ffd5b6003811061476957634e487b7160e01b5f52602160045260245ffd5b9052565b815181526020808301516040830191611fb29084018261474d565b5f5f60808385031215614799575f5ffd5b6147a28361433c565b91506147b18460208501614453565b90509250929050565b5f5f604083850312156147cb575f5ffd5b82356147d68161414a565b915060208301356001600160401b038111156147f0575f5ffd5b6147fc8582860161425f565b9150509250929050565b5f5f5f60a08486031215614818575f5ffd5b6148228585614453565b92506060840135614832816144a2565b915060808401356001600160401b0381111561484c575f5ffd5b614421868287016144b6565b5f82601f830112614867575f5ffd5b813561487561417b82614128565b8082825260208201915060208360061b860101925085831115614896575f5ffd5b602085015b838110156141c757604081880312156148b2575f5ffd5b6148ba6140d6565b6148c38261433c565b815260208201356148d38161414a565b602082810191909152908452929092019160400161489b565b5f5f5f5f5f60a08688031215614900575f5ffd5b853561490b8161414a565b94506020860135935060408601356001600160401b0381111561492c575f5ffd5b61493888828901614858565b9598949750949560608101359550608001359392505050565b5f5f60408385031215614962575f5ffd5b823561496d81614219565b915060208301356001600160401b03811115614987575f5ffd5b8301601f81018513614997575f5ffd5b80356149a561417b82614128565b8082825260208201915060208360051b8501019250878311156149c6575f5ffd5b6020840193505b828410156149e85783358252602093840193909101906149cd565b809450505050509250929050565b602080825282518282018190525f918401906040840190835b81811015614a3357835163ffffffff16835260209384019390920191600101614a0f565b509095945050505050565b5f5f5f5f5f60808688031215614a52575f5ffd5b8535614a5d8161414a565b94506020860135614a6d8161414a565b935060408601356001600160401b03811115614a87575f5ffd5b614a938882890161436a565b93505060608601356001600160401b03811115614aae575f5ffd5b614aba888289016145bb565b969995985093965092949392505050565b60208101610cfd828461474d565b634e487b7160e01b5f52603260045260245ffd5b8051801515811461434c575f5ffd5b5f60208284031215614b0c575f5ffd5b610b2a82614aed565b5f60208284031215614b25575f5ffd5b8151610b2a8161414a565b5f60208284031215614b40575f5ffd5b8151610b2a81614219565b5f60c0820188835260018060a01b038816602084015286604084015260c0606084015280865180835260e0850191506020880192505f5b81811015614bb8578351805160ff1684526020908101516001600160a01b03168185015290930192604090920191600101614b82565b50506080840195909552505060a00152949350505050565b80356002811061434c575f5ffd5b5f60408284031215614bee575f5ffd5b614bf66140d6565b823581526020928301359281019290925250919050565b5f82601f830112614c1c575f5ffd5b614c246140d6565b806040840185811115614c35575f5ffd5b845b81811015614a33578035845260209384019301614c37565b5f818303610100811215614c61575f5ffd5b614c696140ae565b9150614c758484614bde565b8252614c848460408501614bde565b60208301526080607f1982011215614c9a575f5ffd5b50614ca36140d6565b614cb08460808501614c0d565b8152614cbf8460c08501614c0d565b6020820152604082015292915050565b5f5f5f6101408486031215614ce2575f5ffd5b614ceb84614bd0565b925060208401356001600160401b03811115614d05575f5ffd5b614d118682870161425f565b925050614d218560408601614c4f565b90509250925092565b5f5f5f5f5f6101808688031215614d3f575f5ffd5b614d4886614bd0565b945060208601356001600160401b03811115614d62575f5ffd5b614d6e8882890161425f565b945050614d7e8760408801614c4f565b92506101408601356001600160401b03811115614d99575f5ffd5b614da588828901614858565b9250506101608601356001600160401b03811115614dc1575f5ffd5b860160608189031215614dd2575f5ffd5b614dda6140ae565b81356001600160401b03811115614def575f5ffd5b614dfb8a82850161425f565b8252506020828101359082015260409182013591810191909152949793965091945092919050565b634e487b7160e01b5f52601160045260245ffd5b5f60018201614e4857614e48614e23565b5060010190565b606080825284519082018190525f9060208601906080840190835b81811015614e915783516001600160a01b0316835260209384019390920191600101614e6a565b5050838103602080860191909152865180835291810192508601905f5b81811015614ecc578251845260209384019390920191600101614eae565b50505060ff841660408401529050611cac565b5f60208284031215614eef575f5ffd5b81516001600160401b03811115614f04575f5ffd5b8201601f81018413614f14575f5ffd5b8051614f2261417b82614128565b8082825260208201915060208360051b850101925086831115614f43575f5ffd5b6020840193505b8284101561165a57614f5b84614aed565b825260209384019390910190614f4a565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b828152604060208201525f610b276040830184614f6c565b602081525f610b2a6020830184614f6c565b6001600160a01b03831681526040602082018190525f90610b2790830184614f6c565b634e487b7160e01b5f52601260045260245ffd5b5f8261500957615009614fe7565b500690565b60ff8181168382160190811115610cfd57610cfd614e23565b5f6040820160018060a01b03851683526040602084015280845180835260608501915060608160051b8601019250602086015f5b828110156150dc57868503605f190184528151805163ffffffff168652602090810151604082880181905281519088018190529101905f9060608801905b808310156150c45783516001600160a01b031682526020938401936001939093019290910190615099565b5096505050602093840193919091019060010161505b565b5092979650505050505050565b5f8151808452602084019350602083015f5b8281101561513757815180516001600160a01b031687526020908101516001600160601b031681880152604090960195909101906001016150fb565b5093949350505050565b60ff841681526001600160601b0383166020820152606060408201525f61516b60608301846150e9565b95945050505050565b60ff851681526001600160601b038416602082015263ffffffff83166040820152608060608201525f61165a60808301846150e9565b5f602082840312156151ba575f5ffd5b5051919050565b805f5b6002811015610ebd5781518452602093840193909101906001016151c4565b6001600160a01b0384168152825180516020808401919091520151604082015261016081016020848101518051606085015290810151608084015250604084015161523260a0840182516151c1565b6020015161524360e08401826151c1565b5082516101208301526020830151610140830152611cac565b80820180821115610cfd57610cfd614e23565b60018060a01b0384168152826020820152606060408201525f61516b6060830184614f6c565b5f82601f8301126152a4575f5ffd5b81516152b261417b82614128565b8082825260208201915060208360051b8601019250858311156152d3575f5ffd5b602085015b838110156141c75780516152eb816144a2565b8352602092830192016152d8565b5f5f6040838503121561530a575f5ffd5b82516001600160401b0381111561531f575f5ffd5b61532b85828601615295565b92505060208301516001600160401b03811115615346575f5ffd5b6147fc85828601615295565b5f60208284031215615362575f5ffd5b81516001600160401b03811115615377575f5ffd5b8201601f81018413615387575f5ffd5b805161539561417b82614128565b8082825260208201915060208360051b8501019250868311156153b6575f5ffd5b6020840193505b8284101561165a5783516153d081614219565b8252602093840193909101906153bd565b81810381811115610cfd57610cfd614e23565b5f61ffff821661ffff810361540b5761540b614e23565b60010192915050565b602080825282516001600160a01b039081168383015283820151166040808401919091528301516060808401528051608084018190525f929190910190829060a08501905b808310156141c75763ffffffff8451168252602082019150602084019350600183019250615459565b5f60208284031215615492575f5ffd5b8151610b2a816144a2565b6001600160601b038181168382160290811690818114611fb257611fb2614e23565b5f6001600160601b038316806154d7576154d7614fe7565b806001600160601b0384160491505092915050565b5f82518060208501845e5f920191825250919050565b5f60208284031215615512575f5ffd5b81516001600160e01b031981168114610b2a575f5ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220acf44f61bde51b7c74684d096c55f7653fc5a41dee53d94b22aedbd5a5e8d96664736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x032W_5`\xE0\x1C\x80ccG\xC9\0\x11a\x01\xAFW\x80c\xA5\x7FLo\x11a\0\xFEW\x80c\xD7-\x8D\xD6\x11a\0\x9EW\x80c\xEA2\xAF\xAE\x11a\0yW\x80c\xEA2\xAF\xAE\x14a\x08\xF0W\x80c\xF2\xFD\xE3\x8B\x14a\t\x17W\x80c\xFA\xBC\x1C\xBC\x14a\t*W\x80c\xFD9\x10Z\x14a\t=W__\xFD[\x80c\xD7-\x8D\xD6\x14a\x083W\x80c\xDE\x11d\xBB\x14a\x08;W\x80c\xE6W\x97\xAD\x14a\x08NW__\xFD[\x80c\xC3\x91B^\x11a\0\xD9W\x80c\xC3\x91B^\x14a\x07\xB2W\x80c\xC6?\xD5\x02\x14a\x07\xD2W\x80c\xCA\r\xE8\x82\x14a\x07\xE5W\x80c\xCA\x8A\xA7\xC7\x14a\x08\x0CW__\xFD[\x80c\xA5\x7FLo\x14a\x07tW\x80c\xA9ox>\x14a\x07\x87W\x80c\xB5&W\x87\x14a\x07\x90W__\xFD[\x80c\x84\xCAR\x13\x11a\x01iW\x80c\x8D\xA5\xCB[\x11a\x01DW\x80c\x8D\xA5\xCB[\x14a\x06\xF6W\x80c\x9A\xA1e=\x14a\x07\x07W\x80c\x9E\x99#\xC2\x14a\x07&W\x80c\x9F\xEA\xB8Y\x14a\x07MW__\xFD[\x80c\x84\xCAR\x13\x14a\x06\xA9W\x80c\x87\x1E\xF0I\x14a\x06\xBCW\x80c\x88o\x11\x95\x14a\x06\xCFW__\xFD[\x80ccG\xC9\0\x14a\x06.W\x80ch0H5\x14a\x06AW\x80cn;\x17\xDB\x14a\x06hW\x80cqP\x18\xA6\x14a\x06{W\x80csDy\x92\x14a\x06\x83W\x80c\x82\x81\xABu\x14a\x06\x96W__\xFD[\x80c)k\xB0d\x11a\x02\x85W\x80cS\x0B\x97\xA4\x11a\x02%W\x80cZ\xC8j\xB7\x11a\x02\0W\x80cZ\xC8j\xB7\x14a\x05\xCDW\x80c[\x0B\x82\x9F\x14a\x05\xECW\x80c\\\x97Z\xBB\x14a\x05\xFFW\x80c]\xF4YF\x14a\x06\x07W__\xFD[\x80cS\x0B\x97\xA4\x14a\x05\x92W\x80cXe\xC6\x0C\x14a\x05\xA5W\x80cY\\jg\x14a\x05\xC5W__\xFD[\x80c0<\xA9V\x11a\x02`W\x80c0<\xA9V\x14a\x059W\x80c<*\x7FL\x14a\x05LW\x80c>\xEF:Q\x14a\x05lW\x80cQ@\xA5H\x14a\x05\x7FW__\xFD[\x80c)k\xB0d\x14a\x05\0W\x80c)\xD1\xE0\xC3\x14a\x05\x13W\x80c,\xDD\x1E\x86\x14a\x05&W__\xFD[\x80c\x12^\x05\x84\x11a\x02\xF0W\x80c\x14x\x85\x1F\x11a\x02\xCBW\x80c\x14x\x85\x1F\x14a\x04SW\x80c\x1E\xB8\x12\xDA\x14a\x04\x85W\x80c$\x9A\x0CB\x14a\x04\xCEW\x80c(\xF6\x1B1\x14a\x04\xEDW__\xFD[\x80c\x12^\x05\x84\x14a\x03\xF9W\x80c\x13T*N\x14a\x04\x18W\x80c\x13d9\xDD\x14a\x04@W__\xFD[\x80b\xCF*\xB5\x14a\x036W\x80c\x03\xFD4\x92\x14a\x03KW\x80c\x04\xECcQ\x14a\x03}W\x80c\x05C\x10\xE6\x14a\x03\xA8W\x80c\x0C\xF4\xB7g\x14a\x03\xD3W\x80c\r?!4\x14a\x03\xE6W[__\xFD[a\x03Ia\x03D6`\x04aA\xD1V[a\txV[\0[a\x03ja\x03Y6`\x04aB\x02V[_\x90\x81R`\x98` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\x90a\x03\x8B6`\x04aB*V[a\x0B\x19V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x03tV[`\x9DTa\x03\xBB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03tV[a\x03Ia\x03\xE16`\x04aB\xD0V[a\x0B1V[a\x03Ia\x03\xF46`\x04aB\x02V[a\x0B\x93V[a\x03ja\x04\x076`\x04aC\x01V[`\x9F` R_\x90\x81R`@\x90 T\x81V[a\x03ja\x04&6`\x04aC\x01V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x99` R`@\x90 T\x90V[a\x03Ia\x04N6`\x04aB\x02V[a\x0B\xA0V[a\x04ua\x04a6`\x04aB\x02V[`\x9A` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03tV[a\x04\x98a\x04\x936`\x04aC\x1CV[a\x0CuV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x03tV[a\x03ja\x04\xDC6`\x04aCQV[`\x9B` R_\x90\x81R`@\x90 T\x81V[`\x9ETa\x03\xBB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xBBa\x05\x0E6`\x04aB\x02V[a\r\x03V[a\x03Ia\x05!6`\x04aC\x01V[a\r\x8CV[a\x03Ia\x0546`\x04aC\x01V[a\r\x9DV[a\x03Ia\x05G6`\x04aC\xCEV[a\r\xAEV[a\x05_a\x05Z6`\x04aC\x01V[a\x0E)V[`@Qa\x03t\x91\x90aD+V[a\x03Ia\x05z6`\x04aELV[a\x0E\xA7V[a\x03Ia\x05\x8D6`\x04aE\xF8V[a\x0E\xC3V[a\x03Ia\x05\xA06`\x04aF\xD5V[a\x11\xEAV[a\x05\xB8a\x05\xB36`\x04aC\x01V[a\x143V[`@Qa\x03t\x91\x90aGmV[a\x03Ia\x14\xA5V[a\x04ua\x05\xDB6`\x04aCQV[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x03Ia\x05\xFA6`\x04aG\x88V[a\x15TV[`\x01Ta\x03jV[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xBBa\x06<6`\x04aB\x02V[a\x15pV[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ia\x06v6`\x04aG\xBAV[a\x15\x98V[a\x03Ia\x15\xACV[a\x03ja\x06\x916`\x04aC\x01V[a\x15\xBDV[a\x03Ia\x06\xA46`\x04aH\x06V[a\x16\x06V[a\x03ja\x06\xB76`\x04aH\xECV[a\x16\x1BV[a\x03\x90a\x06\xCA6`\x04aB\x02V[a\x16dV[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`dT`\x01`\x01`\xA0\x1B\x03\x16a\x03\xBBV[`\x96Ta\x07\x14\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03tV[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03j\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x03Ia\x07\x826`\x04aC\x01V[a\x16nV[a\x03j`\xA0T\x81V[a\x04ua\x07\x9E6`\x04aC\x01V[`\xA1T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x90V[a\x07\xC5a\x07\xC06`\x04aIQV[a\x16\x7FV[`@Qa\x03t\x91\x90aI\xF6V[a\x03Ia\x07\xE06`\x04aJ>V[a\x16\x8DV[a\x03j\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81V[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x9CTa\x03jV[`\xA1Ta\x03\xBB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x08\xBCa\x08\\6`\x04aCQV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x97\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x03tV[a\x03\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ia\t%6`\x04aC\x01V[a\x186V[a\x03Ia\t86`\x04aB\x02V[a\x18\xACV[a\tka\tK6`\x04aC\x01V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x03t\x91\x90aJ\xCBV[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\t\xA1W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82Q\x81\x10\x15a\x0B\x14W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x82\x81Q\x81\x10a\t\xDFWa\t\xDFaJ\xD9V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a\t\xF9Wa\t\xF9aJ\xD9V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\x99_\x86\x85\x81Q\x81\x10a\nJWa\nJaJ\xD9V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x01T\x81_\x81Q\x81\x10a\n\x86Wa\n\x86aJ\xD9V[` \x02` \x01\x01\x81\x81RPP_a\n\xB5\x82_\x81Q\x81\x10a\n\xA8Wa\n\xA8aJ\xD9V[` \x02` \x01\x01Qa\x19\xC3V[\x90P_a\n\xCA\x82`\x01`\x01`\xC0\x1B\x03\x16a\x19\xCFV[\x90P_[\x81Q\x81\x10\x15a\x0B\x03Wa\n\xFB\x85\x85\x84\x84\x81Q\x81\x10a\n\xEEWa\n\xEEaJ\xD9V[\x01` \x01Q`\xF8\x1Ca\x1A\x98V[`\x01\x01a\n\xCEV[PP`\x01\x90\x93\x01\x92Pa\t\xA3\x91PPV[PPPV[_a\x0B'`\x98\x85\x85\x85a\x1B\xD0V[\x90P[\x93\x92PPPV[`\x013_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x0BYWa\x0BYaG9V[\x14a\x0BwW`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x99` R`@\x90 Ta\x0B\x90\x90\x82a\x1C\xB4V[PV[a\x0B\x9Ba\x1D_V[`\xA0UV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C&\x91\x90aJ\xFCV[a\x0CCW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x81\x81\x16\x81\x14a\x0ChW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Cq\x82a\x1D\xB9V[PPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R_\x83\x81R`\x98` R`@\x90 \x80T\x83\x90\x81\x10a\x0C\xB0Wa\x0C\xB0aJ\xD9V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rhW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xFD\x91\x90aK\x15V[a\r\x94a\x1D_V[a\x0B\x90\x81a\x1D\xF6V[a\r\xA5a\x1D_V[a\x0B\x90\x81a\x1E_V[a\r\xB6a\x1E\xC8V[`\x01\x80T`\x02\x90\x81\x16\x03a\r\xDDW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x0E\x0BW`@Qcf\xE5e\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0E\x15\x83a\x1F\x11V[\x90Pa\x0E\"\x85\x82_a\x1F\xB9V[PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x0C\xFDa\x0E\xA2\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x84`@Q` \x01a\x0E\x87\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\"\x87V[a\"\xD3V[a\x0E\xAFa\x1D_V[a\x0E\xBD\x84\x84\x84`\x01\x85a#]V[PPPPV[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\x0E\xECW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F1\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa'\x88\x90PV[P\x83Q\x82\x14a\x0FSW`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\x0E\"W_\x84\x84\x83\x81\x81\x10a\x0FpWa\x0FpaJ\xD9V[\x88Q\x92\x015`\xF8\x1C\x92P_\x91\x88\x91P\x84\x90\x81\x10a\x0F\x8FWa\x0F\x8FaJ\xD9V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10&\x91\x90aK0V[c\xFF\xFF\xFF\xFF\x16\x81Q\x14a\x10LW`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10fWa\x10fa@\x9AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\x8FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x83Q\x81\x10\x15a\x11\x85W_\x84\x82\x81Q\x81\x10a\x10\xB1Wa\x10\xB1aJ\xD9V[` \x02` \x01\x01Q\x90P`\x99_\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x01T\x84\x83\x81Q\x81\x10a\x10\xF3Wa\x10\xF3aJ\xD9V[` \x02` \x01\x01\x81\x81RPP_a\x11\x15\x85\x84\x81Q\x81\x10a\n\xA8Wa\n\xA8aJ\xD9V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x89\x16\x1C\x81\x16\x14a\x11HW`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x11a\x11zW`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x91P`\x01\x01a\x10\x95V[Pa\x11\x91\x83\x83\x86a\x1A\x98V[`\xFF\x84\x16_\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPPP\x80`\x01\x01\x90Pa\x0FUV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\x08WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12!WP0;\x15\x80\x15a\x12!WP_T`\xFF\x16`\x01\x14[a\x12\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12\xAAW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12\xB3\x86a'\xBCV[a\x12\xBC\x85a\x1D\xF6V[a\x12\xC5\x83a\x1D\xB9V[a\x12\xCE\x84a\x1E_V[a\x12\xD7\x82a(\rV[`\x9C\x80T`\x01\x81\x81\x01\x83U_\x83\x90R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x91\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U\x84T\x80\x84\x01\x86U\x84\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x84T\x80\x84\x01\x86U\x84\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x84T\x92\x83\x01\x90\x94U\x91\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x92\x16\x17\x90U\x80\x15a\x14+W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x99` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x14\x8BWa\x14\x8BaG9V[`\x02\x81\x11\x15a\x14\x9CWa\x14\x9CaG9V[\x90RP\x92\x91PPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15+\x91\x90aJ\xFCV[a\x15HW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15R_\x19a\x1D\xB9V[V[a\x15\\a\x1D_V[\x81a\x15f\x81a(/V[a\x0B\x14\x83\x83a(XV[`\x9C\x81\x81T\x81\x10a\x15\x7FW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[a\x15\xA0a(\xFDV[a\x0Cq\x82\x82`\x01a)(V[a\x15\xB4a\x1D_V[a\x15R_a'\xBCV[_a\x0C\xFD\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x83`@Q` \x01a\x0E\x87\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[a\x16\x0Ea\x1D_V[a\x0B\x14\x83\x83\x83__a#]V[_a\x16Z\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87\x87\x87\x87\x87`@Q` \x01a\x0E\x87\x96\x95\x94\x93\x92\x91\x90aKKV[\x96\x95PPPPPPV[_a\x0C\xFD\x82a\x19\xC3V[a\x16va\x1D_V[a\x0B\x90\x81a(\rV[``a\x0B*`\x98\x84\x84a)\xEAV[a\x16\x95a\x1E\xC8V[`\x01\x80T_\x91\x90\x81\x16\x03a\x16\xBCW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA1T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\x16\xEAW`@Qcf\xE5e\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x16\xF4\x85a\x1F\x11V[\x90P_\x80\x80a\x17\x05\x86\x88\x01\x88aL\xCFV[\x92P\x92P\x92P_a\x17\x16\x8B\x83a*\x99V[\x90P_\x84`\x01\x81\x11\x15a\x17+Wa\x17+aG9V[\x03a\x17\xD4W_a\x17?\x8C\x83\x88\x87`\x01a+\xC7V[Q\x90P_[\x86Q\x81\x10\x15a\x17\xCDW_\x87\x82\x81Q\x81\x10a\x17`Wa\x17`aJ\xD9V[\x01` \x90\x81\x01Q`\xF8\x1C_\x81\x81R`\x97\x90\x92R`@\x90\x91 T\x84Q\x91\x92Pc\xFF\xFF\xFF\xFF\x16\x90\x84\x90\x84\x90\x81\x10a\x17\x97Wa\x17\x97aJ\xD9V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x17\xC4W`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x17DV[PPa\x18)V[`\x01\x84`\x01\x81\x11\x15a\x17\xE8Wa\x17\xE8aG9V[\x03a\x18\x10W_\x80a\x17\xFB\x89\x8B\x01\x8BaM*V[\x94P\x94PPPPa\x17\xCD\x8D\x84\x89\x88\x86\x86a0\x10V[`@Qc5K\xB8\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[a\x18>a\x1D_V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x12\x80V[a\x0B\x90\x81a'\xBCV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19,\x91\x90aK\x15V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19]W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T\x80\x19\x82\x19\x81\x16\x14a\x19\x84W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPV[_a\x0C\xFD`\x98\x83a2\x06V[``__a\x19\xDC\x84a2pV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xF7Wa\x19\xF7a@\x9AV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1A!W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_\x80[\x82Q\x82\x10\x80\x15a\x1A8WPa\x01\0\x81\x10[\x15a\x1A\x8EW`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x1A~W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1AaWa\x1AaaJ\xD9V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x81`\x01\x01\x91P[a\x1A\x87\x81aN7V[\x90Pa\x1A'V[P\x90\x94\x93PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x81`\xF8\x1B\x81_\x81Q\x81\x10a\x1A\xCEWa\x1A\xCEaJ\xD9V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`@Qcl?\xB4\xBF`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cl?\xB4\xBF\x90a\x1B6\x90\x88\x90\x88\x90\x88\x90`\x04\x01aNOV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1BQW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1Bx\x91\x90\x81\x01\x90aN\xDFV[\x90P_[\x85Q\x81\x10\x15a\x14+W\x81\x81\x81Q\x81\x10a\x1B\x97Wa\x1B\x97aJ\xD9V[` \x02` \x01\x01Q\x15a\x1B\xC8Wa\x1B\xC8\x86\x82\x81Q\x81\x10a\x1B\xB9Wa\x1B\xB9aJ\xD9V[` \x02` \x01\x01Q\x84_a)(V[`\x01\x01a\x1B|V[_\x83\x81R` \x85\x90R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1B\xF2Wa\x1B\xF2aJ\xD9V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\x1CbW`@Qcl\xB1\x9A\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x1C\x88WP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x1C\xA5W`@Qc\xBB\xBA`\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x01Q\x90P[\x94\x93PPPPV[`@Qcx!\x9B?`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF0C6~\x90a\x1D\x02\x90\x85\x90\x85\x90`\x04\x01aO\x9AV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1D\x19W__\xFD[PZ\xF1\x15\x80\x15a\x1D+W=__>=_\xFD[PPPP\x81\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x82`@Qa\x19\xB7\x91\x90aO\xB2V[`dT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x12\x80V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x15RW`@Qc#\xD8q\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F-Wa\x1F-a@\x9AV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1FWW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1F\xB2W\x83\x81\x81Q\x81\x10a\x1FwWa\x1FwaJ\xD9V[` \x02` \x01\x01Q`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1F\x94Wa\x1F\x94aJ\xD9V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x1F\\V[P\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x99` R`@\x81 \x80T\x90\x91a\x1F\xDD\x82a\x19\xC3V[\x90P`\x01\x80\x84\x01T`\xFF\x16`\x02\x81\x11\x15a\x1F\xF9Wa\x1F\xF9aG9V[\x14a \x17W`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x96T_\x90a *\x90\x87\x90`\xFF\x16a'\x88V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a SW`@Qch\xB6\xA8u`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a j`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x90\x84\x16\x81\x16\x14\x90V[a \x87W`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xC0\x1B\x03\x81\x81\x16\x19\x83\x16\x16a \xA0\x84\x82a2\x9AV[`\x01`\x01`\xC0\x1B\x03\x81\x16a \xFCW`\x01`\x01`\xA0\x1B\x03\x88\x16_\x81\x81R`\x99` R`@\x80\x82 `\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UQ\x86\x92\x91\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x91\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a!J\x90\x8B\x90\x8B\x90`\x04\x01aO\xC4V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!aW__\xFD[PZ\xF1\x15\x80\x15a!sW=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa!\xC5\x90\x87\x90\x8B\x90`\x04\x01aO\x9AV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!\xDCW__\xFD[PZ\xF1\x15\x80\x15a!\xEEW=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa\"@\x90\x87\x90\x8B\x90`\x04\x01aO\x9AV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"WW__\xFD[PZ\xF1\x15\x80\x15a\"iW=__>=_\xFD[PPPP\x85\x15a\"}Wa\"}\x88\x88a2\xA6V[PPPPPPPPV[_a\x0C\xFDa\"\x93a4\xB5V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R_\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a#\0_Q` aU*_9_Q\x90_R\x86aO\xFBV[\x90P[a#\x0C\x81a5\xDBV[\x90\x93P\x91P_Q` aU*_9_Q\x90_R\x82\x83\t\x83\x03a#DW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` aU*_9_Q\x90_R`\x01\x82\x08\x90Pa#\x03V[`\x96T`\xFF\x16`\xC0\x81\x10a#\x84W`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x96\x80T`\x01\x91\x90_\x90a#\x9C\x90\x84\x90`\xFF\x16aP\x0EV[\x92Pa\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UPa#\xBE\x81\x87a(XV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a#\xD4W\x90PP\x90P_\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x17Wa$\x17a@\x9AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$@W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x86Q\x81\x10\x15a$\x9DW\x86\x81\x81Q\x81\x10a$`Wa$`aJ\xD9V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a$}Wa$}aJ\xD9V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a$EV[P`@Q\x80`@\x01`@R\x80\x84`\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x82_\x81Q\x81\x10a$\xCEWa$\xCEaJ\xD9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\xA1T`@Qc\x010\xFC'`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92c&\x1F\x84\xE0\x92a%.\x92\x91\x90\x91\x16\x90\x86\x90`\x04\x01aP'V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%EW__\xFD[PZ\xF1\x15\x80\x15a%WW=__>=_\xFD[P_\x92Pa%c\x91PPV[\x85`\x01\x81\x11\x15a%uWa%uaG9V[\x03a%\xFCW`@Qc:\xEA\x0B\x9D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cu\xD4\x17:\x90a%\xCA\x90\x86\x90\x8B\x90\x8B\x90`\x04\x01aQAV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%\xE1W__\xFD[PZ\xF1\x15\x80\x15a%\xF3W=__>=_\xFD[PPPPa&\x95V[`\x01\x85`\x01\x81\x11\x15a&\x10Wa&\x10aG9V[\x03a&\x95W`@Qc\x06b\xD3\xE1`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xCCZ| \x90a&g\x90\x86\x90\x8B\x90\x89\x90\x8C\x90`\x04\x01aQtV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&~W__\xFD[PZ\xF1\x15\x80\x15a&\x90W=__>=_\xFD[PPPP[`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c&\xD9A\xF2\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&\xF6W__\xFD[PZ\xF1\x15\x80\x15a'\x08W=__>=_\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'mW__\xFD[PZ\xF1\x15\x80\x15a'\x7FW=__>=_\xFD[PPPPa\"}V[__a'\x93\x84a6WV[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x0B*W`@Qc\xCA\x95s3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\xA1\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x96T`\xFF\x90\x81\x16\x90\x82\x16\x10a\x0B\x90W`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x16_\x81\x81R`\x97` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01a\x19\xB7V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15RW`@Qcv\xD8\xAB\x17`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a)IW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x9F` R`@\x90 B\x90U[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x99` R`@\x81 \x80T`\x96T\x91\x92\x90\x91a)w\x90\x86\x90`\xFF\x16a'\x88V[\x90P_a)\x83\x83a\x19\xC3V[\x90P`\x01\x80\x85\x01T`\xFF\x16`\x02\x81\x11\x15a)\x9FWa)\x9FaG9V[\x14\x80\x15a)\xB4WP`\x01`\x01`\xC0\x1B\x03\x82\x16\x15\x15[\x80\x15a)\xD2WPa)\xD2`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[\x15a)\xE1Wa)\xE1\x87\x87a2\xA6V[PPPPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x06Wa*\x06a@\x9AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a*/W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a*\x90Wa*a\x86\x86\x86\x84\x81Q\x81\x10a*TWa*TaJ\xD9V[` \x02` \x01\x01Qa7\x12V[\x82\x82\x81Q\x81\x10a*sWa*saJ\xD9V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a*4V[P\x94\x93PPPPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x01W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+%\x91\x90aQ\xAAV[\x90P_\x81\x90\x03a\x0C\xFDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBFy\xCEX\x84\x84a+i\x87a\x0E)V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\x87\x93\x92\x91\x90aQ\xE3V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B*\x91\x90aQ\xAAV[a+\xEB`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\x96T_\x90a+\xFE\x90\x86\x90`\xFF\x16a'\x88V[\x90P_a,\n\x87a\x19\xC3V[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a,3W`@Qc\x13\xCAFW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a,]W`@Qc\x0Ch\x16\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA0T`\x01`\x01`\xA0\x1B\x03\x89\x16_\x90\x81R`\x9F` R`@\x90 T`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17\x91B\x91a,\x94\x91\x90aR\\V[\x10a,\xB2W`@Qc\x19hg}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,\xBC\x88\x82a2\x9AV[a,\xC6\x88\x87a\x1C\xB4V[`\x01`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a,\xF7Wa,\xF7aG9V[\x14a-\x8EW`@\x80Q\x80\x82\x01\x82R\x89\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x16_\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a-QWa-QaG9V[\x02\x17\x90UPP`@Q\x89\x91P`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90_\x90\xA3[`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a-\xDC\x90\x8C\x90\x8B\x90`\x04\x01aO\xC4V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\xF3W__\xFD[PZ\xF1\x15\x80\x15a.\x05W=__>=_\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa.Y\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01aRoV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a.tW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra.\x9B\x91\x90\x81\x01\x90aR\xF9V[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a.\xF6\x90\x8B\x90\x8B\x90`\x04\x01aO\x9AV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a/\x11W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra/8\x91\x90\x81\x01\x90aSRV[\x84R\x84\x15a0\x04W_[\x87Q\x81\x10\x15a0\x02W_`\x97_\x8A\x84\x81Q\x81\x10a/aWa/aaJ\xD9V[\x01` \x90\x81\x01Q`\xF8\x1C\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x87Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a/\xCCWa/\xCCaJ\xD9V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a/\xF9W`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a/BV[P[PPP\x95\x94PPPPPV[\x83Q\x82Q\x14a02W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a0>\x86\x86\x84\x84a8*V[_a0L\x87\x87\x87\x87_a+\xC7V[\x90P_[\x85Q\x81\x10\x15a\"}W_`\x97_\x88\x84\x81Q\x81\x10a0oWa0oaJ\xD9V[\x01` \x90\x81\x01Q`\xF8\x1C\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a0\xDAWa0\xDAaJ\xD9V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a1\xFDWa1n\x87\x83\x81Q\x81\x10a1\x03Wa1\x03aJ\xD9V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a1'Wa1'aJ\xD9V[` \x02` \x01\x01Q\x8B\x86` \x01Q\x86\x81Q\x81\x10a1FWa1FaJ\xD9V[` \x02` \x01\x01Q\x89\x87\x81Q\x81\x10a1`Wa1`aJ\xD9V[` \x02` \x01\x01Q\x86a8\xD5V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a1\xA0Wa1\xA0aJ\xD9V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x81_\x81Q\x81\x10a1\xBDWa1\xBDaJ\xD9V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa1\xFB\x86\x84\x81Q\x81\x10a1\xE8Wa1\xE8aJ\xD9V[` \x02` \x01\x01Q` \x01Q\x82_a)(V[P[P`\x01\x01a0PV[_\x81\x81R` \x83\x90R`@\x81 T\x80\x82\x03a2$W_\x91PPa\x0C\xFDV[_\x83\x81R` \x85\x90R`@\x90 a2<`\x01\x83aS\xE1V[\x81T\x81\x10a2LWa2LaJ\xD9V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x91Pa\x0C\xFD\x90PV[_\x80[\x82\x15a\x0C\xFDWa2\x84`\x01\x84aS\xE1V[\x90\x92\x16\x91\x80a2\x92\x81aS\xF4V[\x91PPa2sV[a\x0Cq`\x98\x83\x83a:VV[_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xC0Wa2\xC0a@\x9AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\xE9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x83Q\x81\x10\x15a4\x13W_\x84\x82\x81Q\x81\x10a3\x0BWa3\x0BaJ\xD9V[` \x91\x01\x81\x01Q`@\x80Q\x80\x82\x01\x82R`\xA1T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\xF8\x93\x90\x93\x1C\x93\x81\x01\x84\x81R\x91Qc3\x86\x9D\xD1`\xE1\x1B\x81R\x8A\x84\x16`\x04\x82\x01R\x90Q\x83\x16`$\x82\x01R\x90Qc\xFF\xFF\xFF\xFF\x16`D\x82\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cg\r;\xA2\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xCF\x91\x90aJ\xFCV[\x15a4\nW\x80\x84\x84\x81Q\x81\x10a3\xE7Wa3\xE7aJ\xD9V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a4\x06\x81aN7V[\x93PP[P`\x01\x01a2\xEFV[P\x80\x82R`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R`\xA1T\x81\x16` \x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91cn4\x92\xB5\x91\x90\x81\x01a4m\x87a<\x0FV[\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\x8C\x91\x90aT\x14V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a4\xA3W__\xFD[PZ\xF1\x15\x80\x15a\"}W=__>=_\xFD[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a5\rWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a57WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[_\x80\x80_Q` aU*_9_Q\x90_R`\x03_Q` aU*_9_Q\x90_R\x86_Q` aU*_9_Q\x90_R\x88\x89\t\t\x08\x90P_a6K\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` aU*_9_Q\x90_Ra<\xB4V[\x91\x95\x91\x94P\x90\x92PPPV[_a\x01\0\x82Q\x11\x15a6|W`@Qc}\xA5NG`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q_\x03a6\x8BWP_\x91\x90PV[__\x83_\x81Q\x81\x10a6\x9FWa6\x9FaJ\xD9V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a7\tW\x84\x81\x81Q\x81\x10a6\xCDWa6\xCDaJ\xD9V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a6\xFDW`@Qc\x10\x19\x10i`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x81\x17\x91`\x01\x01a6\xB2V[P\x90\x93\x92PPPV[_\x81\x81R` \x84\x90R`@\x81 T\x81[\x81\x81\x10\x15a7\x95W`\x01a76\x82\x84aS\xE1V[a7@\x91\x90aS\xE1V[\x92P\x84c\xFF\xFF\xFF\xFF\x16\x86_\x86\x81R` \x01\x90\x81R` \x01_ \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a7pWa7paJ\xD9V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a7\x8DWPPa\x0B*V[`\x01\x01a7\"V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x12\x80V[` \x80\x82\x01Q_\x90\x81R`\x9A\x90\x91R`@\x90 T`\xFF\x16\x15a8_W`@Qco\xBE\xFE\xC3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x81`@\x01Q\x10\x15a8\x84W`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x82\x01\x80Q_\x90\x81R`\x9A\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9DT\x90Q\x91\x83\x01Qa\x0E\xBD\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a8\xCE\x91\x88\x91\x88\x91\x88\x91\x90a\x16\x1BV[\x83Qa=-V[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x81\x81R`\x99\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x03a9\x18W`@QcV\x16\x8BA`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87`\xFF\x16\x84_\x01Q`\xFF\x16\x14a9AW`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xD3\x91\x90aT\x82V[\x90Pa9\xDF\x81\x85a=UV[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11a:\x10W`@QcLD\x99]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a:\x1A\x88\x85a=xV[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a:KW`@Qc\xB1\x87\xE8i`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPV[_\x82\x81R` \x84\x90R`@\x81 T\x90\x81\x90\x03a:\xFAW_\x83\x81R` \x85\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\x0E\xBDV[_\x83\x81R` \x85\x90R`@\x81 a;\x12`\x01\x84aS\xE1V[\x81T\x81\x10a;\"Wa;\"aJ\xD9V[_\x91\x82R` \x90\x91 \x01\x80T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a;cW\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\x0E\"V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U_\x87\x81R` \x89\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a<+Wa<+a@\x9AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<TW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1F\xB2W\x83\x81\x81Q\x81\x10a<tWa<taJ\xD9V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x82\x82\x81Q\x81\x10a<\x97Wa<\x97aJ\xD9V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a<YV[__a<\xBEa@^V[a<\xC6a@|V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a=\x03W\xFE[P\x82a=\"W`@Qc\xD5\x1E\xDA\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x95\x94PPPPPV[a=8\x83\x83\x83a=\x91V[a\x0B\x14W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_\x90a'\x10\x90a=n\x90a\xFF\xFF\x16\x85aT\x9DV[a\x0B*\x91\x90aT\xBFV[`@\x81\x01Q_\x90a'\x10\x90a=n\x90a\xFF\xFF\x16\x85aT\x9DV[___a=\x9E\x85\x85a>\xD6V[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a=\xB6Wa=\xB6aG9V[\x14\x80\x15a=\xD4WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a=\xE4W`\x01\x92PPPa\x0B*V[__\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a>\x0B\x92\x91\x90aO\x9AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa>I\x91\x90aT\xECV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a>\x81W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a>\x86V[``\x91P[P\x91P\x91P\x81\x80\x15a>\x99WP\x80Q` \x14[\x80\x15a>\xCAWP\x80Qc\x0B\x13]?`\xE1\x1B\x90a>\xBE\x90\x83\x01` \x90\x81\x01\x90\x84\x01aU\x02V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[__\x82Q`A\x03a?\nW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa>\xFE\x87\x82\x85\x85a?AV[\x94P\x94PPPPa?:V[\x82Q`@\x03a?3W` \x83\x01Q`@\x84\x01Qa?(\x86\x83\x83a@&V[\x93P\x93PPPa?:V[P_\x90P`\x02[\x92P\x92\x90PV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a?vWP_\x90P`\x03a@\x1DV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a?\x8EWP\x84`\xFF\x16`\x1C\x14\x15[\x15a?\x9EWP_\x90P`\x04a@\x1DV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a?\xEFW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a@\x17W_`\x01\x92P\x92PPa@\x1DV[\x91P_\x90P[\x94P\x94\x92PPPV[_\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a@B`\xFF\x86\x90\x1C`\x1BaR\\V[\x90Pa@P\x87\x82\x88\x85a?AV[\x93P\x93PPP\x93P\x93\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\xD0Wa@\xD0a@\x9AV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\xD0Wa@\xD0a@\x9AV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aA WaA a@\x9AV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aA@WaA@a@\x9AV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x90W__\xFD[_\x82`\x1F\x83\x01\x12aAmW__\xFD[\x815aA\x80aA{\x82aA(V[a@\xF8V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aA\xA1W__\xFD[` \x85\x01[\x83\x81\x10\x15aA\xC7W\x805aA\xB9\x81aAJV[\x83R` \x92\x83\x01\x92\x01aA\xA6V[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15aA\xE1W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xF6W__\xFD[a\x1C\xAC\x84\x82\x85\x01aA^V[_` \x82\x84\x03\x12\x15aB\x12W__\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B\x90W__\xFD[___``\x84\x86\x03\x12\x15aB<W__\xFD[\x835\x92P` \x84\x015aBN\x81aB\x19V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_\x82`\x1F\x83\x01\x12aBnW__\xFD[\x815` \x83\x01__`\x01`\x01`@\x1B\x03\x84\x11\x15aB\x8DWaB\x8Da@\x9AV[P`\x1F\x83\x01`\x1F\x19\x16` \x01aB\xA2\x81a@\xF8V[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15aB\xB6W__\xFD[\x82\x82` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15aB\xE0W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xF5W__\xFD[a\x1C\xAC\x84\x82\x85\x01aB_V[_` \x82\x84\x03\x12\x15aC\x11W__\xFD[\x815a\x0B*\x81aAJV[__`@\x83\x85\x03\x12\x15aC-W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14aCLW__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15aCaW__\xFD[a\x0B*\x82aC<V[_\x82`\x1F\x83\x01\x12aCyW__\xFD[\x815aC\x87aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aC\xA8W__\xFD[` \x85\x01[\x83\x81\x10\x15aA\xC7W\x805aC\xC0\x81aB\x19V[\x83R` \x92\x83\x01\x92\x01aC\xADV[___``\x84\x86\x03\x12\x15aC\xE0W__\xFD[\x835aC\xEB\x81aAJV[\x92P` \x84\x015aC\xFB\x81aAJV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD\x15W__\xFD[aD!\x86\x82\x87\x01aCjV[\x91PP\x92P\x92P\x92V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x0C\xFDV[\x805a\xFF\xFF\x81\x16\x81\x14aCLW__\xFD[_``\x82\x84\x03\x12\x15aDcW__\xFD[aDka@\xAEV[\x90P\x815aDx\x81aB\x19V[\x81RaD\x86` \x83\x01aDBV[` \x82\x01RaD\x97`@\x83\x01aDBV[`@\x82\x01R\x92\x91PPV[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x0B\x90W__\xFD[_\x82`\x1F\x83\x01\x12aD\xC5W__\xFD[\x815aD\xD3aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aD\xF4W__\xFD[` \x85\x01[\x83\x81\x10\x15aA\xC7W`@\x81\x88\x03\x12\x15aE\x10W__\xFD[aE\x18a@\xD6V[\x815aE#\x81aAJV[\x81R` \x82\x015aE3\x81aD\xA2V[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aD\xF9V[____`\xC0\x85\x87\x03\x12\x15aE_W__\xFD[aEi\x86\x86aDSV[\x93P``\x85\x015aEy\x81aD\xA2V[\x92P`\x80\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x93W__\xFD[aE\x9F\x87\x82\x88\x01aD\xB6V[\x92PP`\xA0\x85\x015aE\xB0\x81aB\x19V[\x93\x96\x92\x95P\x90\x93PPV[__\x83`\x1F\x84\x01\x12aE\xCBW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xE1W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a?:W__\xFD[___`@\x84\x86\x03\x12\x15aF\nW__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x1FW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aF/W__\xFD[\x805aF=aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15aF^W__\xFD[` \x84\x01[\x83\x81\x10\x15aF\x9EW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x80W__\xFD[aF\x8F\x8B` \x83\x89\x01\x01aA^V[\x84RP` \x92\x83\x01\x92\x01aFcV[P\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xBCW__\xFD[aF\xC8\x86\x82\x87\x01aE\xBBV[\x94\x97\x90\x96P\x93\x94PPPPV[_____`\xA0\x86\x88\x03\x12\x15aF\xE9W__\xFD[\x855aF\xF4\x81aAJV[\x94P` \x86\x015aG\x04\x81aAJV[\x93P`@\x86\x015aG\x14\x81aAJV[\x92P``\x86\x015\x91P`\x80\x86\x015aG+\x81aAJV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10aGiWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91a\x1F\xB2\x90\x84\x01\x82aGMV[__`\x80\x83\x85\x03\x12\x15aG\x99W__\xFD[aG\xA2\x83aC<V[\x91PaG\xB1\x84` \x85\x01aDSV[\x90P\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aG\xCBW__\xFD[\x825aG\xD6\x81aAJV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xF0W__\xFD[aG\xFC\x85\x82\x86\x01aB_V[\x91PP\x92P\x92\x90PV[___`\xA0\x84\x86\x03\x12\x15aH\x18W__\xFD[aH\"\x85\x85aDSV[\x92P``\x84\x015aH2\x81aD\xA2V[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aHLW__\xFD[aD!\x86\x82\x87\x01aD\xB6V[_\x82`\x1F\x83\x01\x12aHgW__\xFD[\x815aHuaA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aH\x96W__\xFD[` \x85\x01[\x83\x81\x10\x15aA\xC7W`@\x81\x88\x03\x12\x15aH\xB2W__\xFD[aH\xBAa@\xD6V[aH\xC3\x82aC<V[\x81R` \x82\x015aH\xD3\x81aAJV[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aH\x9BV[_____`\xA0\x86\x88\x03\x12\x15aI\0W__\xFD[\x855aI\x0B\x81aAJV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI,W__\xFD[aI8\x88\x82\x89\x01aHXV[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x015\x93\x92PPPV[__`@\x83\x85\x03\x12\x15aIbW__\xFD[\x825aIm\x81aB\x19V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x87W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aI\x97W__\xFD[\x805aI\xA5aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aI\xC6W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aI\xE8W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aI\xCDV[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aJ3W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aJ\x0FV[P\x90\x95\x94PPPPPV[_____`\x80\x86\x88\x03\x12\x15aJRW__\xFD[\x855aJ]\x81aAJV[\x94P` \x86\x015aJm\x81aAJV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\x87W__\xFD[aJ\x93\x88\x82\x89\x01aCjV[\x93PP``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xAEW__\xFD[aJ\xBA\x88\x82\x89\x01aE\xBBV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[` \x81\x01a\x0C\xFD\x82\x84aGMV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x80\x15\x15\x81\x14aCLW__\xFD[_` \x82\x84\x03\x12\x15aK\x0CW__\xFD[a\x0B*\x82aJ\xEDV[_` \x82\x84\x03\x12\x15aK%W__\xFD[\x81Qa\x0B*\x81aAJV[_` \x82\x84\x03\x12\x15aK@W__\xFD[\x81Qa\x0B*\x81aB\x19V[_`\xC0\x82\x01\x88\x83R`\x01\x80`\xA0\x1B\x03\x88\x16` \x84\x01R\x86`@\x84\x01R`\xC0``\x84\x01R\x80\x86Q\x80\x83R`\xE0\x85\x01\x91P` \x88\x01\x92P_[\x81\x81\x10\x15aK\xB8W\x83Q\x80Q`\xFF\x16\x84R` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01aK\x82V[PP`\x80\x84\x01\x95\x90\x95RPP`\xA0\x01R\x94\x93PPPPV[\x805`\x02\x81\x10aCLW__\xFD[_`@\x82\x84\x03\x12\x15aK\xEEW__\xFD[aK\xF6a@\xD6V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12aL\x1CW__\xFD[aL$a@\xD6V[\x80`@\x84\x01\x85\x81\x11\x15aL5W__\xFD[\x84[\x81\x81\x10\x15aJ3W\x805\x84R` \x93\x84\x01\x93\x01aL7V[_\x81\x83\x03a\x01\0\x81\x12\x15aLaW__\xFD[aLia@\xAEV[\x91PaLu\x84\x84aK\xDEV[\x82RaL\x84\x84`@\x85\x01aK\xDEV[` \x83\x01R`\x80`\x7F\x19\x82\x01\x12\x15aL\x9AW__\xFD[PaL\xA3a@\xD6V[aL\xB0\x84`\x80\x85\x01aL\rV[\x81RaL\xBF\x84`\xC0\x85\x01aL\rV[` \x82\x01R`@\x82\x01R\x92\x91PPV[___a\x01@\x84\x86\x03\x12\x15aL\xE2W__\xFD[aL\xEB\x84aK\xD0V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x05W__\xFD[aM\x11\x86\x82\x87\x01aB_V[\x92PPaM!\x85`@\x86\x01aLOV[\x90P\x92P\x92P\x92V[_____a\x01\x80\x86\x88\x03\x12\x15aM?W__\xFD[aMH\x86aK\xD0V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aMbW__\xFD[aMn\x88\x82\x89\x01aB_V[\x94PPaM~\x87`@\x88\x01aLOV[\x92Pa\x01@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x99W__\xFD[aM\xA5\x88\x82\x89\x01aHXV[\x92PPa\x01`\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xC1W__\xFD[\x86\x01``\x81\x89\x03\x12\x15aM\xD2W__\xFD[aM\xDAa@\xAEV[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xEFW__\xFD[aM\xFB\x8A\x82\x85\x01aB_V[\x82RP` \x82\x81\x015\x90\x82\x01R`@\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x94\x97\x93\x96P\x91\x94P\x92\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01aNHWaNHaN#V[P`\x01\x01\x90V[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R_\x90` \x86\x01\x90`\x80\x84\x01\x90\x83[\x81\x81\x10\x15aN\x91W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aNjV[PP\x83\x81\x03` \x80\x86\x01\x91\x90\x91R\x86Q\x80\x83R\x91\x81\x01\x92P\x86\x01\x90_[\x81\x81\x10\x15aN\xCCW\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aN\xAEV[PPP`\xFF\x84\x16`@\x84\x01R\x90Pa\x1C\xACV[_` \x82\x84\x03\x12\x15aN\xEFW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\x04W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aO\x14W__\xFD[\x80QaO\"aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aOCW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x16ZWaO[\x84aJ\xEDV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aOJV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x82\x81R`@` \x82\x01R_a\x0B'`@\x83\x01\x84aOlV[` \x81R_a\x0B*` \x83\x01\x84aOlV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x0B'\x90\x83\x01\x84aOlV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aP\tWaP\taO\xE7V[P\x06\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xFDWa\x0C\xFDaN#V[_`@\x82\x01`\x01\x80`\xA0\x1B\x03\x85\x16\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aP\xDCW\x86\x85\x03`_\x19\x01\x84R\x81Q\x80Qc\xFF\xFF\xFF\xFF\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15aP\xC4W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90aP\x99V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aP[V[P\x92\x97\x96PPPPPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aQ7W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x88\x01R`@\x90\x96\x01\x95\x90\x91\x01\x90`\x01\x01aP\xFBV[P\x93\x94\x93PPPPV[`\xFF\x84\x16\x81R`\x01`\x01``\x1B\x03\x83\x16` \x82\x01R```@\x82\x01R_aQk``\x83\x01\x84aP\xE9V[\x95\x94PPPPPV[`\xFF\x85\x16\x81R`\x01`\x01``\x1B\x03\x84\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R_a\x16Z`\x80\x83\x01\x84aP\xE9V[_` \x82\x84\x03\x12\x15aQ\xBAW__\xFD[PQ\x91\x90PV[\x80_[`\x02\x81\x10\x15a\x0E\xBDW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aQ\xC4V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R\x82Q\x80Q` \x80\x84\x01\x91\x90\x91R\x01Q`@\x82\x01Ra\x01`\x81\x01` \x84\x81\x01Q\x80Q``\x85\x01R\x90\x81\x01Q`\x80\x84\x01RP`@\x84\x01QaR2`\xA0\x84\x01\x82QaQ\xC1V[` \x01QaRC`\xE0\x84\x01\x82aQ\xC1V[P\x82Qa\x01 \x83\x01R` \x83\x01Qa\x01@\x83\x01Ra\x1C\xACV[\x80\x82\x01\x80\x82\x11\x15a\x0C\xFDWa\x0C\xFDaN#V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R_aQk``\x83\x01\x84aOlV[_\x82`\x1F\x83\x01\x12aR\xA4W__\xFD[\x81QaR\xB2aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aR\xD3W__\xFD[` \x85\x01[\x83\x81\x10\x15aA\xC7W\x80QaR\xEB\x81aD\xA2V[\x83R` \x92\x83\x01\x92\x01aR\xD8V[__`@\x83\x85\x03\x12\x15aS\nW__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x1FW__\xFD[aS+\x85\x82\x86\x01aR\x95V[\x92PP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aSFW__\xFD[aG\xFC\x85\x82\x86\x01aR\x95V[_` \x82\x84\x03\x12\x15aSbW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aSwW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aS\x87W__\xFD[\x80QaS\x95aA{\x82aA(V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aS\xB6W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x16ZW\x83QaS\xD0\x81aB\x19V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aS\xBDV[\x81\x81\x03\x81\x81\x11\x15a\x0C\xFDWa\x0C\xFDaN#V[_a\xFF\xFF\x82\x16a\xFF\xFF\x81\x03aT\x0BWaT\x0BaN#V[`\x01\x01\x92\x91PPV[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x83\x01R\x83\x82\x01Q\x16`@\x80\x84\x01\x91\x90\x91R\x83\x01Q``\x80\x84\x01R\x80Q`\x80\x84\x01\x81\x90R_\x92\x91\x90\x91\x01\x90\x82\x90`\xA0\x85\x01\x90[\x80\x83\x10\x15aA\xC7Wc\xFF\xFF\xFF\xFF\x84Q\x16\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92PaTYV[_` \x82\x84\x03\x12\x15aT\x92W__\xFD[\x81Qa\x0B*\x81aD\xA2V[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x1F\xB2Wa\x1F\xB2aN#V[_`\x01`\x01``\x1B\x03\x83\x16\x80aT\xD7WaT\xD7aO\xE7V[\x80`\x01`\x01``\x1B\x03\x84\x16\x04\x91PP\x92\x91PPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15aU\x12W__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B*W__\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xAC\xF4Oa\xBD\xE5\x1B|thM\tlU\xF7e?\xC5\xA4\x1D\xEES\xD9K\"\xAE\xDB\xD5\xA5\xE8\xD9fdsolcC\0\x08\x1B\x003",
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AlreadyRegisteredForQuorums> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyRegisteredForQuorums) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyRegisteredForQuorums {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyRegisteredForQuorums {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BitmapUpdateIsAfterBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapUpdateIsAfterBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapUpdateIsAfterBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapUpdateIsAfterBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CannotKickOperatorAboveThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: CannotKickOperatorAboveThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotKickOperatorAboveThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotKickOperatorAboveThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientStakeForChurn> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientStakeForChurn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientStakeForChurn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientStakeForChurn {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `InvalidAVS()` and selector `0x66e565df`.
    ```solidity
    error InvalidAVS();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAVS {}
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
        impl ::core::convert::From<InvalidAVS> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAVS) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAVS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAVS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAVS()";
            const SELECTOR: [u8; 4] = [102u8, 229u8, 101u8, 223u8];
            #[inline]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NextBitmapUpdateIsBeforeBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: NextBitmapUpdateIsBeforeBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NextBitmapUpdateIsBeforeBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NextBitmapUpdateIsBeforeBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QuorumOperatorCountMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumOperatorCountMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumOperatorCountMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumOperatorCountMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ChurnApproverUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 84u8, 87u8, 216u8, 168u8, 254u8, 96u8, 240u8, 74u8, 241u8, 124u8, 22u8,
                    226u8, 245u8, 165u8, 225u8, 219u8, 97u8, 43u8, 49u8, 100u8, 142u8, 88u8, 3u8,
                    3u8, 96u8, 117u8, 158u8, 248u8, 243u8, 82u8, 140u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EjectorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    143u8, 48u8, 171u8, 9u8, 244u8, 58u8, 108u8, 21u8, 125u8, 127u8, 206u8, 126u8,
                    10u8, 19u8, 192u8, 3u8, 4u8, 44u8, 28u8, 149u8, 232u8, 167u8, 46u8, 122u8,
                    20u8, 106u8, 33u8, 192u8, 202u8, 162u8, 77u8, 201u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    57u8, 111u8, 220u8, 177u8, 128u8, 203u8, 15u8, 234u8, 38u8, 146u8, 129u8, 19u8,
                    251u8, 15u8, 209u8, 195u8, 84u8, 152u8, 99u8, 249u8, 205u8, 86u8, 62u8, 106u8,
                    24u8, 79u8, 29u8, 87u8, 129u8, 22u8, 200u8, 228u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    232u8, 230u8, 140u8, 239u8, 28u8, 58u8, 118u8, 30u8, 215u8, 190u8, 126u8,
                    132u8, 99u8, 163u8, 117u8, 242u8, 127u8, 123u8, 195u8, 53u8, 229u8, 24u8, 36u8,
                    34u8, 60u8, 172u8, 206u8, 99u8, 110u8, 197u8, 195u8, 254u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str =
                "OperatorSetParamsUpdated(uint8,(uint32,uint16,uint16))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    62u8, 230u8, 254u8, 141u8, 84u8, 97u8, 2u8, 68u8, 195u8, 233u8, 211u8, 192u8,
                    102u8, 174u8, 74u8, 238u8, 153u8, 120u8, 132u8, 170u8, 40u8, 241u8, 6u8, 22u8,
                    174u8, 130u8, 25u8, 37u8, 64u8, 19u8, 24u8, 172u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &OperatorSetParamsUpdated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorSocketUpdate(bytes32,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    236u8, 41u8, 99u8, 171u8, 33u8, 193u8, 229u8, 14u8, 30u8, 88u8, 42u8, 165u8,
                    66u8, 175u8, 46u8, 75u8, 247u8, 191u8, 56u8, 230u8, 225u8, 64u8, 60u8, 39u8,
                    180u8, 46u8, 28u8, 93u8, 110u8, 98u8, 30u8, 170u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "QuorumBlockNumberUpdated(uint8,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    70u8, 7u8, 125u8, 85u8, 51u8, 7u8, 99u8, 241u8, 98u8, 105u8, 253u8, 117u8,
                    229u8, 118u8, 22u8, 99u8, 244u8, 25u8, 45u8, 39u8, 145u8, 116u8, 124u8, 1u8,
                    137u8, 177u8, 106u8, 211u8, 29u8, 176u8, 125u8, 180u8,
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
                        &self.blocknumber,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &QuorumBlockNumberUpdated) -> alloy_sol_types::private::LogData {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<OPERATOR_CHURN_APPROVAL_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_CHURN_APPROVAL_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_CHURN_APPROVAL_TYPEHASHCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<OPERATOR_CHURN_APPROVAL_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_CHURN_APPROVAL_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_CHURN_APPROVAL_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_CHURN_APPROVAL_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = OPERATOR_CHURN_APPROVAL_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PUBKEY_REGISTRATION_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: PUBKEY_REGISTRATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PUBKEY_REGISTRATION_TYPEHASHCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PUBKEY_REGISTRATION_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: PUBKEY_REGISTRATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PUBKEY_REGISTRATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PUBKEY_REGISTRATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = PUBKEY_REGISTRATION_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `avs()` and selector `0xde1164bb`.
    ```solidity
    function avs() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsCall {}
    ///Container type for the return parameters of the [`avs()`](avsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsReturn {
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
            impl ::core::convert::From<avsCall> for UnderlyingRustTuple<'_> {
                fn from(value: avsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsCall {
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
            impl ::core::convert::From<avsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: avsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avs()";
            const SELECTOR: [u8; 4] = [222u8, 17u8, 100u8, 187u8];
            #[inline]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateOperatorChurnApprovalDigestHashCall>
                for UnderlyingRustTuple<'_>
            {
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
                for calculateOperatorChurnApprovalDigestHashCall
            {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateOperatorChurnApprovalDigestHashReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculateOperatorChurnApprovalDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculateOperatorChurnApprovalDigestHashReturn
            {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateOperatorChurnApprovalDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `calculatePubkeyRegistrationMessageHash(address)` and selector `0x73447992`.
    ```solidity
    function calculatePubkeyRegistrationMessageHash(address operator) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculatePubkeyRegistrationMessageHashCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`calculatePubkeyRegistrationMessageHash(address)`](calculatePubkeyRegistrationMessageHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculatePubkeyRegistrationMessageHashReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculatePubkeyRegistrationMessageHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculatePubkeyRegistrationMessageHashCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculatePubkeyRegistrationMessageHashCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculatePubkeyRegistrationMessageHashReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculatePubkeyRegistrationMessageHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculatePubkeyRegistrationMessageHashReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculatePubkeyRegistrationMessageHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculatePubkeyRegistrationMessageHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculatePubkeyRegistrationMessageHash(address)";
            const SELECTOR: [u8; 4] = [115u8, 68u8, 121u8, 146u8];
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = churnApproverReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createSlashableStakeQuorumCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createSlashableStakeQuorumCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createSlashableStakeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createSlashableStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createSlashableStakeQuorumReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createSlashableStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createTotalDelegatedStakeQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: createTotalDelegatedStakeQuorumCall) -> Self {
                    (
                        value.operatorSetParams,
                        value.minimumStake,
                        value.strategyParams,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createTotalDelegatedStakeQuorumCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createTotalDelegatedStakeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createTotalDelegatedStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createTotalDelegatedStakeQuorumReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createTotalDelegatedStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "createTotalDelegatedStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `deregisterOperator(address,address,uint32[])` and selector `0x303ca956`.
    ```solidity
    function deregisterOperator(address operator, address avs, uint32[] memory operatorSetIds) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`deregisterOperator(address,address,uint32[])`](deregisterOperatorCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<deregisterOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.operator, value.avs, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        avs: tuple.1,
                        operatorSetIds: tuple.2,
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
            impl ::core::convert::From<deregisterOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator(address,address,uint32[])";
            const SELECTOR: [u8; 4] = [48u8, 60u8, 169u8, 86u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ejectionCooldownCall> for UnderlyingRustTuple<'_> {
                fn from(value: ejectionCooldownCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectionCooldownCall {
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
            impl ::core::convert::From<ejectionCooldownReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ejectionCooldownReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectionCooldownReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectionCooldownCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectionCooldownReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentQuorumBitmapCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentQuorumBitmapCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentQuorumBitmapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U192,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentQuorumBitmapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentQuorumBitmapReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentQuorumBitmapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentQuorumBitmapCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentQuorumBitmapReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorInfo as alloy::sol_types::SolType>::RustType,
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorFromIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromIdCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorFromIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                    }
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
            impl ::core::convert::From<getOperatorFromIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorFromIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorFromIdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorFromIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSetParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetParamsCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorSetParam,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorSetParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetParamsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetParamsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSetParamsReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorSetParam,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorStatusCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStatusCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorStatus,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorStatusReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStatusReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorStatusCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorStatusReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorStatus,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapAtBlockNumberByIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapAtBlockNumberByIndexCall) -> Self {
                    (value.operatorId, value.blockNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapAtBlockNumberByIndexCall {
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U192,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapAtBlockNumberByIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapAtBlockNumberByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapAtBlockNumberByIndexReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapAtBlockNumberByIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapHistoryLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapHistoryLengthCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                    }
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
            impl ::core::convert::From<getQuorumBitmapHistoryLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapHistoryLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
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
            impl ::core::convert::From<getQuorumBitmapIndicesAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapIndicesAtBlockNumberCall) -> Self {
                    (value.blockNumber, value.operatorIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapIndicesAtBlockNumberCall {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapIndicesAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapIndicesAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapIndicesAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapIndicesAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapIndicesAtBlockNumberReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapUpdateByIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapUpdateByIndexCall) -> Self {
                    (value.operatorId, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapUpdateByIndexCall {
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
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getQuorumBitmapUpdateByIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapUpdateByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapUpdateByIndexReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapUpdateByIndexReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = indexRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,address,address,uint256,address)` and selector `0x530b97a4`.
    ```solidity
    function initialize(address _initialOwner, address _churnApprover, address _ejector, uint256 _initialPausedStatus, address _avs) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _initialOwner: alloy::sol_types::private::Address,
        pub _churnApprover: alloy::sol_types::private::Address,
        pub _ejector: alloy::sol_types::private::Address,
        pub _initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        pub _avs: alloy::sol_types::private::Address,
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
                        value._initialOwner,
                        value._churnApprover,
                        value._ejector,
                        value._initialPausedStatus,
                        value._avs,
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
                        _avs: tuple.4,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._initialPausedStatus,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._avs,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isChurnApproverSaltUsedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isChurnApproverSaltUsedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isChurnApproverSaltUsedCall {
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
            impl ::core::convert::From<isChurnApproverSaltUsedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isChurnApproverSaltUsedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isChurnApproverSaltUsedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isChurnApproverSaltUsedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isChurnApproverSaltUsedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lastEjectionTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastEjectionTimestampCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastEjectionTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<lastEjectionTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lastEjectionTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastEjectionTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastEjectionTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastEjectionTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = numRegistriesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pubkeyRegistrationMessageHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyRegistrationMessageHashCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyRegistrationMessageHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BN254::G1Point,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<BN254::G1Point as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pubkeyRegistrationMessageHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyRegistrationMessageHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyRegistrationMessageHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pubkeyRegistrationMessageHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyRegistrationMessageHashReturn;
            type ReturnTuple<'a> = (BN254::G1Point,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumUpdateBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumUpdateBlockNumberCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumUpdateBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<quorumUpdateBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumUpdateBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumUpdateBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumUpdateBlockNumberCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumUpdateBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `registerOperator(address,address,uint32[],bytes)` and selector `0xc63fd502`.
    ```solidity
    function registerOperator(address operator, address avs, uint32[] memory operatorSetIds, bytes memory data) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(address,address,uint32[],bytes)`](registerOperatorCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
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
            impl ::core::convert::From<registerOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.operator, value.avs, value.operatorSetIds, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        avs: tuple.1,
                        operatorSetIds: tuple.2,
                        data: tuple.3,
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
            impl ::core::convert::From<registerOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(address,address,uint32[],bytes)";
            const SELECTOR: [u8; 4] = [198u8, 63u8, 213u8, 2u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registriesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `setAvs(address)` and selector `0xa57f4c6f`.
    ```solidity
    function setAvs(address _avs) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAvsCall {
        pub _avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setAvs(address)`](setAvsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAvsReturn {}
    #[allow(
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
            impl ::core::convert::From<setAvsCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAvsCall) -> Self {
                    (value._avs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAvsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _avs: tuple.0 }
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
            impl ::core::convert::From<setAvsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAvsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAvsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAvsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAvsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAvs(address)";
            const SELECTOR: [u8; 4] = [165u8, 127u8, 76u8, 111u8];
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
                        &self._avs,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setChurnApproverCall> for UnderlyingRustTuple<'_> {
                fn from(value: setChurnApproverCall) -> Self {
                    (value._churnApprover,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setChurnApproverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _churnApprover: tuple.0,
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
            impl ::core::convert::From<setChurnApproverReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setChurnApproverReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setChurnApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setChurnApproverCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setChurnApproverReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            impl ::core::convert::From<setEjectionCooldownCall> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectionCooldownCall) -> Self {
                    (value._ejectionCooldown,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectionCooldownCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ejectionCooldown: tuple.0,
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
            impl ::core::convert::From<setEjectionCooldownReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectionCooldownReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectionCooldownReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEjectionCooldownCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEjectionCooldownReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._ejectionCooldown,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEjectorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorSetParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorSetParamsCall) -> Self {
                    (value.quorumNumber, value.operatorSetParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorSetParamsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorSetParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorSetParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorSetParamsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorSetParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<socketRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: socketRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for socketRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for socketRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = socketRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `supportsAVS(address)` and selector `0xb5265787`.
    ```solidity
    function supportsAVS(address _avs) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsAVSCall {
        pub _avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`supportsAVS(address)`](supportsAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsAVSReturn {
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
            impl ::core::convert::From<supportsAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: supportsAVSCall) -> Self {
                    (value._avs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supportsAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _avs: tuple.0 }
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
            impl ::core::convert::From<supportsAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: supportsAVSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supportsAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = supportsAVSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsAVS(address)";
            const SELECTOR: [u8; 4] = [181u8, 38u8, 87u8, 135u8];
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
                        &self._avs,
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
    /**Function with signature `updateOperators(address[])` and selector `0x00cf2ab5`.
    ```solidity
    function updateOperators(address[] memory operators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsCall {
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.operators
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumCall) -> Self {
                    (value.operatorsPerQuorum, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                        alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.operatorsPerQuorum
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateSocketReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`SlashingRegistryCoordinator`](self) function calls.
    pub enum SlashingRegistryCoordinatorCalls {
        OPERATOR_CHURN_APPROVAL_TYPEHASH(OPERATOR_CHURN_APPROVAL_TYPEHASHCall),
        PUBKEY_REGISTRATION_TYPEHASH(PUBKEY_REGISTRATION_TYPEHASHCall),
        allocationManager(allocationManagerCall),
        avs(avsCall),
        blsApkRegistry(blsApkRegistryCall),
        calculateOperatorChurnApprovalDigestHash(calculateOperatorChurnApprovalDigestHashCall),
        calculatePubkeyRegistrationMessageHash(calculatePubkeyRegistrationMessageHashCall),
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
        lastEjectionTimestamp(lastEjectionTimestampCall),
        numRegistries(numRegistriesCall),
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
        setAvs(setAvsCall),
        setChurnApprover(setChurnApproverCall),
        setEjectionCooldown(setEjectionCooldownCall),
        setEjector(setEjectorCall),
        setOperatorSetParams(setOperatorSetParamsCall),
        socketRegistry(socketRegistryCall),
        stakeRegistry(stakeRegistryCall),
        supportsAVS(supportsAVSCall),
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
            [12u8, 244u8, 183u8, 103u8],
            [13u8, 63u8, 33u8, 52u8],
            [18u8, 94u8, 5u8, 132u8],
            [19u8, 84u8, 42u8, 78u8],
            [19u8, 100u8, 57u8, 221u8],
            [20u8, 120u8, 133u8, 31u8],
            [30u8, 184u8, 18u8, 218u8],
            [36u8, 154u8, 12u8, 66u8],
            [40u8, 246u8, 27u8, 49u8],
            [41u8, 107u8, 176u8, 100u8],
            [41u8, 209u8, 224u8, 195u8],
            [44u8, 221u8, 30u8, 134u8],
            [48u8, 60u8, 169u8, 86u8],
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
            [115u8, 68u8, 121u8, 146u8],
            [130u8, 129u8, 171u8, 117u8],
            [132u8, 202u8, 82u8, 19u8],
            [135u8, 30u8, 240u8, 73u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [154u8, 161u8, 101u8, 61u8],
            [158u8, 153u8, 35u8, 194u8],
            [159u8, 234u8, 184u8, 89u8],
            [165u8, 127u8, 76u8, 111u8],
            [169u8, 111u8, 120u8, 62u8],
            [181u8, 38u8, 87u8, 135u8],
            [195u8, 145u8, 66u8, 94u8],
            [198u8, 63u8, 213u8, 2u8],
            [202u8, 13u8, 232u8, 130u8],
            [202u8, 138u8, 167u8, 199u8],
            [215u8, 45u8, 141u8, 214u8],
            [222u8, 17u8, 100u8, 187u8],
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
        const COUNT: usize = 54usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(_) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(_) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avs(_) => <avsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorChurnApprovalDigestHash(_) => {
                    <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculatePubkeyRegistrationMessageHash(_) => {
                    <calculatePubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::lastEjectionTimestamp(_) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::numRegistries(_) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::setAvs(_) => <setAvsCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::supportsAVS(_) => {
                    <supportsAVSCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<
                SlashingRegistryCoordinatorCalls,
            >] = &[
                {
                    fn updateOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <updateOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::updateOperators)
                    }
                    updateOperators
                },
                {
                    fn getQuorumBitmapHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <churnApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::churnApprover)
                    }
                    churnApprover
                },
                {
                    fn updateSocket(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <updateSocketCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::updateSocket)
                    }
                    updateSocket
                },
                {
                    fn setEjectionCooldown(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <setEjectionCooldownCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::setEjectionCooldown)
                    }
                    setEjectionCooldown
                },
                {
                    fn lastEjectionTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::lastEjectionTimestamp)
                    }
                    lastEjectionTimestamp
                },
                {
                    fn getOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getOperatorId)
                    }
                    getOperatorId
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::pause)
                    }
                    pause
                },
                {
                    fn isChurnApproverSaltUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::isChurnApproverSaltUsed)
                    }
                    isChurnApproverSaltUsed
                },
                {
                    fn getQuorumBitmapUpdateByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::quorumUpdateBlockNumber)
                    }
                    quorumUpdateBlockNumber
                },
                {
                    fn ejector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <ejectorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::ejector)
                    }
                    ejector
                },
                {
                    fn getOperatorFromId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getOperatorFromId)
                    }
                    getOperatorFromId
                },
                {
                    fn setChurnApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <setChurnApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::setChurnApprover)
                    }
                    setChurnApprover
                },
                {
                    fn setEjector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <setEjectorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::setEjector)
                    }
                    setEjector
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn pubkeyRegistrationMessageHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::updateOperatorsForQuorum)
                    }
                    updateOperatorsForQuorum
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getOperator)
                    }
                    getOperator
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn setOperatorSetParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <setOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::setOperatorSetParams)
                    }
                    setOperatorSetParams
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn registries(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <registriesCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::registries)
                    }
                    registries
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn ejectOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <ejectOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::ejectOperator)
                    }
                    ejectOperator
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn calculatePubkeyRegistrationMessageHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <calculatePubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::calculatePubkeyRegistrationMessageHash,
                            )
                    }
                    calculatePubkeyRegistrationMessageHash
                },
                {
                    fn createTotalDelegatedStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getCurrentQuorumBitmap)
                    }
                    getCurrentQuorumBitmap
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::owner)
                    }
                    owner
                },
                {
                    fn quorumCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <quorumCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::quorumCount)
                    }
                    quorumCount
                },
                {
                    fn indexRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <indexRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::indexRegistry)
                    }
                    indexRegistry
                },
                {
                    fn PUBKEY_REGISTRATION_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
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
                    fn setAvs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <setAvsCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::setAvs)
                    }
                    setAvs
                },
                {
                    fn ejectionCooldown(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <ejectionCooldownCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::ejectionCooldown)
                    }
                    ejectionCooldown
                },
                {
                    fn supportsAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <supportsAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::supportsAVS)
                    }
                    supportsAVS
                },
                {
                    fn getQuorumBitmapIndicesAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
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
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn OPERATOR_CHURN_APPROVAL_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn numRegistries(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <numRegistriesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::numRegistries)
                    }
                    numRegistries
                },
                {
                    fn avs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <avsCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::avs)
                    }
                    avs
                },
                {
                    fn getOperatorSetParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getOperatorSetParams)
                    }
                    getOperatorSetParams
                },
                {
                    fn socketRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <socketRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::socketRegistry)
                    }
                    socketRegistry
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::unpause)
                    }
                    unpause
                },
                {
                    fn getOperatorStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getOperatorStatus)
                    }
                    getOperatorStatus
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
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avs(inner) => {
                    <avsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::calculatePubkeyRegistrationMessageHash(inner) => {
                    <calculatePubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::lastEjectionTimestamp(inner) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::numRegistries(inner) => {
                    <numRegistriesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setAvs(inner) => {
                    <setAvsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::supportsAVS(inner) => {
                    <supportsAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::avs(inner) => {
                    <avsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::calculatePubkeyRegistrationMessageHash(inner) => {
                    <calculatePubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::lastEjectionTimestamp(inner) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setAvs(inner) => {
                    <setAvsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::supportsAVS(inner) => {
                    <supportsAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        InvalidAVS(InvalidAVS),
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
            [102u8, 229u8, 101u8, 223u8],
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
                Self::BitmapEmpty(_) => <BitmapEmpty as alloy_sol_types::SolError>::SELECTOR,
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
                Self::ExpModFailed(_) => <ExpModFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputLengthMismatch(_) => {
                    <InputLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientStakeForChurn(_) => {
                    <InsufficientStakeForChurn as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAVS(_) => <InvalidAVS as alloy_sol_types::SolError>::SELECTOR,
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
                Self::NotRegistered(_) => <NotRegistered as alloy_sol_types::SolError>::SELECTOR,
                Self::NotRegisteredForQuorum(_) => {
                    <NotRegisteredForQuorum as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSorted(_) => <NotSorted as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyAllocationManager(_) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEjector(_) => <OnlyEjector as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
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
            ) -> alloy_sol_types::Result<
                SlashingRegistryCoordinatorErrors,
            >] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn AlreadyRegisteredForQuorums(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::AlreadyRegisteredForQuorums)
                    }
                    AlreadyRegisteredForQuorums
                },
                {
                    fn BitmapEmpty(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BitmapEmpty as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::BitmapEmpty)
                    }
                    BitmapEmpty
                },
                {
                    fn OnlyAllocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <OnlyAllocationManager as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::OnlyAllocationManager)
                    }
                    OnlyAllocationManager
                },
                {
                    fn CannotReregisterYet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <CannotReregisterYet as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::CannotReregisterYet)
                    }
                    CannotReregisterYet
                },
                {
                    fn InvalidRegistrationType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InvalidRegistrationType as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InvalidRegistrationType)
                    }
                    InvalidRegistrationType
                },
                {
                    fn MaxQuorumsReached(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <MaxQuorumsReached as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::MaxQuorumsReached)
                    }
                    MaxQuorumsReached
                },
                {
                    fn InsufficientStakeForChurn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InsufficientStakeForChurn as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InsufficientStakeForChurn)
                    }
                    InsufficientStakeForChurn
                },
                {
                    fn InvalidAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InvalidAVS as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::InvalidAVS)
                    }
                    InvalidAVS
                },
                {
                    fn BitmapUpdateIsAfterBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::BytesArrayNotOrdered)
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn QuorumOperatorCountMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::QuorumOperatorCountMismatch)
                    }
                    QuorumOperatorCountMismatch
                },
                {
                    fn InputLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InputLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InputLengthMismatch)
                    }
                    InputLengthMismatch
                },
                {
                    fn NotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <NotRegistered as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::NotRegistered)
                    }
                    NotRegistered
                },
                {
                    fn CannotChurnSelf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <CannotChurnSelf as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::CannotChurnSelf)
                    }
                    CannotChurnSelf
                },
                {
                    fn CannotKickOperatorAboveThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn NextBitmapUpdateIsBeforeBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
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
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn NotRegisteredForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <NotRegisteredForQuorum as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::NotRegisteredForQuorum)
                    }
                    NotRegisteredForQuorum
                },
                {
                    fn BitmapCannotBeZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BitmapCannotBeZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::BitmapCannotBeZero)
                    }
                    BitmapCannotBeZero
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn ChurnApproverSaltUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <ChurnApproverSaltUsed as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::ChurnApproverSaltUsed)
                    }
                    ChurnApproverSaltUsed
                },
                {
                    fn QuorumDoesNotExist(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::QuorumDoesNotExist)
                    }
                    QuorumDoesNotExist
                },
                {
                    fn OnlyEjector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <OnlyEjector as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::OnlyEjector)
                    }
                    OnlyEjector
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::BytesArrayLengthTooLong)
                    }
                    BytesArrayLengthTooLong
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
                Self::InvalidAVS(inner) => {
                    <InvalidAVS as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InvalidAVS(inner) => {
                    <InvalidAVS as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                49u8, 84u8, 87u8, 216u8, 168u8, 254u8, 96u8, 240u8, 74u8, 241u8, 124u8, 22u8,
                226u8, 245u8, 165u8, 225u8, 219u8, 97u8, 43u8, 49u8, 100u8, 142u8, 88u8, 3u8, 3u8,
                96u8, 117u8, 158u8, 248u8, 243u8, 82u8, 140u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                57u8, 111u8, 220u8, 177u8, 128u8, 203u8, 15u8, 234u8, 38u8, 146u8, 129u8, 19u8,
                251u8, 15u8, 209u8, 195u8, 84u8, 152u8, 99u8, 249u8, 205u8, 86u8, 62u8, 106u8,
                24u8, 79u8, 29u8, 87u8, 129u8, 22u8, 200u8, 228u8,
            ],
            [
                62u8, 230u8, 254u8, 141u8, 84u8, 97u8, 2u8, 68u8, 195u8, 233u8, 211u8, 192u8,
                102u8, 174u8, 74u8, 238u8, 153u8, 120u8, 132u8, 170u8, 40u8, 241u8, 6u8, 22u8,
                174u8, 130u8, 25u8, 37u8, 64u8, 19u8, 24u8, 172u8,
            ],
            [
                70u8, 7u8, 125u8, 85u8, 51u8, 7u8, 99u8, 241u8, 98u8, 105u8, 253u8, 117u8, 229u8,
                118u8, 22u8, 99u8, 244u8, 25u8, 45u8, 39u8, 145u8, 116u8, 124u8, 1u8, 137u8, 177u8,
                106u8, 211u8, 29u8, 176u8, 125u8, 180u8,
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
                143u8, 48u8, 171u8, 9u8, 244u8, 58u8, 108u8, 21u8, 125u8, 127u8, 206u8, 126u8,
                10u8, 19u8, 192u8, 3u8, 4u8, 44u8, 28u8, 149u8, 232u8, 167u8, 46u8, 122u8, 20u8,
                106u8, 33u8, 192u8, 202u8, 162u8, 77u8, 201u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                232u8, 230u8, 140u8, 239u8, 28u8, 58u8, 118u8, 30u8, 215u8, 190u8, 126u8, 132u8,
                99u8, 163u8, 117u8, 242u8, 127u8, 123u8, 195u8, 53u8, 229u8, 24u8, 36u8, 34u8,
                60u8, 172u8, 206u8, 99u8, 110u8, 197u8, 195u8, 254u8,
            ],
            [
                236u8, 41u8, 99u8, 171u8, 33u8, 193u8, 229u8, 14u8, 30u8, 88u8, 42u8, 165u8, 66u8,
                175u8, 46u8, 75u8, 247u8, 191u8, 56u8, 230u8, 225u8, 64u8, 60u8, 39u8, 180u8, 46u8,
                28u8, 93u8, 110u8, 98u8, 30u8, 170u8,
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
                Some(<ChurnApproverUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChurnApproverUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ChurnApproverUpdated)
                }
                Some(<EjectorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EjectorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::EjectorUpdated)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<OperatorDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorDeregistered)
                }
                Some(<OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorRegistered)
                }
                Some(<OperatorSetParamsUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSetParamsUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSetParamsUpdated)
                }
                Some(<OperatorSocketUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSocketUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSocketUpdate)
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
                Some(<QuorumBlockNumberUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumBlockNumberUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumBlockNumberUpdated)
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
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::QuorumBlockNumberUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
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
        SlashingRegistryCoordinatorInstance::<T, P, N>::deploy(
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
        SlashingRegistryCoordinatorInstance::<T, P, N>::deploy_builder(
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
    pub struct SlashingRegistryCoordinatorInstance<T, P, N = alloy_contract::private::Ethereum> {
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
        > SlashingRegistryCoordinatorInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`SlashingRegistryCoordinator`](self) contract instance.

        See the [wrapper's documentation](`SlashingRegistryCoordinatorInstance`) for more details.*/
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _stakeRegistry,
                        _blsApkRegistry,
                        _indexRegistry,
                        _socketRegistry,
                        _allocationManager,
                        _pauserRegistry,
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
    impl<T, P: ::core::clone::Clone, N> SlashingRegistryCoordinatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SlashingRegistryCoordinatorInstance<T, P, N> {
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
        > SlashingRegistryCoordinatorInstance<T, P, N>
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
        ///Creates a new call builder for the [`OPERATOR_CHURN_APPROVAL_TYPEHASH`] function.
        pub fn OPERATOR_CHURN_APPROVAL_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, OPERATOR_CHURN_APPROVAL_TYPEHASHCall, N>
        {
            self.call_builder(&OPERATOR_CHURN_APPROVAL_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`PUBKEY_REGISTRATION_TYPEHASH`] function.
        pub fn PUBKEY_REGISTRATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, PUBKEY_REGISTRATION_TYPEHASHCall, N> {
            self.call_builder(&PUBKEY_REGISTRATION_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`avs`] function.
        pub fn avs(&self) -> alloy_contract::SolCallBuilder<T, &P, avsCall, N> {
            self.call_builder(&avsCall {})
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
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateOperatorChurnApprovalDigestHashCall, N>
        {
            self.call_builder(&calculateOperatorChurnApprovalDigestHashCall {
                registeringOperator,
                registeringOperatorId,
                operatorKickParams,
                salt,
                expiry,
            })
        }
        ///Creates a new call builder for the [`calculatePubkeyRegistrationMessageHash`] function.
        pub fn calculatePubkeyRegistrationMessageHash(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculatePubkeyRegistrationMessageHashCall, N>
        {
            self.call_builder(&calculatePubkeyRegistrationMessageHashCall { operator })
        }
        ///Creates a new call builder for the [`churnApprover`] function.
        pub fn churnApprover(&self) -> alloy_contract::SolCallBuilder<T, &P, churnApproverCall, N> {
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
            self.call_builder(&createSlashableStakeQuorumCall {
                operatorSetParams,
                minimumStake,
                strategyParams,
                lookAheadPeriod,
            })
        }
        ///Creates a new call builder for the [`createTotalDelegatedStakeQuorum`] function.
        pub fn createTotalDelegatedStakeQuorum(
            &self,
            operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createTotalDelegatedStakeQuorumCall, N> {
            self.call_builder(&createTotalDelegatedStakeQuorumCall {
                operatorSetParams,
                minimumStake,
                strategyParams,
            })
        }
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            avs: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(&deregisterOperatorCall {
                operator,
                avs,
                operatorSetIds,
            })
        }
        ///Creates a new call builder for the [`ejectOperator`] function.
        pub fn ejectOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, ejectOperatorCall, N> {
            self.call_builder(&ejectOperatorCall {
                operator,
                quorumNumbers,
            })
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
            self.call_builder(&getCurrentQuorumBitmapCall { operatorId })
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
            self.call_builder(&getOperatorFromIdCall { operatorId })
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
            self.call_builder(&getOperatorSetParamsCall { quorumNumber })
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
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapAtBlockNumberByIndexCall, N>
        {
            self.call_builder(&getQuorumBitmapAtBlockNumberByIndexCall {
                operatorId,
                blockNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`getQuorumBitmapHistoryLength`] function.
        pub fn getQuorumBitmapHistoryLength(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapHistoryLengthCall, N> {
            self.call_builder(&getQuorumBitmapHistoryLengthCall { operatorId })
        }
        ///Creates a new call builder for the [`getQuorumBitmapIndicesAtBlockNumber`] function.
        pub fn getQuorumBitmapIndicesAtBlockNumber(
            &self,
            blockNumber: u32,
            operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapIndicesAtBlockNumberCall, N>
        {
            self.call_builder(&getQuorumBitmapIndicesAtBlockNumberCall {
                blockNumber,
                operatorIds,
            })
        }
        ///Creates a new call builder for the [`getQuorumBitmapUpdateByIndex`] function.
        pub fn getQuorumBitmapUpdateByIndex(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapUpdateByIndexCall, N> {
            self.call_builder(&getQuorumBitmapUpdateByIndexCall { operatorId, index })
        }
        ///Creates a new call builder for the [`indexRegistry`] function.
        pub fn indexRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, indexRegistryCall, N> {
            self.call_builder(&indexRegistryCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _initialOwner: alloy::sol_types::private::Address,
            _churnApprover: alloy::sol_types::private::Address,
            _ejector: alloy::sol_types::private::Address,
            _initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
            _avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                _initialOwner,
                _churnApprover,
                _ejector,
                _initialPausedStatus,
                _avs,
            })
        }
        ///Creates a new call builder for the [`isChurnApproverSaltUsed`] function.
        pub fn isChurnApproverSaltUsed(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isChurnApproverSaltUsedCall, N> {
            self.call_builder(&isChurnApproverSaltUsedCall { _0 })
        }
        ///Creates a new call builder for the [`lastEjectionTimestamp`] function.
        pub fn lastEjectionTimestamp(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastEjectionTimestampCall, N> {
            self.call_builder(&lastEjectionTimestampCall { _0 })
        }
        ///Creates a new call builder for the [`numRegistries`] function.
        pub fn numRegistries(&self) -> alloy_contract::SolCallBuilder<T, &P, numRegistriesCall, N> {
            self.call_builder(&numRegistriesCall {})
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
        ///Creates a new call builder for the [`pubkeyRegistrationMessageHash`] function.
        pub fn pubkeyRegistrationMessageHash(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, pubkeyRegistrationMessageHashCall, N> {
            self.call_builder(&pubkeyRegistrationMessageHashCall { operator })
        }
        ///Creates a new call builder for the [`quorumCount`] function.
        pub fn quorumCount(&self) -> alloy_contract::SolCallBuilder<T, &P, quorumCountCall, N> {
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
            avs: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(&registerOperatorCall {
                operator,
                avs,
                operatorSetIds,
                data,
            })
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
        ///Creates a new call builder for the [`setAvs`] function.
        pub fn setAvs(
            &self,
            _avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAvsCall, N> {
            self.call_builder(&setAvsCall { _avs })
        }
        ///Creates a new call builder for the [`setChurnApprover`] function.
        pub fn setChurnApprover(
            &self,
            _churnApprover: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setChurnApproverCall, N> {
            self.call_builder(&setChurnApproverCall { _churnApprover })
        }
        ///Creates a new call builder for the [`setEjectionCooldown`] function.
        pub fn setEjectionCooldown(
            &self,
            _ejectionCooldown: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setEjectionCooldownCall, N> {
            self.call_builder(&setEjectionCooldownCall { _ejectionCooldown })
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
            self.call_builder(&setOperatorSetParamsCall {
                quorumNumber,
                operatorSetParams,
            })
        }
        ///Creates a new call builder for the [`socketRegistry`] function.
        pub fn socketRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, socketRegistryCall, N> {
            self.call_builder(&socketRegistryCall {})
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`supportsAVS`] function.
        pub fn supportsAVS(
            &self,
            _avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsAVSCall, N> {
            self.call_builder(&supportsAVSCall { _avs })
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
            self.call_builder(&updateOperatorsForQuorumCall {
                operatorsPerQuorum,
                quorumNumbers,
            })
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
        > SlashingRegistryCoordinatorInstance<T, P, N>
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
        ///Creates a new event filter for the [`ChurnApproverUpdated`] event.
        pub fn ChurnApproverUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ChurnApproverUpdated, N> {
            self.event_filter::<ChurnApproverUpdated>()
        }
        ///Creates a new event filter for the [`EjectorUpdated`] event.
        pub fn EjectorUpdated_filter(&self) -> alloy_contract::Event<T, &P, EjectorUpdated, N> {
            self.event_filter::<EjectorUpdated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
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
