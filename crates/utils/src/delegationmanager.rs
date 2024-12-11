///Module containing a contract's types and functions.
/**

```solidity
library IDelegationManagerTypes {
    struct OperatorDetails { address __deprecated_earningsReceiver; address delegationApprover; uint32 __deprecated_stakerOptOutWindowBlocks; }
    struct QueuedWithdrawalParams { address[] strategies; uint256[] shares; address withdrawer; }
    struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startTimestamp; address[] strategies; uint256[] scaledSharesToWithdraw; }
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
struct OperatorDetails { address __deprecated_earningsReceiver; address delegationApprover; uint32 __deprecated_stakerOptOutWindowBlocks; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorDetails {
        pub __deprecated_earningsReceiver: alloy::sol_types::private::Address,
        pub delegationApprover: alloy::sol_types::private::Address,
        pub __deprecated_stakerOptOutWindowBlocks: u32,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorDetails> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorDetails) -> Self {
                (
                    value.__deprecated_earningsReceiver,
                    value.delegationApprover,
                    value.__deprecated_stakerOptOutWindowBlocks,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorDetails {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    __deprecated_earningsReceiver: tuple.0,
                    delegationApprover: tuple.1,
                    __deprecated_stakerOptOutWindowBlocks: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorDetails {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorDetails {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.__deprecated_earningsReceiver,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegationApprover,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.__deprecated_stakerOptOutWindowBlocks,
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
        impl alloy_sol_types::SolType for OperatorDetails {
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
        impl alloy_sol_types::SolStruct for OperatorDetails {
            const NAME: &'static str = "OperatorDetails";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorDetails(address __deprecated_earningsReceiver,address delegationApprover,uint32 __deprecated_stakerOptOutWindowBlocks)",
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
                            &self.__deprecated_earningsReceiver,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegationApprover,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.__deprecated_stakerOptOutWindowBlocks,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorDetails {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.__deprecated_earningsReceiver,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegationApprover,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.__deprecated_stakerOptOutWindowBlocks,
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
                    &rust.__deprecated_earningsReceiver,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegationApprover,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.__deprecated_stakerOptOutWindowBlocks,
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
struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startTimestamp; address[] strategies; uint256[] scaledSharesToWithdraw; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Withdrawal {
        pub staker: alloy::sol_types::private::Address,
        pub delegatedTo: alloy::sol_types::private::Address,
        pub withdrawer: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub startTimestamp: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub scaledSharesToWithdraw: alloy::sol_types::private::Vec<
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
                    value.startTimestamp,
                    value.strategies,
                    value.scaledSharesToWithdraw,
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
                    startTimestamp: tuple.4,
                    strategies: tuple.5,
                    scaledSharesToWithdraw: tuple.6,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.scaledSharesToWithdraw,
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
                    "Withdrawal(address staker,address delegatedTo,address withdrawer,uint256 nonce,uint32 startTimestamp,address[] strategies,uint256[] scaledSharesToWithdraw)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.scaledSharesToWithdraw,
                        )
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
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.scaledSharesToWithdraw,
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
                    &rust.startTimestamp,
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
                    &rust.scaledSharesToWithdraw,
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
library IDelegationManagerTypes {
    struct OperatorDetails {
        address __deprecated_earningsReceiver;
        address delegationApprover;
        uint32 __deprecated_stakerOptOutWindowBlocks;
    }
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
        uint32 startTimestamp;
        address[] strategies;
        uint256[] scaledSharesToWithdraw;
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
    error AllocationDelaySet();
    error CallerCannotUndelegate();
    error CurrentlyPaused();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InputArrayLengthZero();
    error InvalidNewPausedStatus();
    error InvalidSignature();
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
    error UnauthorizedCaller();
    error WithdrawalDelayExceedsMax();
    error WithdrawalDelayExeedsMax();
    error WithdrawalDelayNotElapsed();
    error WithdrawalDoesNotExist();
    error WithdrawalExceedsMax();
    error WithdrawalNotQueued();
    error WithdrawerNotCaller();
    error WithdrawerNotStaker();

    event BeaconChainScalingFactorDecreased(address staker, uint64 newBeaconChainScalingFactor);
    event DepositScalingFactorUpdated(address staker, address strategy, uint256 newDepositScalingFactor);
    event Initialized(uint8 version);
    event OperatorDetailsModified(address indexed operator, IDelegationManagerTypes.OperatorDetails newOperatorDetails);
    event OperatorMetadataURIUpdated(address indexed operator, string metadataURI);
    event OperatorRegistered(address indexed operator, IDelegationManagerTypes.OperatorDetails operatorDetails);
    event OperatorSharesDecreased(address indexed operator, address staker, address strategy, uint256 shares);
    event OperatorSharesIncreased(address indexed operator, address staker, address strategy, uint256 shares);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    event SlashingWithdrawalCompleted(bytes32 withdrawalRoot);
    event SlashingWithdrawalQueued(bytes32 withdrawalRoot, IDelegationManagerTypes.Withdrawal withdrawal);
    event StakerDelegated(address indexed staker, address indexed operator);
    event StakerForceUndelegated(address indexed staker, address indexed operator);
    event StakerUndelegated(address indexed staker, address indexed operator);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _avsDirectory, address _strategyManager, address _eigenPodManager, address _allocationManager, uint32 _MIN_WITHDRAWAL_DELAY);

    function DELEGATION_APPROVAL_TYPEHASH() external view returns (bytes32);
    function LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS() external view returns (uint256);
    function LEGACY_WITHDRAWAL_CHECK_VALUE() external view returns (uint32);
    function MIN_WITHDRAWAL_DELAY() external view returns (uint32);
    function STAKER_DELEGATION_TYPEHASH() external view returns (bytes32);
    function allocationManager() external view returns (address);
    function avsDirectory() external view returns (address);
    function beaconChainETHStrategy() external view returns (address);
    function calculateCurrentStakerDelegationDigestHash(address staker, address operator, uint256 expiry) external view returns (bytes32);
    function calculateDelegationApprovalDigestHash(address staker, address operator, address approver, bytes32 approverSalt, uint256 expiry) external view returns (bytes32);
    function calculateStakerDelegationDigestHash(address staker, uint256 nonce, address operator, uint256 expiry) external view returns (bytes32);
    function calculateWithdrawalRoot(IDelegationManagerTypes.Withdrawal memory withdrawal) external pure returns (bytes32);
    function completeQueuedWithdrawal(IDelegationManagerTypes.Withdrawal memory withdrawal, address[] memory tokens, bool receiveAsTokens) external;
    function completeQueuedWithdrawals(IDelegationManagerTypes.Withdrawal[] memory withdrawals, address[][] memory tokens, bool[] memory receiveAsTokens) external;
    function cumulativeWithdrawalsQueued(address) external view returns (uint256);
    function decreaseBeaconChainScalingFactor(address staker, uint256 existingDepositShares, uint64 proportionOfOldBalance) external;
    function decreaseOperatorShares(address operator, address strategy, uint64 previousTotalMagnitude, uint64 newTotalMagnitude) external;
    function delegateTo(address operator, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    function delegateToBySignature(address staker, address operator, ISignatureUtils.SignatureWithExpiry memory stakerSignatureAndExpiry, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    function delegatedTo(address) external view returns (address);
    function delegationApprover(address operator) external view returns (address);
    function delegationApproverSaltIsSpent(address, bytes32) external view returns (bool);
    function domainSeparator() external view returns (bytes32);
    function eigenPodManager() external view returns (address);
    function getCompletableTimestamp(uint32 startTimestamp) external view returns (uint32 completableTimestamp);
    function getDepositedShares(address staker) external view returns (address[] memory, uint256[] memory);
    function getOperatorShares(address operator, address[] memory strategies) external view returns (uint256[] memory);
    function getOperatorsShares(address[] memory operators, address[] memory strategies) external view returns (uint256[][] memory);
    function getWithdrawableShares(address staker, address[] memory strategies) external view returns (uint256[] memory withdrawableShares);
    function increaseDelegatedShares(address staker, address strategy, uint256 existingDepositShares, uint256 addedShares) external;
    function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus) external;
    function isDelegated(address staker) external view returns (bool);
    function isOperator(address operator) external view returns (bool);
    function modifyOperatorDetails(IDelegationManagerTypes.OperatorDetails memory newOperatorDetails) external;
    function operatorDetails(address operator) external view returns (IDelegationManagerTypes.OperatorDetails memory);
    function operatorShares(address, address) external view returns (uint256);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function pendingWithdrawals(bytes32) external view returns (bool);
    function queueWithdrawals(IDelegationManagerTypes.QueuedWithdrawalParams[] memory params) external returns (bytes32[] memory);
    function registerAsOperator(IDelegationManagerTypes.OperatorDetails memory registeringOperatorDetails, uint32 allocationDelay, string memory metadataURI) external;
    function renounceOwnership() external;
    function setPauserRegistry(address newPauserRegistry) external;
    function stakerNonce(address) external view returns (uint256);
    function stakerScalingFactor(address, address) external view returns (uint256 depositScalingFactor, bool isBeaconChainScalingFactorSet, uint64 beaconChainScalingFactor);
    function strategyManager() external view returns (address);
    function transferOwnership(address newOwner) external;
    function undelegate(address staker) external returns (bytes32[] memory withdrawalRoots);
    function unpause(uint256 newPausedStatus) external;
    function updateOperatorMetadataURI(string memory metadataURI) external;
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
    "name": "LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS",
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
    "name": "LEGACY_WITHDRAWAL_CHECK_VALUE",
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
    "name": "MIN_WITHDRAWAL_DELAY",
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
    "name": "STAKER_DELEGATION_TYPEHASH",
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
    "name": "avsDirectory",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAVSDirectory"
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
    "name": "calculateCurrentStakerDelegationDigestHash",
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
    "name": "calculateStakerDelegationDigestHash",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
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
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledSharesToWithdraw",
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
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledSharesToWithdraw",
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
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledSharesToWithdraw",
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
    "name": "decreaseBeaconChainScalingFactor",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "existingDepositShares",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "proportionOfOldBalance",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "decreaseOperatorShares",
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
        "name": "previousTotalMagnitude",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "newTotalMagnitude",
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
    "name": "delegateToBySignature",
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
        "name": "stakerSignatureAndExpiry",
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
        "name": "",
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
        "name": "",
        "type": "address",
        "internalType": "address"
      },
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
    "name": "getCompletableTimestamp",
    "inputs": [
      {
        "name": "startTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "completableTimestamp",
        "type": "uint32",
        "internalType": "uint32"
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
        "name": "existingDepositShares",
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
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
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
    "name": "modifyOperatorDetails",
    "inputs": [
      {
        "name": "newOperatorDetails",
        "type": "tuple",
        "internalType": "struct IDelegationManagerTypes.OperatorDetails",
        "components": [
          {
            "name": "__deprecated_earningsReceiver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegationApprover",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "__deprecated_stakerOptOutWindowBlocks",
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
    "name": "operatorDetails",
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
        "internalType": "struct IDelegationManagerTypes.OperatorDetails",
        "components": [
          {
            "name": "__deprecated_earningsReceiver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegationApprover",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "__deprecated_stakerOptOutWindowBlocks",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorShares",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
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
        "name": "registeringOperatorDetails",
        "type": "tuple",
        "internalType": "struct IDelegationManagerTypes.OperatorDetails",
        "components": [
          {
            "name": "__deprecated_earningsReceiver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegationApprover",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "__deprecated_stakerOptOutWindowBlocks",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "setPauserRegistry",
    "inputs": [
      {
        "name": "newPauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "stakerNonce",
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
    "name": "stakerScalingFactor",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "outputs": [
      {
        "name": "depositScalingFactor",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "isBeaconChainScalingFactorSet",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "beaconChainScalingFactor",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
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
    "name": "BeaconChainScalingFactorDecreased",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newBeaconChainScalingFactor",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
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
    "name": "OperatorDetailsModified",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOperatorDetails",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IDelegationManagerTypes.OperatorDetails",
        "components": [
          {
            "name": "__deprecated_earningsReceiver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegationApprover",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "__deprecated_stakerOptOutWindowBlocks",
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
        "name": "operatorDetails",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IDelegationManagerTypes.OperatorDetails",
        "components": [
          {
            "name": "__deprecated_earningsReceiver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegationApprover",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "__deprecated_stakerOptOutWindowBlocks",
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
    "name": "PauserRegistrySet",
    "inputs": [
      {
        "name": "pauserRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "newPauserRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IPauserRegistry"
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
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledSharesToWithdraw",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
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
    "name": "AllocationDelaySet",
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
    "name": "InvalidSignature",
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
    "name": "UnauthorizedCaller",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawalDelayExceedsMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawalDelayExeedsMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawalDelayNotElapsed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawalDoesNotExist",
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
    ///0x610160806040523461023557601f614c0138819003918201601f19168301916001600160401b038311848410176102395780849260a094604052833981010312610235578051906001600160a01b03821682036102355760208101516001600160a01b0381168103610235576040820151906001600160a01b0382168203610235576060830151926001600160a01b038416840361023557608001519363ffffffff851685036102355760805260a05260c05260e0526101005246610120525f54600881901c60ff166101e05760ff808216106101a6575b6040516149b3908161024e823960805181611528015260a05181818161201c0152818161226a015281816131b00152614427015260c05181818161160e01528181611e97015281816122270152818161321201526143fa015260e051818181610711015281816109e501528181610a6701528181610db4015281816112bb015281816116b401528181611b0c015281816120e5015281816126b3015281816136970152614125015261010051818181611a7c0152612fd9015261012051816144550152610140518161447b0152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100d7565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f905f3560e01c90816304a4f97914612872575080630b9f487a146128365780630dd8dd021461264f57806310d67a2f146125b9578063136439dd146124ef57806315c4a288146124b55780631794bb3c146123175780631bbce091146122d157806329c77d4f1461229957806339b70e38146122555780633c651cf214611ff45780633cdeb5e014611fb15780633e28391d14611f745780634337738214611f3a578063457c607014611ec65780634665bcda14611e825780634973006014611aa05780634a5f2b5d14611a5f578063595c6a6714611990578063597b36da146119555780635ac86ab71461191f5780635c975abb146119015780635d9aed23146115e557806365da1264146115a457806366d5ba93146115575780636b3aa72e146115125780636d70f7ae146114e4578063715018a614611487578063778e55f3146114345780637f5480711461112c578063886f1195146111035780638da5cb5b146110da57806390041347146110ab5780639104c319146110835780639435bb4314610f4e57806399be81c814610edc578063a178848414610ea3578063a57ab10b14610d73578063b7f06ebe14610d44578063bb45fef214610cfb578063c5e480db14610c58578063c94b511114610c24578063c978f7ac14610a14578063ca8aa7c7146109cf578063cb00387b146109b2578063cebc04ef14610993578063da8be86414610941578063e4cc3f901461089b578063eea9064b146105f8578063f0e0e6761461044d578063f16172b014610415578063f2fde38b14610384578063f698da25146103615763fabc1cbc14610270575f80fd5b3461035e57602036600319011261035e5760655460405163755b36bd60e11b81526004803592602091839182906001600160a01b03165afa908115610353578391610324575b506001600160a01b031633036103155760665419811981160361030657806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a280f35b63c61dca5d60e01b8252600482fd5b63794821ff60e01b8252600482fd5b610346915060203d60201161034c575b61033e8183612a0c565b810190612f29565b5f6102b6565b503d610334565b6040513d85823e3d90fd5b80fd5b503461035e578060031936011261035e57602061037c614452565b604051908152f35b503461035e57602036600319011261035e5761039e6128aa565b6103a6613fd6565b6001600160a01b038116156103c1576103be90613d74565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b503461035e57606036600319011261035e576104303361343f565b1561043e576103be33613e8f565b6325ec6c1f60e01b8152600490fd5b503461035e57604036600319011261035e576004356001600160401b0381116105f457366023820112156105f45780600401359061048a82612a2d565b916104986040519384612a0c565b8083526024602084019160051b830101913683116105f057602401905b8282106105d8575050506024356001600160401b0381116105d4576104de903690600401612a98565b8151916104ea83612a2d565b926104f86040519485612a0c565b808452610507601f1991612a2d565b01845b8181106105c3575050835b815181101561055a5760019061053e846001600160a01b036105378487612f15565b5116613472565b6105488287612f15565b526105538186612f15565b5001610515565b505050906040519182916020830160208452825180915260408401602060408360051b870101940192905b82821061059457505050500390f35b919360019193955060206105b38192603f198a82030186528851612c12565b9601920192018594939192610585565b80606060208093880101520161050a565b8280fd5b602080916105e5846128ec565b8152019101906104b5565b8480fd5b5080fd5b503461035e57606036600319011261035e576106126128aa565b906024356001600160401b0381116105f457610632903690600401612c60565b335f908152609a6020526040902054604435906001600160a01b031661088c5761065b8461343f565b1561087d57600180606654161461086e576001600160a01b0384811680855260996020526040852060010154909391169081151580610864575b8061085a575b6107cf575b505033808452609a6020526040842080546001600160a01b0319168417905590507fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433048380a36106ee33613183565b60405163547afb8760e01b81529183838061070d848960048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9283156107c45784936107a0575b50835b815181101561079c576001906107966001600160a01b0361076d8386612f15565b51166107798387612f15565b516001600160401b0361078c858a612f15565b511691338b613dbc565b0161074c565b8480f35b6107bd9193503d8086833e6107b58183612a0c565b810190612e3f565b915f610749565b6040513d86823e3d90fd5b602081018051421161084b57828652609c6020526040862084875260205260ff60408720541661083c579061080b610814925185858a33612d1b565b9051908361402e565b8352609c6020526040832090835260205260408220600160ff198254161790555f80806106a0565b630d4c4c9160e21b8652600486fd5b630819bdcd60e01b8652600486fd5b508333141561069b565b5081331415610695565b63840a48d560e01b8352600483fd5b6325ec6c1f60e01b8352600483fd5b633bf2b50360e11b8352600483fd5b503461035e57606036600319011261035e57600435906001600160401b03821161035e5760e0600319833603011261035e576024356001600160401b0381116105f4576108ec903690600401612900565b60443591821515830361093d57600480606654161461092e576109269394610919600260c95414156134da565b600260c95560040161404f565b600160c95580f35b63840a48d560e01b8452600484fd5b8380fd5b503461035e57602036600319011261035e5761095b6128aa565b906002806066541614610984576109806109748361359c565b60405191829182612930565b0390f35b63840a48d560e01b8152600490fd5b503461035e578060031936011261035e576020604051633b9aca008152f35b503461035e578060031936011261035e57602060405161c4e08152f35b503461035e578060031936011261035e576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461035e57610a2336612cdc565b6001600160a01b03808316808552609a602052604080862054905163547afb8760e01b815294959194921692909190828280610a63848860048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610353578392610c08575b50610aaa8151612d8e565b9280941515945b8251811015610bf257610b299060206001600160a01b03610add81610ad68589612f15565b51166143dc565b166001600160a01b03610af08488612f15565b5160405163fe243a1760e01b81526001600160a01b038e8116600483015292909116909116602482015293849190829081906044820190565b03915afa8015610353578390610bbc575b600192508715610bac57610b9a9089855260a260205260408520848060a01b03610b648589612f15565b5116858060a01b03165f5260205260405f2090610b946001600160401b03610b8c868b612f15565b511692613141565b90613f9a565b610ba48288612f15565b525b01610ab1565b610bb68288612f15565b52610ba6565b506020823d8211610bea575b81610bd560209383612a0c565b81010312610be65760019151610b3a565b5f80fd5b3d9150610bc8565b6040516020808252819061098090820188612c12565b610c1d9192503d8085833e6107b58183612a0c565b905f610a9f565b503461035e57608036600319011261035e57602061037c610c436128aa565b610c4b6128d6565b6064359160243590613536565b503461035e57602036600319011261035e576040606091610c776128aa565b81838051610c84816129d6565b828152826020820152015260018060a01b0316815260996020522063ffffffff604051610cb0816129d6565b6001808060a01b0384541693848352015490826040602083019260018060a01b0385168452019260a01c16825260405193845260018060a01b03905116602084015251166040820152f35b503461035e57604036600319011261035e5760209060ff906040906001600160a01b03610d266128aa565b168152609c8452818120602435825284522054166040519015158152f35b503461035e57602036600319011261035e5760ff60406020926004358152609e84522054166040519015158152f35b503461035e57608036600319011261035e57610d8d6128aa565b610d956128c0565b610d9d612bc0565b606435926001600160401b0384168094036105f0577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303610e94577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd9291610e4c610e529260018060a01b03169586885260986020526040882060018060a01b0386165f52602052610e476001600160401b0360405f2054941684614781565b6146fd565b90613176565b9083855260986020526040852060018060a01b0382165f5260205260405f20610e7c838254613176565b9055610e8e604051928392878461394d565b0390a280f35b6323d871a560e01b8552600485fd5b503461035e57602036600319011261035e576020906040906001600160a01b03610ecb6128aa565b168152609f83522054604051908152f35b503461035e57602036600319011261035e576004356001600160401b0381116105f457610f0d90369060040161297a565b610f163361343f565b1561087d57610e8e7f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080909160405191829133958361303a565b503461035e57606036600319011261035e576004356001600160401b0381116105f457610f7f903690600401612900565b906024356001600160401b03811161093d57610f9f903690600401612900565b926044356001600160401b03811161107f57610fbf903690600401612900565b939092600480606654161461107057610fe0600260c99794975414156134da565b600260c9553682900360de190192875b87811015611067578060051b90818501359186831215611063578382101561104f5761101e90850185612df6565b61102c838b8b969496613526565b3590811515820361104b5760019461104593890161404f565b01610ff0565b8c80fd5b634e487b7160e01b8b52603260045260248bfd5b8a80fd5b88600160c95580f35b63840a48d560e01b8752600487fd5b8580fd5b503461035e578060031936011261035e5760206040515f51602061495e5f395f51905f528152f35b503461035e576109806110c66110c036612cdc565b90613472565b604051918291602083526020830190612c12565b503461035e578060031936011261035e576033546040516001600160a01b039091168152602090f35b503461035e578060031936011261035e576065546040516001600160a01b039091168152602090f35b503461035e5760a036600319011261035e576111466128aa565b9061114f6128c0565b916044356001600160401b0381116105d45761116f903690600401612c60565b6064356001600160401b03811161093d5761118e903690600401612c60565b90608435602082018051421161084b576001600160a01b038086165f908152609a602052604090205416611425576111c58761343f565b15611416576001906111fc828060a01b03871694858952609b6020526111f360408a205493518b858b613536565b9051908861402e565b838752609b6020520160408620556001806066541614611407576001600160a01b03868116808752609960205260408720600101549094911690811515806113fd575b806113f3575b611371575b505050808452609a6020526040842080546001600160a01b031916831790557fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433048480a361129681613183565b60405163547afb8760e01b81529290918484806112b7848a60048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa93841561136657859461134a575b50845b8151811015611346576001906113406001600160a01b036113178386612f15565b51166113238388612f15565b516001600160401b03611336858b612f15565b511691878c613dbc565b016112f6565b8580f35b61135f9194503d8087833e6107b58183612a0c565b925f6112f3565b6040513d87823e3d90fd5b60208101428151106113e457828852609c6020526040882084895260205260ff6040892054166113d5579061080b6113ad925185858c8b612d1b565b8552609c6020526040852090855260205260408420600160ff198254161790555f808061124a565b630d4c4c9160e21b8852600488fd5b630819bdcd60e01b8852600488fd5b5084331415611245565b508133141561123f565b63840a48d560e01b8552600485fd5b6325ec6c1f60e01b8652600486fd5b633bf2b50360e11b8652600486fd5b503461035e57604036600319011261035e5760406114506128aa565b916114596128c0565b9260018060a01b031681526098602052209060018060a01b03165f52602052602060405f2054604051908152f35b503461035e578060031936011261035e576114a0613fd6565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b503461035e57602036600319011261035e5760206115086115036128aa565b61343f565b6040519015158152f35b503461035e578060031936011261035e576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461035e57602036600319011261035e5761159661098061157f61157a6128aa565b613183565b604092919251938493604085526040850190612bd6565b908382036020850152612c12565b503461035e57602036600319011261035e576020906001600160a01b036115c96128aa565b168152609a8252604060018060a01b0391205416604051908152f35b503461035e57606036600319011261035e576115ff6128aa565b6024359061160b612bc0565b917f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036118f2576001600160a01b03828116808652609a6020526040808720548151931695919390926116b09190889061166f8682612a0c565b60018152601f1986013660208301375f51602061495e5f395f51905f5261169582612f08565b5285518094819263547afb8760e01b83528b60048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156118e8579161181d9161183e938a926118cc575b50868a5260a2602052858a205f51602061495e5f395f51905f525f526020527fddf935ec8825c7afee6a15d4731e28963ee96dfcb85d0a1e794b43318bbca4fd8661175e815f206117586001600160401b0361175088612f08565b511691613141565b85613f9a565b96898d5260a2602052818d205f51602061495e5f395f51905f525f526020526001806117aa845f20936001600160401b03806117a161179c88613141565b6145a7565b169116906146fd565b92019168ffffffffffffffff0083549160081b169068ffffffffffffffffff191617178091556001600160401b038251918b835260081c166020820152a1868a5260a2602052858a205f51602061495e5f395f51905f525f52602052610b946001600160401b03610b8c885f2094612f08565b6001600160a01b039586165f908152609a6020526040902054909516151590565b611846578580f35b7f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd9361187191613176565b9184865260986020528186205f51602061495e5f395f51905f52875260205281862061189e848254613176565b905581519081525f51602061495e5f395f51905f52602082015290810191909152606090a25f808080808580f35b6118e19192503d808c833e6107b58183612a0c565b905f6116f5565b84513d8a823e3d90fd5b633213a66160e21b8452600484fd5b503461035e578060031936011261035e576020606654604051908152f35b503461035e57602036600319011261035e5760043560ff81168091036105f457600190602092501b806066541614604051908152f35b503461035e57602036600319011261035e57600435906001600160401b03821161035e57602061037c61198b3660048601612b02565b613110565b503461035e578060031936011261035e5760655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa908115611a54578291611a25575b5015611a16575f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a280f35b631d77d47760e21b8152600490fd5b611a47915060203d602011611a4d575b611a3f8183612a0c565b810190612f48565b5f6119db565b503d611a35565b6040513d84823e3d90fd5b503461035e578060031936011261035e57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b5034610be657366003190160a08112610be657606013610be65760643563ffffffff8116809103610be6576084356001600160401b038111610be657611aea90369060040161297a565b335f908152609a6020526040902054909291906001600160a01b0316611e73577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b15610be65760405190632b6241f360e11b825233600483015260248201525f8160448183875af18015611e6857611e53575b50611b7333613e8f565b604051611b7f816129a7565b6060815260208101908582526001806066541614611e4457338652609960205260408620600101546001600160a01b03169182151580611e3a575b80611e33575b611d5f575b505033808652609a6020526040862080546001600160a01b031916821790559050807fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433048680a3611c1433613183565b94909181604051809563547afb8760e01b82528180611c37883360048401613018565b03915afa938415611a54578294611d43575b50815b8351811015611c9c57600190611c966001600160a01b03611c6d8388612f15565b5116611c79838b612f15565b516001600160401b03611c8c858b612f15565b5116913333613dbc565b01611c4c565b50604051856004356001600160a01b038116908190036105f05782526024356001600160a01b038116908190036105f05760208301526044359063ffffffff82168092036105f05782610e8e9260407f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080909501527f8e8485583a2310d41f7c82b9427d0bd49bad74bb9cff9d3402a29d8f9b28a0e260603392a260405191829133958361303a565b611d589194503d8084833e6107b58183612a0c565b925f611c49565b80514211611e2457828752609c6020526040872087805260205260ff604088205416611e15579061080b611ded925160405160208101917f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad83528660408301523360608301523360808301528a60a083015260c082015260c08152611de560e082612a0c565b51902061391a565b8452609c6020526040842084805260205260408420600160ff198254161790555f8080611bc5565b630d4c4c9160e21b8752600487fd5b630819bdcd60e01b8752600487fd5b5086611bc0565b5082331415611bba565b63840a48d560e01b8652600486fd5b611e609194505f90612a0c565b5f925f611b69565b6040513d5f823e3d90fd5b633bf2b50360e11b5f5260045ffd5b34610be6575f366003190112610be6576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610be6576040366003190112610be657611edf6128aa565b611ee76128c0565b9060018060a01b03165f5260a260205260405f209060018060a01b03165f52602052606060405f206001600160401b036001825492015460405192835260ff81161515602084015260081c166040820152f35b34610be6575f366003190112610be65760206040517f39111bc4a4d688e1f685123d7497d4615370152a8ee4a0593e647bd06ad8bb0b8152f35b34610be6576020366003190112610be6576020611508611f926128aa565b6001600160a01b039081165f908152609a602052604090205416151590565b34610be6576020366003190112610be6576001600160a01b03611fd26128aa565b165f526099602052602060018060a01b03600160405f20015416604051908152f35b34610be6576080366003190112610be65761200d6128aa565b6120156128c0565b60643591337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316148015612223575b15612214576001600160a01b038082165f908152609a60205260409020541661207157005b6001600160a01b038181165f818152609a60205260408082205481519197941692916120e1916120a18982612a0c565b60018152601f1989013660208301376120b981612f08565b6001600160a01b0389169052885163547afb8760e01b81529283918291908760048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561220a57926121eb94926001600160401b0361215b6121df947f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f9b975f916121f0575b50612f08565b511691805f526098602052855f2060018060a01b038a165f52602052855f20612185868254612f8e565b90557f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c8651806121b7888d8d8461394d565b0390a25f5260a2602052835f2060018060a01b0388165f52602052835f20926044358461450f565b5490519384938461394d565b0390a1005b61220491503d805f833e6107b58183612a0c565b8c612155565b87513d5f823e3d90fd5b63045206a560e21b5f5260045ffd5b50337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03161461204c565b34610be6575f366003190112610be6576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610be6576020366003190112610be6576001600160a01b036122ba6128aa565b165f52609b602052602060405f2054604051908152f35b34610be6576060366003190112610be657602061037c6122ef6128aa565b6122f76128c0565b6001600160a01b0382165f908152609b8552604090205460443592613536565b34610be6576060366003190112610be6576123306128aa565b6123386128c0565b604435915f549260ff8460081c1615938480956124a8575b8015612491575b156124355760ff1981166001175f5584612424575b506065546001600160a01b03161580612412575b15612403576123c792816123c2926066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2613d17565b613d74565b6123cd57005b61ff00195f54165f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a1005b6339b190bb60e11b5f5260045ffd5b506001600160a01b0383161515612380565b61ffff1916610101175f558461236c565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b1580156123575750600160ff821614612357565b50600160ff821610612350565b34610be6576020366003190112610be65760043563ffffffff81168103610be6576124e1602091612f9b565b63ffffffff60405191168152f35b34610be6576020366003190112610be6576004356024602060018060a01b03606554166040519283809263237dfb4760e11b82523360048301525afa908115611e68575f9161259a575b501561258b576066548181160361257c57806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b63c61dca5d60e01b5f5260045ffd5b631d77d47760e21b5f5260045ffd5b6125b3915060203d602011611a4d57611a3f8183612a0c565b82612539565b34610be6576020366003190112610be6576125d26128aa565b60655460405163755b36bd60e11b815290602090829060049082906001600160a01b03165afa908115611e68575f91612630575b506001600160a01b031633036126215761261f90613d17565b005b63794821ff60e01b5f5260045ffd5b612649915060203d60201161034c5761033e8183612a0c565b82612606565b34610be6576020366003190112610be6576004356001600160401b038111610be65761267f903690600401612900565b60028060665416146128275761269481612d8e565b335f908152609a60205260408120546001600160a01b039081169492937f000000000000000000000000000000000000000000000000000000000000000090911692915b8181106126ed57604051806109808782612930565b6127016126fb828486612dc0565b80612df6565b905061271b612711838587612dc0565b6020810190612df6565b9190500361281857336001600160a01b03612742604061273c858789612dc0565b01612e2b565b160361280957805f61275b6126fb612784948688612dc0565b604051948592839263547afb8760e01b84528c6004850152604060248501526044840191612ec9565b0381885afa918215611e68576001926127de915f916127ef575b506127d66127b06126fb85888a612dc0565b6127ce6127c4612711888b8d979697612dc0565b9490923691612a44565b923691612ab6565b908a3361396f565b6127e88288612f15565b52016126d8565b61280391503d805f833e6107b58183612a0c565b8961279e565b6330c4716960e21b5f5260045ffd5b6343714afd60e01b5f5260045ffd5b63840a48d560e01b5f5260045ffd5b34610be65760a0366003190112610be657602061037c6128546128aa565b61285c6128c0565b906128656128d6565b6084359260643592612d1b565b34610be6575f366003190112610be657807f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad60209252f35b600435906001600160a01b0382168203610be657565b602435906001600160a01b0382168203610be657565b604435906001600160a01b0382168203610be657565b35906001600160a01b0382168203610be657565b9181601f84011215610be6578235916001600160401b038311610be6576020808501948460051b010111610be657565b60206040818301928281528451809452019201905f5b8181106129535750505090565b8251845260209384019390920191600101612946565b359063ffffffff82168203610be657565b9181601f84011215610be6578235916001600160401b038311610be65760208381860195010111610be657565b604081019081106001600160401b038211176129c257604052565b634e487b7160e01b5f52604160045260245ffd5b606081019081106001600160401b038211176129c257604052565b60e081019081106001600160401b038211176129c257604052565b90601f801991011681019081106001600160401b038211176129c257604052565b6001600160401b0381116129c25760051b60200190565b929190612a5081612a2d565b93612a5e6040519586612a0c565b602085838152019160051b8101928311610be657905b828210612a8057505050565b60208091612a8d846128ec565b815201910190612a74565b9080601f83011215610be657816020612ab393359101612a44565b90565b929190612ac281612a2d565b93612ad06040519586612a0c565b602085838152019160051b8101928311610be657905b828210612af257505050565b8135815260209182019101612ae6565b919060e083820312610be65760405190612b1b826129f1565b8193612b26816128ec565b8352612b34602082016128ec565b6020840152612b45604082016128ec565b604084015260608101356060840152612b6060808201612969565b608084015260a08101356001600160401b038111610be65782612b84918301612a98565b60a084015260c0810135906001600160401b038211610be6570181601f82011215610be65760c091816020612bbb93359101612ab6565b910152565b604435906001600160401b0382168203610be657565b90602080835192838152019201905f5b818110612bf35750505090565b82516001600160a01b0316845260209384019390920191600101612be6565b90602080835192838152019201905f5b818110612c2f5750505090565b8251845260209384019390920191600101612c22565b6001600160401b0381116129c257601f01601f191660200190565b9190604083820312610be65760405190612c79826129a7565b819380356001600160401b038111610be65781019082601f83011215610be657813592612ca584612c45565b90612cb36040519283612a0c565b84825260208585010111610be6575f602085819682809701838601378301015284520135910152565b906040600319830112610be6576004356001600160a01b0381168103610be65791602435906001600160401b038211610be657612ab391600401612a98565b9192612ab394916040519360208501957f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad875260018060a01b0316604086015260018060a01b0316606085015260018060a01b0316608084015260a083015260c082015260c08152611de560e082612a0c565b90612d9882612a2d565b612da56040519182612a0c565b8281528092612db6601f1991612a2d565b0190602036910137565b9190811015612de25760051b81013590605e1981360301821215610be6570190565b634e487b7160e01b5f52603260045260245ffd5b903590601e1981360301821215610be657018035906001600160401b038211610be657602001918160051b36038313610be657565b356001600160a01b0381168103610be65790565b602081830312610be6578051906001600160401b038211610be657019080601f83011215610be657815190612e7382612a2d565b92612e816040519485612a0c565b82845260208085019360051b820101918211610be657602001915b818310612ea95750505090565b82516001600160401b0381168103610be657815260209283019201612e9c565b916020908281520191905f5b818110612ee25750505090565b909192602080600192838060a01b03612efa886128ec565b168152019401929101612ed5565b805115612de25760200190565b8051821015612de25760209160051b010190565b90816020910312610be657516001600160a01b0381168103610be65790565b90816020910312610be657518015158103610be65790565b9063ffffffff8091169116019063ffffffff8211612f7a57565b634e487b7160e01b5f52601160045260245ffd5b91908201809211612f7a57565b63ffffffff811690633b9aca00821015612fd5575061c4e08101809111612f7a574310612fc6575f90565b6378f67ae160e11b5f5260045ffd5b90507f00000000000000000000000000000000000000000000000000000000000000006130028183612f60565b63ffffffff42911611612fc657612ab391612f60565b6001600160a01b039091168152604060208201819052612ab392910190612bd6565b90918060409360208452816020850152848401375f828201840152601f01601f1916010190565b919060e081019060018060a01b03845116815260018060a01b03602085015116602082015260018060a01b0360408501511660408201526060840151606082015263ffffffff608085015116608082015260a08401519160e060a08301528251809152602061010083019301905f5b8181106130f15750505060c0612ab3939401519060c0818403910152612c12565b82516001600160a01b03168552602094850194909201916001016130d0565b60405161313b8161312d6020820194602086526040830190613061565b03601f198101835282612a0c565b51902090565b9060405161314e816129d6565b60406001600160401b036001839580548552015460ff81161515602085015260081c16910152565b91908203918211612f7a57565b6040516394f649dd60e01b81526001600160a01b03918216600482018190529092915f90849060249082907f0000000000000000000000000000000000000000000000000000000000000000165afa8015611e68575f935f9161332d575b5060405163fe243a1760e01b815260048101929092525f51602061495e5f395f51905f5260248301526020826044817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215611e68575f926132f9575b5081156132f457835160018101809111612f7a5761326790612d8e565b93805160018101809111612f7a5761327e90612d8e565b925f51602061495e5f395f51905f52613298835188612f15565b526132a4825185612f15565b525f5b81518110156132ee576001906001600160a01b036132c58285612f15565b51166132d18289612f15565b526132dc8185612f15565b516132e78287612f15565b52016132a7565b50505090565b919050565b9091506020813d602011613325575b8161331560209383612a0c565b81010312610be65751905f61324a565b3d9150613308565b9350503d805f853e61333f8185612a0c565b830192604081850312610be65780516001600160401b038111610be65781019084601f83011215610be657815161337581612a2d565b926133836040519485612a0c565b81845260208085019260051b82010190878211610be657602001915b81831061341f575050506020810151906001600160401b038211610be657019380601f86011215610be65784516133d581612a2d565b956133e36040519788612a0c565b81875260208088019260051b820101928311610be657602001905b82821061340f57505050925f6131e1565b81518152602091820191016133fe565b82516001600160a01b0381168103610be65781526020928301920161339f565b6001600160a01b03168015159081613455575090565b5f818152609a60205260409020546001600160a01b031614919050565b9061347d8151612d8e565b905f5b81518110156134d3576001600160a01b038481165f908152609860205260409020600192916134af8386612f15565b5116838060a01b03165f5260205260405f20546134cc8286612f15565b5201613480565b5050905090565b156134e157565b60405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606490fd5b9190811015612de25760051b0190565b919092612ab3936040519260208401947f39111bc4a4d688e1f685123d7497d4615370152a8ee4a0593e647bd06ad8bb0b865260018060a01b0316604085015260018060a01b03166060840152608083015260a082015260a08152611de560c082612a0c565b6001600160a01b038082165f908152609a6020526040902054161561390b576135c48161343f565b6138fc576001600160a01b038116908115612403575f828152609a60205260409020546001600160a01b03169233831480159190826138f3575b80156138d3575b156138c45761361383613183565b939092855f52609a60205260405f206bffffffffffffffffffffffff60a01b815416905586867ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af446765f80a3613899575b8251801561388d5761367390612d8e565b916040519463547afb8760e01b86525f8680613693888c60048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa958615611e68575f96613871575b50909660409560209291905f5b8751811015613864576001906137ce86868b61379f8f8f8051936137008286612a0c565b8985528636602087013761375c8983519761371b858a612a0c565b8c8952893660208b0137845199613732868c612a0c565b8d8b523660208c01378c8060a01b0361374b8386612f15565b511661375689612f08565b52612f15565b51925f5260a2602052815f20906137798a8c8060a01b0392612f15565b51168a8060a01b03165f526020525f2090610b946001600160401b03610b8c8a8d612f15565b6137a883612f08565b526001600160401b036137bb8689612f15565b51166137c684612f08565b52878b61396f565b6137d8828a612f15565b528a5f5260a2602052895f20828060a01b036137f4838c612f15565b5116838060a01b03165f52602052670de0b6b3a76400008a5f20557f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f60608c8b61384485878060a01b0392612f15565b51168d519182526020820152670de0b6b3a76400008d820152a1016136dc565b5050505050509250505090565b6138869196503d805f833e6107b58183612a0c565b945f6136cf565b50509350505050606090565b85857ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a5f80a3613662565b631e499a2360e11b5f5260045ffd5b50845f52609960205260018060a01b03600160405f200154163314613605565b508433146135fe565b6311ca333560e31b5f5260045ffd5b63a5c7c44560e01b5f5260045ffd5b613922614452565b9060405190602082019261190160f01b8452602283015260428201526042815261313b606282612a0c565b6001600160a01b03918216815291166020820152604081019190915260600190565b90949390926001600160a01b038416929190831561240357825115613d08576139988351612d8e565b915f5b8451811015613c3b576139b96001600160a01b03610ad68388612f15565b5f87815260a2602052604090209091906001600160a01b036139db8389612f15565b511660018060a01b03165f526020526139f660405f20613141565b91613a4c613a048387612f15565b51613a47613a2f6001600160401b03613a1d878a612f15565b511692613a2988614594565b90614781565b6001600160401b03613a40886145a7565b1690614781565b614781565b6001600160a01b03918216939091613aa690602090613a6b868c612f15565b51168c604051938492839263fe243a1760e01b845260048401909291602090604083019460018060a01b0316835260018060a01b0316910152565b0381885afa908115611e68575f91613c0a575b508211613bfb57613b01906001600160a01b038d1680613b77575b50613a47613ae28589612f15565b516001600160401b03613a4081613af9898c612f15565b5116946145a7565b613b0b8388612f15565b526001600160a01b03613b1e8389612f15565b511690833b15610be657613b4d935f92838c6040519788958694859363724af42360e01b85526004850161394d565b03925af1918215611e6857600192613b67575b500161399b565b5f613b7191612a0c565b5f613b60565b7f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd6001600160a01b03613baa878d612f15565b51168d613bf2613bba898d612f15565b51855f52609860205260405f2060018060a01b0385165f5260205260405f20613be4828254613176565b90556040519384938461394d565b0390a25f613ad4565b63f020e5b960e01b5f5260045ffd5b90506020813d8211613c33575b81613c2460209383612a0c565b81010312610be657515f613ab9565b3d9150613c17565b5050505f838152609f60205260409020805495969591945091929091825f198114612f7a57600101905560405194613c72866129f1565b8186526001600160a01b03166020860152604085015260608401524263ffffffff16608084015260a083015260c08201527f58883f0678ff43a2c049e6a3a7a8b9b0e9062959f3a99192505888193a0c5fed613d02613cd083613110565b92835f52609e60205260405f20600160ff19825416179055604051918291858352604060208401526040830190613061565b0390a190565b63796cc52560e01b5f5260045ffd5b6001600160a01b0316801561240357606554604080516001600160a01b0383168152602081018490527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb69190a16001600160a01b03191617606555565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b92613e7d7f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f95613e8a93949560018060a01b0316805f52609860205260405f2060018060a01b0388165f5260205260405f20613e19858254612f8e565b90557f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c60405180613e4c878b8b8461394d565b0390a26001600160a01b038581165f90815260a260209081526040808320938a16835292905290812093908461450f565b546040519384938461394d565b0390a1565b6001600160a01b039081165f9081526099602052604090206004359182168203610be65780546001600160a01b0319166001600160a01b03928316178155600101906024359081168103610be65781546001600160a01b0319166001600160a01b039190911617815560443563ffffffff81168103610be657815463ffffffff60a01b191660a09190911b63ffffffff60a01b1617905560405160608101907ffebe5cd24b2cbc7b065b9d0fdeb904461e4afcff57dd57acda1e7832031ba7ac906001600160a01b03613f6260046128ec565b1681526001600160a01b03613f7760246128ec565b16602082015263ffffffff613f8c6044612969565b1660408201528033930390a2565b613fce612ab393926001600160401b03613fc7613fc18295613fbb85614594565b906146fd565b926145a7565b16906146fd565b9116906146fd565b6033546001600160a01b03163303613fea57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b9061403992916145ce565b1561404057565b638baa579f60e01b5f5260045ffd5b9193929360a08301906140628285612df6565b90508103612818576001600160a01b0361407e60408601612e2b565b1633036143cd5761409261198b3686612b02565b94855f52609e60205260ff60405f205416156143be57608085013563ffffffff81168103610be6576140c390612f9b565b905f6140d160208801612e2b565b6140db8689612df6565b60405163843b349f60e01b81526001600160a01b0390931660048401526060602484015291948592839263ffffffff9161411a91606486019190612ec9565b9116604483015203817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215611e68575f926143a2575b505f5b61416a8588612df6565b905081101561435a5761419761419261418d83614187898c612df6565b90613526565b612e2b565b6143dc565b908761421385610e476141b18561418760c0870187612df6565b356001600160a01b036141c386612e2b565b165f5260a26020528a6141e161418d8861418760405f20948a612df6565b60018060a01b03165f526020526001600160401b03613fc761179c8261420b8a60405f2098612f15565b511695613141565b928b156142be576001600160a01b031661423d61418d846141878b61423787612e2b565b96612df6565b9061424c61418d858a8d613526565b91813b15610be657604051630bab906360e21b81526001600160a01b039485166004820152908416602482015291909216604482015260648101939093525f908390608490829084905af1918215611e68576001926142ae575b505b01614160565b5f6142b891612a0c565b5f6142a6565b6001600160a01b03166142db61418d846141878b61423787612e2b565b906142ea61418d858a8d613526565b91813b15610be65760405163c4623ea160e01b81526001600160a01b039485166004820152908416602482015291909216604482015260648101939093525f908390608490829084905af1918215611e685760019261434a575b506142a8565b5f61435491612a0c565b5f614344565b509650505050505060207f1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a0091805f52609e825260405f2060ff198154169055604051908152a1565b6143b79192503d805f833e6107b58183612a0c565b905f61415d565b6387c9d21960e01b5f5260045ffd5b6316110d3560e21b5f5260045ffd5b6001600160a01b03165f51602061495e5f395f51905f5203614425577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b467f00000000000000000000000000000000000000000000000000000000000000000361449d577f000000000000000000000000000000000000000000000000000000000000000090565b600a60206040516144af604082612a0c565b828152016922b4b3b2b72630bcb2b960b11b81522060405160208101917f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866835260408201524660608201523060808201526080815261313b60a082612a0c565b9291811561457057916001600160401b0361455061455893613a298661454a61456c986145458861453f8d613141565b87613f9a565b612f8e565b92612f8e565b911690614781565b6001600160401b03613a4061179c85613141565b9055565b505061456c906001600160401b036145508161458e61179c87613141565b16614833565b5180612ab35750670de0b6b3a764000090565b6020810151156145c157604001516001600160401b031690565b50670de0b6b3a764000090565b9190916145db828461484d565b60058110156146e9571590816146d3575b506146cb575f92602061464b6084869560405193849181830196630b135d3f60e11b88526024840152604060448401528051918291826064860152018484015e87838284010152601f801991011681010301601f198101835282612a0c565b51915afa3d156146c4573d61465f81612c45565b9061466d6040519283612a0c565b81523d5f602083013e5b816146b8575b81614686575090565b9050602081805181010312610be657602001516001600160e01b0319811690819003610be657630b135d3f60e11b1490565b8051602014915061467d565b6060614677565b505050600190565b6001600160a01b0383811691161490505f6145ec565b634e487b7160e01b5f52602160045260245ffd5b9091905f905f19848209908481029283808410930392808403931461476e5782670de0b6b3a7640000111561035e57507faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac106699394670de0b6b3a7640000910990828211900360ee1b910360121c170290565b505050670de0b6b3a76400009192500490565b5f19670de0b6b3a7640000820991670de0b6b3a76400008202918280851094039380850394146148125783821115610be657670de0b6b3a7640000829109600182190182168092046002816003021880820260020302808202600203028082026002030280820260020302808202600203028091026002030293600183805f03040190848311900302920304170290565b508092501561481f570490565b634e487b7160e01b5f52601260045260245ffd5b801561481f576ec097ce7bc90715b34b9f10000000000490565b81516041810361487957509061487591602082015190606060408401519301515f1a906148bb565b9091565b6040036148b25760406020830151920151918260ff1c91601b8301809311612f7a57614875936001600160ff1b03169260ff16906148bb565b50505f90600290565b907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116149525760ff1690601b82141580614947575b61493c576020935f93608093604051938452868401526040830152606082015282805260015afa15611e68575f516001600160a01b0381161561493457905f90565b505f90600190565b505050505f90600490565b50601c8214156148f2565b505050505f9060039056fe000000000000000000000000beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0a264697066735822122069e9bb878ed54371b68a7289ae55055f38a16128d5784836a696459cfabcfb4c64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01`\x80`@R4a\x025W`\x1FaL\x018\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x029W\x80\x84\x92`\xA0\x94`@R\x839\x81\x01\x03\x12a\x025W\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x025W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x025W`@\x82\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x025W``\x83\x01Q\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x84\x03a\x025W`\x80\x01Q\x93c\xFF\xFF\xFF\xFF\x85\x16\x85\x03a\x025W`\x80R`\xA0R`\xC0R`\xE0Ra\x01\0RFa\x01 R_T`\x08\x81\x90\x1C`\xFF\x16a\x01\xE0W`\xFF\x80\x82\x16\x10a\x01\xA6W[`@QaI\xB3\x90\x81a\x02N\x829`\x80Q\x81a\x15(\x01R`\xA0Q\x81\x81\x81a \x1C\x01R\x81\x81a\"j\x01R\x81\x81a1\xB0\x01RaD'\x01R`\xC0Q\x81\x81\x81a\x16\x0E\x01R\x81\x81a\x1E\x97\x01R\x81\x81a\"'\x01R\x81\x81a2\x12\x01RaC\xFA\x01R`\xE0Q\x81\x81\x81a\x07\x11\x01R\x81\x81a\t\xE5\x01R\x81\x81a\ng\x01R\x81\x81a\r\xB4\x01R\x81\x81a\x12\xBB\x01R\x81\x81a\x16\xB4\x01R\x81\x81a\x1B\x0C\x01R\x81\x81a \xE5\x01R\x81\x81a&\xB3\x01R\x81\x81a6\x97\x01RaA%\x01Ra\x01\0Q\x81\x81\x81a\x1A|\x01Ra/\xD9\x01Ra\x01 Q\x81aDU\x01Ra\x01@Q\x81aD{\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xD7V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x04\xA4\xF9y\x14a(rWP\x80c\x0B\x9FHz\x14a(6W\x80c\r\xD8\xDD\x02\x14a&OW\x80c\x10\xD6z/\x14a%\xB9W\x80c\x13d9\xDD\x14a$\xEFW\x80c\x15\xC4\xA2\x88\x14a$\xB5W\x80c\x17\x94\xBB<\x14a#\x17W\x80c\x1B\xBC\xE0\x91\x14a\"\xD1W\x80c)\xC7}O\x14a\"\x99W\x80c9\xB7\x0E8\x14a\"UW\x80c<e\x1C\xF2\x14a\x1F\xF4W\x80c<\xDE\xB5\xE0\x14a\x1F\xB1W\x80c>(9\x1D\x14a\x1FtW\x80cC7s\x82\x14a\x1F:W\x80cE|`p\x14a\x1E\xC6W\x80cFe\xBC\xDA\x14a\x1E\x82W\x80cIs\0`\x14a\x1A\xA0W\x80cJ_+]\x14a\x1A_W\x80cY\\jg\x14a\x19\x90W\x80cY{6\xDA\x14a\x19UW\x80cZ\xC8j\xB7\x14a\x19\x1FW\x80c\\\x97Z\xBB\x14a\x19\x01W\x80c]\x9A\xED#\x14a\x15\xE5W\x80ce\xDA\x12d\x14a\x15\xA4W\x80cf\xD5\xBA\x93\x14a\x15WW\x80ck:\xA7.\x14a\x15\x12W\x80cmp\xF7\xAE\x14a\x14\xE4W\x80cqP\x18\xA6\x14a\x14\x87W\x80cw\x8EU\xF3\x14a\x144W\x80c\x7FT\x80q\x14a\x11,W\x80c\x88o\x11\x95\x14a\x11\x03W\x80c\x8D\xA5\xCB[\x14a\x10\xDAW\x80c\x90\x04\x13G\x14a\x10\xABW\x80c\x91\x04\xC3\x19\x14a\x10\x83W\x80c\x945\xBBC\x14a\x0FNW\x80c\x99\xBE\x81\xC8\x14a\x0E\xDCW\x80c\xA1x\x84\x84\x14a\x0E\xA3W\x80c\xA5z\xB1\x0B\x14a\rsW\x80c\xB7\xF0n\xBE\x14a\rDW\x80c\xBBE\xFE\xF2\x14a\x0C\xFBW\x80c\xC5\xE4\x80\xDB\x14a\x0CXW\x80c\xC9KQ\x11\x14a\x0C$W\x80c\xC9x\xF7\xAC\x14a\n\x14W\x80c\xCA\x8A\xA7\xC7\x14a\t\xCFW\x80c\xCB\08{\x14a\t\xB2W\x80c\xCE\xBC\x04\xEF\x14a\t\x93W\x80c\xDA\x8B\xE8d\x14a\tAW\x80c\xE4\xCC?\x90\x14a\x08\x9BW\x80c\xEE\xA9\x06K\x14a\x05\xF8W\x80c\xF0\xE0\xE6v\x14a\x04MW\x80c\xF1ar\xB0\x14a\x04\x15W\x80c\xF2\xFD\xE3\x8B\x14a\x03\x84W\x80c\xF6\x98\xDA%\x14a\x03aWc\xFA\xBC\x1C\xBC\x14a\x02pW_\x80\xFD[4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`eT`@Qcu[6\xBD`\xE1\x1B\x81R`\x04\x805\x92` \x91\x83\x91\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03SW\x83\x91a\x03$W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x03\x15W`fT\x19\x81\x19\x81\x16\x03a\x03\x06W\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\x80\xF3[c\xC6\x1D\xCA]`\xE0\x1B\x82R`\x04\x82\xFD[cyH!\xFF`\xE0\x1B\x82R`\x04\x82\xFD[a\x03F\x91P` =` \x11a\x03LW[a\x03>\x81\x83a*\x0CV[\x81\x01\x90a/)V[_a\x02\xB6V[P=a\x034V[`@Q=\x85\x82>=\x90\xFD[\x80\xFD[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` a\x03|aDRV[`@Q\x90\x81R\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^Wa\x03\x9Ea(\xAAV[a\x03\xA6a?\xD6V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x03\xC1Wa\x03\xBE\x90a=tV[\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P4a\x03^W``6`\x03\x19\x01\x12a\x03^Wa\x0403a4?V[\x15a\x04>Wa\x03\xBE3a>\x8FV[c%\xECl\x1F`\xE0\x1B\x81R`\x04\x90\xFD[P4a\x03^W`@6`\x03\x19\x01\x12a\x03^W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x05\xF4W6`#\x82\x01\x12\x15a\x05\xF4W\x80`\x04\x015\x90a\x04\x8A\x82a*-V[\x91a\x04\x98`@Q\x93\x84a*\x0CV[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x05\xF0W`$\x01\x90[\x82\x82\x10a\x05\xD8WPPP`$5`\x01`\x01`@\x1B\x03\x81\x11a\x05\xD4Wa\x04\xDE\x906\x90`\x04\x01a*\x98V[\x81Q\x91a\x04\xEA\x83a*-V[\x92a\x04\xF8`@Q\x94\x85a*\x0CV[\x80\x84Ra\x05\x07`\x1F\x19\x91a*-V[\x01\x84[\x81\x81\x10a\x05\xC3WPP\x83[\x81Q\x81\x10\x15a\x05ZW`\x01\x90a\x05>\x84`\x01`\x01`\xA0\x1B\x03a\x057\x84\x87a/\x15V[Q\x16a4rV[a\x05H\x82\x87a/\x15V[Ra\x05S\x81\x86a/\x15V[P\x01a\x05\x15V[PPP\x90`@Q\x91\x82\x91` \x83\x01` \x84R\x82Q\x80\x91R`@\x84\x01` `@\x83`\x05\x1B\x87\x01\x01\x94\x01\x92\x90[\x82\x82\x10a\x05\x94WPPPP\x03\x90\xF3[\x91\x93`\x01\x91\x93\x95P` a\x05\xB3\x81\x92`?\x19\x8A\x82\x03\x01\x86R\x88Qa,\x12V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x05\x85V[\x80``` \x80\x93\x88\x01\x01R\x01a\x05\nV[\x82\x80\xFD[` \x80\x91a\x05\xE5\x84a(\xECV[\x81R\x01\x91\x01\x90a\x04\xB5V[\x84\x80\xFD[P\x80\xFD[P4a\x03^W``6`\x03\x19\x01\x12a\x03^Wa\x06\x12a(\xAAV[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x05\xF4Wa\x062\x906\x90`\x04\x01a,`V[3_\x90\x81R`\x9A` R`@\x90 T`D5\x90`\x01`\x01`\xA0\x1B\x03\x16a\x08\x8CWa\x06[\x84a4?V[\x15a\x08}W`\x01\x80`fT\x16\x14a\x08nW`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x80\x85R`\x99` R`@\x85 `\x01\x01T\x90\x93\x91\x16\x90\x81\x15\x15\x80a\x08dW[\x80a\x08ZW[a\x07\xCFW[PP3\x80\x84R`\x9A` R`@\x84 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90U\x90P\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x83\x80\xA3a\x06\xEE3a1\x83V[`@QcTz\xFB\x87`\xE0\x1B\x81R\x91\x83\x83\x80a\x07\r\x84\x89`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\x07\xC4W\x84\x93a\x07\xA0W[P\x83[\x81Q\x81\x10\x15a\x07\x9CW`\x01\x90a\x07\x96`\x01`\x01`\xA0\x1B\x03a\x07m\x83\x86a/\x15V[Q\x16a\x07y\x83\x87a/\x15V[Q`\x01`\x01`@\x1B\x03a\x07\x8C\x85\x8Aa/\x15V[Q\x16\x913\x8Ba=\xBCV[\x01a\x07LV[\x84\x80\xF3[a\x07\xBD\x91\x93P=\x80\x86\x83>a\x07\xB5\x81\x83a*\x0CV[\x81\x01\x90a.?V[\x91_a\x07IV[`@Q=\x86\x82>=\x90\xFD[` \x81\x01\x80QB\x11a\x08KW\x82\x86R`\x9C` R`@\x86 \x84\x87R` R`\xFF`@\x87 T\x16a\x08<W\x90a\x08\x0Ba\x08\x14\x92Q\x85\x85\x8A3a-\x1BV[\x90Q\x90\x83a@.V[\x83R`\x9C` R`@\x83 \x90\x83R` R`@\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U_\x80\x80a\x06\xA0V[c\rLL\x91`\xE2\x1B\x86R`\x04\x86\xFD[c\x08\x19\xBD\xCD`\xE0\x1B\x86R`\x04\x86\xFD[P\x833\x14\x15a\x06\x9BV[P\x813\x14\x15a\x06\x95V[c\x84\nH\xD5`\xE0\x1B\x83R`\x04\x83\xFD[c%\xECl\x1F`\xE0\x1B\x83R`\x04\x83\xFD[c;\xF2\xB5\x03`\xE1\x1B\x83R`\x04\x83\xFD[P4a\x03^W``6`\x03\x19\x01\x12a\x03^W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03^W`\xE0`\x03\x19\x836\x03\x01\x12a\x03^W`$5`\x01`\x01`@\x1B\x03\x81\x11a\x05\xF4Wa\x08\xEC\x906\x90`\x04\x01a)\0V[`D5\x91\x82\x15\x15\x83\x03a\t=W`\x04\x80`fT\x16\x14a\t.Wa\t&\x93\x94a\t\x19`\x02`\xC9T\x14\x15a4\xDAV[`\x02`\xC9U`\x04\x01a@OV[`\x01`\xC9U\x80\xF3[c\x84\nH\xD5`\xE0\x1B\x84R`\x04\x84\xFD[\x83\x80\xFD[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^Wa\t[a(\xAAV[\x90`\x02\x80`fT\x16\x14a\t\x84Wa\t\x80a\tt\x83a5\x9CV[`@Q\x91\x82\x91\x82a)0V[\x03\x90\xF3[c\x84\nH\xD5`\xE0\x1B\x81R`\x04\x90\xFD[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` `@Qc;\x9A\xCA\0\x81R\xF3[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` `@Qa\xC4\xE0\x81R\xF3[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x03^Wa\n#6a,\xDCV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x80\x85R`\x9A` R`@\x80\x86 T\x90QcTz\xFB\x87`\xE0\x1B\x81R\x94\x95\x91\x94\x92\x16\x92\x90\x91\x90\x82\x82\x80a\nc\x84\x88`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03SW\x83\x92a\x0C\x08W[Pa\n\xAA\x81Qa-\x8EV[\x92\x80\x94\x15\x15\x94[\x82Q\x81\x10\x15a\x0B\xF2Wa\x0B)\x90` `\x01`\x01`\xA0\x1B\x03a\n\xDD\x81a\n\xD6\x85\x89a/\x15V[Q\x16aC\xDCV[\x16`\x01`\x01`\xA0\x1B\x03a\n\xF0\x84\x88a/\x15V[Q`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01R\x92\x90\x91\x16\x90\x91\x16`$\x82\x01R\x93\x84\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x80\x15a\x03SW\x83\x90a\x0B\xBCW[`\x01\x92P\x87\x15a\x0B\xACWa\x0B\x9A\x90\x89\x85R`\xA2` R`@\x85 \x84\x80`\xA0\x1B\x03a\x0Bd\x85\x89a/\x15V[Q\x16\x85\x80`\xA0\x1B\x03\x16_R` R`@_ \x90a\x0B\x94`\x01`\x01`@\x1B\x03a\x0B\x8C\x86\x8Ba/\x15V[Q\x16\x92a1AV[\x90a?\x9AV[a\x0B\xA4\x82\x88a/\x15V[R[\x01a\n\xB1V[a\x0B\xB6\x82\x88a/\x15V[Ra\x0B\xA6V[P` \x82=\x82\x11a\x0B\xEAW[\x81a\x0B\xD5` \x93\x83a*\x0CV[\x81\x01\x03\x12a\x0B\xE6W`\x01\x91Qa\x0B:V[_\x80\xFD[=\x91Pa\x0B\xC8V[`@Q` \x80\x82R\x81\x90a\t\x80\x90\x82\x01\x88a,\x12V[a\x0C\x1D\x91\x92P=\x80\x85\x83>a\x07\xB5\x81\x83a*\x0CV[\x90_a\n\x9FV[P4a\x03^W`\x806`\x03\x19\x01\x12a\x03^W` a\x03|a\x0CCa(\xAAV[a\x0CKa(\xD6V[`d5\x91`$5\x90a56V[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`@``\x91a\x0Cwa(\xAAV[\x81\x83\x80Qa\x0C\x84\x81a)\xD6V[\x82\x81R\x82` \x82\x01R\x01R`\x01\x80`\xA0\x1B\x03\x16\x81R`\x99` R c\xFF\xFF\xFF\xFF`@Qa\x0C\xB0\x81a)\xD6V[`\x01\x80\x80`\xA0\x1B\x03\x84T\x16\x93\x84\x83R\x01T\x90\x82`@` \x83\x01\x92`\x01\x80`\xA0\x1B\x03\x85\x16\x84R\x01\x92`\xA0\x1C\x16\x82R`@Q\x93\x84R`\x01\x80`\xA0\x1B\x03\x90Q\x16` \x84\x01RQ\x16`@\x82\x01R\xF3[P4a\x03^W`@6`\x03\x19\x01\x12a\x03^W` \x90`\xFF\x90`@\x90`\x01`\x01`\xA0\x1B\x03a\r&a(\xAAV[\x16\x81R`\x9C\x84R\x81\x81 `$5\x82R\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`\xFF`@` \x92`\x045\x81R`\x9E\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x03^W`\x806`\x03\x19\x01\x12a\x03^Wa\r\x8Da(\xAAV[a\r\x95a(\xC0V[a\r\x9Da+\xC0V[`d5\x92`\x01`\x01`@\x1B\x03\x84\x16\x80\x94\x03a\x05\xF0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0E\x94W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x92\x91a\x0ELa\x0ER\x92`\x01\x80`\xA0\x1B\x03\x16\x95\x86\x88R`\x98` R`@\x88 `\x01\x80`\xA0\x1B\x03\x86\x16_R` Ra\x0EG`\x01`\x01`@\x1B\x03`@_ T\x94\x16\x84aG\x81V[aF\xFDV[\x90a1vV[\x90\x83\x85R`\x98` R`@\x85 `\x01\x80`\xA0\x1B\x03\x82\x16_R` R`@_ a\x0E|\x83\x82Ta1vV[\x90Ua\x0E\x8E`@Q\x92\x83\x92\x87\x84a9MV[\x03\x90\xA2\x80\xF3[c#\xD8q\xA5`\xE0\x1B\x85R`\x04\x85\xFD[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W` \x90`@\x90`\x01`\x01`\xA0\x1B\x03a\x0E\xCBa(\xAAV[\x16\x81R`\x9F\x83R T`@Q\x90\x81R\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x05\xF4Wa\x0F\r\x906\x90`\x04\x01a)zV[a\x0F\x163a4?V[\x15a\x08}Wa\x0E\x8E\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x91`@Q\x91\x82\x913\x95\x83a0:V[P4a\x03^W``6`\x03\x19\x01\x12a\x03^W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x05\xF4Wa\x0F\x7F\x906\x90`\x04\x01a)\0V[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\t=Wa\x0F\x9F\x906\x90`\x04\x01a)\0V[\x92`D5`\x01`\x01`@\x1B\x03\x81\x11a\x10\x7FWa\x0F\xBF\x906\x90`\x04\x01a)\0V[\x93\x90\x92`\x04\x80`fT\x16\x14a\x10pWa\x0F\xE0`\x02`\xC9\x97\x94\x97T\x14\x15a4\xDAV[`\x02`\xC9U6\x82\x90\x03`\xDE\x19\x01\x92\x87[\x87\x81\x10\x15a\x10gW\x80`\x05\x1B\x90\x81\x85\x015\x91\x86\x83\x12\x15a\x10cW\x83\x82\x10\x15a\x10OWa\x10\x1E\x90\x85\x01\x85a-\xF6V[a\x10,\x83\x8B\x8B\x96\x94\x96a5&V[5\x90\x81\x15\x15\x82\x03a\x10KW`\x01\x94a\x10E\x93\x89\x01a@OV[\x01a\x0F\xF0V[\x8C\x80\xFD[cNH{q`\xE0\x1B\x8BR`2`\x04R`$\x8B\xFD[\x8A\x80\xFD[\x88`\x01`\xC9U\x80\xF3[c\x84\nH\xD5`\xE0\x1B\x87R`\x04\x87\xFD[\x85\x80\xFD[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` `@Q_Q` aI^_9_Q\x90_R\x81R\xF3[P4a\x03^Wa\t\x80a\x10\xC6a\x10\xC06a,\xDCV[\x90a4rV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a,\x12V[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x03^W`\xA06`\x03\x19\x01\x12a\x03^Wa\x11Fa(\xAAV[\x90a\x11Oa(\xC0V[\x91`D5`\x01`\x01`@\x1B\x03\x81\x11a\x05\xD4Wa\x11o\x906\x90`\x04\x01a,`V[`d5`\x01`\x01`@\x1B\x03\x81\x11a\t=Wa\x11\x8E\x906\x90`\x04\x01a,`V[\x90`\x845` \x82\x01\x80QB\x11a\x08KW`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x9A` R`@\x90 T\x16a\x14%Wa\x11\xC5\x87a4?V[\x15a\x14\x16W`\x01\x90a\x11\xFC\x82\x80`\xA0\x1B\x03\x87\x16\x94\x85\x89R`\x9B` Ra\x11\xF3`@\x8A T\x93Q\x8B\x85\x8Ba56V[\x90Q\x90\x88a@.V[\x83\x87R`\x9B` R\x01`@\x86 U`\x01\x80`fT\x16\x14a\x14\x07W`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x80\x87R`\x99` R`@\x87 `\x01\x01T\x90\x94\x91\x16\x90\x81\x15\x15\x80a\x13\xFDW[\x80a\x13\xF3W[a\x13qW[PPP\x80\x84R`\x9A` R`@\x84 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x83\x17\x90U\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x84\x80\xA3a\x12\x96\x81a1\x83V[`@QcTz\xFB\x87`\xE0\x1B\x81R\x92\x90\x91\x84\x84\x80a\x12\xB7\x84\x8A`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x93\x84\x15a\x13fW\x85\x94a\x13JW[P\x84[\x81Q\x81\x10\x15a\x13FW`\x01\x90a\x13@`\x01`\x01`\xA0\x1B\x03a\x13\x17\x83\x86a/\x15V[Q\x16a\x13#\x83\x88a/\x15V[Q`\x01`\x01`@\x1B\x03a\x136\x85\x8Ba/\x15V[Q\x16\x91\x87\x8Ca=\xBCV[\x01a\x12\xF6V[\x85\x80\xF3[a\x13_\x91\x94P=\x80\x87\x83>a\x07\xB5\x81\x83a*\x0CV[\x92_a\x12\xF3V[`@Q=\x87\x82>=\x90\xFD[` \x81\x01B\x81Q\x10a\x13\xE4W\x82\x88R`\x9C` R`@\x88 \x84\x89R` R`\xFF`@\x89 T\x16a\x13\xD5W\x90a\x08\x0Ba\x13\xAD\x92Q\x85\x85\x8C\x8Ba-\x1BV[\x85R`\x9C` R`@\x85 \x90\x85R` R`@\x84 `\x01`\xFF\x19\x82T\x16\x17\x90U_\x80\x80a\x12JV[c\rLL\x91`\xE2\x1B\x88R`\x04\x88\xFD[c\x08\x19\xBD\xCD`\xE0\x1B\x88R`\x04\x88\xFD[P\x843\x14\x15a\x12EV[P\x813\x14\x15a\x12?V[c\x84\nH\xD5`\xE0\x1B\x85R`\x04\x85\xFD[c%\xECl\x1F`\xE0\x1B\x86R`\x04\x86\xFD[c;\xF2\xB5\x03`\xE1\x1B\x86R`\x04\x86\xFD[P4a\x03^W`@6`\x03\x19\x01\x12a\x03^W`@a\x14Pa(\xAAV[\x91a\x14Ya(\xC0V[\x92`\x01\x80`\xA0\x1B\x03\x16\x81R`\x98` R \x90`\x01\x80`\xA0\x1B\x03\x16_R` R` `@_ T`@Q\x90\x81R\xF3[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^Wa\x14\xA0a?\xD6V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W` a\x15\x08a\x15\x03a(\xAAV[a4?V[`@Q\x90\x15\x15\x81R\xF3[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^Wa\x15\x96a\t\x80a\x15\x7Fa\x15za(\xAAV[a1\x83V[`@\x92\x91\x92Q\x93\x84\x93`@\x85R`@\x85\x01\x90a+\xD6V[\x90\x83\x82\x03` \x85\x01Ra,\x12V[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W` \x90`\x01`\x01`\xA0\x1B\x03a\x15\xC9a(\xAAV[\x16\x81R`\x9A\x82R`@`\x01\x80`\xA0\x1B\x03\x91 T\x16`@Q\x90\x81R\xF3[P4a\x03^W``6`\x03\x19\x01\x12a\x03^Wa\x15\xFFa(\xAAV[`$5\x90a\x16\x0Ba+\xC0V[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x18\xF2W`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x80\x86R`\x9A` R`@\x80\x87 T\x81Q\x93\x16\x95\x91\x93\x90\x92a\x16\xB0\x91\x90\x88\x90a\x16o\x86\x82a*\x0CV[`\x01\x81R`\x1F\x19\x86\x016` \x83\x017_Q` aI^_9_Q\x90_Ra\x16\x95\x82a/\x08V[R\x85Q\x80\x94\x81\x92cTz\xFB\x87`\xE0\x1B\x83R\x8B`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x18\xE8W\x91a\x18\x1D\x91a\x18>\x93\x8A\x92a\x18\xCCW[P\x86\x8AR`\xA2` R\x85\x8A _Q` aI^_9_Q\x90_R_R` R\x7F\xDD\xF95\xEC\x88%\xC7\xAF\xEEj\x15\xD4s\x1E(\x96>\xE9m\xFC\xB8]\n\x1EyKC1\x8B\xBC\xA4\xFD\x86a\x17^\x81_ a\x17X`\x01`\x01`@\x1B\x03a\x17P\x88a/\x08V[Q\x16\x91a1AV[\x85a?\x9AV[\x96\x89\x8DR`\xA2` R\x81\x8D _Q` aI^_9_Q\x90_R_R` R`\x01\x80a\x17\xAA\x84_ \x93`\x01`\x01`@\x1B\x03\x80a\x17\xA1a\x17\x9C\x88a1AV[aE\xA7V[\x16\x91\x16\x90aF\xFDV[\x92\x01\x91h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83T\x91`\x08\x1B\x16\x90h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x17\x80\x91U`\x01`\x01`@\x1B\x03\x82Q\x91\x8B\x83R`\x08\x1C\x16` \x82\x01R\xA1\x86\x8AR`\xA2` R\x85\x8A _Q` aI^_9_Q\x90_R_R` Ra\x0B\x94`\x01`\x01`@\x1B\x03a\x0B\x8C\x88_ \x94a/\x08V[`\x01`\x01`\xA0\x1B\x03\x95\x86\x16_\x90\x81R`\x9A` R`@\x90 T\x90\x95\x16\x15\x15\x90V[a\x18FW\x85\x80\xF3[\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x93a\x18q\x91a1vV[\x91\x84\x86R`\x98` R\x81\x86 _Q` aI^_9_Q\x90_R\x87R` R\x81\x86 a\x18\x9E\x84\x82Ta1vV[\x90U\x81Q\x90\x81R_Q` aI^_9_Q\x90_R` \x82\x01R\x90\x81\x01\x91\x90\x91R``\x90\xA2_\x80\x80\x80\x80\x85\x80\xF3[a\x18\xE1\x91\x92P=\x80\x8C\x83>a\x07\xB5\x81\x83a*\x0CV[\x90_a\x16\xF5V[\x84Q=\x8A\x82>=\x90\xFD[c2\x13\xA6a`\xE2\x1B\x84R`\x04\x84\xFD[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` `fT`@Q\x90\x81R\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`\x045`\xFF\x81\x16\x80\x91\x03a\x05\xF4W`\x01\x90` \x92P\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03^W` a\x03|a\x19\x8B6`\x04\x86\x01a+\x02V[a1\x10V[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x1ATW\x82\x91a\x1A%W[P\x15a\x1A\x16W_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\x80\xF3[c\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x90\xFD[a\x1AG\x91P` =` \x11a\x1AMW[a\x1A?\x81\x83a*\x0CV[\x81\x01\x90a/HV[_a\x19\xDBV[P=a\x1A5V[`@Q=\x84\x82>=\x90\xFD[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x0B\xE6W6`\x03\x19\x01`\xA0\x81\x12a\x0B\xE6W``\x13a\x0B\xE6W`d5c\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x0B\xE6W`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xE6Wa\x1A\xEA\x906\x90`\x04\x01a)zV[3_\x90\x81R`\x9A` R`@\x90 T\x90\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1EsW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x0B\xE6W`@Q\x90c+bA\xF3`\xE1\x1B\x82R3`\x04\x83\x01R`$\x82\x01R_\x81`D\x81\x83\x87Z\xF1\x80\x15a\x1EhWa\x1ESW[Pa\x1Bs3a>\x8FV[`@Qa\x1B\x7F\x81a)\xA7V[``\x81R` \x81\x01\x90\x85\x82R`\x01\x80`fT\x16\x14a\x1EDW3\x86R`\x99` R`@\x86 `\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15\x15\x80a\x1E:W[\x80a\x1E3W[a\x1D_W[PP3\x80\x86R`\x9A` R`@\x86 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x82\x17\x90U\x90P\x80\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x86\x80\xA3a\x1C\x143a1\x83V[\x94\x90\x91\x81`@Q\x80\x95cTz\xFB\x87`\xE0\x1B\x82R\x81\x80a\x1C7\x883`\x04\x84\x01a0\x18V[\x03\x91Z\xFA\x93\x84\x15a\x1ATW\x82\x94a\x1DCW[P\x81[\x83Q\x81\x10\x15a\x1C\x9CW`\x01\x90a\x1C\x96`\x01`\x01`\xA0\x1B\x03a\x1Cm\x83\x88a/\x15V[Q\x16a\x1Cy\x83\x8Ba/\x15V[Q`\x01`\x01`@\x1B\x03a\x1C\x8C\x85\x8Ba/\x15V[Q\x16\x9133a=\xBCV[\x01a\x1CLV[P`@Q\x85`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x05\xF0W\x82R`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x05\xF0W` \x83\x01R`D5\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x05\xF0W\x82a\x0E\x8E\x92`@\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x95\x01R\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2``3\x92\xA2`@Q\x91\x82\x913\x95\x83a0:V[a\x1DX\x91\x94P=\x80\x84\x83>a\x07\xB5\x81\x83a*\x0CV[\x92_a\x1CIV[\x80QB\x11a\x1E$W\x82\x87R`\x9C` R`@\x87 \x87\x80R` R`\xFF`@\x88 T\x16a\x1E\x15W\x90a\x08\x0Ba\x1D\xED\x92Q`@Q` \x81\x01\x91\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x83R\x86`@\x83\x01R3``\x83\x01R3`\x80\x83\x01R\x8A`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x1D\xE5`\xE0\x82a*\x0CV[Q\x90 a9\x1AV[\x84R`\x9C` R`@\x84 \x84\x80R` R`@\x84 `\x01`\xFF\x19\x82T\x16\x17\x90U_\x80\x80a\x1B\xC5V[c\rLL\x91`\xE2\x1B\x87R`\x04\x87\xFD[c\x08\x19\xBD\xCD`\xE0\x1B\x87R`\x04\x87\xFD[P\x86a\x1B\xC0V[P\x823\x14\x15a\x1B\xBAV[c\x84\nH\xD5`\xE0\x1B\x86R`\x04\x86\xFD[a\x1E`\x91\x94P_\x90a*\x0CV[_\x92_a\x1BiV[`@Q=_\x82>=\x90\xFD[c;\xF2\xB5\x03`\xE1\x1B_R`\x04_\xFD[4a\x0B\xE6W_6`\x03\x19\x01\x12a\x0B\xE6W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x0B\xE6W`@6`\x03\x19\x01\x12a\x0B\xE6Wa\x1E\xDFa(\xAAV[a\x1E\xE7a(\xC0V[\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R```@_ `\x01`\x01`@\x1B\x03`\x01\x82T\x92\x01T`@Q\x92\x83R`\xFF\x81\x16\x15\x15` \x84\x01R`\x08\x1C\x16`@\x82\x01R\xF3[4a\x0B\xE6W_6`\x03\x19\x01\x12a\x0B\xE6W` `@Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81R\xF3[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W` a\x15\x08a\x1F\x92a(\xAAV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W`\x01`\x01`\xA0\x1B\x03a\x1F\xD2a(\xAAV[\x16_R`\x99` R` `\x01\x80`\xA0\x1B\x03`\x01`@_ \x01T\x16`@Q\x90\x81R\xF3[4a\x0B\xE6W`\x806`\x03\x19\x01\x12a\x0B\xE6Wa \ra(\xAAV[a \x15a(\xC0V[`d5\x913\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\"#W[\x15a\"\x14W`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x9A` R`@\x90 T\x16a qW\0[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x9A` R`@\x80\x82 T\x81Q\x91\x97\x94\x16\x92\x91a \xE1\x91a \xA1\x89\x82a*\x0CV[`\x01\x81R`\x1F\x19\x89\x016` \x83\x017a \xB9\x81a/\x08V[`\x01`\x01`\xA0\x1B\x03\x89\x16\x90R\x88QcTz\xFB\x87`\xE0\x1B\x81R\x92\x83\x91\x82\x91\x90\x87`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\"\nW\x92a!\xEB\x94\x92`\x01`\x01`@\x1B\x03a![a!\xDF\x94\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x9B\x97_\x91a!\xF0W[Pa/\x08V[Q\x16\x91\x80_R`\x98` R\x85_ `\x01\x80`\xA0\x1B\x03\x8A\x16_R` R\x85_ a!\x85\x86\x82Ta/\x8EV[\x90U\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x86Q\x80a!\xB7\x88\x8D\x8D\x84a9MV[\x03\x90\xA2_R`\xA2` R\x83_ `\x01\x80`\xA0\x1B\x03\x88\x16_R` R\x83_ \x92`D5\x84aE\x0FV[T\x90Q\x93\x84\x93\x84a9MV[\x03\x90\xA1\0[a\"\x04\x91P=\x80_\x83>a\x07\xB5\x81\x83a*\x0CV[\x8Ca!UV[\x87Q=_\x82>=\x90\xFD[c\x04R\x06\xA5`\xE2\x1B_R`\x04_\xFD[P3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a LV[4a\x0B\xE6W_6`\x03\x19\x01\x12a\x0B\xE6W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W`\x01`\x01`\xA0\x1B\x03a\"\xBAa(\xAAV[\x16_R`\x9B` R` `@_ T`@Q\x90\x81R\xF3[4a\x0B\xE6W``6`\x03\x19\x01\x12a\x0B\xE6W` a\x03|a\"\xEFa(\xAAV[a\"\xF7a(\xC0V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9B\x85R`@\x90 T`D5\x92a56V[4a\x0B\xE6W``6`\x03\x19\x01\x12a\x0B\xE6Wa#0a(\xAAV[a#8a(\xC0V[`D5\x91_T\x92`\xFF\x84`\x08\x1C\x16\x15\x93\x84\x80\x95a$\xA8W[\x80\x15a$\x91W[\x15a$5W`\xFF\x19\x81\x16`\x01\x17_U\x84a$$W[P`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a$\x12W[\x15a$\x03Wa#\xC7\x92\x81a#\xC2\x92`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2a=\x17V[a=tV[a#\xCDW\0[a\xFF\0\x19_T\x16_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\0[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x15a#\x80V[a\xFF\xFF\x19\x16a\x01\x01\x17_U\x84a#lV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a#WWP`\x01`\xFF\x82\x16\x14a#WV[P`\x01`\xFF\x82\x16\x10a#PV[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W`\x045c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x0B\xE6Wa$\xE1` \x91a/\x9BV[c\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W`\x045`$` `\x01\x80`\xA0\x1B\x03`eT\x16`@Q\x92\x83\x80\x92c#}\xFBG`\xE1\x1B\x82R3`\x04\x83\x01RZ\xFA\x90\x81\x15a\x1EhW_\x91a%\x9AW[P\x15a%\x8BW`fT\x81\x81\x16\x03a%|W\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[a%\xB3\x91P` =` \x11a\x1AMWa\x1A?\x81\x83a*\x0CV[\x82a%9V[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6Wa%\xD2a(\xAAV[`eT`@Qcu[6\xBD`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x1EhW_\x91a&0W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a&!Wa&\x1F\x90a=\x17V[\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a&I\x91P` =` \x11a\x03LWa\x03>\x81\x83a*\x0CV[\x82a&\x06V[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xE6Wa&\x7F\x906\x90`\x04\x01a)\0V[`\x02\x80`fT\x16\x14a('Wa&\x94\x81a-\x8EV[3_\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x92\x93\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x92\x91[\x81\x81\x10a&\xEDW`@Q\x80a\t\x80\x87\x82a)0V[a'\x01a&\xFB\x82\x84\x86a-\xC0V[\x80a-\xF6V[\x90Pa'\x1Ba'\x11\x83\x85\x87a-\xC0V[` \x81\x01\x90a-\xF6V[\x91\x90P\x03a(\x18W3`\x01`\x01`\xA0\x1B\x03a'B`@a'<\x85\x87\x89a-\xC0V[\x01a.+V[\x16\x03a(\tW\x80_a'[a&\xFBa'\x84\x94\x86\x88a-\xC0V[`@Q\x94\x85\x92\x83\x92cTz\xFB\x87`\xE0\x1B\x84R\x8C`\x04\x85\x01R`@`$\x85\x01R`D\x84\x01\x91a.\xC9V[\x03\x81\x88Z\xFA\x91\x82\x15a\x1EhW`\x01\x92a'\xDE\x91_\x91a'\xEFW[Pa'\xD6a'\xB0a&\xFB\x85\x88\x8Aa-\xC0V[a'\xCEa'\xC4a'\x11\x88\x8B\x8D\x97\x96\x97a-\xC0V[\x94\x90\x926\x91a*DV[\x926\x91a*\xB6V[\x90\x8A3a9oV[a'\xE8\x82\x88a/\x15V[R\x01a&\xD8V[a(\x03\x91P=\x80_\x83>a\x07\xB5\x81\x83a*\x0CV[\x89a'\x9EV[c0\xC4qi`\xE2\x1B_R`\x04_\xFD[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[4a\x0B\xE6W`\xA06`\x03\x19\x01\x12a\x0B\xE6W` a\x03|a(Ta(\xAAV[a(\\a(\xC0V[\x90a(ea(\xD6V[`\x845\x92`d5\x92a-\x1BV[4a\x0B\xE6W_6`\x03\x19\x01\x12a\x0B\xE6W\x80\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x92R\xF3[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xE6WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xE6WV[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xE6WV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xE6WV[\x91\x81`\x1F\x84\x01\x12\x15a\x0B\xE6W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x0B\xE6W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x0B\xE6WV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a)SWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a)FV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x0B\xE6WV[\x91\x81`\x1F\x84\x01\x12\x15a\x0B\xE6W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x0B\xE6W` \x83\x81\x86\x01\x95\x01\x01\x11a\x0B\xE6WV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a)\xC2W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a)\xC2W`@RV[`\xE0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a)\xC2W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a)\xC2W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a)\xC2W`\x05\x1B` \x01\x90V[\x92\x91\x90a*P\x81a*-V[\x93a*^`@Q\x95\x86a*\x0CV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x0B\xE6W\x90[\x82\x82\x10a*\x80WPPPV[` \x80\x91a*\x8D\x84a(\xECV[\x81R\x01\x91\x01\x90a*tV[\x90\x80`\x1F\x83\x01\x12\x15a\x0B\xE6W\x81` a*\xB3\x935\x91\x01a*DV[\x90V[\x92\x91\x90a*\xC2\x81a*-V[\x93a*\xD0`@Q\x95\x86a*\x0CV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x0B\xE6W\x90[\x82\x82\x10a*\xF2WPPPV[\x815\x81R` \x91\x82\x01\x91\x01a*\xE6V[\x91\x90`\xE0\x83\x82\x03\x12a\x0B\xE6W`@Q\x90a+\x1B\x82a)\xF1V[\x81\x93a+&\x81a(\xECV[\x83Ra+4` \x82\x01a(\xECV[` \x84\x01Ra+E`@\x82\x01a(\xECV[`@\x84\x01R``\x81\x015``\x84\x01Ra+``\x80\x82\x01a)iV[`\x80\x84\x01R`\xA0\x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xE6W\x82a+\x84\x91\x83\x01a*\x98V[`\xA0\x84\x01R`\xC0\x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x0B\xE6W\x01\x81`\x1F\x82\x01\x12\x15a\x0B\xE6W`\xC0\x91\x81` a+\xBB\x935\x91\x01a*\xB6V[\x91\x01RV[`D5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x0B\xE6WV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a+\xF3WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a+\xE6V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a,/WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a,\"V[`\x01`\x01`@\x1B\x03\x81\x11a)\xC2W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x91\x90`@\x83\x82\x03\x12a\x0B\xE6W`@Q\x90a,y\x82a)\xA7V[\x81\x93\x805`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xE6W\x81\x01\x90\x82`\x1F\x83\x01\x12\x15a\x0B\xE6W\x815\x92a,\xA5\x84a,EV[\x90a,\xB3`@Q\x92\x83a*\x0CV[\x84\x82R` \x85\x85\x01\x01\x11a\x0B\xE6W_` \x85\x81\x96\x82\x80\x97\x01\x83\x86\x017\x83\x01\x01R\x84R\x015\x91\x01RV[\x90`@`\x03\x19\x83\x01\x12a\x0B\xE6W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x0B\xE6W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x0B\xE6Wa*\xB3\x91`\x04\x01a*\x98V[\x91\x92a*\xB3\x94\x91`@Q\x93` \x85\x01\x95\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x87R`\x01\x80`\xA0\x1B\x03\x16`@\x86\x01R`\x01\x80`\xA0\x1B\x03\x16``\x85\x01R`\x01\x80`\xA0\x1B\x03\x16`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x1D\xE5`\xE0\x82a*\x0CV[\x90a-\x98\x82a*-V[a-\xA5`@Q\x91\x82a*\x0CV[\x82\x81R\x80\x92a-\xB6`\x1F\x19\x91a*-V[\x01\x90` 6\x91\x017V[\x91\x90\x81\x10\x15a-\xE2W`\x05\x1B\x81\x015\x90`^\x19\x816\x03\x01\x82\x12\x15a\x0B\xE6W\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x0B\xE6W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x0B\xE6W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x0B\xE6WV[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x0B\xE6W\x90V[` \x81\x83\x03\x12a\x0B\xE6W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x0B\xE6W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x0B\xE6W\x81Q\x90a.s\x82a*-V[\x92a.\x81`@Q\x94\x85a*\x0CV[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a\x0B\xE6W` \x01\x91[\x81\x83\x10a.\xA9WPPP\x90V[\x82Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x0B\xE6W\x81R` \x92\x83\x01\x92\x01a.\x9CV[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a.\xE2WPPP\x90V[\x90\x91\x92` \x80`\x01\x92\x83\x80`\xA0\x1B\x03a.\xFA\x88a(\xECV[\x16\x81R\x01\x94\x01\x92\x91\x01a.\xD5V[\x80Q\x15a-\xE2W` \x01\x90V[\x80Q\x82\x10\x15a-\xE2W` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x0B\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x0B\xE6W\x90V[\x90\x81` \x91\x03\x12a\x0B\xE6WQ\x80\x15\x15\x81\x03a\x0B\xE6W\x90V[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a/zWV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x91\x90\x82\x01\x80\x92\x11a/zWV[c\xFF\xFF\xFF\xFF\x81\x16\x90c;\x9A\xCA\0\x82\x10\x15a/\xD5WPa\xC4\xE0\x81\x01\x80\x91\x11a/zWC\x10a/\xC6W_\x90V[cx\xF6z\xE1`\xE1\x1B_R`\x04_\xFD[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a0\x02\x81\x83a/`V[c\xFF\xFF\xFF\xFFB\x91\x16\x11a/\xC6Wa*\xB3\x91a/`V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra*\xB3\x92\x91\x01\x90a+\xD6V[\x90\x91\x80`@\x93` \x84R\x81` \x85\x01R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90`\xE0\x81\x01\x90`\x01\x80`\xA0\x1B\x03\x84Q\x16\x81R`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03`@\x85\x01Q\x16`@\x82\x01R``\x84\x01Q``\x82\x01Rc\xFF\xFF\xFF\xFF`\x80\x85\x01Q\x16`\x80\x82\x01R`\xA0\x84\x01Q\x91`\xE0`\xA0\x83\x01R\x82Q\x80\x91R` a\x01\0\x83\x01\x93\x01\x90_[\x81\x81\x10a0\xF1WPPP`\xC0a*\xB3\x93\x94\x01Q\x90`\xC0\x81\x84\x03\x91\x01Ra,\x12V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a0\xD0V[`@Qa1;\x81a1-` \x82\x01\x94` \x86R`@\x83\x01\x90a0aV[\x03`\x1F\x19\x81\x01\x83R\x82a*\x0CV[Q\x90 \x90V[\x90`@Qa1N\x81a)\xD6V[`@`\x01`\x01`@\x1B\x03`\x01\x83\x95\x80T\x85R\x01T`\xFF\x81\x16\x15\x15` \x85\x01R`\x08\x1C\x16\x91\x01RV[\x91\x90\x82\x03\x91\x82\x11a/zWV[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01\x81\x90R\x90\x92\x91_\x90\x84\x90`$\x90\x82\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x1EhW_\x93_\x91a3-W[P`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R_Q` aI^_9_Q\x90_R`$\x83\x01R` \x82`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x1EhW_\x92a2\xF9W[P\x81\x15a2\xF4W\x83Q`\x01\x81\x01\x80\x91\x11a/zWa2g\x90a-\x8EV[\x93\x80Q`\x01\x81\x01\x80\x91\x11a/zWa2~\x90a-\x8EV[\x92_Q` aI^_9_Q\x90_Ra2\x98\x83Q\x88a/\x15V[Ra2\xA4\x82Q\x85a/\x15V[R_[\x81Q\x81\x10\x15a2\xEEW`\x01\x90`\x01`\x01`\xA0\x1B\x03a2\xC5\x82\x85a/\x15V[Q\x16a2\xD1\x82\x89a/\x15V[Ra2\xDC\x81\x85a/\x15V[Qa2\xE7\x82\x87a/\x15V[R\x01a2\xA7V[PPP\x90V[\x91\x90PV[\x90\x91P` \x81=` \x11a3%W[\x81a3\x15` \x93\x83a*\x0CV[\x81\x01\x03\x12a\x0B\xE6WQ\x90_a2JV[=\x91Pa3\x08V[\x93PP=\x80_\x85>a3?\x81\x85a*\x0CV[\x83\x01\x92`@\x81\x85\x03\x12a\x0B\xE6W\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xE6W\x81\x01\x90\x84`\x1F\x83\x01\x12\x15a\x0B\xE6W\x81Qa3u\x81a*-V[\x92a3\x83`@Q\x94\x85a*\x0CV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x90\x87\x82\x11a\x0B\xE6W` \x01\x91[\x81\x83\x10a4\x1FWPPP` \x81\x01Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x0B\xE6W\x01\x93\x80`\x1F\x86\x01\x12\x15a\x0B\xE6W\x84Qa3\xD5\x81a*-V[\x95a3\xE3`@Q\x97\x88a*\x0CV[\x81\x87R` \x80\x88\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x0B\xE6W` \x01\x90[\x82\x82\x10a4\x0FWPPP\x92_a1\xE1V[\x81Q\x81R` \x91\x82\x01\x91\x01a3\xFEV[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x0B\xE6W\x81R` \x92\x83\x01\x92\x01a3\x9FV[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x15\x90\x81a4UWP\x90V[_\x81\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14\x91\x90PV[\x90a4}\x81Qa-\x8EV[\x90_[\x81Q\x81\x10\x15a4\xD3W`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x90\x81R`\x98` R`@\x90 `\x01\x92\x91a4\xAF\x83\x86a/\x15V[Q\x16\x83\x80`\xA0\x1B\x03\x16_R` R`@_ Ta4\xCC\x82\x86a/\x15V[R\x01a4\x80V[PP\x90P\x90V[\x15a4\xE1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x90\xFD[\x91\x90\x81\x10\x15a-\xE2W`\x05\x1B\x01\x90V[\x91\x90\x92a*\xB3\x93`@Q\x92` \x84\x01\x94\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x86R`\x01\x80`\xA0\x1B\x03\x16`@\x85\x01R`\x01\x80`\xA0\x1B\x03\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xA0\x81Ra\x1D\xE5`\xC0\x82a*\x0CV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15a9\x0BWa5\xC4\x81a4?V[a8\xFCW`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a$\x03W_\x82\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x923\x83\x14\x80\x15\x91\x90\x82a8\xF3W[\x80\x15a8\xD3W[\x15a8\xC4Wa6\x13\x83a1\x83V[\x93\x90\x92\x85_R`\x9A` R`@_ k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x81T\x16\x90U\x86\x86\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv_\x80\xA3a8\x99W[\x82Q\x80\x15a8\x8DWa6s\x90a-\x8EV[\x91`@Q\x94cTz\xFB\x87`\xE0\x1B\x86R_\x86\x80a6\x93\x88\x8C`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x95\x86\x15a\x1EhW_\x96a8qW[P\x90\x96`@\x95` \x92\x91\x90_[\x87Q\x81\x10\x15a8dW`\x01\x90a7\xCE\x86\x86\x8Ba7\x9F\x8F\x8F\x80Q\x93a7\0\x82\x86a*\x0CV[\x89\x85R\x866` \x87\x017a7\\\x89\x83Q\x97a7\x1B\x85\x8Aa*\x0CV[\x8C\x89R\x896` \x8B\x017\x84Q\x99a72\x86\x8Ca*\x0CV[\x8D\x8BR6` \x8C\x017\x8C\x80`\xA0\x1B\x03a7K\x83\x86a/\x15V[Q\x16a7V\x89a/\x08V[Ra/\x15V[Q\x92_R`\xA2` R\x81_ \x90a7y\x8A\x8C\x80`\xA0\x1B\x03\x92a/\x15V[Q\x16\x8A\x80`\xA0\x1B\x03\x16_R` R_ \x90a\x0B\x94`\x01`\x01`@\x1B\x03a\x0B\x8C\x8A\x8Da/\x15V[a7\xA8\x83a/\x08V[R`\x01`\x01`@\x1B\x03a7\xBB\x86\x89a/\x15V[Q\x16a7\xC6\x84a/\x08V[R\x87\x8Ba9oV[a7\xD8\x82\x8Aa/\x15V[R\x8A_R`\xA2` R\x89_ \x82\x80`\xA0\x1B\x03a7\xF4\x83\x8Ca/\x15V[Q\x16\x83\x80`\xA0\x1B\x03\x16_R` Rg\r\xE0\xB6\xB3\xA7d\0\0\x8A_ U\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F``\x8C\x8Ba8D\x85\x87\x80`\xA0\x1B\x03\x92a/\x15V[Q\x16\x8DQ\x91\x82R` \x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x8D\x82\x01R\xA1\x01a6\xDCV[PPPPPP\x92PPP\x90V[a8\x86\x91\x96P=\x80_\x83>a\x07\xB5\x81\x83a*\x0CV[\x94_a6\xCFV[PP\x93PPPP``\x90V[\x85\x85\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A_\x80\xA3a6bV[c\x1EI\x9A#`\xE1\x1B_R`\x04_\xFD[P\x84_R`\x99` R`\x01\x80`\xA0\x1B\x03`\x01`@_ \x01T\x163\x14a6\x05V[P\x843\x14a5\xFEV[c\x11\xCA35`\xE3\x1B_R`\x04_\xFD[c\xA5\xC7\xC4E`\xE0\x1B_R`\x04_\xFD[a9\"aDRV[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra1;`b\x82a*\x0CV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x91\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[\x90\x94\x93\x90\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x92\x91\x90\x83\x15a$\x03W\x82Q\x15a=\x08Wa9\x98\x83Qa-\x8EV[\x91_[\x84Q\x81\x10\x15a<;Wa9\xB9`\x01`\x01`\xA0\x1B\x03a\n\xD6\x83\x88a/\x15V[_\x87\x81R`\xA2` R`@\x90 \x90\x91\x90`\x01`\x01`\xA0\x1B\x03a9\xDB\x83\x89a/\x15V[Q\x16`\x01\x80`\xA0\x1B\x03\x16_R` Ra9\xF6`@_ a1AV[\x91a:La:\x04\x83\x87a/\x15V[Qa:Ga:/`\x01`\x01`@\x1B\x03a:\x1D\x87\x8Aa/\x15V[Q\x16\x92a:)\x88aE\x94V[\x90aG\x81V[`\x01`\x01`@\x1B\x03a:@\x88aE\xA7V[\x16\x90aG\x81V[aG\x81V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x90\x91a:\xA6\x90` \x90a:k\x86\x8Ca/\x15V[Q\x16\x8C`@Q\x93\x84\x92\x83\x92c\xFE$:\x17`\xE0\x1B\x84R`\x04\x84\x01\x90\x92\x91` \x90`@\x83\x01\x94`\x01\x80`\xA0\x1B\x03\x16\x83R`\x01\x80`\xA0\x1B\x03\x16\x91\x01RV[\x03\x81\x88Z\xFA\x90\x81\x15a\x1EhW_\x91a<\nW[P\x82\x11a;\xFBWa;\x01\x90`\x01`\x01`\xA0\x1B\x03\x8D\x16\x80a;wW[Pa:Ga:\xE2\x85\x89a/\x15V[Q`\x01`\x01`@\x1B\x03a:@\x81a:\xF9\x89\x8Ca/\x15V[Q\x16\x94aE\xA7V[a;\x0B\x83\x88a/\x15V[R`\x01`\x01`\xA0\x1B\x03a;\x1E\x83\x89a/\x15V[Q\x16\x90\x83;\x15a\x0B\xE6Wa;M\x93_\x92\x83\x8C`@Q\x97\x88\x95\x86\x94\x85\x93crJ\xF4#`\xE0\x1B\x85R`\x04\x85\x01a9MV[\x03\x92Z\xF1\x91\x82\x15a\x1EhW`\x01\x92a;gW[P\x01a9\x9BV[_a;q\x91a*\x0CV[_a;`V[\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD`\x01`\x01`\xA0\x1B\x03a;\xAA\x87\x8Da/\x15V[Q\x16\x8Da;\xF2a;\xBA\x89\x8Da/\x15V[Q\x85_R`\x98` R`@_ `\x01\x80`\xA0\x1B\x03\x85\x16_R` R`@_ a;\xE4\x82\x82Ta1vV[\x90U`@Q\x93\x84\x93\x84a9MV[\x03\x90\xA2_a:\xD4V[c\xF0 \xE5\xB9`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=\x82\x11a<3W[\x81a<$` \x93\x83a*\x0CV[\x81\x01\x03\x12a\x0B\xE6WQ_a:\xB9V[=\x91Pa<\x17V[PPP_\x83\x81R`\x9F` R`@\x90 \x80T\x95\x96\x95\x91\x94P\x91\x92\x90\x91\x82_\x19\x81\x14a/zW`\x01\x01\x90U`@Q\x94a<r\x86a)\xF1V[\x81\x86R`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01R`@\x85\x01R``\x84\x01RBc\xFF\xFF\xFF\xFF\x16`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R\x7FX\x88?\x06x\xFFC\xA2\xC0I\xE6\xA3\xA7\xA8\xB9\xB0\xE9\x06)Y\xF3\xA9\x91\x92PX\x88\x19:\x0C_\xEDa=\x02a<\xD0\x83a1\x10V[\x92\x83_R`\x9E` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x91\x82\x91\x85\x83R`@` \x84\x01R`@\x83\x01\x90a0aV[\x03\x90\xA1\x90V[cyl\xC5%`\xE0\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a$\x03W`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R` \x81\x01\x84\x90R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`eUV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x92a>}\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x95a>\x8A\x93\x94\x95`\x01\x80`\xA0\x1B\x03\x16\x80_R`\x98` R`@_ `\x01\x80`\xA0\x1B\x03\x88\x16_R` R`@_ a>\x19\x85\x82Ta/\x8EV[\x90U\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l`@Q\x80a>L\x87\x8B\x8B\x84a9MV[\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R\x90\x81 \x93\x90\x84aE\x0FV[T`@Q\x93\x84\x93\x84a9MV[\x03\x90\xA1V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x99` R`@\x90 `\x045\x91\x82\x16\x82\x03a\x0B\xE6W\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x81U`\x01\x01\x90`$5\x90\x81\x16\x81\x03a\x0B\xE6W\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x81U`D5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x0B\xE6W\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90U`@Q``\x81\x01\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90`\x01`\x01`\xA0\x1B\x03a?b`\x04a(\xECV[\x16\x81R`\x01`\x01`\xA0\x1B\x03a?w`$a(\xECV[\x16` \x82\x01Rc\xFF\xFF\xFF\xFFa?\x8C`Da)iV[\x16`@\x82\x01R\x803\x93\x03\x90\xA2V[a?\xCEa*\xB3\x93\x92`\x01`\x01`@\x1B\x03a?\xC7a?\xC1\x82\x95a?\xBB\x85aE\x94V[\x90aF\xFDV[\x92aE\xA7V[\x16\x90aF\xFDV[\x91\x16\x90aF\xFDV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a?\xEAWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[\x90a@9\x92\x91aE\xCEV[\x15a@@WV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[\x91\x93\x92\x93`\xA0\x83\x01\x90a@b\x82\x85a-\xF6V[\x90P\x81\x03a(\x18W`\x01`\x01`\xA0\x1B\x03a@~`@\x86\x01a.+V[\x163\x03aC\xCDWa@\x92a\x19\x8B6\x86a+\x02V[\x94\x85_R`\x9E` R`\xFF`@_ T\x16\x15aC\xBEW`\x80\x85\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x0B\xE6Wa@\xC3\x90a/\x9BV[\x90_a@\xD1` \x88\x01a.+V[a@\xDB\x86\x89a-\xF6V[`@Qc\x84;4\x9F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\x04\x84\x01R```$\x84\x01R\x91\x94\x85\x92\x83\x92c\xFF\xFF\xFF\xFF\x91aA\x1A\x91`d\x86\x01\x91\x90a.\xC9V[\x91\x16`D\x83\x01R\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x1EhW_\x92aC\xA2W[P_[aAj\x85\x88a-\xF6V[\x90P\x81\x10\x15aCZWaA\x97aA\x92aA\x8D\x83aA\x87\x89\x8Ca-\xF6V[\x90a5&V[a.+V[aC\xDCV[\x90\x87aB\x13\x85a\x0EGaA\xB1\x85aA\x87`\xC0\x87\x01\x87a-\xF6V[5`\x01`\x01`\xA0\x1B\x03aA\xC3\x86a.+V[\x16_R`\xA2` R\x8AaA\xE1aA\x8D\x88aA\x87`@_ \x94\x8Aa-\xF6V[`\x01\x80`\xA0\x1B\x03\x16_R` R`\x01`\x01`@\x1B\x03a?\xC7a\x17\x9C\x82aB\x0B\x8A`@_ \x98a/\x15V[Q\x16\x95a1AV[\x92\x8B\x15aB\xBEW`\x01`\x01`\xA0\x1B\x03\x16aB=aA\x8D\x84aA\x87\x8BaB7\x87a.+V[\x96a-\xF6V[\x90aBLaA\x8D\x85\x8A\x8Da5&V[\x91\x81;\x15a\x0B\xE6W`@Qc\x0B\xAB\x90c`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x90\x84\x16`$\x82\x01R\x91\x90\x92\x16`D\x82\x01R`d\x81\x01\x93\x90\x93R_\x90\x83\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x1EhW`\x01\x92aB\xAEW[P[\x01aA`V[_aB\xB8\x91a*\x0CV[_aB\xA6V[`\x01`\x01`\xA0\x1B\x03\x16aB\xDBaA\x8D\x84aA\x87\x8BaB7\x87a.+V[\x90aB\xEAaA\x8D\x85\x8A\x8Da5&V[\x91\x81;\x15a\x0B\xE6W`@Qc\xC4b>\xA1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x90\x84\x16`$\x82\x01R\x91\x90\x92\x16`D\x82\x01R`d\x81\x01\x93\x90\x93R_\x90\x83\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x1EhW`\x01\x92aCJW[PaB\xA8V[_aCT\x91a*\x0CV[_aCDV[P\x96PPPPPP` \x7F\x1F@@\x08\x89'N\xD0{$\x84^PT\xA8z\x0C\xAB\x96\x9E\xB1'z\xAF\xE6\x1A\xE3R\xE7\xC3*\0\x91\x80_R`\x9E\x82R`@_ `\xFF\x19\x81T\x16\x90U`@Q\x90\x81R\xA1V[aC\xB7\x91\x92P=\x80_\x83>a\x07\xB5\x81\x83a*\x0CV[\x90_aA]V[c\x87\xC9\xD2\x19`\xE0\x1B_R`\x04_\xFD[c\x16\x11\r5`\xE2\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x16_Q` aI^_9_Q\x90_R\x03aD%W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03aD\x9DW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`\n` `@QaD\xAF`@\x82a*\x0CV[\x82\x81R\x01i\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B\x81R `@Q` \x81\x01\x91\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x83R`@\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\x80\x81Ra1;`\xA0\x82a*\x0CV[\x92\x91\x81\x15aEpW\x91`\x01`\x01`@\x1B\x03aEPaEX\x93a:)\x86aEJaEl\x98aEE\x88aE?\x8Da1AV[\x87a?\x9AV[a/\x8EV[\x92a/\x8EV[\x91\x16\x90aG\x81V[`\x01`\x01`@\x1B\x03a:@a\x17\x9C\x85a1AV[\x90UV[PPaEl\x90`\x01`\x01`@\x1B\x03aEP\x81aE\x8Ea\x17\x9C\x87a1AV[\x16aH3V[Q\x80a*\xB3WPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[` \x81\x01Q\x15aE\xC1W`@\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[\x91\x90\x91aE\xDB\x82\x84aHMV[`\x05\x81\x10\x15aF\xE9W\x15\x90\x81aF\xD3W[PaF\xCBW_\x92` aFK`\x84\x86\x95`@Q\x93\x84\x91\x81\x83\x01\x96c\x0B\x13]?`\xE1\x1B\x88R`$\x84\x01R`@`D\x84\x01R\x80Q\x91\x82\x91\x82`d\x86\x01R\x01\x84\x84\x01^\x87\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01`\x1F\x19\x81\x01\x83R\x82a*\x0CV[Q\x91Z\xFA=\x15aF\xC4W=aF_\x81a,EV[\x90aFm`@Q\x92\x83a*\x0CV[\x81R=_` \x83\x01>[\x81aF\xB8W[\x81aF\x86WP\x90V[\x90P` \x81\x80Q\x81\x01\x03\x12a\x0B\xE6W` \x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x90\x81\x90\x03a\x0B\xE6Wc\x0B\x13]?`\xE1\x1B\x14\x90V[\x80Q` \x14\x91PaF}V[``aFwV[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_aE\xECV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90\x91\x90_\x90_\x19\x84\x82\t\x90\x84\x81\x02\x92\x83\x80\x84\x10\x93\x03\x92\x80\x84\x03\x93\x14aGnW\x82g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x03^WP\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x93\x94g\r\xE0\xB6\xB3\xA7d\0\0\x91\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[PPPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x92P\x04\x90V[_\x19g\r\xE0\xB6\xB3\xA7d\0\0\x82\t\x91g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x82\x80\x85\x10\x94\x03\x93\x80\x85\x03\x94\x14aH\x12W\x83\x82\x11\x15a\x0B\xE6Wg\r\xE0\xB6\xB3\xA7d\0\0\x82\x91\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x81`\x03\x02\x18\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x91\x02`\x02\x03\x02\x93`\x01\x83\x80_\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[P\x80\x92P\x15aH\x1FW\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x80\x15aH\x1FWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[\x81Q`A\x81\x03aHyWP\x90aHu\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aH\xBBV[\x90\x91V[`@\x03aH\xB2W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a/zWaHu\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90aH\xBBV[PP_\x90`\x02\x90V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aIRW`\xFF\x16\x90`\x1B\x82\x14\x15\x80aIGW[aI<W` \x93_\x93`\x80\x93`@Q\x93\x84R\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x1EhW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aI4W\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x82\x14\x15aH\xF2V[PPPP_\x90`\x03\x90V\xFE\0\0\0\0\0\0\0\0\0\0\0\0\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\xA2dipfsX\"\x12 i\xE9\xBB\x87\x8E\xD5Cq\xB6\x8Ar\x89\xAEU\x05_8\xA1a(\xD5xH6\xA6\x96E\x9C\xFA\xBC\xFBLdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c90816304a4f97914612872575080630b9f487a146128365780630dd8dd021461264f57806310d67a2f146125b9578063136439dd146124ef57806315c4a288146124b55780631794bb3c146123175780631bbce091146122d157806329c77d4f1461229957806339b70e38146122555780633c651cf214611ff45780633cdeb5e014611fb15780633e28391d14611f745780634337738214611f3a578063457c607014611ec65780634665bcda14611e825780634973006014611aa05780634a5f2b5d14611a5f578063595c6a6714611990578063597b36da146119555780635ac86ab71461191f5780635c975abb146119015780635d9aed23146115e557806365da1264146115a457806366d5ba93146115575780636b3aa72e146115125780636d70f7ae146114e4578063715018a614611487578063778e55f3146114345780637f5480711461112c578063886f1195146111035780638da5cb5b146110da57806390041347146110ab5780639104c319146110835780639435bb4314610f4e57806399be81c814610edc578063a178848414610ea3578063a57ab10b14610d73578063b7f06ebe14610d44578063bb45fef214610cfb578063c5e480db14610c58578063c94b511114610c24578063c978f7ac14610a14578063ca8aa7c7146109cf578063cb00387b146109b2578063cebc04ef14610993578063da8be86414610941578063e4cc3f901461089b578063eea9064b146105f8578063f0e0e6761461044d578063f16172b014610415578063f2fde38b14610384578063f698da25146103615763fabc1cbc14610270575f80fd5b3461035e57602036600319011261035e5760655460405163755b36bd60e11b81526004803592602091839182906001600160a01b03165afa908115610353578391610324575b506001600160a01b031633036103155760665419811981160361030657806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a280f35b63c61dca5d60e01b8252600482fd5b63794821ff60e01b8252600482fd5b610346915060203d60201161034c575b61033e8183612a0c565b810190612f29565b5f6102b6565b503d610334565b6040513d85823e3d90fd5b80fd5b503461035e578060031936011261035e57602061037c614452565b604051908152f35b503461035e57602036600319011261035e5761039e6128aa565b6103a6613fd6565b6001600160a01b038116156103c1576103be90613d74565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b503461035e57606036600319011261035e576104303361343f565b1561043e576103be33613e8f565b6325ec6c1f60e01b8152600490fd5b503461035e57604036600319011261035e576004356001600160401b0381116105f457366023820112156105f45780600401359061048a82612a2d565b916104986040519384612a0c565b8083526024602084019160051b830101913683116105f057602401905b8282106105d8575050506024356001600160401b0381116105d4576104de903690600401612a98565b8151916104ea83612a2d565b926104f86040519485612a0c565b808452610507601f1991612a2d565b01845b8181106105c3575050835b815181101561055a5760019061053e846001600160a01b036105378487612f15565b5116613472565b6105488287612f15565b526105538186612f15565b5001610515565b505050906040519182916020830160208452825180915260408401602060408360051b870101940192905b82821061059457505050500390f35b919360019193955060206105b38192603f198a82030186528851612c12565b9601920192018594939192610585565b80606060208093880101520161050a565b8280fd5b602080916105e5846128ec565b8152019101906104b5565b8480fd5b5080fd5b503461035e57606036600319011261035e576106126128aa565b906024356001600160401b0381116105f457610632903690600401612c60565b335f908152609a6020526040902054604435906001600160a01b031661088c5761065b8461343f565b1561087d57600180606654161461086e576001600160a01b0384811680855260996020526040852060010154909391169081151580610864575b8061085a575b6107cf575b505033808452609a6020526040842080546001600160a01b0319168417905590507fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433048380a36106ee33613183565b60405163547afb8760e01b81529183838061070d848960048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9283156107c45784936107a0575b50835b815181101561079c576001906107966001600160a01b0361076d8386612f15565b51166107798387612f15565b516001600160401b0361078c858a612f15565b511691338b613dbc565b0161074c565b8480f35b6107bd9193503d8086833e6107b58183612a0c565b810190612e3f565b915f610749565b6040513d86823e3d90fd5b602081018051421161084b57828652609c6020526040862084875260205260ff60408720541661083c579061080b610814925185858a33612d1b565b9051908361402e565b8352609c6020526040832090835260205260408220600160ff198254161790555f80806106a0565b630d4c4c9160e21b8652600486fd5b630819bdcd60e01b8652600486fd5b508333141561069b565b5081331415610695565b63840a48d560e01b8352600483fd5b6325ec6c1f60e01b8352600483fd5b633bf2b50360e11b8352600483fd5b503461035e57606036600319011261035e57600435906001600160401b03821161035e5760e0600319833603011261035e576024356001600160401b0381116105f4576108ec903690600401612900565b60443591821515830361093d57600480606654161461092e576109269394610919600260c95414156134da565b600260c95560040161404f565b600160c95580f35b63840a48d560e01b8452600484fd5b8380fd5b503461035e57602036600319011261035e5761095b6128aa565b906002806066541614610984576109806109748361359c565b60405191829182612930565b0390f35b63840a48d560e01b8152600490fd5b503461035e578060031936011261035e576020604051633b9aca008152f35b503461035e578060031936011261035e57602060405161c4e08152f35b503461035e578060031936011261035e576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461035e57610a2336612cdc565b6001600160a01b03808316808552609a602052604080862054905163547afb8760e01b815294959194921692909190828280610a63848860048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610353578392610c08575b50610aaa8151612d8e565b9280941515945b8251811015610bf257610b299060206001600160a01b03610add81610ad68589612f15565b51166143dc565b166001600160a01b03610af08488612f15565b5160405163fe243a1760e01b81526001600160a01b038e8116600483015292909116909116602482015293849190829081906044820190565b03915afa8015610353578390610bbc575b600192508715610bac57610b9a9089855260a260205260408520848060a01b03610b648589612f15565b5116858060a01b03165f5260205260405f2090610b946001600160401b03610b8c868b612f15565b511692613141565b90613f9a565b610ba48288612f15565b525b01610ab1565b610bb68288612f15565b52610ba6565b506020823d8211610bea575b81610bd560209383612a0c565b81010312610be65760019151610b3a565b5f80fd5b3d9150610bc8565b6040516020808252819061098090820188612c12565b610c1d9192503d8085833e6107b58183612a0c565b905f610a9f565b503461035e57608036600319011261035e57602061037c610c436128aa565b610c4b6128d6565b6064359160243590613536565b503461035e57602036600319011261035e576040606091610c776128aa565b81838051610c84816129d6565b828152826020820152015260018060a01b0316815260996020522063ffffffff604051610cb0816129d6565b6001808060a01b0384541693848352015490826040602083019260018060a01b0385168452019260a01c16825260405193845260018060a01b03905116602084015251166040820152f35b503461035e57604036600319011261035e5760209060ff906040906001600160a01b03610d266128aa565b168152609c8452818120602435825284522054166040519015158152f35b503461035e57602036600319011261035e5760ff60406020926004358152609e84522054166040519015158152f35b503461035e57608036600319011261035e57610d8d6128aa565b610d956128c0565b610d9d612bc0565b606435926001600160401b0384168094036105f0577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303610e94577f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd9291610e4c610e529260018060a01b03169586885260986020526040882060018060a01b0386165f52602052610e476001600160401b0360405f2054941684614781565b6146fd565b90613176565b9083855260986020526040852060018060a01b0382165f5260205260405f20610e7c838254613176565b9055610e8e604051928392878461394d565b0390a280f35b6323d871a560e01b8552600485fd5b503461035e57602036600319011261035e576020906040906001600160a01b03610ecb6128aa565b168152609f83522054604051908152f35b503461035e57602036600319011261035e576004356001600160401b0381116105f457610f0d90369060040161297a565b610f163361343f565b1561087d57610e8e7f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080909160405191829133958361303a565b503461035e57606036600319011261035e576004356001600160401b0381116105f457610f7f903690600401612900565b906024356001600160401b03811161093d57610f9f903690600401612900565b926044356001600160401b03811161107f57610fbf903690600401612900565b939092600480606654161461107057610fe0600260c99794975414156134da565b600260c9553682900360de190192875b87811015611067578060051b90818501359186831215611063578382101561104f5761101e90850185612df6565b61102c838b8b969496613526565b3590811515820361104b5760019461104593890161404f565b01610ff0565b8c80fd5b634e487b7160e01b8b52603260045260248bfd5b8a80fd5b88600160c95580f35b63840a48d560e01b8752600487fd5b8580fd5b503461035e578060031936011261035e5760206040515f51602061495e5f395f51905f528152f35b503461035e576109806110c66110c036612cdc565b90613472565b604051918291602083526020830190612c12565b503461035e578060031936011261035e576033546040516001600160a01b039091168152602090f35b503461035e578060031936011261035e576065546040516001600160a01b039091168152602090f35b503461035e5760a036600319011261035e576111466128aa565b9061114f6128c0565b916044356001600160401b0381116105d45761116f903690600401612c60565b6064356001600160401b03811161093d5761118e903690600401612c60565b90608435602082018051421161084b576001600160a01b038086165f908152609a602052604090205416611425576111c58761343f565b15611416576001906111fc828060a01b03871694858952609b6020526111f360408a205493518b858b613536565b9051908861402e565b838752609b6020520160408620556001806066541614611407576001600160a01b03868116808752609960205260408720600101549094911690811515806113fd575b806113f3575b611371575b505050808452609a6020526040842080546001600160a01b031916831790557fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433048480a361129681613183565b60405163547afb8760e01b81529290918484806112b7848a60048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa93841561136657859461134a575b50845b8151811015611346576001906113406001600160a01b036113178386612f15565b51166113238388612f15565b516001600160401b03611336858b612f15565b511691878c613dbc565b016112f6565b8580f35b61135f9194503d8087833e6107b58183612a0c565b925f6112f3565b6040513d87823e3d90fd5b60208101428151106113e457828852609c6020526040882084895260205260ff6040892054166113d5579061080b6113ad925185858c8b612d1b565b8552609c6020526040852090855260205260408420600160ff198254161790555f808061124a565b630d4c4c9160e21b8852600488fd5b630819bdcd60e01b8852600488fd5b5084331415611245565b508133141561123f565b63840a48d560e01b8552600485fd5b6325ec6c1f60e01b8652600486fd5b633bf2b50360e11b8652600486fd5b503461035e57604036600319011261035e5760406114506128aa565b916114596128c0565b9260018060a01b031681526098602052209060018060a01b03165f52602052602060405f2054604051908152f35b503461035e578060031936011261035e576114a0613fd6565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b503461035e57602036600319011261035e5760206115086115036128aa565b61343f565b6040519015158152f35b503461035e578060031936011261035e576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b503461035e57602036600319011261035e5761159661098061157f61157a6128aa565b613183565b604092919251938493604085526040850190612bd6565b908382036020850152612c12565b503461035e57602036600319011261035e576020906001600160a01b036115c96128aa565b168152609a8252604060018060a01b0391205416604051908152f35b503461035e57606036600319011261035e576115ff6128aa565b6024359061160b612bc0565b917f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036118f2576001600160a01b03828116808652609a6020526040808720548151931695919390926116b09190889061166f8682612a0c565b60018152601f1986013660208301375f51602061495e5f395f51905f5261169582612f08565b5285518094819263547afb8760e01b83528b60048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156118e8579161181d9161183e938a926118cc575b50868a5260a2602052858a205f51602061495e5f395f51905f525f526020527fddf935ec8825c7afee6a15d4731e28963ee96dfcb85d0a1e794b43318bbca4fd8661175e815f206117586001600160401b0361175088612f08565b511691613141565b85613f9a565b96898d5260a2602052818d205f51602061495e5f395f51905f525f526020526001806117aa845f20936001600160401b03806117a161179c88613141565b6145a7565b169116906146fd565b92019168ffffffffffffffff0083549160081b169068ffffffffffffffffff191617178091556001600160401b038251918b835260081c166020820152a1868a5260a2602052858a205f51602061495e5f395f51905f525f52602052610b946001600160401b03610b8c885f2094612f08565b6001600160a01b039586165f908152609a6020526040902054909516151590565b611846578580f35b7f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd9361187191613176565b9184865260986020528186205f51602061495e5f395f51905f52875260205281862061189e848254613176565b905581519081525f51602061495e5f395f51905f52602082015290810191909152606090a25f808080808580f35b6118e19192503d808c833e6107b58183612a0c565b905f6116f5565b84513d8a823e3d90fd5b633213a66160e21b8452600484fd5b503461035e578060031936011261035e576020606654604051908152f35b503461035e57602036600319011261035e5760043560ff81168091036105f457600190602092501b806066541614604051908152f35b503461035e57602036600319011261035e57600435906001600160401b03821161035e57602061037c61198b3660048601612b02565b613110565b503461035e578060031936011261035e5760655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa908115611a54578291611a25575b5015611a16575f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a280f35b631d77d47760e21b8152600490fd5b611a47915060203d602011611a4d575b611a3f8183612a0c565b810190612f48565b5f6119db565b503d611a35565b6040513d84823e3d90fd5b503461035e578060031936011261035e57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b5034610be657366003190160a08112610be657606013610be65760643563ffffffff8116809103610be6576084356001600160401b038111610be657611aea90369060040161297a565b335f908152609a6020526040902054909291906001600160a01b0316611e73577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691823b15610be65760405190632b6241f360e11b825233600483015260248201525f8160448183875af18015611e6857611e53575b50611b7333613e8f565b604051611b7f816129a7565b6060815260208101908582526001806066541614611e4457338652609960205260408620600101546001600160a01b03169182151580611e3a575b80611e33575b611d5f575b505033808652609a6020526040862080546001600160a01b031916821790559050807fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433048680a3611c1433613183565b94909181604051809563547afb8760e01b82528180611c37883360048401613018565b03915afa938415611a54578294611d43575b50815b8351811015611c9c57600190611c966001600160a01b03611c6d8388612f15565b5116611c79838b612f15565b516001600160401b03611c8c858b612f15565b5116913333613dbc565b01611c4c565b50604051856004356001600160a01b038116908190036105f05782526024356001600160a01b038116908190036105f05760208301526044359063ffffffff82168092036105f05782610e8e9260407f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080909501527f8e8485583a2310d41f7c82b9427d0bd49bad74bb9cff9d3402a29d8f9b28a0e260603392a260405191829133958361303a565b611d589194503d8084833e6107b58183612a0c565b925f611c49565b80514211611e2457828752609c6020526040872087805260205260ff604088205416611e15579061080b611ded925160405160208101917f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad83528660408301523360608301523360808301528a60a083015260c082015260c08152611de560e082612a0c565b51902061391a565b8452609c6020526040842084805260205260408420600160ff198254161790555f8080611bc5565b630d4c4c9160e21b8752600487fd5b630819bdcd60e01b8752600487fd5b5086611bc0565b5082331415611bba565b63840a48d560e01b8652600486fd5b611e609194505f90612a0c565b5f925f611b69565b6040513d5f823e3d90fd5b633bf2b50360e11b5f5260045ffd5b34610be6575f366003190112610be6576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610be6576040366003190112610be657611edf6128aa565b611ee76128c0565b9060018060a01b03165f5260a260205260405f209060018060a01b03165f52602052606060405f206001600160401b036001825492015460405192835260ff81161515602084015260081c166040820152f35b34610be6575f366003190112610be65760206040517f39111bc4a4d688e1f685123d7497d4615370152a8ee4a0593e647bd06ad8bb0b8152f35b34610be6576020366003190112610be6576020611508611f926128aa565b6001600160a01b039081165f908152609a602052604090205416151590565b34610be6576020366003190112610be6576001600160a01b03611fd26128aa565b165f526099602052602060018060a01b03600160405f20015416604051908152f35b34610be6576080366003190112610be65761200d6128aa565b6120156128c0565b60643591337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316148015612223575b15612214576001600160a01b038082165f908152609a60205260409020541661207157005b6001600160a01b038181165f818152609a60205260408082205481519197941692916120e1916120a18982612a0c565b60018152601f1989013660208301376120b981612f08565b6001600160a01b0389169052885163547afb8760e01b81529283918291908760048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561220a57926121eb94926001600160401b0361215b6121df947f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f9b975f916121f0575b50612f08565b511691805f526098602052855f2060018060a01b038a165f52602052855f20612185868254612f8e565b90557f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c8651806121b7888d8d8461394d565b0390a25f5260a2602052835f2060018060a01b0388165f52602052835f20926044358461450f565b5490519384938461394d565b0390a1005b61220491503d805f833e6107b58183612a0c565b8c612155565b87513d5f823e3d90fd5b63045206a560e21b5f5260045ffd5b50337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03161461204c565b34610be6575f366003190112610be6576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610be6576020366003190112610be6576001600160a01b036122ba6128aa565b165f52609b602052602060405f2054604051908152f35b34610be6576060366003190112610be657602061037c6122ef6128aa565b6122f76128c0565b6001600160a01b0382165f908152609b8552604090205460443592613536565b34610be6576060366003190112610be6576123306128aa565b6123386128c0565b604435915f549260ff8460081c1615938480956124a8575b8015612491575b156124355760ff1981166001175f5584612424575b506065546001600160a01b03161580612412575b15612403576123c792816123c2926066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2613d17565b613d74565b6123cd57005b61ff00195f54165f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a1005b6339b190bb60e11b5f5260045ffd5b506001600160a01b0383161515612380565b61ffff1916610101175f558461236c565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b1580156123575750600160ff821614612357565b50600160ff821610612350565b34610be6576020366003190112610be65760043563ffffffff81168103610be6576124e1602091612f9b565b63ffffffff60405191168152f35b34610be6576020366003190112610be6576004356024602060018060a01b03606554166040519283809263237dfb4760e11b82523360048301525afa908115611e68575f9161259a575b501561258b576066548181160361257c57806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b63c61dca5d60e01b5f5260045ffd5b631d77d47760e21b5f5260045ffd5b6125b3915060203d602011611a4d57611a3f8183612a0c565b82612539565b34610be6576020366003190112610be6576125d26128aa565b60655460405163755b36bd60e11b815290602090829060049082906001600160a01b03165afa908115611e68575f91612630575b506001600160a01b031633036126215761261f90613d17565b005b63794821ff60e01b5f5260045ffd5b612649915060203d60201161034c5761033e8183612a0c565b82612606565b34610be6576020366003190112610be6576004356001600160401b038111610be65761267f903690600401612900565b60028060665416146128275761269481612d8e565b335f908152609a60205260408120546001600160a01b039081169492937f000000000000000000000000000000000000000000000000000000000000000090911692915b8181106126ed57604051806109808782612930565b6127016126fb828486612dc0565b80612df6565b905061271b612711838587612dc0565b6020810190612df6565b9190500361281857336001600160a01b03612742604061273c858789612dc0565b01612e2b565b160361280957805f61275b6126fb612784948688612dc0565b604051948592839263547afb8760e01b84528c6004850152604060248501526044840191612ec9565b0381885afa918215611e68576001926127de915f916127ef575b506127d66127b06126fb85888a612dc0565b6127ce6127c4612711888b8d979697612dc0565b9490923691612a44565b923691612ab6565b908a3361396f565b6127e88288612f15565b52016126d8565b61280391503d805f833e6107b58183612a0c565b8961279e565b6330c4716960e21b5f5260045ffd5b6343714afd60e01b5f5260045ffd5b63840a48d560e01b5f5260045ffd5b34610be65760a0366003190112610be657602061037c6128546128aa565b61285c6128c0565b906128656128d6565b6084359260643592612d1b565b34610be6575f366003190112610be657807f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad60209252f35b600435906001600160a01b0382168203610be657565b602435906001600160a01b0382168203610be657565b604435906001600160a01b0382168203610be657565b35906001600160a01b0382168203610be657565b9181601f84011215610be6578235916001600160401b038311610be6576020808501948460051b010111610be657565b60206040818301928281528451809452019201905f5b8181106129535750505090565b8251845260209384019390920191600101612946565b359063ffffffff82168203610be657565b9181601f84011215610be6578235916001600160401b038311610be65760208381860195010111610be657565b604081019081106001600160401b038211176129c257604052565b634e487b7160e01b5f52604160045260245ffd5b606081019081106001600160401b038211176129c257604052565b60e081019081106001600160401b038211176129c257604052565b90601f801991011681019081106001600160401b038211176129c257604052565b6001600160401b0381116129c25760051b60200190565b929190612a5081612a2d565b93612a5e6040519586612a0c565b602085838152019160051b8101928311610be657905b828210612a8057505050565b60208091612a8d846128ec565b815201910190612a74565b9080601f83011215610be657816020612ab393359101612a44565b90565b929190612ac281612a2d565b93612ad06040519586612a0c565b602085838152019160051b8101928311610be657905b828210612af257505050565b8135815260209182019101612ae6565b919060e083820312610be65760405190612b1b826129f1565b8193612b26816128ec565b8352612b34602082016128ec565b6020840152612b45604082016128ec565b604084015260608101356060840152612b6060808201612969565b608084015260a08101356001600160401b038111610be65782612b84918301612a98565b60a084015260c0810135906001600160401b038211610be6570181601f82011215610be65760c091816020612bbb93359101612ab6565b910152565b604435906001600160401b0382168203610be657565b90602080835192838152019201905f5b818110612bf35750505090565b82516001600160a01b0316845260209384019390920191600101612be6565b90602080835192838152019201905f5b818110612c2f5750505090565b8251845260209384019390920191600101612c22565b6001600160401b0381116129c257601f01601f191660200190565b9190604083820312610be65760405190612c79826129a7565b819380356001600160401b038111610be65781019082601f83011215610be657813592612ca584612c45565b90612cb36040519283612a0c565b84825260208585010111610be6575f602085819682809701838601378301015284520135910152565b906040600319830112610be6576004356001600160a01b0381168103610be65791602435906001600160401b038211610be657612ab391600401612a98565b9192612ab394916040519360208501957f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad875260018060a01b0316604086015260018060a01b0316606085015260018060a01b0316608084015260a083015260c082015260c08152611de560e082612a0c565b90612d9882612a2d565b612da56040519182612a0c565b8281528092612db6601f1991612a2d565b0190602036910137565b9190811015612de25760051b81013590605e1981360301821215610be6570190565b634e487b7160e01b5f52603260045260245ffd5b903590601e1981360301821215610be657018035906001600160401b038211610be657602001918160051b36038313610be657565b356001600160a01b0381168103610be65790565b602081830312610be6578051906001600160401b038211610be657019080601f83011215610be657815190612e7382612a2d565b92612e816040519485612a0c565b82845260208085019360051b820101918211610be657602001915b818310612ea95750505090565b82516001600160401b0381168103610be657815260209283019201612e9c565b916020908281520191905f5b818110612ee25750505090565b909192602080600192838060a01b03612efa886128ec565b168152019401929101612ed5565b805115612de25760200190565b8051821015612de25760209160051b010190565b90816020910312610be657516001600160a01b0381168103610be65790565b90816020910312610be657518015158103610be65790565b9063ffffffff8091169116019063ffffffff8211612f7a57565b634e487b7160e01b5f52601160045260245ffd5b91908201809211612f7a57565b63ffffffff811690633b9aca00821015612fd5575061c4e08101809111612f7a574310612fc6575f90565b6378f67ae160e11b5f5260045ffd5b90507f00000000000000000000000000000000000000000000000000000000000000006130028183612f60565b63ffffffff42911611612fc657612ab391612f60565b6001600160a01b039091168152604060208201819052612ab392910190612bd6565b90918060409360208452816020850152848401375f828201840152601f01601f1916010190565b919060e081019060018060a01b03845116815260018060a01b03602085015116602082015260018060a01b0360408501511660408201526060840151606082015263ffffffff608085015116608082015260a08401519160e060a08301528251809152602061010083019301905f5b8181106130f15750505060c0612ab3939401519060c0818403910152612c12565b82516001600160a01b03168552602094850194909201916001016130d0565b60405161313b8161312d6020820194602086526040830190613061565b03601f198101835282612a0c565b51902090565b9060405161314e816129d6565b60406001600160401b036001839580548552015460ff81161515602085015260081c16910152565b91908203918211612f7a57565b6040516394f649dd60e01b81526001600160a01b03918216600482018190529092915f90849060249082907f0000000000000000000000000000000000000000000000000000000000000000165afa8015611e68575f935f9161332d575b5060405163fe243a1760e01b815260048101929092525f51602061495e5f395f51905f5260248301526020826044817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215611e68575f926132f9575b5081156132f457835160018101809111612f7a5761326790612d8e565b93805160018101809111612f7a5761327e90612d8e565b925f51602061495e5f395f51905f52613298835188612f15565b526132a4825185612f15565b525f5b81518110156132ee576001906001600160a01b036132c58285612f15565b51166132d18289612f15565b526132dc8185612f15565b516132e78287612f15565b52016132a7565b50505090565b919050565b9091506020813d602011613325575b8161331560209383612a0c565b81010312610be65751905f61324a565b3d9150613308565b9350503d805f853e61333f8185612a0c565b830192604081850312610be65780516001600160401b038111610be65781019084601f83011215610be657815161337581612a2d565b926133836040519485612a0c565b81845260208085019260051b82010190878211610be657602001915b81831061341f575050506020810151906001600160401b038211610be657019380601f86011215610be65784516133d581612a2d565b956133e36040519788612a0c565b81875260208088019260051b820101928311610be657602001905b82821061340f57505050925f6131e1565b81518152602091820191016133fe565b82516001600160a01b0381168103610be65781526020928301920161339f565b6001600160a01b03168015159081613455575090565b5f818152609a60205260409020546001600160a01b031614919050565b9061347d8151612d8e565b905f5b81518110156134d3576001600160a01b038481165f908152609860205260409020600192916134af8386612f15565b5116838060a01b03165f5260205260405f20546134cc8286612f15565b5201613480565b5050905090565b156134e157565b60405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606490fd5b9190811015612de25760051b0190565b919092612ab3936040519260208401947f39111bc4a4d688e1f685123d7497d4615370152a8ee4a0593e647bd06ad8bb0b865260018060a01b0316604085015260018060a01b03166060840152608083015260a082015260a08152611de560c082612a0c565b6001600160a01b038082165f908152609a6020526040902054161561390b576135c48161343f565b6138fc576001600160a01b038116908115612403575f828152609a60205260409020546001600160a01b03169233831480159190826138f3575b80156138d3575b156138c45761361383613183565b939092855f52609a60205260405f206bffffffffffffffffffffffff60a01b815416905586867ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af446765f80a3613899575b8251801561388d5761367390612d8e565b916040519463547afb8760e01b86525f8680613693888c60048401613018565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa958615611e68575f96613871575b50909660409560209291905f5b8751811015613864576001906137ce86868b61379f8f8f8051936137008286612a0c565b8985528636602087013761375c8983519761371b858a612a0c565b8c8952893660208b0137845199613732868c612a0c565b8d8b523660208c01378c8060a01b0361374b8386612f15565b511661375689612f08565b52612f15565b51925f5260a2602052815f20906137798a8c8060a01b0392612f15565b51168a8060a01b03165f526020525f2090610b946001600160401b03610b8c8a8d612f15565b6137a883612f08565b526001600160401b036137bb8689612f15565b51166137c684612f08565b52878b61396f565b6137d8828a612f15565b528a5f5260a2602052895f20828060a01b036137f4838c612f15565b5116838060a01b03165f52602052670de0b6b3a76400008a5f20557f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f60608c8b61384485878060a01b0392612f15565b51168d519182526020820152670de0b6b3a76400008d820152a1016136dc565b5050505050509250505090565b6138869196503d805f833e6107b58183612a0c565b945f6136cf565b50509350505050606090565b85857ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a5f80a3613662565b631e499a2360e11b5f5260045ffd5b50845f52609960205260018060a01b03600160405f200154163314613605565b508433146135fe565b6311ca333560e31b5f5260045ffd5b63a5c7c44560e01b5f5260045ffd5b613922614452565b9060405190602082019261190160f01b8452602283015260428201526042815261313b606282612a0c565b6001600160a01b03918216815291166020820152604081019190915260600190565b90949390926001600160a01b038416929190831561240357825115613d08576139988351612d8e565b915f5b8451811015613c3b576139b96001600160a01b03610ad68388612f15565b5f87815260a2602052604090209091906001600160a01b036139db8389612f15565b511660018060a01b03165f526020526139f660405f20613141565b91613a4c613a048387612f15565b51613a47613a2f6001600160401b03613a1d878a612f15565b511692613a2988614594565b90614781565b6001600160401b03613a40886145a7565b1690614781565b614781565b6001600160a01b03918216939091613aa690602090613a6b868c612f15565b51168c604051938492839263fe243a1760e01b845260048401909291602090604083019460018060a01b0316835260018060a01b0316910152565b0381885afa908115611e68575f91613c0a575b508211613bfb57613b01906001600160a01b038d1680613b77575b50613a47613ae28589612f15565b516001600160401b03613a4081613af9898c612f15565b5116946145a7565b613b0b8388612f15565b526001600160a01b03613b1e8389612f15565b511690833b15610be657613b4d935f92838c6040519788958694859363724af42360e01b85526004850161394d565b03925af1918215611e6857600192613b67575b500161399b565b5f613b7191612a0c565b5f613b60565b7f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd6001600160a01b03613baa878d612f15565b51168d613bf2613bba898d612f15565b51855f52609860205260405f2060018060a01b0385165f5260205260405f20613be4828254613176565b90556040519384938461394d565b0390a25f613ad4565b63f020e5b960e01b5f5260045ffd5b90506020813d8211613c33575b81613c2460209383612a0c565b81010312610be657515f613ab9565b3d9150613c17565b5050505f838152609f60205260409020805495969591945091929091825f198114612f7a57600101905560405194613c72866129f1565b8186526001600160a01b03166020860152604085015260608401524263ffffffff16608084015260a083015260c08201527f58883f0678ff43a2c049e6a3a7a8b9b0e9062959f3a99192505888193a0c5fed613d02613cd083613110565b92835f52609e60205260405f20600160ff19825416179055604051918291858352604060208401526040830190613061565b0390a190565b63796cc52560e01b5f5260045ffd5b6001600160a01b0316801561240357606554604080516001600160a01b0383168152602081018490527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb69190a16001600160a01b03191617606555565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b92613e7d7f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f95613e8a93949560018060a01b0316805f52609860205260405f2060018060a01b0388165f5260205260405f20613e19858254612f8e565b90557f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c60405180613e4c878b8b8461394d565b0390a26001600160a01b038581165f90815260a260209081526040808320938a16835292905290812093908461450f565b546040519384938461394d565b0390a1565b6001600160a01b039081165f9081526099602052604090206004359182168203610be65780546001600160a01b0319166001600160a01b03928316178155600101906024359081168103610be65781546001600160a01b0319166001600160a01b039190911617815560443563ffffffff81168103610be657815463ffffffff60a01b191660a09190911b63ffffffff60a01b1617905560405160608101907ffebe5cd24b2cbc7b065b9d0fdeb904461e4afcff57dd57acda1e7832031ba7ac906001600160a01b03613f6260046128ec565b1681526001600160a01b03613f7760246128ec565b16602082015263ffffffff613f8c6044612969565b1660408201528033930390a2565b613fce612ab393926001600160401b03613fc7613fc18295613fbb85614594565b906146fd565b926145a7565b16906146fd565b9116906146fd565b6033546001600160a01b03163303613fea57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b9061403992916145ce565b1561404057565b638baa579f60e01b5f5260045ffd5b9193929360a08301906140628285612df6565b90508103612818576001600160a01b0361407e60408601612e2b565b1633036143cd5761409261198b3686612b02565b94855f52609e60205260ff60405f205416156143be57608085013563ffffffff81168103610be6576140c390612f9b565b905f6140d160208801612e2b565b6140db8689612df6565b60405163843b349f60e01b81526001600160a01b0390931660048401526060602484015291948592839263ffffffff9161411a91606486019190612ec9565b9116604483015203817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215611e68575f926143a2575b505f5b61416a8588612df6565b905081101561435a5761419761419261418d83614187898c612df6565b90613526565b612e2b565b6143dc565b908761421385610e476141b18561418760c0870187612df6565b356001600160a01b036141c386612e2b565b165f5260a26020528a6141e161418d8861418760405f20948a612df6565b60018060a01b03165f526020526001600160401b03613fc761179c8261420b8a60405f2098612f15565b511695613141565b928b156142be576001600160a01b031661423d61418d846141878b61423787612e2b565b96612df6565b9061424c61418d858a8d613526565b91813b15610be657604051630bab906360e21b81526001600160a01b039485166004820152908416602482015291909216604482015260648101939093525f908390608490829084905af1918215611e68576001926142ae575b505b01614160565b5f6142b891612a0c565b5f6142a6565b6001600160a01b03166142db61418d846141878b61423787612e2b565b906142ea61418d858a8d613526565b91813b15610be65760405163c4623ea160e01b81526001600160a01b039485166004820152908416602482015291909216604482015260648101939093525f908390608490829084905af1918215611e685760019261434a575b506142a8565b5f61435491612a0c565b5f614344565b509650505050505060207f1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a0091805f52609e825260405f2060ff198154169055604051908152a1565b6143b79192503d805f833e6107b58183612a0c565b905f61415d565b6387c9d21960e01b5f5260045ffd5b6316110d3560e21b5f5260045ffd5b6001600160a01b03165f51602061495e5f395f51905f5203614425577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690565b467f00000000000000000000000000000000000000000000000000000000000000000361449d577f000000000000000000000000000000000000000000000000000000000000000090565b600a60206040516144af604082612a0c565b828152016922b4b3b2b72630bcb2b960b11b81522060405160208101917f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866835260408201524660608201523060808201526080815261313b60a082612a0c565b9291811561457057916001600160401b0361455061455893613a298661454a61456c986145458861453f8d613141565b87613f9a565b612f8e565b92612f8e565b911690614781565b6001600160401b03613a4061179c85613141565b9055565b505061456c906001600160401b036145508161458e61179c87613141565b16614833565b5180612ab35750670de0b6b3a764000090565b6020810151156145c157604001516001600160401b031690565b50670de0b6b3a764000090565b9190916145db828461484d565b60058110156146e9571590816146d3575b506146cb575f92602061464b6084869560405193849181830196630b135d3f60e11b88526024840152604060448401528051918291826064860152018484015e87838284010152601f801991011681010301601f198101835282612a0c565b51915afa3d156146c4573d61465f81612c45565b9061466d6040519283612a0c565b81523d5f602083013e5b816146b8575b81614686575090565b9050602081805181010312610be657602001516001600160e01b0319811690819003610be657630b135d3f60e11b1490565b8051602014915061467d565b6060614677565b505050600190565b6001600160a01b0383811691161490505f6145ec565b634e487b7160e01b5f52602160045260245ffd5b9091905f905f19848209908481029283808410930392808403931461476e5782670de0b6b3a7640000111561035e57507faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac106699394670de0b6b3a7640000910990828211900360ee1b910360121c170290565b505050670de0b6b3a76400009192500490565b5f19670de0b6b3a7640000820991670de0b6b3a76400008202918280851094039380850394146148125783821115610be657670de0b6b3a7640000829109600182190182168092046002816003021880820260020302808202600203028082026002030280820260020302808202600203028091026002030293600183805f03040190848311900302920304170290565b508092501561481f570490565b634e487b7160e01b5f52601260045260245ffd5b801561481f576ec097ce7bc90715b34b9f10000000000490565b81516041810361487957509061487591602082015190606060408401519301515f1a906148bb565b9091565b6040036148b25760406020830151920151918260ff1c91601b8301809311612f7a57614875936001600160ff1b03169260ff16906148bb565b50505f90600290565b907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116149525760ff1690601b82141580614947575b61493c576020935f93608093604051938452868401526040830152606082015282805260015afa15611e68575f516001600160a01b0381161561493457905f90565b505f90600190565b505050505f90600490565b50601c8214156148f2565b505050505f9060039056fe000000000000000000000000beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0a264697066735822122069e9bb878ed54371b68a7289ae55055f38a16128d5784836a696459cfabcfb4c64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x04\xA4\xF9y\x14a(rWP\x80c\x0B\x9FHz\x14a(6W\x80c\r\xD8\xDD\x02\x14a&OW\x80c\x10\xD6z/\x14a%\xB9W\x80c\x13d9\xDD\x14a$\xEFW\x80c\x15\xC4\xA2\x88\x14a$\xB5W\x80c\x17\x94\xBB<\x14a#\x17W\x80c\x1B\xBC\xE0\x91\x14a\"\xD1W\x80c)\xC7}O\x14a\"\x99W\x80c9\xB7\x0E8\x14a\"UW\x80c<e\x1C\xF2\x14a\x1F\xF4W\x80c<\xDE\xB5\xE0\x14a\x1F\xB1W\x80c>(9\x1D\x14a\x1FtW\x80cC7s\x82\x14a\x1F:W\x80cE|`p\x14a\x1E\xC6W\x80cFe\xBC\xDA\x14a\x1E\x82W\x80cIs\0`\x14a\x1A\xA0W\x80cJ_+]\x14a\x1A_W\x80cY\\jg\x14a\x19\x90W\x80cY{6\xDA\x14a\x19UW\x80cZ\xC8j\xB7\x14a\x19\x1FW\x80c\\\x97Z\xBB\x14a\x19\x01W\x80c]\x9A\xED#\x14a\x15\xE5W\x80ce\xDA\x12d\x14a\x15\xA4W\x80cf\xD5\xBA\x93\x14a\x15WW\x80ck:\xA7.\x14a\x15\x12W\x80cmp\xF7\xAE\x14a\x14\xE4W\x80cqP\x18\xA6\x14a\x14\x87W\x80cw\x8EU\xF3\x14a\x144W\x80c\x7FT\x80q\x14a\x11,W\x80c\x88o\x11\x95\x14a\x11\x03W\x80c\x8D\xA5\xCB[\x14a\x10\xDAW\x80c\x90\x04\x13G\x14a\x10\xABW\x80c\x91\x04\xC3\x19\x14a\x10\x83W\x80c\x945\xBBC\x14a\x0FNW\x80c\x99\xBE\x81\xC8\x14a\x0E\xDCW\x80c\xA1x\x84\x84\x14a\x0E\xA3W\x80c\xA5z\xB1\x0B\x14a\rsW\x80c\xB7\xF0n\xBE\x14a\rDW\x80c\xBBE\xFE\xF2\x14a\x0C\xFBW\x80c\xC5\xE4\x80\xDB\x14a\x0CXW\x80c\xC9KQ\x11\x14a\x0C$W\x80c\xC9x\xF7\xAC\x14a\n\x14W\x80c\xCA\x8A\xA7\xC7\x14a\t\xCFW\x80c\xCB\08{\x14a\t\xB2W\x80c\xCE\xBC\x04\xEF\x14a\t\x93W\x80c\xDA\x8B\xE8d\x14a\tAW\x80c\xE4\xCC?\x90\x14a\x08\x9BW\x80c\xEE\xA9\x06K\x14a\x05\xF8W\x80c\xF0\xE0\xE6v\x14a\x04MW\x80c\xF1ar\xB0\x14a\x04\x15W\x80c\xF2\xFD\xE3\x8B\x14a\x03\x84W\x80c\xF6\x98\xDA%\x14a\x03aWc\xFA\xBC\x1C\xBC\x14a\x02pW_\x80\xFD[4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`eT`@Qcu[6\xBD`\xE1\x1B\x81R`\x04\x805\x92` \x91\x83\x91\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03SW\x83\x91a\x03$W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x03\x15W`fT\x19\x81\x19\x81\x16\x03a\x03\x06W\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\x80\xF3[c\xC6\x1D\xCA]`\xE0\x1B\x82R`\x04\x82\xFD[cyH!\xFF`\xE0\x1B\x82R`\x04\x82\xFD[a\x03F\x91P` =` \x11a\x03LW[a\x03>\x81\x83a*\x0CV[\x81\x01\x90a/)V[_a\x02\xB6V[P=a\x034V[`@Q=\x85\x82>=\x90\xFD[\x80\xFD[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` a\x03|aDRV[`@Q\x90\x81R\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^Wa\x03\x9Ea(\xAAV[a\x03\xA6a?\xD6V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x03\xC1Wa\x03\xBE\x90a=tV[\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P4a\x03^W``6`\x03\x19\x01\x12a\x03^Wa\x0403a4?V[\x15a\x04>Wa\x03\xBE3a>\x8FV[c%\xECl\x1F`\xE0\x1B\x81R`\x04\x90\xFD[P4a\x03^W`@6`\x03\x19\x01\x12a\x03^W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x05\xF4W6`#\x82\x01\x12\x15a\x05\xF4W\x80`\x04\x015\x90a\x04\x8A\x82a*-V[\x91a\x04\x98`@Q\x93\x84a*\x0CV[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x05\xF0W`$\x01\x90[\x82\x82\x10a\x05\xD8WPPP`$5`\x01`\x01`@\x1B\x03\x81\x11a\x05\xD4Wa\x04\xDE\x906\x90`\x04\x01a*\x98V[\x81Q\x91a\x04\xEA\x83a*-V[\x92a\x04\xF8`@Q\x94\x85a*\x0CV[\x80\x84Ra\x05\x07`\x1F\x19\x91a*-V[\x01\x84[\x81\x81\x10a\x05\xC3WPP\x83[\x81Q\x81\x10\x15a\x05ZW`\x01\x90a\x05>\x84`\x01`\x01`\xA0\x1B\x03a\x057\x84\x87a/\x15V[Q\x16a4rV[a\x05H\x82\x87a/\x15V[Ra\x05S\x81\x86a/\x15V[P\x01a\x05\x15V[PPP\x90`@Q\x91\x82\x91` \x83\x01` \x84R\x82Q\x80\x91R`@\x84\x01` `@\x83`\x05\x1B\x87\x01\x01\x94\x01\x92\x90[\x82\x82\x10a\x05\x94WPPPP\x03\x90\xF3[\x91\x93`\x01\x91\x93\x95P` a\x05\xB3\x81\x92`?\x19\x8A\x82\x03\x01\x86R\x88Qa,\x12V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x05\x85V[\x80``` \x80\x93\x88\x01\x01R\x01a\x05\nV[\x82\x80\xFD[` \x80\x91a\x05\xE5\x84a(\xECV[\x81R\x01\x91\x01\x90a\x04\xB5V[\x84\x80\xFD[P\x80\xFD[P4a\x03^W``6`\x03\x19\x01\x12a\x03^Wa\x06\x12a(\xAAV[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x05\xF4Wa\x062\x906\x90`\x04\x01a,`V[3_\x90\x81R`\x9A` R`@\x90 T`D5\x90`\x01`\x01`\xA0\x1B\x03\x16a\x08\x8CWa\x06[\x84a4?V[\x15a\x08}W`\x01\x80`fT\x16\x14a\x08nW`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x80\x85R`\x99` R`@\x85 `\x01\x01T\x90\x93\x91\x16\x90\x81\x15\x15\x80a\x08dW[\x80a\x08ZW[a\x07\xCFW[PP3\x80\x84R`\x9A` R`@\x84 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90U\x90P\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x83\x80\xA3a\x06\xEE3a1\x83V[`@QcTz\xFB\x87`\xE0\x1B\x81R\x91\x83\x83\x80a\x07\r\x84\x89`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\x07\xC4W\x84\x93a\x07\xA0W[P\x83[\x81Q\x81\x10\x15a\x07\x9CW`\x01\x90a\x07\x96`\x01`\x01`\xA0\x1B\x03a\x07m\x83\x86a/\x15V[Q\x16a\x07y\x83\x87a/\x15V[Q`\x01`\x01`@\x1B\x03a\x07\x8C\x85\x8Aa/\x15V[Q\x16\x913\x8Ba=\xBCV[\x01a\x07LV[\x84\x80\xF3[a\x07\xBD\x91\x93P=\x80\x86\x83>a\x07\xB5\x81\x83a*\x0CV[\x81\x01\x90a.?V[\x91_a\x07IV[`@Q=\x86\x82>=\x90\xFD[` \x81\x01\x80QB\x11a\x08KW\x82\x86R`\x9C` R`@\x86 \x84\x87R` R`\xFF`@\x87 T\x16a\x08<W\x90a\x08\x0Ba\x08\x14\x92Q\x85\x85\x8A3a-\x1BV[\x90Q\x90\x83a@.V[\x83R`\x9C` R`@\x83 \x90\x83R` R`@\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U_\x80\x80a\x06\xA0V[c\rLL\x91`\xE2\x1B\x86R`\x04\x86\xFD[c\x08\x19\xBD\xCD`\xE0\x1B\x86R`\x04\x86\xFD[P\x833\x14\x15a\x06\x9BV[P\x813\x14\x15a\x06\x95V[c\x84\nH\xD5`\xE0\x1B\x83R`\x04\x83\xFD[c%\xECl\x1F`\xE0\x1B\x83R`\x04\x83\xFD[c;\xF2\xB5\x03`\xE1\x1B\x83R`\x04\x83\xFD[P4a\x03^W``6`\x03\x19\x01\x12a\x03^W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03^W`\xE0`\x03\x19\x836\x03\x01\x12a\x03^W`$5`\x01`\x01`@\x1B\x03\x81\x11a\x05\xF4Wa\x08\xEC\x906\x90`\x04\x01a)\0V[`D5\x91\x82\x15\x15\x83\x03a\t=W`\x04\x80`fT\x16\x14a\t.Wa\t&\x93\x94a\t\x19`\x02`\xC9T\x14\x15a4\xDAV[`\x02`\xC9U`\x04\x01a@OV[`\x01`\xC9U\x80\xF3[c\x84\nH\xD5`\xE0\x1B\x84R`\x04\x84\xFD[\x83\x80\xFD[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^Wa\t[a(\xAAV[\x90`\x02\x80`fT\x16\x14a\t\x84Wa\t\x80a\tt\x83a5\x9CV[`@Q\x91\x82\x91\x82a)0V[\x03\x90\xF3[c\x84\nH\xD5`\xE0\x1B\x81R`\x04\x90\xFD[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` `@Qc;\x9A\xCA\0\x81R\xF3[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` `@Qa\xC4\xE0\x81R\xF3[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x03^Wa\n#6a,\xDCV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x80\x85R`\x9A` R`@\x80\x86 T\x90QcTz\xFB\x87`\xE0\x1B\x81R\x94\x95\x91\x94\x92\x16\x92\x90\x91\x90\x82\x82\x80a\nc\x84\x88`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03SW\x83\x92a\x0C\x08W[Pa\n\xAA\x81Qa-\x8EV[\x92\x80\x94\x15\x15\x94[\x82Q\x81\x10\x15a\x0B\xF2Wa\x0B)\x90` `\x01`\x01`\xA0\x1B\x03a\n\xDD\x81a\n\xD6\x85\x89a/\x15V[Q\x16aC\xDCV[\x16`\x01`\x01`\xA0\x1B\x03a\n\xF0\x84\x88a/\x15V[Q`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01R\x92\x90\x91\x16\x90\x91\x16`$\x82\x01R\x93\x84\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x80\x15a\x03SW\x83\x90a\x0B\xBCW[`\x01\x92P\x87\x15a\x0B\xACWa\x0B\x9A\x90\x89\x85R`\xA2` R`@\x85 \x84\x80`\xA0\x1B\x03a\x0Bd\x85\x89a/\x15V[Q\x16\x85\x80`\xA0\x1B\x03\x16_R` R`@_ \x90a\x0B\x94`\x01`\x01`@\x1B\x03a\x0B\x8C\x86\x8Ba/\x15V[Q\x16\x92a1AV[\x90a?\x9AV[a\x0B\xA4\x82\x88a/\x15V[R[\x01a\n\xB1V[a\x0B\xB6\x82\x88a/\x15V[Ra\x0B\xA6V[P` \x82=\x82\x11a\x0B\xEAW[\x81a\x0B\xD5` \x93\x83a*\x0CV[\x81\x01\x03\x12a\x0B\xE6W`\x01\x91Qa\x0B:V[_\x80\xFD[=\x91Pa\x0B\xC8V[`@Q` \x80\x82R\x81\x90a\t\x80\x90\x82\x01\x88a,\x12V[a\x0C\x1D\x91\x92P=\x80\x85\x83>a\x07\xB5\x81\x83a*\x0CV[\x90_a\n\x9FV[P4a\x03^W`\x806`\x03\x19\x01\x12a\x03^W` a\x03|a\x0CCa(\xAAV[a\x0CKa(\xD6V[`d5\x91`$5\x90a56V[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`@``\x91a\x0Cwa(\xAAV[\x81\x83\x80Qa\x0C\x84\x81a)\xD6V[\x82\x81R\x82` \x82\x01R\x01R`\x01\x80`\xA0\x1B\x03\x16\x81R`\x99` R c\xFF\xFF\xFF\xFF`@Qa\x0C\xB0\x81a)\xD6V[`\x01\x80\x80`\xA0\x1B\x03\x84T\x16\x93\x84\x83R\x01T\x90\x82`@` \x83\x01\x92`\x01\x80`\xA0\x1B\x03\x85\x16\x84R\x01\x92`\xA0\x1C\x16\x82R`@Q\x93\x84R`\x01\x80`\xA0\x1B\x03\x90Q\x16` \x84\x01RQ\x16`@\x82\x01R\xF3[P4a\x03^W`@6`\x03\x19\x01\x12a\x03^W` \x90`\xFF\x90`@\x90`\x01`\x01`\xA0\x1B\x03a\r&a(\xAAV[\x16\x81R`\x9C\x84R\x81\x81 `$5\x82R\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`\xFF`@` \x92`\x045\x81R`\x9E\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x03^W`\x806`\x03\x19\x01\x12a\x03^Wa\r\x8Da(\xAAV[a\r\x95a(\xC0V[a\r\x9Da+\xC0V[`d5\x92`\x01`\x01`@\x1B\x03\x84\x16\x80\x94\x03a\x05\xF0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0E\x94W\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x92\x91a\x0ELa\x0ER\x92`\x01\x80`\xA0\x1B\x03\x16\x95\x86\x88R`\x98` R`@\x88 `\x01\x80`\xA0\x1B\x03\x86\x16_R` Ra\x0EG`\x01`\x01`@\x1B\x03`@_ T\x94\x16\x84aG\x81V[aF\xFDV[\x90a1vV[\x90\x83\x85R`\x98` R`@\x85 `\x01\x80`\xA0\x1B\x03\x82\x16_R` R`@_ a\x0E|\x83\x82Ta1vV[\x90Ua\x0E\x8E`@Q\x92\x83\x92\x87\x84a9MV[\x03\x90\xA2\x80\xF3[c#\xD8q\xA5`\xE0\x1B\x85R`\x04\x85\xFD[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W` \x90`@\x90`\x01`\x01`\xA0\x1B\x03a\x0E\xCBa(\xAAV[\x16\x81R`\x9F\x83R T`@Q\x90\x81R\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x05\xF4Wa\x0F\r\x906\x90`\x04\x01a)zV[a\x0F\x163a4?V[\x15a\x08}Wa\x0E\x8E\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x91`@Q\x91\x82\x913\x95\x83a0:V[P4a\x03^W``6`\x03\x19\x01\x12a\x03^W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x05\xF4Wa\x0F\x7F\x906\x90`\x04\x01a)\0V[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\t=Wa\x0F\x9F\x906\x90`\x04\x01a)\0V[\x92`D5`\x01`\x01`@\x1B\x03\x81\x11a\x10\x7FWa\x0F\xBF\x906\x90`\x04\x01a)\0V[\x93\x90\x92`\x04\x80`fT\x16\x14a\x10pWa\x0F\xE0`\x02`\xC9\x97\x94\x97T\x14\x15a4\xDAV[`\x02`\xC9U6\x82\x90\x03`\xDE\x19\x01\x92\x87[\x87\x81\x10\x15a\x10gW\x80`\x05\x1B\x90\x81\x85\x015\x91\x86\x83\x12\x15a\x10cW\x83\x82\x10\x15a\x10OWa\x10\x1E\x90\x85\x01\x85a-\xF6V[a\x10,\x83\x8B\x8B\x96\x94\x96a5&V[5\x90\x81\x15\x15\x82\x03a\x10KW`\x01\x94a\x10E\x93\x89\x01a@OV[\x01a\x0F\xF0V[\x8C\x80\xFD[cNH{q`\xE0\x1B\x8BR`2`\x04R`$\x8B\xFD[\x8A\x80\xFD[\x88`\x01`\xC9U\x80\xF3[c\x84\nH\xD5`\xE0\x1B\x87R`\x04\x87\xFD[\x85\x80\xFD[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` `@Q_Q` aI^_9_Q\x90_R\x81R\xF3[P4a\x03^Wa\t\x80a\x10\xC6a\x10\xC06a,\xDCV[\x90a4rV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a,\x12V[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x03^W`\xA06`\x03\x19\x01\x12a\x03^Wa\x11Fa(\xAAV[\x90a\x11Oa(\xC0V[\x91`D5`\x01`\x01`@\x1B\x03\x81\x11a\x05\xD4Wa\x11o\x906\x90`\x04\x01a,`V[`d5`\x01`\x01`@\x1B\x03\x81\x11a\t=Wa\x11\x8E\x906\x90`\x04\x01a,`V[\x90`\x845` \x82\x01\x80QB\x11a\x08KW`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x9A` R`@\x90 T\x16a\x14%Wa\x11\xC5\x87a4?V[\x15a\x14\x16W`\x01\x90a\x11\xFC\x82\x80`\xA0\x1B\x03\x87\x16\x94\x85\x89R`\x9B` Ra\x11\xF3`@\x8A T\x93Q\x8B\x85\x8Ba56V[\x90Q\x90\x88a@.V[\x83\x87R`\x9B` R\x01`@\x86 U`\x01\x80`fT\x16\x14a\x14\x07W`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x80\x87R`\x99` R`@\x87 `\x01\x01T\x90\x94\x91\x16\x90\x81\x15\x15\x80a\x13\xFDW[\x80a\x13\xF3W[a\x13qW[PPP\x80\x84R`\x9A` R`@\x84 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x83\x17\x90U\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x84\x80\xA3a\x12\x96\x81a1\x83V[`@QcTz\xFB\x87`\xE0\x1B\x81R\x92\x90\x91\x84\x84\x80a\x12\xB7\x84\x8A`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x93\x84\x15a\x13fW\x85\x94a\x13JW[P\x84[\x81Q\x81\x10\x15a\x13FW`\x01\x90a\x13@`\x01`\x01`\xA0\x1B\x03a\x13\x17\x83\x86a/\x15V[Q\x16a\x13#\x83\x88a/\x15V[Q`\x01`\x01`@\x1B\x03a\x136\x85\x8Ba/\x15V[Q\x16\x91\x87\x8Ca=\xBCV[\x01a\x12\xF6V[\x85\x80\xF3[a\x13_\x91\x94P=\x80\x87\x83>a\x07\xB5\x81\x83a*\x0CV[\x92_a\x12\xF3V[`@Q=\x87\x82>=\x90\xFD[` \x81\x01B\x81Q\x10a\x13\xE4W\x82\x88R`\x9C` R`@\x88 \x84\x89R` R`\xFF`@\x89 T\x16a\x13\xD5W\x90a\x08\x0Ba\x13\xAD\x92Q\x85\x85\x8C\x8Ba-\x1BV[\x85R`\x9C` R`@\x85 \x90\x85R` R`@\x84 `\x01`\xFF\x19\x82T\x16\x17\x90U_\x80\x80a\x12JV[c\rLL\x91`\xE2\x1B\x88R`\x04\x88\xFD[c\x08\x19\xBD\xCD`\xE0\x1B\x88R`\x04\x88\xFD[P\x843\x14\x15a\x12EV[P\x813\x14\x15a\x12?V[c\x84\nH\xD5`\xE0\x1B\x85R`\x04\x85\xFD[c%\xECl\x1F`\xE0\x1B\x86R`\x04\x86\xFD[c;\xF2\xB5\x03`\xE1\x1B\x86R`\x04\x86\xFD[P4a\x03^W`@6`\x03\x19\x01\x12a\x03^W`@a\x14Pa(\xAAV[\x91a\x14Ya(\xC0V[\x92`\x01\x80`\xA0\x1B\x03\x16\x81R`\x98` R \x90`\x01\x80`\xA0\x1B\x03\x16_R` R` `@_ T`@Q\x90\x81R\xF3[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^Wa\x14\xA0a?\xD6V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W` a\x15\x08a\x15\x03a(\xAAV[a4?V[`@Q\x90\x15\x15\x81R\xF3[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^Wa\x15\x96a\t\x80a\x15\x7Fa\x15za(\xAAV[a1\x83V[`@\x92\x91\x92Q\x93\x84\x93`@\x85R`@\x85\x01\x90a+\xD6V[\x90\x83\x82\x03` \x85\x01Ra,\x12V[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W` \x90`\x01`\x01`\xA0\x1B\x03a\x15\xC9a(\xAAV[\x16\x81R`\x9A\x82R`@`\x01\x80`\xA0\x1B\x03\x91 T\x16`@Q\x90\x81R\xF3[P4a\x03^W``6`\x03\x19\x01\x12a\x03^Wa\x15\xFFa(\xAAV[`$5\x90a\x16\x0Ba+\xC0V[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x18\xF2W`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x80\x86R`\x9A` R`@\x80\x87 T\x81Q\x93\x16\x95\x91\x93\x90\x92a\x16\xB0\x91\x90\x88\x90a\x16o\x86\x82a*\x0CV[`\x01\x81R`\x1F\x19\x86\x016` \x83\x017_Q` aI^_9_Q\x90_Ra\x16\x95\x82a/\x08V[R\x85Q\x80\x94\x81\x92cTz\xFB\x87`\xE0\x1B\x83R\x8B`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x18\xE8W\x91a\x18\x1D\x91a\x18>\x93\x8A\x92a\x18\xCCW[P\x86\x8AR`\xA2` R\x85\x8A _Q` aI^_9_Q\x90_R_R` R\x7F\xDD\xF95\xEC\x88%\xC7\xAF\xEEj\x15\xD4s\x1E(\x96>\xE9m\xFC\xB8]\n\x1EyKC1\x8B\xBC\xA4\xFD\x86a\x17^\x81_ a\x17X`\x01`\x01`@\x1B\x03a\x17P\x88a/\x08V[Q\x16\x91a1AV[\x85a?\x9AV[\x96\x89\x8DR`\xA2` R\x81\x8D _Q` aI^_9_Q\x90_R_R` R`\x01\x80a\x17\xAA\x84_ \x93`\x01`\x01`@\x1B\x03\x80a\x17\xA1a\x17\x9C\x88a1AV[aE\xA7V[\x16\x91\x16\x90aF\xFDV[\x92\x01\x91h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83T\x91`\x08\x1B\x16\x90h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x17\x80\x91U`\x01`\x01`@\x1B\x03\x82Q\x91\x8B\x83R`\x08\x1C\x16` \x82\x01R\xA1\x86\x8AR`\xA2` R\x85\x8A _Q` aI^_9_Q\x90_R_R` Ra\x0B\x94`\x01`\x01`@\x1B\x03a\x0B\x8C\x88_ \x94a/\x08V[`\x01`\x01`\xA0\x1B\x03\x95\x86\x16_\x90\x81R`\x9A` R`@\x90 T\x90\x95\x16\x15\x15\x90V[a\x18FW\x85\x80\xF3[\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x93a\x18q\x91a1vV[\x91\x84\x86R`\x98` R\x81\x86 _Q` aI^_9_Q\x90_R\x87R` R\x81\x86 a\x18\x9E\x84\x82Ta1vV[\x90U\x81Q\x90\x81R_Q` aI^_9_Q\x90_R` \x82\x01R\x90\x81\x01\x91\x90\x91R``\x90\xA2_\x80\x80\x80\x80\x85\x80\xF3[a\x18\xE1\x91\x92P=\x80\x8C\x83>a\x07\xB5\x81\x83a*\x0CV[\x90_a\x16\xF5V[\x84Q=\x8A\x82>=\x90\xFD[c2\x13\xA6a`\xE2\x1B\x84R`\x04\x84\xFD[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` `fT`@Q\x90\x81R\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`\x045`\xFF\x81\x16\x80\x91\x03a\x05\xF4W`\x01\x90` \x92P\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[P4a\x03^W` 6`\x03\x19\x01\x12a\x03^W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03^W` a\x03|a\x19\x8B6`\x04\x86\x01a+\x02V[a1\x10V[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x1ATW\x82\x91a\x1A%W[P\x15a\x1A\x16W_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\x80\xF3[c\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x90\xFD[a\x1AG\x91P` =` \x11a\x1AMW[a\x1A?\x81\x83a*\x0CV[\x81\x01\x90a/HV[_a\x19\xDBV[P=a\x1A5V[`@Q=\x84\x82>=\x90\xFD[P4a\x03^W\x80`\x03\x196\x01\x12a\x03^W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x0B\xE6W6`\x03\x19\x01`\xA0\x81\x12a\x0B\xE6W``\x13a\x0B\xE6W`d5c\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x0B\xE6W`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xE6Wa\x1A\xEA\x906\x90`\x04\x01a)zV[3_\x90\x81R`\x9A` R`@\x90 T\x90\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1EsW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\x0B\xE6W`@Q\x90c+bA\xF3`\xE1\x1B\x82R3`\x04\x83\x01R`$\x82\x01R_\x81`D\x81\x83\x87Z\xF1\x80\x15a\x1EhWa\x1ESW[Pa\x1Bs3a>\x8FV[`@Qa\x1B\x7F\x81a)\xA7V[``\x81R` \x81\x01\x90\x85\x82R`\x01\x80`fT\x16\x14a\x1EDW3\x86R`\x99` R`@\x86 `\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15\x15\x80a\x1E:W[\x80a\x1E3W[a\x1D_W[PP3\x80\x86R`\x9A` R`@\x86 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x82\x17\x90U\x90P\x80\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x86\x80\xA3a\x1C\x143a1\x83V[\x94\x90\x91\x81`@Q\x80\x95cTz\xFB\x87`\xE0\x1B\x82R\x81\x80a\x1C7\x883`\x04\x84\x01a0\x18V[\x03\x91Z\xFA\x93\x84\x15a\x1ATW\x82\x94a\x1DCW[P\x81[\x83Q\x81\x10\x15a\x1C\x9CW`\x01\x90a\x1C\x96`\x01`\x01`\xA0\x1B\x03a\x1Cm\x83\x88a/\x15V[Q\x16a\x1Cy\x83\x8Ba/\x15V[Q`\x01`\x01`@\x1B\x03a\x1C\x8C\x85\x8Ba/\x15V[Q\x16\x9133a=\xBCV[\x01a\x1CLV[P`@Q\x85`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x05\xF0W\x82R`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x05\xF0W` \x83\x01R`D5\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x05\xF0W\x82a\x0E\x8E\x92`@\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x95\x01R\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2``3\x92\xA2`@Q\x91\x82\x913\x95\x83a0:V[a\x1DX\x91\x94P=\x80\x84\x83>a\x07\xB5\x81\x83a*\x0CV[\x92_a\x1CIV[\x80QB\x11a\x1E$W\x82\x87R`\x9C` R`@\x87 \x87\x80R` R`\xFF`@\x88 T\x16a\x1E\x15W\x90a\x08\x0Ba\x1D\xED\x92Q`@Q` \x81\x01\x91\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x83R\x86`@\x83\x01R3``\x83\x01R3`\x80\x83\x01R\x8A`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x1D\xE5`\xE0\x82a*\x0CV[Q\x90 a9\x1AV[\x84R`\x9C` R`@\x84 \x84\x80R` R`@\x84 `\x01`\xFF\x19\x82T\x16\x17\x90U_\x80\x80a\x1B\xC5V[c\rLL\x91`\xE2\x1B\x87R`\x04\x87\xFD[c\x08\x19\xBD\xCD`\xE0\x1B\x87R`\x04\x87\xFD[P\x86a\x1B\xC0V[P\x823\x14\x15a\x1B\xBAV[c\x84\nH\xD5`\xE0\x1B\x86R`\x04\x86\xFD[a\x1E`\x91\x94P_\x90a*\x0CV[_\x92_a\x1BiV[`@Q=_\x82>=\x90\xFD[c;\xF2\xB5\x03`\xE1\x1B_R`\x04_\xFD[4a\x0B\xE6W_6`\x03\x19\x01\x12a\x0B\xE6W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x0B\xE6W`@6`\x03\x19\x01\x12a\x0B\xE6Wa\x1E\xDFa(\xAAV[a\x1E\xE7a(\xC0V[\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R```@_ `\x01`\x01`@\x1B\x03`\x01\x82T\x92\x01T`@Q\x92\x83R`\xFF\x81\x16\x15\x15` \x84\x01R`\x08\x1C\x16`@\x82\x01R\xF3[4a\x0B\xE6W_6`\x03\x19\x01\x12a\x0B\xE6W` `@Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81R\xF3[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W` a\x15\x08a\x1F\x92a(\xAAV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W`\x01`\x01`\xA0\x1B\x03a\x1F\xD2a(\xAAV[\x16_R`\x99` R` `\x01\x80`\xA0\x1B\x03`\x01`@_ \x01T\x16`@Q\x90\x81R\xF3[4a\x0B\xE6W`\x806`\x03\x19\x01\x12a\x0B\xE6Wa \ra(\xAAV[a \x15a(\xC0V[`d5\x913\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\"#W[\x15a\"\x14W`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x9A` R`@\x90 T\x16a qW\0[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x9A` R`@\x80\x82 T\x81Q\x91\x97\x94\x16\x92\x91a \xE1\x91a \xA1\x89\x82a*\x0CV[`\x01\x81R`\x1F\x19\x89\x016` \x83\x017a \xB9\x81a/\x08V[`\x01`\x01`\xA0\x1B\x03\x89\x16\x90R\x88QcTz\xFB\x87`\xE0\x1B\x81R\x92\x83\x91\x82\x91\x90\x87`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\"\nW\x92a!\xEB\x94\x92`\x01`\x01`@\x1B\x03a![a!\xDF\x94\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x9B\x97_\x91a!\xF0W[Pa/\x08V[Q\x16\x91\x80_R`\x98` R\x85_ `\x01\x80`\xA0\x1B\x03\x8A\x16_R` R\x85_ a!\x85\x86\x82Ta/\x8EV[\x90U\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x86Q\x80a!\xB7\x88\x8D\x8D\x84a9MV[\x03\x90\xA2_R`\xA2` R\x83_ `\x01\x80`\xA0\x1B\x03\x88\x16_R` R\x83_ \x92`D5\x84aE\x0FV[T\x90Q\x93\x84\x93\x84a9MV[\x03\x90\xA1\0[a\"\x04\x91P=\x80_\x83>a\x07\xB5\x81\x83a*\x0CV[\x8Ca!UV[\x87Q=_\x82>=\x90\xFD[c\x04R\x06\xA5`\xE2\x1B_R`\x04_\xFD[P3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a LV[4a\x0B\xE6W_6`\x03\x19\x01\x12a\x0B\xE6W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W`\x01`\x01`\xA0\x1B\x03a\"\xBAa(\xAAV[\x16_R`\x9B` R` `@_ T`@Q\x90\x81R\xF3[4a\x0B\xE6W``6`\x03\x19\x01\x12a\x0B\xE6W` a\x03|a\"\xEFa(\xAAV[a\"\xF7a(\xC0V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9B\x85R`@\x90 T`D5\x92a56V[4a\x0B\xE6W``6`\x03\x19\x01\x12a\x0B\xE6Wa#0a(\xAAV[a#8a(\xC0V[`D5\x91_T\x92`\xFF\x84`\x08\x1C\x16\x15\x93\x84\x80\x95a$\xA8W[\x80\x15a$\x91W[\x15a$5W`\xFF\x19\x81\x16`\x01\x17_U\x84a$$W[P`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a$\x12W[\x15a$\x03Wa#\xC7\x92\x81a#\xC2\x92`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2a=\x17V[a=tV[a#\xCDW\0[a\xFF\0\x19_T\x16_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\0[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x15a#\x80V[a\xFF\xFF\x19\x16a\x01\x01\x17_U\x84a#lV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a#WWP`\x01`\xFF\x82\x16\x14a#WV[P`\x01`\xFF\x82\x16\x10a#PV[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W`\x045c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x0B\xE6Wa$\xE1` \x91a/\x9BV[c\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W`\x045`$` `\x01\x80`\xA0\x1B\x03`eT\x16`@Q\x92\x83\x80\x92c#}\xFBG`\xE1\x1B\x82R3`\x04\x83\x01RZ\xFA\x90\x81\x15a\x1EhW_\x91a%\x9AW[P\x15a%\x8BW`fT\x81\x81\x16\x03a%|W\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[a%\xB3\x91P` =` \x11a\x1AMWa\x1A?\x81\x83a*\x0CV[\x82a%9V[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6Wa%\xD2a(\xAAV[`eT`@Qcu[6\xBD`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x1EhW_\x91a&0W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a&!Wa&\x1F\x90a=\x17V[\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a&I\x91P` =` \x11a\x03LWa\x03>\x81\x83a*\x0CV[\x82a&\x06V[4a\x0B\xE6W` 6`\x03\x19\x01\x12a\x0B\xE6W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xE6Wa&\x7F\x906\x90`\x04\x01a)\0V[`\x02\x80`fT\x16\x14a('Wa&\x94\x81a-\x8EV[3_\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x92\x93\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x92\x91[\x81\x81\x10a&\xEDW`@Q\x80a\t\x80\x87\x82a)0V[a'\x01a&\xFB\x82\x84\x86a-\xC0V[\x80a-\xF6V[\x90Pa'\x1Ba'\x11\x83\x85\x87a-\xC0V[` \x81\x01\x90a-\xF6V[\x91\x90P\x03a(\x18W3`\x01`\x01`\xA0\x1B\x03a'B`@a'<\x85\x87\x89a-\xC0V[\x01a.+V[\x16\x03a(\tW\x80_a'[a&\xFBa'\x84\x94\x86\x88a-\xC0V[`@Q\x94\x85\x92\x83\x92cTz\xFB\x87`\xE0\x1B\x84R\x8C`\x04\x85\x01R`@`$\x85\x01R`D\x84\x01\x91a.\xC9V[\x03\x81\x88Z\xFA\x91\x82\x15a\x1EhW`\x01\x92a'\xDE\x91_\x91a'\xEFW[Pa'\xD6a'\xB0a&\xFB\x85\x88\x8Aa-\xC0V[a'\xCEa'\xC4a'\x11\x88\x8B\x8D\x97\x96\x97a-\xC0V[\x94\x90\x926\x91a*DV[\x926\x91a*\xB6V[\x90\x8A3a9oV[a'\xE8\x82\x88a/\x15V[R\x01a&\xD8V[a(\x03\x91P=\x80_\x83>a\x07\xB5\x81\x83a*\x0CV[\x89a'\x9EV[c0\xC4qi`\xE2\x1B_R`\x04_\xFD[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[4a\x0B\xE6W`\xA06`\x03\x19\x01\x12a\x0B\xE6W` a\x03|a(Ta(\xAAV[a(\\a(\xC0V[\x90a(ea(\xD6V[`\x845\x92`d5\x92a-\x1BV[4a\x0B\xE6W_6`\x03\x19\x01\x12a\x0B\xE6W\x80\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x92R\xF3[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xE6WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xE6WV[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xE6WV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xE6WV[\x91\x81`\x1F\x84\x01\x12\x15a\x0B\xE6W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x0B\xE6W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x0B\xE6WV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a)SWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a)FV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x0B\xE6WV[\x91\x81`\x1F\x84\x01\x12\x15a\x0B\xE6W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x0B\xE6W` \x83\x81\x86\x01\x95\x01\x01\x11a\x0B\xE6WV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a)\xC2W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a)\xC2W`@RV[`\xE0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a)\xC2W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a)\xC2W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a)\xC2W`\x05\x1B` \x01\x90V[\x92\x91\x90a*P\x81a*-V[\x93a*^`@Q\x95\x86a*\x0CV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x0B\xE6W\x90[\x82\x82\x10a*\x80WPPPV[` \x80\x91a*\x8D\x84a(\xECV[\x81R\x01\x91\x01\x90a*tV[\x90\x80`\x1F\x83\x01\x12\x15a\x0B\xE6W\x81` a*\xB3\x935\x91\x01a*DV[\x90V[\x92\x91\x90a*\xC2\x81a*-V[\x93a*\xD0`@Q\x95\x86a*\x0CV[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x0B\xE6W\x90[\x82\x82\x10a*\xF2WPPPV[\x815\x81R` \x91\x82\x01\x91\x01a*\xE6V[\x91\x90`\xE0\x83\x82\x03\x12a\x0B\xE6W`@Q\x90a+\x1B\x82a)\xF1V[\x81\x93a+&\x81a(\xECV[\x83Ra+4` \x82\x01a(\xECV[` \x84\x01Ra+E`@\x82\x01a(\xECV[`@\x84\x01R``\x81\x015``\x84\x01Ra+``\x80\x82\x01a)iV[`\x80\x84\x01R`\xA0\x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xE6W\x82a+\x84\x91\x83\x01a*\x98V[`\xA0\x84\x01R`\xC0\x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x0B\xE6W\x01\x81`\x1F\x82\x01\x12\x15a\x0B\xE6W`\xC0\x91\x81` a+\xBB\x935\x91\x01a*\xB6V[\x91\x01RV[`D5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x0B\xE6WV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a+\xF3WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a+\xE6V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a,/WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a,\"V[`\x01`\x01`@\x1B\x03\x81\x11a)\xC2W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x91\x90`@\x83\x82\x03\x12a\x0B\xE6W`@Q\x90a,y\x82a)\xA7V[\x81\x93\x805`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xE6W\x81\x01\x90\x82`\x1F\x83\x01\x12\x15a\x0B\xE6W\x815\x92a,\xA5\x84a,EV[\x90a,\xB3`@Q\x92\x83a*\x0CV[\x84\x82R` \x85\x85\x01\x01\x11a\x0B\xE6W_` \x85\x81\x96\x82\x80\x97\x01\x83\x86\x017\x83\x01\x01R\x84R\x015\x91\x01RV[\x90`@`\x03\x19\x83\x01\x12a\x0B\xE6W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x0B\xE6W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x0B\xE6Wa*\xB3\x91`\x04\x01a*\x98V[\x91\x92a*\xB3\x94\x91`@Q\x93` \x85\x01\x95\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x87R`\x01\x80`\xA0\x1B\x03\x16`@\x86\x01R`\x01\x80`\xA0\x1B\x03\x16``\x85\x01R`\x01\x80`\xA0\x1B\x03\x16`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x1D\xE5`\xE0\x82a*\x0CV[\x90a-\x98\x82a*-V[a-\xA5`@Q\x91\x82a*\x0CV[\x82\x81R\x80\x92a-\xB6`\x1F\x19\x91a*-V[\x01\x90` 6\x91\x017V[\x91\x90\x81\x10\x15a-\xE2W`\x05\x1B\x81\x015\x90`^\x19\x816\x03\x01\x82\x12\x15a\x0B\xE6W\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x0B\xE6W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x0B\xE6W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x0B\xE6WV[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x0B\xE6W\x90V[` \x81\x83\x03\x12a\x0B\xE6W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x0B\xE6W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x0B\xE6W\x81Q\x90a.s\x82a*-V[\x92a.\x81`@Q\x94\x85a*\x0CV[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a\x0B\xE6W` \x01\x91[\x81\x83\x10a.\xA9WPPP\x90V[\x82Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x0B\xE6W\x81R` \x92\x83\x01\x92\x01a.\x9CV[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a.\xE2WPPP\x90V[\x90\x91\x92` \x80`\x01\x92\x83\x80`\xA0\x1B\x03a.\xFA\x88a(\xECV[\x16\x81R\x01\x94\x01\x92\x91\x01a.\xD5V[\x80Q\x15a-\xE2W` \x01\x90V[\x80Q\x82\x10\x15a-\xE2W` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x0B\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x0B\xE6W\x90V[\x90\x81` \x91\x03\x12a\x0B\xE6WQ\x80\x15\x15\x81\x03a\x0B\xE6W\x90V[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a/zWV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x91\x90\x82\x01\x80\x92\x11a/zWV[c\xFF\xFF\xFF\xFF\x81\x16\x90c;\x9A\xCA\0\x82\x10\x15a/\xD5WPa\xC4\xE0\x81\x01\x80\x91\x11a/zWC\x10a/\xC6W_\x90V[cx\xF6z\xE1`\xE1\x1B_R`\x04_\xFD[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a0\x02\x81\x83a/`V[c\xFF\xFF\xFF\xFFB\x91\x16\x11a/\xC6Wa*\xB3\x91a/`V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra*\xB3\x92\x91\x01\x90a+\xD6V[\x90\x91\x80`@\x93` \x84R\x81` \x85\x01R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90`\xE0\x81\x01\x90`\x01\x80`\xA0\x1B\x03\x84Q\x16\x81R`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03`@\x85\x01Q\x16`@\x82\x01R``\x84\x01Q``\x82\x01Rc\xFF\xFF\xFF\xFF`\x80\x85\x01Q\x16`\x80\x82\x01R`\xA0\x84\x01Q\x91`\xE0`\xA0\x83\x01R\x82Q\x80\x91R` a\x01\0\x83\x01\x93\x01\x90_[\x81\x81\x10a0\xF1WPPP`\xC0a*\xB3\x93\x94\x01Q\x90`\xC0\x81\x84\x03\x91\x01Ra,\x12V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a0\xD0V[`@Qa1;\x81a1-` \x82\x01\x94` \x86R`@\x83\x01\x90a0aV[\x03`\x1F\x19\x81\x01\x83R\x82a*\x0CV[Q\x90 \x90V[\x90`@Qa1N\x81a)\xD6V[`@`\x01`\x01`@\x1B\x03`\x01\x83\x95\x80T\x85R\x01T`\xFF\x81\x16\x15\x15` \x85\x01R`\x08\x1C\x16\x91\x01RV[\x91\x90\x82\x03\x91\x82\x11a/zWV[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01\x81\x90R\x90\x92\x91_\x90\x84\x90`$\x90\x82\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x1EhW_\x93_\x91a3-W[P`@Qc\xFE$:\x17`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R_Q` aI^_9_Q\x90_R`$\x83\x01R` \x82`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x1EhW_\x92a2\xF9W[P\x81\x15a2\xF4W\x83Q`\x01\x81\x01\x80\x91\x11a/zWa2g\x90a-\x8EV[\x93\x80Q`\x01\x81\x01\x80\x91\x11a/zWa2~\x90a-\x8EV[\x92_Q` aI^_9_Q\x90_Ra2\x98\x83Q\x88a/\x15V[Ra2\xA4\x82Q\x85a/\x15V[R_[\x81Q\x81\x10\x15a2\xEEW`\x01\x90`\x01`\x01`\xA0\x1B\x03a2\xC5\x82\x85a/\x15V[Q\x16a2\xD1\x82\x89a/\x15V[Ra2\xDC\x81\x85a/\x15V[Qa2\xE7\x82\x87a/\x15V[R\x01a2\xA7V[PPP\x90V[\x91\x90PV[\x90\x91P` \x81=` \x11a3%W[\x81a3\x15` \x93\x83a*\x0CV[\x81\x01\x03\x12a\x0B\xE6WQ\x90_a2JV[=\x91Pa3\x08V[\x93PP=\x80_\x85>a3?\x81\x85a*\x0CV[\x83\x01\x92`@\x81\x85\x03\x12a\x0B\xE6W\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x0B\xE6W\x81\x01\x90\x84`\x1F\x83\x01\x12\x15a\x0B\xE6W\x81Qa3u\x81a*-V[\x92a3\x83`@Q\x94\x85a*\x0CV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x90\x87\x82\x11a\x0B\xE6W` \x01\x91[\x81\x83\x10a4\x1FWPPP` \x81\x01Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x0B\xE6W\x01\x93\x80`\x1F\x86\x01\x12\x15a\x0B\xE6W\x84Qa3\xD5\x81a*-V[\x95a3\xE3`@Q\x97\x88a*\x0CV[\x81\x87R` \x80\x88\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x0B\xE6W` \x01\x90[\x82\x82\x10a4\x0FWPPP\x92_a1\xE1V[\x81Q\x81R` \x91\x82\x01\x91\x01a3\xFEV[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x0B\xE6W\x81R` \x92\x83\x01\x92\x01a3\x9FV[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x15\x90\x81a4UWP\x90V[_\x81\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14\x91\x90PV[\x90a4}\x81Qa-\x8EV[\x90_[\x81Q\x81\x10\x15a4\xD3W`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x90\x81R`\x98` R`@\x90 `\x01\x92\x91a4\xAF\x83\x86a/\x15V[Q\x16\x83\x80`\xA0\x1B\x03\x16_R` R`@_ Ta4\xCC\x82\x86a/\x15V[R\x01a4\x80V[PP\x90P\x90V[\x15a4\xE1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x90\xFD[\x91\x90\x81\x10\x15a-\xE2W`\x05\x1B\x01\x90V[\x91\x90\x92a*\xB3\x93`@Q\x92` \x84\x01\x94\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x86R`\x01\x80`\xA0\x1B\x03\x16`@\x85\x01R`\x01\x80`\xA0\x1B\x03\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xA0\x81Ra\x1D\xE5`\xC0\x82a*\x0CV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15a9\x0BWa5\xC4\x81a4?V[a8\xFCW`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a$\x03W_\x82\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x923\x83\x14\x80\x15\x91\x90\x82a8\xF3W[\x80\x15a8\xD3W[\x15a8\xC4Wa6\x13\x83a1\x83V[\x93\x90\x92\x85_R`\x9A` R`@_ k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x81T\x16\x90U\x86\x86\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv_\x80\xA3a8\x99W[\x82Q\x80\x15a8\x8DWa6s\x90a-\x8EV[\x91`@Q\x94cTz\xFB\x87`\xE0\x1B\x86R_\x86\x80a6\x93\x88\x8C`\x04\x84\x01a0\x18V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x95\x86\x15a\x1EhW_\x96a8qW[P\x90\x96`@\x95` \x92\x91\x90_[\x87Q\x81\x10\x15a8dW`\x01\x90a7\xCE\x86\x86\x8Ba7\x9F\x8F\x8F\x80Q\x93a7\0\x82\x86a*\x0CV[\x89\x85R\x866` \x87\x017a7\\\x89\x83Q\x97a7\x1B\x85\x8Aa*\x0CV[\x8C\x89R\x896` \x8B\x017\x84Q\x99a72\x86\x8Ca*\x0CV[\x8D\x8BR6` \x8C\x017\x8C\x80`\xA0\x1B\x03a7K\x83\x86a/\x15V[Q\x16a7V\x89a/\x08V[Ra/\x15V[Q\x92_R`\xA2` R\x81_ \x90a7y\x8A\x8C\x80`\xA0\x1B\x03\x92a/\x15V[Q\x16\x8A\x80`\xA0\x1B\x03\x16_R` R_ \x90a\x0B\x94`\x01`\x01`@\x1B\x03a\x0B\x8C\x8A\x8Da/\x15V[a7\xA8\x83a/\x08V[R`\x01`\x01`@\x1B\x03a7\xBB\x86\x89a/\x15V[Q\x16a7\xC6\x84a/\x08V[R\x87\x8Ba9oV[a7\xD8\x82\x8Aa/\x15V[R\x8A_R`\xA2` R\x89_ \x82\x80`\xA0\x1B\x03a7\xF4\x83\x8Ca/\x15V[Q\x16\x83\x80`\xA0\x1B\x03\x16_R` Rg\r\xE0\xB6\xB3\xA7d\0\0\x8A_ U\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F``\x8C\x8Ba8D\x85\x87\x80`\xA0\x1B\x03\x92a/\x15V[Q\x16\x8DQ\x91\x82R` \x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x8D\x82\x01R\xA1\x01a6\xDCV[PPPPPP\x92PPP\x90V[a8\x86\x91\x96P=\x80_\x83>a\x07\xB5\x81\x83a*\x0CV[\x94_a6\xCFV[PP\x93PPPP``\x90V[\x85\x85\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A_\x80\xA3a6bV[c\x1EI\x9A#`\xE1\x1B_R`\x04_\xFD[P\x84_R`\x99` R`\x01\x80`\xA0\x1B\x03`\x01`@_ \x01T\x163\x14a6\x05V[P\x843\x14a5\xFEV[c\x11\xCA35`\xE3\x1B_R`\x04_\xFD[c\xA5\xC7\xC4E`\xE0\x1B_R`\x04_\xFD[a9\"aDRV[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra1;`b\x82a*\x0CV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x91\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[\x90\x94\x93\x90\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x92\x91\x90\x83\x15a$\x03W\x82Q\x15a=\x08Wa9\x98\x83Qa-\x8EV[\x91_[\x84Q\x81\x10\x15a<;Wa9\xB9`\x01`\x01`\xA0\x1B\x03a\n\xD6\x83\x88a/\x15V[_\x87\x81R`\xA2` R`@\x90 \x90\x91\x90`\x01`\x01`\xA0\x1B\x03a9\xDB\x83\x89a/\x15V[Q\x16`\x01\x80`\xA0\x1B\x03\x16_R` Ra9\xF6`@_ a1AV[\x91a:La:\x04\x83\x87a/\x15V[Qa:Ga:/`\x01`\x01`@\x1B\x03a:\x1D\x87\x8Aa/\x15V[Q\x16\x92a:)\x88aE\x94V[\x90aG\x81V[`\x01`\x01`@\x1B\x03a:@\x88aE\xA7V[\x16\x90aG\x81V[aG\x81V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x90\x91a:\xA6\x90` \x90a:k\x86\x8Ca/\x15V[Q\x16\x8C`@Q\x93\x84\x92\x83\x92c\xFE$:\x17`\xE0\x1B\x84R`\x04\x84\x01\x90\x92\x91` \x90`@\x83\x01\x94`\x01\x80`\xA0\x1B\x03\x16\x83R`\x01\x80`\xA0\x1B\x03\x16\x91\x01RV[\x03\x81\x88Z\xFA\x90\x81\x15a\x1EhW_\x91a<\nW[P\x82\x11a;\xFBWa;\x01\x90`\x01`\x01`\xA0\x1B\x03\x8D\x16\x80a;wW[Pa:Ga:\xE2\x85\x89a/\x15V[Q`\x01`\x01`@\x1B\x03a:@\x81a:\xF9\x89\x8Ca/\x15V[Q\x16\x94aE\xA7V[a;\x0B\x83\x88a/\x15V[R`\x01`\x01`\xA0\x1B\x03a;\x1E\x83\x89a/\x15V[Q\x16\x90\x83;\x15a\x0B\xE6Wa;M\x93_\x92\x83\x8C`@Q\x97\x88\x95\x86\x94\x85\x93crJ\xF4#`\xE0\x1B\x85R`\x04\x85\x01a9MV[\x03\x92Z\xF1\x91\x82\x15a\x1EhW`\x01\x92a;gW[P\x01a9\x9BV[_a;q\x91a*\x0CV[_a;`V[\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD`\x01`\x01`\xA0\x1B\x03a;\xAA\x87\x8Da/\x15V[Q\x16\x8Da;\xF2a;\xBA\x89\x8Da/\x15V[Q\x85_R`\x98` R`@_ `\x01\x80`\xA0\x1B\x03\x85\x16_R` R`@_ a;\xE4\x82\x82Ta1vV[\x90U`@Q\x93\x84\x93\x84a9MV[\x03\x90\xA2_a:\xD4V[c\xF0 \xE5\xB9`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=\x82\x11a<3W[\x81a<$` \x93\x83a*\x0CV[\x81\x01\x03\x12a\x0B\xE6WQ_a:\xB9V[=\x91Pa<\x17V[PPP_\x83\x81R`\x9F` R`@\x90 \x80T\x95\x96\x95\x91\x94P\x91\x92\x90\x91\x82_\x19\x81\x14a/zW`\x01\x01\x90U`@Q\x94a<r\x86a)\xF1V[\x81\x86R`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01R`@\x85\x01R``\x84\x01RBc\xFF\xFF\xFF\xFF\x16`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R\x7FX\x88?\x06x\xFFC\xA2\xC0I\xE6\xA3\xA7\xA8\xB9\xB0\xE9\x06)Y\xF3\xA9\x91\x92PX\x88\x19:\x0C_\xEDa=\x02a<\xD0\x83a1\x10V[\x92\x83_R`\x9E` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x91\x82\x91\x85\x83R`@` \x84\x01R`@\x83\x01\x90a0aV[\x03\x90\xA1\x90V[cyl\xC5%`\xE0\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a$\x03W`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R` \x81\x01\x84\x90R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`eUV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x92a>}\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x95a>\x8A\x93\x94\x95`\x01\x80`\xA0\x1B\x03\x16\x80_R`\x98` R`@_ `\x01\x80`\xA0\x1B\x03\x88\x16_R` R`@_ a>\x19\x85\x82Ta/\x8EV[\x90U\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l`@Q\x80a>L\x87\x8B\x8B\x84a9MV[\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R\x90\x81 \x93\x90\x84aE\x0FV[T`@Q\x93\x84\x93\x84a9MV[\x03\x90\xA1V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x99` R`@\x90 `\x045\x91\x82\x16\x82\x03a\x0B\xE6W\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x81U`\x01\x01\x90`$5\x90\x81\x16\x81\x03a\x0B\xE6W\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x81U`D5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x0B\xE6W\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90U`@Q``\x81\x01\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90`\x01`\x01`\xA0\x1B\x03a?b`\x04a(\xECV[\x16\x81R`\x01`\x01`\xA0\x1B\x03a?w`$a(\xECV[\x16` \x82\x01Rc\xFF\xFF\xFF\xFFa?\x8C`Da)iV[\x16`@\x82\x01R\x803\x93\x03\x90\xA2V[a?\xCEa*\xB3\x93\x92`\x01`\x01`@\x1B\x03a?\xC7a?\xC1\x82\x95a?\xBB\x85aE\x94V[\x90aF\xFDV[\x92aE\xA7V[\x16\x90aF\xFDV[\x91\x16\x90aF\xFDV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a?\xEAWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[\x90a@9\x92\x91aE\xCEV[\x15a@@WV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[\x91\x93\x92\x93`\xA0\x83\x01\x90a@b\x82\x85a-\xF6V[\x90P\x81\x03a(\x18W`\x01`\x01`\xA0\x1B\x03a@~`@\x86\x01a.+V[\x163\x03aC\xCDWa@\x92a\x19\x8B6\x86a+\x02V[\x94\x85_R`\x9E` R`\xFF`@_ T\x16\x15aC\xBEW`\x80\x85\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x0B\xE6Wa@\xC3\x90a/\x9BV[\x90_a@\xD1` \x88\x01a.+V[a@\xDB\x86\x89a-\xF6V[`@Qc\x84;4\x9F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\x04\x84\x01R```$\x84\x01R\x91\x94\x85\x92\x83\x92c\xFF\xFF\xFF\xFF\x91aA\x1A\x91`d\x86\x01\x91\x90a.\xC9V[\x91\x16`D\x83\x01R\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x1EhW_\x92aC\xA2W[P_[aAj\x85\x88a-\xF6V[\x90P\x81\x10\x15aCZWaA\x97aA\x92aA\x8D\x83aA\x87\x89\x8Ca-\xF6V[\x90a5&V[a.+V[aC\xDCV[\x90\x87aB\x13\x85a\x0EGaA\xB1\x85aA\x87`\xC0\x87\x01\x87a-\xF6V[5`\x01`\x01`\xA0\x1B\x03aA\xC3\x86a.+V[\x16_R`\xA2` R\x8AaA\xE1aA\x8D\x88aA\x87`@_ \x94\x8Aa-\xF6V[`\x01\x80`\xA0\x1B\x03\x16_R` R`\x01`\x01`@\x1B\x03a?\xC7a\x17\x9C\x82aB\x0B\x8A`@_ \x98a/\x15V[Q\x16\x95a1AV[\x92\x8B\x15aB\xBEW`\x01`\x01`\xA0\x1B\x03\x16aB=aA\x8D\x84aA\x87\x8BaB7\x87a.+V[\x96a-\xF6V[\x90aBLaA\x8D\x85\x8A\x8Da5&V[\x91\x81;\x15a\x0B\xE6W`@Qc\x0B\xAB\x90c`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x90\x84\x16`$\x82\x01R\x91\x90\x92\x16`D\x82\x01R`d\x81\x01\x93\x90\x93R_\x90\x83\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x1EhW`\x01\x92aB\xAEW[P[\x01aA`V[_aB\xB8\x91a*\x0CV[_aB\xA6V[`\x01`\x01`\xA0\x1B\x03\x16aB\xDBaA\x8D\x84aA\x87\x8BaB7\x87a.+V[\x90aB\xEAaA\x8D\x85\x8A\x8Da5&V[\x91\x81;\x15a\x0B\xE6W`@Qc\xC4b>\xA1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x90\x84\x16`$\x82\x01R\x91\x90\x92\x16`D\x82\x01R`d\x81\x01\x93\x90\x93R_\x90\x83\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x1EhW`\x01\x92aCJW[PaB\xA8V[_aCT\x91a*\x0CV[_aCDV[P\x96PPPPPP` \x7F\x1F@@\x08\x89'N\xD0{$\x84^PT\xA8z\x0C\xAB\x96\x9E\xB1'z\xAF\xE6\x1A\xE3R\xE7\xC3*\0\x91\x80_R`\x9E\x82R`@_ `\xFF\x19\x81T\x16\x90U`@Q\x90\x81R\xA1V[aC\xB7\x91\x92P=\x80_\x83>a\x07\xB5\x81\x83a*\x0CV[\x90_aA]V[c\x87\xC9\xD2\x19`\xE0\x1B_R`\x04_\xFD[c\x16\x11\r5`\xE2\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x16_Q` aI^_9_Q\x90_R\x03aD%W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90V[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03aD\x9DW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`\n` `@QaD\xAF`@\x82a*\x0CV[\x82\x81R\x01i\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B\x81R `@Q` \x81\x01\x91\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x83R`@\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\x80\x81Ra1;`\xA0\x82a*\x0CV[\x92\x91\x81\x15aEpW\x91`\x01`\x01`@\x1B\x03aEPaEX\x93a:)\x86aEJaEl\x98aEE\x88aE?\x8Da1AV[\x87a?\x9AV[a/\x8EV[\x92a/\x8EV[\x91\x16\x90aG\x81V[`\x01`\x01`@\x1B\x03a:@a\x17\x9C\x85a1AV[\x90UV[PPaEl\x90`\x01`\x01`@\x1B\x03aEP\x81aE\x8Ea\x17\x9C\x87a1AV[\x16aH3V[Q\x80a*\xB3WPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[` \x81\x01Q\x15aE\xC1W`@\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[\x91\x90\x91aE\xDB\x82\x84aHMV[`\x05\x81\x10\x15aF\xE9W\x15\x90\x81aF\xD3W[PaF\xCBW_\x92` aFK`\x84\x86\x95`@Q\x93\x84\x91\x81\x83\x01\x96c\x0B\x13]?`\xE1\x1B\x88R`$\x84\x01R`@`D\x84\x01R\x80Q\x91\x82\x91\x82`d\x86\x01R\x01\x84\x84\x01^\x87\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01`\x1F\x19\x81\x01\x83R\x82a*\x0CV[Q\x91Z\xFA=\x15aF\xC4W=aF_\x81a,EV[\x90aFm`@Q\x92\x83a*\x0CV[\x81R=_` \x83\x01>[\x81aF\xB8W[\x81aF\x86WP\x90V[\x90P` \x81\x80Q\x81\x01\x03\x12a\x0B\xE6W` \x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x90\x81\x90\x03a\x0B\xE6Wc\x0B\x13]?`\xE1\x1B\x14\x90V[\x80Q` \x14\x91PaF}V[``aFwV[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_aE\xECV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90\x91\x90_\x90_\x19\x84\x82\t\x90\x84\x81\x02\x92\x83\x80\x84\x10\x93\x03\x92\x80\x84\x03\x93\x14aGnW\x82g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x03^WP\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x93\x94g\r\xE0\xB6\xB3\xA7d\0\0\x91\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[PPPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x92P\x04\x90V[_\x19g\r\xE0\xB6\xB3\xA7d\0\0\x82\t\x91g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x82\x80\x85\x10\x94\x03\x93\x80\x85\x03\x94\x14aH\x12W\x83\x82\x11\x15a\x0B\xE6Wg\r\xE0\xB6\xB3\xA7d\0\0\x82\x91\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x81`\x03\x02\x18\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x91\x02`\x02\x03\x02\x93`\x01\x83\x80_\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[P\x80\x92P\x15aH\x1FW\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x80\x15aH\x1FWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[\x81Q`A\x81\x03aHyWP\x90aHu\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aH\xBBV[\x90\x91V[`@\x03aH\xB2W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a/zWaHu\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90aH\xBBV[PP_\x90`\x02\x90V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aIRW`\xFF\x16\x90`\x1B\x82\x14\x15\x80aIGW[aI<W` \x93_\x93`\x80\x93`@Q\x93\x84R\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x1EhW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aI4W\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x82\x14\x15aH\xF2V[PPPP_\x90`\x03\x90V\xFE\0\0\0\0\0\0\0\0\0\0\0\0\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\xA2dipfsX\"\x12 i\xE9\xBB\x87\x8E\xD5Cq\xB6\x8Ar\x89\xAEU\x05_8\xA1a(\xD5xH6\xA6\x96E\x9C\xFA\xBC\xFBLdsolcC\0\x08\x1B\x003",
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
    /**Custom error with signature `AllocationDelaySet()` and selector `0x82cb189f`.
```solidity
error AllocationDelaySet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AllocationDelaySet {}
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
        impl ::core::convert::From<AllocationDelaySet> for UnderlyingRustTuple<'_> {
            fn from(value: AllocationDelaySet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AllocationDelaySet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AllocationDelaySet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AllocationDelaySet()";
            const SELECTOR: [u8; 4] = [130u8, 203u8, 24u8, 159u8];
            #[inline]
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
    /**Custom error with signature `UnauthorizedCaller()` and selector `0x5c427cd9`.
```solidity
error UnauthorizedCaller();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnauthorizedCaller {}
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
        impl ::core::convert::From<UnauthorizedCaller> for UnderlyingRustTuple<'_> {
            fn from(value: UnauthorizedCaller) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnauthorizedCaller {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnauthorizedCaller {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnauthorizedCaller()";
            const SELECTOR: [u8; 4] = [92u8, 66u8, 124u8, 217u8];
            #[inline]
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
    /**Custom error with signature `WithdrawalDelayExceedsMax()` and selector `0x9373e7cf`.
```solidity
error WithdrawalDelayExceedsMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalDelayExceedsMax {}
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
        impl ::core::convert::From<WithdrawalDelayExceedsMax>
        for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalDelayExceedsMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for WithdrawalDelayExceedsMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalDelayExceedsMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawalDelayExceedsMax()";
            const SELECTOR: [u8; 4] = [147u8, 115u8, 231u8, 207u8];
            #[inline]
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
    /**Custom error with signature `WithdrawalDelayExeedsMax()` and selector `0x11a255bc`.
```solidity
error WithdrawalDelayExeedsMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalDelayExeedsMax {}
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
        impl ::core::convert::From<WithdrawalDelayExeedsMax>
        for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalDelayExeedsMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for WithdrawalDelayExeedsMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalDelayExeedsMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawalDelayExeedsMax()";
            const SELECTOR: [u8; 4] = [17u8, 162u8, 85u8, 188u8];
            #[inline]
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
    /**Custom error with signature `WithdrawalDoesNotExist()` and selector `0x52b4cdc2`.
```solidity
error WithdrawalDoesNotExist();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalDoesNotExist {}
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
        impl ::core::convert::From<WithdrawalDoesNotExist> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawalDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawalDoesNotExist()";
            const SELECTOR: [u8; 4] = [82u8, 180u8, 205u8, 194u8];
            #[inline]
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
    /**Event with signature `BeaconChainScalingFactorDecreased(address,uint64)` and selector `0xddf935ec8825c7afee6a15d4731e28963ee96dfcb85d0a1e794b43318bbca4fd`.
```solidity
event BeaconChainScalingFactorDecreased(address staker, uint64 newBeaconChainScalingFactor);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BeaconChainScalingFactorDecreased {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newBeaconChainScalingFactor: u64,
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
        impl alloy_sol_types::SolEvent for BeaconChainScalingFactorDecreased {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "BeaconChainScalingFactorDecreased(address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8,
                249u8,
                53u8,
                236u8,
                136u8,
                37u8,
                199u8,
                175u8,
                238u8,
                106u8,
                21u8,
                212u8,
                115u8,
                30u8,
                40u8,
                150u8,
                62u8,
                233u8,
                109u8,
                252u8,
                184u8,
                93u8,
                10u8,
                30u8,
                121u8,
                75u8,
                67u8,
                49u8,
                139u8,
                188u8,
                164u8,
                253u8,
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
                    newBeaconChainScalingFactor: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newBeaconChainScalingFactor,
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
        impl alloy_sol_types::private::IntoLogData
        for BeaconChainScalingFactorDecreased {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BeaconChainScalingFactorDecreased>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &BeaconChainScalingFactorDecreased,
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
    /**Event with signature `OperatorDetailsModified(address,(address,address,uint32))` and selector `0xfebe5cd24b2cbc7b065b9d0fdeb904461e4afcff57dd57acda1e7832031ba7ac`.
```solidity
event OperatorDetailsModified(address indexed operator, IDelegationManagerTypes.OperatorDetails newOperatorDetails);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorDetailsModified {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOperatorDetails: <IDelegationManagerTypes::OperatorDetails as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorDetailsModified {
            type DataTuple<'a> = (IDelegationManagerTypes::OperatorDetails,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorDetailsModified(address,(address,address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                254u8,
                190u8,
                92u8,
                210u8,
                75u8,
                44u8,
                188u8,
                123u8,
                6u8,
                91u8,
                157u8,
                15u8,
                222u8,
                185u8,
                4u8,
                70u8,
                30u8,
                74u8,
                252u8,
                255u8,
                87u8,
                221u8,
                87u8,
                172u8,
                218u8,
                30u8,
                120u8,
                50u8,
                3u8,
                27u8,
                167u8,
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
                    operator: topics.1,
                    newOperatorDetails: data.0,
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
                    <IDelegationManagerTypes::OperatorDetails as alloy_sol_types::SolType>::tokenize(
                        &self.newOperatorDetails,
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
        impl alloy_sol_types::private::IntoLogData for OperatorDetailsModified {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDetailsModified> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorDetailsModified,
            ) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `OperatorRegistered(address,(address,address,uint32))` and selector `0x8e8485583a2310d41f7c82b9427d0bd49bad74bb9cff9d3402a29d8f9b28a0e2`.
```solidity
event OperatorRegistered(address indexed operator, IDelegationManagerTypes.OperatorDetails operatorDetails);
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
        pub operatorDetails: <IDelegationManagerTypes::OperatorDetails as alloy::sol_types::SolType>::RustType,
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
            type DataTuple<'a> = (IDelegationManagerTypes::OperatorDetails,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,(address,address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                142u8,
                132u8,
                133u8,
                88u8,
                58u8,
                35u8,
                16u8,
                212u8,
                31u8,
                124u8,
                130u8,
                185u8,
                66u8,
                125u8,
                11u8,
                212u8,
                155u8,
                173u8,
                116u8,
                187u8,
                156u8,
                255u8,
                157u8,
                52u8,
                2u8,
                162u8,
                157u8,
                143u8,
                155u8,
                40u8,
                160u8,
                226u8,
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
                    operatorDetails: data.0,
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
                    <IDelegationManagerTypes::OperatorDetails as alloy_sol_types::SolType>::tokenize(
                        &self.operatorDetails,
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
    /**Event with signature `PauserRegistrySet(address,address)` and selector `0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6`.
```solidity
event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PauserRegistrySet {
        #[allow(missing_docs)]
        pub pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPauserRegistry: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PauserRegistrySet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PauserRegistrySet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                110u8,
                159u8,
                205u8,
                83u8,
                152u8,
                150u8,
                252u8,
                166u8,
                14u8,
                139u8,
                15u8,
                1u8,
                221u8,
                88u8,
                2u8,
                51u8,
                228u8,
                138u8,
                107u8,
                15u8,
                125u8,
                240u8,
                19u8,
                184u8,
                155u8,
                167u8,
                245u8,
                101u8,
                134u8,
                154u8,
                205u8,
                182u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    pauserRegistry: data.0,
                    newPauserRegistry: data.1,
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
                        &self.pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newPauserRegistry,
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
        impl alloy_sol_types::private::IntoLogData for PauserRegistrySet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PauserRegistrySet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PauserRegistrySet) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `SlashingWithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x58883f0678ff43a2c049e6a3a7a8b9b0e9062959f3a99192505888193a0c5fed`.
```solidity
event SlashingWithdrawalQueued(bytes32 withdrawalRoot, IDelegationManagerTypes.Withdrawal withdrawal);
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
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlashingWithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                88u8,
                136u8,
                63u8,
                6u8,
                120u8,
                255u8,
                67u8,
                162u8,
                192u8,
                73u8,
                230u8,
                163u8,
                167u8,
                168u8,
                185u8,
                176u8,
                233u8,
                6u8,
                41u8,
                89u8,
                243u8,
                169u8,
                145u8,
                146u8,
                80u8,
                88u8,
                136u8,
                25u8,
                58u8,
                12u8,
                95u8,
                237u8,
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
constructor(address _avsDirectory, address _strategyManager, address _eigenPodManager, address _allocationManager, uint32 _MIN_WITHDRAWAL_DELAY);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _avsDirectory: alloy::sol_types::private::Address,
        pub _strategyManager: alloy::sol_types::private::Address,
        pub _eigenPodManager: alloy::sol_types::private::Address,
        pub _allocationManager: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
                        value._avsDirectory,
                        value._strategyManager,
                        value._eigenPodManager,
                        value._allocationManager,
                        value._MIN_WITHDRAWAL_DELAY,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _avsDirectory: tuple.0,
                        _strategyManager: tuple.1,
                        _eigenPodManager: tuple.2,
                        _allocationManager: tuple.3,
                        _MIN_WITHDRAWAL_DELAY: tuple.4,
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
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._strategyManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._eigenPodManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
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
    /**Function with signature `LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS()` and selector `0xcb00387b`.
```solidity
function LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall {}
    ///Container type for the return parameters of the [`LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS()`](LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSReturn {
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
            impl ::core::convert::From<LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall {
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
            impl ::core::convert::From<LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS()";
            const SELECTOR: [u8; 4] = [203u8, 0u8, 56u8, 123u8];
            #[inline]
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
    /**Function with signature `LEGACY_WITHDRAWAL_CHECK_VALUE()` and selector `0xcebc04ef`.
```solidity
function LEGACY_WITHDRAWAL_CHECK_VALUE() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LEGACY_WITHDRAWAL_CHECK_VALUECall {}
    ///Container type for the return parameters of the [`LEGACY_WITHDRAWAL_CHECK_VALUE()`](LEGACY_WITHDRAWAL_CHECK_VALUECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LEGACY_WITHDRAWAL_CHECK_VALUEReturn {
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
            impl ::core::convert::From<LEGACY_WITHDRAWAL_CHECK_VALUECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: LEGACY_WITHDRAWAL_CHECK_VALUECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for LEGACY_WITHDRAWAL_CHECK_VALUECall {
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
            impl ::core::convert::From<LEGACY_WITHDRAWAL_CHECK_VALUEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: LEGACY_WITHDRAWAL_CHECK_VALUEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for LEGACY_WITHDRAWAL_CHECK_VALUEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for LEGACY_WITHDRAWAL_CHECK_VALUECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = LEGACY_WITHDRAWAL_CHECK_VALUEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LEGACY_WITHDRAWAL_CHECK_VALUE()";
            const SELECTOR: [u8; 4] = [206u8, 188u8, 4u8, 239u8];
            #[inline]
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
    /**Function with signature `MIN_WITHDRAWAL_DELAY()` and selector `0x4a5f2b5d`.
```solidity
function MIN_WITHDRAWAL_DELAY() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MIN_WITHDRAWAL_DELAYCall {}
    ///Container type for the return parameters of the [`MIN_WITHDRAWAL_DELAY()`](MIN_WITHDRAWAL_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MIN_WITHDRAWAL_DELAYReturn {
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
            impl ::core::convert::From<MIN_WITHDRAWAL_DELAYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MIN_WITHDRAWAL_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MIN_WITHDRAWAL_DELAYCall {
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
            impl ::core::convert::From<MIN_WITHDRAWAL_DELAYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MIN_WITHDRAWAL_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MIN_WITHDRAWAL_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MIN_WITHDRAWAL_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MIN_WITHDRAWAL_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MIN_WITHDRAWAL_DELAY()";
            const SELECTOR: [u8; 4] = [74u8, 95u8, 43u8, 93u8];
            #[inline]
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
    /**Function with signature `STAKER_DELEGATION_TYPEHASH()` and selector `0x43377382`.
```solidity
function STAKER_DELEGATION_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct STAKER_DELEGATION_TYPEHASHCall {}
    ///Container type for the return parameters of the [`STAKER_DELEGATION_TYPEHASH()`](STAKER_DELEGATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct STAKER_DELEGATION_TYPEHASHReturn {
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
            impl ::core::convert::From<STAKER_DELEGATION_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: STAKER_DELEGATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for STAKER_DELEGATION_TYPEHASHCall {
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
            impl ::core::convert::From<STAKER_DELEGATION_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: STAKER_DELEGATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for STAKER_DELEGATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for STAKER_DELEGATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = STAKER_DELEGATION_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "STAKER_DELEGATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [67u8, 55u8, 115u8, 130u8];
            #[inline]
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
    /**Function with signature `avsDirectory()` and selector `0x6b3aa72e`.
```solidity
function avsDirectory() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryCall {}
    ///Container type for the return parameters of the [`avsDirectory()`](avsDirectoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryReturn {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `calculateCurrentStakerDelegationDigestHash(address,address,uint256)` and selector `0x1bbce091`.
```solidity
function calculateCurrentStakerDelegationDigestHash(address staker, address operator, uint256 expiry) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateCurrentStakerDelegationDigestHashCall {
        pub staker: alloy::sol_types::private::Address,
        pub operator: alloy::sol_types::private::Address,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateCurrentStakerDelegationDigestHash(address,address,uint256)`](calculateCurrentStakerDelegationDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateCurrentStakerDelegationDigestHashReturn {
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<calculateCurrentStakerDelegationDigestHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateCurrentStakerDelegationDigestHashCall) -> Self {
                    (value.staker, value.operator, value.expiry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateCurrentStakerDelegationDigestHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        operator: tuple.1,
                        expiry: tuple.2,
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
            impl ::core::convert::From<calculateCurrentStakerDelegationDigestHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: calculateCurrentStakerDelegationDigestHashReturn,
                ) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateCurrentStakerDelegationDigestHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for calculateCurrentStakerDelegationDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateCurrentStakerDelegationDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateCurrentStakerDelegationDigestHash(address,address,uint256)";
            const SELECTOR: [u8; 4] = [27u8, 188u8, 224u8, 145u8];
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
    /**Function with signature `calculateStakerDelegationDigestHash(address,uint256,address,uint256)` and selector `0xc94b5111`.
```solidity
function calculateStakerDelegationDigestHash(address staker, uint256 nonce, address operator, uint256 expiry) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateStakerDelegationDigestHashCall {
        pub staker: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub operator: alloy::sol_types::private::Address,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateStakerDelegationDigestHash(address,uint256,address,uint256)`](calculateStakerDelegationDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateStakerDelegationDigestHashReturn {
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<calculateStakerDelegationDigestHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateStakerDelegationDigestHashCall) -> Self {
                    (value.staker, value.nonce, value.operator, value.expiry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateStakerDelegationDigestHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        nonce: tuple.1,
                        operator: tuple.2,
                        expiry: tuple.3,
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
            impl ::core::convert::From<calculateStakerDelegationDigestHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateStakerDelegationDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateStakerDelegationDigestHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateStakerDelegationDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateStakerDelegationDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateStakerDelegationDigestHash(address,uint256,address,uint256)";
            const SELECTOR: [u8; 4] = [201u8, 75u8, 81u8, 17u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
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
    /**Function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`.
```solidity
function cumulativeWithdrawalsQueued(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeWithdrawalsQueuedCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`cumulativeWithdrawalsQueued(address)`](cumulativeWithdrawalsQueuedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeWithdrawalsQueuedReturn {
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
            impl ::core::convert::From<cumulativeWithdrawalsQueuedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeWithdrawalsQueuedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeWithdrawalsQueuedCall {
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
            impl ::core::convert::From<cumulativeWithdrawalsQueuedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeWithdrawalsQueuedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeWithdrawalsQueuedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
    /**Function with signature `decreaseBeaconChainScalingFactor(address,uint256,uint64)` and selector `0x5d9aed23`.
```solidity
function decreaseBeaconChainScalingFactor(address staker, uint256 existingDepositShares, uint64 proportionOfOldBalance) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseBeaconChainScalingFactorCall {
        pub staker: alloy::sol_types::private::Address,
        pub existingDepositShares: alloy::sol_types::private::primitives::aliases::U256,
        pub proportionOfOldBalance: u64,
    }
    ///Container type for the return parameters of the [`decreaseBeaconChainScalingFactor(address,uint256,uint64)`](decreaseBeaconChainScalingFactorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseBeaconChainScalingFactorReturn {}
    #[allow(
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
            impl ::core::convert::From<decreaseBeaconChainScalingFactorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: decreaseBeaconChainScalingFactorCall) -> Self {
                    (
                        value.staker,
                        value.existingDepositShares,
                        value.proportionOfOldBalance,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decreaseBeaconChainScalingFactorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        existingDepositShares: tuple.1,
                        proportionOfOldBalance: tuple.2,
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
            impl ::core::convert::From<decreaseBeaconChainScalingFactorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: decreaseBeaconChainScalingFactorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decreaseBeaconChainScalingFactorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decreaseBeaconChainScalingFactorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = decreaseBeaconChainScalingFactorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decreaseBeaconChainScalingFactor(address,uint256,uint64)";
            const SELECTOR: [u8; 4] = [93u8, 154u8, 237u8, 35u8];
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.existingDepositShares,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.proportionOfOldBalance,
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
    /**Function with signature `decreaseOperatorShares(address,address,uint64,uint64)` and selector `0xa57ab10b`.
```solidity
function decreaseOperatorShares(address operator, address strategy, uint64 previousTotalMagnitude, uint64 newTotalMagnitude) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseOperatorSharesCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub previousTotalMagnitude: u64,
        pub newTotalMagnitude: u64,
    }
    ///Container type for the return parameters of the [`decreaseOperatorShares(address,address,uint64,uint64)`](decreaseOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseOperatorSharesReturn {}
    #[allow(
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
            impl ::core::convert::From<decreaseOperatorSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: decreaseOperatorSharesCall) -> Self {
                    (
                        value.operator,
                        value.strategy,
                        value.previousTotalMagnitude,
                        value.newTotalMagnitude,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decreaseOperatorSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                        previousTotalMagnitude: tuple.2,
                        newTotalMagnitude: tuple.3,
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
            impl ::core::convert::From<decreaseOperatorSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: decreaseOperatorSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decreaseOperatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decreaseOperatorSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = decreaseOperatorSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decreaseOperatorShares(address,address,uint64,uint64)";
            const SELECTOR: [u8; 4] = [165u8, 122u8, 177u8, 11u8];
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.previousTotalMagnitude,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.newTotalMagnitude),
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
    /**Function with signature `delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)` and selector `0x7f548071`.
```solidity
function delegateToBySignature(address staker, address operator, ISignatureUtils.SignatureWithExpiry memory stakerSignatureAndExpiry, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToBySignatureCall {
        pub staker: alloy::sol_types::private::Address,
        pub operator: alloy::sol_types::private::Address,
        pub stakerSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        pub approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)`](delegateToBySignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToBySignatureReturn {}
    #[allow(
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
                ISignatureUtils::SignatureWithExpiry,
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<delegateToBySignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegateToBySignatureCall) -> Self {
                    (
                        value.staker,
                        value.operator,
                        value.stakerSignatureAndExpiry,
                        value.approverSignatureAndExpiry,
                        value.approverSalt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegateToBySignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        operator: tuple.1,
                        stakerSignatureAndExpiry: tuple.2,
                        approverSignatureAndExpiry: tuple.3,
                        approverSalt: tuple.4,
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
            impl ::core::convert::From<delegateToBySignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegateToBySignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegateToBySignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateToBySignatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithExpiry,
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateToBySignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)";
            const SELECTOR: [u8; 4] = [127u8, 84u8, 128u8, 113u8];
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
                    <ISignatureUtils::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.stakerSignatureAndExpiry,
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
function delegatedTo(address) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedToCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegatedTo(address)`](delegatedToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedToReturn {
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
            impl ::core::convert::From<delegatedToCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegatedToCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatedToCall {
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
            impl ::core::convert::From<delegatedToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegatedToReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatedToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
function delegationApproverSaltIsSpent(address, bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverSaltIsSpentCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegationApproverSaltIsSpent(address,bytes32)`](delegationApproverSaltIsSpentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverSaltIsSpentReturn {
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
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationApproverSaltIsSpentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationApproverSaltIsSpentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
    /**Function with signature `getCompletableTimestamp(uint32)` and selector `0x15c4a288`.
```solidity
function getCompletableTimestamp(uint32 startTimestamp) external view returns (uint32 completableTimestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompletableTimestampCall {
        pub startTimestamp: u32,
    }
    ///Container type for the return parameters of the [`getCompletableTimestamp(uint32)`](getCompletableTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompletableTimestampReturn {
        pub completableTimestamp: u32,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCompletableTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompletableTimestampCall) -> Self {
                    (value.startTimestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompletableTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { startTimestamp: tuple.0 }
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
            impl ::core::convert::From<getCompletableTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompletableTimestampReturn) -> Self {
                    (value.completableTimestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompletableTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        completableTimestamp: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCompletableTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCompletableTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCompletableTimestamp(uint32)";
            const SELECTOR: [u8; 4] = [21u8, 196u8, 162u8, 136u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
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
    /**Function with signature `getWithdrawableShares(address,address[])` and selector `0xc978f7ac`.
```solidity
function getWithdrawableShares(address staker, address[] memory strategies) external view returns (uint256[] memory withdrawableShares);
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
            impl ::core::convert::From<getWithdrawableSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawableSharesReturn) -> Self {
                    (value.withdrawableShares,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawableSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawableShares: tuple.0,
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
function increaseDelegatedShares(address staker, address strategy, uint256 existingDepositShares, uint256 addedShares) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseDelegatedSharesCall {
        pub staker: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub existingDepositShares: alloy::sol_types::private::primitives::aliases::U256,
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
                        value.existingDepositShares,
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
                        existingDepositShares: tuple.2,
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.existingDepositShares,
                    ),
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
    /**Function with signature `initialize(address,address,uint256)` and selector `0x1794bb3c`.
```solidity
function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub initialOwner: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`initialize(address,address,uint256)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value.initialOwner,
                        value._pauserRegistry,
                        value.initialPausedStatus,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        _pauserRegistry: tuple.1,
                        initialPausedStatus: tuple.2,
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
            const SIGNATURE: &'static str = "initialize(address,address,uint256)";
            const SELECTOR: [u8; 4] = [23u8, 148u8, 187u8, 60u8];
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
                        &self._pauserRegistry,
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
    /**Function with signature `modifyOperatorDetails((address,address,uint32))` and selector `0xf16172b0`.
```solidity
function modifyOperatorDetails(IDelegationManagerTypes.OperatorDetails memory newOperatorDetails) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyOperatorDetailsCall {
        pub newOperatorDetails: <IDelegationManagerTypes::OperatorDetails as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`modifyOperatorDetails((address,address,uint32))`](modifyOperatorDetailsCall) function.
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
            type UnderlyingSolTuple<'a> = (IDelegationManagerTypes::OperatorDetails,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::OperatorDetails as alloy::sol_types::SolType>::RustType,
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
                    (value.newOperatorDetails,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyOperatorDetailsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newOperatorDetails: tuple.0,
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
            type Parameters<'a> = (IDelegationManagerTypes::OperatorDetails,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyOperatorDetailsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyOperatorDetails((address,address,uint32))";
            const SELECTOR: [u8; 4] = [241u8, 97u8, 114u8, 176u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManagerTypes::OperatorDetails as alloy_sol_types::SolType>::tokenize(
                        &self.newOperatorDetails,
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
    /**Function with signature `operatorDetails(address)` and selector `0xc5e480db`.
```solidity
function operatorDetails(address operator) external view returns (IDelegationManagerTypes.OperatorDetails memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorDetailsCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorDetails(address)`](operatorDetailsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorDetailsReturn {
        pub _0: <IDelegationManagerTypes::OperatorDetails as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<operatorDetailsCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorDetailsCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorDetailsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IDelegationManagerTypes::OperatorDetails,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::OperatorDetails as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<operatorDetailsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorDetailsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorDetailsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorDetailsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorDetailsReturn;
            type ReturnTuple<'a> = (IDelegationManagerTypes::OperatorDetails,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorDetails(address)";
            const SELECTOR: [u8; 4] = [197u8, 228u8, 128u8, 219u8];
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
    /**Function with signature `operatorShares(address,address)` and selector `0x778e55f3`.
```solidity
function operatorShares(address, address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSharesCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorShares(address,address)`](operatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSharesReturn {
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
            impl ::core::convert::From<operatorSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorSharesCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
function pendingWithdrawals(bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingWithdrawalsCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`pendingWithdrawals(bytes32)`](pendingWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingWithdrawalsReturn {
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
            impl ::core::convert::From<pendingWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingWithdrawalsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingWithdrawalsCall {
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
            impl ::core::convert::From<pendingWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
    /**Function with signature `registerAsOperator((address,address,uint32),uint32,string)` and selector `0x49730060`.
```solidity
function registerAsOperator(IDelegationManagerTypes.OperatorDetails memory registeringOperatorDetails, uint32 allocationDelay, string memory metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAsOperatorCall {
        pub registeringOperatorDetails: <IDelegationManagerTypes::OperatorDetails as alloy::sol_types::SolType>::RustType,
        pub allocationDelay: u32,
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`registerAsOperator((address,address,uint32),uint32,string)`](registerAsOperatorCall) function.
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
                IDelegationManagerTypes::OperatorDetails,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::OperatorDetails as alloy::sol_types::SolType>::RustType,
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
                        value.registeringOperatorDetails,
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
                        registeringOperatorDetails: tuple.0,
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
                IDelegationManagerTypes::OperatorDetails,
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
            const SIGNATURE: &'static str = "registerAsOperator((address,address,uint32),uint32,string)";
            const SELECTOR: [u8; 4] = [73u8, 115u8, 0u8, 96u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManagerTypes::OperatorDetails as alloy_sol_types::SolType>::tokenize(
                        &self.registeringOperatorDetails,
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
    /**Function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`.
```solidity
function setPauserRegistry(address newPauserRegistry) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryCall {
        pub newPauserRegistry: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setPauserRegistry(address)`](setPauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryReturn {}
    #[allow(
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
            impl ::core::convert::From<setPauserRegistryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryCall) -> Self {
                    (value.newPauserRegistry,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPauserRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPauserRegistry: tuple.0 }
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
            impl ::core::convert::From<setPauserRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPauserRegistryCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPauserRegistryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPauserRegistry(address)";
            const SELECTOR: [u8; 4] = [16u8, 214u8, 122u8, 47u8];
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
                        &self.newPauserRegistry,
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
    /**Function with signature `stakerNonce(address)` and selector `0x29c77d4f`.
```solidity
function stakerNonce(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakerNonceCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`stakerNonce(address)`](stakerNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakerNonceReturn {
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
            impl ::core::convert::From<stakerNonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakerNonceCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakerNonceCall {
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
            impl ::core::convert::From<stakerNonceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakerNonceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakerNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakerNonceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakerNonceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakerNonce(address)";
            const SELECTOR: [u8; 4] = [41u8, 199u8, 125u8, 79u8];
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
    /**Function with signature `stakerScalingFactor(address,address)` and selector `0x457c6070`.
```solidity
function stakerScalingFactor(address, address) external view returns (uint256 depositScalingFactor, bool isBeaconChainScalingFactorSet, uint64 beaconChainScalingFactor);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakerScalingFactorCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`stakerScalingFactor(address,address)`](stakerScalingFactorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakerScalingFactorReturn {
        pub depositScalingFactor: alloy::sol_types::private::primitives::aliases::U256,
        pub isBeaconChainScalingFactorSet: bool,
        pub beaconChainScalingFactor: u64,
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
            impl ::core::convert::From<stakerScalingFactorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: stakerScalingFactorCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stakerScalingFactorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                bool,
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
            impl ::core::convert::From<stakerScalingFactorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: stakerScalingFactorReturn) -> Self {
                    (
                        value.depositScalingFactor,
                        value.isBeaconChainScalingFactorSet,
                        value.beaconChainScalingFactor,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stakerScalingFactorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        depositScalingFactor: tuple.0,
                        isBeaconChainScalingFactorSet: tuple.1,
                        beaconChainScalingFactor: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakerScalingFactorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakerScalingFactorReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakerScalingFactor(address,address)";
            const SELECTOR: [u8; 4] = [69u8, 124u8, 96u8, 112u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
    /**Function with signature `updateOperatorMetadataURI(string)` and selector `0x99be81c8`.
```solidity
function updateOperatorMetadataURI(string memory metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorMetadataURICall {
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateOperatorMetadataURI(string)`](updateOperatorMetadataURICall) function.
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
            impl ::core::convert::From<updateOperatorMetadataURICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorMetadataURICall) -> Self {
                    (value.metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { metadataURI: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorMetadataURI(string)";
            const SELECTOR: [u8; 4] = [153u8, 190u8, 129u8, 200u8];
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
        LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS(LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall),
        LEGACY_WITHDRAWAL_CHECK_VALUE(LEGACY_WITHDRAWAL_CHECK_VALUECall),
        MIN_WITHDRAWAL_DELAY(MIN_WITHDRAWAL_DELAYCall),
        STAKER_DELEGATION_TYPEHASH(STAKER_DELEGATION_TYPEHASHCall),
        allocationManager(allocationManagerCall),
        avsDirectory(avsDirectoryCall),
        beaconChainETHStrategy(beaconChainETHStrategyCall),
        calculateCurrentStakerDelegationDigestHash(
            calculateCurrentStakerDelegationDigestHashCall,
        ),
        calculateDelegationApprovalDigestHash(calculateDelegationApprovalDigestHashCall),
        calculateStakerDelegationDigestHash(calculateStakerDelegationDigestHashCall),
        calculateWithdrawalRoot(calculateWithdrawalRootCall),
        completeQueuedWithdrawal(completeQueuedWithdrawalCall),
        completeQueuedWithdrawals(completeQueuedWithdrawalsCall),
        cumulativeWithdrawalsQueued(cumulativeWithdrawalsQueuedCall),
        decreaseBeaconChainScalingFactor(decreaseBeaconChainScalingFactorCall),
        decreaseOperatorShares(decreaseOperatorSharesCall),
        delegateTo(delegateToCall),
        delegateToBySignature(delegateToBySignatureCall),
        delegatedTo(delegatedToCall),
        delegationApprover(delegationApproverCall),
        delegationApproverSaltIsSpent(delegationApproverSaltIsSpentCall),
        domainSeparator(domainSeparatorCall),
        eigenPodManager(eigenPodManagerCall),
        getCompletableTimestamp(getCompletableTimestampCall),
        getDepositedShares(getDepositedSharesCall),
        getOperatorShares(getOperatorSharesCall),
        getOperatorsShares(getOperatorsSharesCall),
        getWithdrawableShares(getWithdrawableSharesCall),
        increaseDelegatedShares(increaseDelegatedSharesCall),
        initialize(initializeCall),
        isDelegated(isDelegatedCall),
        isOperator(isOperatorCall),
        modifyOperatorDetails(modifyOperatorDetailsCall),
        operatorDetails(operatorDetailsCall),
        operatorShares(operatorSharesCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        pendingWithdrawals(pendingWithdrawalsCall),
        queueWithdrawals(queueWithdrawalsCall),
        registerAsOperator(registerAsOperatorCall),
        renounceOwnership(renounceOwnershipCall),
        setPauserRegistry(setPauserRegistryCall),
        stakerNonce(stakerNonceCall),
        stakerScalingFactor(stakerScalingFactorCall),
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
            [16u8, 214u8, 122u8, 47u8],
            [19u8, 100u8, 57u8, 221u8],
            [21u8, 196u8, 162u8, 136u8],
            [23u8, 148u8, 187u8, 60u8],
            [27u8, 188u8, 224u8, 145u8],
            [41u8, 199u8, 125u8, 79u8],
            [57u8, 183u8, 14u8, 56u8],
            [60u8, 101u8, 28u8, 242u8],
            [60u8, 222u8, 181u8, 224u8],
            [62u8, 40u8, 57u8, 29u8],
            [67u8, 55u8, 115u8, 130u8],
            [69u8, 124u8, 96u8, 112u8],
            [70u8, 101u8, 188u8, 218u8],
            [73u8, 115u8, 0u8, 96u8],
            [74u8, 95u8, 43u8, 93u8],
            [89u8, 92u8, 106u8, 103u8],
            [89u8, 123u8, 54u8, 218u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 154u8, 237u8, 35u8],
            [101u8, 218u8, 18u8, 100u8],
            [102u8, 213u8, 186u8, 147u8],
            [107u8, 58u8, 167u8, 46u8],
            [109u8, 112u8, 247u8, 174u8],
            [113u8, 80u8, 24u8, 166u8],
            [119u8, 142u8, 85u8, 243u8],
            [127u8, 84u8, 128u8, 113u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [144u8, 4u8, 19u8, 71u8],
            [145u8, 4u8, 195u8, 25u8],
            [148u8, 53u8, 187u8, 67u8],
            [153u8, 190u8, 129u8, 200u8],
            [161u8, 120u8, 132u8, 132u8],
            [165u8, 122u8, 177u8, 11u8],
            [183u8, 240u8, 110u8, 190u8],
            [187u8, 69u8, 254u8, 242u8],
            [197u8, 228u8, 128u8, 219u8],
            [201u8, 75u8, 81u8, 17u8],
            [201u8, 120u8, 247u8, 172u8],
            [202u8, 138u8, 167u8, 199u8],
            [203u8, 0u8, 56u8, 123u8],
            [206u8, 188u8, 4u8, 239u8],
            [218u8, 139u8, 232u8, 100u8],
            [228u8, 204u8, 63u8, 144u8],
            [238u8, 169u8, 6u8, 75u8],
            [240u8, 224u8, 230u8, 118u8],
            [241u8, 97u8, 114u8, 176u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 152u8, 218u8, 37u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelegationManagerCalls {
        const NAME: &'static str = "DelegationManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 54usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DELEGATION_APPROVAL_TYPEHASH(_) => {
                    <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS(_) => {
                    <LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::LEGACY_WITHDRAWAL_CHECK_VALUE(_) => {
                    <LEGACY_WITHDRAWAL_CHECK_VALUECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MIN_WITHDRAWAL_DELAY(_) => {
                    <MIN_WITHDRAWAL_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::STAKER_DELEGATION_TYPEHASH(_) => {
                    <STAKER_DELEGATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beaconChainETHStrategy(_) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateCurrentStakerDelegationDigestHash(_) => {
                    <calculateCurrentStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateDelegationApprovalDigestHash(_) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateStakerDelegationDigestHash(_) => {
                    <calculateStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::cumulativeWithdrawalsQueued(_) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::decreaseBeaconChainScalingFactor(_) => {
                    <decreaseBeaconChainScalingFactorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::decreaseOperatorShares(_) => {
                    <decreaseOperatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegateTo(_) => {
                    <delegateToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegateToBySignature(_) => {
                    <delegateToBySignatureCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodManager(_) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCompletableTimestamp(_) => {
                    <getCompletableTimestampCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::modifyOperatorDetails(_) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorDetails(_) => {
                    <operatorDetailsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::queueWithdrawals(_) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerAsOperator(_) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPauserRegistry(_) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakerNonce(_) => {
                    <stakerNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakerScalingFactor(_) => {
                    <stakerScalingFactorCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn setPauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::setPauserRegistry)
                    }
                    setPauserRegistry
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
                    fn getCompletableTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getCompletableTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getCompletableTimestamp)
                    }
                    getCompletableTimestamp
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
                    fn calculateCurrentStakerDelegationDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <calculateCurrentStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::calculateCurrentStakerDelegationDigestHash,
                            )
                    }
                    calculateCurrentStakerDelegationDigestHash
                },
                {
                    fn stakerNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <stakerNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::stakerNonce)
                    }
                    stakerNonce
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
                    fn STAKER_DELEGATION_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <STAKER_DELEGATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::STAKER_DELEGATION_TYPEHASH)
                    }
                    STAKER_DELEGATION_TYPEHASH
                },
                {
                    fn stakerScalingFactor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <stakerScalingFactorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::stakerScalingFactor)
                    }
                    stakerScalingFactor
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
                    fn MIN_WITHDRAWAL_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <MIN_WITHDRAWAL_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::MIN_WITHDRAWAL_DELAY)
                    }
                    MIN_WITHDRAWAL_DELAY
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
                    fn decreaseBeaconChainScalingFactor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <decreaseBeaconChainScalingFactorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::decreaseBeaconChainScalingFactor,
                            )
                    }
                    decreaseBeaconChainScalingFactor
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
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::avsDirectory)
                    }
                    avsDirectory
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
                    fn delegateToBySignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegateToBySignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::delegateToBySignature)
                    }
                    delegateToBySignature
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
                    fn decreaseOperatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <decreaseOperatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::decreaseOperatorShares)
                    }
                    decreaseOperatorShares
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
                    fn operatorDetails(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <operatorDetailsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::operatorDetails)
                    }
                    operatorDetails
                },
                {
                    fn calculateStakerDelegationDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <calculateStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::calculateStakerDelegationDigestHash,
                            )
                    }
                    calculateStakerDelegationDigestHash
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
                    fn LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS,
                            )
                    }
                    LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS
                },
                {
                    fn LEGACY_WITHDRAWAL_CHECK_VALUE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <LEGACY_WITHDRAWAL_CHECK_VALUECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::LEGACY_WITHDRAWAL_CHECK_VALUE)
                    }
                    LEGACY_WITHDRAWAL_CHECK_VALUE
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
                Self::LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS(inner) => {
                    <LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LEGACY_WITHDRAWAL_CHECK_VALUE(inner) => {
                    <LEGACY_WITHDRAWAL_CHECK_VALUECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MIN_WITHDRAWAL_DELAY(inner) => {
                    <MIN_WITHDRAWAL_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::STAKER_DELEGATION_TYPEHASH(inner) => {
                    <STAKER_DELEGATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beaconChainETHStrategy(inner) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateCurrentStakerDelegationDigestHash(inner) => {
                    <calculateCurrentStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateDelegationApprovalDigestHash(inner) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateStakerDelegationDigestHash(inner) => {
                    <calculateStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::cumulativeWithdrawalsQueued(inner) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::decreaseBeaconChainScalingFactor(inner) => {
                    <decreaseBeaconChainScalingFactorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::decreaseOperatorShares(inner) => {
                    <decreaseOperatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegateToBySignature(inner) => {
                    <delegateToBySignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::getCompletableTimestamp(inner) => {
                    <getCompletableTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::modifyOperatorDetails(inner) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorDetails(inner) => {
                    <operatorDetailsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakerNonce(inner) => {
                    <stakerNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakerScalingFactor(inner) => {
                    <stakerScalingFactorCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS(inner) => {
                    <LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LEGACY_WITHDRAWAL_CHECK_VALUE(inner) => {
                    <LEGACY_WITHDRAWAL_CHECK_VALUECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MIN_WITHDRAWAL_DELAY(inner) => {
                    <MIN_WITHDRAWAL_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::STAKER_DELEGATION_TYPEHASH(inner) => {
                    <STAKER_DELEGATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::calculateCurrentStakerDelegationDigestHash(inner) => {
                    <calculateCurrentStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::calculateStakerDelegationDigestHash(inner) => {
                    <calculateStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::cumulativeWithdrawalsQueued(inner) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::decreaseBeaconChainScalingFactor(inner) => {
                    <decreaseBeaconChainScalingFactorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::decreaseOperatorShares(inner) => {
                    <decreaseOperatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::delegateToBySignature(inner) => {
                    <delegateToBySignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getCompletableTimestamp(inner) => {
                    <getCompletableTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::modifyOperatorDetails(inner) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorDetails(inner) => {
                    <operatorDetailsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakerNonce(inner) => {
                    <stakerNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakerScalingFactor(inner) => {
                    <stakerScalingFactorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        AllocationDelaySet(AllocationDelaySet),
        CallerCannotUndelegate(CallerCannotUndelegate),
        CurrentlyPaused(CurrentlyPaused),
        InputAddressZero(InputAddressZero),
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        InputArrayLengthZero(InputArrayLengthZero),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        InvalidSignature(InvalidSignature),
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
        UnauthorizedCaller(UnauthorizedCaller),
        WithdrawalDelayExceedsMax(WithdrawalDelayExceedsMax),
        WithdrawalDelayExeedsMax(WithdrawalDelayExeedsMax),
        WithdrawalDelayNotElapsed(WithdrawalDelayNotElapsed),
        WithdrawalDoesNotExist(WithdrawalDoesNotExist),
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
            [17u8, 162u8, 85u8, 188u8],
            [35u8, 216u8, 113u8, 165u8],
            [37u8, 236u8, 108u8, 31u8],
            [53u8, 49u8, 50u8, 68u8],
            [60u8, 147u8, 52u8, 70u8],
            [67u8, 113u8, 74u8, 253u8],
            [82u8, 180u8, 205u8, 194u8],
            [88u8, 68u8, 52u8, 212u8],
            [92u8, 66u8, 124u8, 217u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [119u8, 229u8, 106u8, 6u8],
            [121u8, 72u8, 33u8, 255u8],
            [121u8, 108u8, 197u8, 37u8],
            [130u8, 203u8, 24u8, 159u8],
            [132u8, 10u8, 72u8, 213u8],
            [135u8, 201u8, 210u8, 25u8],
            [139u8, 170u8, 87u8, 159u8],
            [142u8, 81u8, 153u8, 168u8],
            [147u8, 115u8, 231u8, 207u8],
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
        const COUNT: usize = 28usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ActivelyDelegated(_) => {
                    <ActivelyDelegated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AllocationDelaySet(_) => {
                    <AllocationDelaySet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CallerCannotUndelegate(_) => {
                    <CallerCannotUndelegate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
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
                Self::UnauthorizedCaller(_) => {
                    <UnauthorizedCaller as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalDelayExceedsMax(_) => {
                    <WithdrawalDelayExceedsMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalDelayExeedsMax(_) => {
                    <WithdrawalDelayExeedsMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalDelayNotElapsed(_) => {
                    <WithdrawalDelayNotElapsed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalDoesNotExist(_) => {
                    <WithdrawalDoesNotExist as alloy_sol_types::SolError>::SELECTOR
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
                    fn WithdrawalDelayExeedsMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawalDelayExeedsMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawalDelayExeedsMax)
                    }
                    WithdrawalDelayExeedsMax
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
                    fn WithdrawalDoesNotExist(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawalDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawalDoesNotExist)
                    }
                    WithdrawalDoesNotExist
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
                    fn UnauthorizedCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <UnauthorizedCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::UnauthorizedCaller)
                    }
                    UnauthorizedCaller
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
                    fn AllocationDelaySet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <AllocationDelaySet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::AllocationDelaySet)
                    }
                    AllocationDelaySet
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
                    fn WithdrawalDelayExceedsMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawalDelayExceedsMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawalDelayExceedsMax)
                    }
                    WithdrawalDelayExceedsMax
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
                Self::AllocationDelaySet(inner) => {
                    <AllocationDelaySet as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::UnauthorizedCaller(inner) => {
                    <UnauthorizedCaller as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawalDelayExceedsMax(inner) => {
                    <WithdrawalDelayExceedsMax as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawalDelayExeedsMax(inner) => {
                    <WithdrawalDelayExeedsMax as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawalDelayNotElapsed(inner) => {
                    <WithdrawalDelayNotElapsed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawalDoesNotExist(inner) => {
                    <WithdrawalDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::AllocationDelaySet(inner) => {
                    <AllocationDelaySet as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::UnauthorizedCaller(inner) => {
                    <UnauthorizedCaller as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawalDelayExceedsMax(inner) => {
                    <WithdrawalDelayExceedsMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawalDelayExeedsMax(inner) => {
                    <WithdrawalDelayExeedsMax as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::WithdrawalDoesNotExist(inner) => {
                    <WithdrawalDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
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
        BeaconChainScalingFactorDecreased(BeaconChainScalingFactorDecreased),
        DepositScalingFactorUpdated(DepositScalingFactorUpdated),
        Initialized(Initialized),
        OperatorDetailsModified(OperatorDetailsModified),
        OperatorMetadataURIUpdated(OperatorMetadataURIUpdated),
        OperatorRegistered(OperatorRegistered),
        OperatorSharesDecreased(OperatorSharesDecreased),
        OperatorSharesIncreased(OperatorSharesIncreased),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        PauserRegistrySet(PauserRegistrySet),
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
                88u8,
                136u8,
                63u8,
                6u8,
                120u8,
                255u8,
                67u8,
                162u8,
                192u8,
                73u8,
                230u8,
                163u8,
                167u8,
                168u8,
                185u8,
                176u8,
                233u8,
                6u8,
                41u8,
                89u8,
                243u8,
                169u8,
                145u8,
                146u8,
                80u8,
                88u8,
                136u8,
                25u8,
                58u8,
                12u8,
                95u8,
                237u8,
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
                110u8,
                159u8,
                205u8,
                83u8,
                152u8,
                150u8,
                252u8,
                166u8,
                14u8,
                139u8,
                15u8,
                1u8,
                221u8,
                88u8,
                2u8,
                51u8,
                228u8,
                138u8,
                107u8,
                15u8,
                125u8,
                240u8,
                19u8,
                184u8,
                155u8,
                167u8,
                245u8,
                101u8,
                134u8,
                154u8,
                205u8,
                182u8,
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
                142u8,
                132u8,
                133u8,
                88u8,
                58u8,
                35u8,
                16u8,
                212u8,
                31u8,
                124u8,
                130u8,
                185u8,
                66u8,
                125u8,
                11u8,
                212u8,
                155u8,
                173u8,
                116u8,
                187u8,
                156u8,
                255u8,
                157u8,
                52u8,
                2u8,
                162u8,
                157u8,
                143u8,
                155u8,
                40u8,
                160u8,
                226u8,
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
                221u8,
                249u8,
                53u8,
                236u8,
                136u8,
                37u8,
                199u8,
                175u8,
                238u8,
                106u8,
                21u8,
                212u8,
                115u8,
                30u8,
                40u8,
                150u8,
                62u8,
                233u8,
                109u8,
                252u8,
                184u8,
                93u8,
                10u8,
                30u8,
                121u8,
                75u8,
                67u8,
                49u8,
                139u8,
                188u8,
                164u8,
                253u8,
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
                190u8,
                92u8,
                210u8,
                75u8,
                44u8,
                188u8,
                123u8,
                6u8,
                91u8,
                157u8,
                15u8,
                222u8,
                185u8,
                4u8,
                70u8,
                30u8,
                74u8,
                252u8,
                255u8,
                87u8,
                221u8,
                87u8,
                172u8,
                218u8,
                30u8,
                120u8,
                50u8,
                3u8,
                27u8,
                167u8,
                172u8,
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
        const COUNT: usize = 17usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <BeaconChainScalingFactorDecreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <BeaconChainScalingFactorDecreased as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BeaconChainScalingFactorDecreased)
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
                    <OperatorDetailsModified as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorDetailsModified as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorDetailsModified)
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
                    <PauserRegistrySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PauserRegistrySet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PauserRegistrySet)
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
                Self::BeaconChainScalingFactorDecreased(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DepositScalingFactorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDetailsModified(inner) => {
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
                Self::PauserRegistrySet(inner) => {
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
                Self::BeaconChainScalingFactorDecreased(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DepositScalingFactorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDetailsModified(inner) => {
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
                Self::PauserRegistrySet(inner) => {
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
        _avsDirectory: alloy::sol_types::private::Address,
        _strategyManager: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
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
            _avsDirectory,
            _strategyManager,
            _eigenPodManager,
            _allocationManager,
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
        _avsDirectory: alloy::sol_types::private::Address,
        _strategyManager: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _MIN_WITHDRAWAL_DELAY: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        DelegationManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _avsDirectory,
            _strategyManager,
            _eigenPodManager,
            _allocationManager,
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
            _avsDirectory: alloy::sol_types::private::Address,
            _strategyManager: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _MIN_WITHDRAWAL_DELAY: u32,
        ) -> alloy_contract::Result<DelegationManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _avsDirectory,
                _strategyManager,
                _eigenPodManager,
                _allocationManager,
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
            _avsDirectory: alloy::sol_types::private::Address,
            _strategyManager: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _MIN_WITHDRAWAL_DELAY: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _avsDirectory,
                            _strategyManager,
                            _eigenPodManager,
                            _allocationManager,
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
        ///Creates a new call builder for the [`LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS`] function.
        pub fn LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall,
            N,
        > {
            self.call_builder(
                &LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKSCall {
                },
            )
        }
        ///Creates a new call builder for the [`LEGACY_WITHDRAWAL_CHECK_VALUE`] function.
        pub fn LEGACY_WITHDRAWAL_CHECK_VALUE(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            LEGACY_WITHDRAWAL_CHECK_VALUECall,
            N,
        > {
            self.call_builder(
                &LEGACY_WITHDRAWAL_CHECK_VALUECall {
                },
            )
        }
        ///Creates a new call builder for the [`MIN_WITHDRAWAL_DELAY`] function.
        pub fn MIN_WITHDRAWAL_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MIN_WITHDRAWAL_DELAYCall, N> {
            self.call_builder(&MIN_WITHDRAWAL_DELAYCall {})
        }
        ///Creates a new call builder for the [`STAKER_DELEGATION_TYPEHASH`] function.
        pub fn STAKER_DELEGATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, STAKER_DELEGATION_TYPEHASHCall, N> {
            self.call_builder(&STAKER_DELEGATION_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`beaconChainETHStrategy`] function.
        pub fn beaconChainETHStrategy(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, beaconChainETHStrategyCall, N> {
            self.call_builder(&beaconChainETHStrategyCall {})
        }
        ///Creates a new call builder for the [`calculateCurrentStakerDelegationDigestHash`] function.
        pub fn calculateCurrentStakerDelegationDigestHash(
            &self,
            staker: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            calculateCurrentStakerDelegationDigestHashCall,
            N,
        > {
            self.call_builder(
                &calculateCurrentStakerDelegationDigestHashCall {
                    staker,
                    operator,
                    expiry,
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
        ///Creates a new call builder for the [`calculateStakerDelegationDigestHash`] function.
        pub fn calculateStakerDelegationDigestHash(
            &self,
            staker: alloy::sol_types::private::Address,
            nonce: alloy::sol_types::private::primitives::aliases::U256,
            operator: alloy::sol_types::private::Address,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            calculateStakerDelegationDigestHashCall,
            N,
        > {
            self.call_builder(
                &calculateStakerDelegationDigestHashCall {
                    staker,
                    nonce,
                    operator,
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
        ///Creates a new call builder for the [`cumulativeWithdrawalsQueued`] function.
        pub fn cumulativeWithdrawalsQueued(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, cumulativeWithdrawalsQueuedCall, N> {
            self.call_builder(
                &cumulativeWithdrawalsQueuedCall {
                    _0,
                },
            )
        }
        ///Creates a new call builder for the [`decreaseBeaconChainScalingFactor`] function.
        pub fn decreaseBeaconChainScalingFactor(
            &self,
            staker: alloy::sol_types::private::Address,
            existingDepositShares: alloy::sol_types::private::primitives::aliases::U256,
            proportionOfOldBalance: u64,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            decreaseBeaconChainScalingFactorCall,
            N,
        > {
            self.call_builder(
                &decreaseBeaconChainScalingFactorCall {
                    staker,
                    existingDepositShares,
                    proportionOfOldBalance,
                },
            )
        }
        ///Creates a new call builder for the [`decreaseOperatorShares`] function.
        pub fn decreaseOperatorShares(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            previousTotalMagnitude: u64,
            newTotalMagnitude: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, decreaseOperatorSharesCall, N> {
            self.call_builder(
                &decreaseOperatorSharesCall {
                    operator,
                    strategy,
                    previousTotalMagnitude,
                    newTotalMagnitude,
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
        ///Creates a new call builder for the [`delegateToBySignature`] function.
        pub fn delegateToBySignature(
            &self,
            staker: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
            stakerSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateToBySignatureCall, N> {
            self.call_builder(
                &delegateToBySignatureCall {
                    staker,
                    operator,
                    stakerSignatureAndExpiry,
                    approverSignatureAndExpiry,
                    approverSalt,
                },
            )
        }
        ///Creates a new call builder for the [`delegatedTo`] function.
        pub fn delegatedTo(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegatedToCall, N> {
            self.call_builder(&delegatedToCall { _0 })
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
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            delegationApproverSaltIsSpentCall,
            N,
        > {
            self.call_builder(
                &delegationApproverSaltIsSpentCall {
                    _0,
                    _1,
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
        ///Creates a new call builder for the [`getCompletableTimestamp`] function.
        pub fn getCompletableTimestamp(
            &self,
            startTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCompletableTimestampCall, N> {
            self.call_builder(
                &getCompletableTimestampCall {
                    startTimestamp,
                },
            )
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
            existingDepositShares: alloy::sol_types::private::primitives::aliases::U256,
            addedShares: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, increaseDelegatedSharesCall, N> {
            self.call_builder(
                &increaseDelegatedSharesCall {
                    staker,
                    strategy,
                    existingDepositShares,
                    addedShares,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initialOwner,
                    _pauserRegistry,
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
        ///Creates a new call builder for the [`modifyOperatorDetails`] function.
        pub fn modifyOperatorDetails(
            &self,
            newOperatorDetails: <IDelegationManagerTypes::OperatorDetails as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyOperatorDetailsCall, N> {
            self.call_builder(
                &modifyOperatorDetailsCall {
                    newOperatorDetails,
                },
            )
        }
        ///Creates a new call builder for the [`operatorDetails`] function.
        pub fn operatorDetails(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorDetailsCall, N> {
            self.call_builder(&operatorDetailsCall { operator })
        }
        ///Creates a new call builder for the [`operatorShares`] function.
        pub fn operatorShares(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSharesCall, N> {
            self.call_builder(&operatorSharesCall { _0, _1 })
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
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, pendingWithdrawalsCall, N> {
            self.call_builder(&pendingWithdrawalsCall { _0 })
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
        ///Creates a new call builder for the [`registerAsOperator`] function.
        pub fn registerAsOperator(
            &self,
            registeringOperatorDetails: <IDelegationManagerTypes::OperatorDetails as alloy::sol_types::SolType>::RustType,
            allocationDelay: u32,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerAsOperatorCall, N> {
            self.call_builder(
                &registerAsOperatorCall {
                    registeringOperatorDetails,
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
        ///Creates a new call builder for the [`setPauserRegistry`] function.
        pub fn setPauserRegistry(
            &self,
            newPauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setPauserRegistryCall, N> {
            self.call_builder(
                &setPauserRegistryCall {
                    newPauserRegistry,
                },
            )
        }
        ///Creates a new call builder for the [`stakerNonce`] function.
        pub fn stakerNonce(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakerNonceCall, N> {
            self.call_builder(&stakerNonceCall { _0 })
        }
        ///Creates a new call builder for the [`stakerScalingFactor`] function.
        pub fn stakerScalingFactor(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakerScalingFactorCall, N> {
            self.call_builder(&stakerScalingFactorCall { _0, _1 })
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
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorMetadataURICall, N> {
            self.call_builder(
                &updateOperatorMetadataURICall {
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
        ///Creates a new event filter for the [`BeaconChainScalingFactorDecreased`] event.
        pub fn BeaconChainScalingFactorDecreased_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BeaconChainScalingFactorDecreased, N> {
            self.event_filter::<BeaconChainScalingFactorDecreased>()
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
        ///Creates a new event filter for the [`OperatorDetailsModified`] event.
        pub fn OperatorDetailsModified_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDetailsModified, N> {
            self.event_filter::<OperatorDetailsModified>()
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
        ///Creates a new event filter for the [`PauserRegistrySet`] event.
        pub fn PauserRegistrySet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PauserRegistrySet, N> {
            self.event_filter::<PauserRegistrySet>()
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
