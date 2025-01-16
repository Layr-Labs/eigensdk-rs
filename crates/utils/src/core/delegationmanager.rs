///Module containing a contract's types and functions.
/**

```solidity
library IDelegationManagerTypes {
    struct QueuedWithdrawalParams { address[] strategies; uint256[] depositShares; address withdrawer; }
    struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startBlock; address[] strategies; uint256[] scaledShares; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IDelegationManagerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct QueuedWithdrawalParams { address[] strategies; uint256[] depositShares; address withdrawer; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QueuedWithdrawalParams {
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub depositShares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub withdrawer: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
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
        impl ::core::convert::From<QueuedWithdrawalParams> for UnderlyingRustTuple<'_> {
            fn from(value: QueuedWithdrawalParams) -> Self {
                (value.strategies, value.depositShares, value.withdrawer)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QueuedWithdrawalParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategies: tuple.0,
                    depositShares: tuple.1,
                    withdrawer: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QueuedWithdrawalParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QueuedWithdrawalParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.depositShares),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawer,
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
        impl alloy_sol_types::SolType for QueuedWithdrawalParams {
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
        impl alloy_sol_types::SolStruct for QueuedWithdrawalParams {
            const NAME: &'static str = "QueuedWithdrawalParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QueuedWithdrawalParams(address[] strategies,uint256[] depositShares,address withdrawer)",
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.depositShares)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.withdrawer,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QueuedWithdrawalParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.depositShares,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawer,
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
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.depositShares,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawer,
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
struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startBlock; address[] strategies; uint256[] scaledShares; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Withdrawal {
        pub staker: alloy::sol_types::private::Address,
        pub delegatedTo: alloy::sol_types::private::Address,
        pub withdrawer: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub startBlock: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub scaledShares: alloy::sol_types::private::Vec<
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl ::core::convert::From<Withdrawal> for UnderlyingRustTuple<'_> {
            fn from(value: Withdrawal) -> Self {
                (
                    value.staker,
                    value.delegatedTo,
                    value.withdrawer,
                    value.nonce,
                    value.startBlock,
                    value.strategies,
                    value.scaledShares,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Withdrawal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    staker: tuple.0,
                    delegatedTo: tuple.1,
                    withdrawer: tuple.2,
                    nonce: tuple.3,
                    startBlock: tuple.4,
                    strategies: tuple.5,
                    scaledShares: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Withdrawal {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Withdrawal {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatedTo,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startBlock),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.scaledShares),
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
        impl alloy_sol_types::SolType for Withdrawal {
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
        impl alloy_sol_types::SolStruct for Withdrawal {
            const NAME: &'static str = "Withdrawal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Withdrawal(address staker,address delegatedTo,address withdrawer,uint256 nonce,uint32 startBlock,address[] strategies,uint256[] scaledShares)",
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
                            &self.staker,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegatedTo,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.withdrawer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.startBlock)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.scaledShares)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Withdrawal {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.staker,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegatedTo,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawer,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startBlock,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.scaledShares,
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
                    &rust.staker,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegatedTo,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawer,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.scaledShares,
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
    /**Creates a new wrapper around an on-chain [`IDelegationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IDelegationManagerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IDelegationManagerTypesInstance<T, P, N> {
        IDelegationManagerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IDelegationManagerTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IDelegationManagerTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IDelegationManagerTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IDelegationManagerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IDelegationManagerTypesInstance")
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
    > IDelegationManagerTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IDelegationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IDelegationManagerTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IDelegationManagerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IDelegationManagerTypesInstance<T, P, N> {
            IDelegationManagerTypesInstance {
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
    > IDelegationManagerTypesInstance<T, P, N> {
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
    > IDelegationManagerTypesInstance<T, P, N> {
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
    clippy::style,
    clippy::empty_structs_with_brackets
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
library IDelegationManagerTypes {
    struct QueuedWithdrawalParams {
        address[] strategies;
        uint256[] depositShares;
        address withdrawer;
    }
    struct Withdrawal {
        address staker;
        address delegatedTo;
        address withdrawer;
        uint256 nonce;
        uint32 startBlock;
        address[] strategies;
        uint256[] scaledShares;
    }
}

library ISignatureUtils {
    struct SignatureWithExpiry {
        bytes signature;
        uint256 expiry;
    }
}

interface DelegationManager {
    error ActivelyDelegated();
    error CallerCannotUndelegate();
    error CurrentlyPaused();
    error FullySlashed();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InputArrayLengthZero();
    error InvalidNewPausedStatus();
    error InvalidPermissions();
    error InvalidSignature();
    error InvalidSnapshotOrdering();
    error NotActivelyDelegated();
    error OnlyAllocationManager();
    error OnlyEigenPodManager();
    error OnlyPauser();
    error OnlyStrategyManagerOrEigenPodManager();
    error OnlyUnpauser();
    error OperatorNotRegistered();
    error OperatorsCannotUndelegate();
    error SaltSpent();
    error SignatureExpired();
    error WithdrawalDelayNotElapsed();
    error WithdrawalExceedsMax();
    error WithdrawalNotQueued();
    error WithdrawerNotCaller();
    error WithdrawerNotStaker();

    event DelegationApproverUpdated(address indexed operator, address newDelegationApprover);
    event DepositScalingFactorUpdated(address staker, address strategy, uint256 newDepositScalingFactor);
    event Initialized(uint8 version);
    event OperatorMetadataURIUpdated(address indexed operator, string metadataURI);
    event OperatorRegistered(address indexed operator, address delegationApprover);
    event OperatorSharesBurned(address indexed operator, address strategy, uint256 shares);
    event OperatorSharesDecreased(address indexed operator, address staker, address strategy, uint256 shares);
    event OperatorSharesIncreased(address indexed operator, address staker, address strategy, uint256 shares);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event SlashingWithdrawalCompleted(bytes32 withdrawalRoot);
    event SlashingWithdrawalQueued(bytes32 withdrawalRoot, IDelegationManagerTypes.Withdrawal withdrawal, uint256[] sharesToWithdraw);
    event StakerDelegated(address indexed staker, address indexed operator);
    event StakerForceUndelegated(address indexed staker, address indexed operator);
    event StakerUndelegated(address indexed staker, address indexed operator);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _strategyManager, address _eigenPodManager, address _allocationManager, address _pauserRegistry, address _permissionController, uint32 _MIN_WITHDRAWAL_DELAY);

    function DELEGATION_APPROVAL_TYPEHASH() external view returns (bytes32);
    function allocationManager() external view returns (address);
    function beaconChainETHStrategy() external view returns (address);
    function burnOperatorShares(address operator, address strategy, uint64 prevMaxMagnitude, uint64 newMaxMagnitude) external;
    function calculateDelegationApprovalDigestHash(address staker, address operator, address approver, bytes32 approverSalt, uint256 expiry) external view returns (bytes32);
    function calculateWithdrawalRoot(IDelegationManagerTypes.Withdrawal memory withdrawal) external pure returns (bytes32);
    function completeQueuedWithdrawal(IDelegationManagerTypes.Withdrawal memory withdrawal, address[] memory tokens, bool receiveAsTokens) external;
    function completeQueuedWithdrawals(address[][] memory tokens, bool[] memory receiveAsTokens, uint256 numToComplete) external;
    function completeQueuedWithdrawals(IDelegationManagerTypes.Withdrawal[] memory withdrawals, address[][] memory tokens, bool[] memory receiveAsTokens) external;
    function cumulativeWithdrawalsQueued(address staker) external view returns (uint256 totalQueued);
    function decreaseDelegatedShares(address staker, uint256 curDepositShares, uint64 beaconChainSlashingFactorDecrease) external;
    function delegateTo(address operator, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    function delegatedTo(address staker) external view returns (address operator);
    function delegationApprover(address operator) external view returns (address);
    function delegationApproverSaltIsSpent(address delegationApprover, bytes32 salt) external view returns (bool spent);
    function depositScalingFactor(address staker, address strategy) external view returns (uint256);
    function domainSeparator() external view returns (bytes32);
    function eigenPodManager() external view returns (address);
    function getDepositedShares(address staker) external view returns (address[] memory, uint256[] memory);
    function getOperatorShares(address operator, address[] memory strategies) external view returns (uint256[] memory);
    function getOperatorsShares(address[] memory operators, address[] memory strategies) external view returns (uint256[][] memory);
    function getQueuedWithdrawals(address staker) external view returns (IDelegationManagerTypes.Withdrawal[] memory withdrawals, uint256[][] memory shares);
    function getSlashableSharesInQueue(address operator, address strategy) external view returns (uint256);
    function getWithdrawableShares(address staker, address[] memory strategies) external view returns (uint256[] memory withdrawableShares, uint256[] memory depositShares);
    function increaseDelegatedShares(address staker, address strategy, uint256 prevDepositShares, uint256 addedShares) external;
    function initialize(address initialOwner, uint256 initialPausedStatus) external;
    function isDelegated(address staker) external view returns (bool);
    function isOperator(address operator) external view returns (bool);
    function minWithdrawalDelayBlocks() external view returns (uint32);
    function modifyOperatorDetails(address operator, address newDelegationApprover) external;
    function operatorShares(address operator, address strategy) external view returns (uint256 shares);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function pendingWithdrawals(bytes32 withdrawalRoot) external view returns (bool pending);
    function permissionController() external view returns (address);
    function queueWithdrawals(IDelegationManagerTypes.QueuedWithdrawalParams[] memory params) external returns (bytes32[] memory);
    function queuedWithdrawals(bytes32 withdrawalRoot) external view returns (address staker, address delegatedTo, address withdrawer, uint256 nonce, uint32 startBlock);
    function redelegate(address newOperator, ISignatureUtils.SignatureWithExpiry memory newOperatorApproverSig, bytes32 approverSalt) external returns (bytes32[] memory withdrawalRoots);
    function registerAsOperator(address initDelegationApprover, uint32 allocationDelay, string memory metadataURI) external;
    function renounceOwnership() external;
    function strategyManager() external view returns (address);
    function transferOwnership(address newOwner) external;
    function undelegate(address staker) external returns (bytes32[] memory withdrawalRoots);
    function unpause(uint256 newPausedStatus) external;
    function updateOperatorMetadataURI(address operator, string memory metadataURI) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_strategyManager",
        "type": "address",
        "internalType": "contract IStrategyManager"
      },
      {
        "name": "_eigenPodManager",
        "type": "address",
        "internalType": "contract IEigenPodManager"
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
      },
      {
        "name": "_permissionController",
        "type": "address",
        "internalType": "contract IPermissionController"
      },
      {
        "name": "_MIN_WITHDRAWAL_DELAY",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "DELEGATION_APPROVAL_TYPEHASH",
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
    "name": "beaconChainETHStrategy",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "burnOperatorShares",
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
      },
      {
        "name": "prevMaxMagnitude",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "newMaxMagnitude",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "calculateDelegationApprovalDigestHash",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approver",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approverSalt",
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
    "name": "calculateWithdrawalRoot",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManagerTypes.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "completeQueuedWithdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManagerTypes.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "tokens",
        "type": "address[]",
        "internalType": "contract IERC20[]"
      },
      {
        "name": "receiveAsTokens",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "completeQueuedWithdrawals",
    "inputs": [
      {
        "name": "tokens",
        "type": "address[][]",
        "internalType": "contract IERC20[][]"
      },
      {
        "name": "receiveAsTokens",
        "type": "bool[]",
        "internalType": "bool[]"
      },
      {
        "name": "numToComplete",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "completeQueuedWithdrawals",
    "inputs": [
      {
        "name": "withdrawals",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.Withdrawal[]",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "tokens",
        "type": "address[][]",
        "internalType": "contract IERC20[][]"
      },
      {
        "name": "receiveAsTokens",
        "type": "bool[]",
        "internalType": "bool[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "cumulativeWithdrawalsQueued",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "totalQueued",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "decreaseDelegatedShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "curDepositShares",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "beaconChainSlashingFactorDecrease",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateTo",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approverSignatureAndExpiry",
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
      },
      {
        "name": "approverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegatedTo",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegationApprover",
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
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegationApproverSaltIsSpent",
    "inputs": [
      {
        "name": "delegationApprover",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "spent",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "depositScalingFactor",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
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
    "name": "domainSeparator",
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
    "name": "eigenPodManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IEigenPodManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDepositedShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
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
    "name": "getOperatorShares",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
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
    "name": "getOperatorsShares",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[][]",
        "internalType": "uint256[][]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQueuedWithdrawals",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "withdrawals",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.Withdrawal[]",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "shares",
        "type": "uint256[][]",
        "internalType": "uint256[][]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getSlashableSharesInQueue",
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
    "name": "getWithdrawableShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "outputs": [
      {
        "name": "withdrawableShares",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "depositShares",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "increaseDelegatedShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "prevDepositShares",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "addedShares",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
        "name": "initialPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isDelegated",
    "inputs": [
      {
        "name": "staker",
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
    "name": "isOperator",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "minWithdrawalDelayBlocks",
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
    "name": "modifyOperatorDetails",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "newDelegationApprover",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "operatorShares",
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
    "outputs": [
      {
        "name": "shares",
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
    "name": "pendingWithdrawals",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "pending",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "permissionController",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPermissionController"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "queueWithdrawals",
    "inputs": [
      {
        "name": "params",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.QueuedWithdrawalParams[]",
        "components": [
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "depositShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "queuedWithdrawals",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "delegatedTo",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "withdrawer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "startBlock",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "redelegate",
    "inputs": [
      {
        "name": "newOperator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "newOperatorApproverSig",
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
      },
      {
        "name": "approverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "withdrawalRoots",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerAsOperator",
    "inputs": [
      {
        "name": "initDelegationApprover",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "allocationDelay",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "metadataURI",
        "type": "string",
        "internalType": "string"
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
    "name": "strategyManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategyManager"
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
    "name": "undelegate",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "withdrawalRoots",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
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
    "name": "updateOperatorMetadataURI",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "metadataURI",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "DelegationApproverUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newDelegationApprover",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DepositScalingFactorUpdated",
    "inputs": [
      {
        "name": "staker",
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
        "name": "newDepositScalingFactor",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "OperatorMetadataURIUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "metadataURI",
        "type": "string",
        "indexed": false,
        "internalType": "string"
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
        "name": "delegationApprover",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSharesBurned",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "shares",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSharesDecreased",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "staker",
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
        "name": "shares",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSharesIncreased",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "staker",
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
        "name": "shares",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "SlashingWithdrawalCompleted",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlashingWithdrawalQueued",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "withdrawal",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IDelegationManagerTypes.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "sharesToWithdraw",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakerDelegated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakerForceUndelegated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakerUndelegated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
    "name": "ActivelyDelegated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CallerCannotUndelegate",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FullySlashed",
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
    "name": "InputArrayLengthZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPermissions",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSnapshotOrdering",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotActivelyDelegated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyAllocationManager",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEigenPodManager",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyPauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyStrategyManagerOrEigenPodManager",
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
    "name": "OperatorsCannotUndelegate",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SaltSpent",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SignatureExpired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawalDelayNotElapsed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawalExceedsMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawalNotQueued",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawerNotCaller",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawerNotStaker",
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
pub mod DelegationManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610180604052348015610010575f5ffd5b506040516160b13803806160b183398101604081905261002f91610235565b8186868684876001600160a01b03811661005c576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b0390811660805293841660a05291831660c05290911660e05263ffffffff166101005246610120526100936100b7565b610140526001600160a01b0316610160526100ac610162565b5050505050506102c0565b5f61012051461461015a5750604080518082018252600a81526922b4b3b2b72630bcb2b960b11b60209182015281517f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866818301527f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea81840152466060820152306080808301919091528351808303909101815260a0909101909252815191012090565b506101405190565b5f54610100900460ff16156101cd5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff9081161461021c575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b0381168114610232575f5ffd5b50565b5f5f5f5f5f5f60c0878903121561024a575f5ffd5b86516102558161021e565b60208801519096506102668161021e565b60408801519095506102778161021e565b60608801519094506102888161021e565b60808801519093506102998161021e565b60a088015190925063ffffffff811681146102b2575f5ffd5b809150509295509295509295565b60805160a05160c05160e05161010051610120516101405161016051615cfc6103b55f395f81816103f2015261346d01525f61407701525f613fb701525f8181610741015281816135f80152613ac701525f818161079101528181610ce701528181610e980152818161177001528181611bf7015281816124e701528181612998015261428a01525f818161041901528181610e1e015281816116d701528181611935015281816132560152613f7701525f818161034f01528181610dec01528181611889015281816125d40152613f5101525f818161059401528181610bdf01528181610fb8015261280a0152615cfc5ff3fe608060405234801561000f575f5ffd5b50600436106102b1575f3560e01c8063778e55f31161017b578063bfae3fd2116100e4578063e4cc3f901161009e578063f0e0e67611610079578063f0e0e67614610812578063f2fde38b14610832578063f698da2514610845578063fabc1cbc1461084d575f5ffd5b8063e4cc3f90146107d9578063ee74937f146107ec578063eea9064b146107ff575f5ffd5b8063bfae3fd214610724578063c448feb814610737578063c978f7ac1461076b578063ca8aa7c71461078c578063cd6dc687146107b3578063da8be864146107c6575f5ffd5b80639435bb43116101355780639435bb431461060257806399f5371b14610615578063a1788484146106a3578063a33a3433146106c2578063b7f06ebe146106d5578063bb45fef2146106f7575f5ffd5b8063778e55f31461055257806378296ec51461057c578063886f11951461058f5780638da5cb5b146105b657806390041347146105c75780639104c319146105e7575f5ffd5b8063595c6a671161021d57806360a0d1ce116101d757806360a0d1ce146104c857806365da1264146104db57806366d5ba93146105035780636d70f7ae146105245780636e17444814610537578063715018a61461054a575f5ffd5b8063595c6a671461044e578063597b36da146104565780635ac86ab7146104695780635c975abb1461048c5780635dd68579146104945780635f48e667146104b5575f5ffd5b80633c651cf21161026e5780633c651cf2146103895780633cdeb5e01461039c5780633e28391d146103ca5780634657e26a146103ed5780634665bcda1461041457806354b7c96c1461043b575f5ffd5b806304a4f979146102b55780630b9f487a146102ef5780630dd8dd0214610302578063136439dd146103225780632aa6d8881461033757806339b70e381461034a575b5f5ffd5b6102dc7f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad81565b6040519081526020015b60405180910390f35b6102dc6102fd366004614c1c565b610860565b610315610310366004614cb3565b6108e8565b6040516102e69190614cf1565b610335610330366004614d28565b610bca565b005b610335610345366004614d8f565b610c9f565b6103717f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016102e6565b610335610397366004614ded565b610de1565b6103716103aa366004614e30565b6001600160a01b039081165f908152609960205260409020600101541690565b6103dd6103d8366004614e30565b610f28565b60405190151581526020016102e6565b6103717f000000000000000000000000000000000000000000000000000000000000000081565b6103717f000000000000000000000000000000000000000000000000000000000000000081565b610335610449366004614e4b565b610f47565b610335610fa3565b6102dc6104643660046150bc565b611052565b6103dd6104773660046150ed565b606654600160ff9092169190911b9081161490565b6066546102dc565b6104a76104a2366004614e30565b611081565b6040516102e692919061524c565b6103356104c33660046152b9565b611438565b6103356104d636600461533e565b6116cc565b6103716104e9366004614e30565b609a6020525f90815260409020546001600160a01b031681565b610516610511366004614e30565b611861565b6040516102e692919061537d565b6103dd610532366004614e30565b611b61565b6102dc610545366004614e4b565b611b99565b610335611c9f565b6102dc610560366004614e4b565b609860209081525f928352604080842090915290825290205481565b61033561058a3660046153a1565b611cb0565b6103717f000000000000000000000000000000000000000000000000000000000000000081565b6033546001600160a01b0316610371565b6105da6105d53660046153f1565b611d38565b6040516102e6919061543d565b61037173beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac081565b61033561061036600461544f565b611e0e565b610665610623366004614d28565b60a46020525f9081526040902080546001820154600283015460038401546004909401546001600160a01b039384169492841693909116919063ffffffff1685565b604080516001600160a01b03968716815294861660208601529290941691830191909152606082015263ffffffff909116608082015260a0016102e6565b6102dc6106b1366004614e30565b609f6020525f908152604090205481565b6103156106d03660046154eb565b611ec4565b6103dd6106e3366004614d28565b609e6020525f908152604090205460ff1681565b6103dd6107053660046155d2565b609c60209081525f928352604080842090915290825290205460ff1681565b6102dc610732366004614e4b565b611f61565b60405163ffffffff7f00000000000000000000000000000000000000000000000000000000000000001681526020016102e6565b61077e6107793660046153f1565b611f9d565b6040516102e69291906155fc565b6103717f000000000000000000000000000000000000000000000000000000000000000081565b6103356107c13660046155d2565b61222a565b6103156107d4366004614e30565b612345565b6103356107e736600461561b565b612486565b6103356107fa366004615699565b6124dc565b61033561080d3660046154eb565b612679565b6108256108203660046156e7565b6126dc565b6040516102e69190615794565b610335610840366004614e30565b612781565b6102dc6127fa565b61033561085b366004614d28565b612808565b604080517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad60208201526001600160a01b03808616928201929092528187166060820152908516608082015260a0810183905260c081018290525f906108de9060e0016040516020818303038152906040528051906020012061291f565b9695505050505050565b6066546060906001906002908116036109145760405163840a48d560e01b815260040160405180910390fd5b5f836001600160401b0381111561092d5761092d614e82565b604051908082528060200260200182016040528015610956578160200160208202803683370190505b50335f908152609a60205260408120549192506001600160a01b03909116905b85811015610bbf57868682818110610990576109906157a6565b90506020028101906109a291906157ba565b6109b09060208101906157d8565b90508787838181106109c4576109c46157a6565b90506020028101906109d691906157ba565b6109e090806157d8565b905014610a00576040516343714afd60e01b815260040160405180910390fd5b33878783818110610a1357610a136157a6565b9050602002810190610a2591906157ba565b610a36906060810190604001614e30565b6001600160a01b031614610a5d576040516330c4716960e21b815260040160405180910390fd5b5f610ac733848a8a86818110610a7557610a756157a6565b9050602002810190610a8791906157ba565b610a9190806157d8565b808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061294d92505050565b9050610b9933848a8a86818110610ae057610ae06157a6565b9050602002810190610af291906157ba565b610afc90806157d8565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508e92508d9150889050818110610b4157610b416157a6565b9050602002810190610b5391906157ba565b610b619060208101906157d8565b808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250889250612a94915050565b848381518110610bab57610bab6157a6565b602090810291909101015250600101610976565b509095945050505050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015610c2c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c50919061581d565b610c6d57604051631d77d47760e21b815260040160405180910390fd5b6066548181168114610c925760405163c61dca5d60e01b815260040160405180910390fd5b610c9b8261306d565b5050565b610ca833610f28565b15610cc657604051633bf2b50360e11b815260040160405180910390fd5b604051632b6241f360e11b815233600482015263ffffffff841660248201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906356c483e6906044015f604051808303815f87803b158015610d30575f5ffd5b505af1158015610d42573d5f5f3e3d5ffd5b50505050610d5033856130aa565b610d5a333361310c565b6040516001600160a01b038516815233907fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c19060200160405180910390a2336001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908383604051610dd3929190615838565b60405180910390a250505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480610e405750336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016145b610e5d5760405163045206a560e21b815260040160405180910390fd5b6001600160a01b038481165f908152609a602052604080822054905163152667d960e31b8152908316600482018190528684166024830152927f0000000000000000000000000000000000000000000000000000000000000000169063a9333ec890604401602060405180830381865afa158015610edd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f019190615866565b90505f610f0f87878461320f565b9050610f1f8388888888866132f1565b50505050505050565b6001600160a01b039081165f908152609a602052604090205416151590565b81610f518161342f565b610f6e5760405163932d94f760e01b815260040160405180910390fd5b610f7783611b61565b610f94576040516325ec6c1f60e01b815260040160405180910390fd5b610f9e83836130aa565b505050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611005573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611029919061581d565b61104657604051631d77d47760e21b815260040160405180910390fd5b6110505f1961306d565b565b5f816040516020016110649190615881565b604051602081830303815290604052805190602001209050919050565b6001600160a01b0381165f90815260a36020526040812060609182916110a6906134d9565b8051909150806001600160401b038111156110c3576110c3614e82565b6040519080825280602002602001820160405280156110fc57816020015b6110e9614ad8565b8152602001906001900390816110e15790505b509350806001600160401b0381111561111757611117614e82565b60405190808252806020026020018201604052801561114a57816020015b60608152602001906001900390816111355790505b506001600160a01b038087165f908152609a60205260408120549295509116905b8281101561142f5760a45f858381518110611188576111886157a6565b60209081029190910181015182528181019290925260409081015f20815160e08101835281546001600160a01b03908116825260018301548116828601526002830154168184015260038201546060820152600482015463ffffffff1660808201526005820180548451818702810187019095528085529194929360a086019390929083018282801561124257602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611224575b505050505081526020016006820180548060200260200160405190810160405280929190818152602001828054801561129857602002820191905f5260205f20905b815481526020019060010190808311611284575b5050505050815250508682815181106112b3576112b36157a6565b60200260200101819052508581815181106112d0576112d06157a6565b602002602001015160a00151516001600160401b038111156112f4576112f4614e82565b60405190808252806020026020018201604052801561131d578160200160208202803683370190505b50858281518110611330576113306157a6565b60200260200101819052505f6113648884898581518110611353576113536157a6565b602002602001015160a0015161294d565b90505f5b87838151811061137a5761137a6157a6565b602002602001015160a0015151811015611425576113e78884815181106113a3576113a36157a6565b602002602001015160c0015182815181106113c0576113c06157a6565b60200260200101518385815181106113da576113da6157a6565b60200260200101516134e5565b8784815181106113f9576113f96157a6565b60200260200101518281518110611412576114126157a6565b6020908102919091010152600101611368565b505060010161116b565b50505050915091565b6066546002906004908116036114615760405163840a48d560e01b815260040160405180910390fd5b6114696134f0565b335f90815260a3602052604081209061148182613549565b90508084116114905783611492565b805b93505f846001600160401b038111156114ad576114ad614e82565b6040519080825280602002602001820160405280156114e657816020015b6114d3614ad8565b8152602001906001900390816114cb5790505b5090505f5b81518110156116355760a45f6115018684613552565b815260208082019290925260409081015f20815160e08101835281546001600160a01b03908116825260018301548116828601526002830154168184015260038201546060820152600482015463ffffffff1660808201526005820180548451818702810187019095528085529194929360a08601939092908301828280156115b157602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611593575b505050505081526020016006820180548060200260200160405190810160405280929190818152602001828054801561160757602002820191905f5260205f20905b8154815260200190600101908083116115f3575b505050505081525050828281518110611622576116226157a6565b60209081029190910101526001016114eb565b505f5b81518110156116b6576116ae828281518110611656576116566157a6565b60200260200101518b8b84818110611670576116706157a6565b905060200281019061168291906157d8565b8b8b86818110611694576116946157a6565b90506020020160208101906116a99190615893565b61355d565b600101611638565b505050506116c4600160c955565b505050505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461171557604051633213a66160e21b815260040160405180910390fd5b61171e83610f28565b15610f9e576001600160a01b038381165f908152609a602052604080822054905163152667d960e31b81529083166004820181905273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06024830152927f0000000000000000000000000000000000000000000000000000000000000000169063a9333ec890604401602060405180830381865afa1580156117b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117d99190615866565b6001600160a01b0386165f90815260a26020908152604080832073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac084528252808320815192830190915254815291925061183f866118376001600160401b038087169089166139d7565b8491906139eb565b9050610f1f848873beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac084613a11565b6040516394f649dd60e01b81526001600160a01b03828116600483015260609182915f9182917f000000000000000000000000000000000000000000000000000000000000000016906394f649dd906024015f60405180830381865afa1580156118cd573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526118f49190810190615909565b60405163fe243a1760e01b81526001600160a01b03888116600483015273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac060248301529294509092505f917f0000000000000000000000000000000000000000000000000000000000000000169063fe243a1790604401602060405180830381865afa15801561197a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061199e91906159c4565b9050805f036119b257509094909350915050565b5f835160016119c191906159ef565b6001600160401b038111156119d8576119d8614e82565b604051908082528060200260200182016040528015611a01578160200160208202803683370190505b5090505f84516001611a1391906159ef565b6001600160401b03811115611a2a57611a2a614e82565b604051908082528060200260200182016040528015611a53578160200160208202803683370190505b50905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac082865181518110611a7e57611a7e6157a6565b60200260200101906001600160a01b031690816001600160a01b0316815250508281865181518110611ab257611ab26157a6565b60209081029190910101525f5b8551811015611b5357858181518110611ada57611ada6157a6565b6020026020010151838281518110611af457611af46157a6565b60200260200101906001600160a01b031690816001600160a01b031681525050848181518110611b2657611b266157a6565b6020026020010151828281518110611b4057611b406157a6565b6020908102919091010152600101611abf565b509097909650945050505050565b5f6001600160a01b03821615801590611b9357506001600160a01b038083165f818152609a6020526040902054909116145b92915050565b6040805160018082528183019092525f918291906020808301908036833701905050905082815f81518110611bd057611bd06157a6565b6001600160a01b03928316602091820292909201015260405163547afb8760e01b81525f917f0000000000000000000000000000000000000000000000000000000000000000169063547afb8790611c2e9088908690600401615a02565b5f60405180830381865afa158015611c48573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611c6f9190810190615a25565b5f81518110611c8057611c806157a6565b60200260200101519050611c968585835f613a8b565b95945050505050565b611ca7613b3c565b6110505f613b96565b82611cba8161342f565b611cd75760405163932d94f760e01b815260040160405180910390fd5b611ce084611b61565b611cfd576040516325ec6c1f60e01b815260040160405180910390fd5b836001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908484604051610dd3929190615838565b60605f82516001600160401b03811115611d5457611d54614e82565b604051908082528060200260200182016040528015611d7d578160200160208202803683370190505b5090505f5b8351811015611e06576001600160a01b0385165f9081526098602052604081208551909190869084908110611db957611db96157a6565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f2054828281518110611df357611df36157a6565b6020908102919091010152600101611d82565b509392505050565b606654600290600490811603611e375760405163840a48d560e01b815260040160405180910390fd5b611e3f6134f0565b855f5b81811015611eb857611eb0898983818110611e5f57611e5f6157a6565b9050602002810190611e719190615ab4565b611e7a90615ac8565b888884818110611e8c57611e8c6157a6565b9050602002810190611e9e91906157d8565b888886818110611694576116946157a6565b600101611e42565b5050610f1f600160c955565b6060611ecf33610f28565b611eec5760405163a5c7c44560e01b815260040160405180910390fd5b611ef533611b61565b15611f13576040516311ca333560e31b815260040160405180910390fd5b611f1c84611b61565b611f39576040516325ec6c1f60e01b815260040160405180910390fd5b611f4233613be7565b9050611f5033858585613e46565b611f5a338561310c565b9392505050565b6001600160a01b038083165f90815260a260209081526040808320938516835292815282822083519182019093529154825290611f5a90613f0b565b60608082516001600160401b03811115611fb957611fb9614e82565b604051908082528060200260200182016040528015611fe2578160200160208202803683370190505b50915082516001600160401b03811115611ffe57611ffe614e82565b604051908082528060200260200182016040528015612027578160200160208202803683370190505b506001600160a01b038086165f908152609a602052604081205492935091169061205286838761294d565b90505f5b855181101561221f575f612082878381518110612075576120756157a6565b6020026020010151613f2a565b9050806001600160a01b031663fe243a17898985815181106120a6576120a66157a6565b60200260200101516040518363ffffffff1660e01b81526004016120e09291906001600160a01b0392831681529116602082015260400190565b602060405180830381865afa1580156120fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061211f91906159c4565b858381518110612131576121316157a6565b6020026020010181815250505f60a25f8a6001600160a01b03166001600160a01b031681526020019081526020015f205f898581518110612174576121746157a6565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f8201548152505090506121f88684815181106121c6576121c66157a6565b60200260200101518585815181106121e0576121e06157a6565b6020026020010151836139eb9092919063ffffffff16565b87848151811061220a5761220a6157a6565b60209081029190910101525050600101612056565b5050505b9250929050565b5f54610100900460ff161580801561224857505f54600160ff909116105b806122615750303b15801561226157505f5460ff166001145b6122c95760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156122ea575f805461ff0019166101001790555b6122f38261306d565b6122fc83613b96565b8015610f9e575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050565b606061235082610f28565b61236d5760405163a5c7c44560e01b815260040160405180910390fd5b61237682611b61565b15612394576040516311ca333560e31b815260040160405180910390fd5b6001600160a01b0382166123bb576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b038083165f818152609a6020526040902054909116903314806123e957506123e98161342f565b8061240f57506001600160a01b038181165f908152609960205260409020600101541633145b61242c57604051631e499a2360e11b815260040160405180910390fd5b336001600160a01b0384161461247d57806001600160a01b0316836001600160a01b03167ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a60405160405180910390a35b611f5a83613be7565b6066546002906004908116036124af5760405163840a48d560e01b815260040160405180910390fd5b6124b76134f0565b6124cb6124c386615ac8565b85858561355d565b6124d5600160c955565b5050505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614612525576040516323d871a560e01b815260040160405180910390fd5b6001600160a01b038085165f908152609860209081526040808320938716835292905290812054612563906001600160401b03808616908516613f9c565b90505f61257286868686613a8b565b61257c90836159ef565b905061258a865f8785613a11565b6001600160a01b03851673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0146116c457604051633b9e9f0160e21b81526001600160a01b038681166004830152602482018390527f0000000000000000000000000000000000000000000000000000000000000000169063ee7a7c04906044015f604051808303815f87803b158015612615575f5ffd5b505af1158015612627573d5f5f3e3d5ffd5b5050604080516001600160a01b038981168252602082018690528a1693507feff6aab2bc3f7c648896e1522eae71d6c22e3b0e218206b3f40af0e4d204716b92500160405180910390a2505050505050565b61268233610f28565b156126a057604051633bf2b50360e11b815260040160405180910390fd5b6126a983611b61565b6126c6576040516325ec6c1f60e01b815260040160405180910390fd5b6126d233848484613e46565b610f9e338461310c565b60605f83516001600160401b038111156126f8576126f8614e82565b60405190808252806020026020018201604052801561272b57816020015b60608152602001906001900390816127165790505b5090505f5b8451811015611e065761275c85828151811061274e5761274e6157a6565b602002602001015185611d38565b82828151811061276e5761276e6157a6565b6020908102919091010152600101612730565b612789613b3c565b6001600160a01b0381166127ee5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016122c0565b6127f781613b96565b50565b5f612803613fb4565b905090565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612864573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128889190615ad3565b6001600160a01b0316336001600160a01b0316146128b95760405163794821ff60e01b815260040160405180910390fd5b606654801982198116146128e05760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020015b60405180910390a25050565b5f612928613fb4565b60405161190160f01b6020820152602281019190915260428101839052606201611064565b60605f82516001600160401b0381111561296957612969614e82565b604051908082528060200260200182016040528015612992578160200160208202803683370190505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663547afb8786866040518363ffffffff1660e01b81526004016129e4929190615a02565b5f60405180830381865afa1580156129fe573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612a259190810190615a25565b90505f5b8451811015610bbf57612a6f87868381518110612a4857612a486157a6565b6020026020010151848481518110612a6257612a626157a6565b602002602001015161320f565b838281518110612a8157612a816157a6565b6020908102919091010152600101612a29565b5f6001600160a01b038616612abc576040516339b190bb60e11b815260040160405180910390fd5b83515f03612add5760405163796cc52560e01b815260040160405180910390fd5b5f84516001600160401b03811115612af757612af7614e82565b604051908082528060200260200182016040528015612b20578160200160208202803683370190505b5090505f85516001600160401b03811115612b3d57612b3d614e82565b604051908082528060200260200182016040528015612b66578160200160208202803683370190505b5090505f5b8651811015612ea0575f612b8a888381518110612075576120756157a6565b90505f60a25f8c6001600160a01b03166001600160a01b031681526020019081526020015f205f8a8581518110612bc357612bc36157a6565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f820154815250509050816001600160a01b031663fe243a178c8b8681518110612c2257612c226157a6565b60200260200101516040518363ffffffff1660e01b8152600401612c5c9291906001600160a01b0392831681529116602082015260400190565b602060405180830381865afa158015612c77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c9b91906159c4565b888481518110612cad57612cad6157a6565b60200260200101511115612cd45760405163f020e5b960e01b815260040160405180910390fd5b612d03888481518110612ce957612ce96157a6565b60200260200101518885815181106121e0576121e06157a6565b848481518110612d1557612d156157a6565b602002602001018181525050612d5d848481518110612d3657612d366157a6565b6020026020010151888581518110612d5057612d506157a6565b6020026020010151614099565b858481518110612d6f57612d6f6157a6565b60209081029190910101526001600160a01b038a1615612e0457612dc68a8a8581518110612d9f57612d9f6157a6565b6020026020010151878681518110612db957612db96157a6565b60200260200101516140b2565b612e048a8c8b8681518110612ddd57612ddd6157a6565b6020026020010151878781518110612df757612df76157a6565b6020026020010151613a11565b816001600160a01b031663724af4238c8b8681518110612e2657612e266157a6565b60200260200101518b8781518110612e4057612e406157a6565b60200260200101516040518463ffffffff1660e01b8152600401612e6693929190615aee565b5f604051808303815f87803b158015612e7d575f5ffd5b505af1158015612e8f573d5f5f3e3d5ffd5b505050505050806001019050612b6b565b506001600160a01b0388165f908152609f60205260408120805491829190612ec783615b12565b91905055505f6040518060e001604052808b6001600160a01b031681526020018a6001600160a01b031681526020018b6001600160a01b031681526020018381526020014363ffffffff1681526020018981526020018581525090505f612f2d82611052565b5f818152609e602090815260408083208054600160ff19909116811790915560a4835292819020865181546001600160a01b03199081166001600160a01b039283161783558885015195830180548216968316969096179095559187015160028201805490951692169190911790925560608501516003830155608085015160048301805463ffffffff191663ffffffff90921691909117905560a085015180519394508593612fe39260058501920190614b31565b5060c08201518051612fff916006840191602090910190614b94565b5050506001600160a01b038b165f90815260a3602052604090206130239082614140565b507f26b2aae26516e8719ef50ea2f6831a2efbd4e37dccdf0f6936b27bc08e793e3081838660405161305793929190615b2a565b60405180910390a19a9950505050505050505050565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b6001600160a01b038281165f8181526099602090815260409182902060010180546001600160a01b0319169486169485179055905192835290917f773b54c04d756fcc5e678111f7d730de3be98192000799eee3d63716055a87c69101612913565b6066545f906001908116036131345760405163840a48d560e01b815260040160405180910390fd5b6001600160a01b038381165f818152609a602052604080822080546001600160a01b0319169487169485179055517fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433049190a35f5f61319185611861565b915091505f6131a186868561294d565b90505f5b8351811015610f1f5761320786888684815181106131c5576131c56157a6565b60200260200101515f8786815181106131e0576131e06157a6565b60200260200101518787815181106131fa576131fa6157a6565b60200260200101516132f1565b6001016131a5565b5f73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeabf196001600160a01b038416016132e15760405163a3d75e0960e01b81526001600160a01b0385811660048301525f917f00000000000000000000000000000000000000000000000000000000000000009091169063a3d75e0990602401602060405180830381865afa15801561329d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132c19190615866565b90506132d96001600160401b038481169083166139d7565b915050611f5a565b506001600160401b031692915050565b805f0361331157604051630a33bc6960e21b815260040160405180910390fd5b6001600160a01b038086165f90815260a2602090815260408083209388168352929052206133418185858561414b565b6040805160208101909152815481527f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f908790879061337f90613f0b565b60405161338e93929190615aee565b60405180910390a161339f86610f28565b15610f1f576001600160a01b038088165f908152609860209081526040808320938916835292905290812080548592906133da9084906159ef565b92505081905550866001600160a01b03167f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c87878660405161341e93929190615aee565b60405180910390a250505050505050565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af11580156134b5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b93919061581d565b60605f611f5a836141c0565b5f611f5a83836139d7565b600260c954036135425760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064016122c0565b600260c955565b5f611b93825490565b5f611f5a8383614219565b60a0840151518214613582576040516343714afd60e01b815260040160405180910390fd5b83604001516001600160a01b0316336001600160a01b0316146135b8576040516316110d3560e21b815260040160405180910390fd5b5f6135c285611052565b5f818152609e602052604090205490915060ff166135f3576040516387c9d21960e01b815260040160405180910390fd5b60605f7f000000000000000000000000000000000000000000000000000000000000000087608001516136269190615b54565b90504363ffffffff168163ffffffff161115613655576040516378f67ae160e11b815260040160405180910390fd5b61366c875f015188602001518960a001518461423f565b87516001600160a01b039081165f908152609a60205260408120548a5160a08c015194965092169350916136a29190849061294d565b90505f5b8860a00151518110156138f5575f6136cd8a60a001518381518110612075576120756157a6565b90505f6137038b60c0015184815181106136e9576136e96157a6565b60200260200101518785815181106113da576113da6157a6565b905087156137d357816001600160a01b0316632eae418c8c5f01518d60a001518681518110613734576137346157a6565b60200260200101518d8d8881811061374e5761374e6157a6565b90506020020160208101906137639190614e30565b60405160e085901b6001600160e01b03191681526001600160a01b03938416600482015291831660248301529091166044820152606481018490526084015f604051808303815f87803b1580156137b8575f5ffd5b505af11580156137ca573d5f5f3e3d5ffd5b505050506138eb565b5f5f836001600160a01b031663c4623ea18e5f01518f60a0015188815181106137fe576137fe6157a6565b60200260200101518f8f8a818110613818576138186157a6565b905060200201602081019061382d9190614e30565b60405160e085901b6001600160e01b03191681526001600160a01b039384166004820152918316602483015290911660448201526064810186905260840160408051808303815f875af1158015613886573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138aa9190615b70565b915091506138e8878e5f01518f60a0015188815181106138cc576138cc6157a6565b602002602001015185858b8b815181106131fa576131fa6157a6565b50505b50506001016136a6565b5087516001600160a01b03165f90815260a360205260409020613918908561436d565b505f84815260a46020526040812080546001600160a01b031990811682556001820180548216905560028201805490911690556003810182905560048101805463ffffffff191690559061396f6005830182614bcd565b61397c600683015f614bcd565b50505f848152609e602052604090819020805460ff19169055517f1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a00906139c59086815260200190565b60405180910390a15050505050505050565b5f611f5a8383670de0b6b3a7640000614378565b5f613a0982613a036139fc87613f0b565b86906139d7565b906139d7565b949350505050565b6001600160a01b038085165f90815260986020908152604080832093861683529290529081208054839290613a47908490615b92565b92505081905550836001600160a01b03167f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd848484604051610dd393929190615aee565b6001600160a01b038085165f90815260a56020908152604080832093871683529290529081208190613abc9061445d565b90505f613b16613aec7f000000000000000000000000000000000000000000000000000000000000000043615ba5565b6001600160a01b03808a165f90815260a560209081526040808320938c1683529290522090614477565b90505f613b238284615b92565b9050613b30818787614493565b98975050505050505050565b6033546001600160a01b031633146110505760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016122c0565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b606654606090600190600290811603613c135760405163840a48d560e01b815260040160405180910390fd5b6001600160a01b038084165f818152609a602052604080822080546001600160a01b0319811690915590519316928392917ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af4467691a35f5f613c7286611861565b9150915081515f03613c8657505050613e40565b81516001600160401b03811115613c9f57613c9f614e82565b604051908082528060200260200182016040528015613cc8578160200160208202803683370190505b5094505f613cd787858561294d565b90505f5b8351811015613e3a576040805160018082528183019092525f916020808301908036833750506040805160018082528183019092529293505f9291506020808301908036833750506040805160018082528183019092529293505f92915060208083019080368337019050509050868481518110613d5b57613d5b6157a6565b6020026020010151835f81518110613d7557613d756157a6565b60200260200101906001600160a01b031690816001600160a01b031681525050858481518110613da757613da76157a6565b6020026020010151825f81518110613dc157613dc16157a6565b602002602001018181525050848481518110613ddf57613ddf6157a6565b6020026020010151815f81518110613df957613df96157a6565b602002602001018181525050613e128b89858585612a94565b8a8581518110613e2457613e246157a6565b6020908102919091010152505050600101613cdb565b50505050505b50919050565b6001600160a01b038084165f908152609960205260409020600101541680613e6e5750613f05565b6001600160a01b0381165f908152609c6020908152604080832085845290915290205460ff1615613eb257604051630d4c4c9160e21b815260040160405180910390fd5b6001600160a01b0381165f908152609c602090815260408083208584528252909120805460ff191660011790558301516124d5908290613ef9908890889084908890610860565b855160208701516144b1565b50505050565b80515f9015613f1b578151611b93565b670de0b6b3a764000092915050565b5f6001600160a01b03821673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014613f75577f0000000000000000000000000000000000000000000000000000000000000000611b93565b7f000000000000000000000000000000000000000000000000000000000000000092915050565b5f613faa8483856001614503565b613a099085615b92565b5f7f000000000000000000000000000000000000000000000000000000000000000046146140745750604080518082018252600a81526922b4b3b2b72630bcb2b960b11b60209182015281517f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866818301527f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea81840152466060820152306080808301919091528351808303909101815260a0909101909252815191012090565b507f000000000000000000000000000000000000000000000000000000000000000090565b5f815f036140a857505f611b93565b611f5a8383614552565b6001600160a01b03821673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014610f9e576001600160a01b038084165f90815260a56020908152604080832093861683529290529081206141059061445d565b9050613f054361411584846159ef565b6001600160a01b038088165f90815260a560209081526040808320938a168352929052209190614566565b5f611f5a8383614571565b825f0361416b57614164670de0b6b3a764000082614552565b8455613f05565b6040805160208101909152845481525f906141879085846139eb565b90505f61419484836159ef565b90505f6141b5846141af6141a8888a6159ef565b8590614552565b90614552565b875550505050505050565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561420d57602002820191905f5260205f20905b8154815260200190600101908083116141f9575b50505050509050919050565b5f825f01828154811061422e5761422e6157a6565b905f5260205f200154905092915050565b60605f83516001600160401b0381111561425b5761425b614e82565b604051908082528060200260200182016040528015614284578160200160208202803683370190505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166394d7d00c8787876040518463ffffffff1660e01b81526004016142d893929190615bc1565b5f60405180830381865afa1580156142f2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526143199190810190615a25565b90505f5b85518110156143615761433c88878381518110612a4857612a486157a6565b83828151811061434e5761434e6157a6565b602090810291909101015260010161431d565b50909695505050505050565b5f611f5a83836145bd565b5f80805f19858709858702925082811083820303915050805f036143af578382816143a5576143a5615bfa565b0492505050611f5a565b8084116143f65760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b60448201526064016122c0565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b5f61446882826146a0565b6001600160e01b031692915050565b5f6144838383836146e5565b6001600160e01b03169392505050565b5f613a096144a18385615c0e565b85906001600160401b03166139d7565b428110156144d257604051630819bdcd60e01b815260040160405180910390fd5b6144e66001600160a01b038516848461472e565b613f0557604051638baa579f60e01b815260040160405180910390fd5b5f5f614510868686614378565b9050600183600281111561452657614526615c2d565b14801561454257505f848061453d5761453d615bfa565b868809115b15611c96576108de6001826159ef565b5f611f5a83670de0b6b3a764000084614378565b610f9e838383614782565b5f8181526001830160205260408120546145b657508154600181810184555f848152602080822090930184905584548482528286019093526040902091909155611b93565b505f611b93565b5f8181526001830160205260408120548015614697575f6145df600183615b92565b85549091505f906145f290600190615b92565b9050818114614651575f865f018281548110614610576146106157a6565b905f5260205f200154905080875f018481548110614630576146306157a6565b5f918252602080832090910192909255918252600188019052604090208390555b855486908061466257614662615c41565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050611b93565b5f915050611b93565b81545f9080156146dd576146c6846146b9600184615b92565b5f91825260209091200190565b5464010000000090046001600160e01b0316613a09565b509092915050565b82545f90816146f686868385614888565b905080156147245761470d866146b9600184615b92565b5464010000000090046001600160e01b03166108de565b5091949350505050565b5f5f5f61473b85856148db565b90925090505f81600481111561475357614753615c2d565b1480156147715750856001600160a01b0316826001600160a01b0316145b806108de57506108de86868661491a565b8254801561483a575f61479a856146b9600185615b92565b60408051808201909152905463ffffffff8082168084526401000000009092046001600160e01b0316602084015291925090851610156147ed5760405163151b8e3f60e11b815260040160405180910390fd5b805163ffffffff808616911603614838578261480e866146b9600186615b92565b80546001600160e01b03929092166401000000000263ffffffff9092169190911790555050505050565b505b506040805180820190915263ffffffff92831681526001600160e01b03918216602080830191825285546001810187555f968752952091519051909216640100000000029190921617910155565b5f5b81831015611e06575f61489d8484614a01565b5f8781526020902090915063ffffffff86169082015463ffffffff1611156148c7578092506148d5565b6148d28160016159ef565b93505b5061488a565b5f5f825160410361490f576020830151604084015160608501515f1a61490387828585614a1b565b94509450505050612223565b505f90506002612223565b5f5f5f856001600160a01b0316631626ba7e60e01b8686604051602401614942929190615c55565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516149809190615c91565b5f60405180830381855afa9150503d805f81146149b8576040519150601f19603f3d011682016040523d82523d5f602084013e6149bd565b606091505b50915091508180156149d157506020815110155b80156108de57508051630b135d3f60e11b906149f690830160209081019084016159c4565b149695505050505050565b5f614a0f6002848418615ca7565b611f5a908484166159ef565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115614a5057505f90506003614acf565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015614aa1573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116614ac9575f60019250925050614acf565b91505f90505b94509492505050565b6040518060e001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f63ffffffff16815260200160608152602001606081525090565b828054828255905f5260205f20908101928215614b84579160200282015b82811115614b8457825182546001600160a01b0319166001600160a01b03909116178255602090920191600190910190614b4f565b50614b90929150614be4565b5090565b828054828255905f5260205f20908101928215614b84579160200282015b82811115614b84578251825591602001919060010190614bb2565b5080545f8255905f5260205f20908101906127f791905b5b80821115614b90575f8155600101614be5565b6001600160a01b03811681146127f7575f5ffd5b8035614c1781614bf8565b919050565b5f5f5f5f5f60a08688031215614c30575f5ffd5b8535614c3b81614bf8565b94506020860135614c4b81614bf8565b93506040860135614c5b81614bf8565b94979396509394606081013594506080013592915050565b5f5f83601f840112614c83575f5ffd5b5081356001600160401b03811115614c99575f5ffd5b6020830191508360208260051b8501011115612223575f5ffd5b5f5f60208385031215614cc4575f5ffd5b82356001600160401b03811115614cd9575f5ffd5b614ce585828601614c73565b90969095509350505050565b602080825282518282018190525f918401906040840190835b81811015610bbf578351835260209384019390920191600101614d0a565b5f60208284031215614d38575f5ffd5b5035919050565b803563ffffffff81168114614c17575f5ffd5b5f5f83601f840112614d62575f5ffd5b5081356001600160401b03811115614d78575f5ffd5b602083019150836020828501011115612223575f5ffd5b5f5f5f5f60608587031215614da2575f5ffd5b8435614dad81614bf8565b9350614dbb60208601614d3f565b925060408501356001600160401b03811115614dd5575f5ffd5b614de187828801614d52565b95989497509550505050565b5f5f5f5f60808587031215614e00575f5ffd5b8435614e0b81614bf8565b93506020850135614e1b81614bf8565b93969395505050506040820135916060013590565b5f60208284031215614e40575f5ffd5b8135611f5a81614bf8565b5f5f60408385031215614e5c575f5ffd5b8235614e6781614bf8565b91506020830135614e7781614bf8565b809150509250929050565b634e487b7160e01b5f52604160045260245ffd5b60405160e081016001600160401b0381118282101715614eb857614eb8614e82565b60405290565b604080519081016001600160401b0381118282101715614eb857614eb8614e82565b604051601f8201601f191681016001600160401b0381118282101715614f0857614f08614e82565b604052919050565b5f6001600160401b03821115614f2857614f28614e82565b5060051b60200190565b5f82601f830112614f41575f5ffd5b8135614f54614f4f82614f10565b614ee0565b8082825260208201915060208360051b860101925085831115614f75575f5ffd5b602085015b83811015614f9b578035614f8d81614bf8565b835260209283019201614f7a565b5095945050505050565b5f82601f830112614fb4575f5ffd5b8135614fc2614f4f82614f10565b8082825260208201915060208360051b860101925085831115614fe3575f5ffd5b602085015b83811015614f9b578035835260209283019201614fe8565b5f60e08284031215615010575f5ffd5b615018614e96565b905061502382614c0c565b815261503160208301614c0c565b602082015261504260408301614c0c565b60408201526060828101359082015261505d60808301614d3f565b608082015260a08201356001600160401b0381111561507a575f5ffd5b61508684828501614f32565b60a08301525060c08201356001600160401b038111156150a4575f5ffd5b6150b084828501614fa5565b60c08301525092915050565b5f602082840312156150cc575f5ffd5b81356001600160401b038111156150e1575f5ffd5b613a0984828501615000565b5f602082840312156150fd575f5ffd5b813560ff81168114611f5a575f5ffd5b5f8151808452602084019350602083015f5b828110156151465781516001600160a01b031686526020958601959091019060010161511f565b5093949350505050565b5f8151808452602084019350602083015f5b82811015615146578151865260209586019590910190600101615162565b80516001600160a01b03908116835260208083015182169084015260408083015190911690830152606080820151908301526080808201515f916151cb9085018263ffffffff169052565b5060a082015160e060a08501526151e560e085018261510d565b905060c083015184820360c0860152611c968282615150565b5f82825180855260208501945060208160051b830101602085015f5b8381101561436157601f19858403018852615236838351615150565b602098890198909350919091019060010161521a565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b828110156152a357605f1987860301845261528e858351615180565b94506020938401939190910190600101615272565b505050508281036020840152611c9681856151fe565b5f5f5f5f5f606086880312156152cd575f5ffd5b85356001600160401b038111156152e2575f5ffd5b6152ee88828901614c73565b90965094505060208601356001600160401b0381111561530c575f5ffd5b61531888828901614c73565b96999598509660400135949350505050565b6001600160401b03811681146127f7575f5ffd5b5f5f5f60608486031215615350575f5ffd5b833561535b81614bf8565b92506020840135915060408401356153728161532a565b809150509250925092565b604081525f61538f604083018561510d565b8281036020840152611c968185615150565b5f5f5f604084860312156153b3575f5ffd5b83356153be81614bf8565b925060208401356001600160401b038111156153d8575f5ffd5b6153e486828701614d52565b9497909650939450505050565b5f5f60408385031215615402575f5ffd5b823561540d81614bf8565b915060208301356001600160401b03811115615427575f5ffd5b61543385828601614f32565b9150509250929050565b602081525f611f5a6020830184615150565b5f5f5f5f5f5f60608789031215615464575f5ffd5b86356001600160401b03811115615479575f5ffd5b61548589828a01614c73565b90975095505060208701356001600160401b038111156154a3575f5ffd5b6154af89828a01614c73565b90955093505060408701356001600160401b038111156154cd575f5ffd5b6154d989828a01614c73565b979a9699509497509295939492505050565b5f5f5f606084860312156154fd575f5ffd5b833561550881614bf8565b925060208401356001600160401b03811115615522575f5ffd5b840160408187031215615533575f5ffd5b61553b614ebe565b81356001600160401b03811115615550575f5ffd5b8201601f81018813615560575f5ffd5b80356001600160401b0381111561557957615579614e82565b61558c601f8201601f1916602001614ee0565b8181528960208385010111156155a0575f5ffd5b816020840160208301375f60209282018301528352928301359282019290925293969395505050506040919091013590565b5f5f604083850312156155e3575f5ffd5b82356155ee81614bf8565b946020939093013593505050565b604081525f61538f6040830185615150565b80151581146127f7575f5ffd5b5f5f5f5f6060858703121561562e575f5ffd5b84356001600160401b03811115615643575f5ffd5b850160e08188031215615654575f5ffd5b935060208501356001600160401b0381111561566e575f5ffd5b61567a87828801614c73565b909450925050604085013561568e8161560e565b939692955090935050565b5f5f5f5f608085870312156156ac575f5ffd5b84356156b781614bf8565b935060208501356156c781614bf8565b925060408501356156d78161532a565b9150606085013561568e8161532a565b5f5f604083850312156156f8575f5ffd5b82356001600160401b0381111561570d575f5ffd5b8301601f8101851361571d575f5ffd5b803561572b614f4f82614f10565b8082825260208201915060208360051b85010192508783111561574c575f5ffd5b6020840193505b8284101561577757833561576681614bf8565b825260209384019390910190615753565b945050505060208301356001600160401b03811115615427575f5ffd5b602081525f611f5a60208301846151fe565b634e487b7160e01b5f52603260045260245ffd5b5f8235605e198336030181126157ce575f5ffd5b9190910192915050565b5f5f8335601e198436030181126157ed575f5ffd5b8301803591506001600160401b03821115615806575f5ffd5b6020019150600581901b3603821315612223575f5ffd5b5f6020828403121561582d575f5ffd5b8151611f5a8161560e565b60208152816020820152818360408301375f818301604090810191909152601f909201601f19160101919050565b5f60208284031215615876575f5ffd5b8151611f5a8161532a565b602081525f611f5a6020830184615180565b5f602082840312156158a3575f5ffd5b8135611f5a8161560e565b5f82601f8301126158bd575f5ffd5b81516158cb614f4f82614f10565b8082825260208201915060208360051b8601019250858311156158ec575f5ffd5b602085015b83811015614f9b5780518352602092830192016158f1565b5f5f6040838503121561591a575f5ffd5b82516001600160401b0381111561592f575f5ffd5b8301601f8101851361593f575f5ffd5b805161594d614f4f82614f10565b8082825260208201915060208360051b85010192508783111561596e575f5ffd5b6020840193505b8284101561599957835161598881614bf8565b825260209384019390910190615975565b8095505050505060208301516001600160401b038111156159b8575f5ffd5b615433858286016158ae565b5f602082840312156159d4575f5ffd5b5051919050565b634e487b7160e01b5f52601160045260245ffd5b80820180821115611b9357611b936159db565b6001600160a01b03831681526040602082018190525f90613a099083018461510d565b5f60208284031215615a35575f5ffd5b81516001600160401b03811115615a4a575f5ffd5b8201601f81018413615a5a575f5ffd5b8051615a68614f4f82614f10565b8082825260208201915060208360051b850101925086831115615a89575f5ffd5b6020840193505b828410156108de578351615aa38161532a565b825260209384019390910190615a90565b5f823560de198336030181126157ce575f5ffd5b5f611b933683615000565b5f60208284031215615ae3575f5ffd5b8151611f5a81614bf8565b6001600160a01b039384168152919092166020820152604081019190915260600190565b5f60018201615b2357615b236159db565b5060010190565b838152606060208201525f615b426060830185615180565b82810360408401526108de8185615150565b63ffffffff8181168382160190811115611b9357611b936159db565b5f5f60408385031215615b81575f5ffd5b505080516020909101519092909150565b81810381811115611b9357611b936159db565b63ffffffff8281168282160390811115611b9357611b936159db565b6001600160a01b03841681526060602082018190525f90615be49083018561510d565b905063ffffffff83166040830152949350505050565b634e487b7160e01b5f52601260045260245ffd5b6001600160401b038281168282160390811115611b9357611b936159db565b634e487b7160e01b5f52602160045260245ffd5b634e487b7160e01b5f52603160045260245ffd5b828152604060208201525f82518060408401528060208501606085015e5f606082850101526060601f19601f8301168401019150509392505050565b5f82518060208501845e5f920191825250919050565b5f82615cc157634e487b7160e01b5f52601260045260245ffd5b50049056fea2646970667358221220b302bc35114e1921cf8bb16bbe96de3516e750fa9b91287e8e3ac447f0d9fea764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80`@R4\x80\x15a\0\x10W__\xFD[P`@Qa`\xB18\x03\x80a`\xB1\x839\x81\x01`@\x81\x90Ra\0/\x91a\x025V[\x81\x86\x86\x86\x84\x87`\x01`\x01`\xA0\x1B\x03\x81\x16a\0\\W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x93\x84\x16`\xA0R\x91\x83\x16`\xC0R\x90\x91\x16`\xE0Rc\xFF\xFF\xFF\xFF\x16a\x01\0RFa\x01 Ra\0\x93a\0\xB7V[a\x01@R`\x01`\x01`\xA0\x1B\x03\x16a\x01`Ra\0\xACa\x01bV[PPPPPPa\x02\xC0V[_a\x01 QF\x14a\x01ZWP`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[Pa\x01@Q\x90V[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x14a\x02\x1CW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x022W__\xFD[PV[______`\xC0\x87\x89\x03\x12\x15a\x02JW__\xFD[\x86Qa\x02U\x81a\x02\x1EV[` \x88\x01Q\x90\x96Pa\x02f\x81a\x02\x1EV[`@\x88\x01Q\x90\x95Pa\x02w\x81a\x02\x1EV[``\x88\x01Q\x90\x94Pa\x02\x88\x81a\x02\x1EV[`\x80\x88\x01Q\x90\x93Pa\x02\x99\x81a\x02\x1EV[`\xA0\x88\x01Q\x90\x92Pc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\xB2W__\xFD[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\\\xFCa\x03\xB5_9_\x81\x81a\x03\xF2\x01Ra4m\x01R_a@w\x01R_a?\xB7\x01R_\x81\x81a\x07A\x01R\x81\x81a5\xF8\x01Ra:\xC7\x01R_\x81\x81a\x07\x91\x01R\x81\x81a\x0C\xE7\x01R\x81\x81a\x0E\x98\x01R\x81\x81a\x17p\x01R\x81\x81a\x1B\xF7\x01R\x81\x81a$\xE7\x01R\x81\x81a)\x98\x01RaB\x8A\x01R_\x81\x81a\x04\x19\x01R\x81\x81a\x0E\x1E\x01R\x81\x81a\x16\xD7\x01R\x81\x81a\x195\x01R\x81\x81a2V\x01Ra?w\x01R_\x81\x81a\x03O\x01R\x81\x81a\r\xEC\x01R\x81\x81a\x18\x89\x01R\x81\x81a%\xD4\x01Ra?Q\x01R_\x81\x81a\x05\x94\x01R\x81\x81a\x0B\xDF\x01R\x81\x81a\x0F\xB8\x01Ra(\n\x01Ra\\\xFC_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xB1W_5`\xE0\x1C\x80cw\x8EU\xF3\x11a\x01{W\x80c\xBF\xAE?\xD2\x11a\0\xE4W\x80c\xE4\xCC?\x90\x11a\0\x9EW\x80c\xF0\xE0\xE6v\x11a\0yW\x80c\xF0\xE0\xE6v\x14a\x08\x12W\x80c\xF2\xFD\xE3\x8B\x14a\x082W\x80c\xF6\x98\xDA%\x14a\x08EW\x80c\xFA\xBC\x1C\xBC\x14a\x08MW__\xFD[\x80c\xE4\xCC?\x90\x14a\x07\xD9W\x80c\xEEt\x93\x7F\x14a\x07\xECW\x80c\xEE\xA9\x06K\x14a\x07\xFFW__\xFD[\x80c\xBF\xAE?\xD2\x14a\x07$W\x80c\xC4H\xFE\xB8\x14a\x077W\x80c\xC9x\xF7\xAC\x14a\x07kW\x80c\xCA\x8A\xA7\xC7\x14a\x07\x8CW\x80c\xCDm\xC6\x87\x14a\x07\xB3W\x80c\xDA\x8B\xE8d\x14a\x07\xC6W__\xFD[\x80c\x945\xBBC\x11a\x015W\x80c\x945\xBBC\x14a\x06\x02W\x80c\x99\xF57\x1B\x14a\x06\x15W\x80c\xA1x\x84\x84\x14a\x06\xA3W\x80c\xA3:43\x14a\x06\xC2W\x80c\xB7\xF0n\xBE\x14a\x06\xD5W\x80c\xBBE\xFE\xF2\x14a\x06\xF7W__\xFD[\x80cw\x8EU\xF3\x14a\x05RW\x80cx)n\xC5\x14a\x05|W\x80c\x88o\x11\x95\x14a\x05\x8FW\x80c\x8D\xA5\xCB[\x14a\x05\xB6W\x80c\x90\x04\x13G\x14a\x05\xC7W\x80c\x91\x04\xC3\x19\x14a\x05\xE7W__\xFD[\x80cY\\jg\x11a\x02\x1DW\x80c`\xA0\xD1\xCE\x11a\x01\xD7W\x80c`\xA0\xD1\xCE\x14a\x04\xC8W\x80ce\xDA\x12d\x14a\x04\xDBW\x80cf\xD5\xBA\x93\x14a\x05\x03W\x80cmp\xF7\xAE\x14a\x05$W\x80cn\x17DH\x14a\x057W\x80cqP\x18\xA6\x14a\x05JW__\xFD[\x80cY\\jg\x14a\x04NW\x80cY{6\xDA\x14a\x04VW\x80cZ\xC8j\xB7\x14a\x04iW\x80c\\\x97Z\xBB\x14a\x04\x8CW\x80c]\xD6\x85y\x14a\x04\x94W\x80c_H\xE6g\x14a\x04\xB5W__\xFD[\x80c<e\x1C\xF2\x11a\x02nW\x80c<e\x1C\xF2\x14a\x03\x89W\x80c<\xDE\xB5\xE0\x14a\x03\x9CW\x80c>(9\x1D\x14a\x03\xCAW\x80cFW\xE2j\x14a\x03\xEDW\x80cFe\xBC\xDA\x14a\x04\x14W\x80cT\xB7\xC9l\x14a\x04;W__\xFD[\x80c\x04\xA4\xF9y\x14a\x02\xB5W\x80c\x0B\x9FHz\x14a\x02\xEFW\x80c\r\xD8\xDD\x02\x14a\x03\x02W\x80c\x13d9\xDD\x14a\x03\"W\x80c*\xA6\xD8\x88\x14a\x037W\x80c9\xB7\x0E8\x14a\x03JW[__\xFD[a\x02\xDC\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xDCa\x02\xFD6`\x04aL\x1CV[a\x08`V[a\x03\x15a\x03\x106`\x04aL\xB3V[a\x08\xE8V[`@Qa\x02\xE6\x91\x90aL\xF1V[a\x035a\x0306`\x04aM(V[a\x0B\xCAV[\0[a\x035a\x03E6`\x04aM\x8FV[a\x0C\x9FV[a\x03q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xE6V[a\x035a\x03\x976`\x04aM\xEDV[a\r\xE1V[a\x03qa\x03\xAA6`\x04aN0V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x03\xDDa\x03\xD86`\x04aN0V[a\x0F(V[`@Q\x90\x15\x15\x81R` \x01a\x02\xE6V[a\x03q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x035a\x04I6`\x04aNKV[a\x0FGV[a\x035a\x0F\xA3V[a\x02\xDCa\x04d6`\x04aP\xBCV[a\x10RV[a\x03\xDDa\x04w6`\x04aP\xEDV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x02\xDCV[a\x04\xA7a\x04\xA26`\x04aN0V[a\x10\x81V[`@Qa\x02\xE6\x92\x91\x90aRLV[a\x035a\x04\xC36`\x04aR\xB9V[a\x148V[a\x035a\x04\xD66`\x04aS>V[a\x16\xCCV[a\x03qa\x04\xE96`\x04aN0V[`\x9A` R_\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05\x16a\x05\x116`\x04aN0V[a\x18aV[`@Qa\x02\xE6\x92\x91\x90aS}V[a\x03\xDDa\x0526`\x04aN0V[a\x1BaV[a\x02\xDCa\x05E6`\x04aNKV[a\x1B\x99V[a\x035a\x1C\x9FV[a\x02\xDCa\x05`6`\x04aNKV[`\x98` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x035a\x05\x8A6`\x04aS\xA1V[a\x1C\xB0V[a\x03q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03qV[a\x05\xDAa\x05\xD56`\x04aS\xF1V[a\x1D8V[`@Qa\x02\xE6\x91\x90aT=V[a\x03qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x035a\x06\x106`\x04aTOV[a\x1E\x0EV[a\x06ea\x06#6`\x04aM(V[`\xA4` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x84\x16\x93\x90\x91\x16\x91\x90c\xFF\xFF\xFF\xFF\x16\x85V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x81R\x94\x86\x16` \x86\x01R\x92\x90\x94\x16\x91\x83\x01\x91\x90\x91R``\x82\x01Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x80\x82\x01R`\xA0\x01a\x02\xE6V[a\x02\xDCa\x06\xB16`\x04aN0V[`\x9F` R_\x90\x81R`@\x90 T\x81V[a\x03\x15a\x06\xD06`\x04aT\xEBV[a\x1E\xC4V[a\x03\xDDa\x06\xE36`\x04aM(V[`\x9E` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\xDDa\x07\x056`\x04aU\xD2V[`\x9C` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x02\xDCa\x0726`\x04aNKV[a\x1FaV[`@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x02\xE6V[a\x07~a\x07y6`\x04aS\xF1V[a\x1F\x9DV[`@Qa\x02\xE6\x92\x91\x90aU\xFCV[a\x03q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x035a\x07\xC16`\x04aU\xD2V[a\"*V[a\x03\x15a\x07\xD46`\x04aN0V[a#EV[a\x035a\x07\xE76`\x04aV\x1BV[a$\x86V[a\x035a\x07\xFA6`\x04aV\x99V[a$\xDCV[a\x035a\x08\r6`\x04aT\xEBV[a&yV[a\x08%a\x08 6`\x04aV\xE7V[a&\xDCV[`@Qa\x02\xE6\x91\x90aW\x94V[a\x035a\x08@6`\x04aN0V[a'\x81V[a\x02\xDCa'\xFAV[a\x035a\x08[6`\x04aM(V[a(\x08V[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x92\x82\x01\x92\x90\x92R\x81\x87\x16``\x82\x01R\x90\x85\x16`\x80\x82\x01R`\xA0\x81\x01\x83\x90R`\xC0\x81\x01\x82\x90R_\x90a\x08\xDE\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a)\x1FV[\x96\x95PPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x03a\t\x14W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\t-Wa\t-aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\tVW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P3_\x90\x81R`\x9A` R`@\x81 T\x91\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90[\x85\x81\x10\x15a\x0B\xBFW\x86\x86\x82\x81\x81\x10a\t\x90Wa\t\x90aW\xA6V[\x90P` \x02\x81\x01\x90a\t\xA2\x91\x90aW\xBAV[a\t\xB0\x90` \x81\x01\x90aW\xD8V[\x90P\x87\x87\x83\x81\x81\x10a\t\xC4Wa\t\xC4aW\xA6V[\x90P` \x02\x81\x01\x90a\t\xD6\x91\x90aW\xBAV[a\t\xE0\x90\x80aW\xD8V[\x90P\x14a\n\0W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3\x87\x87\x83\x81\x81\x10a\n\x13Wa\n\x13aW\xA6V[\x90P` \x02\x81\x01\x90a\n%\x91\x90aW\xBAV[a\n6\x90``\x81\x01\x90`@\x01aN0V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\n]W`@Qc0\xC4qi`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\n\xC73\x84\x8A\x8A\x86\x81\x81\x10a\nuWa\nuaW\xA6V[\x90P` \x02\x81\x01\x90a\n\x87\x91\x90aW\xBAV[a\n\x91\x90\x80aW\xD8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa)M\x92PPPV[\x90Pa\x0B\x993\x84\x8A\x8A\x86\x81\x81\x10a\n\xE0Wa\n\xE0aW\xA6V[\x90P` \x02\x81\x01\x90a\n\xF2\x91\x90aW\xBAV[a\n\xFC\x90\x80aW\xD8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x88\x90P\x81\x81\x10a\x0BAWa\x0BAaW\xA6V[\x90P` \x02\x81\x01\x90a\x0BS\x91\x90aW\xBAV[a\x0Ba\x90` \x81\x01\x90aW\xD8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa*\x94\x91PPV[\x84\x83\x81Q\x81\x10a\x0B\xABWa\x0B\xABaW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\tvV[P\x90\x95\x94PPPPPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CP\x91\x90aX\x1DV[a\x0CmW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x81\x14a\x0C\x92W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\x9B\x82a0mV[PPV[a\x0C\xA83a\x0F(V[\x15a\x0C\xC6W`@Qc;\xF2\xB5\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc+bA\xF3`\xE1\x1B\x81R3`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cV\xC4\x83\xE6\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r0W__\xFD[PZ\xF1\x15\x80\x15a\rBW=__>=_\xFD[PPPPa\rP3\x85a0\xAAV[a\rZ33a1\x0CV[`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R3\x90\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90` \x01`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\r\xD3\x92\x91\x90aX8V[`@Q\x80\x91\x03\x90\xA2PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0E@WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x0E]W`@Qc\x04R\x06\xA5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x90\x81R`\x9A` R`@\x80\x82 T\x90Qc\x15&g\xD9`\xE3\x1B\x81R\x90\x83\x16`\x04\x82\x01\x81\x90R\x86\x84\x16`$\x83\x01R\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA93>\xC8\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x01\x91\x90aXfV[\x90P_a\x0F\x0F\x87\x87\x84a2\x0FV[\x90Pa\x0F\x1F\x83\x88\x88\x88\x88\x86a2\xF1V[PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[\x81a\x0FQ\x81a4/V[a\x0FnW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Fw\x83a\x1BaV[a\x0F\x94W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x9E\x83\x83a0\xAAV[PPPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x05W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10)\x91\x90aX\x1DV[a\x10FW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10P_\x19a0mV[V[_\x81`@Q` \x01a\x10d\x91\x90aX\x81V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xA3` R`@\x81 ``\x91\x82\x91a\x10\xA6\x90a4\xD9V[\x80Q\x90\x91P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xC3Wa\x10\xC3aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xFCW\x81` \x01[a\x10\xE9aJ\xD8V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10\xE1W\x90P[P\x93P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x17Wa\x11\x17aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11JW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x115W\x90P[P`\x01`\x01`\xA0\x1B\x03\x80\x87\x16_\x90\x81R`\x9A` R`@\x81 T\x92\x95P\x91\x16\x90[\x82\x81\x10\x15a\x14/W`\xA4_\x85\x83\x81Q\x81\x10a\x11\x88Wa\x11\x88aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x83\x01T\x81\x16\x82\x86\x01R`\x02\x83\x01T\x16\x81\x84\x01R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01Tc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\x05\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x91\x94\x92\x93`\xA0\x86\x01\x93\x90\x92\x90\x83\x01\x82\x82\x80\x15a\x12BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x12$W[PPPPP\x81R` \x01`\x06\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x12\x98W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x12\x84W[PPPPP\x81RPP\x86\x82\x81Q\x81\x10a\x12\xB3Wa\x12\xB3aW\xA6V[` \x02` \x01\x01\x81\x90RP\x85\x81\x81Q\x81\x10a\x12\xD0Wa\x12\xD0aW\xA6V[` \x02` \x01\x01Q`\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xF4Wa\x12\xF4aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\x1DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x85\x82\x81Q\x81\x10a\x130Wa\x130aW\xA6V[` \x02` \x01\x01\x81\x90RP_a\x13d\x88\x84\x89\x85\x81Q\x81\x10a\x13SWa\x13SaW\xA6V[` \x02` \x01\x01Q`\xA0\x01Qa)MV[\x90P_[\x87\x83\x81Q\x81\x10a\x13zWa\x13zaW\xA6V[` \x02` \x01\x01Q`\xA0\x01QQ\x81\x10\x15a\x14%Wa\x13\xE7\x88\x84\x81Q\x81\x10a\x13\xA3Wa\x13\xA3aW\xA6V[` \x02` \x01\x01Q`\xC0\x01Q\x82\x81Q\x81\x10a\x13\xC0Wa\x13\xC0aW\xA6V[` \x02` \x01\x01Q\x83\x85\x81Q\x81\x10a\x13\xDAWa\x13\xDAaW\xA6V[` \x02` \x01\x01Qa4\xE5V[\x87\x84\x81Q\x81\x10a\x13\xF9Wa\x13\xF9aW\xA6V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x14\x12Wa\x14\x12aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13hV[PP`\x01\x01a\x11kV[PPPP\x91P\x91V[`fT`\x02\x90`\x04\x90\x81\x16\x03a\x14aW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14ia4\xF0V[3_\x90\x81R`\xA3` R`@\x81 \x90a\x14\x81\x82a5IV[\x90P\x80\x84\x11a\x14\x90W\x83a\x14\x92V[\x80[\x93P_\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xADWa\x14\xADaN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xE6W\x81` \x01[a\x14\xD3aJ\xD8V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x14\xCBW\x90P[P\x90P_[\x81Q\x81\x10\x15a\x165W`\xA4_a\x15\x01\x86\x84a5RV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x83\x01T\x81\x16\x82\x86\x01R`\x02\x83\x01T\x16\x81\x84\x01R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01Tc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\x05\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x91\x94\x92\x93`\xA0\x86\x01\x93\x90\x92\x90\x83\x01\x82\x82\x80\x15a\x15\xB1W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x15\x93W[PPPPP\x81R` \x01`\x06\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x16\x07W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x15\xF3W[PPPPP\x81RPP\x82\x82\x81Q\x81\x10a\x16\"Wa\x16\"aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x14\xEBV[P_[\x81Q\x81\x10\x15a\x16\xB6Wa\x16\xAE\x82\x82\x81Q\x81\x10a\x16VWa\x16VaW\xA6V[` \x02` \x01\x01Q\x8B\x8B\x84\x81\x81\x10a\x16pWa\x16paW\xA6V[\x90P` \x02\x81\x01\x90a\x16\x82\x91\x90aW\xD8V[\x8B\x8B\x86\x81\x81\x10a\x16\x94Wa\x16\x94aW\xA6V[\x90P` \x02\x01` \x81\x01\x90a\x16\xA9\x91\x90aX\x93V[a5]V[`\x01\x01a\x168V[PPPPa\x16\xC4`\x01`\xC9UV[PPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17\x15W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\x1E\x83a\x0F(V[\x15a\x0F\x9EW`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x9A` R`@\x80\x82 T\x90Qc\x15&g\xD9`\xE3\x1B\x81R\x90\x83\x16`\x04\x82\x01\x81\x90Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x83\x01R\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA93>\xC8\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xD9\x91\x90aXfV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x84R\x82R\x80\x83 \x81Q\x92\x83\x01\x90\x91RT\x81R\x91\x92Pa\x18?\x86a\x187`\x01`\x01`@\x1B\x03\x80\x87\x16\x90\x89\x16a9\xD7V[\x84\x91\x90a9\xEBV[\x90Pa\x0F\x1F\x84\x88s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x84a:\x11V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91_\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x94\xF6I\xDD\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xCDW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18\xF4\x91\x90\x81\x01\x90aY\tV[`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x83\x01R\x92\x94P\x90\x92P_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFE$:\x17\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19zW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x9E\x91\x90aY\xC4V[\x90P\x80_\x03a\x19\xB2WP\x90\x94\x90\x93P\x91PPV[_\x83Q`\x01a\x19\xC1\x91\x90aY\xEFV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xD8Wa\x19\xD8aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x01W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84Q`\x01a\x1A\x13\x91\x90aY\xEFV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A*Wa\x1A*aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1ASW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82\x86Q\x81Q\x81\x10a\x1A~Wa\x1A~aW\xA6V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x82\x81\x86Q\x81Q\x81\x10a\x1A\xB2Wa\x1A\xB2aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x85Q\x81\x10\x15a\x1BSW\x85\x81\x81Q\x81\x10a\x1A\xDAWa\x1A\xDAaW\xA6V[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a\x1A\xF4Wa\x1A\xF4aW\xA6V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81\x81Q\x81\x10a\x1B&Wa\x1B&aW\xA6V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1B@Wa\x1B@aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\xBFV[P\x90\x97\x90\x96P\x94PPPPPV[_`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x1B\x93WP`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x14[\x92\x91PPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x82\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x82\x81_\x81Q\x81\x10a\x1B\xD0Wa\x1B\xD0aW\xA6V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`@QcTz\xFB\x87`\xE0\x1B\x81R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cTz\xFB\x87\x90a\x1C.\x90\x88\x90\x86\x90`\x04\x01aZ\x02V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CHW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1Co\x91\x90\x81\x01\x90aZ%V[_\x81Q\x81\x10a\x1C\x80Wa\x1C\x80aW\xA6V[` \x02` \x01\x01Q\x90Pa\x1C\x96\x85\x85\x83_a:\x8BV[\x95\x94PPPPPV[a\x1C\xA7a;<V[a\x10P_a;\x96V[\x82a\x1C\xBA\x81a4/V[a\x1C\xD7W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\xE0\x84a\x1BaV[a\x1C\xFDW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\r\xD3\x92\x91\x90aX8V[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DTWa\x1DTaN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D}W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1E\x06W`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x98` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x1D\xB9Wa\x1D\xB9aW\xA6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ T\x82\x82\x81Q\x81\x10a\x1D\xF3Wa\x1D\xF3aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1D\x82V[P\x93\x92PPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a\x1E7W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E?a4\xF0V[\x85_[\x81\x81\x10\x15a\x1E\xB8Wa\x1E\xB0\x89\x89\x83\x81\x81\x10a\x1E_Wa\x1E_aW\xA6V[\x90P` \x02\x81\x01\x90a\x1Eq\x91\x90aZ\xB4V[a\x1Ez\x90aZ\xC8V[\x88\x88\x84\x81\x81\x10a\x1E\x8CWa\x1E\x8CaW\xA6V[\x90P` \x02\x81\x01\x90a\x1E\x9E\x91\x90aW\xD8V[\x88\x88\x86\x81\x81\x10a\x16\x94Wa\x16\x94aW\xA6V[`\x01\x01a\x1EBV[PPa\x0F\x1F`\x01`\xC9UV[``a\x1E\xCF3a\x0F(V[a\x1E\xECW`@Qc\xA5\xC7\xC4E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xF53a\x1BaV[\x15a\x1F\x13W`@Qc\x11\xCA35`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\x1C\x84a\x1BaV[a\x1F9W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1FB3a;\xE7V[\x90Pa\x1FP3\x85\x85\x85a>FV[a\x1FZ3\x85a1\x0CV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x83Q\x91\x82\x01\x90\x93R\x91T\x82R\x90a\x1FZ\x90a?\x0BV[``\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xB9Wa\x1F\xB9aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\xE2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xFEWa\x1F\xFEaN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a 'W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x9A` R`@\x81 T\x92\x93P\x91\x16\x90a R\x86\x83\x87a)MV[\x90P_[\x85Q\x81\x10\x15a\"\x1FW_a \x82\x87\x83\x81Q\x81\x10a uWa uaW\xA6V[` \x02` \x01\x01Qa?*V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xFE$:\x17\x89\x89\x85\x81Q\x81\x10a \xA6Wa \xA6aW\xA6V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \xE0\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x1F\x91\x90aY\xC4V[\x85\x83\x81Q\x81\x10a!1Wa!1aW\xA6V[` \x02` \x01\x01\x81\x81RPP_`\xA2_\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x89\x85\x81Q\x81\x10a!tWa!taW\xA6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90Pa!\xF8\x86\x84\x81Q\x81\x10a!\xC6Wa!\xC6aW\xA6V[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a!\xE0Wa!\xE0aW\xA6V[` \x02` \x01\x01Q\x83a9\xEB\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x84\x81Q\x81\x10a\"\nWa\"\naW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a VV[PPP[\x92P\x92\x90PV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\"HWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\"aWP0;\x15\x80\x15a\"aWP_T`\xFF\x16`\x01\x14[a\"\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\"\xEAW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\"\xF3\x82a0mV[a\"\xFC\x83a;\x96V[\x80\x15a\x0F\x9EW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[``a#P\x82a\x0F(V[a#mW`@Qc\xA5\xC7\xC4E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#v\x82a\x1BaV[\x15a#\x94W`@Qc\x11\xCA35`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a#\xBBW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x903\x14\x80a#\xE9WPa#\xE9\x81a4/V[\x80a$\x0FWP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a$,W`@Qc\x1EI\x9A#`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x84\x16\x14a$}W\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[a\x1FZ\x83a;\xE7V[`fT`\x02\x90`\x04\x90\x81\x16\x03a$\xAFW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$\xB7a4\xF0V[a$\xCBa$\xC3\x86aZ\xC8V[\x85\x85\x85a5]V[a$\xD5`\x01`\xC9UV[PPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a%%W`@Qc#\xD8q\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 Ta%c\x90`\x01`\x01`@\x1B\x03\x80\x86\x16\x90\x85\x16a?\x9CV[\x90P_a%r\x86\x86\x86\x86a:\x8BV[a%|\x90\x83aY\xEFV[\x90Pa%\x8A\x86_\x87\x85a:\x11V[`\x01`\x01`\xA0\x1B\x03\x85\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a\x16\xC4W`@Qc;\x9E\x9F\x01`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEEz|\x04\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&\x15W__\xFD[PZ\xF1\x15\x80\x15a&'W=__>=_\xFD[PP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R` \x82\x01\x86\x90R\x8A\x16\x93P\x7F\xEF\xF6\xAA\xB2\xBC?|d\x88\x96\xE1R.\xAEq\xD6\xC2.;\x0E!\x82\x06\xB3\xF4\n\xF0\xE4\xD2\x04qk\x92P\x01`@Q\x80\x91\x03\x90\xA2PPPPPPV[a&\x823a\x0F(V[\x15a&\xA0W`@Qc;\xF2\xB5\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a&\xA9\x83a\x1BaV[a&\xC6W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a&\xD23\x84\x84\x84a>FV[a\x0F\x9E3\x84a1\x0CV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xF8Wa&\xF8aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'+W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a'\x16W\x90P[P\x90P_[\x84Q\x81\x10\x15a\x1E\x06Wa'\\\x85\x82\x81Q\x81\x10a'NWa'NaW\xA6V[` \x02` \x01\x01Q\x85a\x1D8V[\x82\x82\x81Q\x81\x10a'nWa'naW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a'0V[a'\x89a;<V[`\x01`\x01`\xA0\x1B\x03\x81\x16a'\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\"\xC0V[a'\xF7\x81a;\x96V[PV[_a(\x03a?\xB4V[\x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(dW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\x88\x91\x90aZ\xD3V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a(\xB9W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x80\x19\x82\x19\x81\x16\x14a(\xE0W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPV[_a)(a?\xB4V[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01a\x10dV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a)iWa)iaN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\x92W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cTz\xFB\x87\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\xE4\x92\x91\x90aZ\x02V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xFEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra*%\x91\x90\x81\x01\x90aZ%V[\x90P_[\x84Q\x81\x10\x15a\x0B\xBFWa*o\x87\x86\x83\x81Q\x81\x10a*HWa*HaW\xA6V[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a*bWa*baW\xA6V[` \x02` \x01\x01Qa2\x0FV[\x83\x82\x81Q\x81\x10a*\x81Wa*\x81aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a*)V[_`\x01`\x01`\xA0\x1B\x03\x86\x16a*\xBCW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q_\x03a*\xDDW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xF7Wa*\xF7aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+ W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a+=Wa+=aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+fW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x86Q\x81\x10\x15a.\xA0W_a+\x8A\x88\x83\x81Q\x81\x10a uWa uaW\xA6V[\x90P_`\xA2_\x8C`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x8A\x85\x81Q\x81\x10a+\xC3Wa+\xC3aW\xA6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16c\xFE$:\x17\x8C\x8B\x86\x81Q\x81\x10a,\"Wa,\"aW\xA6V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\\\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,wW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x9B\x91\x90aY\xC4V[\x88\x84\x81Q\x81\x10a,\xADWa,\xADaW\xA6V[` \x02` \x01\x01Q\x11\x15a,\xD4W`@Qc\xF0 \xE5\xB9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a-\x03\x88\x84\x81Q\x81\x10a,\xE9Wa,\xE9aW\xA6V[` \x02` \x01\x01Q\x88\x85\x81Q\x81\x10a!\xE0Wa!\xE0aW\xA6V[\x84\x84\x81Q\x81\x10a-\x15Wa-\x15aW\xA6V[` \x02` \x01\x01\x81\x81RPPa-]\x84\x84\x81Q\x81\x10a-6Wa-6aW\xA6V[` \x02` \x01\x01Q\x88\x85\x81Q\x81\x10a-PWa-PaW\xA6V[` \x02` \x01\x01Qa@\x99V[\x85\x84\x81Q\x81\x10a-oWa-oaW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x15a.\x04Wa-\xC6\x8A\x8A\x85\x81Q\x81\x10a-\x9FWa-\x9FaW\xA6V[` \x02` \x01\x01Q\x87\x86\x81Q\x81\x10a-\xB9Wa-\xB9aW\xA6V[` \x02` \x01\x01Qa@\xB2V[a.\x04\x8A\x8C\x8B\x86\x81Q\x81\x10a-\xDDWa-\xDDaW\xA6V[` \x02` \x01\x01Q\x87\x87\x81Q\x81\x10a-\xF7Wa-\xF7aW\xA6V[` \x02` \x01\x01Qa:\x11V[\x81`\x01`\x01`\xA0\x1B\x03\x16crJ\xF4#\x8C\x8B\x86\x81Q\x81\x10a.&Wa.&aW\xA6V[` \x02` \x01\x01Q\x8B\x87\x81Q\x81\x10a.@Wa.@aW\xA6V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.f\x93\x92\x91\x90aZ\xEEV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a.}W__\xFD[PZ\xF1\x15\x80\x15a.\x8FW=__>=_\xFD[PPPPPP\x80`\x01\x01\x90Pa+kV[P`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a.\xC7\x83a[\x12V[\x91\x90PUP_`@Q\x80`\xE0\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89\x81R` \x01\x85\x81RP\x90P_a/-\x82a\x10RV[_\x81\x81R`\x9E` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x19\x90\x91\x16\x81\x17\x90\x91U`\xA4\x83R\x92\x81\x90 \x86Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x83U\x88\x85\x01Q\x95\x83\x01\x80T\x82\x16\x96\x83\x16\x96\x90\x96\x17\x90\x95U\x91\x87\x01Q`\x02\x82\x01\x80T\x90\x95\x16\x92\x16\x91\x90\x91\x17\x90\x92U``\x85\x01Q`\x03\x83\x01U`\x80\x85\x01Q`\x04\x83\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\xA0\x85\x01Q\x80Q\x93\x94P\x85\x93a/\xE3\x92`\x05\x85\x01\x92\x01\x90aK1V[P`\xC0\x82\x01Q\x80Qa/\xFF\x91`\x06\x84\x01\x91` \x90\x91\x01\x90aK\x94V[PPP`\x01`\x01`\xA0\x1B\x03\x8B\x16_\x90\x81R`\xA3` R`@\x90 a0#\x90\x82aA@V[P\x7F&\xB2\xAA\xE2e\x16\xE8q\x9E\xF5\x0E\xA2\xF6\x83\x1A.\xFB\xD4\xE3}\xCC\xDF\x0Fi6\xB2{\xC0\x8Ey>0\x81\x83\x86`@Qa0W\x93\x92\x91\x90a[*V[`@Q\x80\x91\x03\x90\xA1\x9A\x99PPPPPPPPPPV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x99` \x90\x81R`@\x91\x82\x90 `\x01\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x86\x16\x94\x85\x17\x90U\x90Q\x92\x83R\x90\x91\x7Fw;T\xC0Muo\xCC^g\x81\x11\xF7\xD70\xDE;\xE9\x81\x92\0\x07\x99\xEE\xE3\xD67\x16\x05Z\x87\xC6\x91\x01a)\x13V[`fT_\x90`\x01\x90\x81\x16\x03a14W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x87\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3__a1\x91\x85a\x18aV[\x91P\x91P_a1\xA1\x86\x86\x85a)MV[\x90P_[\x83Q\x81\x10\x15a\x0F\x1FWa2\x07\x86\x88\x86\x84\x81Q\x81\x10a1\xC5Wa1\xC5aW\xA6V[` \x02` \x01\x01Q_\x87\x86\x81Q\x81\x10a1\xE0Wa1\xE0aW\xA6V[` \x02` \x01\x01Q\x87\x87\x81Q\x81\x10a1\xFAWa1\xFAaW\xA6V[` \x02` \x01\x01Qa2\xF1V[`\x01\x01a1\xA5V[_s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xBF\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01a2\xE1W`@Qc\xA3\xD7^\t`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA3\xD7^\t\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xC1\x91\x90aXfV[\x90Pa2\xD9`\x01`\x01`@\x1B\x03\x84\x81\x16\x90\x83\x16a9\xD7V[\x91PPa\x1FZV[P`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[\x80_\x03a3\x11W`@Qc\n3\xBCi`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R a3A\x81\x85\x85\x85aAKV[`@\x80Q` \x81\x01\x90\x91R\x81T\x81R\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x90\x87\x90\x87\x90a3\x7F\x90a?\x0BV[`@Qa3\x8E\x93\x92\x91\x90aZ\xEEV[`@Q\x80\x91\x03\x90\xA1a3\x9F\x86a\x0F(V[\x15a\x0F\x1FW`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 \x80T\x85\x92\x90a3\xDA\x90\x84\x90aY\xEFV[\x92PP\x81\x90UP\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x87\x87\x86`@Qa4\x1E\x93\x92\x91\x90aZ\xEEV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a4\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x93\x91\x90aX\x1DV[``_a\x1FZ\x83aA\xC0V[_a\x1FZ\x83\x83a9\xD7V[`\x02`\xC9T\x03a5BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\"\xC0V[`\x02`\xC9UV[_a\x1B\x93\x82T\x90V[_a\x1FZ\x83\x83aB\x19V[`\xA0\x84\x01QQ\x82\x14a5\x82W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a5\xB8W`@Qc\x16\x11\r5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a5\xC2\x85a\x10RV[_\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a5\xF3W`@Qc\x87\xC9\xD2\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87`\x80\x01Qa6&\x91\x90a[TV[\x90PCc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11\x15a6UW`@Qcx\xF6z\xE1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a6l\x87_\x01Q\x88` \x01Q\x89`\xA0\x01Q\x84aB?V[\x87Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x81 T\x8AQ`\xA0\x8C\x01Q\x94\x96P\x92\x16\x93P\x91a6\xA2\x91\x90\x84\x90a)MV[\x90P_[\x88`\xA0\x01QQ\x81\x10\x15a8\xF5W_a6\xCD\x8A`\xA0\x01Q\x83\x81Q\x81\x10a uWa uaW\xA6V[\x90P_a7\x03\x8B`\xC0\x01Q\x84\x81Q\x81\x10a6\xE9Wa6\xE9aW\xA6V[` \x02` \x01\x01Q\x87\x85\x81Q\x81\x10a\x13\xDAWa\x13\xDAaW\xA6V[\x90P\x87\x15a7\xD3W\x81`\x01`\x01`\xA0\x1B\x03\x16c.\xAEA\x8C\x8C_\x01Q\x8D`\xA0\x01Q\x86\x81Q\x81\x10a74Wa74aW\xA6V[` \x02` \x01\x01Q\x8D\x8D\x88\x81\x81\x10a7NWa7NaW\xA6V[\x90P` \x02\x01` \x81\x01\x90a7c\x91\x90aN0V[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x90\x91\x16`D\x82\x01R`d\x81\x01\x84\x90R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a7\xB8W__\xFD[PZ\xF1\x15\x80\x15a7\xCAW=__>=_\xFD[PPPPa8\xEBV[__\x83`\x01`\x01`\xA0\x1B\x03\x16c\xC4b>\xA1\x8E_\x01Q\x8F`\xA0\x01Q\x88\x81Q\x81\x10a7\xFEWa7\xFEaW\xA6V[` \x02` \x01\x01Q\x8F\x8F\x8A\x81\x81\x10a8\x18Wa8\x18aW\xA6V[\x90P` \x02\x01` \x81\x01\x90a8-\x91\x90aN0V[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x90\x91\x16`D\x82\x01R`d\x81\x01\x86\x90R`\x84\x01`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a8\x86W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xAA\x91\x90a[pV[\x91P\x91Pa8\xE8\x87\x8E_\x01Q\x8F`\xA0\x01Q\x88\x81Q\x81\x10a8\xCCWa8\xCCaW\xA6V[` \x02` \x01\x01Q\x85\x85\x8B\x8B\x81Q\x81\x10a1\xFAWa1\xFAaW\xA6V[PP[PP`\x01\x01a6\xA6V[P\x87Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\xA3` R`@\x90 a9\x18\x90\x85aCmV[P_\x84\x81R`\xA4` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x82U`\x01\x82\x01\x80T\x82\x16\x90U`\x02\x82\x01\x80T\x90\x91\x16\x90U`\x03\x81\x01\x82\x90U`\x04\x81\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90U\x90a9o`\x05\x83\x01\x82aK\xCDV[a9|`\x06\x83\x01_aK\xCDV[PP_\x84\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x7F\x1F@@\x08\x89'N\xD0{$\x84^PT\xA8z\x0C\xAB\x96\x9E\xB1'z\xAF\xE6\x1A\xE3R\xE7\xC3*\0\x90a9\xC5\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_a\x1FZ\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aCxV[_a:\t\x82a:\x03a9\xFC\x87a?\x0BV[\x86\x90a9\xD7V[\x90a9\xD7V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a:G\x90\x84\x90a[\x92V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\r\xD3\x93\x92\x91\x90aZ\xEEV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x81\x90a:\xBC\x90aD]V[\x90P_a;\x16a:\xEC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ca[\xA5V[`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x8C\x16\x83R\x92\x90R \x90aDwV[\x90P_a;#\x82\x84a[\x92V[\x90Pa;0\x81\x87\x87aD\x93V[\x98\x97PPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\"\xC0V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x03a<\x13W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x90Q\x93\x16\x92\x83\x92\x91\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv\x91\xA3__a<r\x86a\x18aV[\x91P\x91P\x81Q_\x03a<\x86WPPPa>@V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a<\x9FWa<\x9FaN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\xC8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P_a<\xD7\x87\x85\x85a)MV[\x90P_[\x83Q\x81\x10\x15a>:W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x86\x84\x81Q\x81\x10a=[Wa=[aW\xA6V[` \x02` \x01\x01Q\x83_\x81Q\x81\x10a=uWa=uaW\xA6V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x85\x84\x81Q\x81\x10a=\xA7Wa=\xA7aW\xA6V[` \x02` \x01\x01Q\x82_\x81Q\x81\x10a=\xC1Wa=\xC1aW\xA6V[` \x02` \x01\x01\x81\x81RPP\x84\x84\x81Q\x81\x10a=\xDFWa=\xDFaW\xA6V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a=\xF9Wa=\xF9aW\xA6V[` \x02` \x01\x01\x81\x81RPPa>\x12\x8B\x89\x85\x85\x85a*\x94V[\x8A\x85\x81Q\x81\x10a>$Wa>$aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a<\xDBV[PPPPP[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80a>nWPa?\x05V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T`\xFF\x16\x15a>\xB2W`@Qc\rLL\x91`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x90\x91 \x80T`\xFF\x19\x16`\x01\x17\x90U\x83\x01Qa$\xD5\x90\x82\x90a>\xF9\x90\x88\x90\x88\x90\x84\x90\x88\x90a\x08`V[\x85Q` \x87\x01QaD\xB1V[PPPPV[\x80Q_\x90\x15a?\x1BW\x81Qa\x1B\x93V[g\r\xE0\xB6\xB3\xA7d\0\0\x92\x91PPV[_`\x01`\x01`\xA0\x1B\x03\x82\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a?uW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1B\x93V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91PPV[_a?\xAA\x84\x83\x85`\x01aE\x03V[a:\t\x90\x85a[\x92V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a@tWP`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[_\x81_\x03a@\xA8WP_a\x1B\x93V[a\x1FZ\x83\x83aERV[`\x01`\x01`\xA0\x1B\x03\x82\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a\x0F\x9EW`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 aA\x05\x90aD]V[\x90Pa?\x05CaA\x15\x84\x84aY\xEFV[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x91\x90aEfV[_a\x1FZ\x83\x83aEqV[\x82_\x03aAkWaAdg\r\xE0\xB6\xB3\xA7d\0\0\x82aERV[\x84Ua?\x05V[`@\x80Q` \x81\x01\x90\x91R\x84T\x81R_\x90aA\x87\x90\x85\x84a9\xEBV[\x90P_aA\x94\x84\x83aY\xEFV[\x90P_aA\xB5\x84aA\xAFaA\xA8\x88\x8AaY\xEFV[\x85\x90aERV[\x90aERV[\x87UPPPPPPPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aB\rW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aA\xF9W[PPPPP\x90P\x91\x90PV[_\x82_\x01\x82\x81T\x81\x10aB.WaB.aW\xA6V[\x90_R` _ \x01T\x90P\x92\x91PPV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15aB[WaB[aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aB\x84W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x94\xD7\xD0\x0C\x87\x87\x87`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aB\xD8\x93\x92\x91\x90a[\xC1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\xF2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@RaC\x19\x91\x90\x81\x01\x90aZ%V[\x90P_[\x85Q\x81\x10\x15aCaWaC<\x88\x87\x83\x81Q\x81\x10a*HWa*HaW\xA6V[\x83\x82\x81Q\x81\x10aCNWaCNaW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01aC\x1DV[P\x90\x96\x95PPPPPPV[_a\x1FZ\x83\x83aE\xBDV[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aC\xAFW\x83\x82\x81aC\xA5WaC\xA5a[\xFAV[\x04\x92PPPa\x1FZV[\x80\x84\x11aC\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\"\xC0V[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[_aDh\x82\x82aF\xA0V[`\x01`\x01`\xE0\x1B\x03\x16\x92\x91PPV[_aD\x83\x83\x83\x83aF\xE5V[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[_a:\taD\xA1\x83\x85a\\\x0EV[\x85\x90`\x01`\x01`@\x1B\x03\x16a9\xD7V[B\x81\x10\x15aD\xD2W`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aD\xE6`\x01`\x01`\xA0\x1B\x03\x85\x16\x84\x84aG.V[a?\x05W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__aE\x10\x86\x86\x86aCxV[\x90P`\x01\x83`\x02\x81\x11\x15aE&WaE&a\\-V[\x14\x80\x15aEBWP_\x84\x80aE=WaE=a[\xFAV[\x86\x88\t\x11[\x15a\x1C\x96Wa\x08\xDE`\x01\x82aY\xEFV[_a\x1FZ\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84aCxV[a\x0F\x9E\x83\x83\x83aG\x82V[_\x81\x81R`\x01\x83\x01` R`@\x81 TaE\xB6WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x1B\x93V[P_a\x1B\x93V[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aF\x97W_aE\xDF`\x01\x83a[\x92V[\x85T\x90\x91P_\x90aE\xF2\x90`\x01\x90a[\x92V[\x90P\x81\x81\x14aFQW_\x86_\x01\x82\x81T\x81\x10aF\x10WaF\x10aW\xA6V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10aF0WaF0aW\xA6V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aFbWaFba\\AV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x1B\x93V[_\x91PPa\x1B\x93V[\x81T_\x90\x80\x15aF\xDDWaF\xC6\x84aF\xB9`\x01\x84a[\x92V[_\x91\x82R` \x90\x91 \x01\x90V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a:\tV[P\x90\x92\x91PPV[\x82T_\x90\x81aF\xF6\x86\x86\x83\x85aH\x88V[\x90P\x80\x15aG$WaG\r\x86aF\xB9`\x01\x84a[\x92V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x08\xDEV[P\x91\x94\x93PPPPV[___aG;\x85\x85aH\xDBV[\x90\x92P\x90P_\x81`\x04\x81\x11\x15aGSWaGSa\\-V[\x14\x80\x15aGqWP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\x08\xDEWPa\x08\xDE\x86\x86\x86aI\x1AV[\x82T\x80\x15aH:W_aG\x9A\x85aF\xB9`\x01\x85a[\x92V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84Rd\x01\0\0\0\0\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x85\x16\x10\x15aG\xEDW`@Qc\x15\x1B\x8E?`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qc\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x03aH8W\x82aH\x0E\x86aF\xB9`\x01\x86a[\x92V[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[P[P`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x81R`\x01`\x01`\xE0\x1B\x03\x91\x82\x16` \x80\x83\x01\x91\x82R\x85T`\x01\x81\x01\x87U_\x96\x87R\x95 \x91Q\x90Q\x90\x92\x16d\x01\0\0\0\0\x02\x91\x90\x92\x16\x17\x91\x01UV[_[\x81\x83\x10\x15a\x1E\x06W_aH\x9D\x84\x84aJ\x01V[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15aH\xC7W\x80\x92PaH\xD5V[aH\xD2\x81`\x01aY\xEFV[\x93P[PaH\x8AV[__\x82Q`A\x03aI\x0FW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1AaI\x03\x87\x82\x85\x85aJ\x1BV[\x94P\x94PPPPa\"#V[P_\x90P`\x02a\"#V[___\x85`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x86\x86`@Q`$\x01aIB\x92\x91\x90a\\UV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90QaI\x80\x91\x90a\\\x91V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14aI\xB8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aI\xBDV[``\x91P[P\x91P\x91P\x81\x80\x15aI\xD1WP` \x81Q\x10\x15[\x80\x15a\x08\xDEWP\x80Qc\x0B\x13]?`\xE1\x1B\x90aI\xF6\x90\x83\x01` \x90\x81\x01\x90\x84\x01aY\xC4V[\x14\x96\x95PPPPPPV[_aJ\x0F`\x02\x84\x84\x18a\\\xA7V[a\x1FZ\x90\x84\x84\x16aY\xEFV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aJPWP_\x90P`\x03aJ\xCFV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aJ\xA1W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aJ\xC9W_`\x01\x92P\x92PPaJ\xCFV[\x91P_\x90P[\x94P\x94\x92PPPV[`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aK\x84W\x91` \x02\x82\x01[\x82\x81\x11\x15aK\x84W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90aKOV[PaK\x90\x92\x91PaK\xE4V[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aK\x84W\x91` \x02\x82\x01[\x82\x81\x11\x15aK\x84W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aK\xB2V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a'\xF7\x91\x90[[\x80\x82\x11\x15aK\x90W_\x81U`\x01\x01aK\xE5V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\xF7W__\xFD[\x805aL\x17\x81aK\xF8V[\x91\x90PV[_____`\xA0\x86\x88\x03\x12\x15aL0W__\xFD[\x855aL;\x81aK\xF8V[\x94P` \x86\x015aLK\x81aK\xF8V[\x93P`@\x86\x015aL[\x81aK\xF8V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[__\x83`\x1F\x84\x01\x12aL\x83W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x99W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\"#W__\xFD[__` \x83\x85\x03\x12\x15aL\xC4W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xD9W__\xFD[aL\xE5\x85\x82\x86\x01aLsV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x0B\xBFW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aM\nV[_` \x82\x84\x03\x12\x15aM8W__\xFD[P5\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14aL\x17W__\xFD[__\x83`\x1F\x84\x01\x12aMbW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aMxW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\"#W__\xFD[____``\x85\x87\x03\x12\x15aM\xA2W__\xFD[\x845aM\xAD\x81aK\xF8V[\x93PaM\xBB` \x86\x01aM?V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xD5W__\xFD[aM\xE1\x87\x82\x88\x01aMRV[\x95\x98\x94\x97P\x95PPPPV[____`\x80\x85\x87\x03\x12\x15aN\0W__\xFD[\x845aN\x0B\x81aK\xF8V[\x93P` \x85\x015aN\x1B\x81aK\xF8V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[_` \x82\x84\x03\x12\x15aN@W__\xFD[\x815a\x1FZ\x81aK\xF8V[__`@\x83\x85\x03\x12\x15aN\\W__\xFD[\x825aNg\x81aK\xF8V[\x91P` \x83\x015aNw\x81aK\xF8V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xB8WaN\xB8aN\x82V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xB8WaN\xB8aN\x82V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aO\x08WaO\x08aN\x82V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aO(WaO(aN\x82V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aOAW__\xFD[\x815aOTaOO\x82aO\x10V[aN\xE0V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aOuW__\xFD[` \x85\x01[\x83\x81\x10\x15aO\x9BW\x805aO\x8D\x81aK\xF8V[\x83R` \x92\x83\x01\x92\x01aOzV[P\x95\x94PPPPPV[_\x82`\x1F\x83\x01\x12aO\xB4W__\xFD[\x815aO\xC2aOO\x82aO\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aO\xE3W__\xFD[` \x85\x01[\x83\x81\x10\x15aO\x9BW\x805\x83R` \x92\x83\x01\x92\x01aO\xE8V[_`\xE0\x82\x84\x03\x12\x15aP\x10W__\xFD[aP\x18aN\x96V[\x90PaP#\x82aL\x0CV[\x81RaP1` \x83\x01aL\x0CV[` \x82\x01RaPB`@\x83\x01aL\x0CV[`@\x82\x01R``\x82\x81\x015\x90\x82\x01RaP]`\x80\x83\x01aM?V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aPzW__\xFD[aP\x86\x84\x82\x85\x01aO2V[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xA4W__\xFD[aP\xB0\x84\x82\x85\x01aO\xA5V[`\xC0\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aP\xCCW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xE1W__\xFD[a:\t\x84\x82\x85\x01aP\0V[_` \x82\x84\x03\x12\x15aP\xFDW__\xFD[\x815`\xFF\x81\x16\x81\x14a\x1FZW__\xFD[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aQFW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aQ\x1FV[P\x93\x94\x93PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aQFW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aQbV[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Q_\x91aQ\xCB\x90\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaQ\xE5`\xE0\x85\x01\x82aQ\rV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x1C\x96\x82\x82aQPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aCaW`\x1F\x19\x85\x84\x03\x01\x88RaR6\x83\x83QaQPV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aR\x1AV[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15aR\xA3W`_\x19\x87\x86\x03\x01\x84RaR\x8E\x85\x83QaQ\x80V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aRrV[PPPP\x82\x81\x03` \x84\x01Ra\x1C\x96\x81\x85aQ\xFEV[_____``\x86\x88\x03\x12\x15aR\xCDW__\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xE2W__\xFD[aR\xEE\x88\x82\x89\x01aLsV[\x90\x96P\x94PP` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x0CW__\xFD[aS\x18\x88\x82\x89\x01aLsV[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a'\xF7W__\xFD[___``\x84\x86\x03\x12\x15aSPW__\xFD[\x835aS[\x81aK\xF8V[\x92P` \x84\x015\x91P`@\x84\x015aSr\x81aS*V[\x80\x91PP\x92P\x92P\x92V[`@\x81R_aS\x8F`@\x83\x01\x85aQ\rV[\x82\x81\x03` \x84\x01Ra\x1C\x96\x81\x85aQPV[___`@\x84\x86\x03\x12\x15aS\xB3W__\xFD[\x835aS\xBE\x81aK\xF8V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xD8W__\xFD[aS\xE4\x86\x82\x87\x01aMRV[\x94\x97\x90\x96P\x93\x94PPPPV[__`@\x83\x85\x03\x12\x15aT\x02W__\xFD[\x825aT\r\x81aK\xF8V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT'W__\xFD[aT3\x85\x82\x86\x01aO2V[\x91PP\x92P\x92\x90PV[` \x81R_a\x1FZ` \x83\x01\x84aQPV[______``\x87\x89\x03\x12\x15aTdW__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15aTyW__\xFD[aT\x85\x89\x82\x8A\x01aLsV[\x90\x97P\x95PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xA3W__\xFD[aT\xAF\x89\x82\x8A\x01aLsV[\x90\x95P\x93PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xCDW__\xFD[aT\xD9\x89\x82\x8A\x01aLsV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[___``\x84\x86\x03\x12\x15aT\xFDW__\xFD[\x835aU\x08\x81aK\xF8V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU\"W__\xFD[\x84\x01`@\x81\x87\x03\x12\x15aU3W__\xFD[aU;aN\xBEV[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aUPW__\xFD[\x82\x01`\x1F\x81\x01\x88\x13aU`W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aUyWaUyaN\x82V[aU\x8C`\x1F\x82\x01`\x1F\x19\x16` \x01aN\xE0V[\x81\x81R\x89` \x83\x85\x01\x01\x11\x15aU\xA0W__\xFD[\x81` \x84\x01` \x83\x017_` \x92\x82\x01\x83\x01R\x83R\x92\x83\x015\x92\x82\x01\x92\x90\x92R\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[__`@\x83\x85\x03\x12\x15aU\xE3W__\xFD[\x825aU\xEE\x81aK\xF8V[\x94` \x93\x90\x93\x015\x93PPPV[`@\x81R_aS\x8F`@\x83\x01\x85aQPV[\x80\x15\x15\x81\x14a'\xF7W__\xFD[____``\x85\x87\x03\x12\x15aV.W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15aVCW__\xFD[\x85\x01`\xE0\x81\x88\x03\x12\x15aVTW__\xFD[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aVnW__\xFD[aVz\x87\x82\x88\x01aLsV[\x90\x94P\x92PP`@\x85\x015aV\x8E\x81aV\x0EV[\x93\x96\x92\x95P\x90\x93PPV[____`\x80\x85\x87\x03\x12\x15aV\xACW__\xFD[\x845aV\xB7\x81aK\xF8V[\x93P` \x85\x015aV\xC7\x81aK\xF8V[\x92P`@\x85\x015aV\xD7\x81aS*V[\x91P``\x85\x015aV\x8E\x81aS*V[__`@\x83\x85\x03\x12\x15aV\xF8W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aW\rW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aW\x1DW__\xFD[\x805aW+aOO\x82aO\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aWLW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aWwW\x835aWf\x81aK\xF8V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aWSV[\x94PPPP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT'W__\xFD[` \x81R_a\x1FZ` \x83\x01\x84aQ\xFEV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`^\x19\x836\x03\x01\x81\x12aW\xCEW__\xFD[\x91\x90\x91\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aW\xEDW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aX\x06W__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\"#W__\xFD[_` \x82\x84\x03\x12\x15aX-W__\xFD[\x81Qa\x1FZ\x81aV\x0EV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[_` \x82\x84\x03\x12\x15aXvW__\xFD[\x81Qa\x1FZ\x81aS*V[` \x81R_a\x1FZ` \x83\x01\x84aQ\x80V[_` \x82\x84\x03\x12\x15aX\xA3W__\xFD[\x815a\x1FZ\x81aV\x0EV[_\x82`\x1F\x83\x01\x12aX\xBDW__\xFD[\x81QaX\xCBaOO\x82aO\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aX\xECW__\xFD[` \x85\x01[\x83\x81\x10\x15aO\x9BW\x80Q\x83R` \x92\x83\x01\x92\x01aX\xF1V[__`@\x83\x85\x03\x12\x15aY\x1AW__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aY/W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aY?W__\xFD[\x80QaYMaOO\x82aO\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aYnW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aY\x99W\x83QaY\x88\x81aK\xF8V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aYuV[\x80\x95PPPPP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aY\xB8W__\xFD[aT3\x85\x82\x86\x01aX\xAEV[_` \x82\x84\x03\x12\x15aY\xD4W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1B\x93Wa\x1B\x93aY\xDBV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a:\t\x90\x83\x01\x84aQ\rV[_` \x82\x84\x03\x12\x15aZ5W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZJW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aZZW__\xFD[\x80QaZhaOO\x82aO\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aZ\x89W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x08\xDEW\x83QaZ\xA3\x81aS*V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aZ\x90V[_\x825`\xDE\x19\x836\x03\x01\x81\x12aW\xCEW__\xFD[_a\x1B\x936\x83aP\0V[_` \x82\x84\x03\x12\x15aZ\xE3W__\xFD[\x81Qa\x1FZ\x81aK\xF8V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_`\x01\x82\x01a[#Wa[#aY\xDBV[P`\x01\x01\x90V[\x83\x81R``` \x82\x01R_a[B``\x83\x01\x85aQ\x80V[\x82\x81\x03`@\x84\x01Ra\x08\xDE\x81\x85aQPV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1B\x93Wa\x1B\x93aY\xDBV[__`@\x83\x85\x03\x12\x15a[\x81W__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x81\x81\x03\x81\x81\x11\x15a\x1B\x93Wa\x1B\x93aY\xDBV[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1B\x93Wa\x1B\x93aY\xDBV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R_\x90a[\xE4\x90\x83\x01\x85aQ\rV[\x90Pc\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1B\x93Wa\x1B\x93aY\xDBV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[\x82\x81R`@` \x82\x01R_\x82Q\x80`@\x84\x01R\x80` \x85\x01``\x85\x01^_``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x93\x92PPPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_\x82a\\\xC1WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xB3\x02\xBC5\x11N\x19!\xCF\x8B\xB1k\xBE\x96\xDE5\x16\xE7P\xFA\x9B\x91(~\x8E:\xC4G\xF0\xD9\xFE\xA7dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106102b1575f3560e01c8063778e55f31161017b578063bfae3fd2116100e4578063e4cc3f901161009e578063f0e0e67611610079578063f0e0e67614610812578063f2fde38b14610832578063f698da2514610845578063fabc1cbc1461084d575f5ffd5b8063e4cc3f90146107d9578063ee74937f146107ec578063eea9064b146107ff575f5ffd5b8063bfae3fd214610724578063c448feb814610737578063c978f7ac1461076b578063ca8aa7c71461078c578063cd6dc687146107b3578063da8be864146107c6575f5ffd5b80639435bb43116101355780639435bb431461060257806399f5371b14610615578063a1788484146106a3578063a33a3433146106c2578063b7f06ebe146106d5578063bb45fef2146106f7575f5ffd5b8063778e55f31461055257806378296ec51461057c578063886f11951461058f5780638da5cb5b146105b657806390041347146105c75780639104c319146105e7575f5ffd5b8063595c6a671161021d57806360a0d1ce116101d757806360a0d1ce146104c857806365da1264146104db57806366d5ba93146105035780636d70f7ae146105245780636e17444814610537578063715018a61461054a575f5ffd5b8063595c6a671461044e578063597b36da146104565780635ac86ab7146104695780635c975abb1461048c5780635dd68579146104945780635f48e667146104b5575f5ffd5b80633c651cf21161026e5780633c651cf2146103895780633cdeb5e01461039c5780633e28391d146103ca5780634657e26a146103ed5780634665bcda1461041457806354b7c96c1461043b575f5ffd5b806304a4f979146102b55780630b9f487a146102ef5780630dd8dd0214610302578063136439dd146103225780632aa6d8881461033757806339b70e381461034a575b5f5ffd5b6102dc7f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad81565b6040519081526020015b60405180910390f35b6102dc6102fd366004614c1c565b610860565b610315610310366004614cb3565b6108e8565b6040516102e69190614cf1565b610335610330366004614d28565b610bca565b005b610335610345366004614d8f565b610c9f565b6103717f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016102e6565b610335610397366004614ded565b610de1565b6103716103aa366004614e30565b6001600160a01b039081165f908152609960205260409020600101541690565b6103dd6103d8366004614e30565b610f28565b60405190151581526020016102e6565b6103717f000000000000000000000000000000000000000000000000000000000000000081565b6103717f000000000000000000000000000000000000000000000000000000000000000081565b610335610449366004614e4b565b610f47565b610335610fa3565b6102dc6104643660046150bc565b611052565b6103dd6104773660046150ed565b606654600160ff9092169190911b9081161490565b6066546102dc565b6104a76104a2366004614e30565b611081565b6040516102e692919061524c565b6103356104c33660046152b9565b611438565b6103356104d636600461533e565b6116cc565b6103716104e9366004614e30565b609a6020525f90815260409020546001600160a01b031681565b610516610511366004614e30565b611861565b6040516102e692919061537d565b6103dd610532366004614e30565b611b61565b6102dc610545366004614e4b565b611b99565b610335611c9f565b6102dc610560366004614e4b565b609860209081525f928352604080842090915290825290205481565b61033561058a3660046153a1565b611cb0565b6103717f000000000000000000000000000000000000000000000000000000000000000081565b6033546001600160a01b0316610371565b6105da6105d53660046153f1565b611d38565b6040516102e6919061543d565b61037173beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac081565b61033561061036600461544f565b611e0e565b610665610623366004614d28565b60a46020525f9081526040902080546001820154600283015460038401546004909401546001600160a01b039384169492841693909116919063ffffffff1685565b604080516001600160a01b03968716815294861660208601529290941691830191909152606082015263ffffffff909116608082015260a0016102e6565b6102dc6106b1366004614e30565b609f6020525f908152604090205481565b6103156106d03660046154eb565b611ec4565b6103dd6106e3366004614d28565b609e6020525f908152604090205460ff1681565b6103dd6107053660046155d2565b609c60209081525f928352604080842090915290825290205460ff1681565b6102dc610732366004614e4b565b611f61565b60405163ffffffff7f00000000000000000000000000000000000000000000000000000000000000001681526020016102e6565b61077e6107793660046153f1565b611f9d565b6040516102e69291906155fc565b6103717f000000000000000000000000000000000000000000000000000000000000000081565b6103356107c13660046155d2565b61222a565b6103156107d4366004614e30565b612345565b6103356107e736600461561b565b612486565b6103356107fa366004615699565b6124dc565b61033561080d3660046154eb565b612679565b6108256108203660046156e7565b6126dc565b6040516102e69190615794565b610335610840366004614e30565b612781565b6102dc6127fa565b61033561085b366004614d28565b612808565b604080517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad60208201526001600160a01b03808616928201929092528187166060820152908516608082015260a0810183905260c081018290525f906108de9060e0016040516020818303038152906040528051906020012061291f565b9695505050505050565b6066546060906001906002908116036109145760405163840a48d560e01b815260040160405180910390fd5b5f836001600160401b0381111561092d5761092d614e82565b604051908082528060200260200182016040528015610956578160200160208202803683370190505b50335f908152609a60205260408120549192506001600160a01b03909116905b85811015610bbf57868682818110610990576109906157a6565b90506020028101906109a291906157ba565b6109b09060208101906157d8565b90508787838181106109c4576109c46157a6565b90506020028101906109d691906157ba565b6109e090806157d8565b905014610a00576040516343714afd60e01b815260040160405180910390fd5b33878783818110610a1357610a136157a6565b9050602002810190610a2591906157ba565b610a36906060810190604001614e30565b6001600160a01b031614610a5d576040516330c4716960e21b815260040160405180910390fd5b5f610ac733848a8a86818110610a7557610a756157a6565b9050602002810190610a8791906157ba565b610a9190806157d8565b808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061294d92505050565b9050610b9933848a8a86818110610ae057610ae06157a6565b9050602002810190610af291906157ba565b610afc90806157d8565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508e92508d9150889050818110610b4157610b416157a6565b9050602002810190610b5391906157ba565b610b619060208101906157d8565b808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250889250612a94915050565b848381518110610bab57610bab6157a6565b602090810291909101015250600101610976565b509095945050505050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015610c2c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c50919061581d565b610c6d57604051631d77d47760e21b815260040160405180910390fd5b6066548181168114610c925760405163c61dca5d60e01b815260040160405180910390fd5b610c9b8261306d565b5050565b610ca833610f28565b15610cc657604051633bf2b50360e11b815260040160405180910390fd5b604051632b6241f360e11b815233600482015263ffffffff841660248201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906356c483e6906044015f604051808303815f87803b158015610d30575f5ffd5b505af1158015610d42573d5f5f3e3d5ffd5b50505050610d5033856130aa565b610d5a333361310c565b6040516001600160a01b038516815233907fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c19060200160405180910390a2336001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908383604051610dd3929190615838565b60405180910390a250505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480610e405750336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016145b610e5d5760405163045206a560e21b815260040160405180910390fd5b6001600160a01b038481165f908152609a602052604080822054905163152667d960e31b8152908316600482018190528684166024830152927f0000000000000000000000000000000000000000000000000000000000000000169063a9333ec890604401602060405180830381865afa158015610edd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f019190615866565b90505f610f0f87878461320f565b9050610f1f8388888888866132f1565b50505050505050565b6001600160a01b039081165f908152609a602052604090205416151590565b81610f518161342f565b610f6e5760405163932d94f760e01b815260040160405180910390fd5b610f7783611b61565b610f94576040516325ec6c1f60e01b815260040160405180910390fd5b610f9e83836130aa565b505050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611005573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611029919061581d565b61104657604051631d77d47760e21b815260040160405180910390fd5b6110505f1961306d565b565b5f816040516020016110649190615881565b604051602081830303815290604052805190602001209050919050565b6001600160a01b0381165f90815260a36020526040812060609182916110a6906134d9565b8051909150806001600160401b038111156110c3576110c3614e82565b6040519080825280602002602001820160405280156110fc57816020015b6110e9614ad8565b8152602001906001900390816110e15790505b509350806001600160401b0381111561111757611117614e82565b60405190808252806020026020018201604052801561114a57816020015b60608152602001906001900390816111355790505b506001600160a01b038087165f908152609a60205260408120549295509116905b8281101561142f5760a45f858381518110611188576111886157a6565b60209081029190910181015182528181019290925260409081015f20815160e08101835281546001600160a01b03908116825260018301548116828601526002830154168184015260038201546060820152600482015463ffffffff1660808201526005820180548451818702810187019095528085529194929360a086019390929083018282801561124257602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611224575b505050505081526020016006820180548060200260200160405190810160405280929190818152602001828054801561129857602002820191905f5260205f20905b815481526020019060010190808311611284575b5050505050815250508682815181106112b3576112b36157a6565b60200260200101819052508581815181106112d0576112d06157a6565b602002602001015160a00151516001600160401b038111156112f4576112f4614e82565b60405190808252806020026020018201604052801561131d578160200160208202803683370190505b50858281518110611330576113306157a6565b60200260200101819052505f6113648884898581518110611353576113536157a6565b602002602001015160a0015161294d565b90505f5b87838151811061137a5761137a6157a6565b602002602001015160a0015151811015611425576113e78884815181106113a3576113a36157a6565b602002602001015160c0015182815181106113c0576113c06157a6565b60200260200101518385815181106113da576113da6157a6565b60200260200101516134e5565b8784815181106113f9576113f96157a6565b60200260200101518281518110611412576114126157a6565b6020908102919091010152600101611368565b505060010161116b565b50505050915091565b6066546002906004908116036114615760405163840a48d560e01b815260040160405180910390fd5b6114696134f0565b335f90815260a3602052604081209061148182613549565b90508084116114905783611492565b805b93505f846001600160401b038111156114ad576114ad614e82565b6040519080825280602002602001820160405280156114e657816020015b6114d3614ad8565b8152602001906001900390816114cb5790505b5090505f5b81518110156116355760a45f6115018684613552565b815260208082019290925260409081015f20815160e08101835281546001600160a01b03908116825260018301548116828601526002830154168184015260038201546060820152600482015463ffffffff1660808201526005820180548451818702810187019095528085529194929360a08601939092908301828280156115b157602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611593575b505050505081526020016006820180548060200260200160405190810160405280929190818152602001828054801561160757602002820191905f5260205f20905b8154815260200190600101908083116115f3575b505050505081525050828281518110611622576116226157a6565b60209081029190910101526001016114eb565b505f5b81518110156116b6576116ae828281518110611656576116566157a6565b60200260200101518b8b84818110611670576116706157a6565b905060200281019061168291906157d8565b8b8b86818110611694576116946157a6565b90506020020160208101906116a99190615893565b61355d565b600101611638565b505050506116c4600160c955565b505050505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461171557604051633213a66160e21b815260040160405180910390fd5b61171e83610f28565b15610f9e576001600160a01b038381165f908152609a602052604080822054905163152667d960e31b81529083166004820181905273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06024830152927f0000000000000000000000000000000000000000000000000000000000000000169063a9333ec890604401602060405180830381865afa1580156117b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117d99190615866565b6001600160a01b0386165f90815260a26020908152604080832073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac084528252808320815192830190915254815291925061183f866118376001600160401b038087169089166139d7565b8491906139eb565b9050610f1f848873beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac084613a11565b6040516394f649dd60e01b81526001600160a01b03828116600483015260609182915f9182917f000000000000000000000000000000000000000000000000000000000000000016906394f649dd906024015f60405180830381865afa1580156118cd573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526118f49190810190615909565b60405163fe243a1760e01b81526001600160a01b03888116600483015273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac060248301529294509092505f917f0000000000000000000000000000000000000000000000000000000000000000169063fe243a1790604401602060405180830381865afa15801561197a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061199e91906159c4565b9050805f036119b257509094909350915050565b5f835160016119c191906159ef565b6001600160401b038111156119d8576119d8614e82565b604051908082528060200260200182016040528015611a01578160200160208202803683370190505b5090505f84516001611a1391906159ef565b6001600160401b03811115611a2a57611a2a614e82565b604051908082528060200260200182016040528015611a53578160200160208202803683370190505b50905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac082865181518110611a7e57611a7e6157a6565b60200260200101906001600160a01b031690816001600160a01b0316815250508281865181518110611ab257611ab26157a6565b60209081029190910101525f5b8551811015611b5357858181518110611ada57611ada6157a6565b6020026020010151838281518110611af457611af46157a6565b60200260200101906001600160a01b031690816001600160a01b031681525050848181518110611b2657611b266157a6565b6020026020010151828281518110611b4057611b406157a6565b6020908102919091010152600101611abf565b509097909650945050505050565b5f6001600160a01b03821615801590611b9357506001600160a01b038083165f818152609a6020526040902054909116145b92915050565b6040805160018082528183019092525f918291906020808301908036833701905050905082815f81518110611bd057611bd06157a6565b6001600160a01b03928316602091820292909201015260405163547afb8760e01b81525f917f0000000000000000000000000000000000000000000000000000000000000000169063547afb8790611c2e9088908690600401615a02565b5f60405180830381865afa158015611c48573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611c6f9190810190615a25565b5f81518110611c8057611c806157a6565b60200260200101519050611c968585835f613a8b565b95945050505050565b611ca7613b3c565b6110505f613b96565b82611cba8161342f565b611cd75760405163932d94f760e01b815260040160405180910390fd5b611ce084611b61565b611cfd576040516325ec6c1f60e01b815260040160405180910390fd5b836001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908484604051610dd3929190615838565b60605f82516001600160401b03811115611d5457611d54614e82565b604051908082528060200260200182016040528015611d7d578160200160208202803683370190505b5090505f5b8351811015611e06576001600160a01b0385165f9081526098602052604081208551909190869084908110611db957611db96157a6565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f2054828281518110611df357611df36157a6565b6020908102919091010152600101611d82565b509392505050565b606654600290600490811603611e375760405163840a48d560e01b815260040160405180910390fd5b611e3f6134f0565b855f5b81811015611eb857611eb0898983818110611e5f57611e5f6157a6565b9050602002810190611e719190615ab4565b611e7a90615ac8565b888884818110611e8c57611e8c6157a6565b9050602002810190611e9e91906157d8565b888886818110611694576116946157a6565b600101611e42565b5050610f1f600160c955565b6060611ecf33610f28565b611eec5760405163a5c7c44560e01b815260040160405180910390fd5b611ef533611b61565b15611f13576040516311ca333560e31b815260040160405180910390fd5b611f1c84611b61565b611f39576040516325ec6c1f60e01b815260040160405180910390fd5b611f4233613be7565b9050611f5033858585613e46565b611f5a338561310c565b9392505050565b6001600160a01b038083165f90815260a260209081526040808320938516835292815282822083519182019093529154825290611f5a90613f0b565b60608082516001600160401b03811115611fb957611fb9614e82565b604051908082528060200260200182016040528015611fe2578160200160208202803683370190505b50915082516001600160401b03811115611ffe57611ffe614e82565b604051908082528060200260200182016040528015612027578160200160208202803683370190505b506001600160a01b038086165f908152609a602052604081205492935091169061205286838761294d565b90505f5b855181101561221f575f612082878381518110612075576120756157a6565b6020026020010151613f2a565b9050806001600160a01b031663fe243a17898985815181106120a6576120a66157a6565b60200260200101516040518363ffffffff1660e01b81526004016120e09291906001600160a01b0392831681529116602082015260400190565b602060405180830381865afa1580156120fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061211f91906159c4565b858381518110612131576121316157a6565b6020026020010181815250505f60a25f8a6001600160a01b03166001600160a01b031681526020019081526020015f205f898581518110612174576121746157a6565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f8201548152505090506121f88684815181106121c6576121c66157a6565b60200260200101518585815181106121e0576121e06157a6565b6020026020010151836139eb9092919063ffffffff16565b87848151811061220a5761220a6157a6565b60209081029190910101525050600101612056565b5050505b9250929050565b5f54610100900460ff161580801561224857505f54600160ff909116105b806122615750303b15801561226157505f5460ff166001145b6122c95760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156122ea575f805461ff0019166101001790555b6122f38261306d565b6122fc83613b96565b8015610f9e575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050565b606061235082610f28565b61236d5760405163a5c7c44560e01b815260040160405180910390fd5b61237682611b61565b15612394576040516311ca333560e31b815260040160405180910390fd5b6001600160a01b0382166123bb576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b038083165f818152609a6020526040902054909116903314806123e957506123e98161342f565b8061240f57506001600160a01b038181165f908152609960205260409020600101541633145b61242c57604051631e499a2360e11b815260040160405180910390fd5b336001600160a01b0384161461247d57806001600160a01b0316836001600160a01b03167ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a60405160405180910390a35b611f5a83613be7565b6066546002906004908116036124af5760405163840a48d560e01b815260040160405180910390fd5b6124b76134f0565b6124cb6124c386615ac8565b85858561355d565b6124d5600160c955565b5050505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614612525576040516323d871a560e01b815260040160405180910390fd5b6001600160a01b038085165f908152609860209081526040808320938716835292905290812054612563906001600160401b03808616908516613f9c565b90505f61257286868686613a8b565b61257c90836159ef565b905061258a865f8785613a11565b6001600160a01b03851673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0146116c457604051633b9e9f0160e21b81526001600160a01b038681166004830152602482018390527f0000000000000000000000000000000000000000000000000000000000000000169063ee7a7c04906044015f604051808303815f87803b158015612615575f5ffd5b505af1158015612627573d5f5f3e3d5ffd5b5050604080516001600160a01b038981168252602082018690528a1693507feff6aab2bc3f7c648896e1522eae71d6c22e3b0e218206b3f40af0e4d204716b92500160405180910390a2505050505050565b61268233610f28565b156126a057604051633bf2b50360e11b815260040160405180910390fd5b6126a983611b61565b6126c6576040516325ec6c1f60e01b815260040160405180910390fd5b6126d233848484613e46565b610f9e338461310c565b60605f83516001600160401b038111156126f8576126f8614e82565b60405190808252806020026020018201604052801561272b57816020015b60608152602001906001900390816127165790505b5090505f5b8451811015611e065761275c85828151811061274e5761274e6157a6565b602002602001015185611d38565b82828151811061276e5761276e6157a6565b6020908102919091010152600101612730565b612789613b3c565b6001600160a01b0381166127ee5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016122c0565b6127f781613b96565b50565b5f612803613fb4565b905090565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612864573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128889190615ad3565b6001600160a01b0316336001600160a01b0316146128b95760405163794821ff60e01b815260040160405180910390fd5b606654801982198116146128e05760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020015b60405180910390a25050565b5f612928613fb4565b60405161190160f01b6020820152602281019190915260428101839052606201611064565b60605f82516001600160401b0381111561296957612969614e82565b604051908082528060200260200182016040528015612992578160200160208202803683370190505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663547afb8786866040518363ffffffff1660e01b81526004016129e4929190615a02565b5f60405180830381865afa1580156129fe573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612a259190810190615a25565b90505f5b8451811015610bbf57612a6f87868381518110612a4857612a486157a6565b6020026020010151848481518110612a6257612a626157a6565b602002602001015161320f565b838281518110612a8157612a816157a6565b6020908102919091010152600101612a29565b5f6001600160a01b038616612abc576040516339b190bb60e11b815260040160405180910390fd5b83515f03612add5760405163796cc52560e01b815260040160405180910390fd5b5f84516001600160401b03811115612af757612af7614e82565b604051908082528060200260200182016040528015612b20578160200160208202803683370190505b5090505f85516001600160401b03811115612b3d57612b3d614e82565b604051908082528060200260200182016040528015612b66578160200160208202803683370190505b5090505f5b8651811015612ea0575f612b8a888381518110612075576120756157a6565b90505f60a25f8c6001600160a01b03166001600160a01b031681526020019081526020015f205f8a8581518110612bc357612bc36157a6565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f820154815250509050816001600160a01b031663fe243a178c8b8681518110612c2257612c226157a6565b60200260200101516040518363ffffffff1660e01b8152600401612c5c9291906001600160a01b0392831681529116602082015260400190565b602060405180830381865afa158015612c77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c9b91906159c4565b888481518110612cad57612cad6157a6565b60200260200101511115612cd45760405163f020e5b960e01b815260040160405180910390fd5b612d03888481518110612ce957612ce96157a6565b60200260200101518885815181106121e0576121e06157a6565b848481518110612d1557612d156157a6565b602002602001018181525050612d5d848481518110612d3657612d366157a6565b6020026020010151888581518110612d5057612d506157a6565b6020026020010151614099565b858481518110612d6f57612d6f6157a6565b60209081029190910101526001600160a01b038a1615612e0457612dc68a8a8581518110612d9f57612d9f6157a6565b6020026020010151878681518110612db957612db96157a6565b60200260200101516140b2565b612e048a8c8b8681518110612ddd57612ddd6157a6565b6020026020010151878781518110612df757612df76157a6565b6020026020010151613a11565b816001600160a01b031663724af4238c8b8681518110612e2657612e266157a6565b60200260200101518b8781518110612e4057612e406157a6565b60200260200101516040518463ffffffff1660e01b8152600401612e6693929190615aee565b5f604051808303815f87803b158015612e7d575f5ffd5b505af1158015612e8f573d5f5f3e3d5ffd5b505050505050806001019050612b6b565b506001600160a01b0388165f908152609f60205260408120805491829190612ec783615b12565b91905055505f6040518060e001604052808b6001600160a01b031681526020018a6001600160a01b031681526020018b6001600160a01b031681526020018381526020014363ffffffff1681526020018981526020018581525090505f612f2d82611052565b5f818152609e602090815260408083208054600160ff19909116811790915560a4835292819020865181546001600160a01b03199081166001600160a01b039283161783558885015195830180548216968316969096179095559187015160028201805490951692169190911790925560608501516003830155608085015160048301805463ffffffff191663ffffffff90921691909117905560a085015180519394508593612fe39260058501920190614b31565b5060c08201518051612fff916006840191602090910190614b94565b5050506001600160a01b038b165f90815260a3602052604090206130239082614140565b507f26b2aae26516e8719ef50ea2f6831a2efbd4e37dccdf0f6936b27bc08e793e3081838660405161305793929190615b2a565b60405180910390a19a9950505050505050505050565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b6001600160a01b038281165f8181526099602090815260409182902060010180546001600160a01b0319169486169485179055905192835290917f773b54c04d756fcc5e678111f7d730de3be98192000799eee3d63716055a87c69101612913565b6066545f906001908116036131345760405163840a48d560e01b815260040160405180910390fd5b6001600160a01b038381165f818152609a602052604080822080546001600160a01b0319169487169485179055517fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433049190a35f5f61319185611861565b915091505f6131a186868561294d565b90505f5b8351811015610f1f5761320786888684815181106131c5576131c56157a6565b60200260200101515f8786815181106131e0576131e06157a6565b60200260200101518787815181106131fa576131fa6157a6565b60200260200101516132f1565b6001016131a5565b5f73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeabf196001600160a01b038416016132e15760405163a3d75e0960e01b81526001600160a01b0385811660048301525f917f00000000000000000000000000000000000000000000000000000000000000009091169063a3d75e0990602401602060405180830381865afa15801561329d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132c19190615866565b90506132d96001600160401b038481169083166139d7565b915050611f5a565b506001600160401b031692915050565b805f0361331157604051630a33bc6960e21b815260040160405180910390fd5b6001600160a01b038086165f90815260a2602090815260408083209388168352929052206133418185858561414b565b6040805160208101909152815481527f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f908790879061337f90613f0b565b60405161338e93929190615aee565b60405180910390a161339f86610f28565b15610f1f576001600160a01b038088165f908152609860209081526040808320938916835292905290812080548592906133da9084906159ef565b92505081905550866001600160a01b03167f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c87878660405161341e93929190615aee565b60405180910390a250505050505050565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af11580156134b5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b93919061581d565b60605f611f5a836141c0565b5f611f5a83836139d7565b600260c954036135425760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064016122c0565b600260c955565b5f611b93825490565b5f611f5a8383614219565b60a0840151518214613582576040516343714afd60e01b815260040160405180910390fd5b83604001516001600160a01b0316336001600160a01b0316146135b8576040516316110d3560e21b815260040160405180910390fd5b5f6135c285611052565b5f818152609e602052604090205490915060ff166135f3576040516387c9d21960e01b815260040160405180910390fd5b60605f7f000000000000000000000000000000000000000000000000000000000000000087608001516136269190615b54565b90504363ffffffff168163ffffffff161115613655576040516378f67ae160e11b815260040160405180910390fd5b61366c875f015188602001518960a001518461423f565b87516001600160a01b039081165f908152609a60205260408120548a5160a08c015194965092169350916136a29190849061294d565b90505f5b8860a00151518110156138f5575f6136cd8a60a001518381518110612075576120756157a6565b90505f6137038b60c0015184815181106136e9576136e96157a6565b60200260200101518785815181106113da576113da6157a6565b905087156137d357816001600160a01b0316632eae418c8c5f01518d60a001518681518110613734576137346157a6565b60200260200101518d8d8881811061374e5761374e6157a6565b90506020020160208101906137639190614e30565b60405160e085901b6001600160e01b03191681526001600160a01b03938416600482015291831660248301529091166044820152606481018490526084015f604051808303815f87803b1580156137b8575f5ffd5b505af11580156137ca573d5f5f3e3d5ffd5b505050506138eb565b5f5f836001600160a01b031663c4623ea18e5f01518f60a0015188815181106137fe576137fe6157a6565b60200260200101518f8f8a818110613818576138186157a6565b905060200201602081019061382d9190614e30565b60405160e085901b6001600160e01b03191681526001600160a01b039384166004820152918316602483015290911660448201526064810186905260840160408051808303815f875af1158015613886573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138aa9190615b70565b915091506138e8878e5f01518f60a0015188815181106138cc576138cc6157a6565b602002602001015185858b8b815181106131fa576131fa6157a6565b50505b50506001016136a6565b5087516001600160a01b03165f90815260a360205260409020613918908561436d565b505f84815260a46020526040812080546001600160a01b031990811682556001820180548216905560028201805490911690556003810182905560048101805463ffffffff191690559061396f6005830182614bcd565b61397c600683015f614bcd565b50505f848152609e602052604090819020805460ff19169055517f1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a00906139c59086815260200190565b60405180910390a15050505050505050565b5f611f5a8383670de0b6b3a7640000614378565b5f613a0982613a036139fc87613f0b565b86906139d7565b906139d7565b949350505050565b6001600160a01b038085165f90815260986020908152604080832093861683529290529081208054839290613a47908490615b92565b92505081905550836001600160a01b03167f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd848484604051610dd393929190615aee565b6001600160a01b038085165f90815260a56020908152604080832093871683529290529081208190613abc9061445d565b90505f613b16613aec7f000000000000000000000000000000000000000000000000000000000000000043615ba5565b6001600160a01b03808a165f90815260a560209081526040808320938c1683529290522090614477565b90505f613b238284615b92565b9050613b30818787614493565b98975050505050505050565b6033546001600160a01b031633146110505760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016122c0565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b606654606090600190600290811603613c135760405163840a48d560e01b815260040160405180910390fd5b6001600160a01b038084165f818152609a602052604080822080546001600160a01b0319811690915590519316928392917ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af4467691a35f5f613c7286611861565b9150915081515f03613c8657505050613e40565b81516001600160401b03811115613c9f57613c9f614e82565b604051908082528060200260200182016040528015613cc8578160200160208202803683370190505b5094505f613cd787858561294d565b90505f5b8351811015613e3a576040805160018082528183019092525f916020808301908036833750506040805160018082528183019092529293505f9291506020808301908036833750506040805160018082528183019092529293505f92915060208083019080368337019050509050868481518110613d5b57613d5b6157a6565b6020026020010151835f81518110613d7557613d756157a6565b60200260200101906001600160a01b031690816001600160a01b031681525050858481518110613da757613da76157a6565b6020026020010151825f81518110613dc157613dc16157a6565b602002602001018181525050848481518110613ddf57613ddf6157a6565b6020026020010151815f81518110613df957613df96157a6565b602002602001018181525050613e128b89858585612a94565b8a8581518110613e2457613e246157a6565b6020908102919091010152505050600101613cdb565b50505050505b50919050565b6001600160a01b038084165f908152609960205260409020600101541680613e6e5750613f05565b6001600160a01b0381165f908152609c6020908152604080832085845290915290205460ff1615613eb257604051630d4c4c9160e21b815260040160405180910390fd5b6001600160a01b0381165f908152609c602090815260408083208584528252909120805460ff191660011790558301516124d5908290613ef9908890889084908890610860565b855160208701516144b1565b50505050565b80515f9015613f1b578151611b93565b670de0b6b3a764000092915050565b5f6001600160a01b03821673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014613f75577f0000000000000000000000000000000000000000000000000000000000000000611b93565b7f000000000000000000000000000000000000000000000000000000000000000092915050565b5f613faa8483856001614503565b613a099085615b92565b5f7f000000000000000000000000000000000000000000000000000000000000000046146140745750604080518082018252600a81526922b4b3b2b72630bcb2b960b11b60209182015281517f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866818301527f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea81840152466060820152306080808301919091528351808303909101815260a0909101909252815191012090565b507f000000000000000000000000000000000000000000000000000000000000000090565b5f815f036140a857505f611b93565b611f5a8383614552565b6001600160a01b03821673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014610f9e576001600160a01b038084165f90815260a56020908152604080832093861683529290529081206141059061445d565b9050613f054361411584846159ef565b6001600160a01b038088165f90815260a560209081526040808320938a168352929052209190614566565b5f611f5a8383614571565b825f0361416b57614164670de0b6b3a764000082614552565b8455613f05565b6040805160208101909152845481525f906141879085846139eb565b90505f61419484836159ef565b90505f6141b5846141af6141a8888a6159ef565b8590614552565b90614552565b875550505050505050565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561420d57602002820191905f5260205f20905b8154815260200190600101908083116141f9575b50505050509050919050565b5f825f01828154811061422e5761422e6157a6565b905f5260205f200154905092915050565b60605f83516001600160401b0381111561425b5761425b614e82565b604051908082528060200260200182016040528015614284578160200160208202803683370190505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166394d7d00c8787876040518463ffffffff1660e01b81526004016142d893929190615bc1565b5f60405180830381865afa1580156142f2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526143199190810190615a25565b90505f5b85518110156143615761433c88878381518110612a4857612a486157a6565b83828151811061434e5761434e6157a6565b602090810291909101015260010161431d565b50909695505050505050565b5f611f5a83836145bd565b5f80805f19858709858702925082811083820303915050805f036143af578382816143a5576143a5615bfa565b0492505050611f5a565b8084116143f65760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b60448201526064016122c0565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b5f61446882826146a0565b6001600160e01b031692915050565b5f6144838383836146e5565b6001600160e01b03169392505050565b5f613a096144a18385615c0e565b85906001600160401b03166139d7565b428110156144d257604051630819bdcd60e01b815260040160405180910390fd5b6144e66001600160a01b038516848461472e565b613f0557604051638baa579f60e01b815260040160405180910390fd5b5f5f614510868686614378565b9050600183600281111561452657614526615c2d565b14801561454257505f848061453d5761453d615bfa565b868809115b15611c96576108de6001826159ef565b5f611f5a83670de0b6b3a764000084614378565b610f9e838383614782565b5f8181526001830160205260408120546145b657508154600181810184555f848152602080822090930184905584548482528286019093526040902091909155611b93565b505f611b93565b5f8181526001830160205260408120548015614697575f6145df600183615b92565b85549091505f906145f290600190615b92565b9050818114614651575f865f018281548110614610576146106157a6565b905f5260205f200154905080875f018481548110614630576146306157a6565b5f918252602080832090910192909255918252600188019052604090208390555b855486908061466257614662615c41565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050611b93565b5f915050611b93565b81545f9080156146dd576146c6846146b9600184615b92565b5f91825260209091200190565b5464010000000090046001600160e01b0316613a09565b509092915050565b82545f90816146f686868385614888565b905080156147245761470d866146b9600184615b92565b5464010000000090046001600160e01b03166108de565b5091949350505050565b5f5f5f61473b85856148db565b90925090505f81600481111561475357614753615c2d565b1480156147715750856001600160a01b0316826001600160a01b0316145b806108de57506108de86868661491a565b8254801561483a575f61479a856146b9600185615b92565b60408051808201909152905463ffffffff8082168084526401000000009092046001600160e01b0316602084015291925090851610156147ed5760405163151b8e3f60e11b815260040160405180910390fd5b805163ffffffff808616911603614838578261480e866146b9600186615b92565b80546001600160e01b03929092166401000000000263ffffffff9092169190911790555050505050565b505b506040805180820190915263ffffffff92831681526001600160e01b03918216602080830191825285546001810187555f968752952091519051909216640100000000029190921617910155565b5f5b81831015611e06575f61489d8484614a01565b5f8781526020902090915063ffffffff86169082015463ffffffff1611156148c7578092506148d5565b6148d28160016159ef565b93505b5061488a565b5f5f825160410361490f576020830151604084015160608501515f1a61490387828585614a1b565b94509450505050612223565b505f90506002612223565b5f5f5f856001600160a01b0316631626ba7e60e01b8686604051602401614942929190615c55565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516149809190615c91565b5f60405180830381855afa9150503d805f81146149b8576040519150601f19603f3d011682016040523d82523d5f602084013e6149bd565b606091505b50915091508180156149d157506020815110155b80156108de57508051630b135d3f60e11b906149f690830160209081019084016159c4565b149695505050505050565b5f614a0f6002848418615ca7565b611f5a908484166159ef565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115614a5057505f90506003614acf565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015614aa1573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116614ac9575f60019250925050614acf565b91505f90505b94509492505050565b6040518060e001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f63ffffffff16815260200160608152602001606081525090565b828054828255905f5260205f20908101928215614b84579160200282015b82811115614b8457825182546001600160a01b0319166001600160a01b03909116178255602090920191600190910190614b4f565b50614b90929150614be4565b5090565b828054828255905f5260205f20908101928215614b84579160200282015b82811115614b84578251825591602001919060010190614bb2565b5080545f8255905f5260205f20908101906127f791905b5b80821115614b90575f8155600101614be5565b6001600160a01b03811681146127f7575f5ffd5b8035614c1781614bf8565b919050565b5f5f5f5f5f60a08688031215614c30575f5ffd5b8535614c3b81614bf8565b94506020860135614c4b81614bf8565b93506040860135614c5b81614bf8565b94979396509394606081013594506080013592915050565b5f5f83601f840112614c83575f5ffd5b5081356001600160401b03811115614c99575f5ffd5b6020830191508360208260051b8501011115612223575f5ffd5b5f5f60208385031215614cc4575f5ffd5b82356001600160401b03811115614cd9575f5ffd5b614ce585828601614c73565b90969095509350505050565b602080825282518282018190525f918401906040840190835b81811015610bbf578351835260209384019390920191600101614d0a565b5f60208284031215614d38575f5ffd5b5035919050565b803563ffffffff81168114614c17575f5ffd5b5f5f83601f840112614d62575f5ffd5b5081356001600160401b03811115614d78575f5ffd5b602083019150836020828501011115612223575f5ffd5b5f5f5f5f60608587031215614da2575f5ffd5b8435614dad81614bf8565b9350614dbb60208601614d3f565b925060408501356001600160401b03811115614dd5575f5ffd5b614de187828801614d52565b95989497509550505050565b5f5f5f5f60808587031215614e00575f5ffd5b8435614e0b81614bf8565b93506020850135614e1b81614bf8565b93969395505050506040820135916060013590565b5f60208284031215614e40575f5ffd5b8135611f5a81614bf8565b5f5f60408385031215614e5c575f5ffd5b8235614e6781614bf8565b91506020830135614e7781614bf8565b809150509250929050565b634e487b7160e01b5f52604160045260245ffd5b60405160e081016001600160401b0381118282101715614eb857614eb8614e82565b60405290565b604080519081016001600160401b0381118282101715614eb857614eb8614e82565b604051601f8201601f191681016001600160401b0381118282101715614f0857614f08614e82565b604052919050565b5f6001600160401b03821115614f2857614f28614e82565b5060051b60200190565b5f82601f830112614f41575f5ffd5b8135614f54614f4f82614f10565b614ee0565b8082825260208201915060208360051b860101925085831115614f75575f5ffd5b602085015b83811015614f9b578035614f8d81614bf8565b835260209283019201614f7a565b5095945050505050565b5f82601f830112614fb4575f5ffd5b8135614fc2614f4f82614f10565b8082825260208201915060208360051b860101925085831115614fe3575f5ffd5b602085015b83811015614f9b578035835260209283019201614fe8565b5f60e08284031215615010575f5ffd5b615018614e96565b905061502382614c0c565b815261503160208301614c0c565b602082015261504260408301614c0c565b60408201526060828101359082015261505d60808301614d3f565b608082015260a08201356001600160401b0381111561507a575f5ffd5b61508684828501614f32565b60a08301525060c08201356001600160401b038111156150a4575f5ffd5b6150b084828501614fa5565b60c08301525092915050565b5f602082840312156150cc575f5ffd5b81356001600160401b038111156150e1575f5ffd5b613a0984828501615000565b5f602082840312156150fd575f5ffd5b813560ff81168114611f5a575f5ffd5b5f8151808452602084019350602083015f5b828110156151465781516001600160a01b031686526020958601959091019060010161511f565b5093949350505050565b5f8151808452602084019350602083015f5b82811015615146578151865260209586019590910190600101615162565b80516001600160a01b03908116835260208083015182169084015260408083015190911690830152606080820151908301526080808201515f916151cb9085018263ffffffff169052565b5060a082015160e060a08501526151e560e085018261510d565b905060c083015184820360c0860152611c968282615150565b5f82825180855260208501945060208160051b830101602085015f5b8381101561436157601f19858403018852615236838351615150565b602098890198909350919091019060010161521a565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b828110156152a357605f1987860301845261528e858351615180565b94506020938401939190910190600101615272565b505050508281036020840152611c9681856151fe565b5f5f5f5f5f606086880312156152cd575f5ffd5b85356001600160401b038111156152e2575f5ffd5b6152ee88828901614c73565b90965094505060208601356001600160401b0381111561530c575f5ffd5b61531888828901614c73565b96999598509660400135949350505050565b6001600160401b03811681146127f7575f5ffd5b5f5f5f60608486031215615350575f5ffd5b833561535b81614bf8565b92506020840135915060408401356153728161532a565b809150509250925092565b604081525f61538f604083018561510d565b8281036020840152611c968185615150565b5f5f5f604084860312156153b3575f5ffd5b83356153be81614bf8565b925060208401356001600160401b038111156153d8575f5ffd5b6153e486828701614d52565b9497909650939450505050565b5f5f60408385031215615402575f5ffd5b823561540d81614bf8565b915060208301356001600160401b03811115615427575f5ffd5b61543385828601614f32565b9150509250929050565b602081525f611f5a6020830184615150565b5f5f5f5f5f5f60608789031215615464575f5ffd5b86356001600160401b03811115615479575f5ffd5b61548589828a01614c73565b90975095505060208701356001600160401b038111156154a3575f5ffd5b6154af89828a01614c73565b90955093505060408701356001600160401b038111156154cd575f5ffd5b6154d989828a01614c73565b979a9699509497509295939492505050565b5f5f5f606084860312156154fd575f5ffd5b833561550881614bf8565b925060208401356001600160401b03811115615522575f5ffd5b840160408187031215615533575f5ffd5b61553b614ebe565b81356001600160401b03811115615550575f5ffd5b8201601f81018813615560575f5ffd5b80356001600160401b0381111561557957615579614e82565b61558c601f8201601f1916602001614ee0565b8181528960208385010111156155a0575f5ffd5b816020840160208301375f60209282018301528352928301359282019290925293969395505050506040919091013590565b5f5f604083850312156155e3575f5ffd5b82356155ee81614bf8565b946020939093013593505050565b604081525f61538f6040830185615150565b80151581146127f7575f5ffd5b5f5f5f5f6060858703121561562e575f5ffd5b84356001600160401b03811115615643575f5ffd5b850160e08188031215615654575f5ffd5b935060208501356001600160401b0381111561566e575f5ffd5b61567a87828801614c73565b909450925050604085013561568e8161560e565b939692955090935050565b5f5f5f5f608085870312156156ac575f5ffd5b84356156b781614bf8565b935060208501356156c781614bf8565b925060408501356156d78161532a565b9150606085013561568e8161532a565b5f5f604083850312156156f8575f5ffd5b82356001600160401b0381111561570d575f5ffd5b8301601f8101851361571d575f5ffd5b803561572b614f4f82614f10565b8082825260208201915060208360051b85010192508783111561574c575f5ffd5b6020840193505b8284101561577757833561576681614bf8565b825260209384019390910190615753565b945050505060208301356001600160401b03811115615427575f5ffd5b602081525f611f5a60208301846151fe565b634e487b7160e01b5f52603260045260245ffd5b5f8235605e198336030181126157ce575f5ffd5b9190910192915050565b5f5f8335601e198436030181126157ed575f5ffd5b8301803591506001600160401b03821115615806575f5ffd5b6020019150600581901b3603821315612223575f5ffd5b5f6020828403121561582d575f5ffd5b8151611f5a8161560e565b60208152816020820152818360408301375f818301604090810191909152601f909201601f19160101919050565b5f60208284031215615876575f5ffd5b8151611f5a8161532a565b602081525f611f5a6020830184615180565b5f602082840312156158a3575f5ffd5b8135611f5a8161560e565b5f82601f8301126158bd575f5ffd5b81516158cb614f4f82614f10565b8082825260208201915060208360051b8601019250858311156158ec575f5ffd5b602085015b83811015614f9b5780518352602092830192016158f1565b5f5f6040838503121561591a575f5ffd5b82516001600160401b0381111561592f575f5ffd5b8301601f8101851361593f575f5ffd5b805161594d614f4f82614f10565b8082825260208201915060208360051b85010192508783111561596e575f5ffd5b6020840193505b8284101561599957835161598881614bf8565b825260209384019390910190615975565b8095505050505060208301516001600160401b038111156159b8575f5ffd5b615433858286016158ae565b5f602082840312156159d4575f5ffd5b5051919050565b634e487b7160e01b5f52601160045260245ffd5b80820180821115611b9357611b936159db565b6001600160a01b03831681526040602082018190525f90613a099083018461510d565b5f60208284031215615a35575f5ffd5b81516001600160401b03811115615a4a575f5ffd5b8201601f81018413615a5a575f5ffd5b8051615a68614f4f82614f10565b8082825260208201915060208360051b850101925086831115615a89575f5ffd5b6020840193505b828410156108de578351615aa38161532a565b825260209384019390910190615a90565b5f823560de198336030181126157ce575f5ffd5b5f611b933683615000565b5f60208284031215615ae3575f5ffd5b8151611f5a81614bf8565b6001600160a01b039384168152919092166020820152604081019190915260600190565b5f60018201615b2357615b236159db565b5060010190565b838152606060208201525f615b426060830185615180565b82810360408401526108de8185615150565b63ffffffff8181168382160190811115611b9357611b936159db565b5f5f60408385031215615b81575f5ffd5b505080516020909101519092909150565b81810381811115611b9357611b936159db565b63ffffffff8281168282160390811115611b9357611b936159db565b6001600160a01b03841681526060602082018190525f90615be49083018561510d565b905063ffffffff83166040830152949350505050565b634e487b7160e01b5f52601260045260245ffd5b6001600160401b038281168282160390811115611b9357611b936159db565b634e487b7160e01b5f52602160045260245ffd5b634e487b7160e01b5f52603160045260245ffd5b828152604060208201525f82518060408401528060208501606085015e5f606082850101526060601f19601f8301168401019150509392505050565b5f82518060208501845e5f920191825250919050565b5f82615cc157634e487b7160e01b5f52601260045260245ffd5b50049056fea2646970667358221220b302bc35114e1921cf8bb16bbe96de3516e750fa9b91287e8e3ac447f0d9fea764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xB1W_5`\xE0\x1C\x80cw\x8EU\xF3\x11a\x01{W\x80c\xBF\xAE?\xD2\x11a\0\xE4W\x80c\xE4\xCC?\x90\x11a\0\x9EW\x80c\xF0\xE0\xE6v\x11a\0yW\x80c\xF0\xE0\xE6v\x14a\x08\x12W\x80c\xF2\xFD\xE3\x8B\x14a\x082W\x80c\xF6\x98\xDA%\x14a\x08EW\x80c\xFA\xBC\x1C\xBC\x14a\x08MW__\xFD[\x80c\xE4\xCC?\x90\x14a\x07\xD9W\x80c\xEEt\x93\x7F\x14a\x07\xECW\x80c\xEE\xA9\x06K\x14a\x07\xFFW__\xFD[\x80c\xBF\xAE?\xD2\x14a\x07$W\x80c\xC4H\xFE\xB8\x14a\x077W\x80c\xC9x\xF7\xAC\x14a\x07kW\x80c\xCA\x8A\xA7\xC7\x14a\x07\x8CW\x80c\xCDm\xC6\x87\x14a\x07\xB3W\x80c\xDA\x8B\xE8d\x14a\x07\xC6W__\xFD[\x80c\x945\xBBC\x11a\x015W\x80c\x945\xBBC\x14a\x06\x02W\x80c\x99\xF57\x1B\x14a\x06\x15W\x80c\xA1x\x84\x84\x14a\x06\xA3W\x80c\xA3:43\x14a\x06\xC2W\x80c\xB7\xF0n\xBE\x14a\x06\xD5W\x80c\xBBE\xFE\xF2\x14a\x06\xF7W__\xFD[\x80cw\x8EU\xF3\x14a\x05RW\x80cx)n\xC5\x14a\x05|W\x80c\x88o\x11\x95\x14a\x05\x8FW\x80c\x8D\xA5\xCB[\x14a\x05\xB6W\x80c\x90\x04\x13G\x14a\x05\xC7W\x80c\x91\x04\xC3\x19\x14a\x05\xE7W__\xFD[\x80cY\\jg\x11a\x02\x1DW\x80c`\xA0\xD1\xCE\x11a\x01\xD7W\x80c`\xA0\xD1\xCE\x14a\x04\xC8W\x80ce\xDA\x12d\x14a\x04\xDBW\x80cf\xD5\xBA\x93\x14a\x05\x03W\x80cmp\xF7\xAE\x14a\x05$W\x80cn\x17DH\x14a\x057W\x80cqP\x18\xA6\x14a\x05JW__\xFD[\x80cY\\jg\x14a\x04NW\x80cY{6\xDA\x14a\x04VW\x80cZ\xC8j\xB7\x14a\x04iW\x80c\\\x97Z\xBB\x14a\x04\x8CW\x80c]\xD6\x85y\x14a\x04\x94W\x80c_H\xE6g\x14a\x04\xB5W__\xFD[\x80c<e\x1C\xF2\x11a\x02nW\x80c<e\x1C\xF2\x14a\x03\x89W\x80c<\xDE\xB5\xE0\x14a\x03\x9CW\x80c>(9\x1D\x14a\x03\xCAW\x80cFW\xE2j\x14a\x03\xEDW\x80cFe\xBC\xDA\x14a\x04\x14W\x80cT\xB7\xC9l\x14a\x04;W__\xFD[\x80c\x04\xA4\xF9y\x14a\x02\xB5W\x80c\x0B\x9FHz\x14a\x02\xEFW\x80c\r\xD8\xDD\x02\x14a\x03\x02W\x80c\x13d9\xDD\x14a\x03\"W\x80c*\xA6\xD8\x88\x14a\x037W\x80c9\xB7\x0E8\x14a\x03JW[__\xFD[a\x02\xDC\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xDCa\x02\xFD6`\x04aL\x1CV[a\x08`V[a\x03\x15a\x03\x106`\x04aL\xB3V[a\x08\xE8V[`@Qa\x02\xE6\x91\x90aL\xF1V[a\x035a\x0306`\x04aM(V[a\x0B\xCAV[\0[a\x035a\x03E6`\x04aM\x8FV[a\x0C\x9FV[a\x03q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xE6V[a\x035a\x03\x976`\x04aM\xEDV[a\r\xE1V[a\x03qa\x03\xAA6`\x04aN0V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x03\xDDa\x03\xD86`\x04aN0V[a\x0F(V[`@Q\x90\x15\x15\x81R` \x01a\x02\xE6V[a\x03q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x035a\x04I6`\x04aNKV[a\x0FGV[a\x035a\x0F\xA3V[a\x02\xDCa\x04d6`\x04aP\xBCV[a\x10RV[a\x03\xDDa\x04w6`\x04aP\xEDV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x02\xDCV[a\x04\xA7a\x04\xA26`\x04aN0V[a\x10\x81V[`@Qa\x02\xE6\x92\x91\x90aRLV[a\x035a\x04\xC36`\x04aR\xB9V[a\x148V[a\x035a\x04\xD66`\x04aS>V[a\x16\xCCV[a\x03qa\x04\xE96`\x04aN0V[`\x9A` R_\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05\x16a\x05\x116`\x04aN0V[a\x18aV[`@Qa\x02\xE6\x92\x91\x90aS}V[a\x03\xDDa\x0526`\x04aN0V[a\x1BaV[a\x02\xDCa\x05E6`\x04aNKV[a\x1B\x99V[a\x035a\x1C\x9FV[a\x02\xDCa\x05`6`\x04aNKV[`\x98` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x035a\x05\x8A6`\x04aS\xA1V[a\x1C\xB0V[a\x03q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03qV[a\x05\xDAa\x05\xD56`\x04aS\xF1V[a\x1D8V[`@Qa\x02\xE6\x91\x90aT=V[a\x03qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x035a\x06\x106`\x04aTOV[a\x1E\x0EV[a\x06ea\x06#6`\x04aM(V[`\xA4` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x84\x16\x93\x90\x91\x16\x91\x90c\xFF\xFF\xFF\xFF\x16\x85V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x81R\x94\x86\x16` \x86\x01R\x92\x90\x94\x16\x91\x83\x01\x91\x90\x91R``\x82\x01Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x80\x82\x01R`\xA0\x01a\x02\xE6V[a\x02\xDCa\x06\xB16`\x04aN0V[`\x9F` R_\x90\x81R`@\x90 T\x81V[a\x03\x15a\x06\xD06`\x04aT\xEBV[a\x1E\xC4V[a\x03\xDDa\x06\xE36`\x04aM(V[`\x9E` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\xDDa\x07\x056`\x04aU\xD2V[`\x9C` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x02\xDCa\x0726`\x04aNKV[a\x1FaV[`@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x02\xE6V[a\x07~a\x07y6`\x04aS\xF1V[a\x1F\x9DV[`@Qa\x02\xE6\x92\x91\x90aU\xFCV[a\x03q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x035a\x07\xC16`\x04aU\xD2V[a\"*V[a\x03\x15a\x07\xD46`\x04aN0V[a#EV[a\x035a\x07\xE76`\x04aV\x1BV[a$\x86V[a\x035a\x07\xFA6`\x04aV\x99V[a$\xDCV[a\x035a\x08\r6`\x04aT\xEBV[a&yV[a\x08%a\x08 6`\x04aV\xE7V[a&\xDCV[`@Qa\x02\xE6\x91\x90aW\x94V[a\x035a\x08@6`\x04aN0V[a'\x81V[a\x02\xDCa'\xFAV[a\x035a\x08[6`\x04aM(V[a(\x08V[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x92\x82\x01\x92\x90\x92R\x81\x87\x16``\x82\x01R\x90\x85\x16`\x80\x82\x01R`\xA0\x81\x01\x83\x90R`\xC0\x81\x01\x82\x90R_\x90a\x08\xDE\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a)\x1FV[\x96\x95PPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x03a\t\x14W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\t-Wa\t-aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\tVW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P3_\x90\x81R`\x9A` R`@\x81 T\x91\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90[\x85\x81\x10\x15a\x0B\xBFW\x86\x86\x82\x81\x81\x10a\t\x90Wa\t\x90aW\xA6V[\x90P` \x02\x81\x01\x90a\t\xA2\x91\x90aW\xBAV[a\t\xB0\x90` \x81\x01\x90aW\xD8V[\x90P\x87\x87\x83\x81\x81\x10a\t\xC4Wa\t\xC4aW\xA6V[\x90P` \x02\x81\x01\x90a\t\xD6\x91\x90aW\xBAV[a\t\xE0\x90\x80aW\xD8V[\x90P\x14a\n\0W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3\x87\x87\x83\x81\x81\x10a\n\x13Wa\n\x13aW\xA6V[\x90P` \x02\x81\x01\x90a\n%\x91\x90aW\xBAV[a\n6\x90``\x81\x01\x90`@\x01aN0V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\n]W`@Qc0\xC4qi`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\n\xC73\x84\x8A\x8A\x86\x81\x81\x10a\nuWa\nuaW\xA6V[\x90P` \x02\x81\x01\x90a\n\x87\x91\x90aW\xBAV[a\n\x91\x90\x80aW\xD8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa)M\x92PPPV[\x90Pa\x0B\x993\x84\x8A\x8A\x86\x81\x81\x10a\n\xE0Wa\n\xE0aW\xA6V[\x90P` \x02\x81\x01\x90a\n\xF2\x91\x90aW\xBAV[a\n\xFC\x90\x80aW\xD8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x88\x90P\x81\x81\x10a\x0BAWa\x0BAaW\xA6V[\x90P` \x02\x81\x01\x90a\x0BS\x91\x90aW\xBAV[a\x0Ba\x90` \x81\x01\x90aW\xD8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa*\x94\x91PPV[\x84\x83\x81Q\x81\x10a\x0B\xABWa\x0B\xABaW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\tvV[P\x90\x95\x94PPPPPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CP\x91\x90aX\x1DV[a\x0CmW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x81\x14a\x0C\x92W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\x9B\x82a0mV[PPV[a\x0C\xA83a\x0F(V[\x15a\x0C\xC6W`@Qc;\xF2\xB5\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc+bA\xF3`\xE1\x1B\x81R3`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cV\xC4\x83\xE6\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r0W__\xFD[PZ\xF1\x15\x80\x15a\rBW=__>=_\xFD[PPPPa\rP3\x85a0\xAAV[a\rZ33a1\x0CV[`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R3\x90\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90` \x01`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\r\xD3\x92\x91\x90aX8V[`@Q\x80\x91\x03\x90\xA2PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0E@WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x0E]W`@Qc\x04R\x06\xA5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x90\x81R`\x9A` R`@\x80\x82 T\x90Qc\x15&g\xD9`\xE3\x1B\x81R\x90\x83\x16`\x04\x82\x01\x81\x90R\x86\x84\x16`$\x83\x01R\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA93>\xC8\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x01\x91\x90aXfV[\x90P_a\x0F\x0F\x87\x87\x84a2\x0FV[\x90Pa\x0F\x1F\x83\x88\x88\x88\x88\x86a2\xF1V[PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[\x81a\x0FQ\x81a4/V[a\x0FnW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Fw\x83a\x1BaV[a\x0F\x94W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x9E\x83\x83a0\xAAV[PPPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x05W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10)\x91\x90aX\x1DV[a\x10FW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10P_\x19a0mV[V[_\x81`@Q` \x01a\x10d\x91\x90aX\x81V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xA3` R`@\x81 ``\x91\x82\x91a\x10\xA6\x90a4\xD9V[\x80Q\x90\x91P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xC3Wa\x10\xC3aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xFCW\x81` \x01[a\x10\xE9aJ\xD8V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10\xE1W\x90P[P\x93P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x17Wa\x11\x17aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11JW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x115W\x90P[P`\x01`\x01`\xA0\x1B\x03\x80\x87\x16_\x90\x81R`\x9A` R`@\x81 T\x92\x95P\x91\x16\x90[\x82\x81\x10\x15a\x14/W`\xA4_\x85\x83\x81Q\x81\x10a\x11\x88Wa\x11\x88aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x83\x01T\x81\x16\x82\x86\x01R`\x02\x83\x01T\x16\x81\x84\x01R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01Tc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\x05\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x91\x94\x92\x93`\xA0\x86\x01\x93\x90\x92\x90\x83\x01\x82\x82\x80\x15a\x12BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x12$W[PPPPP\x81R` \x01`\x06\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x12\x98W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x12\x84W[PPPPP\x81RPP\x86\x82\x81Q\x81\x10a\x12\xB3Wa\x12\xB3aW\xA6V[` \x02` \x01\x01\x81\x90RP\x85\x81\x81Q\x81\x10a\x12\xD0Wa\x12\xD0aW\xA6V[` \x02` \x01\x01Q`\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xF4Wa\x12\xF4aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\x1DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x85\x82\x81Q\x81\x10a\x130Wa\x130aW\xA6V[` \x02` \x01\x01\x81\x90RP_a\x13d\x88\x84\x89\x85\x81Q\x81\x10a\x13SWa\x13SaW\xA6V[` \x02` \x01\x01Q`\xA0\x01Qa)MV[\x90P_[\x87\x83\x81Q\x81\x10a\x13zWa\x13zaW\xA6V[` \x02` \x01\x01Q`\xA0\x01QQ\x81\x10\x15a\x14%Wa\x13\xE7\x88\x84\x81Q\x81\x10a\x13\xA3Wa\x13\xA3aW\xA6V[` \x02` \x01\x01Q`\xC0\x01Q\x82\x81Q\x81\x10a\x13\xC0Wa\x13\xC0aW\xA6V[` \x02` \x01\x01Q\x83\x85\x81Q\x81\x10a\x13\xDAWa\x13\xDAaW\xA6V[` \x02` \x01\x01Qa4\xE5V[\x87\x84\x81Q\x81\x10a\x13\xF9Wa\x13\xF9aW\xA6V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x14\x12Wa\x14\x12aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13hV[PP`\x01\x01a\x11kV[PPPP\x91P\x91V[`fT`\x02\x90`\x04\x90\x81\x16\x03a\x14aW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14ia4\xF0V[3_\x90\x81R`\xA3` R`@\x81 \x90a\x14\x81\x82a5IV[\x90P\x80\x84\x11a\x14\x90W\x83a\x14\x92V[\x80[\x93P_\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xADWa\x14\xADaN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xE6W\x81` \x01[a\x14\xD3aJ\xD8V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x14\xCBW\x90P[P\x90P_[\x81Q\x81\x10\x15a\x165W`\xA4_a\x15\x01\x86\x84a5RV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x83\x01T\x81\x16\x82\x86\x01R`\x02\x83\x01T\x16\x81\x84\x01R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01Tc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\x05\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x91\x94\x92\x93`\xA0\x86\x01\x93\x90\x92\x90\x83\x01\x82\x82\x80\x15a\x15\xB1W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x15\x93W[PPPPP\x81R` \x01`\x06\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x16\x07W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x15\xF3W[PPPPP\x81RPP\x82\x82\x81Q\x81\x10a\x16\"Wa\x16\"aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x14\xEBV[P_[\x81Q\x81\x10\x15a\x16\xB6Wa\x16\xAE\x82\x82\x81Q\x81\x10a\x16VWa\x16VaW\xA6V[` \x02` \x01\x01Q\x8B\x8B\x84\x81\x81\x10a\x16pWa\x16paW\xA6V[\x90P` \x02\x81\x01\x90a\x16\x82\x91\x90aW\xD8V[\x8B\x8B\x86\x81\x81\x10a\x16\x94Wa\x16\x94aW\xA6V[\x90P` \x02\x01` \x81\x01\x90a\x16\xA9\x91\x90aX\x93V[a5]V[`\x01\x01a\x168V[PPPPa\x16\xC4`\x01`\xC9UV[PPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17\x15W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\x1E\x83a\x0F(V[\x15a\x0F\x9EW`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x9A` R`@\x80\x82 T\x90Qc\x15&g\xD9`\xE3\x1B\x81R\x90\x83\x16`\x04\x82\x01\x81\x90Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x83\x01R\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA93>\xC8\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xD9\x91\x90aXfV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x84R\x82R\x80\x83 \x81Q\x92\x83\x01\x90\x91RT\x81R\x91\x92Pa\x18?\x86a\x187`\x01`\x01`@\x1B\x03\x80\x87\x16\x90\x89\x16a9\xD7V[\x84\x91\x90a9\xEBV[\x90Pa\x0F\x1F\x84\x88s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x84a:\x11V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91_\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x94\xF6I\xDD\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xCDW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18\xF4\x91\x90\x81\x01\x90aY\tV[`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x83\x01R\x92\x94P\x90\x92P_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFE$:\x17\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19zW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x9E\x91\x90aY\xC4V[\x90P\x80_\x03a\x19\xB2WP\x90\x94\x90\x93P\x91PPV[_\x83Q`\x01a\x19\xC1\x91\x90aY\xEFV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xD8Wa\x19\xD8aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x01W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84Q`\x01a\x1A\x13\x91\x90aY\xEFV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A*Wa\x1A*aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1ASW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82\x86Q\x81Q\x81\x10a\x1A~Wa\x1A~aW\xA6V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x82\x81\x86Q\x81Q\x81\x10a\x1A\xB2Wa\x1A\xB2aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x85Q\x81\x10\x15a\x1BSW\x85\x81\x81Q\x81\x10a\x1A\xDAWa\x1A\xDAaW\xA6V[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a\x1A\xF4Wa\x1A\xF4aW\xA6V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81\x81Q\x81\x10a\x1B&Wa\x1B&aW\xA6V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1B@Wa\x1B@aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\xBFV[P\x90\x97\x90\x96P\x94PPPPPV[_`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x1B\x93WP`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x14[\x92\x91PPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x82\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x82\x81_\x81Q\x81\x10a\x1B\xD0Wa\x1B\xD0aW\xA6V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`@QcTz\xFB\x87`\xE0\x1B\x81R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cTz\xFB\x87\x90a\x1C.\x90\x88\x90\x86\x90`\x04\x01aZ\x02V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CHW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1Co\x91\x90\x81\x01\x90aZ%V[_\x81Q\x81\x10a\x1C\x80Wa\x1C\x80aW\xA6V[` \x02` \x01\x01Q\x90Pa\x1C\x96\x85\x85\x83_a:\x8BV[\x95\x94PPPPPV[a\x1C\xA7a;<V[a\x10P_a;\x96V[\x82a\x1C\xBA\x81a4/V[a\x1C\xD7W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\xE0\x84a\x1BaV[a\x1C\xFDW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\r\xD3\x92\x91\x90aX8V[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DTWa\x1DTaN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D}W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1E\x06W`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x98` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x1D\xB9Wa\x1D\xB9aW\xA6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ T\x82\x82\x81Q\x81\x10a\x1D\xF3Wa\x1D\xF3aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1D\x82V[P\x93\x92PPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a\x1E7W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E?a4\xF0V[\x85_[\x81\x81\x10\x15a\x1E\xB8Wa\x1E\xB0\x89\x89\x83\x81\x81\x10a\x1E_Wa\x1E_aW\xA6V[\x90P` \x02\x81\x01\x90a\x1Eq\x91\x90aZ\xB4V[a\x1Ez\x90aZ\xC8V[\x88\x88\x84\x81\x81\x10a\x1E\x8CWa\x1E\x8CaW\xA6V[\x90P` \x02\x81\x01\x90a\x1E\x9E\x91\x90aW\xD8V[\x88\x88\x86\x81\x81\x10a\x16\x94Wa\x16\x94aW\xA6V[`\x01\x01a\x1EBV[PPa\x0F\x1F`\x01`\xC9UV[``a\x1E\xCF3a\x0F(V[a\x1E\xECW`@Qc\xA5\xC7\xC4E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xF53a\x1BaV[\x15a\x1F\x13W`@Qc\x11\xCA35`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\x1C\x84a\x1BaV[a\x1F9W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1FB3a;\xE7V[\x90Pa\x1FP3\x85\x85\x85a>FV[a\x1FZ3\x85a1\x0CV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x83Q\x91\x82\x01\x90\x93R\x91T\x82R\x90a\x1FZ\x90a?\x0BV[``\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xB9Wa\x1F\xB9aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\xE2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xFEWa\x1F\xFEaN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a 'W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x9A` R`@\x81 T\x92\x93P\x91\x16\x90a R\x86\x83\x87a)MV[\x90P_[\x85Q\x81\x10\x15a\"\x1FW_a \x82\x87\x83\x81Q\x81\x10a uWa uaW\xA6V[` \x02` \x01\x01Qa?*V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xFE$:\x17\x89\x89\x85\x81Q\x81\x10a \xA6Wa \xA6aW\xA6V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \xE0\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x1F\x91\x90aY\xC4V[\x85\x83\x81Q\x81\x10a!1Wa!1aW\xA6V[` \x02` \x01\x01\x81\x81RPP_`\xA2_\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x89\x85\x81Q\x81\x10a!tWa!taW\xA6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90Pa!\xF8\x86\x84\x81Q\x81\x10a!\xC6Wa!\xC6aW\xA6V[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a!\xE0Wa!\xE0aW\xA6V[` \x02` \x01\x01Q\x83a9\xEB\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x84\x81Q\x81\x10a\"\nWa\"\naW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a VV[PPP[\x92P\x92\x90PV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\"HWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\"aWP0;\x15\x80\x15a\"aWP_T`\xFF\x16`\x01\x14[a\"\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\"\xEAW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\"\xF3\x82a0mV[a\"\xFC\x83a;\x96V[\x80\x15a\x0F\x9EW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[``a#P\x82a\x0F(V[a#mW`@Qc\xA5\xC7\xC4E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#v\x82a\x1BaV[\x15a#\x94W`@Qc\x11\xCA35`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a#\xBBW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x903\x14\x80a#\xE9WPa#\xE9\x81a4/V[\x80a$\x0FWP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a$,W`@Qc\x1EI\x9A#`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x84\x16\x14a$}W\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[a\x1FZ\x83a;\xE7V[`fT`\x02\x90`\x04\x90\x81\x16\x03a$\xAFW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$\xB7a4\xF0V[a$\xCBa$\xC3\x86aZ\xC8V[\x85\x85\x85a5]V[a$\xD5`\x01`\xC9UV[PPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a%%W`@Qc#\xD8q\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 Ta%c\x90`\x01`\x01`@\x1B\x03\x80\x86\x16\x90\x85\x16a?\x9CV[\x90P_a%r\x86\x86\x86\x86a:\x8BV[a%|\x90\x83aY\xEFV[\x90Pa%\x8A\x86_\x87\x85a:\x11V[`\x01`\x01`\xA0\x1B\x03\x85\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a\x16\xC4W`@Qc;\x9E\x9F\x01`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEEz|\x04\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&\x15W__\xFD[PZ\xF1\x15\x80\x15a&'W=__>=_\xFD[PP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R` \x82\x01\x86\x90R\x8A\x16\x93P\x7F\xEF\xF6\xAA\xB2\xBC?|d\x88\x96\xE1R.\xAEq\xD6\xC2.;\x0E!\x82\x06\xB3\xF4\n\xF0\xE4\xD2\x04qk\x92P\x01`@Q\x80\x91\x03\x90\xA2PPPPPPV[a&\x823a\x0F(V[\x15a&\xA0W`@Qc;\xF2\xB5\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a&\xA9\x83a\x1BaV[a&\xC6W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a&\xD23\x84\x84\x84a>FV[a\x0F\x9E3\x84a1\x0CV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xF8Wa&\xF8aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'+W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a'\x16W\x90P[P\x90P_[\x84Q\x81\x10\x15a\x1E\x06Wa'\\\x85\x82\x81Q\x81\x10a'NWa'NaW\xA6V[` \x02` \x01\x01Q\x85a\x1D8V[\x82\x82\x81Q\x81\x10a'nWa'naW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a'0V[a'\x89a;<V[`\x01`\x01`\xA0\x1B\x03\x81\x16a'\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\"\xC0V[a'\xF7\x81a;\x96V[PV[_a(\x03a?\xB4V[\x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(dW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\x88\x91\x90aZ\xD3V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a(\xB9W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x80\x19\x82\x19\x81\x16\x14a(\xE0W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPV[_a)(a?\xB4V[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01a\x10dV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a)iWa)iaN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\x92W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cTz\xFB\x87\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\xE4\x92\x91\x90aZ\x02V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xFEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra*%\x91\x90\x81\x01\x90aZ%V[\x90P_[\x84Q\x81\x10\x15a\x0B\xBFWa*o\x87\x86\x83\x81Q\x81\x10a*HWa*HaW\xA6V[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a*bWa*baW\xA6V[` \x02` \x01\x01Qa2\x0FV[\x83\x82\x81Q\x81\x10a*\x81Wa*\x81aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a*)V[_`\x01`\x01`\xA0\x1B\x03\x86\x16a*\xBCW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q_\x03a*\xDDW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xF7Wa*\xF7aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+ W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a+=Wa+=aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+fW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x86Q\x81\x10\x15a.\xA0W_a+\x8A\x88\x83\x81Q\x81\x10a uWa uaW\xA6V[\x90P_`\xA2_\x8C`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x8A\x85\x81Q\x81\x10a+\xC3Wa+\xC3aW\xA6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16c\xFE$:\x17\x8C\x8B\x86\x81Q\x81\x10a,\"Wa,\"aW\xA6V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\\\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,wW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x9B\x91\x90aY\xC4V[\x88\x84\x81Q\x81\x10a,\xADWa,\xADaW\xA6V[` \x02` \x01\x01Q\x11\x15a,\xD4W`@Qc\xF0 \xE5\xB9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a-\x03\x88\x84\x81Q\x81\x10a,\xE9Wa,\xE9aW\xA6V[` \x02` \x01\x01Q\x88\x85\x81Q\x81\x10a!\xE0Wa!\xE0aW\xA6V[\x84\x84\x81Q\x81\x10a-\x15Wa-\x15aW\xA6V[` \x02` \x01\x01\x81\x81RPPa-]\x84\x84\x81Q\x81\x10a-6Wa-6aW\xA6V[` \x02` \x01\x01Q\x88\x85\x81Q\x81\x10a-PWa-PaW\xA6V[` \x02` \x01\x01Qa@\x99V[\x85\x84\x81Q\x81\x10a-oWa-oaW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x15a.\x04Wa-\xC6\x8A\x8A\x85\x81Q\x81\x10a-\x9FWa-\x9FaW\xA6V[` \x02` \x01\x01Q\x87\x86\x81Q\x81\x10a-\xB9Wa-\xB9aW\xA6V[` \x02` \x01\x01Qa@\xB2V[a.\x04\x8A\x8C\x8B\x86\x81Q\x81\x10a-\xDDWa-\xDDaW\xA6V[` \x02` \x01\x01Q\x87\x87\x81Q\x81\x10a-\xF7Wa-\xF7aW\xA6V[` \x02` \x01\x01Qa:\x11V[\x81`\x01`\x01`\xA0\x1B\x03\x16crJ\xF4#\x8C\x8B\x86\x81Q\x81\x10a.&Wa.&aW\xA6V[` \x02` \x01\x01Q\x8B\x87\x81Q\x81\x10a.@Wa.@aW\xA6V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.f\x93\x92\x91\x90aZ\xEEV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a.}W__\xFD[PZ\xF1\x15\x80\x15a.\x8FW=__>=_\xFD[PPPPPP\x80`\x01\x01\x90Pa+kV[P`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a.\xC7\x83a[\x12V[\x91\x90PUP_`@Q\x80`\xE0\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89\x81R` \x01\x85\x81RP\x90P_a/-\x82a\x10RV[_\x81\x81R`\x9E` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x19\x90\x91\x16\x81\x17\x90\x91U`\xA4\x83R\x92\x81\x90 \x86Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x83U\x88\x85\x01Q\x95\x83\x01\x80T\x82\x16\x96\x83\x16\x96\x90\x96\x17\x90\x95U\x91\x87\x01Q`\x02\x82\x01\x80T\x90\x95\x16\x92\x16\x91\x90\x91\x17\x90\x92U``\x85\x01Q`\x03\x83\x01U`\x80\x85\x01Q`\x04\x83\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\xA0\x85\x01Q\x80Q\x93\x94P\x85\x93a/\xE3\x92`\x05\x85\x01\x92\x01\x90aK1V[P`\xC0\x82\x01Q\x80Qa/\xFF\x91`\x06\x84\x01\x91` \x90\x91\x01\x90aK\x94V[PPP`\x01`\x01`\xA0\x1B\x03\x8B\x16_\x90\x81R`\xA3` R`@\x90 a0#\x90\x82aA@V[P\x7F&\xB2\xAA\xE2e\x16\xE8q\x9E\xF5\x0E\xA2\xF6\x83\x1A.\xFB\xD4\xE3}\xCC\xDF\x0Fi6\xB2{\xC0\x8Ey>0\x81\x83\x86`@Qa0W\x93\x92\x91\x90a[*V[`@Q\x80\x91\x03\x90\xA1\x9A\x99PPPPPPPPPPV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x99` \x90\x81R`@\x91\x82\x90 `\x01\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x86\x16\x94\x85\x17\x90U\x90Q\x92\x83R\x90\x91\x7Fw;T\xC0Muo\xCC^g\x81\x11\xF7\xD70\xDE;\xE9\x81\x92\0\x07\x99\xEE\xE3\xD67\x16\x05Z\x87\xC6\x91\x01a)\x13V[`fT_\x90`\x01\x90\x81\x16\x03a14W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x87\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3__a1\x91\x85a\x18aV[\x91P\x91P_a1\xA1\x86\x86\x85a)MV[\x90P_[\x83Q\x81\x10\x15a\x0F\x1FWa2\x07\x86\x88\x86\x84\x81Q\x81\x10a1\xC5Wa1\xC5aW\xA6V[` \x02` \x01\x01Q_\x87\x86\x81Q\x81\x10a1\xE0Wa1\xE0aW\xA6V[` \x02` \x01\x01Q\x87\x87\x81Q\x81\x10a1\xFAWa1\xFAaW\xA6V[` \x02` \x01\x01Qa2\xF1V[`\x01\x01a1\xA5V[_s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xBF\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01a2\xE1W`@Qc\xA3\xD7^\t`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA3\xD7^\t\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xC1\x91\x90aXfV[\x90Pa2\xD9`\x01`\x01`@\x1B\x03\x84\x81\x16\x90\x83\x16a9\xD7V[\x91PPa\x1FZV[P`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[\x80_\x03a3\x11W`@Qc\n3\xBCi`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R a3A\x81\x85\x85\x85aAKV[`@\x80Q` \x81\x01\x90\x91R\x81T\x81R\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x90\x87\x90\x87\x90a3\x7F\x90a?\x0BV[`@Qa3\x8E\x93\x92\x91\x90aZ\xEEV[`@Q\x80\x91\x03\x90\xA1a3\x9F\x86a\x0F(V[\x15a\x0F\x1FW`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 \x80T\x85\x92\x90a3\xDA\x90\x84\x90aY\xEFV[\x92PP\x81\x90UP\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x87\x87\x86`@Qa4\x1E\x93\x92\x91\x90aZ\xEEV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a4\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x93\x91\x90aX\x1DV[``_a\x1FZ\x83aA\xC0V[_a\x1FZ\x83\x83a9\xD7V[`\x02`\xC9T\x03a5BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\"\xC0V[`\x02`\xC9UV[_a\x1B\x93\x82T\x90V[_a\x1FZ\x83\x83aB\x19V[`\xA0\x84\x01QQ\x82\x14a5\x82W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a5\xB8W`@Qc\x16\x11\r5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a5\xC2\x85a\x10RV[_\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a5\xF3W`@Qc\x87\xC9\xD2\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87`\x80\x01Qa6&\x91\x90a[TV[\x90PCc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11\x15a6UW`@Qcx\xF6z\xE1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a6l\x87_\x01Q\x88` \x01Q\x89`\xA0\x01Q\x84aB?V[\x87Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x81 T\x8AQ`\xA0\x8C\x01Q\x94\x96P\x92\x16\x93P\x91a6\xA2\x91\x90\x84\x90a)MV[\x90P_[\x88`\xA0\x01QQ\x81\x10\x15a8\xF5W_a6\xCD\x8A`\xA0\x01Q\x83\x81Q\x81\x10a uWa uaW\xA6V[\x90P_a7\x03\x8B`\xC0\x01Q\x84\x81Q\x81\x10a6\xE9Wa6\xE9aW\xA6V[` \x02` \x01\x01Q\x87\x85\x81Q\x81\x10a\x13\xDAWa\x13\xDAaW\xA6V[\x90P\x87\x15a7\xD3W\x81`\x01`\x01`\xA0\x1B\x03\x16c.\xAEA\x8C\x8C_\x01Q\x8D`\xA0\x01Q\x86\x81Q\x81\x10a74Wa74aW\xA6V[` \x02` \x01\x01Q\x8D\x8D\x88\x81\x81\x10a7NWa7NaW\xA6V[\x90P` \x02\x01` \x81\x01\x90a7c\x91\x90aN0V[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x90\x91\x16`D\x82\x01R`d\x81\x01\x84\x90R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a7\xB8W__\xFD[PZ\xF1\x15\x80\x15a7\xCAW=__>=_\xFD[PPPPa8\xEBV[__\x83`\x01`\x01`\xA0\x1B\x03\x16c\xC4b>\xA1\x8E_\x01Q\x8F`\xA0\x01Q\x88\x81Q\x81\x10a7\xFEWa7\xFEaW\xA6V[` \x02` \x01\x01Q\x8F\x8F\x8A\x81\x81\x10a8\x18Wa8\x18aW\xA6V[\x90P` \x02\x01` \x81\x01\x90a8-\x91\x90aN0V[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x90\x91\x16`D\x82\x01R`d\x81\x01\x86\x90R`\x84\x01`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a8\x86W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xAA\x91\x90a[pV[\x91P\x91Pa8\xE8\x87\x8E_\x01Q\x8F`\xA0\x01Q\x88\x81Q\x81\x10a8\xCCWa8\xCCaW\xA6V[` \x02` \x01\x01Q\x85\x85\x8B\x8B\x81Q\x81\x10a1\xFAWa1\xFAaW\xA6V[PP[PP`\x01\x01a6\xA6V[P\x87Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\xA3` R`@\x90 a9\x18\x90\x85aCmV[P_\x84\x81R`\xA4` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x82U`\x01\x82\x01\x80T\x82\x16\x90U`\x02\x82\x01\x80T\x90\x91\x16\x90U`\x03\x81\x01\x82\x90U`\x04\x81\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90U\x90a9o`\x05\x83\x01\x82aK\xCDV[a9|`\x06\x83\x01_aK\xCDV[PP_\x84\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x7F\x1F@@\x08\x89'N\xD0{$\x84^PT\xA8z\x0C\xAB\x96\x9E\xB1'z\xAF\xE6\x1A\xE3R\xE7\xC3*\0\x90a9\xC5\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_a\x1FZ\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aCxV[_a:\t\x82a:\x03a9\xFC\x87a?\x0BV[\x86\x90a9\xD7V[\x90a9\xD7V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a:G\x90\x84\x90a[\x92V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\r\xD3\x93\x92\x91\x90aZ\xEEV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x81\x90a:\xBC\x90aD]V[\x90P_a;\x16a:\xEC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ca[\xA5V[`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x8C\x16\x83R\x92\x90R \x90aDwV[\x90P_a;#\x82\x84a[\x92V[\x90Pa;0\x81\x87\x87aD\x93V[\x98\x97PPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\"\xC0V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x03a<\x13W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x90Q\x93\x16\x92\x83\x92\x91\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv\x91\xA3__a<r\x86a\x18aV[\x91P\x91P\x81Q_\x03a<\x86WPPPa>@V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a<\x9FWa<\x9FaN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\xC8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P_a<\xD7\x87\x85\x85a)MV[\x90P_[\x83Q\x81\x10\x15a>:W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x86\x84\x81Q\x81\x10a=[Wa=[aW\xA6V[` \x02` \x01\x01Q\x83_\x81Q\x81\x10a=uWa=uaW\xA6V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x85\x84\x81Q\x81\x10a=\xA7Wa=\xA7aW\xA6V[` \x02` \x01\x01Q\x82_\x81Q\x81\x10a=\xC1Wa=\xC1aW\xA6V[` \x02` \x01\x01\x81\x81RPP\x84\x84\x81Q\x81\x10a=\xDFWa=\xDFaW\xA6V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a=\xF9Wa=\xF9aW\xA6V[` \x02` \x01\x01\x81\x81RPPa>\x12\x8B\x89\x85\x85\x85a*\x94V[\x8A\x85\x81Q\x81\x10a>$Wa>$aW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a<\xDBV[PPPPP[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80a>nWPa?\x05V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T`\xFF\x16\x15a>\xB2W`@Qc\rLL\x91`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x90\x91 \x80T`\xFF\x19\x16`\x01\x17\x90U\x83\x01Qa$\xD5\x90\x82\x90a>\xF9\x90\x88\x90\x88\x90\x84\x90\x88\x90a\x08`V[\x85Q` \x87\x01QaD\xB1V[PPPPV[\x80Q_\x90\x15a?\x1BW\x81Qa\x1B\x93V[g\r\xE0\xB6\xB3\xA7d\0\0\x92\x91PPV[_`\x01`\x01`\xA0\x1B\x03\x82\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a?uW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1B\x93V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91PPV[_a?\xAA\x84\x83\x85`\x01aE\x03V[a:\t\x90\x85a[\x92V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a@tWP`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[_\x81_\x03a@\xA8WP_a\x1B\x93V[a\x1FZ\x83\x83aERV[`\x01`\x01`\xA0\x1B\x03\x82\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a\x0F\x9EW`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 aA\x05\x90aD]V[\x90Pa?\x05CaA\x15\x84\x84aY\xEFV[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x91\x90aEfV[_a\x1FZ\x83\x83aEqV[\x82_\x03aAkWaAdg\r\xE0\xB6\xB3\xA7d\0\0\x82aERV[\x84Ua?\x05V[`@\x80Q` \x81\x01\x90\x91R\x84T\x81R_\x90aA\x87\x90\x85\x84a9\xEBV[\x90P_aA\x94\x84\x83aY\xEFV[\x90P_aA\xB5\x84aA\xAFaA\xA8\x88\x8AaY\xEFV[\x85\x90aERV[\x90aERV[\x87UPPPPPPPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aB\rW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aA\xF9W[PPPPP\x90P\x91\x90PV[_\x82_\x01\x82\x81T\x81\x10aB.WaB.aW\xA6V[\x90_R` _ \x01T\x90P\x92\x91PPV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15aB[WaB[aN\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aB\x84W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x94\xD7\xD0\x0C\x87\x87\x87`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aB\xD8\x93\x92\x91\x90a[\xC1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\xF2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@RaC\x19\x91\x90\x81\x01\x90aZ%V[\x90P_[\x85Q\x81\x10\x15aCaWaC<\x88\x87\x83\x81Q\x81\x10a*HWa*HaW\xA6V[\x83\x82\x81Q\x81\x10aCNWaCNaW\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01aC\x1DV[P\x90\x96\x95PPPPPPV[_a\x1FZ\x83\x83aE\xBDV[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aC\xAFW\x83\x82\x81aC\xA5WaC\xA5a[\xFAV[\x04\x92PPPa\x1FZV[\x80\x84\x11aC\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\"\xC0V[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[_aDh\x82\x82aF\xA0V[`\x01`\x01`\xE0\x1B\x03\x16\x92\x91PPV[_aD\x83\x83\x83\x83aF\xE5V[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[_a:\taD\xA1\x83\x85a\\\x0EV[\x85\x90`\x01`\x01`@\x1B\x03\x16a9\xD7V[B\x81\x10\x15aD\xD2W`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aD\xE6`\x01`\x01`\xA0\x1B\x03\x85\x16\x84\x84aG.V[a?\x05W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__aE\x10\x86\x86\x86aCxV[\x90P`\x01\x83`\x02\x81\x11\x15aE&WaE&a\\-V[\x14\x80\x15aEBWP_\x84\x80aE=WaE=a[\xFAV[\x86\x88\t\x11[\x15a\x1C\x96Wa\x08\xDE`\x01\x82aY\xEFV[_a\x1FZ\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84aCxV[a\x0F\x9E\x83\x83\x83aG\x82V[_\x81\x81R`\x01\x83\x01` R`@\x81 TaE\xB6WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x1B\x93V[P_a\x1B\x93V[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aF\x97W_aE\xDF`\x01\x83a[\x92V[\x85T\x90\x91P_\x90aE\xF2\x90`\x01\x90a[\x92V[\x90P\x81\x81\x14aFQW_\x86_\x01\x82\x81T\x81\x10aF\x10WaF\x10aW\xA6V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10aF0WaF0aW\xA6V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aFbWaFba\\AV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x1B\x93V[_\x91PPa\x1B\x93V[\x81T_\x90\x80\x15aF\xDDWaF\xC6\x84aF\xB9`\x01\x84a[\x92V[_\x91\x82R` \x90\x91 \x01\x90V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a:\tV[P\x90\x92\x91PPV[\x82T_\x90\x81aF\xF6\x86\x86\x83\x85aH\x88V[\x90P\x80\x15aG$WaG\r\x86aF\xB9`\x01\x84a[\x92V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x08\xDEV[P\x91\x94\x93PPPPV[___aG;\x85\x85aH\xDBV[\x90\x92P\x90P_\x81`\x04\x81\x11\x15aGSWaGSa\\-V[\x14\x80\x15aGqWP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\x08\xDEWPa\x08\xDE\x86\x86\x86aI\x1AV[\x82T\x80\x15aH:W_aG\x9A\x85aF\xB9`\x01\x85a[\x92V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84Rd\x01\0\0\0\0\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x85\x16\x10\x15aG\xEDW`@Qc\x15\x1B\x8E?`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qc\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x03aH8W\x82aH\x0E\x86aF\xB9`\x01\x86a[\x92V[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[P[P`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x81R`\x01`\x01`\xE0\x1B\x03\x91\x82\x16` \x80\x83\x01\x91\x82R\x85T`\x01\x81\x01\x87U_\x96\x87R\x95 \x91Q\x90Q\x90\x92\x16d\x01\0\0\0\0\x02\x91\x90\x92\x16\x17\x91\x01UV[_[\x81\x83\x10\x15a\x1E\x06W_aH\x9D\x84\x84aJ\x01V[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15aH\xC7W\x80\x92PaH\xD5V[aH\xD2\x81`\x01aY\xEFV[\x93P[PaH\x8AV[__\x82Q`A\x03aI\x0FW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1AaI\x03\x87\x82\x85\x85aJ\x1BV[\x94P\x94PPPPa\"#V[P_\x90P`\x02a\"#V[___\x85`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x86\x86`@Q`$\x01aIB\x92\x91\x90a\\UV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90QaI\x80\x91\x90a\\\x91V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14aI\xB8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aI\xBDV[``\x91P[P\x91P\x91P\x81\x80\x15aI\xD1WP` \x81Q\x10\x15[\x80\x15a\x08\xDEWP\x80Qc\x0B\x13]?`\xE1\x1B\x90aI\xF6\x90\x83\x01` \x90\x81\x01\x90\x84\x01aY\xC4V[\x14\x96\x95PPPPPPV[_aJ\x0F`\x02\x84\x84\x18a\\\xA7V[a\x1FZ\x90\x84\x84\x16aY\xEFV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aJPWP_\x90P`\x03aJ\xCFV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aJ\xA1W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aJ\xC9W_`\x01\x92P\x92PPaJ\xCFV[\x91P_\x90P[\x94P\x94\x92PPPV[`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aK\x84W\x91` \x02\x82\x01[\x82\x81\x11\x15aK\x84W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90aKOV[PaK\x90\x92\x91PaK\xE4V[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aK\x84W\x91` \x02\x82\x01[\x82\x81\x11\x15aK\x84W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aK\xB2V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a'\xF7\x91\x90[[\x80\x82\x11\x15aK\x90W_\x81U`\x01\x01aK\xE5V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\xF7W__\xFD[\x805aL\x17\x81aK\xF8V[\x91\x90PV[_____`\xA0\x86\x88\x03\x12\x15aL0W__\xFD[\x855aL;\x81aK\xF8V[\x94P` \x86\x015aLK\x81aK\xF8V[\x93P`@\x86\x015aL[\x81aK\xF8V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[__\x83`\x1F\x84\x01\x12aL\x83W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x99W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\"#W__\xFD[__` \x83\x85\x03\x12\x15aL\xC4W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xD9W__\xFD[aL\xE5\x85\x82\x86\x01aLsV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x0B\xBFW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aM\nV[_` \x82\x84\x03\x12\x15aM8W__\xFD[P5\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14aL\x17W__\xFD[__\x83`\x1F\x84\x01\x12aMbW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aMxW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\"#W__\xFD[____``\x85\x87\x03\x12\x15aM\xA2W__\xFD[\x845aM\xAD\x81aK\xF8V[\x93PaM\xBB` \x86\x01aM?V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xD5W__\xFD[aM\xE1\x87\x82\x88\x01aMRV[\x95\x98\x94\x97P\x95PPPPV[____`\x80\x85\x87\x03\x12\x15aN\0W__\xFD[\x845aN\x0B\x81aK\xF8V[\x93P` \x85\x015aN\x1B\x81aK\xF8V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[_` \x82\x84\x03\x12\x15aN@W__\xFD[\x815a\x1FZ\x81aK\xF8V[__`@\x83\x85\x03\x12\x15aN\\W__\xFD[\x825aNg\x81aK\xF8V[\x91P` \x83\x015aNw\x81aK\xF8V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xB8WaN\xB8aN\x82V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xB8WaN\xB8aN\x82V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aO\x08WaO\x08aN\x82V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aO(WaO(aN\x82V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aOAW__\xFD[\x815aOTaOO\x82aO\x10V[aN\xE0V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aOuW__\xFD[` \x85\x01[\x83\x81\x10\x15aO\x9BW\x805aO\x8D\x81aK\xF8V[\x83R` \x92\x83\x01\x92\x01aOzV[P\x95\x94PPPPPV[_\x82`\x1F\x83\x01\x12aO\xB4W__\xFD[\x815aO\xC2aOO\x82aO\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aO\xE3W__\xFD[` \x85\x01[\x83\x81\x10\x15aO\x9BW\x805\x83R` \x92\x83\x01\x92\x01aO\xE8V[_`\xE0\x82\x84\x03\x12\x15aP\x10W__\xFD[aP\x18aN\x96V[\x90PaP#\x82aL\x0CV[\x81RaP1` \x83\x01aL\x0CV[` \x82\x01RaPB`@\x83\x01aL\x0CV[`@\x82\x01R``\x82\x81\x015\x90\x82\x01RaP]`\x80\x83\x01aM?V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aPzW__\xFD[aP\x86\x84\x82\x85\x01aO2V[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xA4W__\xFD[aP\xB0\x84\x82\x85\x01aO\xA5V[`\xC0\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aP\xCCW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xE1W__\xFD[a:\t\x84\x82\x85\x01aP\0V[_` \x82\x84\x03\x12\x15aP\xFDW__\xFD[\x815`\xFF\x81\x16\x81\x14a\x1FZW__\xFD[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aQFW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aQ\x1FV[P\x93\x94\x93PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aQFW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aQbV[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Q_\x91aQ\xCB\x90\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaQ\xE5`\xE0\x85\x01\x82aQ\rV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x1C\x96\x82\x82aQPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aCaW`\x1F\x19\x85\x84\x03\x01\x88RaR6\x83\x83QaQPV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aR\x1AV[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15aR\xA3W`_\x19\x87\x86\x03\x01\x84RaR\x8E\x85\x83QaQ\x80V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aRrV[PPPP\x82\x81\x03` \x84\x01Ra\x1C\x96\x81\x85aQ\xFEV[_____``\x86\x88\x03\x12\x15aR\xCDW__\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xE2W__\xFD[aR\xEE\x88\x82\x89\x01aLsV[\x90\x96P\x94PP` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x0CW__\xFD[aS\x18\x88\x82\x89\x01aLsV[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a'\xF7W__\xFD[___``\x84\x86\x03\x12\x15aSPW__\xFD[\x835aS[\x81aK\xF8V[\x92P` \x84\x015\x91P`@\x84\x015aSr\x81aS*V[\x80\x91PP\x92P\x92P\x92V[`@\x81R_aS\x8F`@\x83\x01\x85aQ\rV[\x82\x81\x03` \x84\x01Ra\x1C\x96\x81\x85aQPV[___`@\x84\x86\x03\x12\x15aS\xB3W__\xFD[\x835aS\xBE\x81aK\xF8V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xD8W__\xFD[aS\xE4\x86\x82\x87\x01aMRV[\x94\x97\x90\x96P\x93\x94PPPPV[__`@\x83\x85\x03\x12\x15aT\x02W__\xFD[\x825aT\r\x81aK\xF8V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT'W__\xFD[aT3\x85\x82\x86\x01aO2V[\x91PP\x92P\x92\x90PV[` \x81R_a\x1FZ` \x83\x01\x84aQPV[______``\x87\x89\x03\x12\x15aTdW__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15aTyW__\xFD[aT\x85\x89\x82\x8A\x01aLsV[\x90\x97P\x95PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xA3W__\xFD[aT\xAF\x89\x82\x8A\x01aLsV[\x90\x95P\x93PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xCDW__\xFD[aT\xD9\x89\x82\x8A\x01aLsV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[___``\x84\x86\x03\x12\x15aT\xFDW__\xFD[\x835aU\x08\x81aK\xF8V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU\"W__\xFD[\x84\x01`@\x81\x87\x03\x12\x15aU3W__\xFD[aU;aN\xBEV[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aUPW__\xFD[\x82\x01`\x1F\x81\x01\x88\x13aU`W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aUyWaUyaN\x82V[aU\x8C`\x1F\x82\x01`\x1F\x19\x16` \x01aN\xE0V[\x81\x81R\x89` \x83\x85\x01\x01\x11\x15aU\xA0W__\xFD[\x81` \x84\x01` \x83\x017_` \x92\x82\x01\x83\x01R\x83R\x92\x83\x015\x92\x82\x01\x92\x90\x92R\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[__`@\x83\x85\x03\x12\x15aU\xE3W__\xFD[\x825aU\xEE\x81aK\xF8V[\x94` \x93\x90\x93\x015\x93PPPV[`@\x81R_aS\x8F`@\x83\x01\x85aQPV[\x80\x15\x15\x81\x14a'\xF7W__\xFD[____``\x85\x87\x03\x12\x15aV.W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15aVCW__\xFD[\x85\x01`\xE0\x81\x88\x03\x12\x15aVTW__\xFD[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aVnW__\xFD[aVz\x87\x82\x88\x01aLsV[\x90\x94P\x92PP`@\x85\x015aV\x8E\x81aV\x0EV[\x93\x96\x92\x95P\x90\x93PPV[____`\x80\x85\x87\x03\x12\x15aV\xACW__\xFD[\x845aV\xB7\x81aK\xF8V[\x93P` \x85\x015aV\xC7\x81aK\xF8V[\x92P`@\x85\x015aV\xD7\x81aS*V[\x91P``\x85\x015aV\x8E\x81aS*V[__`@\x83\x85\x03\x12\x15aV\xF8W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aW\rW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aW\x1DW__\xFD[\x805aW+aOO\x82aO\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aWLW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aWwW\x835aWf\x81aK\xF8V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aWSV[\x94PPPP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT'W__\xFD[` \x81R_a\x1FZ` \x83\x01\x84aQ\xFEV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`^\x19\x836\x03\x01\x81\x12aW\xCEW__\xFD[\x91\x90\x91\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aW\xEDW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aX\x06W__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\"#W__\xFD[_` \x82\x84\x03\x12\x15aX-W__\xFD[\x81Qa\x1FZ\x81aV\x0EV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[_` \x82\x84\x03\x12\x15aXvW__\xFD[\x81Qa\x1FZ\x81aS*V[` \x81R_a\x1FZ` \x83\x01\x84aQ\x80V[_` \x82\x84\x03\x12\x15aX\xA3W__\xFD[\x815a\x1FZ\x81aV\x0EV[_\x82`\x1F\x83\x01\x12aX\xBDW__\xFD[\x81QaX\xCBaOO\x82aO\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aX\xECW__\xFD[` \x85\x01[\x83\x81\x10\x15aO\x9BW\x80Q\x83R` \x92\x83\x01\x92\x01aX\xF1V[__`@\x83\x85\x03\x12\x15aY\x1AW__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aY/W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aY?W__\xFD[\x80QaYMaOO\x82aO\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aYnW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aY\x99W\x83QaY\x88\x81aK\xF8V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aYuV[\x80\x95PPPPP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aY\xB8W__\xFD[aT3\x85\x82\x86\x01aX\xAEV[_` \x82\x84\x03\x12\x15aY\xD4W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1B\x93Wa\x1B\x93aY\xDBV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a:\t\x90\x83\x01\x84aQ\rV[_` \x82\x84\x03\x12\x15aZ5W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZJW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aZZW__\xFD[\x80QaZhaOO\x82aO\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aZ\x89W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x08\xDEW\x83QaZ\xA3\x81aS*V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aZ\x90V[_\x825`\xDE\x19\x836\x03\x01\x81\x12aW\xCEW__\xFD[_a\x1B\x936\x83aP\0V[_` \x82\x84\x03\x12\x15aZ\xE3W__\xFD[\x81Qa\x1FZ\x81aK\xF8V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_`\x01\x82\x01a[#Wa[#aY\xDBV[P`\x01\x01\x90V[\x83\x81R``` \x82\x01R_a[B``\x83\x01\x85aQ\x80V[\x82\x81\x03`@\x84\x01Ra\x08\xDE\x81\x85aQPV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1B\x93Wa\x1B\x93aY\xDBV[__`@\x83\x85\x03\x12\x15a[\x81W__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x81\x81\x03\x81\x81\x11\x15a\x1B\x93Wa\x1B\x93aY\xDBV[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1B\x93Wa\x1B\x93aY\xDBV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R_\x90a[\xE4\x90\x83\x01\x85aQ\rV[\x90Pc\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1B\x93Wa\x1B\x93aY\xDBV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[\x82\x81R`@` \x82\x01R_\x82Q\x80`@\x84\x01R\x80` \x85\x01``\x85\x01^_``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x93\x92PPPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_\x82a\\\xC1WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xB3\x02\xBC5\x11N\x19!\xCF\x8B\xB1k\xBE\x96\xDE5\x16\xE7P\xFA\x9B\x91(~\x8E:\xC4G\xF0\xD9\xFE\xA7dsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `ActivelyDelegated()` and selector `0x77e56a06`.
```solidity
error ActivelyDelegated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ActivelyDelegated {}
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
        impl ::core::convert::From<ActivelyDelegated> for UnderlyingRustTuple<'_> {
            fn from(value: ActivelyDelegated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ActivelyDelegated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ActivelyDelegated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ActivelyDelegated()";
            const SELECTOR: [u8; 4] = [119u8, 229u8, 106u8, 6u8];
            #[inline]
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
    /**Custom error with signature `CallerCannotUndelegate()` and selector `0x3c933446`.
```solidity
error CallerCannotUndelegate();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CallerCannotUndelegate {}
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
        impl ::core::convert::From<CallerCannotUndelegate> for UnderlyingRustTuple<'_> {
            fn from(value: CallerCannotUndelegate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CallerCannotUndelegate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CallerCannotUndelegate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CallerCannotUndelegate()";
            const SELECTOR: [u8; 4] = [60u8, 147u8, 52u8, 70u8];
            #[inline]
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
    /**Custom error with signature `FullySlashed()` and selector `0x28cef1a4`.
```solidity
error FullySlashed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FullySlashed {}
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
        impl ::core::convert::From<FullySlashed> for UnderlyingRustTuple<'_> {
            fn from(value: FullySlashed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FullySlashed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FullySlashed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FullySlashed()";
            const SELECTOR: [u8; 4] = [40u8, 206u8, 241u8, 164u8];
            #[inline]
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
    /**Custom error with signature `InputArrayLengthZero()` and selector `0x796cc525`.
```solidity
error InputArrayLengthZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthZero {}
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
        impl ::core::convert::From<InputArrayLengthZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputArrayLengthZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthZero()";
            const SELECTOR: [u8; 4] = [121u8, 108u8, 197u8, 37u8];
            #[inline]
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
    /**Custom error with signature `InvalidPermissions()` and selector `0x932d94f7`.
```solidity
error InvalidPermissions();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPermissions {}
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
        impl ::core::convert::From<InvalidPermissions> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPermissions) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPermissions {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPermissions {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPermissions()";
            const SELECTOR: [u8; 4] = [147u8, 45u8, 148u8, 247u8];
            #[inline]
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
    /**Custom error with signature `InvalidSnapshotOrdering()` and selector `0x2a371c7e`.
```solidity
error InvalidSnapshotOrdering();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSnapshotOrdering {}
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
        impl ::core::convert::From<InvalidSnapshotOrdering> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSnapshotOrdering) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSnapshotOrdering {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSnapshotOrdering {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSnapshotOrdering()";
            const SELECTOR: [u8; 4] = [42u8, 55u8, 28u8, 126u8];
            #[inline]
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
    /**Custom error with signature `NotActivelyDelegated()` and selector `0xa5c7c445`.
```solidity
error NotActivelyDelegated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotActivelyDelegated {}
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
        impl ::core::convert::From<NotActivelyDelegated> for UnderlyingRustTuple<'_> {
            fn from(value: NotActivelyDelegated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotActivelyDelegated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotActivelyDelegated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotActivelyDelegated()";
            const SELECTOR: [u8; 4] = [165u8, 199u8, 196u8, 69u8];
            #[inline]
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
    /**Custom error with signature `OnlyEigenPodManager()` and selector `0xc84e9984`.
```solidity
error OnlyEigenPodManager();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEigenPodManager {}
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
        impl ::core::convert::From<OnlyEigenPodManager> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEigenPodManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyEigenPodManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEigenPodManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEigenPodManager()";
            const SELECTOR: [u8; 4] = [200u8, 78u8, 153u8, 132u8];
            #[inline]
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
    /**Custom error with signature `OnlyStrategyManagerOrEigenPodManager()` and selector `0x11481a94`.
```solidity
error OnlyStrategyManagerOrEigenPodManager();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyStrategyManagerOrEigenPodManager {}
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
        impl ::core::convert::From<OnlyStrategyManagerOrEigenPodManager>
        for UnderlyingRustTuple<'_> {
            fn from(value: OnlyStrategyManagerOrEigenPodManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OnlyStrategyManagerOrEigenPodManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyStrategyManagerOrEigenPodManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyStrategyManagerOrEigenPodManager()";
            const SELECTOR: [u8; 4] = [17u8, 72u8, 26u8, 148u8];
            #[inline]
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
    /**Custom error with signature `OperatorsCannotUndelegate()` and selector `0x8e5199a8`.
```solidity
error OperatorsCannotUndelegate();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorsCannotUndelegate {}
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
        impl ::core::convert::From<OperatorsCannotUndelegate>
        for UnderlyingRustTuple<'_> {
            fn from(value: OperatorsCannotUndelegate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorsCannotUndelegate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorsCannotUndelegate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorsCannotUndelegate()";
            const SELECTOR: [u8; 4] = [142u8, 81u8, 153u8, 168u8];
            #[inline]
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
    /**Custom error with signature `SaltSpent()` and selector `0x35313244`.
```solidity
error SaltSpent();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SaltSpent {}
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
        impl ::core::convert::From<SaltSpent> for UnderlyingRustTuple<'_> {
            fn from(value: SaltSpent) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SaltSpent {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SaltSpent {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SaltSpent()";
            const SELECTOR: [u8; 4] = [53u8, 49u8, 50u8, 68u8];
            #[inline]
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
    /**Custom error with signature `WithdrawalDelayNotElapsed()` and selector `0xf1ecf5c2`.
```solidity
error WithdrawalDelayNotElapsed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalDelayNotElapsed {}
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
        impl ::core::convert::From<WithdrawalDelayNotElapsed>
        for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalDelayNotElapsed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for WithdrawalDelayNotElapsed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalDelayNotElapsed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawalDelayNotElapsed()";
            const SELECTOR: [u8; 4] = [241u8, 236u8, 245u8, 194u8];
            #[inline]
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
    /**Custom error with signature `WithdrawalExceedsMax()` and selector `0xf020e5b9`.
```solidity
error WithdrawalExceedsMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalExceedsMax {}
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
        impl ::core::convert::From<WithdrawalExceedsMax> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalExceedsMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawalExceedsMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalExceedsMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawalExceedsMax()";
            const SELECTOR: [u8; 4] = [240u8, 32u8, 229u8, 185u8];
            #[inline]
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
    /**Custom error with signature `WithdrawalNotQueued()` and selector `0x87c9d219`.
```solidity
error WithdrawalNotQueued();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalNotQueued {}
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
        impl ::core::convert::From<WithdrawalNotQueued> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalNotQueued) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawalNotQueued {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalNotQueued {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawalNotQueued()";
            const SELECTOR: [u8; 4] = [135u8, 201u8, 210u8, 25u8];
            #[inline]
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
    /**Custom error with signature `WithdrawerNotCaller()` and selector `0x584434d4`.
```solidity
error WithdrawerNotCaller();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawerNotCaller {}
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
        impl ::core::convert::From<WithdrawerNotCaller> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawerNotCaller) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawerNotCaller {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawerNotCaller {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawerNotCaller()";
            const SELECTOR: [u8; 4] = [88u8, 68u8, 52u8, 212u8];
            #[inline]
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
    /**Custom error with signature `WithdrawerNotStaker()` and selector `0xc311c5a4`.
```solidity
error WithdrawerNotStaker();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawerNotStaker {}
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
        impl ::core::convert::From<WithdrawerNotStaker> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawerNotStaker) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawerNotStaker {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawerNotStaker {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawerNotStaker()";
            const SELECTOR: [u8; 4] = [195u8, 17u8, 197u8, 164u8];
            #[inline]
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
    /**Event with signature `DelegationApproverUpdated(address,address)` and selector `0x773b54c04d756fcc5e678111f7d730de3be98192000799eee3d63716055a87c6`.
```solidity
event DelegationApproverUpdated(address indexed operator, address newDelegationApprover);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelegationApproverUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newDelegationApprover: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for DelegationApproverUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegationApproverUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                119u8,
                59u8,
                84u8,
                192u8,
                77u8,
                117u8,
                111u8,
                204u8,
                94u8,
                103u8,
                129u8,
                17u8,
                247u8,
                215u8,
                48u8,
                222u8,
                59u8,
                233u8,
                129u8,
                146u8,
                0u8,
                7u8,
                153u8,
                238u8,
                227u8,
                214u8,
                55u8,
                22u8,
                5u8,
                90u8,
                135u8,
                198u8,
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
                    newDelegationApprover: data.0,
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
                        &self.newDelegationApprover,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegationApproverUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegationApproverUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DelegationApproverUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DepositScalingFactorUpdated(address,address,uint256)` and selector `0x8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f`.
```solidity
event DepositScalingFactorUpdated(address staker, address strategy, uint256 newDepositScalingFactor);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DepositScalingFactorUpdated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newDepositScalingFactor: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for DepositScalingFactorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DepositScalingFactorUpdated(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                233u8,
                50u8,
                186u8,
                197u8,
                69u8,
                97u8,
                242u8,
                114u8,
                96u8,
                249u8,
                84u8,
                99u8,
                217u8,
                184u8,
                171u8,
                55u8,
                224u8,
                107u8,
                40u8,
                66u8,
                229u8,
                238u8,
                36u8,
                4u8,
                21u8,
                124u8,
                193u8,
                61u8,
                246u8,
                235u8,
                143u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: data.0,
                    strategy: data.1,
                    newDepositScalingFactor: data.2,
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newDepositScalingFactor,
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
        impl alloy_sol_types::private::IntoLogData for DepositScalingFactorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DepositScalingFactorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DepositScalingFactorUpdated,
            ) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `OperatorMetadataURIUpdated(address,string)` and selector `0x02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b6708090`.
```solidity
event OperatorMetadataURIUpdated(address indexed operator, string metadataURI);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorMetadataURIUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub metadataURI: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for OperatorMetadataURIUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorMetadataURIUpdated(address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                2u8,
                169u8,
                25u8,
                237u8,
                14u8,
                42u8,
                202u8,
                209u8,
                221u8,
                144u8,
                241u8,
                126u8,
                242u8,
                250u8,
                74u8,
                229u8,
                70u8,
                46u8,
                225u8,
                51u8,
                145u8,
                112u8,
                3u8,
                74u8,
                133u8,
                49u8,
                204u8,
                164u8,
                182u8,
                112u8,
                128u8,
                144u8,
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
                    metadataURI: data.0,
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
                        &self.metadataURI,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorMetadataURIUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorMetadataURIUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorMetadataURIUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,address)` and selector `0xa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1`.
```solidity
event OperatorRegistered(address indexed operator, address delegationApprover);
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
        pub delegationApprover: alloy::sol_types::private::Address,
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
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8,
                83u8,
                219u8,
                97u8,
                42u8,
                245u8,
                158u8,
                85u8,
                33u8,
                214u8,
                171u8,
                146u8,
                132u8,
                220u8,
                62u8,
                45u8,
                6u8,
                175u8,
                40u8,
                110u8,
                177u8,
                177u8,
                183u8,
                183u8,
                113u8,
                252u8,
                228u8,
                113u8,
                108u8,
                25u8,
                242u8,
                193u8,
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
                    delegationApprover: data.0,
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
                        &self.delegationApprover,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
    /**Event with signature `OperatorSharesBurned(address,address,uint256)` and selector `0xeff6aab2bc3f7c648896e1522eae71d6c22e3b0e218206b3f40af0e4d204716b`.
```solidity
event OperatorSharesBurned(address indexed operator, address strategy, uint256 shares);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSharesBurned {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OperatorSharesBurned {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorSharesBurned(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                239u8,
                246u8,
                170u8,
                178u8,
                188u8,
                63u8,
                124u8,
                100u8,
                136u8,
                150u8,
                225u8,
                82u8,
                46u8,
                174u8,
                113u8,
                214u8,
                194u8,
                46u8,
                59u8,
                14u8,
                33u8,
                130u8,
                6u8,
                179u8,
                244u8,
                10u8,
                240u8,
                228u8,
                210u8,
                4u8,
                113u8,
                107u8,
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
                    strategy: data.0,
                    shares: data.1,
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
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSharesBurned {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSharesBurned> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSharesBurned) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSharesDecreased(address,address,address,uint256)` and selector `0x6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd`.
```solidity
event OperatorSharesDecreased(address indexed operator, address staker, address strategy, uint256 shares);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSharesDecreased {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OperatorSharesDecreased {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorSharesDecreased(address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                105u8,
                9u8,
                96u8,
                0u8,
                55u8,
                183u8,
                93u8,
                123u8,
                71u8,
                51u8,
                174u8,
                221u8,
                129u8,
                84u8,
                66u8,
                181u8,
                236u8,
                1u8,
                138u8,
                130u8,
                119u8,
                81u8,
                200u8,
                50u8,
                170u8,
                255u8,
                100u8,
                235u8,
                165u8,
                214u8,
                210u8,
                221u8,
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
                    staker: data.0,
                    strategy: data.1,
                    shares: data.2,
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSharesDecreased {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSharesDecreased> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorSharesDecreased,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSharesIncreased(address,address,address,uint256)` and selector `0x1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c`.
```solidity
event OperatorSharesIncreased(address indexed operator, address staker, address strategy, uint256 shares);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSharesIncreased {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OperatorSharesIncreased {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorSharesIncreased(address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                30u8,
                192u8,
                66u8,
                201u8,
                101u8,
                226u8,
                237u8,
                215u8,
                16u8,
                123u8,
                81u8,
                24u8,
                142u8,
                224u8,
                243u8,
                131u8,
                226u8,
                46u8,
                118u8,
                23u8,
                144u8,
                65u8,
                171u8,
                58u8,
                157u8,
                24u8,
                255u8,
                21u8,
                20u8,
                5u8,
                22u8,
                108u8,
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
                    staker: data.0,
                    strategy: data.1,
                    shares: data.2,
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSharesIncreased {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSharesIncreased> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorSharesIncreased,
            ) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `SlashingWithdrawalCompleted(bytes32)` and selector `0x1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a00`.
```solidity
event SlashingWithdrawalCompleted(bytes32 withdrawalRoot);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashingWithdrawalCompleted {
        #[allow(missing_docs)]
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for SlashingWithdrawalCompleted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlashingWithdrawalCompleted(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                31u8,
                64u8,
                64u8,
                8u8,
                137u8,
                39u8,
                78u8,
                208u8,
                123u8,
                36u8,
                132u8,
                94u8,
                80u8,
                84u8,
                168u8,
                122u8,
                12u8,
                171u8,
                150u8,
                158u8,
                177u8,
                39u8,
                122u8,
                175u8,
                230u8,
                26u8,
                227u8,
                82u8,
                231u8,
                195u8,
                42u8,
                0u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { withdrawalRoot: data.0 }
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
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
        impl alloy_sol_types::private::IntoLogData for SlashingWithdrawalCompleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashingWithdrawalCompleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SlashingWithdrawalCompleted,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlashingWithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]),uint256[])` and selector `0x26b2aae26516e8719ef50ea2f6831a2efbd4e37dccdf0f6936b27bc08e793e30`.
```solidity
event SlashingWithdrawalQueued(bytes32 withdrawalRoot, IDelegationManagerTypes.Withdrawal withdrawal, uint256[] sharesToWithdraw);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashingWithdrawalQueued {
        #[allow(missing_docs)]
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sharesToWithdraw: alloy::sol_types::private::Vec<
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SlashingWithdrawalQueued {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IDelegationManagerTypes::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlashingWithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]),uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                38u8,
                178u8,
                170u8,
                226u8,
                101u8,
                22u8,
                232u8,
                113u8,
                158u8,
                245u8,
                14u8,
                162u8,
                246u8,
                131u8,
                26u8,
                46u8,
                251u8,
                212u8,
                227u8,
                125u8,
                204u8,
                223u8,
                15u8,
                105u8,
                54u8,
                178u8,
                123u8,
                192u8,
                142u8,
                121u8,
                62u8,
                48u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    withdrawalRoot: data.0,
                    withdrawal: data.1,
                    sharesToWithdraw: data.2,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
                    <IDelegationManagerTypes::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.sharesToWithdraw),
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
        impl alloy_sol_types::private::IntoLogData for SlashingWithdrawalQueued {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashingWithdrawalQueued> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SlashingWithdrawalQueued,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakerDelegated(address,address)` and selector `0xc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d8743304`.
```solidity
event StakerDelegated(address indexed staker, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakerDelegated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StakerDelegated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerDelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                195u8,
                238u8,
                159u8,
                46u8,
                95u8,
                218u8,
                152u8,
                232u8,
                6u8,
                106u8,
                31u8,
                116u8,
                91u8,
                45u8,
                249u8,
                40u8,
                95u8,
                65u8,
                111u8,
                233u8,
                140u8,
                242u8,
                85u8,
                156u8,
                210u8,
                20u8,
                132u8,
                179u8,
                216u8,
                116u8,
                51u8,
                4u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: topics.1,
                    operator: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self.staker.clone(), self.operator.clone())
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
                    &self.staker,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakerDelegated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakerDelegated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakerDelegated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakerForceUndelegated(address,address)` and selector `0xf0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a`.
```solidity
event StakerForceUndelegated(address indexed staker, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakerForceUndelegated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StakerForceUndelegated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerForceUndelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                240u8,
                237u8,
                223u8,
                7u8,
                230u8,
                234u8,
                20u8,
                243u8,
                136u8,
                180u8,
                126u8,
                30u8,
                148u8,
                160u8,
                244u8,
                100u8,
                236u8,
                189u8,
                158u8,
                237u8,
                65u8,
                113u8,
                19u8,
                14u8,
                15u8,
                192u8,
                233u8,
                159u8,
                180u8,
                3u8,
                10u8,
                138u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: topics.1,
                    operator: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self.staker.clone(), self.operator.clone())
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
                    &self.staker,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakerForceUndelegated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakerForceUndelegated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakerForceUndelegated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakerUndelegated(address,address)` and selector `0xfee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af44676`.
```solidity
event StakerUndelegated(address indexed staker, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakerUndelegated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StakerUndelegated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerUndelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                254u8,
                227u8,
                9u8,
                102u8,
                162u8,
                86u8,
                183u8,
                30u8,
                20u8,
                188u8,
                14u8,
                191u8,
                201u8,
                67u8,
                21u8,
                226u8,
                142u8,
                244u8,
                169u8,
                122u8,
                113u8,
                49u8,
                169u8,
                226u8,
                183u8,
                163u8,
                16u8,
                167u8,
                58u8,
                244u8,
                70u8,
                118u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: topics.1,
                    operator: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self.staker.clone(), self.operator.clone())
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
                    &self.staker,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakerUndelegated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakerUndelegated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakerUndelegated) -> alloy_sol_types::private::LogData {
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
constructor(address _strategyManager, address _eigenPodManager, address _allocationManager, address _pauserRegistry, address _permissionController, uint32 _MIN_WITHDRAWAL_DELAY);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _strategyManager: alloy::sol_types::private::Address,
        pub _eigenPodManager: alloy::sol_types::private::Address,
        pub _allocationManager: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
        pub _permissionController: alloy::sol_types::private::Address,
        pub _MIN_WITHDRAWAL_DELAY: u32,
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
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
                        value._strategyManager,
                        value._eigenPodManager,
                        value._allocationManager,
                        value._pauserRegistry,
                        value._permissionController,
                        value._MIN_WITHDRAWAL_DELAY,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _strategyManager: tuple.0,
                        _eigenPodManager: tuple.1,
                        _allocationManager: tuple.2,
                        _pauserRegistry: tuple.3,
                        _permissionController: tuple.4,
                        _MIN_WITHDRAWAL_DELAY: tuple.5,
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
                        &self._strategyManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._eigenPodManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionController,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._MIN_WITHDRAWAL_DELAY),
                )
            }
        }
    };
    /**Function with signature `DELEGATION_APPROVAL_TYPEHASH()` and selector `0x04a4f979`.
```solidity
function DELEGATION_APPROVAL_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DELEGATION_APPROVAL_TYPEHASHCall {}
    ///Container type for the return parameters of the [`DELEGATION_APPROVAL_TYPEHASH()`](DELEGATION_APPROVAL_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DELEGATION_APPROVAL_TYPEHASHReturn {
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
            impl ::core::convert::From<DELEGATION_APPROVAL_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DELEGATION_APPROVAL_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DELEGATION_APPROVAL_TYPEHASHCall {
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
            impl ::core::convert::From<DELEGATION_APPROVAL_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DELEGATION_APPROVAL_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DELEGATION_APPROVAL_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DELEGATION_APPROVAL_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DELEGATION_APPROVAL_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DELEGATION_APPROVAL_TYPEHASH()";
            const SELECTOR: [u8; 4] = [4u8, 164u8, 249u8, 121u8];
            #[inline]
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
    /**Function with signature `beaconChainETHStrategy()` and selector `0x9104c319`.
```solidity
function beaconChainETHStrategy() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beaconChainETHStrategyCall {}
    ///Container type for the return parameters of the [`beaconChainETHStrategy()`](beaconChainETHStrategyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beaconChainETHStrategyReturn {
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
            impl ::core::convert::From<beaconChainETHStrategyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: beaconChainETHStrategyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beaconChainETHStrategyCall {
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
            impl ::core::convert::From<beaconChainETHStrategyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: beaconChainETHStrategyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beaconChainETHStrategyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for beaconChainETHStrategyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = beaconChainETHStrategyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "beaconChainETHStrategy()";
            const SELECTOR: [u8; 4] = [145u8, 4u8, 195u8, 25u8];
            #[inline]
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
    /**Function with signature `burnOperatorShares(address,address,uint64,uint64)` and selector `0xee74937f`.
```solidity
function burnOperatorShares(address operator, address strategy, uint64 prevMaxMagnitude, uint64 newMaxMagnitude) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnOperatorSharesCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub prevMaxMagnitude: u64,
        pub newMaxMagnitude: u64,
    }
    ///Container type for the return parameters of the [`burnOperatorShares(address,address,uint64,uint64)`](burnOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnOperatorSharesReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u64,
                u64,
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
            impl ::core::convert::From<burnOperatorSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: burnOperatorSharesCall) -> Self {
                    (
                        value.operator,
                        value.strategy,
                        value.prevMaxMagnitude,
                        value.newMaxMagnitude,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for burnOperatorSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                        prevMaxMagnitude: tuple.2,
                        newMaxMagnitude: tuple.3,
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
            impl ::core::convert::From<burnOperatorSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: burnOperatorSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for burnOperatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for burnOperatorSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnOperatorSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "burnOperatorShares(address,address,uint64,uint64)";
            const SELECTOR: [u8; 4] = [238u8, 116u8, 147u8, 127u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevMaxMagnitude),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMaxMagnitude),
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
    /**Function with signature `calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)` and selector `0x0b9f487a`.
```solidity
function calculateDelegationApprovalDigestHash(address staker, address operator, address approver, bytes32 approverSalt, uint256 expiry) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateDelegationApprovalDigestHashCall {
        pub staker: alloy::sol_types::private::Address,
        pub operator: alloy::sol_types::private::Address,
        pub approver: alloy::sol_types::private::Address,
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)`](calculateDelegationApprovalDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateDelegationApprovalDigestHashReturn {
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<calculateDelegationApprovalDigestHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateDelegationApprovalDigestHashCall) -> Self {
                    (
                        value.staker,
                        value.operator,
                        value.approver,
                        value.approverSalt,
                        value.expiry,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateDelegationApprovalDigestHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        operator: tuple.1,
                        approver: tuple.2,
                        approverSalt: tuple.3,
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
            impl ::core::convert::From<calculateDelegationApprovalDigestHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateDelegationApprovalDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateDelegationApprovalDigestHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateDelegationApprovalDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateDelegationApprovalDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [11u8, 159u8, 72u8, 122u8];
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.approver,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.approverSalt),
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
    /**Function with signature `calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x597b36da`.
```solidity
function calculateWithdrawalRoot(IDelegationManagerTypes.Withdrawal memory withdrawal) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateWithdrawalRootCall {
        pub withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))`](calculateWithdrawalRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateWithdrawalRootReturn {
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
            type UnderlyingSolTuple<'a> = (IDelegationManagerTypes::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<calculateWithdrawalRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateWithdrawalRootCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateWithdrawalRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawal: tuple.0 }
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
            impl ::core::convert::From<calculateWithdrawalRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateWithdrawalRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateWithdrawalRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateWithdrawalRootCall {
            type Parameters<'a> = (IDelegationManagerTypes::Withdrawal,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateWithdrawalRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))";
            const SELECTOR: [u8; 4] = [89u8, 123u8, 54u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManagerTypes::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
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
    /**Function with signature `completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],bool)` and selector `0xe4cc3f90`.
```solidity
function completeQueuedWithdrawal(IDelegationManagerTypes.Withdrawal memory withdrawal, address[] memory tokens, bool receiveAsTokens) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalCall {
        pub withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        pub tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub receiveAsTokens: bool,
    }
    ///Container type for the return parameters of the [`completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],bool)`](completeQueuedWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalReturn {}
    #[allow(
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
                IDelegationManagerTypes::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                bool,
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
            impl ::core::convert::From<completeQueuedWithdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalCall) -> Self {
                    (value.withdrawal, value.tokens, value.receiveAsTokens)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                        tokens: tuple.1,
                        receiveAsTokens: tuple.2,
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
            impl ::core::convert::From<completeQueuedWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeQueuedWithdrawalCall {
            type Parameters<'a> = (
                IDelegationManagerTypes::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],bool)";
            const SELECTOR: [u8; 4] = [228u8, 204u8, 63u8, 144u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManagerTypes::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokens),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.receiveAsTokens,
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
    /**Function with signature `completeQueuedWithdrawals(address[][],bool[],uint256)` and selector `0x5f48e667`.
```solidity
function completeQueuedWithdrawals(address[][] memory tokens, bool[] memory receiveAsTokens, uint256 numToComplete) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawals_0Call {
        pub tokens: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        pub receiveAsTokens: alloy::sol_types::private::Vec<bool>,
        pub numToComplete: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`completeQueuedWithdrawals(address[][],bool[],uint256)`](completeQueuedWithdrawals_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawals_0Return {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
                alloy::sol_types::private::Vec<bool>,
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
            impl ::core::convert::From<completeQueuedWithdrawals_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawals_0Call) -> Self {
                    (value.tokens, value.receiveAsTokens, value.numToComplete)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawals_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokens: tuple.0,
                        receiveAsTokens: tuple.1,
                        numToComplete: tuple.2,
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
            impl ::core::convert::From<completeQueuedWithdrawals_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawals_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawals_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeQueuedWithdrawals_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawals_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeQueuedWithdrawals(address[][],bool[],uint256)";
            const SELECTOR: [u8; 4] = [95u8, 72u8, 230u8, 103u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.tokens),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bool,
                    > as alloy_sol_types::SolType>::tokenize(&self.receiveAsTokens),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.numToComplete),
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
    /**Function with signature `completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],bool[])` and selector `0x9435bb43`.
```solidity
function completeQueuedWithdrawals(IDelegationManagerTypes.Withdrawal[] memory withdrawals, address[][] memory tokens, bool[] memory receiveAsTokens) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawals_1Call {
        pub withdrawals: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        >,
        pub tokens: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        pub receiveAsTokens: alloy::sol_types::private::Vec<bool>,
    }
    ///Container type for the return parameters of the [`completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],bool[])`](completeQueuedWithdrawals_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawals_1Return {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
                alloy::sol_types::private::Vec<bool>,
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
            impl ::core::convert::From<completeQueuedWithdrawals_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawals_1Call) -> Self {
                    (value.withdrawals, value.tokens, value.receiveAsTokens)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawals_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawals: tuple.0,
                        tokens: tuple.1,
                        receiveAsTokens: tuple.2,
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
            impl ::core::convert::From<completeQueuedWithdrawals_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawals_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawals_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeQueuedWithdrawals_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawals_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],bool[])";
            const SELECTOR: [u8; 4] = [148u8, 53u8, 187u8, 67u8];
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
                        IDelegationManagerTypes::Withdrawal,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawals),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Address,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokens),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bool,
                    > as alloy_sol_types::SolType>::tokenize(&self.receiveAsTokens),
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
    /**Function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`.
```solidity
function cumulativeWithdrawalsQueued(address staker) external view returns (uint256 totalQueued);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeWithdrawalsQueuedCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`cumulativeWithdrawalsQueued(address)`](cumulativeWithdrawalsQueuedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeWithdrawalsQueuedReturn {
        pub totalQueued: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<cumulativeWithdrawalsQueuedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeWithdrawalsQueuedCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeWithdrawalsQueuedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<cumulativeWithdrawalsQueuedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeWithdrawalsQueuedReturn) -> Self {
                    (value.totalQueued,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeWithdrawalsQueuedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { totalQueued: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cumulativeWithdrawalsQueuedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cumulativeWithdrawalsQueuedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cumulativeWithdrawalsQueued(address)";
            const SELECTOR: [u8; 4] = [161u8, 120u8, 132u8, 132u8];
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
                        &self.staker,
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
    /**Function with signature `decreaseDelegatedShares(address,uint256,uint64)` and selector `0x60a0d1ce`.
```solidity
function decreaseDelegatedShares(address staker, uint256 curDepositShares, uint64 beaconChainSlashingFactorDecrease) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseDelegatedSharesCall {
        pub staker: alloy::sol_types::private::Address,
        pub curDepositShares: alloy::sol_types::private::primitives::aliases::U256,
        pub beaconChainSlashingFactorDecrease: u64,
    }
    ///Container type for the return parameters of the [`decreaseDelegatedShares(address,uint256,uint64)`](decreaseDelegatedSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseDelegatedSharesReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                u64,
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
            impl ::core::convert::From<decreaseDelegatedSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: decreaseDelegatedSharesCall) -> Self {
                    (
                        value.staker,
                        value.curDepositShares,
                        value.beaconChainSlashingFactorDecrease,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decreaseDelegatedSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        curDepositShares: tuple.1,
                        beaconChainSlashingFactorDecrease: tuple.2,
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
            impl ::core::convert::From<decreaseDelegatedSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: decreaseDelegatedSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decreaseDelegatedSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decreaseDelegatedSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = decreaseDelegatedSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decreaseDelegatedShares(address,uint256,uint64)";
            const SELECTOR: [u8; 4] = [96u8, 160u8, 209u8, 206u8];
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.curDepositShares),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.beaconChainSlashingFactorDecrease,
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
    /**Function with signature `delegateTo(address,(bytes,uint256),bytes32)` and selector `0xeea9064b`.
```solidity
function delegateTo(address operator, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToCall {
        pub operator: alloy::sol_types::private::Address,
        pub approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateTo(address,(bytes,uint256),bytes32)`](delegateToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToReturn {}
    #[allow(
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
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<delegateToCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToCall) -> Self {
                    (
                        value.operator,
                        value.approverSignatureAndExpiry,
                        value.approverSalt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        approverSignatureAndExpiry: tuple.1,
                        approverSalt: tuple.2,
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
            impl ::core::convert::From<delegateToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateToCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegateTo(address,(bytes,uint256),bytes32)";
            const SELECTOR: [u8; 4] = [238u8, 169u8, 6u8, 75u8];
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
                    <ISignatureUtils::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.approverSignatureAndExpiry,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.approverSalt),
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
    /**Function with signature `delegatedTo(address)` and selector `0x65da1264`.
```solidity
function delegatedTo(address staker) external view returns (address operator);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedToCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegatedTo(address)`](delegatedToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedToReturn {
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
            impl ::core::convert::From<delegatedToCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegatedToCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatedToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<delegatedToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegatedToReturn) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatedToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatedToCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegatedToReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegatedTo(address)";
            const SELECTOR: [u8; 4] = [101u8, 218u8, 18u8, 100u8];
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
                        &self.staker,
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
    /**Function with signature `delegationApprover(address)` and selector `0x3cdeb5e0`.
```solidity
function delegationApprover(address operator) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegationApprover(address)`](delegationApproverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverReturn {
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
            impl ::core::convert::From<delegationApproverCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationApproverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<delegationApproverReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationApproverCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationApproverReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegationApprover(address)";
            const SELECTOR: [u8; 4] = [60u8, 222u8, 181u8, 224u8];
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
    /**Function with signature `delegationApproverSaltIsSpent(address,bytes32)` and selector `0xbb45fef2`.
```solidity
function delegationApproverSaltIsSpent(address delegationApprover, bytes32 salt) external view returns (bool spent);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverSaltIsSpentCall {
        pub delegationApprover: alloy::sol_types::private::Address,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegationApproverSaltIsSpent(address,bytes32)`](delegationApproverSaltIsSpentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverSaltIsSpentReturn {
        pub spent: bool,
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<delegationApproverSaltIsSpentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverSaltIsSpentCall) -> Self {
                    (value.delegationApprover, value.salt)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationApproverSaltIsSpentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        delegationApprover: tuple.0,
                        salt: tuple.1,
                    }
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
            impl ::core::convert::From<delegationApproverSaltIsSpentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverSaltIsSpentReturn) -> Self {
                    (value.spent,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationApproverSaltIsSpentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { spent: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationApproverSaltIsSpentCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationApproverSaltIsSpentReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegationApproverSaltIsSpent(address,bytes32)";
            const SELECTOR: [u8; 4] = [187u8, 69u8, 254u8, 242u8];
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
                        &self.delegationApprover,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
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
    /**Function with signature `depositScalingFactor(address,address)` and selector `0xbfae3fd2`.
```solidity
function depositScalingFactor(address staker, address strategy) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositScalingFactorCall {
        pub staker: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`depositScalingFactor(address,address)`](depositScalingFactorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositScalingFactorReturn {
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
            impl ::core::convert::From<depositScalingFactorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositScalingFactorCall) -> Self {
                    (value.staker, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositScalingFactorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        strategy: tuple.1,
                    }
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
            impl ::core::convert::From<depositScalingFactorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositScalingFactorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositScalingFactorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositScalingFactorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositScalingFactorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositScalingFactor(address,address)";
            const SELECTOR: [u8; 4] = [191u8, 174u8, 63u8, 210u8];
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
                        &self.staker,
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
    /**Function with signature `domainSeparator()` and selector `0xf698da25`.
```solidity
function domainSeparator() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorCall {}
    ///Container type for the return parameters of the [`domainSeparator()`](domainSeparatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorReturn {
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
            impl ::core::convert::From<domainSeparatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorCall {
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
            impl ::core::convert::From<domainSeparatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for domainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for domainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = domainSeparatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "domainSeparator()";
            const SELECTOR: [u8; 4] = [246u8, 152u8, 218u8, 37u8];
            #[inline]
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
    /**Function with signature `eigenPodManager()` and selector `0x4665bcda`.
```solidity
function eigenPodManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerCall {}
    ///Container type for the return parameters of the [`eigenPodManager()`](eigenPodManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerReturn {
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
            impl ::core::convert::From<eigenPodManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodManagerCall {
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
            impl ::core::convert::From<eigenPodManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenPodManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenPodManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenPodManager()";
            const SELECTOR: [u8; 4] = [70u8, 101u8, 188u8, 218u8];
            #[inline]
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
    /**Function with signature `getDepositedShares(address)` and selector `0x66d5ba93`.
```solidity
function getDepositedShares(address staker) external view returns (address[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDepositedSharesCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getDepositedShares(address)`](getDepositedSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDepositedSharesReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub _1: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getDepositedSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDepositedSharesCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDepositedSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getDepositedSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDepositedSharesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDepositedSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDepositedSharesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDepositedSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDepositedShares(address)";
            const SELECTOR: [u8; 4] = [102u8, 213u8, 186u8, 147u8];
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
                        &self.staker,
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
    /**Function with signature `getOperatorShares(address,address[])` and selector `0x90041347`.
```solidity
function getOperatorShares(address operator, address[] memory strategies) external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSharesCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getOperatorShares(address,address[])`](getOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSharesReturn {
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
            impl ::core::convert::From<getOperatorSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSharesCall) -> Self {
                    (value.operator, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategies: tuple.1,
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
            impl ::core::convert::From<getOperatorSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorShares(address,address[])";
            const SELECTOR: [u8; 4] = [144u8, 4u8, 19u8, 71u8];
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
    /**Function with signature `getOperatorsShares(address[],address[])` and selector `0xf0e0e676`.
```solidity
function getOperatorsShares(address[] memory operators, address[] memory strategies) external view returns (uint256[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorsSharesCall {
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getOperatorsShares(address[],address[])`](getOperatorsSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorsSharesReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getOperatorsSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorsSharesCall) -> Self {
                    (value.operators, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorsSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operators: tuple.0,
                        strategies: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getOperatorsSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorsSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorsSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorsSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorsSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorsShares(address[],address[])";
            const SELECTOR: [u8; 4] = [240u8, 224u8, 230u8, 118u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
    /**Function with signature `getQueuedWithdrawals(address)` and selector `0x5dd68579`.
```solidity
function getQueuedWithdrawals(address staker) external view returns (IDelegationManagerTypes.Withdrawal[] memory withdrawals, uint256[][] memory shares);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalsCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getQueuedWithdrawals(address)`](getQueuedWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalsReturn {
        pub withdrawals: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        >,
        pub shares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getQueuedWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalsCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getQueuedWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalsReturn) -> Self {
                    (value.withdrawals, value.shares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawals: tuple.0,
                        shares: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQueuedWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQueuedWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQueuedWithdrawals(address)";
            const SELECTOR: [u8; 4] = [93u8, 214u8, 133u8, 121u8];
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
                        &self.staker,
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
    /**Function with signature `getSlashableSharesInQueue(address,address)` and selector `0x6e174448`.
```solidity
function getSlashableSharesInQueue(address operator, address strategy) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlashableSharesInQueueCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getSlashableSharesInQueue(address,address)`](getSlashableSharesInQueueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlashableSharesInQueueReturn {
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
            impl ::core::convert::From<getSlashableSharesInQueueCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSlashableSharesInQueueCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSlashableSharesInQueueCall {
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
            impl ::core::convert::From<getSlashableSharesInQueueReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSlashableSharesInQueueReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSlashableSharesInQueueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlashableSharesInQueueCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlashableSharesInQueueReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlashableSharesInQueue(address,address)";
            const SELECTOR: [u8; 4] = [110u8, 23u8, 68u8, 72u8];
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
    /**Function with signature `getWithdrawableShares(address,address[])` and selector `0xc978f7ac`.
```solidity
function getWithdrawableShares(address staker, address[] memory strategies) external view returns (uint256[] memory withdrawableShares, uint256[] memory depositShares);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawableSharesCall {
        pub staker: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getWithdrawableShares(address,address[])`](getWithdrawableSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawableSharesReturn {
        pub withdrawableShares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub depositShares: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getWithdrawableSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawableSharesCall) -> Self {
                    (value.staker, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawableSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        strategies: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<getWithdrawableSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawableSharesReturn) -> Self {
                    (value.withdrawableShares, value.depositShares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawableSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawableShares: tuple.0,
                        depositShares: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawableSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawableSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawableShares(address,address[])";
            const SELECTOR: [u8; 4] = [201u8, 120u8, 247u8, 172u8];
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
    /**Function with signature `increaseDelegatedShares(address,address,uint256,uint256)` and selector `0x3c651cf2`.
```solidity
function increaseDelegatedShares(address staker, address strategy, uint256 prevDepositShares, uint256 addedShares) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseDelegatedSharesCall {
        pub staker: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub prevDepositShares: alloy::sol_types::private::primitives::aliases::U256,
        pub addedShares: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`increaseDelegatedShares(address,address,uint256,uint256)`](increaseDelegatedSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseDelegatedSharesReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<increaseDelegatedSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: increaseDelegatedSharesCall) -> Self {
                    (
                        value.staker,
                        value.strategy,
                        value.prevDepositShares,
                        value.addedShares,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for increaseDelegatedSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        strategy: tuple.1,
                        prevDepositShares: tuple.2,
                        addedShares: tuple.3,
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
            impl ::core::convert::From<increaseDelegatedSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: increaseDelegatedSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for increaseDelegatedSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for increaseDelegatedSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = increaseDelegatedSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "increaseDelegatedShares(address,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [60u8, 101u8, 28u8, 242u8];
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevDepositShares),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.addedShares),
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
    /**Function with signature `initialize(address,uint256)` and selector `0xcd6dc687`.
```solidity
function initialize(address initialOwner, uint256 initialPausedStatus) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub initialOwner: alloy::sol_types::private::Address,
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`initialize(address,uint256)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.initialOwner, value.initialPausedStatus)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        initialPausedStatus: tuple.1,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint256)";
            const SELECTOR: [u8; 4] = [205u8, 109u8, 198u8, 135u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialPausedStatus),
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
    /**Function with signature `isDelegated(address)` and selector `0x3e28391d`.
```solidity
function isDelegated(address staker) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDelegatedCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isDelegated(address)`](isDelegatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDelegatedReturn {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isDelegatedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isDelegatedCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isDelegatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<isDelegatedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isDelegatedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isDelegatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isDelegatedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isDelegatedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isDelegated(address)";
            const SELECTOR: [u8; 4] = [62u8, 40u8, 57u8, 29u8];
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
                        &self.staker,
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
    /**Function with signature `isOperator(address)` and selector `0x6d70f7ae`.
```solidity
function isOperator(address operator) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isOperator(address)`](isOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorReturn {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<isOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperator(address)";
            const SELECTOR: [u8; 4] = [109u8, 112u8, 247u8, 174u8];
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
    /**Function with signature `minWithdrawalDelayBlocks()` and selector `0xc448feb8`.
```solidity
function minWithdrawalDelayBlocks() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minWithdrawalDelayBlocksCall {}
    ///Container type for the return parameters of the [`minWithdrawalDelayBlocks()`](minWithdrawalDelayBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minWithdrawalDelayBlocksReturn {
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
            impl ::core::convert::From<minWithdrawalDelayBlocksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: minWithdrawalDelayBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minWithdrawalDelayBlocksCall {
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
            impl ::core::convert::From<minWithdrawalDelayBlocksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: minWithdrawalDelayBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minWithdrawalDelayBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minWithdrawalDelayBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = minWithdrawalDelayBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minWithdrawalDelayBlocks()";
            const SELECTOR: [u8; 4] = [196u8, 72u8, 254u8, 184u8];
            #[inline]
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
    /**Function with signature `modifyOperatorDetails(address,address)` and selector `0x54b7c96c`.
```solidity
function modifyOperatorDetails(address operator, address newDelegationApprover) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyOperatorDetailsCall {
        pub operator: alloy::sol_types::private::Address,
        pub newDelegationApprover: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`modifyOperatorDetails(address,address)`](modifyOperatorDetailsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyOperatorDetailsReturn {}
    #[allow(
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
            impl ::core::convert::From<modifyOperatorDetailsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyOperatorDetailsCall) -> Self {
                    (value.operator, value.newDelegationApprover)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyOperatorDetailsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        newDelegationApprover: tuple.1,
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
            impl ::core::convert::From<modifyOperatorDetailsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyOperatorDetailsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyOperatorDetailsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyOperatorDetailsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyOperatorDetailsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyOperatorDetails(address,address)";
            const SELECTOR: [u8; 4] = [84u8, 183u8, 201u8, 108u8];
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
                        &self.newDelegationApprover,
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
    /**Function with signature `operatorShares(address,address)` and selector `0x778e55f3`.
```solidity
function operatorShares(address operator, address strategy) external view returns (uint256 shares);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSharesCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorShares(address,address)`](operatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSharesReturn {
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<operatorSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorSharesCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorSharesCall {
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
            impl ::core::convert::From<operatorSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSharesReturn) -> Self {
                    (value.shares,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { shares: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSharesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorShares(address,address)";
            const SELECTOR: [u8; 4] = [119u8, 142u8, 85u8, 243u8];
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
    /**Function with signature `pendingWithdrawals(bytes32)` and selector `0xb7f06ebe`.
```solidity
function pendingWithdrawals(bytes32 withdrawalRoot) external view returns (bool pending);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingWithdrawalsCall {
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`pendingWithdrawals(bytes32)`](pendingWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingWithdrawalsReturn {
        pub pending: bool,
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
            impl ::core::convert::From<pendingWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingWithdrawalsCall) -> Self {
                    (value.withdrawalRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalRoot: tuple.0 }
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
            impl ::core::convert::From<pendingWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingWithdrawalsReturn) -> Self {
                    (value.pending,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pending: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pendingWithdrawalsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pendingWithdrawals(bytes32)";
            const SELECTOR: [u8; 4] = [183u8, 240u8, 110u8, 190u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
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
    /**Function with signature `permissionController()` and selector `0x4657e26a`.
```solidity
function permissionController() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionControllerCall {}
    ///Container type for the return parameters of the [`permissionController()`](permissionControllerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionControllerReturn {
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
            impl ::core::convert::From<permissionControllerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: permissionControllerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for permissionControllerCall {
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
            impl ::core::convert::From<permissionControllerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: permissionControllerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for permissionControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permissionControllerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = permissionControllerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permissionController()";
            const SELECTOR: [u8; 4] = [70u8, 87u8, 226u8, 106u8];
            #[inline]
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
    /**Function with signature `queueWithdrawals((address[],uint256[],address)[])` and selector `0x0dd8dd02`.
```solidity
function queueWithdrawals(IDelegationManagerTypes.QueuedWithdrawalParams[] memory params) external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalsCall {
        pub params: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`queueWithdrawals((address[],uint256[],address)[])`](queueWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalsReturn {
        pub _0: alloy::sol_types::private::Vec<
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
                alloy::sol_types::sol_data::Array<
                    IDelegationManagerTypes::QueuedWithdrawalParams,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<queueWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queueWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<queueWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queueWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queueWithdrawalsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IDelegationManagerTypes::QueuedWithdrawalParams,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queueWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queueWithdrawals((address[],uint256[],address)[])";
            const SELECTOR: [u8; 4] = [13u8, 216u8, 221u8, 2u8];
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
                        IDelegationManagerTypes::QueuedWithdrawalParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.params),
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
    /**Function with signature `queuedWithdrawals(bytes32)` and selector `0x99f5371b`.
```solidity
function queuedWithdrawals(bytes32 withdrawalRoot) external view returns (address staker, address delegatedTo, address withdrawer, uint256 nonce, uint32 startBlock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queuedWithdrawalsCall {
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`queuedWithdrawals(bytes32)`](queuedWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queuedWithdrawalsReturn {
        pub staker: alloy::sol_types::private::Address,
        pub delegatedTo: alloy::sol_types::private::Address,
        pub withdrawer: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub startBlock: u32,
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
            impl ::core::convert::From<queuedWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: queuedWithdrawalsCall) -> Self {
                    (value.withdrawalRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queuedWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalRoot: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<queuedWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: queuedWithdrawalsReturn) -> Self {
                    (
                        value.staker,
                        value.delegatedTo,
                        value.withdrawer,
                        value.nonce,
                        value.startBlock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queuedWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        delegatedTo: tuple.1,
                        withdrawer: tuple.2,
                        nonce: tuple.3,
                        startBlock: tuple.4,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queuedWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queuedWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queuedWithdrawals(bytes32)";
            const SELECTOR: [u8; 4] = [153u8, 245u8, 55u8, 27u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
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
    /**Function with signature `redelegate(address,(bytes,uint256),bytes32)` and selector `0xa33a3433`.
```solidity
function redelegate(address newOperator, ISignatureUtils.SignatureWithExpiry memory newOperatorApproverSig, bytes32 approverSalt) external returns (bytes32[] memory withdrawalRoots);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redelegateCall {
        pub newOperator: alloy::sol_types::private::Address,
        pub newOperatorApproverSig: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`redelegate(address,(bytes,uint256),bytes32)`](redelegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redelegateReturn {
        pub withdrawalRoots: alloy::sol_types::private::Vec<
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
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<redelegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: redelegateCall) -> Self {
                    (value.newOperator, value.newOperatorApproverSig, value.approverSalt)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for redelegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newOperator: tuple.0,
                        newOperatorApproverSig: tuple.1,
                        approverSalt: tuple.2,
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
            impl ::core::convert::From<redelegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: redelegateReturn) -> Self {
                    (value.withdrawalRoots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for redelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalRoots: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for redelegateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = redelegateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "redelegate(address,(bytes,uint256),bytes32)";
            const SELECTOR: [u8; 4] = [163u8, 58u8, 52u8, 51u8];
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
                        &self.newOperator,
                    ),
                    <ISignatureUtils::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.newOperatorApproverSig,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.approverSalt),
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
    /**Function with signature `registerAsOperator(address,uint32,string)` and selector `0x2aa6d888`.
```solidity
function registerAsOperator(address initDelegationApprover, uint32 allocationDelay, string memory metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAsOperatorCall {
        pub initDelegationApprover: alloy::sol_types::private::Address,
        pub allocationDelay: u32,
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`registerAsOperator(address,uint32,string)`](registerAsOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAsOperatorReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<registerAsOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorCall) -> Self {
                    (
                        value.initDelegationApprover,
                        value.allocationDelay,
                        value.metadataURI,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerAsOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initDelegationApprover: tuple.0,
                        allocationDelay: tuple.1,
                        metadataURI: tuple.2,
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
            impl ::core::convert::From<registerAsOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerAsOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerAsOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerAsOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerAsOperator(address,uint32,string)";
            const SELECTOR: [u8; 4] = [42u8, 166u8, 216u8, 136u8];
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
                        &self.initDelegationApprover,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.allocationDelay),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metadataURI,
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
    /**Function with signature `strategyManager()` and selector `0x39b70e38`.
```solidity
function strategyManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyManagerCall {}
    ///Container type for the return parameters of the [`strategyManager()`](strategyManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyManagerReturn {
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
            impl ::core::convert::From<strategyManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyManagerCall {
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
            impl ::core::convert::From<strategyManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyManager()";
            const SELECTOR: [u8; 4] = [57u8, 183u8, 14u8, 56u8];
            #[inline]
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
    /**Function with signature `undelegate(address)` and selector `0xda8be864`.
```solidity
function undelegate(address staker) external returns (bytes32[] memory withdrawalRoots);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct undelegateCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`undelegate(address)`](undelegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct undelegateReturn {
        pub withdrawalRoots: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<undelegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: undelegateCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for undelegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<undelegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: undelegateReturn) -> Self {
                    (value.withdrawalRoots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for undelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalRoots: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for undelegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = undelegateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "undelegate(address)";
            const SELECTOR: [u8; 4] = [218u8, 139u8, 232u8, 100u8];
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
                        &self.staker,
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
    /**Function with signature `updateOperatorMetadataURI(address,string)` and selector `0x78296ec5`.
```solidity
function updateOperatorMetadataURI(address operator, string memory metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorMetadataURICall {
        pub operator: alloy::sol_types::private::Address,
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateOperatorMetadataURI(address,string)`](updateOperatorMetadataURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorMetadataURIReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<updateOperatorMetadataURICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorMetadataURICall) -> Self {
                    (value.operator, value.metadataURI)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        metadataURI: tuple.1,
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
            impl ::core::convert::From<updateOperatorMetadataURIReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorMetadataURICall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorMetadataURI(address,string)";
            const SELECTOR: [u8; 4] = [120u8, 41u8, 110u8, 197u8];
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metadataURI,
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
    ///Container for all the [`DelegationManager`](self) function calls.
    pub enum DelegationManagerCalls {
        DELEGATION_APPROVAL_TYPEHASH(DELEGATION_APPROVAL_TYPEHASHCall),
        allocationManager(allocationManagerCall),
        beaconChainETHStrategy(beaconChainETHStrategyCall),
        burnOperatorShares(burnOperatorSharesCall),
        calculateDelegationApprovalDigestHash(calculateDelegationApprovalDigestHashCall),
        calculateWithdrawalRoot(calculateWithdrawalRootCall),
        completeQueuedWithdrawal(completeQueuedWithdrawalCall),
        completeQueuedWithdrawals_0(completeQueuedWithdrawals_0Call),
        completeQueuedWithdrawals_1(completeQueuedWithdrawals_1Call),
        cumulativeWithdrawalsQueued(cumulativeWithdrawalsQueuedCall),
        decreaseDelegatedShares(decreaseDelegatedSharesCall),
        delegateTo(delegateToCall),
        delegatedTo(delegatedToCall),
        delegationApprover(delegationApproverCall),
        delegationApproverSaltIsSpent(delegationApproverSaltIsSpentCall),
        depositScalingFactor(depositScalingFactorCall),
        domainSeparator(domainSeparatorCall),
        eigenPodManager(eigenPodManagerCall),
        getDepositedShares(getDepositedSharesCall),
        getOperatorShares(getOperatorSharesCall),
        getOperatorsShares(getOperatorsSharesCall),
        getQueuedWithdrawals(getQueuedWithdrawalsCall),
        getSlashableSharesInQueue(getSlashableSharesInQueueCall),
        getWithdrawableShares(getWithdrawableSharesCall),
        increaseDelegatedShares(increaseDelegatedSharesCall),
        initialize(initializeCall),
        isDelegated(isDelegatedCall),
        isOperator(isOperatorCall),
        minWithdrawalDelayBlocks(minWithdrawalDelayBlocksCall),
        modifyOperatorDetails(modifyOperatorDetailsCall),
        operatorShares(operatorSharesCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        pendingWithdrawals(pendingWithdrawalsCall),
        permissionController(permissionControllerCall),
        queueWithdrawals(queueWithdrawalsCall),
        queuedWithdrawals(queuedWithdrawalsCall),
        redelegate(redelegateCall),
        registerAsOperator(registerAsOperatorCall),
        renounceOwnership(renounceOwnershipCall),
        strategyManager(strategyManagerCall),
        transferOwnership(transferOwnershipCall),
        undelegate(undelegateCall),
        unpause(unpauseCall),
        updateOperatorMetadataURI(updateOperatorMetadataURICall),
    }
    #[automatically_derived]
    impl DelegationManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 164u8, 249u8, 121u8],
            [11u8, 159u8, 72u8, 122u8],
            [13u8, 216u8, 221u8, 2u8],
            [19u8, 100u8, 57u8, 221u8],
            [42u8, 166u8, 216u8, 136u8],
            [57u8, 183u8, 14u8, 56u8],
            [60u8, 101u8, 28u8, 242u8],
            [60u8, 222u8, 181u8, 224u8],
            [62u8, 40u8, 57u8, 29u8],
            [70u8, 87u8, 226u8, 106u8],
            [70u8, 101u8, 188u8, 218u8],
            [84u8, 183u8, 201u8, 108u8],
            [89u8, 92u8, 106u8, 103u8],
            [89u8, 123u8, 54u8, 218u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 214u8, 133u8, 121u8],
            [95u8, 72u8, 230u8, 103u8],
            [96u8, 160u8, 209u8, 206u8],
            [101u8, 218u8, 18u8, 100u8],
            [102u8, 213u8, 186u8, 147u8],
            [109u8, 112u8, 247u8, 174u8],
            [110u8, 23u8, 68u8, 72u8],
            [113u8, 80u8, 24u8, 166u8],
            [119u8, 142u8, 85u8, 243u8],
            [120u8, 41u8, 110u8, 197u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [144u8, 4u8, 19u8, 71u8],
            [145u8, 4u8, 195u8, 25u8],
            [148u8, 53u8, 187u8, 67u8],
            [153u8, 245u8, 55u8, 27u8],
            [161u8, 120u8, 132u8, 132u8],
            [163u8, 58u8, 52u8, 51u8],
            [183u8, 240u8, 110u8, 190u8],
            [187u8, 69u8, 254u8, 242u8],
            [191u8, 174u8, 63u8, 210u8],
            [196u8, 72u8, 254u8, 184u8],
            [201u8, 120u8, 247u8, 172u8],
            [202u8, 138u8, 167u8, 199u8],
            [205u8, 109u8, 198u8, 135u8],
            [218u8, 139u8, 232u8, 100u8],
            [228u8, 204u8, 63u8, 144u8],
            [238u8, 116u8, 147u8, 127u8],
            [238u8, 169u8, 6u8, 75u8],
            [240u8, 224u8, 230u8, 118u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 152u8, 218u8, 37u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelegationManagerCalls {
        const NAME: &'static str = "DelegationManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 49usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DELEGATION_APPROVAL_TYPEHASH(_) => {
                    <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beaconChainETHStrategy(_) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::burnOperatorShares(_) => {
                    <burnOperatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateDelegationApprovalDigestHash(_) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateWithdrawalRoot(_) => {
                    <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeQueuedWithdrawal(_) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeQueuedWithdrawals_0(_) => {
                    <completeQueuedWithdrawals_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeQueuedWithdrawals_1(_) => {
                    <completeQueuedWithdrawals_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cumulativeWithdrawalsQueued(_) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::decreaseDelegatedShares(_) => {
                    <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegateTo(_) => {
                    <delegateToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegatedTo(_) => {
                    <delegatedToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegationApprover(_) => {
                    <delegationApproverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegationApproverSaltIsSpent(_) => {
                    <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositScalingFactor(_) => {
                    <depositScalingFactorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodManager(_) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDepositedShares(_) => {
                    <getDepositedSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorShares(_) => {
                    <getOperatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorsShares(_) => {
                    <getOperatorsSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQueuedWithdrawals(_) => {
                    <getQueuedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSlashableSharesInQueue(_) => {
                    <getSlashableSharesInQueueCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawableShares(_) => {
                    <getWithdrawableSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::increaseDelegatedShares(_) => {
                    <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isDelegated(_) => {
                    <isDelegatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperator(_) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minWithdrawalDelayBlocks(_) => {
                    <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::modifyOperatorDetails(_) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorShares(_) => {
                    <operatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pendingWithdrawals(_) => {
                    <pendingWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::permissionController(_) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queueWithdrawals(_) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queuedWithdrawals(_) => {
                    <queuedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::redelegate(_) => {
                    <redelegateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerAsOperator(_) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyManager(_) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::undelegate(_) => {
                    <undelegateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateOperatorMetadataURI(_) => {
                    <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<DelegationManagerCalls>] = &[
                {
                    fn DELEGATION_APPROVAL_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::DELEGATION_APPROVAL_TYPEHASH)
                    }
                    DELEGATION_APPROVAL_TYPEHASH
                },
                {
                    fn calculateDelegationApprovalDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::calculateDelegationApprovalDigestHash,
                            )
                    }
                    calculateDelegationApprovalDigestHash
                },
                {
                    fn queueWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::queueWithdrawals)
                    }
                    queueWithdrawals
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn registerAsOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::registerAsOperator)
                    }
                    registerAsOperator
                },
                {
                    fn strategyManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <strategyManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::strategyManager)
                    }
                    strategyManager
                },
                {
                    fn increaseDelegatedShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::increaseDelegatedShares)
                    }
                    increaseDelegatedShares
                },
                {
                    fn delegationApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegationApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::delegationApprover)
                    }
                    delegationApprover
                },
                {
                    fn isDelegated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <isDelegatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::isDelegated)
                    }
                    isDelegated
                },
                {
                    fn permissionController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <permissionControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::permissionController)
                    }
                    permissionController
                },
                {
                    fn eigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::eigenPodManager)
                    }
                    eigenPodManager
                },
                {
                    fn modifyOperatorDetails(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::modifyOperatorDetails)
                    }
                    modifyOperatorDetails
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn calculateWithdrawalRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::calculateWithdrawalRoot)
                    }
                    calculateWithdrawalRoot
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn getQueuedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getQueuedWithdrawals)
                    }
                    getQueuedWithdrawals
                },
                {
                    fn completeQueuedWithdrawals_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <completeQueuedWithdrawals_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::completeQueuedWithdrawals_0)
                    }
                    completeQueuedWithdrawals_0
                },
                {
                    fn decreaseDelegatedShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::decreaseDelegatedShares)
                    }
                    decreaseDelegatedShares
                },
                {
                    fn delegatedTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegatedToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::delegatedTo)
                    }
                    delegatedTo
                },
                {
                    fn getDepositedShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getDepositedSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getDepositedShares)
                    }
                    getDepositedShares
                },
                {
                    fn isOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <isOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::isOperator)
                    }
                    isOperator
                },
                {
                    fn getSlashableSharesInQueue(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getSlashableSharesInQueueCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getSlashableSharesInQueue)
                    }
                    getSlashableSharesInQueue
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn operatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <operatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::operatorShares)
                    }
                    operatorShares
                },
                {
                    fn updateOperatorMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::updateOperatorMetadataURI)
                    }
                    updateOperatorMetadataURI
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn getOperatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getOperatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getOperatorShares)
                    }
                    getOperatorShares
                },
                {
                    fn beaconChainETHStrategy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::beaconChainETHStrategy)
                    }
                    beaconChainETHStrategy
                },
                {
                    fn completeQueuedWithdrawals_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <completeQueuedWithdrawals_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::completeQueuedWithdrawals_1)
                    }
                    completeQueuedWithdrawals_1
                },
                {
                    fn queuedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <queuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::queuedWithdrawals)
                    }
                    queuedWithdrawals
                },
                {
                    fn cumulativeWithdrawalsQueued(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::cumulativeWithdrawalsQueued)
                    }
                    cumulativeWithdrawalsQueued
                },
                {
                    fn redelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <redelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::redelegate)
                    }
                    redelegate
                },
                {
                    fn pendingWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pendingWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::pendingWithdrawals)
                    }
                    pendingWithdrawals
                },
                {
                    fn delegationApproverSaltIsSpent(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::delegationApproverSaltIsSpent)
                    }
                    delegationApproverSaltIsSpent
                },
                {
                    fn depositScalingFactor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <depositScalingFactorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::depositScalingFactor)
                    }
                    depositScalingFactor
                },
                {
                    fn minWithdrawalDelayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::minWithdrawalDelayBlocks)
                    }
                    minWithdrawalDelayBlocks
                },
                {
                    fn getWithdrawableShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getWithdrawableSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getWithdrawableShares)
                    }
                    getWithdrawableShares
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn undelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <undelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::undelegate)
                    }
                    undelegate
                },
                {
                    fn completeQueuedWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::completeQueuedWithdrawal)
                    }
                    completeQueuedWithdrawal
                },
                {
                    fn burnOperatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <burnOperatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::burnOperatorShares)
                    }
                    burnOperatorShares
                },
                {
                    fn delegateTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegateToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::delegateTo)
                    }
                    delegateTo
                },
                {
                    fn getOperatorsShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getOperatorsSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getOperatorsShares)
                    }
                    getOperatorsShares
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::domainSeparator)
                    }
                    domainSeparator
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::unpause)
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
                Self::DELEGATION_APPROVAL_TYPEHASH(inner) => {
                    <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beaconChainETHStrategy(inner) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::burnOperatorShares(inner) => {
                    <burnOperatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateDelegationApprovalDigestHash(inner) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateWithdrawalRoot(inner) => {
                    <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeQueuedWithdrawal(inner) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeQueuedWithdrawals_0(inner) => {
                    <completeQueuedWithdrawals_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeQueuedWithdrawals_1(inner) => {
                    <completeQueuedWithdrawals_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cumulativeWithdrawalsQueued(inner) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::decreaseDelegatedShares(inner) => {
                    <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegatedTo(inner) => {
                    <delegatedToCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegationApprover(inner) => {
                    <delegationApproverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegationApproverSaltIsSpent(inner) => {
                    <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::depositScalingFactor(inner) => {
                    <depositScalingFactorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDepositedShares(inner) => {
                    <getDepositedSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorShares(inner) => {
                    <getOperatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorsShares(inner) => {
                    <getOperatorsSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQueuedWithdrawals(inner) => {
                    <getQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSlashableSharesInQueue(inner) => {
                    <getSlashableSharesInQueueCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawableShares(inner) => {
                    <getWithdrawableSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::increaseDelegatedShares(inner) => {
                    <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isDelegated(inner) => {
                    <isDelegatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperator(inner) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::minWithdrawalDelayBlocks(inner) => {
                    <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::modifyOperatorDetails(inner) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorShares(inner) => {
                    <operatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::pendingWithdrawals(inner) => {
                    <pendingWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::queuedWithdrawals(inner) => {
                    <queuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::redelegate(inner) => {
                    <redelegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyManager(inner) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::undelegate(inner) => {
                    <undelegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateOperatorMetadataURI(inner) => {
                    <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DELEGATION_APPROVAL_TYPEHASH(inner) => {
                    <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::beaconChainETHStrategy(inner) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::burnOperatorShares(inner) => {
                    <burnOperatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateDelegationApprovalDigestHash(inner) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateWithdrawalRoot(inner) => {
                    <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeQueuedWithdrawal(inner) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeQueuedWithdrawals_0(inner) => {
                    <completeQueuedWithdrawals_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeQueuedWithdrawals_1(inner) => {
                    <completeQueuedWithdrawals_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cumulativeWithdrawalsQueued(inner) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::decreaseDelegatedShares(inner) => {
                    <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegatedTo(inner) => {
                    <delegatedToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegationApprover(inner) => {
                    <delegationApproverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegationApproverSaltIsSpent(inner) => {
                    <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::depositScalingFactor(inner) => {
                    <depositScalingFactorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDepositedShares(inner) => {
                    <getDepositedSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorShares(inner) => {
                    <getOperatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorsShares(inner) => {
                    <getOperatorsSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQueuedWithdrawals(inner) => {
                    <getQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSlashableSharesInQueue(inner) => {
                    <getSlashableSharesInQueueCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawableShares(inner) => {
                    <getWithdrawableSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::increaseDelegatedShares(inner) => {
                    <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isDelegated(inner) => {
                    <isDelegatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperator(inner) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minWithdrawalDelayBlocks(inner) => {
                    <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::modifyOperatorDetails(inner) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorShares(inner) => {
                    <operatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::pendingWithdrawals(inner) => {
                    <pendingWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::queuedWithdrawals(inner) => {
                    <queuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::redelegate(inner) => {
                    <redelegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::strategyManager(inner) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::undelegate(inner) => {
                    <undelegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateOperatorMetadataURI(inner) => {
                    <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`DelegationManager`](self) custom errors.
    pub enum DelegationManagerErrors {
        ActivelyDelegated(ActivelyDelegated),
        CallerCannotUndelegate(CallerCannotUndelegate),
        CurrentlyPaused(CurrentlyPaused),
        FullySlashed(FullySlashed),
        InputAddressZero(InputAddressZero),
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        InputArrayLengthZero(InputArrayLengthZero),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        InvalidPermissions(InvalidPermissions),
        InvalidSignature(InvalidSignature),
        InvalidSnapshotOrdering(InvalidSnapshotOrdering),
        NotActivelyDelegated(NotActivelyDelegated),
        OnlyAllocationManager(OnlyAllocationManager),
        OnlyEigenPodManager(OnlyEigenPodManager),
        OnlyPauser(OnlyPauser),
        OnlyStrategyManagerOrEigenPodManager(OnlyStrategyManagerOrEigenPodManager),
        OnlyUnpauser(OnlyUnpauser),
        OperatorNotRegistered(OperatorNotRegistered),
        OperatorsCannotUndelegate(OperatorsCannotUndelegate),
        SaltSpent(SaltSpent),
        SignatureExpired(SignatureExpired),
        WithdrawalDelayNotElapsed(WithdrawalDelayNotElapsed),
        WithdrawalExceedsMax(WithdrawalExceedsMax),
        WithdrawalNotQueued(WithdrawalNotQueued),
        WithdrawerNotCaller(WithdrawerNotCaller),
        WithdrawerNotStaker(WithdrawerNotStaker),
    }
    #[automatically_derived]
    impl DelegationManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 25u8, 189u8, 205u8],
            [17u8, 72u8, 26u8, 148u8],
            [35u8, 216u8, 113u8, 165u8],
            [37u8, 236u8, 108u8, 31u8],
            [40u8, 206u8, 241u8, 164u8],
            [42u8, 55u8, 28u8, 126u8],
            [53u8, 49u8, 50u8, 68u8],
            [60u8, 147u8, 52u8, 70u8],
            [67u8, 113u8, 74u8, 253u8],
            [88u8, 68u8, 52u8, 212u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [119u8, 229u8, 106u8, 6u8],
            [121u8, 72u8, 33u8, 255u8],
            [121u8, 108u8, 197u8, 37u8],
            [132u8, 10u8, 72u8, 213u8],
            [135u8, 201u8, 210u8, 25u8],
            [139u8, 170u8, 87u8, 159u8],
            [142u8, 81u8, 153u8, 168u8],
            [147u8, 45u8, 148u8, 247u8],
            [165u8, 199u8, 196u8, 69u8],
            [195u8, 17u8, 197u8, 164u8],
            [198u8, 29u8, 202u8, 93u8],
            [200u8, 78u8, 153u8, 132u8],
            [240u8, 32u8, 229u8, 185u8],
            [241u8, 236u8, 245u8, 194u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelegationManagerErrors {
        const NAME: &'static str = "DelegationManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 26usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ActivelyDelegated(_) => {
                    <ActivelyDelegated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CallerCannotUndelegate(_) => {
                    <CallerCannotUndelegate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FullySlashed(_) => {
                    <FullySlashed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthZero(_) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPermissions(_) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSnapshotOrdering(_) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotActivelyDelegated(_) => {
                    <NotActivelyDelegated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyAllocationManager(_) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEigenPodManager(_) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => {
                    <OnlyPauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyStrategyManagerOrEigenPodManager(_) => {
                    <OnlyStrategyManagerOrEigenPodManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorsCannotUndelegate(_) => {
                    <OperatorsCannotUndelegate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SaltSpent(_) => <SaltSpent as alloy_sol_types::SolError>::SELECTOR,
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalDelayNotElapsed(_) => {
                    <WithdrawalDelayNotElapsed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalExceedsMax(_) => {
                    <WithdrawalExceedsMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalNotQueued(_) => {
                    <WithdrawalNotQueued as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawerNotCaller(_) => {
                    <WithdrawerNotCaller as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawerNotStaker(_) => {
                    <WithdrawerNotStaker as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<DelegationManagerErrors>] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn OnlyStrategyManagerOrEigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OnlyStrategyManagerOrEigenPodManager as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerErrors::OnlyStrategyManagerOrEigenPodManager,
                            )
                    }
                    OnlyStrategyManagerOrEigenPodManager
                },
                {
                    fn OnlyAllocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OnlyAllocationManager as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OnlyAllocationManager)
                    }
                    OnlyAllocationManager
                },
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn FullySlashed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <FullySlashed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::FullySlashed)
                    }
                    FullySlashed
                },
                {
                    fn InvalidSnapshotOrdering(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InvalidSnapshotOrdering)
                    }
                    InvalidSnapshotOrdering
                },
                {
                    fn SaltSpent(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <SaltSpent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::SaltSpent)
                    }
                    SaltSpent
                },
                {
                    fn CallerCannotUndelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <CallerCannotUndelegate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::CallerCannotUndelegate)
                    }
                    CallerCannotUndelegate
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn WithdrawerNotCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawerNotCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawerNotCaller)
                    }
                    WithdrawerNotCaller
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn ActivelyDelegated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <ActivelyDelegated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::ActivelyDelegated)
                    }
                    ActivelyDelegated
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn InputArrayLengthZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InputArrayLengthZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InputArrayLengthZero)
                    }
                    InputArrayLengthZero
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn WithdrawalNotQueued(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawalNotQueued as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawalNotQueued)
                    }
                    WithdrawalNotQueued
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn OperatorsCannotUndelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OperatorsCannotUndelegate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OperatorsCannotUndelegate)
                    }
                    OperatorsCannotUndelegate
                },
                {
                    fn InvalidPermissions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InvalidPermissions as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InvalidPermissions)
                    }
                    InvalidPermissions
                },
                {
                    fn NotActivelyDelegated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <NotActivelyDelegated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::NotActivelyDelegated)
                    }
                    NotActivelyDelegated
                },
                {
                    fn WithdrawerNotStaker(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawerNotStaker as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawerNotStaker)
                    }
                    WithdrawerNotStaker
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn OnlyEigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OnlyEigenPodManager)
                    }
                    OnlyEigenPodManager
                },
                {
                    fn WithdrawalExceedsMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawalExceedsMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawalExceedsMax)
                    }
                    WithdrawalExceedsMax
                },
                {
                    fn WithdrawalDelayNotElapsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawalDelayNotElapsed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawalDelayNotElapsed)
                    }
                    WithdrawalDelayNotElapsed
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
                Self::ActivelyDelegated(inner) => {
                    <ActivelyDelegated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CallerCannotUndelegate(inner) => {
                    <CallerCannotUndelegate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FullySlashed(inner) => {
                    <FullySlashed as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InputArrayLengthZero(inner) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPermissions(inner) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSnapshotOrdering(inner) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotActivelyDelegated(inner) => {
                    <NotActivelyDelegated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyAllocationManager(inner) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEigenPodManager(inner) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyStrategyManagerOrEigenPodManager(inner) => {
                    <OnlyStrategyManagerOrEigenPodManager as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::OperatorsCannotUndelegate(inner) => {
                    <OperatorsCannotUndelegate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SaltSpent(inner) => {
                    <SaltSpent as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawalDelayNotElapsed(inner) => {
                    <WithdrawalDelayNotElapsed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawalExceedsMax(inner) => {
                    <WithdrawalExceedsMax as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawalNotQueued(inner) => {
                    <WithdrawalNotQueued as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawerNotCaller(inner) => {
                    <WithdrawerNotCaller as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawerNotStaker(inner) => {
                    <WithdrawerNotStaker as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ActivelyDelegated(inner) => {
                    <ActivelyDelegated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CallerCannotUndelegate(inner) => {
                    <CallerCannotUndelegate as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::FullySlashed(inner) => {
                    <FullySlashed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InputArrayLengthZero(inner) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidPermissions(inner) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidSnapshotOrdering(inner) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotActivelyDelegated(inner) => {
                    <NotActivelyDelegated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyAllocationManager(inner) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEigenPodManager(inner) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyStrategyManagerOrEigenPodManager(inner) => {
                    <OnlyStrategyManagerOrEigenPodManager as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OperatorsCannotUndelegate(inner) => {
                    <OperatorsCannotUndelegate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SaltSpent(inner) => {
                    <SaltSpent as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawalDelayNotElapsed(inner) => {
                    <WithdrawalDelayNotElapsed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawalExceedsMax(inner) => {
                    <WithdrawalExceedsMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawalNotQueued(inner) => {
                    <WithdrawalNotQueued as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawerNotCaller(inner) => {
                    <WithdrawerNotCaller as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawerNotStaker(inner) => {
                    <WithdrawerNotStaker as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`DelegationManager`](self) events.
    pub enum DelegationManagerEvents {
        DelegationApproverUpdated(DelegationApproverUpdated),
        DepositScalingFactorUpdated(DepositScalingFactorUpdated),
        Initialized(Initialized),
        OperatorMetadataURIUpdated(OperatorMetadataURIUpdated),
        OperatorRegistered(OperatorRegistered),
        OperatorSharesBurned(OperatorSharesBurned),
        OperatorSharesDecreased(OperatorSharesDecreased),
        OperatorSharesIncreased(OperatorSharesIncreased),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        SlashingWithdrawalCompleted(SlashingWithdrawalCompleted),
        SlashingWithdrawalQueued(SlashingWithdrawalQueued),
        StakerDelegated(StakerDelegated),
        StakerForceUndelegated(StakerForceUndelegated),
        StakerUndelegated(StakerUndelegated),
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl DelegationManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                2u8,
                169u8,
                25u8,
                237u8,
                14u8,
                42u8,
                202u8,
                209u8,
                221u8,
                144u8,
                241u8,
                126u8,
                242u8,
                250u8,
                74u8,
                229u8,
                70u8,
                46u8,
                225u8,
                51u8,
                145u8,
                112u8,
                3u8,
                74u8,
                133u8,
                49u8,
                204u8,
                164u8,
                182u8,
                112u8,
                128u8,
                144u8,
            ],
            [
                30u8,
                192u8,
                66u8,
                201u8,
                101u8,
                226u8,
                237u8,
                215u8,
                16u8,
                123u8,
                81u8,
                24u8,
                142u8,
                224u8,
                243u8,
                131u8,
                226u8,
                46u8,
                118u8,
                23u8,
                144u8,
                65u8,
                171u8,
                58u8,
                157u8,
                24u8,
                255u8,
                21u8,
                20u8,
                5u8,
                22u8,
                108u8,
            ],
            [
                31u8,
                64u8,
                64u8,
                8u8,
                137u8,
                39u8,
                78u8,
                208u8,
                123u8,
                36u8,
                132u8,
                94u8,
                80u8,
                84u8,
                168u8,
                122u8,
                12u8,
                171u8,
                150u8,
                158u8,
                177u8,
                39u8,
                122u8,
                175u8,
                230u8,
                26u8,
                227u8,
                82u8,
                231u8,
                195u8,
                42u8,
                0u8,
            ],
            [
                38u8,
                178u8,
                170u8,
                226u8,
                101u8,
                22u8,
                232u8,
                113u8,
                158u8,
                245u8,
                14u8,
                162u8,
                246u8,
                131u8,
                26u8,
                46u8,
                251u8,
                212u8,
                227u8,
                125u8,
                204u8,
                223u8,
                15u8,
                105u8,
                54u8,
                178u8,
                123u8,
                192u8,
                142u8,
                121u8,
                62u8,
                48u8,
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
                105u8,
                9u8,
                96u8,
                0u8,
                55u8,
                183u8,
                93u8,
                123u8,
                71u8,
                51u8,
                174u8,
                221u8,
                129u8,
                84u8,
                66u8,
                181u8,
                236u8,
                1u8,
                138u8,
                130u8,
                119u8,
                81u8,
                200u8,
                50u8,
                170u8,
                255u8,
                100u8,
                235u8,
                165u8,
                214u8,
                210u8,
                221u8,
            ],
            [
                119u8,
                59u8,
                84u8,
                192u8,
                77u8,
                117u8,
                111u8,
                204u8,
                94u8,
                103u8,
                129u8,
                17u8,
                247u8,
                215u8,
                48u8,
                222u8,
                59u8,
                233u8,
                129u8,
                146u8,
                0u8,
                7u8,
                153u8,
                238u8,
                227u8,
                214u8,
                55u8,
                22u8,
                5u8,
                90u8,
                135u8,
                198u8,
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
                139u8,
                233u8,
                50u8,
                186u8,
                197u8,
                69u8,
                97u8,
                242u8,
                114u8,
                96u8,
                249u8,
                84u8,
                99u8,
                217u8,
                184u8,
                171u8,
                55u8,
                224u8,
                107u8,
                40u8,
                66u8,
                229u8,
                238u8,
                36u8,
                4u8,
                21u8,
                124u8,
                193u8,
                61u8,
                246u8,
                235u8,
                143u8,
            ],
            [
                164u8,
                83u8,
                219u8,
                97u8,
                42u8,
                245u8,
                158u8,
                85u8,
                33u8,
                214u8,
                171u8,
                146u8,
                132u8,
                220u8,
                62u8,
                45u8,
                6u8,
                175u8,
                40u8,
                110u8,
                177u8,
                177u8,
                183u8,
                183u8,
                113u8,
                252u8,
                228u8,
                113u8,
                108u8,
                25u8,
                242u8,
                193u8,
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
                195u8,
                238u8,
                159u8,
                46u8,
                95u8,
                218u8,
                152u8,
                232u8,
                6u8,
                106u8,
                31u8,
                116u8,
                91u8,
                45u8,
                249u8,
                40u8,
                95u8,
                65u8,
                111u8,
                233u8,
                140u8,
                242u8,
                85u8,
                156u8,
                210u8,
                20u8,
                132u8,
                179u8,
                216u8,
                116u8,
                51u8,
                4u8,
            ],
            [
                239u8,
                246u8,
                170u8,
                178u8,
                188u8,
                63u8,
                124u8,
                100u8,
                136u8,
                150u8,
                225u8,
                82u8,
                46u8,
                174u8,
                113u8,
                214u8,
                194u8,
                46u8,
                59u8,
                14u8,
                33u8,
                130u8,
                6u8,
                179u8,
                244u8,
                10u8,
                240u8,
                228u8,
                210u8,
                4u8,
                113u8,
                107u8,
            ],
            [
                240u8,
                237u8,
                223u8,
                7u8,
                230u8,
                234u8,
                20u8,
                243u8,
                136u8,
                180u8,
                126u8,
                30u8,
                148u8,
                160u8,
                244u8,
                100u8,
                236u8,
                189u8,
                158u8,
                237u8,
                65u8,
                113u8,
                19u8,
                14u8,
                15u8,
                192u8,
                233u8,
                159u8,
                180u8,
                3u8,
                10u8,
                138u8,
            ],
            [
                254u8,
                227u8,
                9u8,
                102u8,
                162u8,
                86u8,
                183u8,
                30u8,
                20u8,
                188u8,
                14u8,
                191u8,
                201u8,
                67u8,
                21u8,
                226u8,
                142u8,
                244u8,
                169u8,
                122u8,
                113u8,
                49u8,
                169u8,
                226u8,
                183u8,
                163u8,
                16u8,
                167u8,
                58u8,
                244u8,
                70u8,
                118u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for DelegationManagerEvents {
        const NAME: &'static str = "DelegationManagerEvents";
        const COUNT: usize = 16usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <DelegationApproverUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DelegationApproverUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DelegationApproverUpdated)
                }
                Some(
                    <DepositScalingFactorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DepositScalingFactorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DepositScalingFactorUpdated)
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
                    <OperatorMetadataURIUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorMetadataURIUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorMetadataURIUpdated)
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
                    <OperatorSharesBurned as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSharesBurned as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSharesBurned)
                }
                Some(
                    <OperatorSharesDecreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSharesDecreased as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSharesDecreased)
                }
                Some(
                    <OperatorSharesIncreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSharesIncreased as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSharesIncreased)
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
                    <SlashingWithdrawalCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SlashingWithdrawalCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashingWithdrawalCompleted)
                }
                Some(
                    <SlashingWithdrawalQueued as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SlashingWithdrawalQueued as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashingWithdrawalQueued)
                }
                Some(<StakerDelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakerDelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakerDelegated)
                }
                Some(
                    <StakerForceUndelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StakerForceUndelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakerForceUndelegated)
                }
                Some(
                    <StakerUndelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StakerUndelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakerUndelegated)
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
    impl alloy_sol_types::private::IntoLogData for DelegationManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DelegationApproverUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DepositScalingFactorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSharesBurned(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSharesDecreased(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSharesIncreased(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlashingWithdrawalCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlashingWithdrawalQueued(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakerDelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakerForceUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakerUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DelegationApproverUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DepositScalingFactorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSharesBurned(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSharesDecreased(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSharesIncreased(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlashingWithdrawalCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlashingWithdrawalQueued(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakerDelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakerForceUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakerUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`DelegationManager`](self) contract instance.

See the [wrapper's documentation](`DelegationManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DelegationManagerInstance<T, P, N> {
        DelegationManagerInstance::<T, P, N>::new(address, provider)
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
        _strategyManager: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _MIN_WITHDRAWAL_DELAY: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<DelegationManagerInstance<T, P, N>>,
    > {
        DelegationManagerInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _strategyManager,
            _eigenPodManager,
            _allocationManager,
            _pauserRegistry,
            _permissionController,
            _MIN_WITHDRAWAL_DELAY,
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
        _strategyManager: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _MIN_WITHDRAWAL_DELAY: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        DelegationManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _strategyManager,
            _eigenPodManager,
            _allocationManager,
            _pauserRegistry,
            _permissionController,
            _MIN_WITHDRAWAL_DELAY,
        )
    }
    /**A [`DelegationManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`DelegationManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DelegationManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for DelegationManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DelegationManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DelegationManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`DelegationManager`](self) contract instance.

See the [wrapper's documentation](`DelegationManagerInstance`) for more details.*/
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
            _strategyManager: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _MIN_WITHDRAWAL_DELAY: u32,
        ) -> alloy_contract::Result<DelegationManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _strategyManager,
                _eigenPodManager,
                _allocationManager,
                _pauserRegistry,
                _permissionController,
                _MIN_WITHDRAWAL_DELAY,
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
            _strategyManager: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _MIN_WITHDRAWAL_DELAY: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _strategyManager,
                            _eigenPodManager,
                            _allocationManager,
                            _pauserRegistry,
                            _permissionController,
                            _MIN_WITHDRAWAL_DELAY,
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
    impl<T, P: ::core::clone::Clone, N> DelegationManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DelegationManagerInstance<T, P, N> {
            DelegationManagerInstance {
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
    > DelegationManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`DELEGATION_APPROVAL_TYPEHASH`] function.
        pub fn DELEGATION_APPROVAL_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DELEGATION_APPROVAL_TYPEHASHCall, N> {
            self.call_builder(
                &DELEGATION_APPROVAL_TYPEHASHCall {
                },
            )
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`beaconChainETHStrategy`] function.
        pub fn beaconChainETHStrategy(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, beaconChainETHStrategyCall, N> {
            self.call_builder(&beaconChainETHStrategyCall {})
        }
        ///Creates a new call builder for the [`burnOperatorShares`] function.
        pub fn burnOperatorShares(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            prevMaxMagnitude: u64,
            newMaxMagnitude: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, burnOperatorSharesCall, N> {
            self.call_builder(
                &burnOperatorSharesCall {
                    operator,
                    strategy,
                    prevMaxMagnitude,
                    newMaxMagnitude,
                },
            )
        }
        ///Creates a new call builder for the [`calculateDelegationApprovalDigestHash`] function.
        pub fn calculateDelegationApprovalDigestHash(
            &self,
            staker: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
            approver: alloy::sol_types::private::Address,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            calculateDelegationApprovalDigestHashCall,
            N,
        > {
            self.call_builder(
                &calculateDelegationApprovalDigestHashCall {
                    staker,
                    operator,
                    approver,
                    approverSalt,
                    expiry,
                },
            )
        }
        ///Creates a new call builder for the [`calculateWithdrawalRoot`] function.
        pub fn calculateWithdrawalRoot(
            &self,
            withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateWithdrawalRootCall, N> {
            self.call_builder(
                &calculateWithdrawalRootCall {
                    withdrawal,
                },
            )
        }
        ///Creates a new call builder for the [`completeQueuedWithdrawal`] function.
        pub fn completeQueuedWithdrawal(
            &self,
            withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
            tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            receiveAsTokens: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeQueuedWithdrawalCall, N> {
            self.call_builder(
                &completeQueuedWithdrawalCall {
                    withdrawal,
                    tokens,
                    receiveAsTokens,
                },
            )
        }
        ///Creates a new call builder for the [`completeQueuedWithdrawals_0`] function.
        pub fn completeQueuedWithdrawals_0(
            &self,
            tokens: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            receiveAsTokens: alloy::sol_types::private::Vec<bool>,
            numToComplete: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeQueuedWithdrawals_0Call, N> {
            self.call_builder(
                &completeQueuedWithdrawals_0Call {
                    tokens,
                    receiveAsTokens,
                    numToComplete,
                },
            )
        }
        ///Creates a new call builder for the [`completeQueuedWithdrawals_1`] function.
        pub fn completeQueuedWithdrawals_1(
            &self,
            withdrawals: alloy::sol_types::private::Vec<
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
            >,
            tokens: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            receiveAsTokens: alloy::sol_types::private::Vec<bool>,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeQueuedWithdrawals_1Call, N> {
            self.call_builder(
                &completeQueuedWithdrawals_1Call {
                    withdrawals,
                    tokens,
                    receiveAsTokens,
                },
            )
        }
        ///Creates a new call builder for the [`cumulativeWithdrawalsQueued`] function.
        pub fn cumulativeWithdrawalsQueued(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, cumulativeWithdrawalsQueuedCall, N> {
            self.call_builder(
                &cumulativeWithdrawalsQueuedCall {
                    staker,
                },
            )
        }
        ///Creates a new call builder for the [`decreaseDelegatedShares`] function.
        pub fn decreaseDelegatedShares(
            &self,
            staker: alloy::sol_types::private::Address,
            curDepositShares: alloy::sol_types::private::primitives::aliases::U256,
            beaconChainSlashingFactorDecrease: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, decreaseDelegatedSharesCall, N> {
            self.call_builder(
                &decreaseDelegatedSharesCall {
                    staker,
                    curDepositShares,
                    beaconChainSlashingFactorDecrease,
                },
            )
        }
        ///Creates a new call builder for the [`delegateTo`] function.
        pub fn delegateTo(
            &self,
            operator: alloy::sol_types::private::Address,
            approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateToCall, N> {
            self.call_builder(
                &delegateToCall {
                    operator,
                    approverSignatureAndExpiry,
                    approverSalt,
                },
            )
        }
        ///Creates a new call builder for the [`delegatedTo`] function.
        pub fn delegatedTo(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegatedToCall, N> {
            self.call_builder(&delegatedToCall { staker })
        }
        ///Creates a new call builder for the [`delegationApprover`] function.
        pub fn delegationApprover(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationApproverCall, N> {
            self.call_builder(&delegationApproverCall { operator })
        }
        ///Creates a new call builder for the [`delegationApproverSaltIsSpent`] function.
        pub fn delegationApproverSaltIsSpent(
            &self,
            delegationApprover: alloy::sol_types::private::Address,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            delegationApproverSaltIsSpentCall,
            N,
        > {
            self.call_builder(
                &delegationApproverSaltIsSpentCall {
                    delegationApprover,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`depositScalingFactor`] function.
        pub fn depositScalingFactor(
            &self,
            staker: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositScalingFactorCall, N> {
            self.call_builder(
                &depositScalingFactorCall {
                    staker,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`domainSeparator`] function.
        pub fn domainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, domainSeparatorCall, N> {
            self.call_builder(&domainSeparatorCall {})
        }
        ///Creates a new call builder for the [`eigenPodManager`] function.
        pub fn eigenPodManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenPodManagerCall, N> {
            self.call_builder(&eigenPodManagerCall {})
        }
        ///Creates a new call builder for the [`getDepositedShares`] function.
        pub fn getDepositedShares(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDepositedSharesCall, N> {
            self.call_builder(&getDepositedSharesCall { staker })
        }
        ///Creates a new call builder for the [`getOperatorShares`] function.
        pub fn getOperatorShares(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSharesCall, N> {
            self.call_builder(
                &getOperatorSharesCall {
                    operator,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorsShares`] function.
        pub fn getOperatorsShares(
            &self,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorsSharesCall, N> {
            self.call_builder(
                &getOperatorsSharesCall {
                    operators,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`getQueuedWithdrawals`] function.
        pub fn getQueuedWithdrawals(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQueuedWithdrawalsCall, N> {
            self.call_builder(&getQueuedWithdrawalsCall { staker })
        }
        ///Creates a new call builder for the [`getSlashableSharesInQueue`] function.
        pub fn getSlashableSharesInQueue(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlashableSharesInQueueCall, N> {
            self.call_builder(
                &getSlashableSharesInQueueCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getWithdrawableShares`] function.
        pub fn getWithdrawableShares(
            &self,
            staker: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawableSharesCall, N> {
            self.call_builder(
                &getWithdrawableSharesCall {
                    staker,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`increaseDelegatedShares`] function.
        pub fn increaseDelegatedShares(
            &self,
            staker: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            prevDepositShares: alloy::sol_types::private::primitives::aliases::U256,
            addedShares: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, increaseDelegatedSharesCall, N> {
            self.call_builder(
                &increaseDelegatedSharesCall {
                    staker,
                    strategy,
                    prevDepositShares,
                    addedShares,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initialOwner,
                    initialPausedStatus,
                },
            )
        }
        ///Creates a new call builder for the [`isDelegated`] function.
        pub fn isDelegated(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isDelegatedCall, N> {
            self.call_builder(&isDelegatedCall { staker })
        }
        ///Creates a new call builder for the [`isOperator`] function.
        pub fn isOperator(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorCall, N> {
            self.call_builder(&isOperatorCall { operator })
        }
        ///Creates a new call builder for the [`minWithdrawalDelayBlocks`] function.
        pub fn minWithdrawalDelayBlocks(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, minWithdrawalDelayBlocksCall, N> {
            self.call_builder(&minWithdrawalDelayBlocksCall {})
        }
        ///Creates a new call builder for the [`modifyOperatorDetails`] function.
        pub fn modifyOperatorDetails(
            &self,
            operator: alloy::sol_types::private::Address,
            newDelegationApprover: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyOperatorDetailsCall, N> {
            self.call_builder(
                &modifyOperatorDetailsCall {
                    operator,
                    newDelegationApprover,
                },
            )
        }
        ///Creates a new call builder for the [`operatorShares`] function.
        pub fn operatorShares(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSharesCall, N> {
            self.call_builder(
                &operatorSharesCall {
                    operator,
                    strategy,
                },
            )
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
        ///Creates a new call builder for the [`pendingWithdrawals`] function.
        pub fn pendingWithdrawals(
            &self,
            withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, pendingWithdrawalsCall, N> {
            self.call_builder(
                &pendingWithdrawalsCall {
                    withdrawalRoot,
                },
            )
        }
        ///Creates a new call builder for the [`permissionController`] function.
        pub fn permissionController(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, permissionControllerCall, N> {
            self.call_builder(&permissionControllerCall {})
        }
        ///Creates a new call builder for the [`queueWithdrawals`] function.
        pub fn queueWithdrawals(
            &self,
            params: alloy::sol_types::private::Vec<
                <IDelegationManagerTypes::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, queueWithdrawalsCall, N> {
            self.call_builder(&queueWithdrawalsCall { params })
        }
        ///Creates a new call builder for the [`queuedWithdrawals`] function.
        pub fn queuedWithdrawals(
            &self,
            withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, queuedWithdrawalsCall, N> {
            self.call_builder(
                &queuedWithdrawalsCall {
                    withdrawalRoot,
                },
            )
        }
        ///Creates a new call builder for the [`redelegate`] function.
        pub fn redelegate(
            &self,
            newOperator: alloy::sol_types::private::Address,
            newOperatorApproverSig: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, redelegateCall, N> {
            self.call_builder(
                &redelegateCall {
                    newOperator,
                    newOperatorApproverSig,
                    approverSalt,
                },
            )
        }
        ///Creates a new call builder for the [`registerAsOperator`] function.
        pub fn registerAsOperator(
            &self,
            initDelegationApprover: alloy::sol_types::private::Address,
            allocationDelay: u32,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerAsOperatorCall, N> {
            self.call_builder(
                &registerAsOperatorCall {
                    initDelegationApprover,
                    allocationDelay,
                    metadataURI,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`strategyManager`] function.
        pub fn strategyManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyManagerCall, N> {
            self.call_builder(&strategyManagerCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`undelegate`] function.
        pub fn undelegate(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, undelegateCall, N> {
            self.call_builder(&undelegateCall { staker })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`updateOperatorMetadataURI`] function.
        pub fn updateOperatorMetadataURI(
            &self,
            operator: alloy::sol_types::private::Address,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorMetadataURICall, N> {
            self.call_builder(
                &updateOperatorMetadataURICall {
                    operator,
                    metadataURI,
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
    > DelegationManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`DelegationApproverUpdated`] event.
        pub fn DelegationApproverUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelegationApproverUpdated, N> {
            self.event_filter::<DelegationApproverUpdated>()
        }
        ///Creates a new event filter for the [`DepositScalingFactorUpdated`] event.
        pub fn DepositScalingFactorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DepositScalingFactorUpdated, N> {
            self.event_filter::<DepositScalingFactorUpdated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OperatorMetadataURIUpdated`] event.
        pub fn OperatorMetadataURIUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorMetadataURIUpdated, N> {
            self.event_filter::<OperatorMetadataURIUpdated>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorSharesBurned`] event.
        pub fn OperatorSharesBurned_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSharesBurned, N> {
            self.event_filter::<OperatorSharesBurned>()
        }
        ///Creates a new event filter for the [`OperatorSharesDecreased`] event.
        pub fn OperatorSharesDecreased_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSharesDecreased, N> {
            self.event_filter::<OperatorSharesDecreased>()
        }
        ///Creates a new event filter for the [`OperatorSharesIncreased`] event.
        pub fn OperatorSharesIncreased_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSharesIncreased, N> {
            self.event_filter::<OperatorSharesIncreased>()
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
        ///Creates a new event filter for the [`SlashingWithdrawalCompleted`] event.
        pub fn SlashingWithdrawalCompleted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashingWithdrawalCompleted, N> {
            self.event_filter::<SlashingWithdrawalCompleted>()
        }
        ///Creates a new event filter for the [`SlashingWithdrawalQueued`] event.
        pub fn SlashingWithdrawalQueued_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashingWithdrawalQueued, N> {
            self.event_filter::<SlashingWithdrawalQueued>()
        }
        ///Creates a new event filter for the [`StakerDelegated`] event.
        pub fn StakerDelegated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakerDelegated, N> {
            self.event_filter::<StakerDelegated>()
        }
        ///Creates a new event filter for the [`StakerForceUndelegated`] event.
        pub fn StakerForceUndelegated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakerForceUndelegated, N> {
            self.event_filter::<StakerForceUndelegated>()
        }
        ///Creates a new event filter for the [`StakerUndelegated`] event.
        pub fn StakerUndelegated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakerUndelegated, N> {
            self.event_filter::<StakerUndelegated>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
