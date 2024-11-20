///Module containing a contract's types and functions.
/**

```solidity
library IOperatorSetManager {
    struct OperatorSet { address avs; bytes4 id; }
    struct SlashingMagnitudeParameters { address strategy; uint64 totalMagnitude; OperatorSet[] operatorSets; uint64[] slashableMagnitudes; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IOperatorSetManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct OperatorSet { address avs; bytes4 id; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSet {
        pub avs: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::FixedBytes<4>,
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
            alloy::sol_types::sol_data::FixedBytes<4>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<OperatorSet> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSet) -> Self {
                (value.avs, value.id)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { avs: tuple.0, id: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorSet {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorSet {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
        impl alloy_sol_types::SolType for OperatorSet {
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
        impl alloy_sol_types::SolStruct for OperatorSet {
            const NAME: &'static str = "OperatorSet";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorSet(address avs,bytes4 id)",
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
                            &self.avs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.id)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorSet {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.avs,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.id)
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
                    &rust.avs,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    4,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.id, out);
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
struct SlashingMagnitudeParameters { address strategy; uint64 totalMagnitude; OperatorSet[] operatorSets; uint64[] slashableMagnitudes; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingMagnitudeParameters {
        pub strategy: alloy::sol_types::private::Address,
        pub totalMagnitude: u64,
        pub operatorSets: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
        >,
        pub slashableMagnitudes: alloy::sol_types::private::Vec<u64>,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Array<OperatorSet>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            u64,
            alloy::sol_types::private::Vec<
                <OperatorSet as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<u64>,
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
        impl ::core::convert::From<SlashingMagnitudeParameters>
        for UnderlyingRustTuple<'_> {
            fn from(value: SlashingMagnitudeParameters) -> Self {
                (
                    value.strategy,
                    value.totalMagnitude,
                    value.operatorSets,
                    value.slashableMagnitudes,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SlashingMagnitudeParameters {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    totalMagnitude: tuple.1,
                    operatorSets: tuple.2,
                    slashableMagnitudes: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SlashingMagnitudeParameters {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for SlashingMagnitudeParameters {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalMagnitude),
                    <alloy::sol_types::sol_data::Array<
                        OperatorSet,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::SolType>::tokenize(&self.slashableMagnitudes),
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
        impl alloy_sol_types::SolType for SlashingMagnitudeParameters {
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
        impl alloy_sol_types::SolStruct for SlashingMagnitudeParameters {
            const NAME: &'static str = "SlashingMagnitudeParameters";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SlashingMagnitudeParameters(address strategy,uint64 totalMagnitude,OperatorSet[] operatorSets,uint64[] slashableMagnitudes)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <OperatorSet as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <OperatorSet as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalMagnitude,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        OperatorSet,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSets)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.slashableMagnitudes,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SlashingMagnitudeParameters {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalMagnitude,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        OperatorSet,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSets,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.slashableMagnitudes,
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalMagnitude,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    OperatorSet,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSets,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<64>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.slashableMagnitudes,
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
    /**Creates a new wrapper around an on-chain [`IOperatorSetManager`](self) contract instance.

See the [wrapper's documentation](`IOperatorSetManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IOperatorSetManagerInstance<T, P, N> {
        IOperatorSetManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IOperatorSetManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IOperatorSetManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IOperatorSetManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IOperatorSetManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IOperatorSetManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IOperatorSetManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IOperatorSetManager`](self) contract instance.

See the [wrapper's documentation](`IOperatorSetManagerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IOperatorSetManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IOperatorSetManagerInstance<T, P, N> {
            IOperatorSetManagerInstance {
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
    > IOperatorSetManagerInstance<T, P, N> {
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
    > IOperatorSetManagerInstance<T, P, N> {
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
    struct SignatureWithExpiry { bytes signature; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod ISignatureUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SignatureWithExpiry { bytes signature; uint256 expiry; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithExpiry {
        pub signature: alloy::sol_types::private::Bytes,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<SignatureWithExpiry> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithExpiry) -> Self {
                (value.signature, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureWithExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    expiry: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignatureWithExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
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
        impl alloy_sol_types::SolType for SignatureWithExpiry {
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
        impl alloy_sol_types::SolStruct for SignatureWithExpiry {
            const NAME: &'static str = "SignatureWithExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithExpiry(bytes signature,uint256 expiry)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
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
library IOperatorSetManager {
    struct OperatorSet {
        address avs;
        bytes4 id;
    }
    struct SlashingMagnitudeParameters {
        address strategy;
        uint64 totalMagnitude;
        OperatorSet[] operatorSets;
        uint64[] slashableMagnitudes;
    }
}

library ISignatureUtils {
    struct SignatureWithExpiry {
        bytes signature;
        uint256 expiry;
    }
}

interface OperatorSetManager {
    event SlashableMagnitudeUpdated(address operator, address strategy, IOperatorSetManager.OperatorSet operatorSet, uint64 slashableMagnitude, uint32 effectEpoch);
    event TotalMagnitudeUpdated(address operator, address strategy, uint64 totalMagnitude, uint32 effectEpoch);

    constructor(address _slasher);

    function getSlashableBips(address operator, IOperatorSetManager.OperatorSet memory operatorSet, address strategy, uint32 epoch) external view returns (uint16 slashableBips);
    function lockMagnitudeUpdatesAtEpoch(address operator, address strategy) external;
    function slasher() external view returns (address);
    function updateSlashingParameters(address operator, IOperatorSetManager.SlashingMagnitudeParameters[] memory slashingMagnitudeParameters, ISignatureUtils.SignatureWithExpiry memory allocatorSignature) external returns (uint32 effectEpoch);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_slasher",
        "type": "address",
        "internalType": "contract ISlasher"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getSlashableBips",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct IOperatorSetManager.OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "bytes4",
            "internalType": "bytes4"
          }
        ]
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "epoch",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "slashableBips",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lockMagnitudeUpdatesAtEpoch",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
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
        "internalType": "contract ISlasher"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "updateSlashingParameters",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "slashingMagnitudeParameters",
        "type": "tuple[]",
        "internalType": "struct IOperatorSetManager.SlashingMagnitudeParameters[]",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "totalMagnitude",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "operatorSets",
            "type": "tuple[]",
            "internalType": "struct IOperatorSetManager.OperatorSet[]",
            "components": [
              {
                "name": "avs",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "id",
                "type": "bytes4",
                "internalType": "bytes4"
              }
            ]
          },
          {
            "name": "slashableMagnitudes",
            "type": "uint64[]",
            "internalType": "uint64[]"
          }
        ]
      },
      {
        "name": "allocatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "effectEpoch",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "SlashableMagnitudeUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IOperatorSetManager.OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "bytes4",
            "internalType": "bytes4"
          }
        ]
      },
      {
        "name": "slashableMagnitude",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "effectEpoch",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TotalMagnitudeUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "totalMagnitude",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "effectEpoch",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
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
pub mod OperatorSetManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405234801561001057600080fd5b50604051611b52380380611b5283398101604081905261002f91610040565b6001600160a01b0316608052610070565b60006020828403121561005257600080fd5b81516001600160a01b038116811461006957600080fd5b9392505050565b608051611ac16100916000396000818160a901526111a10152611ac16000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80633f76c6c714610051578063516b98831461007c578063b1344271146100a4578063fa88909b146100e3575b600080fd5b61006461005f3660046115c0565b6100f8565b60405161ffff90911681526020015b60405180910390f35b61008f61008a366004611624565b6104fe565b60405163ffffffff9091168152602001610073565b6100cb7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610073565b6100f66100f13660046116d0565b61118a565b005b6000610102611441565b63ffffffff168263ffffffff1611156101745760405162461bcd60e51b815260206004820152602960248201527f45706f6368206973206d6f7265207468616e20322065706f63687320696e207460448201526868652066757475726560b81b60648201526084015b60405180910390fd5b6001600160a01b038086166000908152600260209081526040808320938716835292905220545b80156102d8576001600160a01b03808716600090815260026020908152604080832093881683529290522063ffffffff8416906101d960018461171f565b815481106101e9576101e9611736565b60009182526020909120015463ffffffff161180159061026857506001600160a01b03808716600090815260026020908152604080832093881683529290522063ffffffff84169061023c60018461171f565b8154811061024c5761024c611736565b600091825260209091200154600160201b900463ffffffff1610155b156102c6576001600160a01b03808716600090815260026020908152604080832093881683529290522061029d60018361171f565b815481106102ad576102ad611736565b60009182526020909120015463ffffffff1692506102d8565b806102d08161174c565b91505061019b565b506001600160a01b0380861660009081526020818152604080832093871683529290529081208161031661031136899003890189611780565b61145b565b81526020810191909152604001600090812080549092505b80156103bf5763ffffffff85168361034760018461171f565b8154811061035757610357611736565b60009182526020909120015463ffffffff16116103ad578261037a60018361171f565b8154811061038a5761038a611736565b600091825260209091200154600160201b90046001600160401b031691506103bf565b806103b78161174c565b91505061032e565b506001600160a01b0380881660009081526001602090815260408083209389168352929052908120545b80156104c6576001600160a01b03808a166000908152600160208181526040808420948c16845293905291902063ffffffff881691610428908461171f565b8154811061043857610438611736565b60009182526020909120015463ffffffff16116104b4576001600160a01b03808a166000908152600160208181526040808420948c16845293905291902090610481908361171f565b8154811061049157610491611736565b600091825260209091200154600160201b90046001600160401b031691506104c6565b806104be8161174c565b9150506103e9565b50806001600160c01b0316612710836001600160c01b03166104e891906117ea565b6104f29190611809565b98975050505050505050565b6000610508611441565b905060005b838110156111815784848281811061052757610527611736565b9050602002810190610539919061182b565b61054790606081019061184b565b905085858381811061055b5761055b611736565b905060200281019061056d919061182b565b61057b90604081019061189b565b90501461062b5760405162461bcd60e51b815260206004820152606c60248201527f4f70657261746f725365744d616e616765722e7570646174654f70657261746f60448201527f72536574536c617368696e67506172616d65746572733a206f70657261746f7260648201527f5365747320616e6420736c61736861626c654d61676e697475646573206c656e60848201526b0cee8d040dad2e6dac2e8c6d60a31b60a482015260c40161016b565b60408051606081018252600080825260208201819052918101919091526001600160a01b03871660009081526001602052604081208188888681811061067357610673611736565b9050602002810190610685919061182b565b6106939060208101906118e4565b6001600160a01b0316815260208101919091526040016000205490508015610780576001600160a01b0388166000908152600160205260408120908888868181106106e0576106e0611736565b90506020028101906106f2919061182b565b6107009060208101906118e4565b6001600160a01b03168152602081019190915260400160002061072460018361171f565b8154811061073457610734611736565b600091825260209182902060408051606081018252919092015463ffffffff811682526001600160401b03600160201b8204811694830194909452600160601b90049092169082015291505b60005b87878581811061079557610795611736565b90506020028101906107a7919061182b565b6107b590604081019061189b565b9050811015610d8a57600061081b8989878181106107d5576107d5611736565b90506020028101906107e7919061182b565b6107f590604081019061189b565b8481811061080557610805611736565b9050604002018036038101906103119190611780565b60408051808201909152600080825260208201529091506001600160a01b038b166000908152602081905260408120818c8c8a81811061085d5761085d611736565b905060200281019061086f919061182b565b61087d9060208101906118e4565b6001600160a01b031681526020808201929092526040908101600090812086825290925290205490508015610973576001600160a01b038c166000908152602081905260408120908c8c8a8181106108d7576108d7611736565b90506020028101906108e9919061182b565b6108f79060208101906118e4565b6001600160a01b0316815260208082019290925260409081016000908120868252909252902061092860018361171f565b8154811061093857610938611736565b60009182526020918290206040805180820190915291015463ffffffff81168252600160201b90046001600160401b03169181019190915291505b8160200151866040018181516109899190611908565b6001600160401b03169052508a8a888181106109a7576109a7611736565b90506020028101906109b9919061182b565b6109c790606081019061184b565b858181106109d7576109d7611736565b90506020020160208101906109ec9190611930565b866040018181516109fd9190611959565b6001600160401b0316905250815163ffffffff89811691161415610b34578a8a88818110610a2d57610a2d611736565b9050602002810190610a3f919061182b565b610a4d90606081019061184b565b85818110610a5d57610a5d611736565b9050602002016020810190610a729190611930565b6001600160a01b038d166000908152602081905260408120908d8d8b818110610a9d57610a9d611736565b9050602002810190610aaf919061182b565b610abd9060208101906118e4565b6001600160a01b03168152602080820192909252604090810160009081208782529092529020610aee60018461171f565b81548110610afe57610afe611736565b9060005260206000200160000160046101000a8154816001600160401b0302191690836001600160401b03160217905550610c67565b60405180604001604052808963ffffffff1681526020018c8c8a818110610b5d57610b5d611736565b9050602002810190610b6f919061182b565b610b7d90606081019061184b565b87818110610b8d57610b8d611736565b9050602002016020810190610ba29190611930565b6001600160401b031690526001600160a01b038d1660009081526020819052604081209193508c8c8a818110610bda57610bda611736565b9050602002810190610bec919061182b565b610bfa9060208101906118e4565b6001600160a01b03168152602080820192909252604090810160009081208682528352908120805460018101825590825290829020845191018054928501516001600160401b0316600160201b026001600160601b031990931663ffffffff909216919091179190911790555b7fa051327ef1123f482ec636fa78d997e135019b2bcfa0fab32a904626542995068c8c8c8a818110610c9b57610c9b611736565b9050602002810190610cad919061182b565b610cbb9060208101906118e4565b8d8d8b818110610ccd57610ccd611736565b9050602002810190610cdf919061182b565b610ced90604081019061189b565b88818110610cfd57610cfd611736565b9050604002018e8e8c818110610d1557610d15611736565b9050602002810190610d27919061182b565b610d3590606081019061184b565b89818110610d4557610d45611736565b9050602002016020810190610d5a9190611930565b8c604051610d6c959493929190611984565b60405180910390a15050508080610d82906119ef565b915050610783565b50868684818110610d9d57610d9d611736565b9050602002810190610daf919061182b565b610dc0906040810190602001611930565b6001600160401b031682604001516001600160401b03161115610e805760405162461bcd60e51b815260206004820152606660248201527f4f70657261746f725365744d616e616765722e7570646174654f70657261746f60448201527f72536574536c617368696e67506172616d65746572733a20746f74616c416c6c60648201527f6f63617465644d61676e6974756465206578636565647320746f74616c4d61676084820152656e697475646560d01b60a482015260c40161016b565b868684818110610e9257610e92611736565b9050602002810190610ea4919061182b565b610eb5906040810190602001611930565b6001600160401b0390811660208401526040830180519091169052815163ffffffff85811691161415610fcd576001600160a01b03881660009081526001602052604081208391898987818110610f0e57610f0e611736565b9050602002810190610f20919061182b565b610f2e9060208101906118e4565b6001600160a01b031681526020810191909152604001600020610f5260018461171f565b81548110610f6257610f62611736565b6000918252602091829020835191018054928401516040909401516001600160401b03908116600160601b0267ffffffffffffffff60601b1991909516600160201b026001600160601b031990941663ffffffff9093169290921792909217169190911790556110a5565b63ffffffff841682526001600160a01b03881660009081526001602052604081209088888681811061100157611001611736565b9050602002810190611013919061182b565b6110219060208101906118e4565b6001600160a01b031681526020808201929092526040908101600090812080546001810182559082529083902085519101805493860151928601516001600160401b03908116600160601b0267ffffffffffffffff60601b1991909416600160201b026001600160601b031990951663ffffffff9093169290921793909317161790555b7f802e045376358152b85ba2107735ff6f465df424b0fcbcb4690c83951d73ebd6888888868181106110d9576110d9611736565b90506020028101906110eb919061182b565b6110f99060208101906118e4565b89898781811061110b5761110b611736565b905060200281019061111d919061182b565b61112e906040810190602001611930565b604080516001600160a01b0394851681529390921660208401526001600160401b03169082015263ffffffff8616606082015260800160405180910390a150508080611179906119ef565b91505061050d565b50949350505050565b60006111946114ec565b9050336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146112425760405162461bcd60e51b815260206004820152604560248201527f4f70657261746f725365744d616e616765722e6c6f636b5374616b655570646160448201527f746573417445706f63683a2043616c6c6572206973206e6f742074686520736c60648201526430b9b432b960d91b608482015260a40161016b565b6001600160a01b0380841660009081526002602090815260408083209386168352929052205480156113c6576001600160a01b03808516600090815260026020908152604080832093871683529290522063ffffffff8316906112a660018461171f565b815481106112b6576112b6611736565b600091825260209091200154600160201b900463ffffffff1614156112db5750505050565b6001600160a01b03808516600090815260026020908152604080832093871683529290522063ffffffff83169061131360018461171f565b8154811061132357611323611736565b60009182526020909120015461134790600160201b900463ffffffff166001611a0a565b63ffffffff1614156113c6576001600160a01b038085166000908152600260209081526040808320938716835292905220829061138560018461171f565b8154811061139557611395611736565b9060005260206000200160000160046101000a81548163ffffffff021916908363ffffffff16021790555050505050565b506001600160a01b039283166000908152600260209081526040808320949095168252928352838120845180860190955263ffffffff92831680865285850190815281546001810183559183529390912093519301805492518216600160201b0267ffffffffffffffff199093169390911692909217179055565b600061144b6114ec565b611456906003611a0a565b905090565b604080516001808252818301909252600091829190816020015b604080518082019091526000808252602082015281526020019060019003908161147557905050905082816000815181106114b2576114b2611736565b6020026020010181905250806040516020016114ce9190611a29565b60405160208183030381529060405280519060200120915050919050565b6000611456426000635fc6304082101561156e5760405162461bcd60e51b815260206004820152603d60248201527f45706f63685574696c732e67657445706f636846726f6d54696d657374616d7060448201527f3a2074696d657374616d70206973206265666f72652067656e65736973000000606482015260840161016b565b62093a80611580635fc630408461171f565b61158a9190611809565b92915050565b6001600160a01b03811681146115a557600080fd5b50565b6000604082840312156115ba57600080fd5b50919050565b60008060008060a085870312156115d657600080fd5b84356115e181611590565b93506115f086602087016115a8565b9250606085013561160081611590565b9150608085013563ffffffff8116811461161957600080fd5b939692955090935050565b6000806000806060858703121561163a57600080fd5b843561164581611590565b935060208501356001600160401b038082111561166157600080fd5b818701915087601f83011261167557600080fd5b81358181111561168457600080fd5b8860208260051b850101111561169957600080fd5b6020830195508094505060408701359150808211156116b757600080fd5b506116c4878288016115a8565b91505092959194509250565b600080604083850312156116e357600080fd5b82356116ee81611590565b915060208301356116fe81611590565b809150509250929050565b634e487b7160e01b600052601160045260246000fd5b60008282101561173157611731611709565b500390565b634e487b7160e01b600052603260045260246000fd5b60008161175b5761175b611709565b506000190190565b80356001600160e01b03198116811461177b57600080fd5b919050565b60006040828403121561179257600080fd5b604051604081018181106001600160401b03821117156117c257634e487b7160e01b600052604160045260246000fd5b60405282356117d081611590565b81526117de60208401611763565b60208201529392505050565b600081600019048311821515161561180457611804611709565b500290565b60008261182657634e487b7160e01b600052601260045260246000fd5b500490565b60008235607e1983360301811261184157600080fd5b9190910192915050565b6000808335601e1984360301811261186257600080fd5b8301803591506001600160401b0382111561187c57600080fd5b6020019150600581901b360382131561189457600080fd5b9250929050565b6000808335601e198436030181126118b257600080fd5b8301803591506001600160401b038211156118cc57600080fd5b6020019150600681901b360382131561189457600080fd5b6000602082840312156118f657600080fd5b813561190181611590565b9392505050565b60006001600160401b038381169083168181101561192857611928611709565b039392505050565b60006020828403121561194257600080fd5b81356001600160401b038116811461190157600080fd5b60006001600160401b0380831681851680830382111561197b5761197b611709565b01949350505050565b6001600160a01b038681168252858116602083015260c082019085356119a981611590565b1660408301526001600160e01b03196119c460208701611763565b1660608301526001600160401b038416608083015263ffffffff831660a08301529695505050505050565b6000600019821415611a0357611a03611709565b5060010190565b600063ffffffff80831681851680830382111561197b5761197b611709565b602080825282518282018190526000919060409081850190868401855b82811015611a7e57815180516001600160a01b031685528601516001600160e01b031916868501529284019290850190600101611a46565b509197965050505050505056fea264697066735822122041982517f779c033cdd098e2b2ee7c4118aedfb87dc1ab1b765532e2696951fc64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x1BR8\x03\x80a\x1BR\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x1A\xC1a\0\x91`\09`\0\x81\x81`\xA9\x01Ra\x11\xA1\x01Ra\x1A\xC1`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c?v\xC6\xC7\x14a\0QW\x80cQk\x98\x83\x14a\0|W\x80c\xB14Bq\x14a\0\xA4W\x80c\xFA\x88\x90\x9B\x14a\0\xE3W[`\0\x80\xFD[a\0da\0_6`\x04a\x15\xC0V[a\0\xF8V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x8Fa\0\x8A6`\x04a\x16$V[a\x04\xFEV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0sV[a\0\xCB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0sV[a\0\xF6a\0\xF16`\x04a\x16\xD0V[a\x11\x8AV[\0[`\0a\x01\x02a\x14AV[c\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x11\x15a\x01tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FEpoch is more than 2 epochs in t`D\x82\x01Rhhe future`\xB8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R T[\x80\x15a\x02\xD8W`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R c\xFF\xFF\xFF\xFF\x84\x16\x90a\x01\xD9`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x01\xE9Wa\x01\xE9a\x176V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x80\x15\x90a\x02hWP`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R c\xFF\xFF\xFF\xFF\x84\x16\x90a\x02<`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x02LWa\x02La\x176V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x02\xC6W`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R a\x02\x9D`\x01\x83a\x17\x1FV[\x81T\x81\x10a\x02\xADWa\x02\xADa\x176V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x92Pa\x02\xD8V[\x80a\x02\xD0\x81a\x17LV[\x91PPa\x01\x9BV[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x81a\x03\x16a\x03\x116\x89\x90\x03\x89\x01\x89a\x17\x80V[a\x14[V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T\x90\x92P[\x80\x15a\x03\xBFWc\xFF\xFF\xFF\xFF\x85\x16\x83a\x03G`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x03WWa\x03Wa\x176V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x03\xADW\x82a\x03z`\x01\x83a\x17\x1FV[\x81T\x81\x10a\x03\x8AWa\x03\x8Aa\x176V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x91Pa\x03\xBFV[\x80a\x03\xB7\x81a\x17LV[\x91PPa\x03.V[P`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 T[\x80\x15a\x04\xC6W`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x94\x8C\x16\x84R\x93\x90R\x91\x90 c\xFF\xFF\xFF\xFF\x88\x16\x91a\x04(\x90\x84a\x17\x1FV[\x81T\x81\x10a\x048Wa\x048a\x176V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x04\xB4W`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x94\x8C\x16\x84R\x93\x90R\x91\x90 \x90a\x04\x81\x90\x83a\x17\x1FV[\x81T\x81\x10a\x04\x91Wa\x04\x91a\x176V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x91Pa\x04\xC6V[\x80a\x04\xBE\x81a\x17LV[\x91PPa\x03\xE9V[P\x80`\x01`\x01`\xC0\x1B\x03\x16a'\x10\x83`\x01`\x01`\xC0\x1B\x03\x16a\x04\xE8\x91\x90a\x17\xEAV[a\x04\xF2\x91\x90a\x18\tV[\x98\x97PPPPPPPPV[`\0a\x05\x08a\x14AV[\x90P`\0[\x83\x81\x10\x15a\x11\x81W\x84\x84\x82\x81\x81\x10a\x05'Wa\x05'a\x176V[\x90P` \x02\x81\x01\x90a\x059\x91\x90a\x18+V[a\x05G\x90``\x81\x01\x90a\x18KV[\x90P\x85\x85\x83\x81\x81\x10a\x05[Wa\x05[a\x176V[\x90P` \x02\x81\x01\x90a\x05m\x91\x90a\x18+V[a\x05{\x90`@\x81\x01\x90a\x18\x9BV[\x90P\x14a\x06+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FOperatorSetManager.updateOperato`D\x82\x01R\x7FrSetSlashingParameters: operator`d\x82\x01R\x7FSets and slashableMagnitudes len`\x84\x82\x01Rk\x0C\xEE\x8D\x04\r\xAD.m\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x01a\x01kV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x01` R`@\x81 \x81\x88\x88\x86\x81\x81\x10a\x06sWa\x06sa\x176V[\x90P` \x02\x81\x01\x90a\x06\x85\x91\x90a\x18+V[a\x06\x93\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T\x90P\x80\x15a\x07\x80W`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01` R`@\x81 \x90\x88\x88\x86\x81\x81\x10a\x06\xE0Wa\x06\xE0a\x176V[\x90P` \x02\x81\x01\x90a\x06\xF2\x91\x90a\x18+V[a\x07\0\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 a\x07$`\x01\x83a\x17\x1FV[\x81T\x81\x10a\x074Wa\x074a\x176V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x82R`\x01`\x01`@\x1B\x03`\x01` \x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01``\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x91P[`\0[\x87\x87\x85\x81\x81\x10a\x07\x95Wa\x07\x95a\x176V[\x90P` \x02\x81\x01\x90a\x07\xA7\x91\x90a\x18+V[a\x07\xB5\x90`@\x81\x01\x90a\x18\x9BV[\x90P\x81\x10\x15a\r\x8AW`\0a\x08\x1B\x89\x89\x87\x81\x81\x10a\x07\xD5Wa\x07\xD5a\x176V[\x90P` \x02\x81\x01\x90a\x07\xE7\x91\x90a\x18+V[a\x07\xF5\x90`@\x81\x01\x90a\x18\x9BV[\x84\x81\x81\x10a\x08\x05Wa\x08\x05a\x176V[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x03\x11\x91\x90a\x17\x80V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x8B\x16`\0\x90\x81R` \x81\x90R`@\x81 \x81\x8C\x8C\x8A\x81\x81\x10a\x08]Wa\x08]a\x176V[\x90P` \x02\x81\x01\x90a\x08o\x91\x90a\x18+V[a\x08}\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x86\x82R\x90\x92R\x90 T\x90P\x80\x15a\tsW`\x01`\x01`\xA0\x1B\x03\x8C\x16`\0\x90\x81R` \x81\x90R`@\x81 \x90\x8C\x8C\x8A\x81\x81\x10a\x08\xD7Wa\x08\xD7a\x176V[\x90P` \x02\x81\x01\x90a\x08\xE9\x91\x90a\x18+V[a\x08\xF7\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x86\x82R\x90\x92R\x90 a\t(`\x01\x83a\x17\x1FV[\x81T\x81\x10a\t8Wa\t8a\x176V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x82R`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x91P[\x81` \x01Q\x86`@\x01\x81\x81Qa\t\x89\x91\x90a\x19\x08V[`\x01`\x01`@\x1B\x03\x16\x90RP\x8A\x8A\x88\x81\x81\x10a\t\xA7Wa\t\xA7a\x176V[\x90P` \x02\x81\x01\x90a\t\xB9\x91\x90a\x18+V[a\t\xC7\x90``\x81\x01\x90a\x18KV[\x85\x81\x81\x10a\t\xD7Wa\t\xD7a\x176V[\x90P` \x02\x01` \x81\x01\x90a\t\xEC\x91\x90a\x190V[\x86`@\x01\x81\x81Qa\t\xFD\x91\x90a\x19YV[`\x01`\x01`@\x1B\x03\x16\x90RP\x81Qc\xFF\xFF\xFF\xFF\x89\x81\x16\x91\x16\x14\x15a\x0B4W\x8A\x8A\x88\x81\x81\x10a\n-Wa\n-a\x176V[\x90P` \x02\x81\x01\x90a\n?\x91\x90a\x18+V[a\nM\x90``\x81\x01\x90a\x18KV[\x85\x81\x81\x10a\n]Wa\n]a\x176V[\x90P` \x02\x01` \x81\x01\x90a\nr\x91\x90a\x190V[`\x01`\x01`\xA0\x1B\x03\x8D\x16`\0\x90\x81R` \x81\x90R`@\x81 \x90\x8D\x8D\x8B\x81\x81\x10a\n\x9DWa\n\x9Da\x176V[\x90P` \x02\x81\x01\x90a\n\xAF\x91\x90a\x18+V[a\n\xBD\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x87\x82R\x90\x92R\x90 a\n\xEE`\x01\x84a\x17\x1FV[\x81T\x81\x10a\n\xFEWa\n\xFEa\x176V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa\x0CgV[`@Q\x80`@\x01`@R\x80\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8C\x8C\x8A\x81\x81\x10a\x0B]Wa\x0B]a\x176V[\x90P` \x02\x81\x01\x90a\x0Bo\x91\x90a\x18+V[a\x0B}\x90``\x81\x01\x90a\x18KV[\x87\x81\x81\x10a\x0B\x8DWa\x0B\x8Da\x176V[\x90P` \x02\x01` \x81\x01\x90a\x0B\xA2\x91\x90a\x190V[`\x01`\x01`@\x1B\x03\x16\x90R`\x01`\x01`\xA0\x1B\x03\x8D\x16`\0\x90\x81R` \x81\x90R`@\x81 \x91\x93P\x8C\x8C\x8A\x81\x81\x10a\x0B\xDAWa\x0B\xDAa\x176V[\x90P` \x02\x81\x01\x90a\x0B\xEC\x91\x90a\x18+V[a\x0B\xFA\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x86\x82R\x83R\x90\x81 \x80T`\x01\x81\x01\x82U\x90\x82R\x90\x82\x90 \x84Q\x91\x01\x80T\x92\x85\x01Q`\x01`\x01`@\x1B\x03\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x93\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90U[\x7F\xA0Q2~\xF1\x12?H.\xC66\xFAx\xD9\x97\xE15\x01\x9B+\xCF\xA0\xFA\xB3*\x90F&T)\x95\x06\x8C\x8C\x8C\x8A\x81\x81\x10a\x0C\x9BWa\x0C\x9Ba\x176V[\x90P` \x02\x81\x01\x90a\x0C\xAD\x91\x90a\x18+V[a\x0C\xBB\x90` \x81\x01\x90a\x18\xE4V[\x8D\x8D\x8B\x81\x81\x10a\x0C\xCDWa\x0C\xCDa\x176V[\x90P` \x02\x81\x01\x90a\x0C\xDF\x91\x90a\x18+V[a\x0C\xED\x90`@\x81\x01\x90a\x18\x9BV[\x88\x81\x81\x10a\x0C\xFDWa\x0C\xFDa\x176V[\x90P`@\x02\x01\x8E\x8E\x8C\x81\x81\x10a\r\x15Wa\r\x15a\x176V[\x90P` \x02\x81\x01\x90a\r'\x91\x90a\x18+V[a\r5\x90``\x81\x01\x90a\x18KV[\x89\x81\x81\x10a\rEWa\rEa\x176V[\x90P` \x02\x01` \x81\x01\x90a\rZ\x91\x90a\x190V[\x8C`@Qa\rl\x95\x94\x93\x92\x91\x90a\x19\x84V[`@Q\x80\x91\x03\x90\xA1PPP\x80\x80a\r\x82\x90a\x19\xEFV[\x91PPa\x07\x83V[P\x86\x86\x84\x81\x81\x10a\r\x9DWa\r\x9Da\x176V[\x90P` \x02\x81\x01\x90a\r\xAF\x91\x90a\x18+V[a\r\xC0\x90`@\x81\x01\x90` \x01a\x190V[`\x01`\x01`@\x1B\x03\x16\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0E\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FOperatorSetManager.updateOperato`D\x82\x01R\x7FrSetSlashingParameters: totalAll`d\x82\x01R\x7FocatedMagnitude exceeds totalMag`\x84\x82\x01Renitude`\xD0\x1B`\xA4\x82\x01R`\xC4\x01a\x01kV[\x86\x86\x84\x81\x81\x10a\x0E\x92Wa\x0E\x92a\x176V[\x90P` \x02\x81\x01\x90a\x0E\xA4\x91\x90a\x18+V[a\x0E\xB5\x90`@\x81\x01\x90` \x01a\x190V[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x84\x01R`@\x83\x01\x80Q\x90\x91\x16\x90R\x81Qc\xFF\xFF\xFF\xFF\x85\x81\x16\x91\x16\x14\x15a\x0F\xCDW`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01` R`@\x81 \x83\x91\x89\x89\x87\x81\x81\x10a\x0F\x0EWa\x0F\x0Ea\x176V[\x90P` \x02\x81\x01\x90a\x0F \x91\x90a\x18+V[a\x0F.\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 a\x0FR`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x0FbWa\x0Fba\x176V[`\0\x91\x82R` \x91\x82\x90 \x83Q\x91\x01\x80T\x92\x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01``\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x1B\x19\x91\x90\x95\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x94\x16c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90Ua\x10\xA5V[c\xFF\xFF\xFF\xFF\x84\x16\x82R`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01` R`@\x81 \x90\x88\x88\x86\x81\x81\x10a\x10\x01Wa\x10\x01a\x176V[\x90P` \x02\x81\x01\x90a\x10\x13\x91\x90a\x18+V[a\x10!\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x80T`\x01\x81\x01\x82U\x90\x82R\x90\x83\x90 \x85Q\x91\x01\x80T\x93\x86\x01Q\x92\x86\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01``\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x1B\x19\x91\x90\x94\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x95\x16c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x93\x90\x93\x17\x16\x17\x90U[\x7F\x80.\x04Sv5\x81R\xB8[\xA2\x10w5\xFFoF]\xF4$\xB0\xFC\xBC\xB4i\x0C\x83\x95\x1Ds\xEB\xD6\x88\x88\x88\x86\x81\x81\x10a\x10\xD9Wa\x10\xD9a\x176V[\x90P` \x02\x81\x01\x90a\x10\xEB\x91\x90a\x18+V[a\x10\xF9\x90` \x81\x01\x90a\x18\xE4V[\x89\x89\x87\x81\x81\x10a\x11\x0BWa\x11\x0Ba\x176V[\x90P` \x02\x81\x01\x90a\x11\x1D\x91\x90a\x18+V[a\x11.\x90`@\x81\x01\x90` \x01a\x190V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x93\x90\x92\x16` \x84\x01R`\x01`\x01`@\x1B\x03\x16\x90\x82\x01Rc\xFF\xFF\xFF\xFF\x86\x16``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PP\x80\x80a\x11y\x90a\x19\xEFV[\x91PPa\x05\rV[P\x94\x93PPPPV[`\0a\x11\x94a\x14\xECV[\x90P3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x12BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FOperatorSetManager.lockStakeUpda`D\x82\x01R\x7FtesAtEpoch: Caller is not the sl`d\x82\x01Rd0\xB9\xB42\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\x01kV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T\x80\x15a\x13\xC6W`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R c\xFF\xFF\xFF\xFF\x83\x16\x90a\x12\xA6`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x12\xB6Wa\x12\xB6a\x176V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x14\x15a\x12\xDBWPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R c\xFF\xFF\xFF\xFF\x83\x16\x90a\x13\x13`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x13#Wa\x13#a\x176V[`\0\x91\x82R` \x90\x91 \x01Ta\x13G\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a\x1A\nV[c\xFF\xFF\xFF\xFF\x16\x14\x15a\x13\xC6W`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90a\x13\x85`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x13\x95Wa\x13\x95a\x176V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x94\x90\x95\x16\x82R\x92\x83R\x83\x81 \x84Q\x80\x86\x01\x90\x95Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x80\x86R\x85\x85\x01\x90\x81R\x81T`\x01\x81\x01\x83U\x91\x83R\x93\x90\x91 \x93Q\x93\x01\x80T\x92Q\x82\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x91\x16\x92\x90\x92\x17\x17\x90UV[`\0a\x14Ka\x14\xECV[a\x14V\x90`\x03a\x1A\nV[\x90P\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x82\x91\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x14uW\x90PP\x90P\x82\x81`\0\x81Q\x81\x10a\x14\xB2Wa\x14\xB2a\x176V[` \x02` \x01\x01\x81\x90RP\x80`@Q` \x01a\x14\xCE\x91\x90a\x1A)V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[`\0a\x14VB`\0c_\xC60@\x82\x10\x15a\x15nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FEpochUtils.getEpochFromTimestamp`D\x82\x01R\x7F: timestamp is before genesis\0\0\0`d\x82\x01R`\x84\x01a\x01kV[b\t:\x80a\x15\x80c_\xC60@\x84a\x17\x1FV[a\x15\x8A\x91\x90a\x18\tV[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\xA5W`\0\x80\xFD[PV[`\0`@\x82\x84\x03\x12\x15a\x15\xBAW`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0\x80`\xA0\x85\x87\x03\x12\x15a\x15\xD6W`\0\x80\xFD[\x845a\x15\xE1\x81a\x15\x90V[\x93Pa\x15\xF0\x86` \x87\x01a\x15\xA8V[\x92P``\x85\x015a\x16\0\x81a\x15\x90V[\x91P`\x80\x85\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x16\x19W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x16:W`\0\x80\xFD[\x845a\x16E\x81a\x15\x90V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16aW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x16uW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x16\x84W`\0\x80\xFD[\x88` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x16\x99W`\0\x80\xFD[` \x83\x01\x95P\x80\x94PP`@\x87\x015\x91P\x80\x82\x11\x15a\x16\xB7W`\0\x80\xFD[Pa\x16\xC4\x87\x82\x88\x01a\x15\xA8V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xE3W`\0\x80\xFD[\x825a\x16\xEE\x81a\x15\x90V[\x91P` \x83\x015a\x16\xFE\x81a\x15\x90V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x171Wa\x171a\x17\tV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x17[Wa\x17[a\x17\tV[P`\0\x19\x01\x90V[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x17{W`\0\x80\xFD[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x17\x92W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x17\xC2WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x825a\x17\xD0\x81a\x15\x90V[\x81Ra\x17\xDE` \x84\x01a\x17cV[` \x82\x01R\x93\x92PPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x18\x04Wa\x18\x04a\x17\tV[P\x02\x90V[`\0\x82a\x18&WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x825`~\x19\x836\x03\x01\x81\x12a\x18AW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18bW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x18|W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x18\x94W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18\xB2W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x18\xCCW`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x18\x94W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x18\xF6W`\0\x80\xFD[\x815a\x19\x01\x81a\x15\x90V[\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x19(Wa\x19(a\x17\tV[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x19BW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x19\x01W`\0\x80\xFD[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x19{Wa\x19{a\x17\tV[\x01\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x81\x16` \x83\x01R`\xC0\x82\x01\x90\x855a\x19\xA9\x81a\x15\x90V[\x16`@\x83\x01R`\x01`\x01`\xE0\x1B\x03\x19a\x19\xC4` \x87\x01a\x17cV[\x16``\x83\x01R`\x01`\x01`@\x1B\x03\x84\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x83\x16`\xA0\x83\x01R\x96\x95PPPPPPV[`\0`\0\x19\x82\x14\x15a\x1A\x03Wa\x1A\x03a\x17\tV[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x19{Wa\x19{a\x17\tV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x1A~W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x1AFV[P\x91\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 A\x98%\x17\xF7y\xC03\xCD\xD0\x98\xE2\xB2\xEE|A\x18\xAE\xDF\xB8}\xC1\xAB\x1BvU2\xE2iiQ\xFCdsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506004361061004c5760003560e01c80633f76c6c714610051578063516b98831461007c578063b1344271146100a4578063fa88909b146100e3575b600080fd5b61006461005f3660046115c0565b6100f8565b60405161ffff90911681526020015b60405180910390f35b61008f61008a366004611624565b6104fe565b60405163ffffffff9091168152602001610073565b6100cb7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610073565b6100f66100f13660046116d0565b61118a565b005b6000610102611441565b63ffffffff168263ffffffff1611156101745760405162461bcd60e51b815260206004820152602960248201527f45706f6368206973206d6f7265207468616e20322065706f63687320696e207460448201526868652066757475726560b81b60648201526084015b60405180910390fd5b6001600160a01b038086166000908152600260209081526040808320938716835292905220545b80156102d8576001600160a01b03808716600090815260026020908152604080832093881683529290522063ffffffff8416906101d960018461171f565b815481106101e9576101e9611736565b60009182526020909120015463ffffffff161180159061026857506001600160a01b03808716600090815260026020908152604080832093881683529290522063ffffffff84169061023c60018461171f565b8154811061024c5761024c611736565b600091825260209091200154600160201b900463ffffffff1610155b156102c6576001600160a01b03808716600090815260026020908152604080832093881683529290522061029d60018361171f565b815481106102ad576102ad611736565b60009182526020909120015463ffffffff1692506102d8565b806102d08161174c565b91505061019b565b506001600160a01b0380861660009081526020818152604080832093871683529290529081208161031661031136899003890189611780565b61145b565b81526020810191909152604001600090812080549092505b80156103bf5763ffffffff85168361034760018461171f565b8154811061035757610357611736565b60009182526020909120015463ffffffff16116103ad578261037a60018361171f565b8154811061038a5761038a611736565b600091825260209091200154600160201b90046001600160401b031691506103bf565b806103b78161174c565b91505061032e565b506001600160a01b0380881660009081526001602090815260408083209389168352929052908120545b80156104c6576001600160a01b03808a166000908152600160208181526040808420948c16845293905291902063ffffffff881691610428908461171f565b8154811061043857610438611736565b60009182526020909120015463ffffffff16116104b4576001600160a01b03808a166000908152600160208181526040808420948c16845293905291902090610481908361171f565b8154811061049157610491611736565b600091825260209091200154600160201b90046001600160401b031691506104c6565b806104be8161174c565b9150506103e9565b50806001600160c01b0316612710836001600160c01b03166104e891906117ea565b6104f29190611809565b98975050505050505050565b6000610508611441565b905060005b838110156111815784848281811061052757610527611736565b9050602002810190610539919061182b565b61054790606081019061184b565b905085858381811061055b5761055b611736565b905060200281019061056d919061182b565b61057b90604081019061189b565b90501461062b5760405162461bcd60e51b815260206004820152606c60248201527f4f70657261746f725365744d616e616765722e7570646174654f70657261746f60448201527f72536574536c617368696e67506172616d65746572733a206f70657261746f7260648201527f5365747320616e6420736c61736861626c654d61676e697475646573206c656e60848201526b0cee8d040dad2e6dac2e8c6d60a31b60a482015260c40161016b565b60408051606081018252600080825260208201819052918101919091526001600160a01b03871660009081526001602052604081208188888681811061067357610673611736565b9050602002810190610685919061182b565b6106939060208101906118e4565b6001600160a01b0316815260208101919091526040016000205490508015610780576001600160a01b0388166000908152600160205260408120908888868181106106e0576106e0611736565b90506020028101906106f2919061182b565b6107009060208101906118e4565b6001600160a01b03168152602081019190915260400160002061072460018361171f565b8154811061073457610734611736565b600091825260209182902060408051606081018252919092015463ffffffff811682526001600160401b03600160201b8204811694830194909452600160601b90049092169082015291505b60005b87878581811061079557610795611736565b90506020028101906107a7919061182b565b6107b590604081019061189b565b9050811015610d8a57600061081b8989878181106107d5576107d5611736565b90506020028101906107e7919061182b565b6107f590604081019061189b565b8481811061080557610805611736565b9050604002018036038101906103119190611780565b60408051808201909152600080825260208201529091506001600160a01b038b166000908152602081905260408120818c8c8a81811061085d5761085d611736565b905060200281019061086f919061182b565b61087d9060208101906118e4565b6001600160a01b031681526020808201929092526040908101600090812086825290925290205490508015610973576001600160a01b038c166000908152602081905260408120908c8c8a8181106108d7576108d7611736565b90506020028101906108e9919061182b565b6108f79060208101906118e4565b6001600160a01b0316815260208082019290925260409081016000908120868252909252902061092860018361171f565b8154811061093857610938611736565b60009182526020918290206040805180820190915291015463ffffffff81168252600160201b90046001600160401b03169181019190915291505b8160200151866040018181516109899190611908565b6001600160401b03169052508a8a888181106109a7576109a7611736565b90506020028101906109b9919061182b565b6109c790606081019061184b565b858181106109d7576109d7611736565b90506020020160208101906109ec9190611930565b866040018181516109fd9190611959565b6001600160401b0316905250815163ffffffff89811691161415610b34578a8a88818110610a2d57610a2d611736565b9050602002810190610a3f919061182b565b610a4d90606081019061184b565b85818110610a5d57610a5d611736565b9050602002016020810190610a729190611930565b6001600160a01b038d166000908152602081905260408120908d8d8b818110610a9d57610a9d611736565b9050602002810190610aaf919061182b565b610abd9060208101906118e4565b6001600160a01b03168152602080820192909252604090810160009081208782529092529020610aee60018461171f565b81548110610afe57610afe611736565b9060005260206000200160000160046101000a8154816001600160401b0302191690836001600160401b03160217905550610c67565b60405180604001604052808963ffffffff1681526020018c8c8a818110610b5d57610b5d611736565b9050602002810190610b6f919061182b565b610b7d90606081019061184b565b87818110610b8d57610b8d611736565b9050602002016020810190610ba29190611930565b6001600160401b031690526001600160a01b038d1660009081526020819052604081209193508c8c8a818110610bda57610bda611736565b9050602002810190610bec919061182b565b610bfa9060208101906118e4565b6001600160a01b03168152602080820192909252604090810160009081208682528352908120805460018101825590825290829020845191018054928501516001600160401b0316600160201b026001600160601b031990931663ffffffff909216919091179190911790555b7fa051327ef1123f482ec636fa78d997e135019b2bcfa0fab32a904626542995068c8c8c8a818110610c9b57610c9b611736565b9050602002810190610cad919061182b565b610cbb9060208101906118e4565b8d8d8b818110610ccd57610ccd611736565b9050602002810190610cdf919061182b565b610ced90604081019061189b565b88818110610cfd57610cfd611736565b9050604002018e8e8c818110610d1557610d15611736565b9050602002810190610d27919061182b565b610d3590606081019061184b565b89818110610d4557610d45611736565b9050602002016020810190610d5a9190611930565b8c604051610d6c959493929190611984565b60405180910390a15050508080610d82906119ef565b915050610783565b50868684818110610d9d57610d9d611736565b9050602002810190610daf919061182b565b610dc0906040810190602001611930565b6001600160401b031682604001516001600160401b03161115610e805760405162461bcd60e51b815260206004820152606660248201527f4f70657261746f725365744d616e616765722e7570646174654f70657261746f60448201527f72536574536c617368696e67506172616d65746572733a20746f74616c416c6c60648201527f6f63617465644d61676e6974756465206578636565647320746f74616c4d61676084820152656e697475646560d01b60a482015260c40161016b565b868684818110610e9257610e92611736565b9050602002810190610ea4919061182b565b610eb5906040810190602001611930565b6001600160401b0390811660208401526040830180519091169052815163ffffffff85811691161415610fcd576001600160a01b03881660009081526001602052604081208391898987818110610f0e57610f0e611736565b9050602002810190610f20919061182b565b610f2e9060208101906118e4565b6001600160a01b031681526020810191909152604001600020610f5260018461171f565b81548110610f6257610f62611736565b6000918252602091829020835191018054928401516040909401516001600160401b03908116600160601b0267ffffffffffffffff60601b1991909516600160201b026001600160601b031990941663ffffffff9093169290921792909217169190911790556110a5565b63ffffffff841682526001600160a01b03881660009081526001602052604081209088888681811061100157611001611736565b9050602002810190611013919061182b565b6110219060208101906118e4565b6001600160a01b031681526020808201929092526040908101600090812080546001810182559082529083902085519101805493860151928601516001600160401b03908116600160601b0267ffffffffffffffff60601b1991909416600160201b026001600160601b031990951663ffffffff9093169290921793909317161790555b7f802e045376358152b85ba2107735ff6f465df424b0fcbcb4690c83951d73ebd6888888868181106110d9576110d9611736565b90506020028101906110eb919061182b565b6110f99060208101906118e4565b89898781811061110b5761110b611736565b905060200281019061111d919061182b565b61112e906040810190602001611930565b604080516001600160a01b0394851681529390921660208401526001600160401b03169082015263ffffffff8616606082015260800160405180910390a150508080611179906119ef565b91505061050d565b50949350505050565b60006111946114ec565b9050336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146112425760405162461bcd60e51b815260206004820152604560248201527f4f70657261746f725365744d616e616765722e6c6f636b5374616b655570646160448201527f746573417445706f63683a2043616c6c6572206973206e6f742074686520736c60648201526430b9b432b960d91b608482015260a40161016b565b6001600160a01b0380841660009081526002602090815260408083209386168352929052205480156113c6576001600160a01b03808516600090815260026020908152604080832093871683529290522063ffffffff8316906112a660018461171f565b815481106112b6576112b6611736565b600091825260209091200154600160201b900463ffffffff1614156112db5750505050565b6001600160a01b03808516600090815260026020908152604080832093871683529290522063ffffffff83169061131360018461171f565b8154811061132357611323611736565b60009182526020909120015461134790600160201b900463ffffffff166001611a0a565b63ffffffff1614156113c6576001600160a01b038085166000908152600260209081526040808320938716835292905220829061138560018461171f565b8154811061139557611395611736565b9060005260206000200160000160046101000a81548163ffffffff021916908363ffffffff16021790555050505050565b506001600160a01b039283166000908152600260209081526040808320949095168252928352838120845180860190955263ffffffff92831680865285850190815281546001810183559183529390912093519301805492518216600160201b0267ffffffffffffffff199093169390911692909217179055565b600061144b6114ec565b611456906003611a0a565b905090565b604080516001808252818301909252600091829190816020015b604080518082019091526000808252602082015281526020019060019003908161147557905050905082816000815181106114b2576114b2611736565b6020026020010181905250806040516020016114ce9190611a29565b60405160208183030381529060405280519060200120915050919050565b6000611456426000635fc6304082101561156e5760405162461bcd60e51b815260206004820152603d60248201527f45706f63685574696c732e67657445706f636846726f6d54696d657374616d7060448201527f3a2074696d657374616d70206973206265666f72652067656e65736973000000606482015260840161016b565b62093a80611580635fc630408461171f565b61158a9190611809565b92915050565b6001600160a01b03811681146115a557600080fd5b50565b6000604082840312156115ba57600080fd5b50919050565b60008060008060a085870312156115d657600080fd5b84356115e181611590565b93506115f086602087016115a8565b9250606085013561160081611590565b9150608085013563ffffffff8116811461161957600080fd5b939692955090935050565b6000806000806060858703121561163a57600080fd5b843561164581611590565b935060208501356001600160401b038082111561166157600080fd5b818701915087601f83011261167557600080fd5b81358181111561168457600080fd5b8860208260051b850101111561169957600080fd5b6020830195508094505060408701359150808211156116b757600080fd5b506116c4878288016115a8565b91505092959194509250565b600080604083850312156116e357600080fd5b82356116ee81611590565b915060208301356116fe81611590565b809150509250929050565b634e487b7160e01b600052601160045260246000fd5b60008282101561173157611731611709565b500390565b634e487b7160e01b600052603260045260246000fd5b60008161175b5761175b611709565b506000190190565b80356001600160e01b03198116811461177b57600080fd5b919050565b60006040828403121561179257600080fd5b604051604081018181106001600160401b03821117156117c257634e487b7160e01b600052604160045260246000fd5b60405282356117d081611590565b81526117de60208401611763565b60208201529392505050565b600081600019048311821515161561180457611804611709565b500290565b60008261182657634e487b7160e01b600052601260045260246000fd5b500490565b60008235607e1983360301811261184157600080fd5b9190910192915050565b6000808335601e1984360301811261186257600080fd5b8301803591506001600160401b0382111561187c57600080fd5b6020019150600581901b360382131561189457600080fd5b9250929050565b6000808335601e198436030181126118b257600080fd5b8301803591506001600160401b038211156118cc57600080fd5b6020019150600681901b360382131561189457600080fd5b6000602082840312156118f657600080fd5b813561190181611590565b9392505050565b60006001600160401b038381169083168181101561192857611928611709565b039392505050565b60006020828403121561194257600080fd5b81356001600160401b038116811461190157600080fd5b60006001600160401b0380831681851680830382111561197b5761197b611709565b01949350505050565b6001600160a01b038681168252858116602083015260c082019085356119a981611590565b1660408301526001600160e01b03196119c460208701611763565b1660608301526001600160401b038416608083015263ffffffff831660a08301529695505050505050565b6000600019821415611a0357611a03611709565b5060010190565b600063ffffffff80831681851680830382111561197b5761197b611709565b602080825282518282018190526000919060409081850190868401855b82811015611a7e57815180516001600160a01b031685528601516001600160e01b031916868501529284019290850190600101611a46565b509197965050505050505056fea264697066735822122041982517f779c033cdd098e2b2ee7c4118aedfb87dc1ab1b765532e2696951fc64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c?v\xC6\xC7\x14a\0QW\x80cQk\x98\x83\x14a\0|W\x80c\xB14Bq\x14a\0\xA4W\x80c\xFA\x88\x90\x9B\x14a\0\xE3W[`\0\x80\xFD[a\0da\0_6`\x04a\x15\xC0V[a\0\xF8V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x8Fa\0\x8A6`\x04a\x16$V[a\x04\xFEV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0sV[a\0\xCB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0sV[a\0\xF6a\0\xF16`\x04a\x16\xD0V[a\x11\x8AV[\0[`\0a\x01\x02a\x14AV[c\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x11\x15a\x01tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FEpoch is more than 2 epochs in t`D\x82\x01Rhhe future`\xB8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R T[\x80\x15a\x02\xD8W`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R c\xFF\xFF\xFF\xFF\x84\x16\x90a\x01\xD9`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x01\xE9Wa\x01\xE9a\x176V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x80\x15\x90a\x02hWP`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R c\xFF\xFF\xFF\xFF\x84\x16\x90a\x02<`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x02LWa\x02La\x176V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x02\xC6W`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R a\x02\x9D`\x01\x83a\x17\x1FV[\x81T\x81\x10a\x02\xADWa\x02\xADa\x176V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x92Pa\x02\xD8V[\x80a\x02\xD0\x81a\x17LV[\x91PPa\x01\x9BV[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x81a\x03\x16a\x03\x116\x89\x90\x03\x89\x01\x89a\x17\x80V[a\x14[V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T\x90\x92P[\x80\x15a\x03\xBFWc\xFF\xFF\xFF\xFF\x85\x16\x83a\x03G`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x03WWa\x03Wa\x176V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x03\xADW\x82a\x03z`\x01\x83a\x17\x1FV[\x81T\x81\x10a\x03\x8AWa\x03\x8Aa\x176V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x91Pa\x03\xBFV[\x80a\x03\xB7\x81a\x17LV[\x91PPa\x03.V[P`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 T[\x80\x15a\x04\xC6W`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x94\x8C\x16\x84R\x93\x90R\x91\x90 c\xFF\xFF\xFF\xFF\x88\x16\x91a\x04(\x90\x84a\x17\x1FV[\x81T\x81\x10a\x048Wa\x048a\x176V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x04\xB4W`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x94\x8C\x16\x84R\x93\x90R\x91\x90 \x90a\x04\x81\x90\x83a\x17\x1FV[\x81T\x81\x10a\x04\x91Wa\x04\x91a\x176V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x91Pa\x04\xC6V[\x80a\x04\xBE\x81a\x17LV[\x91PPa\x03\xE9V[P\x80`\x01`\x01`\xC0\x1B\x03\x16a'\x10\x83`\x01`\x01`\xC0\x1B\x03\x16a\x04\xE8\x91\x90a\x17\xEAV[a\x04\xF2\x91\x90a\x18\tV[\x98\x97PPPPPPPPV[`\0a\x05\x08a\x14AV[\x90P`\0[\x83\x81\x10\x15a\x11\x81W\x84\x84\x82\x81\x81\x10a\x05'Wa\x05'a\x176V[\x90P` \x02\x81\x01\x90a\x059\x91\x90a\x18+V[a\x05G\x90``\x81\x01\x90a\x18KV[\x90P\x85\x85\x83\x81\x81\x10a\x05[Wa\x05[a\x176V[\x90P` \x02\x81\x01\x90a\x05m\x91\x90a\x18+V[a\x05{\x90`@\x81\x01\x90a\x18\x9BV[\x90P\x14a\x06+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FOperatorSetManager.updateOperato`D\x82\x01R\x7FrSetSlashingParameters: operator`d\x82\x01R\x7FSets and slashableMagnitudes len`\x84\x82\x01Rk\x0C\xEE\x8D\x04\r\xAD.m\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x01a\x01kV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x01` R`@\x81 \x81\x88\x88\x86\x81\x81\x10a\x06sWa\x06sa\x176V[\x90P` \x02\x81\x01\x90a\x06\x85\x91\x90a\x18+V[a\x06\x93\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T\x90P\x80\x15a\x07\x80W`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01` R`@\x81 \x90\x88\x88\x86\x81\x81\x10a\x06\xE0Wa\x06\xE0a\x176V[\x90P` \x02\x81\x01\x90a\x06\xF2\x91\x90a\x18+V[a\x07\0\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 a\x07$`\x01\x83a\x17\x1FV[\x81T\x81\x10a\x074Wa\x074a\x176V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x82R`\x01`\x01`@\x1B\x03`\x01` \x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01``\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x91P[`\0[\x87\x87\x85\x81\x81\x10a\x07\x95Wa\x07\x95a\x176V[\x90P` \x02\x81\x01\x90a\x07\xA7\x91\x90a\x18+V[a\x07\xB5\x90`@\x81\x01\x90a\x18\x9BV[\x90P\x81\x10\x15a\r\x8AW`\0a\x08\x1B\x89\x89\x87\x81\x81\x10a\x07\xD5Wa\x07\xD5a\x176V[\x90P` \x02\x81\x01\x90a\x07\xE7\x91\x90a\x18+V[a\x07\xF5\x90`@\x81\x01\x90a\x18\x9BV[\x84\x81\x81\x10a\x08\x05Wa\x08\x05a\x176V[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x03\x11\x91\x90a\x17\x80V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x8B\x16`\0\x90\x81R` \x81\x90R`@\x81 \x81\x8C\x8C\x8A\x81\x81\x10a\x08]Wa\x08]a\x176V[\x90P` \x02\x81\x01\x90a\x08o\x91\x90a\x18+V[a\x08}\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x86\x82R\x90\x92R\x90 T\x90P\x80\x15a\tsW`\x01`\x01`\xA0\x1B\x03\x8C\x16`\0\x90\x81R` \x81\x90R`@\x81 \x90\x8C\x8C\x8A\x81\x81\x10a\x08\xD7Wa\x08\xD7a\x176V[\x90P` \x02\x81\x01\x90a\x08\xE9\x91\x90a\x18+V[a\x08\xF7\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x86\x82R\x90\x92R\x90 a\t(`\x01\x83a\x17\x1FV[\x81T\x81\x10a\t8Wa\t8a\x176V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x82R`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x91P[\x81` \x01Q\x86`@\x01\x81\x81Qa\t\x89\x91\x90a\x19\x08V[`\x01`\x01`@\x1B\x03\x16\x90RP\x8A\x8A\x88\x81\x81\x10a\t\xA7Wa\t\xA7a\x176V[\x90P` \x02\x81\x01\x90a\t\xB9\x91\x90a\x18+V[a\t\xC7\x90``\x81\x01\x90a\x18KV[\x85\x81\x81\x10a\t\xD7Wa\t\xD7a\x176V[\x90P` \x02\x01` \x81\x01\x90a\t\xEC\x91\x90a\x190V[\x86`@\x01\x81\x81Qa\t\xFD\x91\x90a\x19YV[`\x01`\x01`@\x1B\x03\x16\x90RP\x81Qc\xFF\xFF\xFF\xFF\x89\x81\x16\x91\x16\x14\x15a\x0B4W\x8A\x8A\x88\x81\x81\x10a\n-Wa\n-a\x176V[\x90P` \x02\x81\x01\x90a\n?\x91\x90a\x18+V[a\nM\x90``\x81\x01\x90a\x18KV[\x85\x81\x81\x10a\n]Wa\n]a\x176V[\x90P` \x02\x01` \x81\x01\x90a\nr\x91\x90a\x190V[`\x01`\x01`\xA0\x1B\x03\x8D\x16`\0\x90\x81R` \x81\x90R`@\x81 \x90\x8D\x8D\x8B\x81\x81\x10a\n\x9DWa\n\x9Da\x176V[\x90P` \x02\x81\x01\x90a\n\xAF\x91\x90a\x18+V[a\n\xBD\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x87\x82R\x90\x92R\x90 a\n\xEE`\x01\x84a\x17\x1FV[\x81T\x81\x10a\n\xFEWa\n\xFEa\x176V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa\x0CgV[`@Q\x80`@\x01`@R\x80\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8C\x8C\x8A\x81\x81\x10a\x0B]Wa\x0B]a\x176V[\x90P` \x02\x81\x01\x90a\x0Bo\x91\x90a\x18+V[a\x0B}\x90``\x81\x01\x90a\x18KV[\x87\x81\x81\x10a\x0B\x8DWa\x0B\x8Da\x176V[\x90P` \x02\x01` \x81\x01\x90a\x0B\xA2\x91\x90a\x190V[`\x01`\x01`@\x1B\x03\x16\x90R`\x01`\x01`\xA0\x1B\x03\x8D\x16`\0\x90\x81R` \x81\x90R`@\x81 \x91\x93P\x8C\x8C\x8A\x81\x81\x10a\x0B\xDAWa\x0B\xDAa\x176V[\x90P` \x02\x81\x01\x90a\x0B\xEC\x91\x90a\x18+V[a\x0B\xFA\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x86\x82R\x83R\x90\x81 \x80T`\x01\x81\x01\x82U\x90\x82R\x90\x82\x90 \x84Q\x91\x01\x80T\x92\x85\x01Q`\x01`\x01`@\x1B\x03\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x93\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90U[\x7F\xA0Q2~\xF1\x12?H.\xC66\xFAx\xD9\x97\xE15\x01\x9B+\xCF\xA0\xFA\xB3*\x90F&T)\x95\x06\x8C\x8C\x8C\x8A\x81\x81\x10a\x0C\x9BWa\x0C\x9Ba\x176V[\x90P` \x02\x81\x01\x90a\x0C\xAD\x91\x90a\x18+V[a\x0C\xBB\x90` \x81\x01\x90a\x18\xE4V[\x8D\x8D\x8B\x81\x81\x10a\x0C\xCDWa\x0C\xCDa\x176V[\x90P` \x02\x81\x01\x90a\x0C\xDF\x91\x90a\x18+V[a\x0C\xED\x90`@\x81\x01\x90a\x18\x9BV[\x88\x81\x81\x10a\x0C\xFDWa\x0C\xFDa\x176V[\x90P`@\x02\x01\x8E\x8E\x8C\x81\x81\x10a\r\x15Wa\r\x15a\x176V[\x90P` \x02\x81\x01\x90a\r'\x91\x90a\x18+V[a\r5\x90``\x81\x01\x90a\x18KV[\x89\x81\x81\x10a\rEWa\rEa\x176V[\x90P` \x02\x01` \x81\x01\x90a\rZ\x91\x90a\x190V[\x8C`@Qa\rl\x95\x94\x93\x92\x91\x90a\x19\x84V[`@Q\x80\x91\x03\x90\xA1PPP\x80\x80a\r\x82\x90a\x19\xEFV[\x91PPa\x07\x83V[P\x86\x86\x84\x81\x81\x10a\r\x9DWa\r\x9Da\x176V[\x90P` \x02\x81\x01\x90a\r\xAF\x91\x90a\x18+V[a\r\xC0\x90`@\x81\x01\x90` \x01a\x190V[`\x01`\x01`@\x1B\x03\x16\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0E\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FOperatorSetManager.updateOperato`D\x82\x01R\x7FrSetSlashingParameters: totalAll`d\x82\x01R\x7FocatedMagnitude exceeds totalMag`\x84\x82\x01Renitude`\xD0\x1B`\xA4\x82\x01R`\xC4\x01a\x01kV[\x86\x86\x84\x81\x81\x10a\x0E\x92Wa\x0E\x92a\x176V[\x90P` \x02\x81\x01\x90a\x0E\xA4\x91\x90a\x18+V[a\x0E\xB5\x90`@\x81\x01\x90` \x01a\x190V[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x84\x01R`@\x83\x01\x80Q\x90\x91\x16\x90R\x81Qc\xFF\xFF\xFF\xFF\x85\x81\x16\x91\x16\x14\x15a\x0F\xCDW`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01` R`@\x81 \x83\x91\x89\x89\x87\x81\x81\x10a\x0F\x0EWa\x0F\x0Ea\x176V[\x90P` \x02\x81\x01\x90a\x0F \x91\x90a\x18+V[a\x0F.\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 a\x0FR`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x0FbWa\x0Fba\x176V[`\0\x91\x82R` \x91\x82\x90 \x83Q\x91\x01\x80T\x92\x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01``\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x1B\x19\x91\x90\x95\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x94\x16c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90Ua\x10\xA5V[c\xFF\xFF\xFF\xFF\x84\x16\x82R`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01` R`@\x81 \x90\x88\x88\x86\x81\x81\x10a\x10\x01Wa\x10\x01a\x176V[\x90P` \x02\x81\x01\x90a\x10\x13\x91\x90a\x18+V[a\x10!\x90` \x81\x01\x90a\x18\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x80T`\x01\x81\x01\x82U\x90\x82R\x90\x83\x90 \x85Q\x91\x01\x80T\x93\x86\x01Q\x92\x86\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01``\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x1B\x19\x91\x90\x94\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x95\x16c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x93\x90\x93\x17\x16\x17\x90U[\x7F\x80.\x04Sv5\x81R\xB8[\xA2\x10w5\xFFoF]\xF4$\xB0\xFC\xBC\xB4i\x0C\x83\x95\x1Ds\xEB\xD6\x88\x88\x88\x86\x81\x81\x10a\x10\xD9Wa\x10\xD9a\x176V[\x90P` \x02\x81\x01\x90a\x10\xEB\x91\x90a\x18+V[a\x10\xF9\x90` \x81\x01\x90a\x18\xE4V[\x89\x89\x87\x81\x81\x10a\x11\x0BWa\x11\x0Ba\x176V[\x90P` \x02\x81\x01\x90a\x11\x1D\x91\x90a\x18+V[a\x11.\x90`@\x81\x01\x90` \x01a\x190V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x93\x90\x92\x16` \x84\x01R`\x01`\x01`@\x1B\x03\x16\x90\x82\x01Rc\xFF\xFF\xFF\xFF\x86\x16``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1PP\x80\x80a\x11y\x90a\x19\xEFV[\x91PPa\x05\rV[P\x94\x93PPPPV[`\0a\x11\x94a\x14\xECV[\x90P3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x12BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FOperatorSetManager.lockStakeUpda`D\x82\x01R\x7FtesAtEpoch: Caller is not the sl`d\x82\x01Rd0\xB9\xB42\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\x01kV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T\x80\x15a\x13\xC6W`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R c\xFF\xFF\xFF\xFF\x83\x16\x90a\x12\xA6`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x12\xB6Wa\x12\xB6a\x176V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x14\x15a\x12\xDBWPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R c\xFF\xFF\xFF\xFF\x83\x16\x90a\x13\x13`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x13#Wa\x13#a\x176V[`\0\x91\x82R` \x90\x91 \x01Ta\x13G\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a\x1A\nV[c\xFF\xFF\xFF\xFF\x16\x14\x15a\x13\xC6W`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90a\x13\x85`\x01\x84a\x17\x1FV[\x81T\x81\x10a\x13\x95Wa\x13\x95a\x176V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x94\x90\x95\x16\x82R\x92\x83R\x83\x81 \x84Q\x80\x86\x01\x90\x95Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x80\x86R\x85\x85\x01\x90\x81R\x81T`\x01\x81\x01\x83U\x91\x83R\x93\x90\x91 \x93Q\x93\x01\x80T\x92Q\x82\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x91\x16\x92\x90\x92\x17\x17\x90UV[`\0a\x14Ka\x14\xECV[a\x14V\x90`\x03a\x1A\nV[\x90P\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x82\x91\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x14uW\x90PP\x90P\x82\x81`\0\x81Q\x81\x10a\x14\xB2Wa\x14\xB2a\x176V[` \x02` \x01\x01\x81\x90RP\x80`@Q` \x01a\x14\xCE\x91\x90a\x1A)V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[`\0a\x14VB`\0c_\xC60@\x82\x10\x15a\x15nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FEpochUtils.getEpochFromTimestamp`D\x82\x01R\x7F: timestamp is before genesis\0\0\0`d\x82\x01R`\x84\x01a\x01kV[b\t:\x80a\x15\x80c_\xC60@\x84a\x17\x1FV[a\x15\x8A\x91\x90a\x18\tV[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\xA5W`\0\x80\xFD[PV[`\0`@\x82\x84\x03\x12\x15a\x15\xBAW`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0\x80`\xA0\x85\x87\x03\x12\x15a\x15\xD6W`\0\x80\xFD[\x845a\x15\xE1\x81a\x15\x90V[\x93Pa\x15\xF0\x86` \x87\x01a\x15\xA8V[\x92P``\x85\x015a\x16\0\x81a\x15\x90V[\x91P`\x80\x85\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x16\x19W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x16:W`\0\x80\xFD[\x845a\x16E\x81a\x15\x90V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16aW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x16uW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x16\x84W`\0\x80\xFD[\x88` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x16\x99W`\0\x80\xFD[` \x83\x01\x95P\x80\x94PP`@\x87\x015\x91P\x80\x82\x11\x15a\x16\xB7W`\0\x80\xFD[Pa\x16\xC4\x87\x82\x88\x01a\x15\xA8V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xE3W`\0\x80\xFD[\x825a\x16\xEE\x81a\x15\x90V[\x91P` \x83\x015a\x16\xFE\x81a\x15\x90V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x171Wa\x171a\x17\tV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x17[Wa\x17[a\x17\tV[P`\0\x19\x01\x90V[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x17{W`\0\x80\xFD[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x17\x92W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x17\xC2WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x825a\x17\xD0\x81a\x15\x90V[\x81Ra\x17\xDE` \x84\x01a\x17cV[` \x82\x01R\x93\x92PPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x18\x04Wa\x18\x04a\x17\tV[P\x02\x90V[`\0\x82a\x18&WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x825`~\x19\x836\x03\x01\x81\x12a\x18AW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18bW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x18|W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x18\x94W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18\xB2W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x18\xCCW`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x18\x94W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x18\xF6W`\0\x80\xFD[\x815a\x19\x01\x81a\x15\x90V[\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x19(Wa\x19(a\x17\tV[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x19BW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x19\x01W`\0\x80\xFD[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x19{Wa\x19{a\x17\tV[\x01\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x81\x16` \x83\x01R`\xC0\x82\x01\x90\x855a\x19\xA9\x81a\x15\x90V[\x16`@\x83\x01R`\x01`\x01`\xE0\x1B\x03\x19a\x19\xC4` \x87\x01a\x17cV[\x16``\x83\x01R`\x01`\x01`@\x1B\x03\x84\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x83\x16`\xA0\x83\x01R\x96\x95PPPPPPV[`\0`\0\x19\x82\x14\x15a\x1A\x03Wa\x1A\x03a\x17\tV[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x19{Wa\x19{a\x17\tV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x1A~W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x1AFV[P\x91\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 A\x98%\x17\xF7y\xC03\xCD\xD0\x98\xE2\xB2\xEE|A\x18\xAE\xDF\xB8}\xC1\xAB\x1BvU2\xE2iiQ\xFCdsolcC\0\x08\x0C\x003",
    );
    /**Event with signature `SlashableMagnitudeUpdated(address,address,(address,bytes4),uint64,uint32)` and selector `0xa051327ef1123f482ec636fa78d997e135019b2bcfa0fab32a90462654299506`.
```solidity
event SlashableMagnitudeUpdated(address operator, address strategy, IOperatorSetManager.OperatorSet operatorSet, uint64 slashableMagnitude, uint32 effectEpoch);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashableMagnitudeUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub slashableMagnitude: u64,
        #[allow(missing_docs)]
        pub effectEpoch: u32,
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
        impl alloy_sol_types::SolEvent for SlashableMagnitudeUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                IOperatorSetManager::OperatorSet,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlashableMagnitudeUpdated(address,address,(address,bytes4),uint64,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                160u8,
                81u8,
                50u8,
                126u8,
                241u8,
                18u8,
                63u8,
                72u8,
                46u8,
                198u8,
                54u8,
                250u8,
                120u8,
                217u8,
                151u8,
                225u8,
                53u8,
                1u8,
                155u8,
                43u8,
                207u8,
                160u8,
                250u8,
                179u8,
                42u8,
                144u8,
                70u8,
                38u8,
                84u8,
                41u8,
                149u8,
                6u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: data.0,
                    strategy: data.1,
                    operatorSet: data.2,
                    slashableMagnitude: data.3,
                    effectEpoch: data.4,
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <IOperatorSetManager::OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.slashableMagnitude),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.effectEpoch),
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
        impl alloy_sol_types::private::IntoLogData for SlashableMagnitudeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashableMagnitudeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SlashableMagnitudeUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TotalMagnitudeUpdated(address,address,uint64,uint32)` and selector `0x802e045376358152b85ba2107735ff6f465df424b0fcbcb4690c83951d73ebd6`.
```solidity
event TotalMagnitudeUpdated(address operator, address strategy, uint64 totalMagnitude, uint32 effectEpoch);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TotalMagnitudeUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub totalMagnitude: u64,
        #[allow(missing_docs)]
        pub effectEpoch: u32,
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
        impl alloy_sol_types::SolEvent for TotalMagnitudeUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TotalMagnitudeUpdated(address,address,uint64,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                128u8,
                46u8,
                4u8,
                83u8,
                118u8,
                53u8,
                129u8,
                82u8,
                184u8,
                91u8,
                162u8,
                16u8,
                119u8,
                53u8,
                255u8,
                111u8,
                70u8,
                93u8,
                244u8,
                36u8,
                176u8,
                252u8,
                188u8,
                180u8,
                105u8,
                12u8,
                131u8,
                149u8,
                29u8,
                115u8,
                235u8,
                214u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: data.0,
                    strategy: data.1,
                    totalMagnitude: data.2,
                    effectEpoch: data.3,
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalMagnitude),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.effectEpoch),
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
        impl alloy_sol_types::private::IntoLogData for TotalMagnitudeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TotalMagnitudeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TotalMagnitudeUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _slasher);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _slasher: alloy::sol_types::private::Address,
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
                    (value._slasher,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _slasher: tuple.0 }
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
                        &self._slasher,
                    ),
                )
            }
        }
    };
    /**Function with signature `getSlashableBips(address,(address,bytes4),address,uint32)` and selector `0x3f76c6c7`.
```solidity
function getSlashableBips(address operator, IOperatorSetManager.OperatorSet memory operatorSet, address strategy, uint32 epoch) external view returns (uint16 slashableBips);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlashableBipsCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSet: <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
        pub strategy: alloy::sol_types::private::Address,
        pub epoch: u32,
    }
    ///Container type for the return parameters of the [`getSlashableBips(address,(address,bytes4),address,uint32)`](getSlashableBipsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlashableBipsReturn {
        pub slashableBips: u16,
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
                IOperatorSetManager::OperatorSet,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getSlashableBipsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSlashableBipsCall) -> Self {
                    (value.operator, value.operatorSet, value.strategy, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSlashableBipsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
                        strategy: tuple.2,
                        epoch: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u16,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getSlashableBipsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSlashableBipsReturn) -> Self {
                    (value.slashableBips,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSlashableBipsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slashableBips: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlashableBipsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                IOperatorSetManager::OperatorSet,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlashableBipsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlashableBips(address,(address,bytes4),address,uint32)";
            const SELECTOR: [u8; 4] = [63u8, 118u8, 198u8, 199u8];
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
                    <IOperatorSetManager::OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Function with signature `lockMagnitudeUpdatesAtEpoch(address,address)` and selector `0xfa88909b`.
```solidity
function lockMagnitudeUpdatesAtEpoch(address operator, address strategy) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lockMagnitudeUpdatesAtEpochCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`lockMagnitudeUpdatesAtEpoch(address,address)`](lockMagnitudeUpdatesAtEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lockMagnitudeUpdatesAtEpochReturn {}
    #[allow(
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<lockMagnitudeUpdatesAtEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lockMagnitudeUpdatesAtEpochCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lockMagnitudeUpdatesAtEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
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
            impl ::core::convert::From<lockMagnitudeUpdatesAtEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lockMagnitudeUpdatesAtEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lockMagnitudeUpdatesAtEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lockMagnitudeUpdatesAtEpochCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lockMagnitudeUpdatesAtEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lockMagnitudeUpdatesAtEpoch(address,address)";
            const SELECTOR: [u8; 4] = [250u8, 136u8, 144u8, 155u8];
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
                        &self.strategy,
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
    /**Function with signature `updateSlashingParameters(address,(address,uint64,(address,bytes4)[],uint64[])[],(bytes,uint256))` and selector `0x516b9883`.
```solidity
function updateSlashingParameters(address operator, IOperatorSetManager.SlashingMagnitudeParameters[] memory slashingMagnitudeParameters, ISignatureUtils.SignatureWithExpiry memory allocatorSignature) external returns (uint32 effectEpoch);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSlashingParametersCall {
        pub operator: alloy::sol_types::private::Address,
        pub slashingMagnitudeParameters: alloy::sol_types::private::Vec<
            <IOperatorSetManager::SlashingMagnitudeParameters as alloy::sol_types::SolType>::RustType,
        >,
        pub allocatorSignature: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`updateSlashingParameters(address,(address,uint64,(address,bytes4)[],uint64[])[],(bytes,uint256))`](updateSlashingParametersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSlashingParametersReturn {
        pub effectEpoch: u32,
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
                    IOperatorSetManager::SlashingMagnitudeParameters,
                >,
                ISignatureUtils::SignatureWithExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    <IOperatorSetManager::SlashingMagnitudeParameters as alloy::sol_types::SolType>::RustType,
                >,
                <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<updateSlashingParametersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateSlashingParametersCall) -> Self {
                    (
                        value.operator,
                        value.slashingMagnitudeParameters,
                        value.allocatorSignature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateSlashingParametersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        slashingMagnitudeParameters: tuple.1,
                        allocatorSignature: tuple.2,
                    }
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
            impl ::core::convert::From<updateSlashingParametersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateSlashingParametersReturn) -> Self {
                    (value.effectEpoch,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateSlashingParametersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { effectEpoch: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateSlashingParametersCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    IOperatorSetManager::SlashingMagnitudeParameters,
                >,
                ISignatureUtils::SignatureWithExpiry,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateSlashingParametersReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateSlashingParameters(address,(address,uint64,(address,bytes4)[],uint64[])[],(bytes,uint256))";
            const SELECTOR: [u8; 4] = [81u8, 107u8, 152u8, 131u8];
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
                        IOperatorSetManager::SlashingMagnitudeParameters,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.slashingMagnitudeParameters,
                    ),
                    <ISignatureUtils::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.allocatorSignature,
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
    ///Container for all the [`OperatorSetManager`](self) function calls.
    pub enum OperatorSetManagerCalls {
        getSlashableBips(getSlashableBipsCall),
        lockMagnitudeUpdatesAtEpoch(lockMagnitudeUpdatesAtEpochCall),
        slasher(slasherCall),
        updateSlashingParameters(updateSlashingParametersCall),
    }
    #[automatically_derived]
    impl OperatorSetManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [63u8, 118u8, 198u8, 199u8],
            [81u8, 107u8, 152u8, 131u8],
            [177u8, 52u8, 66u8, 113u8],
            [250u8, 136u8, 144u8, 155u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OperatorSetManagerCalls {
        const NAME: &'static str = "OperatorSetManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getSlashableBips(_) => {
                    <getSlashableBipsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lockMagnitudeUpdatesAtEpoch(_) => {
                    <lockMagnitudeUpdatesAtEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slasher(_) => <slasherCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateSlashingParameters(_) => {
                    <updateSlashingParametersCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<OperatorSetManagerCalls>] = &[
                {
                    fn getSlashableBips(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorSetManagerCalls> {
                        <getSlashableBipsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorSetManagerCalls::getSlashableBips)
                    }
                    getSlashableBips
                },
                {
                    fn updateSlashingParameters(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorSetManagerCalls> {
                        <updateSlashingParametersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorSetManagerCalls::updateSlashingParameters)
                    }
                    updateSlashingParameters
                },
                {
                    fn slasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorSetManagerCalls> {
                        <slasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorSetManagerCalls::slasher)
                    }
                    slasher
                },
                {
                    fn lockMagnitudeUpdatesAtEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorSetManagerCalls> {
                        <lockMagnitudeUpdatesAtEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorSetManagerCalls::lockMagnitudeUpdatesAtEpoch)
                    }
                    lockMagnitudeUpdatesAtEpoch
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
                Self::getSlashableBips(inner) => {
                    <getSlashableBipsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lockMagnitudeUpdatesAtEpoch(inner) => {
                    <lockMagnitudeUpdatesAtEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateSlashingParameters(inner) => {
                    <updateSlashingParametersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::getSlashableBips(inner) => {
                    <getSlashableBipsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lockMagnitudeUpdatesAtEpoch(inner) => {
                    <lockMagnitudeUpdatesAtEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateSlashingParameters(inner) => {
                    <updateSlashingParametersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`OperatorSetManager`](self) events.
    pub enum OperatorSetManagerEvents {
        SlashableMagnitudeUpdated(SlashableMagnitudeUpdated),
        TotalMagnitudeUpdated(TotalMagnitudeUpdated),
    }
    #[automatically_derived]
    impl OperatorSetManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                128u8,
                46u8,
                4u8,
                83u8,
                118u8,
                53u8,
                129u8,
                82u8,
                184u8,
                91u8,
                162u8,
                16u8,
                119u8,
                53u8,
                255u8,
                111u8,
                70u8,
                93u8,
                244u8,
                36u8,
                176u8,
                252u8,
                188u8,
                180u8,
                105u8,
                12u8,
                131u8,
                149u8,
                29u8,
                115u8,
                235u8,
                214u8,
            ],
            [
                160u8,
                81u8,
                50u8,
                126u8,
                241u8,
                18u8,
                63u8,
                72u8,
                46u8,
                198u8,
                54u8,
                250u8,
                120u8,
                217u8,
                151u8,
                225u8,
                53u8,
                1u8,
                155u8,
                43u8,
                207u8,
                160u8,
                250u8,
                179u8,
                42u8,
                144u8,
                70u8,
                38u8,
                84u8,
                41u8,
                149u8,
                6u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for OperatorSetManagerEvents {
        const NAME: &'static str = "OperatorSetManagerEvents";
        const COUNT: usize = 2usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <SlashableMagnitudeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SlashableMagnitudeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashableMagnitudeUpdated)
                }
                Some(
                    <TotalMagnitudeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TotalMagnitudeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TotalMagnitudeUpdated)
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
    impl alloy_sol_types::private::IntoLogData for OperatorSetManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::SlashableMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TotalMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::SlashableMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TotalMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OperatorSetManager`](self) contract instance.

See the [wrapper's documentation](`OperatorSetManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OperatorSetManagerInstance<T, P, N> {
        OperatorSetManagerInstance::<T, P, N>::new(address, provider)
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
        _slasher: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<OperatorSetManagerInstance<T, P, N>>,
    > {
        OperatorSetManagerInstance::<T, P, N>::deploy(provider, _slasher)
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
        _slasher: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        OperatorSetManagerInstance::<T, P, N>::deploy_builder(provider, _slasher)
    }
    /**A [`OperatorSetManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OperatorSetManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OperatorSetManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OperatorSetManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OperatorSetManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OperatorSetManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`OperatorSetManager`](self) contract instance.

See the [wrapper's documentation](`OperatorSetManagerInstance`) for more details.*/
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
            _slasher: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<OperatorSetManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _slasher);
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
            _slasher: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { _slasher },
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
    impl<T, P: ::core::clone::Clone, N> OperatorSetManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OperatorSetManagerInstance<T, P, N> {
            OperatorSetManagerInstance {
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
    > OperatorSetManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`getSlashableBips`] function.
        pub fn getSlashableBips(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
            strategy: alloy::sol_types::private::Address,
            epoch: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlashableBipsCall, N> {
            self.call_builder(
                &getSlashableBipsCall {
                    operator,
                    operatorSet,
                    strategy,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`lockMagnitudeUpdatesAtEpoch`] function.
        pub fn lockMagnitudeUpdatesAtEpoch(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, lockMagnitudeUpdatesAtEpochCall, N> {
            self.call_builder(
                &lockMagnitudeUpdatesAtEpochCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`slasher`] function.
        pub fn slasher(&self) -> alloy_contract::SolCallBuilder<T, &P, slasherCall, N> {
            self.call_builder(&slasherCall {})
        }
        ///Creates a new call builder for the [`updateSlashingParameters`] function.
        pub fn updateSlashingParameters(
            &self,
            operator: alloy::sol_types::private::Address,
            slashingMagnitudeParameters: alloy::sol_types::private::Vec<
                <IOperatorSetManager::SlashingMagnitudeParameters as alloy::sol_types::SolType>::RustType,
            >,
            allocatorSignature: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateSlashingParametersCall, N> {
            self.call_builder(
                &updateSlashingParametersCall {
                    operator,
                    slashingMagnitudeParameters,
                    allocatorSignature,
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
    > OperatorSetManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`SlashableMagnitudeUpdated`] event.
        pub fn SlashableMagnitudeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashableMagnitudeUpdated, N> {
            self.event_filter::<SlashableMagnitudeUpdated>()
        }
        ///Creates a new event filter for the [`TotalMagnitudeUpdated`] event.
        pub fn TotalMagnitudeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TotalMagnitudeUpdated, N> {
            self.event_filter::<TotalMagnitudeUpdated>()
        }
    }
}
