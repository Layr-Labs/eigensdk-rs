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
    clippy::style
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub depositShares:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
        impl alloy_sol_types::SolType for QueuedWithdrawalParams {
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
        impl alloy_sol_types::SolStruct for QueuedWithdrawalParams {
            const NAME: &'static str = "QueuedWithdrawalParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QueuedWithdrawalParams(address[] strategies,uint256[] depositShares,address withdrawer)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub scaledShares:
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
        impl alloy_sol_types::SolType for Withdrawal {
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
        impl alloy_sol_types::SolStruct for Withdrawal {
            const NAME: &'static str = "Withdrawal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Withdrawal(address staker,address delegatedTo,address withdrawer,uint256 nonce,uint32 startBlock,address[] strategies,uint256[] scaledShares)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
    pub struct IDelegationManagerTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
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
        > IDelegationManagerTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IDelegationManagerTypes`](self) contract instance.

        See the [wrapper's documentation](`IDelegationManagerTypesInstance`) for more details.*/
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
        > IDelegationManagerTypesInstance<T, P, N>
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
        > IDelegationManagerTypesInstance<T, P, N>
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.expiry,
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
        impl alloy_sol_types::SolType for SignatureWithExpiry {
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
        impl alloy_sol_types::SolStruct for SignatureWithExpiry {
            const NAME: &'static str = "SignatureWithExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithExpiry(bytes signature,uint256 expiry)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            f.debug_tuple("ISignatureUtilsInstance")
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
        > ISignatureUtilsInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

        See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
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
        > ISignatureUtilsInstance<T, P, N>
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
        > ISignatureUtilsInstance<T, P, N>
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
    clippy::style
)]
pub mod DelegationManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610180806040523461029157601f61563b38819003918201601f19168301916001600160401b038311848410176102955780849260c0946040528339810103126102915780516001600160a01b03811681036102915760208201516001600160a01b0381168103610291576040830151906001600160a01b03821682036102915760608401516001600160a01b0381169390848103610291576080860151956001600160a01b03871687036102915760a001519463ffffffff8616860361029157156102825760805260a05260c05260e052610100524661012052610160525f54600881901c60ff1661022d5760ff808216106101f3575b60405161539190816102aa82396080518181816105e701528181610b830152818161167b0152612598015260a051818181610856015281816108be0152818161220701528181612e54015261490c015260c0518181816109bd01528181610ad50152818161132901528181612ed401528181613b0b01526148df015260e05181818161073701528181610935015281816114a201528181611def0152818161213601528181612c10015281816132f30152614b93015261010051818181611bff01528181613ed6015281816143f1015261453e0152610120518161493a01526101405181614960015261016051818181610a910152613d5a0152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100f7565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b6339b190bb60e11b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c806304a4f979146103245780630b9f487a1461031f5780630dd8dd021461031a578063136439dd146103155780632aa6d8881461031057806339b70e381461030b5780633c651cf2146103065780633cdeb5e0146103015780633e28391d146102fc5780634657e26a146102f75780634665bcda146102f257806354b7c96c146102ed578063595c6a67146102e8578063597b36da146102e35780635ac86ab7146102de5780635c975abb146102d95780635dd68579146102d45780635f48e667146102cf57806360a0d1ce146102ca57806365da1264146102c557806366d5ba93146102c05780636d70f7ae146102bb5780636e174448146102b6578063715018a6146102b1578063778e55f3146102ac57806378296ec5146102a7578063886f1195146102a25780638da5cb5b1461029d57806390041347146102985780639104c319146102935780639435bb431461028e57806399f5371b14610289578063a178848414610284578063a33a34331461027f578063b7f06ebe1461027a578063bb45fef214610275578063bfae3fd214610270578063c448feb81461026b578063c978f7ac14610266578063ca8aa7c714610261578063cd6dc6871461025c578063da8be86414610257578063e4cc3f9014610252578063ee74937f1461024d578063eea9064b14610248578063f0e0e67614610243578063f2fde38b1461023e578063f698da25146102395763fabc1cbc14610234575f80fd5b61256f565b612555565b6124c4565b612403565b6122df565b6120f2565b612058565b611f08565b611e1e565b611dda565b611c48565b611be3565b611b98565b611b4a565b611b1b565b6119e1565b6118d7565b611861565b611766565b611738565b61170a565b6116aa565b611666565b6115d7565b611593565b611538565b61144e565b611402565b6113b2565b61136f565b6112fb565b6111a6565b611021565b610e5e565b610e2b565b610df1565b610b58565b610b04565b610ac0565b610a7c565b610a31565b6109eb565b610885565b610841565b6106c8565b6105b7565b610447565b61038f565b610337565b5f91031261033357565b5f80fd5b34610333575f3660031901126103335760206040517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad8152f35b6001600160a01b0381160361033357565b359061038d82610371565b565b346103335760a03660031901126103335760206103d66004356103b181610371565b6024356103bd81610371565b6044356103c981610371565b6064359160843593612633565b604051908152f35b9181601f84011215610333578235916001600160401b038311610333576020808501948460051b01011161033357565b60206040818301928281528451809452019201905f5b8181106104315750505090565b8251845260209384019390920191600101610424565b34610333576020366003190112610333576004356001600160401b038111610333576104779036906004016103de565b9061048f610489600280606654161490565b156126dd565b610498826126f3565b335f908152609a602052604090209092906104bb905b546001600160a01b031690565b5f5b8281106104d657604051806104d2878261040e565b0390f35b806105136104f06104ea6001948789612739565b8061275b565b905061050a61050084888a612739565b602081019061275b565b91905014612790565b6105423361053c610530604061052a868a8c612739565b016127a6565b6001600160a01b031690565b146127b0565b6105a66105676105606105596104ea85898b612739565b3691610c70565b85336132c1565b8661059e8761059661058c610500886105846104ea82878a612739565b959097612739565b9490923691610c70565b923691610ce4565b9086336135bd565b6105b082886127d3565b52016104bd565b346103335760203660031901126103335760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610672576106419261062d915f91610643575b50612807565b61063c6066548281161461281d565b613950565b005b610665915060203d60201161066b575b61065d8183610c1a565b8101906127e7565b5f610627565b503d610653565b6127fc565b6024359063ffffffff8216820361033357565b359063ffffffff8216820361033357565b9181601f84011215610333578235916001600160401b038311610333576020838186019501011161033357565b34610333576060366003190112610333576004356106e581610371565b6106ed610677565b6044356001600160401b0381116103335761070c90369060040161069b565b335f908152609a6020526040902054919391610735906001600160a01b031615612833565b1590565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561033357604051632b6241f360e11b815233600482015263ffffffff9490941660248501525f908490604490829084905af1918215610672577f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080909361082293610827575b506107d18133613982565b6107db33336139e2565b6040516001600160a01b0391909116815233907fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c190602090a2604051918291339583612849565b0390a2005b806108355f61083b93610c1a565b80610329565b5f6107c6565b34610333575f366003190112610333576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610333576080366003190112610333576004356108a281610371565b6024356108ae81610371565b6064356044356001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016331480156109b9575b156109aa576001600160a01b038481165f908152609a602090815260409182902054915163152667d960e31b81529183166004830181905286841660248401529196919592879060449082907f0000000000000000000000000000000000000000000000000000000000000000165afa9586156106725761064196610975915f9161097b575b508383613aaf565b94613ca3565b61099d915060203d6020116109a3575b6109958183610c1a565b810190612870565b5f61096d565b503d61098b565b63045206a560e21b5f5260045ffd5b50337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316146108e7565b3461033357602036600319011261033357600435610a0881610371565b60018060a01b03165f526099602052602060018060a01b03600160405f20015416604051908152f35b34610333576020366003190112610333576020610a72600435610a5381610371565b6001600160a01b039081165f908152609a602052604090205416151590565b6040519015158152f35b34610333575f366003190112610333576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610333575f366003190112610333576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033357604036600319011261033357610641600435610b2481610371565b60243590610b3182610371565b610b42610b3d82613d11565b612885565b610b53610b4e82613014565b61289b565b613982565b34610333575f3660031901126103335760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561067257610bc3915f916106435750612807565b61064161391c565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b03821117610bfa57604052565b610bcb565b60e081019081106001600160401b03821117610bfa57604052565b90601f801991011681019081106001600160401b03821117610bfa57604052565b6040519061038d60e083610c1a565b6040519061038d604083610c1a565b6001600160401b038111610bfa5760051b60200190565b929190610c7c81610c59565b93610c8a6040519586610c1a565b602085838152019160051b810192831161033357905b828210610cac57505050565b602080918335610cbb81610371565b815201910190610ca0565b9080601f8301121561033357816020610ce193359101610c70565b90565b929190610cf081610c59565b93610cfe6040519586610c1a565b602085838152019160051b810192831161033357905b828210610d2057505050565b8135815260209182019101610d14565b9080601f8301121561033357816020610ce193359101610ce4565b91909160e08184031261033357610d60610c3b565b92610d6a82610382565b8452610d7860208301610382565b6020850152610d8960408301610382565b604085015260608201356060850152610da46080830161068a565b608085015260a08201356001600160401b0381116103335781610dc8918401610cc6565b60a085015260c08201356001600160401b03811161033357610dea9201610d30565b60c0830152565b34610333576020366003190112610333576004356001600160401b038111610333576103d6610e266020923690600401610d4b565b6128b1565b346103335760203660031901126103335760043560ff81168091036103335760016020911b806066541614604051908152f35b34610333575f366003190112610333576020606654604051908152f35b90602080835192838152019201905f5b818110610e985750505090565b82516001600160a01b0316845260209384019390920191600101610e8b565b90602080835192838152019201905f5b818110610ed45750505090565b8251845260209384019390920191600101610ec7565b80516001600160a01b039081168352602080830151821690840152604080830151909116908301526060808201519083015260808082015163ffffffff1690830152610ce19160c0610f4b60a084015160e060a085015260e0840190610e7b565b9201519060c0818403910152610eb7565b9080602083519182815201916020808360051b8301019401925f915b838310610f8757505050505090565b9091929394602080610fa5600193601f198682030187528951610eb7565b97019301930191939290610f78565b929160408401936040815282518095526060810194602060608260051b8401019401905f5b818110610ff657505050610ce19394506020818403910152610f5c565b909194602080611012600193605f19888203018c528951610eea565b97019801910196919096610fd9565b346103335760203660031901126103335760043561103e81610371565b6001600160a01b0381165f90815260a36020526040902061105e906129f1565b9081519161106b836128dc565b9161107584612957565b936110936104ae8360018060a01b03165f52609a60205260405f2090565b905f925b8184106110ad57604051806104d2898983610fb4565b6110d36110ce6110bf86889a986127d3565b515f5260a460205260405f2090565b612a39565b6110dd85886127d3565b526110e884876127d3565b5061110160a06110f886896127d3565b510151516126f3565b61110b85876127d3565b5261111684866127d3565b5061113060a061112686896127d3565b51015184836132c1565b925f5b60a061113f878a6127d3565b51015151811015611195578061117a61116860019360c06111608b8e6127d3565b5101516127d3565b5161117389896127d3565b5190614c41565b61118e826111888a8c6127d3565b516127d3565b5201611133565b509496946001909401939250611097565b34610333576060366003190112610333576004356001600160401b038111610333576111d69036906004016103de565b6024356001600160401b038111610333576111f59036906004016103de565b91906044359161120c610489600480606654161490565b61121b600260c9541415612ae9565b600260c955335f90815260a3602052604090206112449080549094818111156112e357506128dc565b945f5b865181101561128d57806112716110ce61126360019489614b1a565b5f5260a460205260405f2090565b61127b828a6127d3565b5261128681896127d3565b5001611247565b50859350845f5b85518110156112d957806112d36112ad600193896127d3565b516112b9838888612b35565b906112cd6112c886898d612b50565b612b60565b92613e59565b01611294565b610641600160c955565b90506128dc565b6001600160401b0381160361033357565b346103335760603660031901126103335760043561131881610371565b604435602435611327826112ea565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036113605761064192612b8d565b633213a66160e21b5f5260045ffd5b346103335760203660031901126103335760043561138c81610371565b60018060a01b03165f52609a602052602060018060a01b0360405f205416604051908152f35b34610333576020366003190112610333576113f46104d26113dd6004356113d881610371565b612e2b565b604092919251938493604085526040850190610e7b565b908382036020850152610eb7565b34610333576020366003190112610333576020610a7260043561142481610371565b613014565b60409060031901126103335760043561144181610371565b90602435610ce181610371565b346103335761145c36611429565b90611465612ac7565b90815115611533576001600160a01b038316602083015260405163547afb8760e01b8152915f908390819061149e9085600484016130ca565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa928315610672576114fb6114ee6104d295611501955f91611511575b506127c6565b516001600160401b031690565b9161438f565b6040519081529081906020820190565b61152d91503d805f833e6115258183610c1a565b810190613047565b5f6114e8565b612725565b34610333575f366003190112610333576115506145eb565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346103335760206115ce6115a636611429565b6001600160a01b039182165f9081526098855260408082209290931681526020919091522090565b54604051908152f35b34610333576040366003190112610333576004356115f481610371565b6024356001600160401b038111610333576116347f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b670809091369060040161069b565b9092611642610b3d82613d11565b61164e610b4e82613014565b61082260405192839260018060a01b03169583612849565b34610333575f366003190112610333576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610333575f366003190112610333576033546040516001600160a01b039091168152602090f35b906040600319830112610333576004356116eb81610371565b91602435906001600160401b03821161033357610ce191600401610cc6565b34610333576104d261172461171e366116d2565b906130ec565b604051918291602083526020830190610eb7565b34610333575f36600319011261033357602060405173beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac08152f35b34610333576060366003190112610333576004356001600160401b038111610333576117969036906004016103de565b6024356001600160401b038111610333576117b59036906004016103de565b90916044356001600160401b038111610333576117d99093919336906004016103de565b906117eb610489600480606654161490565b6117fa600260c9541415612ae9565b600260c9553686900360de1901925f5b868110156112d9578060051b88013590858212156103335761185b600192611833838a87612b35565b906118568d611843878c8c612b50565b359461184e8661204e565b369101610d4b565b613e59565b0161180a565b3461033357602036600319011261033357600480355f90815260a460209081526040918290208054600182015460028301546003840154939096015485516001600160a01b03938416815291831694820194909452941692840192909252606083019190915263ffffffff16608082015260a090f35b34610333576020366003190112610333576004356118f481610371565b60018060a01b03165f52609f602052602060405f2054604051908152f35b6001600160401b038111610bfa57601f01601f191660200190565b9060606003198301126103335760043561194681610371565b91602435906001600160401b0382116103335760408282036003190112610333576040519161197483610bdf565b80600401356001600160401b03811161033357810191806023840112156103335760048301356119a381611912565b916119b16040519384610c1a565b81835260248583010111610333576020815f92602480970183860137830101528352013560208201529060443590565b3461033357611a486119f23661192d565b335f908152609a602052604090205492939192611a19906001600160a01b0316151561315f565b611a2b611a2533613014565b15613175565b611a37610b4e85613014565b611a403361468b565b9284336147ff565b611a59610489600180606654161490565b335f908152609a6020526040902080546001600160a01b0319166001600160a01b0384161790556001600160a01b038216337fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433045f80a3611ab833612e2b565b611ac38285336132c1565b915f5b8151811015611b0d57600190611b076001600160a01b03611ae783866127d3565b5116611af383876127d3565b51611afe84896127d3565b5191338b613b80565b01611ac6565b604051806104d2878261040e565b34610333576020366003190112610333576004355f52609e602052602060ff60405f2054166040519015158152f35b3461033357604036600319011261033357600435611b6781610371565b6024359060018060a01b03165f52609c60205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103335760206103d6611bde611bd9611bb136611429565b6001600160a01b039182165f90815260a2875260408082209290931681526020919091522090565b612b6a565b6148a7565b34610333575f36600319011261033357602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b9091611c3a610ce193604084526040840190610eb7565b916020818403910152610eb7565b3461033357611c56366116d2565b611c6081516126f3565b611c6a82516126f3565b91611c9281611c8c6104ae8760018060a01b03165f52609a60205260405f2090565b866132c1565b5f5b8251811015611dc857806020611cc8610530611cc3611cb6611d0996896127d3565b516001600160a01b031690565b6148ba565b611cd5611cb684886127d3565b60405163fe243a1760e01b81526001600160a01b03808c166004830152909116602482015293849190829081906044820190565b03915afa8015610672576001925f91611d9a575b50611d2882886127d3565b52611d89611d6d611bd9611d4c8a60018060a01b03165f5260a260205260405f2090565b611d59611cb6868a6127d3565b60018060a01b03165f5260205260405f2090565b611d7783896127d3565b51611d8284876127d3565b51916141ee565b611d9382876127d3565b5201611c94565b611dbb915060203d8111611dc1575b611db38183610c1a565b810190612de8565b5f611d1d565b503d611da9565b5050506104d260405192839283611c23565b34610333575f366003190112610333576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033357604036600319011261033357600435611e3b81610371565b611e806024355f5492611e6660ff600886901c161580958196611efa575b8115611eda575b5061318b565b83611e77600160ff195f5416175f55565b611ec3576131ee565b611e8657005b611e9461ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b611ed561010061ff00195f5416175f55565b6131ee565b303b15915081611eec575b505f611e60565b60ff1660011490505f611ee5565b600160ff8216109150611e59565b34610333576020366003190112610333576104d2611fc1600435611f2b81610371565b6001600160a01b038082165f908152609a6020526040902054611f509116151561315f565b611f64611f5f61073183613014565b613175565b6001600160a01b038116611f798115156131ff565b6001600160a01b0382165f908152609a60205260409020611f99906104ae565b8133148015908161203f575b8015612002575b611fb590613215565b611fcd575b505061468b565b6040519182918261040e565b6001600160a01b0316907ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a5f80a35f80611fba565b50611fb561203661053060016120288660018060a01b03165f52609960205260405f2090565b01546001600160a01b031690565b33149050611fac565b5061204982613d11565b611fa5565b8015150361033357565b34610333576060366003190112610333576004356001600160401b0381116103335760e0600319823603011261033357602435906001600160401b038211610333576120ab6120eb9236906004016103de565b90611856604435936120bc8561204e565b6120cd610489600480606654161490565b6120dc600260c9541415612ae9565b600260c9553690600401610d4b565b600160c955005b346103335760803660031901126103335760043561210f81610371565b6024359061211c82610371565b604435612128816112ea565b606435612134816112ea565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036122d0576121d66121d06121de926121c86121a48861218f8960018060a01b03165f52609860205260405f2090565b9060018060a01b03165f5260205260405f2090565b546121c26001600160401b0388166001600160401b03851683614e3f565b90614218565b9487876144dd565b83612e1e565b918484614225565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeabf196001600160a01b0384160161220557005b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561033357604051633b9e9f0160e21b81526001600160a01b038516600482015260248101839052925f908490604490829084905af1928315610672577feff6aab2bc3f7c648896e1522eae71d6c22e3b0e218206b3f40af0e4d204716b936122bc575b50604080516001600160a01b039586168152602081019390935293169281908101610822565b806108355f6122ca93610c1a565b5f612296565b6323d871a560e01b5f5260045ffd5b346103335761232b6122f03661192d565b335f908152609a6020526040902054929392909190612318906001600160a01b031615612833565b612324610b4e85613014565b83336147ff565b61233c610489600180606654161490565b335f908152609a6020526040902080546001600160a01b0319166001600160a01b0383161790556001600160a01b038116337fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433045f80a361239b33612e2b565b91906123a88183336132c1565b915f5b8251811015610641576001906123ec6001600160a01b036123cc83876127d3565b51166123d883896127d3565b516123e384896127d3565b51913387613b80565b016123ab565b906020610ce1928181520190610f5c565b34610333576040366003190112610333576004356001600160401b03811161033357366023820112156103335780600401359061243f82610c59565b9161244d6040519384610c1a565b8083526024602084019160051b8301019136831161033357602401905b8282106124aa57836024356001600160401b038111610333576104d29161249861249e923690600401610cc6565b9061322b565b604051918291826123f2565b6020809183356124b981610371565b81520191019061246a565b34610333576020366003190112610333576004356124e181610371565b6124e96145eb565b6001600160a01b038116156125015761064190614643565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b34610333575f3660031901126103335760206103d6614937565b346103335760203660031901126103335760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610672575f916125f8575b506001600160a01b031633036125e9576106419061327e565b63794821ff60e01b5f5260045ffd5b90506020813d60201161262b575b8161261360209383610c1a565b81010312610333575161262581610371565b5f6125d0565b3d9150612606565b604080517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad602082019081526001600160a01b0395861692820192909252918416606083015292909116608082015260a081019290925260c0808301939093529181526126a160e082610c1a565b5190206126ac614937565b9060405190602082019261190160f01b845260228301526042820152604281526126d7606282610c1a565b51902090565b156126e457565b63840a48d560e01b5f5260045ffd5b906126fd82610c59565b61270a6040519182610c1a565b828152809261271b601f1991610c59565b0190602036910137565b634e487b7160e01b5f52603260045260245ffd5b91908110156115335760051b81013590605e1981360301821215610333570190565b903590601e198136030182121561033357018035906001600160401b03821161033357602001918160051b3603831361033357565b1561279757565b6343714afd60e01b5f5260045ffd5b35610ce181610371565b156127b757565b6330c4716960e21b5f5260045ffd5b8051156115335760200190565b80518210156115335760209160051b010190565b908160209103126103335751610ce18161204e565b6040513d5f823e3d90fd5b1561280e57565b631d77d47760e21b5f5260045ffd5b1561282457565b63c61dca5d60e01b5f5260045ffd5b1561283a57565b633bf2b50360e11b5f5260045ffd5b90918060409360208452816020850152848401375f828201840152601f01601f1916010190565b908160209103126103335751610ce1816112ea565b1561288c57565b63932d94f760e01b5f5260045ffd5b156128a257565b6325ec6c1f60e01b5f5260045ffd5b6040516126d7816128ce6020820194602086526040830190610eea565b03601f198101835282610c1a565b906128e682610c59565b6128f36040519182610c1a565b8281528092612904601f1991610c59565b01905f5b82811061291457505050565b60209060405161292381610bff565b5f81525f838201525f60408201525f60608201525f6080820152606060a0820152606060c082015282828501015201612908565b9061296182610c59565b61296e6040519182610c1a565b828152809261297f601f1991610c59565b01905f5b82811061298f57505050565b806060602080938501015201612983565b90604051918281549182825260208201905f5260205f20925f5b8181106129cf57505061038d92500383610c1a565b84546001600160a01b03168352600194850194879450602090930192016129ba565b90604051918281549182825260208201905f5260205f20925f5b818110612a2057505061038d92500383610c1a565b8454835260019485019487945060209093019201612a0b565b90604051612a4681610bff565b82546001600160a01b039081168252600184015416602082015291829060c090612ac29060069060028101546001600160a01b0316604086015260038101546060860152612aab612a9e600483015463ffffffff1690565b63ffffffff166080870152565b612ab7600582016129a0565b60a0860152016129f1565b910152565b60408051909190612ad88382610c1a565b6001815291601f1901366020840137565b15612af057565b60405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606490fd5b9082101561153357612b4c9160051b81019061275b565b9091565b91908110156115335760051b0190565b35610ce18161204e565b90604051602081018181106001600160401b03821117610bfa5760405291548252565b6001600160a01b038181165f908152609a60205260409020541615612ce4576001600160a01b0381165f908152609a60205260409020612bcc906104ae565b60405163152667d960e31b81526001600160a01b038216600482015273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac060248201529092602082806044810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156106725761038d95612cb9935f93612cbf575b50612cb390612c96611bd9612c758860018060a01b03165f5260a260205260405f2090565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac05f5260205260405f2090565b936001600160401b0380670de0b6b3a76400005b93169116614d75565b916141ee565b91614298565b612cb3919350612cdd9060203d6020116109a3576109958183610c1a565b9290612c50565b505050565b9080601f83011215610333578151612d0081610c59565b92612d0e6040519485610c1a565b81845260208085019260051b82010192831161033357602001905b828210612d365750505090565b8151815260209182019101612d29565b9190916040818403126103335780516001600160401b03811161033357810183601f8201121561033357805190612d7c82610c59565b91612d8a6040519384610c1a565b80835260208084019160051b8301019186831161033357602001905b828210612dce575050509260208201516001600160401b03811161033357610ce19201612ce9565b602080918351612ddd81610371565b815201910190612da6565b90816020910312610333575190565b634e487b7160e01b5f52601160045260245ffd5b9060018201809211612e1957565b612df7565b91908201809211612e1957565b6040516394f649dd60e01b81526001600160a01b038216600482015291905f83806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610672575f935f92612fe6575b5060405163fe243a1760e01b81526001600160a01b03909116600482015273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac060248201529060208280604481015b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610672575f92612fc5575b508115612fc057612f25612f208551612e0b565b6126f3565b93612f33612f208251612e0b565b92612f5b612f428351886127d3565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac09052565b612f668251856127d3565b525f5b8151811015612fba5780612f9e612f85611cb6600194866127d3565b612f8f838a6127d3565b6001600160a01b039091169052565b612fa881856127d3565b51612fb382876127d3565b5201612f69565b50505090565b919050565b612fdf91925060203d602011611dc157611db38183610c1a565b905f612f0c565b60209450612ed0925061300a903d805f833e6130028183610c1a565b810190612d46565b9490949250612e8e565b6001600160a01b0316801515908161302a575090565b5f818152609a60205260409020546001600160a01b031614919050565b602081830312610333578051906001600160401b03821161033357019080601f8301121561033357815161307a81610c59565b926130886040519485610c1a565b81845260208085019260051b82010192831161033357602001905b8282106130b05750505090565b6020809183516130bf816112ea565b8152019101906130a3565b6001600160a01b039091168152604060208201819052610ce192910190610e7b565b9190916130f983516126f3565b905f5b8451811015613158576001600160a01b038281165f908152609860205260409020600192916131469190613130848a6127d3565b511660018060a01b03165f5260205260405f2090565b5461315182866127d3565b52016130fc565b5090925050565b1561316657565b63a5c7c44560e01b5f5260045ffd5b1561317c57565b6311ca333560e31b5f5260045ffd5b1561319257565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6131fa61038d92613950565b614643565b1561320657565b6339b190bb60e11b5f5260045ffd5b1561321c57565b631e499a2360e11b5f5260045ffd5b906132368251612957565b915f5b8151811015612fba57600190613262846001600160a01b0361325b84876127d3565b51166130ec565b61326c82876127d3565b5261327781866127d3565b5001613239565b61328f60665419821981161461281d565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2565b92916132ef905f816132d381516126f3565b94604051948592839263547afb8760e01b8452600484016130ca565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610672575f92613377575b505f5b815181101561336f578061335e61334a611cb6600194866127d3565b6133576114ee84886127d3565b9089613aaf565b61336882876127d3565b520161332e565b509193505050565b61338c9192503d805f833e6115258183610c1a565b905f61332b565b1561339a57565b63796cc52560e01b5f5260045ffd5b156133b057565b63f020e5b960e01b5f5260045ffd5b6001600160a01b03918216815291166020820152604081019190915260600190565b5f198114612e195760010190565b916134089183549060031b91821b915f19901b19161790565b9055565b91909182821061341b57505050565b5f5260205f2091820191015b818110613432575050565b5f8155600101613427565b90600160401b8111610bfa57815481835561038d9261340c565b8151916001600160401b038311610bfa57602090613475848461343d565b01905f5260205f205f5b83811061348c5750505050565b82516001600160a01b03168183015560209092019160010161347f565b8151916001600160401b038311610bfa576020906134c7848461343d565b01905f5260205f205f5b8381106134de5750505050565b6001906020845194019381840155016134d1565b815181546001600160a01b039182166001600160a01b03199182161783556020840151600184018054918416918316919091179055604084015160028401805491909316911617905560608201516003820155608082015161038d9260069160c091906135789063ffffffff16600486019063ffffffff1663ffffffff19825416179055565b61358960a082015160058601613457565b015191016134a9565b916135af90610ce194928452606060208501526060840190610eea565b916040818403910152610eb7565b9294939091906135d76001600160a01b03851615156131ff565b6135e382511515613393565b6135ed82516126f3565b6135f783516126f3565b925f5b81518110156137f357613613611cc3611cb683856127d3565b90613641611bd96136348a60018060a01b03165f5260a260205260405f2090565b611d59611cb685886127d3565b9161364c828c6127d3565b516001600160a01b039091169261369c602061366b611cb686896127d3565b60405163fe243a1760e01b81526001600160a01b03808f166004830152909116602482015291829081906044820190565b0381885afa8015610672576136d3936136be925f926137d3575b5011156133a9565b6136c8838d6127d3565b51611d8284896127d3565b6136dd82886127d3565b526136fd6136eb82886127d3565b516136f683886127d3565b51906149f4565b61370782866127d3565b526001600160a01b038716613789575b613724611cb682856127d3565b61372e828c6127d3565b51833b156103335761375b935f92838c6040519788958694859363724af42360e01b8552600485016133bf565b03925af191821561067257600192613775575b50016135fa565b806108355f61378393610c1a565b5f61376e565b6137ab613799611cb683866127d3565b6137a383876127d3565b519089614a0a565b6137ce6137bb611cb683866127d3565b6137c583896127d3565b51908a8a614325565b613717565b6137ec91925060203d8111611dc157611db38183610c1a565b905f6136b6565b506001600160a01b0386165f908152609f602052604090208054979850613900977f26b2aae26516e8719ef50ea2f6831a2efbd4e37dccdf0f6936b27bc08e793e3097959695613905955093929161384a826133e1565b9055613874613857610c3b565b6001600160a01b0386168152966001600160a01b03166020880152565b6001600160a01b038416604087015260608601524363ffffffff16608086015260a085015260c08401526138a7836128b1565b9586916138cc6138bf845f52609e60205260405f2090565b805460ff19166001179055565b6138e7856138e2855f5260a460205260405f2090565b6134f2565b6001600160a01b03165f90815260a36020526040902090565b614e69565b506139166040519283928684613592565b0390a190565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b6001600160a01b039081165f8181526099602090815260409182902060010180546001600160a01b0319169590941694851790935551928352917f773b54c04d756fcc5e678111f7d730de3be98192000799eee3d63716055a87c69190a2565b9190916139f6610489600180606654161490565b6001600160a01b038181165f818152609a6020526040812080546001600160a01b03191693871693841790557fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433049080a3613a4f81612e2b565b9091613a5c8386836132c1565b925f5b8151811015613aa657600190613aa06001600160a01b03613a8083866127d3565b5116613a8c83886127d3565b51613a97848a6127d3565b5191878c613b80565b01613a5f565b50505050509050565b91906001600160a01b031673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014613ae2576001600160401b0391501690565b60405163a3d75e0960e01b81526001600160a01b039092166004830152602082806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561067257610ce1925f92613b5f575b506001600160401b0380670de0b6b3a7640000612caa565b613b7991925060203d6020116109a3576109958183610c1a565b905f613b47565b90938015613c94576001600160a01b038581165f90815260a2602090815260408083209387168352929052207f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f91613bf591613be791611bde91611bd99091895f84614ab2565b6040519182918689846133bf565b0390a16001600160a01b038085165f908152609a602052604090205416613c1d575b50505050565b6001600160a01b0381165f908152609860205260409020613c3f90839061218f565b805493808501809511612e19577f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c94613c88925560405193849360018060a01b031696846133bf565b0390a25f808080613c17565b630a33bc6960e21b5f5260045ffd5b919290948015613c9457613be7611bde7f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f93611bd9613bf59460018060a01b038b165f5260a260205289613d0a8a60405f209060018060a01b03165f5260205260405f2090565b9384614ab2565b604051631beb2b9760e31b81526001600160a01b0391821660048201523360248201523060448201525f80356001600160e01b03191660648301529091602091839160849183917f0000000000000000000000000000000000000000000000000000000000000000165af1908115610672575f91613d8d575090565b610ce1915060203d60201161066b5761065d8183610c1a565b15613dad57565b6316110d3560e21b5f5260045ffd5b15613dc357565b6387c9d21960e01b5f5260045ffd5b9063ffffffff8091169116019063ffffffff8211612e1957565b15613df357565b6378f67ae160e11b5f5260045ffd5b9190826040910312610333576020825192015190565b5f600661038d92828155826001820155826002820155826003820155826004820155613e4d836005830180549082815561340c565b0180549082815561340c565b93929360a0810192613e6e8451518214612790565b6040820151613e9090613e89906001600160a01b0316610530565b3314613da6565b613e99826128b1565b613ebd613eb8613eb1835f52609e60205260405f2090565b5460ff1690565b613dbc565b613f34613efb613ed4608086015163ffffffff1690565b7f000000000000000000000000000000000000000000000000000000000000000090613dd2565b613f1363ffffffff431663ffffffff83161115613dec565b84516001600160a01b031660208601516001600160a01b0316885191614b60565b83516001600160a01b03165f908152609a60205260409020909190613f58906104ae565b8451909190613f73906001600160a01b0316838951916132c1565b905f5b885180518210156141625790898989838f95613f99611cc3611cb6848f946127d3565b613fb68c61117385613faf8160c08a01516127d3565b51926127d3565b97156140695792516001600160a01b0393841693613ff393613fee9390929091613fe891611cb69185911699516127d3565b95612b50565b6127a6565b91813b1561033357604051630bab906360e21b81526001600160a01b039485166004820152908416602482015291909216604482015260648101939093525f908390608490829084905af191821561067257600192614055575b505b01613f76565b806108355f61406393610c1a565b5f61404d565b92613fee835f93613fe8611cb660409a999761408e614095975160018060a01b031690565b9a516127d3565b855163c4623ea160e01b81526001600160a01b0395861660048201529285166024840152841660448301526064820196909652948592608492849291165af18015610672578a61411a91600194848c5f925f9461411f575b505161410891611cb6916001600160a01b03165b95516127d3565b614112868a6127d3565b51938a613ca3565b61404f565b611cb691945061410193509161414e6141089360403d811161415b575b6141468183610c1a565b810190613e02565b94909495925050916140ed565b503d61413c565b50509550505050507f1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a009394506141e99250816141aa6138e76141af935160018060a01b031690565b614f0e565b506141ca6141c5825f5260a460205260405f2090565b613e18565b6115016141df825f52609e60205260405f2090565b805460ff19169055565b0390a1565b610ce192916141ff614205926148a7565b90614c41565b614c41565b5f19810191908211612e1957565b91908203918211612e1957565b60018060a01b031691825f5260986020526142538260405f209060018060a01b03165f5260205260405f2090565b918254828103908111612e19577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd93556142936040519283925f846133bf565b0390a2565b91909160018060a01b031691825f52609860205260405f2073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac05f5260205260405f2090815491838303928311612e19577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd9373beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0936142939255604051938493846133bf565b6001600160a01b039081165f818152609860209081526040808320948716835293905291909120805491948083039493928511612e19577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd946142939255604051938493846133bf565b6143ea9060018060a01b031691825f5260a56020526143c96143c48260405f209060018060a01b03165f5260205260405f2090565b614dac565b925f5260a560205260405f209060018060a01b03165f5260205260405f2090565b63ffffffff7f00000000000000000000000000000000000000000000000000000000000000001663ffffffff4316039063ffffffff8211612e195780549163ffffffff165f5b8381106144935750505f92610ce1949261445d92811586146144625750506001600160e01b0384166121c2565b614dd9565b614480614487916144756121c29461420a565b905f5260205f200190565b5460201c90565b6001600160e01b031690565b90928082169080831860011c8201809211612e1957835f528463ffffffff8360205f20015416115f146144c95750925b90614430565b93915060018101809111612e1957906144c3565b6001600160a01b039081165f81815260a56020908152604080832094861683529390529190912090949392916145379161451690614dac565b955f5260a560205260405f209060018060a01b03165f5260205260405f2090565b63ffffffff7f00000000000000000000000000000000000000000000000000000000000000001663ffffffff4316039063ffffffff8211612e195780549163ffffffff165f5b8381106145a15750509461445d91610ce1959681155f1461446257505f90506121c2565b90928082169080831860011c8201809211612e1957835f528463ffffffff8360205f20015416115f146145d75750925b9061457d565b93915060018101809111612e1957906145d1565b6033546001600160a01b031633036145ff57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b610ce1906146a0610489600280606654161490565b6001600160a01b0381165f908152609a6020526040902060609291906146c5906104ae565b906146f36146e38260018060a01b03165f52609a60205260405f2090565b80546001600160a01b0319169055565b6001600160a01b038281169082167ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af446765f80a361472e81612e2b565b91909485519081156147e15750614744906126f3565b926147508682846132c1565b915f5b87518110156147d7576001906147c68961476b612ac7565b614773612ac7565b9061479561478c611cb687614786612ac7565b966127d3565b612f8f836127c6565b61479f858b6127d3565b516147a9836127c6565b526147b4858a6127d3565b516147be846127c6565b5287876135bd565b6147d082896127d3565b5201614753565b5093955050505050565b955050505050565b156147f057565b630d4c4c9160e21b5f5260045ffd5b6001600160a01b038281165f9081526099602052604090206001015491949116929083156148a05761038d9461489691855f52609c60205260405f20815f5260205261485a61485560ff60405f20541615151590565b6147e9565b6148886138bf8261487b8960018060a01b03165f52609c60205260405f2090565b905f5260205260405f2090565b856020850195865193612633565b9051915192614e0a565b5050505050565b5180610ce15750670de0b6b3a764000090565b6001600160a01b031673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac00361490a577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b467f000000000000000000000000000000000000000000000000000000000000000003614982577f000000000000000000000000000000000000000000000000000000000000000090565b600a6020604051614994604082610c1a565b828152016922b4b3b2b72630bcb2b960b11b81522060405160208101917f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a86683526040820152466060820152306080820152608081526126d760a082610c1a565b908015614a0457610ce191614cbb565b50505f90565b90919073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeabf196001600160a01b03841601614a3757505050565b614a8d9060018060a01b031692835f5260a5602052614a6c6143c48260405f209060018060a01b03165f5260205260405f2090565b935f5260a560205260405f209060018060a01b03165f5260205260405f2090565b908201809211612e195761038d916001600160e01b0316904363ffffffff169061516f565b9290918215614afa57614ad482614205614ace611bde88612b6a565b86614c41565b90808201809211612e19578301809311612e195761340892614af591614cbb565b614cbb565b506134089150614d5b565b8054821015611533575f5260205f2001905f90565b90614b2491614b05565b90549060031b1c90565b91614b5963ffffffff9160409396959660018060a01b03168552606060208601526060850190610e7b565b9416910152565b939290915f81614b7081516126f3565b94614b8f6040519586938493632535f40360e21b855260048501614b2e565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610672575f92614bfb575b505f5b815181101561336f5780614bea61334a611cb6600194866127d3565b614bf482876127d3565b5201614bce565b614c109192503d805f833e6115258183610c1a565b905f614bcb565b634e487b7160e01b5f52601260045260245ffd5b8115614c35570490565b614c17565b1561033357565b5f1982820982820291828083109203918083039214614caa5781670de0b6b3a76400001115610333577faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066993670de0b6b3a7640000910990828211900360ee1b910360121c170290565b50670de0b6b3a76400009250500490565b5f19670de0b6b3a7640000820991670de0b6b3a7640000820291828085109403938085039414614d4f57670de0b6b3a76400008291614cfb868411614c3a565b09600182190182168092046002816003021880820260020302808202600203028082026002030280820260020302808202600203028091026002030293600183805f03040190848311900302920304170290565b5090610ce19250614c2b565b8015614c35576ec097ce7bc90715b34b9f10000000000490565b90915f198383099280830292838086109503948086039514614d9f57908291614cfb868411614c3a565b505090610ce19250614c2b565b80549081614dbb57505f919050565b815f19810111612e19575f525f199060205f2001015460201c614487565b916001600160401b03809116911603906001600160401b038211612e19576001600160401b03610ce1921690614c41565b924211614e3057614e1a92615046565b15614e2157565b638baa579f60e01b5f5260045ffd5b630819bdcd60e01b5f5260045ffd5b9190614e4c828285614d75565b928215614c355709614e5b5790565b60018101809111612e195790565b6001810190825f528160205260405f2054155f14614ecc578054600160401b811015610bfa57614eb9614ea3826001879401855584614b05565b819391549060031b91821b915f19901b19161790565b905554915f5260205260405f2055600190565b5050505f90565b80548015614efa575f190190614ee98282614b05565b8154905f199060031b1b1916905555565b634e487b7160e01b5f52603160045260245ffd5b6001810191805f528260205260405f2054928315155f14614f9f575f198401848111612e195783545f19810194908511612e19575f95858361487b94614f5c9803614f62575b505050614ed3565b55600190565b614f88614f8291614f79614b24614f969588614b05565b92839187614b05565b906133ef565b85905f5260205260405f2090565b555f8080614f54565b505050505f90565b60051115614fb157565b634e487b7160e01b5f52602160045260245ffd5b9060609260209183526040828401528051918291826040860152018484015e5f828201840152601f01601f1916010190565b3d15615021573d9061500882611912565b916150166040519384610c1a565b82523d5f602084013e565b606090565b9081602091031261033357516001600160e01b0319811681036103335790565b9190916150538284615248565b61505c81614fa7565b1590816150ed575b506150e5575f926128ce61509185946040519283916020830195630b135d3f60e11b875260248401614fc5565b51915afa61509d614ff7565b816150d9575b816150ac575090565b8051630b135d3f60e11b92506001600160e01b0319916150d491810160209081019101615026565b161490565b805160201491506150a3565b505050600190565b6001600160a01b0383811691161490505f615064565b1561510a57565b63151b8e3f60e11b5f5260045ffd5b8054600160401b811015610bfa5761513691600182018155614b05565b61515c57815160209283015190921b63ffffffff191663ffffffff909216919091179055565b634e487b7160e01b5f525f60045260245ffd5b8054806151ab575b506151a661038d9361519661518a610c4a565b63ffffffff9095168552565b6001600160e01b03166020840152565b615119565b805f19810111612e1957815f5263ffffffff6152196152105f198460205f2001016152066151f8604051926151df84610bdf565b548681169081855260201c602085015263ffffffff1690565b858916958691161115615103565b5163ffffffff1690565b63ffffffff1690565b036151775761038d939250906144756152319261420a565b9063ffffffff82549181199060201b169116179055565b815160418103615270575090612b4c91602082015190606060408401519301515f1a906152b2565b6040036152a95760406020830151920151918260ff1c91601b8301809311612e1957612b4c936001600160ff1b03169260ff16906152b2565b50505f90600290565b9291907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083116153505760ff16601b81141580615345575b61533a576020935f93604051938493608085019385528785015260408401526060830152838052039060015afa15610672575f516001600160a01b0381161561533257905f90565b505f90600190565b505050505f90600490565b50601c8114156152ea565b505050505f9060039056fea2646970667358221220cc6c4be32ce26ae0a9ccd93daa351221a5d87378ab0c9b1f66d6065d3836cf3c64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80\x80`@R4a\x02\x91W`\x1FaV;8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02\x95W\x80\x84\x92`\xC0\x94`@R\x839\x81\x01\x03\x12a\x02\x91W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x91W` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x91W`@\x83\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\x91W``\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x90\x84\x81\x03a\x02\x91W`\x80\x86\x01Q\x95`\x01`\x01`\xA0\x1B\x03\x87\x16\x87\x03a\x02\x91W`\xA0\x01Q\x94c\xFF\xFF\xFF\xFF\x86\x16\x86\x03a\x02\x91W\x15a\x02\x82W`\x80R`\xA0R`\xC0R`\xE0Ra\x01\0RFa\x01 Ra\x01`R_T`\x08\x81\x90\x1C`\xFF\x16a\x02-W`\xFF\x80\x82\x16\x10a\x01\xF3W[`@QaS\x91\x90\x81a\x02\xAA\x829`\x80Q\x81\x81\x81a\x05\xE7\x01R\x81\x81a\x0B\x83\x01R\x81\x81a\x16{\x01Ra%\x98\x01R`\xA0Q\x81\x81\x81a\x08V\x01R\x81\x81a\x08\xBE\x01R\x81\x81a\"\x07\x01R\x81\x81a.T\x01RaI\x0C\x01R`\xC0Q\x81\x81\x81a\t\xBD\x01R\x81\x81a\n\xD5\x01R\x81\x81a\x13)\x01R\x81\x81a.\xD4\x01R\x81\x81a;\x0B\x01RaH\xDF\x01R`\xE0Q\x81\x81\x81a\x077\x01R\x81\x81a\t5\x01R\x81\x81a\x14\xA2\x01R\x81\x81a\x1D\xEF\x01R\x81\x81a!6\x01R\x81\x81a,\x10\x01R\x81\x81a2\xF3\x01RaK\x93\x01Ra\x01\0Q\x81\x81\x81a\x1B\xFF\x01R\x81\x81a>\xD6\x01R\x81\x81aC\xF1\x01RaE>\x01Ra\x01 Q\x81aI:\x01Ra\x01@Q\x81aI`\x01Ra\x01`Q\x81\x81\x81a\n\x91\x01Ra=Z\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xF7V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xA4\xF9y\x14a\x03$W\x80c\x0B\x9FHz\x14a\x03\x1FW\x80c\r\xD8\xDD\x02\x14a\x03\x1AW\x80c\x13d9\xDD\x14a\x03\x15W\x80c*\xA6\xD8\x88\x14a\x03\x10W\x80c9\xB7\x0E8\x14a\x03\x0BW\x80c<e\x1C\xF2\x14a\x03\x06W\x80c<\xDE\xB5\xE0\x14a\x03\x01W\x80c>(9\x1D\x14a\x02\xFCW\x80cFW\xE2j\x14a\x02\xF7W\x80cFe\xBC\xDA\x14a\x02\xF2W\x80cT\xB7\xC9l\x14a\x02\xEDW\x80cY\\jg\x14a\x02\xE8W\x80cY{6\xDA\x14a\x02\xE3W\x80cZ\xC8j\xB7\x14a\x02\xDEW\x80c\\\x97Z\xBB\x14a\x02\xD9W\x80c]\xD6\x85y\x14a\x02\xD4W\x80c_H\xE6g\x14a\x02\xCFW\x80c`\xA0\xD1\xCE\x14a\x02\xCAW\x80ce\xDA\x12d\x14a\x02\xC5W\x80cf\xD5\xBA\x93\x14a\x02\xC0W\x80cmp\xF7\xAE\x14a\x02\xBBW\x80cn\x17DH\x14a\x02\xB6W\x80cqP\x18\xA6\x14a\x02\xB1W\x80cw\x8EU\xF3\x14a\x02\xACW\x80cx)n\xC5\x14a\x02\xA7W\x80c\x88o\x11\x95\x14a\x02\xA2W\x80c\x8D\xA5\xCB[\x14a\x02\x9DW\x80c\x90\x04\x13G\x14a\x02\x98W\x80c\x91\x04\xC3\x19\x14a\x02\x93W\x80c\x945\xBBC\x14a\x02\x8EW\x80c\x99\xF57\x1B\x14a\x02\x89W\x80c\xA1x\x84\x84\x14a\x02\x84W\x80c\xA3:43\x14a\x02\x7FW\x80c\xB7\xF0n\xBE\x14a\x02zW\x80c\xBBE\xFE\xF2\x14a\x02uW\x80c\xBF\xAE?\xD2\x14a\x02pW\x80c\xC4H\xFE\xB8\x14a\x02kW\x80c\xC9x\xF7\xAC\x14a\x02fW\x80c\xCA\x8A\xA7\xC7\x14a\x02aW\x80c\xCDm\xC6\x87\x14a\x02\\W\x80c\xDA\x8B\xE8d\x14a\x02WW\x80c\xE4\xCC?\x90\x14a\x02RW\x80c\xEEt\x93\x7F\x14a\x02MW\x80c\xEE\xA9\x06K\x14a\x02HW\x80c\xF0\xE0\xE6v\x14a\x02CW\x80c\xF2\xFD\xE3\x8B\x14a\x02>W\x80c\xF6\x98\xDA%\x14a\x029Wc\xFA\xBC\x1C\xBC\x14a\x024W_\x80\xFD[a%oV[a%UV[a$\xC4V[a$\x03V[a\"\xDFV[a \xF2V[a XV[a\x1F\x08V[a\x1E\x1EV[a\x1D\xDAV[a\x1CHV[a\x1B\xE3V[a\x1B\x98V[a\x1BJV[a\x1B\x1BV[a\x19\xE1V[a\x18\xD7V[a\x18aV[a\x17fV[a\x178V[a\x17\nV[a\x16\xAAV[a\x16fV[a\x15\xD7V[a\x15\x93V[a\x158V[a\x14NV[a\x14\x02V[a\x13\xB2V[a\x13oV[a\x12\xFBV[a\x11\xA6V[a\x10!V[a\x0E^V[a\x0E+V[a\r\xF1V[a\x0BXV[a\x0B\x04V[a\n\xC0V[a\n|V[a\n1V[a\t\xEBV[a\x08\x85V[a\x08AV[a\x06\xC8V[a\x05\xB7V[a\x04GV[a\x03\x8FV[a\x037V[_\x91\x03\x12a\x033WV[_\x80\xFD[4a\x033W_6`\x03\x19\x01\x12a\x033W` `@Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x033WV[5\x90a\x03\x8D\x82a\x03qV[V[4a\x033W`\xA06`\x03\x19\x01\x12a\x033W` a\x03\xD6`\x045a\x03\xB1\x81a\x03qV[`$5a\x03\xBD\x81a\x03qV[`D5a\x03\xC9\x81a\x03qV[`d5\x91`\x845\x93a&3V[`@Q\x90\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x033W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x033W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x033WV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x041WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x04$V[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x04w\x906\x90`\x04\x01a\x03\xDEV[\x90a\x04\x8Fa\x04\x89`\x02\x80`fT\x16\x14\x90V[\x15a&\xDDV[a\x04\x98\x82a&\xF3V[3_\x90\x81R`\x9A` R`@\x90 \x90\x92\x90a\x04\xBB\x90[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[_[\x82\x81\x10a\x04\xD6W`@Q\x80a\x04\xD2\x87\x82a\x04\x0EV[\x03\x90\xF3[\x80a\x05\x13a\x04\xF0a\x04\xEA`\x01\x94\x87\x89a'9V[\x80a'[V[\x90Pa\x05\na\x05\0\x84\x88\x8Aa'9V[` \x81\x01\x90a'[V[\x91\x90P\x14a'\x90V[a\x05B3a\x05<a\x050`@a\x05*\x86\x8A\x8Ca'9V[\x01a'\xA6V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x14a'\xB0V[a\x05\xA6a\x05ga\x05`a\x05Ya\x04\xEA\x85\x89\x8Ba'9V[6\x91a\x0CpV[\x853a2\xC1V[\x86a\x05\x9E\x87a\x05\x96a\x05\x8Ca\x05\0\x88a\x05\x84a\x04\xEA\x82\x87\x8Aa'9V[\x95\x90\x97a'9V[\x94\x90\x926\x91a\x0CpV[\x926\x91a\x0C\xE4V[\x90\x863a5\xBDV[a\x05\xB0\x82\x88a'\xD3V[R\x01a\x04\xBDV[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06rWa\x06A\x92a\x06-\x91_\x91a\x06CW[Pa(\x07V[a\x06<`fT\x82\x81\x16\x14a(\x1DV[a9PV[\0[a\x06e\x91P` =` \x11a\x06kW[a\x06]\x81\x83a\x0C\x1AV[\x81\x01\x90a'\xE7V[_a\x06'V[P=a\x06SV[a'\xFCV[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x033WV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x033WV[\x91\x81`\x1F\x84\x01\x12\x15a\x033W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x033W` \x83\x81\x86\x01\x95\x01\x01\x11a\x033WV[4a\x033W``6`\x03\x19\x01\x12a\x033W`\x045a\x06\xE5\x81a\x03qV[a\x06\xEDa\x06wV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x07\x0C\x906\x90`\x04\x01a\x06\x9BV[3_\x90\x81R`\x9A` R`@\x90 T\x91\x93\x91a\x075\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a(3V[\x15\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x033W`@Qc+bA\xF3`\xE1\x1B\x81R3`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16`$\x85\x01R_\x90\x84\x90`D\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x06rW\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x93a\x08\"\x93a\x08'W[Pa\x07\xD1\x813a9\x82V[a\x07\xDB33a9\xE2V[`@Q`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R3\x90\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90` \x90\xA2`@Q\x91\x82\x913\x95\x83a(IV[\x03\x90\xA2\0[\x80a\x085_a\x08;\x93a\x0C\x1AV[\x80a\x03)V[_a\x07\xC6V[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x033W`\x806`\x03\x19\x01\x12a\x033W`\x045a\x08\xA2\x81a\x03qV[`$5a\x08\xAE\x81a\x03qV[`d5`D5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x14\x80\x15a\t\xB9W[\x15a\t\xAAW`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x90\x81R`\x9A` \x90\x81R`@\x91\x82\x90 T\x91Qc\x15&g\xD9`\xE3\x1B\x81R\x91\x83\x16`\x04\x83\x01\x81\x90R\x86\x84\x16`$\x84\x01R\x91\x96\x91\x95\x92\x87\x90`D\x90\x82\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x95\x86\x15a\x06rWa\x06A\x96a\tu\x91_\x91a\t{W[P\x83\x83a:\xAFV[\x94a<\xA3V[a\t\x9D\x91P` =` \x11a\t\xA3W[a\t\x95\x81\x83a\x0C\x1AV[\x81\x01\x90a(pV[_a\tmV[P=a\t\x8BV[c\x04R\x06\xA5`\xE2\x1B_R`\x04_\xFD[P3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\xE7V[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045a\n\x08\x81a\x03qV[`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R` `\x01\x80`\xA0\x1B\x03`\x01`@_ \x01T\x16`@Q\x90\x81R\xF3[4a\x033W` 6`\x03\x19\x01\x12a\x033W` a\nr`\x045a\nS\x81a\x03qV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x033W`@6`\x03\x19\x01\x12a\x033Wa\x06A`\x045a\x0B$\x81a\x03qV[`$5\x90a\x0B1\x82a\x03qV[a\x0BBa\x0B=\x82a=\x11V[a(\x85V[a\x0BSa\x0BN\x82a0\x14V[a(\x9BV[a9\x82V[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x06rWa\x0B\xC3\x91_\x91a\x06CWPa(\x07V[a\x06Aa9\x1CV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0B\xFAW`@RV[a\x0B\xCBV[`\xE0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0B\xFAW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0B\xFAW`@RV[`@Q\x90a\x03\x8D`\xE0\x83a\x0C\x1AV[`@Q\x90a\x03\x8D`@\x83a\x0C\x1AV[`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xFAW`\x05\x1B` \x01\x90V[\x92\x91\x90a\x0C|\x81a\x0CYV[\x93a\x0C\x8A`@Q\x95\x86a\x0C\x1AV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x033W\x90[\x82\x82\x10a\x0C\xACWPPPV[` \x80\x91\x835a\x0C\xBB\x81a\x03qV[\x81R\x01\x91\x01\x90a\x0C\xA0V[\x90\x80`\x1F\x83\x01\x12\x15a\x033W\x81` a\x0C\xE1\x935\x91\x01a\x0CpV[\x90V[\x92\x91\x90a\x0C\xF0\x81a\x0CYV[\x93a\x0C\xFE`@Q\x95\x86a\x0C\x1AV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x033W\x90[\x82\x82\x10a\r WPPPV[\x815\x81R` \x91\x82\x01\x91\x01a\r\x14V[\x90\x80`\x1F\x83\x01\x12\x15a\x033W\x81` a\x0C\xE1\x935\x91\x01a\x0C\xE4V[\x91\x90\x91`\xE0\x81\x84\x03\x12a\x033Wa\r`a\x0C;V[\x92a\rj\x82a\x03\x82V[\x84Ra\rx` \x83\x01a\x03\x82V[` \x85\x01Ra\r\x89`@\x83\x01a\x03\x82V[`@\x85\x01R``\x82\x015``\x85\x01Ra\r\xA4`\x80\x83\x01a\x06\x8AV[`\x80\x85\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x033W\x81a\r\xC8\x91\x84\x01a\x0C\xC6V[`\xA0\x85\x01R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\r\xEA\x92\x01a\r0V[`\xC0\x83\x01RV[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x03\xD6a\x0E&` \x926\x90`\x04\x01a\rKV[a(\xB1V[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045`\xFF\x81\x16\x80\x91\x03a\x033W`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x033W_6`\x03\x19\x01\x12a\x033W` `fT`@Q\x90\x81R\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0E\x98WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0E\x8BV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0E\xD4WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0E\xC7V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01Ra\x0C\xE1\x91`\xC0a\x0FK`\xA0\x84\x01Q`\xE0`\xA0\x85\x01R`\xE0\x84\x01\x90a\x0E{V[\x92\x01Q\x90`\xC0\x81\x84\x03\x91\x01Ra\x0E\xB7V[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0F\x87WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0F\xA5`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x0E\xB7V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0FxV[\x92\x91`@\x84\x01\x93`@\x81R\x82Q\x80\x95R``\x81\x01\x94` ``\x82`\x05\x1B\x84\x01\x01\x94\x01\x90_[\x81\x81\x10a\x0F\xF6WPPPa\x0C\xE1\x93\x94P` \x81\x84\x03\x91\x01Ra\x0F\\V[\x90\x91\x94` \x80a\x10\x12`\x01\x93`_\x19\x88\x82\x03\x01\x8CR\x89Qa\x0E\xEAV[\x97\x01\x98\x01\x91\x01\x96\x91\x90\x96a\x0F\xD9V[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045a\x10>\x81a\x03qV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xA3` R`@\x90 a\x10^\x90a)\xF1V[\x90\x81Q\x91a\x10k\x83a(\xDCV[\x91a\x10u\x84a)WV[\x93a\x10\x93a\x04\xAE\x83`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x90_\x92[\x81\x84\x10a\x10\xADW`@Q\x80a\x04\xD2\x89\x89\x83a\x0F\xB4V[a\x10\xD3a\x10\xCEa\x10\xBF\x86\x88\x9A\x98a'\xD3V[Q_R`\xA4` R`@_ \x90V[a*9V[a\x10\xDD\x85\x88a'\xD3V[Ra\x10\xE8\x84\x87a'\xD3V[Pa\x11\x01`\xA0a\x10\xF8\x86\x89a'\xD3V[Q\x01QQa&\xF3V[a\x11\x0B\x85\x87a'\xD3V[Ra\x11\x16\x84\x86a'\xD3V[Pa\x110`\xA0a\x11&\x86\x89a'\xD3V[Q\x01Q\x84\x83a2\xC1V[\x92_[`\xA0a\x11?\x87\x8Aa'\xD3V[Q\x01QQ\x81\x10\x15a\x11\x95W\x80a\x11za\x11h`\x01\x93`\xC0a\x11`\x8B\x8Ea'\xD3V[Q\x01Qa'\xD3V[Qa\x11s\x89\x89a'\xD3V[Q\x90aLAV[a\x11\x8E\x82a\x11\x88\x8A\x8Ca'\xD3V[Qa'\xD3V[R\x01a\x113V[P\x94\x96\x94`\x01\x90\x94\x01\x93\x92Pa\x10\x97V[4a\x033W``6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x11\xD6\x906\x90`\x04\x01a\x03\xDEV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x11\xF5\x906\x90`\x04\x01a\x03\xDEV[\x91\x90`D5\x91a\x12\x0Ca\x04\x89`\x04\x80`fT\x16\x14\x90V[a\x12\x1B`\x02`\xC9T\x14\x15a*\xE9V[`\x02`\xC9U3_\x90\x81R`\xA3` R`@\x90 a\x12D\x90\x80T\x90\x94\x81\x81\x11\x15a\x12\xE3WPa(\xDCV[\x94_[\x86Q\x81\x10\x15a\x12\x8DW\x80a\x12qa\x10\xCEa\x12c`\x01\x94\x89aK\x1AV[_R`\xA4` R`@_ \x90V[a\x12{\x82\x8Aa'\xD3V[Ra\x12\x86\x81\x89a'\xD3V[P\x01a\x12GV[P\x85\x93P\x84_[\x85Q\x81\x10\x15a\x12\xD9W\x80a\x12\xD3a\x12\xAD`\x01\x93\x89a'\xD3V[Qa\x12\xB9\x83\x88\x88a+5V[\x90a\x12\xCDa\x12\xC8\x86\x89\x8Da+PV[a+`V[\x92a>YV[\x01a\x12\x94V[a\x06A`\x01`\xC9UV[\x90Pa(\xDCV[`\x01`\x01`@\x1B\x03\x81\x16\x03a\x033WV[4a\x033W``6`\x03\x19\x01\x12a\x033W`\x045a\x13\x18\x81a\x03qV[`D5`$5a\x13'\x82a\x12\xEAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x13`Wa\x06A\x92a+\x8DV[c2\x13\xA6a`\xE2\x1B_R`\x04_\xFD[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045a\x13\x8C\x81a\x03qV[`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x033W` 6`\x03\x19\x01\x12a\x033Wa\x13\xF4a\x04\xD2a\x13\xDD`\x045a\x13\xD8\x81a\x03qV[a.+V[`@\x92\x91\x92Q\x93\x84\x93`@\x85R`@\x85\x01\x90a\x0E{V[\x90\x83\x82\x03` \x85\x01Ra\x0E\xB7V[4a\x033W` 6`\x03\x19\x01\x12a\x033W` a\nr`\x045a\x14$\x81a\x03qV[a0\x14V[`@\x90`\x03\x19\x01\x12a\x033W`\x045a\x14A\x81a\x03qV[\x90`$5a\x0C\xE1\x81a\x03qV[4a\x033Wa\x14\\6a\x14)V[\x90a\x14ea*\xC7V[\x90\x81Q\x15a\x153W`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R`@QcTz\xFB\x87`\xE0\x1B\x81R\x91_\x90\x83\x90\x81\x90a\x14\x9E\x90\x85`\x04\x84\x01a0\xCAV[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\x06rWa\x14\xFBa\x14\xEEa\x04\xD2\x95a\x15\x01\x95_\x91a\x15\x11W[Pa'\xC6V[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91aC\x8FV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x15-\x91P=\x80_\x83>a\x15%\x81\x83a\x0C\x1AV[\x81\x01\x90a0GV[_a\x14\xE8V[a'%V[4a\x033W_6`\x03\x19\x01\x12a\x033Wa\x15PaE\xEBV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x033W` a\x15\xCEa\x15\xA66a\x14)V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x98\x85R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T`@Q\x90\x81R\xF3[4a\x033W`@6`\x03\x19\x01\x12a\x033W`\x045a\x15\xF4\x81a\x03qV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x164\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x916\x90`\x04\x01a\x06\x9BV[\x90\x92a\x16Ba\x0B=\x82a=\x11V[a\x16Na\x0BN\x82a0\x14V[a\x08\"`@Q\x92\x83\x92`\x01\x80`\xA0\x1B\x03\x16\x95\x83a(IV[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x033W_6`\x03\x19\x01\x12a\x033W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90`@`\x03\x19\x83\x01\x12a\x033W`\x045a\x16\xEB\x81a\x03qV[\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x033Wa\x0C\xE1\x91`\x04\x01a\x0C\xC6V[4a\x033Wa\x04\xD2a\x17$a\x17\x1E6a\x16\xD2V[\x90a0\xECV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x0E\xB7V[4a\x033W_6`\x03\x19\x01\x12a\x033W` `@Qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81R\xF3[4a\x033W``6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x17\x96\x906\x90`\x04\x01a\x03\xDEV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x17\xB5\x906\x90`\x04\x01a\x03\xDEV[\x90\x91`D5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x17\xD9\x90\x93\x91\x936\x90`\x04\x01a\x03\xDEV[\x90a\x17\xEBa\x04\x89`\x04\x80`fT\x16\x14\x90V[a\x17\xFA`\x02`\xC9T\x14\x15a*\xE9V[`\x02`\xC9U6\x86\x90\x03`\xDE\x19\x01\x92_[\x86\x81\x10\x15a\x12\xD9W\x80`\x05\x1B\x88\x015\x90\x85\x82\x12\x15a\x033Wa\x18[`\x01\x92a\x183\x83\x8A\x87a+5V[\x90a\x18V\x8Da\x18C\x87\x8C\x8Ca+PV[5\x94a\x18N\x86a NV[6\x91\x01a\rKV[a>YV[\x01a\x18\nV[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x04\x805_\x90\x81R`\xA4` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T\x93\x90\x96\x01T\x85Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x83\x16\x94\x82\x01\x94\x90\x94R\x94\x16\x92\x84\x01\x92\x90\x92R``\x83\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x90\xF3[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045a\x18\xF4\x81a\x03qV[`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xFAW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x90```\x03\x19\x83\x01\x12a\x033W`\x045a\x19F\x81a\x03qV[\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x033W`@\x82\x82\x03`\x03\x19\x01\x12a\x033W`@Q\x91a\x19t\x83a\x0B\xDFV[\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x033W\x81\x01\x91\x80`#\x84\x01\x12\x15a\x033W`\x04\x83\x015a\x19\xA3\x81a\x19\x12V[\x91a\x19\xB1`@Q\x93\x84a\x0C\x1AV[\x81\x83R`$\x85\x83\x01\x01\x11a\x033W` \x81_\x92`$\x80\x97\x01\x83\x86\x017\x83\x01\x01R\x83R\x015` \x82\x01R\x90`D5\x90V[4a\x033Wa\x1AHa\x19\xF26a\x19-V[3_\x90\x81R`\x9A` R`@\x90 T\x92\x93\x91\x92a\x1A\x19\x90`\x01`\x01`\xA0\x1B\x03\x16\x15\x15a1_V[a\x1A+a\x1A%3a0\x14V[\x15a1uV[a\x1A7a\x0BN\x85a0\x14V[a\x1A@3aF\x8BV[\x92\x843aG\xFFV[a\x1AYa\x04\x89`\x01\x80`fT\x16\x14\x90V[3_\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x163\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04_\x80\xA3a\x1A\xB83a.+V[a\x1A\xC3\x82\x853a2\xC1V[\x91_[\x81Q\x81\x10\x15a\x1B\rW`\x01\x90a\x1B\x07`\x01`\x01`\xA0\x1B\x03a\x1A\xE7\x83\x86a'\xD3V[Q\x16a\x1A\xF3\x83\x87a'\xD3V[Qa\x1A\xFE\x84\x89a'\xD3V[Q\x913\x8Ba;\x80V[\x01a\x1A\xC6V[`@Q\x80a\x04\xD2\x87\x82a\x04\x0EV[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045_R`\x9E` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x033W`@6`\x03\x19\x01\x12a\x033W`\x045a\x1Bg\x81a\x03qV[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x033W` a\x03\xD6a\x1B\xDEa\x1B\xD9a\x1B\xB16a\x14)V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\xA2\x87R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[a+jV[aH\xA7V[4a\x033W_6`\x03\x19\x01\x12a\x033W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[\x90\x91a\x1C:a\x0C\xE1\x93`@\x84R`@\x84\x01\x90a\x0E\xB7V[\x91` \x81\x84\x03\x91\x01Ra\x0E\xB7V[4a\x033Wa\x1CV6a\x16\xD2V[a\x1C`\x81Qa&\xF3V[a\x1Cj\x82Qa&\xF3V[\x91a\x1C\x92\x81a\x1C\x8Ca\x04\xAE\x87`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x86a2\xC1V[_[\x82Q\x81\x10\x15a\x1D\xC8W\x80` a\x1C\xC8a\x050a\x1C\xC3a\x1C\xB6a\x1D\t\x96\x89a'\xD3V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[aH\xBAV[a\x1C\xD5a\x1C\xB6\x84\x88a'\xD3V[`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x8C\x16`\x04\x83\x01R\x90\x91\x16`$\x82\x01R\x93\x84\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x80\x15a\x06rW`\x01\x92_\x91a\x1D\x9AW[Pa\x1D(\x82\x88a'\xD3V[Ra\x1D\x89a\x1Dma\x1B\xD9a\x1DL\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[a\x1DYa\x1C\xB6\x86\x8Aa'\xD3V[`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a\x1Dw\x83\x89a'\xD3V[Qa\x1D\x82\x84\x87a'\xD3V[Q\x91aA\xEEV[a\x1D\x93\x82\x87a'\xD3V[R\x01a\x1C\x94V[a\x1D\xBB\x91P` =\x81\x11a\x1D\xC1W[a\x1D\xB3\x81\x83a\x0C\x1AV[\x81\x01\x90a-\xE8V[_a\x1D\x1DV[P=a\x1D\xA9V[PPPa\x04\xD2`@Q\x92\x83\x92\x83a\x1C#V[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x033W`@6`\x03\x19\x01\x12a\x033W`\x045a\x1E;\x81a\x03qV[a\x1E\x80`$5_T\x92a\x1Ef`\xFF`\x08\x86\x90\x1C\x16\x15\x80\x95\x81\x96a\x1E\xFAW[\x81\x15a\x1E\xDAW[Pa1\x8BV[\x83a\x1Ew`\x01`\xFF\x19_T\x16\x17_UV[a\x1E\xC3Wa1\xEEV[a\x1E\x86W\0[a\x1E\x94a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x1E\xD5a\x01\0a\xFF\0\x19_T\x16\x17_UV[a1\xEEV[0;\x15\x91P\x81a\x1E\xECW[P_a\x1E`V[`\xFF\x16`\x01\x14\x90P_a\x1E\xE5V[`\x01`\xFF\x82\x16\x10\x91Pa\x1EYV[4a\x033W` 6`\x03\x19\x01\x12a\x033Wa\x04\xD2a\x1F\xC1`\x045a\x1F+\x81a\x03qV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x9A` R`@\x90 Ta\x1FP\x91\x16\x15\x15a1_V[a\x1Fda\x1F_a\x071\x83a0\x14V[a1uV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1Fy\x81\x15\x15a1\xFFV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9A` R`@\x90 a\x1F\x99\x90a\x04\xAEV[\x813\x14\x80\x15\x90\x81a ?W[\x80\x15a \x02W[a\x1F\xB5\x90a2\x15V[a\x1F\xCDW[PPaF\x8BV[`@Q\x91\x82\x91\x82a\x04\x0EV[`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A_\x80\xA3_\x80a\x1F\xBAV[Pa\x1F\xB5a 6a\x050`\x01a (\x86`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[3\x14\x90Pa\x1F\xACV[Pa I\x82a=\x11V[a\x1F\xA5V[\x80\x15\x15\x03a\x033WV[4a\x033W``6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033W`\xE0`\x03\x19\x826\x03\x01\x12a\x033W`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x033Wa \xABa \xEB\x926\x90`\x04\x01a\x03\xDEV[\x90a\x18V`D5\x93a \xBC\x85a NV[a \xCDa\x04\x89`\x04\x80`fT\x16\x14\x90V[a \xDC`\x02`\xC9T\x14\x15a*\xE9V[`\x02`\xC9U6\x90`\x04\x01a\rKV[`\x01`\xC9U\0[4a\x033W`\x806`\x03\x19\x01\x12a\x033W`\x045a!\x0F\x81a\x03qV[`$5\x90a!\x1C\x82a\x03qV[`D5a!(\x81a\x12\xEAV[`d5a!4\x81a\x12\xEAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\"\xD0Wa!\xD6a!\xD0a!\xDE\x92a!\xC8a!\xA4\x88a!\x8F\x89`\x01\x80`\xA0\x1B\x03\x16_R`\x98` R`@_ \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta!\xC2`\x01`\x01`@\x1B\x03\x88\x16`\x01`\x01`@\x1B\x03\x85\x16\x83aN?V[\x90aB\x18V[\x94\x87\x87aD\xDDV[\x83a.\x1EV[\x91\x84\x84aB%V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xBF\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01a\"\x05W\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x033W`@Qc;\x9E\x9F\x01`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x92_\x90\x84\x90`D\x90\x82\x90\x84\x90Z\xF1\x92\x83\x15a\x06rW\x7F\xEF\xF6\xAA\xB2\xBC?|d\x88\x96\xE1R.\xAEq\xD6\xC2.;\x0E!\x82\x06\xB3\xF4\n\xF0\xE4\xD2\x04qk\x93a\"\xBCW[P`@\x80Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R` \x81\x01\x93\x90\x93R\x93\x16\x92\x81\x90\x81\x01a\x08\"V[\x80a\x085_a\"\xCA\x93a\x0C\x1AV[_a\"\x96V[c#\xD8q\xA5`\xE0\x1B_R`\x04_\xFD[4a\x033Wa#+a\"\xF06a\x19-V[3_\x90\x81R`\x9A` R`@\x90 T\x92\x93\x92\x90\x91\x90a#\x18\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a(3V[a#$a\x0BN\x85a0\x14V[\x833aG\xFFV[a#<a\x04\x89`\x01\x80`fT\x16\x14\x90V[3_\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x81\x163\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04_\x80\xA3a#\x9B3a.+V[\x91\x90a#\xA8\x81\x833a2\xC1V[\x91_[\x82Q\x81\x10\x15a\x06AW`\x01\x90a#\xEC`\x01`\x01`\xA0\x1B\x03a#\xCC\x83\x87a'\xD3V[Q\x16a#\xD8\x83\x89a'\xD3V[Qa#\xE3\x84\x89a'\xD3V[Q\x913\x87a;\x80V[\x01a#\xABV[\x90` a\x0C\xE1\x92\x81\x81R\x01\x90a\x0F\\V[4a\x033W`@6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033W6`#\x82\x01\x12\x15a\x033W\x80`\x04\x015\x90a$?\x82a\x0CYV[\x91a$M`@Q\x93\x84a\x0C\x1AV[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x033W`$\x01\x90[\x82\x82\x10a$\xAAW\x83`$5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x04\xD2\x91a$\x98a$\x9E\x926\x90`\x04\x01a\x0C\xC6V[\x90a2+V[`@Q\x91\x82\x91\x82a#\xF2V[` \x80\x91\x835a$\xB9\x81a\x03qV[\x81R\x01\x91\x01\x90a$jV[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045a$\xE1\x81a\x03qV[a$\xE9aE\xEBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a%\x01Wa\x06A\x90aFCV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x033W_6`\x03\x19\x01\x12a\x033W` a\x03\xD6aI7V[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06rW_\x91a%\xF8W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a%\xE9Wa\x06A\x90a2~V[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a&+W[\x81a&\x13` \x93\x83a\x0C\x1AV[\x81\x01\x03\x12a\x033WQa&%\x81a\x03qV[_a%\xD0V[=\x91Pa&\x06V[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x92\x82\x01\x92\x90\x92R\x91\x84\x16``\x83\x01R\x92\x90\x91\x16`\x80\x82\x01R`\xA0\x81\x01\x92\x90\x92R`\xC0\x80\x83\x01\x93\x90\x93R\x91\x81Ra&\xA1`\xE0\x82a\x0C\x1AV[Q\x90 a&\xACaI7V[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra&\xD7`b\x82a\x0C\x1AV[Q\x90 \x90V[\x15a&\xE4WV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x90a&\xFD\x82a\x0CYV[a'\n`@Q\x91\x82a\x0C\x1AV[\x82\x81R\x80\x92a'\x1B`\x1F\x19\x91a\x0CYV[\x01\x90` 6\x91\x017V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x81\x10\x15a\x153W`\x05\x1B\x81\x015\x90`^\x19\x816\x03\x01\x82\x12\x15a\x033W\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x033W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x033W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x033WV[\x15a'\x97WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[5a\x0C\xE1\x81a\x03qV[\x15a'\xB7WV[c0\xC4qi`\xE2\x1B_R`\x04_\xFD[\x80Q\x15a\x153W` \x01\x90V[\x80Q\x82\x10\x15a\x153W` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x033WQa\x0C\xE1\x81a NV[`@Q=_\x82>=\x90\xFD[\x15a(\x0EWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a($WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[\x15a(:WV[c;\xF2\xB5\x03`\xE1\x1B_R`\x04_\xFD[\x90\x91\x80`@\x93` \x84R\x81` \x85\x01R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x033WQa\x0C\xE1\x81a\x12\xEAV[\x15a(\x8CWV[c\x93-\x94\xF7`\xE0\x1B_R`\x04_\xFD[\x15a(\xA2WV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[`@Qa&\xD7\x81a(\xCE` \x82\x01\x94` \x86R`@\x83\x01\x90a\x0E\xEAV[\x03`\x1F\x19\x81\x01\x83R\x82a\x0C\x1AV[\x90a(\xE6\x82a\x0CYV[a(\xF3`@Q\x91\x82a\x0C\x1AV[\x82\x81R\x80\x92a)\x04`\x1F\x19\x91a\x0CYV[\x01\x90_[\x82\x81\x10a)\x14WPPPV[` \x90`@Qa)#\x81a\x0B\xFFV[_\x81R_\x83\x82\x01R_`@\x82\x01R_``\x82\x01R_`\x80\x82\x01R```\xA0\x82\x01R```\xC0\x82\x01R\x82\x82\x85\x01\x01R\x01a)\x08V[\x90a)a\x82a\x0CYV[a)n`@Q\x91\x82a\x0C\x1AV[\x82\x81R\x80\x92a)\x7F`\x1F\x19\x91a\x0CYV[\x01\x90_[\x82\x81\x10a)\x8FWPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a)\x83V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_[\x81\x81\x10a)\xCFWPPa\x03\x8D\x92P\x03\x83a\x0C\x1AV[\x84T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01a)\xBAV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_[\x81\x81\x10a* WPPa\x03\x8D\x92P\x03\x83a\x0C\x1AV[\x84T\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01a*\x0BV[\x90`@Qa*F\x81a\x0B\xFFV[\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x84\x01T\x16` \x82\x01R\x91\x82\x90`\xC0\x90a*\xC2\x90`\x06\x90`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16`@\x86\x01R`\x03\x81\x01T``\x86\x01Ra*\xABa*\x9E`\x04\x83\x01Tc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16`\x80\x87\x01RV[a*\xB7`\x05\x82\x01a)\xA0V[`\xA0\x86\x01R\x01a)\xF1V[\x91\x01RV[`@\x80Q\x90\x91\x90a*\xD8\x83\x82a\x0C\x1AV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x15a*\xF0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x90\xFD[\x90\x82\x10\x15a\x153Wa+L\x91`\x05\x1B\x81\x01\x90a'[V[\x90\x91V[\x91\x90\x81\x10\x15a\x153W`\x05\x1B\x01\x90V[5a\x0C\xE1\x81a NV[\x90`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0B\xFAW`@R\x91T\x82RV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15a,\xE4W`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9A` R`@\x90 a+\xCC\x90a\x04\xAEV[`@Qc\x15&g\xD9`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x82\x01R\x90\x92` \x82\x80`D\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06rWa\x03\x8D\x95a,\xB9\x93_\x93a,\xBFW[Pa,\xB3\x90a,\x96a\x1B\xD9a,u\x88`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0_R` R`@_ \x90V[\x93`\x01`\x01`@\x1B\x03\x80g\r\xE0\xB6\xB3\xA7d\0\0[\x93\x16\x91\x16aMuV[\x91aA\xEEV[\x91aB\x98V[a,\xB3\x91\x93Pa,\xDD\x90` =` \x11a\t\xA3Wa\t\x95\x81\x83a\x0C\x1AV[\x92\x90a,PV[PPPV[\x90\x80`\x1F\x83\x01\x12\x15a\x033W\x81Qa-\0\x81a\x0CYV[\x92a-\x0E`@Q\x94\x85a\x0C\x1AV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x033W` \x01\x90[\x82\x82\x10a-6WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a-)V[\x91\x90\x91`@\x81\x84\x03\x12a\x033W\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x033W\x81\x01\x83`\x1F\x82\x01\x12\x15a\x033W\x80Q\x90a-|\x82a\x0CYV[\x91a-\x8A`@Q\x93\x84a\x0C\x1AV[\x80\x83R` \x80\x84\x01\x91`\x05\x1B\x83\x01\x01\x91\x86\x83\x11a\x033W` \x01\x90[\x82\x82\x10a-\xCEWPPP\x92` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x0C\xE1\x92\x01a,\xE9V[` \x80\x91\x83Qa-\xDD\x81a\x03qV[\x81R\x01\x91\x01\x90a-\xA6V[\x90\x81` \x91\x03\x12a\x033WQ\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a.\x19WV[a-\xF7V[\x91\x90\x82\x01\x80\x92\x11a.\x19WV[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R\x91\x90_\x83\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06rW_\x93_\x92a/\xE6W[P`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x82\x01R\x90` \x82\x80`D\x81\x01[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06rW_\x92a/\xC5W[P\x81\x15a/\xC0Wa/%a/ \x85Qa.\x0BV[a&\xF3V[\x93a/3a/ \x82Qa.\x0BV[\x92a/[a/B\x83Q\x88a'\xD3V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x90RV[a/f\x82Q\x85a'\xD3V[R_[\x81Q\x81\x10\x15a/\xBAW\x80a/\x9Ea/\x85a\x1C\xB6`\x01\x94\x86a'\xD3V[a/\x8F\x83\x8Aa'\xD3V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a/\xA8\x81\x85a'\xD3V[Qa/\xB3\x82\x87a'\xD3V[R\x01a/iV[PPP\x90V[\x91\x90PV[a/\xDF\x91\x92P` =` \x11a\x1D\xC1Wa\x1D\xB3\x81\x83a\x0C\x1AV[\x90_a/\x0CV[` \x94Pa.\xD0\x92Pa0\n\x90=\x80_\x83>a0\x02\x81\x83a\x0C\x1AV[\x81\x01\x90a-FV[\x94\x90\x94\x92Pa.\x8EV[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x15\x90\x81a0*WP\x90V[_\x81\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14\x91\x90PV[` \x81\x83\x03\x12a\x033W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x033W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x033W\x81Qa0z\x81a\x0CYV[\x92a0\x88`@Q\x94\x85a\x0C\x1AV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x033W` \x01\x90[\x82\x82\x10a0\xB0WPPP\x90V[` \x80\x91\x83Qa0\xBF\x81a\x12\xEAV[\x81R\x01\x91\x01\x90a0\xA3V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra\x0C\xE1\x92\x91\x01\x90a\x0E{V[\x91\x90\x91a0\xF9\x83Qa&\xF3V[\x90_[\x84Q\x81\x10\x15a1XW`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x98` R`@\x90 `\x01\x92\x91a1F\x91\x90a10\x84\x8Aa'\xD3V[Q\x16`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta1Q\x82\x86a'\xD3V[R\x01a0\xFCV[P\x90\x92PPV[\x15a1fWV[c\xA5\xC7\xC4E`\xE0\x1B_R`\x04_\xFD[\x15a1|WV[c\x11\xCA35`\xE3\x1B_R`\x04_\xFD[\x15a1\x92WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a1\xFAa\x03\x8D\x92a9PV[aFCV[\x15a2\x06WV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[\x15a2\x1CWV[c\x1EI\x9A#`\xE1\x1B_R`\x04_\xFD[\x90a26\x82Qa)WV[\x91_[\x81Q\x81\x10\x15a/\xBAW`\x01\x90a2b\x84`\x01`\x01`\xA0\x1B\x03a2[\x84\x87a'\xD3V[Q\x16a0\xECV[a2l\x82\x87a'\xD3V[Ra2w\x81\x86a'\xD3V[P\x01a29V[a2\x8F`fT\x19\x82\x19\x81\x16\x14a(\x1DV[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2V[\x92\x91a2\xEF\x90_\x81a2\xD3\x81Qa&\xF3V[\x94`@Q\x94\x85\x92\x83\x92cTz\xFB\x87`\xE0\x1B\x84R`\x04\x84\x01a0\xCAV[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06rW_\x92a3wW[P_[\x81Q\x81\x10\x15a3oW\x80a3^a3Ja\x1C\xB6`\x01\x94\x86a'\xD3V[a3Wa\x14\xEE\x84\x88a'\xD3V[\x90\x89a:\xAFV[a3h\x82\x87a'\xD3V[R\x01a3.V[P\x91\x93PPPV[a3\x8C\x91\x92P=\x80_\x83>a\x15%\x81\x83a\x0C\x1AV[\x90_a3+V[\x15a3\x9AWV[cyl\xC5%`\xE0\x1B_R`\x04_\xFD[\x15a3\xB0WV[c\xF0 \xE5\xB9`\xE0\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x91\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_\x19\x81\x14a.\x19W`\x01\x01\x90V[\x91a4\x08\x91\x83T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90UV[\x91\x90\x91\x82\x82\x10a4\x1BWPPPV[_R` _ \x91\x82\x01\x91\x01[\x81\x81\x10a42WPPV[_\x81U`\x01\x01a4'V[\x90`\x01`@\x1B\x81\x11a\x0B\xFAW\x81T\x81\x83Ua\x03\x8D\x92a4\x0CV[\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x0B\xFAW` \x90a4u\x84\x84a4=V[\x01\x90_R` _ _[\x83\x81\x10a4\x8CWPPPPV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x83\x01U` \x90\x92\x01\x91`\x01\x01a4\x7FV[\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x0B\xFAW` \x90a4\xC7\x84\x84a4=V[\x01\x90_R` _ _[\x83\x81\x10a4\xDEWPPPPV[`\x01\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a4\xD1V[\x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U` \x84\x01Q`\x01\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\x02\x84\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U``\x82\x01Q`\x03\x82\x01U`\x80\x82\x01Qa\x03\x8D\x92`\x06\x91`\xC0\x91\x90a5x\x90c\xFF\xFF\xFF\xFF\x16`\x04\x86\x01\x90c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[a5\x89`\xA0\x82\x01Q`\x05\x86\x01a4WV[\x01Q\x91\x01a4\xA9V[\x91a5\xAF\x90a\x0C\xE1\x94\x92\x84R``` \x85\x01R``\x84\x01\x90a\x0E\xEAV[\x91`@\x81\x84\x03\x91\x01Ra\x0E\xB7V[\x92\x94\x93\x90\x91\x90a5\xD7`\x01`\x01`\xA0\x1B\x03\x85\x16\x15\x15a1\xFFV[a5\xE3\x82Q\x15\x15a3\x93V[a5\xED\x82Qa&\xF3V[a5\xF7\x83Qa&\xF3V[\x92_[\x81Q\x81\x10\x15a7\xF3Wa6\x13a\x1C\xC3a\x1C\xB6\x83\x85a'\xD3V[\x90a6Aa\x1B\xD9a64\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[a\x1DYa\x1C\xB6\x85\x88a'\xD3V[\x91a6L\x82\x8Ca'\xD3V[Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92a6\x9C` a6ka\x1C\xB6\x86\x89a'\xD3V[`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x8F\x16`\x04\x83\x01R\x90\x91\x16`$\x82\x01R\x91\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x81\x88Z\xFA\x80\x15a\x06rWa6\xD3\x93a6\xBE\x92_\x92a7\xD3W[P\x11\x15a3\xA9V[a6\xC8\x83\x8Da'\xD3V[Qa\x1D\x82\x84\x89a'\xD3V[a6\xDD\x82\x88a'\xD3V[Ra6\xFDa6\xEB\x82\x88a'\xD3V[Qa6\xF6\x83\x88a'\xD3V[Q\x90aI\xF4V[a7\x07\x82\x86a'\xD3V[R`\x01`\x01`\xA0\x1B\x03\x87\x16a7\x89W[a7$a\x1C\xB6\x82\x85a'\xD3V[a7.\x82\x8Ca'\xD3V[Q\x83;\x15a\x033Wa7[\x93_\x92\x83\x8C`@Q\x97\x88\x95\x86\x94\x85\x93crJ\xF4#`\xE0\x1B\x85R`\x04\x85\x01a3\xBFV[\x03\x92Z\xF1\x91\x82\x15a\x06rW`\x01\x92a7uW[P\x01a5\xFAV[\x80a\x085_a7\x83\x93a\x0C\x1AV[_a7nV[a7\xABa7\x99a\x1C\xB6\x83\x86a'\xD3V[a7\xA3\x83\x87a'\xD3V[Q\x90\x89aJ\nV[a7\xCEa7\xBBa\x1C\xB6\x83\x86a'\xD3V[a7\xC5\x83\x89a'\xD3V[Q\x90\x8A\x8AaC%V[a7\x17V[a7\xEC\x91\x92P` =\x81\x11a\x1D\xC1Wa\x1D\xB3\x81\x83a\x0C\x1AV[\x90_a6\xB6V[P`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x9F` R`@\x90 \x80T\x97\x98Pa9\0\x97\x7F&\xB2\xAA\xE2e\x16\xE8q\x9E\xF5\x0E\xA2\xF6\x83\x1A.\xFB\xD4\xE3}\xCC\xDF\x0Fi6\xB2{\xC0\x8Ey>0\x97\x95\x96\x95a9\x05\x95P\x93\x92\x91a8J\x82a3\xE1V[\x90Ua8ta8Wa\x0C;V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R\x96`\x01`\x01`\xA0\x1B\x03\x16` \x88\x01RV[`\x01`\x01`\xA0\x1B\x03\x84\x16`@\x87\x01R``\x86\x01RCc\xFF\xFF\xFF\xFF\x16`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01Ra8\xA7\x83a(\xB1V[\x95\x86\x91a8\xCCa8\xBF\x84_R`\x9E` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[a8\xE7\x85a8\xE2\x85_R`\xA4` R`@_ \x90V[a4\xF2V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\xA3` R`@\x90 \x90V[aNiV[Pa9\x16`@Q\x92\x83\x92\x86\x84a5\x92V[\x03\x90\xA1\x90V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x99` \x90\x81R`@\x91\x82\x90 `\x01\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x95\x90\x94\x16\x94\x85\x17\x90\x93UQ\x92\x83R\x91\x7Fw;T\xC0Muo\xCC^g\x81\x11\xF7\xD70\xDE;\xE9\x81\x92\0\x07\x99\xEE\xE3\xD67\x16\x05Z\x87\xC6\x91\x90\xA2V[\x91\x90\x91a9\xF6a\x04\x89`\x01\x80`fT\x16\x14\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x9A` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x87\x16\x93\x84\x17\x90U\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x90\x80\xA3a:O\x81a.+V[\x90\x91a:\\\x83\x86\x83a2\xC1V[\x92_[\x81Q\x81\x10\x15a:\xA6W`\x01\x90a:\xA0`\x01`\x01`\xA0\x1B\x03a:\x80\x83\x86a'\xD3V[Q\x16a:\x8C\x83\x88a'\xD3V[Qa:\x97\x84\x8Aa'\xD3V[Q\x91\x87\x8Ca;\x80V[\x01a:_V[PPPPP\x90PV[\x91\x90`\x01`\x01`\xA0\x1B\x03\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a:\xE2W`\x01`\x01`@\x1B\x03\x91P\x16\x90V[`@Qc\xA3\xD7^\t`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R` \x82\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06rWa\x0C\xE1\x92_\x92a;_W[P`\x01`\x01`@\x1B\x03\x80g\r\xE0\xB6\xB3\xA7d\0\0a,\xAAV[a;y\x91\x92P` =` \x11a\t\xA3Wa\t\x95\x81\x83a\x0C\x1AV[\x90_a;GV[\x90\x93\x80\x15a<\x94W`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x91a;\xF5\x91a;\xE7\x91a\x1B\xDE\x91a\x1B\xD9\x90\x91\x89_\x84aJ\xB2V[`@Q\x91\x82\x91\x86\x89\x84a3\xBFV[\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x9A` R`@\x90 T\x16a<\x1DW[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x98` R`@\x90 a<?\x90\x83\x90a!\x8FV[\x80T\x93\x80\x85\x01\x80\x95\x11a.\x19W\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x94a<\x88\x92U`@Q\x93\x84\x93`\x01\x80`\xA0\x1B\x03\x16\x96\x84a3\xBFV[\x03\x90\xA2_\x80\x80\x80a<\x17V[c\n3\xBCi`\xE2\x1B_R`\x04_\xFD[\x91\x92\x90\x94\x80\x15a<\x94Wa;\xE7a\x1B\xDE\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x93a\x1B\xD9a;\xF5\x94`\x01\x80`\xA0\x1B\x03\x8B\x16_R`\xA2` R\x89a=\n\x8A`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x93\x84aJ\xB2V[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R3`$\x82\x01R0`D\x82\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x90\x91` \x91\x83\x91`\x84\x91\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xF1\x90\x81\x15a\x06rW_\x91a=\x8DWP\x90V[a\x0C\xE1\x91P` =` \x11a\x06kWa\x06]\x81\x83a\x0C\x1AV[\x15a=\xADWV[c\x16\x11\r5`\xE2\x1B_R`\x04_\xFD[\x15a=\xC3WV[c\x87\xC9\xD2\x19`\xE0\x1B_R`\x04_\xFD[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a.\x19WV[\x15a=\xF3WV[cx\xF6z\xE1`\xE1\x1B_R`\x04_\xFD[\x91\x90\x82`@\x91\x03\x12a\x033W` \x82Q\x92\x01Q\x90V[_`\x06a\x03\x8D\x92\x82\x81U\x82`\x01\x82\x01U\x82`\x02\x82\x01U\x82`\x03\x82\x01U\x82`\x04\x82\x01Ua>M\x83`\x05\x83\x01\x80T\x90\x82\x81Ua4\x0CV[\x01\x80T\x90\x82\x81Ua4\x0CV[\x93\x92\x93`\xA0\x81\x01\x92a>n\x84QQ\x82\x14a'\x90V[`@\x82\x01Qa>\x90\x90a>\x89\x90`\x01`\x01`\xA0\x1B\x03\x16a\x050V[3\x14a=\xA6V[a>\x99\x82a(\xB1V[a>\xBDa>\xB8a>\xB1\x83_R`\x9E` R`@_ \x90V[T`\xFF\x16\x90V[a=\xBCV[a?4a>\xFBa>\xD4`\x80\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a=\xD2V[a?\x13c\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x83\x16\x11\x15a=\xECV[\x84Q`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x88Q\x91aK`V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9A` R`@\x90 \x90\x91\x90a?X\x90a\x04\xAEV[\x84Q\x90\x91\x90a?s\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x89Q\x91a2\xC1V[\x90_[\x88Q\x80Q\x82\x10\x15aAbW\x90\x89\x89\x89\x83\x8F\x95a?\x99a\x1C\xC3a\x1C\xB6\x84\x8F\x94a'\xD3V[a?\xB6\x8Ca\x11s\x85a?\xAF\x81`\xC0\x8A\x01Qa'\xD3V[Q\x92a'\xD3V[\x97\x15a@iW\x92Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93a?\xF3\x93a?\xEE\x93\x90\x92\x90\x91a?\xE8\x91a\x1C\xB6\x91\x85\x91\x16\x99Qa'\xD3V[\x95a+PV[a'\xA6V[\x91\x81;\x15a\x033W`@Qc\x0B\xAB\x90c`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x90\x84\x16`$\x82\x01R\x91\x90\x92\x16`D\x82\x01R`d\x81\x01\x93\x90\x93R_\x90\x83\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x06rW`\x01\x92a@UW[P[\x01a?vV[\x80a\x085_a@c\x93a\x0C\x1AV[_a@MV[\x92a?\xEE\x83_\x93a?\xE8a\x1C\xB6`@\x9A\x99\x97a@\x8Ea@\x95\x97Q`\x01\x80`\xA0\x1B\x03\x16\x90V[\x9AQa'\xD3V[\x85Qc\xC4b>\xA1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x04\x82\x01R\x92\x85\x16`$\x84\x01R\x84\x16`D\x83\x01R`d\x82\x01\x96\x90\x96R\x94\x85\x92`\x84\x92\x84\x92\x91\x16Z\xF1\x80\x15a\x06rW\x8AaA\x1A\x91`\x01\x94\x84\x8C_\x92_\x94aA\x1FW[PQaA\x08\x91a\x1C\xB6\x91`\x01`\x01`\xA0\x1B\x03\x16[\x95Qa'\xD3V[aA\x12\x86\x8Aa'\xD3V[Q\x93\x8Aa<\xA3V[a@OV[a\x1C\xB6\x91\x94PaA\x01\x93P\x91aANaA\x08\x93`@=\x81\x11aA[W[aAF\x81\x83a\x0C\x1AV[\x81\x01\x90a>\x02V[\x94\x90\x94\x95\x92PP\x91a@\xEDV[P=aA<V[PP\x95PPPPP\x7F\x1F@@\x08\x89'N\xD0{$\x84^PT\xA8z\x0C\xAB\x96\x9E\xB1'z\xAF\xE6\x1A\xE3R\xE7\xC3*\0\x93\x94PaA\xE9\x92P\x81aA\xAAa8\xE7aA\xAF\x93Q`\x01\x80`\xA0\x1B\x03\x16\x90V[aO\x0EV[PaA\xCAaA\xC5\x82_R`\xA4` R`@_ \x90V[a>\x18V[a\x15\x01aA\xDF\x82_R`\x9E` R`@_ \x90V[\x80T`\xFF\x19\x16\x90UV[\x03\x90\xA1V[a\x0C\xE1\x92\x91aA\xFFaB\x05\x92aH\xA7V[\x90aLAV[aLAV[_\x19\x81\x01\x91\x90\x82\x11a.\x19WV[\x91\x90\x82\x03\x91\x82\x11a.\x19WV[`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\x98` RaBS\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x91\x82T\x82\x81\x03\x90\x81\x11a.\x19W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x93UaB\x93`@Q\x92\x83\x92_\x84a3\xBFV[\x03\x90\xA2V[\x91\x90\x91`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\x98` R`@_ s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0_R` R`@_ \x90\x81T\x91\x83\x83\x03\x92\x83\x11a.\x19W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x93s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x93aB\x93\x92U`@Q\x93\x84\x93\x84a3\xBFV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x98` \x90\x81R`@\x80\x83 \x94\x87\x16\x83R\x93\x90R\x91\x90\x91 \x80T\x91\x94\x80\x83\x03\x94\x93\x92\x85\x11a.\x19W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x94aB\x93\x92U`@Q\x93\x84\x93\x84a3\xBFV[aC\xEA\x90`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\xA5` RaC\xC9aC\xC4\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[aM\xACV[\x92_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xFF\xFF\xFF\xFFC\x16\x03\x90c\xFF\xFF\xFF\xFF\x82\x11a.\x19W\x80T\x91c\xFF\xFF\xFF\xFF\x16_[\x83\x81\x10aD\x93WPP_\x92a\x0C\xE1\x94\x92aD]\x92\x81\x15\x86\x14aDbWPP`\x01`\x01`\xE0\x1B\x03\x84\x16a!\xC2V[aM\xD9V[aD\x80aD\x87\x91aDua!\xC2\x94aB\nV[\x90_R` _ \x01\x90V[T` \x1C\x90V[`\x01`\x01`\xE0\x1B\x03\x16\x90V[\x90\x92\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a.\x19W\x83_R\x84c\xFF\xFF\xFF\xFF\x83` _ \x01T\x16\x11_\x14aD\xC9WP\x92[\x90aD0V[\x93\x91P`\x01\x81\x01\x80\x91\x11a.\x19W\x90aD\xC3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\xA5` \x90\x81R`@\x80\x83 \x94\x86\x16\x83R\x93\x90R\x91\x90\x91 \x90\x94\x93\x92\x91aE7\x91aE\x16\x90aM\xACV[\x95_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xFF\xFF\xFF\xFFC\x16\x03\x90c\xFF\xFF\xFF\xFF\x82\x11a.\x19W\x80T\x91c\xFF\xFF\xFF\xFF\x16_[\x83\x81\x10aE\xA1WPP\x94aD]\x91a\x0C\xE1\x95\x96\x81\x15_\x14aDbWP_\x90Pa!\xC2V[\x90\x92\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a.\x19W\x83_R\x84c\xFF\xFF\xFF\xFF\x83` _ \x01T\x16\x11_\x14aE\xD7WP\x92[\x90aE}V[\x93\x91P`\x01\x81\x01\x80\x91\x11a.\x19W\x90aE\xD1V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aE\xFFWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\x0C\xE1\x90aF\xA0a\x04\x89`\x02\x80`fT\x16\x14\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9A` R`@\x90 ``\x92\x91\x90aF\xC5\x90a\x04\xAEV[\x90aF\xF3aF\xE3\x82`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x90\x82\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv_\x80\xA3aG.\x81a.+V[\x91\x90\x94\x85Q\x90\x81\x15aG\xE1WPaGD\x90a&\xF3V[\x92aGP\x86\x82\x84a2\xC1V[\x91_[\x87Q\x81\x10\x15aG\xD7W`\x01\x90aG\xC6\x89aGka*\xC7V[aGsa*\xC7V[\x90aG\x95aG\x8Ca\x1C\xB6\x87aG\x86a*\xC7V[\x96a'\xD3V[a/\x8F\x83a'\xC6V[aG\x9F\x85\x8Ba'\xD3V[QaG\xA9\x83a'\xC6V[RaG\xB4\x85\x8Aa'\xD3V[QaG\xBE\x84a'\xC6V[R\x87\x87a5\xBDV[aG\xD0\x82\x89a'\xD3V[R\x01aGSV[P\x93\x95PPPPPV[\x95PPPPPV[\x15aG\xF0WV[c\rLL\x91`\xE2\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x91\x94\x91\x16\x92\x90\x83\x15aH\xA0Wa\x03\x8D\x94aH\x96\x91\x85_R`\x9C` R`@_ \x81_R` RaHZaHU`\xFF`@_ T\x16\x15\x15\x15\x90V[aG\xE9V[aH\x88a8\xBF\x82aH{\x89`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R`@_ \x90V[\x90_R` R`@_ \x90V[\x85` \x85\x01\x95\x86Q\x93a&3V[\x90Q\x91Q\x92aN\nV[PPPPPV[Q\x80a\x0C\xE1WPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[`\x01`\x01`\xA0\x1B\x03\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x03aI\nW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03aI\x82W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`\n` `@QaI\x94`@\x82a\x0C\x1AV[\x82\x81R\x01i\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B\x81R `@Q` \x81\x01\x91\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x83R`@\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\x80\x81Ra&\xD7`\xA0\x82a\x0C\x1AV[\x90\x80\x15aJ\x04Wa\x0C\xE1\x91aL\xBBV[PP_\x90V[\x90\x91\x90s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xBF\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01aJ7WPPPV[aJ\x8D\x90`\x01\x80`\xA0\x1B\x03\x16\x92\x83_R`\xA5` RaJlaC\xC4\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x93_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x90\x82\x01\x80\x92\x11a.\x19Wa\x03\x8D\x91`\x01`\x01`\xE0\x1B\x03\x16\x90Cc\xFF\xFF\xFF\xFF\x16\x90aQoV[\x92\x90\x91\x82\x15aJ\xFAWaJ\xD4\x82aB\x05aJ\xCEa\x1B\xDE\x88a+jV[\x86aLAV[\x90\x80\x82\x01\x80\x92\x11a.\x19W\x83\x01\x80\x93\x11a.\x19Wa4\x08\x92aJ\xF5\x91aL\xBBV[aL\xBBV[Pa4\x08\x91PaM[V[\x80T\x82\x10\x15a\x153W_R` _ \x01\x90_\x90V[\x90aK$\x91aK\x05V[\x90T\x90`\x03\x1B\x1C\x90V[\x91aKYc\xFF\xFF\xFF\xFF\x91`@\x93\x96\x95\x96`\x01\x80`\xA0\x1B\x03\x16\x85R``` \x86\x01R``\x85\x01\x90a\x0E{V[\x94\x16\x91\x01RV[\x93\x92\x90\x91_\x81aKp\x81Qa&\xF3V[\x94aK\x8F`@Q\x95\x86\x93\x84\x93c%5\xF4\x03`\xE2\x1B\x85R`\x04\x85\x01aK.V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06rW_\x92aK\xFBW[P_[\x81Q\x81\x10\x15a3oW\x80aK\xEAa3Ja\x1C\xB6`\x01\x94\x86a'\xD3V[aK\xF4\x82\x87a'\xD3V[R\x01aK\xCEV[aL\x10\x91\x92P=\x80_\x83>a\x15%\x81\x83a\x0C\x1AV[\x90_aK\xCBV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x15aL5W\x04\x90V[aL\x17V[\x15a\x033WV[_\x19\x82\x82\t\x82\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14aL\xAAW\x81g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x033W\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x93g\r\xE0\xB6\xB3\xA7d\0\0\x91\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x92PP\x04\x90V[_\x19g\r\xE0\xB6\xB3\xA7d\0\0\x82\t\x91g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x82\x80\x85\x10\x94\x03\x93\x80\x85\x03\x94\x14aMOWg\r\xE0\xB6\xB3\xA7d\0\0\x82\x91aL\xFB\x86\x84\x11aL:V[\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x81`\x03\x02\x18\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x91\x02`\x02\x03\x02\x93`\x01\x83\x80_\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[P\x90a\x0C\xE1\x92PaL+V[\x80\x15aL5Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[\x90\x91_\x19\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14aM\x9FW\x90\x82\x91aL\xFB\x86\x84\x11aL:V[PP\x90a\x0C\xE1\x92PaL+V[\x80T\x90\x81aM\xBBWP_\x91\x90PV[\x81_\x19\x81\x01\x11a.\x19W_R_\x19\x90` _ \x01\x01T` \x1CaD\x87V[\x91`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01`@\x1B\x03\x82\x11a.\x19W`\x01`\x01`@\x1B\x03a\x0C\xE1\x92\x16\x90aLAV[\x92B\x11aN0WaN\x1A\x92aPFV[\x15aN!WV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[c\x08\x19\xBD\xCD`\xE0\x1B_R`\x04_\xFD[\x91\x90aNL\x82\x82\x85aMuV[\x92\x82\x15aL5W\taN[W\x90V[`\x01\x81\x01\x80\x91\x11a.\x19W\x90V[`\x01\x81\x01\x90\x82_R\x81` R`@_ T\x15_\x14aN\xCCW\x80T`\x01`@\x1B\x81\x10\x15a\x0B\xFAWaN\xB9aN\xA3\x82`\x01\x87\x94\x01\x85U\x84aK\x05V[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90UT\x91_R` R`@_ U`\x01\x90V[PPP_\x90V[\x80T\x80\x15aN\xFAW_\x19\x01\x90aN\xE9\x82\x82aK\x05V[\x81T\x90_\x19\x90`\x03\x1B\x1B\x19\x16\x90UUV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`\x01\x81\x01\x91\x80_R\x82` R`@_ T\x92\x83\x15\x15_\x14aO\x9FW_\x19\x84\x01\x84\x81\x11a.\x19W\x83T_\x19\x81\x01\x94\x90\x85\x11a.\x19W_\x95\x85\x83aH{\x94aO\\\x98\x03aObW[PPPaN\xD3V[U`\x01\x90V[aO\x88aO\x82\x91aOyaK$aO\x96\x95\x88aK\x05V[\x92\x83\x91\x87aK\x05V[\x90a3\xEFV[\x85\x90_R` R`@_ \x90V[U_\x80\x80aOTV[PPPP_\x90V[`\x05\x11\x15aO\xB1WV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90``\x92` \x91\x83R`@\x82\x84\x01R\x80Q\x91\x82\x91\x82`@\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[=\x15aP!W=\x90aP\x08\x82a\x19\x12V[\x91aP\x16`@Q\x93\x84a\x0C\x1AV[\x82R=_` \x84\x01>V[``\x90V[\x90\x81` \x91\x03\x12a\x033WQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x033W\x90V[\x91\x90\x91aPS\x82\x84aRHV[aP\\\x81aO\xA7V[\x15\x90\x81aP\xEDW[PaP\xE5W_\x92a(\xCEaP\x91\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01aO\xC5V[Q\x91Z\xFAaP\x9DaO\xF7V[\x81aP\xD9W[\x81aP\xACWP\x90V[\x80Qc\x0B\x13]?`\xE1\x1B\x92P`\x01`\x01`\xE0\x1B\x03\x19\x91aP\xD4\x91\x81\x01` \x90\x81\x01\x91\x01aP&V[\x16\x14\x90V[\x80Q` \x14\x91PaP\xA3V[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_aPdV[\x15aQ\nWV[c\x15\x1B\x8E?`\xE1\x1B_R`\x04_\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x0B\xFAWaQ6\x91`\x01\x82\x01\x81UaK\x05V[aQ\\W\x81Q` \x92\x83\x01Q\x90\x92\x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80T\x80aQ\xABW[PaQ\xA6a\x03\x8D\x93aQ\x96aQ\x8Aa\x0CJV[c\xFF\xFF\xFF\xFF\x90\x95\x16\x85RV[`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01RV[aQ\x19V[\x80_\x19\x81\x01\x11a.\x19W\x81_Rc\xFF\xFF\xFF\xFFaR\x19aR\x10_\x19\x84` _ \x01\x01aR\x06aQ\xF8`@Q\x92aQ\xDF\x84a\x0B\xDFV[T\x86\x81\x16\x90\x81\x85R` \x1C` \x85\x01Rc\xFF\xFF\xFF\xFF\x16\x90V[\x85\x89\x16\x95\x86\x91\x16\x11\x15aQ\x03V[Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[\x03aQwWa\x03\x8D\x93\x92P\x90aDuaR1\x92aB\nV[\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV[\x81Q`A\x81\x03aRpWP\x90a+L\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aR\xB2V[`@\x03aR\xA9W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a.\x19Wa+L\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90aR\xB2V[PP_\x90`\x02\x90V[\x92\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11aSPW`\xFF\x16`\x1B\x81\x14\x15\x80aSEW[aS:W` \x93_\x93`@Q\x93\x84\x93`\x80\x85\x01\x93\x85R\x87\x85\x01R`@\x84\x01R``\x83\x01R\x83\x80R\x03\x90`\x01Z\xFA\x15a\x06rW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aS2W\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x81\x14\x15aR\xEAV[PPPP_\x90`\x03\x90V\xFE\xA2dipfsX\"\x12 \xCClK\xE3,\xE2j\xE0\xA9\xCC\xD9=\xAA5\x12!\xA5\xD8sx\xAB\x0C\x9B\x1Ff\xD6\x06]86\xCF<dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c806304a4f979146103245780630b9f487a1461031f5780630dd8dd021461031a578063136439dd146103155780632aa6d8881461031057806339b70e381461030b5780633c651cf2146103065780633cdeb5e0146103015780633e28391d146102fc5780634657e26a146102f75780634665bcda146102f257806354b7c96c146102ed578063595c6a67146102e8578063597b36da146102e35780635ac86ab7146102de5780635c975abb146102d95780635dd68579146102d45780635f48e667146102cf57806360a0d1ce146102ca57806365da1264146102c557806366d5ba93146102c05780636d70f7ae146102bb5780636e174448146102b6578063715018a6146102b1578063778e55f3146102ac57806378296ec5146102a7578063886f1195146102a25780638da5cb5b1461029d57806390041347146102985780639104c319146102935780639435bb431461028e57806399f5371b14610289578063a178848414610284578063a33a34331461027f578063b7f06ebe1461027a578063bb45fef214610275578063bfae3fd214610270578063c448feb81461026b578063c978f7ac14610266578063ca8aa7c714610261578063cd6dc6871461025c578063da8be86414610257578063e4cc3f9014610252578063ee74937f1461024d578063eea9064b14610248578063f0e0e67614610243578063f2fde38b1461023e578063f698da25146102395763fabc1cbc14610234575f80fd5b61256f565b612555565b6124c4565b612403565b6122df565b6120f2565b612058565b611f08565b611e1e565b611dda565b611c48565b611be3565b611b98565b611b4a565b611b1b565b6119e1565b6118d7565b611861565b611766565b611738565b61170a565b6116aa565b611666565b6115d7565b611593565b611538565b61144e565b611402565b6113b2565b61136f565b6112fb565b6111a6565b611021565b610e5e565b610e2b565b610df1565b610b58565b610b04565b610ac0565b610a7c565b610a31565b6109eb565b610885565b610841565b6106c8565b6105b7565b610447565b61038f565b610337565b5f91031261033357565b5f80fd5b34610333575f3660031901126103335760206040517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad8152f35b6001600160a01b0381160361033357565b359061038d82610371565b565b346103335760a03660031901126103335760206103d66004356103b181610371565b6024356103bd81610371565b6044356103c981610371565b6064359160843593612633565b604051908152f35b9181601f84011215610333578235916001600160401b038311610333576020808501948460051b01011161033357565b60206040818301928281528451809452019201905f5b8181106104315750505090565b8251845260209384019390920191600101610424565b34610333576020366003190112610333576004356001600160401b038111610333576104779036906004016103de565b9061048f610489600280606654161490565b156126dd565b610498826126f3565b335f908152609a602052604090209092906104bb905b546001600160a01b031690565b5f5b8281106104d657604051806104d2878261040e565b0390f35b806105136104f06104ea6001948789612739565b8061275b565b905061050a61050084888a612739565b602081019061275b565b91905014612790565b6105423361053c610530604061052a868a8c612739565b016127a6565b6001600160a01b031690565b146127b0565b6105a66105676105606105596104ea85898b612739565b3691610c70565b85336132c1565b8661059e8761059661058c610500886105846104ea82878a612739565b959097612739565b9490923691610c70565b923691610ce4565b9086336135bd565b6105b082886127d3565b52016104bd565b346103335760203660031901126103335760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610672576106419261062d915f91610643575b50612807565b61063c6066548281161461281d565b613950565b005b610665915060203d60201161066b575b61065d8183610c1a565b8101906127e7565b5f610627565b503d610653565b6127fc565b6024359063ffffffff8216820361033357565b359063ffffffff8216820361033357565b9181601f84011215610333578235916001600160401b038311610333576020838186019501011161033357565b34610333576060366003190112610333576004356106e581610371565b6106ed610677565b6044356001600160401b0381116103335761070c90369060040161069b565b335f908152609a6020526040902054919391610735906001600160a01b031615612833565b1590565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561033357604051632b6241f360e11b815233600482015263ffffffff9490941660248501525f908490604490829084905af1918215610672577f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080909361082293610827575b506107d18133613982565b6107db33336139e2565b6040516001600160a01b0391909116815233907fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c190602090a2604051918291339583612849565b0390a2005b806108355f61083b93610c1a565b80610329565b5f6107c6565b34610333575f366003190112610333576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610333576080366003190112610333576004356108a281610371565b6024356108ae81610371565b6064356044356001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016331480156109b9575b156109aa576001600160a01b038481165f908152609a602090815260409182902054915163152667d960e31b81529183166004830181905286841660248401529196919592879060449082907f0000000000000000000000000000000000000000000000000000000000000000165afa9586156106725761064196610975915f9161097b575b508383613aaf565b94613ca3565b61099d915060203d6020116109a3575b6109958183610c1a565b810190612870565b5f61096d565b503d61098b565b63045206a560e21b5f5260045ffd5b50337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316146108e7565b3461033357602036600319011261033357600435610a0881610371565b60018060a01b03165f526099602052602060018060a01b03600160405f20015416604051908152f35b34610333576020366003190112610333576020610a72600435610a5381610371565b6001600160a01b039081165f908152609a602052604090205416151590565b6040519015158152f35b34610333575f366003190112610333576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610333575f366003190112610333576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033357604036600319011261033357610641600435610b2481610371565b60243590610b3182610371565b610b42610b3d82613d11565b612885565b610b53610b4e82613014565b61289b565b613982565b34610333575f3660031901126103335760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561067257610bc3915f916106435750612807565b61064161391c565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b03821117610bfa57604052565b610bcb565b60e081019081106001600160401b03821117610bfa57604052565b90601f801991011681019081106001600160401b03821117610bfa57604052565b6040519061038d60e083610c1a565b6040519061038d604083610c1a565b6001600160401b038111610bfa5760051b60200190565b929190610c7c81610c59565b93610c8a6040519586610c1a565b602085838152019160051b810192831161033357905b828210610cac57505050565b602080918335610cbb81610371565b815201910190610ca0565b9080601f8301121561033357816020610ce193359101610c70565b90565b929190610cf081610c59565b93610cfe6040519586610c1a565b602085838152019160051b810192831161033357905b828210610d2057505050565b8135815260209182019101610d14565b9080601f8301121561033357816020610ce193359101610ce4565b91909160e08184031261033357610d60610c3b565b92610d6a82610382565b8452610d7860208301610382565b6020850152610d8960408301610382565b604085015260608201356060850152610da46080830161068a565b608085015260a08201356001600160401b0381116103335781610dc8918401610cc6565b60a085015260c08201356001600160401b03811161033357610dea9201610d30565b60c0830152565b34610333576020366003190112610333576004356001600160401b038111610333576103d6610e266020923690600401610d4b565b6128b1565b346103335760203660031901126103335760043560ff81168091036103335760016020911b806066541614604051908152f35b34610333575f366003190112610333576020606654604051908152f35b90602080835192838152019201905f5b818110610e985750505090565b82516001600160a01b0316845260209384019390920191600101610e8b565b90602080835192838152019201905f5b818110610ed45750505090565b8251845260209384019390920191600101610ec7565b80516001600160a01b039081168352602080830151821690840152604080830151909116908301526060808201519083015260808082015163ffffffff1690830152610ce19160c0610f4b60a084015160e060a085015260e0840190610e7b565b9201519060c0818403910152610eb7565b9080602083519182815201916020808360051b8301019401925f915b838310610f8757505050505090565b9091929394602080610fa5600193601f198682030187528951610eb7565b97019301930191939290610f78565b929160408401936040815282518095526060810194602060608260051b8401019401905f5b818110610ff657505050610ce19394506020818403910152610f5c565b909194602080611012600193605f19888203018c528951610eea565b97019801910196919096610fd9565b346103335760203660031901126103335760043561103e81610371565b6001600160a01b0381165f90815260a36020526040902061105e906129f1565b9081519161106b836128dc565b9161107584612957565b936110936104ae8360018060a01b03165f52609a60205260405f2090565b905f925b8184106110ad57604051806104d2898983610fb4565b6110d36110ce6110bf86889a986127d3565b515f5260a460205260405f2090565b612a39565b6110dd85886127d3565b526110e884876127d3565b5061110160a06110f886896127d3565b510151516126f3565b61110b85876127d3565b5261111684866127d3565b5061113060a061112686896127d3565b51015184836132c1565b925f5b60a061113f878a6127d3565b51015151811015611195578061117a61116860019360c06111608b8e6127d3565b5101516127d3565b5161117389896127d3565b5190614c41565b61118e826111888a8c6127d3565b516127d3565b5201611133565b509496946001909401939250611097565b34610333576060366003190112610333576004356001600160401b038111610333576111d69036906004016103de565b6024356001600160401b038111610333576111f59036906004016103de565b91906044359161120c610489600480606654161490565b61121b600260c9541415612ae9565b600260c955335f90815260a3602052604090206112449080549094818111156112e357506128dc565b945f5b865181101561128d57806112716110ce61126360019489614b1a565b5f5260a460205260405f2090565b61127b828a6127d3565b5261128681896127d3565b5001611247565b50859350845f5b85518110156112d957806112d36112ad600193896127d3565b516112b9838888612b35565b906112cd6112c886898d612b50565b612b60565b92613e59565b01611294565b610641600160c955565b90506128dc565b6001600160401b0381160361033357565b346103335760603660031901126103335760043561131881610371565b604435602435611327826112ea565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036113605761064192612b8d565b633213a66160e21b5f5260045ffd5b346103335760203660031901126103335760043561138c81610371565b60018060a01b03165f52609a602052602060018060a01b0360405f205416604051908152f35b34610333576020366003190112610333576113f46104d26113dd6004356113d881610371565b612e2b565b604092919251938493604085526040850190610e7b565b908382036020850152610eb7565b34610333576020366003190112610333576020610a7260043561142481610371565b613014565b60409060031901126103335760043561144181610371565b90602435610ce181610371565b346103335761145c36611429565b90611465612ac7565b90815115611533576001600160a01b038316602083015260405163547afb8760e01b8152915f908390819061149e9085600484016130ca565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa928315610672576114fb6114ee6104d295611501955f91611511575b506127c6565b516001600160401b031690565b9161438f565b6040519081529081906020820190565b61152d91503d805f833e6115258183610c1a565b810190613047565b5f6114e8565b612725565b34610333575f366003190112610333576115506145eb565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346103335760206115ce6115a636611429565b6001600160a01b039182165f9081526098855260408082209290931681526020919091522090565b54604051908152f35b34610333576040366003190112610333576004356115f481610371565b6024356001600160401b038111610333576116347f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b670809091369060040161069b565b9092611642610b3d82613d11565b61164e610b4e82613014565b61082260405192839260018060a01b03169583612849565b34610333575f366003190112610333576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610333575f366003190112610333576033546040516001600160a01b039091168152602090f35b906040600319830112610333576004356116eb81610371565b91602435906001600160401b03821161033357610ce191600401610cc6565b34610333576104d261172461171e366116d2565b906130ec565b604051918291602083526020830190610eb7565b34610333575f36600319011261033357602060405173beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac08152f35b34610333576060366003190112610333576004356001600160401b038111610333576117969036906004016103de565b6024356001600160401b038111610333576117b59036906004016103de565b90916044356001600160401b038111610333576117d99093919336906004016103de565b906117eb610489600480606654161490565b6117fa600260c9541415612ae9565b600260c9553686900360de1901925f5b868110156112d9578060051b88013590858212156103335761185b600192611833838a87612b35565b906118568d611843878c8c612b50565b359461184e8661204e565b369101610d4b565b613e59565b0161180a565b3461033357602036600319011261033357600480355f90815260a460209081526040918290208054600182015460028301546003840154939096015485516001600160a01b03938416815291831694820194909452941692840192909252606083019190915263ffffffff16608082015260a090f35b34610333576020366003190112610333576004356118f481610371565b60018060a01b03165f52609f602052602060405f2054604051908152f35b6001600160401b038111610bfa57601f01601f191660200190565b9060606003198301126103335760043561194681610371565b91602435906001600160401b0382116103335760408282036003190112610333576040519161197483610bdf565b80600401356001600160401b03811161033357810191806023840112156103335760048301356119a381611912565b916119b16040519384610c1a565b81835260248583010111610333576020815f92602480970183860137830101528352013560208201529060443590565b3461033357611a486119f23661192d565b335f908152609a602052604090205492939192611a19906001600160a01b0316151561315f565b611a2b611a2533613014565b15613175565b611a37610b4e85613014565b611a403361468b565b9284336147ff565b611a59610489600180606654161490565b335f908152609a6020526040902080546001600160a01b0319166001600160a01b0384161790556001600160a01b038216337fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433045f80a3611ab833612e2b565b611ac38285336132c1565b915f5b8151811015611b0d57600190611b076001600160a01b03611ae783866127d3565b5116611af383876127d3565b51611afe84896127d3565b5191338b613b80565b01611ac6565b604051806104d2878261040e565b34610333576020366003190112610333576004355f52609e602052602060ff60405f2054166040519015158152f35b3461033357604036600319011261033357600435611b6781610371565b6024359060018060a01b03165f52609c60205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103335760206103d6611bde611bd9611bb136611429565b6001600160a01b039182165f90815260a2875260408082209290931681526020919091522090565b612b6a565b6148a7565b34610333575f36600319011261033357602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b9091611c3a610ce193604084526040840190610eb7565b916020818403910152610eb7565b3461033357611c56366116d2565b611c6081516126f3565b611c6a82516126f3565b91611c9281611c8c6104ae8760018060a01b03165f52609a60205260405f2090565b866132c1565b5f5b8251811015611dc857806020611cc8610530611cc3611cb6611d0996896127d3565b516001600160a01b031690565b6148ba565b611cd5611cb684886127d3565b60405163fe243a1760e01b81526001600160a01b03808c166004830152909116602482015293849190829081906044820190565b03915afa8015610672576001925f91611d9a575b50611d2882886127d3565b52611d89611d6d611bd9611d4c8a60018060a01b03165f5260a260205260405f2090565b611d59611cb6868a6127d3565b60018060a01b03165f5260205260405f2090565b611d7783896127d3565b51611d8284876127d3565b51916141ee565b611d9382876127d3565b5201611c94565b611dbb915060203d8111611dc1575b611db38183610c1a565b810190612de8565b5f611d1d565b503d611da9565b5050506104d260405192839283611c23565b34610333575f366003190112610333576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033357604036600319011261033357600435611e3b81610371565b611e806024355f5492611e6660ff600886901c161580958196611efa575b8115611eda575b5061318b565b83611e77600160ff195f5416175f55565b611ec3576131ee565b611e8657005b611e9461ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b611ed561010061ff00195f5416175f55565b6131ee565b303b15915081611eec575b505f611e60565b60ff1660011490505f611ee5565b600160ff8216109150611e59565b34610333576020366003190112610333576104d2611fc1600435611f2b81610371565b6001600160a01b038082165f908152609a6020526040902054611f509116151561315f565b611f64611f5f61073183613014565b613175565b6001600160a01b038116611f798115156131ff565b6001600160a01b0382165f908152609a60205260409020611f99906104ae565b8133148015908161203f575b8015612002575b611fb590613215565b611fcd575b505061468b565b6040519182918261040e565b6001600160a01b0316907ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a5f80a35f80611fba565b50611fb561203661053060016120288660018060a01b03165f52609960205260405f2090565b01546001600160a01b031690565b33149050611fac565b5061204982613d11565b611fa5565b8015150361033357565b34610333576060366003190112610333576004356001600160401b0381116103335760e0600319823603011261033357602435906001600160401b038211610333576120ab6120eb9236906004016103de565b90611856604435936120bc8561204e565b6120cd610489600480606654161490565b6120dc600260c9541415612ae9565b600260c9553690600401610d4b565b600160c955005b346103335760803660031901126103335760043561210f81610371565b6024359061211c82610371565b604435612128816112ea565b606435612134816112ea565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036122d0576121d66121d06121de926121c86121a48861218f8960018060a01b03165f52609860205260405f2090565b9060018060a01b03165f5260205260405f2090565b546121c26001600160401b0388166001600160401b03851683614e3f565b90614218565b9487876144dd565b83612e1e565b918484614225565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeabf196001600160a01b0384160161220557005b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b1561033357604051633b9e9f0160e21b81526001600160a01b038516600482015260248101839052925f908490604490829084905af1928315610672577feff6aab2bc3f7c648896e1522eae71d6c22e3b0e218206b3f40af0e4d204716b936122bc575b50604080516001600160a01b039586168152602081019390935293169281908101610822565b806108355f6122ca93610c1a565b5f612296565b6323d871a560e01b5f5260045ffd5b346103335761232b6122f03661192d565b335f908152609a6020526040902054929392909190612318906001600160a01b031615612833565b612324610b4e85613014565b83336147ff565b61233c610489600180606654161490565b335f908152609a6020526040902080546001600160a01b0319166001600160a01b0383161790556001600160a01b038116337fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433045f80a361239b33612e2b565b91906123a88183336132c1565b915f5b8251811015610641576001906123ec6001600160a01b036123cc83876127d3565b51166123d883896127d3565b516123e384896127d3565b51913387613b80565b016123ab565b906020610ce1928181520190610f5c565b34610333576040366003190112610333576004356001600160401b03811161033357366023820112156103335780600401359061243f82610c59565b9161244d6040519384610c1a565b8083526024602084019160051b8301019136831161033357602401905b8282106124aa57836024356001600160401b038111610333576104d29161249861249e923690600401610cc6565b9061322b565b604051918291826123f2565b6020809183356124b981610371565b81520191019061246a565b34610333576020366003190112610333576004356124e181610371565b6124e96145eb565b6001600160a01b038116156125015761064190614643565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b34610333575f3660031901126103335760206103d6614937565b346103335760203660031901126103335760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610672575f916125f8575b506001600160a01b031633036125e9576106419061327e565b63794821ff60e01b5f5260045ffd5b90506020813d60201161262b575b8161261360209383610c1a565b81010312610333575161262581610371565b5f6125d0565b3d9150612606565b604080517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad602082019081526001600160a01b0395861692820192909252918416606083015292909116608082015260a081019290925260c0808301939093529181526126a160e082610c1a565b5190206126ac614937565b9060405190602082019261190160f01b845260228301526042820152604281526126d7606282610c1a565b51902090565b156126e457565b63840a48d560e01b5f5260045ffd5b906126fd82610c59565b61270a6040519182610c1a565b828152809261271b601f1991610c59565b0190602036910137565b634e487b7160e01b5f52603260045260245ffd5b91908110156115335760051b81013590605e1981360301821215610333570190565b903590601e198136030182121561033357018035906001600160401b03821161033357602001918160051b3603831361033357565b1561279757565b6343714afd60e01b5f5260045ffd5b35610ce181610371565b156127b757565b6330c4716960e21b5f5260045ffd5b8051156115335760200190565b80518210156115335760209160051b010190565b908160209103126103335751610ce18161204e565b6040513d5f823e3d90fd5b1561280e57565b631d77d47760e21b5f5260045ffd5b1561282457565b63c61dca5d60e01b5f5260045ffd5b1561283a57565b633bf2b50360e11b5f5260045ffd5b90918060409360208452816020850152848401375f828201840152601f01601f1916010190565b908160209103126103335751610ce1816112ea565b1561288c57565b63932d94f760e01b5f5260045ffd5b156128a257565b6325ec6c1f60e01b5f5260045ffd5b6040516126d7816128ce6020820194602086526040830190610eea565b03601f198101835282610c1a565b906128e682610c59565b6128f36040519182610c1a565b8281528092612904601f1991610c59565b01905f5b82811061291457505050565b60209060405161292381610bff565b5f81525f838201525f60408201525f60608201525f6080820152606060a0820152606060c082015282828501015201612908565b9061296182610c59565b61296e6040519182610c1a565b828152809261297f601f1991610c59565b01905f5b82811061298f57505050565b806060602080938501015201612983565b90604051918281549182825260208201905f5260205f20925f5b8181106129cf57505061038d92500383610c1a565b84546001600160a01b03168352600194850194879450602090930192016129ba565b90604051918281549182825260208201905f5260205f20925f5b818110612a2057505061038d92500383610c1a565b8454835260019485019487945060209093019201612a0b565b90604051612a4681610bff565b82546001600160a01b039081168252600184015416602082015291829060c090612ac29060069060028101546001600160a01b0316604086015260038101546060860152612aab612a9e600483015463ffffffff1690565b63ffffffff166080870152565b612ab7600582016129a0565b60a0860152016129f1565b910152565b60408051909190612ad88382610c1a565b6001815291601f1901366020840137565b15612af057565b60405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606490fd5b9082101561153357612b4c9160051b81019061275b565b9091565b91908110156115335760051b0190565b35610ce18161204e565b90604051602081018181106001600160401b03821117610bfa5760405291548252565b6001600160a01b038181165f908152609a60205260409020541615612ce4576001600160a01b0381165f908152609a60205260409020612bcc906104ae565b60405163152667d960e31b81526001600160a01b038216600482015273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac060248201529092602082806044810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156106725761038d95612cb9935f93612cbf575b50612cb390612c96611bd9612c758860018060a01b03165f5260a260205260405f2090565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac05f5260205260405f2090565b936001600160401b0380670de0b6b3a76400005b93169116614d75565b916141ee565b91614298565b612cb3919350612cdd9060203d6020116109a3576109958183610c1a565b9290612c50565b505050565b9080601f83011215610333578151612d0081610c59565b92612d0e6040519485610c1a565b81845260208085019260051b82010192831161033357602001905b828210612d365750505090565b8151815260209182019101612d29565b9190916040818403126103335780516001600160401b03811161033357810183601f8201121561033357805190612d7c82610c59565b91612d8a6040519384610c1a565b80835260208084019160051b8301019186831161033357602001905b828210612dce575050509260208201516001600160401b03811161033357610ce19201612ce9565b602080918351612ddd81610371565b815201910190612da6565b90816020910312610333575190565b634e487b7160e01b5f52601160045260245ffd5b9060018201809211612e1957565b612df7565b91908201809211612e1957565b6040516394f649dd60e01b81526001600160a01b038216600482015291905f83806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610672575f935f92612fe6575b5060405163fe243a1760e01b81526001600160a01b03909116600482015273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac060248201529060208280604481015b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610672575f92612fc5575b508115612fc057612f25612f208551612e0b565b6126f3565b93612f33612f208251612e0b565b92612f5b612f428351886127d3565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac09052565b612f668251856127d3565b525f5b8151811015612fba5780612f9e612f85611cb6600194866127d3565b612f8f838a6127d3565b6001600160a01b039091169052565b612fa881856127d3565b51612fb382876127d3565b5201612f69565b50505090565b919050565b612fdf91925060203d602011611dc157611db38183610c1a565b905f612f0c565b60209450612ed0925061300a903d805f833e6130028183610c1a565b810190612d46565b9490949250612e8e565b6001600160a01b0316801515908161302a575090565b5f818152609a60205260409020546001600160a01b031614919050565b602081830312610333578051906001600160401b03821161033357019080601f8301121561033357815161307a81610c59565b926130886040519485610c1a565b81845260208085019260051b82010192831161033357602001905b8282106130b05750505090565b6020809183516130bf816112ea565b8152019101906130a3565b6001600160a01b039091168152604060208201819052610ce192910190610e7b565b9190916130f983516126f3565b905f5b8451811015613158576001600160a01b038281165f908152609860205260409020600192916131469190613130848a6127d3565b511660018060a01b03165f5260205260405f2090565b5461315182866127d3565b52016130fc565b5090925050565b1561316657565b63a5c7c44560e01b5f5260045ffd5b1561317c57565b6311ca333560e31b5f5260045ffd5b1561319257565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6131fa61038d92613950565b614643565b1561320657565b6339b190bb60e11b5f5260045ffd5b1561321c57565b631e499a2360e11b5f5260045ffd5b906132368251612957565b915f5b8151811015612fba57600190613262846001600160a01b0361325b84876127d3565b51166130ec565b61326c82876127d3565b5261327781866127d3565b5001613239565b61328f60665419821981161461281d565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2565b92916132ef905f816132d381516126f3565b94604051948592839263547afb8760e01b8452600484016130ca565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610672575f92613377575b505f5b815181101561336f578061335e61334a611cb6600194866127d3565b6133576114ee84886127d3565b9089613aaf565b61336882876127d3565b520161332e565b509193505050565b61338c9192503d805f833e6115258183610c1a565b905f61332b565b1561339a57565b63796cc52560e01b5f5260045ffd5b156133b057565b63f020e5b960e01b5f5260045ffd5b6001600160a01b03918216815291166020820152604081019190915260600190565b5f198114612e195760010190565b916134089183549060031b91821b915f19901b19161790565b9055565b91909182821061341b57505050565b5f5260205f2091820191015b818110613432575050565b5f8155600101613427565b90600160401b8111610bfa57815481835561038d9261340c565b8151916001600160401b038311610bfa57602090613475848461343d565b01905f5260205f205f5b83811061348c5750505050565b82516001600160a01b03168183015560209092019160010161347f565b8151916001600160401b038311610bfa576020906134c7848461343d565b01905f5260205f205f5b8381106134de5750505050565b6001906020845194019381840155016134d1565b815181546001600160a01b039182166001600160a01b03199182161783556020840151600184018054918416918316919091179055604084015160028401805491909316911617905560608201516003820155608082015161038d9260069160c091906135789063ffffffff16600486019063ffffffff1663ffffffff19825416179055565b61358960a082015160058601613457565b015191016134a9565b916135af90610ce194928452606060208501526060840190610eea565b916040818403910152610eb7565b9294939091906135d76001600160a01b03851615156131ff565b6135e382511515613393565b6135ed82516126f3565b6135f783516126f3565b925f5b81518110156137f357613613611cc3611cb683856127d3565b90613641611bd96136348a60018060a01b03165f5260a260205260405f2090565b611d59611cb685886127d3565b9161364c828c6127d3565b516001600160a01b039091169261369c602061366b611cb686896127d3565b60405163fe243a1760e01b81526001600160a01b03808f166004830152909116602482015291829081906044820190565b0381885afa8015610672576136d3936136be925f926137d3575b5011156133a9565b6136c8838d6127d3565b51611d8284896127d3565b6136dd82886127d3565b526136fd6136eb82886127d3565b516136f683886127d3565b51906149f4565b61370782866127d3565b526001600160a01b038716613789575b613724611cb682856127d3565b61372e828c6127d3565b51833b156103335761375b935f92838c6040519788958694859363724af42360e01b8552600485016133bf565b03925af191821561067257600192613775575b50016135fa565b806108355f61378393610c1a565b5f61376e565b6137ab613799611cb683866127d3565b6137a383876127d3565b519089614a0a565b6137ce6137bb611cb683866127d3565b6137c583896127d3565b51908a8a614325565b613717565b6137ec91925060203d8111611dc157611db38183610c1a565b905f6136b6565b506001600160a01b0386165f908152609f602052604090208054979850613900977f26b2aae26516e8719ef50ea2f6831a2efbd4e37dccdf0f6936b27bc08e793e3097959695613905955093929161384a826133e1565b9055613874613857610c3b565b6001600160a01b0386168152966001600160a01b03166020880152565b6001600160a01b038416604087015260608601524363ffffffff16608086015260a085015260c08401526138a7836128b1565b9586916138cc6138bf845f52609e60205260405f2090565b805460ff19166001179055565b6138e7856138e2855f5260a460205260405f2090565b6134f2565b6001600160a01b03165f90815260a36020526040902090565b614e69565b506139166040519283928684613592565b0390a190565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b6001600160a01b039081165f8181526099602090815260409182902060010180546001600160a01b0319169590941694851790935551928352917f773b54c04d756fcc5e678111f7d730de3be98192000799eee3d63716055a87c69190a2565b9190916139f6610489600180606654161490565b6001600160a01b038181165f818152609a6020526040812080546001600160a01b03191693871693841790557fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433049080a3613a4f81612e2b565b9091613a5c8386836132c1565b925f5b8151811015613aa657600190613aa06001600160a01b03613a8083866127d3565b5116613a8c83886127d3565b51613a97848a6127d3565b5191878c613b80565b01613a5f565b50505050509050565b91906001600160a01b031673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014613ae2576001600160401b0391501690565b60405163a3d75e0960e01b81526001600160a01b039092166004830152602082806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561067257610ce1925f92613b5f575b506001600160401b0380670de0b6b3a7640000612caa565b613b7991925060203d6020116109a3576109958183610c1a565b905f613b47565b90938015613c94576001600160a01b038581165f90815260a2602090815260408083209387168352929052207f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f91613bf591613be791611bde91611bd99091895f84614ab2565b6040519182918689846133bf565b0390a16001600160a01b038085165f908152609a602052604090205416613c1d575b50505050565b6001600160a01b0381165f908152609860205260409020613c3f90839061218f565b805493808501809511612e19577f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c94613c88925560405193849360018060a01b031696846133bf565b0390a25f808080613c17565b630a33bc6960e21b5f5260045ffd5b919290948015613c9457613be7611bde7f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f93611bd9613bf59460018060a01b038b165f5260a260205289613d0a8a60405f209060018060a01b03165f5260205260405f2090565b9384614ab2565b604051631beb2b9760e31b81526001600160a01b0391821660048201523360248201523060448201525f80356001600160e01b03191660648301529091602091839160849183917f0000000000000000000000000000000000000000000000000000000000000000165af1908115610672575f91613d8d575090565b610ce1915060203d60201161066b5761065d8183610c1a565b15613dad57565b6316110d3560e21b5f5260045ffd5b15613dc357565b6387c9d21960e01b5f5260045ffd5b9063ffffffff8091169116019063ffffffff8211612e1957565b15613df357565b6378f67ae160e11b5f5260045ffd5b9190826040910312610333576020825192015190565b5f600661038d92828155826001820155826002820155826003820155826004820155613e4d836005830180549082815561340c565b0180549082815561340c565b93929360a0810192613e6e8451518214612790565b6040820151613e9090613e89906001600160a01b0316610530565b3314613da6565b613e99826128b1565b613ebd613eb8613eb1835f52609e60205260405f2090565b5460ff1690565b613dbc565b613f34613efb613ed4608086015163ffffffff1690565b7f000000000000000000000000000000000000000000000000000000000000000090613dd2565b613f1363ffffffff431663ffffffff83161115613dec565b84516001600160a01b031660208601516001600160a01b0316885191614b60565b83516001600160a01b03165f908152609a60205260409020909190613f58906104ae565b8451909190613f73906001600160a01b0316838951916132c1565b905f5b885180518210156141625790898989838f95613f99611cc3611cb6848f946127d3565b613fb68c61117385613faf8160c08a01516127d3565b51926127d3565b97156140695792516001600160a01b0393841693613ff393613fee9390929091613fe891611cb69185911699516127d3565b95612b50565b6127a6565b91813b1561033357604051630bab906360e21b81526001600160a01b039485166004820152908416602482015291909216604482015260648101939093525f908390608490829084905af191821561067257600192614055575b505b01613f76565b806108355f61406393610c1a565b5f61404d565b92613fee835f93613fe8611cb660409a999761408e614095975160018060a01b031690565b9a516127d3565b855163c4623ea160e01b81526001600160a01b0395861660048201529285166024840152841660448301526064820196909652948592608492849291165af18015610672578a61411a91600194848c5f925f9461411f575b505161410891611cb6916001600160a01b03165b95516127d3565b614112868a6127d3565b51938a613ca3565b61404f565b611cb691945061410193509161414e6141089360403d811161415b575b6141468183610c1a565b810190613e02565b94909495925050916140ed565b503d61413c565b50509550505050507f1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a009394506141e99250816141aa6138e76141af935160018060a01b031690565b614f0e565b506141ca6141c5825f5260a460205260405f2090565b613e18565b6115016141df825f52609e60205260405f2090565b805460ff19169055565b0390a1565b610ce192916141ff614205926148a7565b90614c41565b614c41565b5f19810191908211612e1957565b91908203918211612e1957565b60018060a01b031691825f5260986020526142538260405f209060018060a01b03165f5260205260405f2090565b918254828103908111612e19577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd93556142936040519283925f846133bf565b0390a2565b91909160018060a01b031691825f52609860205260405f2073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac05f5260205260405f2090815491838303928311612e19577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd9373beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0936142939255604051938493846133bf565b6001600160a01b039081165f818152609860209081526040808320948716835293905291909120805491948083039493928511612e19577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd946142939255604051938493846133bf565b6143ea9060018060a01b031691825f5260a56020526143c96143c48260405f209060018060a01b03165f5260205260405f2090565b614dac565b925f5260a560205260405f209060018060a01b03165f5260205260405f2090565b63ffffffff7f00000000000000000000000000000000000000000000000000000000000000001663ffffffff4316039063ffffffff8211612e195780549163ffffffff165f5b8381106144935750505f92610ce1949261445d92811586146144625750506001600160e01b0384166121c2565b614dd9565b614480614487916144756121c29461420a565b905f5260205f200190565b5460201c90565b6001600160e01b031690565b90928082169080831860011c8201809211612e1957835f528463ffffffff8360205f20015416115f146144c95750925b90614430565b93915060018101809111612e1957906144c3565b6001600160a01b039081165f81815260a56020908152604080832094861683529390529190912090949392916145379161451690614dac565b955f5260a560205260405f209060018060a01b03165f5260205260405f2090565b63ffffffff7f00000000000000000000000000000000000000000000000000000000000000001663ffffffff4316039063ffffffff8211612e195780549163ffffffff165f5b8381106145a15750509461445d91610ce1959681155f1461446257505f90506121c2565b90928082169080831860011c8201809211612e1957835f528463ffffffff8360205f20015416115f146145d75750925b9061457d565b93915060018101809111612e1957906145d1565b6033546001600160a01b031633036145ff57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b610ce1906146a0610489600280606654161490565b6001600160a01b0381165f908152609a6020526040902060609291906146c5906104ae565b906146f36146e38260018060a01b03165f52609a60205260405f2090565b80546001600160a01b0319169055565b6001600160a01b038281169082167ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af446765f80a361472e81612e2b565b91909485519081156147e15750614744906126f3565b926147508682846132c1565b915f5b87518110156147d7576001906147c68961476b612ac7565b614773612ac7565b9061479561478c611cb687614786612ac7565b966127d3565b612f8f836127c6565b61479f858b6127d3565b516147a9836127c6565b526147b4858a6127d3565b516147be846127c6565b5287876135bd565b6147d082896127d3565b5201614753565b5093955050505050565b955050505050565b156147f057565b630d4c4c9160e21b5f5260045ffd5b6001600160a01b038281165f9081526099602052604090206001015491949116929083156148a05761038d9461489691855f52609c60205260405f20815f5260205261485a61485560ff60405f20541615151590565b6147e9565b6148886138bf8261487b8960018060a01b03165f52609c60205260405f2090565b905f5260205260405f2090565b856020850195865193612633565b9051915192614e0a565b5050505050565b5180610ce15750670de0b6b3a764000090565b6001600160a01b031673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac00361490a577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b467f000000000000000000000000000000000000000000000000000000000000000003614982577f000000000000000000000000000000000000000000000000000000000000000090565b600a6020604051614994604082610c1a565b828152016922b4b3b2b72630bcb2b960b11b81522060405160208101917f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a86683526040820152466060820152306080820152608081526126d760a082610c1a565b908015614a0457610ce191614cbb565b50505f90565b90919073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeabf196001600160a01b03841601614a3757505050565b614a8d9060018060a01b031692835f5260a5602052614a6c6143c48260405f209060018060a01b03165f5260205260405f2090565b935f5260a560205260405f209060018060a01b03165f5260205260405f2090565b908201809211612e195761038d916001600160e01b0316904363ffffffff169061516f565b9290918215614afa57614ad482614205614ace611bde88612b6a565b86614c41565b90808201809211612e19578301809311612e195761340892614af591614cbb565b614cbb565b506134089150614d5b565b8054821015611533575f5260205f2001905f90565b90614b2491614b05565b90549060031b1c90565b91614b5963ffffffff9160409396959660018060a01b03168552606060208601526060850190610e7b565b9416910152565b939290915f81614b7081516126f3565b94614b8f6040519586938493632535f40360e21b855260048501614b2e565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610672575f92614bfb575b505f5b815181101561336f5780614bea61334a611cb6600194866127d3565b614bf482876127d3565b5201614bce565b614c109192503d805f833e6115258183610c1a565b905f614bcb565b634e487b7160e01b5f52601260045260245ffd5b8115614c35570490565b614c17565b1561033357565b5f1982820982820291828083109203918083039214614caa5781670de0b6b3a76400001115610333577faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066993670de0b6b3a7640000910990828211900360ee1b910360121c170290565b50670de0b6b3a76400009250500490565b5f19670de0b6b3a7640000820991670de0b6b3a7640000820291828085109403938085039414614d4f57670de0b6b3a76400008291614cfb868411614c3a565b09600182190182168092046002816003021880820260020302808202600203028082026002030280820260020302808202600203028091026002030293600183805f03040190848311900302920304170290565b5090610ce19250614c2b565b8015614c35576ec097ce7bc90715b34b9f10000000000490565b90915f198383099280830292838086109503948086039514614d9f57908291614cfb868411614c3a565b505090610ce19250614c2b565b80549081614dbb57505f919050565b815f19810111612e19575f525f199060205f2001015460201c614487565b916001600160401b03809116911603906001600160401b038211612e19576001600160401b03610ce1921690614c41565b924211614e3057614e1a92615046565b15614e2157565b638baa579f60e01b5f5260045ffd5b630819bdcd60e01b5f5260045ffd5b9190614e4c828285614d75565b928215614c355709614e5b5790565b60018101809111612e195790565b6001810190825f528160205260405f2054155f14614ecc578054600160401b811015610bfa57614eb9614ea3826001879401855584614b05565b819391549060031b91821b915f19901b19161790565b905554915f5260205260405f2055600190565b5050505f90565b80548015614efa575f190190614ee98282614b05565b8154905f199060031b1b1916905555565b634e487b7160e01b5f52603160045260245ffd5b6001810191805f528260205260405f2054928315155f14614f9f575f198401848111612e195783545f19810194908511612e19575f95858361487b94614f5c9803614f62575b505050614ed3565b55600190565b614f88614f8291614f79614b24614f969588614b05565b92839187614b05565b906133ef565b85905f5260205260405f2090565b555f8080614f54565b505050505f90565b60051115614fb157565b634e487b7160e01b5f52602160045260245ffd5b9060609260209183526040828401528051918291826040860152018484015e5f828201840152601f01601f1916010190565b3d15615021573d9061500882611912565b916150166040519384610c1a565b82523d5f602084013e565b606090565b9081602091031261033357516001600160e01b0319811681036103335790565b9190916150538284615248565b61505c81614fa7565b1590816150ed575b506150e5575f926128ce61509185946040519283916020830195630b135d3f60e11b875260248401614fc5565b51915afa61509d614ff7565b816150d9575b816150ac575090565b8051630b135d3f60e11b92506001600160e01b0319916150d491810160209081019101615026565b161490565b805160201491506150a3565b505050600190565b6001600160a01b0383811691161490505f615064565b1561510a57565b63151b8e3f60e11b5f5260045ffd5b8054600160401b811015610bfa5761513691600182018155614b05565b61515c57815160209283015190921b63ffffffff191663ffffffff909216919091179055565b634e487b7160e01b5f525f60045260245ffd5b8054806151ab575b506151a661038d9361519661518a610c4a565b63ffffffff9095168552565b6001600160e01b03166020840152565b615119565b805f19810111612e1957815f5263ffffffff6152196152105f198460205f2001016152066151f8604051926151df84610bdf565b548681169081855260201c602085015263ffffffff1690565b858916958691161115615103565b5163ffffffff1690565b63ffffffff1690565b036151775761038d939250906144756152319261420a565b9063ffffffff82549181199060201b169116179055565b815160418103615270575090612b4c91602082015190606060408401519301515f1a906152b2565b6040036152a95760406020830151920151918260ff1c91601b8301809311612e1957612b4c936001600160ff1b03169260ff16906152b2565b50505f90600290565b9291907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083116153505760ff16601b81141580615345575b61533a576020935f93604051938493608085019385528785015260408401526060830152838052039060015afa15610672575f516001600160a01b0381161561533257905f90565b505f90600190565b505050505f90600490565b50601c8114156152ea565b505050505f9060039056fea2646970667358221220cc6c4be32ce26ae0a9ccd93daa351221a5d87378ab0c9b1f66d6065d3836cf3c64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xA4\xF9y\x14a\x03$W\x80c\x0B\x9FHz\x14a\x03\x1FW\x80c\r\xD8\xDD\x02\x14a\x03\x1AW\x80c\x13d9\xDD\x14a\x03\x15W\x80c*\xA6\xD8\x88\x14a\x03\x10W\x80c9\xB7\x0E8\x14a\x03\x0BW\x80c<e\x1C\xF2\x14a\x03\x06W\x80c<\xDE\xB5\xE0\x14a\x03\x01W\x80c>(9\x1D\x14a\x02\xFCW\x80cFW\xE2j\x14a\x02\xF7W\x80cFe\xBC\xDA\x14a\x02\xF2W\x80cT\xB7\xC9l\x14a\x02\xEDW\x80cY\\jg\x14a\x02\xE8W\x80cY{6\xDA\x14a\x02\xE3W\x80cZ\xC8j\xB7\x14a\x02\xDEW\x80c\\\x97Z\xBB\x14a\x02\xD9W\x80c]\xD6\x85y\x14a\x02\xD4W\x80c_H\xE6g\x14a\x02\xCFW\x80c`\xA0\xD1\xCE\x14a\x02\xCAW\x80ce\xDA\x12d\x14a\x02\xC5W\x80cf\xD5\xBA\x93\x14a\x02\xC0W\x80cmp\xF7\xAE\x14a\x02\xBBW\x80cn\x17DH\x14a\x02\xB6W\x80cqP\x18\xA6\x14a\x02\xB1W\x80cw\x8EU\xF3\x14a\x02\xACW\x80cx)n\xC5\x14a\x02\xA7W\x80c\x88o\x11\x95\x14a\x02\xA2W\x80c\x8D\xA5\xCB[\x14a\x02\x9DW\x80c\x90\x04\x13G\x14a\x02\x98W\x80c\x91\x04\xC3\x19\x14a\x02\x93W\x80c\x945\xBBC\x14a\x02\x8EW\x80c\x99\xF57\x1B\x14a\x02\x89W\x80c\xA1x\x84\x84\x14a\x02\x84W\x80c\xA3:43\x14a\x02\x7FW\x80c\xB7\xF0n\xBE\x14a\x02zW\x80c\xBBE\xFE\xF2\x14a\x02uW\x80c\xBF\xAE?\xD2\x14a\x02pW\x80c\xC4H\xFE\xB8\x14a\x02kW\x80c\xC9x\xF7\xAC\x14a\x02fW\x80c\xCA\x8A\xA7\xC7\x14a\x02aW\x80c\xCDm\xC6\x87\x14a\x02\\W\x80c\xDA\x8B\xE8d\x14a\x02WW\x80c\xE4\xCC?\x90\x14a\x02RW\x80c\xEEt\x93\x7F\x14a\x02MW\x80c\xEE\xA9\x06K\x14a\x02HW\x80c\xF0\xE0\xE6v\x14a\x02CW\x80c\xF2\xFD\xE3\x8B\x14a\x02>W\x80c\xF6\x98\xDA%\x14a\x029Wc\xFA\xBC\x1C\xBC\x14a\x024W_\x80\xFD[a%oV[a%UV[a$\xC4V[a$\x03V[a\"\xDFV[a \xF2V[a XV[a\x1F\x08V[a\x1E\x1EV[a\x1D\xDAV[a\x1CHV[a\x1B\xE3V[a\x1B\x98V[a\x1BJV[a\x1B\x1BV[a\x19\xE1V[a\x18\xD7V[a\x18aV[a\x17fV[a\x178V[a\x17\nV[a\x16\xAAV[a\x16fV[a\x15\xD7V[a\x15\x93V[a\x158V[a\x14NV[a\x14\x02V[a\x13\xB2V[a\x13oV[a\x12\xFBV[a\x11\xA6V[a\x10!V[a\x0E^V[a\x0E+V[a\r\xF1V[a\x0BXV[a\x0B\x04V[a\n\xC0V[a\n|V[a\n1V[a\t\xEBV[a\x08\x85V[a\x08AV[a\x06\xC8V[a\x05\xB7V[a\x04GV[a\x03\x8FV[a\x037V[_\x91\x03\x12a\x033WV[_\x80\xFD[4a\x033W_6`\x03\x19\x01\x12a\x033W` `@Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x033WV[5\x90a\x03\x8D\x82a\x03qV[V[4a\x033W`\xA06`\x03\x19\x01\x12a\x033W` a\x03\xD6`\x045a\x03\xB1\x81a\x03qV[`$5a\x03\xBD\x81a\x03qV[`D5a\x03\xC9\x81a\x03qV[`d5\x91`\x845\x93a&3V[`@Q\x90\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x033W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x033W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x033WV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x041WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x04$V[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x04w\x906\x90`\x04\x01a\x03\xDEV[\x90a\x04\x8Fa\x04\x89`\x02\x80`fT\x16\x14\x90V[\x15a&\xDDV[a\x04\x98\x82a&\xF3V[3_\x90\x81R`\x9A` R`@\x90 \x90\x92\x90a\x04\xBB\x90[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[_[\x82\x81\x10a\x04\xD6W`@Q\x80a\x04\xD2\x87\x82a\x04\x0EV[\x03\x90\xF3[\x80a\x05\x13a\x04\xF0a\x04\xEA`\x01\x94\x87\x89a'9V[\x80a'[V[\x90Pa\x05\na\x05\0\x84\x88\x8Aa'9V[` \x81\x01\x90a'[V[\x91\x90P\x14a'\x90V[a\x05B3a\x05<a\x050`@a\x05*\x86\x8A\x8Ca'9V[\x01a'\xA6V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x14a'\xB0V[a\x05\xA6a\x05ga\x05`a\x05Ya\x04\xEA\x85\x89\x8Ba'9V[6\x91a\x0CpV[\x853a2\xC1V[\x86a\x05\x9E\x87a\x05\x96a\x05\x8Ca\x05\0\x88a\x05\x84a\x04\xEA\x82\x87\x8Aa'9V[\x95\x90\x97a'9V[\x94\x90\x926\x91a\x0CpV[\x926\x91a\x0C\xE4V[\x90\x863a5\xBDV[a\x05\xB0\x82\x88a'\xD3V[R\x01a\x04\xBDV[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06rWa\x06A\x92a\x06-\x91_\x91a\x06CW[Pa(\x07V[a\x06<`fT\x82\x81\x16\x14a(\x1DV[a9PV[\0[a\x06e\x91P` =` \x11a\x06kW[a\x06]\x81\x83a\x0C\x1AV[\x81\x01\x90a'\xE7V[_a\x06'V[P=a\x06SV[a'\xFCV[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x033WV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x033WV[\x91\x81`\x1F\x84\x01\x12\x15a\x033W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x033W` \x83\x81\x86\x01\x95\x01\x01\x11a\x033WV[4a\x033W``6`\x03\x19\x01\x12a\x033W`\x045a\x06\xE5\x81a\x03qV[a\x06\xEDa\x06wV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x07\x0C\x906\x90`\x04\x01a\x06\x9BV[3_\x90\x81R`\x9A` R`@\x90 T\x91\x93\x91a\x075\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a(3V[\x15\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x033W`@Qc+bA\xF3`\xE1\x1B\x81R3`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16`$\x85\x01R_\x90\x84\x90`D\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x06rW\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x93a\x08\"\x93a\x08'W[Pa\x07\xD1\x813a9\x82V[a\x07\xDB33a9\xE2V[`@Q`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R3\x90\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90` \x90\xA2`@Q\x91\x82\x913\x95\x83a(IV[\x03\x90\xA2\0[\x80a\x085_a\x08;\x93a\x0C\x1AV[\x80a\x03)V[_a\x07\xC6V[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x033W`\x806`\x03\x19\x01\x12a\x033W`\x045a\x08\xA2\x81a\x03qV[`$5a\x08\xAE\x81a\x03qV[`d5`D5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x14\x80\x15a\t\xB9W[\x15a\t\xAAW`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x90\x81R`\x9A` \x90\x81R`@\x91\x82\x90 T\x91Qc\x15&g\xD9`\xE3\x1B\x81R\x91\x83\x16`\x04\x83\x01\x81\x90R\x86\x84\x16`$\x84\x01R\x91\x96\x91\x95\x92\x87\x90`D\x90\x82\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x95\x86\x15a\x06rWa\x06A\x96a\tu\x91_\x91a\t{W[P\x83\x83a:\xAFV[\x94a<\xA3V[a\t\x9D\x91P` =` \x11a\t\xA3W[a\t\x95\x81\x83a\x0C\x1AV[\x81\x01\x90a(pV[_a\tmV[P=a\t\x8BV[c\x04R\x06\xA5`\xE2\x1B_R`\x04_\xFD[P3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\xE7V[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045a\n\x08\x81a\x03qV[`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R` `\x01\x80`\xA0\x1B\x03`\x01`@_ \x01T\x16`@Q\x90\x81R\xF3[4a\x033W` 6`\x03\x19\x01\x12a\x033W` a\nr`\x045a\nS\x81a\x03qV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x033W`@6`\x03\x19\x01\x12a\x033Wa\x06A`\x045a\x0B$\x81a\x03qV[`$5\x90a\x0B1\x82a\x03qV[a\x0BBa\x0B=\x82a=\x11V[a(\x85V[a\x0BSa\x0BN\x82a0\x14V[a(\x9BV[a9\x82V[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x06rWa\x0B\xC3\x91_\x91a\x06CWPa(\x07V[a\x06Aa9\x1CV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0B\xFAW`@RV[a\x0B\xCBV[`\xE0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0B\xFAW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0B\xFAW`@RV[`@Q\x90a\x03\x8D`\xE0\x83a\x0C\x1AV[`@Q\x90a\x03\x8D`@\x83a\x0C\x1AV[`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xFAW`\x05\x1B` \x01\x90V[\x92\x91\x90a\x0C|\x81a\x0CYV[\x93a\x0C\x8A`@Q\x95\x86a\x0C\x1AV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x033W\x90[\x82\x82\x10a\x0C\xACWPPPV[` \x80\x91\x835a\x0C\xBB\x81a\x03qV[\x81R\x01\x91\x01\x90a\x0C\xA0V[\x90\x80`\x1F\x83\x01\x12\x15a\x033W\x81` a\x0C\xE1\x935\x91\x01a\x0CpV[\x90V[\x92\x91\x90a\x0C\xF0\x81a\x0CYV[\x93a\x0C\xFE`@Q\x95\x86a\x0C\x1AV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x033W\x90[\x82\x82\x10a\r WPPPV[\x815\x81R` \x91\x82\x01\x91\x01a\r\x14V[\x90\x80`\x1F\x83\x01\x12\x15a\x033W\x81` a\x0C\xE1\x935\x91\x01a\x0C\xE4V[\x91\x90\x91`\xE0\x81\x84\x03\x12a\x033Wa\r`a\x0C;V[\x92a\rj\x82a\x03\x82V[\x84Ra\rx` \x83\x01a\x03\x82V[` \x85\x01Ra\r\x89`@\x83\x01a\x03\x82V[`@\x85\x01R``\x82\x015``\x85\x01Ra\r\xA4`\x80\x83\x01a\x06\x8AV[`\x80\x85\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x033W\x81a\r\xC8\x91\x84\x01a\x0C\xC6V[`\xA0\x85\x01R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\r\xEA\x92\x01a\r0V[`\xC0\x83\x01RV[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x03\xD6a\x0E&` \x926\x90`\x04\x01a\rKV[a(\xB1V[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045`\xFF\x81\x16\x80\x91\x03a\x033W`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x033W_6`\x03\x19\x01\x12a\x033W` `fT`@Q\x90\x81R\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0E\x98WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0E\x8BV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0E\xD4WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0E\xC7V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01Ra\x0C\xE1\x91`\xC0a\x0FK`\xA0\x84\x01Q`\xE0`\xA0\x85\x01R`\xE0\x84\x01\x90a\x0E{V[\x92\x01Q\x90`\xC0\x81\x84\x03\x91\x01Ra\x0E\xB7V[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0F\x87WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0F\xA5`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x0E\xB7V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0FxV[\x92\x91`@\x84\x01\x93`@\x81R\x82Q\x80\x95R``\x81\x01\x94` ``\x82`\x05\x1B\x84\x01\x01\x94\x01\x90_[\x81\x81\x10a\x0F\xF6WPPPa\x0C\xE1\x93\x94P` \x81\x84\x03\x91\x01Ra\x0F\\V[\x90\x91\x94` \x80a\x10\x12`\x01\x93`_\x19\x88\x82\x03\x01\x8CR\x89Qa\x0E\xEAV[\x97\x01\x98\x01\x91\x01\x96\x91\x90\x96a\x0F\xD9V[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045a\x10>\x81a\x03qV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xA3` R`@\x90 a\x10^\x90a)\xF1V[\x90\x81Q\x91a\x10k\x83a(\xDCV[\x91a\x10u\x84a)WV[\x93a\x10\x93a\x04\xAE\x83`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x90_\x92[\x81\x84\x10a\x10\xADW`@Q\x80a\x04\xD2\x89\x89\x83a\x0F\xB4V[a\x10\xD3a\x10\xCEa\x10\xBF\x86\x88\x9A\x98a'\xD3V[Q_R`\xA4` R`@_ \x90V[a*9V[a\x10\xDD\x85\x88a'\xD3V[Ra\x10\xE8\x84\x87a'\xD3V[Pa\x11\x01`\xA0a\x10\xF8\x86\x89a'\xD3V[Q\x01QQa&\xF3V[a\x11\x0B\x85\x87a'\xD3V[Ra\x11\x16\x84\x86a'\xD3V[Pa\x110`\xA0a\x11&\x86\x89a'\xD3V[Q\x01Q\x84\x83a2\xC1V[\x92_[`\xA0a\x11?\x87\x8Aa'\xD3V[Q\x01QQ\x81\x10\x15a\x11\x95W\x80a\x11za\x11h`\x01\x93`\xC0a\x11`\x8B\x8Ea'\xD3V[Q\x01Qa'\xD3V[Qa\x11s\x89\x89a'\xD3V[Q\x90aLAV[a\x11\x8E\x82a\x11\x88\x8A\x8Ca'\xD3V[Qa'\xD3V[R\x01a\x113V[P\x94\x96\x94`\x01\x90\x94\x01\x93\x92Pa\x10\x97V[4a\x033W``6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x11\xD6\x906\x90`\x04\x01a\x03\xDEV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x11\xF5\x906\x90`\x04\x01a\x03\xDEV[\x91\x90`D5\x91a\x12\x0Ca\x04\x89`\x04\x80`fT\x16\x14\x90V[a\x12\x1B`\x02`\xC9T\x14\x15a*\xE9V[`\x02`\xC9U3_\x90\x81R`\xA3` R`@\x90 a\x12D\x90\x80T\x90\x94\x81\x81\x11\x15a\x12\xE3WPa(\xDCV[\x94_[\x86Q\x81\x10\x15a\x12\x8DW\x80a\x12qa\x10\xCEa\x12c`\x01\x94\x89aK\x1AV[_R`\xA4` R`@_ \x90V[a\x12{\x82\x8Aa'\xD3V[Ra\x12\x86\x81\x89a'\xD3V[P\x01a\x12GV[P\x85\x93P\x84_[\x85Q\x81\x10\x15a\x12\xD9W\x80a\x12\xD3a\x12\xAD`\x01\x93\x89a'\xD3V[Qa\x12\xB9\x83\x88\x88a+5V[\x90a\x12\xCDa\x12\xC8\x86\x89\x8Da+PV[a+`V[\x92a>YV[\x01a\x12\x94V[a\x06A`\x01`\xC9UV[\x90Pa(\xDCV[`\x01`\x01`@\x1B\x03\x81\x16\x03a\x033WV[4a\x033W``6`\x03\x19\x01\x12a\x033W`\x045a\x13\x18\x81a\x03qV[`D5`$5a\x13'\x82a\x12\xEAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x13`Wa\x06A\x92a+\x8DV[c2\x13\xA6a`\xE2\x1B_R`\x04_\xFD[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045a\x13\x8C\x81a\x03qV[`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x033W` 6`\x03\x19\x01\x12a\x033Wa\x13\xF4a\x04\xD2a\x13\xDD`\x045a\x13\xD8\x81a\x03qV[a.+V[`@\x92\x91\x92Q\x93\x84\x93`@\x85R`@\x85\x01\x90a\x0E{V[\x90\x83\x82\x03` \x85\x01Ra\x0E\xB7V[4a\x033W` 6`\x03\x19\x01\x12a\x033W` a\nr`\x045a\x14$\x81a\x03qV[a0\x14V[`@\x90`\x03\x19\x01\x12a\x033W`\x045a\x14A\x81a\x03qV[\x90`$5a\x0C\xE1\x81a\x03qV[4a\x033Wa\x14\\6a\x14)V[\x90a\x14ea*\xC7V[\x90\x81Q\x15a\x153W`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R`@QcTz\xFB\x87`\xE0\x1B\x81R\x91_\x90\x83\x90\x81\x90a\x14\x9E\x90\x85`\x04\x84\x01a0\xCAV[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\x06rWa\x14\xFBa\x14\xEEa\x04\xD2\x95a\x15\x01\x95_\x91a\x15\x11W[Pa'\xC6V[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91aC\x8FV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x15-\x91P=\x80_\x83>a\x15%\x81\x83a\x0C\x1AV[\x81\x01\x90a0GV[_a\x14\xE8V[a'%V[4a\x033W_6`\x03\x19\x01\x12a\x033Wa\x15PaE\xEBV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x033W` a\x15\xCEa\x15\xA66a\x14)V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x98\x85R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T`@Q\x90\x81R\xF3[4a\x033W`@6`\x03\x19\x01\x12a\x033W`\x045a\x15\xF4\x81a\x03qV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x164\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x916\x90`\x04\x01a\x06\x9BV[\x90\x92a\x16Ba\x0B=\x82a=\x11V[a\x16Na\x0BN\x82a0\x14V[a\x08\"`@Q\x92\x83\x92`\x01\x80`\xA0\x1B\x03\x16\x95\x83a(IV[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x033W_6`\x03\x19\x01\x12a\x033W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90`@`\x03\x19\x83\x01\x12a\x033W`\x045a\x16\xEB\x81a\x03qV[\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x033Wa\x0C\xE1\x91`\x04\x01a\x0C\xC6V[4a\x033Wa\x04\xD2a\x17$a\x17\x1E6a\x16\xD2V[\x90a0\xECV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x0E\xB7V[4a\x033W_6`\x03\x19\x01\x12a\x033W` `@Qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81R\xF3[4a\x033W``6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x17\x96\x906\x90`\x04\x01a\x03\xDEV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x17\xB5\x906\x90`\x04\x01a\x03\xDEV[\x90\x91`D5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x17\xD9\x90\x93\x91\x936\x90`\x04\x01a\x03\xDEV[\x90a\x17\xEBa\x04\x89`\x04\x80`fT\x16\x14\x90V[a\x17\xFA`\x02`\xC9T\x14\x15a*\xE9V[`\x02`\xC9U6\x86\x90\x03`\xDE\x19\x01\x92_[\x86\x81\x10\x15a\x12\xD9W\x80`\x05\x1B\x88\x015\x90\x85\x82\x12\x15a\x033Wa\x18[`\x01\x92a\x183\x83\x8A\x87a+5V[\x90a\x18V\x8Da\x18C\x87\x8C\x8Ca+PV[5\x94a\x18N\x86a NV[6\x91\x01a\rKV[a>YV[\x01a\x18\nV[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x04\x805_\x90\x81R`\xA4` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T\x93\x90\x96\x01T\x85Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x83\x16\x94\x82\x01\x94\x90\x94R\x94\x16\x92\x84\x01\x92\x90\x92R``\x83\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x90\xF3[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045a\x18\xF4\x81a\x03qV[`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xFAW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x90```\x03\x19\x83\x01\x12a\x033W`\x045a\x19F\x81a\x03qV[\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x033W`@\x82\x82\x03`\x03\x19\x01\x12a\x033W`@Q\x91a\x19t\x83a\x0B\xDFV[\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x033W\x81\x01\x91\x80`#\x84\x01\x12\x15a\x033W`\x04\x83\x015a\x19\xA3\x81a\x19\x12V[\x91a\x19\xB1`@Q\x93\x84a\x0C\x1AV[\x81\x83R`$\x85\x83\x01\x01\x11a\x033W` \x81_\x92`$\x80\x97\x01\x83\x86\x017\x83\x01\x01R\x83R\x015` \x82\x01R\x90`D5\x90V[4a\x033Wa\x1AHa\x19\xF26a\x19-V[3_\x90\x81R`\x9A` R`@\x90 T\x92\x93\x91\x92a\x1A\x19\x90`\x01`\x01`\xA0\x1B\x03\x16\x15\x15a1_V[a\x1A+a\x1A%3a0\x14V[\x15a1uV[a\x1A7a\x0BN\x85a0\x14V[a\x1A@3aF\x8BV[\x92\x843aG\xFFV[a\x1AYa\x04\x89`\x01\x80`fT\x16\x14\x90V[3_\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x163\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04_\x80\xA3a\x1A\xB83a.+V[a\x1A\xC3\x82\x853a2\xC1V[\x91_[\x81Q\x81\x10\x15a\x1B\rW`\x01\x90a\x1B\x07`\x01`\x01`\xA0\x1B\x03a\x1A\xE7\x83\x86a'\xD3V[Q\x16a\x1A\xF3\x83\x87a'\xD3V[Qa\x1A\xFE\x84\x89a'\xD3V[Q\x913\x8Ba;\x80V[\x01a\x1A\xC6V[`@Q\x80a\x04\xD2\x87\x82a\x04\x0EV[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045_R`\x9E` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x033W`@6`\x03\x19\x01\x12a\x033W`\x045a\x1Bg\x81a\x03qV[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x033W` a\x03\xD6a\x1B\xDEa\x1B\xD9a\x1B\xB16a\x14)V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\xA2\x87R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[a+jV[aH\xA7V[4a\x033W_6`\x03\x19\x01\x12a\x033W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[\x90\x91a\x1C:a\x0C\xE1\x93`@\x84R`@\x84\x01\x90a\x0E\xB7V[\x91` \x81\x84\x03\x91\x01Ra\x0E\xB7V[4a\x033Wa\x1CV6a\x16\xD2V[a\x1C`\x81Qa&\xF3V[a\x1Cj\x82Qa&\xF3V[\x91a\x1C\x92\x81a\x1C\x8Ca\x04\xAE\x87`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x86a2\xC1V[_[\x82Q\x81\x10\x15a\x1D\xC8W\x80` a\x1C\xC8a\x050a\x1C\xC3a\x1C\xB6a\x1D\t\x96\x89a'\xD3V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[aH\xBAV[a\x1C\xD5a\x1C\xB6\x84\x88a'\xD3V[`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x8C\x16`\x04\x83\x01R\x90\x91\x16`$\x82\x01R\x93\x84\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x80\x15a\x06rW`\x01\x92_\x91a\x1D\x9AW[Pa\x1D(\x82\x88a'\xD3V[Ra\x1D\x89a\x1Dma\x1B\xD9a\x1DL\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[a\x1DYa\x1C\xB6\x86\x8Aa'\xD3V[`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a\x1Dw\x83\x89a'\xD3V[Qa\x1D\x82\x84\x87a'\xD3V[Q\x91aA\xEEV[a\x1D\x93\x82\x87a'\xD3V[R\x01a\x1C\x94V[a\x1D\xBB\x91P` =\x81\x11a\x1D\xC1W[a\x1D\xB3\x81\x83a\x0C\x1AV[\x81\x01\x90a-\xE8V[_a\x1D\x1DV[P=a\x1D\xA9V[PPPa\x04\xD2`@Q\x92\x83\x92\x83a\x1C#V[4a\x033W_6`\x03\x19\x01\x12a\x033W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x033W`@6`\x03\x19\x01\x12a\x033W`\x045a\x1E;\x81a\x03qV[a\x1E\x80`$5_T\x92a\x1Ef`\xFF`\x08\x86\x90\x1C\x16\x15\x80\x95\x81\x96a\x1E\xFAW[\x81\x15a\x1E\xDAW[Pa1\x8BV[\x83a\x1Ew`\x01`\xFF\x19_T\x16\x17_UV[a\x1E\xC3Wa1\xEEV[a\x1E\x86W\0[a\x1E\x94a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x1E\xD5a\x01\0a\xFF\0\x19_T\x16\x17_UV[a1\xEEV[0;\x15\x91P\x81a\x1E\xECW[P_a\x1E`V[`\xFF\x16`\x01\x14\x90P_a\x1E\xE5V[`\x01`\xFF\x82\x16\x10\x91Pa\x1EYV[4a\x033W` 6`\x03\x19\x01\x12a\x033Wa\x04\xD2a\x1F\xC1`\x045a\x1F+\x81a\x03qV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x9A` R`@\x90 Ta\x1FP\x91\x16\x15\x15a1_V[a\x1Fda\x1F_a\x071\x83a0\x14V[a1uV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1Fy\x81\x15\x15a1\xFFV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9A` R`@\x90 a\x1F\x99\x90a\x04\xAEV[\x813\x14\x80\x15\x90\x81a ?W[\x80\x15a \x02W[a\x1F\xB5\x90a2\x15V[a\x1F\xCDW[PPaF\x8BV[`@Q\x91\x82\x91\x82a\x04\x0EV[`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A_\x80\xA3_\x80a\x1F\xBAV[Pa\x1F\xB5a 6a\x050`\x01a (\x86`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[3\x14\x90Pa\x1F\xACV[Pa I\x82a=\x11V[a\x1F\xA5V[\x80\x15\x15\x03a\x033WV[4a\x033W``6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033W`\xE0`\x03\x19\x826\x03\x01\x12a\x033W`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x033Wa \xABa \xEB\x926\x90`\x04\x01a\x03\xDEV[\x90a\x18V`D5\x93a \xBC\x85a NV[a \xCDa\x04\x89`\x04\x80`fT\x16\x14\x90V[a \xDC`\x02`\xC9T\x14\x15a*\xE9V[`\x02`\xC9U6\x90`\x04\x01a\rKV[`\x01`\xC9U\0[4a\x033W`\x806`\x03\x19\x01\x12a\x033W`\x045a!\x0F\x81a\x03qV[`$5\x90a!\x1C\x82a\x03qV[`D5a!(\x81a\x12\xEAV[`d5a!4\x81a\x12\xEAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\"\xD0Wa!\xD6a!\xD0a!\xDE\x92a!\xC8a!\xA4\x88a!\x8F\x89`\x01\x80`\xA0\x1B\x03\x16_R`\x98` R`@_ \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta!\xC2`\x01`\x01`@\x1B\x03\x88\x16`\x01`\x01`@\x1B\x03\x85\x16\x83aN?V[\x90aB\x18V[\x94\x87\x87aD\xDDV[\x83a.\x1EV[\x91\x84\x84aB%V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xBF\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01a\"\x05W\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x033W`@Qc;\x9E\x9F\x01`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x92_\x90\x84\x90`D\x90\x82\x90\x84\x90Z\xF1\x92\x83\x15a\x06rW\x7F\xEF\xF6\xAA\xB2\xBC?|d\x88\x96\xE1R.\xAEq\xD6\xC2.;\x0E!\x82\x06\xB3\xF4\n\xF0\xE4\xD2\x04qk\x93a\"\xBCW[P`@\x80Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R` \x81\x01\x93\x90\x93R\x93\x16\x92\x81\x90\x81\x01a\x08\"V[\x80a\x085_a\"\xCA\x93a\x0C\x1AV[_a\"\x96V[c#\xD8q\xA5`\xE0\x1B_R`\x04_\xFD[4a\x033Wa#+a\"\xF06a\x19-V[3_\x90\x81R`\x9A` R`@\x90 T\x92\x93\x92\x90\x91\x90a#\x18\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a(3V[a#$a\x0BN\x85a0\x14V[\x833aG\xFFV[a#<a\x04\x89`\x01\x80`fT\x16\x14\x90V[3_\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x81\x163\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04_\x80\xA3a#\x9B3a.+V[\x91\x90a#\xA8\x81\x833a2\xC1V[\x91_[\x82Q\x81\x10\x15a\x06AW`\x01\x90a#\xEC`\x01`\x01`\xA0\x1B\x03a#\xCC\x83\x87a'\xD3V[Q\x16a#\xD8\x83\x89a'\xD3V[Qa#\xE3\x84\x89a'\xD3V[Q\x913\x87a;\x80V[\x01a#\xABV[\x90` a\x0C\xE1\x92\x81\x81R\x01\x90a\x0F\\V[4a\x033W`@6`\x03\x19\x01\x12a\x033W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x033W6`#\x82\x01\x12\x15a\x033W\x80`\x04\x015\x90a$?\x82a\x0CYV[\x91a$M`@Q\x93\x84a\x0C\x1AV[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x033W`$\x01\x90[\x82\x82\x10a$\xAAW\x83`$5`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x04\xD2\x91a$\x98a$\x9E\x926\x90`\x04\x01a\x0C\xC6V[\x90a2+V[`@Q\x91\x82\x91\x82a#\xF2V[` \x80\x91\x835a$\xB9\x81a\x03qV[\x81R\x01\x91\x01\x90a$jV[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045a$\xE1\x81a\x03qV[a$\xE9aE\xEBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a%\x01Wa\x06A\x90aFCV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x033W_6`\x03\x19\x01\x12a\x033W` a\x03\xD6aI7V[4a\x033W` 6`\x03\x19\x01\x12a\x033W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06rW_\x91a%\xF8W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a%\xE9Wa\x06A\x90a2~V[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a&+W[\x81a&\x13` \x93\x83a\x0C\x1AV[\x81\x01\x03\x12a\x033WQa&%\x81a\x03qV[_a%\xD0V[=\x91Pa&\x06V[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x92\x82\x01\x92\x90\x92R\x91\x84\x16``\x83\x01R\x92\x90\x91\x16`\x80\x82\x01R`\xA0\x81\x01\x92\x90\x92R`\xC0\x80\x83\x01\x93\x90\x93R\x91\x81Ra&\xA1`\xE0\x82a\x0C\x1AV[Q\x90 a&\xACaI7V[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra&\xD7`b\x82a\x0C\x1AV[Q\x90 \x90V[\x15a&\xE4WV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x90a&\xFD\x82a\x0CYV[a'\n`@Q\x91\x82a\x0C\x1AV[\x82\x81R\x80\x92a'\x1B`\x1F\x19\x91a\x0CYV[\x01\x90` 6\x91\x017V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x81\x10\x15a\x153W`\x05\x1B\x81\x015\x90`^\x19\x816\x03\x01\x82\x12\x15a\x033W\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x033W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x033W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x033WV[\x15a'\x97WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[5a\x0C\xE1\x81a\x03qV[\x15a'\xB7WV[c0\xC4qi`\xE2\x1B_R`\x04_\xFD[\x80Q\x15a\x153W` \x01\x90V[\x80Q\x82\x10\x15a\x153W` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x033WQa\x0C\xE1\x81a NV[`@Q=_\x82>=\x90\xFD[\x15a(\x0EWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a($WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[\x15a(:WV[c;\xF2\xB5\x03`\xE1\x1B_R`\x04_\xFD[\x90\x91\x80`@\x93` \x84R\x81` \x85\x01R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x033WQa\x0C\xE1\x81a\x12\xEAV[\x15a(\x8CWV[c\x93-\x94\xF7`\xE0\x1B_R`\x04_\xFD[\x15a(\xA2WV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[`@Qa&\xD7\x81a(\xCE` \x82\x01\x94` \x86R`@\x83\x01\x90a\x0E\xEAV[\x03`\x1F\x19\x81\x01\x83R\x82a\x0C\x1AV[\x90a(\xE6\x82a\x0CYV[a(\xF3`@Q\x91\x82a\x0C\x1AV[\x82\x81R\x80\x92a)\x04`\x1F\x19\x91a\x0CYV[\x01\x90_[\x82\x81\x10a)\x14WPPPV[` \x90`@Qa)#\x81a\x0B\xFFV[_\x81R_\x83\x82\x01R_`@\x82\x01R_``\x82\x01R_`\x80\x82\x01R```\xA0\x82\x01R```\xC0\x82\x01R\x82\x82\x85\x01\x01R\x01a)\x08V[\x90a)a\x82a\x0CYV[a)n`@Q\x91\x82a\x0C\x1AV[\x82\x81R\x80\x92a)\x7F`\x1F\x19\x91a\x0CYV[\x01\x90_[\x82\x81\x10a)\x8FWPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a)\x83V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_[\x81\x81\x10a)\xCFWPPa\x03\x8D\x92P\x03\x83a\x0C\x1AV[\x84T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01a)\xBAV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_[\x81\x81\x10a* WPPa\x03\x8D\x92P\x03\x83a\x0C\x1AV[\x84T\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01a*\x0BV[\x90`@Qa*F\x81a\x0B\xFFV[\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x84\x01T\x16` \x82\x01R\x91\x82\x90`\xC0\x90a*\xC2\x90`\x06\x90`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16`@\x86\x01R`\x03\x81\x01T``\x86\x01Ra*\xABa*\x9E`\x04\x83\x01Tc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16`\x80\x87\x01RV[a*\xB7`\x05\x82\x01a)\xA0V[`\xA0\x86\x01R\x01a)\xF1V[\x91\x01RV[`@\x80Q\x90\x91\x90a*\xD8\x83\x82a\x0C\x1AV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x15a*\xF0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x90\xFD[\x90\x82\x10\x15a\x153Wa+L\x91`\x05\x1B\x81\x01\x90a'[V[\x90\x91V[\x91\x90\x81\x10\x15a\x153W`\x05\x1B\x01\x90V[5a\x0C\xE1\x81a NV[\x90`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0B\xFAW`@R\x91T\x82RV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15a,\xE4W`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9A` R`@\x90 a+\xCC\x90a\x04\xAEV[`@Qc\x15&g\xD9`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x82\x01R\x90\x92` \x82\x80`D\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06rWa\x03\x8D\x95a,\xB9\x93_\x93a,\xBFW[Pa,\xB3\x90a,\x96a\x1B\xD9a,u\x88`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0_R` R`@_ \x90V[\x93`\x01`\x01`@\x1B\x03\x80g\r\xE0\xB6\xB3\xA7d\0\0[\x93\x16\x91\x16aMuV[\x91aA\xEEV[\x91aB\x98V[a,\xB3\x91\x93Pa,\xDD\x90` =` \x11a\t\xA3Wa\t\x95\x81\x83a\x0C\x1AV[\x92\x90a,PV[PPPV[\x90\x80`\x1F\x83\x01\x12\x15a\x033W\x81Qa-\0\x81a\x0CYV[\x92a-\x0E`@Q\x94\x85a\x0C\x1AV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x033W` \x01\x90[\x82\x82\x10a-6WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a-)V[\x91\x90\x91`@\x81\x84\x03\x12a\x033W\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x033W\x81\x01\x83`\x1F\x82\x01\x12\x15a\x033W\x80Q\x90a-|\x82a\x0CYV[\x91a-\x8A`@Q\x93\x84a\x0C\x1AV[\x80\x83R` \x80\x84\x01\x91`\x05\x1B\x83\x01\x01\x91\x86\x83\x11a\x033W` \x01\x90[\x82\x82\x10a-\xCEWPPP\x92` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x033Wa\x0C\xE1\x92\x01a,\xE9V[` \x80\x91\x83Qa-\xDD\x81a\x03qV[\x81R\x01\x91\x01\x90a-\xA6V[\x90\x81` \x91\x03\x12a\x033WQ\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x01\x80\x92\x11a.\x19WV[a-\xF7V[\x91\x90\x82\x01\x80\x92\x11a.\x19WV[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R\x91\x90_\x83\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06rW_\x93_\x92a/\xE6W[P`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x82\x01R\x90` \x82\x80`D\x81\x01[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06rW_\x92a/\xC5W[P\x81\x15a/\xC0Wa/%a/ \x85Qa.\x0BV[a&\xF3V[\x93a/3a/ \x82Qa.\x0BV[\x92a/[a/B\x83Q\x88a'\xD3V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x90RV[a/f\x82Q\x85a'\xD3V[R_[\x81Q\x81\x10\x15a/\xBAW\x80a/\x9Ea/\x85a\x1C\xB6`\x01\x94\x86a'\xD3V[a/\x8F\x83\x8Aa'\xD3V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a/\xA8\x81\x85a'\xD3V[Qa/\xB3\x82\x87a'\xD3V[R\x01a/iV[PPP\x90V[\x91\x90PV[a/\xDF\x91\x92P` =` \x11a\x1D\xC1Wa\x1D\xB3\x81\x83a\x0C\x1AV[\x90_a/\x0CV[` \x94Pa.\xD0\x92Pa0\n\x90=\x80_\x83>a0\x02\x81\x83a\x0C\x1AV[\x81\x01\x90a-FV[\x94\x90\x94\x92Pa.\x8EV[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x15\x90\x81a0*WP\x90V[_\x81\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14\x91\x90PV[` \x81\x83\x03\x12a\x033W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x033W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x033W\x81Qa0z\x81a\x0CYV[\x92a0\x88`@Q\x94\x85a\x0C\x1AV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x033W` \x01\x90[\x82\x82\x10a0\xB0WPPP\x90V[` \x80\x91\x83Qa0\xBF\x81a\x12\xEAV[\x81R\x01\x91\x01\x90a0\xA3V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra\x0C\xE1\x92\x91\x01\x90a\x0E{V[\x91\x90\x91a0\xF9\x83Qa&\xF3V[\x90_[\x84Q\x81\x10\x15a1XW`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x98` R`@\x90 `\x01\x92\x91a1F\x91\x90a10\x84\x8Aa'\xD3V[Q\x16`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta1Q\x82\x86a'\xD3V[R\x01a0\xFCV[P\x90\x92PPV[\x15a1fWV[c\xA5\xC7\xC4E`\xE0\x1B_R`\x04_\xFD[\x15a1|WV[c\x11\xCA35`\xE3\x1B_R`\x04_\xFD[\x15a1\x92WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a1\xFAa\x03\x8D\x92a9PV[aFCV[\x15a2\x06WV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[\x15a2\x1CWV[c\x1EI\x9A#`\xE1\x1B_R`\x04_\xFD[\x90a26\x82Qa)WV[\x91_[\x81Q\x81\x10\x15a/\xBAW`\x01\x90a2b\x84`\x01`\x01`\xA0\x1B\x03a2[\x84\x87a'\xD3V[Q\x16a0\xECV[a2l\x82\x87a'\xD3V[Ra2w\x81\x86a'\xD3V[P\x01a29V[a2\x8F`fT\x19\x82\x19\x81\x16\x14a(\x1DV[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2V[\x92\x91a2\xEF\x90_\x81a2\xD3\x81Qa&\xF3V[\x94`@Q\x94\x85\x92\x83\x92cTz\xFB\x87`\xE0\x1B\x84R`\x04\x84\x01a0\xCAV[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06rW_\x92a3wW[P_[\x81Q\x81\x10\x15a3oW\x80a3^a3Ja\x1C\xB6`\x01\x94\x86a'\xD3V[a3Wa\x14\xEE\x84\x88a'\xD3V[\x90\x89a:\xAFV[a3h\x82\x87a'\xD3V[R\x01a3.V[P\x91\x93PPPV[a3\x8C\x91\x92P=\x80_\x83>a\x15%\x81\x83a\x0C\x1AV[\x90_a3+V[\x15a3\x9AWV[cyl\xC5%`\xE0\x1B_R`\x04_\xFD[\x15a3\xB0WV[c\xF0 \xE5\xB9`\xE0\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x91\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_\x19\x81\x14a.\x19W`\x01\x01\x90V[\x91a4\x08\x91\x83T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90UV[\x91\x90\x91\x82\x82\x10a4\x1BWPPPV[_R` _ \x91\x82\x01\x91\x01[\x81\x81\x10a42WPPV[_\x81U`\x01\x01a4'V[\x90`\x01`@\x1B\x81\x11a\x0B\xFAW\x81T\x81\x83Ua\x03\x8D\x92a4\x0CV[\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x0B\xFAW` \x90a4u\x84\x84a4=V[\x01\x90_R` _ _[\x83\x81\x10a4\x8CWPPPPV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x83\x01U` \x90\x92\x01\x91`\x01\x01a4\x7FV[\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x0B\xFAW` \x90a4\xC7\x84\x84a4=V[\x01\x90_R` _ _[\x83\x81\x10a4\xDEWPPPPV[`\x01\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a4\xD1V[\x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U` \x84\x01Q`\x01\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\x02\x84\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U``\x82\x01Q`\x03\x82\x01U`\x80\x82\x01Qa\x03\x8D\x92`\x06\x91`\xC0\x91\x90a5x\x90c\xFF\xFF\xFF\xFF\x16`\x04\x86\x01\x90c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[a5\x89`\xA0\x82\x01Q`\x05\x86\x01a4WV[\x01Q\x91\x01a4\xA9V[\x91a5\xAF\x90a\x0C\xE1\x94\x92\x84R``` \x85\x01R``\x84\x01\x90a\x0E\xEAV[\x91`@\x81\x84\x03\x91\x01Ra\x0E\xB7V[\x92\x94\x93\x90\x91\x90a5\xD7`\x01`\x01`\xA0\x1B\x03\x85\x16\x15\x15a1\xFFV[a5\xE3\x82Q\x15\x15a3\x93V[a5\xED\x82Qa&\xF3V[a5\xF7\x83Qa&\xF3V[\x92_[\x81Q\x81\x10\x15a7\xF3Wa6\x13a\x1C\xC3a\x1C\xB6\x83\x85a'\xD3V[\x90a6Aa\x1B\xD9a64\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[a\x1DYa\x1C\xB6\x85\x88a'\xD3V[\x91a6L\x82\x8Ca'\xD3V[Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92a6\x9C` a6ka\x1C\xB6\x86\x89a'\xD3V[`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x8F\x16`\x04\x83\x01R\x90\x91\x16`$\x82\x01R\x91\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x81\x88Z\xFA\x80\x15a\x06rWa6\xD3\x93a6\xBE\x92_\x92a7\xD3W[P\x11\x15a3\xA9V[a6\xC8\x83\x8Da'\xD3V[Qa\x1D\x82\x84\x89a'\xD3V[a6\xDD\x82\x88a'\xD3V[Ra6\xFDa6\xEB\x82\x88a'\xD3V[Qa6\xF6\x83\x88a'\xD3V[Q\x90aI\xF4V[a7\x07\x82\x86a'\xD3V[R`\x01`\x01`\xA0\x1B\x03\x87\x16a7\x89W[a7$a\x1C\xB6\x82\x85a'\xD3V[a7.\x82\x8Ca'\xD3V[Q\x83;\x15a\x033Wa7[\x93_\x92\x83\x8C`@Q\x97\x88\x95\x86\x94\x85\x93crJ\xF4#`\xE0\x1B\x85R`\x04\x85\x01a3\xBFV[\x03\x92Z\xF1\x91\x82\x15a\x06rW`\x01\x92a7uW[P\x01a5\xFAV[\x80a\x085_a7\x83\x93a\x0C\x1AV[_a7nV[a7\xABa7\x99a\x1C\xB6\x83\x86a'\xD3V[a7\xA3\x83\x87a'\xD3V[Q\x90\x89aJ\nV[a7\xCEa7\xBBa\x1C\xB6\x83\x86a'\xD3V[a7\xC5\x83\x89a'\xD3V[Q\x90\x8A\x8AaC%V[a7\x17V[a7\xEC\x91\x92P` =\x81\x11a\x1D\xC1Wa\x1D\xB3\x81\x83a\x0C\x1AV[\x90_a6\xB6V[P`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x9F` R`@\x90 \x80T\x97\x98Pa9\0\x97\x7F&\xB2\xAA\xE2e\x16\xE8q\x9E\xF5\x0E\xA2\xF6\x83\x1A.\xFB\xD4\xE3}\xCC\xDF\x0Fi6\xB2{\xC0\x8Ey>0\x97\x95\x96\x95a9\x05\x95P\x93\x92\x91a8J\x82a3\xE1V[\x90Ua8ta8Wa\x0C;V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R\x96`\x01`\x01`\xA0\x1B\x03\x16` \x88\x01RV[`\x01`\x01`\xA0\x1B\x03\x84\x16`@\x87\x01R``\x86\x01RCc\xFF\xFF\xFF\xFF\x16`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01Ra8\xA7\x83a(\xB1V[\x95\x86\x91a8\xCCa8\xBF\x84_R`\x9E` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[a8\xE7\x85a8\xE2\x85_R`\xA4` R`@_ \x90V[a4\xF2V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\xA3` R`@\x90 \x90V[aNiV[Pa9\x16`@Q\x92\x83\x92\x86\x84a5\x92V[\x03\x90\xA1\x90V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x99` \x90\x81R`@\x91\x82\x90 `\x01\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x95\x90\x94\x16\x94\x85\x17\x90\x93UQ\x92\x83R\x91\x7Fw;T\xC0Muo\xCC^g\x81\x11\xF7\xD70\xDE;\xE9\x81\x92\0\x07\x99\xEE\xE3\xD67\x16\x05Z\x87\xC6\x91\x90\xA2V[\x91\x90\x91a9\xF6a\x04\x89`\x01\x80`fT\x16\x14\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x9A` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x87\x16\x93\x84\x17\x90U\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x90\x80\xA3a:O\x81a.+V[\x90\x91a:\\\x83\x86\x83a2\xC1V[\x92_[\x81Q\x81\x10\x15a:\xA6W`\x01\x90a:\xA0`\x01`\x01`\xA0\x1B\x03a:\x80\x83\x86a'\xD3V[Q\x16a:\x8C\x83\x88a'\xD3V[Qa:\x97\x84\x8Aa'\xD3V[Q\x91\x87\x8Ca;\x80V[\x01a:_V[PPPPP\x90PV[\x91\x90`\x01`\x01`\xA0\x1B\x03\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a:\xE2W`\x01`\x01`@\x1B\x03\x91P\x16\x90V[`@Qc\xA3\xD7^\t`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R` \x82\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06rWa\x0C\xE1\x92_\x92a;_W[P`\x01`\x01`@\x1B\x03\x80g\r\xE0\xB6\xB3\xA7d\0\0a,\xAAV[a;y\x91\x92P` =` \x11a\t\xA3Wa\t\x95\x81\x83a\x0C\x1AV[\x90_a;GV[\x90\x93\x80\x15a<\x94W`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x91a;\xF5\x91a;\xE7\x91a\x1B\xDE\x91a\x1B\xD9\x90\x91\x89_\x84aJ\xB2V[`@Q\x91\x82\x91\x86\x89\x84a3\xBFV[\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x9A` R`@\x90 T\x16a<\x1DW[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x98` R`@\x90 a<?\x90\x83\x90a!\x8FV[\x80T\x93\x80\x85\x01\x80\x95\x11a.\x19W\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x94a<\x88\x92U`@Q\x93\x84\x93`\x01\x80`\xA0\x1B\x03\x16\x96\x84a3\xBFV[\x03\x90\xA2_\x80\x80\x80a<\x17V[c\n3\xBCi`\xE2\x1B_R`\x04_\xFD[\x91\x92\x90\x94\x80\x15a<\x94Wa;\xE7a\x1B\xDE\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x93a\x1B\xD9a;\xF5\x94`\x01\x80`\xA0\x1B\x03\x8B\x16_R`\xA2` R\x89a=\n\x8A`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x93\x84aJ\xB2V[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R3`$\x82\x01R0`D\x82\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x90\x91` \x91\x83\x91`\x84\x91\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xF1\x90\x81\x15a\x06rW_\x91a=\x8DWP\x90V[a\x0C\xE1\x91P` =` \x11a\x06kWa\x06]\x81\x83a\x0C\x1AV[\x15a=\xADWV[c\x16\x11\r5`\xE2\x1B_R`\x04_\xFD[\x15a=\xC3WV[c\x87\xC9\xD2\x19`\xE0\x1B_R`\x04_\xFD[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a.\x19WV[\x15a=\xF3WV[cx\xF6z\xE1`\xE1\x1B_R`\x04_\xFD[\x91\x90\x82`@\x91\x03\x12a\x033W` \x82Q\x92\x01Q\x90V[_`\x06a\x03\x8D\x92\x82\x81U\x82`\x01\x82\x01U\x82`\x02\x82\x01U\x82`\x03\x82\x01U\x82`\x04\x82\x01Ua>M\x83`\x05\x83\x01\x80T\x90\x82\x81Ua4\x0CV[\x01\x80T\x90\x82\x81Ua4\x0CV[\x93\x92\x93`\xA0\x81\x01\x92a>n\x84QQ\x82\x14a'\x90V[`@\x82\x01Qa>\x90\x90a>\x89\x90`\x01`\x01`\xA0\x1B\x03\x16a\x050V[3\x14a=\xA6V[a>\x99\x82a(\xB1V[a>\xBDa>\xB8a>\xB1\x83_R`\x9E` R`@_ \x90V[T`\xFF\x16\x90V[a=\xBCV[a?4a>\xFBa>\xD4`\x80\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a=\xD2V[a?\x13c\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x83\x16\x11\x15a=\xECV[\x84Q`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x88Q\x91aK`V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9A` R`@\x90 \x90\x91\x90a?X\x90a\x04\xAEV[\x84Q\x90\x91\x90a?s\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x89Q\x91a2\xC1V[\x90_[\x88Q\x80Q\x82\x10\x15aAbW\x90\x89\x89\x89\x83\x8F\x95a?\x99a\x1C\xC3a\x1C\xB6\x84\x8F\x94a'\xD3V[a?\xB6\x8Ca\x11s\x85a?\xAF\x81`\xC0\x8A\x01Qa'\xD3V[Q\x92a'\xD3V[\x97\x15a@iW\x92Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93a?\xF3\x93a?\xEE\x93\x90\x92\x90\x91a?\xE8\x91a\x1C\xB6\x91\x85\x91\x16\x99Qa'\xD3V[\x95a+PV[a'\xA6V[\x91\x81;\x15a\x033W`@Qc\x0B\xAB\x90c`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x90\x84\x16`$\x82\x01R\x91\x90\x92\x16`D\x82\x01R`d\x81\x01\x93\x90\x93R_\x90\x83\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x06rW`\x01\x92a@UW[P[\x01a?vV[\x80a\x085_a@c\x93a\x0C\x1AV[_a@MV[\x92a?\xEE\x83_\x93a?\xE8a\x1C\xB6`@\x9A\x99\x97a@\x8Ea@\x95\x97Q`\x01\x80`\xA0\x1B\x03\x16\x90V[\x9AQa'\xD3V[\x85Qc\xC4b>\xA1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x04\x82\x01R\x92\x85\x16`$\x84\x01R\x84\x16`D\x83\x01R`d\x82\x01\x96\x90\x96R\x94\x85\x92`\x84\x92\x84\x92\x91\x16Z\xF1\x80\x15a\x06rW\x8AaA\x1A\x91`\x01\x94\x84\x8C_\x92_\x94aA\x1FW[PQaA\x08\x91a\x1C\xB6\x91`\x01`\x01`\xA0\x1B\x03\x16[\x95Qa'\xD3V[aA\x12\x86\x8Aa'\xD3V[Q\x93\x8Aa<\xA3V[a@OV[a\x1C\xB6\x91\x94PaA\x01\x93P\x91aANaA\x08\x93`@=\x81\x11aA[W[aAF\x81\x83a\x0C\x1AV[\x81\x01\x90a>\x02V[\x94\x90\x94\x95\x92PP\x91a@\xEDV[P=aA<V[PP\x95PPPPP\x7F\x1F@@\x08\x89'N\xD0{$\x84^PT\xA8z\x0C\xAB\x96\x9E\xB1'z\xAF\xE6\x1A\xE3R\xE7\xC3*\0\x93\x94PaA\xE9\x92P\x81aA\xAAa8\xE7aA\xAF\x93Q`\x01\x80`\xA0\x1B\x03\x16\x90V[aO\x0EV[PaA\xCAaA\xC5\x82_R`\xA4` R`@_ \x90V[a>\x18V[a\x15\x01aA\xDF\x82_R`\x9E` R`@_ \x90V[\x80T`\xFF\x19\x16\x90UV[\x03\x90\xA1V[a\x0C\xE1\x92\x91aA\xFFaB\x05\x92aH\xA7V[\x90aLAV[aLAV[_\x19\x81\x01\x91\x90\x82\x11a.\x19WV[\x91\x90\x82\x03\x91\x82\x11a.\x19WV[`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\x98` RaBS\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x91\x82T\x82\x81\x03\x90\x81\x11a.\x19W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x93UaB\x93`@Q\x92\x83\x92_\x84a3\xBFV[\x03\x90\xA2V[\x91\x90\x91`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\x98` R`@_ s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0_R` R`@_ \x90\x81T\x91\x83\x83\x03\x92\x83\x11a.\x19W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x93s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x93aB\x93\x92U`@Q\x93\x84\x93\x84a3\xBFV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x98` \x90\x81R`@\x80\x83 \x94\x87\x16\x83R\x93\x90R\x91\x90\x91 \x80T\x91\x94\x80\x83\x03\x94\x93\x92\x85\x11a.\x19W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x94aB\x93\x92U`@Q\x93\x84\x93\x84a3\xBFV[aC\xEA\x90`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\xA5` RaC\xC9aC\xC4\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[aM\xACV[\x92_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xFF\xFF\xFF\xFFC\x16\x03\x90c\xFF\xFF\xFF\xFF\x82\x11a.\x19W\x80T\x91c\xFF\xFF\xFF\xFF\x16_[\x83\x81\x10aD\x93WPP_\x92a\x0C\xE1\x94\x92aD]\x92\x81\x15\x86\x14aDbWPP`\x01`\x01`\xE0\x1B\x03\x84\x16a!\xC2V[aM\xD9V[aD\x80aD\x87\x91aDua!\xC2\x94aB\nV[\x90_R` _ \x01\x90V[T` \x1C\x90V[`\x01`\x01`\xE0\x1B\x03\x16\x90V[\x90\x92\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a.\x19W\x83_R\x84c\xFF\xFF\xFF\xFF\x83` _ \x01T\x16\x11_\x14aD\xC9WP\x92[\x90aD0V[\x93\x91P`\x01\x81\x01\x80\x91\x11a.\x19W\x90aD\xC3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\xA5` \x90\x81R`@\x80\x83 \x94\x86\x16\x83R\x93\x90R\x91\x90\x91 \x90\x94\x93\x92\x91aE7\x91aE\x16\x90aM\xACV[\x95_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xFF\xFF\xFF\xFFC\x16\x03\x90c\xFF\xFF\xFF\xFF\x82\x11a.\x19W\x80T\x91c\xFF\xFF\xFF\xFF\x16_[\x83\x81\x10aE\xA1WPP\x94aD]\x91a\x0C\xE1\x95\x96\x81\x15_\x14aDbWP_\x90Pa!\xC2V[\x90\x92\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a.\x19W\x83_R\x84c\xFF\xFF\xFF\xFF\x83` _ \x01T\x16\x11_\x14aE\xD7WP\x92[\x90aE}V[\x93\x91P`\x01\x81\x01\x80\x91\x11a.\x19W\x90aE\xD1V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aE\xFFWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\x0C\xE1\x90aF\xA0a\x04\x89`\x02\x80`fT\x16\x14\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9A` R`@\x90 ``\x92\x91\x90aF\xC5\x90a\x04\xAEV[\x90aF\xF3aF\xE3\x82`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x90\x82\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv_\x80\xA3aG.\x81a.+V[\x91\x90\x94\x85Q\x90\x81\x15aG\xE1WPaGD\x90a&\xF3V[\x92aGP\x86\x82\x84a2\xC1V[\x91_[\x87Q\x81\x10\x15aG\xD7W`\x01\x90aG\xC6\x89aGka*\xC7V[aGsa*\xC7V[\x90aG\x95aG\x8Ca\x1C\xB6\x87aG\x86a*\xC7V[\x96a'\xD3V[a/\x8F\x83a'\xC6V[aG\x9F\x85\x8Ba'\xD3V[QaG\xA9\x83a'\xC6V[RaG\xB4\x85\x8Aa'\xD3V[QaG\xBE\x84a'\xC6V[R\x87\x87a5\xBDV[aG\xD0\x82\x89a'\xD3V[R\x01aGSV[P\x93\x95PPPPPV[\x95PPPPPV[\x15aG\xF0WV[c\rLL\x91`\xE2\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x91\x94\x91\x16\x92\x90\x83\x15aH\xA0Wa\x03\x8D\x94aH\x96\x91\x85_R`\x9C` R`@_ \x81_R` RaHZaHU`\xFF`@_ T\x16\x15\x15\x15\x90V[aG\xE9V[aH\x88a8\xBF\x82aH{\x89`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R`@_ \x90V[\x90_R` R`@_ \x90V[\x85` \x85\x01\x95\x86Q\x93a&3V[\x90Q\x91Q\x92aN\nV[PPPPPV[Q\x80a\x0C\xE1WPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[`\x01`\x01`\xA0\x1B\x03\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x03aI\nW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03aI\x82W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`\n` `@QaI\x94`@\x82a\x0C\x1AV[\x82\x81R\x01i\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B\x81R `@Q` \x81\x01\x91\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x83R`@\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\x80\x81Ra&\xD7`\xA0\x82a\x0C\x1AV[\x90\x80\x15aJ\x04Wa\x0C\xE1\x91aL\xBBV[PP_\x90V[\x90\x91\x90s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xBF\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01aJ7WPPPV[aJ\x8D\x90`\x01\x80`\xA0\x1B\x03\x16\x92\x83_R`\xA5` RaJlaC\xC4\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x93_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x90\x82\x01\x80\x92\x11a.\x19Wa\x03\x8D\x91`\x01`\x01`\xE0\x1B\x03\x16\x90Cc\xFF\xFF\xFF\xFF\x16\x90aQoV[\x92\x90\x91\x82\x15aJ\xFAWaJ\xD4\x82aB\x05aJ\xCEa\x1B\xDE\x88a+jV[\x86aLAV[\x90\x80\x82\x01\x80\x92\x11a.\x19W\x83\x01\x80\x93\x11a.\x19Wa4\x08\x92aJ\xF5\x91aL\xBBV[aL\xBBV[Pa4\x08\x91PaM[V[\x80T\x82\x10\x15a\x153W_R` _ \x01\x90_\x90V[\x90aK$\x91aK\x05V[\x90T\x90`\x03\x1B\x1C\x90V[\x91aKYc\xFF\xFF\xFF\xFF\x91`@\x93\x96\x95\x96`\x01\x80`\xA0\x1B\x03\x16\x85R``` \x86\x01R``\x85\x01\x90a\x0E{V[\x94\x16\x91\x01RV[\x93\x92\x90\x91_\x81aKp\x81Qa&\xF3V[\x94aK\x8F`@Q\x95\x86\x93\x84\x93c%5\xF4\x03`\xE2\x1B\x85R`\x04\x85\x01aK.V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06rW_\x92aK\xFBW[P_[\x81Q\x81\x10\x15a3oW\x80aK\xEAa3Ja\x1C\xB6`\x01\x94\x86a'\xD3V[aK\xF4\x82\x87a'\xD3V[R\x01aK\xCEV[aL\x10\x91\x92P=\x80_\x83>a\x15%\x81\x83a\x0C\x1AV[\x90_aK\xCBV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x15aL5W\x04\x90V[aL\x17V[\x15a\x033WV[_\x19\x82\x82\t\x82\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14aL\xAAW\x81g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x033W\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x93g\r\xE0\xB6\xB3\xA7d\0\0\x91\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x92PP\x04\x90V[_\x19g\r\xE0\xB6\xB3\xA7d\0\0\x82\t\x91g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x82\x80\x85\x10\x94\x03\x93\x80\x85\x03\x94\x14aMOWg\r\xE0\xB6\xB3\xA7d\0\0\x82\x91aL\xFB\x86\x84\x11aL:V[\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x81`\x03\x02\x18\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x91\x02`\x02\x03\x02\x93`\x01\x83\x80_\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[P\x90a\x0C\xE1\x92PaL+V[\x80\x15aL5Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[\x90\x91_\x19\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14aM\x9FW\x90\x82\x91aL\xFB\x86\x84\x11aL:V[PP\x90a\x0C\xE1\x92PaL+V[\x80T\x90\x81aM\xBBWP_\x91\x90PV[\x81_\x19\x81\x01\x11a.\x19W_R_\x19\x90` _ \x01\x01T` \x1CaD\x87V[\x91`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01`@\x1B\x03\x82\x11a.\x19W`\x01`\x01`@\x1B\x03a\x0C\xE1\x92\x16\x90aLAV[\x92B\x11aN0WaN\x1A\x92aPFV[\x15aN!WV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[c\x08\x19\xBD\xCD`\xE0\x1B_R`\x04_\xFD[\x91\x90aNL\x82\x82\x85aMuV[\x92\x82\x15aL5W\taN[W\x90V[`\x01\x81\x01\x80\x91\x11a.\x19W\x90V[`\x01\x81\x01\x90\x82_R\x81` R`@_ T\x15_\x14aN\xCCW\x80T`\x01`@\x1B\x81\x10\x15a\x0B\xFAWaN\xB9aN\xA3\x82`\x01\x87\x94\x01\x85U\x84aK\x05V[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90UT\x91_R` R`@_ U`\x01\x90V[PPP_\x90V[\x80T\x80\x15aN\xFAW_\x19\x01\x90aN\xE9\x82\x82aK\x05V[\x81T\x90_\x19\x90`\x03\x1B\x1B\x19\x16\x90UUV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`\x01\x81\x01\x91\x80_R\x82` R`@_ T\x92\x83\x15\x15_\x14aO\x9FW_\x19\x84\x01\x84\x81\x11a.\x19W\x83T_\x19\x81\x01\x94\x90\x85\x11a.\x19W_\x95\x85\x83aH{\x94aO\\\x98\x03aObW[PPPaN\xD3V[U`\x01\x90V[aO\x88aO\x82\x91aOyaK$aO\x96\x95\x88aK\x05V[\x92\x83\x91\x87aK\x05V[\x90a3\xEFV[\x85\x90_R` R`@_ \x90V[U_\x80\x80aOTV[PPPP_\x90V[`\x05\x11\x15aO\xB1WV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90``\x92` \x91\x83R`@\x82\x84\x01R\x80Q\x91\x82\x91\x82`@\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[=\x15aP!W=\x90aP\x08\x82a\x19\x12V[\x91aP\x16`@Q\x93\x84a\x0C\x1AV[\x82R=_` \x84\x01>V[``\x90V[\x90\x81` \x91\x03\x12a\x033WQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x033W\x90V[\x91\x90\x91aPS\x82\x84aRHV[aP\\\x81aO\xA7V[\x15\x90\x81aP\xEDW[PaP\xE5W_\x92a(\xCEaP\x91\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01aO\xC5V[Q\x91Z\xFAaP\x9DaO\xF7V[\x81aP\xD9W[\x81aP\xACWP\x90V[\x80Qc\x0B\x13]?`\xE1\x1B\x92P`\x01`\x01`\xE0\x1B\x03\x19\x91aP\xD4\x91\x81\x01` \x90\x81\x01\x91\x01aP&V[\x16\x14\x90V[\x80Q` \x14\x91PaP\xA3V[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_aPdV[\x15aQ\nWV[c\x15\x1B\x8E?`\xE1\x1B_R`\x04_\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x0B\xFAWaQ6\x91`\x01\x82\x01\x81UaK\x05V[aQ\\W\x81Q` \x92\x83\x01Q\x90\x92\x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80T\x80aQ\xABW[PaQ\xA6a\x03\x8D\x93aQ\x96aQ\x8Aa\x0CJV[c\xFF\xFF\xFF\xFF\x90\x95\x16\x85RV[`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01RV[aQ\x19V[\x80_\x19\x81\x01\x11a.\x19W\x81_Rc\xFF\xFF\xFF\xFFaR\x19aR\x10_\x19\x84` _ \x01\x01aR\x06aQ\xF8`@Q\x92aQ\xDF\x84a\x0B\xDFV[T\x86\x81\x16\x90\x81\x85R` \x1C` \x85\x01Rc\xFF\xFF\xFF\xFF\x16\x90V[\x85\x89\x16\x95\x86\x91\x16\x11\x15aQ\x03V[Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[\x03aQwWa\x03\x8D\x93\x92P\x90aDuaR1\x92aB\nV[\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV[\x81Q`A\x81\x03aRpWP\x90a+L\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aR\xB2V[`@\x03aR\xA9W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a.\x19Wa+L\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90aR\xB2V[PP_\x90`\x02\x90V[\x92\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11aSPW`\xFF\x16`\x1B\x81\x14\x15\x80aSEW[aS:W` \x93_\x93`@Q\x93\x84\x93`\x80\x85\x01\x93\x85R\x87\x85\x01R`@\x84\x01R``\x83\x01R\x83\x80R\x03\x90`\x01Z\xFA\x15a\x06rW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aS2W\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x81\x14\x15aR\xEAV[PPPP_\x90`\x03\x90V\xFE\xA2dipfsX\"\x12 \xCClK\xE3,\xE2j\xE0\xA9\xCC\xD9=\xAA5\x12!\xA5\xD8sx\xAB\x0C\x9B\x1Ff\xD6\x06]86\xCF<dsolcC\0\x08\x1B\x003",
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyStrategyManagerOrEigenPodManager> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyStrategyManagerOrEigenPodManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyStrategyManagerOrEigenPodManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyStrategyManagerOrEigenPodManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorsCannotUndelegate> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorsCannotUndelegate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorsCannotUndelegate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorsCannotUndelegate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<WithdrawalDelayNotElapsed> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalDelayNotElapsed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawalDelayNotElapsed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalDelayNotElapsed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegationApproverUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    119u8, 59u8, 84u8, 192u8, 77u8, 117u8, 111u8, 204u8, 94u8, 103u8, 129u8, 17u8,
                    247u8, 215u8, 48u8, 222u8, 59u8, 233u8, 129u8, 146u8, 0u8, 7u8, 153u8, 238u8,
                    227u8, 214u8, 55u8, 22u8, 5u8, 90u8, 135u8, 198u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &DelegationApproverUpdated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DepositScalingFactorUpdated(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 233u8, 50u8, 186u8, 197u8, 69u8, 97u8, 242u8, 114u8, 96u8, 249u8, 84u8,
                    99u8, 217u8, 184u8, 171u8, 55u8, 224u8, 107u8, 40u8, 66u8, 229u8, 238u8, 36u8,
                    4u8, 21u8, 124u8, 193u8, 61u8, 246u8, 235u8, 143u8,
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &DepositScalingFactorUpdated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorMetadataURIUpdated(address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    2u8, 169u8, 25u8, 237u8, 14u8, 42u8, 202u8, 209u8, 221u8, 144u8, 241u8, 126u8,
                    242u8, 250u8, 74u8, 229u8, 70u8, 46u8, 225u8, 51u8, 145u8, 112u8, 3u8, 74u8,
                    133u8, 49u8, 204u8, 164u8, 182u8, 112u8, 128u8, 144u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &OperatorMetadataURIUpdated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    164u8, 83u8, 219u8, 97u8, 42u8, 245u8, 158u8, 85u8, 33u8, 214u8, 171u8, 146u8,
                    132u8, 220u8, 62u8, 45u8, 6u8, 175u8, 40u8, 110u8, 177u8, 177u8, 183u8, 183u8,
                    113u8, 252u8, 228u8, 113u8, 108u8, 25u8, 242u8, 193u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorSharesBurned(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    239u8, 246u8, 170u8, 178u8, 188u8, 63u8, 124u8, 100u8, 136u8, 150u8, 225u8,
                    82u8, 46u8, 174u8, 113u8, 214u8, 194u8, 46u8, 59u8, 14u8, 33u8, 130u8, 6u8,
                    179u8, 244u8, 10u8, 240u8, 228u8, 210u8, 4u8, 113u8, 107u8,
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
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.shares,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "OperatorSharesDecreased(address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    105u8, 9u8, 96u8, 0u8, 55u8, 183u8, 93u8, 123u8, 71u8, 51u8, 174u8, 221u8,
                    129u8, 84u8, 66u8, 181u8, 236u8, 1u8, 138u8, 130u8, 119u8, 81u8, 200u8, 50u8,
                    170u8, 255u8, 100u8, 235u8, 165u8, 214u8, 210u8, 221u8,
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.shares,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &OperatorSharesDecreased) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "OperatorSharesIncreased(address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    30u8, 192u8, 66u8, 201u8, 101u8, 226u8, 237u8, 215u8, 16u8, 123u8, 81u8, 24u8,
                    142u8, 224u8, 243u8, 131u8, 226u8, 46u8, 118u8, 23u8, 144u8, 65u8, 171u8, 58u8,
                    157u8, 24u8, 255u8, 21u8, 20u8, 5u8, 22u8, 108u8,
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.shares,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &OperatorSharesIncreased) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlashingWithdrawalCompleted(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    31u8, 64u8, 64u8, 8u8, 137u8, 39u8, 78u8, 208u8, 123u8, 36u8, 132u8, 94u8,
                    80u8, 84u8, 168u8, 122u8, 12u8, 171u8, 150u8, 158u8, 177u8, 39u8, 122u8, 175u8,
                    230u8, 26u8, 227u8, 82u8, 231u8, 195u8, 42u8, 0u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &SlashingWithdrawalCompleted) -> alloy_sol_types::private::LogData {
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
        pub withdrawal:
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sharesToWithdraw:
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SlashingWithdrawalQueued {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IDelegationManagerTypes::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlashingWithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]),uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    38u8, 178u8, 170u8, 226u8, 101u8, 22u8, 232u8, 113u8, 158u8, 245u8, 14u8,
                    162u8, 246u8, 131u8, 26u8, 46u8, 251u8, 212u8, 227u8, 125u8, 204u8, 223u8,
                    15u8, 105u8, 54u8, 178u8, 123u8, 192u8, 142u8, 121u8, 62u8, 48u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &SlashingWithdrawalQueued) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerDelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    195u8, 238u8, 159u8, 46u8, 95u8, 218u8, 152u8, 232u8, 6u8, 106u8, 31u8, 116u8,
                    91u8, 45u8, 249u8, 40u8, 95u8, 65u8, 111u8, 233u8, 140u8, 242u8, 85u8, 156u8,
                    210u8, 20u8, 132u8, 179u8, 216u8, 116u8, 51u8, 4u8,
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
                    self.staker.clone(),
                    self.operator.clone(),
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerForceUndelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    240u8, 237u8, 223u8, 7u8, 230u8, 234u8, 20u8, 243u8, 136u8, 180u8, 126u8, 30u8,
                    148u8, 160u8, 244u8, 100u8, 236u8, 189u8, 158u8, 237u8, 65u8, 113u8, 19u8,
                    14u8, 15u8, 192u8, 233u8, 159u8, 180u8, 3u8, 10u8, 138u8,
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
                    self.staker.clone(),
                    self.operator.clone(),
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerUndelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    254u8, 227u8, 9u8, 102u8, 162u8, 86u8, 183u8, 30u8, 20u8, 188u8, 14u8, 191u8,
                    201u8, 67u8, 21u8, 226u8, 142u8, 244u8, 169u8, 122u8, 113u8, 49u8, 169u8,
                    226u8, 183u8, 163u8, 16u8, 167u8, 58u8, 244u8, 70u8, 118u8,
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
                    self.staker.clone(),
                    self.operator.clone(),
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._MIN_WITHDRAWAL_DELAY,
                    ),
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DELEGATION_APPROVAL_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: DELEGATION_APPROVAL_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DELEGATION_APPROVAL_TYPEHASHCall {
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
            impl ::core::convert::From<DELEGATION_APPROVAL_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: DELEGATION_APPROVAL_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DELEGATION_APPROVAL_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DELEGATION_APPROVAL_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = DELEGATION_APPROVAL_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<beaconChainETHStrategyCall> for UnderlyingRustTuple<'_> {
                fn from(value: beaconChainETHStrategyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for beaconChainETHStrategyCall {
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
            impl ::core::convert::From<beaconChainETHStrategyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: beaconChainETHStrategyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for beaconChainETHStrategyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for beaconChainETHStrategyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = beaconChainETHStrategyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnOperatorSharesCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnOperatorSharesCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnOperatorSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: burnOperatorSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnOperatorSharesReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnOperatorSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.prevMaxMagnitude,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.newMaxMagnitude,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateDelegationApprovalDigestHashCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateDelegationApprovalDigestHashCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateDelegationApprovalDigestHashReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculateDelegationApprovalDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculateDelegationApprovalDigestHashReturn
            {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateDelegationApprovalDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub withdrawal:
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingRustTuple<'a> =
                (<IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateWithdrawalRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateWithdrawalRootCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateWithdrawalRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
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
            impl ::core::convert::From<calculateWithdrawalRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculateWithdrawalRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateWithdrawalRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateWithdrawalRootCall {
            type Parameters<'a> = (IDelegationManagerTypes::Withdrawal,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateWithdrawalRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub withdrawal:
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeQueuedWithdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalCall) -> Self {
                    (value.withdrawal, value.tokens, value.receiveAsTokens)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeQueuedWithdrawalCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeQueuedWithdrawalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeQueuedWithdrawalReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeQueuedWithdrawals_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawals_0Call) -> Self {
                    (value.tokens, value.receiveAsTokens, value.numToComplete)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeQueuedWithdrawals_0Call {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeQueuedWithdrawals_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawals_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeQueuedWithdrawals_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeQueuedWithdrawals_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawals_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeQueuedWithdrawals_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawals_1Call) -> Self {
                    (value.withdrawals, value.tokens, value.receiveAsTokens)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeQueuedWithdrawals_1Call {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeQueuedWithdrawals_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawals_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeQueuedWithdrawals_1Return {
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawals_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cumulativeWithdrawalsQueuedCall> for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeWithdrawalsQueuedCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cumulativeWithdrawalsQueuedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<cumulativeWithdrawalsQueuedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeWithdrawalsQueuedReturn) -> Self {
                    (value.totalQueued,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cumulativeWithdrawalsQueuedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        totalQueued: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cumulativeWithdrawalsQueuedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = cumulativeWithdrawalsQueuedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<decreaseDelegatedSharesCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decreaseDelegatedSharesCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<decreaseDelegatedSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: decreaseDelegatedSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decreaseDelegatedSharesReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = decreaseDelegatedSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.curDepositShares,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.beaconChainSlashingFactorDecrease,
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
    /**Function with signature `delegateTo(address,(bytes,uint256),bytes32)` and selector `0xeea9064b`.
    ```solidity
    function delegateTo(address operator, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToCall {
        pub operator: alloy::sol_types::private::Address,
        pub approverSignatureAndExpiry:
            <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegatedToReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationApproverCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationApproverCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationApproverReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationApproverCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationApproverReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationApproverSaltIsSpentCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverSaltIsSpentCall) -> Self {
                    (value.delegationApprover, value.salt)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationApproverSaltIsSpentCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationApproverSaltIsSpentReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverSaltIsSpentReturn) -> Self {
                    (value.spent,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationApproverSaltIsSpentReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationApproverSaltIsSpentReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositScalingFactorCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositScalingFactorCall) -> Self {
                    (value.staker, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositScalingFactorCall {
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
            impl ::core::convert::From<depositScalingFactorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositScalingFactorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositScalingFactorReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositScalingFactorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<domainSeparatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for domainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = domainSeparatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eigenPodManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenPodManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub _1:
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
            impl ::core::convert::From<getDepositedSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getDepositedSharesCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDepositedSharesCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDepositedSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getDepositedSharesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDepositedSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDepositedSharesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDepositedSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`getOperatorShares(address,address[])`](getOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSharesReturn {
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
            impl ::core::convert::From<getOperatorSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSharesCall) -> Self {
                    (value.operator, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSharesCall {
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
            impl ::core::convert::From<getOperatorSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSharesReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSharesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`getOperatorsShares(address[],address[])`](getOperatorsSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorsSharesReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorsSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorsSharesCall) -> Self {
                    (value.operators, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorsSharesCall {
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorsSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorsSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorsSharesReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorsSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQueuedWithdrawalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalsCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQueuedWithdrawalsCall {
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQueuedWithdrawalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalsReturn) -> Self {
                    (value.withdrawals, value.shares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQueuedWithdrawalsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQueuedWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getSlashableSharesInQueueCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSlashableSharesInQueueCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlashableSharesInQueueCall {
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
            impl ::core::convert::From<getSlashableSharesInQueueReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSlashableSharesInQueueReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlashableSharesInQueueReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlashableSharesInQueueReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`getWithdrawableShares(address,address[])`](getWithdrawableSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawableSharesReturn {
        pub withdrawableShares:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        pub depositShares:
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
            impl ::core::convert::From<getWithdrawableSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawableSharesCall) -> Self {
                    (value.staker, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getWithdrawableSharesCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getWithdrawableSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawableSharesReturn) -> Self {
                    (value.withdrawableShares, value.depositShares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getWithdrawableSharesReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawableSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<increaseDelegatedSharesCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for increaseDelegatedSharesCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<increaseDelegatedSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: increaseDelegatedSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for increaseDelegatedSharesReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = increaseDelegatedSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "increaseDelegatedShares(address,address,uint256,uint256)";
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.prevDepositShares,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.addedShares,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.initialPausedStatus,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isDelegatedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minWithdrawalDelayBlocksCall> for UnderlyingRustTuple<'_> {
                fn from(value: minWithdrawalDelayBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minWithdrawalDelayBlocksCall {
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
            impl ::core::convert::From<minWithdrawalDelayBlocksReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minWithdrawalDelayBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minWithdrawalDelayBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minWithdrawalDelayBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = minWithdrawalDelayBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<modifyOperatorDetailsCall> for UnderlyingRustTuple<'_> {
                fn from(value: modifyOperatorDetailsCall) -> Self {
                    (value.operator, value.newDelegationApprover)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyOperatorDetailsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<modifyOperatorDetailsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: modifyOperatorDetailsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyOperatorDetailsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyOperatorDetailsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            impl ::core::convert::From<operatorSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorSharesReturn) -> Self {
                    (value.shares,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorSharesReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSharesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pendingWithdrawalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: pendingWithdrawalsCall) -> Self {
                    (value.withdrawalRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawalRoot: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pendingWithdrawalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pendingWithdrawalsReturn) -> Self {
                    (value.pending,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pending: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pendingWithdrawalsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<permissionControllerCall> for UnderlyingRustTuple<'_> {
                fn from(value: permissionControllerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permissionControllerCall {
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
            impl ::core::convert::From<permissionControllerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: permissionControllerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permissionControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permissionControllerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = permissionControllerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::QueuedWithdrawalParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<queueWithdrawalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<queueWithdrawalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queueWithdrawalsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::QueuedWithdrawalParams>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = queueWithdrawalsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    IDelegationManagerTypes::QueuedWithdrawalParams,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.params
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<queuedWithdrawalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: queuedWithdrawalsCall) -> Self {
                    (value.withdrawalRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queuedWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawalRoot: tuple.0,
                    }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<queuedWithdrawalsReturn> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queuedWithdrawalsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = queuedWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub newOperatorApproverSig:
            <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`redelegate(address,(bytes,uint256),bytes32)`](redelegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redelegateReturn {
        pub withdrawalRoots:
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    (
                        value.newOperator,
                        value.newOperatorApproverSig,
                        value.approverSalt,
                    )
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
            impl ::core::convert::From<redelegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: redelegateReturn) -> Self {
                    (value.withdrawalRoots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for redelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawalRoots: tuple.0,
                    }
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = redelegateReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerAsOperatorCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerAsOperatorCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerAsOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerAsOperatorReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerAsOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.allocationDelay,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        pub withdrawalRoots:
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<undelegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: undelegateReturn) -> Self {
                    (value.withdrawalRoots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for undelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawalRoots: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for undelegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = undelegateReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorMetadataURICall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorMetadataURICall) -> Self {
                    (value.operator, value.metadataURI)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorMetadataURICall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorMetadataURIReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorMetadataURIReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
                -> alloy_sol_types::Result<DelegationManagerCalls>] = &[
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
                            data, validate,
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
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <isOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <redelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn undelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <undelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                        <delegateToCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::unpause)
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
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
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
                Self::FullySlashed(_) => <FullySlashed as alloy_sol_types::SolError>::SELECTOR,
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
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyStrategyManagerOrEigenPodManager(_) => {
                    <OnlyStrategyManagerOrEigenPodManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
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
                -> alloy_sol_types::Result<DelegationManagerErrors>] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <FullySlashed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <SaltSpent as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(DelegationManagerErrors::WithdrawalDelayNotElapsed)
                    }
                    WithdrawalDelayNotElapsed
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
                2u8, 169u8, 25u8, 237u8, 14u8, 42u8, 202u8, 209u8, 221u8, 144u8, 241u8, 126u8,
                242u8, 250u8, 74u8, 229u8, 70u8, 46u8, 225u8, 51u8, 145u8, 112u8, 3u8, 74u8, 133u8,
                49u8, 204u8, 164u8, 182u8, 112u8, 128u8, 144u8,
            ],
            [
                30u8, 192u8, 66u8, 201u8, 101u8, 226u8, 237u8, 215u8, 16u8, 123u8, 81u8, 24u8,
                142u8, 224u8, 243u8, 131u8, 226u8, 46u8, 118u8, 23u8, 144u8, 65u8, 171u8, 58u8,
                157u8, 24u8, 255u8, 21u8, 20u8, 5u8, 22u8, 108u8,
            ],
            [
                31u8, 64u8, 64u8, 8u8, 137u8, 39u8, 78u8, 208u8, 123u8, 36u8, 132u8, 94u8, 80u8,
                84u8, 168u8, 122u8, 12u8, 171u8, 150u8, 158u8, 177u8, 39u8, 122u8, 175u8, 230u8,
                26u8, 227u8, 82u8, 231u8, 195u8, 42u8, 0u8,
            ],
            [
                38u8, 178u8, 170u8, 226u8, 101u8, 22u8, 232u8, 113u8, 158u8, 245u8, 14u8, 162u8,
                246u8, 131u8, 26u8, 46u8, 251u8, 212u8, 227u8, 125u8, 204u8, 223u8, 15u8, 105u8,
                54u8, 178u8, 123u8, 192u8, 142u8, 121u8, 62u8, 48u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                105u8, 9u8, 96u8, 0u8, 55u8, 183u8, 93u8, 123u8, 71u8, 51u8, 174u8, 221u8, 129u8,
                84u8, 66u8, 181u8, 236u8, 1u8, 138u8, 130u8, 119u8, 81u8, 200u8, 50u8, 170u8,
                255u8, 100u8, 235u8, 165u8, 214u8, 210u8, 221u8,
            ],
            [
                119u8, 59u8, 84u8, 192u8, 77u8, 117u8, 111u8, 204u8, 94u8, 103u8, 129u8, 17u8,
                247u8, 215u8, 48u8, 222u8, 59u8, 233u8, 129u8, 146u8, 0u8, 7u8, 153u8, 238u8,
                227u8, 214u8, 55u8, 22u8, 5u8, 90u8, 135u8, 198u8,
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
                139u8, 233u8, 50u8, 186u8, 197u8, 69u8, 97u8, 242u8, 114u8, 96u8, 249u8, 84u8,
                99u8, 217u8, 184u8, 171u8, 55u8, 224u8, 107u8, 40u8, 66u8, 229u8, 238u8, 36u8, 4u8,
                21u8, 124u8, 193u8, 61u8, 246u8, 235u8, 143u8,
            ],
            [
                164u8, 83u8, 219u8, 97u8, 42u8, 245u8, 158u8, 85u8, 33u8, 214u8, 171u8, 146u8,
                132u8, 220u8, 62u8, 45u8, 6u8, 175u8, 40u8, 110u8, 177u8, 177u8, 183u8, 183u8,
                113u8, 252u8, 228u8, 113u8, 108u8, 25u8, 242u8, 193u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                195u8, 238u8, 159u8, 46u8, 95u8, 218u8, 152u8, 232u8, 6u8, 106u8, 31u8, 116u8,
                91u8, 45u8, 249u8, 40u8, 95u8, 65u8, 111u8, 233u8, 140u8, 242u8, 85u8, 156u8,
                210u8, 20u8, 132u8, 179u8, 216u8, 116u8, 51u8, 4u8,
            ],
            [
                239u8, 246u8, 170u8, 178u8, 188u8, 63u8, 124u8, 100u8, 136u8, 150u8, 225u8, 82u8,
                46u8, 174u8, 113u8, 214u8, 194u8, 46u8, 59u8, 14u8, 33u8, 130u8, 6u8, 179u8, 244u8,
                10u8, 240u8, 228u8, 210u8, 4u8, 113u8, 107u8,
            ],
            [
                240u8, 237u8, 223u8, 7u8, 230u8, 234u8, 20u8, 243u8, 136u8, 180u8, 126u8, 30u8,
                148u8, 160u8, 244u8, 100u8, 236u8, 189u8, 158u8, 237u8, 65u8, 113u8, 19u8, 14u8,
                15u8, 192u8, 233u8, 159u8, 180u8, 3u8, 10u8, 138u8,
            ],
            [
                254u8, 227u8, 9u8, 102u8, 162u8, 86u8, 183u8, 30u8, 20u8, 188u8, 14u8, 191u8,
                201u8, 67u8, 21u8, 226u8, 142u8, 244u8, 169u8, 122u8, 113u8, 49u8, 169u8, 226u8,
                183u8, 163u8, 16u8, 167u8, 58u8, 244u8, 70u8, 118u8,
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
                Some(<DelegationApproverUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DelegationApproverUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::DelegationApproverUpdated)
                }
                Some(
                    <DepositScalingFactorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <DepositScalingFactorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::DepositScalingFactorUpdated),
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<OperatorMetadataURIUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorMetadataURIUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorMetadataURIUpdated)
                }
                Some(<OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorRegistered)
                }
                Some(<OperatorSharesBurned as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSharesBurned as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSharesBurned)
                }
                Some(<OperatorSharesDecreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSharesDecreased as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSharesDecreased)
                }
                Some(<OperatorSharesIncreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSharesIncreased as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSharesIncreased)
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
                Some(
                    <SlashingWithdrawalCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <SlashingWithdrawalCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::SlashingWithdrawalCompleted),
                Some(<SlashingWithdrawalQueued as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlashingWithdrawalQueued as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::SlashingWithdrawalQueued)
                }
                Some(<StakerDelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakerDelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StakerDelegated)
                }
                Some(<StakerForceUndelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakerForceUndelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StakerForceUndelegated)
                }
                Some(<StakerUndelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakerUndelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StakerUndelegated)
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
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<DelegationManagerInstance<T, P, N>>>
    {
        DelegationManagerInstance::<T, P, N>::deploy(
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
        DelegationManagerInstance::<T, P, N>::deploy_builder(
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
            f.debug_tuple("DelegationManagerInstance")
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
        > DelegationManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`DelegationManager`](self) contract instance.

        See the [wrapper's documentation](`DelegationManagerInstance`) for more details.*/
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _strategyManager,
                        _eigenPodManager,
                        _allocationManager,
                        _pauserRegistry,
                        _permissionController,
                        _MIN_WITHDRAWAL_DELAY,
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
        > DelegationManagerInstance<T, P, N>
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
        ///Creates a new call builder for the [`DELEGATION_APPROVAL_TYPEHASH`] function.
        pub fn DELEGATION_APPROVAL_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DELEGATION_APPROVAL_TYPEHASHCall, N> {
            self.call_builder(&DELEGATION_APPROVAL_TYPEHASHCall {})
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
            self.call_builder(&burnOperatorSharesCall {
                operator,
                strategy,
                prevMaxMagnitude,
                newMaxMagnitude,
            })
        }
        ///Creates a new call builder for the [`calculateDelegationApprovalDigestHash`] function.
        pub fn calculateDelegationApprovalDigestHash(
            &self,
            staker: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
            approver: alloy::sol_types::private::Address,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateDelegationApprovalDigestHashCall, N>
        {
            self.call_builder(&calculateDelegationApprovalDigestHashCall {
                staker,
                operator,
                approver,
                approverSalt,
                expiry,
            })
        }
        ///Creates a new call builder for the [`calculateWithdrawalRoot`] function.
        pub fn calculateWithdrawalRoot(
            &self,
            withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateWithdrawalRootCall, N> {
            self.call_builder(&calculateWithdrawalRootCall { withdrawal })
        }
        ///Creates a new call builder for the [`completeQueuedWithdrawal`] function.
        pub fn completeQueuedWithdrawal(
            &self,
            withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
            tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            receiveAsTokens: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeQueuedWithdrawalCall, N> {
            self.call_builder(&completeQueuedWithdrawalCall {
                withdrawal,
                tokens,
                receiveAsTokens,
            })
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
            self.call_builder(&completeQueuedWithdrawals_0Call {
                tokens,
                receiveAsTokens,
                numToComplete,
            })
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
            self.call_builder(&completeQueuedWithdrawals_1Call {
                withdrawals,
                tokens,
                receiveAsTokens,
            })
        }
        ///Creates a new call builder for the [`cumulativeWithdrawalsQueued`] function.
        pub fn cumulativeWithdrawalsQueued(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, cumulativeWithdrawalsQueuedCall, N> {
            self.call_builder(&cumulativeWithdrawalsQueuedCall { staker })
        }
        ///Creates a new call builder for the [`decreaseDelegatedShares`] function.
        pub fn decreaseDelegatedShares(
            &self,
            staker: alloy::sol_types::private::Address,
            curDepositShares: alloy::sol_types::private::primitives::aliases::U256,
            beaconChainSlashingFactorDecrease: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, decreaseDelegatedSharesCall, N> {
            self.call_builder(&decreaseDelegatedSharesCall {
                staker,
                curDepositShares,
                beaconChainSlashingFactorDecrease,
            })
        }
        ///Creates a new call builder for the [`delegateTo`] function.
        pub fn delegateTo(
            &self,
            operator: alloy::sol_types::private::Address,
            approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateToCall, N> {
            self.call_builder(&delegateToCall {
                operator,
                approverSignatureAndExpiry,
                approverSalt,
            })
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
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationApproverSaltIsSpentCall, N> {
            self.call_builder(&delegationApproverSaltIsSpentCall {
                delegationApprover,
                salt,
            })
        }
        ///Creates a new call builder for the [`depositScalingFactor`] function.
        pub fn depositScalingFactor(
            &self,
            staker: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositScalingFactorCall, N> {
            self.call_builder(&depositScalingFactorCall { staker, strategy })
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
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSharesCall, N> {
            self.call_builder(&getOperatorSharesCall {
                operator,
                strategies,
            })
        }
        ///Creates a new call builder for the [`getOperatorsShares`] function.
        pub fn getOperatorsShares(
            &self,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorsSharesCall, N> {
            self.call_builder(&getOperatorsSharesCall {
                operators,
                strategies,
            })
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
            self.call_builder(&getSlashableSharesInQueueCall { operator, strategy })
        }
        ///Creates a new call builder for the [`getWithdrawableShares`] function.
        pub fn getWithdrawableShares(
            &self,
            staker: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawableSharesCall, N> {
            self.call_builder(&getWithdrawableSharesCall { staker, strategies })
        }
        ///Creates a new call builder for the [`increaseDelegatedShares`] function.
        pub fn increaseDelegatedShares(
            &self,
            staker: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            prevDepositShares: alloy::sol_types::private::primitives::aliases::U256,
            addedShares: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, increaseDelegatedSharesCall, N> {
            self.call_builder(&increaseDelegatedSharesCall {
                staker,
                strategy,
                prevDepositShares,
                addedShares,
            })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                initialPausedStatus,
            })
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
            self.call_builder(&modifyOperatorDetailsCall {
                operator,
                newDelegationApprover,
            })
        }
        ///Creates a new call builder for the [`operatorShares`] function.
        pub fn operatorShares(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSharesCall, N> {
            self.call_builder(&operatorSharesCall { operator, strategy })
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
        ///Creates a new call builder for the [`pendingWithdrawals`] function.
        pub fn pendingWithdrawals(
            &self,
            withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, pendingWithdrawalsCall, N> {
            self.call_builder(&pendingWithdrawalsCall { withdrawalRoot })
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
            self.call_builder(&queuedWithdrawalsCall { withdrawalRoot })
        }
        ///Creates a new call builder for the [`redelegate`] function.
        pub fn redelegate(
            &self,
            newOperator: alloy::sol_types::private::Address,
            newOperatorApproverSig: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, redelegateCall, N> {
            self.call_builder(&redelegateCall {
                newOperator,
                newOperatorApproverSig,
                approverSalt,
            })
        }
        ///Creates a new call builder for the [`registerAsOperator`] function.
        pub fn registerAsOperator(
            &self,
            initDelegationApprover: alloy::sol_types::private::Address,
            allocationDelay: u32,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerAsOperatorCall, N> {
            self.call_builder(&registerAsOperatorCall {
                initDelegationApprover,
                allocationDelay,
                metadataURI,
            })
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
            self.call_builder(&updateOperatorMetadataURICall {
                operator,
                metadataURI,
            })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > DelegationManagerInstance<T, P, N>
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
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
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
        pub fn StakerDelegated_filter(&self) -> alloy_contract::Event<T, &P, StakerDelegated, N> {
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
