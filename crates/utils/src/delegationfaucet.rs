///Module containing a contract's types and functions.
/**

```solidity
library IDelegationManager {
    struct QueuedWithdrawalParams { address[] strategies; uint256[] shares; address withdrawer; }
    struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startBlock; address[] strategies; uint256[] shares; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IDelegationManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct QueuedWithdrawalParams { address[] strategies; uint256[] shares; address withdrawer; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QueuedWithdrawalParams {
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub shares: alloy::sol_types::private::Vec<
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
                (value.strategies, value.shares, value.withdrawer)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QueuedWithdrawalParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategies: tuple.0,
                    shares: tuple.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
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
                    "QueuedWithdrawalParams(address[] strategies,uint256[] shares,address withdrawer)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.shares)
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
                        &rust.shares,
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
                    &rust.shares,
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
struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startBlock; address[] strategies; uint256[] shares; }
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
        pub shares: alloy::sol_types::private::Vec<
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
                    value.shares,
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
                    shares: tuple.6,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
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
                    "Withdrawal(address staker,address delegatedTo,address withdrawer,uint256 nonce,uint32 startBlock,address[] strategies,uint256[] shares)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.shares)
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
                        &rust.shares,
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
                    &rust.shares,
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
    /**Creates a new wrapper around an on-chain [`IDelegationManager`](self) contract instance.

See the [wrapper's documentation](`IDelegationManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IDelegationManagerInstance<T, P, N> {
        IDelegationManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IDelegationManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IDelegationManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IDelegationManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IDelegationManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IDelegationManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IDelegationManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IDelegationManager`](self) contract instance.

See the [wrapper's documentation](`IDelegationManagerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IDelegationManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IDelegationManagerInstance<T, P, N> {
            IDelegationManagerInstance {
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
    > IDelegationManagerInstance<T, P, N> {
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
    > IDelegationManagerInstance<T, P, N> {
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
library IDelegationManager {
    struct QueuedWithdrawalParams {
        address[] strategies;
        uint256[] shares;
        address withdrawer;
    }
    struct Withdrawal {
        address staker;
        address delegatedTo;
        address withdrawer;
        uint256 nonce;
        uint32 startBlock;
        address[] strategies;
        uint256[] shares;
    }
}

library ISignatureUtils {
    struct SignatureWithExpiry {
        bytes signature;
        uint256 expiry;
    }
}

interface DelegationFaucet {
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    constructor(address _strategyManager, address _delegation, address _token, address _strategy);

    function DEFAULT_AMOUNT() external view returns (uint256);
    function callAddress(address to, bytes memory data) external payable returns (bytes memory);
    function completeQueuedWithdrawal(address staker, IDelegationManager.Withdrawal memory queuedWithdrawal, address[] memory tokens, uint256 middlewareTimesIndex, bool receiveAsTokens) external returns (bytes memory);
    function delegateTo(address _operator, ISignatureUtils.SignatureWithExpiry memory _approverSignatureAndExpiry, bytes32 _approverSalt) external returns (bytes memory);
    function depositIntoStrategy(address _staker, address _strategy, address _token, uint256 _amount) external returns (bytes memory);
    function getStaker(address _operator) external view returns (address);
    function mintDepositAndDelegate(address _operator, ISignatureUtils.SignatureWithExpiry memory _approverSignatureAndExpiry, bytes32 _approverSalt, uint256 _depositAmount) external;
    function owner() external view returns (address);
    function queueWithdrawal(address staker, IDelegationManager.QueuedWithdrawalParams[] memory queuedWithdrawalParams) external returns (bytes memory);
    function renounceOwnership() external;
    function transfer(address staker, address token, address to, uint256 amount) external returns (bytes memory);
    function transferOwnership(address newOwner) external;
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
        "name": "_delegation",
        "type": "address",
        "internalType": "contract IDelegationManager"
      },
      {
        "name": "_token",
        "type": "address",
        "internalType": "contract ERC20PresetMinterPauser"
      },
      {
        "name": "_strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "DEFAULT_AMOUNT",
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
    "name": "callAddress",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "completeQueuedWithdrawal",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "queuedWithdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManager.Withdrawal",
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
            "name": "shares",
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
        "name": "middlewareTimesIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "receiveAsTokens",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateTo",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_approverSignatureAndExpiry",
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
        "name": "_approverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "depositIntoStrategy",
    "inputs": [
      {
        "name": "_staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "_token",
        "type": "address",
        "internalType": "contract IERC20"
      },
      {
        "name": "_amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getStaker",
    "inputs": [
      {
        "name": "_operator",
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
    "name": "mintDepositAndDelegate",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_approverSignatureAndExpiry",
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
        "name": "_approverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_depositAmount",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "queueWithdrawal",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "queuedWithdrawalParams",
        "type": "tuple[]",
        "internalType": "struct IDelegationManager.QueuedWithdrawalParams[]",
        "components": [
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "shares",
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
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
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
    "name": "transfer",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
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
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod DelegationFaucet {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101006040523480156200001257600080fd5b5060405162001f9038038062001f908339810160408190526200003591620000cc565b620000403362000063565b6001600160a01b0393841660805291831660e052821660a0521660c05262000134565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6001600160a01b0381168114620000c957600080fd5b50565b60008060008060808587031215620000e357600080fd5b8451620000f081620000b3565b60208601519094506200010381620000b3565b60408601519093506200011681620000b3565b60608601519092506200012981620000b3565b939692955090935050565b60805160a05160c05160e051611dce620001c260003960008181610317015281816103c301528181610680015281816109980152610a89015260008181610619015261083e015260008181610534015281816105b40152818161063a0152818161078e01528181610801015261089e0152600081816105030152818161075d0152610e7c0152611dce6000f3fe608060405260043610620000b55760003560e01c8063a49ca158116200006c578063a49ca15814620001bd578063a76a9d2d14620001e2578063eea9064b1462000207578063f18d03cc146200022c578063f2fde38b1462000251578063f6e8f39d146200027657600080fd5b8063063dcd5014620000ba5780633448950614620000f75780636850e5c1146200011e578063715018a6146200014c5780638da5cb5b1462000164578063a23c44b11462000198575b600080fd5b348015620000c757600080fd5b50620000df620000d936600462000fea565b6200028d565b604051620000ee9190620010f6565b60405180910390f35b3480156200010457600080fd5b506200011c6200011636600462001252565b6200039a565b005b3480156200012b57600080fd5b506200013d68056bc75e2d6310000081565b604051908152602001620000ee565b3480156200015957600080fd5b506200011c62000709565b3480156200017157600080fd5b506000546001600160a01b03165b6040516001600160a01b039091168152602001620000ee565b348015620001a557600080fd5b506200017f620001b7366004620012b8565b62000721565b348015620001ca57600080fd5b50620000df620001dc366004620012d8565b620007f3565b348015620001ef57600080fd5b50620000df6200020136600462001330565b62000914565b3480156200021457600080fd5b50620000df620002263660046200138b565b62000a0d565b3480156200023957600080fd5b50620000df6200024b366004620012d8565b62000ac8565b3480156200025e57600080fd5b506200011c62000270366004620012b8565b62000b8e565b620000df62000287366004620013ea565b62000c0d565b60606200029962000caa565b60006360d7faed60e01b8787878787604051602401620002be95949392919062001525565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905163f6e8f39d60e01b81529091506001600160a01b0389169063f6e8f39d9062000342907f000000000000000000000000000000000000000000000000000000000000000090859060040162001629565b6000604051808303816000875af115801562000362573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526200038c91908101906200164f565b9150505b9695505050505050565b620003a462000caa565b6040516336b87bd760e11b81526001600160a01b0385811660048301527f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90602401602060405180830381865afa1580156200040b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620004319190620016c6565b620004955760405162461bcd60e51b815260206004820152602960248201527f44656c65676174696f6e4661756365743a204f70657261746f72206e6f7420726044820152681959da5cdd195c995960ba1b60648201526084015b60405180910390fd5b6000620004a28562000721565b905081620004b75768056bc75e2d6310000091505b6001600160a01b0381163b6200058e576200058c6000866001600160a01b031660001b60405180602001620004ec9062000f56565b601f1982820381018352601f9091011660408181527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0390811660208401527f0000000000000000000000000000000000000000000000000000000000000000168183015280518083038201815260608301909152620005779291608001620016e6565b60405160208183030381529060405262000d06565b505b6040516340c10f1960e01b81526001600160a01b038281166004830152602482018490527f000000000000000000000000000000000000000000000000000000000000000016906340c10f1990604401600060405180830381600087803b158015620005f957600080fd5b505af11580156200060e573d6000803e3d6000fd5b5050505062000660817f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000008562000e0e565b50604051633e28391d60e01b81526001600160a01b0382811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633e28391d90602401602060405180830381865afa158015620006c8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620006ee9190620016c6565b62000702576200070085858562000a0d565b505b5050505050565b6200071362000caa565b6200071f600062000ea7565b565b6000620007ed826001600160a01b031660001b60405180602001620007469062000f56565b601f1982820381018352601f9091011660408181527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0390811660208401527f0000000000000000000000000000000000000000000000000000000000000000168183015280518083038201815260608301909152620007d19291608001620016e6565b6040516020818303038152906040528051906020012062000ef7565b92915050565b6060620007ff62000caa565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316836001600160a01b03161480156200087257507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316846001600160a01b0316145b15620008fd576040516340c10f1960e01b81526001600160a01b038681166004830152602482018490527f000000000000000000000000000000000000000000000000000000000000000016906340c10f1990604401600060405180830381600087803b158015620008e357600080fd5b505af1158015620008f8573d6000803e3d6000fd5b505050505b6200090b8585858562000e0e565b95945050505050565b60606200092062000caa565b6000630dd8dd0260e01b84846040516024016200093f92919062001719565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905163f6e8f39d60e01b81529091506001600160a01b0386169063f6e8f39d90620009c3907f000000000000000000000000000000000000000000000000000000000000000090859060040162001629565b6000604051808303816000875af1158015620009e3573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526200090b91908101906200164f565b606062000a1962000caa565b600063eea9064b60e01b85858560405160240162000a3a93929190620017f0565b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152905062000a798562000721565b6001600160a01b031663f6e8f39d7f0000000000000000000000000000000000000000000000000000000000000000836040518363ffffffff1660e01b8152600401620009c392919062001629565b606062000ad462000caa565b604080516001600160a01b038581166024830152604480830186905283518084039091018152606490920183526020820180516001600160e01b031663a9059cbb60e01b179052915163f6e8f39d60e01b8152909187169063f6e8f39d9062000b44908890859060040162001629565b6000604051808303816000875af115801562000b64573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526200039091908101906200164f565b62000b9862000caa565b6001600160a01b03811662000bff5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016200048c565b62000c0a8162000ea7565b50565b606062000c1962000caa565b600080846001600160a01b0316348560405162000c37919062001833565b60006040518083038185875af1925050503d806000811462000c76576040519150601f19603f3d011682016040523d82523d6000602084013e62000c7b565b606091505b50915091508162000ca2578060405162461bcd60e51b81526004016200048c9190620010f6565b949350505050565b6000546001600160a01b031633146200071f5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016200048c565b6000808447101562000d5b5760405162461bcd60e51b815260206004820152601d60248201527f437265617465323a20696e73756666696369656e742062616c616e636500000060448201526064016200048c565b825162000dab5760405162461bcd60e51b815260206004820181905260248201527f437265617465323a2062797465636f6465206c656e677468206973207a65726f60448201526064016200048c565b8383516020850187f590506001600160a01b03811662000ca25760405162461bcd60e51b815260206004820152601960248201527f437265617465323a204661696c6564206f6e206465706c6f790000000000000060448201526064016200048c565b604080516001600160a01b0385811660248301528481166044830152606480830185905283518084039091018152608490920183526020820180516001600160e01b03166373d0285560e11b179052915163f6e8f39d60e01b815260609287169063f6e8f39d9062000b44907f000000000000000000000000000000000000000000000000000000000000000090859060040162001629565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b604080516001600160f81b03196020808301919091526bffffffffffffffffffffffff193060601b16602183015260358201859052605580830185905283518084039091018152607590920190925280519101206000905b9392505050565b610547806200185283390190565b6001600160a01b038116811462000c0a57600080fd5b803562000f878162000f64565b919050565b60008083601f84011262000f9f57600080fd5b50813567ffffffffffffffff81111562000fb857600080fd5b6020830191508360208260051b850101111562000fd457600080fd5b9250929050565b801515811462000c0a57600080fd5b60008060008060008060a087890312156200100457600080fd5b8635620010118162000f64565b9550602087013567ffffffffffffffff808211156200102f57600080fd5b9088019060e0828b0312156200104457600080fd5b909550604088013590808211156200105b57600080fd5b506200106a89828a0162000f8c565b909550935050606087013591506080870135620010878162000fdb565b809150509295509295509295565b60005b83811015620010b257818101518382015260200162001098565b83811115620010c2576000848401525b50505050565b60008151808452620010e281602086016020860162001095565b601f01601f19169290920160200192915050565b60208152600062000f4f6020830184620010c8565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff811182821017156200114d576200114d6200110b565b604052919050565b600067ffffffffffffffff8211156200117257620011726200110b565b50601f01601f191660200190565b600082601f8301126200119257600080fd5b8135620011a9620011a38262001155565b62001121565b818152846020838601011115620011bf57600080fd5b816020850160208301376000918101602001919091529392505050565b600060408284031215620011ef57600080fd5b6040516040810167ffffffffffffffff82821081831117156200121657620012166200110b565b8160405282935084359150808211156200122f57600080fd5b506200123e8582860162001180565b825250602083013560208201525092915050565b600080600080608085870312156200126957600080fd5b8435620012768162000f64565b9350602085013567ffffffffffffffff8111156200129357600080fd5b620012a187828801620011dc565b949794965050505060408301359260600135919050565b600060208284031215620012cb57600080fd5b813562000f4f8162000f64565b60008060008060808587031215620012ef57600080fd5b8435620012fc8162000f64565b935060208501356200130e8162000f64565b92506040850135620013208162000f64565b9396929550929360600135925050565b6000806000604084860312156200134657600080fd5b8335620013538162000f64565b9250602084013567ffffffffffffffff8111156200137057600080fd5b6200137e8682870162000f8c565b9497909650939450505050565b600080600060608486031215620013a157600080fd5b8335620013ae8162000f64565b9250602084013567ffffffffffffffff811115620013cb57600080fd5b620013d986828701620011dc565b925050604084013590509250925092565b60008060408385031215620013fe57600080fd5b82356200140b8162000f64565b9150602083013567ffffffffffffffff8111156200142857600080fd5b620014368582860162001180565b9150509250929050565b803563ffffffff8116811462000f8757600080fd5b6000808335601e198436030181126200146d57600080fd5b830160208101925035905067ffffffffffffffff8111156200148e57600080fd5b8060051b360383131562000fd457600080fd5b8183526000602080850194508260005b85811015620014e3578135620014c78162000f64565b6001600160a01b031687529582019590820190600101620014b1565b509495945050505050565b81835260006001600160fb1b038311156200150857600080fd5b8260051b8083602087013760009401602001938452509192915050565b6080815260008635620015388162000f64565b6001600160a01b031660808301526020870135620015568162000f64565b6001600160a01b031660a0830152620015726040880162000f7a565b6001600160a01b031660c0830152606087013560e0830152620015986080880162001440565b63ffffffff16610100830152620015b360a088018862001455565b60e0610120850152620015cc61016085018284620014a1565b915050620015de60c089018962001455565b848303607f1901610140860152620015f8838284620014ee565b92505050828103602084015262001611818789620014a1565b91505083604083015262000390606083018415159052565b6001600160a01b038316815260406020820181905260009062000ca290830184620010c8565b6000602082840312156200166257600080fd5b815167ffffffffffffffff8111156200167a57600080fd5b8201601f810184136200168c57600080fd5b80516200169d620011a38262001155565b818152856020838501011115620016b357600080fd5b6200090b82602083016020860162001095565b600060208284031215620016d957600080fd5b815162000f4f8162000fdb565b60008351620016fa81846020880162001095565b8351908301906200171081836020880162001095565b01949350505050565b60208082528181018390526000906040808401600586901b850182018785805b89811015620017e157888403603f190185528235368c9003605e1901811262001760578283fd5b8b01606062001770828062001455565b828852620017828389018284620014a1565b92505050620017948983018362001455565b8783038b890152620017a8838284620014ee565b92505050878201359150620017bd8262000f64565b6001600160a01b039190911694870194909452938601939186019160010162001739565b50919998505050505050505050565b60018060a01b03841681526060602082015260008351604060608401526200181c60a0840182620010c8565b602095909501516080840152505060400152919050565b600082516200184781846020870162001095565b919091019291505056fe608060405234801561001057600080fd5b5060405161054738038061054783398101604081905261002f9161011c565b610038336100b4565b60405163095ea7b360e01b81526001600160a01b038381166004830152600019602483015282169063095ea7b3906044016020604051808303816000875af1158015610088573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100ac9190610156565b50505061017f565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6001600160a01b038116811461011957600080fd5b50565b6000806040838503121561012f57600080fd5b825161013a81610104565b602084015190925061014b81610104565b809150509250929050565b60006020828403121561016857600080fd5b8151801515811461017857600080fd5b9392505050565b6103b98061018e6000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c8063715018a6146100515780638da5cb5b1461005b578063f2fde38b1461007b578063f6e8f39d1461008e575b600080fd5b6100596100ae565b005b6000546040516001600160a01b0390911681526020015b60405180910390f35b610059610089366004610234565b6100c2565b6100a161009c36600461026c565b610140565b604051610072919061032e565b6100b661016e565b6100c060006101c8565b565b6100ca61016e565b6001600160a01b0381166101345760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084015b60405180910390fd5b61013d816101c8565b50565b606061014a61016e565b81516060600080836020870134895af1503d81523d6000602083013e949350505050565b6000546001600160a01b031633146100c05760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161012b565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b80356001600160a01b038116811461022f57600080fd5b919050565b60006020828403121561024657600080fd5b61024f82610218565b9392505050565b634e487b7160e01b600052604160045260246000fd5b6000806040838503121561027f57600080fd5b61028883610218565b9150602083013567ffffffffffffffff808211156102a557600080fd5b818501915085601f8301126102b957600080fd5b8135818111156102cb576102cb610256565b604051601f8201601f19908116603f011681019083821181831017156102f3576102f3610256565b8160405282815288602084870101111561030c57600080fd5b8260208601602083013760006020848301015280955050505050509250929050565b600060208083528351808285015260005b8181101561035b5785810183015185820160400152820161033f565b8181111561036d576000604083870101525b50601f01601f191692909201604001939250505056fea2646970667358221220b82e1c181805b1befa2ed814c8d49d18ebf7a66e70bd16f657fb83541156316f64736f6c634300080c0033a2646970667358221220b8f81e353f9ed1fce14779118aeebe4b20e2643944cfce2889dc96b38977346364736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x1F\x908\x03\x80b\0\x1F\x90\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\0\xCCV[b\0\0@3b\0\0cV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x80R\x91\x83\x16`\xE0R\x82\x16`\xA0R\x16`\xC0Rb\0\x014V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xC9W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\0\xE3W`\0\x80\xFD[\x84Qb\0\0\xF0\x81b\0\0\xB3V[` \x86\x01Q\x90\x94Pb\0\x01\x03\x81b\0\0\xB3V[`@\x86\x01Q\x90\x93Pb\0\x01\x16\x81b\0\0\xB3V[``\x86\x01Q\x90\x92Pb\0\x01)\x81b\0\0\xB3V[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x1D\xCEb\0\x01\xC2`\09`\0\x81\x81a\x03\x17\x01R\x81\x81a\x03\xC3\x01R\x81\x81a\x06\x80\x01R\x81\x81a\t\x98\x01Ra\n\x89\x01R`\0\x81\x81a\x06\x19\x01Ra\x08>\x01R`\0\x81\x81a\x054\x01R\x81\x81a\x05\xB4\x01R\x81\x81a\x06:\x01R\x81\x81a\x07\x8E\x01R\x81\x81a\x08\x01\x01Ra\x08\x9E\x01R`\0\x81\x81a\x05\x03\x01R\x81\x81a\x07]\x01Ra\x0E|\x01Ra\x1D\xCE`\0\xF3\xFE`\x80`@R`\x046\x10b\0\0\xB5W`\x005`\xE0\x1C\x80c\xA4\x9C\xA1X\x11b\0\0lW\x80c\xA4\x9C\xA1X\x14b\0\x01\xBDW\x80c\xA7j\x9D-\x14b\0\x01\xE2W\x80c\xEE\xA9\x06K\x14b\0\x02\x07W\x80c\xF1\x8D\x03\xCC\x14b\0\x02,W\x80c\xF2\xFD\xE3\x8B\x14b\0\x02QW\x80c\xF6\xE8\xF3\x9D\x14b\0\x02vW`\0\x80\xFD[\x80c\x06=\xCDP\x14b\0\0\xBAW\x80c4H\x95\x06\x14b\0\0\xF7W\x80chP\xE5\xC1\x14b\0\x01\x1EW\x80cqP\x18\xA6\x14b\0\x01LW\x80c\x8D\xA5\xCB[\x14b\0\x01dW\x80c\xA2<D\xB1\x14b\0\x01\x98W[`\0\x80\xFD[4\x80\x15b\0\0\xC7W`\0\x80\xFD[Pb\0\0\xDFb\0\0\xD96`\x04b\0\x0F\xEAV[b\0\x02\x8DV[`@Qb\0\0\xEE\x91\x90b\0\x10\xF6V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15b\0\x01\x04W`\0\x80\xFD[Pb\0\x01\x1Cb\0\x01\x166`\x04b\0\x12RV[b\0\x03\x9AV[\0[4\x80\x15b\0\x01+W`\0\x80\xFD[Pb\0\x01=h\x05k\xC7^-c\x10\0\0\x81V[`@Q\x90\x81R` \x01b\0\0\xEEV[4\x80\x15b\0\x01YW`\0\x80\xFD[Pb\0\x01\x1Cb\0\x07\tV[4\x80\x15b\0\x01qW`\0\x80\xFD[P`\0T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01b\0\0\xEEV[4\x80\x15b\0\x01\xA5W`\0\x80\xFD[Pb\0\x01\x7Fb\0\x01\xB76`\x04b\0\x12\xB8V[b\0\x07!V[4\x80\x15b\0\x01\xCAW`\0\x80\xFD[Pb\0\0\xDFb\0\x01\xDC6`\x04b\0\x12\xD8V[b\0\x07\xF3V[4\x80\x15b\0\x01\xEFW`\0\x80\xFD[Pb\0\0\xDFb\0\x02\x016`\x04b\0\x130V[b\0\t\x14V[4\x80\x15b\0\x02\x14W`\0\x80\xFD[Pb\0\0\xDFb\0\x02&6`\x04b\0\x13\x8BV[b\0\n\rV[4\x80\x15b\0\x029W`\0\x80\xFD[Pb\0\0\xDFb\0\x02K6`\x04b\0\x12\xD8V[b\0\n\xC8V[4\x80\x15b\0\x02^W`\0\x80\xFD[Pb\0\x01\x1Cb\0\x02p6`\x04b\0\x12\xB8V[b\0\x0B\x8EV[b\0\0\xDFb\0\x02\x876`\x04b\0\x13\xEAV[b\0\x0C\rV[``b\0\x02\x99b\0\x0C\xAAV[`\0c`\xD7\xFA\xED`\xE0\x1B\x87\x87\x87\x87\x87`@Q`$\x01b\0\x02\xBE\x95\x94\x93\x92\x91\x90b\0\x15%V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qc\xF6\xE8\xF3\x9D`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\xF6\xE8\xF3\x9D\x90b\0\x03B\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90`\x04\x01b\0\x16)V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x03bW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x03\x8C\x91\x90\x81\x01\x90b\0\x16OV[\x91PP[\x96\x95PPPPPPV[b\0\x03\xA4b\0\x0C\xAAV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x04\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x041\x91\x90b\0\x16\xC6V[b\0\x04\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FDelegationFaucet: Operator not r`D\x82\x01Rh\x19Y\xDA\\\xDD\x19\\\x99Y`\xBA\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0b\0\x04\xA2\x85b\0\x07!V[\x90P\x81b\0\x04\xB7Wh\x05k\xC7^-c\x10\0\0\x91P[`\x01`\x01`\xA0\x1B\x03\x81\x16;b\0\x05\x8EWb\0\x05\x8C`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\0\x1B`@Q\x80` \x01b\0\x04\xEC\x90b\0\x0FVV[`\x1F\x19\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@\x81\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x83\x01R\x80Q\x80\x83\x03\x82\x01\x81R``\x83\x01\x90\x91Rb\0\x05w\x92\x91`\x80\x01b\0\x16\xE6V[`@Q` \x81\x83\x03\x03\x81R\x90`@Rb\0\r\x06V[P[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x05\xF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x06\x0EW=`\0\x80>=`\0\xFD[PPPPb\0\x06`\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85b\0\x0E\x0EV[P`@Qc>(9\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c>(9\x1D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06\xEE\x91\x90b\0\x16\xC6V[b\0\x07\x02Wb\0\x07\0\x85\x85\x85b\0\n\rV[P[PPPPPV[b\0\x07\x13b\0\x0C\xAAV[b\0\x07\x1F`\0b\0\x0E\xA7V[V[`\0b\0\x07\xED\x82`\x01`\x01`\xA0\x1B\x03\x16`\0\x1B`@Q\x80` \x01b\0\x07F\x90b\0\x0FVV[`\x1F\x19\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@\x81\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x83\x01R\x80Q\x80\x83\x03\x82\x01\x81R``\x83\x01\x90\x91Rb\0\x07\xD1\x92\x91`\x80\x01b\0\x16\xE6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 b\0\x0E\xF7V[\x92\x91PPV[``b\0\x07\xFFb\0\x0C\xAAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15b\0\x08rWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15b\0\x08\xFDW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x08\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x08\xF8W=`\0\x80>=`\0\xFD[PPPP[b\0\t\x0B\x85\x85\x85\x85b\0\x0E\x0EV[\x95\x94PPPPPV[``b\0\t b\0\x0C\xAAV[`\0c\r\xD8\xDD\x02`\xE0\x1B\x84\x84`@Q`$\x01b\0\t?\x92\x91\x90b\0\x17\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qc\xF6\xE8\xF3\x9D`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\xF6\xE8\xF3\x9D\x90b\0\t\xC3\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90`\x04\x01b\0\x16)V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\t\xE3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\t\x0B\x91\x90\x81\x01\x90b\0\x16OV[``b\0\n\x19b\0\x0C\xAAV[`\0c\xEE\xA9\x06K`\xE0\x1B\x85\x85\x85`@Q`$\x01b\0\n:\x93\x92\x91\x90b\0\x17\xF0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91R\x90Pb\0\ny\x85b\0\x07!V[`\x01`\x01`\xA0\x1B\x03\x16c\xF6\xE8\xF3\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\t\xC3\x92\x91\x90b\0\x16)V[``b\0\n\xD4b\0\x0C\xAAV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R`D\x80\x83\x01\x86\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Qc\xF6\xE8\xF3\x9D`\xE0\x1B\x81R\x90\x91\x87\x16\x90c\xF6\xE8\xF3\x9D\x90b\0\x0BD\x90\x88\x90\x85\x90`\x04\x01b\0\x16)V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x0BdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x03\x90\x91\x90\x81\x01\x90b\0\x16OV[b\0\x0B\x98b\0\x0C\xAAV[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x0B\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x04\x8CV[b\0\x0C\n\x81b\0\x0E\xA7V[PV[``b\0\x0C\x19b\0\x0C\xAAV[`\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x164\x85`@Qb\0\x0C7\x91\x90b\0\x183V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14b\0\x0CvW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x0C{V[``\x91P[P\x91P\x91P\x81b\0\x0C\xA2W\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x04\x8C\x91\x90b\0\x10\xF6V[\x94\x93PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x07\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x04\x8CV[`\0\x80\x84G\x10\x15b\0\r[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FCreate2: insufficient balance\0\0\0`D\x82\x01R`d\x01b\0\x04\x8CV[\x82Qb\0\r\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCreate2: bytecode length is zero`D\x82\x01R`d\x01b\0\x04\x8CV[\x83\x83Q` \x85\x01\x87\xF5\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x0C\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FCreate2: Failed on deploy\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x04\x8CV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cs\xD0(U`\xE1\x1B\x17\x90R\x91Qc\xF6\xE8\xF3\x9D`\xE0\x1B\x81R``\x92\x87\x16\x90c\xF6\xE8\xF3\x9D\x90b\0\x0BD\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90`\x04\x01b\0\x16)V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`@\x80Q`\x01`\x01`\xF8\x1B\x03\x19` \x80\x83\x01\x91\x90\x91Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16`!\x83\x01R`5\x82\x01\x85\x90R`U\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`u\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90[\x93\x92PPPV[a\x05G\x80b\0\x18R\x839\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x0C\nW`\0\x80\xFD[\x805b\0\x0F\x87\x81b\0\x0FdV[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12b\0\x0F\x9FW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x0F\xB8W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15b\0\x0F\xD4W`\0\x80\xFD[\x92P\x92\x90PV[\x80\x15\x15\x81\x14b\0\x0C\nW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15b\0\x10\x04W`\0\x80\xFD[\x865b\0\x10\x11\x81b\0\x0FdV[\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\x10/W`\0\x80\xFD[\x90\x88\x01\x90`\xE0\x82\x8B\x03\x12\x15b\0\x10DW`\0\x80\xFD[\x90\x95P`@\x88\x015\x90\x80\x82\x11\x15b\0\x10[W`\0\x80\xFD[Pb\0\x10j\x89\x82\x8A\x01b\0\x0F\x8CV[\x90\x95P\x93PP``\x87\x015\x91P`\x80\x87\x015b\0\x10\x87\x81b\0\x0F\xDBV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0[\x83\x81\x10\x15b\0\x10\xB2W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x10\x98V[\x83\x81\x11\x15b\0\x10\xC2W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Rb\0\x10\xE2\x81` \x86\x01` \x86\x01b\0\x10\x95V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0\x0FO` \x83\x01\x84b\0\x10\xC8V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0\x11MWb\0\x11Mb\0\x11\x0BV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x11rWb\0\x11rb\0\x11\x0BV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12b\0\x11\x92W`\0\x80\xFD[\x815b\0\x11\xA9b\0\x11\xA3\x82b\0\x11UV[b\0\x11!V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15b\0\x11\xBFW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15b\0\x11\xEFW`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15b\0\x12\x16Wb\0\x12\x16b\0\x11\x0BV[\x81`@R\x82\x93P\x845\x91P\x80\x82\x11\x15b\0\x12/W`\0\x80\xFD[Pb\0\x12>\x85\x82\x86\x01b\0\x11\x80V[\x82RP` \x83\x015` \x82\x01RP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x12iW`\0\x80\xFD[\x845b\0\x12v\x81b\0\x0FdV[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x12\x93W`\0\x80\xFD[b\0\x12\xA1\x87\x82\x88\x01b\0\x11\xDCV[\x94\x97\x94\x96PPPP`@\x83\x015\x92``\x015\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\0\x12\xCBW`\0\x80\xFD[\x815b\0\x0FO\x81b\0\x0FdV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x12\xEFW`\0\x80\xFD[\x845b\0\x12\xFC\x81b\0\x0FdV[\x93P` \x85\x015b\0\x13\x0E\x81b\0\x0FdV[\x92P`@\x85\x015b\0\x13 \x81b\0\x0FdV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0`@\x84\x86\x03\x12\x15b\0\x13FW`\0\x80\xFD[\x835b\0\x13S\x81b\0\x0FdV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x13pW`\0\x80\xFD[b\0\x13~\x86\x82\x87\x01b\0\x0F\x8CV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x13\xA1W`\0\x80\xFD[\x835b\0\x13\xAE\x81b\0\x0FdV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x13\xCBW`\0\x80\xFD[b\0\x13\xD9\x86\x82\x87\x01b\0\x11\xDCV[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15b\0\x13\xFEW`\0\x80\xFD[\x825b\0\x14\x0B\x81b\0\x0FdV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x14(W`\0\x80\xFD[b\0\x146\x85\x82\x86\x01b\0\x11\x80V[\x91PP\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x0F\x87W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12b\0\x14mW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x14\x8EW`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15b\0\x0F\xD4W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15b\0\x14\xE3W\x815b\0\x14\xC7\x81b\0\x0FdV[`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01b\0\x14\xB1V[P\x94\x95\x94PPPPPV[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15b\0\x15\x08W`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[`\x80\x81R`\0\x865b\0\x158\x81b\0\x0FdV[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x83\x01R` \x87\x015b\0\x15V\x81b\0\x0FdV[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x83\x01Rb\0\x15r`@\x88\x01b\0\x0FzV[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x83\x01R``\x87\x015`\xE0\x83\x01Rb\0\x15\x98`\x80\x88\x01b\0\x14@V[c\xFF\xFF\xFF\xFF\x16a\x01\0\x83\x01Rb\0\x15\xB3`\xA0\x88\x01\x88b\0\x14UV[`\xE0a\x01 \x85\x01Rb\0\x15\xCCa\x01`\x85\x01\x82\x84b\0\x14\xA1V[\x91PPb\0\x15\xDE`\xC0\x89\x01\x89b\0\x14UV[\x84\x83\x03`\x7F\x19\x01a\x01@\x86\x01Rb\0\x15\xF8\x83\x82\x84b\0\x14\xEEV[\x92PPP\x82\x81\x03` \x84\x01Rb\0\x16\x11\x81\x87\x89b\0\x14\xA1V[\x91PP\x83`@\x83\x01Rb\0\x03\x90``\x83\x01\x84\x15\x15\x90RV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90b\0\x0C\xA2\x90\x83\x01\x84b\0\x10\xC8V[`\0` \x82\x84\x03\x12\x15b\0\x16bW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x16zW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13b\0\x16\x8CW`\0\x80\xFD[\x80Qb\0\x16\x9Db\0\x11\xA3\x82b\0\x11UV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15b\0\x16\xB3W`\0\x80\xFD[b\0\t\x0B\x82` \x83\x01` \x86\x01b\0\x10\x95V[`\0` \x82\x84\x03\x12\x15b\0\x16\xD9W`\0\x80\xFD[\x81Qb\0\x0FO\x81b\0\x0F\xDBV[`\0\x83Qb\0\x16\xFA\x81\x84` \x88\x01b\0\x10\x95V[\x83Q\x90\x83\x01\x90b\0\x17\x10\x81\x83` \x88\x01b\0\x10\x95V[\x01\x94\x93PPPPV[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@\x80\x84\x01`\x05\x86\x90\x1B\x85\x01\x82\x01\x87\x85\x80[\x89\x81\x10\x15b\0\x17\xE1W\x88\x84\x03`?\x19\x01\x85R\x8256\x8C\x90\x03`^\x19\x01\x81\x12b\0\x17`W\x82\x83\xFD[\x8B\x01``b\0\x17p\x82\x80b\0\x14UV[\x82\x88Rb\0\x17\x82\x83\x89\x01\x82\x84b\0\x14\xA1V[\x92PPPb\0\x17\x94\x89\x83\x01\x83b\0\x14UV[\x87\x83\x03\x8B\x89\x01Rb\0\x17\xA8\x83\x82\x84b\0\x14\xEEV[\x92PPP\x87\x82\x015\x91Pb\0\x17\xBD\x82b\0\x0FdV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x94\x87\x01\x94\x90\x94R\x93\x86\x01\x93\x91\x86\x01\x91`\x01\x01b\0\x179V[P\x91\x99\x98PPPPPPPPPV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01R`\0\x83Q`@``\x84\x01Rb\0\x18\x1C`\xA0\x84\x01\x82b\0\x10\xC8V[` \x95\x90\x95\x01Q`\x80\x84\x01RPP`@\x01R\x91\x90PV[`\0\x82Qb\0\x18G\x81\x84` \x87\x01b\0\x10\x95V[\x91\x90\x91\x01\x92\x91PPV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x05G8\x03\x80a\x05G\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x1CV[a\083a\0\xB4V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x19`$\x83\x01R\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xAC\x91\x90a\x01VV[PPPa\x01\x7FV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x19W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x01/W`\0\x80\xFD[\x82Qa\x01:\x81a\x01\x04V[` \x84\x01Q\x90\x92Pa\x01K\x81a\x01\x04V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x01hW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x01xW`\0\x80\xFD[\x93\x92PPPV[a\x03\xB9\x80a\x01\x8E`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80cqP\x18\xA6\x14a\0QW\x80c\x8D\xA5\xCB[\x14a\0[W\x80c\xF2\xFD\xE3\x8B\x14a\0{W\x80c\xF6\xE8\xF3\x9D\x14a\0\x8EW[`\0\x80\xFD[a\0Ya\0\xAEV[\0[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0Ya\0\x896`\x04a\x024V[a\0\xC2V[a\0\xA1a\0\x9C6`\x04a\x02lV[a\x01@V[`@Qa\0r\x91\x90a\x03.V[a\0\xB6a\x01nV[a\0\xC0`\0a\x01\xC8V[V[a\0\xCAa\x01nV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x014W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x01=\x81a\x01\xC8V[PV[``a\x01Ja\x01nV[\x81Q```\0\x80\x83` \x87\x014\x89Z\xF1P=\x81R=`\0` \x83\x01>\x94\x93PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01+V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02/W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02FW`\0\x80\xFD[a\x02O\x82a\x02\x18V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x02\x7FW`\0\x80\xFD[a\x02\x88\x83a\x02\x18V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xA5W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02\xB9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\xCBWa\x02\xCBa\x02VV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02\xF3Wa\x02\xF3a\x02VV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x03\x0CW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x03[W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x03?V[\x81\x81\x11\x15a\x03mW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xB8.\x1C\x18\x18\x05\xB1\xBE\xFA.\xD8\x14\xC8\xD4\x9D\x18\xEB\xF7\xA6np\xBD\x16\xF6W\xFB\x83T\x11V1odsolcC\0\x08\x0C\x003\xA2dipfsX\"\x12 \xB8\xF8\x1E5?\x9E\xD1\xFC\xE1Gy\x11\x8A\xEE\xBEK \xE2d9D\xCF\xCE(\x89\xDC\x96\xB3\x89w4cdsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610620000b55760003560e01c8063a49ca158116200006c578063a49ca15814620001bd578063a76a9d2d14620001e2578063eea9064b1462000207578063f18d03cc146200022c578063f2fde38b1462000251578063f6e8f39d146200027657600080fd5b8063063dcd5014620000ba5780633448950614620000f75780636850e5c1146200011e578063715018a6146200014c5780638da5cb5b1462000164578063a23c44b11462000198575b600080fd5b348015620000c757600080fd5b50620000df620000d936600462000fea565b6200028d565b604051620000ee9190620010f6565b60405180910390f35b3480156200010457600080fd5b506200011c6200011636600462001252565b6200039a565b005b3480156200012b57600080fd5b506200013d68056bc75e2d6310000081565b604051908152602001620000ee565b3480156200015957600080fd5b506200011c62000709565b3480156200017157600080fd5b506000546001600160a01b03165b6040516001600160a01b039091168152602001620000ee565b348015620001a557600080fd5b506200017f620001b7366004620012b8565b62000721565b348015620001ca57600080fd5b50620000df620001dc366004620012d8565b620007f3565b348015620001ef57600080fd5b50620000df6200020136600462001330565b62000914565b3480156200021457600080fd5b50620000df620002263660046200138b565b62000a0d565b3480156200023957600080fd5b50620000df6200024b366004620012d8565b62000ac8565b3480156200025e57600080fd5b506200011c62000270366004620012b8565b62000b8e565b620000df62000287366004620013ea565b62000c0d565b60606200029962000caa565b60006360d7faed60e01b8787878787604051602401620002be95949392919062001525565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905163f6e8f39d60e01b81529091506001600160a01b0389169063f6e8f39d9062000342907f000000000000000000000000000000000000000000000000000000000000000090859060040162001629565b6000604051808303816000875af115801562000362573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526200038c91908101906200164f565b9150505b9695505050505050565b620003a462000caa565b6040516336b87bd760e11b81526001600160a01b0385811660048301527f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90602401602060405180830381865afa1580156200040b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620004319190620016c6565b620004955760405162461bcd60e51b815260206004820152602960248201527f44656c65676174696f6e4661756365743a204f70657261746f72206e6f7420726044820152681959da5cdd195c995960ba1b60648201526084015b60405180910390fd5b6000620004a28562000721565b905081620004b75768056bc75e2d6310000091505b6001600160a01b0381163b6200058e576200058c6000866001600160a01b031660001b60405180602001620004ec9062000f56565b601f1982820381018352601f9091011660408181527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0390811660208401527f0000000000000000000000000000000000000000000000000000000000000000168183015280518083038201815260608301909152620005779291608001620016e6565b60405160208183030381529060405262000d06565b505b6040516340c10f1960e01b81526001600160a01b038281166004830152602482018490527f000000000000000000000000000000000000000000000000000000000000000016906340c10f1990604401600060405180830381600087803b158015620005f957600080fd5b505af11580156200060e573d6000803e3d6000fd5b5050505062000660817f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000008562000e0e565b50604051633e28391d60e01b81526001600160a01b0382811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633e28391d90602401602060405180830381865afa158015620006c8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620006ee9190620016c6565b62000702576200070085858562000a0d565b505b5050505050565b6200071362000caa565b6200071f600062000ea7565b565b6000620007ed826001600160a01b031660001b60405180602001620007469062000f56565b601f1982820381018352601f9091011660408181527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0390811660208401527f0000000000000000000000000000000000000000000000000000000000000000168183015280518083038201815260608301909152620007d19291608001620016e6565b6040516020818303038152906040528051906020012062000ef7565b92915050565b6060620007ff62000caa565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316836001600160a01b03161480156200087257507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316846001600160a01b0316145b15620008fd576040516340c10f1960e01b81526001600160a01b038681166004830152602482018490527f000000000000000000000000000000000000000000000000000000000000000016906340c10f1990604401600060405180830381600087803b158015620008e357600080fd5b505af1158015620008f8573d6000803e3d6000fd5b505050505b6200090b8585858562000e0e565b95945050505050565b60606200092062000caa565b6000630dd8dd0260e01b84846040516024016200093f92919062001719565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905163f6e8f39d60e01b81529091506001600160a01b0386169063f6e8f39d90620009c3907f000000000000000000000000000000000000000000000000000000000000000090859060040162001629565b6000604051808303816000875af1158015620009e3573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526200090b91908101906200164f565b606062000a1962000caa565b600063eea9064b60e01b85858560405160240162000a3a93929190620017f0565b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152905062000a798562000721565b6001600160a01b031663f6e8f39d7f0000000000000000000000000000000000000000000000000000000000000000836040518363ffffffff1660e01b8152600401620009c392919062001629565b606062000ad462000caa565b604080516001600160a01b038581166024830152604480830186905283518084039091018152606490920183526020820180516001600160e01b031663a9059cbb60e01b179052915163f6e8f39d60e01b8152909187169063f6e8f39d9062000b44908890859060040162001629565b6000604051808303816000875af115801562000b64573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526200039091908101906200164f565b62000b9862000caa565b6001600160a01b03811662000bff5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016200048c565b62000c0a8162000ea7565b50565b606062000c1962000caa565b600080846001600160a01b0316348560405162000c37919062001833565b60006040518083038185875af1925050503d806000811462000c76576040519150601f19603f3d011682016040523d82523d6000602084013e62000c7b565b606091505b50915091508162000ca2578060405162461bcd60e51b81526004016200048c9190620010f6565b949350505050565b6000546001600160a01b031633146200071f5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016200048c565b6000808447101562000d5b5760405162461bcd60e51b815260206004820152601d60248201527f437265617465323a20696e73756666696369656e742062616c616e636500000060448201526064016200048c565b825162000dab5760405162461bcd60e51b815260206004820181905260248201527f437265617465323a2062797465636f6465206c656e677468206973207a65726f60448201526064016200048c565b8383516020850187f590506001600160a01b03811662000ca25760405162461bcd60e51b815260206004820152601960248201527f437265617465323a204661696c6564206f6e206465706c6f790000000000000060448201526064016200048c565b604080516001600160a01b0385811660248301528481166044830152606480830185905283518084039091018152608490920183526020820180516001600160e01b03166373d0285560e11b179052915163f6e8f39d60e01b815260609287169063f6e8f39d9062000b44907f000000000000000000000000000000000000000000000000000000000000000090859060040162001629565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b604080516001600160f81b03196020808301919091526bffffffffffffffffffffffff193060601b16602183015260358201859052605580830185905283518084039091018152607590920190925280519101206000905b9392505050565b610547806200185283390190565b6001600160a01b038116811462000c0a57600080fd5b803562000f878162000f64565b919050565b60008083601f84011262000f9f57600080fd5b50813567ffffffffffffffff81111562000fb857600080fd5b6020830191508360208260051b850101111562000fd457600080fd5b9250929050565b801515811462000c0a57600080fd5b60008060008060008060a087890312156200100457600080fd5b8635620010118162000f64565b9550602087013567ffffffffffffffff808211156200102f57600080fd5b9088019060e0828b0312156200104457600080fd5b909550604088013590808211156200105b57600080fd5b506200106a89828a0162000f8c565b909550935050606087013591506080870135620010878162000fdb565b809150509295509295509295565b60005b83811015620010b257818101518382015260200162001098565b83811115620010c2576000848401525b50505050565b60008151808452620010e281602086016020860162001095565b601f01601f19169290920160200192915050565b60208152600062000f4f6020830184620010c8565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff811182821017156200114d576200114d6200110b565b604052919050565b600067ffffffffffffffff8211156200117257620011726200110b565b50601f01601f191660200190565b600082601f8301126200119257600080fd5b8135620011a9620011a38262001155565b62001121565b818152846020838601011115620011bf57600080fd5b816020850160208301376000918101602001919091529392505050565b600060408284031215620011ef57600080fd5b6040516040810167ffffffffffffffff82821081831117156200121657620012166200110b565b8160405282935084359150808211156200122f57600080fd5b506200123e8582860162001180565b825250602083013560208201525092915050565b600080600080608085870312156200126957600080fd5b8435620012768162000f64565b9350602085013567ffffffffffffffff8111156200129357600080fd5b620012a187828801620011dc565b949794965050505060408301359260600135919050565b600060208284031215620012cb57600080fd5b813562000f4f8162000f64565b60008060008060808587031215620012ef57600080fd5b8435620012fc8162000f64565b935060208501356200130e8162000f64565b92506040850135620013208162000f64565b9396929550929360600135925050565b6000806000604084860312156200134657600080fd5b8335620013538162000f64565b9250602084013567ffffffffffffffff8111156200137057600080fd5b6200137e8682870162000f8c565b9497909650939450505050565b600080600060608486031215620013a157600080fd5b8335620013ae8162000f64565b9250602084013567ffffffffffffffff811115620013cb57600080fd5b620013d986828701620011dc565b925050604084013590509250925092565b60008060408385031215620013fe57600080fd5b82356200140b8162000f64565b9150602083013567ffffffffffffffff8111156200142857600080fd5b620014368582860162001180565b9150509250929050565b803563ffffffff8116811462000f8757600080fd5b6000808335601e198436030181126200146d57600080fd5b830160208101925035905067ffffffffffffffff8111156200148e57600080fd5b8060051b360383131562000fd457600080fd5b8183526000602080850194508260005b85811015620014e3578135620014c78162000f64565b6001600160a01b031687529582019590820190600101620014b1565b509495945050505050565b81835260006001600160fb1b038311156200150857600080fd5b8260051b8083602087013760009401602001938452509192915050565b6080815260008635620015388162000f64565b6001600160a01b031660808301526020870135620015568162000f64565b6001600160a01b031660a0830152620015726040880162000f7a565b6001600160a01b031660c0830152606087013560e0830152620015986080880162001440565b63ffffffff16610100830152620015b360a088018862001455565b60e0610120850152620015cc61016085018284620014a1565b915050620015de60c089018962001455565b848303607f1901610140860152620015f8838284620014ee565b92505050828103602084015262001611818789620014a1565b91505083604083015262000390606083018415159052565b6001600160a01b038316815260406020820181905260009062000ca290830184620010c8565b6000602082840312156200166257600080fd5b815167ffffffffffffffff8111156200167a57600080fd5b8201601f810184136200168c57600080fd5b80516200169d620011a38262001155565b818152856020838501011115620016b357600080fd5b6200090b82602083016020860162001095565b600060208284031215620016d957600080fd5b815162000f4f8162000fdb565b60008351620016fa81846020880162001095565b8351908301906200171081836020880162001095565b01949350505050565b60208082528181018390526000906040808401600586901b850182018785805b89811015620017e157888403603f190185528235368c9003605e1901811262001760578283fd5b8b01606062001770828062001455565b828852620017828389018284620014a1565b92505050620017948983018362001455565b8783038b890152620017a8838284620014ee565b92505050878201359150620017bd8262000f64565b6001600160a01b039190911694870194909452938601939186019160010162001739565b50919998505050505050505050565b60018060a01b03841681526060602082015260008351604060608401526200181c60a0840182620010c8565b602095909501516080840152505060400152919050565b600082516200184781846020870162001095565b919091019291505056fe608060405234801561001057600080fd5b5060405161054738038061054783398101604081905261002f9161011c565b610038336100b4565b60405163095ea7b360e01b81526001600160a01b038381166004830152600019602483015282169063095ea7b3906044016020604051808303816000875af1158015610088573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100ac9190610156565b50505061017f565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6001600160a01b038116811461011957600080fd5b50565b6000806040838503121561012f57600080fd5b825161013a81610104565b602084015190925061014b81610104565b809150509250929050565b60006020828403121561016857600080fd5b8151801515811461017857600080fd5b9392505050565b6103b98061018e6000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c8063715018a6146100515780638da5cb5b1461005b578063f2fde38b1461007b578063f6e8f39d1461008e575b600080fd5b6100596100ae565b005b6000546040516001600160a01b0390911681526020015b60405180910390f35b610059610089366004610234565b6100c2565b6100a161009c36600461026c565b610140565b604051610072919061032e565b6100b661016e565b6100c060006101c8565b565b6100ca61016e565b6001600160a01b0381166101345760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084015b60405180910390fd5b61013d816101c8565b50565b606061014a61016e565b81516060600080836020870134895af1503d81523d6000602083013e949350505050565b6000546001600160a01b031633146100c05760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161012b565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b80356001600160a01b038116811461022f57600080fd5b919050565b60006020828403121561024657600080fd5b61024f82610218565b9392505050565b634e487b7160e01b600052604160045260246000fd5b6000806040838503121561027f57600080fd5b61028883610218565b9150602083013567ffffffffffffffff808211156102a557600080fd5b818501915085601f8301126102b957600080fd5b8135818111156102cb576102cb610256565b604051601f8201601f19908116603f011681019083821181831017156102f3576102f3610256565b8160405282815288602084870101111561030c57600080fd5b8260208601602083013760006020848301015280955050505050509250929050565b600060208083528351808285015260005b8181101561035b5785810183015185820160400152820161033f565b8181111561036d576000604083870101525b50601f01601f191692909201604001939250505056fea2646970667358221220b82e1c181805b1befa2ed814c8d49d18ebf7a66e70bd16f657fb83541156316f64736f6c634300080c0033a2646970667358221220b8f81e353f9ed1fce14779118aeebe4b20e2643944cfce2889dc96b38977346364736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10b\0\0\xB5W`\x005`\xE0\x1C\x80c\xA4\x9C\xA1X\x11b\0\0lW\x80c\xA4\x9C\xA1X\x14b\0\x01\xBDW\x80c\xA7j\x9D-\x14b\0\x01\xE2W\x80c\xEE\xA9\x06K\x14b\0\x02\x07W\x80c\xF1\x8D\x03\xCC\x14b\0\x02,W\x80c\xF2\xFD\xE3\x8B\x14b\0\x02QW\x80c\xF6\xE8\xF3\x9D\x14b\0\x02vW`\0\x80\xFD[\x80c\x06=\xCDP\x14b\0\0\xBAW\x80c4H\x95\x06\x14b\0\0\xF7W\x80chP\xE5\xC1\x14b\0\x01\x1EW\x80cqP\x18\xA6\x14b\0\x01LW\x80c\x8D\xA5\xCB[\x14b\0\x01dW\x80c\xA2<D\xB1\x14b\0\x01\x98W[`\0\x80\xFD[4\x80\x15b\0\0\xC7W`\0\x80\xFD[Pb\0\0\xDFb\0\0\xD96`\x04b\0\x0F\xEAV[b\0\x02\x8DV[`@Qb\0\0\xEE\x91\x90b\0\x10\xF6V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15b\0\x01\x04W`\0\x80\xFD[Pb\0\x01\x1Cb\0\x01\x166`\x04b\0\x12RV[b\0\x03\x9AV[\0[4\x80\x15b\0\x01+W`\0\x80\xFD[Pb\0\x01=h\x05k\xC7^-c\x10\0\0\x81V[`@Q\x90\x81R` \x01b\0\0\xEEV[4\x80\x15b\0\x01YW`\0\x80\xFD[Pb\0\x01\x1Cb\0\x07\tV[4\x80\x15b\0\x01qW`\0\x80\xFD[P`\0T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01b\0\0\xEEV[4\x80\x15b\0\x01\xA5W`\0\x80\xFD[Pb\0\x01\x7Fb\0\x01\xB76`\x04b\0\x12\xB8V[b\0\x07!V[4\x80\x15b\0\x01\xCAW`\0\x80\xFD[Pb\0\0\xDFb\0\x01\xDC6`\x04b\0\x12\xD8V[b\0\x07\xF3V[4\x80\x15b\0\x01\xEFW`\0\x80\xFD[Pb\0\0\xDFb\0\x02\x016`\x04b\0\x130V[b\0\t\x14V[4\x80\x15b\0\x02\x14W`\0\x80\xFD[Pb\0\0\xDFb\0\x02&6`\x04b\0\x13\x8BV[b\0\n\rV[4\x80\x15b\0\x029W`\0\x80\xFD[Pb\0\0\xDFb\0\x02K6`\x04b\0\x12\xD8V[b\0\n\xC8V[4\x80\x15b\0\x02^W`\0\x80\xFD[Pb\0\x01\x1Cb\0\x02p6`\x04b\0\x12\xB8V[b\0\x0B\x8EV[b\0\0\xDFb\0\x02\x876`\x04b\0\x13\xEAV[b\0\x0C\rV[``b\0\x02\x99b\0\x0C\xAAV[`\0c`\xD7\xFA\xED`\xE0\x1B\x87\x87\x87\x87\x87`@Q`$\x01b\0\x02\xBE\x95\x94\x93\x92\x91\x90b\0\x15%V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qc\xF6\xE8\xF3\x9D`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\xF6\xE8\xF3\x9D\x90b\0\x03B\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90`\x04\x01b\0\x16)V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x03bW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x03\x8C\x91\x90\x81\x01\x90b\0\x16OV[\x91PP[\x96\x95PPPPPPV[b\0\x03\xA4b\0\x0C\xAAV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x04\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x041\x91\x90b\0\x16\xC6V[b\0\x04\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FDelegationFaucet: Operator not r`D\x82\x01Rh\x19Y\xDA\\\xDD\x19\\\x99Y`\xBA\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0b\0\x04\xA2\x85b\0\x07!V[\x90P\x81b\0\x04\xB7Wh\x05k\xC7^-c\x10\0\0\x91P[`\x01`\x01`\xA0\x1B\x03\x81\x16;b\0\x05\x8EWb\0\x05\x8C`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\0\x1B`@Q\x80` \x01b\0\x04\xEC\x90b\0\x0FVV[`\x1F\x19\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@\x81\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x83\x01R\x80Q\x80\x83\x03\x82\x01\x81R``\x83\x01\x90\x91Rb\0\x05w\x92\x91`\x80\x01b\0\x16\xE6V[`@Q` \x81\x83\x03\x03\x81R\x90`@Rb\0\r\x06V[P[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x05\xF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x06\x0EW=`\0\x80>=`\0\xFD[PPPPb\0\x06`\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85b\0\x0E\x0EV[P`@Qc>(9\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c>(9\x1D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06\xEE\x91\x90b\0\x16\xC6V[b\0\x07\x02Wb\0\x07\0\x85\x85\x85b\0\n\rV[P[PPPPPV[b\0\x07\x13b\0\x0C\xAAV[b\0\x07\x1F`\0b\0\x0E\xA7V[V[`\0b\0\x07\xED\x82`\x01`\x01`\xA0\x1B\x03\x16`\0\x1B`@Q\x80` \x01b\0\x07F\x90b\0\x0FVV[`\x1F\x19\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@\x81\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x83\x01R\x80Q\x80\x83\x03\x82\x01\x81R``\x83\x01\x90\x91Rb\0\x07\xD1\x92\x91`\x80\x01b\0\x16\xE6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 b\0\x0E\xF7V[\x92\x91PPV[``b\0\x07\xFFb\0\x0C\xAAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15b\0\x08rWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15b\0\x08\xFDW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x08\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x08\xF8W=`\0\x80>=`\0\xFD[PPPP[b\0\t\x0B\x85\x85\x85\x85b\0\x0E\x0EV[\x95\x94PPPPPV[``b\0\t b\0\x0C\xAAV[`\0c\r\xD8\xDD\x02`\xE0\x1B\x84\x84`@Q`$\x01b\0\t?\x92\x91\x90b\0\x17\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qc\xF6\xE8\xF3\x9D`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\xF6\xE8\xF3\x9D\x90b\0\t\xC3\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90`\x04\x01b\0\x16)V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\t\xE3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\t\x0B\x91\x90\x81\x01\x90b\0\x16OV[``b\0\n\x19b\0\x0C\xAAV[`\0c\xEE\xA9\x06K`\xE0\x1B\x85\x85\x85`@Q`$\x01b\0\n:\x93\x92\x91\x90b\0\x17\xF0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91R\x90Pb\0\ny\x85b\0\x07!V[`\x01`\x01`\xA0\x1B\x03\x16c\xF6\xE8\xF3\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\t\xC3\x92\x91\x90b\0\x16)V[``b\0\n\xD4b\0\x0C\xAAV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R`D\x80\x83\x01\x86\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Qc\xF6\xE8\xF3\x9D`\xE0\x1B\x81R\x90\x91\x87\x16\x90c\xF6\xE8\xF3\x9D\x90b\0\x0BD\x90\x88\x90\x85\x90`\x04\x01b\0\x16)V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x0BdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x03\x90\x91\x90\x81\x01\x90b\0\x16OV[b\0\x0B\x98b\0\x0C\xAAV[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x0B\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x04\x8CV[b\0\x0C\n\x81b\0\x0E\xA7V[PV[``b\0\x0C\x19b\0\x0C\xAAV[`\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x164\x85`@Qb\0\x0C7\x91\x90b\0\x183V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14b\0\x0CvW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x0C{V[``\x91P[P\x91P\x91P\x81b\0\x0C\xA2W\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x04\x8C\x91\x90b\0\x10\xF6V[\x94\x93PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x07\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x04\x8CV[`\0\x80\x84G\x10\x15b\0\r[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FCreate2: insufficient balance\0\0\0`D\x82\x01R`d\x01b\0\x04\x8CV[\x82Qb\0\r\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCreate2: bytecode length is zero`D\x82\x01R`d\x01b\0\x04\x8CV[\x83\x83Q` \x85\x01\x87\xF5\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x0C\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FCreate2: Failed on deploy\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x04\x8CV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cs\xD0(U`\xE1\x1B\x17\x90R\x91Qc\xF6\xE8\xF3\x9D`\xE0\x1B\x81R``\x92\x87\x16\x90c\xF6\xE8\xF3\x9D\x90b\0\x0BD\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90`\x04\x01b\0\x16)V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`@\x80Q`\x01`\x01`\xF8\x1B\x03\x19` \x80\x83\x01\x91\x90\x91Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16`!\x83\x01R`5\x82\x01\x85\x90R`U\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`u\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90[\x93\x92PPPV[a\x05G\x80b\0\x18R\x839\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x0C\nW`\0\x80\xFD[\x805b\0\x0F\x87\x81b\0\x0FdV[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12b\0\x0F\x9FW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x0F\xB8W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15b\0\x0F\xD4W`\0\x80\xFD[\x92P\x92\x90PV[\x80\x15\x15\x81\x14b\0\x0C\nW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15b\0\x10\x04W`\0\x80\xFD[\x865b\0\x10\x11\x81b\0\x0FdV[\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\x10/W`\0\x80\xFD[\x90\x88\x01\x90`\xE0\x82\x8B\x03\x12\x15b\0\x10DW`\0\x80\xFD[\x90\x95P`@\x88\x015\x90\x80\x82\x11\x15b\0\x10[W`\0\x80\xFD[Pb\0\x10j\x89\x82\x8A\x01b\0\x0F\x8CV[\x90\x95P\x93PP``\x87\x015\x91P`\x80\x87\x015b\0\x10\x87\x81b\0\x0F\xDBV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0[\x83\x81\x10\x15b\0\x10\xB2W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x10\x98V[\x83\x81\x11\x15b\0\x10\xC2W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Rb\0\x10\xE2\x81` \x86\x01` \x86\x01b\0\x10\x95V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0\x0FO` \x83\x01\x84b\0\x10\xC8V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0\x11MWb\0\x11Mb\0\x11\x0BV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x11rWb\0\x11rb\0\x11\x0BV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12b\0\x11\x92W`\0\x80\xFD[\x815b\0\x11\xA9b\0\x11\xA3\x82b\0\x11UV[b\0\x11!V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15b\0\x11\xBFW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15b\0\x11\xEFW`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15b\0\x12\x16Wb\0\x12\x16b\0\x11\x0BV[\x81`@R\x82\x93P\x845\x91P\x80\x82\x11\x15b\0\x12/W`\0\x80\xFD[Pb\0\x12>\x85\x82\x86\x01b\0\x11\x80V[\x82RP` \x83\x015` \x82\x01RP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x12iW`\0\x80\xFD[\x845b\0\x12v\x81b\0\x0FdV[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x12\x93W`\0\x80\xFD[b\0\x12\xA1\x87\x82\x88\x01b\0\x11\xDCV[\x94\x97\x94\x96PPPP`@\x83\x015\x92``\x015\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\0\x12\xCBW`\0\x80\xFD[\x815b\0\x0FO\x81b\0\x0FdV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x12\xEFW`\0\x80\xFD[\x845b\0\x12\xFC\x81b\0\x0FdV[\x93P` \x85\x015b\0\x13\x0E\x81b\0\x0FdV[\x92P`@\x85\x015b\0\x13 \x81b\0\x0FdV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0`@\x84\x86\x03\x12\x15b\0\x13FW`\0\x80\xFD[\x835b\0\x13S\x81b\0\x0FdV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x13pW`\0\x80\xFD[b\0\x13~\x86\x82\x87\x01b\0\x0F\x8CV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x13\xA1W`\0\x80\xFD[\x835b\0\x13\xAE\x81b\0\x0FdV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x13\xCBW`\0\x80\xFD[b\0\x13\xD9\x86\x82\x87\x01b\0\x11\xDCV[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15b\0\x13\xFEW`\0\x80\xFD[\x825b\0\x14\x0B\x81b\0\x0FdV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x14(W`\0\x80\xFD[b\0\x146\x85\x82\x86\x01b\0\x11\x80V[\x91PP\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x0F\x87W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12b\0\x14mW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x14\x8EW`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15b\0\x0F\xD4W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15b\0\x14\xE3W\x815b\0\x14\xC7\x81b\0\x0FdV[`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01b\0\x14\xB1V[P\x94\x95\x94PPPPPV[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15b\0\x15\x08W`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[`\x80\x81R`\0\x865b\0\x158\x81b\0\x0FdV[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x83\x01R` \x87\x015b\0\x15V\x81b\0\x0FdV[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x83\x01Rb\0\x15r`@\x88\x01b\0\x0FzV[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x83\x01R``\x87\x015`\xE0\x83\x01Rb\0\x15\x98`\x80\x88\x01b\0\x14@V[c\xFF\xFF\xFF\xFF\x16a\x01\0\x83\x01Rb\0\x15\xB3`\xA0\x88\x01\x88b\0\x14UV[`\xE0a\x01 \x85\x01Rb\0\x15\xCCa\x01`\x85\x01\x82\x84b\0\x14\xA1V[\x91PPb\0\x15\xDE`\xC0\x89\x01\x89b\0\x14UV[\x84\x83\x03`\x7F\x19\x01a\x01@\x86\x01Rb\0\x15\xF8\x83\x82\x84b\0\x14\xEEV[\x92PPP\x82\x81\x03` \x84\x01Rb\0\x16\x11\x81\x87\x89b\0\x14\xA1V[\x91PP\x83`@\x83\x01Rb\0\x03\x90``\x83\x01\x84\x15\x15\x90RV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90b\0\x0C\xA2\x90\x83\x01\x84b\0\x10\xC8V[`\0` \x82\x84\x03\x12\x15b\0\x16bW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x16zW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13b\0\x16\x8CW`\0\x80\xFD[\x80Qb\0\x16\x9Db\0\x11\xA3\x82b\0\x11UV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15b\0\x16\xB3W`\0\x80\xFD[b\0\t\x0B\x82` \x83\x01` \x86\x01b\0\x10\x95V[`\0` \x82\x84\x03\x12\x15b\0\x16\xD9W`\0\x80\xFD[\x81Qb\0\x0FO\x81b\0\x0F\xDBV[`\0\x83Qb\0\x16\xFA\x81\x84` \x88\x01b\0\x10\x95V[\x83Q\x90\x83\x01\x90b\0\x17\x10\x81\x83` \x88\x01b\0\x10\x95V[\x01\x94\x93PPPPV[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@\x80\x84\x01`\x05\x86\x90\x1B\x85\x01\x82\x01\x87\x85\x80[\x89\x81\x10\x15b\0\x17\xE1W\x88\x84\x03`?\x19\x01\x85R\x8256\x8C\x90\x03`^\x19\x01\x81\x12b\0\x17`W\x82\x83\xFD[\x8B\x01``b\0\x17p\x82\x80b\0\x14UV[\x82\x88Rb\0\x17\x82\x83\x89\x01\x82\x84b\0\x14\xA1V[\x92PPPb\0\x17\x94\x89\x83\x01\x83b\0\x14UV[\x87\x83\x03\x8B\x89\x01Rb\0\x17\xA8\x83\x82\x84b\0\x14\xEEV[\x92PPP\x87\x82\x015\x91Pb\0\x17\xBD\x82b\0\x0FdV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x94\x87\x01\x94\x90\x94R\x93\x86\x01\x93\x91\x86\x01\x91`\x01\x01b\0\x179V[P\x91\x99\x98PPPPPPPPPV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01R`\0\x83Q`@``\x84\x01Rb\0\x18\x1C`\xA0\x84\x01\x82b\0\x10\xC8V[` \x95\x90\x95\x01Q`\x80\x84\x01RPP`@\x01R\x91\x90PV[`\0\x82Qb\0\x18G\x81\x84` \x87\x01b\0\x10\x95V[\x91\x90\x91\x01\x92\x91PPV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x05G8\x03\x80a\x05G\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x1CV[a\083a\0\xB4V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x19`$\x83\x01R\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xAC\x91\x90a\x01VV[PPPa\x01\x7FV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x19W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x01/W`\0\x80\xFD[\x82Qa\x01:\x81a\x01\x04V[` \x84\x01Q\x90\x92Pa\x01K\x81a\x01\x04V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x01hW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x01xW`\0\x80\xFD[\x93\x92PPPV[a\x03\xB9\x80a\x01\x8E`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80cqP\x18\xA6\x14a\0QW\x80c\x8D\xA5\xCB[\x14a\0[W\x80c\xF2\xFD\xE3\x8B\x14a\0{W\x80c\xF6\xE8\xF3\x9D\x14a\0\x8EW[`\0\x80\xFD[a\0Ya\0\xAEV[\0[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0Ya\0\x896`\x04a\x024V[a\0\xC2V[a\0\xA1a\0\x9C6`\x04a\x02lV[a\x01@V[`@Qa\0r\x91\x90a\x03.V[a\0\xB6a\x01nV[a\0\xC0`\0a\x01\xC8V[V[a\0\xCAa\x01nV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x014W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x01=\x81a\x01\xC8V[PV[``a\x01Ja\x01nV[\x81Q```\0\x80\x83` \x87\x014\x89Z\xF1P=\x81R=`\0` \x83\x01>\x94\x93PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01+V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02/W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02FW`\0\x80\xFD[a\x02O\x82a\x02\x18V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x02\x7FW`\0\x80\xFD[a\x02\x88\x83a\x02\x18V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xA5W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02\xB9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\xCBWa\x02\xCBa\x02VV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02\xF3Wa\x02\xF3a\x02VV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x03\x0CW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x03[W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x03?V[\x81\x81\x11\x15a\x03mW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xB8.\x1C\x18\x18\x05\xB1\xBE\xFA.\xD8\x14\xC8\xD4\x9D\x18\xEB\xF7\xA6np\xBD\x16\xF6W\xFB\x83T\x11V1odsolcC\0\x08\x0C\x003\xA2dipfsX\"\x12 \xB8\xF8\x1E5?\x9E\xD1\xFC\xE1Gy\x11\x8A\xEE\xBEK \xE2d9D\xCF\xCE(\x89\xDC\x96\xB3\x89w4cdsolcC\0\x08\x0C\x003",
    );
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
    /**Constructor`.
```solidity
constructor(address _strategyManager, address _delegation, address _token, address _strategy);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _strategyManager: alloy::sol_types::private::Address,
        pub _delegation: alloy::sol_types::private::Address,
        pub _token: alloy::sol_types::private::Address,
        pub _strategy: alloy::sol_types::private::Address,
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
                        value._strategyManager,
                        value._delegation,
                        value._token,
                        value._strategy,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _strategyManager: tuple.0,
                        _delegation: tuple.1,
                        _token: tuple.2,
                        _strategy: tuple.3,
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
                        &self._delegation,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._token,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._strategy,
                    ),
                )
            }
        }
    };
    /**Function with signature `DEFAULT_AMOUNT()` and selector `0x6850e5c1`.
```solidity
function DEFAULT_AMOUNT() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_AMOUNTCall {}
    ///Container type for the return parameters of the [`DEFAULT_AMOUNT()`](DEFAULT_AMOUNTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_AMOUNTReturn {
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
            impl ::core::convert::From<DEFAULT_AMOUNTCall> for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_AMOUNTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DEFAULT_AMOUNTCall {
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
            impl ::core::convert::From<DEFAULT_AMOUNTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_AMOUNTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_AMOUNTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_AMOUNTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DEFAULT_AMOUNTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_AMOUNT()";
            const SELECTOR: [u8; 4] = [104u8, 80u8, 229u8, 193u8];
            #[inline]
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
    /**Function with signature `callAddress(address,bytes)` and selector `0xf6e8f39d`.
```solidity
function callAddress(address to, bytes memory data) external payable returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct callAddressCall {
        pub to: alloy::sol_types::private::Address,
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`callAddress(address,bytes)`](callAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct callAddressReturn {
        pub _0: alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<callAddressCall> for UnderlyingRustTuple<'_> {
                fn from(value: callAddressCall) -> Self {
                    (value.to, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for callAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { to: tuple.0, data: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<callAddressReturn> for UnderlyingRustTuple<'_> {
                fn from(value: callAddressReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for callAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for callAddressCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = callAddressReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "callAddress(address,bytes)";
            const SELECTOR: [u8; 4] = [246u8, 232u8, 243u8, 157u8];
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
                        &self.to,
                    ),
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
    /**Function with signature `completeQueuedWithdrawal(address,(address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)` and selector `0x063dcd50`.
```solidity
function completeQueuedWithdrawal(address staker, IDelegationManager.Withdrawal memory queuedWithdrawal, address[] memory tokens, uint256 middlewareTimesIndex, bool receiveAsTokens) external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalCall {
        pub staker: alloy::sol_types::private::Address,
        pub queuedWithdrawal: <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
        pub tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub middlewareTimesIndex: alloy::sol_types::private::primitives::aliases::U256,
        pub receiveAsTokens: bool,
    }
    ///Container type for the return parameters of the [`completeQueuedWithdrawal(address,(address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)`](completeQueuedWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalReturn {
        pub _0: alloy::sol_types::private::Bytes,
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
                IDelegationManager::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::primitives::aliases::U256,
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
                    (
                        value.staker,
                        value.queuedWithdrawal,
                        value.tokens,
                        value.middlewareTimesIndex,
                        value.receiveAsTokens,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        queuedWithdrawal: tuple.1,
                        tokens: tuple.2,
                        middlewareTimesIndex: tuple.3,
                        receiveAsTokens: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeQueuedWithdrawalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                IDelegationManager::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawalReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeQueuedWithdrawal(address,(address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)";
            const SELECTOR: [u8; 4] = [6u8, 61u8, 205u8, 80u8];
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
                    <IDelegationManager::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.queuedWithdrawal,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokens),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.middlewareTimesIndex),
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
    /**Function with signature `delegateTo(address,(bytes,uint256),bytes32)` and selector `0xeea9064b`.
```solidity
function delegateTo(address _operator, ISignatureUtils.SignatureWithExpiry memory _approverSignatureAndExpiry, bytes32 _approverSalt) external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToCall {
        pub _operator: alloy::sol_types::private::Address,
        pub _approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        pub _approverSalt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateTo(address,(bytes,uint256),bytes32)`](delegateToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToReturn {
        pub _0: alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<delegateToCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToCall) -> Self {
                    (
                        value._operator,
                        value._approverSignatureAndExpiry,
                        value._approverSalt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operator: tuple.0,
                        _approverSignatureAndExpiry: tuple.1,
                        _approverSalt: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
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
                        &self._operator,
                    ),
                    <ISignatureUtils::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self._approverSignatureAndExpiry,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._approverSalt),
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
    /**Function with signature `depositIntoStrategy(address,address,address,uint256)` and selector `0xa49ca158`.
```solidity
function depositIntoStrategy(address _staker, address _strategy, address _token, uint256 _amount) external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositIntoStrategyCall {
        pub _staker: alloy::sol_types::private::Address,
        pub _strategy: alloy::sol_types::private::Address,
        pub _token: alloy::sol_types::private::Address,
        pub _amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`depositIntoStrategy(address,address,address,uint256)`](depositIntoStrategyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositIntoStrategyReturn {
        pub _0: alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<depositIntoStrategyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositIntoStrategyCall) -> Self {
                    (value._staker, value._strategy, value._token, value._amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositIntoStrategyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _staker: tuple.0,
                        _strategy: tuple.1,
                        _token: tuple.2,
                        _amount: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositIntoStrategyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositIntoStrategyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositIntoStrategyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositIntoStrategyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositIntoStrategyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositIntoStrategy(address,address,address,uint256)";
            const SELECTOR: [u8; 4] = [164u8, 156u8, 161u8, 88u8];
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
                        &self._staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._strategy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._amount),
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
    /**Function with signature `getStaker(address)` and selector `0xa23c44b1`.
```solidity
function getStaker(address _operator) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakerCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getStaker(address)`](getStakerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakerReturn {
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
            impl ::core::convert::From<getStakerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakerCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
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
            impl ::core::convert::From<getStakerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStaker(address)";
            const SELECTOR: [u8; 4] = [162u8, 60u8, 68u8, 177u8];
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
                        &self._operator,
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
    /**Function with signature `mintDepositAndDelegate(address,(bytes,uint256),bytes32,uint256)` and selector `0x34489506`.
```solidity
function mintDepositAndDelegate(address _operator, ISignatureUtils.SignatureWithExpiry memory _approverSignatureAndExpiry, bytes32 _approverSalt, uint256 _depositAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintDepositAndDelegateCall {
        pub _operator: alloy::sol_types::private::Address,
        pub _approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        pub _approverSalt: alloy::sol_types::private::FixedBytes<32>,
        pub _depositAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`mintDepositAndDelegate(address,(bytes,uint256),bytes32,uint256)`](mintDepositAndDelegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintDepositAndDelegateReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<mintDepositAndDelegateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: mintDepositAndDelegateCall) -> Self {
                    (
                        value._operator,
                        value._approverSignatureAndExpiry,
                        value._approverSalt,
                        value._depositAmount,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mintDepositAndDelegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operator: tuple.0,
                        _approverSignatureAndExpiry: tuple.1,
                        _approverSalt: tuple.2,
                        _depositAmount: tuple.3,
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
            impl ::core::convert::From<mintDepositAndDelegateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: mintDepositAndDelegateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mintDepositAndDelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintDepositAndDelegateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintDepositAndDelegateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mintDepositAndDelegate(address,(bytes,uint256),bytes32,uint256)";
            const SELECTOR: [u8; 4] = [52u8, 72u8, 149u8, 6u8];
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
                        &self._operator,
                    ),
                    <ISignatureUtils::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self._approverSignatureAndExpiry,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._approverSalt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._depositAmount),
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
    /**Function with signature `queueWithdrawal(address,(address[],uint256[],address)[])` and selector `0xa76a9d2d`.
```solidity
function queueWithdrawal(address staker, IDelegationManager.QueuedWithdrawalParams[] memory queuedWithdrawalParams) external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalCall {
        pub staker: alloy::sol_types::private::Address,
        pub queuedWithdrawalParams: alloy::sol_types::private::Vec<
            <IDelegationManager::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`queueWithdrawal(address,(address[],uint256[],address)[])`](queueWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalReturn {
        pub _0: alloy::sol_types::private::Bytes,
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
                    IDelegationManager::QueuedWithdrawalParams,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    <IDelegationManager::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<queueWithdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalCall) -> Self {
                    (value.staker, value.queuedWithdrawalParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        queuedWithdrawalParams: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<queueWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queueWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queueWithdrawalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    IDelegationManager::QueuedWithdrawalParams,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queueWithdrawalReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queueWithdrawal(address,(address[],uint256[],address)[])";
            const SELECTOR: [u8; 4] = [167u8, 106u8, 157u8, 45u8];
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
                        IDelegationManager::QueuedWithdrawalParams,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.queuedWithdrawalParams,
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
    /**Function with signature `transfer(address,address,address,uint256)` and selector `0xf18d03cc`.
```solidity
function transfer(address staker, address token, address to, uint256 amount) external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferCall {
        pub staker: alloy::sol_types::private::Address,
        pub token: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`transfer(address,address,address,uint256)`](transferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferReturn {
        pub _0: alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<transferCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferCall) -> Self {
                    (value.staker, value.token, value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        token: tuple.1,
                        to: tuple.2,
                        amount: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfer(address,address,address,uint256)";
            const SELECTOR: [u8; 4] = [241u8, 141u8, 3u8, 204u8];
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
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    ///Container for all the [`DelegationFaucet`](self) function calls.
    pub enum DelegationFaucetCalls {
        DEFAULT_AMOUNT(DEFAULT_AMOUNTCall),
        callAddress(callAddressCall),
        completeQueuedWithdrawal(completeQueuedWithdrawalCall),
        delegateTo(delegateToCall),
        depositIntoStrategy(depositIntoStrategyCall),
        getStaker(getStakerCall),
        mintDepositAndDelegate(mintDepositAndDelegateCall),
        owner(ownerCall),
        queueWithdrawal(queueWithdrawalCall),
        renounceOwnership(renounceOwnershipCall),
        transfer(transferCall),
        transferOwnership(transferOwnershipCall),
    }
    #[automatically_derived]
    impl DelegationFaucetCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 61u8, 205u8, 80u8],
            [52u8, 72u8, 149u8, 6u8],
            [104u8, 80u8, 229u8, 193u8],
            [113u8, 80u8, 24u8, 166u8],
            [141u8, 165u8, 203u8, 91u8],
            [162u8, 60u8, 68u8, 177u8],
            [164u8, 156u8, 161u8, 88u8],
            [167u8, 106u8, 157u8, 45u8],
            [238u8, 169u8, 6u8, 75u8],
            [241u8, 141u8, 3u8, 204u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 232u8, 243u8, 157u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelegationFaucetCalls {
        const NAME: &'static str = "DelegationFaucetCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 12usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DEFAULT_AMOUNT(_) => {
                    <DEFAULT_AMOUNTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::callAddress(_) => {
                    <callAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeQueuedWithdrawal(_) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegateTo(_) => {
                    <delegateToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositIntoStrategy(_) => {
                    <depositIntoStrategyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStaker(_) => {
                    <getStakerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mintDepositAndDelegate(_) => {
                    <mintDepositAndDelegateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::queueWithdrawal(_) => {
                    <queueWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transfer(_) => <transferCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<DelegationFaucetCalls>] = &[
                {
                    fn completeQueuedWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::completeQueuedWithdrawal)
                    }
                    completeQueuedWithdrawal
                },
                {
                    fn mintDepositAndDelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <mintDepositAndDelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::mintDepositAndDelegate)
                    }
                    mintDepositAndDelegate
                },
                {
                    fn DEFAULT_AMOUNT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <DEFAULT_AMOUNTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::DEFAULT_AMOUNT)
                    }
                    DEFAULT_AMOUNT
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::owner)
                    }
                    owner
                },
                {
                    fn getStaker(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <getStakerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::getStaker)
                    }
                    getStaker
                },
                {
                    fn depositIntoStrategy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <depositIntoStrategyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::depositIntoStrategy)
                    }
                    depositIntoStrategy
                },
                {
                    fn queueWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <queueWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::queueWithdrawal)
                    }
                    queueWithdrawal
                },
                {
                    fn delegateTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <delegateToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::delegateTo)
                    }
                    delegateTo
                },
                {
                    fn transfer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::transfer)
                    }
                    transfer
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn callAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationFaucetCalls> {
                        <callAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationFaucetCalls::callAddress)
                    }
                    callAddress
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
                Self::DEFAULT_AMOUNT(inner) => {
                    <DEFAULT_AMOUNTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::callAddress(inner) => {
                    <callAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeQueuedWithdrawal(inner) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::depositIntoStrategy(inner) => {
                    <depositIntoStrategyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStaker(inner) => {
                    <getStakerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::mintDepositAndDelegate(inner) => {
                    <mintDepositAndDelegateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::queueWithdrawal(inner) => {
                    <queueWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DEFAULT_AMOUNT(inner) => {
                    <DEFAULT_AMOUNTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::callAddress(inner) => {
                    <callAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::depositIntoStrategy(inner) => {
                    <depositIntoStrategyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStaker(inner) => {
                    <getStakerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mintDepositAndDelegate(inner) => {
                    <mintDepositAndDelegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::queueWithdrawal(inner) => {
                    <queueWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`DelegationFaucet`](self) events.
    pub enum DelegationFaucetEvents {
        OwnershipTransferred(OwnershipTransferred),
    }
    #[automatically_derived]
    impl DelegationFaucetEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
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
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for DelegationFaucetEvents {
        const NAME: &'static str = "DelegationFaucetEvents";
        const COUNT: usize = 1usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
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
    impl alloy_sol_types::private::IntoLogData for DelegationFaucetEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`DelegationFaucet`](self) contract instance.

See the [wrapper's documentation](`DelegationFaucetInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DelegationFaucetInstance<T, P, N> {
        DelegationFaucetInstance::<T, P, N>::new(address, provider)
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
        _delegation: alloy::sol_types::private::Address,
        _token: alloy::sol_types::private::Address,
        _strategy: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<DelegationFaucetInstance<T, P, N>>,
    > {
        DelegationFaucetInstance::<
            T,
            P,
            N,
        >::deploy(provider, _strategyManager, _delegation, _token, _strategy)
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
        _delegation: alloy::sol_types::private::Address,
        _token: alloy::sol_types::private::Address,
        _strategy: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        DelegationFaucetInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _strategyManager, _delegation, _token, _strategy)
    }
    /**A [`DelegationFaucet`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`DelegationFaucet`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DelegationFaucetInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for DelegationFaucetInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DelegationFaucetInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DelegationFaucetInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`DelegationFaucet`](self) contract instance.

See the [wrapper's documentation](`DelegationFaucetInstance`) for more details.*/
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
            _delegation: alloy::sol_types::private::Address,
            _token: alloy::sol_types::private::Address,
            _strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<DelegationFaucetInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _strategyManager,
                _delegation,
                _token,
                _strategy,
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
            _delegation: alloy::sol_types::private::Address,
            _token: alloy::sol_types::private::Address,
            _strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _strategyManager,
                            _delegation,
                            _token,
                            _strategy,
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
    impl<T, P: ::core::clone::Clone, N> DelegationFaucetInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DelegationFaucetInstance<T, P, N> {
            DelegationFaucetInstance {
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
    > DelegationFaucetInstance<T, P, N> {
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
        ///Creates a new call builder for the [`DEFAULT_AMOUNT`] function.
        pub fn DEFAULT_AMOUNT(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEFAULT_AMOUNTCall, N> {
            self.call_builder(&DEFAULT_AMOUNTCall {})
        }
        ///Creates a new call builder for the [`callAddress`] function.
        pub fn callAddress(
            &self,
            to: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, callAddressCall, N> {
            self.call_builder(&callAddressCall { to, data })
        }
        ///Creates a new call builder for the [`completeQueuedWithdrawal`] function.
        pub fn completeQueuedWithdrawal(
            &self,
            staker: alloy::sol_types::private::Address,
            queuedWithdrawal: <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
            tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            middlewareTimesIndex: alloy::sol_types::private::primitives::aliases::U256,
            receiveAsTokens: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeQueuedWithdrawalCall, N> {
            self.call_builder(
                &completeQueuedWithdrawalCall {
                    staker,
                    queuedWithdrawal,
                    tokens,
                    middlewareTimesIndex,
                    receiveAsTokens,
                },
            )
        }
        ///Creates a new call builder for the [`delegateTo`] function.
        pub fn delegateTo(
            &self,
            _operator: alloy::sol_types::private::Address,
            _approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            _approverSalt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateToCall, N> {
            self.call_builder(
                &delegateToCall {
                    _operator,
                    _approverSignatureAndExpiry,
                    _approverSalt,
                },
            )
        }
        ///Creates a new call builder for the [`depositIntoStrategy`] function.
        pub fn depositIntoStrategy(
            &self,
            _staker: alloy::sol_types::private::Address,
            _strategy: alloy::sol_types::private::Address,
            _token: alloy::sol_types::private::Address,
            _amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositIntoStrategyCall, N> {
            self.call_builder(
                &depositIntoStrategyCall {
                    _staker,
                    _strategy,
                    _token,
                    _amount,
                },
            )
        }
        ///Creates a new call builder for the [`getStaker`] function.
        pub fn getStaker(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakerCall, N> {
            self.call_builder(&getStakerCall { _operator })
        }
        ///Creates a new call builder for the [`mintDepositAndDelegate`] function.
        pub fn mintDepositAndDelegate(
            &self,
            _operator: alloy::sol_types::private::Address,
            _approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            _approverSalt: alloy::sol_types::private::FixedBytes<32>,
            _depositAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, mintDepositAndDelegateCall, N> {
            self.call_builder(
                &mintDepositAndDelegateCall {
                    _operator,
                    _approverSignatureAndExpiry,
                    _approverSalt,
                    _depositAmount,
                },
            )
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`queueWithdrawal`] function.
        pub fn queueWithdrawal(
            &self,
            staker: alloy::sol_types::private::Address,
            queuedWithdrawalParams: alloy::sol_types::private::Vec<
                <IDelegationManager::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, queueWithdrawalCall, N> {
            self.call_builder(
                &queueWithdrawalCall {
                    staker,
                    queuedWithdrawalParams,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`transfer`] function.
        pub fn transfer(
            &self,
            staker: alloy::sol_types::private::Address,
            token: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferCall, N> {
            self.call_builder(
                &transferCall {
                    staker,
                    token,
                    to,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DelegationFaucetInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
    }
}
