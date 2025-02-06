///Module containing a contract's types and functions.
/**

```solidity
library IDelegationManagerTypes {
    struct QueuedWithdrawalParams { address[] strategies; uint256[] depositShares; address __deprecated_withdrawer; }
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
struct QueuedWithdrawalParams { address[] strategies; uint256[] depositShares; address __deprecated_withdrawer; }
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
        pub __deprecated_withdrawer: alloy::sol_types::private::Address,
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
                (value.strategies, value.depositShares, value.__deprecated_withdrawer)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QueuedWithdrawalParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategies: tuple.0,
                    depositShares: tuple.1,
                    __deprecated_withdrawer: tuple.2,
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
                        &self.__deprecated_withdrawer,
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
                    "QueuedWithdrawalParams(address[] strategies,uint256[] depositShares,address __deprecated_withdrawer)",
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
                            &self.__deprecated_withdrawer,
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
                        &rust.__deprecated_withdrawer,
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
                    &rust.__deprecated_withdrawer,
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
        address __deprecated_withdrawer;
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
    error WithdrawalNotQueued();
    error WithdrawerNotCaller();

    event DelegationApproverUpdated(address indexed operator, address newDelegationApprover);
    event DepositScalingFactorUpdated(address staker, address strategy, uint256 newDepositScalingFactor);
    event Initialized(uint8 version);
    event OperatorMetadataURIUpdated(address indexed operator, string metadataURI);
    event OperatorRegistered(address indexed operator, address delegationApprover);
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
    function calculateDelegationApprovalDigestHash(address staker, address operator, address approver, bytes32 approverSalt, uint256 expiry) external view returns (bytes32);
    function calculateWithdrawalRoot(IDelegationManagerTypes.Withdrawal memory withdrawal) external pure returns (bytes32);
    function completeQueuedWithdrawal(IDelegationManagerTypes.Withdrawal memory withdrawal, address[] memory tokens, bool receiveAsTokens) external;
    function completeQueuedWithdrawals(IDelegationManagerTypes.Withdrawal[] memory withdrawals, address[][] memory tokens, bool[] memory receiveAsTokens) external;
    function convertToDepositShares(address staker, address[] memory strategies, uint256[] memory withdrawableShares) external view returns (uint256[] memory);
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
    function getQueuedWithdrawal(bytes32 withdrawalRoot) external view returns (IDelegationManagerTypes.Withdrawal memory);
    function getQueuedWithdrawalRoots(address staker) external view returns (bytes32[] memory);
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
    function redelegate(address newOperator, ISignatureUtils.SignatureWithExpiry memory newOperatorApproverSig, bytes32 approverSalt) external returns (bytes32[] memory withdrawalRoots);
    function registerAsOperator(address initDelegationApprover, uint32 allocationDelay, string memory metadataURI) external;
    function renounceOwnership() external;
    function slashOperatorShares(address operator, address strategy, uint64 prevMaxMagnitude, uint64 newMaxMagnitude) external;
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
    "name": "convertToDepositShares",
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
      },
      {
        "name": "withdrawableShares",
        "type": "uint256[]",
        "internalType": "uint256[]"
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
    "name": "getQueuedWithdrawal",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQueuedWithdrawalRoots",
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
        "type": "bytes32[]",
        "internalType": "bytes32[]"
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
            "name": "__deprecated_withdrawer",
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
    "name": "slashOperatorShares",
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
    "name": "WithdrawalNotQueued",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawerNotCaller",
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
    ///0x61018080604052346103145760c08161554a80380380916100208285610318565b8339810103126103145780516001600160a01b03811681036103145760208201516001600160a01b0381168103610314576040830151906001600160a01b03821682036103145760608401516001600160a01b0381169390848103610314576080860151956001600160a01b03871687036103145760a001519463ffffffff8616860361031457156103055760805260a05260c05260e052610100524661012052604080519081016001600160401b038111828210176102f157600a91602091604052828152016922b4b3b2b72630bcb2b960b11b81522060405160208101917f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866835260408201524660608201523060808201526080815261014360a082610318565b51902061014052610160525f5460ff8160081c1661029c5760ff80821610610262575b60405161520e908161033c82396080518181816105c001528181610e4a01528181611a2401526123ae015260a051818181610b0f01528181610b7701528181612c5d0152614230015260c051818181610c7601528181610d9c015281816115c001528181612cdd01528181613b9c0152614203015260e0518181816109f001528181610bee01528181611471015281816117240152818161200c01528181612a4d015281816133d10152613e9c01526101005181818161128c015281816117c001528181611e3d01528181613fcf015261445b015261012051816131fd01526101405181613223015261016051818181610d580152613deb0152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f610166565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b634e487b7160e01b5f52604160045260245ffd5b6339b190bb60e11b5f5260045ffd5b5f80fd5b601f909101601f19168101906001600160401b038211908210176102f15760405256fe60806040526004361015610011575f80fd5b5f3560e01c806304a4f979146103345780630b9f487a1461032f5780630dd8dd021461032a578063136439dd1461032557806325df922e146103205780632aa6d8881461031b57806339b70e38146103165780633c651cf2146103115780633cdeb5e01461030c5780633e28391d146103075780634657e26a146103025780634665bcda146102fd57806354b7c96c146102f8578063595c6a67146102f3578063597b36da146102ee5780635ac86ab7146102e95780635c975abb146102e45780635d975e88146102df5780635dd68579146102da578063601bb36f146102d557806360a0d1ce146102d057806365da1264146102cb57806366d5ba93146102c65780636d70f7ae146102c15780636e174448146102bc578063715018a6146102b7578063778e55f3146102b257806378296ec5146102ad578063886f1195146102a85780638da5cb5b146102a3578063900413471461029e5780639104c319146102995780639435bb4314610294578063a17884841461028f578063a33a34331461028a578063b7f06ebe14610285578063bb45fef214610280578063bfae3fd21461027b578063c448feb814610276578063c978f7ac14610271578063ca8aa7c71461026c578063cd6dc68714610267578063da8be86414610262578063e4cc3f901461025d578063eea9064b14610258578063f0e0e67614610253578063f2fde38b1461024e578063f698da2514610249578063fabc1cbc146102445763fd8aa88d1461023f575f80fd5b612449565b612385565b61236b565b6122da565b612219565b6121f1565b612157565b612125565b61203b565b611ff7565b611e86565b611e21565b611ddb565b611d8d565b611d5e565b611d2b565b611c21565b611b0f565b611ae1565b611ab3565b611a53565b611a0f565b611980565b61193c565b6118e1565b6116e5565b611699565b611649565b611606565b611592565b61142d565b611209565b611081565b610fa5565b610f72565b610f38565b610e1f565b610dcb565b610d87565b610d43565b610cf8565b610ca4565b610b3e565b610afa565b610985565b610814565b610590565b610457565b61039f565b610347565b5f91031261034357565b5f80fd5b34610343575f3660031901126103435760206040517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad8152f35b6001600160a01b0381160361034357565b359061039d82610381565b565b346103435760a03660031901126103435760206103e66004356103c181610381565b6024356103cd81610381565b6044356103d981610381565b60643591608435936124be565b604051908152f35b9181601f84011215610343578235916001600160401b038311610343576020808501948460051b01011161034357565b60206040818301928281528451809452019201905f5b8181106104415750505090565b8251845260209384019390920191600101610434565b34610343576020366003190112610343576004356001600160401b038111610343576104879036906004016103ee565b9061049f610499600280606654161490565b15612568565b6104a88261257e565b335f908152609a60205260409020549092906001600160a01b03165f5b8281106104de57604051806104da878261041e565b0390f35b8061051b6104f86104f260019487896125c4565b806125e6565b905061051261050884888a6125c4565b60208101906125e6565b9190501461261b565b61057f6105406105396105326104f285898b6125c4565b36916106f5565b853361339f565b866105778761056f6105656105088861055d6104f282878a6125c4565b9590976125c4565b94909236916106f5565b923691610769565b9086336136b0565b610589828861263e565b52016104c5565b346103435760203660031901126103435760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561064b5761061a92610606915f9161061c575b50612672565b61061560665482811614612688565b6139c5565b005b61063e915060203d602011610644575b610636818361069f565b810190612652565b5f610600565b503d61062c565b612667565b634e487b7160e01b5f52604160045260245ffd5b60e081019081106001600160401b0382111761067f57604052565b610650565b604081019081106001600160401b0382111761067f57604052565b90601f801991011681019081106001600160401b0382111761067f57604052565b6040519061039d60e08361069f565b6040519061039d60408361069f565b6001600160401b03811161067f5760051b60200190565b929190610701816106de565b9361070f604051958661069f565b602085838152019160051b810192831161034357905b82821061073157505050565b60208091833561074081610381565b815201910190610725565b9080601f8301121561034357816020610766933591016106f5565b90565b929190610775816106de565b93610783604051958661069f565b602085838152019160051b810192831161034357905b8282106107a557505050565b8135815260209182019101610799565b9080601f830112156103435781602061076693359101610769565b90602080835192838152019201905f5b8181106107ed5750505090565b82518452602093840193909201916001016107e0565b9060206107669281815201906107d0565b346103435760603660031901126103435760043561083181610381565b6024356001600160401b0381116103435761085090369060040161074b565b906044356001600160401b038111610343576108709036906004016107b5565b6001600160a01b038281165f818152609a60205260409020549093610898928692169061339f565b6108a2845161257e565b935f5b815181101561092657600190855f5260a26020526109156108f96108f460405f206108e06108d3868961263e565b516001600160a01b031690565b60018060a01b03165f5260205260405f2090565b6126c0565b610903838861263e565b5161090e848861263e565b51916139f7565b61091f828961263e565b52016108a5565b604051806104da8882610803565b6024359063ffffffff8216820361034357565b359063ffffffff8216820361034357565b9181601f84011215610343578235916001600160401b038311610343576020838186019501011161034357565b34610343576060366003190112610343576004356109a281610381565b6109aa610934565b6044356001600160401b038111610343576109c9903690600401610958565b335f908152609a60205260409020549193916109ee906001600160a01b0316156126e3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561034357604051632b6241f360e11b815233600482015263ffffffff9490941660248501525f908490604490829084905af191821561064b577f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b670809093610adb93610ae0575b50610a8a8133613a13565b610a943333613a73565b6040516001600160a01b0391909116815233907fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c190602090a26040519182913395836126f9565b0390a2005b80610aee5f610af49361069f565b80610339565b5f610a7f565b34610343575f366003190112610343576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461034357608036600319011261034357600435610b5b81610381565b602435610b6781610381565b6064356044356001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001633148015610c72575b15610c63576001600160a01b038481165f908152609a602090815260409182902054915163152667d960e31b81529183166004830181905286841660248401529196919592879060449082907f0000000000000000000000000000000000000000000000000000000000000000165afa95861561064b5761061a96610c2e915f91610c34575b508383613b40565b94613d34565b610c56915060203d602011610c5c575b610c4e818361069f565b810190612720565b5f610c26565b503d610c44565b63045206a560e21b5f5260045ffd5b50337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614610ba0565b34610343576020366003190112610343576020610ce6600435610cc681610381565b6001600160a01b039081165f908152609960205260409020600101541690565b6040516001600160a01b039091168152f35b34610343576020366003190112610343576020610d39600435610d1a81610381565b6001600160a01b039081165f908152609a602052604090205416151590565b6040519015158152f35b34610343575f366003190112610343576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610343575f366003190112610343576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103435760403660031901126103435761061a600435610deb81610381565b60243590610df882610381565b610e09610e0482613da2565b612735565b610e1a610e1582612e1d565b61274b565b613a13565b34610343575f3660031901126103435760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561064b57610e8a915f9161061c5750612672565b61061a613991565b91909160e08184031261034357610ea76106c0565b92610eb182610392565b8452610ebf60208301610392565b6020850152610ed060408301610392565b604085015260608201356060850152610eeb60808301610947565b608085015260a08201356001600160401b0381116103435781610f0f91840161074b565b60a085015260c08201356001600160401b03811161034357610f3192016107b5565b60c0830152565b34610343576020366003190112610343576004356001600160401b038111610343576103e6610f6d6020923690600401610e92565b612761565b346103435760203660031901126103435760043560ff81168091036103435760016020911b806066541614604051908152f35b34610343575f366003190112610343576020606654604051908152f35b90602080835192838152019201905f5b818110610fdf5750505090565b82516001600160a01b0316845260209384019390920191600101610fd2565b80516001600160a01b039081168352602080830151821690840152604080830151909116908301526060808201519083015260808082015163ffffffff16908301526107669160c061105f60a084015160e060a085015260e0840190610fc2565b9201519060c08184039101526107d0565b906020610766928181520190610ffe565b346103435760203660031901126103435760043561109d61278c565b505f5260a46020526104da60405f206111336006604051926110be84610664565b80546001600160a01b0316845260018101546001600160a01b0316602085015260028101546001600160a01b031660408501526003810154606085015261111c61110f600483015463ffffffff1690565b63ffffffff166080860152565b611128600582016127c2565b60a085015201612813565b60c082015260405191829182611070565b9080602083519182815201916020808360051b8301019401925f915b83831061116f57505050505090565b909192939460208061118d600193601f1986820301875289516107d0565b97019301930191939290611160565b929160408401936040815282518095526060810194602060608260051b8401019401905f5b8181106111de575050506107669394506020818403910152611144565b9091946020806111fa600193605f19888203018c528951610ffe565b970198019101969190966111c1565b346103435760203660031901126103435760043561122681610381565b6001600160a01b0381165f90815260a36020526040902061124690612813565b805190611252826128e9565b9161125c81612938565b9361128761127a8260018060a01b03165f52609a60205260405f2090565b546001600160a01b031690565b915f927f00000000000000000000000000000000000000000000000000000000000000009263ffffffff4316905b8386106112cb57604051806104da8b8b8361119c565b6112f16112ec6112dd888a9c9a61263e565b515f5260a460205260405f2090565b61285b565b6112fb878a61263e565b52611306868961263e565b5061131f60a0611316888b61263e565b5101515161257e565b611329878961263e565b52611334868861263e565b506113598561135460806113488a8d61263e565b51015163ffffffff1690565b612995565b8263ffffffff8216105f146113fc576113829060a0611378898c61263e565b5101518584613e69565b945b5f5b60a0611392898c61263e565b510151518110156113eb5780896113e4826113de8c8f8d6113d1856113ca60019b60c06113c2886113d89861263e565b51015161263e565b519261263e565b5190614bc3565b9461263e565b5161263e565b5201611386565b5096989660019096019594506112b5565b5061141660a061140c888b61263e565b510151848361339f565b94611384565b6001600160401b0381160361034357565b346103435760803660031901126103435760043561144a81610381565b60243561145681610381565b6044356114628161141c565b6064359061146f8261141c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303611583578261151261150c611518946115046114e0856114cb8b60018060a01b03165f52609860205260405f2090565b9060018060a01b03165f5260205260405f2090565b546114fe6001600160401b0388166001600160401b03851683614a85565b90613f2e565b948489613f6d565b836129bd565b94614074565b611530611524826141de565b6001600160a01b031690565b91823b156103435760405163debe1eab60e01b81526001600160a01b039290921660048301526024820152905f908290604490829084905af1801561064b5761157557005b80610aee5f61061a9361069f565b6323d871a560e01b5f5260045ffd5b34610343576060366003190112610343576004356115af81610381565b6044356024356115be8261141c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036115f75761061a926129ca565b633213a66160e21b5f5260045ffd5b346103435760203660031901126103435760043561162381610381565b60018060a01b03165f52609a602052602060018060a01b0360405f205416604051908152f35b346103435760203660031901126103435761168b6104da61167460043561166f81610381565b612c34565b604092919251938493604085526040850190610fc2565b9083820360208501526107d0565b34610343576020366003190112610343576020610d396004356116bb81610381565b612e1d565b6040906003190112610343576004356116d881610381565b9060243561076681610381565b34610343576116f3366116c0565b60405163152667d960e31b81526001600160a01b0380841660048301528216602482015291602083806044810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa92831561064b575f936118bc575b506117b89060018060a01b031691825f5260a56020526117976117928260405f209060018060a01b03165f5260205260405f2090565b614ab4565b925f5260a560205260405f209060018060a01b03165f5260205260405f2090565b6117f06117eb7f000000000000000000000000000000000000000000000000000000000000000063ffffffff4316613f53565b613f3b565b8154919063ffffffff165f5b83811061186d576104da61182c5f886118278989898161183c5750506001600160e01b0384166114fe565b614ae1565b6040519081529081906020820190565b61185a6118619161184f6114fe94613f20565b905f5260205f200190565b5460201c90565b6001600160e01b031690565b90928082169080831860011c82018092116118b757835f528463ffffffff8360205f20015416115f146118a35750925b906117fc565b939150600181018091116118b7579061189d565b612981565b6117b89193506118da9060203d602011610c5c57610c4e818361069f565b929061175c565b34610343575f366003190112610343576118f9614277565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461034357602061197761194f366116c0565b6001600160a01b039182165f9081526098855260408082209290931681526020919091522090565b54604051908152f35b346103435760403660031901126103435760043561199d81610381565b6024356001600160401b038111610343576119dd7f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b6708090913690600401610958565b90926119eb610e0482613da2565b6119f7610e1582612e1d565b610adb60405192839260018060a01b031695836126f9565b34610343575f366003190112610343576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610343575f366003190112610343576033546040516001600160a01b039091168152602090f35b90604060031983011261034357600435611a9481610381565b91602435906001600160401b038211610343576107669160040161074b565b34610343576104da611acd611ac736611a7b565b90612e50565b6040519182916020835260208301906107d0565b34610343575f36600319011261034357602060405173beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac08152f35b34610343576060366003190112610343576004356001600160401b03811161034357611b3f9036906004016103ee565b6024356001600160401b03811161034357611b5e9036906004016103ee565b916044356001600160401b03811161034357611b819093919336906004016103ee565b90611b93610499600480606654161490565b611ba2600260c9541415612ec3565b600260c9553686900360de1901925f5b86811015611c17578060051b908189013591868312156103435783821015611c1257600192611be6611c0c928a018a6125e6565b90611c078d611bfe611bf9888d8d612f0f565b612f1f565b94369101610e92565b6143ba565b01611bb2565b6125b0565b61061a600160c955565b3461034357602036600319011261034357600435611c3e81610381565b60018060a01b03165f52609f602052602060405f2054604051908152f35b6001600160401b03811161067f57601f01601f191660200190565b90606060031983011261034357600435611c9081610381565b91602435906001600160401b03821161034357604082820360031901126103435760405191611cbe83610684565b80600401356001600160401b0381116103435781019180602384011215610343576004830135611ced81611c5c565b91611cfb604051938461069f565b81835260248583010111610343576020815f92602480970183860137830101528352013560208201529060443590565b34610343576104da611d52611d3f36611c77565b90611d4c93929333612f9d565b9361309c565b6040519182918261041e565b34610343576020366003190112610343576004355f52609e602052602060ff60405f2054166040519015158152f35b3461034357604036600319011261034357600435611daa81610381565b6024359060018060a01b03165f52609c60205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103435760206103e6611e1c6108f4611df4366116c0565b6001600160a01b039182165f90815260a2875260408082209290931681526020919091522090565b61473f565b34610343575f36600319011261034357602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b9091611e78610766936040845260408401906107d0565b9160208184039101526107d0565b3461034357611e9436611a7b565b611e9e815161257e565b611ea8825161257e565b91611ed081611eca61127a8760018060a01b03165f52609a60205260405f2090565b8661339f565b5f5b8251811015611fe557806020611ef9611524611ef46108d3611f3a968961263e565b6141de565b611f066108d3848861263e565b60405163fe243a1760e01b81526001600160a01b03808c166004830152909116602482015293849190829081906044820190565b03915afa801561064b576001925f91611fb7575b50611f59828861263e565b52611fa6611f8a6108f4611f7d8a60018060a01b03165f5260a260205260405f2090565b6108e06108d3868a61263e565b611f94838961263e565b51611f9f848761263e565b519161425b565b611fb0828761263e565b5201611ed2565b611fd8915060203d8111611fde575b611fd0818361069f565b810190612c25565b5f611f4e565b503d611fc6565b5050506104da60405192839283611e61565b34610343575f366003190112610343576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103435760403660031901126103435760043561205881610381565b61209d6024355f549261208360ff600886901c161580958196612117575b81156120f7575b50612f29565b83612094600160ff195f5416175f55565b6120e057612f8c565b6120a357005b6120b161ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b6120f261010061ff00195f5416175f55565b612f8c565b303b15915081612109575b505f61207d565b60ff1660011490505f612102565b600160ff8216109150612076565b34610343576020366003190112610343576104da611d5260043561214881610381565b612f9d565b8015150361034357565b34610343576060366003190112610343576004356001600160401b0381116103435760e0600319823603011261034357602435906001600160401b038211610343576121aa6121ea9236906004016103ee565b90611c07604435936121bb8561214d565b6121cc610499600480606654161490565b6121db600260c9541415612ec3565b600260c9553690600401610e92565b600160c955005b346103435761061a61220236611c77565b9161309c565b906020610766928181520190611144565b34610343576040366003190112610343576004356001600160401b038111610343573660238201121561034357806004013590612255826106de565b91612263604051938461069f565b8083526024602084019160051b8301019136831161034357602401905b8282106122c057836024356001600160401b038111610343576104da916122ae6122b492369060040161074b565b906131a7565b60405191829182612208565b6020809183356122cf81610381565b815201910190612280565b34610343576020366003190112610343576004356122f781610381565b6122ff614277565b6001600160a01b038116156123175761061a906142cf565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b34610343575f3660031901126103435760206103e66131fa565b346103435760203660031901126103435760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561064b575f9161240e575b506001600160a01b031633036123ff5761061a906132b7565b63794821ff60e01b5f5260045ffd5b90506020813d602011612441575b816124296020938361069f565b81010312610343575161243b81610381565b5f6123e6565b3d915061241c565b346103435760203660031901126103435760043561246681610381565b60018060a01b03165f5260a360205260405f206040519081602082549182815201915f5260205f20905f5b8181106124a8576104da85611d528187038261069f565b8254845260209093019260019283019201612491565b604080517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad602082019081526001600160a01b0395861692820192909252918416606083015292909116608082015260a081019290925260c08083019390935291815261252c60e08261069f565b5190206125376131fa565b9060405190602082019261190160f01b8452602283015260428201526042815261256260628261069f565b51902090565b1561256f57565b63840a48d560e01b5f5260045ffd5b90612588826106de565b612595604051918261069f565b82815280926125a6601f19916106de565b0190602036910137565b634e487b7160e01b5f52603260045260245ffd5b9190811015611c125760051b81013590605e1981360301821215610343570190565b903590601e198136030182121561034357018035906001600160401b03821161034357602001918160051b3603831361034357565b1561262257565b6343714afd60e01b5f5260045ffd5b805115611c125760200190565b8051821015611c125760209160051b010190565b9081602091031261034357516107668161214d565b6040513d5f823e3d90fd5b1561267957565b631d77d47760e21b5f5260045ffd5b1561268f57565b63c61dca5d60e01b5f5260045ffd5b604080519091906126af838261069f565b6001815291601f1901366020840137565b90604051602081018181106001600160401b0382111761067f5760405291548252565b156126ea57565b633bf2b50360e11b5f5260045ffd5b90918060409360208452816020850152848401375f828201840152601f01601f1916010190565b9081602091031261034357516107668161141c565b1561273c57565b63932d94f760e01b5f5260045ffd5b1561275257565b6325ec6c1f60e01b5f5260045ffd5b6040516125628161277e6020820194602086526020860190610ffe565b03601f19810183528261069f565b6040519061279982610664565b606060c0835f81525f60208201525f60408201525f838201525f60808201528260a08201520152565b90604051918281549182825260208201905f5260205f20925f5b8181106127f157505061039d9250038361069f565b84546001600160a01b03168352600194850194879450602090930192016127dc565b90604051918281549182825260208201905f5260205f20925f5b81811061284257505061039d9250038361069f565b845483526001948501948794506020909301920161282d565b9060405161286881610664565b82546001600160a01b039081168252600184015416602082015291829060c0906128e49060069060028101546001600160a01b03166040860152600381015460608601526128cd6128c0600483015463ffffffff1690565b63ffffffff166080870152565b6128d9600582016127c2565b60a086015201612813565b910152565b906128f3826106de565b612900604051918261069f565b8281528092612911601f19916106de565b01905f5b82811061292157505050565b60209061292c61278c565b82828501015201612915565b90612942826106de565b61294f604051918261069f565b8281528092612960601f19916106de565b01905f5b82811061297057505050565b806060602080938501015201612964565b634e487b7160e01b5f52601160045260245ffd5b9063ffffffff8091169116019063ffffffff82116118b757565b90600182018092116118b757565b919082018092116118b757565b6001600160a01b038181165f908152609a60205260409020541615612b21576001600160a01b0381165f908152609a60205260409020612a099061127a565b60405163152667d960e31b81526001600160a01b038216600482015273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac060248201529092602082806044810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561064b5761039d95612af6935f93612afc575b50612af090612ad36108f4612ab28860018060a01b03165f5260a260205260405f2090565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac05f5260205260405f2090565b936001600160401b0380670de0b6b3a76400005b93169116614c57565b9161425b565b916140e7565b612af0919350612b1a9060203d602011610c5c57610c4e818361069f565b9290612a8d565b505050565b9080601f83011215610343578151612b3d816106de565b92612b4b604051948561069f565b81845260208085019260051b82010192831161034357602001905b828210612b735750505090565b8151815260209182019101612b66565b9190916040818403126103435780516001600160401b03811161034357810183601f8201121561034357805190612bb9826106de565b91612bc7604051938461069f565b80835260208084019160051b8301019186831161034357602001905b828210612c0b575050509260208201516001600160401b038111610343576107669201612b26565b602080918351612c1a81610381565b815201910190612be3565b90816020910312610343575190565b6040516394f649dd60e01b81526001600160a01b038216600482015291905f83806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561064b575f935f92612def575b5060405163fe243a1760e01b81526001600160a01b03909116600482015273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac060248201529060208280604481015b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561064b575f92612dce575b508115612dc957612d2e612d2985516129af565b61257e565b93612d3c612d2982516129af565b92612d64612d4b83518861263e565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac09052565b612d6f82518561263e565b525f5b8151811015612dc35780612da7612d8e6108d36001948661263e565b612d98838a61263e565b6001600160a01b039091169052565b612db1818561263e565b51612dbc828761263e565b5201612d72565b50505090565b919050565b612de891925060203d602011611fde57611fd0818361069f565b905f612d15565b60209450612cd99250612e13903d805f833e612e0b818361069f565b810190612b83565b9490949250612c97565b6001600160a01b03168015159081612e33575090565b5f818152609a60205260409020546001600160a01b031614919050565b919091612e5d835161257e565b905f5b8451811015612ebc576001600160a01b038281165f90815260986020526040902060019291612eaa9190612e94848a61263e565b511660018060a01b03165f5260205260405f2090565b54612eb5828661263e565b5201612e60565b5090925050565b15612eca57565b60405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606490fd5b9190811015611c125760051b0190565b356107668161214d565b15612f3057565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b612f9861039d926139c5565b6142cf565b6001600160a01b038082165f908152609a6020526040902054161561308d57612fc581612e1d565b61307e576001600160a01b0381169033829003612fe7575b6107669150614752565b6001600160a01b0381165f908152609a60205260409020546001600160a01b031661301181613da2565b8015613059575b1561304a57610766927ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a5f80a3612fdd565b631e499a2360e11b5f5260045ffd5b506001600160a01b038181165f90815260996020526040902060010154163314613018565b6311ca333560e31b5f5260045ffd5b63a5c7c44560e01b5f5260045ffd5b335f908152609a60205260409020549093926130da9290916130c7906001600160a01b0316156126e3565b6130d3610e1586612e1d565b84336148c6565b6130eb610499600180606654161490565b335f908152609a6020526040902080546001600160a01b0319166001600160a01b0384161790556001600160a01b038216337fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433045f80a361314a33612c34565b61315582853361339f565b915f5b815181101561319f576001906131996001600160a01b03613179838661263e565b5116613185838761263e565b51613190848961263e565b5191338b613c11565b01613158565b505050509050565b906131b28251612938565b915f5b8151811015612dc3576001906131de846001600160a01b036131d7848761263e565b5116612e50565b6131e8828761263e565b526131f3818661263e565b50016131b5565b467f000000000000000000000000000000000000000000000000000000000000000003613245577f000000000000000000000000000000000000000000000000000000000000000090565b600a602060405161325760408261069f565b828152016922b4b3b2b72630bcb2b960b11b81522060405160208101917f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866835260408201524660608201523060808201526080815261256260a08261069f565b6132c8606654198219811614612688565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2565b602081830312610343578051906001600160401b03821161034357019080601f8301121561034357815161332d816106de565b9261333b604051948561069f565b81845260208085019260051b82010192831161034357602001905b8282106133635750505090565b6020809183516133728161141c565b815201910190613356565b6001600160a01b03909116815260406020820181905261076692910190610fc2565b92916133cd905f816133b1815161257e565b94604051948592839263547afb8760e01b84526004840161337d565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561064b575f92613462575b505f5b815181101561345a57806134496134286108d36001948661263e565b613442613435848861263e565b516001600160401b031690565b9089613b40565b613453828761263e565b520161340c565b509193505050565b61347f9192503d805f833e613477818361069f565b8101906132fa565b905f613409565b1561348d57565b6339b190bb60e11b5f5260045ffd5b156134a357565b63796cc52560e01b5f5260045ffd5b6001600160a01b03918216815291166020820152604081019190915260600190565b5f1981146118b75760010190565b916134fb9183549060031b91821b915f19901b19161790565b9055565b91909182821061350e57505050565b5f5260205f2091820191015b818110613525575050565b5f815560010161351a565b90600160401b811161067f57815481835561039d926134ff565b8151916001600160401b03831161067f576020906135688484613530565b01905f5260205f205f5b83811061357f5750505050565b82516001600160a01b031681830155602090920191600101613572565b8151916001600160401b03831161067f576020906135ba8484613530565b01905f5260205f205f5b8381106135d15750505050565b6001906020845194019381840155016135c4565b815181546001600160a01b039182166001600160a01b03199182161783556020840151600184018054918416918316919091179055604084015160028401805491909316911617905560608201516003820155608082015161039d9260069160c0919061366b9063ffffffff16600486019063ffffffff1663ffffffff19825416179055565b61367c60a08201516005860161354a565b0151910161359c565b916136a29061076694928452606060208501526060840190610ffe565b9160408184039101526107d0565b929493906136c86001600160a01b0385161515613486565b6136d48351151561349c565b6136de835161257e565b906136e9845161257e565b5f5b855181101561385157866137718a6137556137428561373c6108f48d6108e06108d38561373c613721611ef46108d3848861263e565b6001600160a01b03909d165f90815260a26020526040902090565b9361263e565b5161374d868b61263e565b51908361425b565b61375f858761263e565b5261376a848d61263e565b519061496e565b61377b838761263e565b526001600160a01b038416613807575b6001600160a01b0316906137a26108d3828961263e565b6137ac828c61263e565b51833b15610343576137d9935f92838c6040519788958694859363724af42360e01b8552600485016134b2565b03925af191821561064b576001926137f3575b50016136eb565b80610aee5f6138019361069f565b5f6137ec565b6138296138176108d3848a61263e565b613821848861263e565b51908661497b565b61384c6138396108d3848a61263e565b613843848661263e565b51908a87614174565b61378b565b50936139759697507f26b2aae26516e8719ef50ea2f6831a2efbd4e37dccdf0f6936b27bc08e793e30959193509161397a9261389d8360018060a01b03165f52609f60205260405f2090565b546001600160a01b0384165f908152609f602052604090206138bf81546134d4565b90556138e96138cc6106c0565b6001600160a01b0386168152966001600160a01b03166020880152565b6001600160a01b038416604087015260608601524363ffffffff16608086015260a085015260c084015261391c83612761565b958691613941613934845f52609e60205260405f2090565b805460ff19166001179055565b61395c85613957855f5260a460205260405f2090565b6135e5565b6001600160a01b03165f90815260a36020526040902090565b614cd8565b5061398b6040519283928684613685565b0390a190565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b6107669291613a08613a0e9261473f565b90614b23565b614b23565b6001600160a01b039081165f8181526099602090815260409182902060010180546001600160a01b0319169590941694851790935551928352917f773b54c04d756fcc5e678111f7d730de3be98192000799eee3d63716055a87c69190a2565b919091613a87610499600180606654161490565b6001600160a01b038181165f818152609a6020526040812080546001600160a01b03191693871693841790557fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433049080a3613ae081612c34565b9091613aed83868361339f565b925f5b8151811015613b3757600190613b316001600160a01b03613b11838661263e565b5116613b1d838861263e565b51613b28848a61263e565b5191878c613c11565b01613af0565b50505050509050565b91906001600160a01b031673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014613b73576001600160401b0391501690565b60405163a3d75e0960e01b81526001600160a01b039092166004830152602082806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561064b57610766925f92613bf0575b506001600160401b0380670de0b6b3a7640000612ae7565b613c0a91925060203d602011610c5c57610c4e818361069f565b905f613bd8565b90938015613d25576001600160a01b038581165f90815260a2602090815260408083209387168352929052207f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f91613c8691613c7891611e1c916108f49091895f84614a23565b6040519182918689846134b2565b0390a16001600160a01b038085165f908152609a602052604090205416613cae575b50505050565b6001600160a01b0381165f908152609860205260409020613cd09083906114cb565b8054938085018095116118b7577f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c94613d19925560405193849360018060a01b031696846134b2565b0390a25f808080613ca8565b630a33bc6960e21b5f5260045ffd5b919290948015613d2557613c78611e1c7f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f936108f4613c869460018060a01b038b165f5260a260205289613d9b8a60405f209060018060a01b03165f5260205260405f2090565b9384614a23565b604051631beb2b9760e31b81526001600160a01b0391821660048201523360248201523060448201525f80356001600160e01b03191660648301529091602091839160849183917f0000000000000000000000000000000000000000000000000000000000000000165af190811561064b575f91613e1e575090565b610766915060203d60201161064457610636818361069f565b91613e6263ffffffff9160409396959660018060a01b03168552606060208601526060850190610fc2565b9416910152565b939290915f81613e79815161257e565b94613e986040519586938493632535f40360e21b855260048501613e37565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561064b575f92613f04575b505f5b815181101561345a5780613ef36134286108d36001948661263e565b613efd828761263e565b5201613ed7565b613f199192503d805f833e613477818361069f565b905f613ed4565b5f198101919082116118b757565b919082039182116118b757565b63ffffffff5f199116019063ffffffff82116118b757565b9063ffffffff8091169116039063ffffffff82116118b757565b6001600160a01b039081165f81815260a5602090815260408083209486168352939052919091209094939291613fc791613fa690614ab4565b955f5260a560205260405f209060018060a01b03165f5260205260405f2090565b613ffa6117eb7f000000000000000000000000000000000000000000000000000000000000000063ffffffff4316613f53565b8154919063ffffffff165f5b83811061402a5750509461182791610766959681155f1461183c57505f90506114fe565b90928082169080831860011c82018092116118b757835f528463ffffffff8360205f20015416115f146140605750925b90614006565b939150600181018091116118b7579061405a565b60018060a01b031691825f5260986020526140a28260405f209060018060a01b03165f5260205260405f2090565b9182548281039081116118b7577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd93556140e26040519283925f846134b2565b0390a2565b91909160018060a01b031691825f52609860205260405f2073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac05f5260205260405f20908154918383039283116118b7577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd9373beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0936140e29255604051938493846134b2565b6001600160a01b039081165f8181526098602090815260408083209487168352939052919091208054919480830394939285116118b7577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd946140e29255604051938493846134b2565b6001600160a01b031673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac00361422e577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b610766929161426c6142729261473f565b90614bc3565b614bc3565b6033546001600160a01b0316330361428b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b1561431e57565b6316110d3560e21b5f5260045ffd5b1561433457565b6387c9d21960e01b5f5260045ffd5b1561434a57565b6378f67ae160e11b5f5260045ffd5b5f600661039d9282815582600182015582600282015582600382015582600482015561438e83600583018054908281556134ff565b018054908281556134ff565b3561076681610381565b9190826040910312610343576020825192015190565b93929360a08101926143cf845151821461261b565b60408201516143f1906143ea906001600160a01b0316611524565b3314614317565b6143fa82612761565b61441e614419614412835f52609e60205260405f2090565b5460ff1690565b61432d565b7f1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a0061450f6144b8614480614459608088015163ffffffff1690565b7f000000000000000000000000000000000000000000000000000000000000000090612995565b61449763ffffffff821663ffffffff431611614343565b86516001600160a01b031660208801516001600160a01b03168a5191613e69565b926144d5816144d061395c895160018060a01b031690565b614d7d565b506144f06144eb825f5260a460205260405f2090565b614359565b61182c614505825f52609e60205260405f2090565b805460ff19169055565b0390a182516001600160a01b03165f908152609a602052604090206145339061127a565b835161454b906001600160a01b03168288519161339f565b5f5b87518051821015614732579088888883898f96611ef46108d3846145709361263e565b6145868b6113d1856113ca8160c08a015161263e565b97156146395792516001600160a01b03938416936145c3936145be93909290916145b8916108d391859116995161263e565b95612f0f565b61439a565b91813b1561034357604051630bab906360e21b81526001600160a01b039485166004820152908416602482015291909216604482015260648101939093525f908390608490829084905af191821561064b57600192614625575b505b0161454d565b80610aee5f6146339361069f565b5f61461d565b926145be835f936145b86108d360409a999761465e614665975160018060a01b031690565b9a5161263e565b855163c4623ea160e01b81526001600160a01b0395861660048201529285166024840152841660448301526064820196909652948592608492849291165af1801561064b57896146ea91600194848b5f925f946146ef575b50516146d8916108d3916001600160a01b03165b955161263e565b6146e2868961263e565b519389613d34565b61461f565b6108d39194506146d193509161471e6146d89360403d811161472b575b614716818361069f565b8101906143a4565b94909495925050916146bd565b503d61470c565b5050505050505050509050565b51806107665750670de0b6b3a764000090565b61076690614767610499600280606654161490565b6001600160a01b0381165f908152609a60205260409020606092919061478c9061127a565b906147ba6147aa8260018060a01b03165f52609a60205260405f2090565b80546001600160a01b0319169055565b6001600160a01b038281169082167ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af446765f80a36147f581612c34565b91909485519081156148a8575061480b9061257e565b9261481786828461339f565b915f5b875181101561489e5760019061488d8961483261269e565b61483a61269e565b9061485c6148536108d38761484d61269e565b9661263e565b612d9883612631565b614866858b61263e565b5161487083612631565b5261487b858a61263e565b5161488584612631565b5287876136b0565b614897828961263e565b520161481a565b5093955050505050565b955050505050565b156148b757565b630d4c4c9160e21b5f5260045ffd5b6001600160a01b038281165f9081526099602052604090206001015491949116929083156149675761039d9461495d91855f52609c60205260405f20815f5260205261492161491c60ff60405f20541615151590565b6148b0565b61494f613934826149428960018060a01b03165f52609c60205260405f2090565b905f5260205260405f2090565b8560208501958651936124be565b9051915192614c8e565b5050505050565b9061426c6107669261473f565b90919073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeabf196001600160a01b038416016149a857505050565b6149fe9060018060a01b031692835f5260a56020526149dd6117928260405f209060018060a01b03165f5260205260405f2090565b935f5260a560205260405f209060018060a01b03165f5260205260405f2090565b9082018092116118b75761039d916001600160e01b0316904363ffffffff1690614fe8565b9290918215614a6657614a4582614272614a3f611e1c886126c0565b86614bc3565b908082018092116118b75783018093116118b7576134fb92613a0e91614b23565b506134fb9150614c3d565b634e487b7160e01b5f52601260045260245ffd5b9190614a92828285614c57565b928215614aaf5709614aa15790565b600181018091116118b75790565b614a71565b80549081614ac357505f919050565b815f198101116118b7575f525f199060205f2001015460201c611861565b916001600160401b03809116911603906001600160401b0382116118b7576001600160401b03610766921690614bc3565b8115614aaf570490565b1561034357565b5f19670de0b6b3a7640000820991670de0b6b3a7640000820291828085109403938085039414614bb757670de0b6b3a76400008291614b63868411614b1c565b09600182190182168092046002816003021880820260020302808202600203028082026002030280820260020302808202600203028091026002030293600183805f03040190848311900302920304170290565b50906107669250614b12565b5f1982820982820291828083109203918083039214614c2c5781670de0b6b3a76400001115610343577faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066993670de0b6b3a7640000910990828211900360ee1b910360121c170290565b50670de0b6b3a76400009250500490565b8015614aaf576ec097ce7bc90715b34b9f10000000000490565b90915f198383099280830292838086109503948086039514614c8157908291614b63868411614b1c565b5050906107669250614b12565b924211614cb457614c9e92614ebf565b15614ca557565b638baa579f60e01b5f5260045ffd5b630819bdcd60e01b5f5260045ffd5b8054821015611c12575f5260205f2001905f90565b6001810190825f528160205260405f2054155f14614d3b578054600160401b81101561067f57614d28614d12826001879401855584614cc3565b819391549060031b91821b915f19901b19161790565b905554915f5260205260405f2055600190565b5050505f90565b80548015614d69575f190190614d588282614cc3565b8154905f199060031b1b1916905555565b634e487b7160e01b5f52603160045260245ffd5b6001810191805f528260205260405f2054928315155f14614e18575f1984018481116118b75783545f198101949085116118b7575f95858361494294614dcb9803614dd1575b505050614d42565b55600190565b614e01614dfb91614df2614de8614e0f9588614cc3565b90549060031b1c90565b92839187614cc3565b906134e2565b85905f5260205260405f2090565b555f8080614dc3565b505050505f90565b60051115614e2a57565b634e487b7160e01b5f52602160045260245ffd5b9060609260209183526040828401528051918291826040860152018484015e5f828201840152601f01601f1916010190565b3d15614e9a573d90614e8182611c5c565b91614e8f604051938461069f565b82523d5f602084013e565b606090565b9081602091031261034357516001600160e01b0319811681036103435790565b919091614ecc82846150c1565b614ed581614e20565b159081614f66575b50614f5e575f9261277e614f0a85946040519283916020830195630b135d3f60e11b875260248401614e3e565b51915afa614f16614e70565b81614f52575b81614f25575090565b8051630b135d3f60e11b92506001600160e01b031991614f4d91810160209081019101614e9f565b161490565b80516020149150614f1c565b505050600190565b6001600160a01b0383811691161490505f614edd565b15614f8357565b63151b8e3f60e11b5f5260045ffd5b8054600160401b81101561067f57614faf91600182018155614cc3565b614fd557815160209283015190921b63ffffffff191663ffffffff909216919091179055565b634e487b7160e01b5f525f60045260245ffd5b805480615024575b5061501f61039d9361500f6150036106cf565b63ffffffff9095168552565b6001600160e01b03166020840152565b614f92565b805f198101116118b757815f5263ffffffff6150926150895f198460205f20010161507f6150716040519261505884610684565b548681169081855260201c602085015263ffffffff1690565b858916958691161115614f7c565b5163ffffffff1690565b63ffffffff1690565b03614ff05761039d9392509061184f6150aa92613f20565b9063ffffffff82549181199060201b169116179055565b8151604181036150ed5750906150e991602082015190606060408401519301515f1a9061512f565b9091565b6040036151265760406020830151920151918260ff1c91601b83018093116118b7576150e9936001600160ff1b03169260ff169061512f565b50505f90600290565b9291907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083116151cd5760ff16601b811415806151c2575b6151b7576020935f93604051938493608085019385528785015260408401526060830152838052039060015afa1561064b575f516001600160a01b038116156151af57905f90565b505f90600190565b505050505f90600490565b50601c811415615167565b505050505f9060039056fea26469706673582212203b359e406fa02b81faabca67e601f91055d39ddef3162db32899cc128ee4b6d064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80\x80`@R4a\x03\x14W`\xC0\x81aUJ\x808\x03\x80\x91a\0 \x82\x85a\x03\x18V[\x839\x81\x01\x03\x12a\x03\x14W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x14W` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x14W`@\x83\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\x14W``\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x90\x84\x81\x03a\x03\x14W`\x80\x86\x01Q\x95`\x01`\x01`\xA0\x1B\x03\x87\x16\x87\x03a\x03\x14W`\xA0\x01Q\x94c\xFF\xFF\xFF\xFF\x86\x16\x86\x03a\x03\x14W\x15a\x03\x05W`\x80R`\xA0R`\xC0R`\xE0Ra\x01\0RFa\x01 R`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17a\x02\xF1W`\n\x91` \x91`@R\x82\x81R\x01i\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B\x81R `@Q` \x81\x01\x91\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x83R`@\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\x80\x81Ra\x01C`\xA0\x82a\x03\x18V[Q\x90 a\x01@Ra\x01`R_T`\xFF\x81`\x08\x1C\x16a\x02\x9CW`\xFF\x80\x82\x16\x10a\x02bW[`@QaR\x0E\x90\x81a\x03<\x829`\x80Q\x81\x81\x81a\x05\xC0\x01R\x81\x81a\x0EJ\x01R\x81\x81a\x1A$\x01Ra#\xAE\x01R`\xA0Q\x81\x81\x81a\x0B\x0F\x01R\x81\x81a\x0Bw\x01R\x81\x81a,]\x01RaB0\x01R`\xC0Q\x81\x81\x81a\x0Cv\x01R\x81\x81a\r\x9C\x01R\x81\x81a\x15\xC0\x01R\x81\x81a,\xDD\x01R\x81\x81a;\x9C\x01RaB\x03\x01R`\xE0Q\x81\x81\x81a\t\xF0\x01R\x81\x81a\x0B\xEE\x01R\x81\x81a\x14q\x01R\x81\x81a\x17$\x01R\x81\x81a \x0C\x01R\x81\x81a*M\x01R\x81\x81a3\xD1\x01Ra>\x9C\x01Ra\x01\0Q\x81\x81\x81a\x12\x8C\x01R\x81\x81a\x17\xC0\x01R\x81\x81a\x1E=\x01R\x81\x81a?\xCF\x01RaD[\x01Ra\x01 Q\x81a1\xFD\x01Ra\x01@Q\x81a2#\x01Ra\x01`Q\x81\x81\x81a\rX\x01Ra=\xEB\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\x01fV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[_\x80\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x02\xF1W`@RV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xA4\xF9y\x14a\x034W\x80c\x0B\x9FHz\x14a\x03/W\x80c\r\xD8\xDD\x02\x14a\x03*W\x80c\x13d9\xDD\x14a\x03%W\x80c%\xDF\x92.\x14a\x03 W\x80c*\xA6\xD8\x88\x14a\x03\x1BW\x80c9\xB7\x0E8\x14a\x03\x16W\x80c<e\x1C\xF2\x14a\x03\x11W\x80c<\xDE\xB5\xE0\x14a\x03\x0CW\x80c>(9\x1D\x14a\x03\x07W\x80cFW\xE2j\x14a\x03\x02W\x80cFe\xBC\xDA\x14a\x02\xFDW\x80cT\xB7\xC9l\x14a\x02\xF8W\x80cY\\jg\x14a\x02\xF3W\x80cY{6\xDA\x14a\x02\xEEW\x80cZ\xC8j\xB7\x14a\x02\xE9W\x80c\\\x97Z\xBB\x14a\x02\xE4W\x80c]\x97^\x88\x14a\x02\xDFW\x80c]\xD6\x85y\x14a\x02\xDAW\x80c`\x1B\xB3o\x14a\x02\xD5W\x80c`\xA0\xD1\xCE\x14a\x02\xD0W\x80ce\xDA\x12d\x14a\x02\xCBW\x80cf\xD5\xBA\x93\x14a\x02\xC6W\x80cmp\xF7\xAE\x14a\x02\xC1W\x80cn\x17DH\x14a\x02\xBCW\x80cqP\x18\xA6\x14a\x02\xB7W\x80cw\x8EU\xF3\x14a\x02\xB2W\x80cx)n\xC5\x14a\x02\xADW\x80c\x88o\x11\x95\x14a\x02\xA8W\x80c\x8D\xA5\xCB[\x14a\x02\xA3W\x80c\x90\x04\x13G\x14a\x02\x9EW\x80c\x91\x04\xC3\x19\x14a\x02\x99W\x80c\x945\xBBC\x14a\x02\x94W\x80c\xA1x\x84\x84\x14a\x02\x8FW\x80c\xA3:43\x14a\x02\x8AW\x80c\xB7\xF0n\xBE\x14a\x02\x85W\x80c\xBBE\xFE\xF2\x14a\x02\x80W\x80c\xBF\xAE?\xD2\x14a\x02{W\x80c\xC4H\xFE\xB8\x14a\x02vW\x80c\xC9x\xF7\xAC\x14a\x02qW\x80c\xCA\x8A\xA7\xC7\x14a\x02lW\x80c\xCDm\xC6\x87\x14a\x02gW\x80c\xDA\x8B\xE8d\x14a\x02bW\x80c\xE4\xCC?\x90\x14a\x02]W\x80c\xEE\xA9\x06K\x14a\x02XW\x80c\xF0\xE0\xE6v\x14a\x02SW\x80c\xF2\xFD\xE3\x8B\x14a\x02NW\x80c\xF6\x98\xDA%\x14a\x02IW\x80c\xFA\xBC\x1C\xBC\x14a\x02DWc\xFD\x8A\xA8\x8D\x14a\x02?W_\x80\xFD[a$IV[a#\x85V[a#kV[a\"\xDAV[a\"\x19V[a!\xF1V[a!WV[a!%V[a ;V[a\x1F\xF7V[a\x1E\x86V[a\x1E!V[a\x1D\xDBV[a\x1D\x8DV[a\x1D^V[a\x1D+V[a\x1C!V[a\x1B\x0FV[a\x1A\xE1V[a\x1A\xB3V[a\x1ASV[a\x1A\x0FV[a\x19\x80V[a\x19<V[a\x18\xE1V[a\x16\xE5V[a\x16\x99V[a\x16IV[a\x16\x06V[a\x15\x92V[a\x14-V[a\x12\tV[a\x10\x81V[a\x0F\xA5V[a\x0FrV[a\x0F8V[a\x0E\x1FV[a\r\xCBV[a\r\x87V[a\rCV[a\x0C\xF8V[a\x0C\xA4V[a\x0B>V[a\n\xFAV[a\t\x85V[a\x08\x14V[a\x05\x90V[a\x04WV[a\x03\x9FV[a\x03GV[_\x91\x03\x12a\x03CWV[_\x80\xFD[4a\x03CW_6`\x03\x19\x01\x12a\x03CW` `@Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03CWV[5\x90a\x03\x9D\x82a\x03\x81V[V[4a\x03CW`\xA06`\x03\x19\x01\x12a\x03CW` a\x03\xE6`\x045a\x03\xC1\x81a\x03\x81V[`$5a\x03\xCD\x81a\x03\x81V[`D5a\x03\xD9\x81a\x03\x81V[`d5\x91`\x845\x93a$\xBEV[`@Q\x90\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x03CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03CW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x03CWV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x04AWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x044V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x04\x87\x906\x90`\x04\x01a\x03\xEEV[\x90a\x04\x9Fa\x04\x99`\x02\x80`fT\x16\x14\x90V[\x15a%hV[a\x04\xA8\x82a%~V[3_\x90\x81R`\x9A` R`@\x90 T\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x16_[\x82\x81\x10a\x04\xDEW`@Q\x80a\x04\xDA\x87\x82a\x04\x1EV[\x03\x90\xF3[\x80a\x05\x1Ba\x04\xF8a\x04\xF2`\x01\x94\x87\x89a%\xC4V[\x80a%\xE6V[\x90Pa\x05\x12a\x05\x08\x84\x88\x8Aa%\xC4V[` \x81\x01\x90a%\xE6V[\x91\x90P\x14a&\x1BV[a\x05\x7Fa\x05@a\x059a\x052a\x04\xF2\x85\x89\x8Ba%\xC4V[6\x91a\x06\xF5V[\x853a3\x9FV[\x86a\x05w\x87a\x05oa\x05ea\x05\x08\x88a\x05]a\x04\xF2\x82\x87\x8Aa%\xC4V[\x95\x90\x97a%\xC4V[\x94\x90\x926\x91a\x06\xF5V[\x926\x91a\x07iV[\x90\x863a6\xB0V[a\x05\x89\x82\x88a&>V[R\x01a\x04\xC5V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06KWa\x06\x1A\x92a\x06\x06\x91_\x91a\x06\x1CW[Pa&rV[a\x06\x15`fT\x82\x81\x16\x14a&\x88V[a9\xC5V[\0[a\x06>\x91P` =` \x11a\x06DW[a\x066\x81\x83a\x06\x9FV[\x81\x01\x90a&RV[_a\x06\0V[P=a\x06,V[a&gV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\xE0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\x7FW`@RV[a\x06PV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\x7FW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\x7FW`@RV[`@Q\x90a\x03\x9D`\xE0\x83a\x06\x9FV[`@Q\x90a\x03\x9D`@\x83a\x06\x9FV[`\x01`\x01`@\x1B\x03\x81\x11a\x06\x7FW`\x05\x1B` \x01\x90V[\x92\x91\x90a\x07\x01\x81a\x06\xDEV[\x93a\x07\x0F`@Q\x95\x86a\x06\x9FV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x03CW\x90[\x82\x82\x10a\x071WPPPV[` \x80\x91\x835a\x07@\x81a\x03\x81V[\x81R\x01\x91\x01\x90a\x07%V[\x90\x80`\x1F\x83\x01\x12\x15a\x03CW\x81` a\x07f\x935\x91\x01a\x06\xF5V[\x90V[\x92\x91\x90a\x07u\x81a\x06\xDEV[\x93a\x07\x83`@Q\x95\x86a\x06\x9FV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x03CW\x90[\x82\x82\x10a\x07\xA5WPPPV[\x815\x81R` \x91\x82\x01\x91\x01a\x07\x99V[\x90\x80`\x1F\x83\x01\x12\x15a\x03CW\x81` a\x07f\x935\x91\x01a\x07iV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x07\xEDWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x07\xE0V[\x90` a\x07f\x92\x81\x81R\x01\x90a\x07\xD0V[4a\x03CW``6`\x03\x19\x01\x12a\x03CW`\x045a\x081\x81a\x03\x81V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x08P\x906\x90`\x04\x01a\x07KV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x08p\x906\x90`\x04\x01a\x07\xB5V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x9A` R`@\x90 T\x90\x93a\x08\x98\x92\x86\x92\x16\x90a3\x9FV[a\x08\xA2\x84Qa%~V[\x93_[\x81Q\x81\x10\x15a\t&W`\x01\x90\x85_R`\xA2` Ra\t\x15a\x08\xF9a\x08\xF4`@_ a\x08\xE0a\x08\xD3\x86\x89a&>V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a&\xC0V[a\t\x03\x83\x88a&>V[Qa\t\x0E\x84\x88a&>V[Q\x91a9\xF7V[a\t\x1F\x82\x89a&>V[R\x01a\x08\xA5V[`@Q\x80a\x04\xDA\x88\x82a\x08\x03V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03CWV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03CWV[\x91\x81`\x1F\x84\x01\x12\x15a\x03CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03CW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03CWV[4a\x03CW``6`\x03\x19\x01\x12a\x03CW`\x045a\t\xA2\x81a\x03\x81V[a\t\xAAa\t4V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\t\xC9\x906\x90`\x04\x01a\tXV[3_\x90\x81R`\x9A` R`@\x90 T\x91\x93\x91a\t\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a&\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03CW`@Qc+bA\xF3`\xE1\x1B\x81R3`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16`$\x85\x01R_\x90\x84\x90`D\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x06KW\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x93a\n\xDB\x93a\n\xE0W[Pa\n\x8A\x813a:\x13V[a\n\x9433a:sV[`@Q`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R3\x90\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90` \x90\xA2`@Q\x91\x82\x913\x95\x83a&\xF9V[\x03\x90\xA2\0[\x80a\n\xEE_a\n\xF4\x93a\x06\x9FV[\x80a\x039V[_a\n\x7FV[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03CW`\x806`\x03\x19\x01\x12a\x03CW`\x045a\x0B[\x81a\x03\x81V[`$5a\x0Bg\x81a\x03\x81V[`d5`D5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x14\x80\x15a\x0CrW[\x15a\x0CcW`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x90\x81R`\x9A` \x90\x81R`@\x91\x82\x90 T\x91Qc\x15&g\xD9`\xE3\x1B\x81R\x91\x83\x16`\x04\x83\x01\x81\x90R\x86\x84\x16`$\x84\x01R\x91\x96\x91\x95\x92\x87\x90`D\x90\x82\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x95\x86\x15a\x06KWa\x06\x1A\x96a\x0C.\x91_\x91a\x0C4W[P\x83\x83a;@V[\x94a=4V[a\x0CV\x91P` =` \x11a\x0C\\W[a\x0CN\x81\x83a\x06\x9FV[\x81\x01\x90a' V[_a\x0C&V[P=a\x0CDV[c\x04R\x06\xA5`\xE2\x1B_R`\x04_\xFD[P3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xA0V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW` a\x0C\xE6`\x045a\x0C\xC6\x81a\x03\x81V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW` a\r9`\x045a\r\x1A\x81a\x03\x81V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03CW`@6`\x03\x19\x01\x12a\x03CWa\x06\x1A`\x045a\r\xEB\x81a\x03\x81V[`$5\x90a\r\xF8\x82a\x03\x81V[a\x0E\ta\x0E\x04\x82a=\xA2V[a'5V[a\x0E\x1Aa\x0E\x15\x82a.\x1DV[a'KV[a:\x13V[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x06KWa\x0E\x8A\x91_\x91a\x06\x1CWPa&rV[a\x06\x1Aa9\x91V[\x91\x90\x91`\xE0\x81\x84\x03\x12a\x03CWa\x0E\xA7a\x06\xC0V[\x92a\x0E\xB1\x82a\x03\x92V[\x84Ra\x0E\xBF` \x83\x01a\x03\x92V[` \x85\x01Ra\x0E\xD0`@\x83\x01a\x03\x92V[`@\x85\x01R``\x82\x015``\x85\x01Ra\x0E\xEB`\x80\x83\x01a\tGV[`\x80\x85\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03CW\x81a\x0F\x0F\x91\x84\x01a\x07KV[`\xA0\x85\x01R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x0F1\x92\x01a\x07\xB5V[`\xC0\x83\x01RV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x03\xE6a\x0Fm` \x926\x90`\x04\x01a\x0E\x92V[a'aV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045`\xFF\x81\x16\x80\x91\x03a\x03CW`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x03CW_6`\x03\x19\x01\x12a\x03CW` `fT`@Q\x90\x81R\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0F\xDFWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0F\xD2V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01Ra\x07f\x91`\xC0a\x10_`\xA0\x84\x01Q`\xE0`\xA0\x85\x01R`\xE0\x84\x01\x90a\x0F\xC2V[\x92\x01Q\x90`\xC0\x81\x84\x03\x91\x01Ra\x07\xD0V[\x90` a\x07f\x92\x81\x81R\x01\x90a\x0F\xFEV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a\x10\x9Da'\x8CV[P_R`\xA4` Ra\x04\xDA`@_ a\x113`\x06`@Q\x92a\x10\xBE\x84a\x06dV[\x80T`\x01`\x01`\xA0\x1B\x03\x16\x84R`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16` \x85\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16`@\x85\x01R`\x03\x81\x01T``\x85\x01Ra\x11\x1Ca\x11\x0F`\x04\x83\x01Tc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16`\x80\x86\x01RV[a\x11(`\x05\x82\x01a'\xC2V[`\xA0\x85\x01R\x01a(\x13V[`\xC0\x82\x01R`@Q\x91\x82\x91\x82a\x10pV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x11oWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x11\x8D`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x07\xD0V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x11`V[\x92\x91`@\x84\x01\x93`@\x81R\x82Q\x80\x95R``\x81\x01\x94` ``\x82`\x05\x1B\x84\x01\x01\x94\x01\x90_[\x81\x81\x10a\x11\xDEWPPPa\x07f\x93\x94P` \x81\x84\x03\x91\x01Ra\x11DV[\x90\x91\x94` \x80a\x11\xFA`\x01\x93`_\x19\x88\x82\x03\x01\x8CR\x89Qa\x0F\xFEV[\x97\x01\x98\x01\x91\x01\x96\x91\x90\x96a\x11\xC1V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a\x12&\x81a\x03\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xA3` R`@\x90 a\x12F\x90a(\x13V[\x80Q\x90a\x12R\x82a(\xE9V[\x91a\x12\\\x81a)8V[\x93a\x12\x87a\x12z\x82`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91_\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92c\xFF\xFF\xFF\xFFC\x16\x90[\x83\x86\x10a\x12\xCBW`@Q\x80a\x04\xDA\x8B\x8B\x83a\x11\x9CV[a\x12\xF1a\x12\xECa\x12\xDD\x88\x8A\x9C\x9Aa&>V[Q_R`\xA4` R`@_ \x90V[a([V[a\x12\xFB\x87\x8Aa&>V[Ra\x13\x06\x86\x89a&>V[Pa\x13\x1F`\xA0a\x13\x16\x88\x8Ba&>V[Q\x01QQa%~V[a\x13)\x87\x89a&>V[Ra\x134\x86\x88a&>V[Pa\x13Y\x85a\x13T`\x80a\x13H\x8A\x8Da&>V[Q\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[a)\x95V[\x82c\xFF\xFF\xFF\xFF\x82\x16\x10_\x14a\x13\xFCWa\x13\x82\x90`\xA0a\x13x\x89\x8Ca&>V[Q\x01Q\x85\x84a>iV[\x94[_[`\xA0a\x13\x92\x89\x8Ca&>V[Q\x01QQ\x81\x10\x15a\x13\xEBW\x80\x89a\x13\xE4\x82a\x13\xDE\x8C\x8F\x8Da\x13\xD1\x85a\x13\xCA`\x01\x9B`\xC0a\x13\xC2\x88a\x13\xD8\x98a&>V[Q\x01Qa&>V[Q\x92a&>V[Q\x90aK\xC3V[\x94a&>V[Qa&>V[R\x01a\x13\x86V[P\x96\x98\x96`\x01\x90\x96\x01\x95\x94Pa\x12\xB5V[Pa\x14\x16`\xA0a\x14\x0C\x88\x8Ba&>V[Q\x01Q\x84\x83a3\x9FV[\x94a\x13\x84V[`\x01`\x01`@\x1B\x03\x81\x16\x03a\x03CWV[4a\x03CW`\x806`\x03\x19\x01\x12a\x03CW`\x045a\x14J\x81a\x03\x81V[`$5a\x14V\x81a\x03\x81V[`D5a\x14b\x81a\x14\x1CV[`d5\x90a\x14o\x82a\x14\x1CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x15\x83W\x82a\x15\x12a\x15\x0Ca\x15\x18\x94a\x15\x04a\x14\xE0\x85a\x14\xCB\x8B`\x01\x80`\xA0\x1B\x03\x16_R`\x98` R`@_ \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta\x14\xFE`\x01`\x01`@\x1B\x03\x88\x16`\x01`\x01`@\x1B\x03\x85\x16\x83aJ\x85V[\x90a?.V[\x94\x84\x89a?mV[\x83a)\xBDV[\x94a@tV[a\x150a\x15$\x82aA\xDEV[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x82;\x15a\x03CW`@Qc\xDE\xBE\x1E\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90_\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x06KWa\x15uW\0[\x80a\n\xEE_a\x06\x1A\x93a\x06\x9FV[c#\xD8q\xA5`\xE0\x1B_R`\x04_\xFD[4a\x03CW``6`\x03\x19\x01\x12a\x03CW`\x045a\x15\xAF\x81a\x03\x81V[`D5`$5a\x15\xBE\x82a\x14\x1CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x15\xF7Wa\x06\x1A\x92a)\xCAV[c2\x13\xA6a`\xE2\x1B_R`\x04_\xFD[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a\x16#\x81a\x03\x81V[`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x03CW` 6`\x03\x19\x01\x12a\x03CWa\x16\x8Ba\x04\xDAa\x16t`\x045a\x16o\x81a\x03\x81V[a,4V[`@\x92\x91\x92Q\x93\x84\x93`@\x85R`@\x85\x01\x90a\x0F\xC2V[\x90\x83\x82\x03` \x85\x01Ra\x07\xD0V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW` a\r9`\x045a\x16\xBB\x81a\x03\x81V[a.\x1DV[`@\x90`\x03\x19\x01\x12a\x03CW`\x045a\x16\xD8\x81a\x03\x81V[\x90`$5a\x07f\x81a\x03\x81V[4a\x03CWa\x16\xF36a\x16\xC0V[`@Qc\x15&g\xD9`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R\x91` \x83\x80`D\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\x06KW_\x93a\x18\xBCW[Pa\x17\xB8\x90`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\xA5` Ra\x17\x97a\x17\x92\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[aJ\xB4V[\x92_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a\x17\xF0a\x17\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFFC\x16a?SV[a?;V[\x81T\x91\x90c\xFF\xFF\xFF\xFF\x16_[\x83\x81\x10a\x18mWa\x04\xDAa\x18,_\x88a\x18'\x89\x89\x89\x81a\x18<WPP`\x01`\x01`\xE0\x1B\x03\x84\x16a\x14\xFEV[aJ\xE1V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x18Za\x18a\x91a\x18Oa\x14\xFE\x94a? V[\x90_R` _ \x01\x90V[T` \x1C\x90V[`\x01`\x01`\xE0\x1B\x03\x16\x90V[\x90\x92\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a\x18\xB7W\x83_R\x84c\xFF\xFF\xFF\xFF\x83` _ \x01T\x16\x11_\x14a\x18\xA3WP\x92[\x90a\x17\xFCV[\x93\x91P`\x01\x81\x01\x80\x91\x11a\x18\xB7W\x90a\x18\x9DV[a)\x81V[a\x17\xB8\x91\x93Pa\x18\xDA\x90` =` \x11a\x0C\\Wa\x0CN\x81\x83a\x06\x9FV[\x92\x90a\x17\\V[4a\x03CW_6`\x03\x19\x01\x12a\x03CWa\x18\xF9aBwV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03CW` a\x19wa\x19O6a\x16\xC0V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x98\x85R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T`@Q\x90\x81R\xF3[4a\x03CW`@6`\x03\x19\x01\x12a\x03CW`\x045a\x19\x9D\x81a\x03\x81V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x19\xDD\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x916\x90`\x04\x01a\tXV[\x90\x92a\x19\xEBa\x0E\x04\x82a=\xA2V[a\x19\xF7a\x0E\x15\x82a.\x1DV[a\n\xDB`@Q\x92\x83\x92`\x01\x80`\xA0\x1B\x03\x16\x95\x83a&\xF9V[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90`@`\x03\x19\x83\x01\x12a\x03CW`\x045a\x1A\x94\x81a\x03\x81V[\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03CWa\x07f\x91`\x04\x01a\x07KV[4a\x03CWa\x04\xDAa\x1A\xCDa\x1A\xC76a\x1A{V[\x90a.PV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xD0V[4a\x03CW_6`\x03\x19\x01\x12a\x03CW` `@Qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81R\xF3[4a\x03CW``6`\x03\x19\x01\x12a\x03CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x1B?\x906\x90`\x04\x01a\x03\xEEV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x1B^\x906\x90`\x04\x01a\x03\xEEV[\x91`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x1B\x81\x90\x93\x91\x936\x90`\x04\x01a\x03\xEEV[\x90a\x1B\x93a\x04\x99`\x04\x80`fT\x16\x14\x90V[a\x1B\xA2`\x02`\xC9T\x14\x15a.\xC3V[`\x02`\xC9U6\x86\x90\x03`\xDE\x19\x01\x92_[\x86\x81\x10\x15a\x1C\x17W\x80`\x05\x1B\x90\x81\x89\x015\x91\x86\x83\x12\x15a\x03CW\x83\x82\x10\x15a\x1C\x12W`\x01\x92a\x1B\xE6a\x1C\x0C\x92\x8A\x01\x8Aa%\xE6V[\x90a\x1C\x07\x8Da\x1B\xFEa\x1B\xF9\x88\x8D\x8Da/\x0FV[a/\x1FV[\x946\x91\x01a\x0E\x92V[aC\xBAV[\x01a\x1B\xB2V[a%\xB0V[a\x06\x1A`\x01`\xC9UV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a\x1C>\x81a\x03\x81V[`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x06\x7FW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x90```\x03\x19\x83\x01\x12a\x03CW`\x045a\x1C\x90\x81a\x03\x81V[\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03CW`@\x82\x82\x03`\x03\x19\x01\x12a\x03CW`@Q\x91a\x1C\xBE\x83a\x06\x84V[\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03CW\x81\x01\x91\x80`#\x84\x01\x12\x15a\x03CW`\x04\x83\x015a\x1C\xED\x81a\x1C\\V[\x91a\x1C\xFB`@Q\x93\x84a\x06\x9FV[\x81\x83R`$\x85\x83\x01\x01\x11a\x03CW` \x81_\x92`$\x80\x97\x01\x83\x86\x017\x83\x01\x01R\x83R\x015` \x82\x01R\x90`D5\x90V[4a\x03CWa\x04\xDAa\x1DRa\x1D?6a\x1CwV[\x90a\x1DL\x93\x92\x933a/\x9DV[\x93a0\x9CV[`@Q\x91\x82\x91\x82a\x04\x1EV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045_R`\x9E` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03CW`@6`\x03\x19\x01\x12a\x03CW`\x045a\x1D\xAA\x81a\x03\x81V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03CW` a\x03\xE6a\x1E\x1Ca\x08\xF4a\x1D\xF46a\x16\xC0V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\xA2\x87R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[aG?V[4a\x03CW_6`\x03\x19\x01\x12a\x03CW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[\x90\x91a\x1Exa\x07f\x93`@\x84R`@\x84\x01\x90a\x07\xD0V[\x91` \x81\x84\x03\x91\x01Ra\x07\xD0V[4a\x03CWa\x1E\x946a\x1A{V[a\x1E\x9E\x81Qa%~V[a\x1E\xA8\x82Qa%~V[\x91a\x1E\xD0\x81a\x1E\xCAa\x12z\x87`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x86a3\x9FV[_[\x82Q\x81\x10\x15a\x1F\xE5W\x80` a\x1E\xF9a\x15$a\x1E\xF4a\x08\xD3a\x1F:\x96\x89a&>V[aA\xDEV[a\x1F\x06a\x08\xD3\x84\x88a&>V[`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x8C\x16`\x04\x83\x01R\x90\x91\x16`$\x82\x01R\x93\x84\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x80\x15a\x06KW`\x01\x92_\x91a\x1F\xB7W[Pa\x1FY\x82\x88a&>V[Ra\x1F\xA6a\x1F\x8Aa\x08\xF4a\x1F}\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[a\x08\xE0a\x08\xD3\x86\x8Aa&>V[a\x1F\x94\x83\x89a&>V[Qa\x1F\x9F\x84\x87a&>V[Q\x91aB[V[a\x1F\xB0\x82\x87a&>V[R\x01a\x1E\xD2V[a\x1F\xD8\x91P` =\x81\x11a\x1F\xDEW[a\x1F\xD0\x81\x83a\x06\x9FV[\x81\x01\x90a,%V[_a\x1FNV[P=a\x1F\xC6V[PPPa\x04\xDA`@Q\x92\x83\x92\x83a\x1EaV[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03CW`@6`\x03\x19\x01\x12a\x03CW`\x045a X\x81a\x03\x81V[a \x9D`$5_T\x92a \x83`\xFF`\x08\x86\x90\x1C\x16\x15\x80\x95\x81\x96a!\x17W[\x81\x15a \xF7W[Pa/)V[\x83a \x94`\x01`\xFF\x19_T\x16\x17_UV[a \xE0Wa/\x8CV[a \xA3W\0[a \xB1a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a \xF2a\x01\0a\xFF\0\x19_T\x16\x17_UV[a/\x8CV[0;\x15\x91P\x81a!\tW[P_a }V[`\xFF\x16`\x01\x14\x90P_a!\x02V[`\x01`\xFF\x82\x16\x10\x91Pa vV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CWa\x04\xDAa\x1DR`\x045a!H\x81a\x03\x81V[a/\x9DV[\x80\x15\x15\x03a\x03CWV[4a\x03CW``6`\x03\x19\x01\x12a\x03CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03CW`\xE0`\x03\x19\x826\x03\x01\x12a\x03CW`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03CWa!\xAAa!\xEA\x926\x90`\x04\x01a\x03\xEEV[\x90a\x1C\x07`D5\x93a!\xBB\x85a!MV[a!\xCCa\x04\x99`\x04\x80`fT\x16\x14\x90V[a!\xDB`\x02`\xC9T\x14\x15a.\xC3V[`\x02`\xC9U6\x90`\x04\x01a\x0E\x92V[`\x01`\xC9U\0[4a\x03CWa\x06\x1Aa\"\x026a\x1CwV[\x91a0\x9CV[\x90` a\x07f\x92\x81\x81R\x01\x90a\x11DV[4a\x03CW`@6`\x03\x19\x01\x12a\x03CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03CW6`#\x82\x01\x12\x15a\x03CW\x80`\x04\x015\x90a\"U\x82a\x06\xDEV[\x91a\"c`@Q\x93\x84a\x06\x9FV[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03CW`$\x01\x90[\x82\x82\x10a\"\xC0W\x83`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x04\xDA\x91a\"\xAEa\"\xB4\x926\x90`\x04\x01a\x07KV[\x90a1\xA7V[`@Q\x91\x82\x91\x82a\"\x08V[` \x80\x91\x835a\"\xCF\x81a\x03\x81V[\x81R\x01\x91\x01\x90a\"\x80V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a\"\xF7\x81a\x03\x81V[a\"\xFFaBwV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a#\x17Wa\x06\x1A\x90aB\xCFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03CW_6`\x03\x19\x01\x12a\x03CW` a\x03\xE6a1\xFAV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06KW_\x91a$\x0EW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a#\xFFWa\x06\x1A\x90a2\xB7V[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a$AW[\x81a$)` \x93\x83a\x06\x9FV[\x81\x01\x03\x12a\x03CWQa$;\x81a\x03\x81V[_a#\xE6V[=\x91Pa$\x1CV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a$f\x81a\x03\x81V[`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a$\xA8Wa\x04\xDA\x85a\x1DR\x81\x87\x03\x82a\x06\x9FV[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a$\x91V[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x92\x82\x01\x92\x90\x92R\x91\x84\x16``\x83\x01R\x92\x90\x91\x16`\x80\x82\x01R`\xA0\x81\x01\x92\x90\x92R`\xC0\x80\x83\x01\x93\x90\x93R\x91\x81Ra%,`\xE0\x82a\x06\x9FV[Q\x90 a%7a1\xFAV[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra%b`b\x82a\x06\x9FV[Q\x90 \x90V[\x15a%oWV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x90a%\x88\x82a\x06\xDEV[a%\x95`@Q\x91\x82a\x06\x9FV[\x82\x81R\x80\x92a%\xA6`\x1F\x19\x91a\x06\xDEV[\x01\x90` 6\x91\x017V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x81\x10\x15a\x1C\x12W`\x05\x1B\x81\x015\x90`^\x19\x816\x03\x01\x82\x12\x15a\x03CW\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03CW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03CW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x03CWV[\x15a&\"WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x80Q\x15a\x1C\x12W` \x01\x90V[\x80Q\x82\x10\x15a\x1C\x12W` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03CWQa\x07f\x81a!MV[`@Q=_\x82>=\x90\xFD[\x15a&yWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a&\x8FWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[`@\x80Q\x90\x91\x90a&\xAF\x83\x82a\x06\x9FV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\x7FW`@R\x91T\x82RV[\x15a&\xEAWV[c;\xF2\xB5\x03`\xE1\x1B_R`\x04_\xFD[\x90\x91\x80`@\x93` \x84R\x81` \x85\x01R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03CWQa\x07f\x81a\x14\x1CV[\x15a'<WV[c\x93-\x94\xF7`\xE0\x1B_R`\x04_\xFD[\x15a'RWV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[`@Qa%b\x81a'~` \x82\x01\x94` \x86R` \x86\x01\x90a\x0F\xFEV[\x03`\x1F\x19\x81\x01\x83R\x82a\x06\x9FV[`@Q\x90a'\x99\x82a\x06dV[```\xC0\x83_\x81R_` \x82\x01R_`@\x82\x01R_\x83\x82\x01R_`\x80\x82\x01R\x82`\xA0\x82\x01R\x01RV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_[\x81\x81\x10a'\xF1WPPa\x03\x9D\x92P\x03\x83a\x06\x9FV[\x84T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01a'\xDCV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_[\x81\x81\x10a(BWPPa\x03\x9D\x92P\x03\x83a\x06\x9FV[\x84T\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01a(-V[\x90`@Qa(h\x81a\x06dV[\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x84\x01T\x16` \x82\x01R\x91\x82\x90`\xC0\x90a(\xE4\x90`\x06\x90`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16`@\x86\x01R`\x03\x81\x01T``\x86\x01Ra(\xCDa(\xC0`\x04\x83\x01Tc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16`\x80\x87\x01RV[a(\xD9`\x05\x82\x01a'\xC2V[`\xA0\x86\x01R\x01a(\x13V[\x91\x01RV[\x90a(\xF3\x82a\x06\xDEV[a)\0`@Q\x91\x82a\x06\x9FV[\x82\x81R\x80\x92a)\x11`\x1F\x19\x91a\x06\xDEV[\x01\x90_[\x82\x81\x10a)!WPPPV[` \x90a),a'\x8CV[\x82\x82\x85\x01\x01R\x01a)\x15V[\x90a)B\x82a\x06\xDEV[a)O`@Q\x91\x82a\x06\x9FV[\x82\x81R\x80\x92a)``\x1F\x19\x91a\x06\xDEV[\x01\x90_[\x82\x81\x10a)pWPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a)dV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x18\xB7WV[\x90`\x01\x82\x01\x80\x92\x11a\x18\xB7WV[\x91\x90\x82\x01\x80\x92\x11a\x18\xB7WV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15a+!W`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9A` R`@\x90 a*\t\x90a\x12zV[`@Qc\x15&g\xD9`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x82\x01R\x90\x92` \x82\x80`D\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06KWa\x03\x9D\x95a*\xF6\x93_\x93a*\xFCW[Pa*\xF0\x90a*\xD3a\x08\xF4a*\xB2\x88`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0_R` R`@_ \x90V[\x93`\x01`\x01`@\x1B\x03\x80g\r\xE0\xB6\xB3\xA7d\0\0[\x93\x16\x91\x16aLWV[\x91aB[V[\x91a@\xE7V[a*\xF0\x91\x93Pa+\x1A\x90` =` \x11a\x0C\\Wa\x0CN\x81\x83a\x06\x9FV[\x92\x90a*\x8DV[PPPV[\x90\x80`\x1F\x83\x01\x12\x15a\x03CW\x81Qa+=\x81a\x06\xDEV[\x92a+K`@Q\x94\x85a\x06\x9FV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03CW` \x01\x90[\x82\x82\x10a+sWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a+fV[\x91\x90\x91`@\x81\x84\x03\x12a\x03CW\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x03CW\x81\x01\x83`\x1F\x82\x01\x12\x15a\x03CW\x80Q\x90a+\xB9\x82a\x06\xDEV[\x91a+\xC7`@Q\x93\x84a\x06\x9FV[\x80\x83R` \x80\x84\x01\x91`\x05\x1B\x83\x01\x01\x91\x86\x83\x11a\x03CW` \x01\x90[\x82\x82\x10a,\x0BWPPP\x92` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x07f\x92\x01a+&V[` \x80\x91\x83Qa,\x1A\x81a\x03\x81V[\x81R\x01\x91\x01\x90a+\xE3V[\x90\x81` \x91\x03\x12a\x03CWQ\x90V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R\x91\x90_\x83\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06KW_\x93_\x92a-\xEFW[P`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x82\x01R\x90` \x82\x80`D\x81\x01[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06KW_\x92a-\xCEW[P\x81\x15a-\xC9Wa-.a-)\x85Qa)\xAFV[a%~V[\x93a-<a-)\x82Qa)\xAFV[\x92a-da-K\x83Q\x88a&>V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x90RV[a-o\x82Q\x85a&>V[R_[\x81Q\x81\x10\x15a-\xC3W\x80a-\xA7a-\x8Ea\x08\xD3`\x01\x94\x86a&>V[a-\x98\x83\x8Aa&>V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a-\xB1\x81\x85a&>V[Qa-\xBC\x82\x87a&>V[R\x01a-rV[PPP\x90V[\x91\x90PV[a-\xE8\x91\x92P` =` \x11a\x1F\xDEWa\x1F\xD0\x81\x83a\x06\x9FV[\x90_a-\x15V[` \x94Pa,\xD9\x92Pa.\x13\x90=\x80_\x83>a.\x0B\x81\x83a\x06\x9FV[\x81\x01\x90a+\x83V[\x94\x90\x94\x92Pa,\x97V[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x15\x90\x81a.3WP\x90V[_\x81\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14\x91\x90PV[\x91\x90\x91a.]\x83Qa%~V[\x90_[\x84Q\x81\x10\x15a.\xBCW`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x98` R`@\x90 `\x01\x92\x91a.\xAA\x91\x90a.\x94\x84\x8Aa&>V[Q\x16`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta.\xB5\x82\x86a&>V[R\x01a.`V[P\x90\x92PPV[\x15a.\xCAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x90\xFD[\x91\x90\x81\x10\x15a\x1C\x12W`\x05\x1B\x01\x90V[5a\x07f\x81a!MV[\x15a/0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a/\x98a\x03\x9D\x92a9\xC5V[aB\xCFV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15a0\x8DWa/\xC5\x81a.\x1DV[a0~W`\x01`\x01`\xA0\x1B\x03\x81\x16\x903\x82\x90\x03a/\xE7W[a\x07f\x91PaGRV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a0\x11\x81a=\xA2V[\x80\x15a0YW[\x15a0JWa\x07f\x92\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A_\x80\xA3a/\xDDV[c\x1EI\x9A#`\xE1\x1B_R`\x04_\xFD[P`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14a0\x18V[c\x11\xCA35`\xE3\x1B_R`\x04_\xFD[c\xA5\xC7\xC4E`\xE0\x1B_R`\x04_\xFD[3_\x90\x81R`\x9A` R`@\x90 T\x90\x93\x92a0\xDA\x92\x90\x91a0\xC7\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a&\xE3V[a0\xD3a\x0E\x15\x86a.\x1DV[\x843aH\xC6V[a0\xEBa\x04\x99`\x01\x80`fT\x16\x14\x90V[3_\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x163\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04_\x80\xA3a1J3a,4V[a1U\x82\x853a3\x9FV[\x91_[\x81Q\x81\x10\x15a1\x9FW`\x01\x90a1\x99`\x01`\x01`\xA0\x1B\x03a1y\x83\x86a&>V[Q\x16a1\x85\x83\x87a&>V[Qa1\x90\x84\x89a&>V[Q\x913\x8Ba<\x11V[\x01a1XV[PPPP\x90PV[\x90a1\xB2\x82Qa)8V[\x91_[\x81Q\x81\x10\x15a-\xC3W`\x01\x90a1\xDE\x84`\x01`\x01`\xA0\x1B\x03a1\xD7\x84\x87a&>V[Q\x16a.PV[a1\xE8\x82\x87a&>V[Ra1\xF3\x81\x86a&>V[P\x01a1\xB5V[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a2EW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`\n` `@Qa2W`@\x82a\x06\x9FV[\x82\x81R\x01i\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B\x81R `@Q` \x81\x01\x91\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x83R`@\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\x80\x81Ra%b`\xA0\x82a\x06\x9FV[a2\xC8`fT\x19\x82\x19\x81\x16\x14a&\x88V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2V[` \x81\x83\x03\x12a\x03CW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03CW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03CW\x81Qa3-\x81a\x06\xDEV[\x92a3;`@Q\x94\x85a\x06\x9FV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03CW` \x01\x90[\x82\x82\x10a3cWPPP\x90V[` \x80\x91\x83Qa3r\x81a\x14\x1CV[\x81R\x01\x91\x01\x90a3VV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra\x07f\x92\x91\x01\x90a\x0F\xC2V[\x92\x91a3\xCD\x90_\x81a3\xB1\x81Qa%~V[\x94`@Q\x94\x85\x92\x83\x92cTz\xFB\x87`\xE0\x1B\x84R`\x04\x84\x01a3}V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06KW_\x92a4bW[P_[\x81Q\x81\x10\x15a4ZW\x80a4Ia4(a\x08\xD3`\x01\x94\x86a&>V[a4Ba45\x84\x88a&>V[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x90\x89a;@V[a4S\x82\x87a&>V[R\x01a4\x0CV[P\x91\x93PPPV[a4\x7F\x91\x92P=\x80_\x83>a4w\x81\x83a\x06\x9FV[\x81\x01\x90a2\xFAV[\x90_a4\tV[\x15a4\x8DWV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[\x15a4\xA3WV[cyl\xC5%`\xE0\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x91\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_\x19\x81\x14a\x18\xB7W`\x01\x01\x90V[\x91a4\xFB\x91\x83T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90UV[\x91\x90\x91\x82\x82\x10a5\x0EWPPPV[_R` _ \x91\x82\x01\x91\x01[\x81\x81\x10a5%WPPV[_\x81U`\x01\x01a5\x1AV[\x90`\x01`@\x1B\x81\x11a\x06\x7FW\x81T\x81\x83Ua\x03\x9D\x92a4\xFFV[\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x06\x7FW` \x90a5h\x84\x84a50V[\x01\x90_R` _ _[\x83\x81\x10a5\x7FWPPPPV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x83\x01U` \x90\x92\x01\x91`\x01\x01a5rV[\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x06\x7FW` \x90a5\xBA\x84\x84a50V[\x01\x90_R` _ _[\x83\x81\x10a5\xD1WPPPPV[`\x01\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a5\xC4V[\x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U` \x84\x01Q`\x01\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\x02\x84\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U``\x82\x01Q`\x03\x82\x01U`\x80\x82\x01Qa\x03\x9D\x92`\x06\x91`\xC0\x91\x90a6k\x90c\xFF\xFF\xFF\xFF\x16`\x04\x86\x01\x90c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[a6|`\xA0\x82\x01Q`\x05\x86\x01a5JV[\x01Q\x91\x01a5\x9CV[\x91a6\xA2\x90a\x07f\x94\x92\x84R``` \x85\x01R``\x84\x01\x90a\x0F\xFEV[\x91`@\x81\x84\x03\x91\x01Ra\x07\xD0V[\x92\x94\x93\x90a6\xC8`\x01`\x01`\xA0\x1B\x03\x85\x16\x15\x15a4\x86V[a6\xD4\x83Q\x15\x15a4\x9CV[a6\xDE\x83Qa%~V[\x90a6\xE9\x84Qa%~V[_[\x85Q\x81\x10\x15a8QW\x86a7q\x8Aa7Ua7B\x85a7<a\x08\xF4\x8Da\x08\xE0a\x08\xD3\x85a7<a7!a\x1E\xF4a\x08\xD3\x84\x88a&>V[`\x01`\x01`\xA0\x1B\x03\x90\x9D\x16_\x90\x81R`\xA2` R`@\x90 \x90V[\x93a&>V[Qa7M\x86\x8Ba&>V[Q\x90\x83aB[V[a7_\x85\x87a&>V[Ra7j\x84\x8Da&>V[Q\x90aInV[a7{\x83\x87a&>V[R`\x01`\x01`\xA0\x1B\x03\x84\x16a8\x07W[`\x01`\x01`\xA0\x1B\x03\x16\x90a7\xA2a\x08\xD3\x82\x89a&>V[a7\xAC\x82\x8Ca&>V[Q\x83;\x15a\x03CWa7\xD9\x93_\x92\x83\x8C`@Q\x97\x88\x95\x86\x94\x85\x93crJ\xF4#`\xE0\x1B\x85R`\x04\x85\x01a4\xB2V[\x03\x92Z\xF1\x91\x82\x15a\x06KW`\x01\x92a7\xF3W[P\x01a6\xEBV[\x80a\n\xEE_a8\x01\x93a\x06\x9FV[_a7\xECV[a8)a8\x17a\x08\xD3\x84\x8Aa&>V[a8!\x84\x88a&>V[Q\x90\x86aI{V[a8La89a\x08\xD3\x84\x8Aa&>V[a8C\x84\x86a&>V[Q\x90\x8A\x87aAtV[a7\x8BV[P\x93a9u\x96\x97P\x7F&\xB2\xAA\xE2e\x16\xE8q\x9E\xF5\x0E\xA2\xF6\x83\x1A.\xFB\xD4\xE3}\xCC\xDF\x0Fi6\xB2{\xC0\x8Ey>0\x95\x91\x93P\x91a9z\x92a8\x9D\x83`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R`@_ \x90V[T`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\x9F` R`@\x90 a8\xBF\x81Ta4\xD4V[\x90Ua8\xE9a8\xCCa\x06\xC0V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R\x96`\x01`\x01`\xA0\x1B\x03\x16` \x88\x01RV[`\x01`\x01`\xA0\x1B\x03\x84\x16`@\x87\x01R``\x86\x01RCc\xFF\xFF\xFF\xFF\x16`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01Ra9\x1C\x83a'aV[\x95\x86\x91a9Aa94\x84_R`\x9E` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[a9\\\x85a9W\x85_R`\xA4` R`@_ \x90V[a5\xE5V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\xA3` R`@\x90 \x90V[aL\xD8V[Pa9\x8B`@Q\x92\x83\x92\x86\x84a6\x85V[\x03\x90\xA1\x90V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[a\x07f\x92\x91a:\x08a:\x0E\x92aG?V[\x90aK#V[aK#V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x99` \x90\x81R`@\x91\x82\x90 `\x01\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x95\x90\x94\x16\x94\x85\x17\x90\x93UQ\x92\x83R\x91\x7Fw;T\xC0Muo\xCC^g\x81\x11\xF7\xD70\xDE;\xE9\x81\x92\0\x07\x99\xEE\xE3\xD67\x16\x05Z\x87\xC6\x91\x90\xA2V[\x91\x90\x91a:\x87a\x04\x99`\x01\x80`fT\x16\x14\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x9A` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x87\x16\x93\x84\x17\x90U\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x90\x80\xA3a:\xE0\x81a,4V[\x90\x91a:\xED\x83\x86\x83a3\x9FV[\x92_[\x81Q\x81\x10\x15a;7W`\x01\x90a;1`\x01`\x01`\xA0\x1B\x03a;\x11\x83\x86a&>V[Q\x16a;\x1D\x83\x88a&>V[Qa;(\x84\x8Aa&>V[Q\x91\x87\x8Ca<\x11V[\x01a:\xF0V[PPPPP\x90PV[\x91\x90`\x01`\x01`\xA0\x1B\x03\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a;sW`\x01`\x01`@\x1B\x03\x91P\x16\x90V[`@Qc\xA3\xD7^\t`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R` \x82\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06KWa\x07f\x92_\x92a;\xF0W[P`\x01`\x01`@\x1B\x03\x80g\r\xE0\xB6\xB3\xA7d\0\0a*\xE7V[a<\n\x91\x92P` =` \x11a\x0C\\Wa\x0CN\x81\x83a\x06\x9FV[\x90_a;\xD8V[\x90\x93\x80\x15a=%W`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x91a<\x86\x91a<x\x91a\x1E\x1C\x91a\x08\xF4\x90\x91\x89_\x84aJ#V[`@Q\x91\x82\x91\x86\x89\x84a4\xB2V[\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x9A` R`@\x90 T\x16a<\xAEW[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x98` R`@\x90 a<\xD0\x90\x83\x90a\x14\xCBV[\x80T\x93\x80\x85\x01\x80\x95\x11a\x18\xB7W\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x94a=\x19\x92U`@Q\x93\x84\x93`\x01\x80`\xA0\x1B\x03\x16\x96\x84a4\xB2V[\x03\x90\xA2_\x80\x80\x80a<\xA8V[c\n3\xBCi`\xE2\x1B_R`\x04_\xFD[\x91\x92\x90\x94\x80\x15a=%Wa<xa\x1E\x1C\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x93a\x08\xF4a<\x86\x94`\x01\x80`\xA0\x1B\x03\x8B\x16_R`\xA2` R\x89a=\x9B\x8A`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x93\x84aJ#V[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R3`$\x82\x01R0`D\x82\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x90\x91` \x91\x83\x91`\x84\x91\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xF1\x90\x81\x15a\x06KW_\x91a>\x1EWP\x90V[a\x07f\x91P` =` \x11a\x06DWa\x066\x81\x83a\x06\x9FV[\x91a>bc\xFF\xFF\xFF\xFF\x91`@\x93\x96\x95\x96`\x01\x80`\xA0\x1B\x03\x16\x85R``` \x86\x01R``\x85\x01\x90a\x0F\xC2V[\x94\x16\x91\x01RV[\x93\x92\x90\x91_\x81a>y\x81Qa%~V[\x94a>\x98`@Q\x95\x86\x93\x84\x93c%5\xF4\x03`\xE2\x1B\x85R`\x04\x85\x01a>7V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06KW_\x92a?\x04W[P_[\x81Q\x81\x10\x15a4ZW\x80a>\xF3a4(a\x08\xD3`\x01\x94\x86a&>V[a>\xFD\x82\x87a&>V[R\x01a>\xD7V[a?\x19\x91\x92P=\x80_\x83>a4w\x81\x83a\x06\x9FV[\x90_a>\xD4V[_\x19\x81\x01\x91\x90\x82\x11a\x18\xB7WV[\x91\x90\x82\x03\x91\x82\x11a\x18\xB7WV[c\xFF\xFF\xFF\xFF_\x19\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x18\xB7WV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x03\x90c\xFF\xFF\xFF\xFF\x82\x11a\x18\xB7WV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\xA5` \x90\x81R`@\x80\x83 \x94\x86\x16\x83R\x93\x90R\x91\x90\x91 \x90\x94\x93\x92\x91a?\xC7\x91a?\xA6\x90aJ\xB4V[\x95_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a?\xFAa\x17\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFFC\x16a?SV[\x81T\x91\x90c\xFF\xFF\xFF\xFF\x16_[\x83\x81\x10a@*WPP\x94a\x18'\x91a\x07f\x95\x96\x81\x15_\x14a\x18<WP_\x90Pa\x14\xFEV[\x90\x92\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a\x18\xB7W\x83_R\x84c\xFF\xFF\xFF\xFF\x83` _ \x01T\x16\x11_\x14a@`WP\x92[\x90a@\x06V[\x93\x91P`\x01\x81\x01\x80\x91\x11a\x18\xB7W\x90a@ZV[`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\x98` Ra@\xA2\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x91\x82T\x82\x81\x03\x90\x81\x11a\x18\xB7W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x93Ua@\xE2`@Q\x92\x83\x92_\x84a4\xB2V[\x03\x90\xA2V[\x91\x90\x91`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\x98` R`@_ s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0_R` R`@_ \x90\x81T\x91\x83\x83\x03\x92\x83\x11a\x18\xB7W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x93s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x93a@\xE2\x92U`@Q\x93\x84\x93\x84a4\xB2V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x98` \x90\x81R`@\x80\x83 \x94\x87\x16\x83R\x93\x90R\x91\x90\x91 \x80T\x91\x94\x80\x83\x03\x94\x93\x92\x85\x11a\x18\xB7W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x94a@\xE2\x92U`@Q\x93\x84\x93\x84a4\xB2V[`\x01`\x01`\xA0\x1B\x03\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x03aB.W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x07f\x92\x91aBlaBr\x92aG?V[\x90aK\xC3V[aK\xC3V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aB\x8BWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x15aC\x1EWV[c\x16\x11\r5`\xE2\x1B_R`\x04_\xFD[\x15aC4WV[c\x87\xC9\xD2\x19`\xE0\x1B_R`\x04_\xFD[\x15aCJWV[cx\xF6z\xE1`\xE1\x1B_R`\x04_\xFD[_`\x06a\x03\x9D\x92\x82\x81U\x82`\x01\x82\x01U\x82`\x02\x82\x01U\x82`\x03\x82\x01U\x82`\x04\x82\x01UaC\x8E\x83`\x05\x83\x01\x80T\x90\x82\x81Ua4\xFFV[\x01\x80T\x90\x82\x81Ua4\xFFV[5a\x07f\x81a\x03\x81V[\x91\x90\x82`@\x91\x03\x12a\x03CW` \x82Q\x92\x01Q\x90V[\x93\x92\x93`\xA0\x81\x01\x92aC\xCF\x84QQ\x82\x14a&\x1BV[`@\x82\x01QaC\xF1\x90aC\xEA\x90`\x01`\x01`\xA0\x1B\x03\x16a\x15$V[3\x14aC\x17V[aC\xFA\x82a'aV[aD\x1EaD\x19aD\x12\x83_R`\x9E` R`@_ \x90V[T`\xFF\x16\x90V[aC-V[\x7F\x1F@@\x08\x89'N\xD0{$\x84^PT\xA8z\x0C\xAB\x96\x9E\xB1'z\xAF\xE6\x1A\xE3R\xE7\xC3*\0aE\x0FaD\xB8aD\x80aDY`\x80\x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a)\x95V[aD\x97c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFFC\x16\x11aCCV[\x86Q`\x01`\x01`\xA0\x1B\x03\x16` \x88\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8AQ\x91a>iV[\x92aD\xD5\x81aD\xD0a9\\\x89Q`\x01\x80`\xA0\x1B\x03\x16\x90V[aM}V[PaD\xF0aD\xEB\x82_R`\xA4` R`@_ \x90V[aCYV[a\x18,aE\x05\x82_R`\x9E` R`@_ \x90V[\x80T`\xFF\x19\x16\x90UV[\x03\x90\xA1\x82Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9A` R`@\x90 aE3\x90a\x12zV[\x83QaEK\x90`\x01`\x01`\xA0\x1B\x03\x16\x82\x88Q\x91a3\x9FV[_[\x87Q\x80Q\x82\x10\x15aG2W\x90\x88\x88\x88\x83\x89\x8F\x96a\x1E\xF4a\x08\xD3\x84aEp\x93a&>V[aE\x86\x8Ba\x13\xD1\x85a\x13\xCA\x81`\xC0\x8A\x01Qa&>V[\x97\x15aF9W\x92Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93aE\xC3\x93aE\xBE\x93\x90\x92\x90\x91aE\xB8\x91a\x08\xD3\x91\x85\x91\x16\x99Qa&>V[\x95a/\x0FV[aC\x9AV[\x91\x81;\x15a\x03CW`@Qc\x0B\xAB\x90c`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x90\x84\x16`$\x82\x01R\x91\x90\x92\x16`D\x82\x01R`d\x81\x01\x93\x90\x93R_\x90\x83\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x06KW`\x01\x92aF%W[P[\x01aEMV[\x80a\n\xEE_aF3\x93a\x06\x9FV[_aF\x1DV[\x92aE\xBE\x83_\x93aE\xB8a\x08\xD3`@\x9A\x99\x97aF^aFe\x97Q`\x01\x80`\xA0\x1B\x03\x16\x90V[\x9AQa&>V[\x85Qc\xC4b>\xA1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x04\x82\x01R\x92\x85\x16`$\x84\x01R\x84\x16`D\x83\x01R`d\x82\x01\x96\x90\x96R\x94\x85\x92`\x84\x92\x84\x92\x91\x16Z\xF1\x80\x15a\x06KW\x89aF\xEA\x91`\x01\x94\x84\x8B_\x92_\x94aF\xEFW[PQaF\xD8\x91a\x08\xD3\x91`\x01`\x01`\xA0\x1B\x03\x16[\x95Qa&>V[aF\xE2\x86\x89a&>V[Q\x93\x89a=4V[aF\x1FV[a\x08\xD3\x91\x94PaF\xD1\x93P\x91aG\x1EaF\xD8\x93`@=\x81\x11aG+W[aG\x16\x81\x83a\x06\x9FV[\x81\x01\x90aC\xA4V[\x94\x90\x94\x95\x92PP\x91aF\xBDV[P=aG\x0CV[PPPPPPPPP\x90PV[Q\x80a\x07fWPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[a\x07f\x90aGga\x04\x99`\x02\x80`fT\x16\x14\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9A` R`@\x90 ``\x92\x91\x90aG\x8C\x90a\x12zV[\x90aG\xBAaG\xAA\x82`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x90\x82\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv_\x80\xA3aG\xF5\x81a,4V[\x91\x90\x94\x85Q\x90\x81\x15aH\xA8WPaH\x0B\x90a%~V[\x92aH\x17\x86\x82\x84a3\x9FV[\x91_[\x87Q\x81\x10\x15aH\x9EW`\x01\x90aH\x8D\x89aH2a&\x9EV[aH:a&\x9EV[\x90aH\\aHSa\x08\xD3\x87aHMa&\x9EV[\x96a&>V[a-\x98\x83a&1V[aHf\x85\x8Ba&>V[QaHp\x83a&1V[RaH{\x85\x8Aa&>V[QaH\x85\x84a&1V[R\x87\x87a6\xB0V[aH\x97\x82\x89a&>V[R\x01aH\x1AV[P\x93\x95PPPPPV[\x95PPPPPV[\x15aH\xB7WV[c\rLL\x91`\xE2\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x91\x94\x91\x16\x92\x90\x83\x15aIgWa\x03\x9D\x94aI]\x91\x85_R`\x9C` R`@_ \x81_R` RaI!aI\x1C`\xFF`@_ T\x16\x15\x15\x15\x90V[aH\xB0V[aIOa94\x82aIB\x89`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R`@_ \x90V[\x90_R` R`@_ \x90V[\x85` \x85\x01\x95\x86Q\x93a$\xBEV[\x90Q\x91Q\x92aL\x8EV[PPPPPV[\x90aBla\x07f\x92aG?V[\x90\x91\x90s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xBF\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01aI\xA8WPPPV[aI\xFE\x90`\x01\x80`\xA0\x1B\x03\x16\x92\x83_R`\xA5` RaI\xDDa\x17\x92\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x93_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x90\x82\x01\x80\x92\x11a\x18\xB7Wa\x03\x9D\x91`\x01`\x01`\xE0\x1B\x03\x16\x90Cc\xFF\xFF\xFF\xFF\x16\x90aO\xE8V[\x92\x90\x91\x82\x15aJfWaJE\x82aBraJ?a\x1E\x1C\x88a&\xC0V[\x86aK\xC3V[\x90\x80\x82\x01\x80\x92\x11a\x18\xB7W\x83\x01\x80\x93\x11a\x18\xB7Wa4\xFB\x92a:\x0E\x91aK#V[Pa4\xFB\x91PaL=V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x91\x90aJ\x92\x82\x82\x85aLWV[\x92\x82\x15aJ\xAFW\taJ\xA1W\x90V[`\x01\x81\x01\x80\x91\x11a\x18\xB7W\x90V[aJqV[\x80T\x90\x81aJ\xC3WP_\x91\x90PV[\x81_\x19\x81\x01\x11a\x18\xB7W_R_\x19\x90` _ \x01\x01T` \x1Ca\x18aV[\x91`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01`@\x1B\x03\x82\x11a\x18\xB7W`\x01`\x01`@\x1B\x03a\x07f\x92\x16\x90aK\xC3V[\x81\x15aJ\xAFW\x04\x90V[\x15a\x03CWV[_\x19g\r\xE0\xB6\xB3\xA7d\0\0\x82\t\x91g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x82\x80\x85\x10\x94\x03\x93\x80\x85\x03\x94\x14aK\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x82\x91aKc\x86\x84\x11aK\x1CV[\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x81`\x03\x02\x18\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x91\x02`\x02\x03\x02\x93`\x01\x83\x80_\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[P\x90a\x07f\x92PaK\x12V[_\x19\x82\x82\t\x82\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14aL,W\x81g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x03CW\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x93g\r\xE0\xB6\xB3\xA7d\0\0\x91\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x92PP\x04\x90V[\x80\x15aJ\xAFWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[\x90\x91_\x19\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14aL\x81W\x90\x82\x91aKc\x86\x84\x11aK\x1CV[PP\x90a\x07f\x92PaK\x12V[\x92B\x11aL\xB4WaL\x9E\x92aN\xBFV[\x15aL\xA5WV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[c\x08\x19\xBD\xCD`\xE0\x1B_R`\x04_\xFD[\x80T\x82\x10\x15a\x1C\x12W_R` _ \x01\x90_\x90V[`\x01\x81\x01\x90\x82_R\x81` R`@_ T\x15_\x14aM;W\x80T`\x01`@\x1B\x81\x10\x15a\x06\x7FWaM(aM\x12\x82`\x01\x87\x94\x01\x85U\x84aL\xC3V[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90UT\x91_R` R`@_ U`\x01\x90V[PPP_\x90V[\x80T\x80\x15aMiW_\x19\x01\x90aMX\x82\x82aL\xC3V[\x81T\x90_\x19\x90`\x03\x1B\x1B\x19\x16\x90UUV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`\x01\x81\x01\x91\x80_R\x82` R`@_ T\x92\x83\x15\x15_\x14aN\x18W_\x19\x84\x01\x84\x81\x11a\x18\xB7W\x83T_\x19\x81\x01\x94\x90\x85\x11a\x18\xB7W_\x95\x85\x83aIB\x94aM\xCB\x98\x03aM\xD1W[PPPaMBV[U`\x01\x90V[aN\x01aM\xFB\x91aM\xF2aM\xE8aN\x0F\x95\x88aL\xC3V[\x90T\x90`\x03\x1B\x1C\x90V[\x92\x83\x91\x87aL\xC3V[\x90a4\xE2V[\x85\x90_R` R`@_ \x90V[U_\x80\x80aM\xC3V[PPPP_\x90V[`\x05\x11\x15aN*WV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90``\x92` \x91\x83R`@\x82\x84\x01R\x80Q\x91\x82\x91\x82`@\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[=\x15aN\x9AW=\x90aN\x81\x82a\x1C\\V[\x91aN\x8F`@Q\x93\x84a\x06\x9FV[\x82R=_` \x84\x01>V[``\x90V[\x90\x81` \x91\x03\x12a\x03CWQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x03CW\x90V[\x91\x90\x91aN\xCC\x82\x84aP\xC1V[aN\xD5\x81aN V[\x15\x90\x81aOfW[PaO^W_\x92a'~aO\n\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01aN>V[Q\x91Z\xFAaO\x16aNpV[\x81aORW[\x81aO%WP\x90V[\x80Qc\x0B\x13]?`\xE1\x1B\x92P`\x01`\x01`\xE0\x1B\x03\x19\x91aOM\x91\x81\x01` \x90\x81\x01\x91\x01aN\x9FV[\x16\x14\x90V[\x80Q` \x14\x91PaO\x1CV[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_aN\xDDV[\x15aO\x83WV[c\x15\x1B\x8E?`\xE1\x1B_R`\x04_\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x06\x7FWaO\xAF\x91`\x01\x82\x01\x81UaL\xC3V[aO\xD5W\x81Q` \x92\x83\x01Q\x90\x92\x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80T\x80aP$W[PaP\x1Fa\x03\x9D\x93aP\x0FaP\x03a\x06\xCFV[c\xFF\xFF\xFF\xFF\x90\x95\x16\x85RV[`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01RV[aO\x92V[\x80_\x19\x81\x01\x11a\x18\xB7W\x81_Rc\xFF\xFF\xFF\xFFaP\x92aP\x89_\x19\x84` _ \x01\x01aP\x7FaPq`@Q\x92aPX\x84a\x06\x84V[T\x86\x81\x16\x90\x81\x85R` \x1C` \x85\x01Rc\xFF\xFF\xFF\xFF\x16\x90V[\x85\x89\x16\x95\x86\x91\x16\x11\x15aO|V[Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[\x03aO\xF0Wa\x03\x9D\x93\x92P\x90a\x18OaP\xAA\x92a? V[\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV[\x81Q`A\x81\x03aP\xEDWP\x90aP\xE9\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aQ/V[\x90\x91V[`@\x03aQ&W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a\x18\xB7WaP\xE9\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90aQ/V[PP_\x90`\x02\x90V[\x92\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11aQ\xCDW`\xFF\x16`\x1B\x81\x14\x15\x80aQ\xC2W[aQ\xB7W` \x93_\x93`@Q\x93\x84\x93`\x80\x85\x01\x93\x85R\x87\x85\x01R`@\x84\x01R``\x83\x01R\x83\x80R\x03\x90`\x01Z\xFA\x15a\x06KW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aQ\xAFW\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x81\x14\x15aQgV[PPPP_\x90`\x03\x90V\xFE\xA2dipfsX\"\x12 ;5\x9E@o\xA0+\x81\xFA\xAB\xCAg\xE6\x01\xF9\x10U\xD3\x9D\xDE\xF3\x16-\xB3(\x99\xCC\x12\x8E\xE4\xB6\xD0dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c806304a4f979146103345780630b9f487a1461032f5780630dd8dd021461032a578063136439dd1461032557806325df922e146103205780632aa6d8881461031b57806339b70e38146103165780633c651cf2146103115780633cdeb5e01461030c5780633e28391d146103075780634657e26a146103025780634665bcda146102fd57806354b7c96c146102f8578063595c6a67146102f3578063597b36da146102ee5780635ac86ab7146102e95780635c975abb146102e45780635d975e88146102df5780635dd68579146102da578063601bb36f146102d557806360a0d1ce146102d057806365da1264146102cb57806366d5ba93146102c65780636d70f7ae146102c15780636e174448146102bc578063715018a6146102b7578063778e55f3146102b257806378296ec5146102ad578063886f1195146102a85780638da5cb5b146102a3578063900413471461029e5780639104c319146102995780639435bb4314610294578063a17884841461028f578063a33a34331461028a578063b7f06ebe14610285578063bb45fef214610280578063bfae3fd21461027b578063c448feb814610276578063c978f7ac14610271578063ca8aa7c71461026c578063cd6dc68714610267578063da8be86414610262578063e4cc3f901461025d578063eea9064b14610258578063f0e0e67614610253578063f2fde38b1461024e578063f698da2514610249578063fabc1cbc146102445763fd8aa88d1461023f575f80fd5b612449565b612385565b61236b565b6122da565b612219565b6121f1565b612157565b612125565b61203b565b611ff7565b611e86565b611e21565b611ddb565b611d8d565b611d5e565b611d2b565b611c21565b611b0f565b611ae1565b611ab3565b611a53565b611a0f565b611980565b61193c565b6118e1565b6116e5565b611699565b611649565b611606565b611592565b61142d565b611209565b611081565b610fa5565b610f72565b610f38565b610e1f565b610dcb565b610d87565b610d43565b610cf8565b610ca4565b610b3e565b610afa565b610985565b610814565b610590565b610457565b61039f565b610347565b5f91031261034357565b5f80fd5b34610343575f3660031901126103435760206040517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad8152f35b6001600160a01b0381160361034357565b359061039d82610381565b565b346103435760a03660031901126103435760206103e66004356103c181610381565b6024356103cd81610381565b6044356103d981610381565b60643591608435936124be565b604051908152f35b9181601f84011215610343578235916001600160401b038311610343576020808501948460051b01011161034357565b60206040818301928281528451809452019201905f5b8181106104415750505090565b8251845260209384019390920191600101610434565b34610343576020366003190112610343576004356001600160401b038111610343576104879036906004016103ee565b9061049f610499600280606654161490565b15612568565b6104a88261257e565b335f908152609a60205260409020549092906001600160a01b03165f5b8281106104de57604051806104da878261041e565b0390f35b8061051b6104f86104f260019487896125c4565b806125e6565b905061051261050884888a6125c4565b60208101906125e6565b9190501461261b565b61057f6105406105396105326104f285898b6125c4565b36916106f5565b853361339f565b866105778761056f6105656105088861055d6104f282878a6125c4565b9590976125c4565b94909236916106f5565b923691610769565b9086336136b0565b610589828861263e565b52016104c5565b346103435760203660031901126103435760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561064b5761061a92610606915f9161061c575b50612672565b61061560665482811614612688565b6139c5565b005b61063e915060203d602011610644575b610636818361069f565b810190612652565b5f610600565b503d61062c565b612667565b634e487b7160e01b5f52604160045260245ffd5b60e081019081106001600160401b0382111761067f57604052565b610650565b604081019081106001600160401b0382111761067f57604052565b90601f801991011681019081106001600160401b0382111761067f57604052565b6040519061039d60e08361069f565b6040519061039d60408361069f565b6001600160401b03811161067f5760051b60200190565b929190610701816106de565b9361070f604051958661069f565b602085838152019160051b810192831161034357905b82821061073157505050565b60208091833561074081610381565b815201910190610725565b9080601f8301121561034357816020610766933591016106f5565b90565b929190610775816106de565b93610783604051958661069f565b602085838152019160051b810192831161034357905b8282106107a557505050565b8135815260209182019101610799565b9080601f830112156103435781602061076693359101610769565b90602080835192838152019201905f5b8181106107ed5750505090565b82518452602093840193909201916001016107e0565b9060206107669281815201906107d0565b346103435760603660031901126103435760043561083181610381565b6024356001600160401b0381116103435761085090369060040161074b565b906044356001600160401b038111610343576108709036906004016107b5565b6001600160a01b038281165f818152609a60205260409020549093610898928692169061339f565b6108a2845161257e565b935f5b815181101561092657600190855f5260a26020526109156108f96108f460405f206108e06108d3868961263e565b516001600160a01b031690565b60018060a01b03165f5260205260405f2090565b6126c0565b610903838861263e565b5161090e848861263e565b51916139f7565b61091f828961263e565b52016108a5565b604051806104da8882610803565b6024359063ffffffff8216820361034357565b359063ffffffff8216820361034357565b9181601f84011215610343578235916001600160401b038311610343576020838186019501011161034357565b34610343576060366003190112610343576004356109a281610381565b6109aa610934565b6044356001600160401b038111610343576109c9903690600401610958565b335f908152609a60205260409020549193916109ee906001600160a01b0316156126e3565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561034357604051632b6241f360e11b815233600482015263ffffffff9490941660248501525f908490604490829084905af191821561064b577f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b670809093610adb93610ae0575b50610a8a8133613a13565b610a943333613a73565b6040516001600160a01b0391909116815233907fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c190602090a26040519182913395836126f9565b0390a2005b80610aee5f610af49361069f565b80610339565b5f610a7f565b34610343575f366003190112610343576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461034357608036600319011261034357600435610b5b81610381565b602435610b6781610381565b6064356044356001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001633148015610c72575b15610c63576001600160a01b038481165f908152609a602090815260409182902054915163152667d960e31b81529183166004830181905286841660248401529196919592879060449082907f0000000000000000000000000000000000000000000000000000000000000000165afa95861561064b5761061a96610c2e915f91610c34575b508383613b40565b94613d34565b610c56915060203d602011610c5c575b610c4e818361069f565b810190612720565b5f610c26565b503d610c44565b63045206a560e21b5f5260045ffd5b50337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614610ba0565b34610343576020366003190112610343576020610ce6600435610cc681610381565b6001600160a01b039081165f908152609960205260409020600101541690565b6040516001600160a01b039091168152f35b34610343576020366003190112610343576020610d39600435610d1a81610381565b6001600160a01b039081165f908152609a602052604090205416151590565b6040519015158152f35b34610343575f366003190112610343576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610343575f366003190112610343576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103435760403660031901126103435761061a600435610deb81610381565b60243590610df882610381565b610e09610e0482613da2565b612735565b610e1a610e1582612e1d565b61274b565b613a13565b34610343575f3660031901126103435760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561064b57610e8a915f9161061c5750612672565b61061a613991565b91909160e08184031261034357610ea76106c0565b92610eb182610392565b8452610ebf60208301610392565b6020850152610ed060408301610392565b604085015260608201356060850152610eeb60808301610947565b608085015260a08201356001600160401b0381116103435781610f0f91840161074b565b60a085015260c08201356001600160401b03811161034357610f3192016107b5565b60c0830152565b34610343576020366003190112610343576004356001600160401b038111610343576103e6610f6d6020923690600401610e92565b612761565b346103435760203660031901126103435760043560ff81168091036103435760016020911b806066541614604051908152f35b34610343575f366003190112610343576020606654604051908152f35b90602080835192838152019201905f5b818110610fdf5750505090565b82516001600160a01b0316845260209384019390920191600101610fd2565b80516001600160a01b039081168352602080830151821690840152604080830151909116908301526060808201519083015260808082015163ffffffff16908301526107669160c061105f60a084015160e060a085015260e0840190610fc2565b9201519060c08184039101526107d0565b906020610766928181520190610ffe565b346103435760203660031901126103435760043561109d61278c565b505f5260a46020526104da60405f206111336006604051926110be84610664565b80546001600160a01b0316845260018101546001600160a01b0316602085015260028101546001600160a01b031660408501526003810154606085015261111c61110f600483015463ffffffff1690565b63ffffffff166080860152565b611128600582016127c2565b60a085015201612813565b60c082015260405191829182611070565b9080602083519182815201916020808360051b8301019401925f915b83831061116f57505050505090565b909192939460208061118d600193601f1986820301875289516107d0565b97019301930191939290611160565b929160408401936040815282518095526060810194602060608260051b8401019401905f5b8181106111de575050506107669394506020818403910152611144565b9091946020806111fa600193605f19888203018c528951610ffe565b970198019101969190966111c1565b346103435760203660031901126103435760043561122681610381565b6001600160a01b0381165f90815260a36020526040902061124690612813565b805190611252826128e9565b9161125c81612938565b9361128761127a8260018060a01b03165f52609a60205260405f2090565b546001600160a01b031690565b915f927f00000000000000000000000000000000000000000000000000000000000000009263ffffffff4316905b8386106112cb57604051806104da8b8b8361119c565b6112f16112ec6112dd888a9c9a61263e565b515f5260a460205260405f2090565b61285b565b6112fb878a61263e565b52611306868961263e565b5061131f60a0611316888b61263e565b5101515161257e565b611329878961263e565b52611334868861263e565b506113598561135460806113488a8d61263e565b51015163ffffffff1690565b612995565b8263ffffffff8216105f146113fc576113829060a0611378898c61263e565b5101518584613e69565b945b5f5b60a0611392898c61263e565b510151518110156113eb5780896113e4826113de8c8f8d6113d1856113ca60019b60c06113c2886113d89861263e565b51015161263e565b519261263e565b5190614bc3565b9461263e565b5161263e565b5201611386565b5096989660019096019594506112b5565b5061141660a061140c888b61263e565b510151848361339f565b94611384565b6001600160401b0381160361034357565b346103435760803660031901126103435760043561144a81610381565b60243561145681610381565b6044356114628161141c565b6064359061146f8261141c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303611583578261151261150c611518946115046114e0856114cb8b60018060a01b03165f52609860205260405f2090565b9060018060a01b03165f5260205260405f2090565b546114fe6001600160401b0388166001600160401b03851683614a85565b90613f2e565b948489613f6d565b836129bd565b94614074565b611530611524826141de565b6001600160a01b031690565b91823b156103435760405163debe1eab60e01b81526001600160a01b039290921660048301526024820152905f908290604490829084905af1801561064b5761157557005b80610aee5f61061a9361069f565b6323d871a560e01b5f5260045ffd5b34610343576060366003190112610343576004356115af81610381565b6044356024356115be8261141c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036115f75761061a926129ca565b633213a66160e21b5f5260045ffd5b346103435760203660031901126103435760043561162381610381565b60018060a01b03165f52609a602052602060018060a01b0360405f205416604051908152f35b346103435760203660031901126103435761168b6104da61167460043561166f81610381565b612c34565b604092919251938493604085526040850190610fc2565b9083820360208501526107d0565b34610343576020366003190112610343576020610d396004356116bb81610381565b612e1d565b6040906003190112610343576004356116d881610381565b9060243561076681610381565b34610343576116f3366116c0565b60405163152667d960e31b81526001600160a01b0380841660048301528216602482015291602083806044810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa92831561064b575f936118bc575b506117b89060018060a01b031691825f5260a56020526117976117928260405f209060018060a01b03165f5260205260405f2090565b614ab4565b925f5260a560205260405f209060018060a01b03165f5260205260405f2090565b6117f06117eb7f000000000000000000000000000000000000000000000000000000000000000063ffffffff4316613f53565b613f3b565b8154919063ffffffff165f5b83811061186d576104da61182c5f886118278989898161183c5750506001600160e01b0384166114fe565b614ae1565b6040519081529081906020820190565b61185a6118619161184f6114fe94613f20565b905f5260205f200190565b5460201c90565b6001600160e01b031690565b90928082169080831860011c82018092116118b757835f528463ffffffff8360205f20015416115f146118a35750925b906117fc565b939150600181018091116118b7579061189d565b612981565b6117b89193506118da9060203d602011610c5c57610c4e818361069f565b929061175c565b34610343575f366003190112610343576118f9614277565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461034357602061197761194f366116c0565b6001600160a01b039182165f9081526098855260408082209290931681526020919091522090565b54604051908152f35b346103435760403660031901126103435760043561199d81610381565b6024356001600160401b038111610343576119dd7f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b6708090913690600401610958565b90926119eb610e0482613da2565b6119f7610e1582612e1d565b610adb60405192839260018060a01b031695836126f9565b34610343575f366003190112610343576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610343575f366003190112610343576033546040516001600160a01b039091168152602090f35b90604060031983011261034357600435611a9481610381565b91602435906001600160401b038211610343576107669160040161074b565b34610343576104da611acd611ac736611a7b565b90612e50565b6040519182916020835260208301906107d0565b34610343575f36600319011261034357602060405173beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac08152f35b34610343576060366003190112610343576004356001600160401b03811161034357611b3f9036906004016103ee565b6024356001600160401b03811161034357611b5e9036906004016103ee565b916044356001600160401b03811161034357611b819093919336906004016103ee565b90611b93610499600480606654161490565b611ba2600260c9541415612ec3565b600260c9553686900360de1901925f5b86811015611c17578060051b908189013591868312156103435783821015611c1257600192611be6611c0c928a018a6125e6565b90611c078d611bfe611bf9888d8d612f0f565b612f1f565b94369101610e92565b6143ba565b01611bb2565b6125b0565b61061a600160c955565b3461034357602036600319011261034357600435611c3e81610381565b60018060a01b03165f52609f602052602060405f2054604051908152f35b6001600160401b03811161067f57601f01601f191660200190565b90606060031983011261034357600435611c9081610381565b91602435906001600160401b03821161034357604082820360031901126103435760405191611cbe83610684565b80600401356001600160401b0381116103435781019180602384011215610343576004830135611ced81611c5c565b91611cfb604051938461069f565b81835260248583010111610343576020815f92602480970183860137830101528352013560208201529060443590565b34610343576104da611d52611d3f36611c77565b90611d4c93929333612f9d565b9361309c565b6040519182918261041e565b34610343576020366003190112610343576004355f52609e602052602060ff60405f2054166040519015158152f35b3461034357604036600319011261034357600435611daa81610381565b6024359060018060a01b03165f52609c60205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103435760206103e6611e1c6108f4611df4366116c0565b6001600160a01b039182165f90815260a2875260408082209290931681526020919091522090565b61473f565b34610343575f36600319011261034357602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b9091611e78610766936040845260408401906107d0565b9160208184039101526107d0565b3461034357611e9436611a7b565b611e9e815161257e565b611ea8825161257e565b91611ed081611eca61127a8760018060a01b03165f52609a60205260405f2090565b8661339f565b5f5b8251811015611fe557806020611ef9611524611ef46108d3611f3a968961263e565b6141de565b611f066108d3848861263e565b60405163fe243a1760e01b81526001600160a01b03808c166004830152909116602482015293849190829081906044820190565b03915afa801561064b576001925f91611fb7575b50611f59828861263e565b52611fa6611f8a6108f4611f7d8a60018060a01b03165f5260a260205260405f2090565b6108e06108d3868a61263e565b611f94838961263e565b51611f9f848761263e565b519161425b565b611fb0828761263e565b5201611ed2565b611fd8915060203d8111611fde575b611fd0818361069f565b810190612c25565b5f611f4e565b503d611fc6565b5050506104da60405192839283611e61565b34610343575f366003190112610343576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103435760403660031901126103435760043561205881610381565b61209d6024355f549261208360ff600886901c161580958196612117575b81156120f7575b50612f29565b83612094600160ff195f5416175f55565b6120e057612f8c565b6120a357005b6120b161ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b6120f261010061ff00195f5416175f55565b612f8c565b303b15915081612109575b505f61207d565b60ff1660011490505f612102565b600160ff8216109150612076565b34610343576020366003190112610343576104da611d5260043561214881610381565b612f9d565b8015150361034357565b34610343576060366003190112610343576004356001600160401b0381116103435760e0600319823603011261034357602435906001600160401b038211610343576121aa6121ea9236906004016103ee565b90611c07604435936121bb8561214d565b6121cc610499600480606654161490565b6121db600260c9541415612ec3565b600260c9553690600401610e92565b600160c955005b346103435761061a61220236611c77565b9161309c565b906020610766928181520190611144565b34610343576040366003190112610343576004356001600160401b038111610343573660238201121561034357806004013590612255826106de565b91612263604051938461069f565b8083526024602084019160051b8301019136831161034357602401905b8282106122c057836024356001600160401b038111610343576104da916122ae6122b492369060040161074b565b906131a7565b60405191829182612208565b6020809183356122cf81610381565b815201910190612280565b34610343576020366003190112610343576004356122f781610381565b6122ff614277565b6001600160a01b038116156123175761061a906142cf565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b34610343575f3660031901126103435760206103e66131fa565b346103435760203660031901126103435760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561064b575f9161240e575b506001600160a01b031633036123ff5761061a906132b7565b63794821ff60e01b5f5260045ffd5b90506020813d602011612441575b816124296020938361069f565b81010312610343575161243b81610381565b5f6123e6565b3d915061241c565b346103435760203660031901126103435760043561246681610381565b60018060a01b03165f5260a360205260405f206040519081602082549182815201915f5260205f20905f5b8181106124a8576104da85611d528187038261069f565b8254845260209093019260019283019201612491565b604080517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad602082019081526001600160a01b0395861692820192909252918416606083015292909116608082015260a081019290925260c08083019390935291815261252c60e08261069f565b5190206125376131fa565b9060405190602082019261190160f01b8452602283015260428201526042815261256260628261069f565b51902090565b1561256f57565b63840a48d560e01b5f5260045ffd5b90612588826106de565b612595604051918261069f565b82815280926125a6601f19916106de565b0190602036910137565b634e487b7160e01b5f52603260045260245ffd5b9190811015611c125760051b81013590605e1981360301821215610343570190565b903590601e198136030182121561034357018035906001600160401b03821161034357602001918160051b3603831361034357565b1561262257565b6343714afd60e01b5f5260045ffd5b805115611c125760200190565b8051821015611c125760209160051b010190565b9081602091031261034357516107668161214d565b6040513d5f823e3d90fd5b1561267957565b631d77d47760e21b5f5260045ffd5b1561268f57565b63c61dca5d60e01b5f5260045ffd5b604080519091906126af838261069f565b6001815291601f1901366020840137565b90604051602081018181106001600160401b0382111761067f5760405291548252565b156126ea57565b633bf2b50360e11b5f5260045ffd5b90918060409360208452816020850152848401375f828201840152601f01601f1916010190565b9081602091031261034357516107668161141c565b1561273c57565b63932d94f760e01b5f5260045ffd5b1561275257565b6325ec6c1f60e01b5f5260045ffd5b6040516125628161277e6020820194602086526020860190610ffe565b03601f19810183528261069f565b6040519061279982610664565b606060c0835f81525f60208201525f60408201525f838201525f60808201528260a08201520152565b90604051918281549182825260208201905f5260205f20925f5b8181106127f157505061039d9250038361069f565b84546001600160a01b03168352600194850194879450602090930192016127dc565b90604051918281549182825260208201905f5260205f20925f5b81811061284257505061039d9250038361069f565b845483526001948501948794506020909301920161282d565b9060405161286881610664565b82546001600160a01b039081168252600184015416602082015291829060c0906128e49060069060028101546001600160a01b03166040860152600381015460608601526128cd6128c0600483015463ffffffff1690565b63ffffffff166080870152565b6128d9600582016127c2565b60a086015201612813565b910152565b906128f3826106de565b612900604051918261069f565b8281528092612911601f19916106de565b01905f5b82811061292157505050565b60209061292c61278c565b82828501015201612915565b90612942826106de565b61294f604051918261069f565b8281528092612960601f19916106de565b01905f5b82811061297057505050565b806060602080938501015201612964565b634e487b7160e01b5f52601160045260245ffd5b9063ffffffff8091169116019063ffffffff82116118b757565b90600182018092116118b757565b919082018092116118b757565b6001600160a01b038181165f908152609a60205260409020541615612b21576001600160a01b0381165f908152609a60205260409020612a099061127a565b60405163152667d960e31b81526001600160a01b038216600482015273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac060248201529092602082806044810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561064b5761039d95612af6935f93612afc575b50612af090612ad36108f4612ab28860018060a01b03165f5260a260205260405f2090565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac05f5260205260405f2090565b936001600160401b0380670de0b6b3a76400005b93169116614c57565b9161425b565b916140e7565b612af0919350612b1a9060203d602011610c5c57610c4e818361069f565b9290612a8d565b505050565b9080601f83011215610343578151612b3d816106de565b92612b4b604051948561069f565b81845260208085019260051b82010192831161034357602001905b828210612b735750505090565b8151815260209182019101612b66565b9190916040818403126103435780516001600160401b03811161034357810183601f8201121561034357805190612bb9826106de565b91612bc7604051938461069f565b80835260208084019160051b8301019186831161034357602001905b828210612c0b575050509260208201516001600160401b038111610343576107669201612b26565b602080918351612c1a81610381565b815201910190612be3565b90816020910312610343575190565b6040516394f649dd60e01b81526001600160a01b038216600482015291905f83806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561064b575f935f92612def575b5060405163fe243a1760e01b81526001600160a01b03909116600482015273beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac060248201529060208280604481015b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561064b575f92612dce575b508115612dc957612d2e612d2985516129af565b61257e565b93612d3c612d2982516129af565b92612d64612d4b83518861263e565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac09052565b612d6f82518561263e565b525f5b8151811015612dc35780612da7612d8e6108d36001948661263e565b612d98838a61263e565b6001600160a01b039091169052565b612db1818561263e565b51612dbc828761263e565b5201612d72565b50505090565b919050565b612de891925060203d602011611fde57611fd0818361069f565b905f612d15565b60209450612cd99250612e13903d805f833e612e0b818361069f565b810190612b83565b9490949250612c97565b6001600160a01b03168015159081612e33575090565b5f818152609a60205260409020546001600160a01b031614919050565b919091612e5d835161257e565b905f5b8451811015612ebc576001600160a01b038281165f90815260986020526040902060019291612eaa9190612e94848a61263e565b511660018060a01b03165f5260205260405f2090565b54612eb5828661263e565b5201612e60565b5090925050565b15612eca57565b60405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606490fd5b9190811015611c125760051b0190565b356107668161214d565b15612f3057565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b612f9861039d926139c5565b6142cf565b6001600160a01b038082165f908152609a6020526040902054161561308d57612fc581612e1d565b61307e576001600160a01b0381169033829003612fe7575b6107669150614752565b6001600160a01b0381165f908152609a60205260409020546001600160a01b031661301181613da2565b8015613059575b1561304a57610766927ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a5f80a3612fdd565b631e499a2360e11b5f5260045ffd5b506001600160a01b038181165f90815260996020526040902060010154163314613018565b6311ca333560e31b5f5260045ffd5b63a5c7c44560e01b5f5260045ffd5b335f908152609a60205260409020549093926130da9290916130c7906001600160a01b0316156126e3565b6130d3610e1586612e1d565b84336148c6565b6130eb610499600180606654161490565b335f908152609a6020526040902080546001600160a01b0319166001600160a01b0384161790556001600160a01b038216337fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433045f80a361314a33612c34565b61315582853361339f565b915f5b815181101561319f576001906131996001600160a01b03613179838661263e565b5116613185838761263e565b51613190848961263e565b5191338b613c11565b01613158565b505050509050565b906131b28251612938565b915f5b8151811015612dc3576001906131de846001600160a01b036131d7848761263e565b5116612e50565b6131e8828761263e565b526131f3818661263e565b50016131b5565b467f000000000000000000000000000000000000000000000000000000000000000003613245577f000000000000000000000000000000000000000000000000000000000000000090565b600a602060405161325760408261069f565b828152016922b4b3b2b72630bcb2b960b11b81522060405160208101917f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866835260408201524660608201523060808201526080815261256260a08261069f565b6132c8606654198219811614612688565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2565b602081830312610343578051906001600160401b03821161034357019080601f8301121561034357815161332d816106de565b9261333b604051948561069f565b81845260208085019260051b82010192831161034357602001905b8282106133635750505090565b6020809183516133728161141c565b815201910190613356565b6001600160a01b03909116815260406020820181905261076692910190610fc2565b92916133cd905f816133b1815161257e565b94604051948592839263547afb8760e01b84526004840161337d565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561064b575f92613462575b505f5b815181101561345a57806134496134286108d36001948661263e565b613442613435848861263e565b516001600160401b031690565b9089613b40565b613453828761263e565b520161340c565b509193505050565b61347f9192503d805f833e613477818361069f565b8101906132fa565b905f613409565b1561348d57565b6339b190bb60e11b5f5260045ffd5b156134a357565b63796cc52560e01b5f5260045ffd5b6001600160a01b03918216815291166020820152604081019190915260600190565b5f1981146118b75760010190565b916134fb9183549060031b91821b915f19901b19161790565b9055565b91909182821061350e57505050565b5f5260205f2091820191015b818110613525575050565b5f815560010161351a565b90600160401b811161067f57815481835561039d926134ff565b8151916001600160401b03831161067f576020906135688484613530565b01905f5260205f205f5b83811061357f5750505050565b82516001600160a01b031681830155602090920191600101613572565b8151916001600160401b03831161067f576020906135ba8484613530565b01905f5260205f205f5b8381106135d15750505050565b6001906020845194019381840155016135c4565b815181546001600160a01b039182166001600160a01b03199182161783556020840151600184018054918416918316919091179055604084015160028401805491909316911617905560608201516003820155608082015161039d9260069160c0919061366b9063ffffffff16600486019063ffffffff1663ffffffff19825416179055565b61367c60a08201516005860161354a565b0151910161359c565b916136a29061076694928452606060208501526060840190610ffe565b9160408184039101526107d0565b929493906136c86001600160a01b0385161515613486565b6136d48351151561349c565b6136de835161257e565b906136e9845161257e565b5f5b855181101561385157866137718a6137556137428561373c6108f48d6108e06108d38561373c613721611ef46108d3848861263e565b6001600160a01b03909d165f90815260a26020526040902090565b9361263e565b5161374d868b61263e565b51908361425b565b61375f858761263e565b5261376a848d61263e565b519061496e565b61377b838761263e565b526001600160a01b038416613807575b6001600160a01b0316906137a26108d3828961263e565b6137ac828c61263e565b51833b15610343576137d9935f92838c6040519788958694859363724af42360e01b8552600485016134b2565b03925af191821561064b576001926137f3575b50016136eb565b80610aee5f6138019361069f565b5f6137ec565b6138296138176108d3848a61263e565b613821848861263e565b51908661497b565b61384c6138396108d3848a61263e565b613843848661263e565b51908a87614174565b61378b565b50936139759697507f26b2aae26516e8719ef50ea2f6831a2efbd4e37dccdf0f6936b27bc08e793e30959193509161397a9261389d8360018060a01b03165f52609f60205260405f2090565b546001600160a01b0384165f908152609f602052604090206138bf81546134d4565b90556138e96138cc6106c0565b6001600160a01b0386168152966001600160a01b03166020880152565b6001600160a01b038416604087015260608601524363ffffffff16608086015260a085015260c084015261391c83612761565b958691613941613934845f52609e60205260405f2090565b805460ff19166001179055565b61395c85613957855f5260a460205260405f2090565b6135e5565b6001600160a01b03165f90815260a36020526040902090565b614cd8565b5061398b6040519283928684613685565b0390a190565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b6107669291613a08613a0e9261473f565b90614b23565b614b23565b6001600160a01b039081165f8181526099602090815260409182902060010180546001600160a01b0319169590941694851790935551928352917f773b54c04d756fcc5e678111f7d730de3be98192000799eee3d63716055a87c69190a2565b919091613a87610499600180606654161490565b6001600160a01b038181165f818152609a6020526040812080546001600160a01b03191693871693841790557fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433049080a3613ae081612c34565b9091613aed83868361339f565b925f5b8151811015613b3757600190613b316001600160a01b03613b11838661263e565b5116613b1d838861263e565b51613b28848a61263e565b5191878c613c11565b01613af0565b50505050509050565b91906001600160a01b031673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014613b73576001600160401b0391501690565b60405163a3d75e0960e01b81526001600160a01b039092166004830152602082806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561064b57610766925f92613bf0575b506001600160401b0380670de0b6b3a7640000612ae7565b613c0a91925060203d602011610c5c57610c4e818361069f565b905f613bd8565b90938015613d25576001600160a01b038581165f90815260a2602090815260408083209387168352929052207f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f91613c8691613c7891611e1c916108f49091895f84614a23565b6040519182918689846134b2565b0390a16001600160a01b038085165f908152609a602052604090205416613cae575b50505050565b6001600160a01b0381165f908152609860205260409020613cd09083906114cb565b8054938085018095116118b7577f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c94613d19925560405193849360018060a01b031696846134b2565b0390a25f808080613ca8565b630a33bc6960e21b5f5260045ffd5b919290948015613d2557613c78611e1c7f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f936108f4613c869460018060a01b038b165f5260a260205289613d9b8a60405f209060018060a01b03165f5260205260405f2090565b9384614a23565b604051631beb2b9760e31b81526001600160a01b0391821660048201523360248201523060448201525f80356001600160e01b03191660648301529091602091839160849183917f0000000000000000000000000000000000000000000000000000000000000000165af190811561064b575f91613e1e575090565b610766915060203d60201161064457610636818361069f565b91613e6263ffffffff9160409396959660018060a01b03168552606060208601526060850190610fc2565b9416910152565b939290915f81613e79815161257e565b94613e986040519586938493632535f40360e21b855260048501613e37565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561064b575f92613f04575b505f5b815181101561345a5780613ef36134286108d36001948661263e565b613efd828761263e565b5201613ed7565b613f199192503d805f833e613477818361069f565b905f613ed4565b5f198101919082116118b757565b919082039182116118b757565b63ffffffff5f199116019063ffffffff82116118b757565b9063ffffffff8091169116039063ffffffff82116118b757565b6001600160a01b039081165f81815260a5602090815260408083209486168352939052919091209094939291613fc791613fa690614ab4565b955f5260a560205260405f209060018060a01b03165f5260205260405f2090565b613ffa6117eb7f000000000000000000000000000000000000000000000000000000000000000063ffffffff4316613f53565b8154919063ffffffff165f5b83811061402a5750509461182791610766959681155f1461183c57505f90506114fe565b90928082169080831860011c82018092116118b757835f528463ffffffff8360205f20015416115f146140605750925b90614006565b939150600181018091116118b7579061405a565b60018060a01b031691825f5260986020526140a28260405f209060018060a01b03165f5260205260405f2090565b9182548281039081116118b7577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd93556140e26040519283925f846134b2565b0390a2565b91909160018060a01b031691825f52609860205260405f2073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac05f5260205260405f20908154918383039283116118b7577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd9373beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0936140e29255604051938493846134b2565b6001600160a01b039081165f8181526098602090815260408083209487168352939052919091208054919480830394939285116118b7577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd946140e29255604051938493846134b2565b6001600160a01b031673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac00361422e577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b610766929161426c6142729261473f565b90614bc3565b614bc3565b6033546001600160a01b0316330361428b57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b1561431e57565b6316110d3560e21b5f5260045ffd5b1561433457565b6387c9d21960e01b5f5260045ffd5b1561434a57565b6378f67ae160e11b5f5260045ffd5b5f600661039d9282815582600182015582600282015582600382015582600482015561438e83600583018054908281556134ff565b018054908281556134ff565b3561076681610381565b9190826040910312610343576020825192015190565b93929360a08101926143cf845151821461261b565b60408201516143f1906143ea906001600160a01b0316611524565b3314614317565b6143fa82612761565b61441e614419614412835f52609e60205260405f2090565b5460ff1690565b61432d565b7f1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a0061450f6144b8614480614459608088015163ffffffff1690565b7f000000000000000000000000000000000000000000000000000000000000000090612995565b61449763ffffffff821663ffffffff431611614343565b86516001600160a01b031660208801516001600160a01b03168a5191613e69565b926144d5816144d061395c895160018060a01b031690565b614d7d565b506144f06144eb825f5260a460205260405f2090565b614359565b61182c614505825f52609e60205260405f2090565b805460ff19169055565b0390a182516001600160a01b03165f908152609a602052604090206145339061127a565b835161454b906001600160a01b03168288519161339f565b5f5b87518051821015614732579088888883898f96611ef46108d3846145709361263e565b6145868b6113d1856113ca8160c08a015161263e565b97156146395792516001600160a01b03938416936145c3936145be93909290916145b8916108d391859116995161263e565b95612f0f565b61439a565b91813b1561034357604051630bab906360e21b81526001600160a01b039485166004820152908416602482015291909216604482015260648101939093525f908390608490829084905af191821561064b57600192614625575b505b0161454d565b80610aee5f6146339361069f565b5f61461d565b926145be835f936145b86108d360409a999761465e614665975160018060a01b031690565b9a5161263e565b855163c4623ea160e01b81526001600160a01b0395861660048201529285166024840152841660448301526064820196909652948592608492849291165af1801561064b57896146ea91600194848b5f925f946146ef575b50516146d8916108d3916001600160a01b03165b955161263e565b6146e2868961263e565b519389613d34565b61461f565b6108d39194506146d193509161471e6146d89360403d811161472b575b614716818361069f565b8101906143a4565b94909495925050916146bd565b503d61470c565b5050505050505050509050565b51806107665750670de0b6b3a764000090565b61076690614767610499600280606654161490565b6001600160a01b0381165f908152609a60205260409020606092919061478c9061127a565b906147ba6147aa8260018060a01b03165f52609a60205260405f2090565b80546001600160a01b0319169055565b6001600160a01b038281169082167ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af446765f80a36147f581612c34565b91909485519081156148a8575061480b9061257e565b9261481786828461339f565b915f5b875181101561489e5760019061488d8961483261269e565b61483a61269e565b9061485c6148536108d38761484d61269e565b9661263e565b612d9883612631565b614866858b61263e565b5161487083612631565b5261487b858a61263e565b5161488584612631565b5287876136b0565b614897828961263e565b520161481a565b5093955050505050565b955050505050565b156148b757565b630d4c4c9160e21b5f5260045ffd5b6001600160a01b038281165f9081526099602052604090206001015491949116929083156149675761039d9461495d91855f52609c60205260405f20815f5260205261492161491c60ff60405f20541615151590565b6148b0565b61494f613934826149428960018060a01b03165f52609c60205260405f2090565b905f5260205260405f2090565b8560208501958651936124be565b9051915192614c8e565b5050505050565b9061426c6107669261473f565b90919073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeabf196001600160a01b038416016149a857505050565b6149fe9060018060a01b031692835f5260a56020526149dd6117928260405f209060018060a01b03165f5260205260405f2090565b935f5260a560205260405f209060018060a01b03165f5260205260405f2090565b9082018092116118b75761039d916001600160e01b0316904363ffffffff1690614fe8565b9290918215614a6657614a4582614272614a3f611e1c886126c0565b86614bc3565b908082018092116118b75783018093116118b7576134fb92613a0e91614b23565b506134fb9150614c3d565b634e487b7160e01b5f52601260045260245ffd5b9190614a92828285614c57565b928215614aaf5709614aa15790565b600181018091116118b75790565b614a71565b80549081614ac357505f919050565b815f198101116118b7575f525f199060205f2001015460201c611861565b916001600160401b03809116911603906001600160401b0382116118b7576001600160401b03610766921690614bc3565b8115614aaf570490565b1561034357565b5f19670de0b6b3a7640000820991670de0b6b3a7640000820291828085109403938085039414614bb757670de0b6b3a76400008291614b63868411614b1c565b09600182190182168092046002816003021880820260020302808202600203028082026002030280820260020302808202600203028091026002030293600183805f03040190848311900302920304170290565b50906107669250614b12565b5f1982820982820291828083109203918083039214614c2c5781670de0b6b3a76400001115610343577faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066993670de0b6b3a7640000910990828211900360ee1b910360121c170290565b50670de0b6b3a76400009250500490565b8015614aaf576ec097ce7bc90715b34b9f10000000000490565b90915f198383099280830292838086109503948086039514614c8157908291614b63868411614b1c565b5050906107669250614b12565b924211614cb457614c9e92614ebf565b15614ca557565b638baa579f60e01b5f5260045ffd5b630819bdcd60e01b5f5260045ffd5b8054821015611c12575f5260205f2001905f90565b6001810190825f528160205260405f2054155f14614d3b578054600160401b81101561067f57614d28614d12826001879401855584614cc3565b819391549060031b91821b915f19901b19161790565b905554915f5260205260405f2055600190565b5050505f90565b80548015614d69575f190190614d588282614cc3565b8154905f199060031b1b1916905555565b634e487b7160e01b5f52603160045260245ffd5b6001810191805f528260205260405f2054928315155f14614e18575f1984018481116118b75783545f198101949085116118b7575f95858361494294614dcb9803614dd1575b505050614d42565b55600190565b614e01614dfb91614df2614de8614e0f9588614cc3565b90549060031b1c90565b92839187614cc3565b906134e2565b85905f5260205260405f2090565b555f8080614dc3565b505050505f90565b60051115614e2a57565b634e487b7160e01b5f52602160045260245ffd5b9060609260209183526040828401528051918291826040860152018484015e5f828201840152601f01601f1916010190565b3d15614e9a573d90614e8182611c5c565b91614e8f604051938461069f565b82523d5f602084013e565b606090565b9081602091031261034357516001600160e01b0319811681036103435790565b919091614ecc82846150c1565b614ed581614e20565b159081614f66575b50614f5e575f9261277e614f0a85946040519283916020830195630b135d3f60e11b875260248401614e3e565b51915afa614f16614e70565b81614f52575b81614f25575090565b8051630b135d3f60e11b92506001600160e01b031991614f4d91810160209081019101614e9f565b161490565b80516020149150614f1c565b505050600190565b6001600160a01b0383811691161490505f614edd565b15614f8357565b63151b8e3f60e11b5f5260045ffd5b8054600160401b81101561067f57614faf91600182018155614cc3565b614fd557815160209283015190921b63ffffffff191663ffffffff909216919091179055565b634e487b7160e01b5f525f60045260245ffd5b805480615024575b5061501f61039d9361500f6150036106cf565b63ffffffff9095168552565b6001600160e01b03166020840152565b614f92565b805f198101116118b757815f5263ffffffff6150926150895f198460205f20010161507f6150716040519261505884610684565b548681169081855260201c602085015263ffffffff1690565b858916958691161115614f7c565b5163ffffffff1690565b63ffffffff1690565b03614ff05761039d9392509061184f6150aa92613f20565b9063ffffffff82549181199060201b169116179055565b8151604181036150ed5750906150e991602082015190606060408401519301515f1a9061512f565b9091565b6040036151265760406020830151920151918260ff1c91601b83018093116118b7576150e9936001600160ff1b03169260ff169061512f565b50505f90600290565b9291907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083116151cd5760ff16601b811415806151c2575b6151b7576020935f93604051938493608085019385528785015260408401526060830152838052039060015afa1561064b575f516001600160a01b038116156151af57905f90565b505f90600190565b505050505f90600490565b50601c811415615167565b505050505f9060039056fea26469706673582212203b359e406fa02b81faabca67e601f91055d39ddef3162db32899cc128ee4b6d064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xA4\xF9y\x14a\x034W\x80c\x0B\x9FHz\x14a\x03/W\x80c\r\xD8\xDD\x02\x14a\x03*W\x80c\x13d9\xDD\x14a\x03%W\x80c%\xDF\x92.\x14a\x03 W\x80c*\xA6\xD8\x88\x14a\x03\x1BW\x80c9\xB7\x0E8\x14a\x03\x16W\x80c<e\x1C\xF2\x14a\x03\x11W\x80c<\xDE\xB5\xE0\x14a\x03\x0CW\x80c>(9\x1D\x14a\x03\x07W\x80cFW\xE2j\x14a\x03\x02W\x80cFe\xBC\xDA\x14a\x02\xFDW\x80cT\xB7\xC9l\x14a\x02\xF8W\x80cY\\jg\x14a\x02\xF3W\x80cY{6\xDA\x14a\x02\xEEW\x80cZ\xC8j\xB7\x14a\x02\xE9W\x80c\\\x97Z\xBB\x14a\x02\xE4W\x80c]\x97^\x88\x14a\x02\xDFW\x80c]\xD6\x85y\x14a\x02\xDAW\x80c`\x1B\xB3o\x14a\x02\xD5W\x80c`\xA0\xD1\xCE\x14a\x02\xD0W\x80ce\xDA\x12d\x14a\x02\xCBW\x80cf\xD5\xBA\x93\x14a\x02\xC6W\x80cmp\xF7\xAE\x14a\x02\xC1W\x80cn\x17DH\x14a\x02\xBCW\x80cqP\x18\xA6\x14a\x02\xB7W\x80cw\x8EU\xF3\x14a\x02\xB2W\x80cx)n\xC5\x14a\x02\xADW\x80c\x88o\x11\x95\x14a\x02\xA8W\x80c\x8D\xA5\xCB[\x14a\x02\xA3W\x80c\x90\x04\x13G\x14a\x02\x9EW\x80c\x91\x04\xC3\x19\x14a\x02\x99W\x80c\x945\xBBC\x14a\x02\x94W\x80c\xA1x\x84\x84\x14a\x02\x8FW\x80c\xA3:43\x14a\x02\x8AW\x80c\xB7\xF0n\xBE\x14a\x02\x85W\x80c\xBBE\xFE\xF2\x14a\x02\x80W\x80c\xBF\xAE?\xD2\x14a\x02{W\x80c\xC4H\xFE\xB8\x14a\x02vW\x80c\xC9x\xF7\xAC\x14a\x02qW\x80c\xCA\x8A\xA7\xC7\x14a\x02lW\x80c\xCDm\xC6\x87\x14a\x02gW\x80c\xDA\x8B\xE8d\x14a\x02bW\x80c\xE4\xCC?\x90\x14a\x02]W\x80c\xEE\xA9\x06K\x14a\x02XW\x80c\xF0\xE0\xE6v\x14a\x02SW\x80c\xF2\xFD\xE3\x8B\x14a\x02NW\x80c\xF6\x98\xDA%\x14a\x02IW\x80c\xFA\xBC\x1C\xBC\x14a\x02DWc\xFD\x8A\xA8\x8D\x14a\x02?W_\x80\xFD[a$IV[a#\x85V[a#kV[a\"\xDAV[a\"\x19V[a!\xF1V[a!WV[a!%V[a ;V[a\x1F\xF7V[a\x1E\x86V[a\x1E!V[a\x1D\xDBV[a\x1D\x8DV[a\x1D^V[a\x1D+V[a\x1C!V[a\x1B\x0FV[a\x1A\xE1V[a\x1A\xB3V[a\x1ASV[a\x1A\x0FV[a\x19\x80V[a\x19<V[a\x18\xE1V[a\x16\xE5V[a\x16\x99V[a\x16IV[a\x16\x06V[a\x15\x92V[a\x14-V[a\x12\tV[a\x10\x81V[a\x0F\xA5V[a\x0FrV[a\x0F8V[a\x0E\x1FV[a\r\xCBV[a\r\x87V[a\rCV[a\x0C\xF8V[a\x0C\xA4V[a\x0B>V[a\n\xFAV[a\t\x85V[a\x08\x14V[a\x05\x90V[a\x04WV[a\x03\x9FV[a\x03GV[_\x91\x03\x12a\x03CWV[_\x80\xFD[4a\x03CW_6`\x03\x19\x01\x12a\x03CW` `@Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03CWV[5\x90a\x03\x9D\x82a\x03\x81V[V[4a\x03CW`\xA06`\x03\x19\x01\x12a\x03CW` a\x03\xE6`\x045a\x03\xC1\x81a\x03\x81V[`$5a\x03\xCD\x81a\x03\x81V[`D5a\x03\xD9\x81a\x03\x81V[`d5\x91`\x845\x93a$\xBEV[`@Q\x90\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x03CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03CW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x03CWV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x04AWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x044V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x04\x87\x906\x90`\x04\x01a\x03\xEEV[\x90a\x04\x9Fa\x04\x99`\x02\x80`fT\x16\x14\x90V[\x15a%hV[a\x04\xA8\x82a%~V[3_\x90\x81R`\x9A` R`@\x90 T\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x16_[\x82\x81\x10a\x04\xDEW`@Q\x80a\x04\xDA\x87\x82a\x04\x1EV[\x03\x90\xF3[\x80a\x05\x1Ba\x04\xF8a\x04\xF2`\x01\x94\x87\x89a%\xC4V[\x80a%\xE6V[\x90Pa\x05\x12a\x05\x08\x84\x88\x8Aa%\xC4V[` \x81\x01\x90a%\xE6V[\x91\x90P\x14a&\x1BV[a\x05\x7Fa\x05@a\x059a\x052a\x04\xF2\x85\x89\x8Ba%\xC4V[6\x91a\x06\xF5V[\x853a3\x9FV[\x86a\x05w\x87a\x05oa\x05ea\x05\x08\x88a\x05]a\x04\xF2\x82\x87\x8Aa%\xC4V[\x95\x90\x97a%\xC4V[\x94\x90\x926\x91a\x06\xF5V[\x926\x91a\x07iV[\x90\x863a6\xB0V[a\x05\x89\x82\x88a&>V[R\x01a\x04\xC5V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06KWa\x06\x1A\x92a\x06\x06\x91_\x91a\x06\x1CW[Pa&rV[a\x06\x15`fT\x82\x81\x16\x14a&\x88V[a9\xC5V[\0[a\x06>\x91P` =` \x11a\x06DW[a\x066\x81\x83a\x06\x9FV[\x81\x01\x90a&RV[_a\x06\0V[P=a\x06,V[a&gV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\xE0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\x7FW`@RV[a\x06PV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\x7FW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\x7FW`@RV[`@Q\x90a\x03\x9D`\xE0\x83a\x06\x9FV[`@Q\x90a\x03\x9D`@\x83a\x06\x9FV[`\x01`\x01`@\x1B\x03\x81\x11a\x06\x7FW`\x05\x1B` \x01\x90V[\x92\x91\x90a\x07\x01\x81a\x06\xDEV[\x93a\x07\x0F`@Q\x95\x86a\x06\x9FV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x03CW\x90[\x82\x82\x10a\x071WPPPV[` \x80\x91\x835a\x07@\x81a\x03\x81V[\x81R\x01\x91\x01\x90a\x07%V[\x90\x80`\x1F\x83\x01\x12\x15a\x03CW\x81` a\x07f\x935\x91\x01a\x06\xF5V[\x90V[\x92\x91\x90a\x07u\x81a\x06\xDEV[\x93a\x07\x83`@Q\x95\x86a\x06\x9FV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x03CW\x90[\x82\x82\x10a\x07\xA5WPPPV[\x815\x81R` \x91\x82\x01\x91\x01a\x07\x99V[\x90\x80`\x1F\x83\x01\x12\x15a\x03CW\x81` a\x07f\x935\x91\x01a\x07iV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x07\xEDWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x07\xE0V[\x90` a\x07f\x92\x81\x81R\x01\x90a\x07\xD0V[4a\x03CW``6`\x03\x19\x01\x12a\x03CW`\x045a\x081\x81a\x03\x81V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x08P\x906\x90`\x04\x01a\x07KV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x08p\x906\x90`\x04\x01a\x07\xB5V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x9A` R`@\x90 T\x90\x93a\x08\x98\x92\x86\x92\x16\x90a3\x9FV[a\x08\xA2\x84Qa%~V[\x93_[\x81Q\x81\x10\x15a\t&W`\x01\x90\x85_R`\xA2` Ra\t\x15a\x08\xF9a\x08\xF4`@_ a\x08\xE0a\x08\xD3\x86\x89a&>V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a&\xC0V[a\t\x03\x83\x88a&>V[Qa\t\x0E\x84\x88a&>V[Q\x91a9\xF7V[a\t\x1F\x82\x89a&>V[R\x01a\x08\xA5V[`@Q\x80a\x04\xDA\x88\x82a\x08\x03V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03CWV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03CWV[\x91\x81`\x1F\x84\x01\x12\x15a\x03CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03CW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03CWV[4a\x03CW``6`\x03\x19\x01\x12a\x03CW`\x045a\t\xA2\x81a\x03\x81V[a\t\xAAa\t4V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\t\xC9\x906\x90`\x04\x01a\tXV[3_\x90\x81R`\x9A` R`@\x90 T\x91\x93\x91a\t\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a&\xE3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03CW`@Qc+bA\xF3`\xE1\x1B\x81R3`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16`$\x85\x01R_\x90\x84\x90`D\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x06KW\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x93a\n\xDB\x93a\n\xE0W[Pa\n\x8A\x813a:\x13V[a\n\x9433a:sV[`@Q`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R3\x90\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90` \x90\xA2`@Q\x91\x82\x913\x95\x83a&\xF9V[\x03\x90\xA2\0[\x80a\n\xEE_a\n\xF4\x93a\x06\x9FV[\x80a\x039V[_a\n\x7FV[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03CW`\x806`\x03\x19\x01\x12a\x03CW`\x045a\x0B[\x81a\x03\x81V[`$5a\x0Bg\x81a\x03\x81V[`d5`D5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x14\x80\x15a\x0CrW[\x15a\x0CcW`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x90\x81R`\x9A` \x90\x81R`@\x91\x82\x90 T\x91Qc\x15&g\xD9`\xE3\x1B\x81R\x91\x83\x16`\x04\x83\x01\x81\x90R\x86\x84\x16`$\x84\x01R\x91\x96\x91\x95\x92\x87\x90`D\x90\x82\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x95\x86\x15a\x06KWa\x06\x1A\x96a\x0C.\x91_\x91a\x0C4W[P\x83\x83a;@V[\x94a=4V[a\x0CV\x91P` =` \x11a\x0C\\W[a\x0CN\x81\x83a\x06\x9FV[\x81\x01\x90a' V[_a\x0C&V[P=a\x0CDV[c\x04R\x06\xA5`\xE2\x1B_R`\x04_\xFD[P3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xA0V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW` a\x0C\xE6`\x045a\x0C\xC6\x81a\x03\x81V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW` a\r9`\x045a\r\x1A\x81a\x03\x81V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03CW`@6`\x03\x19\x01\x12a\x03CWa\x06\x1A`\x045a\r\xEB\x81a\x03\x81V[`$5\x90a\r\xF8\x82a\x03\x81V[a\x0E\ta\x0E\x04\x82a=\xA2V[a'5V[a\x0E\x1Aa\x0E\x15\x82a.\x1DV[a'KV[a:\x13V[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x06KWa\x0E\x8A\x91_\x91a\x06\x1CWPa&rV[a\x06\x1Aa9\x91V[\x91\x90\x91`\xE0\x81\x84\x03\x12a\x03CWa\x0E\xA7a\x06\xC0V[\x92a\x0E\xB1\x82a\x03\x92V[\x84Ra\x0E\xBF` \x83\x01a\x03\x92V[` \x85\x01Ra\x0E\xD0`@\x83\x01a\x03\x92V[`@\x85\x01R``\x82\x015``\x85\x01Ra\x0E\xEB`\x80\x83\x01a\tGV[`\x80\x85\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03CW\x81a\x0F\x0F\x91\x84\x01a\x07KV[`\xA0\x85\x01R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x0F1\x92\x01a\x07\xB5V[`\xC0\x83\x01RV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x03\xE6a\x0Fm` \x926\x90`\x04\x01a\x0E\x92V[a'aV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045`\xFF\x81\x16\x80\x91\x03a\x03CW`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x03CW_6`\x03\x19\x01\x12a\x03CW` `fT`@Q\x90\x81R\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0F\xDFWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0F\xD2V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01Ra\x07f\x91`\xC0a\x10_`\xA0\x84\x01Q`\xE0`\xA0\x85\x01R`\xE0\x84\x01\x90a\x0F\xC2V[\x92\x01Q\x90`\xC0\x81\x84\x03\x91\x01Ra\x07\xD0V[\x90` a\x07f\x92\x81\x81R\x01\x90a\x0F\xFEV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a\x10\x9Da'\x8CV[P_R`\xA4` Ra\x04\xDA`@_ a\x113`\x06`@Q\x92a\x10\xBE\x84a\x06dV[\x80T`\x01`\x01`\xA0\x1B\x03\x16\x84R`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16` \x85\x01R`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16`@\x85\x01R`\x03\x81\x01T``\x85\x01Ra\x11\x1Ca\x11\x0F`\x04\x83\x01Tc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16`\x80\x86\x01RV[a\x11(`\x05\x82\x01a'\xC2V[`\xA0\x85\x01R\x01a(\x13V[`\xC0\x82\x01R`@Q\x91\x82\x91\x82a\x10pV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x11oWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x11\x8D`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x07\xD0V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x11`V[\x92\x91`@\x84\x01\x93`@\x81R\x82Q\x80\x95R``\x81\x01\x94` ``\x82`\x05\x1B\x84\x01\x01\x94\x01\x90_[\x81\x81\x10a\x11\xDEWPPPa\x07f\x93\x94P` \x81\x84\x03\x91\x01Ra\x11DV[\x90\x91\x94` \x80a\x11\xFA`\x01\x93`_\x19\x88\x82\x03\x01\x8CR\x89Qa\x0F\xFEV[\x97\x01\x98\x01\x91\x01\x96\x91\x90\x96a\x11\xC1V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a\x12&\x81a\x03\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xA3` R`@\x90 a\x12F\x90a(\x13V[\x80Q\x90a\x12R\x82a(\xE9V[\x91a\x12\\\x81a)8V[\x93a\x12\x87a\x12z\x82`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91_\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92c\xFF\xFF\xFF\xFFC\x16\x90[\x83\x86\x10a\x12\xCBW`@Q\x80a\x04\xDA\x8B\x8B\x83a\x11\x9CV[a\x12\xF1a\x12\xECa\x12\xDD\x88\x8A\x9C\x9Aa&>V[Q_R`\xA4` R`@_ \x90V[a([V[a\x12\xFB\x87\x8Aa&>V[Ra\x13\x06\x86\x89a&>V[Pa\x13\x1F`\xA0a\x13\x16\x88\x8Ba&>V[Q\x01QQa%~V[a\x13)\x87\x89a&>V[Ra\x134\x86\x88a&>V[Pa\x13Y\x85a\x13T`\x80a\x13H\x8A\x8Da&>V[Q\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[a)\x95V[\x82c\xFF\xFF\xFF\xFF\x82\x16\x10_\x14a\x13\xFCWa\x13\x82\x90`\xA0a\x13x\x89\x8Ca&>V[Q\x01Q\x85\x84a>iV[\x94[_[`\xA0a\x13\x92\x89\x8Ca&>V[Q\x01QQ\x81\x10\x15a\x13\xEBW\x80\x89a\x13\xE4\x82a\x13\xDE\x8C\x8F\x8Da\x13\xD1\x85a\x13\xCA`\x01\x9B`\xC0a\x13\xC2\x88a\x13\xD8\x98a&>V[Q\x01Qa&>V[Q\x92a&>V[Q\x90aK\xC3V[\x94a&>V[Qa&>V[R\x01a\x13\x86V[P\x96\x98\x96`\x01\x90\x96\x01\x95\x94Pa\x12\xB5V[Pa\x14\x16`\xA0a\x14\x0C\x88\x8Ba&>V[Q\x01Q\x84\x83a3\x9FV[\x94a\x13\x84V[`\x01`\x01`@\x1B\x03\x81\x16\x03a\x03CWV[4a\x03CW`\x806`\x03\x19\x01\x12a\x03CW`\x045a\x14J\x81a\x03\x81V[`$5a\x14V\x81a\x03\x81V[`D5a\x14b\x81a\x14\x1CV[`d5\x90a\x14o\x82a\x14\x1CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x15\x83W\x82a\x15\x12a\x15\x0Ca\x15\x18\x94a\x15\x04a\x14\xE0\x85a\x14\xCB\x8B`\x01\x80`\xA0\x1B\x03\x16_R`\x98` R`@_ \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta\x14\xFE`\x01`\x01`@\x1B\x03\x88\x16`\x01`\x01`@\x1B\x03\x85\x16\x83aJ\x85V[\x90a?.V[\x94\x84\x89a?mV[\x83a)\xBDV[\x94a@tV[a\x150a\x15$\x82aA\xDEV[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x82;\x15a\x03CW`@Qc\xDE\xBE\x1E\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90_\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x06KWa\x15uW\0[\x80a\n\xEE_a\x06\x1A\x93a\x06\x9FV[c#\xD8q\xA5`\xE0\x1B_R`\x04_\xFD[4a\x03CW``6`\x03\x19\x01\x12a\x03CW`\x045a\x15\xAF\x81a\x03\x81V[`D5`$5a\x15\xBE\x82a\x14\x1CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x15\xF7Wa\x06\x1A\x92a)\xCAV[c2\x13\xA6a`\xE2\x1B_R`\x04_\xFD[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a\x16#\x81a\x03\x81V[`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x03CW` 6`\x03\x19\x01\x12a\x03CWa\x16\x8Ba\x04\xDAa\x16t`\x045a\x16o\x81a\x03\x81V[a,4V[`@\x92\x91\x92Q\x93\x84\x93`@\x85R`@\x85\x01\x90a\x0F\xC2V[\x90\x83\x82\x03` \x85\x01Ra\x07\xD0V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW` a\r9`\x045a\x16\xBB\x81a\x03\x81V[a.\x1DV[`@\x90`\x03\x19\x01\x12a\x03CW`\x045a\x16\xD8\x81a\x03\x81V[\x90`$5a\x07f\x81a\x03\x81V[4a\x03CWa\x16\xF36a\x16\xC0V[`@Qc\x15&g\xD9`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R\x91` \x83\x80`D\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\x06KW_\x93a\x18\xBCW[Pa\x17\xB8\x90`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\xA5` Ra\x17\x97a\x17\x92\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[aJ\xB4V[\x92_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a\x17\xF0a\x17\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFFC\x16a?SV[a?;V[\x81T\x91\x90c\xFF\xFF\xFF\xFF\x16_[\x83\x81\x10a\x18mWa\x04\xDAa\x18,_\x88a\x18'\x89\x89\x89\x81a\x18<WPP`\x01`\x01`\xE0\x1B\x03\x84\x16a\x14\xFEV[aJ\xE1V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x18Za\x18a\x91a\x18Oa\x14\xFE\x94a? V[\x90_R` _ \x01\x90V[T` \x1C\x90V[`\x01`\x01`\xE0\x1B\x03\x16\x90V[\x90\x92\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a\x18\xB7W\x83_R\x84c\xFF\xFF\xFF\xFF\x83` _ \x01T\x16\x11_\x14a\x18\xA3WP\x92[\x90a\x17\xFCV[\x93\x91P`\x01\x81\x01\x80\x91\x11a\x18\xB7W\x90a\x18\x9DV[a)\x81V[a\x17\xB8\x91\x93Pa\x18\xDA\x90` =` \x11a\x0C\\Wa\x0CN\x81\x83a\x06\x9FV[\x92\x90a\x17\\V[4a\x03CW_6`\x03\x19\x01\x12a\x03CWa\x18\xF9aBwV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03CW` a\x19wa\x19O6a\x16\xC0V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x98\x85R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T`@Q\x90\x81R\xF3[4a\x03CW`@6`\x03\x19\x01\x12a\x03CW`\x045a\x19\x9D\x81a\x03\x81V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x19\xDD\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x916\x90`\x04\x01a\tXV[\x90\x92a\x19\xEBa\x0E\x04\x82a=\xA2V[a\x19\xF7a\x0E\x15\x82a.\x1DV[a\n\xDB`@Q\x92\x83\x92`\x01\x80`\xA0\x1B\x03\x16\x95\x83a&\xF9V[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90`@`\x03\x19\x83\x01\x12a\x03CW`\x045a\x1A\x94\x81a\x03\x81V[\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03CWa\x07f\x91`\x04\x01a\x07KV[4a\x03CWa\x04\xDAa\x1A\xCDa\x1A\xC76a\x1A{V[\x90a.PV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xD0V[4a\x03CW_6`\x03\x19\x01\x12a\x03CW` `@Qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81R\xF3[4a\x03CW``6`\x03\x19\x01\x12a\x03CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x1B?\x906\x90`\x04\x01a\x03\xEEV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x1B^\x906\x90`\x04\x01a\x03\xEEV[\x91`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x1B\x81\x90\x93\x91\x936\x90`\x04\x01a\x03\xEEV[\x90a\x1B\x93a\x04\x99`\x04\x80`fT\x16\x14\x90V[a\x1B\xA2`\x02`\xC9T\x14\x15a.\xC3V[`\x02`\xC9U6\x86\x90\x03`\xDE\x19\x01\x92_[\x86\x81\x10\x15a\x1C\x17W\x80`\x05\x1B\x90\x81\x89\x015\x91\x86\x83\x12\x15a\x03CW\x83\x82\x10\x15a\x1C\x12W`\x01\x92a\x1B\xE6a\x1C\x0C\x92\x8A\x01\x8Aa%\xE6V[\x90a\x1C\x07\x8Da\x1B\xFEa\x1B\xF9\x88\x8D\x8Da/\x0FV[a/\x1FV[\x946\x91\x01a\x0E\x92V[aC\xBAV[\x01a\x1B\xB2V[a%\xB0V[a\x06\x1A`\x01`\xC9UV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a\x1C>\x81a\x03\x81V[`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x06\x7FW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x90```\x03\x19\x83\x01\x12a\x03CW`\x045a\x1C\x90\x81a\x03\x81V[\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03CW`@\x82\x82\x03`\x03\x19\x01\x12a\x03CW`@Q\x91a\x1C\xBE\x83a\x06\x84V[\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03CW\x81\x01\x91\x80`#\x84\x01\x12\x15a\x03CW`\x04\x83\x015a\x1C\xED\x81a\x1C\\V[\x91a\x1C\xFB`@Q\x93\x84a\x06\x9FV[\x81\x83R`$\x85\x83\x01\x01\x11a\x03CW` \x81_\x92`$\x80\x97\x01\x83\x86\x017\x83\x01\x01R\x83R\x015` \x82\x01R\x90`D5\x90V[4a\x03CWa\x04\xDAa\x1DRa\x1D?6a\x1CwV[\x90a\x1DL\x93\x92\x933a/\x9DV[\x93a0\x9CV[`@Q\x91\x82\x91\x82a\x04\x1EV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045_R`\x9E` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03CW`@6`\x03\x19\x01\x12a\x03CW`\x045a\x1D\xAA\x81a\x03\x81V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03CW` a\x03\xE6a\x1E\x1Ca\x08\xF4a\x1D\xF46a\x16\xC0V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\xA2\x87R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[aG?V[4a\x03CW_6`\x03\x19\x01\x12a\x03CW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[\x90\x91a\x1Exa\x07f\x93`@\x84R`@\x84\x01\x90a\x07\xD0V[\x91` \x81\x84\x03\x91\x01Ra\x07\xD0V[4a\x03CWa\x1E\x946a\x1A{V[a\x1E\x9E\x81Qa%~V[a\x1E\xA8\x82Qa%~V[\x91a\x1E\xD0\x81a\x1E\xCAa\x12z\x87`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x86a3\x9FV[_[\x82Q\x81\x10\x15a\x1F\xE5W\x80` a\x1E\xF9a\x15$a\x1E\xF4a\x08\xD3a\x1F:\x96\x89a&>V[aA\xDEV[a\x1F\x06a\x08\xD3\x84\x88a&>V[`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x8C\x16`\x04\x83\x01R\x90\x91\x16`$\x82\x01R\x93\x84\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x80\x15a\x06KW`\x01\x92_\x91a\x1F\xB7W[Pa\x1FY\x82\x88a&>V[Ra\x1F\xA6a\x1F\x8Aa\x08\xF4a\x1F}\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[a\x08\xE0a\x08\xD3\x86\x8Aa&>V[a\x1F\x94\x83\x89a&>V[Qa\x1F\x9F\x84\x87a&>V[Q\x91aB[V[a\x1F\xB0\x82\x87a&>V[R\x01a\x1E\xD2V[a\x1F\xD8\x91P` =\x81\x11a\x1F\xDEW[a\x1F\xD0\x81\x83a\x06\x9FV[\x81\x01\x90a,%V[_a\x1FNV[P=a\x1F\xC6V[PPPa\x04\xDA`@Q\x92\x83\x92\x83a\x1EaV[4a\x03CW_6`\x03\x19\x01\x12a\x03CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03CW`@6`\x03\x19\x01\x12a\x03CW`\x045a X\x81a\x03\x81V[a \x9D`$5_T\x92a \x83`\xFF`\x08\x86\x90\x1C\x16\x15\x80\x95\x81\x96a!\x17W[\x81\x15a \xF7W[Pa/)V[\x83a \x94`\x01`\xFF\x19_T\x16\x17_UV[a \xE0Wa/\x8CV[a \xA3W\0[a \xB1a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a \xF2a\x01\0a\xFF\0\x19_T\x16\x17_UV[a/\x8CV[0;\x15\x91P\x81a!\tW[P_a }V[`\xFF\x16`\x01\x14\x90P_a!\x02V[`\x01`\xFF\x82\x16\x10\x91Pa vV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CWa\x04\xDAa\x1DR`\x045a!H\x81a\x03\x81V[a/\x9DV[\x80\x15\x15\x03a\x03CWV[4a\x03CW``6`\x03\x19\x01\x12a\x03CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03CW`\xE0`\x03\x19\x826\x03\x01\x12a\x03CW`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03CWa!\xAAa!\xEA\x926\x90`\x04\x01a\x03\xEEV[\x90a\x1C\x07`D5\x93a!\xBB\x85a!MV[a!\xCCa\x04\x99`\x04\x80`fT\x16\x14\x90V[a!\xDB`\x02`\xC9T\x14\x15a.\xC3V[`\x02`\xC9U6\x90`\x04\x01a\x0E\x92V[`\x01`\xC9U\0[4a\x03CWa\x06\x1Aa\"\x026a\x1CwV[\x91a0\x9CV[\x90` a\x07f\x92\x81\x81R\x01\x90a\x11DV[4a\x03CW`@6`\x03\x19\x01\x12a\x03CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03CW6`#\x82\x01\x12\x15a\x03CW\x80`\x04\x015\x90a\"U\x82a\x06\xDEV[\x91a\"c`@Q\x93\x84a\x06\x9FV[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03CW`$\x01\x90[\x82\x82\x10a\"\xC0W\x83`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x04\xDA\x91a\"\xAEa\"\xB4\x926\x90`\x04\x01a\x07KV[\x90a1\xA7V[`@Q\x91\x82\x91\x82a\"\x08V[` \x80\x91\x835a\"\xCF\x81a\x03\x81V[\x81R\x01\x91\x01\x90a\"\x80V[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a\"\xF7\x81a\x03\x81V[a\"\xFFaBwV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a#\x17Wa\x06\x1A\x90aB\xCFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03CW_6`\x03\x19\x01\x12a\x03CW` a\x03\xE6a1\xFAV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06KW_\x91a$\x0EW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a#\xFFWa\x06\x1A\x90a2\xB7V[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a$AW[\x81a$)` \x93\x83a\x06\x9FV[\x81\x01\x03\x12a\x03CWQa$;\x81a\x03\x81V[_a#\xE6V[=\x91Pa$\x1CV[4a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\x045a$f\x81a\x03\x81V[`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a$\xA8Wa\x04\xDA\x85a\x1DR\x81\x87\x03\x82a\x06\x9FV[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a$\x91V[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x92\x82\x01\x92\x90\x92R\x91\x84\x16``\x83\x01R\x92\x90\x91\x16`\x80\x82\x01R`\xA0\x81\x01\x92\x90\x92R`\xC0\x80\x83\x01\x93\x90\x93R\x91\x81Ra%,`\xE0\x82a\x06\x9FV[Q\x90 a%7a1\xFAV[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra%b`b\x82a\x06\x9FV[Q\x90 \x90V[\x15a%oWV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x90a%\x88\x82a\x06\xDEV[a%\x95`@Q\x91\x82a\x06\x9FV[\x82\x81R\x80\x92a%\xA6`\x1F\x19\x91a\x06\xDEV[\x01\x90` 6\x91\x017V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x81\x10\x15a\x1C\x12W`\x05\x1B\x81\x015\x90`^\x19\x816\x03\x01\x82\x12\x15a\x03CW\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03CW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03CW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x03CWV[\x15a&\"WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x80Q\x15a\x1C\x12W` \x01\x90V[\x80Q\x82\x10\x15a\x1C\x12W` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03CWQa\x07f\x81a!MV[`@Q=_\x82>=\x90\xFD[\x15a&yWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a&\x8FWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[`@\x80Q\x90\x91\x90a&\xAF\x83\x82a\x06\x9FV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\x7FW`@R\x91T\x82RV[\x15a&\xEAWV[c;\xF2\xB5\x03`\xE1\x1B_R`\x04_\xFD[\x90\x91\x80`@\x93` \x84R\x81` \x85\x01R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03CWQa\x07f\x81a\x14\x1CV[\x15a'<WV[c\x93-\x94\xF7`\xE0\x1B_R`\x04_\xFD[\x15a'RWV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[`@Qa%b\x81a'~` \x82\x01\x94` \x86R` \x86\x01\x90a\x0F\xFEV[\x03`\x1F\x19\x81\x01\x83R\x82a\x06\x9FV[`@Q\x90a'\x99\x82a\x06dV[```\xC0\x83_\x81R_` \x82\x01R_`@\x82\x01R_\x83\x82\x01R_`\x80\x82\x01R\x82`\xA0\x82\x01R\x01RV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_[\x81\x81\x10a'\xF1WPPa\x03\x9D\x92P\x03\x83a\x06\x9FV[\x84T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01a'\xDCV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_[\x81\x81\x10a(BWPPa\x03\x9D\x92P\x03\x83a\x06\x9FV[\x84T\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01a(-V[\x90`@Qa(h\x81a\x06dV[\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x84\x01T\x16` \x82\x01R\x91\x82\x90`\xC0\x90a(\xE4\x90`\x06\x90`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16`@\x86\x01R`\x03\x81\x01T``\x86\x01Ra(\xCDa(\xC0`\x04\x83\x01Tc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16`\x80\x87\x01RV[a(\xD9`\x05\x82\x01a'\xC2V[`\xA0\x86\x01R\x01a(\x13V[\x91\x01RV[\x90a(\xF3\x82a\x06\xDEV[a)\0`@Q\x91\x82a\x06\x9FV[\x82\x81R\x80\x92a)\x11`\x1F\x19\x91a\x06\xDEV[\x01\x90_[\x82\x81\x10a)!WPPPV[` \x90a),a'\x8CV[\x82\x82\x85\x01\x01R\x01a)\x15V[\x90a)B\x82a\x06\xDEV[a)O`@Q\x91\x82a\x06\x9FV[\x82\x81R\x80\x92a)``\x1F\x19\x91a\x06\xDEV[\x01\x90_[\x82\x81\x10a)pWPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a)dV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x18\xB7WV[\x90`\x01\x82\x01\x80\x92\x11a\x18\xB7WV[\x91\x90\x82\x01\x80\x92\x11a\x18\xB7WV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15a+!W`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9A` R`@\x90 a*\t\x90a\x12zV[`@Qc\x15&g\xD9`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x82\x01R\x90\x92` \x82\x80`D\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06KWa\x03\x9D\x95a*\xF6\x93_\x93a*\xFCW[Pa*\xF0\x90a*\xD3a\x08\xF4a*\xB2\x88`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0_R` R`@_ \x90V[\x93`\x01`\x01`@\x1B\x03\x80g\r\xE0\xB6\xB3\xA7d\0\0[\x93\x16\x91\x16aLWV[\x91aB[V[\x91a@\xE7V[a*\xF0\x91\x93Pa+\x1A\x90` =` \x11a\x0C\\Wa\x0CN\x81\x83a\x06\x9FV[\x92\x90a*\x8DV[PPPV[\x90\x80`\x1F\x83\x01\x12\x15a\x03CW\x81Qa+=\x81a\x06\xDEV[\x92a+K`@Q\x94\x85a\x06\x9FV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03CW` \x01\x90[\x82\x82\x10a+sWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a+fV[\x91\x90\x91`@\x81\x84\x03\x12a\x03CW\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x03CW\x81\x01\x83`\x1F\x82\x01\x12\x15a\x03CW\x80Q\x90a+\xB9\x82a\x06\xDEV[\x91a+\xC7`@Q\x93\x84a\x06\x9FV[\x80\x83R` \x80\x84\x01\x91`\x05\x1B\x83\x01\x01\x91\x86\x83\x11a\x03CW` \x01\x90[\x82\x82\x10a,\x0BWPPP\x92` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x03CWa\x07f\x92\x01a+&V[` \x80\x91\x83Qa,\x1A\x81a\x03\x81V[\x81R\x01\x91\x01\x90a+\xE3V[\x90\x81` \x91\x03\x12a\x03CWQ\x90V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R\x91\x90_\x83\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06KW_\x93_\x92a-\xEFW[P`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`$\x82\x01R\x90` \x82\x80`D\x81\x01[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06KW_\x92a-\xCEW[P\x81\x15a-\xC9Wa-.a-)\x85Qa)\xAFV[a%~V[\x93a-<a-)\x82Qa)\xAFV[\x92a-da-K\x83Q\x88a&>V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x90RV[a-o\x82Q\x85a&>V[R_[\x81Q\x81\x10\x15a-\xC3W\x80a-\xA7a-\x8Ea\x08\xD3`\x01\x94\x86a&>V[a-\x98\x83\x8Aa&>V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[a-\xB1\x81\x85a&>V[Qa-\xBC\x82\x87a&>V[R\x01a-rV[PPP\x90V[\x91\x90PV[a-\xE8\x91\x92P` =` \x11a\x1F\xDEWa\x1F\xD0\x81\x83a\x06\x9FV[\x90_a-\x15V[` \x94Pa,\xD9\x92Pa.\x13\x90=\x80_\x83>a.\x0B\x81\x83a\x06\x9FV[\x81\x01\x90a+\x83V[\x94\x90\x94\x92Pa,\x97V[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x15\x90\x81a.3WP\x90V[_\x81\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14\x91\x90PV[\x91\x90\x91a.]\x83Qa%~V[\x90_[\x84Q\x81\x10\x15a.\xBCW`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x98` R`@\x90 `\x01\x92\x91a.\xAA\x91\x90a.\x94\x84\x8Aa&>V[Q\x16`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta.\xB5\x82\x86a&>V[R\x01a.`V[P\x90\x92PPV[\x15a.\xCAWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x90\xFD[\x91\x90\x81\x10\x15a\x1C\x12W`\x05\x1B\x01\x90V[5a\x07f\x81a!MV[\x15a/0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a/\x98a\x03\x9D\x92a9\xC5V[aB\xCFV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15a0\x8DWa/\xC5\x81a.\x1DV[a0~W`\x01`\x01`\xA0\x1B\x03\x81\x16\x903\x82\x90\x03a/\xE7W[a\x07f\x91PaGRV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a0\x11\x81a=\xA2V[\x80\x15a0YW[\x15a0JWa\x07f\x92\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A_\x80\xA3a/\xDDV[c\x1EI\x9A#`\xE1\x1B_R`\x04_\xFD[P`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14a0\x18V[c\x11\xCA35`\xE3\x1B_R`\x04_\xFD[c\xA5\xC7\xC4E`\xE0\x1B_R`\x04_\xFD[3_\x90\x81R`\x9A` R`@\x90 T\x90\x93\x92a0\xDA\x92\x90\x91a0\xC7\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a&\xE3V[a0\xD3a\x0E\x15\x86a.\x1DV[\x843aH\xC6V[a0\xEBa\x04\x99`\x01\x80`fT\x16\x14\x90V[3_\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x163\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04_\x80\xA3a1J3a,4V[a1U\x82\x853a3\x9FV[\x91_[\x81Q\x81\x10\x15a1\x9FW`\x01\x90a1\x99`\x01`\x01`\xA0\x1B\x03a1y\x83\x86a&>V[Q\x16a1\x85\x83\x87a&>V[Qa1\x90\x84\x89a&>V[Q\x913\x8Ba<\x11V[\x01a1XV[PPPP\x90PV[\x90a1\xB2\x82Qa)8V[\x91_[\x81Q\x81\x10\x15a-\xC3W`\x01\x90a1\xDE\x84`\x01`\x01`\xA0\x1B\x03a1\xD7\x84\x87a&>V[Q\x16a.PV[a1\xE8\x82\x87a&>V[Ra1\xF3\x81\x86a&>V[P\x01a1\xB5V[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a2EW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`\n` `@Qa2W`@\x82a\x06\x9FV[\x82\x81R\x01i\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B\x81R `@Q` \x81\x01\x91\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x83R`@\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\x80\x81Ra%b`\xA0\x82a\x06\x9FV[a2\xC8`fT\x19\x82\x19\x81\x16\x14a&\x88V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2V[` \x81\x83\x03\x12a\x03CW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03CW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03CW\x81Qa3-\x81a\x06\xDEV[\x92a3;`@Q\x94\x85a\x06\x9FV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03CW` \x01\x90[\x82\x82\x10a3cWPPP\x90V[` \x80\x91\x83Qa3r\x81a\x14\x1CV[\x81R\x01\x91\x01\x90a3VV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra\x07f\x92\x91\x01\x90a\x0F\xC2V[\x92\x91a3\xCD\x90_\x81a3\xB1\x81Qa%~V[\x94`@Q\x94\x85\x92\x83\x92cTz\xFB\x87`\xE0\x1B\x84R`\x04\x84\x01a3}V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06KW_\x92a4bW[P_[\x81Q\x81\x10\x15a4ZW\x80a4Ia4(a\x08\xD3`\x01\x94\x86a&>V[a4Ba45\x84\x88a&>V[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x90\x89a;@V[a4S\x82\x87a&>V[R\x01a4\x0CV[P\x91\x93PPPV[a4\x7F\x91\x92P=\x80_\x83>a4w\x81\x83a\x06\x9FV[\x81\x01\x90a2\xFAV[\x90_a4\tV[\x15a4\x8DWV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[\x15a4\xA3WV[cyl\xC5%`\xE0\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x91\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_\x19\x81\x14a\x18\xB7W`\x01\x01\x90V[\x91a4\xFB\x91\x83T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90UV[\x91\x90\x91\x82\x82\x10a5\x0EWPPPV[_R` _ \x91\x82\x01\x91\x01[\x81\x81\x10a5%WPPV[_\x81U`\x01\x01a5\x1AV[\x90`\x01`@\x1B\x81\x11a\x06\x7FW\x81T\x81\x83Ua\x03\x9D\x92a4\xFFV[\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x06\x7FW` \x90a5h\x84\x84a50V[\x01\x90_R` _ _[\x83\x81\x10a5\x7FWPPPPV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x83\x01U` \x90\x92\x01\x91`\x01\x01a5rV[\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x06\x7FW` \x90a5\xBA\x84\x84a50V[\x01\x90_R` _ _[\x83\x81\x10a5\xD1WPPPPV[`\x01\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a5\xC4V[\x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U` \x84\x01Q`\x01\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\x02\x84\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U``\x82\x01Q`\x03\x82\x01U`\x80\x82\x01Qa\x03\x9D\x92`\x06\x91`\xC0\x91\x90a6k\x90c\xFF\xFF\xFF\xFF\x16`\x04\x86\x01\x90c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[a6|`\xA0\x82\x01Q`\x05\x86\x01a5JV[\x01Q\x91\x01a5\x9CV[\x91a6\xA2\x90a\x07f\x94\x92\x84R``` \x85\x01R``\x84\x01\x90a\x0F\xFEV[\x91`@\x81\x84\x03\x91\x01Ra\x07\xD0V[\x92\x94\x93\x90a6\xC8`\x01`\x01`\xA0\x1B\x03\x85\x16\x15\x15a4\x86V[a6\xD4\x83Q\x15\x15a4\x9CV[a6\xDE\x83Qa%~V[\x90a6\xE9\x84Qa%~V[_[\x85Q\x81\x10\x15a8QW\x86a7q\x8Aa7Ua7B\x85a7<a\x08\xF4\x8Da\x08\xE0a\x08\xD3\x85a7<a7!a\x1E\xF4a\x08\xD3\x84\x88a&>V[`\x01`\x01`\xA0\x1B\x03\x90\x9D\x16_\x90\x81R`\xA2` R`@\x90 \x90V[\x93a&>V[Qa7M\x86\x8Ba&>V[Q\x90\x83aB[V[a7_\x85\x87a&>V[Ra7j\x84\x8Da&>V[Q\x90aInV[a7{\x83\x87a&>V[R`\x01`\x01`\xA0\x1B\x03\x84\x16a8\x07W[`\x01`\x01`\xA0\x1B\x03\x16\x90a7\xA2a\x08\xD3\x82\x89a&>V[a7\xAC\x82\x8Ca&>V[Q\x83;\x15a\x03CWa7\xD9\x93_\x92\x83\x8C`@Q\x97\x88\x95\x86\x94\x85\x93crJ\xF4#`\xE0\x1B\x85R`\x04\x85\x01a4\xB2V[\x03\x92Z\xF1\x91\x82\x15a\x06KW`\x01\x92a7\xF3W[P\x01a6\xEBV[\x80a\n\xEE_a8\x01\x93a\x06\x9FV[_a7\xECV[a8)a8\x17a\x08\xD3\x84\x8Aa&>V[a8!\x84\x88a&>V[Q\x90\x86aI{V[a8La89a\x08\xD3\x84\x8Aa&>V[a8C\x84\x86a&>V[Q\x90\x8A\x87aAtV[a7\x8BV[P\x93a9u\x96\x97P\x7F&\xB2\xAA\xE2e\x16\xE8q\x9E\xF5\x0E\xA2\xF6\x83\x1A.\xFB\xD4\xE3}\xCC\xDF\x0Fi6\xB2{\xC0\x8Ey>0\x95\x91\x93P\x91a9z\x92a8\x9D\x83`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R`@_ \x90V[T`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\x9F` R`@\x90 a8\xBF\x81Ta4\xD4V[\x90Ua8\xE9a8\xCCa\x06\xC0V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R\x96`\x01`\x01`\xA0\x1B\x03\x16` \x88\x01RV[`\x01`\x01`\xA0\x1B\x03\x84\x16`@\x87\x01R``\x86\x01RCc\xFF\xFF\xFF\xFF\x16`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01Ra9\x1C\x83a'aV[\x95\x86\x91a9Aa94\x84_R`\x9E` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[a9\\\x85a9W\x85_R`\xA4` R`@_ \x90V[a5\xE5V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\xA3` R`@\x90 \x90V[aL\xD8V[Pa9\x8B`@Q\x92\x83\x92\x86\x84a6\x85V[\x03\x90\xA1\x90V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[a\x07f\x92\x91a:\x08a:\x0E\x92aG?V[\x90aK#V[aK#V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x99` \x90\x81R`@\x91\x82\x90 `\x01\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x95\x90\x94\x16\x94\x85\x17\x90\x93UQ\x92\x83R\x91\x7Fw;T\xC0Muo\xCC^g\x81\x11\xF7\xD70\xDE;\xE9\x81\x92\0\x07\x99\xEE\xE3\xD67\x16\x05Z\x87\xC6\x91\x90\xA2V[\x91\x90\x91a:\x87a\x04\x99`\x01\x80`fT\x16\x14\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x9A` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x87\x16\x93\x84\x17\x90U\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x90\x80\xA3a:\xE0\x81a,4V[\x90\x91a:\xED\x83\x86\x83a3\x9FV[\x92_[\x81Q\x81\x10\x15a;7W`\x01\x90a;1`\x01`\x01`\xA0\x1B\x03a;\x11\x83\x86a&>V[Q\x16a;\x1D\x83\x88a&>V[Qa;(\x84\x8Aa&>V[Q\x91\x87\x8Ca<\x11V[\x01a:\xF0V[PPPPP\x90PV[\x91\x90`\x01`\x01`\xA0\x1B\x03\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a;sW`\x01`\x01`@\x1B\x03\x91P\x16\x90V[`@Qc\xA3\xD7^\t`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R` \x82\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06KWa\x07f\x92_\x92a;\xF0W[P`\x01`\x01`@\x1B\x03\x80g\r\xE0\xB6\xB3\xA7d\0\0a*\xE7V[a<\n\x91\x92P` =` \x11a\x0C\\Wa\x0CN\x81\x83a\x06\x9FV[\x90_a;\xD8V[\x90\x93\x80\x15a=%W`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x91a<\x86\x91a<x\x91a\x1E\x1C\x91a\x08\xF4\x90\x91\x89_\x84aJ#V[`@Q\x91\x82\x91\x86\x89\x84a4\xB2V[\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x9A` R`@\x90 T\x16a<\xAEW[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x98` R`@\x90 a<\xD0\x90\x83\x90a\x14\xCBV[\x80T\x93\x80\x85\x01\x80\x95\x11a\x18\xB7W\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x94a=\x19\x92U`@Q\x93\x84\x93`\x01\x80`\xA0\x1B\x03\x16\x96\x84a4\xB2V[\x03\x90\xA2_\x80\x80\x80a<\xA8V[c\n3\xBCi`\xE2\x1B_R`\x04_\xFD[\x91\x92\x90\x94\x80\x15a=%Wa<xa\x1E\x1C\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x93a\x08\xF4a<\x86\x94`\x01\x80`\xA0\x1B\x03\x8B\x16_R`\xA2` R\x89a=\x9B\x8A`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x93\x84aJ#V[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R3`$\x82\x01R0`D\x82\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x90\x91` \x91\x83\x91`\x84\x91\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xF1\x90\x81\x15a\x06KW_\x91a>\x1EWP\x90V[a\x07f\x91P` =` \x11a\x06DWa\x066\x81\x83a\x06\x9FV[\x91a>bc\xFF\xFF\xFF\xFF\x91`@\x93\x96\x95\x96`\x01\x80`\xA0\x1B\x03\x16\x85R``` \x86\x01R``\x85\x01\x90a\x0F\xC2V[\x94\x16\x91\x01RV[\x93\x92\x90\x91_\x81a>y\x81Qa%~V[\x94a>\x98`@Q\x95\x86\x93\x84\x93c%5\xF4\x03`\xE2\x1B\x85R`\x04\x85\x01a>7V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06KW_\x92a?\x04W[P_[\x81Q\x81\x10\x15a4ZW\x80a>\xF3a4(a\x08\xD3`\x01\x94\x86a&>V[a>\xFD\x82\x87a&>V[R\x01a>\xD7V[a?\x19\x91\x92P=\x80_\x83>a4w\x81\x83a\x06\x9FV[\x90_a>\xD4V[_\x19\x81\x01\x91\x90\x82\x11a\x18\xB7WV[\x91\x90\x82\x03\x91\x82\x11a\x18\xB7WV[c\xFF\xFF\xFF\xFF_\x19\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x18\xB7WV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x03\x90c\xFF\xFF\xFF\xFF\x82\x11a\x18\xB7WV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\xA5` \x90\x81R`@\x80\x83 \x94\x86\x16\x83R\x93\x90R\x91\x90\x91 \x90\x94\x93\x92\x91a?\xC7\x91a?\xA6\x90aJ\xB4V[\x95_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a?\xFAa\x17\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFFC\x16a?SV[\x81T\x91\x90c\xFF\xFF\xFF\xFF\x16_[\x83\x81\x10a@*WPP\x94a\x18'\x91a\x07f\x95\x96\x81\x15_\x14a\x18<WP_\x90Pa\x14\xFEV[\x90\x92\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a\x18\xB7W\x83_R\x84c\xFF\xFF\xFF\xFF\x83` _ \x01T\x16\x11_\x14a@`WP\x92[\x90a@\x06V[\x93\x91P`\x01\x81\x01\x80\x91\x11a\x18\xB7W\x90a@ZV[`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\x98` Ra@\xA2\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x91\x82T\x82\x81\x03\x90\x81\x11a\x18\xB7W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x93Ua@\xE2`@Q\x92\x83\x92_\x84a4\xB2V[\x03\x90\xA2V[\x91\x90\x91`\x01\x80`\xA0\x1B\x03\x16\x91\x82_R`\x98` R`@_ s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0_R` R`@_ \x90\x81T\x91\x83\x83\x03\x92\x83\x11a\x18\xB7W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x93s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x93a@\xE2\x92U`@Q\x93\x84\x93\x84a4\xB2V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x98` \x90\x81R`@\x80\x83 \x94\x87\x16\x83R\x93\x90R\x91\x90\x91 \x80T\x91\x94\x80\x83\x03\x94\x93\x92\x85\x11a\x18\xB7W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x94a@\xE2\x92U`@Q\x93\x84\x93\x84a4\xB2V[`\x01`\x01`\xA0\x1B\x03\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x03aB.W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x07f\x92\x91aBlaBr\x92aG?V[\x90aK\xC3V[aK\xC3V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aB\x8BWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x15aC\x1EWV[c\x16\x11\r5`\xE2\x1B_R`\x04_\xFD[\x15aC4WV[c\x87\xC9\xD2\x19`\xE0\x1B_R`\x04_\xFD[\x15aCJWV[cx\xF6z\xE1`\xE1\x1B_R`\x04_\xFD[_`\x06a\x03\x9D\x92\x82\x81U\x82`\x01\x82\x01U\x82`\x02\x82\x01U\x82`\x03\x82\x01U\x82`\x04\x82\x01UaC\x8E\x83`\x05\x83\x01\x80T\x90\x82\x81Ua4\xFFV[\x01\x80T\x90\x82\x81Ua4\xFFV[5a\x07f\x81a\x03\x81V[\x91\x90\x82`@\x91\x03\x12a\x03CW` \x82Q\x92\x01Q\x90V[\x93\x92\x93`\xA0\x81\x01\x92aC\xCF\x84QQ\x82\x14a&\x1BV[`@\x82\x01QaC\xF1\x90aC\xEA\x90`\x01`\x01`\xA0\x1B\x03\x16a\x15$V[3\x14aC\x17V[aC\xFA\x82a'aV[aD\x1EaD\x19aD\x12\x83_R`\x9E` R`@_ \x90V[T`\xFF\x16\x90V[aC-V[\x7F\x1F@@\x08\x89'N\xD0{$\x84^PT\xA8z\x0C\xAB\x96\x9E\xB1'z\xAF\xE6\x1A\xE3R\xE7\xC3*\0aE\x0FaD\xB8aD\x80aDY`\x80\x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a)\x95V[aD\x97c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFFC\x16\x11aCCV[\x86Q`\x01`\x01`\xA0\x1B\x03\x16` \x88\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8AQ\x91a>iV[\x92aD\xD5\x81aD\xD0a9\\\x89Q`\x01\x80`\xA0\x1B\x03\x16\x90V[aM}V[PaD\xF0aD\xEB\x82_R`\xA4` R`@_ \x90V[aCYV[a\x18,aE\x05\x82_R`\x9E` R`@_ \x90V[\x80T`\xFF\x19\x16\x90UV[\x03\x90\xA1\x82Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9A` R`@\x90 aE3\x90a\x12zV[\x83QaEK\x90`\x01`\x01`\xA0\x1B\x03\x16\x82\x88Q\x91a3\x9FV[_[\x87Q\x80Q\x82\x10\x15aG2W\x90\x88\x88\x88\x83\x89\x8F\x96a\x1E\xF4a\x08\xD3\x84aEp\x93a&>V[aE\x86\x8Ba\x13\xD1\x85a\x13\xCA\x81`\xC0\x8A\x01Qa&>V[\x97\x15aF9W\x92Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93aE\xC3\x93aE\xBE\x93\x90\x92\x90\x91aE\xB8\x91a\x08\xD3\x91\x85\x91\x16\x99Qa&>V[\x95a/\x0FV[aC\x9AV[\x91\x81;\x15a\x03CW`@Qc\x0B\xAB\x90c`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x90\x84\x16`$\x82\x01R\x91\x90\x92\x16`D\x82\x01R`d\x81\x01\x93\x90\x93R_\x90\x83\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x06KW`\x01\x92aF%W[P[\x01aEMV[\x80a\n\xEE_aF3\x93a\x06\x9FV[_aF\x1DV[\x92aE\xBE\x83_\x93aE\xB8a\x08\xD3`@\x9A\x99\x97aF^aFe\x97Q`\x01\x80`\xA0\x1B\x03\x16\x90V[\x9AQa&>V[\x85Qc\xC4b>\xA1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x04\x82\x01R\x92\x85\x16`$\x84\x01R\x84\x16`D\x83\x01R`d\x82\x01\x96\x90\x96R\x94\x85\x92`\x84\x92\x84\x92\x91\x16Z\xF1\x80\x15a\x06KW\x89aF\xEA\x91`\x01\x94\x84\x8B_\x92_\x94aF\xEFW[PQaF\xD8\x91a\x08\xD3\x91`\x01`\x01`\xA0\x1B\x03\x16[\x95Qa&>V[aF\xE2\x86\x89a&>V[Q\x93\x89a=4V[aF\x1FV[a\x08\xD3\x91\x94PaF\xD1\x93P\x91aG\x1EaF\xD8\x93`@=\x81\x11aG+W[aG\x16\x81\x83a\x06\x9FV[\x81\x01\x90aC\xA4V[\x94\x90\x94\x95\x92PP\x91aF\xBDV[P=aG\x0CV[PPPPPPPPP\x90PV[Q\x80a\x07fWPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[a\x07f\x90aGga\x04\x99`\x02\x80`fT\x16\x14\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9A` R`@\x90 ``\x92\x91\x90aG\x8C\x90a\x12zV[\x90aG\xBAaG\xAA\x82`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x90\x82\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv_\x80\xA3aG\xF5\x81a,4V[\x91\x90\x94\x85Q\x90\x81\x15aH\xA8WPaH\x0B\x90a%~V[\x92aH\x17\x86\x82\x84a3\x9FV[\x91_[\x87Q\x81\x10\x15aH\x9EW`\x01\x90aH\x8D\x89aH2a&\x9EV[aH:a&\x9EV[\x90aH\\aHSa\x08\xD3\x87aHMa&\x9EV[\x96a&>V[a-\x98\x83a&1V[aHf\x85\x8Ba&>V[QaHp\x83a&1V[RaH{\x85\x8Aa&>V[QaH\x85\x84a&1V[R\x87\x87a6\xB0V[aH\x97\x82\x89a&>V[R\x01aH\x1AV[P\x93\x95PPPPPV[\x95PPPPPV[\x15aH\xB7WV[c\rLL\x91`\xE2\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x91\x94\x91\x16\x92\x90\x83\x15aIgWa\x03\x9D\x94aI]\x91\x85_R`\x9C` R`@_ \x81_R` RaI!aI\x1C`\xFF`@_ T\x16\x15\x15\x15\x90V[aH\xB0V[aIOa94\x82aIB\x89`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R`@_ \x90V[\x90_R` R`@_ \x90V[\x85` \x85\x01\x95\x86Q\x93a$\xBEV[\x90Q\x91Q\x92aL\x8EV[PPPPPV[\x90aBla\x07f\x92aG?V[\x90\x91\x90s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xBF\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01aI\xA8WPPPV[aI\xFE\x90`\x01\x80`\xA0\x1B\x03\x16\x92\x83_R`\xA5` RaI\xDDa\x17\x92\x82`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x93_R`\xA5` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x90\x82\x01\x80\x92\x11a\x18\xB7Wa\x03\x9D\x91`\x01`\x01`\xE0\x1B\x03\x16\x90Cc\xFF\xFF\xFF\xFF\x16\x90aO\xE8V[\x92\x90\x91\x82\x15aJfWaJE\x82aBraJ?a\x1E\x1C\x88a&\xC0V[\x86aK\xC3V[\x90\x80\x82\x01\x80\x92\x11a\x18\xB7W\x83\x01\x80\x93\x11a\x18\xB7Wa4\xFB\x92a:\x0E\x91aK#V[Pa4\xFB\x91PaL=V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x91\x90aJ\x92\x82\x82\x85aLWV[\x92\x82\x15aJ\xAFW\taJ\xA1W\x90V[`\x01\x81\x01\x80\x91\x11a\x18\xB7W\x90V[aJqV[\x80T\x90\x81aJ\xC3WP_\x91\x90PV[\x81_\x19\x81\x01\x11a\x18\xB7W_R_\x19\x90` _ \x01\x01T` \x1Ca\x18aV[\x91`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01`@\x1B\x03\x82\x11a\x18\xB7W`\x01`\x01`@\x1B\x03a\x07f\x92\x16\x90aK\xC3V[\x81\x15aJ\xAFW\x04\x90V[\x15a\x03CWV[_\x19g\r\xE0\xB6\xB3\xA7d\0\0\x82\t\x91g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x82\x80\x85\x10\x94\x03\x93\x80\x85\x03\x94\x14aK\xB7Wg\r\xE0\xB6\xB3\xA7d\0\0\x82\x91aKc\x86\x84\x11aK\x1CV[\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x81`\x03\x02\x18\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x91\x02`\x02\x03\x02\x93`\x01\x83\x80_\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[P\x90a\x07f\x92PaK\x12V[_\x19\x82\x82\t\x82\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14aL,W\x81g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x03CW\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x93g\r\xE0\xB6\xB3\xA7d\0\0\x91\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x92PP\x04\x90V[\x80\x15aJ\xAFWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[\x90\x91_\x19\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14aL\x81W\x90\x82\x91aKc\x86\x84\x11aK\x1CV[PP\x90a\x07f\x92PaK\x12V[\x92B\x11aL\xB4WaL\x9E\x92aN\xBFV[\x15aL\xA5WV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[c\x08\x19\xBD\xCD`\xE0\x1B_R`\x04_\xFD[\x80T\x82\x10\x15a\x1C\x12W_R` _ \x01\x90_\x90V[`\x01\x81\x01\x90\x82_R\x81` R`@_ T\x15_\x14aM;W\x80T`\x01`@\x1B\x81\x10\x15a\x06\x7FWaM(aM\x12\x82`\x01\x87\x94\x01\x85U\x84aL\xC3V[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90UT\x91_R` R`@_ U`\x01\x90V[PPP_\x90V[\x80T\x80\x15aMiW_\x19\x01\x90aMX\x82\x82aL\xC3V[\x81T\x90_\x19\x90`\x03\x1B\x1B\x19\x16\x90UUV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`\x01\x81\x01\x91\x80_R\x82` R`@_ T\x92\x83\x15\x15_\x14aN\x18W_\x19\x84\x01\x84\x81\x11a\x18\xB7W\x83T_\x19\x81\x01\x94\x90\x85\x11a\x18\xB7W_\x95\x85\x83aIB\x94aM\xCB\x98\x03aM\xD1W[PPPaMBV[U`\x01\x90V[aN\x01aM\xFB\x91aM\xF2aM\xE8aN\x0F\x95\x88aL\xC3V[\x90T\x90`\x03\x1B\x1C\x90V[\x92\x83\x91\x87aL\xC3V[\x90a4\xE2V[\x85\x90_R` R`@_ \x90V[U_\x80\x80aM\xC3V[PPPP_\x90V[`\x05\x11\x15aN*WV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90``\x92` \x91\x83R`@\x82\x84\x01R\x80Q\x91\x82\x91\x82`@\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[=\x15aN\x9AW=\x90aN\x81\x82a\x1C\\V[\x91aN\x8F`@Q\x93\x84a\x06\x9FV[\x82R=_` \x84\x01>V[``\x90V[\x90\x81` \x91\x03\x12a\x03CWQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x03CW\x90V[\x91\x90\x91aN\xCC\x82\x84aP\xC1V[aN\xD5\x81aN V[\x15\x90\x81aOfW[PaO^W_\x92a'~aO\n\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01aN>V[Q\x91Z\xFAaO\x16aNpV[\x81aORW[\x81aO%WP\x90V[\x80Qc\x0B\x13]?`\xE1\x1B\x92P`\x01`\x01`\xE0\x1B\x03\x19\x91aOM\x91\x81\x01` \x90\x81\x01\x91\x01aN\x9FV[\x16\x14\x90V[\x80Q` \x14\x91PaO\x1CV[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_aN\xDDV[\x15aO\x83WV[c\x15\x1B\x8E?`\xE1\x1B_R`\x04_\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x06\x7FWaO\xAF\x91`\x01\x82\x01\x81UaL\xC3V[aO\xD5W\x81Q` \x92\x83\x01Q\x90\x92\x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80T\x80aP$W[PaP\x1Fa\x03\x9D\x93aP\x0FaP\x03a\x06\xCFV[c\xFF\xFF\xFF\xFF\x90\x95\x16\x85RV[`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01RV[aO\x92V[\x80_\x19\x81\x01\x11a\x18\xB7W\x81_Rc\xFF\xFF\xFF\xFFaP\x92aP\x89_\x19\x84` _ \x01\x01aP\x7FaPq`@Q\x92aPX\x84a\x06\x84V[T\x86\x81\x16\x90\x81\x85R` \x1C` \x85\x01Rc\xFF\xFF\xFF\xFF\x16\x90V[\x85\x89\x16\x95\x86\x91\x16\x11\x15aO|V[Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[\x03aO\xF0Wa\x03\x9D\x93\x92P\x90a\x18OaP\xAA\x92a? V[\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV[\x81Q`A\x81\x03aP\xEDWP\x90aP\xE9\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aQ/V[\x90\x91V[`@\x03aQ&W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a\x18\xB7WaP\xE9\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90aQ/V[PP_\x90`\x02\x90V[\x92\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11aQ\xCDW`\xFF\x16`\x1B\x81\x14\x15\x80aQ\xC2W[aQ\xB7W` \x93_\x93`@Q\x93\x84\x93`\x80\x85\x01\x93\x85R\x87\x85\x01R`@\x84\x01R``\x83\x01R\x83\x80R\x03\x90`\x01Z\xFA\x15a\x06KW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aQ\xAFW\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x81\x14\x15aQgV[PPPP_\x90`\x03\x90V\xFE\xA2dipfsX\"\x12 ;5\x9E@o\xA0+\x81\xFA\xAB\xCAg\xE6\x01\xF9\x10U\xD3\x9D\xDE\xF3\x16-\xB3(\x99\xCC\x12\x8E\xE4\xB6\xD0dsolcC\0\x08\x1B\x003",
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
    /**Function with signature `completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],bool[])` and selector `0x9435bb43`.
```solidity
function completeQueuedWithdrawals(IDelegationManagerTypes.Withdrawal[] memory withdrawals, address[][] memory tokens, bool[] memory receiveAsTokens) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalsCall {
        pub withdrawals: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        >,
        pub tokens: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        pub receiveAsTokens: alloy::sol_types::private::Vec<bool>,
    }
    ///Container type for the return parameters of the [`completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],bool[])`](completeQueuedWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalsReturn {}
    #[allow(
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
            impl ::core::convert::From<completeQueuedWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalsCall) -> Self {
                    (value.withdrawals, value.tokens, value.receiveAsTokens)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawalsCall {
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
            impl ::core::convert::From<completeQueuedWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeQueuedWithdrawalsCall {
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
            type Return = completeQueuedWithdrawalsReturn;
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
    /**Function with signature `convertToDepositShares(address,address[],uint256[])` and selector `0x25df922e`.
```solidity
function convertToDepositShares(address staker, address[] memory strategies, uint256[] memory withdrawableShares) external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct convertToDepositSharesCall {
        pub staker: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub withdrawableShares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`convertToDepositShares(address,address[],uint256[])`](convertToDepositSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct convertToDepositSharesReturn {
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<convertToDepositSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: convertToDepositSharesCall) -> Self {
                    (value.staker, value.strategies, value.withdrawableShares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for convertToDepositSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        strategies: tuple.1,
                        withdrawableShares: tuple.2,
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
            impl ::core::convert::From<convertToDepositSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: convertToDepositSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for convertToDepositSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for convertToDepositSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = convertToDepositSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "convertToDepositShares(address,address[],uint256[])";
            const SELECTOR: [u8; 4] = [37u8, 223u8, 146u8, 46u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawableShares),
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
    /**Function with signature `getQueuedWithdrawal(bytes32)` and selector `0x5d975e88`.
```solidity
function getQueuedWithdrawal(bytes32 withdrawalRoot) external view returns (IDelegationManagerTypes.Withdrawal memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalCall {
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getQueuedWithdrawal(bytes32)`](getQueuedWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalReturn {
        pub _0: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getQueuedWithdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalCall) -> Self {
                    (value.withdrawalRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalRoot: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<getQueuedWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQueuedWithdrawalCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQueuedWithdrawalReturn;
            type ReturnTuple<'a> = (IDelegationManagerTypes::Withdrawal,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQueuedWithdrawal(bytes32)";
            const SELECTOR: [u8; 4] = [93u8, 151u8, 94u8, 136u8];
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
    /**Function with signature `getQueuedWithdrawalRoots(address)` and selector `0xfd8aa88d`.
```solidity
function getQueuedWithdrawalRoots(address staker) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalRootsCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getQueuedWithdrawalRoots(address)`](getQueuedWithdrawalRootsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalRootsReturn {
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
            impl ::core::convert::From<getQueuedWithdrawalRootsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalRootsCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalRootsCall {
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
            impl ::core::convert::From<getQueuedWithdrawalRootsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalRootsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalRootsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQueuedWithdrawalRootsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQueuedWithdrawalRootsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQueuedWithdrawalRoots(address)";
            const SELECTOR: [u8; 4] = [253u8, 138u8, 168u8, 141u8];
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
    /**Function with signature `slashOperatorShares(address,address,uint64,uint64)` and selector `0x601bb36f`.
```solidity
function slashOperatorShares(address operator, address strategy, uint64 prevMaxMagnitude, uint64 newMaxMagnitude) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorSharesCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub prevMaxMagnitude: u64,
        pub newMaxMagnitude: u64,
    }
    ///Container type for the return parameters of the [`slashOperatorShares(address,address,uint64,uint64)`](slashOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorSharesReturn {}
    #[allow(
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
            impl ::core::convert::From<slashOperatorSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorSharesCall) -> Self {
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
            for slashOperatorSharesCall {
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
            impl ::core::convert::From<slashOperatorSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashOperatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashOperatorSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashOperatorShares(address,address,uint64,uint64)";
            const SELECTOR: [u8; 4] = [96u8, 27u8, 179u8, 111u8];
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
        calculateDelegationApprovalDigestHash(calculateDelegationApprovalDigestHashCall),
        calculateWithdrawalRoot(calculateWithdrawalRootCall),
        completeQueuedWithdrawal(completeQueuedWithdrawalCall),
        completeQueuedWithdrawals(completeQueuedWithdrawalsCall),
        convertToDepositShares(convertToDepositSharesCall),
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
        getQueuedWithdrawal(getQueuedWithdrawalCall),
        getQueuedWithdrawalRoots(getQueuedWithdrawalRootsCall),
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
        redelegate(redelegateCall),
        registerAsOperator(registerAsOperatorCall),
        renounceOwnership(renounceOwnershipCall),
        slashOperatorShares(slashOperatorSharesCall),
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
            [37u8, 223u8, 146u8, 46u8],
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
            [93u8, 151u8, 94u8, 136u8],
            [93u8, 214u8, 133u8, 121u8],
            [96u8, 27u8, 179u8, 111u8],
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
            [238u8, 169u8, 6u8, 75u8],
            [240u8, 224u8, 230u8, 118u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 152u8, 218u8, 37u8],
            [250u8, 188u8, 28u8, 188u8],
            [253u8, 138u8, 168u8, 141u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelegationManagerCalls {
        const NAME: &'static str = "DelegationManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 50usize;
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
                Self::calculateDelegationApprovalDigestHash(_) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateWithdrawalRoot(_) => {
                    <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeQueuedWithdrawal(_) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeQueuedWithdrawals(_) => {
                    <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::convertToDepositShares(_) => {
                    <convertToDepositSharesCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getQueuedWithdrawal(_) => {
                    <getQueuedWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQueuedWithdrawalRoots(_) => {
                    <getQueuedWithdrawalRootsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::redelegate(_) => {
                    <redelegateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerAsOperator(_) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashOperatorShares(_) => {
                    <slashOperatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn convertToDepositShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <convertToDepositSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::convertToDepositShares)
                    }
                    convertToDepositShares
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
                    fn getQueuedWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getQueuedWithdrawal)
                    }
                    getQueuedWithdrawal
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
                    fn slashOperatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <slashOperatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::slashOperatorShares)
                    }
                    slashOperatorShares
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
                    fn completeQueuedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::completeQueuedWithdrawals)
                    }
                    completeQueuedWithdrawals
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
                {
                    fn getQueuedWithdrawalRoots(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getQueuedWithdrawalRootsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getQueuedWithdrawalRoots)
                    }
                    getQueuedWithdrawalRoots
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
                Self::completeQueuedWithdrawals(inner) => {
                    <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::convertToDepositShares(inner) => {
                    <convertToDepositSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getQueuedWithdrawal(inner) => {
                    <getQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQueuedWithdrawalRoots(inner) => {
                    <getQueuedWithdrawalRootsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::slashOperatorShares(inner) => {
                    <slashOperatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::completeQueuedWithdrawals(inner) => {
                    <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::convertToDepositShares(inner) => {
                    <convertToDepositSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getQueuedWithdrawal(inner) => {
                    <getQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQueuedWithdrawalRoots(inner) => {
                    <getQueuedWithdrawalRootsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::slashOperatorShares(inner) => {
                    <slashOperatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        WithdrawalNotQueued(WithdrawalNotQueued),
        WithdrawerNotCaller(WithdrawerNotCaller),
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
            [198u8, 29u8, 202u8, 93u8],
            [200u8, 78u8, 153u8, 132u8],
            [241u8, 236u8, 245u8, 194u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelegationManagerErrors {
        const NAME: &'static str = "DelegationManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 24usize;
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
                Self::WithdrawalNotQueued(_) => {
                    <WithdrawalNotQueued as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawerNotCaller(_) => {
                    <WithdrawerNotCaller as alloy_sol_types::SolError>::SELECTOR
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
        const COUNT: usize = 15usize;
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
        ///Creates a new call builder for the [`completeQueuedWithdrawals`] function.
        pub fn completeQueuedWithdrawals(
            &self,
            withdrawals: alloy::sol_types::private::Vec<
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
            >,
            tokens: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            receiveAsTokens: alloy::sol_types::private::Vec<bool>,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeQueuedWithdrawalsCall, N> {
            self.call_builder(
                &completeQueuedWithdrawalsCall {
                    withdrawals,
                    tokens,
                    receiveAsTokens,
                },
            )
        }
        ///Creates a new call builder for the [`convertToDepositShares`] function.
        pub fn convertToDepositShares(
            &self,
            staker: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            withdrawableShares: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, convertToDepositSharesCall, N> {
            self.call_builder(
                &convertToDepositSharesCall {
                    staker,
                    strategies,
                    withdrawableShares,
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
        ///Creates a new call builder for the [`getQueuedWithdrawal`] function.
        pub fn getQueuedWithdrawal(
            &self,
            withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQueuedWithdrawalCall, N> {
            self.call_builder(
                &getQueuedWithdrawalCall {
                    withdrawalRoot,
                },
            )
        }
        ///Creates a new call builder for the [`getQueuedWithdrawalRoots`] function.
        pub fn getQueuedWithdrawalRoots(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQueuedWithdrawalRootsCall, N> {
            self.call_builder(
                &getQueuedWithdrawalRootsCall {
                    staker,
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
        ///Creates a new call builder for the [`slashOperatorShares`] function.
        pub fn slashOperatorShares(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            prevMaxMagnitude: u64,
            newMaxMagnitude: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashOperatorSharesCall, N> {
            self.call_builder(
                &slashOperatorSharesCall {
                    operator,
                    strategy,
                    prevMaxMagnitude,
                    newMaxMagnitude,
                },
            )
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
